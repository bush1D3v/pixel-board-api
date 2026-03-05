use actix_web::HttpResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

pub struct AppError;

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> HttpResponse {
        HttpResponse::BadRequest().json(ErrorResponse::new("bad_request", message))
    }

    pub fn unauthorized(message: impl Into<String>) -> HttpResponse {
        HttpResponse::Unauthorized().json(ErrorResponse::new("unauthorized", message))
    }

    pub fn forbidden(message: impl Into<String>) -> HttpResponse {
        HttpResponse::Forbidden().json(ErrorResponse::new("forbidden", message))
    }

    pub fn not_found(message: impl Into<String>) -> HttpResponse {
        HttpResponse::NotFound().json(ErrorResponse::new("not_found", message))
    }

    pub fn conflict(message: impl Into<String>) -> HttpResponse {
        HttpResponse::Conflict().json(ErrorResponse::new("conflict", message))
    }

    pub fn unprocessable(message: impl Into<String>) -> HttpResponse {
        HttpResponse::UnprocessableEntity().json(ErrorResponse::new("unprocessable_entity", message))
    }

    pub fn internal(message: impl Into<String>) -> HttpResponse {
        HttpResponse::InternalServerError().json(ErrorResponse::new("internal_server_error", message))
    }

    pub fn service_unavailable(message: impl Into<String>) -> HttpResponse {
        HttpResponse::ServiceUnavailable().json(ErrorResponse::new("service_unavailable", message))
    }
}
