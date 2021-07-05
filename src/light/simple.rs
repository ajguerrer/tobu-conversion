use std::convert::TryFrom;

use crate::{
    error::AbsorbError,
    value::{Message, Rule, Value},
};

pub struct Simple {
    inner: Message,
}

impl Simple {
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
}

impl From<Simple> for Message {
    fn from(m: Simple) -> Self {
        m.inner
    }
}

impl TryFrom<Message> for Simple {
    type Error = AbsorbError;

    fn try_from(mut m: Message) -> Result<Self, Self::Error> {
        if m.fields.len() != 1 {
            return Err(AbsorbError::invalid_length(1, m.fields.len()));
        }

        match m.fields[0].get_or_insert(Value::Bool(Rule::Singular(false))) {
            Value::Bool(Rule::Singular(_)) => {}
            v => return Err(AbsorbError::invalid_type("simple_bool", &v)),
        };

        Ok(Simple { inner: m })
    }
}
