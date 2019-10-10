use actix_web::{HttpResponse, Error};
use actix_web::web::Path;
use serde::Deserialize;
use actix_web::cookie::Cookie;

#[derive(Deserialize)]
pub struct UserAuthTokenExtractor {
    pub user_token: String
}

pub fn api_auth_handle(params: Path<UserAuthTokenExtractor>) -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .cookie(Cookie::new("auth", params.user_token.clone()))
            .body("HI")
    )
}
