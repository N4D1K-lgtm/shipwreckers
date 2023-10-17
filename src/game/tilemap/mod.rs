use bevy::log;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;

use constants::*;
use resources::{ChunkManager, NoiseResource, TilemapResource};
use systems::*;

pub struct WorldPlugin;

// #[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
// pub enum WorldState {
//     #[default]
//     Generation,
// Render,
// Exit,
// }

use crate::AppState;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TilemapResource>()
            .init_resource::<NoiseResource>()
            // .add_state::<WorldState>()
            .add_systems(OnEnter(AppState::Setup), create_tilemap_chunks)
            .add_systems(OnExit(AppState::Setup), log_tilemap_info);
        log::info!("Loaded World Plugin");
    }
}

pub struct MyTilemapPlugin;

impl Plugin for MyTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
        .add_plugins(TilemapPlugin)
        .insert_resource(ChunkManager::default())
        .add_systems(
            Update,
            (
                spawn_chunks_around_camera,
                despawn_outofrange_chunks.after(spawn_chunks_around_camera),
            )
                .run_if(in_state(AppState::Game)),
        );
    }
}
