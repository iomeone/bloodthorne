use std::io::{Result, Error, ErrorKind};
use std::collections::HashMap;

use dota::demo::CDemoSendTables;
use dota::netmessages::{CSVCMsg_FlattenedSerializer, ProtoFlattenedSerializer_t};
use bitstream::BitStream;
use protobuf;
use replay::Replay;
use property_serializers::PropertySerializerTable;
use flattened_serializers::{FlattenedSerializers, DataTable};

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

    let mut flattened_serializers = FlattenedSerializers::new(flattened_serializer.clone(),
                                                              property_serializer_table,
                                                              replay.game_build());

    for serializer in flattened_serializer.get_serializers() {
        if serializer.has_serializer_name_sym() {
            // TODO: handle case where i32 < 0 or i32 > get_symbols().len()
            let index = serializer.get_serializer_name_sym() as usize;
            let ref name = flattened_serializer.get_symbols()[index];
            let version = serializer.get_serializer_version();

            let mut indexes_to_datatables =
                flattened_serializers.serializers.entry(name.to_string()).or_insert(HashMap::new());
            indexes_to_datatables.insert(version, recurse_table(serializer));
        }
    }

    Ok(flattened_serializers)
}

fn recurse_table(serializer: &ProtoFlattenedSerializer_t) -> DataTable {
    DataTable {
        name: String::new(),
        flags: -1,
        version: -1,
        properties: Vec::new(),
    }
}