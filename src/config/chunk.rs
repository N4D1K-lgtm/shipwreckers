use bevy::prelude::*;

use crate::config::ConfigResource;

use crate::config::ReflectConfigResource;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ChunkConfig {
    pub despawn_distance: f32,
}

impl ConfigResource for ChunkConfig {}

pub struct ChunkConfigPlugin;

impl Plugin for ChunkConfigPlugin {
    fn name(&self) -> &str {
        "ChunkConfig"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ChunkConfig>()
            .register_type::<ChunkConfig>()
            .register_type_data::<ChunkConfig, ReflectConfigResource>();
    }
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            despawn_distance: 320.0,
        }
    }
}
