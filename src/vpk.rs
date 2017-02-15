// Valve Pak format


enum VPKValue {
    None,
    String(String),
    Int(i32),
    Float(f32),
    Pointer,
    WideString,
    Color,
    Uint64,
    End,
}

pub struct Parser {
    bytes: Vec<u8>,
}
