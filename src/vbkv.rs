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
    EndOfField = 11,
}

#[derive(PartialEq, Debug)]
pub enum VBKVValue {
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

type KeyValue = (String, VBKVValue);

pub struct Parser {
    bitstream: BitStream,
}

impl Parser {
    pub fn parse_key_value(&mut self) -> Result<KeyValue> {
        if self.bitstream.remaining_bytes() == 0 {
            return Ok((String::from(""), VBKVValue::None));
        }

        let flag = self.bitstream.read_byte()?;

        match flag {
            flag if flag == VBKVFlag::EndOfField as u8 => Ok((String::from(""), VBKVValue::None)),
            flag if flag == VBKVFlag::Object as u8 => self.parse_object(),
            flag if flag == VBKVFlag::String as u8 => {
                let k = self.bitstream.read_string()?;
                let v = VBKVValue::String(self.bitstream.read_string()?);
                Ok((k, v))
            }
            flag if flag == VBKVFlag::I32 as u8 => {
                let k = self.bitstream.read_string()?;
                let v = VBKVValue::I32(self.bitstream.read_i32()?);
                Ok((k, v))
            }
            flag if flag == VBKVFlag::F32 as u8 => {
                let k = self.bitstream.read_string()?;
                let v = VBKVValue::F32(self.bitstream.read_f32()?);
                Ok((k, v))
            }
            flag if flag == VBKVFlag::U64 as u8 => {
                let k = self.bitstream.read_string()?;
                let v = VBKVValue::U64(self.bitstream.read_u64()?);
                Ok((k, v))
            }
            _ => Err(Error::new(ErrorKind::InvalidData, format!("Unknown vpk flag {}", flag))),
        }
    }

    fn parse_entity(&mut self) {}

    fn parse_object(&mut self) -> Result<KeyValue> {
        let mut object = HashMap::new();
        let name = self.bitstream.read_string()?;

        loop {
            let (k, v) = self.parse_key_value()?;
            if v == VBKVValue::None {
                break;
            }

            object.insert(k, Box::new(v));

        }

        Ok((name, VBKVValue::Object(object)))
    }

    pub fn new(bytes: Vec<u8>) -> Parser {
        Parser { bitstream: BitStream::new(bytes) }
    }
}

#[cfg(test)]
mod tests {
    use vbkv::{Parser, VBKVValue};

    #[test]
    fn test_read_i32() {
        /// `2` announces a (string, i32) pair
        /// [118, 101, 114, 115, 105, 111, 110, 0] is "version\0"
        /// `[1, 0, 0, 0]` is the i32
        let mut parser = Parser::new(vec![2, 118, 101, 114, 115, 105, 111, 110, 0, 1, 0, 0, 0]);
        let kv = parser.parse_key_value();

        match kv {
            Ok((string, VBKVValue::I32(1))) => {
                assert_eq!(string, "version");
            }
            _ => panic!(format!("Wrong kv pair: {:?}", kv)),
        };
    }

    #[test]
    fn test_read_u64() {
        /// `7` announces a (string, u64) pair
        /// [109, 97, 116, 99, 104, 105, 100, 0] is "matchid\0"
        /// `[0, 43, 219, 177, 0, 0, 0, 0]` is the u64
        let mut parser = Parser::new(vec![7, 109, 97, 116, 99, 104, 105, 100, 0, 0, 43, 219, 177,
                                          0, 0, 0, 0]);
        let kv = parser.parse_key_value();

        match kv {
            Ok((string, VBKVValue::U64(2983930624))) => {
                assert_eq!(string, "matchid");
            }
            _ => panic!(format!("Wrong kv pair: {:?}", kv)),
        };
    }

    #[test]
    fn test_read_string() {
        /// `1` announces a (string, string) pair
        /// [110, 97, 109, 101, 0] is "name\0"
        /// `[66, 114, 111, 107, 101, 110, 0]` is "Broken\0"
        let mut parser = Parser::new(vec![1, 110, 97, 109, 101, 0, 66, 114, 111, 107, 101, 110, 0]);
        let kv = parser.parse_key_value();

        match kv {
            Ok((string1, VBKVValue::String(string2))) => {
                assert_eq!(string1, "name");
                assert_eq!(string2, "Broken");
            }
            _ => panic!(format!("Wrong kv pair: {:?}", kv)),
        };
    }

    #[test]
    fn test_read_simple_object() {
        /// `0` announces a (string, object) pair
        /// In json the object would look like:
        /// {
        ///    "m_iName": "Announcer"
        /// }
        let mut bytes = Vec::new();
        bytes.push(0);
        bytes.extend(b"npc_dota_hero_announcer\0");
        bytes.push(1); // String incoming
        bytes.extend(b"m_iName\0");
        bytes.extend(b"Announcer\0");
        bytes.push(11);
        // bytes.push(11);

        let mut parser = Parser::new(bytes);
        let kv = parser.parse_key_value();

        match kv {
            Ok((string, VBKVValue::Object(hashmap))) => {
                assert_eq!(string, "npc_dota_hero_announcer");
                assert_eq!(hashmap.len(), 1);
                let ref box_vbkv_announcer_name: Box<VBKVValue> = *(hashmap.get("m_iName")
                    .unwrap());

                if let VBKVValue::String(ref announcer_name) = **box_vbkv_announcer_name {
                    assert_eq!(announcer_name, "Announcer");
                } else {
                    panic!();
                }

            }
            _ => panic!("Wrong kv pair"),
        };
    }

    #[test]
    fn test_read_object() {
        /// `0` announces a (string, object) pair
        /// In json the object would look like:
        //// {
        ///    "npc_dota_hero_announcer": {
        ///        "m_iName": "Announcer"
        ///     }
        /// }
        let mut bytes = vec![0];
        bytes.extend(b"Units\0");

        bytes.push(0); // Object incoming
        bytes.extend(b"npc_dota_hero_announcer\0");
        bytes.push(1); // String incoming
        bytes.extend(b"m_iName\0");
        bytes.extend(b"Announcer\0");
        bytes.push(11);

        bytes.push(11);

        let mut parser = Parser::new(bytes);
        let kv = parser.parse_key_value();

        match kv {
            Ok((string, VBKVValue::Object(hashmap1))) => {
                assert_eq!(string, "Units");
                assert_eq!(hashmap1.len(), 1);
                let ref box_vbkv_obj2: Box<VBKVValue> = *(hashmap1.get("npc_dota_hero_announcer")
                    .unwrap());

                match **box_vbkv_obj2 {
                    VBKVValue::Object(ref obj2) => {
                        assert_eq!(obj2.len(), 1);
                        let ref box_vbkv_announcer_name = *(obj2.get("m_iName").unwrap());


                        if let VBKVValue::String(ref announcer_name) = **box_vbkv_announcer_name {
                            assert_eq!(announcer_name, "Announcer");
                        } else {
                            panic!();
                        }
                    }
                    _ => panic!("Wrong kv pair"),
                }


            }
            _ => panic!(format!("Wrong kv pair: {:?}", kv)),
        };
    }
}
