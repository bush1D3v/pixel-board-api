use aws_sdk_s3::{primitives::ByteStream, Client};
use std::env;

use crate::shared::exceptions::error::AppError;

use super::upload_dtos::{UploadResponseDTO, ALLOWED_CONTENT_TYPES, MAX_FILE_SIZE};

use actix_web::HttpResponse;

pub async fn upload_image_service(
    s3: &Client,
    _original_filename: &str,
    content_type: &str,
    data: Vec<u8>,
) -> Result<UploadResponseDTO, HttpResponse> {
    // Validate content type
    if !ALLOWED_CONTENT_TYPES.contains(&content_type) {
        return Err(AppError::bad_request(format!(
            "Tipo de arquivo não permitido. Aceitos: {}",
            ALLOWED_CONTENT_TYPES.join(", ")
        )));
    }

    // Validate file size
    if data.len() > MAX_FILE_SIZE {
        return Err(AppError::bad_request(format!(
            "Arquivo muito grande. Máximo: {} MB.",
            MAX_FILE_SIZE / 1024 / 1024
        )));
    }

    if data.is_empty() {
        return Err(AppError::bad_request("O arquivo enviado está vazio."));
    }

    let bucket = env::var("MINIO_BUCKET").unwrap_or_else(|_| "pixel-board".to_string());
    let public_url = env::var("MINIO_PUBLIC_URL")
        .unwrap_or_else(|_| env::var("MINIO_ENDPOINT").expect("MINIO_ENDPOINT must be set"));

    // Generate unique key: uploads/{uuid}.{ext}
    let extension = mime_guess::get_mime_extensions_str(content_type)
        .and_then(|exts| exts.first().copied())
        .unwrap_or("bin");
    let key = format!("uploads/{}.{}", uuid::Uuid::new_v4(), extension);

    // Upload to MinIO
    s3.put_object()
        .bucket(&bucket)
        .key(&key)
        .body(ByteStream::from(data.clone()))
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| {
            log::error!("Falha ao enviar imagem para MinIO: {e}");
            AppError::internal("Erro ao fazer upload da imagem.")
        })?;

    // Build the public URL
    let url = format!("{}/{}/{}", public_url, bucket, key);

    Ok(UploadResponseDTO {
        url,
        filename: key,
        content_type: content_type.to_string(),
        size: data.len(),
    })
}
