use actix_web::{web::Data, HttpResponse};
use deadpool_postgres::Pool;

use crate::shared::exceptions::error::AppError;

use super::user_dtos::UserRow;

pub async fn insert_user_repository(
    pool: &Data<Pool>,
    id: &str,
    name: &str,
    email: &str,
    password_hash: &str,
) -> Result<UserRow, HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let row = client
        .query_one(
            "INSERT INTO users (id, name, email, password) VALUES ($1, $2, $3, $4)
             RETURNING id, name, email, password, created_at::TEXT, updated_at::TEXT",
            &[&id, &name, &email, &password_hash],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao inserir usuário: {e}")))?;

    Ok(UserRow {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
        password: row.get("password"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

pub async fn find_user_by_email(pool: &Data<Pool>, email: &str) -> Result<Option<UserRow>, HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let row = client
        .query_opt(
            "SELECT id, name, email, password, created_at::TEXT, updated_at::TEXT FROM users WHERE email = $1",
            &[&email],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao buscar usuário: {e}")))?;

    Ok(row.map(|r| UserRow {
        id: r.get("id"),
        name: r.get("name"),
        email: r.get("email"),
        password: r.get("password"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}

pub async fn find_user_by_id(pool: &Data<Pool>, user_id: &str) -> Result<Option<UserRow>, HttpResponse> {
    let client = pool.get().await.map_err(|e| {
        AppError::service_unavailable(format!("Erro ao conectar ao banco de dados: {e}"))
    })?;

    let row = client
        .query_opt(
            "SELECT id, name, email, password, created_at::TEXT, updated_at::TEXT FROM users WHERE id = $1",
            &[&user_id],
        )
        .await
        .map_err(|e| AppError::internal(format!("Erro ao buscar usuário: {e}")))?;

    Ok(row.map(|r| UserRow {
        id: r.get("id"),
        name: r.get("name"),
        email: r.get("email"),
        password: r.get("password"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }))
}
