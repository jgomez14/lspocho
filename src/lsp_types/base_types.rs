use std::collections::HashMap;

pub enum LspAny {
    Object(HashMap<String, LspAny>),
    Array(Vec<LspAny>),
    String(String),
    Integer(i32),
    UInteger(u32),
    Decimal(f32),
    Bool(bool),
    None
}

pub enum LspRequestId {
    Integer(i32),
    String(String)
}

pub enum LspObjectOrArray {
    Object(HashMap<String, LspAny>),
    Array(Vec<LspAny>)
}
