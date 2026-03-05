use actix_web::{web::Data, HttpResponse};
use deadpool_postgres::Pool;

use crate::shared::{exceptions::error::AppError, treaties::jwt::Jwt};

use super::{
    user_dtos::{LoginResponseDTO, LoginUserDTO, RegisterUserDTO, UserResponseDTO},
    user_repositories::{find_user_by_email, insert_user_repository},
};

pub async fn register_user_service(
    pool: &Data<Pool>,
    body: RegisterUserDTO,
) -> Result<UserResponseDTO, HttpResponse> {
    // Check if email already exists
    if let Some(_) = find_user_by_email(pool, &body.email).await? {
        return Err(AppError::conflict("Este e-mail já está sendo utilizado."));
    }

    let user_id = uuid::Uuid::new_v4().to_string();
    let password_hash =
        bcrypt::hash(&body.password, bcrypt::DEFAULT_COST).map_err(|e| AppError::internal(format!("Erro ao gerar hash da senha: {e}")))?;

    let user = insert_user_repository(pool, &user_id, &body.name, &body.email, &password_hash).await?;

    Ok(UserResponseDTO {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
    })
}

pub async fn login_user_service(
    pool: &Data<Pool>,
    body: LoginUserDTO,
) -> Result<LoginResponseDTO, HttpResponse> {
    let user = find_user_by_email(pool, &body.email)
        .await?
        .ok_or_else(|| AppError::not_found("Usuário não encontrado com este e-mail."))?;

    let password_valid = bcrypt::verify(&body.password, &user.password)
        .map_err(|e| AppError::internal(format!("Erro ao verificar senha: {e}")))?;

    if !password_valid {
        return Err(AppError::unauthorized("E-mail e/ou senha incorretos."));
    }

    let (access_token, access_expires_in) =
        Jwt::access_token(&user.id).map_err(|e| AppError::internal(e))?;
    let (refresh_token, refresh_expires_in) =
        Jwt::refresh_token(&user.id).map_err(|e| AppError::internal(e))?;

    Ok(LoginResponseDTO {
        access_token,
        access_expires_in,
        refresh_token,
        refresh_expires_in,
    })
}
