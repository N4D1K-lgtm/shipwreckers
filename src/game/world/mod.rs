use bevy::prelude::*;

mod resources;
mod systems;

use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, generate_voronoi)
            .add_systems(Startup, generate_islands)
            .add_systems(PostUpdate, print_islands);
    }
}
