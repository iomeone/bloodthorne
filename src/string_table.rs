use std::collections::HashMap;
use std::collections::VecDeque;
use std::string::String;
use std::vec::Vec;
use std::io::Result;

use bitstream::BitStream;
use dota::netmessages::CSVCMsg_CreateStringTable;

#[derive(Debug)]
pub struct StringTableItem {
    index: i32,
    key: String,
    value: Vec<u8>,
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

#[derive(Debug)]
pub struct StringTable {
    index: i32,
    name: String,
    items: HashMap<i32, StringTableItem>,
    user_data_fixed_size: bool,
    user_data_size: i32,
}

impl StringTable {
    pub fn new(s: &CSVCMsg_CreateStringTable, index: i32) -> StringTable {
        StringTable {
            index: index,
            name: s.get_name().to_string(),
            items: HashMap::new(),
            user_data_fixed_size: s.get_user_data_fixed_size(),
            user_data_size: s.get_user_data_size(),
        }
    }

    pub fn add_items(&mut self, items: Vec<StringTableItem>) {
        for item in items {
            self.items.insert(item.index, item);
        }
    }

    pub fn user_data_fixed_size(&self) -> bool {
        self.user_data_fixed_size
    }

    pub fn user_data_size(&self) -> i32 {
        self.user_data_size
    }
}

#[derive(Debug)]
pub struct StringTables {
    index_to_tables: HashMap<i32, StringTable>,
    name_to_index: HashMap<String, i32>,
    next_index: i32,
}

impl StringTables {
    pub fn new() -> StringTables {
        StringTables {
            index_to_tables: HashMap::new(),
            name_to_index: HashMap::new(),
            next_index: 0,
        }
    }

    pub fn add_table(&mut self, table: StringTable) {
        self.name_to_index.insert(table.name.clone(), table.index);
        self.index_to_tables.insert(table.index, table);
    }

    pub fn get_table(&mut self, index: i32) -> Option<&mut StringTable> {
        self.index_to_tables.get_mut(&index)
    }

    pub fn incr_next_index(&mut self) {
        self.next_index += 1;
    }

    pub fn next_index(&self) -> i32 {
        self.next_index
    }
}
