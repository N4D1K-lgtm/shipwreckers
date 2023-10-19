use bevy::prelude::Resource;
use bevy::reflect::Reflect;
use bevy_inspector_egui::prelude::*;

use noise::{NoiseFn, Perlin, Worley};
// Wrapper for Perlin
// #[derive(Reflect)]
#[derive(Clone, Reflect)]
pub struct PerlinWrapper {
    pub seed: u32,
}

// Wrapper for Worley
// #[derive(Reflect)]
#[derive(Clone, Reflect)]
pub struct WorleyWrapper {
    pub seed: u32,
    pub frequency: f64,
}

// Use the wrappers in your enum
#[allow(dead_code)]
#[derive(Reflect, Clone)]
pub enum NoiseGeneratorType {
    Perlin(PerlinWrapper),
    Worley(WorleyWrapper),
}

impl From<PerlinWrapper> for Perlin {
    fn from(wrapper: PerlinWrapper) -> Self {
        // Convert from PerlinWrapper to Perlin
        Perlin::new(wrapper.seed)
    }
}

impl From<WorleyWrapper> for Worley {
    fn from(wrapper: WorleyWrapper) -> Self {
        Worley::new(wrapper.seed).set_frequency(wrapper.frequency)
    }
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct NoiseResource {
    #[inspector(min = 0.0, max = 1.0)]
    pub scale: f64,
    pub threshold: f64,
    pub generator_type: NoiseGeneratorType,
}

impl Default for NoiseResource {
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

impl NoiseResource {
    pub fn get_noise_value(&self, x: f64, y: f64) -> f64 {
        match &self.generator_type {
            NoiseGeneratorType::Perlin(perlin_wrapper) => {
                let perlin: Perlin = perlin_wrapper.clone().into();
                perlin.get([x * self.scale, y * self.scale])
            }
            NoiseGeneratorType::Worley(worley_wrapper) => {
                let worley: Worley = worley_wrapper.clone().into();
                worley.get([x * self.scale, y * self.scale])
            }
        }
    }
}
