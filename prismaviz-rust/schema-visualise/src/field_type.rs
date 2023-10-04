use std::fmt::Display;

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

pub struct RefinedFieldType {
    pub raw: String,
    pub html: String,
}

impl RefinedFieldType {
    pub fn new(raw: String) -> RefinedFieldType {
        return RefinedFieldType {
            raw,
            html: String::from(""),
        };
    }

    pub fn resolve_psl_type(&self) {
        
    }
}
