use std::collections::HashMap;
use property_serializers::{PropertySerializer, PropertySerializerTable};
use dota::netmessages::CSVCMsg_FlattenedSerializer;

pub struct DataTableProperty {
    field: DataTableField,
    table: DataTable,
}

pub struct DataTableField {
    name: String,
    encoder: String,
    kind: String,
    index: i32,
    flags: i32,
    bit_count: i32,
    low_value: f32,
    high_value: f32,
    version: i32,
    serializer: PropertySerializer,
    build: u32,
}

pub struct DataTable {
    name: String,
    flags: i32,
    version: i32,
    properties: Vec<DataTableProperty>,
}

pub struct FlattenedSerializers {
    pub serializers: HashMap<String, HashMap<i32, DataTable>>,
    proto: CSVCMsg_FlattenedSerializer,
    property_serializer_table: PropertySerializerTable,
    build: u32,
}

impl FlattenedSerializers {
    pub fn new(proto: CSVCMsg_FlattenedSerializer,
               property_serializer_table: PropertySerializerTable,
               build: u32)
               -> FlattenedSerializers {
        FlattenedSerializers {
            serializers: HashMap::new(),
            proto: proto,
            property_serializer_table: property_serializer_table,
            build: build,
        }
    }
}