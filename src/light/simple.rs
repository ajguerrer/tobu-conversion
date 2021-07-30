use std::convert::TryFrom;

use crate::{
    error::AbsorbError,
    value::{Message, Rule, Value},
};

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Simple {
    inner: Message,
}

impl Simple {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn simple_bool(&self) -> bool {
        match &self.inner.fields[0] {
            Some(Value::Bool(Rule::Singular(v))) => *v,
            _ => unreachable!(),
        }
    }

    pub fn simple_bool_mut(&mut self) -> &mut bool {
        match &mut self.inner.fields[0] {
            Some(Value::Bool(Rule::Singular(v))) => v,
            _ => unreachable!(),
        }
    }

    fn validate(m: &Message) -> Option<AbsorbError> {
        if m.fields.len() != 1 {
            return Some(AbsorbError::invalid_length(1, m.fields.len()));
        }

        match &m.fields[0] {
            Some(Value::Bool(Rule::Singular(_))) => None,
            Some(v) => Some(AbsorbError::invalid_type("simple_bool", v)),
            None => Some(AbsorbError::not_optional("simple_bool")),
        }
    }
}

impl Default for Simple {
    fn default() -> Self {
        Simple {
            inner: Message {
                fields: vec![Some(Value::Bool(Rule::Singular(false)))],
            },
        }
    }
}

impl From<Simple> for Message {
    fn from(m: Simple) -> Self {
        m.inner
    }
}

impl TryFrom<Message> for Simple {
    type Error = AbsorbError;

    fn try_from(m: Message) -> Result<Self, Self::Error> {
        if let Some(err) = Self::validate(&m) {
            return Err(err);
        }

        Ok(Simple { inner: m })
    }
}
