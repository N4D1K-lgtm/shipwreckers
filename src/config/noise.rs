use crate::prelude::{NoiseGeneratorType, WorleyWrapper};
use bevy::prelude::*;

use crate::config::ConfigResource;
use crate::config::ReflectConfigResource;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct NoiseConfig {
    pub scale: f64,
    pub threshold: f64,
    pub generator_type: NoiseGeneratorType,
}

impl ConfigResource for NoiseConfig {}

pub struct NoiseConfigPlugin;

impl Plugin for NoiseConfigPlugin {
    fn name(&self) -> &str {
        "NoiseConfig"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<NoiseConfig>()
            .register_type::<NoiseConfig>()
            .register_type_data::<NoiseConfig, ReflectConfigResource>();
    }
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            scale: 0.5,
            threshold: 0.3,
            generator_type: NoiseGeneratorType::Worley(WorleyWrapper {
                seed: 1,
                frequency: 0.5,
            }),
        }
    }
}
