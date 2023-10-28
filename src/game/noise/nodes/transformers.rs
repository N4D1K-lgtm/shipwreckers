use super::{ConstantOrValue, NoiseError, NoiseMap, Validatable};
use bevy::log;
use serde::{Deserialize, Serialize};
use std::fmt;

// Transformers
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Transformer {
    Rotate {
        input: String,
        #[serde(default)]
        rotation: ConstantOrValue<Vec<f64>>,
    },
    Scale {
        input: String,
        #[serde(default)]
        scale: ConstantOrValue<Vec<f64>>,
    },
    Translate {
        input: String,
        #[serde(default)]
        translation: ConstantOrValue<Vec<f64>>,
    },
}

impl fmt::Display for Transformer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Transformer::Rotate { input, rotation } => {
                write!(f, "Rotate(Input: {}, Rotation: {:?})", input, rotation)
            }
            Transformer::Scale { input, scale } => {
                write!(f, "Scale(Input: '{}', Scale: {:?})", input, scale)
            }
            Transformer::Translate { input, translation } => {
                write!(
                    f,
                    "Translate(Input: {}, Translation: {:?})",
                    input, translation
                )
            }
        }
    }
}

impl Validatable for Transformer {
    fn validate(&self, map: &NoiseMap) -> Result<(), NoiseError> {
        // log::debug!("Validating: {}", self);
        println!("Validating: {}", self);
        match self {
            Transformer::Rotate { input, rotation } => {
                map.validate_node_name(input)?;
                rotation.validate(map)?;
            }
            Transformer::Scale { input, scale } => {
                map.validate_node_name(input)?;
                scale.validate(map)?;
            }
            Transformer::Translate { input, translation } => {
                map.validate_node_name(input)?;
                translation.validate(map)?;
            }
        }
        Ok(())
    }
}
