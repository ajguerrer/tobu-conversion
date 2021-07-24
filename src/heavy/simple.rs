use std::convert::TryFrom;

use crate::{
    error::AbsorbError,
    value::{Message, Rule, Value},
};

pub struct Simple {
    pub simple_bool: bool,
}

impl Simple {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Simple {
    fn default() -> Self {
        Simple { simple_bool: false }
    }
}

impl From<Simple> for Message {
    fn from(m: Simple) -> Self {
        Message {
            fields: vec![Some(Value::Bool(Rule::Singular(m.simple_bool)))],
        }
    }
}

impl TryFrom<Message> for Simple {
    type Error = AbsorbError;

    fn try_from(m: Message) -> Result<Self, Self::Error> {
        if m.fields.len() != 1 {
            return Err(AbsorbError::invalid_length(1, m.fields.len()));
        }

        let mut fields = m.fields.into_iter();
        Ok(Simple {
            simple_bool: match fields
                .next()
                .unwrap()
                .unwrap_or(Value::Bool(Rule::Singular(false)))
            {
                Value::Bool(Rule::Singular(v)) => v,
                v => return Err(AbsorbError::invalid_type("simple_bool", &v)),
            },
        })
    }
}
