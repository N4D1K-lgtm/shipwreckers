use super::{ConstantOrValue, NoiseError, NoiseMap, Validatable};
use bevy::log;
use serde::{Deserialize, Serialize};
use std::fmt;

// Generators
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Generator {
    Constant { value: ConstantOrValue<f64> },
    Checkerboard,
    ImprovedPerlin { seed: ConstantOrValue<u64> },
    Perlin { seed: ConstantOrValue<u64> },
    Value { seed: ConstantOrValue<u64> },
    Simplex { seed: ConstantOrValue<u64> },
    Worley { seed: ConstantOrValue<u64> },
}

impl fmt::Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Generator::Constant { value } => {
                write!(f, "Constant(Value: {})", value)
            }
            Generator::Checkerboard => {
                write!(f, "Checkerboard")
            }
            Generator::ImprovedPerlin { seed } => {
                write!(f, "ImprovedPerlin(Seed: {})", seed)
            }
            Generator::Perlin { seed } => {
                write!(f, "Perlin(Seed: {})", seed)
            }
            Generator::Value { seed } => {
                write!(f, "Value(Seed: {})", seed)
            }
            Generator::Simplex { seed } => {
                write!(f, "Simplex(Seed: {})", seed)
            }
            Generator::Worley { seed } => {
                write!(f, "Worley(Seed: {})", seed)
            }
        }
    }
}

impl Validatable for Generator {
    fn validate(&self, map: &NoiseMap) -> Result<(), NoiseError> {
        // log::debug!("Validating: {}", self);
        println!("Validating: {}", self);
        match self {
            Generator::Constant { value } => value.validate(map),
            Generator::Checkerboard => Ok(()), // No validation needed for Checkerboard
            Generator::ImprovedPerlin { seed } => seed.validate(map),
            Generator::Perlin { seed } => seed.validate(map),
            Generator::Value { seed } => seed.validate(map),
            Generator::Simplex { seed } => seed.validate(map),
            Generator::Worley { seed } => seed.validate(map),
        }
    }
}
