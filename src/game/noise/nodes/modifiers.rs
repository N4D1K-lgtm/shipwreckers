use super::{ConstantOrValue, NoiseError, NoiseMap, Validatable};
use bevy::log;
use serde::{Deserialize, Serialize};
use std::fmt;

// Some default constants for noise parameters.
pub const DEFAULT_OCTAVES: usize = 6;
pub const DEFAULT_FREQUENCY: f64 = 2.0;
pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
pub const DEFAULT_PERSISTENCE: f64 = 0.5;

// Represents the parameters for noise generation functions. (Fbm, Billow and MultiRidged)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NoiseParameters {
    pub octaves: ConstantOrValue<usize>,
    pub frequency: ConstantOrValue<f64>,
    pub lacunarity: ConstantOrValue<f64>,
    pub persistence: ConstantOrValue<f64>,
}

impl Default for NoiseParameters {
    fn default() -> Self {
        Self {
            octaves: ConstantOrValue::RawValue(DEFAULT_OCTAVES),
            frequency: ConstantOrValue::RawValue(DEFAULT_FREQUENCY),
            lacunarity: ConstantOrValue::RawValue(DEFAULT_LACUNARITY),
            persistence: ConstantOrValue::RawValue(DEFAULT_PERSISTENCE),
        }
    }
}

impl fmt::Display for NoiseParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Octaves: {}, Frequency: {}, Lacunarity: {}, Persistence: {}",
            self.octaves, self.frequency, self.lacunarity, self.persistence
        )
    }
}

// Modifiers
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Modifier {
    Fbm {
        input: String,
        #[serde(default)]
        #[serde(flatten)]
        parameters: NoiseParameters,
    },
    RidgedMulti {
        input: String,
        #[serde(default)]
        #[serde(flatten)]
        parameters: NoiseParameters,
    },
    Billow {
        input: String,
        #[serde(flatten)]
        #[serde(default)]
        parameters: NoiseParameters,
    },
    Abs {
        input: String,
    },
    Exp {
        input: String,
    },
    Neg {
        input: String,
    },
    Add {
        input: String,
        #[serde(default)]
        value: ConstantOrValue<f64>,
    },
    Mul {
        input: String,
        #[serde(default)]
        value: ConstantOrValue<f64>,
    },
    Clamp {
        input: String,
        #[serde(default)]
        min: ConstantOrValue<f64>,
        #[serde(default)]
        max: ConstantOrValue<f64>,
    },
    Pow {
        input: String,
        #[serde(default)]
        exponent: ConstantOrValue<f64>,
    },
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modifier::Fbm { input, parameters } => {
                write!(f, "Fbm(Input: {}, Parameters: {})", input, parameters)
            }
            Modifier::RidgedMulti { input, parameters } => {
                write!(
                    f,
                    "RidgedMulti(Input: {}, Parameters: {})",
                    input, parameters
                )
            }
            Modifier::Billow { input, parameters } => {
                write!(f, "Billow(Input: {}, Parameters: {})", input, parameters)
            }
            Modifier::Abs { input } => {
                write!(f, "Abs(Input: {})", input)
            }
            Modifier::Exp { input } => {
                write!(f, "Exp(Input: {})", input)
            }
            Modifier::Neg { input } => {
                write!(f, "Neg(Input: {})", input)
            }
            Modifier::Add { input, value } => {
                write!(f, "Add(Input: {}, Value: {})", input, value)
            }
            Modifier::Mul { input, value } => {
                write!(f, "Mul(Input: {}, Value: {})", input, value)
            }
            Modifier::Clamp { input, min, max } => {
                write!(f, "Clamp(Input: {}, Min: {}, Max: {})", input, min, max)
            }
            Modifier::Pow { input, exponent } => {
                write!(f, "Pow(Input: {}, Exponent: {})", input, exponent)
            }
        }
    }
}

impl Validatable for Modifier {
    fn validate(&self, map: &NoiseMap) -> Result<(), NoiseError> {
        // log::debug!("Validating: {}", self);
        println!("Validating: {}", self);
        match self {
            Modifier::Fbm { input, parameters }
            | Modifier::RidgedMulti { input, parameters }
            | Modifier::Billow { input, parameters } => {
                map.validate_node_name(input)?;
                parameters.octaves.validate(map)?;
                parameters.frequency.validate(map)?;
                parameters.lacunarity.validate(map)?;
                parameters.persistence.validate(map)?;
            }
            Modifier::Abs { input } | Modifier::Exp { input } | Modifier::Neg { input } => {
                map.validate_node_name(input)?;
            }
            Modifier::Add { input, value } | Modifier::Mul { input, value } => {
                map.validate_node_name(input)?;
                value.validate(map)?;
            }
            Modifier::Clamp { input, min, max } => {
                map.validate_node_name(input)?;
                min.validate(map)?;
                max.validate(map)?;
            }
            Modifier::Pow { input, exponent } => {
                map.validate_node_name(input)?;
                exponent.validate(map)?;
            }
        }
        Ok(())
    }
}
