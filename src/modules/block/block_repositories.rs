use actix_web::{web::Data, HttpResponse};
use deadpool_postgres::Pool;

use crate::shared::exceptions::error::AppError;

use super::block_dtos::BlockRow;

pub async fn insert_block_repository(
    pool: &Data<Pool>,
    id: &str,
    user_id: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    image_url: &str,
    link: &str,
    title: &str,
    description: Option<&str>,
    price_cents: i64,
) -> Result<BlockRow, HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let row = client
        .query_one(
            "INSERT INTO blocks (id, user_id, x, y, width, height, image_url, link, title, description, status, price_cents)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'active', $11)
             RETURNING id, user_id, x, y, width, height, image_url, link, title, description, status, price_cents, created_at::TEXT, updated_at::TEXT",
            &[&id, &user_id, &x, &y, &width, &height, &image_url, &link, &title, &description, &price_cents],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao inserir bloco: {e}")))?;

    Ok(row_to_block(&row))
}

pub async fn check_area_available(
    pool: &Data<Pool>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<bool, HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let x2 = x + width;
    let y2 = y + height;

    // Check for overlapping blocks (active or reserved)
    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM blocks
             WHERE status IN ('active', 'reserved')
               AND x < $1 AND (x + width) > $2
               AND y < $3 AND (y + height) > $4",
            &[&x2, &x, &y2, &y],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao verificar área: {e}")))?;

    let count: i64 = row.get("cnt");
    Ok(count == 0)
}

pub async fn find_block_by_id(pool: &Data<Pool>, block_id: &str) -> Result<Option<BlockRow>, HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let row = client
        .query_opt(
            "SELECT id, user_id, x, y, width, height, image_url, link, title, description, status, price_cents, created_at::TEXT, updated_at::TEXT
             FROM blocks WHERE id = $1",
            &[&block_id],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao buscar bloco: {e}")))?;

    Ok(row.map(|r| row_to_block(&r)))
}

pub async fn update_block_content(
    pool: &Data<Pool>,
    block_id: &str,
    image_url: Option<&str>,
    link: Option<&str>,
    title: Option<&str>,
    description: Option<&str>,
) -> Result<BlockRow, HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let row = client
        .query_one(
            "UPDATE blocks SET
                image_url = COALESCE($2, image_url),
                link = COALESCE($3, link),
                title = COALESCE($4, title),
                description = COALESCE($5, description),
                updated_at = NOW()
             WHERE id = $1
             RETURNING id, user_id, x, y, width, height, image_url, link, title, description, status, price_cents, created_at::TEXT, updated_at::TEXT",
            &[&block_id, &image_url, &link, &title, &description],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao atualizar bloco: {e}")))?;

    Ok(row_to_block(&row))
}

pub async fn find_all_active_blocks(pool: &Data<Pool>) -> Result<Vec<BlockRow>, HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let rows = client
        .query(
            "SELECT id, user_id, x, y, width, height, image_url, link, title, description, status, price_cents, created_at::TEXT, updated_at::TEXT
             FROM blocks WHERE status = 'active' ORDER BY created_at ASC",
            &[],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao buscar blocos: {e}")))?;

    Ok(rows.iter().map(row_to_block).collect())
}

pub async fn count_blocks_and_pixels(pool: &Data<Pool>) -> Result<(i64, i64), HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let row = client
        .query_one(
            "SELECT COUNT(*) as block_count, COALESCE(SUM(width * height), 0) as pixel_count
             FROM blocks WHERE status = 'active'",
            &[],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao contar blocos: {e}")))?;

    Ok((row.get("block_count"), row.get("pixel_count")))
}

fn row_to_block(row: &tokio_postgres::Row) -> BlockRow {
    BlockRow {
        id: row.get("id"),
        user_id: row.get("user_id"),
        x: row.get("x"),
        y: row.get("y"),
        width: row.get("width"),
        height: row.get("height"),
        image_url: row.get("image_url"),
        link: row.get("link"),
        title: row.get("title"),
        description: row.get("description"),
        status: row.get("status"),
        price_cents: row.get("price_cents"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}
