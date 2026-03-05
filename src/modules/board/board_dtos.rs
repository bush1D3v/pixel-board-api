use serde::Serialize;
use utoipa::ToSchema;

use crate::modules::block::block_dtos::{BlockResponseDTO, BOARD_HEIGHT, BOARD_WIDTH};

#[derive(Serialize, ToSchema)]
pub struct BoardResponseDTO {
    pub width: i32,
    pub height: i32,
    pub blocks: Vec<BlockResponseDTO>,
}

#[derive(Serialize, ToSchema)]
pub struct BoardStatsDTO {
    pub total_pixels: i64,
    pub sold_pixels: i64,
    pub available_pixels: i64,
    pub sold_blocks: i64,
    pub fill_percentage: f64,
}

impl BoardStatsDTO {
    pub fn new(sold_blocks: i64, sold_pixels: i64) -> Self {
        let total_pixels = (BOARD_WIDTH as i64) * (BOARD_HEIGHT as i64);
        let available_pixels = total_pixels - sold_pixels;
        let fill_percentage = if total_pixels > 0 {
            (sold_pixels as f64 / total_pixels as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total_pixels,
            sold_pixels,
            available_pixels,
            sold_blocks,
            fill_percentage,
        }
    }
}
