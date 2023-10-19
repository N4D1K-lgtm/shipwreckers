use crate::prelude::{NoiseGeneratorType, WorleyWrapper};
use bevy::log::info;
use bevy::math::UVec2;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::prelude::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileConfig::default())
            .insert_resource(NoiseConfig::default())
            .insert_resource(ChunkConfig::default())
            .register_type::<TileConfig>()
            .register_type::<NoiseConfig>()
            .register_type::<ChunkConfig>()
            .add_systems(Update, resource_change_detection);
    }
}

fn resource_change_detection(
    noise: Res<NoiseConfig>,
    tile: Res<TileConfig>,
    chunk: Res<ChunkConfig>,
) {
    if noise.is_changed() {
        info!("NoiseConfig changed!");
    }

    if tile.is_changed() {
        info!("TileConfig changed!");
    }

    if chunk.is_changed() {
        info!("ChunkConfig changed!");
    }
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct TileConfig {
    pub tile_size: TilemapTileSize,
    pub chunk_size: UVec2,
    pub render_chunk_size: UVec2,
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

#[derive(Resource, Reflect)]
pub struct NoiseConfig {
    pub scale: f64,
    pub threshold: f64,
    pub generator_type: NoiseGeneratorType,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            scale: 0.5,
            threshold: 0.3,
            generator_type: NoiseGeneratorType::Worley(WorleyWrapper {
                seed: 1,
                frequency: 0.5,
            }),
        }
    }
}

#[derive(Resource, Reflect)]
pub struct ChunkConfig {
    pub despawn_distance: f32,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            despawn_distance: 320.0,
        }
    }
}
