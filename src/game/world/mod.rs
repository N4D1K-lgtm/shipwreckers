use bevy::log;
use bevy::prelude::*;

mod components;
mod resources;
mod systems;
mod types;

use resources::TilemapResource;
use systems::*;

pub struct WorldPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TilemapSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DebugSystemSet;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Startup, TilemapSystemSet.before(DebugSystemSet))
            .init_resource::<TilemapResource>()
            .add_systems(
                Startup,
                (
                    populate_tilemap.in_set(TilemapSystemSet),
                    print_tilemap_stats.in_set(DebugSystemSet),
                ),
            );
        log::info!("Loaded World Plugin");
    }
}
