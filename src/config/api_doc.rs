use actix_web::{HttpResponse, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::modules::{
    block::block_dtos::*, board::board_dtos::*, upload::upload_dtos::*, user::user_dtos::*,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "PixelBoard API",
        version = "0.1.0",
        description = "API para o PixelBoard.net — outdoor virtual limitado onde marcas, criadores e pequenos negócios compram blocos visuais em um grande mural online.",
        license(name = "MIT")
    ),
    paths(
        crate::modules::user::user_controllers::register_user,
        crate::modules::user::user_controllers::login_user,
        crate::modules::user::user_controllers::detail_user,
        crate::modules::block::block_controllers::purchase_block,
        crate::modules::block::block_controllers::detail_block,
        crate::modules::block::block_controllers::update_block,
        crate::modules::board::board_controllers::get_board,
        crate::modules::board::board_controllers::get_board_stats,
        crate::modules::upload::upload_controllers::upload_image,
    ),
    components(schemas(
        RegisterUserDTO,
        LoginUserDTO,
        UserResponseDTO,
        LoginResponseDTO,
        PurchaseBlockDTO,
        UpdateBlockDTO,
        BlockResponseDTO,
        BoardResponseDTO,
        BoardStatsDTO,
        UploadResponseDTO,
    )),
    tags(
        (name = "user", description = "Autenticação e gerenciamento de usuários"),
        (name = "block", description = "Compra e gerenciamento de blocos no mural"),
        (name = "board", description = "Visualização do mural e estatísticas"),
        (name = "upload", description = "Upload de imagens para o MinIO")
    )
)]
pub struct ApiDoc;

pub fn api_doc() -> impl FnOnce(&mut web::ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        cfg.route(
            "/docs",
            web::get().to(|| async {
                HttpResponse::PermanentRedirect()
                    .append_header(("Location", "/docs/"))
                    .finish()
            }),
        )
        .service(SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", ApiDoc::openapi()));
    }
}
