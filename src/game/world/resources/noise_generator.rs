use bevy::prelude::Resource;
use noise::{NoiseFn, Perlin, Worley};

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
            scale: 1.0,
            threshold: 0.5,
            generator_type: NoiseGeneratorType::Perlin { seed: 0 },
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
