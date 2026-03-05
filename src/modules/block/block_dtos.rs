use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ─── Constants ───

pub const BOARD_WIDTH: i32 = 1000;
pub const BOARD_HEIGHT: i32 = 1000;
pub const MIN_BLOCK_SIZE: i32 = 10;
pub const RESERVATION_TTL_SECONDS: u64 = 600; // 10 minutes

// ─── Request DTOs ───

#[derive(Deserialize, Validate, ToSchema)]
pub struct PurchaseBlockDTO {
    /// Coordenada X do canto superior esquerdo do bloco
    #[validate(range(min = 0, message = "A coordenada X deve ser positiva."))]
    pub x: i32,

    /// Coordenada Y do canto superior esquerdo do bloco
    #[validate(range(min = 0, message = "A coordenada Y deve ser positiva."))]
    pub y: i32,

    /// Largura do bloco em pixels (mínimo 10)
    #[validate(range(min = 10, message = "A largura mínima é de 10 pixels."))]
    pub width: i32,

    /// Altura do bloco em pixels (mínimo 10)
    #[validate(range(min = 10, message = "A altura mínima é de 10 pixels."))]
    pub height: i32,

    /// URL da imagem/logo do bloco
    #[validate(url(message = "A URL da imagem deve ser válida."))]
    #[validate(length(max = 512, message = "A URL da imagem deve ter no máximo 512 caracteres."))]
    pub image_url: String,

    /// Link de destino ao clicar no bloco
    #[validate(url(message = "O link deve ser uma URL válida."))]
    #[validate(length(max = 512, message = "O link deve ter no máximo 512 caracteres."))]
    pub link: String,

    /// Título curto do bloco
    #[validate(length(
        min = 1,
        max = 63,
        message = "O título deve ter entre 1 e 63 caracteres."
    ))]
    pub title: String,

    /// Descrição curta (opcional)
    #[validate(length(max = 255, message = "A descrição deve ter no máximo 255 caracteres."))]
    pub description: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateBlockDTO {
    #[validate(url(message = "A URL da imagem deve ser válida."))]
    #[validate(length(max = 512, message = "A URL da imagem deve ter no máximo 512 caracteres."))]
    pub image_url: Option<String>,

    #[validate(url(message = "O link deve ser uma URL válida."))]
    #[validate(length(max = 512, message = "O link deve ter no máximo 512 caracteres."))]
    pub link: Option<String>,

    #[validate(length(
        min = 1,
        max = 63,
        message = "O título deve ter entre 1 e 63 caracteres."
    ))]
    pub title: Option<String>,

    #[validate(length(max = 255, message = "A descrição deve ter no máximo 255 caracteres."))]
    pub description: Option<String>,
}

// ─── Response DTOs ───

#[derive(Serialize, ToSchema)]
pub struct BlockResponseDTO {
    pub id: String,
    pub user_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub image_url: String,
    pub link: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub pixel_count: i32,
    pub price_cents: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
}

// ─── Internal Row ───

#[derive(Clone)]
pub struct BlockRow {
    pub id: String,
    pub user_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub image_url: String,
    pub link: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub price_cents: i64,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl BlockRow {
    pub fn pixel_count(&self) -> i32 {
        self.width * self.height
    }

    pub fn to_response(self) -> BlockResponseDTO {
        let pixel_count = self.pixel_count();
        BlockResponseDTO {
            id: self.id,
            user_id: self.user_id,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            image_url: self.image_url,
            link: self.link,
            title: self.title,
            description: self.description,
            status: self.status,
            pixel_count,
            price_cents: self.price_cents,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
