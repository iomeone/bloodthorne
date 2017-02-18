use dota::demo::CDemoSendTables;
use dota::netmessages::CSVCMsg_FlattenedSerializer;
use bitstream::BitStream;
use std::io::{Result, Error, ErrorKind};
use protobuf;
use replay::Replay;

use property_serializers::PropertySerializerTable;
use flattened_serializers::FlattenedSerializers;

pub fn parse_send_tables(replay: &Replay,
                         mut s: CDemoSendTables,
                         property_serializer_table: PropertySerializerTable)
                         -> Result<FlattenedSerializers> {
    let mut bitstream = BitStream::new(s.take_data());
    let size = bitstream.read_u32var()? as usize;

    if size != bitstream.remaining_bytes() {
        return Err(Error::new(ErrorKind::InvalidData, "Size != remaining bytes"));
    }

    let buffer = bitstream.read_bytes(size)?;
    let flattened_serializer =
        protobuf::parse_from_bytes::<CSVCMsg_FlattenedSerializer>(&buffer).map_err(Error::from)?;

    let flattened_serializers = FlattenedSerializers::new(flattened_serializer,
                                                          property_serializer_table,
                                                          replay.game_build());

    // TODO

    Ok(flattened_serializers)
}