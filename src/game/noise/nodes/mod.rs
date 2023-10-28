use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use std::fmt;

mod combiners;
mod error;
mod generators;
mod modifiers;
mod transformers;

use super::constant_types::ConstantTypes;

pub use combiners::Combiner;
pub use error::NoiseError;
pub use generators::Generator;
pub use modifiers::{Modifier, NoiseParameters};
pub use transformers::Transformer;

trait Validatable {
    fn validate(&self, noise_map: &NoiseMap) -> Result<(), NoiseError>;
}

// An enumeration representing either a named constant or a raw value.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ConstantOrValue<T: Default> {
    #[serde(rename = "constant")]
    Constant(String),
    #[serde(rename = "value")]
    RawValue(T),
}

impl<T: Default + fmt::Display> fmt::Display for ConstantOrValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantOrValue::Constant(name) => write!(f, "Constant({})", name),
            ConstantOrValue::RawValue(value) => write!(f, "RawValue({})", value),
        }
    }
}

impl<T: Default> Default for ConstantOrValue<T> {
    fn default() -> Self {
        ConstantOrValue::RawValue(Default::default())
    }
}

impl<T: Default> Validatable for ConstantOrValue<T> {
    fn validate(&self, noise_map: &NoiseMap) -> Result<(), NoiseError> {
        match self {
            ConstantOrValue::Constant(name) => {
                if !noise_map.constants.contains_key(name) {
                    return Err(NoiseError::UnresolvedConstant(name.to_string()));
                }
            }
            ConstantOrValue::RawValue(_) => {
                // If there are any validations needed for raw values,
                // add them here.
                // Otherwise, this is always valid.
            }
        }
        Ok(())
    }
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

impl Validatable for Node {
    fn validate(&self, noise_map: &NoiseMap) -> Result<(), NoiseError> {
        match self {
            Node::Generator(gen) => gen.validate(noise_map),
            Node::Modifier(modifier) => modifier.validate(noise_map),
            Node::Combiner(combiner) => combiner.validate(noise_map),
            Node::Transformer(transformer) => transformer.validate(noise_map),
        }
    }
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

impl Validatable for NodeTemplate {
    fn validate(&self, noise_map: &NoiseMap) -> Result<(), NoiseError> {
        for node in self.nodes.values() {
            node.validate(noise_map)?;
        }
        Ok(())
    }
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

impl Validatable for NodeInstance {
    fn validate(&self, noise_map: &NoiseMap) -> Result<(), NoiseError> {
        match self {
            NodeInstance::Direct(node) => node.validate(noise_map),
            NodeInstance::FromTemplate { template, inputs } => {
                // Check if the template exists
                let template = noise_map
                    .node_templates
                    .get(template)
                    .ok_or_else(|| NoiseError::TemplateNotFound(template.to_string()))?;

                // Validate inputs
                for (template_input, actual_input) in inputs {
                    if !template.inputs.contains(template_input) {
                        return Err(NoiseError::InvalidTemplateInput(template_input.clone()));
                    }
                    noise_map.validate_node_name(actual_input)?;
                }

                Ok(())
            }
        }
    }
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

impl Validatable for NoiseMap {
    fn validate(&self, _: &NoiseMap) -> Result<(), NoiseError> {
        // TODO: Validate constants

        // TODO: Validate node templates

        // TODO: Validate node instances

        // Validate nodes in graph
        for node_instance in self.graph.values() {
            node_instance.validate(self)?;
        }
        Ok(())
    }
}

impl NoiseMap {
    fn validate_node_name(&self, name: &str) -> Result<(), NoiseError> {
        if !self.graph.contains_key(name) && !self.node_templates.contains_key(name) {
            // Check if the name exists in either the graph or node_templates
            Err(NoiseError::NodeNotFound(name.to_string()))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_NOISE_MAP_JSON: &str = r#"
{
    "name": "TerrainNoiseMap",
    "constants": {
        "MAIN_FREQUENCY": {
            "float": 2.5
        },
        "MAIN_OCTAVES": {
            "usize": 5
        },
        "SCALE_FACTOR": {
            "float": 3.0
        },
        "TERRAIN_SEED": {
            "unsignedint": 1234567890
        }
    },
    "node_templates": {
        "ForestLayer": {
            "name": "ForestLayer",
            "inputs": [
                "base_noise"
            ],
            "output": "forest_ridged",
            "nodes": {
                "forest_ridged": {
                    "RidgedMulti": {
                        "input": "base_noise",
                        "octaves": {
                            "value": 6
                        },
                        "frequency": {
                            "value": 2.0
                        },
                        "lacunarity": {
                            "value": 2.0943951023931953
                        },
                        "persistence": {
                            "value": 0.5
                        }
                    }
                }
            }
        },
        "MountainBase": {
            "name": "MountainBase",
            "inputs": [
                "base_noise",
                "mountain_exponent"
            ],
            "output": "mountain_power",
            "nodes": {
                "mountain_power": {
                    "Power": {
                        "base": "base_noise",
                        "exponent": "mountain_exponent"
                    }
                }
            }
        }
    },
    "output": "final_terrain",
    "graph": {
        "mountain_layer": {
            "template": "MountainBase",
            "inputs": {
                "base_noise": "base_noise"
            }
        },
        "base_noise": {
            "ImprovedPerlin": {
                "seed": {
                    "constant": "TERRAIN_SEED"
                }
            }
        },
        "forest_layer": {
            "template": "ForestLayer",
            "inputs": {
                "base_noise": "base_noise",
                "forest_layer": "forest_layer"
            }
        },
        "final_terrain": {
            "Blend": {
                "base": "mountain_layer",
                "other": "forest_layer",
                "control": "base_noise"
            }
        }
    }
}"#;

    #[test]
    fn test_valid_noisemap() {
        let noise_map: NoiseMap = serde_json::from_str(VALID_NOISE_MAP_JSON).unwrap();
        match noise_map.validate(&noise_map) {
            Ok(_) => (),
            Err(err) => {
                println!("Validation error: {:?}", err);
                panic!("Expected validation to succeed");
            }
        }
    }
    #[test]
    fn test_invalid_constant_in_node() {
        let mut noise_map: NoiseMap = serde_json::from_str(VALID_NOISE_MAP_JSON).unwrap();
        // Modify a node to reference a non-existent constant
        if let Some(NodeInstance::Direct(Node::Generator(Generator::ImprovedPerlin {
            ref mut seed,
        }))) = noise_map.graph.get_mut("base_noise")
        {
            *seed = ConstantOrValue::Constant("INVALID_CONSTANT".to_string());
        }
        assert!(noise_map.validate(&noise_map).is_err());
    }

    #[test]
    fn test_invalid_node_template() {
        let mut noise_map: NoiseMap = serde_json::from_str(VALID_NOISE_MAP_JSON).unwrap();
        // Remove a node template entirely
        noise_map.node_templates.remove("MountainBase");
        assert!(noise_map.validate(&noise_map).is_err());
    }

    // ... More tests for other types of errors ...
}
