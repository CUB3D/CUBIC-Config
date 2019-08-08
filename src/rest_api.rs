use actix_web::HttpResponse;
use actix_web::web::{Form, Path};
use actix_web::Error;
use serde::Deserialize;
use json::object::Object;

#[derive(Deserialize)]
pub struct ApiConfigHandleRequestData {
    pub id: String
}

pub fn api_config_handle(
    params: Path<ApiConfigHandleRequestData>
) -> Result<HttpResponse, Error> {

    let mut o = json::JsonValue::new_object();
    o["MAPBOX_TOKEN"] = "no".into();


    let r = HttpResponse::Ok()
        .content_type("application/json")
        .body(o.dump());

    Ok(
        r
    )
}