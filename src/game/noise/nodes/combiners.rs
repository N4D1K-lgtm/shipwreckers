use super::{ConstantOrValue, NoiseError, NoiseMap, Validatable};
use bevy::log;
use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for Combiner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Combiner::Blend {
                base,
                other,
                control,
            } => {
                write!(
                    f,
                    "Blend(Base: {}, Other: {}, Control: {})",
                    base, other, control
                )
            }
            Combiner::Max { base, other } => {
                write!(f, "Max(Base: {}, Other: {})", base, other)
            }
            Combiner::Min { base, other } => {
                write!(f, "Min(Base: {}, Other: {})", base, other)
            }
            Combiner::Power { base, exponent } => {
                write!(f, "Power(Base: {}, Exponent: {})", base, exponent)
            }
            Combiner::Product { base, other } => {
                write!(f, "Product(Base: {}, Other: {})", base, other)
            }
            Combiner::Sum { base, other } => {
                write!(f, "Sum(Base: {}, Other: {})", base, other)
            }
            Combiner::Select {
                base,
                other,
                control,
                lower_bound,
                upper_bound,
            } => {
                write!(
                    f,
                    "Select(Base: {}, Other: {}, Control: {}, Lower Bound: {}, Upper Bound: {})",
                    base, other, control, lower_bound, upper_bound
                )
            }
        }
    }
}

impl Validatable for Combiner {
    fn validate(&self, map: &NoiseMap) -> Result<(), NoiseError> {
        // log::debug!("Validating: {}", self);
        println!("Validating: {}", self);
        match self {
            Combiner::Blend {
                base,
                other,
                control,
            } => {
                map.validate_node_name(base)?;
                map.validate_node_name(other)?;
                map.validate_node_name(control)?;
            }
            Combiner::Max { base, other }
            | Combiner::Min { base, other }
            | Combiner::Product { base, other }
            | Combiner::Sum { base, other } => {
                map.validate_node_name(base)?;
                map.validate_node_name(other)?;
            }
            Combiner::Power { base, exponent } => {
                map.validate_node_name(base)?;
                map.validate_node_name(exponent)?;
            }
            Combiner::Select {
                base,
                other,
                control,
                lower_bound,
                upper_bound,
            } => {
                map.validate_node_name(base)?;
                map.validate_node_name(other)?;
                map.validate_node_name(control)?;

                // Validate the bounds
                lower_bound.validate(map)?;
                upper_bound.validate(map)?;
            }
        }
        Ok(())
    }
}
