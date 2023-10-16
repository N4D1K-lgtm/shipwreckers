use bevy::prelude::*;

mod enemy;
mod player;
mod systems;
mod world;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use systems::*;
use world::WorldPlugin;

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
        app.add_plugins((PlayerPlugin, EnemyPlugin, WorldPlugin))
            .add_state::<SimulationState>()
            .add_systems(Update, toggle_simulaltion.run_if(in_state(AppState::Game)));
    }
}
