use std::{collections::HashMap, convert::TryFrom};

use crate::{
    error::AbsorbError,
    value::{Enum, Key, Message, Rule, Value},
};

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Complex {
    inner: Message,
}

impl Complex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn optional_enum(&self) -> ComplexEnum {
        match &self.inner.fields[0] {
            Some(Value::Enum(Rule::Singular(v))) => unsafe {
                // Safety: ComplexEnum is repr(i32) and
                // Enum is a repr(transparent) wrapper around i32
                *(v as *const Enum as *const ComplexEnum)
            },
            Some(_) => unreachable!(),
            None => ComplexEnum::default(),
        }
    }

    pub fn optional_enum_mut(&mut self) -> &mut ComplexEnum {
        match self.inner.fields[0]
            .get_or_insert(Value::Enum(Rule::Singular(ComplexEnum::default().into())))
        {
            Value::Enum(Rule::Singular(v)) => unsafe {
                // Safety: ComplexEnum is repr(i32) and
                // Enum is a repr(transparent) wrapper around i32
                &mut *(v as *mut Enum as *mut ComplexEnum)
            },
            _ => unreachable!(),
        }
    }

    pub fn clear_optional_enum(&mut self) {
        self.inner.fields[0] = None;
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
            _ => unreachable!(),
        }
    }

    pub fn map_message(&self) -> &HashMap<i32, ComplexNested> {
        match &self.inner.fields[2] {
            Some(Value::Message(Rule::Map(Key::I32(v)))) => unsafe {
                // Safety: ComplexNested is a repr(transparent) wrapper around a Message
                &*(v as *const HashMap<i32, Message> as *const HashMap<i32, ComplexNested>)
            },
            _ => unreachable!(),
        }
    }

    pub fn map_message_mut(&mut self) -> &mut HashMap<i32, ComplexNested> {
        match &mut self.inner.fields[2] {
            Some(Value::Message(Rule::Map(Key::I32(v)))) => unsafe {
                // Safety: ComplexNested is a repr(transparent) wrapper around a Message
                &mut *(v as *mut HashMap<i32, Message> as *mut HashMap<i32, ComplexNested>)
            },
            _ => unreachable!(),
        }
    }

    fn validate(m: &Message) -> Option<AbsorbError> {
        if m.fields.len() != 3 {
            return Some(AbsorbError::invalid_length(3, m.fields.len()));
        }

        match &m.fields[0] {
            Some(Value::Enum(Rule::Singular(v))) => ComplexEnum::validate(v),
            Some(v) => Some(AbsorbError::invalid_type("optional_enum", v)),
            None => None,
        }?;

        match &m.fields[1] {
            Some(Value::Bytes(Rule::Repeated(_))) => None,
            Some(v) => Some(AbsorbError::invalid_type("repeated_bytes", v)),
            None => Some(AbsorbError::not_optional("repeated_bytes")),
        }?;

        match &m.fields[2] {
            Some(Value::Message(Rule::Map(Key::I32(v)))) => {
                v.values().find_map(ComplexNested::validate)
            }
            Some(v) => Some(AbsorbError::invalid_type("map_message", v)),
            None => Some(AbsorbError::not_optional("map_message")),
        }
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

    fn try_from(m: Message) -> Result<Self, Self::Error> {
        if let Some(err) = Self::validate(&m) {
            return Err(err);
        }

        Ok(Complex { inner: m })
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ComplexNested {
    inner: Message,
}

impl ComplexNested {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn optional_string(&self) -> &str {
        match &self.inner.fields[0] {
            Some(Value::String(Rule::Singular(v))) => v,
            Some(_) => unreachable!(),
            None => "",
        }
    }

    pub fn optional_string_mut(&mut self) -> &mut String {
        match self.inner.fields[0].get_or_insert(Value::String(Rule::Singular("".to_string()))) {
            Value::String(Rule::Singular(v)) => v,
            _ => unreachable!(),
        }
    }

    pub fn clear_optional_enum(&mut self) {
        if let Some(v) = &mut self.inner.fields[0] {
            *v = Value::String(Rule::Singular("".to_string()))
        }
    }

    pub fn has_optional_enum(&self) -> bool {
        self.inner.fields[0].is_some()
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

    fn try_from(m: Message) -> Result<Self, Self::Error> {
        if let Some(err) = Self::validate(&m) {
            return Err(err);
        }

        Ok(ComplexNested { inner: m })
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn map_works() {
//        let mut c = Complex::new();
//
//        let mut cn = ComplexNested::new();
//        *cn.optional_string_mut() = "hello".to_string();
//        c.map_message_mut().insert(1, cn);
//
//        let mut cn = ComplexNested::new();
//        *cn.optional_string_mut() = "world".to_string();
//        c.map_message_mut().insert(2, cn);
//
//        let m = match &c.inner.fields[2] {
//            Some(Value::Message(Rule::Map(Key::I32(v)))) => v,
//            _ => unreachable!(),
//        };
//
//        let v = match &m[&1].fields[0] {
//            Some(Value::String(Rule::Singular(v))) => v,
//            _ => unreachable!(),
//        };
//
//        assert_eq!(v, "hello");
//
//        let v = match &m[&2].fields[0] {
//            Some(Value::String(Rule::Singular(v))) => v,
//            _ => unreachable!(),
//        };
//
//        assert_eq!(v, "world");
//    }
//}
