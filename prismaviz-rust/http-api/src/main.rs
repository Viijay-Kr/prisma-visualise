#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use prismaviz::SchemaVisualiser;
use rocket::config::Config;
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::http::Method;
use rocket::serde::{json::Json, Serialize};
use rocket::Request;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};
use uuid::Uuid;
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
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct VisualiseOutput {
    result: Vec<Model>,
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    let static_path = std::env::var("ROCKET_ASSETS_DIR").unwrap_or("default_value".to_string());
    println!("static_path {}", static_path);
    let page_directory_path = format!("{}", static_path);
    println!("page_directory_path {}", page_directory_path);
    NamedFile::open(Path::new(&page_directory_path).join("index.html"))
        .await
        .ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let static_path = std::env::var("ROCKET_ASSETS_DIR").unwrap_or("default_value".to_string());
    let page_directory_path = format!("{}", static_path);
    NamedFile::open(Path::new(&page_directory_path).join(file))
        .await
        .ok()
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
                    id: Uuid::new_v4().hyphenated().to_string(),
                    name: m.name.clone(),
                    code: m.code.clone(),
                    fields: m
                        .fields
                        .iter()
                        .map(|f| Field {
                            r#type: f.data_type.to_string(),
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
    dotenv().ok();
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
        .mount("/", routes![index, visualise, files])
        .register("/api/v1/visualise", catchers![bad_request])
        .attach(cors)
}
