use bevy::log;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use resources::{NoiseResource, TilemapResource};
use systems::*;

pub struct WorldPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TilemapSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DebugSystemSet;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TilemapResource>()
            .init_resource::<NoiseResource>()
            .configure_set(PreStartup, TilemapSystemSet.before(DebugSystemSet))
            .configure_set(PostStartup, DebugSystemSet.after(TilemapSystemSet))
            .add_systems(
                Startup,
                (
                    create_tilemap_chunks.in_set(TilemapSystemSet),
                    print_tilemap_data.in_set(DebugSystemSet),
                ),
            );
        log::info!("Loaded World Plugin");
    }
}
