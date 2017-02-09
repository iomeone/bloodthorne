// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CDemoFileHeader {
    // message fields
    demo_file_stamp: ::protobuf::SingularField<::std::string::String>,
    network_protocol: ::std::option::Option<i32>,
    server_name: ::protobuf::SingularField<::std::string::String>,
    client_name: ::protobuf::SingularField<::std::string::String>,
    map_name: ::protobuf::SingularField<::std::string::String>,
    game_directory: ::protobuf::SingularField<::std::string::String>,
    fullpackets_version: ::std::option::Option<i32>,
    allow_clientside_entities: ::std::option::Option<bool>,
    allow_clientside_particles: ::std::option::Option<bool>,
    addons: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoFileHeader {}

impl CDemoFileHeader {
    pub fn new() -> CDemoFileHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoFileHeader {
        static mut instance: ::protobuf::lazy::Lazy<CDemoFileHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoFileHeader,
        };
        unsafe {
            instance.get(CDemoFileHeader::new)
        }
    }

    // required string demo_file_stamp = 1;

    pub fn clear_demo_file_stamp(&mut self) {
        self.demo_file_stamp.clear();
    }

    pub fn has_demo_file_stamp(&self) -> bool {
        self.demo_file_stamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_demo_file_stamp(&mut self, v: ::std::string::String) {
        self.demo_file_stamp = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_demo_file_stamp(&mut self) -> &mut ::std::string::String {
        if self.demo_file_stamp.is_none() {
            self.demo_file_stamp.set_default();
        };
        self.demo_file_stamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_demo_file_stamp(&mut self) -> ::std::string::String {
        self.demo_file_stamp.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_demo_file_stamp(&self) -> &str {
        match self.demo_file_stamp.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_demo_file_stamp_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.demo_file_stamp
    }

    fn mut_demo_file_stamp_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.demo_file_stamp
    }

    // optional int32 network_protocol = 2;

    pub fn clear_network_protocol(&mut self) {
        self.network_protocol = ::std::option::Option::None;
    }

    pub fn has_network_protocol(&self) -> bool {
        self.network_protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network_protocol(&mut self, v: i32) {
        self.network_protocol = ::std::option::Option::Some(v);
    }

    pub fn get_network_protocol(&self) -> i32 {
        self.network_protocol.unwrap_or(0)
    }

    fn get_network_protocol_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.network_protocol
    }

    fn mut_network_protocol_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.network_protocol
    }

    // optional string server_name = 3;

    pub fn clear_server_name(&mut self) {
        self.server_name.clear();
    }

    pub fn has_server_name(&self) -> bool {
        self.server_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_name(&mut self, v: ::std::string::String) {
        self.server_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_name(&mut self) -> &mut ::std::string::String {
        if self.server_name.is_none() {
            self.server_name.set_default();
        };
        self.server_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_name(&mut self) -> ::std::string::String {
        self.server_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_server_name(&self) -> &str {
        match self.server_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_server_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.server_name
    }

    fn mut_server_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.server_name
    }

    // optional string client_name = 4;

    pub fn clear_client_name(&mut self) {
        self.client_name.clear();
    }

    pub fn has_client_name(&self) -> bool {
        self.client_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_name(&mut self, v: ::std::string::String) {
        self.client_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_name(&mut self) -> &mut ::std::string::String {
        if self.client_name.is_none() {
            self.client_name.set_default();
        };
        self.client_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_name(&mut self) -> ::std::string::String {
        self.client_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_client_name(&self) -> &str {
        match self.client_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_client_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.client_name
    }

    fn mut_client_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.client_name
    }

    // optional string map_name = 5;

    pub fn clear_map_name(&mut self) {
        self.map_name.clear();
    }

    pub fn has_map_name(&self) -> bool {
        self.map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_name(&mut self, v: ::std::string::String) {
        self.map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_name(&mut self) -> &mut ::std::string::String {
        if self.map_name.is_none() {
            self.map_name.set_default();
        };
        self.map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_name(&mut self) -> ::std::string::String {
        self.map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_name(&self) -> &str {
        match self.map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map_name
    }

    fn mut_map_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map_name
    }

    // optional string game_directory = 6;

    pub fn clear_game_directory(&mut self) {
        self.game_directory.clear();
    }

    pub fn has_game_directory(&self) -> bool {
        self.game_directory.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_directory(&mut self, v: ::std::string::String) {
        self.game_directory = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_directory(&mut self) -> &mut ::std::string::String {
        if self.game_directory.is_none() {
            self.game_directory.set_default();
        };
        self.game_directory.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_directory(&mut self) -> ::std::string::String {
        self.game_directory.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_directory(&self) -> &str {
        match self.game_directory.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_directory_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_directory
    }

    fn mut_game_directory_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_directory
    }

    // optional int32 fullpackets_version = 7;

    pub fn clear_fullpackets_version(&mut self) {
        self.fullpackets_version = ::std::option::Option::None;
    }

    pub fn has_fullpackets_version(&self) -> bool {
        self.fullpackets_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullpackets_version(&mut self, v: i32) {
        self.fullpackets_version = ::std::option::Option::Some(v);
    }

    pub fn get_fullpackets_version(&self) -> i32 {
        self.fullpackets_version.unwrap_or(0)
    }

    fn get_fullpackets_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.fullpackets_version
    }

    fn mut_fullpackets_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.fullpackets_version
    }

    // optional bool allow_clientside_entities = 8;

    pub fn clear_allow_clientside_entities(&mut self) {
        self.allow_clientside_entities = ::std::option::Option::None;
    }

    pub fn has_allow_clientside_entities(&self) -> bool {
        self.allow_clientside_entities.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_clientside_entities(&mut self, v: bool) {
        self.allow_clientside_entities = ::std::option::Option::Some(v);
    }

    pub fn get_allow_clientside_entities(&self) -> bool {
        self.allow_clientside_entities.unwrap_or(false)
    }

    fn get_allow_clientside_entities_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_clientside_entities
    }

    fn mut_allow_clientside_entities_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_clientside_entities
    }

    // optional bool allow_clientside_particles = 9;

    pub fn clear_allow_clientside_particles(&mut self) {
        self.allow_clientside_particles = ::std::option::Option::None;
    }

    pub fn has_allow_clientside_particles(&self) -> bool {
        self.allow_clientside_particles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_clientside_particles(&mut self, v: bool) {
        self.allow_clientside_particles = ::std::option::Option::Some(v);
    }

    pub fn get_allow_clientside_particles(&self) -> bool {
        self.allow_clientside_particles.unwrap_or(false)
    }

    fn get_allow_clientside_particles_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_clientside_particles
    }

    fn mut_allow_clientside_particles_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_clientside_particles
    }

    // optional string addons = 10;

    pub fn clear_addons(&mut self) {
        self.addons.clear();
    }

    pub fn has_addons(&self) -> bool {
        self.addons.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addons(&mut self, v: ::std::string::String) {
        self.addons = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addons(&mut self) -> &mut ::std::string::String {
        if self.addons.is_none() {
            self.addons.set_default();
        };
        self.addons.as_mut().unwrap()
    }

    // Take field
    pub fn take_addons(&mut self) -> ::std::string::String {
        self.addons.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_addons(&self) -> &str {
        match self.addons.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_addons_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.addons
    }

    fn mut_addons_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.addons
    }
}

impl ::protobuf::Message for CDemoFileHeader {
    fn is_initialized(&self) -> bool {
        if self.demo_file_stamp.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.demo_file_stamp)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.network_protocol = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.server_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_directory)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.fullpackets_version = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allow_clientside_entities = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allow_clientside_particles = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.addons)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.demo_file_stamp.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.network_protocol {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.server_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.client_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.game_directory.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.fullpackets_version {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.allow_clientside_entities {
            my_size += 2;
        };
        if let Some(v) = self.allow_clientside_particles {
            my_size += 2;
        };
        if let Some(v) = self.addons.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.demo_file_stamp.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.network_protocol {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.server_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.client_name.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.map_name.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.game_directory.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.fullpackets_version {
            os.write_int32(7, v)?;
        };
        if let Some(v) = self.allow_clientside_entities {
            os.write_bool(8, v)?;
        };
        if let Some(v) = self.allow_clientside_particles {
            os.write_bool(9, v)?;
        };
        if let Some(v) = self.addons.as_ref() {
            os.write_string(10, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoFileHeader {
    fn new() -> CDemoFileHeader {
        CDemoFileHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoFileHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "demo_file_stamp",
                    CDemoFileHeader::get_demo_file_stamp_for_reflect,
                    CDemoFileHeader::mut_demo_file_stamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "network_protocol",
                    CDemoFileHeader::get_network_protocol_for_reflect,
                    CDemoFileHeader::mut_network_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "server_name",
                    CDemoFileHeader::get_server_name_for_reflect,
                    CDemoFileHeader::mut_server_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "client_name",
                    CDemoFileHeader::get_client_name_for_reflect,
                    CDemoFileHeader::mut_client_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_name",
                    CDemoFileHeader::get_map_name_for_reflect,
                    CDemoFileHeader::mut_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_directory",
                    CDemoFileHeader::get_game_directory_for_reflect,
                    CDemoFileHeader::mut_game_directory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "fullpackets_version",
                    CDemoFileHeader::get_fullpackets_version_for_reflect,
                    CDemoFileHeader::mut_fullpackets_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_clientside_entities",
                    CDemoFileHeader::get_allow_clientside_entities_for_reflect,
                    CDemoFileHeader::mut_allow_clientside_entities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_clientside_particles",
                    CDemoFileHeader::get_allow_clientside_particles_for_reflect,
                    CDemoFileHeader::mut_allow_clientside_particles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addons",
                    CDemoFileHeader::get_addons_for_reflect,
                    CDemoFileHeader::mut_addons_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoFileHeader>(
                    "CDemoFileHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoFileHeader {
    fn clear(&mut self) {
        self.clear_demo_file_stamp();
        self.clear_network_protocol();
        self.clear_server_name();
        self.clear_client_name();
        self.clear_map_name();
        self.clear_game_directory();
        self.clear_fullpackets_version();
        self.clear_allow_clientside_entities();
        self.clear_allow_clientside_particles();
        self.clear_addons();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoFileHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoFileHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGameInfo {
    // message fields
    dota: ::protobuf::SingularPtrField<CGameInfo_CDotaGameInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGameInfo {}

impl CGameInfo {
    pub fn new() -> CGameInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGameInfo {
        static mut instance: ::protobuf::lazy::Lazy<CGameInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGameInfo,
        };
        unsafe {
            instance.get(CGameInfo::new)
        }
    }

    // optional .dota.CGameInfo.CDotaGameInfo dota = 4;

    pub fn clear_dota(&mut self) {
        self.dota.clear();
    }

    pub fn has_dota(&self) -> bool {
        self.dota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota(&mut self, v: CGameInfo_CDotaGameInfo) {
        self.dota = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dota(&mut self) -> &mut CGameInfo_CDotaGameInfo {
        if self.dota.is_none() {
            self.dota.set_default();
        };
        self.dota.as_mut().unwrap()
    }

    // Take field
    pub fn take_dota(&mut self) -> CGameInfo_CDotaGameInfo {
        self.dota.take().unwrap_or_else(|| CGameInfo_CDotaGameInfo::new())
    }

    pub fn get_dota(&self) -> &CGameInfo_CDotaGameInfo {
        self.dota.as_ref().unwrap_or_else(|| CGameInfo_CDotaGameInfo::default_instance())
    }

    fn get_dota_for_reflect(&self) -> &::protobuf::SingularPtrField<CGameInfo_CDotaGameInfo> {
        &self.dota
    }

    fn mut_dota_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CGameInfo_CDotaGameInfo> {
        &mut self.dota
    }
}

impl ::protobuf::Message for CGameInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dota)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.dota.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dota.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGameInfo {
    fn new() -> CGameInfo {
        CGameInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGameInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGameInfo_CDotaGameInfo>>(
                    "dota",
                    CGameInfo::get_dota_for_reflect,
                    CGameInfo::mut_dota_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGameInfo>(
                    "CGameInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGameInfo {
    fn clear(&mut self) {
        self.clear_dota();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGameInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGameInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGameInfo_CDotaGameInfo {
    // message fields
    match_id: ::std::option::Option<u64>,
    game_mode: ::std::option::Option<i32>,
    game_winner: ::std::option::Option<i32>,
    player_info: ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo>,
    leagueid: ::std::option::Option<u32>,
    picks_bans: ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CHeroSelectEvent>,
    radiant_team_id: ::std::option::Option<u32>,
    dire_team_id: ::std::option::Option<u32>,
    radiant_team_tag: ::protobuf::SingularField<::std::string::String>,
    dire_team_tag: ::protobuf::SingularField<::std::string::String>,
    end_time: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGameInfo_CDotaGameInfo {}

impl CGameInfo_CDotaGameInfo {
    pub fn new() -> CGameInfo_CDotaGameInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGameInfo_CDotaGameInfo {
        static mut instance: ::protobuf::lazy::Lazy<CGameInfo_CDotaGameInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGameInfo_CDotaGameInfo,
        };
        unsafe {
            instance.get(CGameInfo_CDotaGameInfo::new)
        }
    }

    // optional uint64 match_id = 1;

    pub fn clear_match_id(&mut self) {
        self.match_id = ::std::option::Option::None;
    }

    pub fn has_match_id(&self) -> bool {
        self.match_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_id(&mut self, v: u64) {
        self.match_id = ::std::option::Option::Some(v);
    }

    pub fn get_match_id(&self) -> u64 {
        self.match_id.unwrap_or(0)
    }

    fn get_match_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.match_id
    }

    fn mut_match_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.match_id
    }

    // optional int32 game_mode = 2;

    pub fn clear_game_mode(&mut self) {
        self.game_mode = ::std::option::Option::None;
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: i32) {
        self.game_mode = ::std::option::Option::Some(v);
    }

    pub fn get_game_mode(&self) -> i32 {
        self.game_mode.unwrap_or(0)
    }

    fn get_game_mode_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.game_mode
    }

    fn mut_game_mode_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.game_mode
    }

    // optional int32 game_winner = 3;

    pub fn clear_game_winner(&mut self) {
        self.game_winner = ::std::option::Option::None;
    }

    pub fn has_game_winner(&self) -> bool {
        self.game_winner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_winner(&mut self, v: i32) {
        self.game_winner = ::std::option::Option::Some(v);
    }

    pub fn get_game_winner(&self) -> i32 {
        self.game_winner.unwrap_or(0)
    }

    fn get_game_winner_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.game_winner
    }

    fn mut_game_winner_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.game_winner
    }

    // repeated .dota.CGameInfo.CDotaGameInfo.CPlayerInfo player_info = 4;

    pub fn clear_player_info(&mut self) {
        self.player_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_info(&mut self, v: ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo>) {
        self.player_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_info(&mut self) -> &mut ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo> {
        &mut self.player_info
    }

    // Take field
    pub fn take_player_info(&mut self) -> ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo> {
        ::std::mem::replace(&mut self.player_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_info(&self) -> &[CGameInfo_CDotaGameInfo_CPlayerInfo] {
        &self.player_info
    }

    fn get_player_info_for_reflect(&self) -> &::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo> {
        &self.player_info
    }

    fn mut_player_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CPlayerInfo> {
        &mut self.player_info
    }

    // optional uint32 leagueid = 5;

    pub fn clear_leagueid(&mut self) {
        self.leagueid = ::std::option::Option::None;
    }

    pub fn has_leagueid(&self) -> bool {
        self.leagueid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leagueid(&mut self, v: u32) {
        self.leagueid = ::std::option::Option::Some(v);
    }

    pub fn get_leagueid(&self) -> u32 {
        self.leagueid.unwrap_or(0)
    }

    fn get_leagueid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.leagueid
    }

    fn mut_leagueid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.leagueid
    }

    // repeated .dota.CGameInfo.CDotaGameInfo.CHeroSelectEvent picks_bans = 6;

    pub fn clear_picks_bans(&mut self) {
        self.picks_bans.clear();
    }

    // Param is passed by value, moved
    pub fn set_picks_bans(&mut self, v: ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CHeroSelectEvent>) {
        self.picks_bans = v;
    }

    // Mutable pointer to the field.
    pub fn mut_picks_bans(&mut self) -> &mut ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CHeroSelectEvent> {
        &mut self.picks_bans
    }

    // Take field
    pub fn take_picks_bans(&mut self) -> ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CHeroSelectEvent> {
        ::std::mem::replace(&mut self.picks_bans, ::protobuf::RepeatedField::new())
    }

    pub fn get_picks_bans(&self) -> &[CGameInfo_CDotaGameInfo_CHeroSelectEvent] {
        &self.picks_bans
    }

    fn get_picks_bans_for_reflect(&self) -> &::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CHeroSelectEvent> {
        &self.picks_bans
    }

    fn mut_picks_bans_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CGameInfo_CDotaGameInfo_CHeroSelectEvent> {
        &mut self.picks_bans
    }

    // optional uint32 radiant_team_id = 7;

    pub fn clear_radiant_team_id(&mut self) {
        self.radiant_team_id = ::std::option::Option::None;
    }

    pub fn has_radiant_team_id(&self) -> bool {
        self.radiant_team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_team_id(&mut self, v: u32) {
        self.radiant_team_id = ::std::option::Option::Some(v);
    }

    pub fn get_radiant_team_id(&self) -> u32 {
        self.radiant_team_id.unwrap_or(0)
    }

    fn get_radiant_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.radiant_team_id
    }

    fn mut_radiant_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.radiant_team_id
    }

    // optional uint32 dire_team_id = 8;

    pub fn clear_dire_team_id(&mut self) {
        self.dire_team_id = ::std::option::Option::None;
    }

    pub fn has_dire_team_id(&self) -> bool {
        self.dire_team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dire_team_id(&mut self, v: u32) {
        self.dire_team_id = ::std::option::Option::Some(v);
    }

    pub fn get_dire_team_id(&self) -> u32 {
        self.dire_team_id.unwrap_or(0)
    }

    fn get_dire_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dire_team_id
    }

    fn mut_dire_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dire_team_id
    }

    // optional string radiant_team_tag = 9;

    pub fn clear_radiant_team_tag(&mut self) {
        self.radiant_team_tag.clear();
    }

    pub fn has_radiant_team_tag(&self) -> bool {
        self.radiant_team_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_team_tag(&mut self, v: ::std::string::String) {
        self.radiant_team_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_radiant_team_tag(&mut self) -> &mut ::std::string::String {
        if self.radiant_team_tag.is_none() {
            self.radiant_team_tag.set_default();
        };
        self.radiant_team_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_radiant_team_tag(&mut self) -> ::std::string::String {
        self.radiant_team_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_radiant_team_tag(&self) -> &str {
        match self.radiant_team_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_radiant_team_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.radiant_team_tag
    }

    fn mut_radiant_team_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.radiant_team_tag
    }

    // optional string dire_team_tag = 10;

    pub fn clear_dire_team_tag(&mut self) {
        self.dire_team_tag.clear();
    }

    pub fn has_dire_team_tag(&self) -> bool {
        self.dire_team_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dire_team_tag(&mut self, v: ::std::string::String) {
        self.dire_team_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dire_team_tag(&mut self) -> &mut ::std::string::String {
        if self.dire_team_tag.is_none() {
            self.dire_team_tag.set_default();
        };
        self.dire_team_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_dire_team_tag(&mut self) -> ::std::string::String {
        self.dire_team_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_dire_team_tag(&self) -> &str {
        match self.dire_team_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_dire_team_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.dire_team_tag
    }

    fn mut_dire_team_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.dire_team_tag
    }

    // optional uint32 end_time = 11;

    pub fn clear_end_time(&mut self) {
        self.end_time = ::std::option::Option::None;
    }

    pub fn has_end_time(&self) -> bool {
        self.end_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_time(&mut self, v: u32) {
        self.end_time = ::std::option::Option::Some(v);
    }

    pub fn get_end_time(&self) -> u32 {
        self.end_time.unwrap_or(0)
    }

    fn get_end_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.end_time
    }

    fn mut_end_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.end_time
    }
}

impl ::protobuf::Message for CGameInfo_CDotaGameInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.game_winner = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_info)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.leagueid = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.picks_bans)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.radiant_team_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.dire_team_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.radiant_team_tag)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.dire_team_tag)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.end_time = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_winner {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.player_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.leagueid {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.picks_bans {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.radiant_team_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dire_team_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radiant_team_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        };
        if let Some(v) = self.dire_team_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        };
        if let Some(v) = self.end_time {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.match_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.game_mode {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.game_winner {
            os.write_int32(3, v)?;
        };
        for v in &self.player_info {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.leagueid {
            os.write_uint32(5, v)?;
        };
        for v in &self.picks_bans {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.radiant_team_id {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.dire_team_id {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.radiant_team_tag.as_ref() {
            os.write_string(9, &v)?;
        };
        if let Some(v) = self.dire_team_tag.as_ref() {
            os.write_string(10, &v)?;
        };
        if let Some(v) = self.end_time {
            os.write_uint32(11, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGameInfo_CDotaGameInfo {
    fn new() -> CGameInfo_CDotaGameInfo {
        CGameInfo_CDotaGameInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGameInfo_CDotaGameInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CGameInfo_CDotaGameInfo::get_match_id_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "game_mode",
                    CGameInfo_CDotaGameInfo::get_game_mode_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "game_winner",
                    CGameInfo_CDotaGameInfo::get_game_winner_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_game_winner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGameInfo_CDotaGameInfo_CPlayerInfo>>(
                    "player_info",
                    CGameInfo_CDotaGameInfo::get_player_info_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_player_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leagueid",
                    CGameInfo_CDotaGameInfo::get_leagueid_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_leagueid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGameInfo_CDotaGameInfo_CHeroSelectEvent>>(
                    "picks_bans",
                    CGameInfo_CDotaGameInfo::get_picks_bans_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_picks_bans_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "radiant_team_id",
                    CGameInfo_CDotaGameInfo::get_radiant_team_id_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_radiant_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dire_team_id",
                    CGameInfo_CDotaGameInfo::get_dire_team_id_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_dire_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "radiant_team_tag",
                    CGameInfo_CDotaGameInfo::get_radiant_team_tag_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_radiant_team_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dire_team_tag",
                    CGameInfo_CDotaGameInfo::get_dire_team_tag_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_dire_team_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "end_time",
                    CGameInfo_CDotaGameInfo::get_end_time_for_reflect,
                    CGameInfo_CDotaGameInfo::mut_end_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGameInfo_CDotaGameInfo>(
                    "CGameInfo_CDotaGameInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGameInfo_CDotaGameInfo {
    fn clear(&mut self) {
        self.clear_match_id();
        self.clear_game_mode();
        self.clear_game_winner();
        self.clear_player_info();
        self.clear_leagueid();
        self.clear_picks_bans();
        self.clear_radiant_team_id();
        self.clear_dire_team_id();
        self.clear_radiant_team_tag();
        self.clear_dire_team_tag();
        self.clear_end_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGameInfo_CDotaGameInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGameInfo_CDotaGameInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGameInfo_CDotaGameInfo_CPlayerInfo {
    // message fields
    hero_name: ::protobuf::SingularField<::std::string::String>,
    player_name: ::protobuf::SingularField<::std::string::String>,
    is_fake_client: ::std::option::Option<bool>,
    steamid: ::std::option::Option<u64>,
    game_team: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGameInfo_CDotaGameInfo_CPlayerInfo {}

impl CGameInfo_CDotaGameInfo_CPlayerInfo {
    pub fn new() -> CGameInfo_CDotaGameInfo_CPlayerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGameInfo_CDotaGameInfo_CPlayerInfo {
        static mut instance: ::protobuf::lazy::Lazy<CGameInfo_CDotaGameInfo_CPlayerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGameInfo_CDotaGameInfo_CPlayerInfo,
        };
        unsafe {
            instance.get(CGameInfo_CDotaGameInfo_CPlayerInfo::new)
        }
    }

    // optional string hero_name = 1;

    pub fn clear_hero_name(&mut self) {
        self.hero_name.clear();
    }

    pub fn has_hero_name(&self) -> bool {
        self.hero_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_name(&mut self, v: ::std::string::String) {
        self.hero_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero_name(&mut self) -> &mut ::std::string::String {
        if self.hero_name.is_none() {
            self.hero_name.set_default();
        };
        self.hero_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_hero_name(&mut self) -> ::std::string::String {
        self.hero_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hero_name(&self) -> &str {
        match self.hero_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hero_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hero_name
    }

    fn mut_hero_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hero_name
    }

    // optional string player_name = 2;

    pub fn clear_player_name(&mut self) {
        self.player_name.clear();
    }

    pub fn has_player_name(&self) -> bool {
        self.player_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_name(&mut self, v: ::std::string::String) {
        self.player_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_name(&mut self) -> &mut ::std::string::String {
        if self.player_name.is_none() {
            self.player_name.set_default();
        };
        self.player_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_name(&mut self) -> ::std::string::String {
        self.player_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_player_name(&self) -> &str {
        match self.player_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_player_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.player_name
    }

    fn mut_player_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.player_name
    }

    // optional bool is_fake_client = 3;

    pub fn clear_is_fake_client(&mut self) {
        self.is_fake_client = ::std::option::Option::None;
    }

    pub fn has_is_fake_client(&self) -> bool {
        self.is_fake_client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_fake_client(&mut self, v: bool) {
        self.is_fake_client = ::std::option::Option::Some(v);
    }

    pub fn get_is_fake_client(&self) -> bool {
        self.is_fake_client.unwrap_or(false)
    }

    fn get_is_fake_client_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_fake_client
    }

    fn mut_is_fake_client_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_fake_client
    }

    // optional uint64 steamid = 4;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional int32 game_team = 5;

    pub fn clear_game_team(&mut self) {
        self.game_team = ::std::option::Option::None;
    }

    pub fn has_game_team(&self) -> bool {
        self.game_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_team(&mut self, v: i32) {
        self.game_team = ::std::option::Option::Some(v);
    }

    pub fn get_game_team(&self) -> i32 {
        self.game_team.unwrap_or(0)
    }

    fn get_game_team_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.game_team
    }

    fn mut_game_team_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.game_team
    }
}

impl ::protobuf::Message for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hero_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.player_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_fake_client = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.game_team = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.hero_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.player_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.is_fake_client {
            my_size += 2;
        };
        if let Some(v) = self.steamid {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_team {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hero_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.player_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.is_fake_client {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.steamid {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.game_team {
            os.write_int32(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn new() -> CGameInfo_CDotaGameInfo_CPlayerInfo {
        CGameInfo_CDotaGameInfo_CPlayerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGameInfo_CDotaGameInfo_CPlayerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hero_name",
                    CGameInfo_CDotaGameInfo_CPlayerInfo::get_hero_name_for_reflect,
                    CGameInfo_CDotaGameInfo_CPlayerInfo::mut_hero_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "player_name",
                    CGameInfo_CDotaGameInfo_CPlayerInfo::get_player_name_for_reflect,
                    CGameInfo_CDotaGameInfo_CPlayerInfo::mut_player_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_fake_client",
                    CGameInfo_CDotaGameInfo_CPlayerInfo::get_is_fake_client_for_reflect,
                    CGameInfo_CDotaGameInfo_CPlayerInfo::mut_is_fake_client_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "steamid",
                    CGameInfo_CDotaGameInfo_CPlayerInfo::get_steamid_for_reflect,
                    CGameInfo_CDotaGameInfo_CPlayerInfo::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "game_team",
                    CGameInfo_CDotaGameInfo_CPlayerInfo::get_game_team_for_reflect,
                    CGameInfo_CDotaGameInfo_CPlayerInfo::mut_game_team_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGameInfo_CDotaGameInfo_CPlayerInfo>(
                    "CGameInfo_CDotaGameInfo_CPlayerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn clear(&mut self) {
        self.clear_hero_name();
        self.clear_player_name();
        self.clear_is_fake_client();
        self.clear_steamid();
        self.clear_game_team();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGameInfo_CDotaGameInfo_CPlayerInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGameInfo_CDotaGameInfo_CHeroSelectEvent {
    // message fields
    is_pick: ::std::option::Option<bool>,
    team: ::std::option::Option<u32>,
    hero_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGameInfo_CDotaGameInfo_CHeroSelectEvent {}

impl CGameInfo_CDotaGameInfo_CHeroSelectEvent {
    pub fn new() -> CGameInfo_CDotaGameInfo_CHeroSelectEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGameInfo_CDotaGameInfo_CHeroSelectEvent {
        static mut instance: ::protobuf::lazy::Lazy<CGameInfo_CDotaGameInfo_CHeroSelectEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGameInfo_CDotaGameInfo_CHeroSelectEvent,
        };
        unsafe {
            instance.get(CGameInfo_CDotaGameInfo_CHeroSelectEvent::new)
        }
    }

    // optional bool is_pick = 1;

    pub fn clear_is_pick(&mut self) {
        self.is_pick = ::std::option::Option::None;
    }

    pub fn has_is_pick(&self) -> bool {
        self.is_pick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_pick(&mut self, v: bool) {
        self.is_pick = ::std::option::Option::Some(v);
    }

    pub fn get_is_pick(&self) -> bool {
        self.is_pick.unwrap_or(false)
    }

    fn get_is_pick_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_pick
    }

    fn mut_is_pick_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_pick
    }

    // optional uint32 team = 2;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: u32) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> u32 {
        self.team.unwrap_or(0)
    }

    fn get_team_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team
    }

    // optional uint32 hero_id = 3;

    pub fn clear_hero_id(&mut self) {
        self.hero_id = ::std::option::Option::None;
    }

    pub fn has_hero_id(&self) -> bool {
        self.hero_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_id(&mut self, v: u32) {
        self.hero_id = ::std::option::Option::Some(v);
    }

    pub fn get_hero_id(&self) -> u32 {
        self.hero_id.unwrap_or(0)
    }

    fn get_hero_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hero_id
    }

    fn mut_hero_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hero_id
    }
}

impl ::protobuf::Message for CGameInfo_CDotaGameInfo_CHeroSelectEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_pick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.is_pick {
            my_size += 2;
        };
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_pick {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.team {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.hero_id {
            os.write_uint32(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGameInfo_CDotaGameInfo_CHeroSelectEvent {
    fn new() -> CGameInfo_CDotaGameInfo_CHeroSelectEvent {
        CGameInfo_CDotaGameInfo_CHeroSelectEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGameInfo_CDotaGameInfo_CHeroSelectEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_pick",
                    CGameInfo_CDotaGameInfo_CHeroSelectEvent::get_is_pick_for_reflect,
                    CGameInfo_CDotaGameInfo_CHeroSelectEvent::mut_is_pick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team",
                    CGameInfo_CDotaGameInfo_CHeroSelectEvent::get_team_for_reflect,
                    CGameInfo_CDotaGameInfo_CHeroSelectEvent::mut_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CGameInfo_CDotaGameInfo_CHeroSelectEvent::get_hero_id_for_reflect,
                    CGameInfo_CDotaGameInfo_CHeroSelectEvent::mut_hero_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGameInfo_CDotaGameInfo_CHeroSelectEvent>(
                    "CGameInfo_CDotaGameInfo_CHeroSelectEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGameInfo_CDotaGameInfo_CHeroSelectEvent {
    fn clear(&mut self) {
        self.clear_is_pick();
        self.clear_team();
        self.clear_hero_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGameInfo_CDotaGameInfo_CHeroSelectEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGameInfo_CDotaGameInfo_CHeroSelectEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoFileInfo {
    // message fields
    playback_time: ::std::option::Option<f32>,
    playback_ticks: ::std::option::Option<i32>,
    playback_frames: ::std::option::Option<i32>,
    game_info: ::protobuf::SingularPtrField<CGameInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoFileInfo {}

impl CDemoFileInfo {
    pub fn new() -> CDemoFileInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoFileInfo {
        static mut instance: ::protobuf::lazy::Lazy<CDemoFileInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoFileInfo,
        };
        unsafe {
            instance.get(CDemoFileInfo::new)
        }
    }

    // optional float playback_time = 1;

    pub fn clear_playback_time(&mut self) {
        self.playback_time = ::std::option::Option::None;
    }

    pub fn has_playback_time(&self) -> bool {
        self.playback_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playback_time(&mut self, v: f32) {
        self.playback_time = ::std::option::Option::Some(v);
    }

    pub fn get_playback_time(&self) -> f32 {
        self.playback_time.unwrap_or(0.)
    }

    fn get_playback_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.playback_time
    }

    fn mut_playback_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.playback_time
    }

    // optional int32 playback_ticks = 2;

    pub fn clear_playback_ticks(&mut self) {
        self.playback_ticks = ::std::option::Option::None;
    }

    pub fn has_playback_ticks(&self) -> bool {
        self.playback_ticks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playback_ticks(&mut self, v: i32) {
        self.playback_ticks = ::std::option::Option::Some(v);
    }

    pub fn get_playback_ticks(&self) -> i32 {
        self.playback_ticks.unwrap_or(0)
    }

    fn get_playback_ticks_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.playback_ticks
    }

    fn mut_playback_ticks_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.playback_ticks
    }

    // optional int32 playback_frames = 3;

    pub fn clear_playback_frames(&mut self) {
        self.playback_frames = ::std::option::Option::None;
    }

    pub fn has_playback_frames(&self) -> bool {
        self.playback_frames.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playback_frames(&mut self, v: i32) {
        self.playback_frames = ::std::option::Option::Some(v);
    }

    pub fn get_playback_frames(&self) -> i32 {
        self.playback_frames.unwrap_or(0)
    }

    fn get_playback_frames_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.playback_frames
    }

    fn mut_playback_frames_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.playback_frames
    }

    // optional .dota.CGameInfo game_info = 4;

    pub fn clear_game_info(&mut self) {
        self.game_info.clear();
    }

    pub fn has_game_info(&self) -> bool {
        self.game_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_info(&mut self, v: CGameInfo) {
        self.game_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_info(&mut self) -> &mut CGameInfo {
        if self.game_info.is_none() {
            self.game_info.set_default();
        };
        self.game_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_info(&mut self) -> CGameInfo {
        self.game_info.take().unwrap_or_else(|| CGameInfo::new())
    }

    pub fn get_game_info(&self) -> &CGameInfo {
        self.game_info.as_ref().unwrap_or_else(|| CGameInfo::default_instance())
    }

    fn get_game_info_for_reflect(&self) -> &::protobuf::SingularPtrField<CGameInfo> {
        &self.game_info
    }

    fn mut_game_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CGameInfo> {
        &mut self.game_info
    }
}

impl ::protobuf::Message for CDemoFileInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.playback_time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.playback_ticks = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.playback_frames = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.game_info)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.playback_time {
            my_size += 5;
        };
        if let Some(v) = self.playback_ticks {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.playback_frames {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.playback_time {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.playback_ticks {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.playback_frames {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.game_info.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoFileInfo {
    fn new() -> CDemoFileInfo {
        CDemoFileInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoFileInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "playback_time",
                    CDemoFileInfo::get_playback_time_for_reflect,
                    CDemoFileInfo::mut_playback_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "playback_ticks",
                    CDemoFileInfo::get_playback_ticks_for_reflect,
                    CDemoFileInfo::mut_playback_ticks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "playback_frames",
                    CDemoFileInfo::get_playback_frames_for_reflect,
                    CDemoFileInfo::mut_playback_frames_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGameInfo>>(
                    "game_info",
                    CDemoFileInfo::get_game_info_for_reflect,
                    CDemoFileInfo::mut_game_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoFileInfo>(
                    "CDemoFileInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoFileInfo {
    fn clear(&mut self) {
        self.clear_playback_time();
        self.clear_playback_ticks();
        self.clear_playback_frames();
        self.clear_game_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoFileInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoFileInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoPacket {
    // message fields
    sequence_in: ::std::option::Option<i32>,
    sequence_out_ack: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoPacket {}

impl CDemoPacket {
    pub fn new() -> CDemoPacket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoPacket {
        static mut instance: ::protobuf::lazy::Lazy<CDemoPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoPacket,
        };
        unsafe {
            instance.get(CDemoPacket::new)
        }
    }

    // optional int32 sequence_in = 1;

    pub fn clear_sequence_in(&mut self) {
        self.sequence_in = ::std::option::Option::None;
    }

    pub fn has_sequence_in(&self) -> bool {
        self.sequence_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_in(&mut self, v: i32) {
        self.sequence_in = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_in(&self) -> i32 {
        self.sequence_in.unwrap_or(0)
    }

    fn get_sequence_in_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_in
    }

    fn mut_sequence_in_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_in
    }

    // optional int32 sequence_out_ack = 2;

    pub fn clear_sequence_out_ack(&mut self) {
        self.sequence_out_ack = ::std::option::Option::None;
    }

    pub fn has_sequence_out_ack(&self) -> bool {
        self.sequence_out_ack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_out_ack(&mut self, v: i32) {
        self.sequence_out_ack = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_out_ack(&self) -> i32 {
        self.sequence_out_ack.unwrap_or(0)
    }

    fn get_sequence_out_ack_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_out_ack
    }

    fn mut_sequence_out_ack_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_out_ack
    }

    // optional bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for CDemoPacket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.sequence_in = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.sequence_out_ack = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.sequence_in {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sequence_out_ack {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sequence_in {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.sequence_out_ack {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoPacket {
    fn new() -> CDemoPacket {
        CDemoPacket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoPacket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_in",
                    CDemoPacket::get_sequence_in_for_reflect,
                    CDemoPacket::mut_sequence_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_out_ack",
                    CDemoPacket::get_sequence_out_ack_for_reflect,
                    CDemoPacket::mut_sequence_out_ack_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CDemoPacket::get_data_for_reflect,
                    CDemoPacket::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoPacket>(
                    "CDemoPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoPacket {
    fn clear(&mut self) {
        self.clear_sequence_in();
        self.clear_sequence_out_ack();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoPacket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoFullPacket {
    // message fields
    string_table: ::protobuf::SingularPtrField<CDemoStringTables>,
    packet: ::protobuf::SingularPtrField<CDemoPacket>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoFullPacket {}

impl CDemoFullPacket {
    pub fn new() -> CDemoFullPacket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoFullPacket {
        static mut instance: ::protobuf::lazy::Lazy<CDemoFullPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoFullPacket,
        };
        unsafe {
            instance.get(CDemoFullPacket::new)
        }
    }

    // optional .dota.CDemoStringTables string_table = 1;

    pub fn clear_string_table(&mut self) {
        self.string_table.clear();
    }

    pub fn has_string_table(&self) -> bool {
        self.string_table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_table(&mut self, v: CDemoStringTables) {
        self.string_table = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_table(&mut self) -> &mut CDemoStringTables {
        if self.string_table.is_none() {
            self.string_table.set_default();
        };
        self.string_table.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_table(&mut self) -> CDemoStringTables {
        self.string_table.take().unwrap_or_else(|| CDemoStringTables::new())
    }

    pub fn get_string_table(&self) -> &CDemoStringTables {
        self.string_table.as_ref().unwrap_or_else(|| CDemoStringTables::default_instance())
    }

    fn get_string_table_for_reflect(&self) -> &::protobuf::SingularPtrField<CDemoStringTables> {
        &self.string_table
    }

    fn mut_string_table_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CDemoStringTables> {
        &mut self.string_table
    }

    // optional .dota.CDemoPacket packet = 2;

    pub fn clear_packet(&mut self) {
        self.packet.clear();
    }

    pub fn has_packet(&self) -> bool {
        self.packet.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packet(&mut self, v: CDemoPacket) {
        self.packet = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packet(&mut self) -> &mut CDemoPacket {
        if self.packet.is_none() {
            self.packet.set_default();
        };
        self.packet.as_mut().unwrap()
    }

    // Take field
    pub fn take_packet(&mut self) -> CDemoPacket {
        self.packet.take().unwrap_or_else(|| CDemoPacket::new())
    }

    pub fn get_packet(&self) -> &CDemoPacket {
        self.packet.as_ref().unwrap_or_else(|| CDemoPacket::default_instance())
    }

    fn get_packet_for_reflect(&self) -> &::protobuf::SingularPtrField<CDemoPacket> {
        &self.packet
    }

    fn mut_packet_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CDemoPacket> {
        &mut self.packet
    }
}

impl ::protobuf::Message for CDemoFullPacket {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.string_table)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.packet)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.string_table.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.packet.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.string_table.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.packet.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoFullPacket {
    fn new() -> CDemoFullPacket {
        CDemoFullPacket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoFullPacket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDemoStringTables>>(
                    "string_table",
                    CDemoFullPacket::get_string_table_for_reflect,
                    CDemoFullPacket::mut_string_table_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDemoPacket>>(
                    "packet",
                    CDemoFullPacket::get_packet_for_reflect,
                    CDemoFullPacket::mut_packet_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoFullPacket>(
                    "CDemoFullPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoFullPacket {
    fn clear(&mut self) {
        self.clear_string_table();
        self.clear_packet();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoFullPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoFullPacket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoSaveGame {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    steam_id: ::std::option::Option<u64>,
    signature: ::std::option::Option<u64>,
    version: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoSaveGame {}

impl CDemoSaveGame {
    pub fn new() -> CDemoSaveGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoSaveGame {
        static mut instance: ::protobuf::lazy::Lazy<CDemoSaveGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoSaveGame,
        };
        unsafe {
            instance.get(CDemoSaveGame::new)
        }
    }

    // optional bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }

    // optional fixed64 steam_id = 2;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional fixed64 signature = 3;

    pub fn clear_signature(&mut self) {
        self.signature = ::std::option::Option::None;
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: u64) {
        self.signature = ::std::option::Option::Some(v);
    }

    pub fn get_signature(&self) -> u64 {
        self.signature.unwrap_or(0)
    }

    fn get_signature_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.signature
    }

    // optional int32 version = 4;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> i32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.version
    }
}

impl ::protobuf::Message for CDemoSaveGame {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.signature = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.steam_id {
            my_size += 9;
        };
        if let Some(v) = self.signature {
            my_size += 9;
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.steam_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.signature {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.version {
            os.write_int32(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoSaveGame {
    fn new() -> CDemoSaveGame {
        CDemoSaveGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoSaveGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CDemoSaveGame::get_data_for_reflect,
                    CDemoSaveGame::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CDemoSaveGame::get_steam_id_for_reflect,
                    CDemoSaveGame::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "signature",
                    CDemoSaveGame::get_signature_for_reflect,
                    CDemoSaveGame::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    CDemoSaveGame::get_version_for_reflect,
                    CDemoSaveGame::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoSaveGame>(
                    "CDemoSaveGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoSaveGame {
    fn clear(&mut self) {
        self.clear_data();
        self.clear_steam_id();
        self.clear_signature();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoSaveGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoSaveGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoSyncTick {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoSyncTick {}

impl CDemoSyncTick {
    pub fn new() -> CDemoSyncTick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoSyncTick {
        static mut instance: ::protobuf::lazy::Lazy<CDemoSyncTick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoSyncTick,
        };
        unsafe {
            instance.get(CDemoSyncTick::new)
        }
    }
}

impl ::protobuf::Message for CDemoSyncTick {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoSyncTick {
    fn new() -> CDemoSyncTick {
        CDemoSyncTick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoSyncTick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDemoSyncTick>(
                    "CDemoSyncTick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoSyncTick {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoSyncTick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoSyncTick {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoConsoleCmd {
    // message fields
    cmdstring: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoConsoleCmd {}

impl CDemoConsoleCmd {
    pub fn new() -> CDemoConsoleCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoConsoleCmd {
        static mut instance: ::protobuf::lazy::Lazy<CDemoConsoleCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoConsoleCmd,
        };
        unsafe {
            instance.get(CDemoConsoleCmd::new)
        }
    }

    // optional string cmdstring = 1;

    pub fn clear_cmdstring(&mut self) {
        self.cmdstring.clear();
    }

    pub fn has_cmdstring(&self) -> bool {
        self.cmdstring.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmdstring(&mut self, v: ::std::string::String) {
        self.cmdstring = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmdstring(&mut self) -> &mut ::std::string::String {
        if self.cmdstring.is_none() {
            self.cmdstring.set_default();
        };
        self.cmdstring.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmdstring(&mut self) -> ::std::string::String {
        self.cmdstring.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cmdstring(&self) -> &str {
        match self.cmdstring.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cmdstring_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cmdstring
    }

    fn mut_cmdstring_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cmdstring
    }
}

impl ::protobuf::Message for CDemoConsoleCmd {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cmdstring)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.cmdstring.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmdstring.as_ref() {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoConsoleCmd {
    fn new() -> CDemoConsoleCmd {
        CDemoConsoleCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoConsoleCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cmdstring",
                    CDemoConsoleCmd::get_cmdstring_for_reflect,
                    CDemoConsoleCmd::mut_cmdstring_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoConsoleCmd>(
                    "CDemoConsoleCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoConsoleCmd {
    fn clear(&mut self) {
        self.clear_cmdstring();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoConsoleCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoConsoleCmd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoSendTables {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoSendTables {}

impl CDemoSendTables {
    pub fn new() -> CDemoSendTables {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoSendTables {
        static mut instance: ::protobuf::lazy::Lazy<CDemoSendTables> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoSendTables,
        };
        unsafe {
            instance.get(CDemoSendTables::new)
        }
    }

    // optional bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for CDemoSendTables {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoSendTables {
    fn new() -> CDemoSendTables {
        CDemoSendTables::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoSendTables>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CDemoSendTables::get_data_for_reflect,
                    CDemoSendTables::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoSendTables>(
                    "CDemoSendTables",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoSendTables {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoSendTables {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoSendTables {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoClassInfo {
    // message fields
    classes: ::protobuf::RepeatedField<CDemoClassInfo_class_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoClassInfo {}

impl CDemoClassInfo {
    pub fn new() -> CDemoClassInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoClassInfo {
        static mut instance: ::protobuf::lazy::Lazy<CDemoClassInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoClassInfo,
        };
        unsafe {
            instance.get(CDemoClassInfo::new)
        }
    }

    // repeated .dota.CDemoClassInfo.class_t classes = 1;

    pub fn clear_classes(&mut self) {
        self.classes.clear();
    }

    // Param is passed by value, moved
    pub fn set_classes(&mut self, v: ::protobuf::RepeatedField<CDemoClassInfo_class_t>) {
        self.classes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classes(&mut self) -> &mut ::protobuf::RepeatedField<CDemoClassInfo_class_t> {
        &mut self.classes
    }

    // Take field
    pub fn take_classes(&mut self) -> ::protobuf::RepeatedField<CDemoClassInfo_class_t> {
        ::std::mem::replace(&mut self.classes, ::protobuf::RepeatedField::new())
    }

    pub fn get_classes(&self) -> &[CDemoClassInfo_class_t] {
        &self.classes
    }

    fn get_classes_for_reflect(&self) -> &::protobuf::RepeatedField<CDemoClassInfo_class_t> {
        &self.classes
    }

    fn mut_classes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDemoClassInfo_class_t> {
        &mut self.classes
    }
}

impl ::protobuf::Message for CDemoClassInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.classes)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.classes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.classes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoClassInfo {
    fn new() -> CDemoClassInfo {
        CDemoClassInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoClassInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDemoClassInfo_class_t>>(
                    "classes",
                    CDemoClassInfo::get_classes_for_reflect,
                    CDemoClassInfo::mut_classes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoClassInfo>(
                    "CDemoClassInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoClassInfo {
    fn clear(&mut self) {
        self.clear_classes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoClassInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoClassInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoClassInfo_class_t {
    // message fields
    class_id: ::std::option::Option<i32>,
    network_name: ::protobuf::SingularField<::std::string::String>,
    table_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoClassInfo_class_t {}

impl CDemoClassInfo_class_t {
    pub fn new() -> CDemoClassInfo_class_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoClassInfo_class_t {
        static mut instance: ::protobuf::lazy::Lazy<CDemoClassInfo_class_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoClassInfo_class_t,
        };
        unsafe {
            instance.get(CDemoClassInfo_class_t::new)
        }
    }

    // optional int32 class_id = 1;

    pub fn clear_class_id(&mut self) {
        self.class_id = ::std::option::Option::None;
    }

    pub fn has_class_id(&self) -> bool {
        self.class_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_class_id(&mut self, v: i32) {
        self.class_id = ::std::option::Option::Some(v);
    }

    pub fn get_class_id(&self) -> i32 {
        self.class_id.unwrap_or(0)
    }

    fn get_class_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.class_id
    }

    fn mut_class_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.class_id
    }

    // optional string network_name = 2;

    pub fn clear_network_name(&mut self) {
        self.network_name.clear();
    }

    pub fn has_network_name(&self) -> bool {
        self.network_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network_name(&mut self, v: ::std::string::String) {
        self.network_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_network_name(&mut self) -> &mut ::std::string::String {
        if self.network_name.is_none() {
            self.network_name.set_default();
        };
        self.network_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_network_name(&mut self) -> ::std::string::String {
        self.network_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_network_name(&self) -> &str {
        match self.network_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_network_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.network_name
    }

    fn mut_network_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.network_name
    }

    // optional string table_name = 3;

    pub fn clear_table_name(&mut self) {
        self.table_name.clear();
    }

    pub fn has_table_name(&self) -> bool {
        self.table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_name(&mut self, v: ::std::string::String) {
        self.table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_name(&mut self) -> &mut ::std::string::String {
        if self.table_name.is_none() {
            self.table_name.set_default();
        };
        self.table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_name(&mut self) -> ::std::string::String {
        self.table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_table_name(&self) -> &str {
        match self.table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.table_name
    }

    fn mut_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.table_name
    }
}

impl ::protobuf::Message for CDemoClassInfo_class_t {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.class_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.network_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.class_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.network_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.class_id {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.network_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.table_name.as_ref() {
            os.write_string(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoClassInfo_class_t {
    fn new() -> CDemoClassInfo_class_t {
        CDemoClassInfo_class_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoClassInfo_class_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "class_id",
                    CDemoClassInfo_class_t::get_class_id_for_reflect,
                    CDemoClassInfo_class_t::mut_class_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "network_name",
                    CDemoClassInfo_class_t::get_network_name_for_reflect,
                    CDemoClassInfo_class_t::mut_network_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "table_name",
                    CDemoClassInfo_class_t::get_table_name_for_reflect,
                    CDemoClassInfo_class_t::mut_table_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoClassInfo_class_t>(
                    "CDemoClassInfo_class_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoClassInfo_class_t {
    fn clear(&mut self) {
        self.clear_class_id();
        self.clear_network_name();
        self.clear_table_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoClassInfo_class_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoClassInfo_class_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoCustomData {
    // message fields
    callback_index: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoCustomData {}

impl CDemoCustomData {
    pub fn new() -> CDemoCustomData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoCustomData {
        static mut instance: ::protobuf::lazy::Lazy<CDemoCustomData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoCustomData,
        };
        unsafe {
            instance.get(CDemoCustomData::new)
        }
    }

    // optional int32 callback_index = 1;

    pub fn clear_callback_index(&mut self) {
        self.callback_index = ::std::option::Option::None;
    }

    pub fn has_callback_index(&self) -> bool {
        self.callback_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_callback_index(&mut self, v: i32) {
        self.callback_index = ::std::option::Option::Some(v);
    }

    pub fn get_callback_index(&self) -> i32 {
        self.callback_index.unwrap_or(0)
    }

    fn get_callback_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.callback_index
    }

    fn mut_callback_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.callback_index
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for CDemoCustomData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.callback_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.callback_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.callback_index {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoCustomData {
    fn new() -> CDemoCustomData {
        CDemoCustomData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoCustomData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "callback_index",
                    CDemoCustomData::get_callback_index_for_reflect,
                    CDemoCustomData::mut_callback_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CDemoCustomData::get_data_for_reflect,
                    CDemoCustomData::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoCustomData>(
                    "CDemoCustomData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoCustomData {
    fn clear(&mut self) {
        self.clear_callback_index();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoCustomData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoCustomData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoCustomDataCallbacks {
    // message fields
    save_id: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoCustomDataCallbacks {}

impl CDemoCustomDataCallbacks {
    pub fn new() -> CDemoCustomDataCallbacks {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoCustomDataCallbacks {
        static mut instance: ::protobuf::lazy::Lazy<CDemoCustomDataCallbacks> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoCustomDataCallbacks,
        };
        unsafe {
            instance.get(CDemoCustomDataCallbacks::new)
        }
    }

    // repeated string save_id = 1;

    pub fn clear_save_id(&mut self) {
        self.save_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_save_id(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.save_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_save_id(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.save_id
    }

    // Take field
    pub fn take_save_id(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.save_id, ::protobuf::RepeatedField::new())
    }

    pub fn get_save_id(&self) -> &[::std::string::String] {
        &self.save_id
    }

    fn get_save_id_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.save_id
    }

    fn mut_save_id_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.save_id
    }
}

impl ::protobuf::Message for CDemoCustomDataCallbacks {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.save_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.save_id {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.save_id {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoCustomDataCallbacks {
    fn new() -> CDemoCustomDataCallbacks {
        CDemoCustomDataCallbacks::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoCustomDataCallbacks>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "save_id",
                    CDemoCustomDataCallbacks::get_save_id_for_reflect,
                    CDemoCustomDataCallbacks::mut_save_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoCustomDataCallbacks>(
                    "CDemoCustomDataCallbacks",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoCustomDataCallbacks {
    fn clear(&mut self) {
        self.clear_save_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoCustomDataCallbacks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoCustomDataCallbacks {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoStringTables {
    // message fields
    tables: ::protobuf::RepeatedField<CDemoStringTables_table_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoStringTables {}

impl CDemoStringTables {
    pub fn new() -> CDemoStringTables {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoStringTables {
        static mut instance: ::protobuf::lazy::Lazy<CDemoStringTables> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoStringTables,
        };
        unsafe {
            instance.get(CDemoStringTables::new)
        }
    }

    // repeated .dota.CDemoStringTables.table_t tables = 1;

    pub fn clear_tables(&mut self) {
        self.tables.clear();
    }

    // Param is passed by value, moved
    pub fn set_tables(&mut self, v: ::protobuf::RepeatedField<CDemoStringTables_table_t>) {
        self.tables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tables(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_table_t> {
        &mut self.tables
    }

    // Take field
    pub fn take_tables(&mut self) -> ::protobuf::RepeatedField<CDemoStringTables_table_t> {
        ::std::mem::replace(&mut self.tables, ::protobuf::RepeatedField::new())
    }

    pub fn get_tables(&self) -> &[CDemoStringTables_table_t] {
        &self.tables
    }

    fn get_tables_for_reflect(&self) -> &::protobuf::RepeatedField<CDemoStringTables_table_t> {
        &self.tables
    }

    fn mut_tables_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_table_t> {
        &mut self.tables
    }
}

impl ::protobuf::Message for CDemoStringTables {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tables)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.tables {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tables {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoStringTables {
    fn new() -> CDemoStringTables {
        CDemoStringTables::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoStringTables>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDemoStringTables_table_t>>(
                    "tables",
                    CDemoStringTables::get_tables_for_reflect,
                    CDemoStringTables::mut_tables_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoStringTables>(
                    "CDemoStringTables",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoStringTables {
    fn clear(&mut self) {
        self.clear_tables();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoStringTables {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoStringTables {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoStringTables_items_t {
    // message fields
    str: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoStringTables_items_t {}

impl CDemoStringTables_items_t {
    pub fn new() -> CDemoStringTables_items_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoStringTables_items_t {
        static mut instance: ::protobuf::lazy::Lazy<CDemoStringTables_items_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoStringTables_items_t,
        };
        unsafe {
            instance.get(CDemoStringTables_items_t::new)
        }
    }

    // optional string str = 1;

    pub fn clear_str(&mut self) {
        self.str.clear();
    }

    pub fn has_str(&self) -> bool {
        self.str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_str(&mut self, v: ::std::string::String) {
        self.str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_str(&mut self) -> &mut ::std::string::String {
        if self.str.is_none() {
            self.str.set_default();
        };
        self.str.as_mut().unwrap()
    }

    // Take field
    pub fn take_str(&mut self) -> ::std::string::String {
        self.str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_str(&self) -> &str {
        match self.str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_str_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.str
    }

    fn mut_str_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.str
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for CDemoStringTables_items_t {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.str)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.str.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.str.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoStringTables_items_t {
    fn new() -> CDemoStringTables_items_t {
        CDemoStringTables_items_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoStringTables_items_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "str",
                    CDemoStringTables_items_t::get_str_for_reflect,
                    CDemoStringTables_items_t::mut_str_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CDemoStringTables_items_t::get_data_for_reflect,
                    CDemoStringTables_items_t::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoStringTables_items_t>(
                    "CDemoStringTables_items_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoStringTables_items_t {
    fn clear(&mut self) {
        self.clear_str();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoStringTables_items_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoStringTables_items_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoStringTables_table_t {
    // message fields
    table_name: ::protobuf::SingularField<::std::string::String>,
    items: ::protobuf::RepeatedField<CDemoStringTables_items_t>,
    items_clientside: ::protobuf::RepeatedField<CDemoStringTables_items_t>,
    table_flags: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoStringTables_table_t {}

impl CDemoStringTables_table_t {
    pub fn new() -> CDemoStringTables_table_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoStringTables_table_t {
        static mut instance: ::protobuf::lazy::Lazy<CDemoStringTables_table_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoStringTables_table_t,
        };
        unsafe {
            instance.get(CDemoStringTables_table_t::new)
        }
    }

    // optional string table_name = 1;

    pub fn clear_table_name(&mut self) {
        self.table_name.clear();
    }

    pub fn has_table_name(&self) -> bool {
        self.table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_name(&mut self, v: ::std::string::String) {
        self.table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table_name(&mut self) -> &mut ::std::string::String {
        if self.table_name.is_none() {
            self.table_name.set_default();
        };
        self.table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_table_name(&mut self) -> ::std::string::String {
        self.table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_table_name(&self) -> &str {
        match self.table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.table_name
    }

    fn mut_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.table_name
    }

    // repeated .dota.CDemoStringTables.items_t items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CDemoStringTables_items_t>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CDemoStringTables_items_t] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<CDemoStringTables_items_t> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        &mut self.items
    }

    // repeated .dota.CDemoStringTables.items_t items_clientside = 3;

    pub fn clear_items_clientside(&mut self) {
        self.items_clientside.clear();
    }

    // Param is passed by value, moved
    pub fn set_items_clientside(&mut self, v: ::protobuf::RepeatedField<CDemoStringTables_items_t>) {
        self.items_clientside = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items_clientside(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        &mut self.items_clientside
    }

    // Take field
    pub fn take_items_clientside(&mut self) -> ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        ::std::mem::replace(&mut self.items_clientside, ::protobuf::RepeatedField::new())
    }

    pub fn get_items_clientside(&self) -> &[CDemoStringTables_items_t] {
        &self.items_clientside
    }

    fn get_items_clientside_for_reflect(&self) -> &::protobuf::RepeatedField<CDemoStringTables_items_t> {
        &self.items_clientside
    }

    fn mut_items_clientside_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDemoStringTables_items_t> {
        &mut self.items_clientside
    }

    // optional int32 table_flags = 4;

    pub fn clear_table_flags(&mut self) {
        self.table_flags = ::std::option::Option::None;
    }

    pub fn has_table_flags(&self) -> bool {
        self.table_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_flags(&mut self, v: i32) {
        self.table_flags = ::std::option::Option::Some(v);
    }

    pub fn get_table_flags(&self) -> i32 {
        self.table_flags.unwrap_or(0)
    }

    fn get_table_flags_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.table_flags
    }

    fn mut_table_flags_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.table_flags
    }
}

impl ::protobuf::Message for CDemoStringTables_table_t {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.table_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items_clientside)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.table_flags = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.items_clientside {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.table_flags {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_name.as_ref() {
            os.write_string(1, &v)?;
        };
        for v in &self.items {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.items_clientside {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.table_flags {
            os.write_int32(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoStringTables_table_t {
    fn new() -> CDemoStringTables_table_t {
        CDemoStringTables_table_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoStringTables_table_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "table_name",
                    CDemoStringTables_table_t::get_table_name_for_reflect,
                    CDemoStringTables_table_t::mut_table_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDemoStringTables_items_t>>(
                    "items",
                    CDemoStringTables_table_t::get_items_for_reflect,
                    CDemoStringTables_table_t::mut_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDemoStringTables_items_t>>(
                    "items_clientside",
                    CDemoStringTables_table_t::get_items_clientside_for_reflect,
                    CDemoStringTables_table_t::mut_items_clientside_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "table_flags",
                    CDemoStringTables_table_t::get_table_flags_for_reflect,
                    CDemoStringTables_table_t::mut_table_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoStringTables_table_t>(
                    "CDemoStringTables_table_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoStringTables_table_t {
    fn clear(&mut self) {
        self.clear_table_name();
        self.clear_items();
        self.clear_items_clientside();
        self.clear_table_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoStringTables_table_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoStringTables_table_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoStop {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoStop {}

impl CDemoStop {
    pub fn new() -> CDemoStop {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoStop {
        static mut instance: ::protobuf::lazy::Lazy<CDemoStop> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoStop,
        };
        unsafe {
            instance.get(CDemoStop::new)
        }
    }
}

impl ::protobuf::Message for CDemoStop {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoStop {
    fn new() -> CDemoStop {
        CDemoStop::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoStop>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDemoStop>(
                    "CDemoStop",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoStop {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoStop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoStop {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoUserCmd {
    // message fields
    cmd_number: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoUserCmd {}

impl CDemoUserCmd {
    pub fn new() -> CDemoUserCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoUserCmd {
        static mut instance: ::protobuf::lazy::Lazy<CDemoUserCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoUserCmd,
        };
        unsafe {
            instance.get(CDemoUserCmd::new)
        }
    }

    // optional int32 cmd_number = 1;

    pub fn clear_cmd_number(&mut self) {
        self.cmd_number = ::std::option::Option::None;
    }

    pub fn has_cmd_number(&self) -> bool {
        self.cmd_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_number(&mut self, v: i32) {
        self.cmd_number = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_number(&self) -> i32 {
        self.cmd_number.unwrap_or(0)
    }

    fn get_cmd_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.cmd_number
    }

    fn mut_cmd_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.cmd_number
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for CDemoUserCmd {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.cmd_number = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.cmd_number {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd_number {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoUserCmd {
    fn new() -> CDemoUserCmd {
        CDemoUserCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoUserCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cmd_number",
                    CDemoUserCmd::get_cmd_number_for_reflect,
                    CDemoUserCmd::mut_cmd_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CDemoUserCmd::get_data_for_reflect,
                    CDemoUserCmd::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoUserCmd>(
                    "CDemoUserCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoUserCmd {
    fn clear(&mut self) {
        self.clear_cmd_number();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoUserCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoUserCmd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDemoSpawnGroups {
    // message fields
    msgs: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDemoSpawnGroups {}

impl CDemoSpawnGroups {
    pub fn new() -> CDemoSpawnGroups {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDemoSpawnGroups {
        static mut instance: ::protobuf::lazy::Lazy<CDemoSpawnGroups> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDemoSpawnGroups,
        };
        unsafe {
            instance.get(CDemoSpawnGroups::new)
        }
    }

    // repeated bytes msgs = 3;

    pub fn clear_msgs(&mut self) {
        self.msgs.clear();
    }

    // Param is passed by value, moved
    pub fn set_msgs(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.msgs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_msgs(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.msgs
    }

    // Take field
    pub fn take_msgs(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.msgs, ::protobuf::RepeatedField::new())
    }

    pub fn get_msgs(&self) -> &[::std::vec::Vec<u8>] {
        &self.msgs
    }

    fn get_msgs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.msgs
    }

    fn mut_msgs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.msgs
    }
}

impl ::protobuf::Message for CDemoSpawnGroups {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.msgs)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.msgs {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.msgs {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDemoSpawnGroups {
    fn new() -> CDemoSpawnGroups {
        CDemoSpawnGroups::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDemoSpawnGroups>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "msgs",
                    CDemoSpawnGroups::get_msgs_for_reflect,
                    CDemoSpawnGroups::mut_msgs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDemoSpawnGroups>(
                    "CDemoSpawnGroups",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDemoSpawnGroups {
    fn clear(&mut self) {
        self.clear_msgs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDemoSpawnGroups {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDemoSpawnGroups {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDemoCommands {
    DEM_Error = -1,
    DEM_Stop = 0,
    DEM_FileHeader = 1,
    DEM_FileInfo = 2,
    DEM_SyncTick = 3,
    DEM_SendTables = 4,
    DEM_ClassInfo = 5,
    DEM_StringTables = 6,
    DEM_Packet = 7,
    DEM_SignonPacket = 8,
    DEM_ConsoleCmd = 9,
    DEM_CustomData = 10,
    DEM_CustomDataCallbacks = 11,
    DEM_UserCmd = 12,
    DEM_FullPacket = 13,
    DEM_SaveGame = 14,
    DEM_SpawnGroups = 15,
    DEM_Max = 16,
    DEM_IsCompressed = 64,
}

impl ::protobuf::ProtobufEnum for EDemoCommands {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDemoCommands> {
        match value {
            -1 => ::std::option::Option::Some(EDemoCommands::DEM_Error),
            0 => ::std::option::Option::Some(EDemoCommands::DEM_Stop),
            1 => ::std::option::Option::Some(EDemoCommands::DEM_FileHeader),
            2 => ::std::option::Option::Some(EDemoCommands::DEM_FileInfo),
            3 => ::std::option::Option::Some(EDemoCommands::DEM_SyncTick),
            4 => ::std::option::Option::Some(EDemoCommands::DEM_SendTables),
            5 => ::std::option::Option::Some(EDemoCommands::DEM_ClassInfo),
            6 => ::std::option::Option::Some(EDemoCommands::DEM_StringTables),
            7 => ::std::option::Option::Some(EDemoCommands::DEM_Packet),
            8 => ::std::option::Option::Some(EDemoCommands::DEM_SignonPacket),
            9 => ::std::option::Option::Some(EDemoCommands::DEM_ConsoleCmd),
            10 => ::std::option::Option::Some(EDemoCommands::DEM_CustomData),
            11 => ::std::option::Option::Some(EDemoCommands::DEM_CustomDataCallbacks),
            12 => ::std::option::Option::Some(EDemoCommands::DEM_UserCmd),
            13 => ::std::option::Option::Some(EDemoCommands::DEM_FullPacket),
            14 => ::std::option::Option::Some(EDemoCommands::DEM_SaveGame),
            15 => ::std::option::Option::Some(EDemoCommands::DEM_SpawnGroups),
            16 => ::std::option::Option::Some(EDemoCommands::DEM_Max),
            64 => ::std::option::Option::Some(EDemoCommands::DEM_IsCompressed),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDemoCommands] = &[
            EDemoCommands::DEM_Error,
            EDemoCommands::DEM_Stop,
            EDemoCommands::DEM_FileHeader,
            EDemoCommands::DEM_FileInfo,
            EDemoCommands::DEM_SyncTick,
            EDemoCommands::DEM_SendTables,
            EDemoCommands::DEM_ClassInfo,
            EDemoCommands::DEM_StringTables,
            EDemoCommands::DEM_Packet,
            EDemoCommands::DEM_SignonPacket,
            EDemoCommands::DEM_ConsoleCmd,
            EDemoCommands::DEM_CustomData,
            EDemoCommands::DEM_CustomDataCallbacks,
            EDemoCommands::DEM_UserCmd,
            EDemoCommands::DEM_FullPacket,
            EDemoCommands::DEM_SaveGame,
            EDemoCommands::DEM_SpawnGroups,
            EDemoCommands::DEM_Max,
            EDemoCommands::DEM_IsCompressed,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EDemoCommands>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDemoCommands", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDemoCommands {
}

impl ::protobuf::reflect::ProtobufValue for EDemoCommands {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x64, 0x65, 0x6d, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f,
    0x74, 0x61, 0x22, 0xab, 0x03, 0x0a, 0x0f, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x46, 0x69, 0x6c, 0x65,
    0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x26, 0x0a, 0x0f, 0x64, 0x65, 0x6d, 0x6f, 0x5f, 0x66,
    0x69, 0x6c, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x52,
    0x0d, 0x64, 0x65, 0x6d, 0x6f, 0x46, 0x69, 0x6c, 0x65, 0x53, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x29,
    0x0a, 0x10, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0a, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x6d,
    0x61, 0x70, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d,
    0x61, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x64,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d,
    0x67, 0x61, 0x6d, 0x65, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x12, 0x2f, 0x0a,
    0x13, 0x66, 0x75, 0x6c, 0x6c, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x12, 0x66, 0x75, 0x6c, 0x6c,
    0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x3a,
    0x0a, 0x19, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x69,
    0x64, 0x65, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x17, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x69,
    0x64, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x12, 0x3c, 0x0a, 0x1a, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x69, 0x64, 0x65, 0x5f, 0x70,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x18,
    0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x69, 0x64, 0x65, 0x50,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x64, 0x64, 0x6f,
    0x6e, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x64, 0x64, 0x6f, 0x6e, 0x73,
    0x22, 0x98, 0x06, 0x0a, 0x09, 0x43, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x31,
    0x0a, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x43, 0x44,
    0x6f, 0x74, 0x61, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x04, 0x64, 0x6f, 0x74,
    0x61, 0x1a, 0xd7, 0x05, 0x0a, 0x0d, 0x43, 0x44, 0x6f, 0x74, 0x61, 0x47, 0x61, 0x6d, 0x65, 0x49,
    0x6e, 0x66, 0x6f, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x12, 0x1b,
    0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x67,
    0x61, 0x6d, 0x65, 0x5f, 0x77, 0x69, 0x6e, 0x6e, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x57, 0x69, 0x6e, 0x6e, 0x65, 0x72, 0x12, 0x4a, 0x0a, 0x0b,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x29, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e,
    0x66, 0x6f, 0x2e, 0x43, 0x44, 0x6f, 0x74, 0x61, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f,
    0x2e, 0x43, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0a, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x65, 0x61, 0x67,
    0x75, 0x65, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x6c, 0x65, 0x61, 0x67,
    0x75, 0x65, 0x69, 0x64, 0x12, 0x4d, 0x0a, 0x0a, 0x70, 0x69, 0x63, 0x6b, 0x73, 0x5f, 0x62, 0x61,
    0x6e, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x43, 0x44, 0x6f, 0x74, 0x61, 0x47,
    0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x43, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c,
    0x65, 0x63, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x09, 0x70, 0x69, 0x63, 0x6b, 0x73, 0x42,
    0x61, 0x6e, 0x73, 0x12, 0x26, 0x0a, 0x0f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x5f, 0x74,
    0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x72, 0x61,
    0x64, 0x69, 0x61, 0x6e, 0x74, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x20, 0x0a, 0x0c, 0x64,
    0x69, 0x72, 0x65, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0a, 0x64, 0x69, 0x72, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x28, 0x0a,
    0x10, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x74, 0x61,
    0x67, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74,
    0x54, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x67, 0x12, 0x22, 0x0a, 0x0d, 0x64, 0x69, 0x72, 0x65, 0x5f,
    0x74, 0x65, 0x61, 0x6d, 0x5f, 0x74, 0x61, 0x67, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x64, 0x69, 0x72, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x67, 0x12, 0x19, 0x0a, 0x08, 0x65,
    0x6e, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x65,
    0x6e, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x1a, 0xa8, 0x01, 0x0a, 0x0b, 0x43, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x68, 0x65, 0x72, 0x6f, 0x4e,
    0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x4e, 0x61, 0x6d, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x69, 0x73, 0x5f, 0x66, 0x61, 0x6b, 0x65, 0x5f,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x69, 0x73,
    0x46, 0x61, 0x6b, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74,
    0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x73, 0x74, 0x65,
    0x61, 0x6d, 0x69, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x74, 0x65, 0x61,
    0x6d, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x54, 0x65, 0x61,
    0x6d, 0x1a, 0x58, 0x0a, 0x10, 0x43, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x73, 0x5f, 0x70, 0x69, 0x63, 0x6b,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x69, 0x73, 0x50, 0x69, 0x63, 0x6b, 0x12, 0x12,
    0x0a, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x74, 0x65,
    0x61, 0x6d, 0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49, 0x64, 0x22, 0xb2, 0x01, 0x0a, 0x0d,
    0x43, 0x44, 0x65, 0x6d, 0x6f, 0x46, 0x69, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x23, 0x0a,
    0x0d, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x69,
    0x6d, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x74,
    0x69, 0x63, 0x6b, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x70, 0x6c, 0x61, 0x79,
    0x62, 0x61, 0x63, 0x6b, 0x54, 0x69, 0x63, 0x6b, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x6c, 0x61,
    0x79, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0e, 0x70, 0x6c, 0x61, 0x79, 0x62, 0x61, 0x63, 0x6b, 0x46, 0x72, 0x61, 0x6d,
    0x65, 0x73, 0x12, 0x2c, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x47, 0x61,
    0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x66, 0x6f,
    0x22, 0x6c, 0x0a, 0x0b, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x12,
    0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x69, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x49, 0x6e,
    0x12, 0x28, 0x0a, 0x10, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x6f, 0x75, 0x74,
    0x5f, 0x61, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x73, 0x65, 0x71, 0x75,
    0x65, 0x6e, 0x63, 0x65, 0x4f, 0x75, 0x74, 0x41, 0x63, 0x6b, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61,
    0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x78,
    0x0a, 0x0f, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x46, 0x75, 0x6c, 0x6c, 0x50, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x12, 0x3a, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x61, 0x62, 0x6c,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73,
    0x52, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x29, 0x0a,
    0x06, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74,
    0x52, 0x06, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x22, 0x76, 0x0a, 0x0d, 0x43, 0x44, 0x65, 0x6d,
    0x6f, 0x53, 0x61, 0x76, 0x65, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x19, 0x0a,
    0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e,
    0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x73, 0x69, 0x67,
    0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x22, 0x0f, 0x0a, 0x0d, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x79, 0x6e, 0x63, 0x54, 0x69, 0x63,
    0x6b, 0x22, 0x2f, 0x0a, 0x0f, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x43, 0x6f, 0x6e, 0x73, 0x6f, 0x6c,
    0x65, 0x43, 0x6d, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x6d, 0x64, 0x73, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x63, 0x6d, 0x64, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x22, 0x25, 0x0a, 0x0f, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x65, 0x6e, 0x64, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0xb0, 0x01, 0x0a, 0x0e, 0x43, 0x44,
    0x65, 0x6d, 0x6f, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x36, 0x0a, 0x07,
    0x63, 0x6c, 0x61, 0x73, 0x73, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x49,
    0x6e, 0x66, 0x6f, 0x2e, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x74, 0x52, 0x07, 0x63, 0x6c, 0x61,
    0x73, 0x73, 0x65, 0x73, 0x1a, 0x66, 0x0a, 0x07, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x74, 0x12,
    0x19, 0x0a, 0x08, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x07, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x6e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0b, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a,
    0x0a, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x4c, 0x0a, 0x0f,
    0x43, 0x44, 0x65, 0x6d, 0x6f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x25, 0x0a, 0x0e, 0x63, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x63, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63,
    0x6b, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x33, 0x0a, 0x18, 0x43, 0x44,
    0x65, 0x6d, 0x6f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x43, 0x61, 0x6c,
    0x6c, 0x62, 0x61, 0x63, 0x6b, 0x73, 0x12, 0x17, 0x0a, 0x07, 0x73, 0x61, 0x76, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x06, 0x73, 0x61, 0x76, 0x65, 0x49, 0x64, 0x22,
    0xcc, 0x02, 0x0a, 0x11, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x73, 0x12, 0x37, 0x0a, 0x06, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x65,
    0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x2e, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x5f, 0x74, 0x52, 0x06, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x1a, 0x2f,
    0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x73, 0x74, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x73, 0x74, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x1a,
    0xcc, 0x01, 0x0a, 0x07, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x09, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x35, 0x0a, 0x05, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c,
    0x65, 0x73, 0x2e, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x74, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d,
    0x73, 0x12, 0x4a, 0x0a, 0x10, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x73, 0x69, 0x64, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61,
    0x62, 0x6c, 0x65, 0x73, 0x2e, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x74, 0x52, 0x0f, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x69, 0x64, 0x65, 0x12, 0x1f, 0x0a,
    0x0b, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0a, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x22, 0x0b,
    0x0a, 0x09, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x74, 0x6f, 0x70, 0x22, 0x41, 0x0a, 0x0c, 0x43,
    0x44, 0x65, 0x6d, 0x6f, 0x55, 0x73, 0x65, 0x72, 0x43, 0x6d, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x63,
    0x6d, 0x64, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x09, 0x63, 0x6d, 0x64, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61,
    0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x26,
    0x0a, 0x10, 0x43, 0x44, 0x65, 0x6d, 0x6f, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x47, 0x72, 0x6f, 0x75,
    0x70, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x73, 0x67, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0c,
    0x52, 0x04, 0x6d, 0x73, 0x67, 0x73, 0x2a, 0x84, 0x03, 0x0a, 0x0d, 0x45, 0x44, 0x65, 0x6d, 0x6f,
    0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x12, 0x16, 0x0a, 0x09, 0x44, 0x45, 0x4d, 0x5f,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
    0x12, 0x0c, 0x0a, 0x08, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x74, 0x6f, 0x70, 0x10, 0x00, 0x12, 0x12,
    0x0a, 0x0e, 0x44, 0x45, 0x4d, 0x5f, 0x46, 0x69, 0x6c, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c, 0x44, 0x45, 0x4d, 0x5f, 0x46, 0x69, 0x6c, 0x65, 0x49, 0x6e,
    0x66, 0x6f, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x79, 0x6e, 0x63,
    0x54, 0x69, 0x63, 0x6b, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x65,
    0x6e, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x10, 0x04, 0x12, 0x11, 0x0a, 0x0d, 0x44, 0x45,
    0x4d, 0x5f, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x05, 0x12, 0x14, 0x0a,
    0x10, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x73, 0x10, 0x06, 0x12, 0x0e, 0x0a, 0x0a, 0x44, 0x45, 0x4d, 0x5f, 0x50, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x10, 0x07, 0x12, 0x14, 0x0a, 0x10, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x69, 0x67, 0x6e, 0x6f,
    0x6e, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x10, 0x08, 0x12, 0x12, 0x0a, 0x0e, 0x44, 0x45, 0x4d,
    0x5f, 0x43, 0x6f, 0x6e, 0x73, 0x6f, 0x6c, 0x65, 0x43, 0x6d, 0x64, 0x10, 0x09, 0x12, 0x12, 0x0a,
    0x0e, 0x44, 0x45, 0x4d, 0x5f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x10,
    0x0a, 0x12, 0x1b, 0x0a, 0x17, 0x44, 0x45, 0x4d, 0x5f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44,
    0x61, 0x74, 0x61, 0x43, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x73, 0x10, 0x0b, 0x12, 0x0f,
    0x0a, 0x0b, 0x44, 0x45, 0x4d, 0x5f, 0x55, 0x73, 0x65, 0x72, 0x43, 0x6d, 0x64, 0x10, 0x0c, 0x12,
    0x12, 0x0a, 0x0e, 0x44, 0x45, 0x4d, 0x5f, 0x46, 0x75, 0x6c, 0x6c, 0x50, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x10, 0x0d, 0x12, 0x10, 0x0a, 0x0c, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x61, 0x76, 0x65, 0x47,
    0x61, 0x6d, 0x65, 0x10, 0x0e, 0x12, 0x13, 0x0a, 0x0f, 0x44, 0x45, 0x4d, 0x5f, 0x53, 0x70, 0x61,
    0x77, 0x6e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x10, 0x0f, 0x12, 0x0b, 0x0a, 0x07, 0x44, 0x45,
    0x4d, 0x5f, 0x4d, 0x61, 0x78, 0x10, 0x10, 0x12, 0x14, 0x0a, 0x10, 0x44, 0x45, 0x4d, 0x5f, 0x49,
    0x73, 0x43, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x10, 0x40, 0x42, 0x03, 0x80,
    0x01, 0x00, 0x4a, 0xa4, 0x30, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x9a, 0x01, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x08, 0x0c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x04, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x06, 0x00, 0x1a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x06, 0x05, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x07, 0x14, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08,
    0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x08, 0x13, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x09, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x0a, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0a, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x0b, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x0c, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x0c, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0c,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x08, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x0d, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x0e, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x0e, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12,
    0x03, 0x0e, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x0f, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x0f, 0x15, 0x16, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x10, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x10, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09,
    0x02, 0x12, 0x03, 0x10, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03,
    0x11, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x11, 0x08,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x11, 0x19, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x12, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x12, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0b, 0x02, 0x12, 0x03, 0x12, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c,
    0x12, 0x03, 0x13, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x13, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x13, 0x22,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x14, 0x08, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x14, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x14, 0x16, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x0e, 0x12, 0x03, 0x15, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x01,
    0x12, 0x03, 0x15, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03,
    0x15, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x16, 0x08, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x16, 0x08, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x16, 0x17, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x10, 0x12, 0x03, 0x17, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x10, 0x01, 0x12, 0x03, 0x17, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x02,
    0x12, 0x03, 0x17, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x11, 0x12, 0x03, 0x18,
    0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x18, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x02, 0x12, 0x03, 0x18, 0x12, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x12, 0x12, 0x03, 0x19, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x19, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x12, 0x02, 0x12, 0x03, 0x19, 0x1b, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1c,
    0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x17, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1d, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1d, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x08,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1e, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x17, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x1f, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1f,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x18, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x26, 0x27, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x20, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x20, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x20, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x20, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x21, 0x08, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x21, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x21, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x21, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x21, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x22, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x22, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x22, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x22, 0x18, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x22, 0x29, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x23, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x23, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x23, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x23,
    0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x24, 0x08, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x24, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x24, 0x16, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x24, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12,
    0x03, 0x25, 0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x25,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x25, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x25, 0x16, 0x30, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x25, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x26, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x09, 0x04, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x26, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x26, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x26, 0x21,
    0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x29, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x29, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03,
    0x00, 0x12, 0x04, 0x2a, 0x08, 0x44, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x2a, 0x10, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x12,
    0x04, 0x2b, 0x10, 0x31, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x2b, 0x18, 0x23, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x2c, 0x18, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x2c, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x28, 0x31, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x34, 0x35, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x18, 0x38, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x18,
    0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x2d, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2d, 0x28, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x2d, 0x36, 0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x18, 0x39, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2e, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x21, 0x25, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x26, 0x34, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x37, 0x38,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x18,
    0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x2f, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x2f, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x2f, 0x28, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2f, 0x32, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x30, 0x18, 0x35, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x30, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x30, 0x21, 0x26, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x30, 0x27, 0x30,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x30,
    0x33, 0x34, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x12, 0x04, 0x33, 0x10,
    0x37, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x33,
    0x18, 0x28, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x34, 0x18, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x34, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x34, 0x21, 0x25, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x26, 0x2d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x34, 0x30, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x35, 0x18, 0x31, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x35, 0x18, 0x20, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x35, 0x21, 0x27,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35,
    0x28, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x35, 0x2f, 0x30, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x36, 0x18, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x36, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x36, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x36, 0x28, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x36, 0x32, 0x33, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x39, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x39, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x39, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x20, 0x28, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3a, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x1f, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x3b, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3b, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3b, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x1f, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3b, 0x2d, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x3c, 0x10, 0x4d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3c, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x3c, 0x19, 0x3c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3c, 0x3d, 0x48, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3c, 0x4b, 0x4c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x3d, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x3d, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x3d, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x3d, 0x20, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x3d, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x3e, 0x10, 0x51, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x3e, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x05, 0x06, 0x12, 0x03, 0x3e, 0x19, 0x41, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x3e, 0x42, 0x4c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x3e, 0x4f, 0x50, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x3f, 0x10, 0x34, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x3f, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x3f, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x3f, 0x20, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x3f, 0x32, 0x33, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07,
    0x12, 0x03, 0x40, 0x10, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x04,
    0x12, 0x03, 0x40, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x05,
    0x12, 0x03, 0x40, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x40, 0x20, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x40, 0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x12,
    0x03, 0x41, 0x10, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x04, 0x12,
    0x03, 0x41, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x41, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x41, 0x20, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x41, 0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x12, 0x03,
    0x42, 0x10, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03,
    0x42, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x42, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x42, 0x20, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03,
    0x42, 0x30, 0x32, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x43,
    0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x43,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x43,
    0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x43,
    0x20, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x43,
    0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x46, 0x08, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x46, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x46, 0x11, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x29, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x46, 0x30, 0x31, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x49,
    0x00, 0x4e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x49, 0x08, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x4a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x4a, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x4a, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4b, 0x08,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4b, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x17, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4b, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x4c, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x4c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4c,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x17, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4c, 0x29, 0x2a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4d, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x4d, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x4d, 0x1b, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x4d, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x50, 0x00, 0x54, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x50, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x51, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x51, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x51, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x51,
    0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x51, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x52, 0x08, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x52, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x52, 0x17, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x52, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x53,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x53, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x53, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x53, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x53, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x56, 0x00, 0x59, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x56,
    0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x57, 0x08, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x57, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x57, 0x11, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x23, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x57, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x58, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x58,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x58, 0x11, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x58, 0x1d, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x58, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x5b, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12,
    0x03, 0x5b, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x08,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5c, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5c, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x01, 0x12, 0x03, 0x5d, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x5d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5d,
    0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x19, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x24, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x5e, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x5e, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x5e, 0x19, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x5e, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x08, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x5f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x5f, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x5f, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x62, 0x00, 0x63, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x62, 0x08, 0x15,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x65, 0x00, 0x67, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x07, 0x01, 0x12, 0x03, 0x65, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00,
    0x12, 0x03, 0x66, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x66, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x66, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x18, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x66, 0x24, 0x25, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x08, 0x12, 0x04, 0x69, 0x00, 0x6b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01,
    0x12, 0x03, 0x69, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x6a,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6a, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6a, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x04, 0x6d, 0x00, 0x75, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x6d,
    0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x03, 0x00, 0x12, 0x04, 0x6e, 0x08, 0x72, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x03, 0x00, 0x01, 0x12, 0x03, 0x6e, 0x10, 0x17, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6f, 0x10, 0x2c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6f, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6f, 0x19, 0x1e, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6f, 0x1f, 0x27, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6f, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x09, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x70, 0x10, 0x31, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x09, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x70, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x09, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x70, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x09, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x70, 0x20, 0x2c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x09, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x70, 0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x09, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x71, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x09, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x71, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x09, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x71, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x09, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x71, 0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x09, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x71, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x00, 0x12, 0x03, 0x74, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x74, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x74, 0x11, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x74,
    0x28, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x74, 0x32, 0x33,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x77, 0x00, 0x7a, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0a, 0x01, 0x12, 0x03, 0x77, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00,
    0x12, 0x03, 0x78, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x78, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x78, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x78, 0x17, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x78, 0x28, 0x29, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x79, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x79, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x79, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x79, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x79,
    0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x7c, 0x00, 0x7e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x00, 0x12, 0x03, 0x7d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x7d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x7d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7d, 0x18,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7d, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x80, 0x01, 0x00, 0x8e, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x80, 0x01, 0x08, 0x19, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0c,
    0x03, 0x00, 0x12, 0x06, 0x81, 0x01, 0x08, 0x84, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x03, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x04, 0x82, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x82, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0c, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x01, 0x20, 0x23, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x01, 0x26, 0x27, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x83, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x83, 0x01, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x83, 0x01, 0x19, 0x1e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x83, 0x01, 0x1f, 0x23,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x83, 0x01, 0x26,
    0x27, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0c, 0x03, 0x01, 0x12, 0x06, 0x86, 0x01, 0x08, 0x8b, 0x01,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x03, 0x01, 0x01, 0x12, 0x04, 0x86, 0x01, 0x10, 0x17,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0x87, 0x01, 0x10, 0x2f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0x87, 0x01,
    0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87,
    0x01, 0x20, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x87, 0x01, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x01, 0x12, 0x04,
    0x88, 0x01, 0x10, 0x3d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x88, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x01, 0x06,
    0x12, 0x04, 0x88, 0x01, 0x19, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x88, 0x01, 0x33, 0x38, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x88, 0x01, 0x3b, 0x3c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x01,
    0x02, 0x02, 0x12, 0x04, 0x89, 0x01, 0x10, 0x48, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x89, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03,
    0x01, 0x02, 0x02, 0x06, 0x12, 0x04, 0x89, 0x01, 0x19, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c,
    0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0x89, 0x01, 0x33, 0x43, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0c, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x04, 0x89, 0x01, 0x46, 0x47, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x0c, 0x03, 0x01, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0c, 0x03, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8a, 0x01, 0x19, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x1f, 0x2a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x2d, 0x2e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x8d, 0x01, 0x08, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8d, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8d, 0x01, 0x11, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x2b, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0d,
    0x12, 0x06, 0x90, 0x01, 0x00, 0x91, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12,
    0x04, 0x90, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0x93, 0x01, 0x00,
    0x96, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0x93, 0x01, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0x94, 0x01, 0x08, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0x94, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x01, 0x12, 0x04, 0x95, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x95, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x95, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x95, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x95, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x98, 0x01, 0x00, 0x9a,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x98, 0x01, 0x08, 0x18, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0x99, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x99, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x99, 0x01, 0x1e, 0x1f,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
