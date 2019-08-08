use actix_web::{HttpServer, App, middleware, web, http, HttpResponse, Error};
use yew::services::fetch::StatusCode;
use actix::{Actor, Context};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::env;
use dotenv::dotenv;
use std::io::SeekFrom::Start;
use crate::models::NewProject;
use actix_web::web::Form;
use uuid::Uuid;
use yew::services::fetch::Credentials::Include;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;

mod models;
mod schema;
mod rest_api;

fn start_db_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Unable to connect to {}", database_url))
}

fn root_handler() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../static/main.html"))
    )
}

#[derive(Deserialize)]
struct CreateProject {
    project_name: String
}

fn project_create_handle(params: Form<CreateProject>) -> Result<HttpResponse, Error> {
    use schema::Projects;
    let data = params.into_inner();

    let db_connection = start_db_connection();

    let project_uuid = Uuid::new_v4().to_string();

    let new_project = NewProject {
        name: data.project_name.as_str(),
        projectUUID: project_uuid.as_str()
    };

    diesel::insert_into(Projects::table)
        .values(&new_project)
        .execute(&db_connection)
        .expect("Unable to create project");

    Ok(
        HttpResponse::PermanentRedirect()
            .header(http::header::LOCATION, format!("/project/{}", data.project_name))
            .finish()
    )
}

#[derive(Deserialize)]
struct ViewProjectExtractor {
    project_name: String
}

fn handle_view_project(params: Form<ViewProjectExtractor>) -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../static/project.html"))
    )
}

struct RemoteConfigServer {}

impl Actor for RemoteConfigServer {
    type Context = Context<Self>;
}

impl Default for RemoteConfigServer {
    fn default() -> Self {
        RemoteConfigServer {}
    }
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let system = actix::System::new("RemoteConfig");

    let server = RemoteConfigServer::default().start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .service(web::resource("/create-project")
                .name("create_project")
                .route(web::post().to(project_create_handle))
                .route(web::get().to(root_handler)),
            )
            .service(web::resource("/project/{project_name}").to(handle_view_project))
            .service(actix_files::Files::new("/", "./static/"))
            .service(web::resource("/api/config/{id}")
                .route(web::post().to(api_config_handle))
            )
            .wrap(middleware::Logger::default())
    })
        .bind("0.0.0.0:8080").unwrap()
        .start();

    system.run()
}
