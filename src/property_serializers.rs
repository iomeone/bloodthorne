use std::collections::HashMap;

#[derive(Clone)]
pub struct PropertySerializer {
    // decode
    // decode container
    is_array: bool,
    length: u32,
    array_serializer: Box<PropertySerializer>,
    name: String,
}

pub struct PropertySerializerTable {
    serializers: HashMap<String, PropertySerializer>,
}

impl PropertySerializerTable {
    pub fn new() -> PropertySerializerTable {
        PropertySerializerTable { serializers: HashMap::new() }
    }
}