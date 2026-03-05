use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use aws_sdk_s3::Client;
use futures_util::StreamExt;

use crate::middlewares::auth_middleware::validate_token;

use super::{upload_dtos::UploadResponseDTO, upload_services::upload_image_service};

pub fn upload_routes() -> actix_web::Scope {
    web::scope("/upload").service(upload_image)
}

#[utoipa::path(
    tag = "upload",
    path = "/upload",
    security(("bearer_auth" = [])),
    request_body(content_type = "multipart/form-data", content = String, description = "Campo 'file' com a imagem (PNG, JPEG, WebP ou GIF, máx 5 MB)"),
    responses(
        (status = 201, description = "Imagem enviada com sucesso", body = UploadResponseDTO),
        (status = 400, description = "Arquivo inválido, vazio ou tipo não permitido"),
        (status = 401, description = "Token inválido ou ausente"),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[post("")]
pub async fn upload_image(
    mut payload: Multipart,
    s3: web::Data<Client>,
    req: HttpRequest,
) -> impl Responder {
    // Require authentication
    if let Err(e) = validate_token(&req) {
        return e;
    }

    // Read the first field named "file"
    while let Some(Ok(mut field)) = payload.next().await {
        let disposition = field.content_disposition().cloned();
        let field_name = disposition
            .as_ref()
            .and_then(|d| d.get_name().map(|s| s.to_string()))
            .unwrap_or_default();

        if field_name != "file" {
            continue;
        }

        let content_type = field
            .content_type()
            .map(|m| m.to_string())
            .unwrap_or_default();

        let original_filename = disposition
            .as_ref()
            .and_then(|d| d.get_filename().map(|s| s.to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        // Collect all chunks
        let mut data = Vec::new();
        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(bytes) => data.extend_from_slice(&bytes),
                Err(e) => {
                    log::error!("Erro ao ler chunk do multipart: {e}");
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "code": "bad_request",
                        "message": "Erro ao processar o arquivo enviado."
                    }));
                }
            }
        }

        log::info!(
            "Upload recebido: filename={original_filename}, content_type={content_type}, size={}",
            data.len()
        );

        return match upload_image_service(&s3, &original_filename, &content_type, data).await {
            Ok(resp) => HttpResponse::Created().json(resp),
            Err(e) => e,
        };
    }

    HttpResponse::BadRequest().json(serde_json::json!({
        "code": "bad_request",
        "message": "Nenhum arquivo encontrado no campo 'file'."
    }))
}
