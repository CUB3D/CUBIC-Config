use actix_web::{HttpServer, App, middleware, web, http, HttpResponse, Error, HttpMessage, HttpRequest};
use actix::{Actor};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use std::env;
use dotenv::dotenv;
use actix_web::web::{Path, Form};
use uuid::Uuid;
use serde::Deserialize;
use askama::Template;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Validation, Algorithm, dangerous_unsafe_decode};
use crate::models::{NewProject, NewLayer, Project};

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

mod models;
mod schema;
mod rest_api;
mod property_type;
mod api_user_auth;

use crate::rest_api::{api_config_handle, get_project_layers, get_layer_properties};
use crate::api_user_auth::api_auth_handle;
use actix_web::middleware::BodyEncoding;
use actix_web::error::UrlencodedError::ContentType;
use actix_web::dev::Service;
use futures::Future;
use futures::future::ok;
use actix_web::cookie::Cookie;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    userId: u32,
}

fn get_request_claims(req: HttpRequest) -> Option<Claims> {
    req.cookie("UK_APP_AUTH").map(| c | get_cookie_claims(c))
}

fn get_cookie_claims(auth: Cookie) -> Claims {
    let token = auth.value();
    println!("msg: {}", &token);

    let validation = Validation::new(Algorithm::RS256);

    let key = include_bytes!("../public.der");

    let token_data = match dangerous_unsafe_decode::<Claims>(&token) { //}, key.as_ref(), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors: {}", err),
        },
    };
    println!("{:?}", &token_data.claims);
    println!("{:?}", token_data.header);

    return token_data.claims;
}

#[derive(Template)]
#[template(path = "project.html")]
struct ProjectTemplate<'a> {
    project_name: &'a str,
    props: Vec<(&'a str, &'a str, i32)>
}

#[derive(Template)]
#[template(path = "Index_ProjectsList.html")]
struct ProjectListTemplate<'a> {
    project_details: Vec<(&'a str, Vec<(String, String)>)>
}

fn start_db_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Unable to connect to {}", database_url))
}

fn root_handler(
    req: HttpRequest
) -> impl Future<Item = HttpResponse, Error = Error> {
    use schema::Projects;
    use self::schema::Projects::dsl::*;

    if let Some(claims) = get_request_claims(req) {
        let db_connection = start_db_connection();

        let user_projects: Vec<Project> = Projects.filter(owner.eq(claims.userId))
            .load(&db_connection)
            .expect("DATA");

        println!("Found {} projects", &user_projects.len());

        let mut details = Vec::<(&str, Vec<(String, String)>)>::new();

        for p in &user_projects {
            let layers = get_project_layers(&db_connection, p.projectUUID.as_str());
            let default = &layers[0];

            let properties = get_layer_properties(&db_connection, default.1);

            let mut props = Vec::<(String, String)>::new();

            for prop in &properties {
                props.push((prop.0.clone(), prop.1.clone().unwrap_or("NULL".to_string())))
            }

            details.push((p.name.as_str(), props));
        }

        let project = ProjectListTemplate {
            project_details: details
        };

        let content = project.render().unwrap();

        ok(
            HttpResponse::Ok()
                .body(content)
        )
    } else {
        ok(
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(include_str!("../templates/test_login.html"))
        )
    }
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
        projectUUID: project_uuid.as_str(),
        owner: &1
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

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let system = actix::System::new("RemoteConfig");

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to_async(root_handler))
            .service(web::resource("/create-project")
                .name("create_project")
                .route(web::post().to(project_create_handle))
                .route(web::get().to_async(root_handler)),
            )
            .service(web::resource("/project/{project_name}").to(handle_view_project))
            .service(web::resource("/api/config/{project_id}/{device_id}").to(api_config_handle))
            .service(web::resource("/api/user/auth/{user_token}").to(api_auth_handle))
            .service(actix_files::Files::new("/", "./static/"))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
    })
        .bind("0.0.0.0:8080").unwrap()
        .start();

    system.run()
}
