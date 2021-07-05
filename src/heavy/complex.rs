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

pub struct ComplexNested {
    pub optional_string: Option<String>,
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
        let mut fields = m.fields.into_iter();

        if fields.len() != 3 {
            return Err(AbsorbError::invalid_length(3, fields.len()));
        }

        Ok(Complex {
            optional_enum: fields
                .next()
                .unwrap()
                .map(|v| match v {
                    Value::Enum(Rule::Singular(v)) => ComplexEnum::new(v.number)
                        .ok_or_else(|| AbsorbError::invalid_enum("ComplexEnum", &v)),
                    v => Err(AbsorbError::invalid_type("optional_enum", &v)),
                })
                .transpose()?,
            repeated_bytes: match fields
                .next()
                .unwrap()
                .unwrap_or_else(|| Value::Bytes(Rule::Repeated(Vec::new())))
            {
                Value::Bytes(Rule::Repeated(v)) => Ok(v),
                v => Err(AbsorbError::invalid_type("repeated_bytes", &v)),
            }?,
            map_message: match fields
                .next()
                .unwrap()
                .unwrap_or_else(|| Value::Message(Rule::Map(Key::I32(HashMap::new()))))
            {
                Value::Message(Rule::Map(Key::I32(v))) => {
                    v.into_iter().map(|(k, v)| Ok((k, v.try_into()?))).collect()
                }
                v => Err(AbsorbError::invalid_type("map_message", &v)),
            }?,
        })
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
