use serde::{Deserialize, Serialize};

pub const DEFAULT_OCTAVES: usize = 6;
pub const DEFAULT_FREQUENCY: f64 = 2.0;
pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
pub const DEFAULT_PERSISTENCE: f64 = 0.5;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum NoiseType {
    Perlin(#[serde(default)] Seed),
    OpenSimplex(#[serde(default)] Seed),
    SuperSimplex(#[serde(default)] Seed),
    Value(#[serde(default)] Seed),
    Worley {
        #[serde(default)]
        return_type: ReturnType,
        frequency: f64,
    },
    Fbm {
        octaves: usize,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    },
    Billow {
        octaves: usize,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    },
    HybridMulti {
        octaves: usize,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Seed {
    pub seed: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FbmFields {
    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

pub fn default_octaves() -> usize {
    DEFAULT_OCTAVES
}

pub fn default_frequency() -> f64 {
    DEFAULT_FREQUENCY
}

pub fn default_lacunarity() -> f64 {
    DEFAULT_LACUNARITY
}

pub fn default_persistence() -> f64 {
    DEFAULT_PERSISTENCE
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Default)]
pub enum ReturnType {
    #[default]
    Distance,
    Value,
}

impl Default for NoiseType {
    fn default() -> Self {
        NoiseType::Perlin(Seed::default())
    }
}
