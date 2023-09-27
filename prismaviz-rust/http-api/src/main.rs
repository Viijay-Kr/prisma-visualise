#[macro_use]
extern crate rocket;
use prismaviz::SchemaVisualiser;
use rocket::config::Config;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Method;
use rocket::serde::{json::Json, Serialize};
use rocket::Request;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::net::{IpAddr, Ipv4Addr};
use uuid::Uuid;
#[get("/")]
fn index() -> &'static str {
    "Schema Visualiser"
}

#[derive(Debug, FromForm)]
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

#[catch(400)]
fn bad_request(req: &Request) {
    format!("Something wrong with the request {}", req.uri());
}
#[post("/api/v1/visualise", data = "<input>")]
async fn visualise(input: Form<VisualiseInput<'_>>) -> Option<Json<VisualiseOutput>> {
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
            std::fs::remove_file::<_>(v);
            Some(Json(VisualiseOutput { result }))
        }
        Err(e) => {
            format!("failed to read temp path");
            e
        }
    }
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();
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
    let mut config = Config::release_default();
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    config.address = IpAddr::V4(ip);
    rocket::custom(config)
        .mount("/", routes![index, visualise])
        .register("/api/v1/visualise", catchers![bad_request])
        .attach(cors)
}
