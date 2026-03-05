use actix_web::{http::header, HttpRequest, HttpResponse};

use crate::shared::{exceptions::error::AppError, treaties::jwt::Jwt};

pub fn extract_bearer_token(req: &HttpRequest) -> Result<String, HttpResponse> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("Token de autorização não fornecido."))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::unauthorized("Formato de token inválido."));
    }

    Ok(auth_header[7..].to_string())
}

pub fn validate_token(req: &HttpRequest) -> Result<String, HttpResponse> {
    let token = extract_bearer_token(req)?;
    let claims =
        Jwt::validate_access_token(&token).map_err(|_| AppError::unauthorized("Token inválido ou expirado."))?;
    Ok(claims.sub)
}

pub fn validate_user_ownership(req: &HttpRequest, user_id: &str) -> Result<(), HttpResponse> {
    let token_user_id = validate_token(req)?;
    if token_user_id != user_id {
        return Err(AppError::forbidden(
            "Você não tem permissão para acessar este recurso.",
        ));
    }
    Ok(())
}
