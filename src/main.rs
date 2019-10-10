use actix_web::{HttpServer, App, middleware, web, http, HttpResponse, Error};
use actix::{Actor, Context};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use std::env;
use dotenv::dotenv;
use crate::models::{NewProject, NewLayer, Project};
use actix_web::web::{Path, Form};
use uuid::Uuid;
use serde::Deserialize;
use askama::Template;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

mod models;
mod schema;
mod rest_api;
mod property_type;
mod api_user_auth;

use crate::rest_api::{api_config_handle, get_project_layers, get_layer_properties};
use crate::api_user_auth::api_auth_handle;
use actix_web::middleware::BodyEncoding;
use actix_web::error::UrlencodedError::ContentType;

#[derive(Template)]
#[template(path = "project.html")]
struct ProjectTemplate<'a> {
    project_name: &'a str,
    props: Vec<(&'a str, &'a str, i32)>
}

fn start_db_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Unable to connect to {}", database_url))
}

fn root_handler() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../static/index.html"))
    )
}

#[derive(Deserialize)]
struct CreateProject {
    project_name: String
}

fn project_create_handle(params: Form<CreateProject>) -> Result<HttpResponse, Error> {
    use schema::Projects;
    use schema::Layers;

    let data = params.into_inner();

    let db_connection = start_db_connection();

    let project_uuid = Uuid::new_v4().to_string();

    // Add the project into the  db
    let new_project = NewProject {
        name: data.project_name.as_str(),
        projectUUID: project_uuid.as_str()
    };

    diesel::insert_into(Projects::table)
        .values(&new_project)
        .execute(&db_connection)
        .expect("Unable to create project");

    use self::schema::Projects::dsl::*;

    // Get the project back so we can get the id
    let x: Vec<Project> = Projects.filter(projectUUID.eq(project_uuid))
        .limit(1)
        .load(&db_connection)
        .expect("Unable to retrieve new project");

    // Add the default layer into the db
    let default_layer = NewLayer {
        name: "Default",
        _condition: "",
        projectID: x.first().unwrap().id
    };

    diesel::insert_into(Layers::table)
        .values(&default_layer)
        .execute(&db_connection)
        .expect("Unable to add default layer");

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

fn handle_view_project(params: Path<ViewProjectExtractor>) -> Result<HttpResponse, Error> {

    let db = start_db_connection();

    let layers = get_project_layers(&db, params.project_name.as_str());

    let default_layer = layers.get(0).unwrap();
    let default_layer_id = default_layer.1;

    let properties = get_layer_properties(&db, default_layer_id);

    let mut props: Vec<(&str, &str, i32)> = Vec::new();

    for (name, value, type_) in &properties {

        let value_str = match value {
            Some(x) => x.as_str(),
            None => "<undefined>"
        };

        props.push((name.as_str(), value_str, *type_))
    }

    let project = ProjectTemplate {
        project_name: "test123123",
        props
    };

    let content = project.render().unwrap();

    Ok(
        HttpResponse::Ok()
            .body(content)
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
            .service(web::resource("/").to(root_handler))
            .service(web::resource("/create-project")
                .name("create_project")
                .route(web::post().to(project_create_handle))
                .route(web::get().to(root_handler)),
            )
            .service(web::resource("/project/{project_name}").to(handle_view_project))
            .service(web::resource("/api/config/{project_id}/{device_id}").to(api_config_handle))
            .service(web::resource("/api/user/auth/{user_token}").to(api_auth_handle))
            .service(actix_files::Files::new("/", "./static/"))
            .wrap(middleware::Logger::default())
    })
        .bind("0.0.0.0:8080").unwrap()
        .start();

    system.run()
}
