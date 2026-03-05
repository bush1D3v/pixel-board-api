use serde::Serialize;
use utoipa::ToSchema;

/// Tipos de imagem aceitos
pub const ALLOWED_CONTENT_TYPES: &[&str] = &["image/png", "image/jpeg", "image/webp", "image/gif"];

/// Tamanho máximo de upload: 5 MB
pub const MAX_FILE_SIZE: usize = 5 * 1024 * 1024;

// ─── Response DTOs ───

#[derive(Serialize, ToSchema)]
pub struct UploadResponseDTO {
    /// URL pública da imagem enviada
    pub url: String,
    /// Nome do arquivo gerado
    pub filename: String,
    /// Content-Type da imagem
    pub content_type: String,
    /// Tamanho em bytes
    pub size: usize,
}
