use std::fmt::Display;

use psl_core::{
    parser_database::{
        walkers::{RefinedFieldWalker, Walker},
        ScalarType,
    },
    schema_ast::ast::{FieldArity, FieldId, ModelId},
};

use crate::attributes::{PslArgument, PslAttribute};

#[derive(Clone, Debug)]
pub enum DataTypes {
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
    Unknown(String),
}

impl Display for DataTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug)]
pub struct PrismaVizFieldType {
    pub data_type: DataTypes,
    pub modifier: String,
}

impl Display for PrismaVizFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.data_type, self.modifier)
    }
}

impl PrismaVizFieldType {
    pub fn new() -> PrismaVizFieldType {
        PrismaVizFieldType {
            data_type: DataTypes::Unknown("pending".to_string()),
            modifier: "".to_string(),
        }
    }
    pub fn resolve_data_type(&mut self, name: String, modifier: String) {
        self.modifier = modifier;
        self.data_type = match name.as_str() {
            "Int" => DataTypes::Int(name.clone()),
            "String" => DataTypes::VarChar(name.clone()),
            "BigInt" => DataTypes::BigInt(name.clone()),
            "Float" => DataTypes::Float(name.clone()),
            "Decimal" => DataTypes::Decimal(name.clone()),
            "DateTime" => DataTypes::DateTime(name.clone()),
            "Boolean" => DataTypes::Boolean(name.clone()),
            "Json" => DataTypes::Json(name.clone()),
            "Bytes" => DataTypes::Bytes(name.clone()),
            "Unsupported" => DataTypes::Unsupported(name.clone()),
            _ => DataTypes::Relational(name.clone()),
        };
    }
    pub fn get_data_type(&self) -> String {
        match &self.data_type {
            DataTypes::Int(v) => v.clone(),
            DataTypes::VarChar(v) => v.clone(),
            DataTypes::Boolean(v) => v.clone(),
            DataTypes::BigInt(v) => v.clone(),
            DataTypes::Float(v) => v.clone(),
            DataTypes::Decimal(v) => v.clone(),
            DataTypes::DateTime(v) => v.clone(),
            DataTypes::Json(v) => v.clone(),
            DataTypes::Bytes(v) => v.clone(),
            DataTypes::Unsupported(v) => v.clone(),
            DataTypes::Relational(v) => v.clone(),
            DataTypes::Unknown(v) => v.clone(),
        }
    }
    pub fn resolve_with_modifier(&self) -> String {
        self.get_data_type().to_owned() + &self.modifier.to_owned()
    }
}

pub struct PslField {
    pub name: String,
    pub field_type: String,
    pub modifier: String,
    pub attributes: Vec<PslAttribute>,
    pub is_relational: bool,
}

impl PslField {
    pub fn new(name: String) -> PslField {
        return PslField {
            name,
            field_type: "".to_string(),
            modifier: String::from(""),
            attributes: vec![],
            is_relational: false,
        };
    }

    pub fn resolve_field(&mut self, field: Walker<'_, (ModelId, FieldId)>) {
        let refined_field_type = field.refine();
        self.modifier = match field.ast_field().arity {
            FieldArity::Required => "".to_owned(),
            FieldArity::Optional => "?".to_owned(),
            FieldArity::List => "[]".to_owned(),
        };
        match refined_field_type {
            RefinedFieldWalker::Scalar(f) => {
                self.field_type = match f.scalar_type() {
                    Some(v) => match v {
                        ScalarType::Int => "Int".to_string(),
                        ScalarType::BigInt => "BigInt".to_string(),
                        ScalarType::Float => "Float".to_string(),
                        ScalarType::Boolean => "Boolean".to_string(),
                        ScalarType::String => "String".to_string(),
                        ScalarType::DateTime => "DateTime".to_string(),
                        ScalarType::Json => "Json".to_string(),
                        ScalarType::Bytes => "Bytes".to_string(),
                        ScalarType::Decimal => "Decimal".to_string(),
                    },
                    None => {
                        println!("Looks like field type is not resolvable");
                        "".to_string()
                    }
                };
                // Start filling the attributes
                f.ast_field()
                    .attributes
                    .clone()
                    .into_iter()
                    .for_each(|attr| {
                        let mut attribute = PslAttribute::new(attr.name.name, false);
                        attribute.resolve_arguments(attr.arguments);
                        self.attributes.push(attribute)
                    })
            }
            RefinedFieldWalker::Relation(f) => {
                self.field_type = f.name().to_string();
                self.is_relational = true;
                f.ast_field()
                    .attributes
                    .clone()
                    .into_iter()
                    .for_each(|attr| {
                        let mut attribute = PslAttribute::new(attr.name.name, true);
                        attribute.resolve_arguments(attr.arguments);
                        self.attributes.push(attribute)
                    })
            }
        }
    }

    pub fn resolve_field_name_markup(&self) -> String {
        format!(r#"<span class="field-name">{}</span>"#, self.name)
    }

    pub fn resolve_field_type_markup(&self) -> String {
        format!(
            r#"
                    <span class="field-type">
                        {}
                        <span class="field-modifier">
                            {}
                        </span>
                    </span>"#,
            self.field_type, self.modifier
        )
    }

    pub fn resolve_attributes_markup(&self) -> Vec<String> {
        self.attributes
            .iter()
            .map(|attr| {
                let attr_name_markup = attr.attribute_name_markup();
                let arguments_markup = attr.arguments_markup();
                let arguments_open = attr.arugments_open();
                let arguments_close = attr.arugments_close();
                format!(
                    r#"
                        <span class="attributes">
                            {}
                            {}
                            {}
                            {}
                        </span>
                    "#,
                    attr_name_markup,
                    arguments_open,
                    arguments_markup.join(r#"<span>,</span>"#),
                    arguments_close
                )
            })
            .collect::<Vec<String>>()
    }
}
