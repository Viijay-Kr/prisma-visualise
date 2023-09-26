mod attributes;
mod constraints;
mod relations;

pub use crate::{attributes::ModelAttributes, constraints::Contraints, relations::RelationShips};
use prettytable::row;
use prettytable::Table;
use psl_core::schema_ast::ast::FieldType;
use psl_core::schema_ast::ast::WithIdentifier;
use psl_core::{
    diagnostics::Diagnostics,
    schema_ast::{self, ast::SchemaAst},
};

pub struct PrismaVizModelField {
    pub attributes: ModelAttributes,
    pub relation_ships: RelationShips,
    pub constraints: Contraints,
    pub name: String,
    pub data_type: String,
    pub is_index: String,
}

impl PrismaVizModelField {
    fn new(name: String, data_type: String, is_index: &str) -> PrismaVizModelField {
        PrismaVizModelField {
            attributes: ModelAttributes { values: vec![] },
            relation_ships: RelationShips { relations: vec![] },
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
}

impl PrismaVizModel {
    pub fn new(name: String) -> PrismaVizModel {
        PrismaVizModel {
            name,
            fields: vec![],
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
                )
            });
        model_fields.for_each(|(model, fields, attributes)| {
            let mut prisma_viz_model = PrismaVizModel::new(model);

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
                let mut relationships = RelationShips::new();
                relationships.populate(&field.attributes);
                let is_index = model_attributes.is_index(field.name());

                let mut prisma_vis_model_field =
                    PrismaVizModelField::new(field.name().to_string(), field_type, is_index);
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
                table.add_row(row![
                    field.name,
                    field.data_type,
                    constraint_strings,
                    field.relation_ships.fields(),
                    field.relation_ships.references(),
                    field.is_index,
                ]);
            });

            table.printstd();
            println!("");
        })
    }
}
