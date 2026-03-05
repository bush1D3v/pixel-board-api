use actix_cors::Cors;
use actix_web::http;
use std::env;

pub fn cors() -> Cors {
    let allowed_origin = env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "*".to_string());

    if allowed_origin == "*" {
        Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
    } else {
        Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
    }
}
