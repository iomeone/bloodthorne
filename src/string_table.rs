use std::collections::HashMap;
use std::collections::VecDeque;
use std::string::String;
use std::vec::Vec;
use std::io::Result;

use bitstream::BitStream;

#[derive(Debug)]
pub struct StringTableItem {
    index: i32,
    key: String,
    value: Vec<u8>,
}

pub struct StringTable {
    index: i32,
    name: String,
    items: HashMap<i32, StringTableItem>,
    user_data_fixed_size: bool,
    user_data_size: i32,
}

pub struct StringTables {
    index_to_tables: HashMap<i32, StringTable>,
    name_to_index: HashMap<String, i32>,
    next_index: i32,
}

impl StringTableItem {
    pub fn parse_string_table(data: Vec<u8>,
                              entries_count: i32,
                              is_data_fixed: bool,
                              user_data_size: i32)
                              -> Result<Vec<StringTableItem>> {
        let mut result = Vec::new();
        let mut index: i32 = -1;
        let mut keys = VecDeque::<String>::new();
        const KEY_HISTORY_SIZE: usize = 32;

        if data.is_empty() {
            return Ok(result);
        }

        let mut bitstream = BitStream::new(data);

        for _ in 0..entries_count {
            let increment = bitstream.read_bool()?;

            if increment {
                index += 1;
            } else {
                index = bitstream.read_u32var()? as i32 + 1;
            }

            let has_key = bitstream.read_bool()?;
            let mut key = String::new();

            if has_key {
                let use_history = bitstream.read_bool()?;

                if use_history {
                    let position = bitstream.read_bits(5)? as usize;
                    let size = bitstream.read_bits(5)? as usize;

                    if position >= keys.len() {
                        key.push_str(&bitstream.read_string().unwrap()); // FIXME
                    } else {
                        let ref string = keys[position as usize];

                        if size as usize > string.chars().count() {
                            key.push_str(string);
                        } else {
                            let s: String = string.chars().take(size as usize).collect();
                            key.push_str(&s);
                        }
                        key.push_str(&bitstream.read_string().unwrap()); // FIXME
                    }
                } else {
                    key = bitstream.read_string().unwrap(); // FIXME
                }

                if keys.len() >= KEY_HISTORY_SIZE {
                    keys.pop_front();
                }
                keys.push_back(key.clone());
            }

            let has_value = bitstream.read_bool()?;
            let mut value = Vec::new();

            if has_value {
                if is_data_fixed {
                    value = bitstream.read_bits_as_bytes(user_data_size as usize)?;
                } else {
                    let size = bitstream.read_bits(14)?;
                    bitstream.read_bits(3)?;
                    value = bitstream.read_bytes(size as usize)?;
                }
            }

            result.push(StringTableItem {
                index: index,
                key: key,
                value: value,
            });
        }

        Ok(result)
    }
}
