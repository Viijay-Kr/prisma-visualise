use psl_core::schema_ast::ast::{Attribute, Expression};

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
