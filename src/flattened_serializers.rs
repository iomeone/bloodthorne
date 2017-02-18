use std::collections::HashMap;
use property_serializers::{PropertySerializer, PropertySerializerTable};
use dota::netmessages::{CSVCMsg_FlattenedSerializer, ProtoFlattenedSerializer_t};

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
    pub name: String,
    pub flags: i32,
    pub version: i32,
    pub properties: Vec<DataTableProperty>,
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

    pub fn recurse_table(&self, serializer: &ProtoFlattenedSerializer_t) -> DataTable {
        let data_table = DataTable {
            name: self.proto.get_symbols()[serializer.get_serializer_name_sym() as usize]
                .to_string(),
            flags: 0,
            version: serializer.get_serializer_version(),
            properties: Vec::new(),
        };

        // let props =

        data_table
    }
}