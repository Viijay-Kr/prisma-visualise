use clap::Parser;

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
struct Args {
    /// Optional file arugment indicating the full path to your 'primsa.schema' file
    #[arg(long)]
    file: Option<String>,
}

mod attributes;
mod constraints;
mod relations;
#[macro_use]
extern crate prettytable;

use prettytable::Table;

use psl_core::{
    diagnostics::Diagnostics,
    schema_ast::{
        self,
        ast::{FieldType, SchemaAst, WithIdentifier},
    },
};

use crate::{attributes::ModelAttributes, constraints::Contraints, relations::RelationShips};

fn main() {
    let args = Args::parse();
    match args.file {
        None => {
            panic!("Err!!!You forgot to pass a path to a file");
        }
        Some(v) => {
            parse_schema(&v);
        }
    }
}

pub fn parse_schema(path: &String) {
    let file = std::fs::read_to_string(&path).unwrap();
    let mut diagnostics = Diagnostics::default();
    let result: SchemaAst = schema_ast::parse_schema(&file, &mut diagnostics);
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
            )
        });

    model_fields.for_each(|(model, fields, attributes)| {
        println!("Model {}", model);

        let mut table: Table = Table::new();
        table.add_row(row![
            "Name",
            "Type",
            "Attributes_Constraints",
            "Relation_Fields",
            "Relation_References",
            "Index",
        ]);
        let mut model_attributes = ModelAttributes::new();
        model_attributes.populate(&attributes);

        fields.for_each(|(_, field)| {
            let model_attributes = model_attributes.to_owned();
            let field_type = match &field.field_type {
                FieldType::Unsupported(t, _) => t.to_string(),
                FieldType::Supported(t) => {
                    let name = t.name.to_string();
                    name
                }
            };

            let mut constraints = Contraints::new();
            constraints.populate(&field.attributes);
            let mut constraint_strings = constraints.to_string();
            let mut relationships = RelationShips::new();
            relationships.populate(&field.attributes);
            let is_index = model_attributes.is_index(field.name());
            constraint_strings = format!(
                "{}\n{}",
                constraint_strings,
                model_attributes.constraint_strings(field.name())
            );
            table.add_row(row![
                field.name(),
                field_type,
                constraint_strings,
                relationships.fields(),
                relationships.references(),
                is_index,
            ]);
        });

        table.printstd();
        println!("");
    });
}
