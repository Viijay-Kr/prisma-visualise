use std::{
    fmt::{format, Arguments},
    mem::ManuallyDrop,
};

use psl_core::schema_ast::ast::{Argument, ArgumentsList, Attribute, Expression};

use crate::constraints::Constraint;

#[derive(Clone)]
pub struct Index {
    name: String,
}

#[derive(Clone)]
pub enum ModelAttributTypes {
    Index(Index),
    Id(Constraint),
    Unique(Constraint),
}
pub struct ModelAttributes {
    pub(crate) values: Vec<ModelAttributTypes>,
}

impl Clone for ModelAttributes {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }
}

impl ModelAttributes {
    pub fn new() -> ModelAttributes {
        ModelAttributes { values: vec![] }
    }
    pub fn populate(&mut self, model_attributes: &Vec<Attribute>) {
        model_attributes.iter().for_each(|attr| {
            attr.arguments
                .arguments
                .iter()
                .for_each(|arg| match &arg.name {
                    Some(_n) => {}
                    None => match &arg.value {
                        Expression::Array(v, _s) => v.iter().for_each(|exp| match exp {
                            Expression::ConstantValue(v, _s) => {
                                if attr.name.name == "index" {
                                    self.values.push(ModelAttributTypes::Index(Index {
                                        name: String::from(v),
                                    }));
                                }
                                if attr.name.name == "unique" {
                                    self.values.push(ModelAttributTypes::Unique(Constraint {
                                        name: String::from(v),
                                        argument: vec![],
                                    }));
                                }
                                if attr.name.name == "id" {
                                    self.values.push(ModelAttributTypes::Id(Constraint {
                                        name: String::from(v),
                                        argument: vec![],
                                    }));
                                }
                            }
                            _ => {}
                        }),
                        _ => {}
                    },
                })
        })
    }
    pub fn is_index(&self, field: &str) -> &str {
        let index = self.values.iter().find(|x| match x {
            ModelAttributTypes::Index(index) => index.name == field,
            ModelAttributTypes::Id(_) => false,
            ModelAttributTypes::Unique(_) => false,
        });
        match index {
            Some(_v) => "true",
            None => "false",
        }
    }
    pub fn constraint_strings(&self, field: &str) -> String {
        self.values
            .iter()
            .filter(|v| match v {
                ModelAttributTypes::Id(id) => id.name == field,
                ModelAttributTypes::Unique(uq) => uq.name == field,
                _ => false,
            })
            .map(|v| match v {
                ModelAttributTypes::Id(..) => "id".to_owned(),
                ModelAttributTypes::Unique(..) => "unique".to_owned(),
                _ => "".to_owned(),
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Clone, Copy)]
pub enum ArgumentValueKind {
    Array,
    Function,
    Number,
    StringLiteral,
    ConstantValue,
}

trait ArgumentMarkup {}
#[derive(Clone)]
pub struct CombinedArgument {
    pub name: String,
    pub value: String,
    pub kind: ArgumentValueKind,
}

impl CombinedArgument {
    pub fn markup(&self, is_part_of_relational: bool) -> String {
        let name_markup = if self.name.len() > 0 {
            if is_part_of_relational {
                format!(
                    r#"<span class="argument-name argument-name-is-relational">{}:</span>"#,
                    self.name
                )
            } else {
                format!(r#"<span class="argument-name">{}:</span>"#, self.name)
            }
        } else {
            "".to_string()
        };
        if is_part_of_relational {
            format!(
                r#"
                            <span class="arguments-container">
                                {}          
                                <span class="argument-type argument-type-is-relational">{}</span>
                            </span>
                    "#,
                name_markup, self.value
            )
        } else {
            format!(
                r#"
                        <span class="arguments-container">
                            {}          
                            <span class="argument-type">{}</span>
                        </span>
                "#,
                name_markup, self.value
            )
        }
    }
}
#[derive(Clone)]
pub struct NonCombinedArgument {
    pub name: String,
}
impl NonCombinedArgument {
    pub fn markup(&self) -> String {
        format!(
            r#"
                            <span class="argument-name">{}</span>
                        "#,
            self.name
        )
    }
}

#[repr(C)]
pub union PslArgument {
    pub combined_argument: ManuallyDrop<CombinedArgument>,
    pub non_combined_argument: ManuallyDrop<NonCombinedArgument>,
}
pub struct PslAttribute {
    pub name: String,
    pub arguments: Vec<PslArgument>,
    pub is_relation: bool,
}
impl PslAttribute {
    pub fn new(name: String, is_relation: bool) -> PslAttribute {
        PslAttribute {
            name,
            arguments: vec![],
            is_relation,
        }
    }
    pub fn resolve_arguments(&mut self, args: ArgumentsList) {
        if args.empty_arguments.len() > 0 {
            args.empty_arguments.iter().for_each(|empty_arg| {
                self.arguments.push(PslArgument {
                    non_combined_argument: ManuallyDrop::new(NonCombinedArgument {
                        name: empty_arg.name.name.clone(),
                    }),
                })
            })
        } else if args.arguments.len() > 0 {
            args.arguments.iter().for_each(|non_empty_arg| {
                self.arguments.push(PslArgument {
                    combined_argument: ManuallyDrop::new(CombinedArgument {
                        name: match &non_empty_arg.name {
                            Some(n) => n.name.clone(),
                            None => "".to_string(),
                        },
                        value: format!("{}", non_empty_arg.value),
                        kind: match non_empty_arg.value {
                            Expression::NumericValue(_, _) => ArgumentValueKind::Number,
                            Expression::StringValue(_, _) => ArgumentValueKind::StringLiteral,
                            Expression::ConstantValue(_, _) => ArgumentValueKind::ConstantValue,
                            Expression::Function(_, _, _) => ArgumentValueKind::Function,
                            Expression::Array(_, _) => ArgumentValueKind::Array,
                        },
                    }),
                })
            })
        }
    }
    pub fn attribute_name_markup(&self) -> String {
        format!(
            r#"
                    <span class="attribute-name">
                        @{}
                    </span>
                "#,
            self.name
        )
    }
    pub fn arguments_markup(&self) -> Vec<String> {
        self.arguments
            .iter()
            .map(|arg| unsafe {
                match arg {
                    PslArgument { combined_argument } => combined_argument.markup(self.is_relation),
                    PslArgument {
                        non_combined_argument,
                    } => non_combined_argument.markup(),
                }
            })
            .collect::<Vec<String>>()
    }
    pub fn arugments_open(&self) -> String {
        if self.arguments.len() > 0 {
            format!(r#"<span class="open-argument">(</span>"#)
        } else {
            format!("")
        }
    }

    pub fn arugments_close(&self) -> String {
        if self.arguments.len() > 0 {
            format!(r#"<span class="close-argument">)</span>"#)
        } else {
            format!("")
        }
    }
}
