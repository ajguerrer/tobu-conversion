#[derive(Debug)]
pub struct MessageDescriptor {
    pub name: &'static str,
    pub fields: &'static [FieldDescriptor],
}

#[derive(Debug)]
pub struct FieldDescriptor {
    pub ty: Type,
    pub label: Label,
}

#[derive(Debug)]
pub enum Type {
    Double = 1,
    Float = 2,
    Int64 = 3,
    UInt64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    Group = 10,
    Message = 11,
    Bytes = 12,
    UInt32 = 13,
    Enum = 14,
    SFixed64 = 15,
    SFixed32 = 16,
    SInt32 = 17,
    SInt64 = 18,
}

#[derive(Debug)]
pub enum Label {
    Optional = 1,
    Required = 2,
    Repeated = 3,
}

#[derive(Debug)]
pub struct EnumDescriptor {
    pub values: &'static [i32],
}
