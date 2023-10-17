use bevy::prelude::Resource;
use noise::{NoiseFn, Perlin, Worley};

#[allow(dead_code)]
pub enum NoiseGeneratorType {
    Perlin { seed: u32 },
    Worley { seed: u32, frequency: f64 },
}

#[derive(Resource)]
pub struct NoiseResource {
    pub scale: f64,
    pub threshold: f64,
    pub generator_type: NoiseGeneratorType,
}

impl Default for NoiseResource {
    fn default() -> Self {
        Self {
            scale: 0.5,     // Lower value for smoother, larger features
            threshold: 0.3, // Higher value for more water
            generator_type: NoiseGeneratorType::Worley {
                seed: 1,
                frequency: 0.5, // Lower value for smoother islands
            },
        }
    }
}

impl NoiseResource {
    pub fn get_noise_value(&self, x: f64, y: f64) -> f64 {
        match &self.generator_type {
            NoiseGeneratorType::Perlin { seed } => {
                let perlin = Perlin::new(*seed);
                perlin.get([x * self.scale, y * self.scale])
            }
            NoiseGeneratorType::Worley { seed, frequency } => {
                let worley = Worley::new(*seed).set_frequency(*frequency);
                worley.get([x * self.scale, y * self.scale])
            }
        }
    }
}
