use std::collections::HashMap;

enum LspAny {
    Object(HashMap<String, LspAny>),
    Array(Vec<LspAny>),
    String(String),
    Integer(i32),
    UInteger(u32),
    Decimal(f32),
    Bool(bool),
    None
}
