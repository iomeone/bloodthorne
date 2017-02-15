// Valve Pak format

use std::io::{Result, Error, ErrorKind};
use std::collections::HashMap;
use bitstream::BitStream;

enum VPKFlag {
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
enum VPKValue {
    None,
    Object(HashMap<String, Box<VPKValue>>),
    String(String),
    I32(i32),
    F32(f32),
    // Pointer,
    // WideString,
    // Color,
    U64(u64),
}

type KeyValue = (VPKValue, VPKValue);

pub struct Parser {
    bitstream: BitStream,
}

impl Parser {
    fn parse_key_value(&mut self) -> Result<KeyValue> {
        if self.bitstream.remaining_bytes() == 0 {
            return Ok((VPKValue::None, VPKValue::None));
        }

        let flag = self.bitstream.read_byte()?;

        match flag {
            flag if flag == VPKFlag::Skip as u8 => Ok((VPKValue::None, VPKValue::None)),
            flag if flag == VPKFlag::Object as u8 => self.parse_object(),
            flag if flag == VPKFlag::String as u8 => {
                let k = VPKValue::String(self.bitstream.read_string()?);
                let v = VPKValue::String(self.bitstream.read_string()?);
                Ok((k, v))
            }
            flag if flag == VPKFlag::I32 as u8 => {
                let k = VPKValue::String(self.bitstream.read_string()?);
                let v = VPKValue::I32(self.bitstream.read_i32()?);
                Ok((k, v))
            }
            flag if flag == VPKFlag::F32 as u8 => {
                let k = VPKValue::String(self.bitstream.read_string()?);
                let v = VPKValue::F32(self.bitstream.read_f32()?);
                Ok((k, v))
            }
            flag if flag == VPKFlag::U64 as u8 => {
                let k = VPKValue::String(self.bitstream.read_string()?);
                let v = VPKValue::U64(self.bitstream.read_u64()?);
                Ok((k, v))
            }
            _ => Err(Error::new(ErrorKind::InvalidData, format!("Unknown vpk flag {}", flag))),
        }
    }

    fn parse_entity(&mut self) {}

    fn parse_object(&mut self) -> Result<KeyValue> {
        let mut object = HashMap::new();
        let name = VPKValue::String(self.bitstream.read_string()?);

        loop {
            let (k, v) = self.parse_key_value()?;
            if v == VPKValue::None {
                break;
            }

            if let VPKValue::String(k) = k {
                object.insert(k, Box::new(v));
            } else {
                return Err(Error::new(ErrorKind::InvalidData,
                                      format!("Invalid object key type: {:?}", k)));
            }

        }

        Ok((name, VPKValue::Object(object)))
    }

    pub fn new(bytes: Vec<u8>) -> Parser {
        Parser { bitstream: BitStream::new(bytes) }
    }
}
