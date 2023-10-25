use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use libnoise::{
    Abs, Add, Billow, Blend, Checkerboard, Clamp, Constant, Exp, Fbm, ImprovedPerlin, Max, Min,
    Mul, Neg, Perlin, Pow, Power, Product, RidgedMulti, Scale, Select, Sum, Translate, Value,
    Worley,
};

#[derive(Debug, Deserialize, Serialize)]
pub enum Node {
    // Sources
    Constant {
        seed: f64,
    },
    Value {
        seed: f64,
    },
    Perlin {
        seed: f64,
    },
    ImprovedPerlin {
        seed: f64,
    },
    Worley {
        seed: f64,
    },
    Checkerboard {
        seed: f64,
    },

    // Adapters and Modifiers
    Abs {
        input: String,
    },
    Add {
        input: String,
        value: f64,
    },
    Clamp {
        input: String,
        min: f64,
        max: f64,
    },
    Exp {
        input: String,
    },
    Mul {
        input: String,
        value: f64,
    },
    Neg {
        input: String,
    },
    PowF64 {
        input: String,
        exponent: f64,
    },
    PowI32 {
        input: String,
        exponent: i32,
    },
    Scale {
        input: String,
        scale: Vec<f64>,
    },
    Translate {
        input: String,
        translation: Vec<f64>,
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
        lower_bound: f64,
        upper_bound: f64,
    },
    Composite(Vec<Node>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomNode {
    pub name: String,
    pub chain: Vec<NodeInstance>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NodeValue {
    ConstantName(String), // Reference to a constant defined in the config
    RawValue(f64),        // Direct f64 value
    Connection(String),   // Name/ID of the NodeInstance to connect to
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
