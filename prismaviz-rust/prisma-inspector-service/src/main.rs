mod code_highlight;
mod visualise;

#[macro_use]
extern crate rocket;
// use dotenv::dotenv;
use rocket::config::Config;
use rocket::fs::{relative, NamedFile};

use rocket::http::Method;
use rocket::Request;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};

#[get("/")]
async fn index() -> Option<NamedFile> {
    let page_directory_path = relative!("../prisma-inspector-web/dist");
    NamedFile::open(Path::new(&page_directory_path).join("index.html"))
        .await
        .ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let page_directory_path = relative!("../prisma-inspector-web/dist");
    NamedFile::open(Path::new(&page_directory_path).join(file))
        .await
        .ok()
}

#[catch(400)]
fn bad_request(req: &Request) {
    format!("Something wrong with the request {}", req.uri());
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    // dotenv().ok();
    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Cors setup failed");
    let mut config = Config::release_default();
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    config.address = IpAddr::V4(ip);
    let rocket = rocket::custom(config)
        .mount(
            "/",
            routes![
                index,
                visualise::visualise,
                code_highlight::code_highlight,
                files
            ],
        )
        .register("/api/v1/visualise", catchers![bad_request])
        .attach(cors);

    Ok(rocket.into())
}
