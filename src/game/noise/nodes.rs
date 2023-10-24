use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use libnoise::{
    Abs, Add, Billow, Blend, Checkerboard, Clamp, Constant, Exp, Fbm, ImprovedPerlin, Max, Min,
    Mul, Neg, Perlin, Pow, Power, Product, RidgedMulti, Scale, Select, Sum, Translate, Value,
    Worley,
};

pub enum BaseNode<const D: usize> {
    // Sources
    Constant(Constant<D>),
    Value(Value<D>),
    Perlin(Perlin<D>),
    ImprovedPerlin(ImprovedPerlin<D>),
    Worley(Worley<D>),
    Checkerboard(Checkerboard<D>),

    // Adapters and Modifiers
    Abs(Abs<D, Box<BaseNode<D>>>),
    Add(Add<D, Box<BaseNode<D>>>),
    Clamp(Clamp<D, Box<BaseNode<D>>>),
    Exp(Exp<D, Box<BaseNode<D>>>),
    Mul(Mul<D, Box<BaseNode<D>>>),
    Neg(Neg<D, Box<BaseNode<D>>>),
    PowF64(Pow<D, Box<BaseNode<D>>, f64>),
    PowI32(Pow<D, Box<BaseNode<D>>, i32>),
    Scale(Scale<D, Box<BaseNode<D>>>),
    Translate(Translate<D, Box<BaseNode<D>>>),
    Max(Max<D, Box<BaseNode<D>>, Box<BaseNode<D>>>),
    Min(Min<D, Box<BaseNode<D>>, Box<BaseNode<D>>>),
    Power(Power<D, Box<BaseNode<D>>, Box<BaseNode<D>>>),
    Product(Product<D, Box<BaseNode<D>>, Box<BaseNode<D>>>),
    Sum(Sum<D, Box<BaseNode<D>>, Box<BaseNode<D>>>),
    Blend(Blend<D, Box<BaseNode<D>>, Box<BaseNode<D>>, Box<BaseNode<D>>>),
    Select(Select<D, Box<BaseNode<D>>, Box<BaseNode<D>>, Box<BaseNode<D>>>),
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
    pub node_type: BaseNodeType,
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
