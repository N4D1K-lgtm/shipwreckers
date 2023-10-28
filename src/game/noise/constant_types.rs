use anyhow::Error;
use serde::{Deserialize, Serialize};

/// Enumeration of possible constant types that can be defined in a noise map.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConstantTypes {
    Float(f64),
    Int(i32),
    UnsignedInt(u64),
    Usize(usize),
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
