use actix_web::HttpResponse;
use actix_web::web::Form;
use actix_web::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiConfigHandleRequestData {
    pub id: String
}

pub fn api_config_handle(params: Form<ApiConfigHandleRequestData>) -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok()
            .body("{\"MAPBOX_TOKEN\": \"no\"}")
    )
}