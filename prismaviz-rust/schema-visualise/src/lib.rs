mod attributes;
mod constraints;
mod relations;

use std::fmt::Display;

pub use crate::{attributes::ModelAttributes, constraints::Contraints, relations::RelationShips};
use prettytable::row;
use prettytable::Table;
use psl_core::schema_ast::ast::FieldType;
use psl_core::schema_ast::ast::WithIdentifier;
use psl_core::{
    diagnostics::Diagnostics,
    schema_ast::{self, ast::SchemaAst},
};

#[derive(Clone, Debug)]
pub enum DataTypeEnum {
    Int(String),
    VarChar(String),
    Boolean(String),
    BigInt(String),
    Float(String),
    Decimal(String),
    DateTime(String),
    Json(String),
    Bytes(String),
    Unsupported(String),
    Relational(String),
}

impl Display for DataTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct DataType {
    pub data_type: DataTypeEnum,
    pub modifier: String,
}
impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.data_type, self.modifier)
    }
}
pub struct PrismaVizModelField {
    pub attributes: ModelAttributes,
    pub relation_ships: RelationShips,
    pub constraints: Contraints,
    pub name: String,
    pub data_type: DataType,
    pub is_index: String,
}

impl PrismaVizModelField {
    fn new(name: String, data_type: DataType, is_index: &str) -> PrismaVizModelField {
        PrismaVizModelField {
            attributes: ModelAttributes { values: vec![] },
            relation_ships: RelationShips {
                relations: vec![],
                r#type: relations::RelationshipType::None,
                on: String::from(""),
            },
            constraints: Contraints {
                constraints: vec![],
            },
            name,
            data_type,
            is_index: is_index.to_string(),
        }
    }
}
pub struct PrismaVizModel {
    pub name: String,
    pub fields: Vec<PrismaVizModelField>,
    pub code: String,
}

impl PrismaVizModel {
    pub fn new(name: String, code: String) -> PrismaVizModel {
        PrismaVizModel {
            name,
            fields: vec![],
            code,
        }
    }
}

pub struct SchemaVisualiser {
    pub schema: String,
    pub models: Vec<PrismaVizModel>,
}

impl SchemaVisualiser {
    pub fn new(contents: String) -> SchemaVisualiser {
        SchemaVisualiser {
            schema: contents,
            models: vec![],
        }
    }
    pub fn get_models(self) -> Vec<PrismaVizModel> {
        self.models
    }
    pub fn parse(&mut self) {
        let mut diagnostics = Diagnostics::default();
        let result: SchemaAst = schema_ast::parse_schema(self.schema.as_str(), &mut diagnostics);
        let model_fields = result
            .iter_tops()
            .filter(|(_, top)| top.get_type() == "model")
            .map(|(_, top)| top.as_model())
            .flatten()
            .map(|m| {
                (
                    m.identifier().name.clone(),
                    m.iter_fields(),
                    m.attributes.clone(),
                    m.span.clone(),
                )
            });
        model_fields.for_each(|(model, fields, attributes, span)| {
            let code = &self.schema.as_str()[span.start..span.end];
            let mut prisma_viz_model = PrismaVizModel::new(model, String::from(code));

            let mut model_attributes = ModelAttributes::new();
            model_attributes.populate(&attributes);
            fields.for_each(|(_, field)| {
                let model_attributes = model_attributes.to_owned();

                let mut constraints = Contraints::new();
                constraints.populate(&field.attributes);
                let mut relationships = RelationShips::new();
                relationships.populate(&field.attributes);
                let data_type = match &field.field_type {
                    FieldType::Unsupported(t, _) => {
                        format!("Field type {}", t);
                        let dtype = DataType {
                            data_type: DataTypeEnum::Unsupported(t.to_owned()),
                            modifier: String::from(""),
                        };
                        dtype
                    }
                    FieldType::Supported(t) => {
                        let name = t.name.to_string();
                        let len = name.len();
                        let mut modifier =
                            String::from(&self.schema[t.span.start + len..t.span.end + 2]);
                        if modifier != String::from("[]") {
                            modifier = String::from("");
                        }
                        let dtype = match name.as_str() {
                            "Int" => DataType {
                                data_type: DataTypeEnum::Int(name.clone()),
                                modifier,
                            },
                            "String" => DataType {
                                data_type: DataTypeEnum::VarChar(name.clone()),
                                modifier,
                            },
                            "BigInt" => DataType {
                                data_type: DataTypeEnum::BigInt(name.clone()),
                                modifier,
                            },
                            "Float" => DataType {
                                data_type: DataTypeEnum::Float(name.clone()),
                                modifier,
                            },
                            "Decimal" => DataType {
                                data_type: DataTypeEnum::Decimal(name.clone()),
                                modifier,
                            },
                            "DateTime" => DataType {
                                data_type: DataTypeEnum::DateTime(name.clone()),
                                modifier,
                            },
                            "Boolean" => DataType {
                                data_type: DataTypeEnum::Boolean(name.clone()),
                                modifier,
                            },
                            "Json" => DataType {
                                data_type: DataTypeEnum::Json(name.clone()),
                                modifier,
                            },
                            "Bytes" => DataType {
                                data_type: DataTypeEnum::Bytes(name.clone()),
                                modifier,
                            },
                            _ => DataType {
                                data_type: DataTypeEnum::Relational(name.clone()),
                                modifier,
                            },
                        };
                        dtype
                    }
                };

                let is_index = model_attributes.is_index(field.name());

                let mut prisma_vis_model_field =
                    PrismaVizModelField::new(field.name().to_string(), data_type, is_index);
                prisma_vis_model_field.relation_ships = relationships;
                prisma_vis_model_field.constraints = constraints;
                prisma_vis_model_field.attributes = model_attributes;
                prisma_viz_model.fields.push(prisma_vis_model_field);
            });

            self.models.push(prisma_viz_model);
        });
    }
    pub fn print_as_table(&mut self) {
        self.parse();
        self.models.iter().for_each(|model| {
            println!("Model {}", model.name);
            let mut table: Table = Table::new();
            table.add_row(row![
                "Name",
                "Type",
                "Attributes_Constraints",
                "Relation_Fields",
                "Relation_References",
                "Index",
            ]);

            model.fields.iter().for_each(|field| {
                let mut constraint_strings = field.constraints.to_string();
                constraint_strings = format!(
                    "{}\n{}",
                    constraint_strings,
                    field.attributes.constraint_strings(&field.name)
                );
                let field_type = match &field.data_type.data_type {
                    DataTypeEnum::Int(v) => v,
                    DataTypeEnum::VarChar(v) => v,
                    DataTypeEnum::Boolean(v) => v,
                    DataTypeEnum::BigInt(v) => v,
                    DataTypeEnum::Float(v) => v,
                    DataTypeEnum::Decimal(v) => v,
                    DataTypeEnum::DateTime(v) => v,
                    DataTypeEnum::Json(v) => v,
                    DataTypeEnum::Bytes(v) => v,
                    DataTypeEnum::Unsupported(v) => v,
                    DataTypeEnum::Relational(v) => v,
                };
                table.add_row(row![
                    field.name,
                    field_type.to_owned() + &field.data_type.modifier,
                    constraint_strings,
                    field.relation_ships.fields().join("\n"),
                    field.relation_ships.references().join("\n"),
                    field.is_index,
                ]);
            });

            table.printstd();
            println!("");
        })
    }
}
