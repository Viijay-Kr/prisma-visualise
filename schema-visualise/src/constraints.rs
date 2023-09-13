use psl_core::schema_ast::ast::{Attribute, Expression};

#[derive(PartialEq, Clone, Eq, Hash)]
pub enum ArgumentType {
    String(String),
    Numeric(String),
    FunctionCall(String),
}

#[derive(PartialEq, Clone, Eq, Hash)]
pub struct Constraint {
    pub name: String,
    pub argument: Vec<ArgumentType>,
}

impl Constraint {
    pub fn to_string(&self) -> String {
        let mut arguments = self
            .argument
            .iter()
            .map(|arg| match arg {
                ArgumentType::Numeric(v) => {
                    return String::from(v);
                }
                ArgumentType::String(v) => {
                    return String::from(v);
                }
                ArgumentType::FunctionCall(v) => {
                    return String::from(v);
                }
            })
            .collect::<Vec<String>>()
            .join(",");

        arguments = match self.argument.len() {
            0 => "".to_string(),
            _ => {
                let temp = format!("{}{}{}", "(", arguments.clone(), ")");
                temp
            }
        };
        return self.name.clone() + &arguments.clone();
    }
}
pub struct Contraints {
    pub constraints: Vec<Constraint>,
}

impl Contraints {
    pub fn new() -> Contraints {
        Contraints {
            constraints: vec![],
        }
    }
    pub fn populate(&mut self, attributes: &Vec<Attribute>) {
        attributes.iter().for_each(|a| {
            let attr_name = a.name.name.clone();
            let mut attribute_constraint = Constraint {
                name: attr_name.clone(),
                argument: vec![],
            };
            if attr_name != "relation" {
                a.arguments
                    .arguments
                    .iter()
                    .for_each(|arg| match &arg.value {
                        Expression::NumericValue(v, _s) => attribute_constraint
                            .argument
                            .push(ArgumentType::String(v.to_string())),
                        Expression::StringValue(v, _s) => attribute_constraint
                            .argument
                            .push(ArgumentType::Numeric(v.to_string())),
                        Expression::Function(v, _arg, _) => attribute_constraint
                            .argument
                            .push(ArgumentType::FunctionCall(v.to_string() + "()")),
                        _ => {}
                    })
            } else {
                a.arguments
                    .arguments
                    .iter()
                    .for_each(|arg| match &arg.name {
                        Some(v) => {
                            if v.name == "name" {
                                match &arg.value {
                                    Expression::StringValue(v, _s) => attribute_constraint
                                        .argument
                                        .push(ArgumentType::String(v.to_string())),
                                    _ => {}
                                }
                            }
                        }
                        None => match &arg.value {
                            Expression::StringValue(v, _s) => attribute_constraint
                                .argument
                                .push(ArgumentType::String(v.to_string())),
                            _ => {}
                        },
                    })
            }
            self.constraints.push(attribute_constraint);
        });
    }

    pub fn to_string(&self) -> String {
        let output = self.constraints.iter().map(|c| c.to_string());
        return output.collect::<Vec<String>>().join("\n");
    }
}
