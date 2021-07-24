use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    vec,
};

use crate::{
    error::AbsorbError,
    value::{Enum, Key, Message, Rule, Value},
};

pub struct Complex {
    pub optional_enum: Option<ComplexEnum>,
    pub repeated_bytes: Vec<Vec<u8>>,
    pub map_message: HashMap<i32, ComplexNested>,
}

impl Complex {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Complex {
    fn default() -> Self {
        Complex {
            optional_enum: None,
            repeated_bytes: Vec::new(),
            map_message: HashMap::new(),
        }
    }
}

impl From<Complex> for Message {
    fn from(m: Complex) -> Self {
        Message {
            fields: vec![
                m.optional_enum
                    .map(|v| Value::Enum(Rule::Singular(Enum { number: v as i32 }))),
                Some(Value::Bytes(Rule::Repeated(m.repeated_bytes))),
                Some(Value::Message(Rule::Map(Key::I32(
                    m.map_message
                        .into_iter()
                        .map(|(k, v)| (k, v.into()))
                        .collect(),
                )))),
            ],
        }
    }
}

impl TryFrom<Message> for Complex {
    type Error = AbsorbError;

    fn try_from(m: Message) -> Result<Self, Self::Error> {
        if m.fields.len() != 3 {
            return Err(AbsorbError::invalid_length(3, m.fields.len()));
        }

        Ok(Complex {
            optional_enum: m.fields[0]
                .map(|v| match v {
                    Value::Enum(Rule::Singular(v)) => ComplexEnum::new(v.number)
                        .ok_or_else(|| AbsorbError::invalid_enum("ComplexEnum", &v)),
                    v => Err(AbsorbError::invalid_type("optional_enum", &v)),
                })
                .transpose()?,
            repeated_bytes: match m.fields[1] {
                Some(Value::Bytes(Rule::Repeated(v))) => Ok(v),
                Some(v) => Err(AbsorbError::invalid_type("repeated_bytes", &v)),
                None => Err(AbsorbError::not_optional("repeated_bytes")),
            }?,
            map_message: match m.fields[2] {
                Some(Value::Message(Rule::Map(Key::I32(v)))) => {
                    v.into_iter().map(|(k, v)| Ok((k, v.try_into()?))).collect()
                }
                Some(v) => Err(AbsorbError::invalid_type("map_message", &v)),
                None => Err(AbsorbError::not_optional("map_message")),
            }?,
        })
    }
}

#[repr(i32)]
pub enum ComplexEnum {
    One = 1,
    Two = 2,
    Ten = 10,
}

impl ComplexEnum {
    pub fn new(number: i32) -> Option<ComplexEnum> {
        match number {
            1 => Some(ComplexEnum::One),
            2 => Some(ComplexEnum::Two),
            10 => Some(ComplexEnum::Ten),
            _ => None,
        }
    }
}

impl Default for ComplexEnum {
    fn default() -> Self {
        ComplexEnum::One
    }
}

impl From<ComplexEnum> for Enum {
    fn from(e: ComplexEnum) -> Self {
        Enum { number: e as i32 }
    }
}

pub struct ComplexNested {
    pub optional_string: Option<String>,
}

impl ComplexNested {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ComplexNested {
    fn default() -> Self {
        ComplexNested {
            optional_string: None,
        }
    }
}

impl From<ComplexNested> for Message {
    fn from(m: ComplexNested) -> Self {
        Message {
            fields: vec![m.optional_string.map(|v| Value::String(Rule::Singular(v)))],
        }
    }
}

impl TryFrom<Message> for ComplexNested {
    type Error = AbsorbError;

    fn try_from(m: Message) -> Result<Self, Self::Error> {
        if m.fields.len() != 1 {
            return Err(AbsorbError::invalid_length(1, m.fields.len()));
        }

        let mut fields = m.fields.into_iter();
        Ok(ComplexNested {
            optional_string: fields
                .next()
                .unwrap()
                .map(|v| match v {
                    Value::String(Rule::Singular(v)) => Ok(v),
                    v => Err(AbsorbError::invalid_type("optional_string", &v)),
                })
                .transpose()?,
        })
    }
}
