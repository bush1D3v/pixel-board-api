use actix_web::HttpResponse;
use std::collections::HashMap;
use validator::ValidationErrors;

use crate::shared::exceptions::error::ErrorResponse;

pub fn validate_body_error(errors: &ValidationErrors) -> HttpResponse {
    let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();

    for (field, errs) in errors.field_errors() {
        let messages: Vec<String> = errs
            .iter()
            .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
            .collect();
        field_errors.insert(field.to_string(), messages);
    }

    if field_errors.is_empty() {
        HttpResponse::BadRequest().json(ErrorResponse::new(
            "validation_error",
            "Erro de validação nos campos enviados.",
        ))
    } else {
        HttpResponse::BadRequest().json(field_errors)
    }
}
