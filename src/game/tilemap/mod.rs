use bevy::{prelude::*, utils::HashSet};

#[derive(Component)]
pub struct ChunkMarker;

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

use bevy::log;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin as BevyTilemapPlugin;

use crate::AppState;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: UVec2 { x: 16, y: 16 },
            ..Default::default()
        })
        .add_plugins(BevyTilemapPlugin);
    }
}
