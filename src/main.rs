use crate::models::{NewLayer, NewProject, Project};
use actix_web::web::{Form, Path, Data};
use actix_web::{
    http, middleware, web, App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer,
};
use askama::Template;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{dangerous_unsafe_decode, Algorithm, Validation};
use serde::Deserialize;
use uuid::Uuid;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;

mod api_user_auth;
mod models;
mod property_type;
mod rest_api;
mod schema;
mod database;

use crate::api_user_auth::api_auth_handle;
use crate::rest_api::{api_config_handle, get_layer_properties, get_project_layers};

use actix_web::cookie::Cookie;
use crate::database::start_db_connection;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    userId: u32,
}

fn get_request_claims(req: HttpRequest) -> Option<Claims> {
    req.cookie("UK_APP_AUTH").map(|c| get_cookie_claims(c))
}

fn get_cookie_claims(auth: Cookie) -> Claims {
    let token = auth.value();
    println!("msg: {}", &token);

    let _validation = Validation::new(Algorithm::RS256);

    let _key = include_bytes!("../public.der");

    let token_data = match dangerous_unsafe_decode::<Claims>(&token) {
        //}, key.as_ref(), &validation) {
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
    props: Vec<(&'a str, &'a str, i32)>,
}

#[derive(Template)]
#[template(path = "Index_ProjectsList.html")]
struct ProjectListTemplate<'a> {
    project_details: Vec<(&'a str, Vec<(String, String)>)>,
}

async fn root_handler(
    db: Data<MysqlConnection>,
    req: HttpRequest
) -> Result<HttpResponse, Error> {
    use self::schema::Projects::dsl::*;

    if let Some(claims) = get_request_claims(req) {

        let user_projects: Vec<Project> = Projects
            .filter(owner.eq(claims.userId))
            .load(db.get_ref())
            .expect("DATA");

        println!("Found {} projects", &user_projects.len());

        let mut details = Vec::<(&str, Vec<(String, String)>)>::new();

        for p in &user_projects {
            let layers = get_project_layers(db.get_ref(), p.projectUUID.as_str());
            let default = &layers[0];

            let properties = get_layer_properties(db.get_ref(), default.1);

            let mut props = Vec::<(String, String)>::new();

            for prop in &properties {
                props.push((prop.0.clone(), prop.1.clone().unwrap_or("NULL".to_string())))
            }

            details.push((p.name.as_str(), props));
        }

        let project = ProjectListTemplate {
            project_details: details,
        };

        let content = project.render().unwrap();

        Ok(HttpResponse::Ok().body(content))
    } else {
        Ok(HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../templates/test_login.html")))
    }
}

#[derive(Deserialize)]
struct CreateProject {
    project_name: String,
}

async fn project_create_handle(
    db: Data<MysqlConnection>,
    params: Form<CreateProject>
) -> Result<HttpResponse, Error> {
    use schema::Layers;
    use schema::Projects;

    let data = params.into_inner();

    let project_uuid = Uuid::new_v4().to_string();

    // Add the project into the  db
    let new_project = NewProject {
        name: data.project_name.as_str(),
        projectUUID: project_uuid.as_str(),
        owner: &1,
    };

    diesel::insert_into(Projects::table)
        .values(&new_project)
        .execute(db.get_ref())
        .expect("Unable to create project");

    use self::schema::Projects::dsl::*;

    // Get the project back so we can get the id
    let x: Vec<Project> = Projects
        .filter(projectUUID.eq(project_uuid))
        .limit(1)
        .load(db.get_ref())
        .expect("Unable to retrieve new project");

    // Add the default layer into the db
    let default_layer = NewLayer {
        name: "Default",
        _condition: "",
        projectID: x.first().unwrap().id,
    };

    diesel::insert_into(Layers::table)
        .values(&default_layer)
        .execute(db.get_ref())
        .expect("Unable to add default layer");

    Ok(HttpResponse::PermanentRedirect()
        .header(
            http::header::LOCATION,
            format!("/project/{}", data.project_name),
        )
        .finish())
}

#[derive(Deserialize)]
struct ViewProjectExtractor {
    project_name: String,
}

async fn handle_view_project(
    db: Data<MysqlConnection>,
    params: Path<ViewProjectExtractor>
) -> Result<HttpResponse, Error> {

    let layers = get_project_layers(&db, params.project_name.as_str());

    let default_layer = layers.get(0).expect("No layers available");
    let default_layer_id = default_layer.1;

    let properties = get_layer_properties(&db, default_layer_id);

    let mut props: Vec<(&str, &str, i32)> = Vec::new();

    for (name, value, type_) in &properties {
        let value_str = match value {
            Some(x) => x.as_str(),
            None => "<undefined>",
        };

        props.push((name.as_str(), value_str, *type_))
    }

    let project = ProjectTemplate {
        project_name: "test123123",
        props,
    };

    let content = project.render().expect("Render failed");

    Ok(HttpResponse::Ok().body(content))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();

    HttpServer::new( || {
        App::new()
            .data(start_db_connection())
            .service(web::resource("/").to(root_handler))
            .service(
                web::resource("/create-project")
                    .name("create_project")
                    .route(web::post().to(project_create_handle))
                    .route(web::get().to(root_handler)),
            )
            .service(web::resource("/project/{project_name}").route(
                web::get().to(handle_view_project)
            ))
            .service(web::resource("/api/config/{project_id}/{device_id}").route(
                web::get().to(api_config_handle)
            ))
            .service(web::resource("/api/user/auth/{user_token}").route(
                web::get().to(api_auth_handle)
            ))
            .service(actix_files::Files::new("/", "./static/"))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
    })
    .bind("0.0.0.0:8080").expect("Address not available")
    .run()
    .await
}
