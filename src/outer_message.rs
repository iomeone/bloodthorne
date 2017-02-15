use protobuf::stream::CodedInputStream;
use std::io::{Result, Error};
use dota::demo::EDemoCommands;

extern crate snap;
use self::snap::Decoder;

pub struct OuterMessage {
    pub kind: i32,
    tick: u32,
    pub data: Vec<u8>,
}

impl OuterMessage {
    pub fn new(stream: &mut CodedInputStream) -> Result<OuterMessage> {
        let command = stream.read_uint32()?;
        let is_compressed_bitmask = EDemoCommands::DEM_IsCompressed as u32;
        let is_compressed = (command & is_compressed_bitmask) == is_compressed_bitmask;

        let mut kind: i32 = command as i32 & !is_compressed_bitmask as i32;
        if is_compressed {
            kind = kind & !is_compressed_bitmask as i32;
        }

        let mut tick = stream.read_uint32().map_err(Error::from)?;
        if tick == <u32>::max_value() {
            tick = 0;
        }

        let size = stream.read_uint32().map_err(Error::from)?;

        let mut data = stream.read_raw_bytes(size).map_err(Error::from)?;
        if is_compressed {
            let mut decoder = Decoder::new();
            data = decoder.decompress_vec(&data).map_err(Error::from)?;
        }

        Ok(OuterMessage {
            kind: kind,
            tick: tick,
            data: data,
        })
    }
}