use actix_web::cookie::Cookie;
use actix_web::web::Path;
use actix_web::{Error, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserAuthTokenExtractor {
    pub user_token: String,
}

pub async fn api_auth_handle(params: Path<UserAuthTokenExtractor>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .cookie(Cookie::new("auth", params.user_token.clone()))
        .body("HI"))
}
