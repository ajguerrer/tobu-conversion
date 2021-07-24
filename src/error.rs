use thiserror::Error;

use crate::value::{Enum, Value};

#[derive(Debug, Error)]
pub enum AbsorbError {
    #[error("{value} is not a valid variant of {name}")]
    InvalidEnum { name: String, value: i32 },

    #[error("Expected {expected} fields, but message contains {actual} fields")]
    InvalidLength { expected: usize, actual: usize },

    #[error("Field type {ty} does not match type of field {name}")]
    InvalidType { name: String, ty: String },

    #[error("Field {name} is not optional")]
    TypeNotOptional { name: String },
}

impl AbsorbError {
    pub fn invalid_enum(name: &str, enumeration: &Enum) -> Self {
        Self::InvalidEnum {
            name: name.to_string(),
            value: enumeration.number,
        }
    }

    pub fn invalid_length(expected: usize, actual: usize) -> Self {
        Self::InvalidLength { expected, actual }
    }

    pub fn invalid_type(name: &str, value: &Value) -> Self {
        Self::InvalidType {
            name: name.to_string(),
            ty: value.type_string(),
        }
    }

    pub fn not_optional(name: &str) -> Self {
        Self::TypeNotOptional {
            name: name.to_string(),
        }
    }
}
