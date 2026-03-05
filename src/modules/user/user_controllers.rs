use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use validator::Validate;

use crate::{
    middlewares::auth_middleware::validate_token,
    modules::user::user_repositories::find_user_by_id,
    utils::validate_body::validate_body_error,
};

use super::{
    user_dtos::{LoginResponseDTO, LoginUserDTO, RegisterUserDTO, UserResponseDTO},
    user_services::{login_user_service, register_user_service},
};

pub fn user_routes() -> actix_web::Scope {
    web::scope("/user")
        .service(register_user)
        .service(login_user)
        .service(detail_user)
}

#[utoipa::path(
    tag = "user",
    path = "/user",
    request_body = RegisterUserDTO,
    responses(
        (status = 201, description = "Usuário criado com sucesso", body = UserResponseDTO),
        (status = 400, description = "Erro de validação"),
        (status = 409, description = "E-mail já cadastrado"),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[post("")]
pub async fn register_user(
    body: web::Json<RegisterUserDTO>,
    pool: web::Data<deadpool_postgres::Pool>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return validate_body_error(&e);
    }

    match register_user_service(&pool, body.into_inner()).await {
        Ok(user) => HttpResponse::Created()
            .append_header(("Location", format!("/user/{}", user.id)))
            .json(user),
        Err(e) => e,
    }
}

#[utoipa::path(
    tag = "user",
    path = "/user/login",
    request_body = LoginUserDTO,
    responses(
        (status = 200, description = "Login realizado com sucesso", body = LoginResponseDTO),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Credenciais inválidas"),
        (status = 404, description = "Usuário não encontrado"),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[post("login")]
pub async fn login_user(
    body: web::Json<LoginUserDTO>,
    pool: web::Data<deadpool_postgres::Pool>,
) -> impl Responder {
    if let Err(e) = body.validate() {
        return validate_body_error(&e);
    }

    match login_user_service(&pool, body.into_inner()).await {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => e,
    }
}

#[utoipa::path(
    tag = "user",
    path = "/user/{user_id}",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Detalhamento do usuário", body = UserResponseDTO),
        (status = 401, description = "Token inválido ou ausente"),
        (status = 404, description = "Usuário não encontrado"),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[get("{user_id}")]
pub async fn detail_user(
    pool: web::Data<deadpool_postgres::Pool>,
    user_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    if let Err(e) = validate_token(&req) {
        return e;
    }

    match find_user_by_id(&pool, &user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(UserResponseDTO {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at,
        }),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "code": "not_found",
            "message": "Usuário não encontrado."
        })),
        Err(e) => e,
    }
}
