use prismaviz::SchemaVisualiser;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::serde::{json::Json, Deserialize, Serialize};

use uuid::Uuid;

use crate::code_highlight;
#[derive(Debug, FromForm)]
pub(crate) struct VisualiseInput<'r> {
    schema: TempFile<'r>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Field {
    pub name: String,
    pub r#type: String,
    pub is_index: String,
    pub relation_ship_fields: Vec<String>,
    pub relation_ship_references: Vec<String>,
    pub constraints: Vec<String>,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Model {
    pub id: String,
    pub name: String,
    pub fields: Vec<Field>,
    pub code: String,
    pub span: code_highlight::WeakSpan,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct VisualiseOutput {
    result: Vec<Model>,
    schema: String,
}

#[post("/api/v1/visualise", data = "<input>")]
pub async fn visualise(input: Form<VisualiseInput<'_>>) -> Option<Json<VisualiseOutput>> {
    let uuid = Uuid::new_v4();
    let temp_file_path = Ok(std::env::temp_dir().join(uuid.hyphenated().to_string() + ".prisma"));
    match temp_file_path {
        Ok(v) => {
            input
                .into_inner()
                .schema
                .copy_to(&v)
                .await
                .expect("Failed to read file contents");
            let contents = std::fs::read_to_string(&v).unwrap();
            let mut visualiser = SchemaVisualiser::new(contents.clone());
            visualiser.parse();
            let models = visualiser.get_models();
            let result = models
                .iter()
                .map(|m| Model {
                    id: Uuid::new_v4().hyphenated().to_string(),
                    name: m.name.clone(),
                    code: m.code.clone(),
                    span: code_highlight::WeakSpan {
                        start: m.span.start,
                        end: m.span.end,
                    },
                    fields: m
                        .fields
                        .iter()
                        .map(|f| Field {
                            r#type: f.r#type.resolve_with_modifier(),
                            name: f.name.clone(),
                            constraints: f.constraints.as_vec(),
                            relation_ship_fields: f.relation_ships.fields(),
                            relation_ship_references: f.relation_ships.references(),
                            is_index: f.is_index.clone(),
                        })
                        .collect::<Vec<Field>>(),
                })
                .collect::<Vec<Model>>();
            let _ = std::fs::remove_file::<_>(v);
            Some(Json(VisualiseOutput {
                result,
                schema: contents,
            }))
        }
        Err(e) => {
            format!("failed to read temp path");
            e
        }
    }
}
