use std::collections::HashMap;
use property_serializers::{PropertySerializer, PropertySerializerTable};
use dota::netmessages::{CSVCMsg_FlattenedSerializer, ProtoFlattenedSerializer_t,
                        ProtoFlattenedSerializerField_t};

macro_rules! get(
    ($e:expr) => (match $e { Some(e) => e, None => return None })
);

#[derive(Clone)]
pub struct DataTableProperty {
    field: DataTableField,
    table: Option<DataTable>,
}

#[derive(Clone)]
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
    serializer: Option<PropertySerializer>,
    build: u32,
}

#[derive(Clone)]
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
        let mut data_table = DataTable {
            name: self.proto.get_symbols()[serializer.get_serializer_name_sym() as usize]
                .to_string(),
            flags: 0,
            version: serializer.get_serializer_version(),
            properties: Vec::new(),
        };

        let props = self.proto.get_fields();

        for index in serializer.get_fields_index() {
            let index = *index as usize;
            let ref field = props[index];

            let encoder = if field.has_var_encoder_sym() {
                self.proto.get_symbols()[field.get_var_encoder_sym() as usize].to_string() // TODO: bound check
            } else {
                String::new()
            };

            let f = DataTableField {
                name: self.proto.get_symbols()[field.get_var_name_sym() as usize].to_string(), // TODO: bound check
                index: -1,
                encoder: encoder,
                flags: field.get_encode_flags(),
                bit_count: field.get_bit_count(),
                low_value: field.get_low_value(),
                high_value: field.get_high_value(),
                kind: self.proto.get_symbols()[field.get_var_type_sym() as usize].to_string(), // TODO: bound check
                version: field.get_field_serializer_version(),
                serializer: None,
                build: self.build,
            };

            let prop = DataTableProperty {
                field: f,
                table: self.get_data_table(field),
            };

            data_table.properties.push(prop);
        }

        data_table
    }

    fn get_data_table(&self, field: &ProtoFlattenedSerializerField_t) -> Option<DataTable> {
        let name_sym = if field.has_field_serializer_name_sym() {
            Some(field.get_field_serializer_name_sym())
        } else {
            None
        };

        let ref name = self.proto.get_symbols()[field.get_field_serializer_name_sym() as usize]; // TODO: bound check
        let ref version = field.get_field_serializer_version();

        let version_to_serializer = get!(self.serializers.get(name));
        version_to_serializer.get(version).cloned()
    }
}
