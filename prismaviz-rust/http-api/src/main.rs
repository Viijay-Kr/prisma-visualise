#[macro_use]
extern crate rocket;
use prismaviz::SchemaVisualiser;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Method;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Schema Visualiser"
}

#[derive(Debug, FromForm)]
#[warn(renamed_and_removed_lints)]
struct VisualiseInput<'r> {
    schema: TempFile<'r>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Field {
    pub name: String,
    pub r#type: String,
    pub is_index: String,
    pub relation_ship_fields: String,
    pub relation_ship_references: String,
    pub constraints: String,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Model {
    pub name: String,
    pub fields: Vec<Field>,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct VisualiseOutput {
    result: Vec<Model>,
}

#[post("/api/v1/visualise", data = "<input>")]
async fn visualise(input: Form<VisualiseInput<'_>>) -> Option<Json<VisualiseOutput>> {
    let temp_path = std::env::temp_dir().join("temp.prisma");
    input
        .into_inner()
        .schema
        .copy_to(&temp_path)
        .await
        .expect("Failed to read file contents");
    let contents = std::fs::read_to_string(&temp_path).unwrap();
    let mut visualiser = SchemaVisualiser::new(contents);
    visualiser.parse();
    let models = visualiser.get_models();
    let result = models
        .iter()
        .map(|m| Model {
            name: m.name.clone(),
            fields: m
                .fields
                .iter()
                .map(|f| Field {
                    r#type: f.data_type.clone(),
                    name: f.name.clone(),
                    constraints: f.constraints.to_string(),
                    relation_ship_fields: f.relation_ships.fields(),
                    relation_ship_references: f.relation_ships.references(),
                    is_index: f.is_index.clone(),
                })
                .collect::<Vec<Field>>(),
        })
        .collect::<Vec<Model>>();

    Some(Json(VisualiseOutput { result }))
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:5173"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Cors setup failed");

    rocket::build()
        .mount("/", routes![index, visualise])
        .attach(cors)
}
