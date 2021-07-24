use std::{collections::HashMap, convert::TryFrom};

use crate::{
    error::AbsorbError,
    value::{Enum, Key, Message, Rule, Value},
};

#[repr(transparent)]
pub struct Complex {
    inner: Message,
}

impl Complex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn optional_enum(&self) -> ComplexEnum {
        match &self.inner.fields[0] {
            Some(Value::Enum(Rule::Singular(v))) => *ComplexEnum::cast(v).unwrap(),
            Some(v) => unreachable!(),
            None => ComplexEnum::default(),
        }
    }

    pub fn optional_enum_mut(&mut self) -> &mut ComplexEnum {
        match self.inner.fields[0]
            .get_or_insert(Value::Enum(Rule::Singular(ComplexEnum::default().into())))
        {
            Value::Enum(Rule::Singular(v)) => ComplexEnum::cast_mut(v).unwrap(),
            v => unreachable!(),
        }
    }

    pub fn clear_optional_enum(&mut self) {
        if let Some(v) = &mut self.inner.fields[0] {
            *v = Value::Enum(Rule::Singular(ComplexEnum::default().into()))
        }
    }

    pub fn has_optional_enum(&self) -> bool {
        self.inner.fields[0].is_some()
    }

    pub fn repeated_bytes(&self) -> &Vec<Vec<u8>> {
        match &self.inner.fields[1] {
            Some(Value::Bytes(Rule::Repeated(v))) => v,
            _ => unreachable!(),
        }
    }

    pub fn repeated_bytes_mut(&mut self) -> &mut Vec<Vec<u8>> {
        match &mut self.inner.fields[1] {
            Some(Value::Bytes(Rule::Repeated(v))) => v,
            v => unreachable!(),
        }
    }

    pub fn map_message(&self) -> &HashMap<i32, &ComplexNested> {
        match &self.inner.fields[2] {
            Some(Value::Message(Rule::Map(Key::I32(v)))) => &v
                .iter()
                .map(|(k, v)| (*k, ComplexNested::cast(v).unwrap()))
                .collect(),
            _ => unreachable!(),
        }
    }

    pub fn map_message_mut(&mut self) -> &mut HashMap<i32, &mut ComplexNested> {
        match &mut self.inner.fields[2] {
            Some(Value::Message(Rule::Map(Key::I32(v)))) => &mut v
                .iter_mut()
                .map(|(k, v)| (*k, ComplexNested::cast_mut(v).unwrap()))
                .collect(),
            v => unreachable!(),
        }
    }

    fn validate(m: &Message) -> Option<AbsorbError> {
        if m.fields.len() != 3 {
            return Some(AbsorbError::invalid_length(3, m.fields.len()));
        }

        match &m.fields[0] {
            Some(Value::Enum(Rule::Singular(v))) => ComplexEnum::cast(v).err(),
            Some(v) => Some(AbsorbError::invalid_type("optional_enum", v)),
            None => None,
        }?;

        match &m.fields[1] {
            Some(Value::Bytes(Rule::Repeated(v))) => None,
            Some(v) => Some(AbsorbError::invalid_type("repeated_bytes", v)),
            None => Some(AbsorbError::not_optional("repeated_bytes")),
        }?;

        match &m.fields[2] {
            Some(Value::Message(Rule::Map(Key::I32(v)))) => {
                v.values().find_map(|v| ComplexNested::cast(v).err())
            }
            Some(v) => Some(AbsorbError::invalid_type("map_message", v)),
            None => Some(AbsorbError::not_optional("map_message")),
        }
    }

    fn cast(m: &Message) -> Result<&Self, AbsorbError> {
        if let Some(err) = Self::validate(m) {
            return Err(err);
        }

        // Safety: Complex is repr(transparent) wrapper around a single Message field
        Ok(unsafe { &*(m as *const Message as *const Complex) })
    }

    fn cast_mut(m: &mut Message) -> Result<&mut Self, AbsorbError> {
        if let Some(err) = Self::validate(m) {
            return Err(err);
        }

        // Safety: Complex is repr(transparent) wrapper around a single Message field
        Ok(unsafe { &mut *(m as *mut Message as *mut Complex) })
    }
}

impl Default for Complex {
    fn default() -> Self {
        Complex {
            inner: Message {
                fields: vec![
                    None,
                    Some(Value::Bytes(Rule::Repeated(Vec::new()))),
                    Some(Value::Message(Rule::Map(Key::I32(HashMap::new())))),
                ],
            },
        }
    }
}

impl From<Complex> for Message {
    fn from(m: Complex) -> Self {
        m.inner
    }
}

impl TryFrom<Message> for Complex {
    type Error = AbsorbError;

    fn try_from(mut m: Message) -> Result<Self, Self::Error> {
        Complex::cast(&m)?;
        Ok(Complex { inner: m })
    }
}

#[repr(i32)]
#[derive(Copy, Clone)]
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

    fn validate(e: &Enum) -> Option<AbsorbError> {
        match e.number {
            1 | 2 | 10 => None,
            _ => Some(AbsorbError::invalid_enum("ComplexEnum", e)),
        }
    }

    fn cast(e: &Enum) -> Result<&Self, AbsorbError> {
        if let Some(err) = Self::validate(e) {
            return Err(err);
        }

        // Safety: ComplexEnum is repr(i32)
        Ok(unsafe { &*(e.number as *const i32 as *const ComplexEnum) })
    }

    fn cast_mut(e: &mut Enum) -> Result<&mut Self, AbsorbError> {
        if let Some(err) = Self::validate(e) {
            return Err(err);
        }

        // Safety: ComplexEnum is repr(i32)
        Ok(unsafe { &mut *(e.number as *mut i32 as *mut ComplexEnum) })
    }
}

impl From<ComplexEnum> for Enum {
    fn from(e: ComplexEnum) -> Self {
        Enum { number: e as i32 }
    }
}

impl Default for ComplexEnum {
    fn default() -> Self {
        ComplexEnum::One
    }
}

#[repr(transparent)]
pub struct ComplexNested {
    inner: Message,
}

impl ComplexNested {
    pub fn new() -> Self {
        Self::default()
    }

    fn validate(m: &Message) -> Option<AbsorbError> {
        if m.fields.len() != 1 {
            return Some(AbsorbError::invalid_length(1, m.fields.len()));
        }

        match &m.fields[0] {
            Some(Value::String(Rule::Singular(_))) => None,
            Some(v) => Some(AbsorbError::invalid_type("optional_string", v)),
            None => None,
        }
    }

    fn cast(m: &Message) -> Result<&Self, AbsorbError> {
        if let Some(err) = Self::validate(m) {
            return Err(err);
        }

        // Safety: ComplexNested is repr(transparent) wrapper around a single Message field
        Ok(unsafe { &*(m as *const Message as *const ComplexNested) })
    }

    fn cast_mut(m: &mut Message) -> Result<&mut Self, AbsorbError> {
        if let Some(err) = Self::validate(m) {
            return Err(err);
        }

        // Safety: ComplexNested is repr(transparent) wrapper around a single Message field
        Ok(unsafe { &mut *(m as *mut Message as *mut ComplexNested) })
    }
}

impl Default for ComplexNested {
    fn default() -> Self {
        ComplexNested {
            inner: Message { fields: vec![None] },
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
        ComplexNested::cast(&m)?;
        Ok(ComplexNested { inner: m })
    }
}
