use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use systems::*;

pub const NUM_ENEMIES: usize = 4;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
    }
}
