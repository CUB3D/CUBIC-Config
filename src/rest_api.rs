use actix_web::web::Path;
use actix_web::Error;
use actix_web::HttpResponse;
use serde::Deserialize;

use crate::start_db_connection;
use diesel::prelude::*;

use crate::property_type::into_property_type;
use crate::property_type::PropertyType::{Int, Str};

use json::JsonValue;

#[derive(Deserialize)]
pub struct ApiConfigHandleRequestData {
    pub project_id: String,
    pub device_id: String,
}

pub fn get_project_layers(con: &MysqlConnection, project_name: &str) -> Vec<(String, i32)> {
    use crate::schema::Layers;
    use crate::schema::Projects;

    let layers = Layers::table
        .inner_join(Projects::table)
        .select((Layers::name, Layers::id))
        .filter(Projects::projectUUID.eq(project_name))
        .load(con);

    layers.unwrap_or_else(|_| panic!("Unable to fetch layers for {}", project_name))
}

pub fn get_layer_properties(
    con: &MysqlConnection,
    layer_id: i32,
) -> Vec<(String, Option<String>, i32)> {
    use crate::schema::Layers;
    use crate::schema::Property;

    let layers = Property::table
        .inner_join(Layers::table)
        .select((Property::name, Property::value, Property::type_))
        .filter(Layers::id.eq(layer_id))
        .load(con);

    layers.unwrap_or_else(|_| panic!("Unable to fetch properties for layer {}", layer_id))
}

pub async fn api_config_handle(
    params: Path<ApiConfigHandleRequestData>,
) -> Result<HttpResponse, Error> {
    //    use crate::schema::{Projects, Layers, Property};
    //    use crate::models::Property as Prop;

    let db_connection = start_db_connection();

    let layers = get_project_layers(&db_connection, params.project_id.as_str());

    println!("Found {} layers", layers.len());

    let tmp: Vec<i32> = layers
        .iter()
        .filter(|layer| layer.0 == *"Default")
        .map(|layer| layer.1)
        .collect();
    let default_layer_id = tmp.first().expect("Unable to find default layer");

    println!("Default layer id: {}", default_layer_id);

    let properties = get_layer_properties(&db_connection, *default_layer_id);

    let mut o = json::JsonValue::new_object();

    for (name, value, type_) in properties {
        let property_type = into_property_type(type_);

        if let Some(val) = value {
            match property_type {
                Str => {
                    o[name] = val.into();
                }
                Int => {
                    o[name] = val
                        .parse::<i32>()
                        .unwrap_or_else(|_| panic!("Unable to convert {} into a int", val))
                        .into();
                }
                _ => {
                    o[name] = JsonValue::Null;
                }
            }
        } else {
            o[name] = JsonValue::Null;
        }
    }

    let r = HttpResponse::Ok()
        .content_type("application/json")
        .body(o.dump());

    Ok(r)
}
