use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ─── Request DTOs ───

#[derive(Deserialize, Validate, ToSchema)]
pub struct RegisterUserDTO {
    #[validate(length(min = 2, max = 63, message = "O nome deve ter entre 2 e 63 caracteres."))]
    pub name: String,

    #[validate(email(message = "O e-mail deve ser um endereço válido."))]
    #[validate(length(
        min = 5,
        max = 127,
        message = "O e-mail deve ter entre 5 e 127 caracteres."
    ))]
    pub email: String,

    #[validate(length(
        min = 8,
        max = 255,
        message = "A senha deve ter entre 8 e 255 caracteres."
    ))]
    pub password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginUserDTO {
    #[validate(email(message = "O e-mail deve ser um endereço válido."))]
    pub email: String,

    #[validate(length(min = 8, message = "A senha deve ter pelo menos 8 caracteres."))]
    pub password: String,
}

// ─── Response DTOs ───

#[derive(Serialize, ToSchema)]
pub struct UserResponseDTO {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponseDTO {
    pub access_token: String,
    pub access_expires_in: i64,
    pub refresh_token: String,
    pub refresh_expires_in: i64,
}

// ─── Internal ───

#[derive(Clone, Serialize, Deserialize)]
pub struct UserRow {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}
