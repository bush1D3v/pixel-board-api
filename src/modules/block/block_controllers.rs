use actix_web::{get, patch, post, web, HttpRequest, HttpResponse, Responder};
use validator::Validate;

use crate::{
    middlewares::auth_middleware::validate_token,
    modules::block::block_repositories::find_block_by_id,
    utils::validate_body::validate_body_error,
};

use super::block_dtos::BlockResponseDTO;

use super::{
    block_dtos::{PurchaseBlockDTO, UpdateBlockDTO},
    block_services::{purchase_block_service, update_block_service},
};

pub fn block_routes() -> actix_web::Scope {
    web::scope("/block")
        .service(purchase_block)
        .service(detail_block)
        .service(update_block)
}

#[utoipa::path(
    tag = "block",
    path = "/block",
    security(("bearer_auth" = [])),
    request_body = PurchaseBlockDTO,
    responses(
        (status = 201, description = "Bloco comprado com sucesso", body = BlockResponseDTO),
        (status = 400, description = "Erro de validação ou bloco fora dos limites"),
        (status = 401, description = "Token inválido ou ausente"),
        (status = 409, description = "Área já ocupada ou reservada"),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[post("")]
pub async fn purchase_block(
    body: web::Json<PurchaseBlockDTO>,
    pool: web::Data<deadpool_postgres::Pool>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = match validate_token(&req) {
        Ok(id) => id,
        Err(e) => return e,
    };

    if let Err(e) = body.validate() {
        return validate_body_error(&e);
    }

    match purchase_block_service(&pool, &user_id, body.into_inner()).await {
        Ok(block) => HttpResponse::Created().json(block),
        Err(e) => e,
    }
}

#[utoipa::path(
    tag = "block",
    path = "/block/{block_id}",
    responses(
        (status = 200, description = "Detalhamento do bloco", body = BlockResponseDTO),
        (status = 404, description = "Bloco não encontrado"),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[get("{block_id}")]
pub async fn detail_block(
    pool: web::Data<deadpool_postgres::Pool>,
    block_id: web::Path<String>,
) -> impl Responder {
    match find_block_by_id(&pool, &block_id).await {
        Ok(Some(block)) => HttpResponse::Ok().json(block.to_response()),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "code": "not_found",
            "message": "Bloco não encontrado."
        })),
        Err(e) => e,
    }
}

#[utoipa::path(
    tag = "block",
    path = "/block/{block_id}",
    security(("bearer_auth" = [])),
    request_body = UpdateBlockDTO,
    responses(
        (status = 200, description = "Bloco atualizado com sucesso", body = BlockResponseDTO),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Token inválido ou ausente"),
        (status = 403, description = "Sem permissão para editar este bloco"),
        (status = 404, description = "Bloco não encontrado"),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[patch("{block_id}")]
pub async fn update_block(
    body: web::Json<UpdateBlockDTO>,
    pool: web::Data<deadpool_postgres::Pool>,
    block_id: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let user_id = match validate_token(&req) {
        Ok(id) => id,
        Err(e) => return e,
    };

    if let Err(e) = body.validate() {
        return validate_body_error(&e);
    }

    match update_block_service(&pool, &block_id, &user_id, body.into_inner()).await {
        Ok(block) => HttpResponse::Ok().json(block),
        Err(e) => e,
    }
}
