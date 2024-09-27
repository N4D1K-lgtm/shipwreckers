use crate::prelude::*;
use bevy::prelude::*;

pub mod entities;
pub mod noise;
pub mod systems;
pub mod tilemap;

use noise::NoisePlugin;
use systems::*;
use tilemap::TilemapPlugin;

use super::AppState;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, EnemyPlugin, TilemapPlugin, NoisePlugin))
            .add_state::<SimulationState>()
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}
