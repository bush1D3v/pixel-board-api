use actix_web::{web::Data, HttpResponse};
use deadpool_postgres::Pool;

use crate::shared::exceptions::error::AppError;

use super::{
    block_dtos::{
        BlockResponseDTO, PurchaseBlockDTO, UpdateBlockDTO, BOARD_HEIGHT, BOARD_WIDTH,
        MIN_BLOCK_SIZE,
    },
    block_repositories::{
        check_area_available, find_block_by_id, insert_block_repository, update_block_content,
    },
};

/// Price per pixel in cents (R$ 0,20 = 20 centavos)
const PRICE_PER_PIXEL_CENTS: i64 = 20;

pub fn calculate_price(width: i32, height: i32) -> i64 {
    (width as i64) * (height as i64) * PRICE_PER_PIXEL_CENTS
}

pub async fn purchase_block_service(
    pool: &Data<Pool>,
    user_id: &str,
    body: PurchaseBlockDTO,
) -> Result<BlockResponseDTO, HttpResponse> {
    // Validate minimum block size
    if body.width < MIN_BLOCK_SIZE || body.height < MIN_BLOCK_SIZE {
        return Err(AppError::bad_request(format!(
            "O bloco mínimo é {MIN_BLOCK_SIZE}x{MIN_BLOCK_SIZE} pixels."
        )));
    }

    // Validate block is within board bounds
    if body.x + body.width > BOARD_WIDTH || body.y + body.height > BOARD_HEIGHT {
        return Err(AppError::bad_request(
            "O bloco ultrapassa os limites do mural.",
        ));
    }

    if body.x < 0 || body.y < 0 {
        return Err(AppError::bad_request(
            "As coordenadas devem ser positivas.",
        ));
    }

    // Check if area is fully available
    let available = check_area_available(pool, body.x, body.y, body.width, body.height).await?;
    if !available {
        return Err(AppError::conflict(
            "A área selecionada já está ocupada ou reservada.",
        ));
    }

    let block_id = uuid::Uuid::new_v4().to_string();
    let price_cents = calculate_price(body.width, body.height);

    let block = insert_block_repository(
        pool,
        &block_id,
        user_id,
        body.x,
        body.y,
        body.width,
        body.height,
        &body.image_url,
        &body.link,
        &body.title,
        body.description.as_deref(),
        price_cents,
    )
    .await?;

    Ok(block.to_response())
}

pub async fn update_block_service(
    pool: &Data<Pool>,
    block_id: &str,
    user_id: &str,
    body: UpdateBlockDTO,
) -> Result<BlockResponseDTO, HttpResponse> {
    let block = find_block_by_id(pool, block_id)
        .await?
        .ok_or_else(|| AppError::not_found("Bloco não encontrado."))?;

    if block.user_id != user_id {
        return Err(AppError::forbidden(
            "Você não tem permissão para editar este bloco.",
        ));
    }

    if block.status != "active" {
        return Err(AppError::bad_request(
            "Somente blocos ativos podem ser editados.",
        ));
    }

    let updated = update_block_content(
        pool,
        block_id,
        body.image_url.as_deref(),
        body.link.as_deref(),
        body.title.as_deref(),
        body.description.as_deref(),
    )
    .await?;

    Ok(updated.to_response())
}
