pub mod chunk;
pub mod noise;
pub mod tilemap;

use bevy::prelude::*;

use chunk::ChunkConfigPlugin;
use noise::NoiseConfigPlugin;
use tilemap::TileConfigPlugin;

#[reflect_trait]
pub trait ConfigResource {}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TileConfigPlugin, NoiseConfigPlugin, ChunkConfigPlugin));
    }
}
