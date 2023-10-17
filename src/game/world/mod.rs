use bevy::log;
use bevy::prelude::*;

mod components;
mod events;
mod resources;
mod systems;

use events::GenerationFinishedEvent;
use resources::{NoiseResource, TilemapResource};
use systems::*;

pub struct WorldPlugin;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum WorldState {
    #[default]
    Generation,
    Simulation,
    Destroy,
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TilemapResource>()
            .init_resource::<NoiseResource>()
            .add_state::<WorldState>()
            .add_systems(OnEnter(WorldState::Generation), create_tilemap_chunks)
            .add_systems(OnEnter(WorldState::Simulation), log_tilemap_info);
        log::info!("Loaded World Plugin");
    }
}
