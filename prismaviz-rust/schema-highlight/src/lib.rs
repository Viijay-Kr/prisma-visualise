use psl_core::{
    diagnostics::Diagnostics,
    schema_ast::{self, ast::SchemaAst}, parser_database::walkers,
};

pub fn hightlight(schema: &str) {
    let mut diagnostics = Diagnostics::default();
    let result: SchemaAst = schema_ast::parse_schema(schema, &mut diagnostics);
    let tops = result.tops.into_iter();
    walkers
}
