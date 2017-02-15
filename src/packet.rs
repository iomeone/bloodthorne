
use bitstream::BitStream;
use std::io::{Result, Error};
use dota::demo::CDemoPacket;

pub struct PacketData {
    pub kind: u32,
    pub data: Vec<u8>,
}

impl PacketData {
    fn new(bitstream: &mut BitStream) -> Result<PacketData> {
        let kind = bitstream.read_ubitvarint().map_err(Error::from)?;
        let size = bitstream.read_u32var().map_err(Error::from)?;
        let data = bitstream.read_bytes(size as usize).map_err(Error::from)?;

        Ok(PacketData {
            kind: kind,
            data: data,
        })
    }

    pub fn from_packet(packet: &CDemoPacket) -> Result<Vec<PacketData>> {
        let data = packet.get_data().to_vec();
        let mut bitstream = BitStream::new(data);

        let mut res = Vec::new();

        while bitstream.remaining_bytes() > 0 {
            let packet_data = PacketData::new(&mut bitstream)?;
            res.push(packet_data);
        }

        Ok(res)
    }
}