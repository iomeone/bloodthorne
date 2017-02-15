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
    None,
    String(String),
    I32(i32),
    F32(f32),
    Pointer,
    WideString,
    Color,
    U64(u64),
}

type KeyValue = (VPKValue, VPKValue);

pub struct Parser {
    bitstream: BitStream,
}

impl Parser {
    pub fn parse_key_value(&mut self) -> Result<KeyValue> {
        if self.bitstream.remaining_bytes() == 0 {
            return Ok((VPKValue::None, VPKValue::None));
        }

        let flag = self.bitstream.read_byte()?;

        match flag {
            flag if flag == VPKFlag::Skip as u8 => Ok((VPKValue::None, VPKValue::None)),
            flag if flag == VPKFlag::None as u8 => unimplemented!(),
            flag if flag == VPKFlag::String as u8 => {
                let k = VPKValue::String(self.bitstream.read_string()?);
                let v = VPKValue::String(self.bitstream.read_string()?);
                Ok((k, v))
            }
            _ => unreachable!(),
        }
    }

    pub fn new(bytes: Vec<u8>) -> Parser {
        Parser { bitstream: BitStream::new(bytes) }
    }
}
