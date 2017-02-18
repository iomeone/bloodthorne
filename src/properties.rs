use std::collections::HashMap;

enum Value {
    Bool(bool),
    U32(u32),
    I32(i32),
    U64(u64),
    F32(f32),
    String(String),
}

struct Properties(HashMap<String, Value>);