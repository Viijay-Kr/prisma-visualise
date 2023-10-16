use psl_core::schema_ast::ast::{Attribute, Expression};

pub struct RelationalField {
    pub value: String,
}
pub struct RelationalReference {
    pub value: String,
}
pub enum Relations {
    Field(RelationalField),
    Reference(RelationalReference),
}

pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToMany,
    SelfRelation,
    MayBe(Box<RelationshipType>),
    None,
}
pub struct RelationShips {
    pub relations: Vec<Relations>,
    pub r#type: RelationshipType,
    pub on: String,
}

impl RelationShips {
    pub fn new() -> RelationShips {
        RelationShips {
            relations: vec![],
            r#type: RelationshipType::None,
            on: String::from(""),
        }
    }
    pub fn populate(&mut self, attributes: &Vec<Attribute>) {
        attributes.iter().for_each(|attribute| {
            if attribute.name.name == "relation" {
                attribute
                    .arguments
                    .arguments
                    .iter()
                    .for_each(|arg| match &arg.value {
                        Expression::Array(v, _s) => {
                            for exp in v.iter() {
                                match exp {
                                    Expression::ConstantValue(v, _s) => match &arg.name {
                                        Some(n) => {
                                            if n.name == "fields" {
                                                self.relations.push(Relations::Field(
                                                    RelationalField { value: v.clone() },
                                                ));
                                            }
                                            if n.name == "references" {
                                                self.relations.push(Relations::Reference(
                                                    RelationalReference { value: v.clone() },
                                                ));
                                            }
                                        }
                                        None => {}
                                    },
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    })
            } else {
            }
        })
    }
    pub fn fields(&self) -> Vec<String> {
        self.relations
            .iter()
            .map(|rel| match rel {
                Relations::Field(rf) => String::from(rf.value.clone()),
                _ => String::from("none"),
            })
            .filter(|rel| rel != "none")
            .collect::<Vec<String>>()
    }
    pub fn references(&self) -> Vec<String> {
        self.relations
            .iter()
            .map(|rel| match rel {
                Relations::Reference(rr) => String::from(rr.value.clone()),
                _ => String::from("none"),
            })
            .filter(|rel| rel != "none")
            .collect::<Vec<String>>()
    }
}

impl Default for RelationShips {
    fn default() -> Self {
        Self::new()
    }
}
