// Valve Pak format

use std::io::{Result, Error, ErrorKind};
use std::collections::HashMap;
use bitstream::BitStream;

enum VBKVFlag {
    Object = 0,
    String = 1,
    I32 = 2,
    F32 = 3,
    // Pointer = 4,
    // WideString = 5,
    // Color = 6,
    U64 = 7,
    // End = 8,
    Skip = 11,
}

#[derive(PartialEq, Debug)]
enum VBKVValue {
    None,
    Object(HashMap<String, Box<VBKVValue>>),
    String(String),
    I32(i32),
    F32(f32),
    // Pointer,
    // WideString,
    // Color,
    U64(u64),
}

type KeyValue = (VBKVValue, VBKVValue);

pub struct Parser {
    bitstream: BitStream,
}

impl Parser {
    fn parse_key_value(&mut self) -> Result<KeyValue> {
        if self.bitstream.remaining_bytes() == 0 {
            return Ok((VBKVValue::None, VBKVValue::None));
        }

        let flag = self.bitstream.read_byte()?;

        match flag {
            flag if flag == VBKVFlag::Skip as u8 => Ok((VBKVValue::None, VBKVValue::None)),
            flag if flag == VBKVFlag::Object as u8 => self.parse_object(),
            flag if flag == VBKVFlag::String as u8 => {
                let k = VBKVValue::String(self.bitstream.read_string()?);
                let v = VBKVValue::String(self.bitstream.read_string()?);
                Ok((k, v))
            }
            flag if flag == VBKVFlag::I32 as u8 => {
                let k = VBKVValue::String(self.bitstream.read_string()?);
                let v = VBKVValue::I32(self.bitstream.read_i32()?);
                Ok((k, v))
            }
            flag if flag == VBKVFlag::F32 as u8 => {
                let k = VBKVValue::String(self.bitstream.read_string()?);
                let v = VBKVValue::F32(self.bitstream.read_f32()?);
                Ok((k, v))
            }
            flag if flag == VBKVFlag::U64 as u8 => {
                let k = VBKVValue::String(self.bitstream.read_string()?);
                let v = VBKVValue::U64(self.bitstream.read_u64()?);
                Ok((k, v))
            }
            _ => Err(Error::new(ErrorKind::InvalidData, format!("Unknown vpk flag {}", flag))),
        }
    }

    fn parse_entity(&mut self) {}

    fn parse_object(&mut self) -> Result<KeyValue> {
        let mut object = HashMap::new();
        let name = VBKVValue::String(self.bitstream.read_string()?);

        loop {
            let (k, v) = self.parse_key_value()?;
            if v == VBKVValue::None {
                break;
            }

            if let VBKVValue::String(k) = k {
                object.insert(k, Box::new(v));
            } else {
                return Err(Error::new(ErrorKind::InvalidData,
                                      format!("Invalid object key type: {:?}", k)));
            }

        }

        Ok((name, VBKVValue::Object(object)))
    }

    pub fn new(bytes: Vec<u8>) -> Parser {
        Parser { bitstream: BitStream::new(bytes) }
    }
}
