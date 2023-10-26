/// This module provides structures and utilities for defining and manipulating noise maps.
use anyhow::Error;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use std::fmt;

// Some default constants for noise parameters.
pub const DEFAULT_OCTAVES: usize = 6;
pub const DEFAULT_FREQUENCY: f64 = 2.0;
pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
pub const DEFAULT_PERSISTENCE: f64 = 0.5;

/// Enumeration of possible constant types that can be defined in a noise map.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConstantTypes {
    Float(f64),
    Int(i32),
    UnsignedInt(u64),
    Usize(usize),
}

/// Error types specific to the noise-related operations.
#[derive(Debug)]
pub enum NoiseError {
    InvalidConversion(String),
    NodeNotFound,
    CircularReference,
}

impl fmt::Display for NoiseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoiseError::InvalidConversion(detail) => write!(f, "Invalid conversion: {}", detail),
            _ => write!(f, "NoiseError"),
        }
    }
}

#[allow(dead_code)]
impl ConstantTypes {
    /// Convert the current `ConstantTypes` variant to `f64`.
    pub fn as_f64(&self) -> Result<f64, Error> {
        match self {
            ConstantTypes::Float(f) => Ok(*f),
            ConstantTypes::Int(i) => Ok(*i as f64),
            ConstantTypes::UnsignedInt(u) => Ok(*u as f64),
            ConstantTypes::Usize(u) => Ok(*u as f64),
        }
    }

    /// Convert the current `ConstantTypes` variant to `usize`.
    pub fn as_usize(&self) -> Result<usize, Error> {
        match self {
            ConstantTypes::Float(f) => {
                if *f < 0.0 || (*f as usize as f64 - *f).abs() > f64::EPSILON {
                    Err(Error::msg("Invalid conversion from float to usize"))
                } else {
                    Ok(*f as usize)
                }
            }
            ConstantTypes::Int(i) => {
                if *i < 0 {
                    Err(Error::msg("Invalid conversion from negative i32 to usize"))
                } else {
                    Ok(*i as usize)
                }
            }
            ConstantTypes::UnsignedInt(u) => {
                if *u > usize::MAX as u64 {
                    Err(Error::msg("u64 value too large to convert to usize"))
                } else {
                    Ok(*u as usize)
                }
            }
            ConstantTypes::Usize(u) => Ok(*u),
        }
    }

    /// Convert the current `ConstantTypes` variant to `i32`.
    pub fn as_i32(&self) -> Result<i32, Error> {
        match self {
            ConstantTypes::Float(f) => {
                if *f < i32::MIN as f64 || *f > i32::MAX as f64 {
                    Err(Error::msg("Invalid conversion from float to i32"))
                } else {
                    Ok(*f as i32)
                }
            }
            ConstantTypes::Int(i) => Ok(*i),
            ConstantTypes::UnsignedInt(u) => {
                if *u > i32::MAX as u64 {
                    Err(Error::msg("u64 value too large to convert to i32"))
                } else {
                    Ok(*u as i32)
                }
            }
            ConstantTypes::Usize(u) => {
                if *u > i32::MAX as usize {
                    Err(Error::msg("usize value too large to convert to i32"))
                } else {
                    Ok(*u as i32)
                }
            }
        }
    }

    /// Convert the current `ConstantTypes` variant to `u64`.
    pub fn as_u64(&self) -> Result<u64, Error> {
        match self {
            ConstantTypes::Float(f) => {
                if *f < 0.0 {
                    Err(Error::msg("Invalid conversion from negative float to u64"))
                } else {
                    Ok(*f as u64)
                }
            }
            ConstantTypes::Int(i) => {
                if *i < 0 {
                    Err(Error::msg("Invalid conversion from negative i32 to u64"))
                } else {
                    Ok(*i as u64)
                }
            }
            ConstantTypes::UnsignedInt(u) => Ok(*u),
            ConstantTypes::Usize(u) => Ok(*u as u64),
        }
    }
}

/// An enumeration representing either a named constant or a raw value.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ConstantOrValue<T: Default> {
    #[serde(rename = "constant")]
    Constant(String),
    #[serde(rename = "value")]
    RawValue(T),
}

impl<T: Default> Default for ConstantOrValue<T> {
    fn default() -> Self {
        ConstantOrValue::RawValue(Default::default())
    }
}

/// Represents the parameters for noise generation functions. (Fbm, Billow and MultiRidged)
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NoiseParameters {
    octaves: ConstantOrValue<usize>,
    frequency: ConstantOrValue<f64>,
    lacunarity: ConstantOrValue<f64>,
    persistence: ConstantOrValue<f64>,
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

// Generators
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Generator {
    Constant(ConstantOrValue<f64>),
    Checkerboard,
    ImprovedPerlin { seed: ConstantOrValue<u64> },
    Perlin { seed: ConstantOrValue<u64> },
    Value { seed: ConstantOrValue<u64> },
    Simplex { seed: ConstantOrValue<u64> },
    Worley { seed: ConstantOrValue<u64> },
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

// Combiners
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Combiner {
    Blend {
        base: String,
        other: String,
        control: String,
    },
    Max {
        base: String,
        other: String,
    },
    Min {
        base: String,
        other: String,
    },
    Power {
        base: String,
        exponent: String,
    },
    Product {
        base: String,
        other: String,
    },
    Sum {
        base: String,
        other: String,
    },
    Select {
        base: String,
        other: String,
        control: String,
        #[serde(default)]
        lower_bound: ConstantOrValue<f64>,
        #[serde(default)]
        upper_bound: ConstantOrValue<f64>,
    },
}

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

// Base Node enum
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Node {
    Generator(Generator),
    Modifier(Modifier),
    Combiner(Combiner),
    Transformer(Transformer),
}

// Node templates are defined as reusable subgraphs of nodes that can be instantiated in the node graph.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NodeTemplate {
    pub name: String,
    pub inputs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    pub nodes: HashMap<String, Node>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum NodeInstance {
    Direct(Node),
    FromTemplate {
        template: String,
        inputs: HashMap<String, String>, // Map from template input names to actual node names
    },
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NoiseMap {
    name: String,
    constants: HashMap<String, ConstantTypes>,
    node_templates: HashMap<String, NodeTemplate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    output: Option<String>,
    graph: HashMap<String, NodeInstance>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noise_map_serialization() {
        // Constants for the terrain
        let mut constants = HashMap::new();
        constants.insert("MAIN_OCTAVES".to_string(), ConstantTypes::Usize(5));
        constants.insert("MAIN_FREQUENCY".to_string(), ConstantTypes::Float(2.5));
        constants.insert(
            "TERRAIN_SEED".to_string(),
            ConstantTypes::UnsignedInt(1234567890),
        );
        constants.insert("SCALE_FACTOR".to_string(), ConstantTypes::Float(3.0));

        // Node templates
        let mut node_templates = HashMap::new();

        let mountain_nodes = [(
            "mountain_power".to_string(),
            Node::Combiner(Combiner::Power {
                base: "base_noise".to_string(),
                exponent: "mountain_exponent".to_string(),
            }),
        )]
        .iter()
        .cloned()
        .collect();

        let mountain_template = NodeTemplate {
            name: "MountainBase".to_string(),
            inputs: vec![
                "base_noise".to_string(),
                "mountain_exponent".to_string(), // Added the missing input
            ],
            output: Some("mountain_power".to_string()), // Adjusted the output name
            nodes: mountain_nodes,
        };

        let forest_nodes = [(
            "forest_ridged".to_string(),
            Node::Modifier(Modifier::RidgedMulti {
                input: "base_noise".to_string(),
                parameters: NoiseParameters {
                    octaves: ConstantOrValue::RawValue(6),
                    frequency: ConstantOrValue::RawValue(2.0),
                    ..Default::default()
                },
            }),
        )]
        .iter()
        .cloned()
        .collect();

        let forest_template = NodeTemplate {
            name: "ForestLayer".to_string(),
            inputs: vec!["base_noise".to_string()],
            output: Some("forest_ridged".to_string()), // Adjusted the output name
            nodes: forest_nodes,
        };
        node_templates.insert("MountainBase".to_string(), mountain_template);
        node_templates.insert("ForestLayer".to_string(), forest_template);

        // Graph definition
        let mut graph = HashMap::new();

        graph.insert(
            "base_noise".to_string(),
            NodeInstance::Direct(Node::Generator(Generator::ImprovedPerlin {
                seed: ConstantOrValue::Constant("TERRAIN_SEED".to_string()),
            })),
        );

        graph.insert(
            "mountain_layer".to_string(),
            NodeInstance::FromTemplate {
                template: "MountainBase".to_string(),
                inputs: [("base_noise".to_string(), "base_noise".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            },
        );

        graph.insert(
            "forest_layer".to_string(),
            NodeInstance::FromTemplate {
                template: "ForestLayer".to_string(),
                inputs: [
                    ("base_noise".to_string(), "base_noise".to_string()),
                    ("forest_layer".to_string(), "forest_layer".to_string()),
                ]
                .iter()
                .cloned()
                .collect(),
            },
        );

        graph.insert(
            "final_terrain".to_string(),
            NodeInstance::Direct(Node::Combiner(Combiner::Blend {
                base: "mountain_layer".to_string(),
                other: "forest_layer".to_string(),
                control: "base_noise".to_string(),
            })),
        );

        let noise_map = NoiseMap {
            name: "TerrainNoiseMap".to_string(),
            constants,
            node_templates,
            output: Some("final_terrain".to_string()),
            graph,
        };

        // Serialize the NoiseMap instance to a JSON string
        let json_string = serde_json::to_string_pretty(&noise_map).unwrap();
        println!("{}", json_string);

        // Deserialize back to a NoiseMap instance
        let deserialized_map: NoiseMap = serde_json::from_str(&json_string).unwrap();

        // Assertions
        assert_eq!(
            noise_map, deserialized_map,
            "Deserialized data does not match the original"
        );
    }
}
