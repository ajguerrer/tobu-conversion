use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(Rule<bool>),
    Bytes(Rule<Vec<u8>>),
    Enum(Rule<Enum>),
    F32(Rule<f32>),
    F64(Rule<f64>),
    I32(Rule<i32>),
    I64(Rule<i64>),
    Message(Rule<Message>),
    String(Rule<String>),
    U32(Rule<u32>),
    U64(Rule<u64>),
}

#[derive(Debug, Clone)]
pub enum Rule<T> {
    Singular(T),
    Repeated(Vec<T>),
    Map(Key<T>),
}

#[derive(Debug, Clone)]
pub enum Key<T> {
    Bool(HashMap<bool, T>),
    I32(HashMap<i32, T>),
    I64(HashMap<i64, T>),
    String(HashMap<String, T>),
    U32(HashMap<u32, T>),
    U64(HashMap<u64, T>),
}

impl Value {
    pub fn type_string(&self) -> String {
        match self {
            Value::Bool(v) => format!("Value::Bool({})", v.type_string()),
            Value::Bytes(v) => format!("Value::Bytes({})", v.type_string()),
            Value::Enum(v) => format!("Value::Enum({})", v.type_string()),
            Value::F32(v) => format!("Value::F32({})", v.type_string()),
            Value::F64(v) => format!("Value::F64({})", v.type_string()),
            Value::I32(v) => format!("Value::I32({})", v.type_string()),
            Value::I64(v) => format!("Value::I64({})", v.type_string()),
            Value::Message(v) => format!("Value::Message({})", v.type_string()),
            Value::String(v) => format!("Value::String({})", v.type_string()),
            Value::U32(v) => format!("Value::U32({})", v.type_string()),
            Value::U64(v) => format!("Value::U64({})", v.type_string()),
        }
    }
}

impl<T> Rule<T> {
    pub fn type_string(&self) -> String {
        match self {
            Rule::Singular(v) => "Rule::Singular()".to_owned(),
            Rule::Repeated(v) => "Rule::Repeated()".to_owned(),
            Rule::Map(v) => format!("Rule::Map({})", v.type_string()),
        }
    }
}

impl<T> Key<T> {
    pub fn type_string(&self) -> String {
        match self {
            Key::Bool(_) => "Key::Bool".to_owned(),
            Key::I32(_) => "Key::I32".to_owned(),
            Key::I64(_) => "Key::I64".to_owned(),
            Key::String(_) => "Key::String".to_owned(),
            Key::U32(_) => "Key::U32".to_owned(),
            Key::U64(_) => "Key::U64".to_owned(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Enum {
    pub number: i32,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub fields: Vec<Option<Value>>,
}
