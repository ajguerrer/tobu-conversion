use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

use crate::{
    error::AbsorbError,
    value::{Enum, Key, Message, Rule, Value},
};

pub struct Complex {
    inner: Message,
}

#[repr(i32)]
pub enum ComplexEnum {
    One = 1,
    Two = 2,
    Ten = 10,
}

impl Default for ComplexEnum {
    fn default() -> Self {
        ComplexEnum::One
    }
}

pub struct ComplexNested {
    inner: Message,
}

impl From<Complex> for Message {
    fn from(m: Complex) -> Self {
        m.inner
    }
}

impl TryFrom<Message> for Complex {
    type Error = AbsorbError;

    fn try_from(mut m: Message) -> Result<Self, Self::Error> {
        if m.fields.len() != 3 {
            return Err(AbsorbError::invalid_length(3, m.fields.len()));
        }

        match m.fields[0].get_or_insert(Value::Enum(Rule::Singular(ComplexEnum::default().into())))
        {
            Value::Bool(Rule::Singular(_)) => {}
            v => return Err(AbsorbError::invalid_type("simple_bool", &v)),
        };

        Ok(Complex { inner: m })
    }
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

impl From<ComplexNested> for Message {
    fn from(m: ComplexNested) -> Self {
        m.inner
    }
}

impl TryFrom<Message> for ComplexNested {
    type Error = AbsorbError;

    fn try_from(mut m: Message) -> Result<Self, Self::Error> {
        for (i, f) in m.fields.iter().enumerate() {
            match i {}
        }

        if m.fields.len() != 1 {
            return Err(AbsorbError::invalid_length(1, m.fields.len()));
        }

        match &m.fields[0] {
            None => {}
            Some(Value::Enum(Rule::Singular(e))) => {
                if ComplexEnum::new(e.number).is_none() {
                    return Err(AbsorbError::invalid_enum("ComplexEnum", e));
                }
            }
            Some(v) => return Err(AbsorbError::invalid_type("simple_bool", &v)),
        };

        match &m.fields[1].get_or_insert(Value::Bytes(Rule::Repeated(Vec::new()))) {
            Value::Bytes(Rule::Repeated(_)) => {}
            v => return Err(AbsorbError::invalid_type("repeated_bytes", v)),
        };

        match &m.fields[2].get_or_insert(Value::Message(Rule::Map(Key::I32(HashMap::new())))) {
            Value::Message(Rule::Map(Key::I32(v))) => v
                .values()
                .map(|v| ComplexNested::try_from(v.clone()))
                .collect::<Result>()?,
            v => return Err(AbsorbError::invalid_type("map_message", v)),
        };

        Ok(ComplexNested { inner: m })
    }
}
