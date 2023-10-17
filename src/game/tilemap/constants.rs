use bevy::math::UVec2;
use bevy_ecs_tilemap::prelude::*;

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 64.0, y: 64.0 };
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
