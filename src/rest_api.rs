use actix_web::HttpResponse;
use actix_web::web::Form;

#[derive(Deserialise)]
struct ApiConfigHandleRequestData {
    pub id: String
}

fn api_config_handle(params: Form<ApiConfigHandleRequestData>) -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok()
            .body('{"MAPBOX_TOKEN": "pk.eyJ1IjoiY3ViM2R1ayIsImEiOiJjanIzbnNndTUwd28wM3hxbXg0aWcxbnNmIn0.zXMx7BMPn18XxW46kYUvLQ"}')
    )
}