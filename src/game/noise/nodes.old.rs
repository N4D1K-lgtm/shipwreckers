use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::any::Any;

use libnoise::{
    Abs, Add, Billow, Blend, Checkerboard, Clamp, Constant, Exp, Fbm, Generator, ImprovedPerlin,
    Max, Min, Mul, Neg, Perlin, Pow, Power, Product, RidgedMulti, Scale, Select, Sum, Translate,
    Value, Worley,
};

use assert_matches::assert_matches;

#[derive(Debug)]
pub enum NoiseError {
    NodeNotFound,
    InvalidParameters,
    CircularReference,
    // ... other error types
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Node {
    // Sources
    Constant {
        seed: ConstantOrValue<f64>,
    },

    Value {
        seed: ConstantOrValue<u64>,
    },

    Perlin {
        seed: ConstantOrValue<u64>,
    },

    ImprovedPerlin {
        seed: ConstantOrValue<u64>,
    },

    Worley {
        seed: ConstantOrValue<u64>,
    },

    Checkerboard {
        seed: ConstantOrValue<u64>,
    },

    // Complex Generators
    Fbm {
        input: String,
        octaves: ConstantOrValue<u32>,
        frequency: ConstantOrValue<f64>,
        lacunarity: ConstantOrValue<f64>,
        persistence: ConstantOrValue<f64>,
    },

    Billow {
        input: String,
        octaves: ConstantOrValue<u32>,
        frequency: ConstantOrValue<f64>,
        lacunarity: ConstantOrValue<f64>,
        persistence: ConstantOrValue<f64>,
    },

    RidgedMulti {
        input: String,
        octaves: ConstantOrValue<u32>,
        frequency: ConstantOrValue<f64>,
        lacunarity: ConstantOrValue<f64>,
        persistence: ConstantOrValue<f64>,
    },

    // Adapters and Modifiers
    Abs {
        input: String,
    },

    Add {
        input: String,
        value: ConstantOrValue<f64>,
    },

    Clamp {
        input: String,
        min: ConstantOrValue<f64>,
        max: ConstantOrValue<f64>,
    },

    Exp {
        input: String,
    },

    Mul {
        input: String,
        value: ConstantOrValue<f64>,
    },

    Neg {
        input: String,
    },

    PowF64 {
        input: String,
        exponent: ConstantOrValue<f64>,
    },

    PowI32 {
        input: String,
        exponent: ConstantOrValue<i32>,
    },

    Scale {
        input: String,
        scale: ConstantOrValue<Vec<f64>>,
    },

    Translate {
        input: String,
        translation: ConstantOrValue<Vec<f64>>,
    },

    Max {
        a: String,
        b: String,
    },

    Min {
        a: String,
        b: String,
    },

    Power {
        base: String,
        exponent: String,
    },

    Product {
        a: String,
        b: String,
    },

    Sum {
        a: String,
        b: String,
    },

    Blend {
        a: String,
        b: String,
        control: String,
    },

    Select {
        a: String,
        b: String,
        control: String,
        lower_bound: ConstantOrValue<f64>,
        upper_bound: ConstantOrValue<f64>,
    },
}

impl Node {
    pub fn generate(&self) -> Result<Box<dyn Any>, NoiseError> {
        self.validate()?;
        let mut node_with_defaults = self.clone(); // Make a clone to apply defaults
        node_with_defaults.with_defaults();
        node_with_defaults.to_generator()
    }

    pub fn to_generator(&self) -> Result<Box<dyn Any>, NoiseError> {
        match self {
            Node::Constant { seed } => Ok(Box::new(Constant::<2>::new(*seed))),
            Node::Value { seed } => Ok(Box::new(Value::<2>::new(*seed))),
            // ... and so on for other nodes.
            _ => Err(NoiseError::InvalidParameters),
        }
    }

    fn with_defaults(&mut self) {
        // Changed to take a mutable reference
        match self {
            Node::Constant { seed } => {
                if *seed == 0.0 {
                    *seed = 1.0;
                }
            }
            Node::Value { seed } => {
                if *seed == 0 {
                    *seed = 1;
                }
            }
            // ... and so on for other nodes with their specific defaulting logic.
            _ => {}
        }
    }
    fn validate(&self) -> Result<(), NoiseError> {
        match self {
            Node::Constant { seed } => {
                if *seed < 0.0 {
                    return Err(NoiseError::InvalidParameters);
                }
            }
            Node::Value { seed } => {
                if *seed == 0 {
                    return Err(NoiseError::InvalidParameters);
                }
            }
            // ... and so on for other nodes with their specific validation logic.
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ConstantOrValue<T> {
    Constant(String),
    RawValue(T),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInput {
    input: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInstance {
    pub id: String, // Unique identifier for the instance
    pub node_type: Node,
    pub parameters: HashMap<String, NodeValue>, // Parameters and their values or references
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Edge {
    pub from: String, // NodeInstance name
    pub to: String,   // NodeInstance name
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub constants: HashMap<String, f64>,
    pub nodes: Vec<CustomNode>,
    pub instances: HashMap<String, NodeInstance>,
    pub edges: Vec<Edge>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_constant() {
        let node = Node::Constant { seed: 5.0 };
        match node.generate() {
            Ok(_generator) => {
                // Here, you can further test the generator if you can cast it or use it in some way.
            }
            Err(e) => panic!("Failed to generate with error: {:?}", e),
        }
    }

    #[test]
    fn test_generate_value() {
        let node = Node::Value { seed: 2 };
        match node.generate() {
            Ok(_generator) => {
                // Here, you can further test the generator if you can cast it or use it in some way.
            }
            Err(e) => panic!("Failed to generate with error: {:?}", e),
        }
    }

    #[test]
    fn test_validate_invalid_constant() {
        let node = Node::Constant { seed: -1.0 };
        assert_matches!(node.validate(), Err(NoiseError::InvalidParameters));
    }

    #[test]
    fn test_validate_invalid_value() {
        let node = Node::Value { seed: 0 };
        assert_matches!(node.validate(), Err(NoiseError::InvalidParameters));
    }

    #[test]
    fn test_with_defaults_constant() {
        let mut node = Node::Constant { seed: 0.0 };
        node.with_defaults();
        if let Node::Constant { seed } = &node {
            assert_eq!(*seed, 1.0);
        } else {
            panic!("Expected a Node::Constant after calling with_defaults");
        }
    }

    #[test]
    fn test_with_defaults_value() {
        let mut node = Node::Value { seed: 0 };
        node.with_defaults();
        if let Node::Value { seed } = &node {
            assert_eq!(*seed, 1);
        } else {
            panic!("Expected a Node::Value after calling with_defaults");
        }
    }
}
