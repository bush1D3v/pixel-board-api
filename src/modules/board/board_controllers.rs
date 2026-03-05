use actix_web::{get, web, HttpResponse, Responder};

use crate::modules::block::{
    block_dtos::{BOARD_HEIGHT, BOARD_WIDTH},
    block_repositories::{count_blocks_and_pixels, find_all_active_blocks},
};

use super::board_dtos::{BoardResponseDTO, BoardStatsDTO};

pub fn board_routes() -> actix_web::Scope {
    web::scope("/board")
        .service(get_board)
        .service(get_board_stats)
}

#[utoipa::path(
    tag = "board",
    path = "/board",
    responses(
        (status = 200, description = "Mural completo com todos os blocos ativos", body = BoardResponseDTO),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[get("")]
pub async fn get_board(pool: web::Data<deadpool_postgres::Pool>) -> impl Responder {
    match find_all_active_blocks(&pool).await {
        Ok(blocks) => {
            let board = BoardResponseDTO {
                width: BOARD_WIDTH,
                height: BOARD_HEIGHT,
                blocks: blocks.into_iter().map(|b| b.to_response()).collect(),
            };
            HttpResponse::Ok().json(board)
        }
        Err(e) => e,
    }
}

#[utoipa::path(
    tag = "board",
    path = "/board/stats",
    responses(
        (status = 200, description = "Estatísticas do mural", body = BoardStatsDTO),
        (status = 500, description = "Erro interno do servidor")
    )
)]
#[get("stats")]
pub async fn get_board_stats(pool: web::Data<deadpool_postgres::Pool>) -> impl Responder {
    match count_blocks_and_pixels(&pool).await {
        Ok((block_count, pixel_count)) => {
            HttpResponse::Ok().json(BoardStatsDTO::new(block_count, pixel_count))
        }
        Err(e) => e,
    }
}
