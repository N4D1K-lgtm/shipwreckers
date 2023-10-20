use bevy::log;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin as BevyTilemapPlugin;

pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;

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
            .register_type::<TilemapResource>()
            .add_systems(OnEnter(AppState::Setup), create_tilemap_chunks)
            .add_systems(OnExit(AppState::Setup), log_tilemap_info);
        log::info!("Loaded World Plugin");
    }
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: UVec2 { x: 16, y: 16 },
            ..Default::default()
        })
        .add_plugins(BevyTilemapPlugin)
        .insert_resource(ChunkManager::default())
        .add_systems(
            Update,
            (spawn_chunks_around_camera, ping_if_resource_changed).run_if(in_state(AppState::Game)),
        );
    }
}

fn ping_if_resource_changed(tilemap: Res<TilemapResource>) {
    if tilemap.is_changed() {
        println!("Tilemap was changed");
    }
}
