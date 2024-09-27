use bevy::prelude::Resource;
use noise::{NoiseFn, Perlin, Worley};

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

pub struct NoiseResource;

#[derive(Resource)]
impl NoiseResource {
    // Generalized function for 2D noise
    pub fn noise_2d<N: NoiseFn<[f64; 2]>>(noise_fn: &N, x: f64, y: f64) -> f64 {
        noise_fn.get([x, y])
    }

    // Generalized function for 3D noise
    pub fn noise_3d<N: NoiseFn<[f64; 3]>>(noise_fn: &N, x: f64, y: f64, z: f64) -> f64 {
        noise_fn.get([x, y, z])
    }
}
