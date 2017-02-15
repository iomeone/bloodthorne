// Valve Pak format

use std::io::{Result, Error, ErrorKind};
use bitstream::BitStream;

enum VPKFlag {
    None = 0,
    String = 1,
    Int = 2,
    Float = 3,
    Pointer = 4,
    WideString = 5,
    Color = 6,
    Uint64 = 7,
    End = 8,
    Skip = 11,
}

enum VPKValue {
    String(String),
    I32(i32),
    F32(f32),
    Pointer,
    WideString,
    Color,
    U64(u64),
}

type KeyValue = (String, VPKValue);

pub struct Parser {
    bitstream: BitStream,
}

impl Parser {
    pub fn parse_key_value(&mut self) -> Result<Option<KeyValue>> {
        if self.bitstream.remaining_bytes() == 0 {
            return Ok(None);
        }

        let flag = self.bitstream.read_byte()?;

        match flag {
            flag if flag == VPKFlag::Skip as u8 => Ok(None),
            _ => unimplemented!(),
        }
    }

    pub fn new(bytes: Vec<u8>) -> Parser {
        Parser { bitstream: BitStream::new(bytes) }
    }
}
