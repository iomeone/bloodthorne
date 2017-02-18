use std::io::{Result, Error, ErrorKind};
use std::collections::HashMap;

use dota::demo::CDemoSendTables;
use dota::netmessages::{CSVCMsg_FlattenedSerializer, ProtoFlattenedSerializer_t};
use bitstream::BitStream;
use protobuf;
use replay::Replay;
use property_serializers::PropertySerializerTable;
use flattened_serializers::{FlattenedSerializers, DataTable};

struct ToInsert(String, i32, DataTable);

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

    let flattened_serializers = FlattenedSerializers::new(flattened_serializer.clone(),
                                                          property_serializer_table,
                                                          replay.game_build());

    let symbols = flattened_serializer.get_symbols();
    let to_insert = flattened_serializer.get_serializers()
        .iter()
        .filter(|s| {
            s.has_serializer_name_sym() && s.get_serializer_name_sym() >= 0 &&
            (s.get_serializer_name_sym() as usize) < symbols.len()
        })
        .map(|s| {
            ToInsert(symbols[s.get_serializer_name_sym() as usize].to_string(),
                     s.get_serializer_version(),
                     flattened_serializers.recurse_table(s))
        })
        .collect::<Vec<ToInsert>>();


    let mut fs = flattened_serializers;
    for t in to_insert {
        let ToInsert(name, version, data_table) = t;
        let mut indexes_to_datatables = fs.serializers
            .entry(name)
            .or_insert(HashMap::new());
        indexes_to_datatables.insert(version, data_table);
    }


    Ok(fs)
}