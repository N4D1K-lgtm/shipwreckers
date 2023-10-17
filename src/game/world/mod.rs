use bevy::log;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use resources::{NoiseResource, TilemapResource};
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
