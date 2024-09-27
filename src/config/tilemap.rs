use bevy::math::UVec2;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilemapTileSize;

use crate::config::ConfigResource;
use crate::config::ReflectConfigResource;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TileConfig {
    pub tile_size: TilemapTileSize,
    pub chunk_size: UVec2,
    pub render_chunk_size: UVec2,
}

impl ConfigResource for TileConfig {}

pub struct TileConfigPlugin;

impl Plugin for TileConfigPlugin {
    fn name(&self) -> &str {
        "TileConfig"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<TileConfig>()
            .register_type::<TileConfig>()
            .register_type_data::<TileConfig, ReflectConfigResource>();
    }
}

impl Default for TileConfig {
    fn default() -> Self {
        Self {
            tile_size: TilemapTileSize { x: 64.0, y: 64.0 },
            chunk_size: UVec2 { x: 8, y: 8 },
            render_chunk_size: UVec2 { x: 16, y: 16 },
        }
    }
}
