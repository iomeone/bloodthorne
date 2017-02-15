use dota::demo::CDemoSendTables;
use dota::netmessages::CSVCMsg_FlattenedSerializer;
use bitstream::BitStream;
use std::io::{Result, Error, ErrorKind};
use protobuf;

pub fn parse_send_tables(mut s: CDemoSendTables) -> Result<()> {
    let mut bitstream = BitStream::new(s.take_data());
    let size = bitstream.read_u32var()? as usize;

    if size != bitstream.remaining_bytes() {
        return Err(Error::new(ErrorKind::InvalidData, "Size != remaining bytes"));
    }

    let buffer = bitstream.read_bytes(size)?;
    let flattened_serializer =
        protobuf::parse_from_bytes::<CSVCMsg_FlattenedSerializer>(&buffer).map_err(Error::from)?;

    // TODO
    Ok(())
}