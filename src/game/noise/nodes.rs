use serde::{Deserialize, Serialize};

pub const DEFAULT_OCTAVES: usize = 6;
pub const DEFAULT_FREQUENCY: f64 = 2.0;
pub const DEFAULT_LACUNARITY: f64 = std::f64::consts::PI * 2.0 / 3.0;
pub const DEFAULT_PERSISTENCE: f64 = 0.5;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ConstantOrValue<T: Default> {
    Constant(String),
    RawValue(T),
}

impl<T: Default> Default for ConstantOrValue<T> {
    fn default() -> Self {
        ConstantOrValue::RawValue(Default::default())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NoiseParameters {
    octaves: ConstantOrValue<usize>,
    frequency: ConstantOrValue<f64>,
    lacunarity: ConstantOrValue<f64>,
    persistence: ConstantOrValue<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct OperationParameters {
    value: ConstantOrValue<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct VectorOperationParameters {
    vector: ConstantOrValue<Vec<f64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BinaryOperationParameters {
    a: String,
    b: String,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Node {
    // Sources
    Constant {
        constant: ConstantOrValue<f64>,
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

    Checkerboard,

    // Complex Generators
    Fbm {
        input: String,
        parameters: NoiseParameters,
    },

    Billow {
        input: String,
        parameters: NoiseParameters,
    },

    RidgedMulti {
        input: String,
        parameters: NoiseParameters,
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
