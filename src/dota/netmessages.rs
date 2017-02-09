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
pub struct CCLCMsg_ClientInfo {
    // message fields
    send_table_crc: ::std::option::Option<u32>,
    server_count: ::std::option::Option<u32>,
    is_hltv: ::std::option::Option<bool>,
    is_replay: ::std::option::Option<bool>,
    friends_id: ::std::option::Option<u32>,
    friends_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ClientInfo {}

impl CCLCMsg_ClientInfo {
    pub fn new() -> CCLCMsg_ClientInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ClientInfo {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ClientInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ClientInfo,
        };
        unsafe {
            instance.get(CCLCMsg_ClientInfo::new)
        }
    }

    // optional fixed32 send_table_crc = 1;

    pub fn clear_send_table_crc(&mut self) {
        self.send_table_crc = ::std::option::Option::None;
    }

    pub fn has_send_table_crc(&self) -> bool {
        self.send_table_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_table_crc(&mut self, v: u32) {
        self.send_table_crc = ::std::option::Option::Some(v);
    }

    pub fn get_send_table_crc(&self) -> u32 {
        self.send_table_crc.unwrap_or(0)
    }

    fn get_send_table_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.send_table_crc
    }

    fn mut_send_table_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.send_table_crc
    }

    // optional uint32 server_count = 2;

    pub fn clear_server_count(&mut self) {
        self.server_count = ::std::option::Option::None;
    }

    pub fn has_server_count(&self) -> bool {
        self.server_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_count(&mut self, v: u32) {
        self.server_count = ::std::option::Option::Some(v);
    }

    pub fn get_server_count(&self) -> u32 {
        self.server_count.unwrap_or(0)
    }

    fn get_server_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_count
    }

    fn mut_server_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_count
    }

    // optional bool is_hltv = 3;

    pub fn clear_is_hltv(&mut self) {
        self.is_hltv = ::std::option::Option::None;
    }

    pub fn has_is_hltv(&self) -> bool {
        self.is_hltv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_hltv(&mut self, v: bool) {
        self.is_hltv = ::std::option::Option::Some(v);
    }

    pub fn get_is_hltv(&self) -> bool {
        self.is_hltv.unwrap_or(false)
    }

    fn get_is_hltv_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_hltv
    }

    fn mut_is_hltv_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_hltv
    }

    // optional bool is_replay = 4;

    pub fn clear_is_replay(&mut self) {
        self.is_replay = ::std::option::Option::None;
    }

    pub fn has_is_replay(&self) -> bool {
        self.is_replay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_replay(&mut self, v: bool) {
        self.is_replay = ::std::option::Option::Some(v);
    }

    pub fn get_is_replay(&self) -> bool {
        self.is_replay.unwrap_or(false)
    }

    fn get_is_replay_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_replay
    }

    fn mut_is_replay_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_replay
    }

    // optional uint32 friends_id = 5;

    pub fn clear_friends_id(&mut self) {
        self.friends_id = ::std::option::Option::None;
    }

    pub fn has_friends_id(&self) -> bool {
        self.friends_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friends_id(&mut self, v: u32) {
        self.friends_id = ::std::option::Option::Some(v);
    }

    pub fn get_friends_id(&self) -> u32 {
        self.friends_id.unwrap_or(0)
    }

    fn get_friends_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.friends_id
    }

    fn mut_friends_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.friends_id
    }

    // optional string friends_name = 6;

    pub fn clear_friends_name(&mut self) {
        self.friends_name.clear();
    }

    pub fn has_friends_name(&self) -> bool {
        self.friends_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friends_name(&mut self, v: ::std::string::String) {
        self.friends_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friends_name(&mut self) -> &mut ::std::string::String {
        if self.friends_name.is_none() {
            self.friends_name.set_default();
        };
        self.friends_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_friends_name(&mut self) -> ::std::string::String {
        self.friends_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_friends_name(&self) -> &str {
        match self.friends_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_friends_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.friends_name
    }

    fn mut_friends_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.friends_name
    }
}

impl ::protobuf::Message for CCLCMsg_ClientInfo {
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
                    let tmp = is.read_fixed32()?;
                    self.send_table_crc = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.server_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_hltv = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_replay = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.friends_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.friends_name)?;
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
        if let Some(v) = self.send_table_crc {
            my_size += 5;
        };
        if let Some(v) = self.server_count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.is_hltv {
            my_size += 2;
        };
        if let Some(v) = self.is_replay {
            my_size += 2;
        };
        if let Some(v) = self.friends_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.friends_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.send_table_crc {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.server_count {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.is_hltv {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.is_replay {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.friends_id {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.friends_name.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_ClientInfo {
    fn new() -> CCLCMsg_ClientInfo {
        CCLCMsg_ClientInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ClientInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "send_table_crc",
                    CCLCMsg_ClientInfo::get_send_table_crc_for_reflect,
                    CCLCMsg_ClientInfo::mut_send_table_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_count",
                    CCLCMsg_ClientInfo::get_server_count_for_reflect,
                    CCLCMsg_ClientInfo::mut_server_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_hltv",
                    CCLCMsg_ClientInfo::get_is_hltv_for_reflect,
                    CCLCMsg_ClientInfo::mut_is_hltv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_replay",
                    CCLCMsg_ClientInfo::get_is_replay_for_reflect,
                    CCLCMsg_ClientInfo::mut_is_replay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "friends_id",
                    CCLCMsg_ClientInfo::get_friends_id_for_reflect,
                    CCLCMsg_ClientInfo::mut_friends_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "friends_name",
                    CCLCMsg_ClientInfo::get_friends_name_for_reflect,
                    CCLCMsg_ClientInfo::mut_friends_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ClientInfo>(
                    "CCLCMsg_ClientInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ClientInfo {
    fn clear(&mut self) {
        self.clear_send_table_crc();
        self.clear_server_count();
        self.clear_is_hltv();
        self.clear_is_replay();
        self.clear_friends_id();
        self.clear_friends_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_ClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_ClientInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_Move {
    // message fields
    num_backup_commands: ::std::option::Option<u32>,
    num_new_commands: ::std::option::Option<u32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_Move {}

impl CCLCMsg_Move {
    pub fn new() -> CCLCMsg_Move {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_Move {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_Move> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_Move,
        };
        unsafe {
            instance.get(CCLCMsg_Move::new)
        }
    }

    // optional uint32 num_backup_commands = 1;

    pub fn clear_num_backup_commands(&mut self) {
        self.num_backup_commands = ::std::option::Option::None;
    }

    pub fn has_num_backup_commands(&self) -> bool {
        self.num_backup_commands.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_backup_commands(&mut self, v: u32) {
        self.num_backup_commands = ::std::option::Option::Some(v);
    }

    pub fn get_num_backup_commands(&self) -> u32 {
        self.num_backup_commands.unwrap_or(0)
    }

    fn get_num_backup_commands_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_backup_commands
    }

    fn mut_num_backup_commands_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_backup_commands
    }

    // optional uint32 num_new_commands = 2;

    pub fn clear_num_new_commands(&mut self) {
        self.num_new_commands = ::std::option::Option::None;
    }

    pub fn has_num_new_commands(&self) -> bool {
        self.num_new_commands.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_new_commands(&mut self, v: u32) {
        self.num_new_commands = ::std::option::Option::Some(v);
    }

    pub fn get_num_new_commands(&self) -> u32 {
        self.num_new_commands.unwrap_or(0)
    }

    fn get_num_new_commands_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_new_commands
    }

    fn mut_num_new_commands_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_new_commands
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

impl ::protobuf::Message for CCLCMsg_Move {
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
                    let tmp = is.read_uint32()?;
                    self.num_backup_commands = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.num_new_commands = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.num_backup_commands {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_new_commands {
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
        if let Some(v) = self.num_backup_commands {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.num_new_commands {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_Move {
    fn new() -> CCLCMsg_Move {
        CCLCMsg_Move::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_Move>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_backup_commands",
                    CCLCMsg_Move::get_num_backup_commands_for_reflect,
                    CCLCMsg_Move::mut_num_backup_commands_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_new_commands",
                    CCLCMsg_Move::get_num_new_commands_for_reflect,
                    CCLCMsg_Move::mut_num_new_commands_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CCLCMsg_Move::get_data_for_reflect,
                    CCLCMsg_Move::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_Move>(
                    "CCLCMsg_Move",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_Move {
    fn clear(&mut self) {
        self.clear_num_backup_commands();
        self.clear_num_new_commands();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_Move {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_Move {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgVoiceAudio {
    // message fields
    format: ::std::option::Option<VoiceDataFormat_t>,
    voice_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    sequence_bytes: ::std::option::Option<i32>,
    section_number: ::std::option::Option<u32>,
    sample_rate: ::std::option::Option<u32>,
    uncompressed_sample_offset: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgVoiceAudio {}

impl CMsgVoiceAudio {
    pub fn new() -> CMsgVoiceAudio {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgVoiceAudio {
        static mut instance: ::protobuf::lazy::Lazy<CMsgVoiceAudio> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgVoiceAudio,
        };
        unsafe {
            instance.get(CMsgVoiceAudio::new)
        }
    }

    // optional .dota.VoiceDataFormat_t format = 1;

    pub fn clear_format(&mut self) {
        self.format = ::std::option::Option::None;
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: VoiceDataFormat_t) {
        self.format = ::std::option::Option::Some(v);
    }

    pub fn get_format(&self) -> VoiceDataFormat_t {
        self.format.unwrap_or(VoiceDataFormat_t::VOICEDATA_FORMAT_STEAM)
    }

    fn get_format_for_reflect(&self) -> &::std::option::Option<VoiceDataFormat_t> {
        &self.format
    }

    fn mut_format_for_reflect(&mut self) -> &mut ::std::option::Option<VoiceDataFormat_t> {
        &mut self.format
    }

    // optional bytes voice_data = 2;

    pub fn clear_voice_data(&mut self) {
        self.voice_data.clear();
    }

    pub fn has_voice_data(&self) -> bool {
        self.voice_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voice_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.voice_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_voice_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.voice_data.is_none() {
            self.voice_data.set_default();
        };
        self.voice_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_voice_data(&mut self) -> ::std::vec::Vec<u8> {
        self.voice_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_voice_data(&self) -> &[u8] {
        match self.voice_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_voice_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.voice_data
    }

    fn mut_voice_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.voice_data
    }

    // optional int32 sequence_bytes = 3;

    pub fn clear_sequence_bytes(&mut self) {
        self.sequence_bytes = ::std::option::Option::None;
    }

    pub fn has_sequence_bytes(&self) -> bool {
        self.sequence_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_bytes(&mut self, v: i32) {
        self.sequence_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_bytes(&self) -> i32 {
        self.sequence_bytes.unwrap_or(0)
    }

    fn get_sequence_bytes_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_bytes
    }

    fn mut_sequence_bytes_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_bytes
    }

    // optional uint32 section_number = 4;

    pub fn clear_section_number(&mut self) {
        self.section_number = ::std::option::Option::None;
    }

    pub fn has_section_number(&self) -> bool {
        self.section_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_section_number(&mut self, v: u32) {
        self.section_number = ::std::option::Option::Some(v);
    }

    pub fn get_section_number(&self) -> u32 {
        self.section_number.unwrap_or(0)
    }

    fn get_section_number_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.section_number
    }

    fn mut_section_number_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.section_number
    }

    // optional uint32 sample_rate = 5;

    pub fn clear_sample_rate(&mut self) {
        self.sample_rate = ::std::option::Option::None;
    }

    pub fn has_sample_rate(&self) -> bool {
        self.sample_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sample_rate(&mut self, v: u32) {
        self.sample_rate = ::std::option::Option::Some(v);
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate.unwrap_or(0)
    }

    fn get_sample_rate_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sample_rate
    }

    fn mut_sample_rate_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sample_rate
    }

    // optional uint32 uncompressed_sample_offset = 6;

    pub fn clear_uncompressed_sample_offset(&mut self) {
        self.uncompressed_sample_offset = ::std::option::Option::None;
    }

    pub fn has_uncompressed_sample_offset(&self) -> bool {
        self.uncompressed_sample_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uncompressed_sample_offset(&mut self, v: u32) {
        self.uncompressed_sample_offset = ::std::option::Option::Some(v);
    }

    pub fn get_uncompressed_sample_offset(&self) -> u32 {
        self.uncompressed_sample_offset.unwrap_or(0)
    }

    fn get_uncompressed_sample_offset_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.uncompressed_sample_offset
    }

    fn mut_uncompressed_sample_offset_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.uncompressed_sample_offset
    }
}

impl ::protobuf::Message for CMsgVoiceAudio {
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
                    let tmp = is.read_enum()?;
                    self.format = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.voice_data)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.sequence_bytes = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.section_number = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.sample_rate = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.uncompressed_sample_offset = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.format {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.voice_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.sequence_bytes {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.section_number {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sample_rate {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.uncompressed_sample_offset {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.format {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.voice_data.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.sequence_bytes {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.section_number {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.sample_rate {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.uncompressed_sample_offset {
            os.write_uint32(6, v)?;
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

impl ::protobuf::MessageStatic for CMsgVoiceAudio {
    fn new() -> CMsgVoiceAudio {
        CMsgVoiceAudio::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgVoiceAudio>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<VoiceDataFormat_t>>(
                    "format",
                    CMsgVoiceAudio::get_format_for_reflect,
                    CMsgVoiceAudio::mut_format_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "voice_data",
                    CMsgVoiceAudio::get_voice_data_for_reflect,
                    CMsgVoiceAudio::mut_voice_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_bytes",
                    CMsgVoiceAudio::get_sequence_bytes_for_reflect,
                    CMsgVoiceAudio::mut_sequence_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "section_number",
                    CMsgVoiceAudio::get_section_number_for_reflect,
                    CMsgVoiceAudio::mut_section_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sample_rate",
                    CMsgVoiceAudio::get_sample_rate_for_reflect,
                    CMsgVoiceAudio::mut_sample_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "uncompressed_sample_offset",
                    CMsgVoiceAudio::get_uncompressed_sample_offset_for_reflect,
                    CMsgVoiceAudio::mut_uncompressed_sample_offset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgVoiceAudio>(
                    "CMsgVoiceAudio",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgVoiceAudio {
    fn clear(&mut self) {
        self.clear_format();
        self.clear_voice_data();
        self.clear_sequence_bytes();
        self.clear_section_number();
        self.clear_sample_rate();
        self.clear_uncompressed_sample_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgVoiceAudio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgVoiceAudio {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_VoiceData {
    // message fields
    audio: ::protobuf::SingularPtrField<CMsgVoiceAudio>,
    xuid: ::std::option::Option<u64>,
    tick: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_VoiceData {}

impl CCLCMsg_VoiceData {
    pub fn new() -> CCLCMsg_VoiceData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_VoiceData {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_VoiceData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_VoiceData,
        };
        unsafe {
            instance.get(CCLCMsg_VoiceData::new)
        }
    }

    // optional .dota.CMsgVoiceAudio audio = 1;

    pub fn clear_audio(&mut self) {
        self.audio.clear();
    }

    pub fn has_audio(&self) -> bool {
        self.audio.is_some()
    }

    // Param is passed by value, moved
    pub fn set_audio(&mut self, v: CMsgVoiceAudio) {
        self.audio = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_audio(&mut self) -> &mut CMsgVoiceAudio {
        if self.audio.is_none() {
            self.audio.set_default();
        };
        self.audio.as_mut().unwrap()
    }

    // Take field
    pub fn take_audio(&mut self) -> CMsgVoiceAudio {
        self.audio.take().unwrap_or_else(|| CMsgVoiceAudio::new())
    }

    pub fn get_audio(&self) -> &CMsgVoiceAudio {
        self.audio.as_ref().unwrap_or_else(|| CMsgVoiceAudio::default_instance())
    }

    fn get_audio_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgVoiceAudio> {
        &self.audio
    }

    fn mut_audio_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgVoiceAudio> {
        &mut self.audio
    }

    // optional fixed64 xuid = 2;

    pub fn clear_xuid(&mut self) {
        self.xuid = ::std::option::Option::None;
    }

    pub fn has_xuid(&self) -> bool {
        self.xuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xuid(&mut self, v: u64) {
        self.xuid = ::std::option::Option::Some(v);
    }

    pub fn get_xuid(&self) -> u64 {
        self.xuid.unwrap_or(0)
    }

    fn get_xuid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.xuid
    }

    fn mut_xuid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.xuid
    }

    // optional uint32 tick = 3;

    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: u32) {
        self.tick = ::std::option::Option::Some(v);
    }

    pub fn get_tick(&self) -> u32 {
        self.tick.unwrap_or(0)
    }

    fn get_tick_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tick
    }

    fn mut_tick_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tick
    }
}

impl ::protobuf::Message for CCLCMsg_VoiceData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.audio)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.xuid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tick = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.audio.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.xuid {
            my_size += 9;
        };
        if let Some(v) = self.tick {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.audio.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.xuid {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.tick {
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

impl ::protobuf::MessageStatic for CCLCMsg_VoiceData {
    fn new() -> CCLCMsg_VoiceData {
        CCLCMsg_VoiceData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_VoiceData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgVoiceAudio>>(
                    "audio",
                    CCLCMsg_VoiceData::get_audio_for_reflect,
                    CCLCMsg_VoiceData::mut_audio_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "xuid",
                    CCLCMsg_VoiceData::get_xuid_for_reflect,
                    CCLCMsg_VoiceData::mut_xuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tick",
                    CCLCMsg_VoiceData::get_tick_for_reflect,
                    CCLCMsg_VoiceData::mut_tick_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_VoiceData>(
                    "CCLCMsg_VoiceData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_VoiceData {
    fn clear(&mut self) {
        self.clear_audio();
        self.clear_xuid();
        self.clear_tick();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_VoiceData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_VoiceData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_BaselineAck {
    // message fields
    baseline_tick: ::std::option::Option<i32>,
    baseline_nr: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_BaselineAck {}

impl CCLCMsg_BaselineAck {
    pub fn new() -> CCLCMsg_BaselineAck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_BaselineAck {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_BaselineAck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_BaselineAck,
        };
        unsafe {
            instance.get(CCLCMsg_BaselineAck::new)
        }
    }

    // optional int32 baseline_tick = 1;

    pub fn clear_baseline_tick(&mut self) {
        self.baseline_tick = ::std::option::Option::None;
    }

    pub fn has_baseline_tick(&self) -> bool {
        self.baseline_tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline_tick(&mut self, v: i32) {
        self.baseline_tick = ::std::option::Option::Some(v);
    }

    pub fn get_baseline_tick(&self) -> i32 {
        self.baseline_tick.unwrap_or(0)
    }

    fn get_baseline_tick_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.baseline_tick
    }

    fn mut_baseline_tick_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.baseline_tick
    }

    // optional int32 baseline_nr = 2;

    pub fn clear_baseline_nr(&mut self) {
        self.baseline_nr = ::std::option::Option::None;
    }

    pub fn has_baseline_nr(&self) -> bool {
        self.baseline_nr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline_nr(&mut self, v: i32) {
        self.baseline_nr = ::std::option::Option::Some(v);
    }

    pub fn get_baseline_nr(&self) -> i32 {
        self.baseline_nr.unwrap_or(0)
    }

    fn get_baseline_nr_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.baseline_nr
    }

    fn mut_baseline_nr_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.baseline_nr
    }
}

impl ::protobuf::Message for CCLCMsg_BaselineAck {
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
                    self.baseline_tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.baseline_nr = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.baseline_tick {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.baseline_nr {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.baseline_tick {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.baseline_nr {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_BaselineAck {
    fn new() -> CCLCMsg_BaselineAck {
        CCLCMsg_BaselineAck::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_BaselineAck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "baseline_tick",
                    CCLCMsg_BaselineAck::get_baseline_tick_for_reflect,
                    CCLCMsg_BaselineAck::mut_baseline_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "baseline_nr",
                    CCLCMsg_BaselineAck::get_baseline_nr_for_reflect,
                    CCLCMsg_BaselineAck::mut_baseline_nr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_BaselineAck>(
                    "CCLCMsg_BaselineAck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_BaselineAck {
    fn clear(&mut self) {
        self.clear_baseline_tick();
        self.clear_baseline_nr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_BaselineAck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_BaselineAck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_ListenEvents {
    // message fields
    event_mask: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ListenEvents {}

impl CCLCMsg_ListenEvents {
    pub fn new() -> CCLCMsg_ListenEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ListenEvents {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ListenEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ListenEvents,
        };
        unsafe {
            instance.get(CCLCMsg_ListenEvents::new)
        }
    }

    // repeated fixed32 event_mask = 1;

    pub fn clear_event_mask(&mut self) {
        self.event_mask.clear();
    }

    // Param is passed by value, moved
    pub fn set_event_mask(&mut self, v: ::std::vec::Vec<u32>) {
        self.event_mask = v;
    }

    // Mutable pointer to the field.
    pub fn mut_event_mask(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.event_mask
    }

    // Take field
    pub fn take_event_mask(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.event_mask, ::std::vec::Vec::new())
    }

    pub fn get_event_mask(&self) -> &[u32] {
        &self.event_mask
    }

    fn get_event_mask_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.event_mask
    }

    fn mut_event_mask_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.event_mask
    }
}

impl ::protobuf::Message for CCLCMsg_ListenEvents {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.event_mask)?;
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
        my_size += 5 * self.event_mask.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.event_mask {
            os.write_fixed32(1, *v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_ListenEvents {
    fn new() -> CCLCMsg_ListenEvents {
        CCLCMsg_ListenEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ListenEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "event_mask",
                    CCLCMsg_ListenEvents::get_event_mask_for_reflect,
                    CCLCMsg_ListenEvents::mut_event_mask_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ListenEvents>(
                    "CCLCMsg_ListenEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ListenEvents {
    fn clear(&mut self) {
        self.clear_event_mask();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_ListenEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_ListenEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_RespondCvarValue {
    // message fields
    cookie: ::std::option::Option<i32>,
    status_code: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_RespondCvarValue {}

impl CCLCMsg_RespondCvarValue {
    pub fn new() -> CCLCMsg_RespondCvarValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_RespondCvarValue {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_RespondCvarValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_RespondCvarValue,
        };
        unsafe {
            instance.get(CCLCMsg_RespondCvarValue::new)
        }
    }

    // optional int32 cookie = 1;

    pub fn clear_cookie(&mut self) {
        self.cookie = ::std::option::Option::None;
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: i32) {
        self.cookie = ::std::option::Option::Some(v);
    }

    pub fn get_cookie(&self) -> i32 {
        self.cookie.unwrap_or(0)
    }

    fn get_cookie_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.cookie
    }

    fn mut_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.cookie
    }

    // optional int32 status_code = 2;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    fn get_status_code_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.status_code
    }

    fn mut_status_code_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.status_code
    }

    // optional string name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string value = 4;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for CCLCMsg_RespondCvarValue {
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
                    self.cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
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
        if let Some(v) = self.cookie {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.status_code {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cookie {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.status_code {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_RespondCvarValue {
    fn new() -> CCLCMsg_RespondCvarValue {
        CCLCMsg_RespondCvarValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_RespondCvarValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cookie",
                    CCLCMsg_RespondCvarValue::get_cookie_for_reflect,
                    CCLCMsg_RespondCvarValue::mut_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "status_code",
                    CCLCMsg_RespondCvarValue::get_status_code_for_reflect,
                    CCLCMsg_RespondCvarValue::mut_status_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CCLCMsg_RespondCvarValue::get_name_for_reflect,
                    CCLCMsg_RespondCvarValue::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CCLCMsg_RespondCvarValue::get_value_for_reflect,
                    CCLCMsg_RespondCvarValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_RespondCvarValue>(
                    "CCLCMsg_RespondCvarValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_RespondCvarValue {
    fn clear(&mut self) {
        self.clear_cookie();
        self.clear_status_code();
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_RespondCvarValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_RespondCvarValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_FileCRCCheck {
    // message fields
    code_path: ::std::option::Option<i32>,
    path: ::protobuf::SingularField<::std::string::String>,
    code_filename: ::std::option::Option<i32>,
    filename: ::protobuf::SingularField<::std::string::String>,
    crc: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_FileCRCCheck {}

impl CCLCMsg_FileCRCCheck {
    pub fn new() -> CCLCMsg_FileCRCCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_FileCRCCheck {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_FileCRCCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_FileCRCCheck,
        };
        unsafe {
            instance.get(CCLCMsg_FileCRCCheck::new)
        }
    }

    // optional int32 code_path = 1;

    pub fn clear_code_path(&mut self) {
        self.code_path = ::std::option::Option::None;
    }

    pub fn has_code_path(&self) -> bool {
        self.code_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code_path(&mut self, v: i32) {
        self.code_path = ::std::option::Option::Some(v);
    }

    pub fn get_code_path(&self) -> i32 {
        self.code_path.unwrap_or(0)
    }

    fn get_code_path_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.code_path
    }

    fn mut_code_path_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.code_path
    }

    // optional string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // optional int32 code_filename = 3;

    pub fn clear_code_filename(&mut self) {
        self.code_filename = ::std::option::Option::None;
    }

    pub fn has_code_filename(&self) -> bool {
        self.code_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code_filename(&mut self, v: i32) {
        self.code_filename = ::std::option::Option::Some(v);
    }

    pub fn get_code_filename(&self) -> i32 {
        self.code_filename.unwrap_or(0)
    }

    fn get_code_filename_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.code_filename
    }

    fn mut_code_filename_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.code_filename
    }

    // optional string filename = 4;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    pub fn has_filename(&self) -> bool {
        self.filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if self.filename.is_none() {
            self.filename.set_default();
        };
        self.filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        self.filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        match self.filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_filename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.filename
    }

    fn mut_filename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.filename
    }

    // optional fixed32 crc = 5;

    pub fn clear_crc(&mut self) {
        self.crc = ::std::option::Option::None;
    }

    pub fn has_crc(&self) -> bool {
        self.crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crc(&mut self, v: u32) {
        self.crc = ::std::option::Option::Some(v);
    }

    pub fn get_crc(&self) -> u32 {
        self.crc.unwrap_or(0)
    }

    fn get_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.crc
    }

    fn mut_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.crc
    }
}

impl ::protobuf::Message for CCLCMsg_FileCRCCheck {
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
                    self.code_path = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.code_filename = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.crc = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.code_path {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.code_filename {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.crc {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code_path {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.path.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.code_filename {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.filename.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.crc {
            os.write_fixed32(5, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_FileCRCCheck {
    fn new() -> CCLCMsg_FileCRCCheck {
        CCLCMsg_FileCRCCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_FileCRCCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code_path",
                    CCLCMsg_FileCRCCheck::get_code_path_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_code_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    CCLCMsg_FileCRCCheck::get_path_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code_filename",
                    CCLCMsg_FileCRCCheck::get_code_filename_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_code_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    CCLCMsg_FileCRCCheck::get_filename_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "crc",
                    CCLCMsg_FileCRCCheck::get_crc_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_crc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_FileCRCCheck>(
                    "CCLCMsg_FileCRCCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_FileCRCCheck {
    fn clear(&mut self) {
        self.clear_code_path();
        self.clear_path();
        self.clear_code_filename();
        self.clear_filename();
        self.clear_crc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_FileCRCCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_FileCRCCheck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_LoadingProgress {
    // message fields
    progress: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_LoadingProgress {}

impl CCLCMsg_LoadingProgress {
    pub fn new() -> CCLCMsg_LoadingProgress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_LoadingProgress {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_LoadingProgress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_LoadingProgress,
        };
        unsafe {
            instance.get(CCLCMsg_LoadingProgress::new)
        }
    }

    // optional int32 progress = 1;

    pub fn clear_progress(&mut self) {
        self.progress = ::std::option::Option::None;
    }

    pub fn has_progress(&self) -> bool {
        self.progress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_progress(&mut self, v: i32) {
        self.progress = ::std::option::Option::Some(v);
    }

    pub fn get_progress(&self) -> i32 {
        self.progress.unwrap_or(0)
    }

    fn get_progress_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.progress
    }

    fn mut_progress_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.progress
    }
}

impl ::protobuf::Message for CCLCMsg_LoadingProgress {
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
                    self.progress = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.progress {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.progress {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_LoadingProgress {
    fn new() -> CCLCMsg_LoadingProgress {
        CCLCMsg_LoadingProgress::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_LoadingProgress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "progress",
                    CCLCMsg_LoadingProgress::get_progress_for_reflect,
                    CCLCMsg_LoadingProgress::mut_progress_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_LoadingProgress>(
                    "CCLCMsg_LoadingProgress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_LoadingProgress {
    fn clear(&mut self) {
        self.clear_progress();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_LoadingProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_LoadingProgress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_SplitPlayerConnect {
    // message fields
    playername: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_SplitPlayerConnect {}

impl CCLCMsg_SplitPlayerConnect {
    pub fn new() -> CCLCMsg_SplitPlayerConnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_SplitPlayerConnect {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_SplitPlayerConnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_SplitPlayerConnect,
        };
        unsafe {
            instance.get(CCLCMsg_SplitPlayerConnect::new)
        }
    }

    // optional string playername = 1;

    pub fn clear_playername(&mut self) {
        self.playername.clear();
    }

    pub fn has_playername(&self) -> bool {
        self.playername.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playername(&mut self, v: ::std::string::String) {
        self.playername = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_playername(&mut self) -> &mut ::std::string::String {
        if self.playername.is_none() {
            self.playername.set_default();
        };
        self.playername.as_mut().unwrap()
    }

    // Take field
    pub fn take_playername(&mut self) -> ::std::string::String {
        self.playername.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_playername(&self) -> &str {
        match self.playername.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_playername_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.playername
    }

    fn mut_playername_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.playername
    }
}

impl ::protobuf::Message for CCLCMsg_SplitPlayerConnect {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.playername)?;
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
        if let Some(v) = self.playername.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.playername.as_ref() {
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

impl ::protobuf::MessageStatic for CCLCMsg_SplitPlayerConnect {
    fn new() -> CCLCMsg_SplitPlayerConnect {
        CCLCMsg_SplitPlayerConnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_SplitPlayerConnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "playername",
                    CCLCMsg_SplitPlayerConnect::get_playername_for_reflect,
                    CCLCMsg_SplitPlayerConnect::mut_playername_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_SplitPlayerConnect>(
                    "CCLCMsg_SplitPlayerConnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_SplitPlayerConnect {
    fn clear(&mut self) {
        self.clear_playername();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_SplitPlayerConnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_SplitPlayerConnect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_ClientMessage {
    // message fields
    msg_type: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ClientMessage {}

impl CCLCMsg_ClientMessage {
    pub fn new() -> CCLCMsg_ClientMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ClientMessage {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ClientMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ClientMessage,
        };
        unsafe {
            instance.get(CCLCMsg_ClientMessage::new)
        }
    }

    // optional int32 msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: i32) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> i32 {
        self.msg_type.unwrap_or(0)
    }

    fn get_msg_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.msg_type
    }

    fn mut_msg_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.msg_type
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

impl ::protobuf::Message for CCLCMsg_ClientMessage {
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
                    self.msg_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.msg_type {
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
        if let Some(v) = self.msg_type {
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

impl ::protobuf::MessageStatic for CCLCMsg_ClientMessage {
    fn new() -> CCLCMsg_ClientMessage {
        CCLCMsg_ClientMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ClientMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "msg_type",
                    CCLCMsg_ClientMessage::get_msg_type_for_reflect,
                    CCLCMsg_ClientMessage::mut_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CCLCMsg_ClientMessage::get_data_for_reflect,
                    CCLCMsg_ClientMessage::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ClientMessage>(
                    "CCLCMsg_ClientMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ClientMessage {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_ClientMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_ClientMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_SplitPlayerDisconnect {
    // message fields
    slot: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_SplitPlayerDisconnect {}

impl CCLCMsg_SplitPlayerDisconnect {
    pub fn new() -> CCLCMsg_SplitPlayerDisconnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_SplitPlayerDisconnect {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_SplitPlayerDisconnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_SplitPlayerDisconnect,
        };
        unsafe {
            instance.get(CCLCMsg_SplitPlayerDisconnect::new)
        }
    }

    // optional int32 slot = 1;

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: i32) {
        self.slot = ::std::option::Option::Some(v);
    }

    pub fn get_slot(&self) -> i32 {
        self.slot.unwrap_or(0)
    }

    fn get_slot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slot
    }

    fn mut_slot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slot
    }
}

impl ::protobuf::Message for CCLCMsg_SplitPlayerDisconnect {
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
                    self.slot = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slot {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_SplitPlayerDisconnect {
    fn new() -> CCLCMsg_SplitPlayerDisconnect {
        CCLCMsg_SplitPlayerDisconnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_SplitPlayerDisconnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slot",
                    CCLCMsg_SplitPlayerDisconnect::get_slot_for_reflect,
                    CCLCMsg_SplitPlayerDisconnect::mut_slot_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_SplitPlayerDisconnect>(
                    "CCLCMsg_SplitPlayerDisconnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_SplitPlayerDisconnect {
    fn clear(&mut self) {
        self.clear_slot();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_SplitPlayerDisconnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_SplitPlayerDisconnect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_ServerStatus {
    // message fields
    simplified: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ServerStatus {}

impl CCLCMsg_ServerStatus {
    pub fn new() -> CCLCMsg_ServerStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ServerStatus {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ServerStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ServerStatus,
        };
        unsafe {
            instance.get(CCLCMsg_ServerStatus::new)
        }
    }

    // optional bool simplified = 1;

    pub fn clear_simplified(&mut self) {
        self.simplified = ::std::option::Option::None;
    }

    pub fn has_simplified(&self) -> bool {
        self.simplified.is_some()
    }

    // Param is passed by value, moved
    pub fn set_simplified(&mut self, v: bool) {
        self.simplified = ::std::option::Option::Some(v);
    }

    pub fn get_simplified(&self) -> bool {
        self.simplified.unwrap_or(false)
    }

    fn get_simplified_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.simplified
    }

    fn mut_simplified_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.simplified
    }
}

impl ::protobuf::Message for CCLCMsg_ServerStatus {
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
                    self.simplified = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.simplified {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.simplified {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_ServerStatus {
    fn new() -> CCLCMsg_ServerStatus {
        CCLCMsg_ServerStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ServerStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "simplified",
                    CCLCMsg_ServerStatus::get_simplified_for_reflect,
                    CCLCMsg_ServerStatus::mut_simplified_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ServerStatus>(
                    "CCLCMsg_ServerStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ServerStatus {
    fn clear(&mut self) {
        self.clear_simplified();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_ServerStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_ServerStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_ServerPing {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ServerPing {}

impl CCLCMsg_ServerPing {
    pub fn new() -> CCLCMsg_ServerPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ServerPing {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ServerPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ServerPing,
        };
        unsafe {
            instance.get(CCLCMsg_ServerPing::new)
        }
    }
}

impl ::protobuf::Message for CCLCMsg_ServerPing {
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

impl ::protobuf::MessageStatic for CCLCMsg_ServerPing {
    fn new() -> CCLCMsg_ServerPing {
        CCLCMsg_ServerPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ServerPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ServerPing>(
                    "CCLCMsg_ServerPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ServerPing {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_ServerPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_ServerPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_RequestPause {
    // message fields
    pause_type: ::std::option::Option<RequestPause_t>,
    pause_group: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_RequestPause {}

impl CCLCMsg_RequestPause {
    pub fn new() -> CCLCMsg_RequestPause {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_RequestPause {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_RequestPause> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_RequestPause,
        };
        unsafe {
            instance.get(CCLCMsg_RequestPause::new)
        }
    }

    // optional .dota.RequestPause_t pause_type = 1;

    pub fn clear_pause_type(&mut self) {
        self.pause_type = ::std::option::Option::None;
    }

    pub fn has_pause_type(&self) -> bool {
        self.pause_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pause_type(&mut self, v: RequestPause_t) {
        self.pause_type = ::std::option::Option::Some(v);
    }

    pub fn get_pause_type(&self) -> RequestPause_t {
        self.pause_type.unwrap_or(RequestPause_t::RP_PAUSE)
    }

    fn get_pause_type_for_reflect(&self) -> &::std::option::Option<RequestPause_t> {
        &self.pause_type
    }

    fn mut_pause_type_for_reflect(&mut self) -> &mut ::std::option::Option<RequestPause_t> {
        &mut self.pause_type
    }

    // optional int32 pause_group = 2;

    pub fn clear_pause_group(&mut self) {
        self.pause_group = ::std::option::Option::None;
    }

    pub fn has_pause_group(&self) -> bool {
        self.pause_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pause_group(&mut self, v: i32) {
        self.pause_group = ::std::option::Option::Some(v);
    }

    pub fn get_pause_group(&self) -> i32 {
        self.pause_group.unwrap_or(0)
    }

    fn get_pause_group_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.pause_group
    }

    fn mut_pause_group_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.pause_group
    }
}

impl ::protobuf::Message for CCLCMsg_RequestPause {
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
                    let tmp = is.read_enum()?;
                    self.pause_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.pause_group = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.pause_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.pause_group {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pause_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.pause_group {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_RequestPause {
    fn new() -> CCLCMsg_RequestPause {
        CCLCMsg_RequestPause::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_RequestPause>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RequestPause_t>>(
                    "pause_type",
                    CCLCMsg_RequestPause::get_pause_type_for_reflect,
                    CCLCMsg_RequestPause::mut_pause_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "pause_group",
                    CCLCMsg_RequestPause::get_pause_group_for_reflect,
                    CCLCMsg_RequestPause::mut_pause_group_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_RequestPause>(
                    "CCLCMsg_RequestPause",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_RequestPause {
    fn clear(&mut self) {
        self.clear_pause_type();
        self.clear_pause_group();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_RequestPause {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_RequestPause {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_CmdKeyValues {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_CmdKeyValues {}

impl CCLCMsg_CmdKeyValues {
    pub fn new() -> CCLCMsg_CmdKeyValues {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_CmdKeyValues {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_CmdKeyValues> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_CmdKeyValues,
        };
        unsafe {
            instance.get(CCLCMsg_CmdKeyValues::new)
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

impl ::protobuf::Message for CCLCMsg_CmdKeyValues {
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

impl ::protobuf::MessageStatic for CCLCMsg_CmdKeyValues {
    fn new() -> CCLCMsg_CmdKeyValues {
        CCLCMsg_CmdKeyValues::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_CmdKeyValues>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CCLCMsg_CmdKeyValues::get_data_for_reflect,
                    CCLCMsg_CmdKeyValues::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_CmdKeyValues>(
                    "CCLCMsg_CmdKeyValues",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_CmdKeyValues {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_CmdKeyValues {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_CmdKeyValues {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_ServerInfo {
    // message fields
    protocol: ::std::option::Option<i32>,
    server_count: ::std::option::Option<i32>,
    is_dedicated: ::std::option::Option<bool>,
    is_hltv: ::std::option::Option<bool>,
    is_replay: ::std::option::Option<bool>,
    c_os: ::std::option::Option<i32>,
    map_crc: ::std::option::Option<u32>,
    client_crc: ::std::option::Option<u32>,
    string_table_crc: ::std::option::Option<u32>,
    max_clients: ::std::option::Option<i32>,
    max_classes: ::std::option::Option<i32>,
    player_slot: ::std::option::Option<i32>,
    tick_interval: ::std::option::Option<f32>,
    game_dir: ::protobuf::SingularField<::std::string::String>,
    map_name: ::protobuf::SingularField<::std::string::String>,
    sky_name: ::protobuf::SingularField<::std::string::String>,
    host_name: ::protobuf::SingularField<::std::string::String>,
    addon_name: ::protobuf::SingularField<::std::string::String>,
    game_session_config: ::protobuf::SingularPtrField<super::networkbasetypes::CSVCMsg_GameSessionConfiguration>,
    game_session_manifest: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ServerInfo {}

impl CSVCMsg_ServerInfo {
    pub fn new() -> CSVCMsg_ServerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ServerInfo {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ServerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ServerInfo,
        };
        unsafe {
            instance.get(CSVCMsg_ServerInfo::new)
        }
    }

    // optional int32 protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol = ::std::option::Option::None;
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: i32) {
        self.protocol = ::std::option::Option::Some(v);
    }

    pub fn get_protocol(&self) -> i32 {
        self.protocol.unwrap_or(0)
    }

    fn get_protocol_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.protocol
    }

    // optional int32 server_count = 2;

    pub fn clear_server_count(&mut self) {
        self.server_count = ::std::option::Option::None;
    }

    pub fn has_server_count(&self) -> bool {
        self.server_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_count(&mut self, v: i32) {
        self.server_count = ::std::option::Option::Some(v);
    }

    pub fn get_server_count(&self) -> i32 {
        self.server_count.unwrap_or(0)
    }

    fn get_server_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.server_count
    }

    fn mut_server_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.server_count
    }

    // optional bool is_dedicated = 3;

    pub fn clear_is_dedicated(&mut self) {
        self.is_dedicated = ::std::option::Option::None;
    }

    pub fn has_is_dedicated(&self) -> bool {
        self.is_dedicated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_dedicated(&mut self, v: bool) {
        self.is_dedicated = ::std::option::Option::Some(v);
    }

    pub fn get_is_dedicated(&self) -> bool {
        self.is_dedicated.unwrap_or(false)
    }

    fn get_is_dedicated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_dedicated
    }

    fn mut_is_dedicated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_dedicated
    }

    // optional bool is_hltv = 4;

    pub fn clear_is_hltv(&mut self) {
        self.is_hltv = ::std::option::Option::None;
    }

    pub fn has_is_hltv(&self) -> bool {
        self.is_hltv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_hltv(&mut self, v: bool) {
        self.is_hltv = ::std::option::Option::Some(v);
    }

    pub fn get_is_hltv(&self) -> bool {
        self.is_hltv.unwrap_or(false)
    }

    fn get_is_hltv_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_hltv
    }

    fn mut_is_hltv_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_hltv
    }

    // optional bool is_replay = 5;

    pub fn clear_is_replay(&mut self) {
        self.is_replay = ::std::option::Option::None;
    }

    pub fn has_is_replay(&self) -> bool {
        self.is_replay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_replay(&mut self, v: bool) {
        self.is_replay = ::std::option::Option::Some(v);
    }

    pub fn get_is_replay(&self) -> bool {
        self.is_replay.unwrap_or(false)
    }

    fn get_is_replay_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_replay
    }

    fn mut_is_replay_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_replay
    }

    // optional int32 c_os = 6;

    pub fn clear_c_os(&mut self) {
        self.c_os = ::std::option::Option::None;
    }

    pub fn has_c_os(&self) -> bool {
        self.c_os.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c_os(&mut self, v: i32) {
        self.c_os = ::std::option::Option::Some(v);
    }

    pub fn get_c_os(&self) -> i32 {
        self.c_os.unwrap_or(0)
    }

    fn get_c_os_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.c_os
    }

    fn mut_c_os_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.c_os
    }

    // optional fixed32 map_crc = 7;

    pub fn clear_map_crc(&mut self) {
        self.map_crc = ::std::option::Option::None;
    }

    pub fn has_map_crc(&self) -> bool {
        self.map_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_crc(&mut self, v: u32) {
        self.map_crc = ::std::option::Option::Some(v);
    }

    pub fn get_map_crc(&self) -> u32 {
        self.map_crc.unwrap_or(0)
    }

    fn get_map_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.map_crc
    }

    fn mut_map_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.map_crc
    }

    // optional fixed32 client_crc = 8;

    pub fn clear_client_crc(&mut self) {
        self.client_crc = ::std::option::Option::None;
    }

    pub fn has_client_crc(&self) -> bool {
        self.client_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_crc(&mut self, v: u32) {
        self.client_crc = ::std::option::Option::Some(v);
    }

    pub fn get_client_crc(&self) -> u32 {
        self.client_crc.unwrap_or(0)
    }

    fn get_client_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_crc
    }

    fn mut_client_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_crc
    }

    // optional fixed32 string_table_crc = 9;

    pub fn clear_string_table_crc(&mut self) {
        self.string_table_crc = ::std::option::Option::None;
    }

    pub fn has_string_table_crc(&self) -> bool {
        self.string_table_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_table_crc(&mut self, v: u32) {
        self.string_table_crc = ::std::option::Option::Some(v);
    }

    pub fn get_string_table_crc(&self) -> u32 {
        self.string_table_crc.unwrap_or(0)
    }

    fn get_string_table_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.string_table_crc
    }

    fn mut_string_table_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.string_table_crc
    }

    // optional int32 max_clients = 10;

    pub fn clear_max_clients(&mut self) {
        self.max_clients = ::std::option::Option::None;
    }

    pub fn has_max_clients(&self) -> bool {
        self.max_clients.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_clients(&mut self, v: i32) {
        self.max_clients = ::std::option::Option::Some(v);
    }

    pub fn get_max_clients(&self) -> i32 {
        self.max_clients.unwrap_or(0)
    }

    fn get_max_clients_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_clients
    }

    fn mut_max_clients_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_clients
    }

    // optional int32 max_classes = 11;

    pub fn clear_max_classes(&mut self) {
        self.max_classes = ::std::option::Option::None;
    }

    pub fn has_max_classes(&self) -> bool {
        self.max_classes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_classes(&mut self, v: i32) {
        self.max_classes = ::std::option::Option::Some(v);
    }

    pub fn get_max_classes(&self) -> i32 {
        self.max_classes.unwrap_or(0)
    }

    fn get_max_classes_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_classes
    }

    fn mut_max_classes_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_classes
    }

    // optional int32 player_slot = 12;

    pub fn clear_player_slot(&mut self) {
        self.player_slot = ::std::option::Option::None;
    }

    pub fn has_player_slot(&self) -> bool {
        self.player_slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_slot(&mut self, v: i32) {
        self.player_slot = ::std::option::Option::Some(v);
    }

    pub fn get_player_slot(&self) -> i32 {
        self.player_slot.unwrap_or(0)
    }

    fn get_player_slot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_slot
    }

    fn mut_player_slot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_slot
    }

    // optional float tick_interval = 13;

    pub fn clear_tick_interval(&mut self) {
        self.tick_interval = ::std::option::Option::None;
    }

    pub fn has_tick_interval(&self) -> bool {
        self.tick_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick_interval(&mut self, v: f32) {
        self.tick_interval = ::std::option::Option::Some(v);
    }

    pub fn get_tick_interval(&self) -> f32 {
        self.tick_interval.unwrap_or(0.)
    }

    fn get_tick_interval_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.tick_interval
    }

    fn mut_tick_interval_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.tick_interval
    }

    // optional string game_dir = 14;

    pub fn clear_game_dir(&mut self) {
        self.game_dir.clear();
    }

    pub fn has_game_dir(&self) -> bool {
        self.game_dir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_dir(&mut self, v: ::std::string::String) {
        self.game_dir = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_dir(&mut self) -> &mut ::std::string::String {
        if self.game_dir.is_none() {
            self.game_dir.set_default();
        };
        self.game_dir.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_dir(&mut self) -> ::std::string::String {
        self.game_dir.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_dir(&self) -> &str {
        match self.game_dir.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_dir_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_dir
    }

    fn mut_game_dir_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_dir
    }

    // optional string map_name = 15;

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

    // optional string sky_name = 16;

    pub fn clear_sky_name(&mut self) {
        self.sky_name.clear();
    }

    pub fn has_sky_name(&self) -> bool {
        self.sky_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sky_name(&mut self, v: ::std::string::String) {
        self.sky_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sky_name(&mut self) -> &mut ::std::string::String {
        if self.sky_name.is_none() {
            self.sky_name.set_default();
        };
        self.sky_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_sky_name(&mut self) -> ::std::string::String {
        self.sky_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sky_name(&self) -> &str {
        match self.sky_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sky_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sky_name
    }

    fn mut_sky_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sky_name
    }

    // optional string host_name = 17;

    pub fn clear_host_name(&mut self) {
        self.host_name.clear();
    }

    pub fn has_host_name(&self) -> bool {
        self.host_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_name(&mut self, v: ::std::string::String) {
        self.host_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host_name(&mut self) -> &mut ::std::string::String {
        if self.host_name.is_none() {
            self.host_name.set_default();
        };
        self.host_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_host_name(&mut self) -> ::std::string::String {
        self.host_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_host_name(&self) -> &str {
        match self.host_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_host_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.host_name
    }

    fn mut_host_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.host_name
    }

    // optional string addon_name = 18;

    pub fn clear_addon_name(&mut self) {
        self.addon_name.clear();
    }

    pub fn has_addon_name(&self) -> bool {
        self.addon_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addon_name(&mut self, v: ::std::string::String) {
        self.addon_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addon_name(&mut self) -> &mut ::std::string::String {
        if self.addon_name.is_none() {
            self.addon_name.set_default();
        };
        self.addon_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_addon_name(&mut self) -> ::std::string::String {
        self.addon_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_addon_name(&self) -> &str {
        match self.addon_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_addon_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.addon_name
    }

    fn mut_addon_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.addon_name
    }

    // optional .dota.CSVCMsg_GameSessionConfiguration game_session_config = 19;

    pub fn clear_game_session_config(&mut self) {
        self.game_session_config.clear();
    }

    pub fn has_game_session_config(&self) -> bool {
        self.game_session_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_session_config(&mut self, v: super::networkbasetypes::CSVCMsg_GameSessionConfiguration) {
        self.game_session_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_session_config(&mut self) -> &mut super::networkbasetypes::CSVCMsg_GameSessionConfiguration {
        if self.game_session_config.is_none() {
            self.game_session_config.set_default();
        };
        self.game_session_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_session_config(&mut self) -> super::networkbasetypes::CSVCMsg_GameSessionConfiguration {
        self.game_session_config.take().unwrap_or_else(|| super::networkbasetypes::CSVCMsg_GameSessionConfiguration::new())
    }

    pub fn get_game_session_config(&self) -> &super::networkbasetypes::CSVCMsg_GameSessionConfiguration {
        self.game_session_config.as_ref().unwrap_or_else(|| super::networkbasetypes::CSVCMsg_GameSessionConfiguration::default_instance())
    }

    fn get_game_session_config_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CSVCMsg_GameSessionConfiguration> {
        &self.game_session_config
    }

    fn mut_game_session_config_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CSVCMsg_GameSessionConfiguration> {
        &mut self.game_session_config
    }

    // optional bytes game_session_manifest = 20;

    pub fn clear_game_session_manifest(&mut self) {
        self.game_session_manifest.clear();
    }

    pub fn has_game_session_manifest(&self) -> bool {
        self.game_session_manifest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_session_manifest(&mut self, v: ::std::vec::Vec<u8>) {
        self.game_session_manifest = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_session_manifest(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.game_session_manifest.is_none() {
            self.game_session_manifest.set_default();
        };
        self.game_session_manifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_session_manifest(&mut self) -> ::std::vec::Vec<u8> {
        self.game_session_manifest.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_game_session_manifest(&self) -> &[u8] {
        match self.game_session_manifest.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_game_session_manifest_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.game_session_manifest
    }

    fn mut_game_session_manifest_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.game_session_manifest
    }
}

impl ::protobuf::Message for CSVCMsg_ServerInfo {
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
                    self.protocol = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.server_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_dedicated = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_hltv = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_replay = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.c_os = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.map_crc = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_crc = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.string_table_crc = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.max_clients = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.max_classes = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.player_slot = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.tick_interval = ::std::option::Option::Some(tmp);
                },
                14 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_dir)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sky_name)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.host_name)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.addon_name)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.game_session_config)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.game_session_manifest)?;
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
        if let Some(v) = self.protocol {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.server_count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.is_dedicated {
            my_size += 2;
        };
        if let Some(v) = self.is_hltv {
            my_size += 2;
        };
        if let Some(v) = self.is_replay {
            my_size += 2;
        };
        if let Some(v) = self.c_os {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.map_crc {
            my_size += 5;
        };
        if let Some(v) = self.client_crc {
            my_size += 5;
        };
        if let Some(v) = self.string_table_crc {
            my_size += 5;
        };
        if let Some(v) = self.max_clients {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.max_classes {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.player_slot {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tick_interval {
            my_size += 5;
        };
        if let Some(v) = self.game_dir.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        };
        if let Some(v) = self.map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        };
        if let Some(v) = self.sky_name.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        };
        if let Some(v) = self.host_name.as_ref() {
            my_size += ::protobuf::rt::string_size(17, &v);
        };
        if let Some(v) = self.addon_name.as_ref() {
            my_size += ::protobuf::rt::string_size(18, &v);
        };
        if let Some(v) = self.game_session_config.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.game_session_manifest.as_ref() {
            my_size += ::protobuf::rt::bytes_size(20, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.protocol {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.server_count {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.is_dedicated {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.is_hltv {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.is_replay {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.c_os {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.map_crc {
            os.write_fixed32(7, v)?;
        };
        if let Some(v) = self.client_crc {
            os.write_fixed32(8, v)?;
        };
        if let Some(v) = self.string_table_crc {
            os.write_fixed32(9, v)?;
        };
        if let Some(v) = self.max_clients {
            os.write_int32(10, v)?;
        };
        if let Some(v) = self.max_classes {
            os.write_int32(11, v)?;
        };
        if let Some(v) = self.player_slot {
            os.write_int32(12, v)?;
        };
        if let Some(v) = self.tick_interval {
            os.write_float(13, v)?;
        };
        if let Some(v) = self.game_dir.as_ref() {
            os.write_string(14, &v)?;
        };
        if let Some(v) = self.map_name.as_ref() {
            os.write_string(15, &v)?;
        };
        if let Some(v) = self.sky_name.as_ref() {
            os.write_string(16, &v)?;
        };
        if let Some(v) = self.host_name.as_ref() {
            os.write_string(17, &v)?;
        };
        if let Some(v) = self.addon_name.as_ref() {
            os.write_string(18, &v)?;
        };
        if let Some(v) = self.game_session_config.as_ref() {
            os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.game_session_manifest.as_ref() {
            os.write_bytes(20, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_ServerInfo {
    fn new() -> CSVCMsg_ServerInfo {
        CSVCMsg_ServerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ServerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "protocol",
                    CSVCMsg_ServerInfo::get_protocol_for_reflect,
                    CSVCMsg_ServerInfo::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "server_count",
                    CSVCMsg_ServerInfo::get_server_count_for_reflect,
                    CSVCMsg_ServerInfo::mut_server_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_dedicated",
                    CSVCMsg_ServerInfo::get_is_dedicated_for_reflect,
                    CSVCMsg_ServerInfo::mut_is_dedicated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_hltv",
                    CSVCMsg_ServerInfo::get_is_hltv_for_reflect,
                    CSVCMsg_ServerInfo::mut_is_hltv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_replay",
                    CSVCMsg_ServerInfo::get_is_replay_for_reflect,
                    CSVCMsg_ServerInfo::mut_is_replay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "c_os",
                    CSVCMsg_ServerInfo::get_c_os_for_reflect,
                    CSVCMsg_ServerInfo::mut_c_os_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "map_crc",
                    CSVCMsg_ServerInfo::get_map_crc_for_reflect,
                    CSVCMsg_ServerInfo::mut_map_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_crc",
                    CSVCMsg_ServerInfo::get_client_crc_for_reflect,
                    CSVCMsg_ServerInfo::mut_client_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "string_table_crc",
                    CSVCMsg_ServerInfo::get_string_table_crc_for_reflect,
                    CSVCMsg_ServerInfo::mut_string_table_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_clients",
                    CSVCMsg_ServerInfo::get_max_clients_for_reflect,
                    CSVCMsg_ServerInfo::mut_max_clients_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_classes",
                    CSVCMsg_ServerInfo::get_max_classes_for_reflect,
                    CSVCMsg_ServerInfo::mut_max_classes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_slot",
                    CSVCMsg_ServerInfo::get_player_slot_for_reflect,
                    CSVCMsg_ServerInfo::mut_player_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "tick_interval",
                    CSVCMsg_ServerInfo::get_tick_interval_for_reflect,
                    CSVCMsg_ServerInfo::mut_tick_interval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_dir",
                    CSVCMsg_ServerInfo::get_game_dir_for_reflect,
                    CSVCMsg_ServerInfo::mut_game_dir_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_name",
                    CSVCMsg_ServerInfo::get_map_name_for_reflect,
                    CSVCMsg_ServerInfo::mut_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sky_name",
                    CSVCMsg_ServerInfo::get_sky_name_for_reflect,
                    CSVCMsg_ServerInfo::mut_sky_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host_name",
                    CSVCMsg_ServerInfo::get_host_name_for_reflect,
                    CSVCMsg_ServerInfo::mut_host_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addon_name",
                    CSVCMsg_ServerInfo::get_addon_name_for_reflect,
                    CSVCMsg_ServerInfo::mut_addon_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CSVCMsg_GameSessionConfiguration>>(
                    "game_session_config",
                    CSVCMsg_ServerInfo::get_game_session_config_for_reflect,
                    CSVCMsg_ServerInfo::mut_game_session_config_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "game_session_manifest",
                    CSVCMsg_ServerInfo::get_game_session_manifest_for_reflect,
                    CSVCMsg_ServerInfo::mut_game_session_manifest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ServerInfo>(
                    "CSVCMsg_ServerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ServerInfo {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_server_count();
        self.clear_is_dedicated();
        self.clear_is_hltv();
        self.clear_is_replay();
        self.clear_c_os();
        self.clear_map_crc();
        self.clear_client_crc();
        self.clear_string_table_crc();
        self.clear_max_clients();
        self.clear_max_classes();
        self.clear_player_slot();
        self.clear_tick_interval();
        self.clear_game_dir();
        self.clear_map_name();
        self.clear_sky_name();
        self.clear_host_name();
        self.clear_addon_name();
        self.clear_game_session_config();
        self.clear_game_session_manifest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_ServerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_ServerInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_ClassInfo {
    // message fields
    create_on_client: ::std::option::Option<bool>,
    classes: ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ClassInfo {}

impl CSVCMsg_ClassInfo {
    pub fn new() -> CSVCMsg_ClassInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ClassInfo {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ClassInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ClassInfo,
        };
        unsafe {
            instance.get(CSVCMsg_ClassInfo::new)
        }
    }

    // optional bool create_on_client = 1;

    pub fn clear_create_on_client(&mut self) {
        self.create_on_client = ::std::option::Option::None;
    }

    pub fn has_create_on_client(&self) -> bool {
        self.create_on_client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_on_client(&mut self, v: bool) {
        self.create_on_client = ::std::option::Option::Some(v);
    }

    pub fn get_create_on_client(&self) -> bool {
        self.create_on_client.unwrap_or(false)
    }

    fn get_create_on_client_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.create_on_client
    }

    fn mut_create_on_client_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.create_on_client
    }

    // repeated .dota.CSVCMsg_ClassInfo.class_t classes = 2;

    pub fn clear_classes(&mut self) {
        self.classes.clear();
    }

    // Param is passed by value, moved
    pub fn set_classes(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t>) {
        self.classes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classes(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        &mut self.classes
    }

    // Take field
    pub fn take_classes(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        ::std::mem::replace(&mut self.classes, ::protobuf::RepeatedField::new())
    }

    pub fn get_classes(&self) -> &[CSVCMsg_ClassInfo_class_t] {
        &self.classes
    }

    fn get_classes_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        &self.classes
    }

    fn mut_classes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        &mut self.classes
    }
}

impl ::protobuf::Message for CSVCMsg_ClassInfo {
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
                    self.create_on_client = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        if let Some(v) = self.create_on_client {
            my_size += 2;
        };
        for value in &self.classes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.create_on_client {
            os.write_bool(1, v)?;
        };
        for v in &self.classes {
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

impl ::protobuf::MessageStatic for CSVCMsg_ClassInfo {
    fn new() -> CSVCMsg_ClassInfo {
        CSVCMsg_ClassInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ClassInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "create_on_client",
                    CSVCMsg_ClassInfo::get_create_on_client_for_reflect,
                    CSVCMsg_ClassInfo::mut_create_on_client_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_ClassInfo_class_t>>(
                    "classes",
                    CSVCMsg_ClassInfo::get_classes_for_reflect,
                    CSVCMsg_ClassInfo::mut_classes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ClassInfo>(
                    "CSVCMsg_ClassInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ClassInfo {
    fn clear(&mut self) {
        self.clear_create_on_client();
        self.clear_classes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_ClassInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_ClassInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_ClassInfo_class_t {
    // message fields
    class_id: ::std::option::Option<i32>,
    data_table_name: ::protobuf::SingularField<::std::string::String>,
    class_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ClassInfo_class_t {}

impl CSVCMsg_ClassInfo_class_t {
    pub fn new() -> CSVCMsg_ClassInfo_class_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ClassInfo_class_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ClassInfo_class_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ClassInfo_class_t,
        };
        unsafe {
            instance.get(CSVCMsg_ClassInfo_class_t::new)
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

    // optional string data_table_name = 2;

    pub fn clear_data_table_name(&mut self) {
        self.data_table_name.clear();
    }

    pub fn has_data_table_name(&self) -> bool {
        self.data_table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_table_name(&mut self, v: ::std::string::String) {
        self.data_table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_table_name(&mut self) -> &mut ::std::string::String {
        if self.data_table_name.is_none() {
            self.data_table_name.set_default();
        };
        self.data_table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_data_table_name(&mut self) -> ::std::string::String {
        self.data_table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data_table_name(&self) -> &str {
        match self.data_table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data_table_name
    }

    fn mut_data_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data_table_name
    }

    // optional string class_name = 3;

    pub fn clear_class_name(&mut self) {
        self.class_name.clear();
    }

    pub fn has_class_name(&self) -> bool {
        self.class_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_class_name(&mut self, v: ::std::string::String) {
        self.class_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_class_name(&mut self) -> &mut ::std::string::String {
        if self.class_name.is_none() {
            self.class_name.set_default();
        };
        self.class_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_class_name(&mut self) -> ::std::string::String {
        self.class_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_class_name(&self) -> &str {
        match self.class_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_class_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.class_name
    }

    fn mut_class_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.class_name
    }
}

impl ::protobuf::Message for CSVCMsg_ClassInfo_class_t {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data_table_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.class_name)?;
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
        if let Some(v) = self.data_table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.class_name.as_ref() {
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
        if let Some(v) = self.data_table_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.class_name.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_ClassInfo_class_t {
    fn new() -> CSVCMsg_ClassInfo_class_t {
        CSVCMsg_ClassInfo_class_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ClassInfo_class_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "class_id",
                    CSVCMsg_ClassInfo_class_t::get_class_id_for_reflect,
                    CSVCMsg_ClassInfo_class_t::mut_class_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data_table_name",
                    CSVCMsg_ClassInfo_class_t::get_data_table_name_for_reflect,
                    CSVCMsg_ClassInfo_class_t::mut_data_table_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "class_name",
                    CSVCMsg_ClassInfo_class_t::get_class_name_for_reflect,
                    CSVCMsg_ClassInfo_class_t::mut_class_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ClassInfo_class_t>(
                    "CSVCMsg_ClassInfo_class_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ClassInfo_class_t {
    fn clear(&mut self) {
        self.clear_class_id();
        self.clear_data_table_name();
        self.clear_class_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_ClassInfo_class_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_ClassInfo_class_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SetPause {
    // message fields
    paused: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SetPause {}

impl CSVCMsg_SetPause {
    pub fn new() -> CSVCMsg_SetPause {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SetPause {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SetPause> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SetPause,
        };
        unsafe {
            instance.get(CSVCMsg_SetPause::new)
        }
    }

    // optional bool paused = 1;

    pub fn clear_paused(&mut self) {
        self.paused = ::std::option::Option::None;
    }

    pub fn has_paused(&self) -> bool {
        self.paused.is_some()
    }

    // Param is passed by value, moved
    pub fn set_paused(&mut self, v: bool) {
        self.paused = ::std::option::Option::Some(v);
    }

    pub fn get_paused(&self) -> bool {
        self.paused.unwrap_or(false)
    }

    fn get_paused_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.paused
    }

    fn mut_paused_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.paused
    }
}

impl ::protobuf::Message for CSVCMsg_SetPause {
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
                    self.paused = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.paused {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.paused {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_SetPause {
    fn new() -> CSVCMsg_SetPause {
        CSVCMsg_SetPause::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SetPause>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "paused",
                    CSVCMsg_SetPause::get_paused_for_reflect,
                    CSVCMsg_SetPause::mut_paused_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SetPause>(
                    "CSVCMsg_SetPause",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SetPause {
    fn clear(&mut self) {
        self.clear_paused();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SetPause {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SetPause {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_VoiceInit {
    // message fields
    quality: ::std::option::Option<i32>,
    codec: ::protobuf::SingularField<::std::string::String>,
    version: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_VoiceInit {}

impl CSVCMsg_VoiceInit {
    pub fn new() -> CSVCMsg_VoiceInit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_VoiceInit {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_VoiceInit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_VoiceInit,
        };
        unsafe {
            instance.get(CSVCMsg_VoiceInit::new)
        }
    }

    // optional int32 quality = 1;

    pub fn clear_quality(&mut self) {
        self.quality = ::std::option::Option::None;
    }

    pub fn has_quality(&self) -> bool {
        self.quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality(&mut self, v: i32) {
        self.quality = ::std::option::Option::Some(v);
    }

    pub fn get_quality(&self) -> i32 {
        self.quality.unwrap_or(0)
    }

    fn get_quality_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.quality
    }

    fn mut_quality_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.quality
    }

    // optional string codec = 2;

    pub fn clear_codec(&mut self) {
        self.codec.clear();
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: ::std::string::String) {
        self.codec = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codec(&mut self) -> &mut ::std::string::String {
        if self.codec.is_none() {
            self.codec.set_default();
        };
        self.codec.as_mut().unwrap()
    }

    // Take field
    pub fn take_codec(&mut self) -> ::std::string::String {
        self.codec.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_codec(&self) -> &str {
        match self.codec.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_codec_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.codec
    }

    fn mut_codec_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.codec
    }

    // optional int32 version = 3;

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
        self.version.unwrap_or(0i32)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.version
    }
}

impl ::protobuf::Message for CSVCMsg_VoiceInit {
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
                    self.quality = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.codec)?;
                },
                3 => {
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
        if let Some(v) = self.quality {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.codec.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quality {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.codec.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.version {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_VoiceInit {
    fn new() -> CSVCMsg_VoiceInit {
        CSVCMsg_VoiceInit::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_VoiceInit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "quality",
                    CSVCMsg_VoiceInit::get_quality_for_reflect,
                    CSVCMsg_VoiceInit::mut_quality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "codec",
                    CSVCMsg_VoiceInit::get_codec_for_reflect,
                    CSVCMsg_VoiceInit::mut_codec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    CSVCMsg_VoiceInit::get_version_for_reflect,
                    CSVCMsg_VoiceInit::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_VoiceInit>(
                    "CSVCMsg_VoiceInit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_VoiceInit {
    fn clear(&mut self) {
        self.clear_quality();
        self.clear_codec();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_VoiceInit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_VoiceInit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Print {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Print {}

impl CSVCMsg_Print {
    pub fn new() -> CSVCMsg_Print {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Print {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Print> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Print,
        };
        unsafe {
            instance.get(CSVCMsg_Print::new)
        }
    }

    // optional string text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.text
    }
}

impl ::protobuf::Message for CSVCMsg_Print {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
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
        if let Some(v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.text.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_Print {
    fn new() -> CSVCMsg_Print {
        CSVCMsg_Print::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Print>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CSVCMsg_Print::get_text_for_reflect,
                    CSVCMsg_Print::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Print>(
                    "CSVCMsg_Print",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Print {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Print {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Print {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Sounds {
    // message fields
    reliable_sound: ::std::option::Option<bool>,
    sounds: ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Sounds {}

impl CSVCMsg_Sounds {
    pub fn new() -> CSVCMsg_Sounds {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Sounds {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Sounds> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Sounds,
        };
        unsafe {
            instance.get(CSVCMsg_Sounds::new)
        }
    }

    // optional bool reliable_sound = 1;

    pub fn clear_reliable_sound(&mut self) {
        self.reliable_sound = ::std::option::Option::None;
    }

    pub fn has_reliable_sound(&self) -> bool {
        self.reliable_sound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable_sound(&mut self, v: bool) {
        self.reliable_sound = ::std::option::Option::Some(v);
    }

    pub fn get_reliable_sound(&self) -> bool {
        self.reliable_sound.unwrap_or(false)
    }

    fn get_reliable_sound_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.reliable_sound
    }

    fn mut_reliable_sound_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.reliable_sound
    }

    // repeated .dota.CSVCMsg_Sounds.sounddata_t sounds = 2;

    pub fn clear_sounds(&mut self) {
        self.sounds.clear();
    }

    // Param is passed by value, moved
    pub fn set_sounds(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t>) {
        self.sounds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sounds(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        &mut self.sounds
    }

    // Take field
    pub fn take_sounds(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        ::std::mem::replace(&mut self.sounds, ::protobuf::RepeatedField::new())
    }

    pub fn get_sounds(&self) -> &[CSVCMsg_Sounds_sounddata_t] {
        &self.sounds
    }

    fn get_sounds_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        &self.sounds
    }

    fn mut_sounds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        &mut self.sounds
    }
}

impl ::protobuf::Message for CSVCMsg_Sounds {
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
                    self.reliable_sound = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sounds)?;
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
        if let Some(v) = self.reliable_sound {
            my_size += 2;
        };
        for value in &self.sounds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reliable_sound {
            os.write_bool(1, v)?;
        };
        for v in &self.sounds {
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

impl ::protobuf::MessageStatic for CSVCMsg_Sounds {
    fn new() -> CSVCMsg_Sounds {
        CSVCMsg_Sounds::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Sounds>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reliable_sound",
                    CSVCMsg_Sounds::get_reliable_sound_for_reflect,
                    CSVCMsg_Sounds::mut_reliable_sound_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_Sounds_sounddata_t>>(
                    "sounds",
                    CSVCMsg_Sounds::get_sounds_for_reflect,
                    CSVCMsg_Sounds::mut_sounds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Sounds>(
                    "CSVCMsg_Sounds",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Sounds {
    fn clear(&mut self) {
        self.clear_reliable_sound();
        self.clear_sounds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Sounds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Sounds {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Sounds_sounddata_t {
    // message fields
    origin_x: ::std::option::Option<i32>,
    origin_y: ::std::option::Option<i32>,
    origin_z: ::std::option::Option<i32>,
    volume: ::std::option::Option<u32>,
    delay_value: ::std::option::Option<f32>,
    sequence_number: ::std::option::Option<i32>,
    entity_index: ::std::option::Option<i32>,
    channel: ::std::option::Option<i32>,
    pitch: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    sound_num: ::std::option::Option<u32>,
    sound_num_handle: ::std::option::Option<u32>,
    speaker_entity: ::std::option::Option<i32>,
    random_seed: ::std::option::Option<i32>,
    sound_level: ::std::option::Option<i32>,
    is_sentence: ::std::option::Option<bool>,
    is_ambient: ::std::option::Option<bool>,
    guid: ::std::option::Option<u32>,
    sound_resource_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Sounds_sounddata_t {}

impl CSVCMsg_Sounds_sounddata_t {
    pub fn new() -> CSVCMsg_Sounds_sounddata_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Sounds_sounddata_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Sounds_sounddata_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Sounds_sounddata_t,
        };
        unsafe {
            instance.get(CSVCMsg_Sounds_sounddata_t::new)
        }
    }

    // optional sint32 origin_x = 1;

    pub fn clear_origin_x(&mut self) {
        self.origin_x = ::std::option::Option::None;
    }

    pub fn has_origin_x(&self) -> bool {
        self.origin_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_x(&mut self, v: i32) {
        self.origin_x = ::std::option::Option::Some(v);
    }

    pub fn get_origin_x(&self) -> i32 {
        self.origin_x.unwrap_or(0)
    }

    fn get_origin_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.origin_x
    }

    fn mut_origin_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.origin_x
    }

    // optional sint32 origin_y = 2;

    pub fn clear_origin_y(&mut self) {
        self.origin_y = ::std::option::Option::None;
    }

    pub fn has_origin_y(&self) -> bool {
        self.origin_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_y(&mut self, v: i32) {
        self.origin_y = ::std::option::Option::Some(v);
    }

    pub fn get_origin_y(&self) -> i32 {
        self.origin_y.unwrap_or(0)
    }

    fn get_origin_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.origin_y
    }

    fn mut_origin_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.origin_y
    }

    // optional sint32 origin_z = 3;

    pub fn clear_origin_z(&mut self) {
        self.origin_z = ::std::option::Option::None;
    }

    pub fn has_origin_z(&self) -> bool {
        self.origin_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_z(&mut self, v: i32) {
        self.origin_z = ::std::option::Option::Some(v);
    }

    pub fn get_origin_z(&self) -> i32 {
        self.origin_z.unwrap_or(0)
    }

    fn get_origin_z_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.origin_z
    }

    fn mut_origin_z_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.origin_z
    }

    // optional uint32 volume = 4;

    pub fn clear_volume(&mut self) {
        self.volume = ::std::option::Option::None;
    }

    pub fn has_volume(&self) -> bool {
        self.volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: u32) {
        self.volume = ::std::option::Option::Some(v);
    }

    pub fn get_volume(&self) -> u32 {
        self.volume.unwrap_or(0)
    }

    fn get_volume_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.volume
    }

    fn mut_volume_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.volume
    }

    // optional float delay_value = 5;

    pub fn clear_delay_value(&mut self) {
        self.delay_value = ::std::option::Option::None;
    }

    pub fn has_delay_value(&self) -> bool {
        self.delay_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay_value(&mut self, v: f32) {
        self.delay_value = ::std::option::Option::Some(v);
    }

    pub fn get_delay_value(&self) -> f32 {
        self.delay_value.unwrap_or(0.)
    }

    fn get_delay_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.delay_value
    }

    fn mut_delay_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.delay_value
    }

    // optional int32 sequence_number = 6;

    pub fn clear_sequence_number(&mut self) {
        self.sequence_number = ::std::option::Option::None;
    }

    pub fn has_sequence_number(&self) -> bool {
        self.sequence_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_number(&mut self, v: i32) {
        self.sequence_number = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_number(&self) -> i32 {
        self.sequence_number.unwrap_or(0)
    }

    fn get_sequence_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_number
    }

    fn mut_sequence_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_number
    }

    // optional int32 entity_index = 7;

    pub fn clear_entity_index(&mut self) {
        self.entity_index = ::std::option::Option::None;
    }

    pub fn has_entity_index(&self) -> bool {
        self.entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_index(&mut self, v: i32) {
        self.entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_entity_index(&self) -> i32 {
        self.entity_index.unwrap_or(0)
    }

    fn get_entity_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_index
    }

    fn mut_entity_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_index
    }

    // optional int32 channel = 8;

    pub fn clear_channel(&mut self) {
        self.channel = ::std::option::Option::None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: i32) {
        self.channel = ::std::option::Option::Some(v);
    }

    pub fn get_channel(&self) -> i32 {
        self.channel.unwrap_or(0)
    }

    fn get_channel_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.channel
    }

    // optional int32 pitch = 9;

    pub fn clear_pitch(&mut self) {
        self.pitch = ::std::option::Option::None;
    }

    pub fn has_pitch(&self) -> bool {
        self.pitch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pitch(&mut self, v: i32) {
        self.pitch = ::std::option::Option::Some(v);
    }

    pub fn get_pitch(&self) -> i32 {
        self.pitch.unwrap_or(0)
    }

    fn get_pitch_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.pitch
    }

    fn mut_pitch_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.pitch
    }

    // optional int32 flags = 10;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: i32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> i32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.flags
    }

    // optional uint32 sound_num = 11;

    pub fn clear_sound_num(&mut self) {
        self.sound_num = ::std::option::Option::None;
    }

    pub fn has_sound_num(&self) -> bool {
        self.sound_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_num(&mut self, v: u32) {
        self.sound_num = ::std::option::Option::Some(v);
    }

    pub fn get_sound_num(&self) -> u32 {
        self.sound_num.unwrap_or(0)
    }

    fn get_sound_num_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sound_num
    }

    fn mut_sound_num_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sound_num
    }

    // optional fixed32 sound_num_handle = 12;

    pub fn clear_sound_num_handle(&mut self) {
        self.sound_num_handle = ::std::option::Option::None;
    }

    pub fn has_sound_num_handle(&self) -> bool {
        self.sound_num_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_num_handle(&mut self, v: u32) {
        self.sound_num_handle = ::std::option::Option::Some(v);
    }

    pub fn get_sound_num_handle(&self) -> u32 {
        self.sound_num_handle.unwrap_or(0)
    }

    fn get_sound_num_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sound_num_handle
    }

    fn mut_sound_num_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sound_num_handle
    }

    // optional int32 speaker_entity = 13;

    pub fn clear_speaker_entity(&mut self) {
        self.speaker_entity = ::std::option::Option::None;
    }

    pub fn has_speaker_entity(&self) -> bool {
        self.speaker_entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speaker_entity(&mut self, v: i32) {
        self.speaker_entity = ::std::option::Option::Some(v);
    }

    pub fn get_speaker_entity(&self) -> i32 {
        self.speaker_entity.unwrap_or(0)
    }

    fn get_speaker_entity_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.speaker_entity
    }

    fn mut_speaker_entity_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.speaker_entity
    }

    // optional int32 random_seed = 14;

    pub fn clear_random_seed(&mut self) {
        self.random_seed = ::std::option::Option::None;
    }

    pub fn has_random_seed(&self) -> bool {
        self.random_seed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_random_seed(&mut self, v: i32) {
        self.random_seed = ::std::option::Option::Some(v);
    }

    pub fn get_random_seed(&self) -> i32 {
        self.random_seed.unwrap_or(0)
    }

    fn get_random_seed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.random_seed
    }

    fn mut_random_seed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.random_seed
    }

    // optional int32 sound_level = 15;

    pub fn clear_sound_level(&mut self) {
        self.sound_level = ::std::option::Option::None;
    }

    pub fn has_sound_level(&self) -> bool {
        self.sound_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_level(&mut self, v: i32) {
        self.sound_level = ::std::option::Option::Some(v);
    }

    pub fn get_sound_level(&self) -> i32 {
        self.sound_level.unwrap_or(0)
    }

    fn get_sound_level_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sound_level
    }

    fn mut_sound_level_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sound_level
    }

    // optional bool is_sentence = 16;

    pub fn clear_is_sentence(&mut self) {
        self.is_sentence = ::std::option::Option::None;
    }

    pub fn has_is_sentence(&self) -> bool {
        self.is_sentence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_sentence(&mut self, v: bool) {
        self.is_sentence = ::std::option::Option::Some(v);
    }

    pub fn get_is_sentence(&self) -> bool {
        self.is_sentence.unwrap_or(false)
    }

    fn get_is_sentence_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_sentence
    }

    fn mut_is_sentence_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_sentence
    }

    // optional bool is_ambient = 17;

    pub fn clear_is_ambient(&mut self) {
        self.is_ambient = ::std::option::Option::None;
    }

    pub fn has_is_ambient(&self) -> bool {
        self.is_ambient.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_ambient(&mut self, v: bool) {
        self.is_ambient = ::std::option::Option::Some(v);
    }

    pub fn get_is_ambient(&self) -> bool {
        self.is_ambient.unwrap_or(false)
    }

    fn get_is_ambient_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_ambient
    }

    fn mut_is_ambient_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_ambient
    }

    // optional uint32 guid = 18;

    pub fn clear_guid(&mut self) {
        self.guid = ::std::option::Option::None;
    }

    pub fn has_guid(&self) -> bool {
        self.guid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guid(&mut self, v: u32) {
        self.guid = ::std::option::Option::Some(v);
    }

    pub fn get_guid(&self) -> u32 {
        self.guid.unwrap_or(0)
    }

    fn get_guid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guid
    }

    fn mut_guid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guid
    }

    // optional fixed64 sound_resource_id = 19;

    pub fn clear_sound_resource_id(&mut self) {
        self.sound_resource_id = ::std::option::Option::None;
    }

    pub fn has_sound_resource_id(&self) -> bool {
        self.sound_resource_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_resource_id(&mut self, v: u64) {
        self.sound_resource_id = ::std::option::Option::Some(v);
    }

    pub fn get_sound_resource_id(&self) -> u64 {
        self.sound_resource_id.unwrap_or(0)
    }

    fn get_sound_resource_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sound_resource_id
    }

    fn mut_sound_resource_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sound_resource_id
    }
}

impl ::protobuf::Message for CSVCMsg_Sounds_sounddata_t {
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
                    let tmp = is.read_sint32()?;
                    self.origin_x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint32()?;
                    self.origin_y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint32()?;
                    self.origin_z = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.volume = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.delay_value = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.sequence_number = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.entity_index = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.channel = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.pitch = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.sound_num = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.sound_num_handle = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.speaker_entity = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.random_seed = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.sound_level = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_sentence = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_ambient = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.guid = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.sound_resource_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_x {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        };
        if let Some(v) = self.origin_y {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
        };
        if let Some(v) = self.origin_z {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        };
        if let Some(v) = self.volume {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.delay_value {
            my_size += 5;
        };
        if let Some(v) = self.sequence_number {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entity_index {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.channel {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pitch {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sound_num {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sound_num_handle {
            my_size += 5;
        };
        if let Some(v) = self.speaker_entity {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.random_seed {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sound_level {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.is_sentence {
            my_size += 3;
        };
        if let Some(v) = self.is_ambient {
            my_size += 3;
        };
        if let Some(v) = self.guid {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sound_resource_id {
            my_size += 10;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_x {
            os.write_sint32(1, v)?;
        };
        if let Some(v) = self.origin_y {
            os.write_sint32(2, v)?;
        };
        if let Some(v) = self.origin_z {
            os.write_sint32(3, v)?;
        };
        if let Some(v) = self.volume {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.delay_value {
            os.write_float(5, v)?;
        };
        if let Some(v) = self.sequence_number {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.entity_index {
            os.write_int32(7, v)?;
        };
        if let Some(v) = self.channel {
            os.write_int32(8, v)?;
        };
        if let Some(v) = self.pitch {
            os.write_int32(9, v)?;
        };
        if let Some(v) = self.flags {
            os.write_int32(10, v)?;
        };
        if let Some(v) = self.sound_num {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.sound_num_handle {
            os.write_fixed32(12, v)?;
        };
        if let Some(v) = self.speaker_entity {
            os.write_int32(13, v)?;
        };
        if let Some(v) = self.random_seed {
            os.write_int32(14, v)?;
        };
        if let Some(v) = self.sound_level {
            os.write_int32(15, v)?;
        };
        if let Some(v) = self.is_sentence {
            os.write_bool(16, v)?;
        };
        if let Some(v) = self.is_ambient {
            os.write_bool(17, v)?;
        };
        if let Some(v) = self.guid {
            os.write_uint32(18, v)?;
        };
        if let Some(v) = self.sound_resource_id {
            os.write_fixed64(19, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_Sounds_sounddata_t {
    fn new() -> CSVCMsg_Sounds_sounddata_t {
        CSVCMsg_Sounds_sounddata_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Sounds_sounddata_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "origin_x",
                    CSVCMsg_Sounds_sounddata_t::get_origin_x_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_origin_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "origin_y",
                    CSVCMsg_Sounds_sounddata_t::get_origin_y_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_origin_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "origin_z",
                    CSVCMsg_Sounds_sounddata_t::get_origin_z_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_origin_z_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "volume",
                    CSVCMsg_Sounds_sounddata_t::get_volume_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_volume_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "delay_value",
                    CSVCMsg_Sounds_sounddata_t::get_delay_value_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_delay_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_number",
                    CSVCMsg_Sounds_sounddata_t::get_sequence_number_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sequence_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_index",
                    CSVCMsg_Sounds_sounddata_t::get_entity_index_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_entity_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "channel",
                    CSVCMsg_Sounds_sounddata_t::get_channel_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "pitch",
                    CSVCMsg_Sounds_sounddata_t::get_pitch_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_pitch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CSVCMsg_Sounds_sounddata_t::get_flags_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sound_num",
                    CSVCMsg_Sounds_sounddata_t::get_sound_num_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sound_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "sound_num_handle",
                    CSVCMsg_Sounds_sounddata_t::get_sound_num_handle_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sound_num_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "speaker_entity",
                    CSVCMsg_Sounds_sounddata_t::get_speaker_entity_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_speaker_entity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "random_seed",
                    CSVCMsg_Sounds_sounddata_t::get_random_seed_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_random_seed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sound_level",
                    CSVCMsg_Sounds_sounddata_t::get_sound_level_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sound_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_sentence",
                    CSVCMsg_Sounds_sounddata_t::get_is_sentence_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_is_sentence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_ambient",
                    CSVCMsg_Sounds_sounddata_t::get_is_ambient_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_is_ambient_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guid",
                    CSVCMsg_Sounds_sounddata_t::get_guid_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_guid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "sound_resource_id",
                    CSVCMsg_Sounds_sounddata_t::get_sound_resource_id_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sound_resource_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Sounds_sounddata_t>(
                    "CSVCMsg_Sounds_sounddata_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Sounds_sounddata_t {
    fn clear(&mut self) {
        self.clear_origin_x();
        self.clear_origin_y();
        self.clear_origin_z();
        self.clear_volume();
        self.clear_delay_value();
        self.clear_sequence_number();
        self.clear_entity_index();
        self.clear_channel();
        self.clear_pitch();
        self.clear_flags();
        self.clear_sound_num();
        self.clear_sound_num_handle();
        self.clear_speaker_entity();
        self.clear_random_seed();
        self.clear_sound_level();
        self.clear_is_sentence();
        self.clear_is_ambient();
        self.clear_guid();
        self.clear_sound_resource_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Sounds_sounddata_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Sounds_sounddata_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Prefetch {
    // message fields
    sound_index: ::std::option::Option<i32>,
    resource_type: ::std::option::Option<PrefetchType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Prefetch {}

impl CSVCMsg_Prefetch {
    pub fn new() -> CSVCMsg_Prefetch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Prefetch {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Prefetch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Prefetch,
        };
        unsafe {
            instance.get(CSVCMsg_Prefetch::new)
        }
    }

    // optional int32 sound_index = 1;

    pub fn clear_sound_index(&mut self) {
        self.sound_index = ::std::option::Option::None;
    }

    pub fn has_sound_index(&self) -> bool {
        self.sound_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_index(&mut self, v: i32) {
        self.sound_index = ::std::option::Option::Some(v);
    }

    pub fn get_sound_index(&self) -> i32 {
        self.sound_index.unwrap_or(0)
    }

    fn get_sound_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sound_index
    }

    fn mut_sound_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sound_index
    }

    // optional .dota.PrefetchType resource_type = 2;

    pub fn clear_resource_type(&mut self) {
        self.resource_type = ::std::option::Option::None;
    }

    pub fn has_resource_type(&self) -> bool {
        self.resource_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resource_type(&mut self, v: PrefetchType) {
        self.resource_type = ::std::option::Option::Some(v);
    }

    pub fn get_resource_type(&self) -> PrefetchType {
        self.resource_type.unwrap_or(PrefetchType::PFT_SOUND)
    }

    fn get_resource_type_for_reflect(&self) -> &::std::option::Option<PrefetchType> {
        &self.resource_type
    }

    fn mut_resource_type_for_reflect(&mut self) -> &mut ::std::option::Option<PrefetchType> {
        &mut self.resource_type
    }
}

impl ::protobuf::Message for CSVCMsg_Prefetch {
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
                    self.sound_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.resource_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.sound_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.resource_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sound_index {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.resource_type {
            os.write_enum(2, v.value())?;
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

impl ::protobuf::MessageStatic for CSVCMsg_Prefetch {
    fn new() -> CSVCMsg_Prefetch {
        CSVCMsg_Prefetch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Prefetch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sound_index",
                    CSVCMsg_Prefetch::get_sound_index_for_reflect,
                    CSVCMsg_Prefetch::mut_sound_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PrefetchType>>(
                    "resource_type",
                    CSVCMsg_Prefetch::get_resource_type_for_reflect,
                    CSVCMsg_Prefetch::mut_resource_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Prefetch>(
                    "CSVCMsg_Prefetch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Prefetch {
    fn clear(&mut self) {
        self.clear_sound_index();
        self.clear_resource_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Prefetch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Prefetch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SetView {
    // message fields
    entity_index: ::std::option::Option<i32>,
    slot: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SetView {}

impl CSVCMsg_SetView {
    pub fn new() -> CSVCMsg_SetView {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SetView {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SetView> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SetView,
        };
        unsafe {
            instance.get(CSVCMsg_SetView::new)
        }
    }

    // optional int32 entity_index = 1;

    pub fn clear_entity_index(&mut self) {
        self.entity_index = ::std::option::Option::None;
    }

    pub fn has_entity_index(&self) -> bool {
        self.entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_index(&mut self, v: i32) {
        self.entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_entity_index(&self) -> i32 {
        self.entity_index.unwrap_or(0)
    }

    fn get_entity_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_index
    }

    fn mut_entity_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_index
    }

    // optional int32 slot = 2;

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: i32) {
        self.slot = ::std::option::Option::Some(v);
    }

    pub fn get_slot(&self) -> i32 {
        self.slot.unwrap_or(0)
    }

    fn get_slot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slot
    }

    fn mut_slot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slot
    }
}

impl ::protobuf::Message for CSVCMsg_SetView {
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
                    self.entity_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.slot = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.entity_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entity_index {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.slot {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_SetView {
    fn new() -> CSVCMsg_SetView {
        CSVCMsg_SetView::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SetView>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_index",
                    CSVCMsg_SetView::get_entity_index_for_reflect,
                    CSVCMsg_SetView::mut_entity_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slot",
                    CSVCMsg_SetView::get_slot_for_reflect,
                    CSVCMsg_SetView::mut_slot_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SetView>(
                    "CSVCMsg_SetView",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SetView {
    fn clear(&mut self) {
        self.clear_entity_index();
        self.clear_slot();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SetView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SetView {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_FixAngle {
    // message fields
    relative: ::std::option::Option<bool>,
    angle: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_FixAngle {}

impl CSVCMsg_FixAngle {
    pub fn new() -> CSVCMsg_FixAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_FixAngle {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_FixAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_FixAngle,
        };
        unsafe {
            instance.get(CSVCMsg_FixAngle::new)
        }
    }

    // optional bool relative = 1;

    pub fn clear_relative(&mut self) {
        self.relative = ::std::option::Option::None;
    }

    pub fn has_relative(&self) -> bool {
        self.relative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relative(&mut self, v: bool) {
        self.relative = ::std::option::Option::Some(v);
    }

    pub fn get_relative(&self) -> bool {
        self.relative.unwrap_or(false)
    }

    fn get_relative_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.relative
    }

    fn mut_relative_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.relative
    }

    // optional .dota.CMsgQAngle angle = 2;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angle(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angle.is_none() {
            self.angle.set_default();
        };
        self.angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_angle(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angle.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angle(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angle.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angle_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angle
    }
}

impl ::protobuf::Message for CSVCMsg_FixAngle {
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
                    self.relative = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angle)?;
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
        if let Some(v) = self.relative {
            my_size += 2;
        };
        if let Some(v) = self.angle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.relative {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.angle.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_FixAngle {
    fn new() -> CSVCMsg_FixAngle {
        CSVCMsg_FixAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_FixAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "relative",
                    CSVCMsg_FixAngle::get_relative_for_reflect,
                    CSVCMsg_FixAngle::mut_relative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angle",
                    CSVCMsg_FixAngle::get_angle_for_reflect,
                    CSVCMsg_FixAngle::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_FixAngle>(
                    "CSVCMsg_FixAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_FixAngle {
    fn clear(&mut self) {
        self.clear_relative();
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_FixAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_FixAngle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_CrosshairAngle {
    // message fields
    angle: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_CrosshairAngle {}

impl CSVCMsg_CrosshairAngle {
    pub fn new() -> CSVCMsg_CrosshairAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_CrosshairAngle {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_CrosshairAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_CrosshairAngle,
        };
        unsafe {
            instance.get(CSVCMsg_CrosshairAngle::new)
        }
    }

    // optional .dota.CMsgQAngle angle = 1;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angle(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angle.is_none() {
            self.angle.set_default();
        };
        self.angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_angle(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angle.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angle(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angle.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angle_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angle
    }
}

impl ::protobuf::Message for CSVCMsg_CrosshairAngle {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angle)?;
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
        if let Some(v) = self.angle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.angle.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_CrosshairAngle {
    fn new() -> CSVCMsg_CrosshairAngle {
        CSVCMsg_CrosshairAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_CrosshairAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angle",
                    CSVCMsg_CrosshairAngle::get_angle_for_reflect,
                    CSVCMsg_CrosshairAngle::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_CrosshairAngle>(
                    "CSVCMsg_CrosshairAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_CrosshairAngle {
    fn clear(&mut self) {
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_CrosshairAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_CrosshairAngle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_BSPDecal {
    // message fields
    pos: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    decal_texture_index: ::std::option::Option<i32>,
    entity_index: ::std::option::Option<i32>,
    model_index: ::std::option::Option<i32>,
    low_priority: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_BSPDecal {}

impl CSVCMsg_BSPDecal {
    pub fn new() -> CSVCMsg_BSPDecal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_BSPDecal {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_BSPDecal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_BSPDecal,
        };
        unsafe {
            instance.get(CSVCMsg_BSPDecal::new)
        }
    }

    // optional .dota.CMsgVector pos = 1;

    pub fn clear_pos(&mut self) {
        self.pos.clear();
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.pos.is_none() {
            self.pos.set_default();
        };
        self.pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_pos(&mut self) -> super::networkbasetypes::CMsgVector {
        self.pos.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_pos(&self) -> &super::networkbasetypes::CMsgVector {
        self.pos.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.pos
    }

    // optional int32 decal_texture_index = 2;

    pub fn clear_decal_texture_index(&mut self) {
        self.decal_texture_index = ::std::option::Option::None;
    }

    pub fn has_decal_texture_index(&self) -> bool {
        self.decal_texture_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decal_texture_index(&mut self, v: i32) {
        self.decal_texture_index = ::std::option::Option::Some(v);
    }

    pub fn get_decal_texture_index(&self) -> i32 {
        self.decal_texture_index.unwrap_or(0)
    }

    fn get_decal_texture_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.decal_texture_index
    }

    fn mut_decal_texture_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.decal_texture_index
    }

    // optional int32 entity_index = 3;

    pub fn clear_entity_index(&mut self) {
        self.entity_index = ::std::option::Option::None;
    }

    pub fn has_entity_index(&self) -> bool {
        self.entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_index(&mut self, v: i32) {
        self.entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_entity_index(&self) -> i32 {
        self.entity_index.unwrap_or(0)
    }

    fn get_entity_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_index
    }

    fn mut_entity_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_index
    }

    // optional int32 model_index = 4;

    pub fn clear_model_index(&mut self) {
        self.model_index = ::std::option::Option::None;
    }

    pub fn has_model_index(&self) -> bool {
        self.model_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_index(&mut self, v: i32) {
        self.model_index = ::std::option::Option::Some(v);
    }

    pub fn get_model_index(&self) -> i32 {
        self.model_index.unwrap_or(0)
    }

    fn get_model_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.model_index
    }

    fn mut_model_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.model_index
    }

    // optional bool low_priority = 5;

    pub fn clear_low_priority(&mut self) {
        self.low_priority = ::std::option::Option::None;
    }

    pub fn has_low_priority(&self) -> bool {
        self.low_priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_priority(&mut self, v: bool) {
        self.low_priority = ::std::option::Option::Some(v);
    }

    pub fn get_low_priority(&self) -> bool {
        self.low_priority.unwrap_or(false)
    }

    fn get_low_priority_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.low_priority
    }

    fn mut_low_priority_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.low_priority
    }
}

impl ::protobuf::Message for CSVCMsg_BSPDecal {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pos)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.decal_texture_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.entity_index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.model_index = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.low_priority = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.decal_texture_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entity_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.model_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.low_priority {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pos.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.decal_texture_index {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.entity_index {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.model_index {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.low_priority {
            os.write_bool(5, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_BSPDecal {
    fn new() -> CSVCMsg_BSPDecal {
        CSVCMsg_BSPDecal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_BSPDecal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "pos",
                    CSVCMsg_BSPDecal::get_pos_for_reflect,
                    CSVCMsg_BSPDecal::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "decal_texture_index",
                    CSVCMsg_BSPDecal::get_decal_texture_index_for_reflect,
                    CSVCMsg_BSPDecal::mut_decal_texture_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_index",
                    CSVCMsg_BSPDecal::get_entity_index_for_reflect,
                    CSVCMsg_BSPDecal::mut_entity_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "model_index",
                    CSVCMsg_BSPDecal::get_model_index_for_reflect,
                    CSVCMsg_BSPDecal::mut_model_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "low_priority",
                    CSVCMsg_BSPDecal::get_low_priority_for_reflect,
                    CSVCMsg_BSPDecal::mut_low_priority_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_BSPDecal>(
                    "CSVCMsg_BSPDecal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_BSPDecal {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_decal_texture_index();
        self.clear_entity_index();
        self.clear_model_index();
        self.clear_low_priority();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_BSPDecal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_BSPDecal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SplitScreen {
    // message fields
    field_type: ::std::option::Option<ESplitScreenMessageType>,
    slot: ::std::option::Option<i32>,
    player_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SplitScreen {}

impl CSVCMsg_SplitScreen {
    pub fn new() -> CSVCMsg_SplitScreen {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SplitScreen {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SplitScreen> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SplitScreen,
        };
        unsafe {
            instance.get(CSVCMsg_SplitScreen::new)
        }
    }

    // optional .dota.ESplitScreenMessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ESplitScreenMessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ESplitScreenMessageType {
        self.field_type.unwrap_or(ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<ESplitScreenMessageType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<ESplitScreenMessageType> {
        &mut self.field_type
    }

    // optional int32 slot = 2;

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: i32) {
        self.slot = ::std::option::Option::Some(v);
    }

    pub fn get_slot(&self) -> i32 {
        self.slot.unwrap_or(0)
    }

    fn get_slot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slot
    }

    fn mut_slot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slot
    }

    // optional int32 player_index = 3;

    pub fn clear_player_index(&mut self) {
        self.player_index = ::std::option::Option::None;
    }

    pub fn has_player_index(&self) -> bool {
        self.player_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_index(&mut self, v: i32) {
        self.player_index = ::std::option::Option::Some(v);
    }

    pub fn get_player_index(&self) -> i32 {
        self.player_index.unwrap_or(0)
    }

    fn get_player_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_index
    }

    fn mut_player_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_index
    }
}

impl ::protobuf::Message for CSVCMsg_SplitScreen {
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
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.slot = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.player_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.player_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.slot {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.player_index {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_SplitScreen {
    fn new() -> CSVCMsg_SplitScreen {
        CSVCMsg_SplitScreen::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SplitScreen>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ESplitScreenMessageType>>(
                    "type",
                    CSVCMsg_SplitScreen::get_field_type_for_reflect,
                    CSVCMsg_SplitScreen::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slot",
                    CSVCMsg_SplitScreen::get_slot_for_reflect,
                    CSVCMsg_SplitScreen::mut_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_index",
                    CSVCMsg_SplitScreen::get_player_index_for_reflect,
                    CSVCMsg_SplitScreen::mut_player_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SplitScreen>(
                    "CSVCMsg_SplitScreen",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SplitScreen {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_slot();
        self.clear_player_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SplitScreen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SplitScreen {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GetCvarValue {
    // message fields
    cookie: ::std::option::Option<i32>,
    cvar_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GetCvarValue {}

impl CSVCMsg_GetCvarValue {
    pub fn new() -> CSVCMsg_GetCvarValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GetCvarValue {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GetCvarValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GetCvarValue,
        };
        unsafe {
            instance.get(CSVCMsg_GetCvarValue::new)
        }
    }

    // optional int32 cookie = 1;

    pub fn clear_cookie(&mut self) {
        self.cookie = ::std::option::Option::None;
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: i32) {
        self.cookie = ::std::option::Option::Some(v);
    }

    pub fn get_cookie(&self) -> i32 {
        self.cookie.unwrap_or(0)
    }

    fn get_cookie_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.cookie
    }

    fn mut_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.cookie
    }

    // optional string cvar_name = 2;

    pub fn clear_cvar_name(&mut self) {
        self.cvar_name.clear();
    }

    pub fn has_cvar_name(&self) -> bool {
        self.cvar_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cvar_name(&mut self, v: ::std::string::String) {
        self.cvar_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cvar_name(&mut self) -> &mut ::std::string::String {
        if self.cvar_name.is_none() {
            self.cvar_name.set_default();
        };
        self.cvar_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_cvar_name(&mut self) -> ::std::string::String {
        self.cvar_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cvar_name(&self) -> &str {
        match self.cvar_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cvar_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cvar_name
    }

    fn mut_cvar_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cvar_name
    }
}

impl ::protobuf::Message for CSVCMsg_GetCvarValue {
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
                    self.cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cvar_name)?;
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
        if let Some(v) = self.cookie {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.cvar_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cookie {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.cvar_name.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_GetCvarValue {
    fn new() -> CSVCMsg_GetCvarValue {
        CSVCMsg_GetCvarValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GetCvarValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cookie",
                    CSVCMsg_GetCvarValue::get_cookie_for_reflect,
                    CSVCMsg_GetCvarValue::mut_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cvar_name",
                    CSVCMsg_GetCvarValue::get_cvar_name_for_reflect,
                    CSVCMsg_GetCvarValue::mut_cvar_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GetCvarValue>(
                    "CSVCMsg_GetCvarValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GetCvarValue {
    fn clear(&mut self) {
        self.clear_cookie();
        self.clear_cvar_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GetCvarValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GetCvarValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Menu {
    // message fields
    dialog_type: ::std::option::Option<i32>,
    menu_key_values: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Menu {}

impl CSVCMsg_Menu {
    pub fn new() -> CSVCMsg_Menu {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Menu {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Menu> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Menu,
        };
        unsafe {
            instance.get(CSVCMsg_Menu::new)
        }
    }

    // optional int32 dialog_type = 1;

    pub fn clear_dialog_type(&mut self) {
        self.dialog_type = ::std::option::Option::None;
    }

    pub fn has_dialog_type(&self) -> bool {
        self.dialog_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dialog_type(&mut self, v: i32) {
        self.dialog_type = ::std::option::Option::Some(v);
    }

    pub fn get_dialog_type(&self) -> i32 {
        self.dialog_type.unwrap_or(0)
    }

    fn get_dialog_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dialog_type
    }

    fn mut_dialog_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dialog_type
    }

    // optional bytes menu_key_values = 2;

    pub fn clear_menu_key_values(&mut self) {
        self.menu_key_values.clear();
    }

    pub fn has_menu_key_values(&self) -> bool {
        self.menu_key_values.is_some()
    }

    // Param is passed by value, moved
    pub fn set_menu_key_values(&mut self, v: ::std::vec::Vec<u8>) {
        self.menu_key_values = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_menu_key_values(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.menu_key_values.is_none() {
            self.menu_key_values.set_default();
        };
        self.menu_key_values.as_mut().unwrap()
    }

    // Take field
    pub fn take_menu_key_values(&mut self) -> ::std::vec::Vec<u8> {
        self.menu_key_values.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_menu_key_values(&self) -> &[u8] {
        match self.menu_key_values.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_menu_key_values_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.menu_key_values
    }

    fn mut_menu_key_values_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.menu_key_values
    }
}

impl ::protobuf::Message for CSVCMsg_Menu {
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
                    self.dialog_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.menu_key_values)?;
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
        if let Some(v) = self.dialog_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.menu_key_values.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dialog_type {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.menu_key_values.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_Menu {
    fn new() -> CSVCMsg_Menu {
        CSVCMsg_Menu::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Menu>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dialog_type",
                    CSVCMsg_Menu::get_dialog_type_for_reflect,
                    CSVCMsg_Menu::mut_dialog_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "menu_key_values",
                    CSVCMsg_Menu::get_menu_key_values_for_reflect,
                    CSVCMsg_Menu::mut_menu_key_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Menu>(
                    "CSVCMsg_Menu",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Menu {
    fn clear(&mut self) {
        self.clear_dialog_type();
        self.clear_menu_key_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Menu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Menu {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SendTable {
    // message fields
    is_end: ::std::option::Option<bool>,
    net_table_name: ::protobuf::SingularField<::std::string::String>,
    needs_decoder: ::std::option::Option<bool>,
    props: ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SendTable {}

impl CSVCMsg_SendTable {
    pub fn new() -> CSVCMsg_SendTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SendTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SendTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SendTable,
        };
        unsafe {
            instance.get(CSVCMsg_SendTable::new)
        }
    }

    // optional bool is_end = 1;

    pub fn clear_is_end(&mut self) {
        self.is_end = ::std::option::Option::None;
    }

    pub fn has_is_end(&self) -> bool {
        self.is_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_end(&mut self, v: bool) {
        self.is_end = ::std::option::Option::Some(v);
    }

    pub fn get_is_end(&self) -> bool {
        self.is_end.unwrap_or(false)
    }

    fn get_is_end_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_end
    }

    fn mut_is_end_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_end
    }

    // optional string net_table_name = 2;

    pub fn clear_net_table_name(&mut self) {
        self.net_table_name.clear();
    }

    pub fn has_net_table_name(&self) -> bool {
        self.net_table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_table_name(&mut self, v: ::std::string::String) {
        self.net_table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_net_table_name(&mut self) -> &mut ::std::string::String {
        if self.net_table_name.is_none() {
            self.net_table_name.set_default();
        };
        self.net_table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_net_table_name(&mut self) -> ::std::string::String {
        self.net_table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_net_table_name(&self) -> &str {
        match self.net_table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_net_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.net_table_name
    }

    fn mut_net_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.net_table_name
    }

    // optional bool needs_decoder = 3;

    pub fn clear_needs_decoder(&mut self) {
        self.needs_decoder = ::std::option::Option::None;
    }

    pub fn has_needs_decoder(&self) -> bool {
        self.needs_decoder.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needs_decoder(&mut self, v: bool) {
        self.needs_decoder = ::std::option::Option::Some(v);
    }

    pub fn get_needs_decoder(&self) -> bool {
        self.needs_decoder.unwrap_or(false)
    }

    fn get_needs_decoder_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.needs_decoder
    }

    fn mut_needs_decoder_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.needs_decoder
    }

    // repeated .dota.CSVCMsg_SendTable.sendprop_t props = 4;

    pub fn clear_props(&mut self) {
        self.props.clear();
    }

    // Param is passed by value, moved
    pub fn set_props(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t>) {
        self.props = v;
    }

    // Mutable pointer to the field.
    pub fn mut_props(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        &mut self.props
    }

    // Take field
    pub fn take_props(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        ::std::mem::replace(&mut self.props, ::protobuf::RepeatedField::new())
    }

    pub fn get_props(&self) -> &[CSVCMsg_SendTable_sendprop_t] {
        &self.props
    }

    fn get_props_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        &self.props
    }

    fn mut_props_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        &mut self.props
    }
}

impl ::protobuf::Message for CSVCMsg_SendTable {
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
                    self.is_end = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.net_table_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.needs_decoder = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.props)?;
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
        if let Some(v) = self.is_end {
            my_size += 2;
        };
        if let Some(v) = self.net_table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.needs_decoder {
            my_size += 2;
        };
        for value in &self.props {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_end {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.net_table_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.needs_decoder {
            os.write_bool(3, v)?;
        };
        for v in &self.props {
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

impl ::protobuf::MessageStatic for CSVCMsg_SendTable {
    fn new() -> CSVCMsg_SendTable {
        CSVCMsg_SendTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SendTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_end",
                    CSVCMsg_SendTable::get_is_end_for_reflect,
                    CSVCMsg_SendTable::mut_is_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "net_table_name",
                    CSVCMsg_SendTable::get_net_table_name_for_reflect,
                    CSVCMsg_SendTable::mut_net_table_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "needs_decoder",
                    CSVCMsg_SendTable::get_needs_decoder_for_reflect,
                    CSVCMsg_SendTable::mut_needs_decoder_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_SendTable_sendprop_t>>(
                    "props",
                    CSVCMsg_SendTable::get_props_for_reflect,
                    CSVCMsg_SendTable::mut_props_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SendTable>(
                    "CSVCMsg_SendTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SendTable {
    fn clear(&mut self) {
        self.clear_is_end();
        self.clear_net_table_name();
        self.clear_needs_decoder();
        self.clear_props();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SendTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SendTable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SendTable_sendprop_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    var_name: ::protobuf::SingularField<::std::string::String>,
    flags: ::std::option::Option<i32>,
    priority: ::std::option::Option<i32>,
    dt_name: ::protobuf::SingularField<::std::string::String>,
    num_elements: ::std::option::Option<i32>,
    low_value: ::std::option::Option<f32>,
    high_value: ::std::option::Option<f32>,
    num_bits: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SendTable_sendprop_t {}

impl CSVCMsg_SendTable_sendprop_t {
    pub fn new() -> CSVCMsg_SendTable_sendprop_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SendTable_sendprop_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SendTable_sendprop_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SendTable_sendprop_t,
        };
        unsafe {
            instance.get(CSVCMsg_SendTable_sendprop_t::new)
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_type
    }

    // optional string var_name = 2;

    pub fn clear_var_name(&mut self) {
        self.var_name.clear();
    }

    pub fn has_var_name(&self) -> bool {
        self.var_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_var_name(&mut self, v: ::std::string::String) {
        self.var_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_var_name(&mut self) -> &mut ::std::string::String {
        if self.var_name.is_none() {
            self.var_name.set_default();
        };
        self.var_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_var_name(&mut self) -> ::std::string::String {
        self.var_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_var_name(&self) -> &str {
        match self.var_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_var_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.var_name
    }

    fn mut_var_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.var_name
    }

    // optional int32 flags = 3;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: i32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> i32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.flags
    }

    // optional int32 priority = 4;

    pub fn clear_priority(&mut self) {
        self.priority = ::std::option::Option::None;
    }

    pub fn has_priority(&self) -> bool {
        self.priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: i32) {
        self.priority = ::std::option::Option::Some(v);
    }

    pub fn get_priority(&self) -> i32 {
        self.priority.unwrap_or(0)
    }

    fn get_priority_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.priority
    }

    fn mut_priority_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.priority
    }

    // optional string dt_name = 5;

    pub fn clear_dt_name(&mut self) {
        self.dt_name.clear();
    }

    pub fn has_dt_name(&self) -> bool {
        self.dt_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dt_name(&mut self, v: ::std::string::String) {
        self.dt_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dt_name(&mut self) -> &mut ::std::string::String {
        if self.dt_name.is_none() {
            self.dt_name.set_default();
        };
        self.dt_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_dt_name(&mut self) -> ::std::string::String {
        self.dt_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_dt_name(&self) -> &str {
        match self.dt_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_dt_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.dt_name
    }

    fn mut_dt_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.dt_name
    }

    // optional int32 num_elements = 6;

    pub fn clear_num_elements(&mut self) {
        self.num_elements = ::std::option::Option::None;
    }

    pub fn has_num_elements(&self) -> bool {
        self.num_elements.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_elements(&mut self, v: i32) {
        self.num_elements = ::std::option::Option::Some(v);
    }

    pub fn get_num_elements(&self) -> i32 {
        self.num_elements.unwrap_or(0)
    }

    fn get_num_elements_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_elements
    }

    fn mut_num_elements_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_elements
    }

    // optional float low_value = 7;

    pub fn clear_low_value(&mut self) {
        self.low_value = ::std::option::Option::None;
    }

    pub fn has_low_value(&self) -> bool {
        self.low_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_value(&mut self, v: f32) {
        self.low_value = ::std::option::Option::Some(v);
    }

    pub fn get_low_value(&self) -> f32 {
        self.low_value.unwrap_or(0.)
    }

    fn get_low_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.low_value
    }

    fn mut_low_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.low_value
    }

    // optional float high_value = 8;

    pub fn clear_high_value(&mut self) {
        self.high_value = ::std::option::Option::None;
    }

    pub fn has_high_value(&self) -> bool {
        self.high_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_high_value(&mut self, v: f32) {
        self.high_value = ::std::option::Option::Some(v);
    }

    pub fn get_high_value(&self) -> f32 {
        self.high_value.unwrap_or(0.)
    }

    fn get_high_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.high_value
    }

    fn mut_high_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.high_value
    }

    // optional int32 num_bits = 9;

    pub fn clear_num_bits(&mut self) {
        self.num_bits = ::std::option::Option::None;
    }

    pub fn has_num_bits(&self) -> bool {
        self.num_bits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_bits(&mut self, v: i32) {
        self.num_bits = ::std::option::Option::Some(v);
    }

    pub fn get_num_bits(&self) -> i32 {
        self.num_bits.unwrap_or(0)
    }

    fn get_num_bits_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_bits
    }

    fn mut_num_bits_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_bits
    }
}

impl ::protobuf::Message for CSVCMsg_SendTable_sendprop_t {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.var_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.priority = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.dt_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_elements = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.low_value = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.high_value = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_bits = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.var_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.priority {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dt_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.num_elements {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.low_value {
            my_size += 5;
        };
        if let Some(v) = self.high_value {
            my_size += 5;
        };
        if let Some(v) = self.num_bits {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.var_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.flags {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.priority {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.dt_name.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.num_elements {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.low_value {
            os.write_float(7, v)?;
        };
        if let Some(v) = self.high_value {
            os.write_float(8, v)?;
        };
        if let Some(v) = self.num_bits {
            os.write_int32(9, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_SendTable_sendprop_t {
    fn new() -> CSVCMsg_SendTable_sendprop_t {
        CSVCMsg_SendTable_sendprop_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SendTable_sendprop_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CSVCMsg_SendTable_sendprop_t::get_field_type_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "var_name",
                    CSVCMsg_SendTable_sendprop_t::get_var_name_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_var_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CSVCMsg_SendTable_sendprop_t::get_flags_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "priority",
                    CSVCMsg_SendTable_sendprop_t::get_priority_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_priority_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dt_name",
                    CSVCMsg_SendTable_sendprop_t::get_dt_name_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_dt_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_elements",
                    CSVCMsg_SendTable_sendprop_t::get_num_elements_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_num_elements_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "low_value",
                    CSVCMsg_SendTable_sendprop_t::get_low_value_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_low_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "high_value",
                    CSVCMsg_SendTable_sendprop_t::get_high_value_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_high_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_bits",
                    CSVCMsg_SendTable_sendprop_t::get_num_bits_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_num_bits_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SendTable_sendprop_t>(
                    "CSVCMsg_SendTable_sendprop_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SendTable_sendprop_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_var_name();
        self.clear_flags();
        self.clear_priority();
        self.clear_dt_name();
        self.clear_num_elements();
        self.clear_low_value();
        self.clear_high_value();
        self.clear_num_bits();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SendTable_sendprop_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SendTable_sendprop_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEventList {
    // message fields
    descriptors: ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList {}

impl CSVCMsg_GameEventList {
    pub fn new() -> CSVCMsg_GameEventList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList,
        };
        unsafe {
            instance.get(CSVCMsg_GameEventList::new)
        }
    }

    // repeated .dota.CSVCMsg_GameEventList.descriptor_t descriptors = 1;

    pub fn clear_descriptors(&mut self) {
        self.descriptors.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptors(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t>) {
        self.descriptors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_descriptors(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        &mut self.descriptors
    }

    // Take field
    pub fn take_descriptors(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        ::std::mem::replace(&mut self.descriptors, ::protobuf::RepeatedField::new())
    }

    pub fn get_descriptors(&self) -> &[CSVCMsg_GameEventList_descriptor_t] {
        &self.descriptors
    }

    fn get_descriptors_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        &self.descriptors
    }

    fn mut_descriptors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        &mut self.descriptors
    }
}

impl ::protobuf::Message for CSVCMsg_GameEventList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.descriptors)?;
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
        for value in &self.descriptors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.descriptors {
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList {
    fn new() -> CSVCMsg_GameEventList {
        CSVCMsg_GameEventList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_GameEventList_descriptor_t>>(
                    "descriptors",
                    CSVCMsg_GameEventList::get_descriptors_for_reflect,
                    CSVCMsg_GameEventList::mut_descriptors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList>(
                    "CSVCMsg_GameEventList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList {
    fn clear(&mut self) {
        self.clear_descriptors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEventList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEventList_key_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList_key_t {}

impl CSVCMsg_GameEventList_key_t {
    pub fn new() -> CSVCMsg_GameEventList_key_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList_key_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList_key_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList_key_t,
        };
        unsafe {
            instance.get(CSVCMsg_GameEventList_key_t::new)
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_type
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for CSVCMsg_GameEventList_key_t {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList_key_t {
    fn new() -> CSVCMsg_GameEventList_key_t {
        CSVCMsg_GameEventList_key_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList_key_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CSVCMsg_GameEventList_key_t::get_field_type_for_reflect,
                    CSVCMsg_GameEventList_key_t::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSVCMsg_GameEventList_key_t::get_name_for_reflect,
                    CSVCMsg_GameEventList_key_t::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList_key_t>(
                    "CSVCMsg_GameEventList_key_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList_key_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList_key_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEventList_key_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEventList_descriptor_t {
    // message fields
    eventid: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    keys: ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList_descriptor_t {}

impl CSVCMsg_GameEventList_descriptor_t {
    pub fn new() -> CSVCMsg_GameEventList_descriptor_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList_descriptor_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList_descriptor_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList_descriptor_t,
        };
        unsafe {
            instance.get(CSVCMsg_GameEventList_descriptor_t::new)
        }
    }

    // optional int32 eventid = 1;

    pub fn clear_eventid(&mut self) {
        self.eventid = ::std::option::Option::None;
    }

    pub fn has_eventid(&self) -> bool {
        self.eventid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eventid(&mut self, v: i32) {
        self.eventid = ::std::option::Option::Some(v);
    }

    pub fn get_eventid(&self) -> i32 {
        self.eventid.unwrap_or(0)
    }

    fn get_eventid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eventid
    }

    fn mut_eventid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eventid
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated .dota.CSVCMsg_GameEventList.key_t keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CSVCMsg_GameEventList_key_t] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CSVCMsg_GameEventList_descriptor_t {
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
                    self.eventid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys)?;
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
        if let Some(v) = self.eventid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eventid {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        for v in &self.keys {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList_descriptor_t {
    fn new() -> CSVCMsg_GameEventList_descriptor_t {
        CSVCMsg_GameEventList_descriptor_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList_descriptor_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eventid",
                    CSVCMsg_GameEventList_descriptor_t::get_eventid_for_reflect,
                    CSVCMsg_GameEventList_descriptor_t::mut_eventid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSVCMsg_GameEventList_descriptor_t::get_name_for_reflect,
                    CSVCMsg_GameEventList_descriptor_t::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_GameEventList_key_t>>(
                    "keys",
                    CSVCMsg_GameEventList_descriptor_t::get_keys_for_reflect,
                    CSVCMsg_GameEventList_descriptor_t::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList_descriptor_t>(
                    "CSVCMsg_GameEventList_descriptor_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList_descriptor_t {
    fn clear(&mut self) {
        self.clear_eventid();
        self.clear_name();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList_descriptor_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEventList_descriptor_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_PacketEntities {
    // message fields
    max_entries: ::std::option::Option<i32>,
    updated_entries: ::std::option::Option<i32>,
    is_delta: ::std::option::Option<bool>,
    update_baseline: ::std::option::Option<bool>,
    baseline: ::std::option::Option<i32>,
    delta_from: ::std::option::Option<i32>,
    entity_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    pending_full_frame: ::std::option::Option<bool>,
    active_spawngroup_handle: ::std::option::Option<u32>,
    max_spawngroup_creationsequence: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_PacketEntities {}

impl CSVCMsg_PacketEntities {
    pub fn new() -> CSVCMsg_PacketEntities {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_PacketEntities {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_PacketEntities> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_PacketEntities,
        };
        unsafe {
            instance.get(CSVCMsg_PacketEntities::new)
        }
    }

    // optional int32 max_entries = 1;

    pub fn clear_max_entries(&mut self) {
        self.max_entries = ::std::option::Option::None;
    }

    pub fn has_max_entries(&self) -> bool {
        self.max_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_entries(&mut self, v: i32) {
        self.max_entries = ::std::option::Option::Some(v);
    }

    pub fn get_max_entries(&self) -> i32 {
        self.max_entries.unwrap_or(0)
    }

    fn get_max_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_entries
    }

    fn mut_max_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_entries
    }

    // optional int32 updated_entries = 2;

    pub fn clear_updated_entries(&mut self) {
        self.updated_entries = ::std::option::Option::None;
    }

    pub fn has_updated_entries(&self) -> bool {
        self.updated_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updated_entries(&mut self, v: i32) {
        self.updated_entries = ::std::option::Option::Some(v);
    }

    pub fn get_updated_entries(&self) -> i32 {
        self.updated_entries.unwrap_or(0)
    }

    fn get_updated_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.updated_entries
    }

    fn mut_updated_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.updated_entries
    }

    // optional bool is_delta = 3;

    pub fn clear_is_delta(&mut self) {
        self.is_delta = ::std::option::Option::None;
    }

    pub fn has_is_delta(&self) -> bool {
        self.is_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_delta(&mut self, v: bool) {
        self.is_delta = ::std::option::Option::Some(v);
    }

    pub fn get_is_delta(&self) -> bool {
        self.is_delta.unwrap_or(false)
    }

    fn get_is_delta_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_delta
    }

    fn mut_is_delta_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_delta
    }

    // optional bool update_baseline = 4;

    pub fn clear_update_baseline(&mut self) {
        self.update_baseline = ::std::option::Option::None;
    }

    pub fn has_update_baseline(&self) -> bool {
        self.update_baseline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_baseline(&mut self, v: bool) {
        self.update_baseline = ::std::option::Option::Some(v);
    }

    pub fn get_update_baseline(&self) -> bool {
        self.update_baseline.unwrap_or(false)
    }

    fn get_update_baseline_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_baseline
    }

    fn mut_update_baseline_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_baseline
    }

    // optional int32 baseline = 5;

    pub fn clear_baseline(&mut self) {
        self.baseline = ::std::option::Option::None;
    }

    pub fn has_baseline(&self) -> bool {
        self.baseline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline(&mut self, v: i32) {
        self.baseline = ::std::option::Option::Some(v);
    }

    pub fn get_baseline(&self) -> i32 {
        self.baseline.unwrap_or(0)
    }

    fn get_baseline_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.baseline
    }

    fn mut_baseline_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.baseline
    }

    // optional int32 delta_from = 6;

    pub fn clear_delta_from(&mut self) {
        self.delta_from = ::std::option::Option::None;
    }

    pub fn has_delta_from(&self) -> bool {
        self.delta_from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delta_from(&mut self, v: i32) {
        self.delta_from = ::std::option::Option::Some(v);
    }

    pub fn get_delta_from(&self) -> i32 {
        self.delta_from.unwrap_or(0)
    }

    fn get_delta_from_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.delta_from
    }

    fn mut_delta_from_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.delta_from
    }

    // optional bytes entity_data = 7;

    pub fn clear_entity_data(&mut self) {
        self.entity_data.clear();
    }

    pub fn has_entity_data(&self) -> bool {
        self.entity_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.entity_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.entity_data.is_none() {
            self.entity_data.set_default();
        };
        self.entity_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_data(&mut self) -> ::std::vec::Vec<u8> {
        self.entity_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_entity_data(&self) -> &[u8] {
        match self.entity_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_entity_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.entity_data
    }

    fn mut_entity_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.entity_data
    }

    // optional bool pending_full_frame = 8;

    pub fn clear_pending_full_frame(&mut self) {
        self.pending_full_frame = ::std::option::Option::None;
    }

    pub fn has_pending_full_frame(&self) -> bool {
        self.pending_full_frame.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pending_full_frame(&mut self, v: bool) {
        self.pending_full_frame = ::std::option::Option::Some(v);
    }

    pub fn get_pending_full_frame(&self) -> bool {
        self.pending_full_frame.unwrap_or(false)
    }

    fn get_pending_full_frame_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.pending_full_frame
    }

    fn mut_pending_full_frame_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.pending_full_frame
    }

    // optional uint32 active_spawngroup_handle = 9;

    pub fn clear_active_spawngroup_handle(&mut self) {
        self.active_spawngroup_handle = ::std::option::Option::None;
    }

    pub fn has_active_spawngroup_handle(&self) -> bool {
        self.active_spawngroup_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_spawngroup_handle(&mut self, v: u32) {
        self.active_spawngroup_handle = ::std::option::Option::Some(v);
    }

    pub fn get_active_spawngroup_handle(&self) -> u32 {
        self.active_spawngroup_handle.unwrap_or(0)
    }

    fn get_active_spawngroup_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.active_spawngroup_handle
    }

    fn mut_active_spawngroup_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.active_spawngroup_handle
    }

    // optional uint32 max_spawngroup_creationsequence = 10;

    pub fn clear_max_spawngroup_creationsequence(&mut self) {
        self.max_spawngroup_creationsequence = ::std::option::Option::None;
    }

    pub fn has_max_spawngroup_creationsequence(&self) -> bool {
        self.max_spawngroup_creationsequence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_spawngroup_creationsequence(&mut self, v: u32) {
        self.max_spawngroup_creationsequence = ::std::option::Option::Some(v);
    }

    pub fn get_max_spawngroup_creationsequence(&self) -> u32 {
        self.max_spawngroup_creationsequence.unwrap_or(0)
    }

    fn get_max_spawngroup_creationsequence_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_spawngroup_creationsequence
    }

    fn mut_max_spawngroup_creationsequence_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_spawngroup_creationsequence
    }
}

impl ::protobuf::Message for CSVCMsg_PacketEntities {
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
                    self.max_entries = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.updated_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_delta = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.update_baseline = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.baseline = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.delta_from = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.entity_data)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.pending_full_frame = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.active_spawngroup_handle = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_spawngroup_creationsequence = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.max_entries {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.updated_entries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.is_delta {
            my_size += 2;
        };
        if let Some(v) = self.update_baseline {
            my_size += 2;
        };
        if let Some(v) = self.baseline {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.delta_from {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entity_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        };
        if let Some(v) = self.pending_full_frame {
            my_size += 2;
        };
        if let Some(v) = self.active_spawngroup_handle {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.max_spawngroup_creationsequence {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.max_entries {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.updated_entries {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.is_delta {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.update_baseline {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.baseline {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.delta_from {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.entity_data.as_ref() {
            os.write_bytes(7, &v)?;
        };
        if let Some(v) = self.pending_full_frame {
            os.write_bool(8, v)?;
        };
        if let Some(v) = self.active_spawngroup_handle {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.max_spawngroup_creationsequence {
            os.write_uint32(10, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_PacketEntities {
    fn new() -> CSVCMsg_PacketEntities {
        CSVCMsg_PacketEntities::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_PacketEntities>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_entries",
                    CSVCMsg_PacketEntities::get_max_entries_for_reflect,
                    CSVCMsg_PacketEntities::mut_max_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "updated_entries",
                    CSVCMsg_PacketEntities::get_updated_entries_for_reflect,
                    CSVCMsg_PacketEntities::mut_updated_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_delta",
                    CSVCMsg_PacketEntities::get_is_delta_for_reflect,
                    CSVCMsg_PacketEntities::mut_is_delta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_baseline",
                    CSVCMsg_PacketEntities::get_update_baseline_for_reflect,
                    CSVCMsg_PacketEntities::mut_update_baseline_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "baseline",
                    CSVCMsg_PacketEntities::get_baseline_for_reflect,
                    CSVCMsg_PacketEntities::mut_baseline_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "delta_from",
                    CSVCMsg_PacketEntities::get_delta_from_for_reflect,
                    CSVCMsg_PacketEntities::mut_delta_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "entity_data",
                    CSVCMsg_PacketEntities::get_entity_data_for_reflect,
                    CSVCMsg_PacketEntities::mut_entity_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "pending_full_frame",
                    CSVCMsg_PacketEntities::get_pending_full_frame_for_reflect,
                    CSVCMsg_PacketEntities::mut_pending_full_frame_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "active_spawngroup_handle",
                    CSVCMsg_PacketEntities::get_active_spawngroup_handle_for_reflect,
                    CSVCMsg_PacketEntities::mut_active_spawngroup_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_spawngroup_creationsequence",
                    CSVCMsg_PacketEntities::get_max_spawngroup_creationsequence_for_reflect,
                    CSVCMsg_PacketEntities::mut_max_spawngroup_creationsequence_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_PacketEntities>(
                    "CSVCMsg_PacketEntities",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_PacketEntities {
    fn clear(&mut self) {
        self.clear_max_entries();
        self.clear_updated_entries();
        self.clear_is_delta();
        self.clear_update_baseline();
        self.clear_baseline();
        self.clear_delta_from();
        self.clear_entity_data();
        self.clear_pending_full_frame();
        self.clear_active_spawngroup_handle();
        self.clear_max_spawngroup_creationsequence();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_PacketEntities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_PacketEntities {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_TempEntities {
    // message fields
    reliable: ::std::option::Option<bool>,
    num_entries: ::std::option::Option<i32>,
    entity_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_TempEntities {}

impl CSVCMsg_TempEntities {
    pub fn new() -> CSVCMsg_TempEntities {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_TempEntities {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_TempEntities> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_TempEntities,
        };
        unsafe {
            instance.get(CSVCMsg_TempEntities::new)
        }
    }

    // optional bool reliable = 1;

    pub fn clear_reliable(&mut self) {
        self.reliable = ::std::option::Option::None;
    }

    pub fn has_reliable(&self) -> bool {
        self.reliable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable(&mut self, v: bool) {
        self.reliable = ::std::option::Option::Some(v);
    }

    pub fn get_reliable(&self) -> bool {
        self.reliable.unwrap_or(false)
    }

    fn get_reliable_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.reliable
    }

    fn mut_reliable_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.reliable
    }

    // optional int32 num_entries = 2;

    pub fn clear_num_entries(&mut self) {
        self.num_entries = ::std::option::Option::None;
    }

    pub fn has_num_entries(&self) -> bool {
        self.num_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_entries(&mut self, v: i32) {
        self.num_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_entries(&self) -> i32 {
        self.num_entries.unwrap_or(0)
    }

    fn get_num_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_entries
    }

    fn mut_num_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_entries
    }

    // optional bytes entity_data = 3;

    pub fn clear_entity_data(&mut self) {
        self.entity_data.clear();
    }

    pub fn has_entity_data(&self) -> bool {
        self.entity_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.entity_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.entity_data.is_none() {
            self.entity_data.set_default();
        };
        self.entity_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_data(&mut self) -> ::std::vec::Vec<u8> {
        self.entity_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_entity_data(&self) -> &[u8] {
        match self.entity_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_entity_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.entity_data
    }

    fn mut_entity_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.entity_data
    }
}

impl ::protobuf::Message for CSVCMsg_TempEntities {
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
                    self.reliable = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.entity_data)?;
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
        if let Some(v) = self.reliable {
            my_size += 2;
        };
        if let Some(v) = self.num_entries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entity_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reliable {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.num_entries {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.entity_data.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_TempEntities {
    fn new() -> CSVCMsg_TempEntities {
        CSVCMsg_TempEntities::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_TempEntities>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reliable",
                    CSVCMsg_TempEntities::get_reliable_for_reflect,
                    CSVCMsg_TempEntities::mut_reliable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_entries",
                    CSVCMsg_TempEntities::get_num_entries_for_reflect,
                    CSVCMsg_TempEntities::mut_num_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "entity_data",
                    CSVCMsg_TempEntities::get_entity_data_for_reflect,
                    CSVCMsg_TempEntities::mut_entity_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_TempEntities>(
                    "CSVCMsg_TempEntities",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_TempEntities {
    fn clear(&mut self) {
        self.clear_reliable();
        self.clear_num_entries();
        self.clear_entity_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_TempEntities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_TempEntities {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_CreateStringTable {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    num_entries: ::std::option::Option<i32>,
    user_data_fixed_size: ::std::option::Option<bool>,
    user_data_size: ::std::option::Option<i32>,
    user_data_size_bits: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    string_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    uncompressed_size: ::std::option::Option<i32>,
    data_compressed: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_CreateStringTable {}

impl CSVCMsg_CreateStringTable {
    pub fn new() -> CSVCMsg_CreateStringTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_CreateStringTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_CreateStringTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_CreateStringTable,
        };
        unsafe {
            instance.get(CSVCMsg_CreateStringTable::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional int32 num_entries = 2;

    pub fn clear_num_entries(&mut self) {
        self.num_entries = ::std::option::Option::None;
    }

    pub fn has_num_entries(&self) -> bool {
        self.num_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_entries(&mut self, v: i32) {
        self.num_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_entries(&self) -> i32 {
        self.num_entries.unwrap_or(0)
    }

    fn get_num_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_entries
    }

    fn mut_num_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_entries
    }

    // optional bool user_data_fixed_size = 3;

    pub fn clear_user_data_fixed_size(&mut self) {
        self.user_data_fixed_size = ::std::option::Option::None;
    }

    pub fn has_user_data_fixed_size(&self) -> bool {
        self.user_data_fixed_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_fixed_size(&mut self, v: bool) {
        self.user_data_fixed_size = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_fixed_size(&self) -> bool {
        self.user_data_fixed_size.unwrap_or(false)
    }

    fn get_user_data_fixed_size_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.user_data_fixed_size
    }

    fn mut_user_data_fixed_size_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.user_data_fixed_size
    }

    // optional int32 user_data_size = 4;

    pub fn clear_user_data_size(&mut self) {
        self.user_data_size = ::std::option::Option::None;
    }

    pub fn has_user_data_size(&self) -> bool {
        self.user_data_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_size(&mut self, v: i32) {
        self.user_data_size = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_size(&self) -> i32 {
        self.user_data_size.unwrap_or(0)
    }

    fn get_user_data_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.user_data_size
    }

    fn mut_user_data_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.user_data_size
    }

    // optional int32 user_data_size_bits = 5;

    pub fn clear_user_data_size_bits(&mut self) {
        self.user_data_size_bits = ::std::option::Option::None;
    }

    pub fn has_user_data_size_bits(&self) -> bool {
        self.user_data_size_bits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_size_bits(&mut self, v: i32) {
        self.user_data_size_bits = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_size_bits(&self) -> i32 {
        self.user_data_size_bits.unwrap_or(0)
    }

    fn get_user_data_size_bits_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.user_data_size_bits
    }

    fn mut_user_data_size_bits_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.user_data_size_bits
    }

    // optional int32 flags = 6;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: i32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> i32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.flags
    }

    // optional bytes string_data = 7;

    pub fn clear_string_data(&mut self) {
        self.string_data.clear();
    }

    pub fn has_string_data(&self) -> bool {
        self.string_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.string_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.string_data.is_none() {
            self.string_data.set_default();
        };
        self.string_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_data(&mut self) -> ::std::vec::Vec<u8> {
        self.string_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_string_data(&self) -> &[u8] {
        match self.string_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_string_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.string_data
    }

    fn mut_string_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.string_data
    }

    // optional int32 uncompressed_size = 8;

    pub fn clear_uncompressed_size(&mut self) {
        self.uncompressed_size = ::std::option::Option::None;
    }

    pub fn has_uncompressed_size(&self) -> bool {
        self.uncompressed_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uncompressed_size(&mut self, v: i32) {
        self.uncompressed_size = ::std::option::Option::Some(v);
    }

    pub fn get_uncompressed_size(&self) -> i32 {
        self.uncompressed_size.unwrap_or(0)
    }

    fn get_uncompressed_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.uncompressed_size
    }

    fn mut_uncompressed_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.uncompressed_size
    }

    // optional bool data_compressed = 9;

    pub fn clear_data_compressed(&mut self) {
        self.data_compressed = ::std::option::Option::None;
    }

    pub fn has_data_compressed(&self) -> bool {
        self.data_compressed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_compressed(&mut self, v: bool) {
        self.data_compressed = ::std::option::Option::Some(v);
    }

    pub fn get_data_compressed(&self) -> bool {
        self.data_compressed.unwrap_or(false)
    }

    fn get_data_compressed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.data_compressed
    }

    fn mut_data_compressed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.data_compressed
    }
}

impl ::protobuf::Message for CSVCMsg_CreateStringTable {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.user_data_fixed_size = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.user_data_size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.user_data_size_bits = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.string_data)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.uncompressed_size = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.data_compressed = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.num_entries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.user_data_fixed_size {
            my_size += 2;
        };
        if let Some(v) = self.user_data_size {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.user_data_size_bits {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.string_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        };
        if let Some(v) = self.uncompressed_size {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.data_compressed {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.num_entries {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.user_data_fixed_size {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.user_data_size {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.user_data_size_bits {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.flags {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.string_data.as_ref() {
            os.write_bytes(7, &v)?;
        };
        if let Some(v) = self.uncompressed_size {
            os.write_int32(8, v)?;
        };
        if let Some(v) = self.data_compressed {
            os.write_bool(9, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_CreateStringTable {
    fn new() -> CSVCMsg_CreateStringTable {
        CSVCMsg_CreateStringTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_CreateStringTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSVCMsg_CreateStringTable::get_name_for_reflect,
                    CSVCMsg_CreateStringTable::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_entries",
                    CSVCMsg_CreateStringTable::get_num_entries_for_reflect,
                    CSVCMsg_CreateStringTable::mut_num_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "user_data_fixed_size",
                    CSVCMsg_CreateStringTable::get_user_data_fixed_size_for_reflect,
                    CSVCMsg_CreateStringTable::mut_user_data_fixed_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "user_data_size",
                    CSVCMsg_CreateStringTable::get_user_data_size_for_reflect,
                    CSVCMsg_CreateStringTable::mut_user_data_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "user_data_size_bits",
                    CSVCMsg_CreateStringTable::get_user_data_size_bits_for_reflect,
                    CSVCMsg_CreateStringTable::mut_user_data_size_bits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CSVCMsg_CreateStringTable::get_flags_for_reflect,
                    CSVCMsg_CreateStringTable::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "string_data",
                    CSVCMsg_CreateStringTable::get_string_data_for_reflect,
                    CSVCMsg_CreateStringTable::mut_string_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "uncompressed_size",
                    CSVCMsg_CreateStringTable::get_uncompressed_size_for_reflect,
                    CSVCMsg_CreateStringTable::mut_uncompressed_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "data_compressed",
                    CSVCMsg_CreateStringTable::get_data_compressed_for_reflect,
                    CSVCMsg_CreateStringTable::mut_data_compressed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_CreateStringTable>(
                    "CSVCMsg_CreateStringTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_CreateStringTable {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_num_entries();
        self.clear_user_data_fixed_size();
        self.clear_user_data_size();
        self.clear_user_data_size_bits();
        self.clear_flags();
        self.clear_string_data();
        self.clear_uncompressed_size();
        self.clear_data_compressed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_CreateStringTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_CreateStringTable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_UpdateStringTable {
    // message fields
    table_id: ::std::option::Option<i32>,
    num_changed_entries: ::std::option::Option<i32>,
    string_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_UpdateStringTable {}

impl CSVCMsg_UpdateStringTable {
    pub fn new() -> CSVCMsg_UpdateStringTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_UpdateStringTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_UpdateStringTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_UpdateStringTable,
        };
        unsafe {
            instance.get(CSVCMsg_UpdateStringTable::new)
        }
    }

    // optional int32 table_id = 1;

    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None;
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i32) {
        self.table_id = ::std::option::Option::Some(v);
    }

    pub fn get_table_id(&self) -> i32 {
        self.table_id.unwrap_or(0)
    }

    fn get_table_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.table_id
    }

    // optional int32 num_changed_entries = 2;

    pub fn clear_num_changed_entries(&mut self) {
        self.num_changed_entries = ::std::option::Option::None;
    }

    pub fn has_num_changed_entries(&self) -> bool {
        self.num_changed_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_changed_entries(&mut self, v: i32) {
        self.num_changed_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_changed_entries(&self) -> i32 {
        self.num_changed_entries.unwrap_or(0)
    }

    fn get_num_changed_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_changed_entries
    }

    fn mut_num_changed_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_changed_entries
    }

    // optional bytes string_data = 3;

    pub fn clear_string_data(&mut self) {
        self.string_data.clear();
    }

    pub fn has_string_data(&self) -> bool {
        self.string_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.string_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.string_data.is_none() {
            self.string_data.set_default();
        };
        self.string_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_data(&mut self) -> ::std::vec::Vec<u8> {
        self.string_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_string_data(&self) -> &[u8] {
        match self.string_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_string_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.string_data
    }

    fn mut_string_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.string_data
    }
}

impl ::protobuf::Message for CSVCMsg_UpdateStringTable {
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
                    self.table_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_changed_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.string_data)?;
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
        if let Some(v) = self.table_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_changed_entries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.string_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_id {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.num_changed_entries {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.string_data.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_UpdateStringTable {
    fn new() -> CSVCMsg_UpdateStringTable {
        CSVCMsg_UpdateStringTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_UpdateStringTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "table_id",
                    CSVCMsg_UpdateStringTable::get_table_id_for_reflect,
                    CSVCMsg_UpdateStringTable::mut_table_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_changed_entries",
                    CSVCMsg_UpdateStringTable::get_num_changed_entries_for_reflect,
                    CSVCMsg_UpdateStringTable::mut_num_changed_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "string_data",
                    CSVCMsg_UpdateStringTable::get_string_data_for_reflect,
                    CSVCMsg_UpdateStringTable::mut_string_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_UpdateStringTable>(
                    "CSVCMsg_UpdateStringTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_UpdateStringTable {
    fn clear(&mut self) {
        self.clear_table_id();
        self.clear_num_changed_entries();
        self.clear_string_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_UpdateStringTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_UpdateStringTable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_VoiceData {
    // message fields
    audio: ::protobuf::SingularPtrField<CMsgVoiceAudio>,
    client: ::std::option::Option<i32>,
    proximity: ::std::option::Option<bool>,
    xuid: ::std::option::Option<u64>,
    audible_mask: ::std::option::Option<i32>,
    tick: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_VoiceData {}

impl CSVCMsg_VoiceData {
    pub fn new() -> CSVCMsg_VoiceData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_VoiceData {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_VoiceData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_VoiceData,
        };
        unsafe {
            instance.get(CSVCMsg_VoiceData::new)
        }
    }

    // optional .dota.CMsgVoiceAudio audio = 1;

    pub fn clear_audio(&mut self) {
        self.audio.clear();
    }

    pub fn has_audio(&self) -> bool {
        self.audio.is_some()
    }

    // Param is passed by value, moved
    pub fn set_audio(&mut self, v: CMsgVoiceAudio) {
        self.audio = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_audio(&mut self) -> &mut CMsgVoiceAudio {
        if self.audio.is_none() {
            self.audio.set_default();
        };
        self.audio.as_mut().unwrap()
    }

    // Take field
    pub fn take_audio(&mut self) -> CMsgVoiceAudio {
        self.audio.take().unwrap_or_else(|| CMsgVoiceAudio::new())
    }

    pub fn get_audio(&self) -> &CMsgVoiceAudio {
        self.audio.as_ref().unwrap_or_else(|| CMsgVoiceAudio::default_instance())
    }

    fn get_audio_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgVoiceAudio> {
        &self.audio
    }

    fn mut_audio_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgVoiceAudio> {
        &mut self.audio
    }

    // optional int32 client = 2;

    pub fn clear_client(&mut self) {
        self.client = ::std::option::Option::None;
    }

    pub fn has_client(&self) -> bool {
        self.client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client(&mut self, v: i32) {
        self.client = ::std::option::Option::Some(v);
    }

    pub fn get_client(&self) -> i32 {
        self.client.unwrap_or(0)
    }

    fn get_client_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.client
    }

    fn mut_client_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.client
    }

    // optional bool proximity = 3;

    pub fn clear_proximity(&mut self) {
        self.proximity = ::std::option::Option::None;
    }

    pub fn has_proximity(&self) -> bool {
        self.proximity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proximity(&mut self, v: bool) {
        self.proximity = ::std::option::Option::Some(v);
    }

    pub fn get_proximity(&self) -> bool {
        self.proximity.unwrap_or(false)
    }

    fn get_proximity_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.proximity
    }

    fn mut_proximity_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.proximity
    }

    // optional fixed64 xuid = 4;

    pub fn clear_xuid(&mut self) {
        self.xuid = ::std::option::Option::None;
    }

    pub fn has_xuid(&self) -> bool {
        self.xuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xuid(&mut self, v: u64) {
        self.xuid = ::std::option::Option::Some(v);
    }

    pub fn get_xuid(&self) -> u64 {
        self.xuid.unwrap_or(0)
    }

    fn get_xuid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.xuid
    }

    fn mut_xuid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.xuid
    }

    // optional int32 audible_mask = 5;

    pub fn clear_audible_mask(&mut self) {
        self.audible_mask = ::std::option::Option::None;
    }

    pub fn has_audible_mask(&self) -> bool {
        self.audible_mask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_audible_mask(&mut self, v: i32) {
        self.audible_mask = ::std::option::Option::Some(v);
    }

    pub fn get_audible_mask(&self) -> i32 {
        self.audible_mask.unwrap_or(0)
    }

    fn get_audible_mask_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.audible_mask
    }

    fn mut_audible_mask_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.audible_mask
    }

    // optional uint32 tick = 6;

    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: u32) {
        self.tick = ::std::option::Option::Some(v);
    }

    pub fn get_tick(&self) -> u32 {
        self.tick.unwrap_or(0)
    }

    fn get_tick_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tick
    }

    fn mut_tick_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tick
    }
}

impl ::protobuf::Message for CSVCMsg_VoiceData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.audio)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.client = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.proximity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.xuid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.audible_mask = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tick = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.audio.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.client {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.proximity {
            my_size += 2;
        };
        if let Some(v) = self.xuid {
            my_size += 9;
        };
        if let Some(v) = self.audible_mask {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tick {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.audio.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.client {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.proximity {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.xuid {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.audible_mask {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.tick {
            os.write_uint32(6, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_VoiceData {
    fn new() -> CSVCMsg_VoiceData {
        CSVCMsg_VoiceData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_VoiceData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgVoiceAudio>>(
                    "audio",
                    CSVCMsg_VoiceData::get_audio_for_reflect,
                    CSVCMsg_VoiceData::mut_audio_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "client",
                    CSVCMsg_VoiceData::get_client_for_reflect,
                    CSVCMsg_VoiceData::mut_client_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "proximity",
                    CSVCMsg_VoiceData::get_proximity_for_reflect,
                    CSVCMsg_VoiceData::mut_proximity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "xuid",
                    CSVCMsg_VoiceData::get_xuid_for_reflect,
                    CSVCMsg_VoiceData::mut_xuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "audible_mask",
                    CSVCMsg_VoiceData::get_audible_mask_for_reflect,
                    CSVCMsg_VoiceData::mut_audible_mask_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tick",
                    CSVCMsg_VoiceData::get_tick_for_reflect,
                    CSVCMsg_VoiceData::mut_tick_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_VoiceData>(
                    "CSVCMsg_VoiceData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_VoiceData {
    fn clear(&mut self) {
        self.clear_audio();
        self.clear_client();
        self.clear_proximity();
        self.clear_xuid();
        self.clear_audible_mask();
        self.clear_tick();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_VoiceData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_VoiceData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_PacketReliable {
    // message fields
    tick: ::std::option::Option<i32>,
    messagessize: ::std::option::Option<i32>,
    state: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_PacketReliable {}

impl CSVCMsg_PacketReliable {
    pub fn new() -> CSVCMsg_PacketReliable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_PacketReliable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_PacketReliable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_PacketReliable,
        };
        unsafe {
            instance.get(CSVCMsg_PacketReliable::new)
        }
    }

    // optional int32 tick = 1;

    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: i32) {
        self.tick = ::std::option::Option::Some(v);
    }

    pub fn get_tick(&self) -> i32 {
        self.tick.unwrap_or(0)
    }

    fn get_tick_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.tick
    }

    fn mut_tick_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.tick
    }

    // optional int32 messagessize = 2;

    pub fn clear_messagessize(&mut self) {
        self.messagessize = ::std::option::Option::None;
    }

    pub fn has_messagessize(&self) -> bool {
        self.messagessize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_messagessize(&mut self, v: i32) {
        self.messagessize = ::std::option::Option::Some(v);
    }

    pub fn get_messagessize(&self) -> i32 {
        self.messagessize.unwrap_or(0)
    }

    fn get_messagessize_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.messagessize
    }

    fn mut_messagessize_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.messagessize
    }

    // optional bool state = 3;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: bool) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> bool {
        self.state.unwrap_or(false)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.state
    }
}

impl ::protobuf::Message for CSVCMsg_PacketReliable {
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
                    self.tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.messagessize = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.state = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tick {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.messagessize {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.state {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.messagessize {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.state {
            os.write_bool(3, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_PacketReliable {
    fn new() -> CSVCMsg_PacketReliable {
        CSVCMsg_PacketReliable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_PacketReliable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tick",
                    CSVCMsg_PacketReliable::get_tick_for_reflect,
                    CSVCMsg_PacketReliable::mut_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "messagessize",
                    CSVCMsg_PacketReliable::get_messagessize_for_reflect,
                    CSVCMsg_PacketReliable::mut_messagessize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "state",
                    CSVCMsg_PacketReliable::get_state_for_reflect,
                    CSVCMsg_PacketReliable::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_PacketReliable>(
                    "CSVCMsg_PacketReliable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_PacketReliable {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_messagessize();
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_PacketReliable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_PacketReliable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_FullFrameSplit {
    // message fields
    tick: ::std::option::Option<i32>,
    section: ::std::option::Option<i32>,
    total: ::std::option::Option<i32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_FullFrameSplit {}

impl CSVCMsg_FullFrameSplit {
    pub fn new() -> CSVCMsg_FullFrameSplit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_FullFrameSplit {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_FullFrameSplit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_FullFrameSplit,
        };
        unsafe {
            instance.get(CSVCMsg_FullFrameSplit::new)
        }
    }

    // optional int32 tick = 1;

    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: i32) {
        self.tick = ::std::option::Option::Some(v);
    }

    pub fn get_tick(&self) -> i32 {
        self.tick.unwrap_or(0)
    }

    fn get_tick_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.tick
    }

    fn mut_tick_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.tick
    }

    // optional int32 section = 2;

    pub fn clear_section(&mut self) {
        self.section = ::std::option::Option::None;
    }

    pub fn has_section(&self) -> bool {
        self.section.is_some()
    }

    // Param is passed by value, moved
    pub fn set_section(&mut self, v: i32) {
        self.section = ::std::option::Option::Some(v);
    }

    pub fn get_section(&self) -> i32 {
        self.section.unwrap_or(0)
    }

    fn get_section_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.section
    }

    fn mut_section_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.section
    }

    // optional int32 total = 3;

    pub fn clear_total(&mut self) {
        self.total = ::std::option::Option::None;
    }

    pub fn has_total(&self) -> bool {
        self.total.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total(&mut self, v: i32) {
        self.total = ::std::option::Option::Some(v);
    }

    pub fn get_total(&self) -> i32 {
        self.total.unwrap_or(0)
    }

    fn get_total_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.total
    }

    fn mut_total_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.total
    }

    // optional bytes data = 4;

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

impl ::protobuf::Message for CSVCMsg_FullFrameSplit {
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
                    self.tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.section = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.total = ::std::option::Option::Some(tmp);
                },
                4 => {
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
        if let Some(v) = self.tick {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.section {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.total {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.section {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.total {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_FullFrameSplit {
    fn new() -> CSVCMsg_FullFrameSplit {
        CSVCMsg_FullFrameSplit::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_FullFrameSplit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tick",
                    CSVCMsg_FullFrameSplit::get_tick_for_reflect,
                    CSVCMsg_FullFrameSplit::mut_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "section",
                    CSVCMsg_FullFrameSplit::get_section_for_reflect,
                    CSVCMsg_FullFrameSplit::mut_section_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "total",
                    CSVCMsg_FullFrameSplit::get_total_for_reflect,
                    CSVCMsg_FullFrameSplit::mut_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CSVCMsg_FullFrameSplit::get_data_for_reflect,
                    CSVCMsg_FullFrameSplit::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_FullFrameSplit>(
                    "CSVCMsg_FullFrameSplit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_FullFrameSplit {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_section();
        self.clear_total();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_FullFrameSplit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_FullFrameSplit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_HLTVStatus {
    // message fields
    master: ::protobuf::SingularField<::std::string::String>,
    clients: ::std::option::Option<i32>,
    slots: ::std::option::Option<i32>,
    proxies: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_HLTVStatus {}

impl CSVCMsg_HLTVStatus {
    pub fn new() -> CSVCMsg_HLTVStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_HLTVStatus {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_HLTVStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_HLTVStatus,
        };
        unsafe {
            instance.get(CSVCMsg_HLTVStatus::new)
        }
    }

    // optional string master = 1;

    pub fn clear_master(&mut self) {
        self.master.clear();
    }

    pub fn has_master(&self) -> bool {
        self.master.is_some()
    }

    // Param is passed by value, moved
    pub fn set_master(&mut self, v: ::std::string::String) {
        self.master = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_master(&mut self) -> &mut ::std::string::String {
        if self.master.is_none() {
            self.master.set_default();
        };
        self.master.as_mut().unwrap()
    }

    // Take field
    pub fn take_master(&mut self) -> ::std::string::String {
        self.master.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_master(&self) -> &str {
        match self.master.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_master_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.master
    }

    fn mut_master_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.master
    }

    // optional int32 clients = 2;

    pub fn clear_clients(&mut self) {
        self.clients = ::std::option::Option::None;
    }

    pub fn has_clients(&self) -> bool {
        self.clients.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clients(&mut self, v: i32) {
        self.clients = ::std::option::Option::Some(v);
    }

    pub fn get_clients(&self) -> i32 {
        self.clients.unwrap_or(0)
    }

    fn get_clients_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.clients
    }

    fn mut_clients_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.clients
    }

    // optional int32 slots = 3;

    pub fn clear_slots(&mut self) {
        self.slots = ::std::option::Option::None;
    }

    pub fn has_slots(&self) -> bool {
        self.slots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slots(&mut self, v: i32) {
        self.slots = ::std::option::Option::Some(v);
    }

    pub fn get_slots(&self) -> i32 {
        self.slots.unwrap_or(0)
    }

    fn get_slots_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slots
    }

    fn mut_slots_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slots
    }

    // optional int32 proxies = 4;

    pub fn clear_proxies(&mut self) {
        self.proxies = ::std::option::Option::None;
    }

    pub fn has_proxies(&self) -> bool {
        self.proxies.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proxies(&mut self, v: i32) {
        self.proxies = ::std::option::Option::Some(v);
    }

    pub fn get_proxies(&self) -> i32 {
        self.proxies.unwrap_or(0)
    }

    fn get_proxies_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.proxies
    }

    fn mut_proxies_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.proxies
    }
}

impl ::protobuf::Message for CSVCMsg_HLTVStatus {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.master)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.clients = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.slots = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.proxies = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.master.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.clients {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.slots {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.proxies {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.master.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.clients {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.slots {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.proxies {
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

impl ::protobuf::MessageStatic for CSVCMsg_HLTVStatus {
    fn new() -> CSVCMsg_HLTVStatus {
        CSVCMsg_HLTVStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_HLTVStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "master",
                    CSVCMsg_HLTVStatus::get_master_for_reflect,
                    CSVCMsg_HLTVStatus::mut_master_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "clients",
                    CSVCMsg_HLTVStatus::get_clients_for_reflect,
                    CSVCMsg_HLTVStatus::mut_clients_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slots",
                    CSVCMsg_HLTVStatus::get_slots_for_reflect,
                    CSVCMsg_HLTVStatus::mut_slots_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "proxies",
                    CSVCMsg_HLTVStatus::get_proxies_for_reflect,
                    CSVCMsg_HLTVStatus::mut_proxies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_HLTVStatus>(
                    "CSVCMsg_HLTVStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_HLTVStatus {
    fn clear(&mut self) {
        self.clear_master();
        self.clear_clients();
        self.clear_slots();
        self.clear_proxies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_HLTVStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_HLTVStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_ServerSteamID {
    // message fields
    steam_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ServerSteamID {}

impl CSVCMsg_ServerSteamID {
    pub fn new() -> CSVCMsg_ServerSteamID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ServerSteamID {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ServerSteamID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ServerSteamID,
        };
        unsafe {
            instance.get(CSVCMsg_ServerSteamID::new)
        }
    }

    // optional uint64 steam_id = 1;

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
}

impl ::protobuf::Message for CSVCMsg_ServerSteamID {
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
                    self.steam_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.steam_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_ServerSteamID {
    fn new() -> CSVCMsg_ServerSteamID {
        CSVCMsg_ServerSteamID::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ServerSteamID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "steam_id",
                    CSVCMsg_ServerSteamID::get_steam_id_for_reflect,
                    CSVCMsg_ServerSteamID::mut_steam_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ServerSteamID>(
                    "CSVCMsg_ServerSteamID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ServerSteamID {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_ServerSteamID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_ServerSteamID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_CmdKeyValues {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_CmdKeyValues {}

impl CSVCMsg_CmdKeyValues {
    pub fn new() -> CSVCMsg_CmdKeyValues {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_CmdKeyValues {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_CmdKeyValues> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_CmdKeyValues,
        };
        unsafe {
            instance.get(CSVCMsg_CmdKeyValues::new)
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

impl ::protobuf::Message for CSVCMsg_CmdKeyValues {
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

impl ::protobuf::MessageStatic for CSVCMsg_CmdKeyValues {
    fn new() -> CSVCMsg_CmdKeyValues {
        CSVCMsg_CmdKeyValues::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_CmdKeyValues>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CSVCMsg_CmdKeyValues::get_data_for_reflect,
                    CSVCMsg_CmdKeyValues::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_CmdKeyValues>(
                    "CSVCMsg_CmdKeyValues",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_CmdKeyValues {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_CmdKeyValues {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_CmdKeyValues {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgIPCAddress {
    // message fields
    computer_guid: ::std::option::Option<u64>,
    process_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgIPCAddress {}

impl CMsgIPCAddress {
    pub fn new() -> CMsgIPCAddress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgIPCAddress {
        static mut instance: ::protobuf::lazy::Lazy<CMsgIPCAddress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgIPCAddress,
        };
        unsafe {
            instance.get(CMsgIPCAddress::new)
        }
    }

    // optional fixed64 computer_guid = 1;

    pub fn clear_computer_guid(&mut self) {
        self.computer_guid = ::std::option::Option::None;
    }

    pub fn has_computer_guid(&self) -> bool {
        self.computer_guid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_computer_guid(&mut self, v: u64) {
        self.computer_guid = ::std::option::Option::Some(v);
    }

    pub fn get_computer_guid(&self) -> u64 {
        self.computer_guid.unwrap_or(0)
    }

    fn get_computer_guid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.computer_guid
    }

    fn mut_computer_guid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.computer_guid
    }

    // optional uint32 process_id = 2;

    pub fn clear_process_id(&mut self) {
        self.process_id = ::std::option::Option::None;
    }

    pub fn has_process_id(&self) -> bool {
        self.process_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_process_id(&mut self, v: u32) {
        self.process_id = ::std::option::Option::Some(v);
    }

    pub fn get_process_id(&self) -> u32 {
        self.process_id.unwrap_or(0)
    }

    fn get_process_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.process_id
    }

    fn mut_process_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.process_id
    }
}

impl ::protobuf::Message for CMsgIPCAddress {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.computer_guid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.process_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.computer_guid {
            my_size += 9;
        };
        if let Some(v) = self.process_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.computer_guid {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.process_id {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for CMsgIPCAddress {
    fn new() -> CMsgIPCAddress {
        CMsgIPCAddress::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgIPCAddress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "computer_guid",
                    CMsgIPCAddress::get_computer_guid_for_reflect,
                    CMsgIPCAddress::mut_computer_guid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "process_id",
                    CMsgIPCAddress::get_process_id_for_reflect,
                    CMsgIPCAddress::mut_process_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgIPCAddress>(
                    "CMsgIPCAddress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgIPCAddress {
    fn clear(&mut self) {
        self.clear_computer_guid();
        self.clear_process_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgIPCAddress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgIPCAddress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgServerPeer {
    // message fields
    player_slot: ::std::option::Option<i32>,
    steamid: ::std::option::Option<u64>,
    ipc: ::protobuf::SingularPtrField<CMsgIPCAddress>,
    they_hear_you: ::std::option::Option<bool>,
    you_hear_them: ::std::option::Option<bool>,
    is_listenserver_host: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgServerPeer {}

impl CMsgServerPeer {
    pub fn new() -> CMsgServerPeer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgServerPeer {
        static mut instance: ::protobuf::lazy::Lazy<CMsgServerPeer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgServerPeer,
        };
        unsafe {
            instance.get(CMsgServerPeer::new)
        }
    }

    // optional int32 player_slot = 1;

    pub fn clear_player_slot(&mut self) {
        self.player_slot = ::std::option::Option::None;
    }

    pub fn has_player_slot(&self) -> bool {
        self.player_slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_slot(&mut self, v: i32) {
        self.player_slot = ::std::option::Option::Some(v);
    }

    pub fn get_player_slot(&self) -> i32 {
        self.player_slot.unwrap_or(0)
    }

    fn get_player_slot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_slot
    }

    fn mut_player_slot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_slot
    }

    // optional fixed64 steamid = 2;

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

    // optional .dota.CMsgIPCAddress ipc = 3;

    pub fn clear_ipc(&mut self) {
        self.ipc.clear();
    }

    pub fn has_ipc(&self) -> bool {
        self.ipc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipc(&mut self, v: CMsgIPCAddress) {
        self.ipc = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ipc(&mut self) -> &mut CMsgIPCAddress {
        if self.ipc.is_none() {
            self.ipc.set_default();
        };
        self.ipc.as_mut().unwrap()
    }

    // Take field
    pub fn take_ipc(&mut self) -> CMsgIPCAddress {
        self.ipc.take().unwrap_or_else(|| CMsgIPCAddress::new())
    }

    pub fn get_ipc(&self) -> &CMsgIPCAddress {
        self.ipc.as_ref().unwrap_or_else(|| CMsgIPCAddress::default_instance())
    }

    fn get_ipc_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgIPCAddress> {
        &self.ipc
    }

    fn mut_ipc_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgIPCAddress> {
        &mut self.ipc
    }

    // optional bool they_hear_you = 4;

    pub fn clear_they_hear_you(&mut self) {
        self.they_hear_you = ::std::option::Option::None;
    }

    pub fn has_they_hear_you(&self) -> bool {
        self.they_hear_you.is_some()
    }

    // Param is passed by value, moved
    pub fn set_they_hear_you(&mut self, v: bool) {
        self.they_hear_you = ::std::option::Option::Some(v);
    }

    pub fn get_they_hear_you(&self) -> bool {
        self.they_hear_you.unwrap_or(false)
    }

    fn get_they_hear_you_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.they_hear_you
    }

    fn mut_they_hear_you_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.they_hear_you
    }

    // optional bool you_hear_them = 5;

    pub fn clear_you_hear_them(&mut self) {
        self.you_hear_them = ::std::option::Option::None;
    }

    pub fn has_you_hear_them(&self) -> bool {
        self.you_hear_them.is_some()
    }

    // Param is passed by value, moved
    pub fn set_you_hear_them(&mut self, v: bool) {
        self.you_hear_them = ::std::option::Option::Some(v);
    }

    pub fn get_you_hear_them(&self) -> bool {
        self.you_hear_them.unwrap_or(false)
    }

    fn get_you_hear_them_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.you_hear_them
    }

    fn mut_you_hear_them_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.you_hear_them
    }

    // optional bool is_listenserver_host = 6;

    pub fn clear_is_listenserver_host(&mut self) {
        self.is_listenserver_host = ::std::option::Option::None;
    }

    pub fn has_is_listenserver_host(&self) -> bool {
        self.is_listenserver_host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_listenserver_host(&mut self, v: bool) {
        self.is_listenserver_host = ::std::option::Option::Some(v);
    }

    pub fn get_is_listenserver_host(&self) -> bool {
        self.is_listenserver_host.unwrap_or(false)
    }

    fn get_is_listenserver_host_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_listenserver_host
    }

    fn mut_is_listenserver_host_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_listenserver_host
    }
}

impl ::protobuf::Message for CMsgServerPeer {
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
                    self.player_slot = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ipc)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.they_hear_you = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.you_hear_them = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_listenserver_host = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.player_slot {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.steamid {
            my_size += 9;
        };
        if let Some(v) = self.ipc.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.they_hear_you {
            my_size += 2;
        };
        if let Some(v) = self.you_hear_them {
            my_size += 2;
        };
        if let Some(v) = self.is_listenserver_host {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_slot {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.steamid {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.ipc.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.they_hear_you {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.you_hear_them {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.is_listenserver_host {
            os.write_bool(6, v)?;
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

impl ::protobuf::MessageStatic for CMsgServerPeer {
    fn new() -> CMsgServerPeer {
        CMsgServerPeer::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgServerPeer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_slot",
                    CMsgServerPeer::get_player_slot_for_reflect,
                    CMsgServerPeer::mut_player_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgServerPeer::get_steamid_for_reflect,
                    CMsgServerPeer::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgIPCAddress>>(
                    "ipc",
                    CMsgServerPeer::get_ipc_for_reflect,
                    CMsgServerPeer::mut_ipc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "they_hear_you",
                    CMsgServerPeer::get_they_hear_you_for_reflect,
                    CMsgServerPeer::mut_they_hear_you_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "you_hear_them",
                    CMsgServerPeer::get_you_hear_them_for_reflect,
                    CMsgServerPeer::mut_you_hear_them_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_listenserver_host",
                    CMsgServerPeer::get_is_listenserver_host_for_reflect,
                    CMsgServerPeer::mut_is_listenserver_host_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgServerPeer>(
                    "CMsgServerPeer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgServerPeer {
    fn clear(&mut self) {
        self.clear_player_slot();
        self.clear_steamid();
        self.clear_ipc();
        self.clear_they_hear_you();
        self.clear_you_hear_them();
        self.clear_is_listenserver_host();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgServerPeer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgServerPeer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_PeerList {
    // message fields
    peer: ::protobuf::RepeatedField<CMsgServerPeer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_PeerList {}

impl CSVCMsg_PeerList {
    pub fn new() -> CSVCMsg_PeerList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_PeerList {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_PeerList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_PeerList,
        };
        unsafe {
            instance.get(CSVCMsg_PeerList::new)
        }
    }

    // repeated .dota.CMsgServerPeer peer = 1;

    pub fn clear_peer(&mut self) {
        self.peer.clear();
    }

    // Param is passed by value, moved
    pub fn set_peer(&mut self, v: ::protobuf::RepeatedField<CMsgServerPeer>) {
        self.peer = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peer(&mut self) -> &mut ::protobuf::RepeatedField<CMsgServerPeer> {
        &mut self.peer
    }

    // Take field
    pub fn take_peer(&mut self) -> ::protobuf::RepeatedField<CMsgServerPeer> {
        ::std::mem::replace(&mut self.peer, ::protobuf::RepeatedField::new())
    }

    pub fn get_peer(&self) -> &[CMsgServerPeer] {
        &self.peer
    }

    fn get_peer_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgServerPeer> {
        &self.peer
    }

    fn mut_peer_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgServerPeer> {
        &mut self.peer
    }
}

impl ::protobuf::Message for CSVCMsg_PeerList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.peer)?;
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
        for value in &self.peer {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.peer {
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

impl ::protobuf::MessageStatic for CSVCMsg_PeerList {
    fn new() -> CSVCMsg_PeerList {
        CSVCMsg_PeerList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_PeerList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgServerPeer>>(
                    "peer",
                    CSVCMsg_PeerList::get_peer_for_reflect,
                    CSVCMsg_PeerList::mut_peer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_PeerList>(
                    "CSVCMsg_PeerList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_PeerList {
    fn clear(&mut self) {
        self.clear_peer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_PeerList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_PeerList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_ClearAllStringTables {
    // message fields
    mapname: ::protobuf::SingularField<::std::string::String>,
    map_crc: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ClearAllStringTables {}

impl CSVCMsg_ClearAllStringTables {
    pub fn new() -> CSVCMsg_ClearAllStringTables {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ClearAllStringTables {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ClearAllStringTables> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ClearAllStringTables,
        };
        unsafe {
            instance.get(CSVCMsg_ClearAllStringTables::new)
        }
    }

    // optional string mapname = 1;

    pub fn clear_mapname(&mut self) {
        self.mapname.clear();
    }

    pub fn has_mapname(&self) -> bool {
        self.mapname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mapname(&mut self, v: ::std::string::String) {
        self.mapname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mapname(&mut self) -> &mut ::std::string::String {
        if self.mapname.is_none() {
            self.mapname.set_default();
        };
        self.mapname.as_mut().unwrap()
    }

    // Take field
    pub fn take_mapname(&mut self) -> ::std::string::String {
        self.mapname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_mapname(&self) -> &str {
        match self.mapname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_mapname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.mapname
    }

    fn mut_mapname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.mapname
    }

    // optional uint32 map_crc = 2;

    pub fn clear_map_crc(&mut self) {
        self.map_crc = ::std::option::Option::None;
    }

    pub fn has_map_crc(&self) -> bool {
        self.map_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_crc(&mut self, v: u32) {
        self.map_crc = ::std::option::Option::Some(v);
    }

    pub fn get_map_crc(&self) -> u32 {
        self.map_crc.unwrap_or(0)
    }

    fn get_map_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.map_crc
    }

    fn mut_map_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.map_crc
    }
}

impl ::protobuf::Message for CSVCMsg_ClearAllStringTables {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.mapname)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.map_crc = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.mapname.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.map_crc {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mapname.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.map_crc {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_ClearAllStringTables {
    fn new() -> CSVCMsg_ClearAllStringTables {
        CSVCMsg_ClearAllStringTables::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ClearAllStringTables>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mapname",
                    CSVCMsg_ClearAllStringTables::get_mapname_for_reflect,
                    CSVCMsg_ClearAllStringTables::mut_mapname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "map_crc",
                    CSVCMsg_ClearAllStringTables::get_map_crc_for_reflect,
                    CSVCMsg_ClearAllStringTables::mut_map_crc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ClearAllStringTables>(
                    "CSVCMsg_ClearAllStringTables",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ClearAllStringTables {
    fn clear(&mut self) {
        self.clear_mapname();
        self.clear_map_crc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_ClearAllStringTables {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_ClearAllStringTables {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProtoFlattenedSerializerField_t {
    // message fields
    var_type_sym: ::std::option::Option<i32>,
    var_name_sym: ::std::option::Option<i32>,
    bit_count: ::std::option::Option<i32>,
    low_value: ::std::option::Option<f32>,
    high_value: ::std::option::Option<f32>,
    encode_flags: ::std::option::Option<i32>,
    field_serializer_name_sym: ::std::option::Option<i32>,
    field_serializer_version: ::std::option::Option<i32>,
    send_node_sym: ::std::option::Option<i32>,
    var_encoder_sym: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProtoFlattenedSerializerField_t {}

impl ProtoFlattenedSerializerField_t {
    pub fn new() -> ProtoFlattenedSerializerField_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProtoFlattenedSerializerField_t {
        static mut instance: ::protobuf::lazy::Lazy<ProtoFlattenedSerializerField_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProtoFlattenedSerializerField_t,
        };
        unsafe {
            instance.get(ProtoFlattenedSerializerField_t::new)
        }
    }

    // optional int32 var_type_sym = 1;

    pub fn clear_var_type_sym(&mut self) {
        self.var_type_sym = ::std::option::Option::None;
    }

    pub fn has_var_type_sym(&self) -> bool {
        self.var_type_sym.is_some()
    }

    // Param is passed by value, moved
    pub fn set_var_type_sym(&mut self, v: i32) {
        self.var_type_sym = ::std::option::Option::Some(v);
    }

    pub fn get_var_type_sym(&self) -> i32 {
        self.var_type_sym.unwrap_or(0)
    }

    fn get_var_type_sym_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.var_type_sym
    }

    fn mut_var_type_sym_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.var_type_sym
    }

    // optional int32 var_name_sym = 2;

    pub fn clear_var_name_sym(&mut self) {
        self.var_name_sym = ::std::option::Option::None;
    }

    pub fn has_var_name_sym(&self) -> bool {
        self.var_name_sym.is_some()
    }

    // Param is passed by value, moved
    pub fn set_var_name_sym(&mut self, v: i32) {
        self.var_name_sym = ::std::option::Option::Some(v);
    }

    pub fn get_var_name_sym(&self) -> i32 {
        self.var_name_sym.unwrap_or(0)
    }

    fn get_var_name_sym_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.var_name_sym
    }

    fn mut_var_name_sym_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.var_name_sym
    }

    // optional int32 bit_count = 3;

    pub fn clear_bit_count(&mut self) {
        self.bit_count = ::std::option::Option::None;
    }

    pub fn has_bit_count(&self) -> bool {
        self.bit_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bit_count(&mut self, v: i32) {
        self.bit_count = ::std::option::Option::Some(v);
    }

    pub fn get_bit_count(&self) -> i32 {
        self.bit_count.unwrap_or(0)
    }

    fn get_bit_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.bit_count
    }

    fn mut_bit_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.bit_count
    }

    // optional float low_value = 4;

    pub fn clear_low_value(&mut self) {
        self.low_value = ::std::option::Option::None;
    }

    pub fn has_low_value(&self) -> bool {
        self.low_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_value(&mut self, v: f32) {
        self.low_value = ::std::option::Option::Some(v);
    }

    pub fn get_low_value(&self) -> f32 {
        self.low_value.unwrap_or(0.)
    }

    fn get_low_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.low_value
    }

    fn mut_low_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.low_value
    }

    // optional float high_value = 5;

    pub fn clear_high_value(&mut self) {
        self.high_value = ::std::option::Option::None;
    }

    pub fn has_high_value(&self) -> bool {
        self.high_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_high_value(&mut self, v: f32) {
        self.high_value = ::std::option::Option::Some(v);
    }

    pub fn get_high_value(&self) -> f32 {
        self.high_value.unwrap_or(0.)
    }

    fn get_high_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.high_value
    }

    fn mut_high_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.high_value
    }

    // optional int32 encode_flags = 6;

    pub fn clear_encode_flags(&mut self) {
        self.encode_flags = ::std::option::Option::None;
    }

    pub fn has_encode_flags(&self) -> bool {
        self.encode_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encode_flags(&mut self, v: i32) {
        self.encode_flags = ::std::option::Option::Some(v);
    }

    pub fn get_encode_flags(&self) -> i32 {
        self.encode_flags.unwrap_or(0)
    }

    fn get_encode_flags_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.encode_flags
    }

    fn mut_encode_flags_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.encode_flags
    }

    // optional int32 field_serializer_name_sym = 7;

    pub fn clear_field_serializer_name_sym(&mut self) {
        self.field_serializer_name_sym = ::std::option::Option::None;
    }

    pub fn has_field_serializer_name_sym(&self) -> bool {
        self.field_serializer_name_sym.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_serializer_name_sym(&mut self, v: i32) {
        self.field_serializer_name_sym = ::std::option::Option::Some(v);
    }

    pub fn get_field_serializer_name_sym(&self) -> i32 {
        self.field_serializer_name_sym.unwrap_or(0)
    }

    fn get_field_serializer_name_sym_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_serializer_name_sym
    }

    fn mut_field_serializer_name_sym_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_serializer_name_sym
    }

    // optional int32 field_serializer_version = 8;

    pub fn clear_field_serializer_version(&mut self) {
        self.field_serializer_version = ::std::option::Option::None;
    }

    pub fn has_field_serializer_version(&self) -> bool {
        self.field_serializer_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_serializer_version(&mut self, v: i32) {
        self.field_serializer_version = ::std::option::Option::Some(v);
    }

    pub fn get_field_serializer_version(&self) -> i32 {
        self.field_serializer_version.unwrap_or(0)
    }

    fn get_field_serializer_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_serializer_version
    }

    fn mut_field_serializer_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_serializer_version
    }

    // optional int32 send_node_sym = 9;

    pub fn clear_send_node_sym(&mut self) {
        self.send_node_sym = ::std::option::Option::None;
    }

    pub fn has_send_node_sym(&self) -> bool {
        self.send_node_sym.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_node_sym(&mut self, v: i32) {
        self.send_node_sym = ::std::option::Option::Some(v);
    }

    pub fn get_send_node_sym(&self) -> i32 {
        self.send_node_sym.unwrap_or(0)
    }

    fn get_send_node_sym_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.send_node_sym
    }

    fn mut_send_node_sym_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.send_node_sym
    }

    // optional int32 var_encoder_sym = 10;

    pub fn clear_var_encoder_sym(&mut self) {
        self.var_encoder_sym = ::std::option::Option::None;
    }

    pub fn has_var_encoder_sym(&self) -> bool {
        self.var_encoder_sym.is_some()
    }

    // Param is passed by value, moved
    pub fn set_var_encoder_sym(&mut self, v: i32) {
        self.var_encoder_sym = ::std::option::Option::Some(v);
    }

    pub fn get_var_encoder_sym(&self) -> i32 {
        self.var_encoder_sym.unwrap_or(0)
    }

    fn get_var_encoder_sym_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.var_encoder_sym
    }

    fn mut_var_encoder_sym_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.var_encoder_sym
    }
}

impl ::protobuf::Message for ProtoFlattenedSerializerField_t {
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
                    self.var_type_sym = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.var_name_sym = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.bit_count = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.low_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.high_value = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.encode_flags = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.field_serializer_name_sym = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.field_serializer_version = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.send_node_sym = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.var_encoder_sym = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.var_type_sym {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.var_name_sym {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.bit_count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.low_value {
            my_size += 5;
        };
        if let Some(v) = self.high_value {
            my_size += 5;
        };
        if let Some(v) = self.encode_flags {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.field_serializer_name_sym {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.field_serializer_version {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.send_node_sym {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.var_encoder_sym {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.var_type_sym {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.var_name_sym {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.bit_count {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.low_value {
            os.write_float(4, v)?;
        };
        if let Some(v) = self.high_value {
            os.write_float(5, v)?;
        };
        if let Some(v) = self.encode_flags {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.field_serializer_name_sym {
            os.write_int32(7, v)?;
        };
        if let Some(v) = self.field_serializer_version {
            os.write_int32(8, v)?;
        };
        if let Some(v) = self.send_node_sym {
            os.write_int32(9, v)?;
        };
        if let Some(v) = self.var_encoder_sym {
            os.write_int32(10, v)?;
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

impl ::protobuf::MessageStatic for ProtoFlattenedSerializerField_t {
    fn new() -> ProtoFlattenedSerializerField_t {
        ProtoFlattenedSerializerField_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProtoFlattenedSerializerField_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "var_type_sym",
                    ProtoFlattenedSerializerField_t::get_var_type_sym_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_var_type_sym_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "var_name_sym",
                    ProtoFlattenedSerializerField_t::get_var_name_sym_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_var_name_sym_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "bit_count",
                    ProtoFlattenedSerializerField_t::get_bit_count_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_bit_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "low_value",
                    ProtoFlattenedSerializerField_t::get_low_value_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_low_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "high_value",
                    ProtoFlattenedSerializerField_t::get_high_value_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_high_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "encode_flags",
                    ProtoFlattenedSerializerField_t::get_encode_flags_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_encode_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "field_serializer_name_sym",
                    ProtoFlattenedSerializerField_t::get_field_serializer_name_sym_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_field_serializer_name_sym_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "field_serializer_version",
                    ProtoFlattenedSerializerField_t::get_field_serializer_version_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_field_serializer_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "send_node_sym",
                    ProtoFlattenedSerializerField_t::get_send_node_sym_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_send_node_sym_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "var_encoder_sym",
                    ProtoFlattenedSerializerField_t::get_var_encoder_sym_for_reflect,
                    ProtoFlattenedSerializerField_t::mut_var_encoder_sym_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProtoFlattenedSerializerField_t>(
                    "ProtoFlattenedSerializerField_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProtoFlattenedSerializerField_t {
    fn clear(&mut self) {
        self.clear_var_type_sym();
        self.clear_var_name_sym();
        self.clear_bit_count();
        self.clear_low_value();
        self.clear_high_value();
        self.clear_encode_flags();
        self.clear_field_serializer_name_sym();
        self.clear_field_serializer_version();
        self.clear_send_node_sym();
        self.clear_var_encoder_sym();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProtoFlattenedSerializerField_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtoFlattenedSerializerField_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProtoFlattenedSerializer_t {
    // message fields
    serializer_name_sym: ::std::option::Option<i32>,
    serializer_version: ::std::option::Option<i32>,
    fields_index: ::std::vec::Vec<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProtoFlattenedSerializer_t {}

impl ProtoFlattenedSerializer_t {
    pub fn new() -> ProtoFlattenedSerializer_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProtoFlattenedSerializer_t {
        static mut instance: ::protobuf::lazy::Lazy<ProtoFlattenedSerializer_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProtoFlattenedSerializer_t,
        };
        unsafe {
            instance.get(ProtoFlattenedSerializer_t::new)
        }
    }

    // optional int32 serializer_name_sym = 1;

    pub fn clear_serializer_name_sym(&mut self) {
        self.serializer_name_sym = ::std::option::Option::None;
    }

    pub fn has_serializer_name_sym(&self) -> bool {
        self.serializer_name_sym.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serializer_name_sym(&mut self, v: i32) {
        self.serializer_name_sym = ::std::option::Option::Some(v);
    }

    pub fn get_serializer_name_sym(&self) -> i32 {
        self.serializer_name_sym.unwrap_or(0)
    }

    fn get_serializer_name_sym_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.serializer_name_sym
    }

    fn mut_serializer_name_sym_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.serializer_name_sym
    }

    // optional int32 serializer_version = 2;

    pub fn clear_serializer_version(&mut self) {
        self.serializer_version = ::std::option::Option::None;
    }

    pub fn has_serializer_version(&self) -> bool {
        self.serializer_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serializer_version(&mut self, v: i32) {
        self.serializer_version = ::std::option::Option::Some(v);
    }

    pub fn get_serializer_version(&self) -> i32 {
        self.serializer_version.unwrap_or(0)
    }

    fn get_serializer_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.serializer_version
    }

    fn mut_serializer_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.serializer_version
    }

    // repeated int32 fields_index = 3;

    pub fn clear_fields_index(&mut self) {
        self.fields_index.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields_index(&mut self, v: ::std::vec::Vec<i32>) {
        self.fields_index = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields_index(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.fields_index
    }

    // Take field
    pub fn take_fields_index(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.fields_index, ::std::vec::Vec::new())
    }

    pub fn get_fields_index(&self) -> &[i32] {
        &self.fields_index
    }

    fn get_fields_index_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.fields_index
    }

    fn mut_fields_index_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.fields_index
    }
}

impl ::protobuf::Message for ProtoFlattenedSerializer_t {
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
                    self.serializer_name_sym = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.serializer_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.fields_index)?;
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
        if let Some(v) = self.serializer_name_sym {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.serializer_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.fields_index {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.serializer_name_sym {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.serializer_version {
            os.write_int32(2, v)?;
        };
        for v in &self.fields_index {
            os.write_int32(3, *v)?;
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

impl ::protobuf::MessageStatic for ProtoFlattenedSerializer_t {
    fn new() -> ProtoFlattenedSerializer_t {
        ProtoFlattenedSerializer_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProtoFlattenedSerializer_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "serializer_name_sym",
                    ProtoFlattenedSerializer_t::get_serializer_name_sym_for_reflect,
                    ProtoFlattenedSerializer_t::mut_serializer_name_sym_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "serializer_version",
                    ProtoFlattenedSerializer_t::get_serializer_version_for_reflect,
                    ProtoFlattenedSerializer_t::mut_serializer_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "fields_index",
                    ProtoFlattenedSerializer_t::get_fields_index_for_reflect,
                    ProtoFlattenedSerializer_t::mut_fields_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProtoFlattenedSerializer_t>(
                    "ProtoFlattenedSerializer_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProtoFlattenedSerializer_t {
    fn clear(&mut self) {
        self.clear_serializer_name_sym();
        self.clear_serializer_version();
        self.clear_fields_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProtoFlattenedSerializer_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProtoFlattenedSerializer_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_FlattenedSerializer {
    // message fields
    serializers: ::protobuf::RepeatedField<ProtoFlattenedSerializer_t>,
    symbols: ::protobuf::RepeatedField<::std::string::String>,
    fields: ::protobuf::RepeatedField<ProtoFlattenedSerializerField_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_FlattenedSerializer {}

impl CSVCMsg_FlattenedSerializer {
    pub fn new() -> CSVCMsg_FlattenedSerializer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_FlattenedSerializer {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_FlattenedSerializer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_FlattenedSerializer,
        };
        unsafe {
            instance.get(CSVCMsg_FlattenedSerializer::new)
        }
    }

    // repeated .dota.ProtoFlattenedSerializer_t serializers = 1;

    pub fn clear_serializers(&mut self) {
        self.serializers.clear();
    }

    // Param is passed by value, moved
    pub fn set_serializers(&mut self, v: ::protobuf::RepeatedField<ProtoFlattenedSerializer_t>) {
        self.serializers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_serializers(&mut self) -> &mut ::protobuf::RepeatedField<ProtoFlattenedSerializer_t> {
        &mut self.serializers
    }

    // Take field
    pub fn take_serializers(&mut self) -> ::protobuf::RepeatedField<ProtoFlattenedSerializer_t> {
        ::std::mem::replace(&mut self.serializers, ::protobuf::RepeatedField::new())
    }

    pub fn get_serializers(&self) -> &[ProtoFlattenedSerializer_t] {
        &self.serializers
    }

    fn get_serializers_for_reflect(&self) -> &::protobuf::RepeatedField<ProtoFlattenedSerializer_t> {
        &self.serializers
    }

    fn mut_serializers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ProtoFlattenedSerializer_t> {
        &mut self.serializers
    }

    // repeated string symbols = 2;

    pub fn clear_symbols(&mut self) {
        self.symbols.clear();
    }

    // Param is passed by value, moved
    pub fn set_symbols(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.symbols = v;
    }

    // Mutable pointer to the field.
    pub fn mut_symbols(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.symbols
    }

    // Take field
    pub fn take_symbols(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.symbols, ::protobuf::RepeatedField::new())
    }

    pub fn get_symbols(&self) -> &[::std::string::String] {
        &self.symbols
    }

    fn get_symbols_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.symbols
    }

    fn mut_symbols_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.symbols
    }

    // repeated .dota.ProtoFlattenedSerializerField_t fields = 3;

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::protobuf::RepeatedField<ProtoFlattenedSerializerField_t>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields(&mut self) -> &mut ::protobuf::RepeatedField<ProtoFlattenedSerializerField_t> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::protobuf::RepeatedField<ProtoFlattenedSerializerField_t> {
        ::std::mem::replace(&mut self.fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_fields(&self) -> &[ProtoFlattenedSerializerField_t] {
        &self.fields
    }

    fn get_fields_for_reflect(&self) -> &::protobuf::RepeatedField<ProtoFlattenedSerializerField_t> {
        &self.fields
    }

    fn mut_fields_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ProtoFlattenedSerializerField_t> {
        &mut self.fields
    }
}

impl ::protobuf::Message for CSVCMsg_FlattenedSerializer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.serializers)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.symbols)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fields)?;
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
        for value in &self.serializers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.symbols {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.serializers {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.symbols {
            os.write_string(2, &v)?;
        };
        for v in &self.fields {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_FlattenedSerializer {
    fn new() -> CSVCMsg_FlattenedSerializer {
        CSVCMsg_FlattenedSerializer::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_FlattenedSerializer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProtoFlattenedSerializer_t>>(
                    "serializers",
                    CSVCMsg_FlattenedSerializer::get_serializers_for_reflect,
                    CSVCMsg_FlattenedSerializer::mut_serializers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "symbols",
                    CSVCMsg_FlattenedSerializer::get_symbols_for_reflect,
                    CSVCMsg_FlattenedSerializer::mut_symbols_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ProtoFlattenedSerializerField_t>>(
                    "fields",
                    CSVCMsg_FlattenedSerializer::get_fields_for_reflect,
                    CSVCMsg_FlattenedSerializer::mut_fields_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_FlattenedSerializer>(
                    "CSVCMsg_FlattenedSerializer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_FlattenedSerializer {
    fn clear(&mut self) {
        self.clear_serializers();
        self.clear_symbols();
        self.clear_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_FlattenedSerializer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_FlattenedSerializer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_StopSound {
    // message fields
    guid: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_StopSound {}

impl CSVCMsg_StopSound {
    pub fn new() -> CSVCMsg_StopSound {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_StopSound {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_StopSound> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_StopSound,
        };
        unsafe {
            instance.get(CSVCMsg_StopSound::new)
        }
    }

    // optional fixed32 guid = 1;

    pub fn clear_guid(&mut self) {
        self.guid = ::std::option::Option::None;
    }

    pub fn has_guid(&self) -> bool {
        self.guid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guid(&mut self, v: u32) {
        self.guid = ::std::option::Option::Some(v);
    }

    pub fn get_guid(&self) -> u32 {
        self.guid.unwrap_or(0)
    }

    fn get_guid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guid
    }

    fn mut_guid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guid
    }
}

impl ::protobuf::Message for CSVCMsg_StopSound {
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
                    let tmp = is.read_fixed32()?;
                    self.guid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guid {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guid {
            os.write_fixed32(1, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_StopSound {
    fn new() -> CSVCMsg_StopSound {
        CSVCMsg_StopSound::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_StopSound>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "guid",
                    CSVCMsg_StopSound::get_guid_for_reflect,
                    CSVCMsg_StopSound::mut_guid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_StopSound>(
                    "CSVCMsg_StopSound",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_StopSound {
    fn clear(&mut self) {
        self.clear_guid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_StopSound {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_StopSound {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CBidirMsg_RebroadcastGameEvent {
    // message fields
    posttoserver: ::std::option::Option<bool>,
    buftype: ::std::option::Option<i32>,
    clientbitcount: ::std::option::Option<u32>,
    receivingclients: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CBidirMsg_RebroadcastGameEvent {}

impl CBidirMsg_RebroadcastGameEvent {
    pub fn new() -> CBidirMsg_RebroadcastGameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CBidirMsg_RebroadcastGameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CBidirMsg_RebroadcastGameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CBidirMsg_RebroadcastGameEvent,
        };
        unsafe {
            instance.get(CBidirMsg_RebroadcastGameEvent::new)
        }
    }

    // optional bool posttoserver = 1;

    pub fn clear_posttoserver(&mut self) {
        self.posttoserver = ::std::option::Option::None;
    }

    pub fn has_posttoserver(&self) -> bool {
        self.posttoserver.is_some()
    }

    // Param is passed by value, moved
    pub fn set_posttoserver(&mut self, v: bool) {
        self.posttoserver = ::std::option::Option::Some(v);
    }

    pub fn get_posttoserver(&self) -> bool {
        self.posttoserver.unwrap_or(false)
    }

    fn get_posttoserver_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.posttoserver
    }

    fn mut_posttoserver_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.posttoserver
    }

    // optional int32 buftype = 2;

    pub fn clear_buftype(&mut self) {
        self.buftype = ::std::option::Option::None;
    }

    pub fn has_buftype(&self) -> bool {
        self.buftype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buftype(&mut self, v: i32) {
        self.buftype = ::std::option::Option::Some(v);
    }

    pub fn get_buftype(&self) -> i32 {
        self.buftype.unwrap_or(0)
    }

    fn get_buftype_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.buftype
    }

    fn mut_buftype_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.buftype
    }

    // optional uint32 clientbitcount = 3;

    pub fn clear_clientbitcount(&mut self) {
        self.clientbitcount = ::std::option::Option::None;
    }

    pub fn has_clientbitcount(&self) -> bool {
        self.clientbitcount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientbitcount(&mut self, v: u32) {
        self.clientbitcount = ::std::option::Option::Some(v);
    }

    pub fn get_clientbitcount(&self) -> u32 {
        self.clientbitcount.unwrap_or(0)
    }

    fn get_clientbitcount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.clientbitcount
    }

    fn mut_clientbitcount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.clientbitcount
    }

    // optional uint64 receivingclients = 4;

    pub fn clear_receivingclients(&mut self) {
        self.receivingclients = ::std::option::Option::None;
    }

    pub fn has_receivingclients(&self) -> bool {
        self.receivingclients.is_some()
    }

    // Param is passed by value, moved
    pub fn set_receivingclients(&mut self, v: u64) {
        self.receivingclients = ::std::option::Option::Some(v);
    }

    pub fn get_receivingclients(&self) -> u64 {
        self.receivingclients.unwrap_or(0)
    }

    fn get_receivingclients_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.receivingclients
    }

    fn mut_receivingclients_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.receivingclients
    }
}

impl ::protobuf::Message for CBidirMsg_RebroadcastGameEvent {
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
                    self.posttoserver = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.buftype = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.clientbitcount = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.receivingclients = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.posttoserver {
            my_size += 2;
        };
        if let Some(v) = self.buftype {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.clientbitcount {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.receivingclients {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.posttoserver {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.buftype {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.clientbitcount {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.receivingclients {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for CBidirMsg_RebroadcastGameEvent {
    fn new() -> CBidirMsg_RebroadcastGameEvent {
        CBidirMsg_RebroadcastGameEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CBidirMsg_RebroadcastGameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "posttoserver",
                    CBidirMsg_RebroadcastGameEvent::get_posttoserver_for_reflect,
                    CBidirMsg_RebroadcastGameEvent::mut_posttoserver_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "buftype",
                    CBidirMsg_RebroadcastGameEvent::get_buftype_for_reflect,
                    CBidirMsg_RebroadcastGameEvent::mut_buftype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "clientbitcount",
                    CBidirMsg_RebroadcastGameEvent::get_clientbitcount_for_reflect,
                    CBidirMsg_RebroadcastGameEvent::mut_clientbitcount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "receivingclients",
                    CBidirMsg_RebroadcastGameEvent::get_receivingclients_for_reflect,
                    CBidirMsg_RebroadcastGameEvent::mut_receivingclients_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CBidirMsg_RebroadcastGameEvent>(
                    "CBidirMsg_RebroadcastGameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CBidirMsg_RebroadcastGameEvent {
    fn clear(&mut self) {
        self.clear_posttoserver();
        self.clear_buftype();
        self.clear_clientbitcount();
        self.clear_receivingclients();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CBidirMsg_RebroadcastGameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CBidirMsg_RebroadcastGameEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CBidirMsg_RebroadcastSource {
    // message fields
    eventsource: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CBidirMsg_RebroadcastSource {}

impl CBidirMsg_RebroadcastSource {
    pub fn new() -> CBidirMsg_RebroadcastSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CBidirMsg_RebroadcastSource {
        static mut instance: ::protobuf::lazy::Lazy<CBidirMsg_RebroadcastSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CBidirMsg_RebroadcastSource,
        };
        unsafe {
            instance.get(CBidirMsg_RebroadcastSource::new)
        }
    }

    // optional int32 eventsource = 1;

    pub fn clear_eventsource(&mut self) {
        self.eventsource = ::std::option::Option::None;
    }

    pub fn has_eventsource(&self) -> bool {
        self.eventsource.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eventsource(&mut self, v: i32) {
        self.eventsource = ::std::option::Option::Some(v);
    }

    pub fn get_eventsource(&self) -> i32 {
        self.eventsource.unwrap_or(0)
    }

    fn get_eventsource_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eventsource
    }

    fn mut_eventsource_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eventsource
    }
}

impl ::protobuf::Message for CBidirMsg_RebroadcastSource {
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
                    self.eventsource = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.eventsource {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eventsource {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for CBidirMsg_RebroadcastSource {
    fn new() -> CBidirMsg_RebroadcastSource {
        CBidirMsg_RebroadcastSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<CBidirMsg_RebroadcastSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eventsource",
                    CBidirMsg_RebroadcastSource::get_eventsource_for_reflect,
                    CBidirMsg_RebroadcastSource::mut_eventsource_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CBidirMsg_RebroadcastSource>(
                    "CBidirMsg_RebroadcastSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CBidirMsg_RebroadcastSource {
    fn clear(&mut self) {
        self.clear_eventsource();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CBidirMsg_RebroadcastSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CBidirMsg_RebroadcastSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SerializedNetAddress_t {
    // message fields
    serializedAddress: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SerializedNetAddress_t {}

impl SerializedNetAddress_t {
    pub fn new() -> SerializedNetAddress_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SerializedNetAddress_t {
        static mut instance: ::protobuf::lazy::Lazy<SerializedNetAddress_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SerializedNetAddress_t,
        };
        unsafe {
            instance.get(SerializedNetAddress_t::new)
        }
    }

    // required bytes serializedAddress = 1;

    pub fn clear_serializedAddress(&mut self) {
        self.serializedAddress.clear();
    }

    pub fn has_serializedAddress(&self) -> bool {
        self.serializedAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serializedAddress(&mut self, v: ::std::vec::Vec<u8>) {
        self.serializedAddress = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serializedAddress(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.serializedAddress.is_none() {
            self.serializedAddress.set_default();
        };
        self.serializedAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_serializedAddress(&mut self) -> ::std::vec::Vec<u8> {
        self.serializedAddress.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_serializedAddress(&self) -> &[u8] {
        match self.serializedAddress.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_serializedAddress_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.serializedAddress
    }

    fn mut_serializedAddress_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.serializedAddress
    }
}

impl ::protobuf::Message for SerializedNetAddress_t {
    fn is_initialized(&self) -> bool {
        if self.serializedAddress.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.serializedAddress)?;
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
        if let Some(v) = self.serializedAddress.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.serializedAddress.as_ref() {
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

impl ::protobuf::MessageStatic for SerializedNetAddress_t {
    fn new() -> SerializedNetAddress_t {
        SerializedNetAddress_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<SerializedNetAddress_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "serializedAddress",
                    SerializedNetAddress_t::get_serializedAddress_for_reflect,
                    SerializedNetAddress_t::mut_serializedAddress_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SerializedNetAddress_t>(
                    "SerializedNetAddress_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SerializedNetAddress_t {
    fn clear(&mut self) {
        self.clear_serializedAddress();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SerializedNetAddress_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SerializedNetAddress_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CBidirMsg_RelayInfo {
    // message fields
    operation: ::std::option::Option<CBidirMsg_RelayInfo_Operation_t>,
    serializedTargetAddress: ::protobuf::SingularPtrField<SerializedNetAddress_t>,
    additionalHops: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CBidirMsg_RelayInfo {}

impl CBidirMsg_RelayInfo {
    pub fn new() -> CBidirMsg_RelayInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CBidirMsg_RelayInfo {
        static mut instance: ::protobuf::lazy::Lazy<CBidirMsg_RelayInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CBidirMsg_RelayInfo,
        };
        unsafe {
            instance.get(CBidirMsg_RelayInfo::new)
        }
    }

    // required .dota.CBidirMsg_RelayInfo.Operation_t operation = 1;

    pub fn clear_operation(&mut self) {
        self.operation = ::std::option::Option::None;
    }

    pub fn has_operation(&self) -> bool {
        self.operation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operation(&mut self, v: CBidirMsg_RelayInfo_Operation_t) {
        self.operation = ::std::option::Option::Some(v);
    }

    pub fn get_operation(&self) -> CBidirMsg_RelayInfo_Operation_t {
        self.operation.unwrap_or(CBidirMsg_RelayInfo_Operation_t::RIO_REQUEST_RELAY)
    }

    fn get_operation_for_reflect(&self) -> &::std::option::Option<CBidirMsg_RelayInfo_Operation_t> {
        &self.operation
    }

    fn mut_operation_for_reflect(&mut self) -> &mut ::std::option::Option<CBidirMsg_RelayInfo_Operation_t> {
        &mut self.operation
    }

    // optional .dota.SerializedNetAddress_t serializedTargetAddress = 2;

    pub fn clear_serializedTargetAddress(&mut self) {
        self.serializedTargetAddress.clear();
    }

    pub fn has_serializedTargetAddress(&self) -> bool {
        self.serializedTargetAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serializedTargetAddress(&mut self, v: SerializedNetAddress_t) {
        self.serializedTargetAddress = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serializedTargetAddress(&mut self) -> &mut SerializedNetAddress_t {
        if self.serializedTargetAddress.is_none() {
            self.serializedTargetAddress.set_default();
        };
        self.serializedTargetAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_serializedTargetAddress(&mut self) -> SerializedNetAddress_t {
        self.serializedTargetAddress.take().unwrap_or_else(|| SerializedNetAddress_t::new())
    }

    pub fn get_serializedTargetAddress(&self) -> &SerializedNetAddress_t {
        self.serializedTargetAddress.as_ref().unwrap_or_else(|| SerializedNetAddress_t::default_instance())
    }

    fn get_serializedTargetAddress_for_reflect(&self) -> &::protobuf::SingularPtrField<SerializedNetAddress_t> {
        &self.serializedTargetAddress
    }

    fn mut_serializedTargetAddress_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SerializedNetAddress_t> {
        &mut self.serializedTargetAddress
    }

    // optional uint32 additionalHops = 3;

    pub fn clear_additionalHops(&mut self) {
        self.additionalHops = ::std::option::Option::None;
    }

    pub fn has_additionalHops(&self) -> bool {
        self.additionalHops.is_some()
    }

    // Param is passed by value, moved
    pub fn set_additionalHops(&mut self, v: u32) {
        self.additionalHops = ::std::option::Option::Some(v);
    }

    pub fn get_additionalHops(&self) -> u32 {
        self.additionalHops.unwrap_or(0)
    }

    fn get_additionalHops_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.additionalHops
    }

    fn mut_additionalHops_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.additionalHops
    }
}

impl ::protobuf::Message for CBidirMsg_RelayInfo {
    fn is_initialized(&self) -> bool {
        if self.operation.is_none() {
            return false;
        };
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
                    let tmp = is.read_enum()?;
                    self.operation = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.serializedTargetAddress)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.additionalHops = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.operation {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.serializedTargetAddress.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.additionalHops {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.operation {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.serializedTargetAddress.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.additionalHops {
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

impl ::protobuf::MessageStatic for CBidirMsg_RelayInfo {
    fn new() -> CBidirMsg_RelayInfo {
        CBidirMsg_RelayInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CBidirMsg_RelayInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CBidirMsg_RelayInfo_Operation_t>>(
                    "operation",
                    CBidirMsg_RelayInfo::get_operation_for_reflect,
                    CBidirMsg_RelayInfo::mut_operation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SerializedNetAddress_t>>(
                    "serializedTargetAddress",
                    CBidirMsg_RelayInfo::get_serializedTargetAddress_for_reflect,
                    CBidirMsg_RelayInfo::mut_serializedTargetAddress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "additionalHops",
                    CBidirMsg_RelayInfo::get_additionalHops_for_reflect,
                    CBidirMsg_RelayInfo::mut_additionalHops_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CBidirMsg_RelayInfo>(
                    "CBidirMsg_RelayInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CBidirMsg_RelayInfo {
    fn clear(&mut self) {
        self.clear_operation();
        self.clear_serializedTargetAddress();
        self.clear_additionalHops();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CBidirMsg_RelayInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CBidirMsg_RelayInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CBidirMsg_RelayInfo_Operation_t {
    RIO_REQUEST_RELAY = 0,
    RIO_WILL_RELAY = 1,
    RIO_NO_ROUTE = 2,
    RIO_REJECT_RELAY = 3,
    RIO_ESTABLISH_CONNECTION = 4,
}

impl ::protobuf::ProtobufEnum for CBidirMsg_RelayInfo_Operation_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CBidirMsg_RelayInfo_Operation_t> {
        match value {
            0 => ::std::option::Option::Some(CBidirMsg_RelayInfo_Operation_t::RIO_REQUEST_RELAY),
            1 => ::std::option::Option::Some(CBidirMsg_RelayInfo_Operation_t::RIO_WILL_RELAY),
            2 => ::std::option::Option::Some(CBidirMsg_RelayInfo_Operation_t::RIO_NO_ROUTE),
            3 => ::std::option::Option::Some(CBidirMsg_RelayInfo_Operation_t::RIO_REJECT_RELAY),
            4 => ::std::option::Option::Some(CBidirMsg_RelayInfo_Operation_t::RIO_ESTABLISH_CONNECTION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CBidirMsg_RelayInfo_Operation_t] = &[
            CBidirMsg_RelayInfo_Operation_t::RIO_REQUEST_RELAY,
            CBidirMsg_RelayInfo_Operation_t::RIO_WILL_RELAY,
            CBidirMsg_RelayInfo_Operation_t::RIO_NO_ROUTE,
            CBidirMsg_RelayInfo_Operation_t::RIO_REJECT_RELAY,
            CBidirMsg_RelayInfo_Operation_t::RIO_ESTABLISH_CONNECTION,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CBidirMsg_RelayInfo_Operation_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CBidirMsg_RelayInfo_Operation_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CBidirMsg_RelayInfo_Operation_t {
}

impl ::protobuf::reflect::ProtobufValue for CBidirMsg_RelayInfo_Operation_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignedPayload_t {
    // message fields
    payloadData: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::std::option::Option<u32>,
    bPayloadEncrypted: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignedPayload_t {}

impl SignedPayload_t {
    pub fn new() -> SignedPayload_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedPayload_t {
        static mut instance: ::protobuf::lazy::Lazy<SignedPayload_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedPayload_t,
        };
        unsafe {
            instance.get(SignedPayload_t::new)
        }
    }

    // required bytes payloadData = 1;

    pub fn clear_payloadData(&mut self) {
        self.payloadData.clear();
    }

    pub fn has_payloadData(&self) -> bool {
        self.payloadData.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payloadData(&mut self, v: ::std::vec::Vec<u8>) {
        self.payloadData = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payloadData(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payloadData.is_none() {
            self.payloadData.set_default();
        };
        self.payloadData.as_mut().unwrap()
    }

    // Take field
    pub fn take_payloadData(&mut self) -> ::std::vec::Vec<u8> {
        self.payloadData.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payloadData(&self) -> &[u8] {
        match self.payloadData.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_payloadData_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.payloadData
    }

    fn mut_payloadData_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.payloadData
    }

    // required uint32 signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature = ::std::option::Option::None;
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: u32) {
        self.signature = ::std::option::Option::Some(v);
    }

    pub fn get_signature(&self) -> u32 {
        self.signature.unwrap_or(0)
    }

    fn get_signature_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.signature
    }

    // required bool bPayloadEncrypted = 3;

    pub fn clear_bPayloadEncrypted(&mut self) {
        self.bPayloadEncrypted = ::std::option::Option::None;
    }

    pub fn has_bPayloadEncrypted(&self) -> bool {
        self.bPayloadEncrypted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bPayloadEncrypted(&mut self, v: bool) {
        self.bPayloadEncrypted = ::std::option::Option::Some(v);
    }

    pub fn get_bPayloadEncrypted(&self) -> bool {
        self.bPayloadEncrypted.unwrap_or(false)
    }

    fn get_bPayloadEncrypted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.bPayloadEncrypted
    }

    fn mut_bPayloadEncrypted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.bPayloadEncrypted
    }
}

impl ::protobuf::Message for SignedPayload_t {
    fn is_initialized(&self) -> bool {
        if self.payloadData.is_none() {
            return false;
        };
        if self.signature.is_none() {
            return false;
        };
        if self.bPayloadEncrypted.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payloadData)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.signature = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.bPayloadEncrypted = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.payloadData.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.signature {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.bPayloadEncrypted {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.payloadData.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.signature {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.bPayloadEncrypted {
            os.write_bool(3, v)?;
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

impl ::protobuf::MessageStatic for SignedPayload_t {
    fn new() -> SignedPayload_t {
        SignedPayload_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignedPayload_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payloadData",
                    SignedPayload_t::get_payloadData_for_reflect,
                    SignedPayload_t::mut_payloadData_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "signature",
                    SignedPayload_t::get_signature_for_reflect,
                    SignedPayload_t::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "bPayloadEncrypted",
                    SignedPayload_t::get_bPayloadEncrypted_for_reflect,
                    SignedPayload_t::mut_bPayloadEncrypted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedPayload_t>(
                    "SignedPayload_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedPayload_t {
    fn clear(&mut self) {
        self.clear_payloadData();
        self.clear_signature();
        self.clear_bPayloadEncrypted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignedPayload_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignedPayload_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CBidirMsg_RelayPacket {
    // message fields
    prevhopcount: ::std::option::Option<u32>,
    originalSender: ::protobuf::SingularPtrField<SerializedNetAddress_t>,
    signedPayload: ::protobuf::SingularPtrField<SignedPayload_t>,
    recipientList: ::protobuf::RepeatedField<CBidirMsg_RelayPacket_SignedDestinationAddress_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CBidirMsg_RelayPacket {}

impl CBidirMsg_RelayPacket {
    pub fn new() -> CBidirMsg_RelayPacket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CBidirMsg_RelayPacket {
        static mut instance: ::protobuf::lazy::Lazy<CBidirMsg_RelayPacket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CBidirMsg_RelayPacket,
        };
        unsafe {
            instance.get(CBidirMsg_RelayPacket::new)
        }
    }

    // required uint32 prevhopcount = 1;

    pub fn clear_prevhopcount(&mut self) {
        self.prevhopcount = ::std::option::Option::None;
    }

    pub fn has_prevhopcount(&self) -> bool {
        self.prevhopcount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prevhopcount(&mut self, v: u32) {
        self.prevhopcount = ::std::option::Option::Some(v);
    }

    pub fn get_prevhopcount(&self) -> u32 {
        self.prevhopcount.unwrap_or(0)
    }

    fn get_prevhopcount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.prevhopcount
    }

    fn mut_prevhopcount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.prevhopcount
    }

    // required .dota.SerializedNetAddress_t originalSender = 2;

    pub fn clear_originalSender(&mut self) {
        self.originalSender.clear();
    }

    pub fn has_originalSender(&self) -> bool {
        self.originalSender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_originalSender(&mut self, v: SerializedNetAddress_t) {
        self.originalSender = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_originalSender(&mut self) -> &mut SerializedNetAddress_t {
        if self.originalSender.is_none() {
            self.originalSender.set_default();
        };
        self.originalSender.as_mut().unwrap()
    }

    // Take field
    pub fn take_originalSender(&mut self) -> SerializedNetAddress_t {
        self.originalSender.take().unwrap_or_else(|| SerializedNetAddress_t::new())
    }

    pub fn get_originalSender(&self) -> &SerializedNetAddress_t {
        self.originalSender.as_ref().unwrap_or_else(|| SerializedNetAddress_t::default_instance())
    }

    fn get_originalSender_for_reflect(&self) -> &::protobuf::SingularPtrField<SerializedNetAddress_t> {
        &self.originalSender
    }

    fn mut_originalSender_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SerializedNetAddress_t> {
        &mut self.originalSender
    }

    // required .dota.SignedPayload_t signedPayload = 3;

    pub fn clear_signedPayload(&mut self) {
        self.signedPayload.clear();
    }

    pub fn has_signedPayload(&self) -> bool {
        self.signedPayload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signedPayload(&mut self, v: SignedPayload_t) {
        self.signedPayload = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signedPayload(&mut self) -> &mut SignedPayload_t {
        if self.signedPayload.is_none() {
            self.signedPayload.set_default();
        };
        self.signedPayload.as_mut().unwrap()
    }

    // Take field
    pub fn take_signedPayload(&mut self) -> SignedPayload_t {
        self.signedPayload.take().unwrap_or_else(|| SignedPayload_t::new())
    }

    pub fn get_signedPayload(&self) -> &SignedPayload_t {
        self.signedPayload.as_ref().unwrap_or_else(|| SignedPayload_t::default_instance())
    }

    fn get_signedPayload_for_reflect(&self) -> &::protobuf::SingularPtrField<SignedPayload_t> {
        &self.signedPayload
    }

    fn mut_signedPayload_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SignedPayload_t> {
        &mut self.signedPayload
    }

    // repeated .dota.CBidirMsg_RelayPacket.SignedDestinationAddress_t recipientList = 4;

    pub fn clear_recipientList(&mut self) {
        self.recipientList.clear();
    }

    // Param is passed by value, moved
    pub fn set_recipientList(&mut self, v: ::protobuf::RepeatedField<CBidirMsg_RelayPacket_SignedDestinationAddress_t>) {
        self.recipientList = v;
    }

    // Mutable pointer to the field.
    pub fn mut_recipientList(&mut self) -> &mut ::protobuf::RepeatedField<CBidirMsg_RelayPacket_SignedDestinationAddress_t> {
        &mut self.recipientList
    }

    // Take field
    pub fn take_recipientList(&mut self) -> ::protobuf::RepeatedField<CBidirMsg_RelayPacket_SignedDestinationAddress_t> {
        ::std::mem::replace(&mut self.recipientList, ::protobuf::RepeatedField::new())
    }

    pub fn get_recipientList(&self) -> &[CBidirMsg_RelayPacket_SignedDestinationAddress_t] {
        &self.recipientList
    }

    fn get_recipientList_for_reflect(&self) -> &::protobuf::RepeatedField<CBidirMsg_RelayPacket_SignedDestinationAddress_t> {
        &self.recipientList
    }

    fn mut_recipientList_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CBidirMsg_RelayPacket_SignedDestinationAddress_t> {
        &mut self.recipientList
    }
}

impl ::protobuf::Message for CBidirMsg_RelayPacket {
    fn is_initialized(&self) -> bool {
        if self.prevhopcount.is_none() {
            return false;
        };
        if self.originalSender.is_none() {
            return false;
        };
        if self.signedPayload.is_none() {
            return false;
        };
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
                    let tmp = is.read_uint32()?;
                    self.prevhopcount = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.originalSender)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signedPayload)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.recipientList)?;
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
        if let Some(v) = self.prevhopcount {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.originalSender.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.signedPayload.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.recipientList {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.prevhopcount {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.originalSender.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.signedPayload.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.recipientList {
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

impl ::protobuf::MessageStatic for CBidirMsg_RelayPacket {
    fn new() -> CBidirMsg_RelayPacket {
        CBidirMsg_RelayPacket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CBidirMsg_RelayPacket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "prevhopcount",
                    CBidirMsg_RelayPacket::get_prevhopcount_for_reflect,
                    CBidirMsg_RelayPacket::mut_prevhopcount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SerializedNetAddress_t>>(
                    "originalSender",
                    CBidirMsg_RelayPacket::get_originalSender_for_reflect,
                    CBidirMsg_RelayPacket::mut_originalSender_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SignedPayload_t>>(
                    "signedPayload",
                    CBidirMsg_RelayPacket::get_signedPayload_for_reflect,
                    CBidirMsg_RelayPacket::mut_signedPayload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CBidirMsg_RelayPacket_SignedDestinationAddress_t>>(
                    "recipientList",
                    CBidirMsg_RelayPacket::get_recipientList_for_reflect,
                    CBidirMsg_RelayPacket::mut_recipientList_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CBidirMsg_RelayPacket>(
                    "CBidirMsg_RelayPacket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CBidirMsg_RelayPacket {
    fn clear(&mut self) {
        self.clear_prevhopcount();
        self.clear_originalSender();
        self.clear_signedPayload();
        self.clear_recipientList();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CBidirMsg_RelayPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CBidirMsg_RelayPacket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CBidirMsg_RelayPacket_SignedDestinationAddress_t {
    // message fields
    serializedAddr: ::protobuf::SingularPtrField<SerializedNetAddress_t>,
    signature: ::std::option::Option<u32>,
    encryptedPayloadKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CBidirMsg_RelayPacket_SignedDestinationAddress_t {}

impl CBidirMsg_RelayPacket_SignedDestinationAddress_t {
    pub fn new() -> CBidirMsg_RelayPacket_SignedDestinationAddress_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CBidirMsg_RelayPacket_SignedDestinationAddress_t {
        static mut instance: ::protobuf::lazy::Lazy<CBidirMsg_RelayPacket_SignedDestinationAddress_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CBidirMsg_RelayPacket_SignedDestinationAddress_t,
        };
        unsafe {
            instance.get(CBidirMsg_RelayPacket_SignedDestinationAddress_t::new)
        }
    }

    // required .dota.SerializedNetAddress_t serializedAddr = 1;

    pub fn clear_serializedAddr(&mut self) {
        self.serializedAddr.clear();
    }

    pub fn has_serializedAddr(&self) -> bool {
        self.serializedAddr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serializedAddr(&mut self, v: SerializedNetAddress_t) {
        self.serializedAddr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serializedAddr(&mut self) -> &mut SerializedNetAddress_t {
        if self.serializedAddr.is_none() {
            self.serializedAddr.set_default();
        };
        self.serializedAddr.as_mut().unwrap()
    }

    // Take field
    pub fn take_serializedAddr(&mut self) -> SerializedNetAddress_t {
        self.serializedAddr.take().unwrap_or_else(|| SerializedNetAddress_t::new())
    }

    pub fn get_serializedAddr(&self) -> &SerializedNetAddress_t {
        self.serializedAddr.as_ref().unwrap_or_else(|| SerializedNetAddress_t::default_instance())
    }

    fn get_serializedAddr_for_reflect(&self) -> &::protobuf::SingularPtrField<SerializedNetAddress_t> {
        &self.serializedAddr
    }

    fn mut_serializedAddr_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SerializedNetAddress_t> {
        &mut self.serializedAddr
    }

    // required uint32 signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature = ::std::option::Option::None;
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: u32) {
        self.signature = ::std::option::Option::Some(v);
    }

    pub fn get_signature(&self) -> u32 {
        self.signature.unwrap_or(0)
    }

    fn get_signature_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.signature
    }

    // optional bytes encryptedPayloadKey = 3;

    pub fn clear_encryptedPayloadKey(&mut self) {
        self.encryptedPayloadKey.clear();
    }

    pub fn has_encryptedPayloadKey(&self) -> bool {
        self.encryptedPayloadKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptedPayloadKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.encryptedPayloadKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encryptedPayloadKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encryptedPayloadKey.is_none() {
            self.encryptedPayloadKey.set_default();
        };
        self.encryptedPayloadKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_encryptedPayloadKey(&mut self) -> ::std::vec::Vec<u8> {
        self.encryptedPayloadKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encryptedPayloadKey(&self) -> &[u8] {
        match self.encryptedPayloadKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encryptedPayloadKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encryptedPayloadKey
    }

    fn mut_encryptedPayloadKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encryptedPayloadKey
    }
}

impl ::protobuf::Message for CBidirMsg_RelayPacket_SignedDestinationAddress_t {
    fn is_initialized(&self) -> bool {
        if self.serializedAddr.is_none() {
            return false;
        };
        if self.signature.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.serializedAddr)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.signature = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encryptedPayloadKey)?;
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
        if let Some(v) = self.serializedAddr.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.signature {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.encryptedPayloadKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.serializedAddr.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.signature {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.encryptedPayloadKey.as_ref() {
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

impl ::protobuf::MessageStatic for CBidirMsg_RelayPacket_SignedDestinationAddress_t {
    fn new() -> CBidirMsg_RelayPacket_SignedDestinationAddress_t {
        CBidirMsg_RelayPacket_SignedDestinationAddress_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CBidirMsg_RelayPacket_SignedDestinationAddress_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SerializedNetAddress_t>>(
                    "serializedAddr",
                    CBidirMsg_RelayPacket_SignedDestinationAddress_t::get_serializedAddr_for_reflect,
                    CBidirMsg_RelayPacket_SignedDestinationAddress_t::mut_serializedAddr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "signature",
                    CBidirMsg_RelayPacket_SignedDestinationAddress_t::get_signature_for_reflect,
                    CBidirMsg_RelayPacket_SignedDestinationAddress_t::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encryptedPayloadKey",
                    CBidirMsg_RelayPacket_SignedDestinationAddress_t::get_encryptedPayloadKey_for_reflect,
                    CBidirMsg_RelayPacket_SignedDestinationAddress_t::mut_encryptedPayloadKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CBidirMsg_RelayPacket_SignedDestinationAddress_t>(
                    "CBidirMsg_RelayPacket_SignedDestinationAddress_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CBidirMsg_RelayPacket_SignedDestinationAddress_t {
    fn clear(&mut self) {
        self.clear_serializedAddr();
        self.clear_signature();
        self.clear_encryptedPayloadKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CBidirMsg_RelayPacket_SignedDestinationAddress_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CBidirMsg_RelayPacket_SignedDestinationAddress_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgServerNetworkStats {
    // message fields
    dedicated: ::std::option::Option<bool>,
    cpu_usage: ::std::option::Option<i32>,
    memory_used_mb: ::std::option::Option<i32>,
    memory_free_mb: ::std::option::Option<i32>,
    uptime: ::std::option::Option<i32>,
    spawn_count: ::std::option::Option<i32>,
    num_clients: ::std::option::Option<i32>,
    num_bots: ::std::option::Option<i32>,
    num_spectators: ::std::option::Option<i32>,
    num_tv_relays: ::std::option::Option<i32>,
    fps: ::std::option::Option<f32>,
    ports: ::protobuf::RepeatedField<CMsgServerNetworkStats_Port>,
    avg_latency_out: ::std::option::Option<f32>,
    avg_latency_in: ::std::option::Option<f32>,
    avg_packets_out: ::std::option::Option<f32>,
    avg_packets_in: ::std::option::Option<f32>,
    avg_loss_out: ::std::option::Option<f32>,
    avg_loss_in: ::std::option::Option<f32>,
    avg_data_out: ::std::option::Option<f32>,
    avg_data_in: ::std::option::Option<f32>,
    total_data_in: ::std::option::Option<u64>,
    total_packets_in: ::std::option::Option<u64>,
    total_data_out: ::std::option::Option<u64>,
    total_packets_out: ::std::option::Option<u64>,
    players: ::protobuf::RepeatedField<CMsgServerNetworkStats_Player>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgServerNetworkStats {}

impl CMsgServerNetworkStats {
    pub fn new() -> CMsgServerNetworkStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgServerNetworkStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgServerNetworkStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgServerNetworkStats,
        };
        unsafe {
            instance.get(CMsgServerNetworkStats::new)
        }
    }

    // optional bool dedicated = 1;

    pub fn clear_dedicated(&mut self) {
        self.dedicated = ::std::option::Option::None;
    }

    pub fn has_dedicated(&self) -> bool {
        self.dedicated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dedicated(&mut self, v: bool) {
        self.dedicated = ::std::option::Option::Some(v);
    }

    pub fn get_dedicated(&self) -> bool {
        self.dedicated.unwrap_or(false)
    }

    fn get_dedicated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.dedicated
    }

    fn mut_dedicated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.dedicated
    }

    // optional int32 cpu_usage = 2;

    pub fn clear_cpu_usage(&mut self) {
        self.cpu_usage = ::std::option::Option::None;
    }

    pub fn has_cpu_usage(&self) -> bool {
        self.cpu_usage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_usage(&mut self, v: i32) {
        self.cpu_usage = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_usage(&self) -> i32 {
        self.cpu_usage.unwrap_or(0)
    }

    fn get_cpu_usage_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.cpu_usage
    }

    fn mut_cpu_usage_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.cpu_usage
    }

    // optional int32 memory_used_mb = 3;

    pub fn clear_memory_used_mb(&mut self) {
        self.memory_used_mb = ::std::option::Option::None;
    }

    pub fn has_memory_used_mb(&self) -> bool {
        self.memory_used_mb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_memory_used_mb(&mut self, v: i32) {
        self.memory_used_mb = ::std::option::Option::Some(v);
    }

    pub fn get_memory_used_mb(&self) -> i32 {
        self.memory_used_mb.unwrap_or(0)
    }

    fn get_memory_used_mb_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.memory_used_mb
    }

    fn mut_memory_used_mb_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.memory_used_mb
    }

    // optional int32 memory_free_mb = 4;

    pub fn clear_memory_free_mb(&mut self) {
        self.memory_free_mb = ::std::option::Option::None;
    }

    pub fn has_memory_free_mb(&self) -> bool {
        self.memory_free_mb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_memory_free_mb(&mut self, v: i32) {
        self.memory_free_mb = ::std::option::Option::Some(v);
    }

    pub fn get_memory_free_mb(&self) -> i32 {
        self.memory_free_mb.unwrap_or(0)
    }

    fn get_memory_free_mb_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.memory_free_mb
    }

    fn mut_memory_free_mb_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.memory_free_mb
    }

    // optional int32 uptime = 5;

    pub fn clear_uptime(&mut self) {
        self.uptime = ::std::option::Option::None;
    }

    pub fn has_uptime(&self) -> bool {
        self.uptime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uptime(&mut self, v: i32) {
        self.uptime = ::std::option::Option::Some(v);
    }

    pub fn get_uptime(&self) -> i32 {
        self.uptime.unwrap_or(0)
    }

    fn get_uptime_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.uptime
    }

    fn mut_uptime_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.uptime
    }

    // optional int32 spawn_count = 6;

    pub fn clear_spawn_count(&mut self) {
        self.spawn_count = ::std::option::Option::None;
    }

    pub fn has_spawn_count(&self) -> bool {
        self.spawn_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawn_count(&mut self, v: i32) {
        self.spawn_count = ::std::option::Option::Some(v);
    }

    pub fn get_spawn_count(&self) -> i32 {
        self.spawn_count.unwrap_or(0)
    }

    fn get_spawn_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.spawn_count
    }

    fn mut_spawn_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.spawn_count
    }

    // optional int32 num_clients = 8;

    pub fn clear_num_clients(&mut self) {
        self.num_clients = ::std::option::Option::None;
    }

    pub fn has_num_clients(&self) -> bool {
        self.num_clients.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_clients(&mut self, v: i32) {
        self.num_clients = ::std::option::Option::Some(v);
    }

    pub fn get_num_clients(&self) -> i32 {
        self.num_clients.unwrap_or(0)
    }

    fn get_num_clients_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_clients
    }

    fn mut_num_clients_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_clients
    }

    // optional int32 num_bots = 9;

    pub fn clear_num_bots(&mut self) {
        self.num_bots = ::std::option::Option::None;
    }

    pub fn has_num_bots(&self) -> bool {
        self.num_bots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_bots(&mut self, v: i32) {
        self.num_bots = ::std::option::Option::Some(v);
    }

    pub fn get_num_bots(&self) -> i32 {
        self.num_bots.unwrap_or(0)
    }

    fn get_num_bots_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_bots
    }

    fn mut_num_bots_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_bots
    }

    // optional int32 num_spectators = 10;

    pub fn clear_num_spectators(&mut self) {
        self.num_spectators = ::std::option::Option::None;
    }

    pub fn has_num_spectators(&self) -> bool {
        self.num_spectators.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_spectators(&mut self, v: i32) {
        self.num_spectators = ::std::option::Option::Some(v);
    }

    pub fn get_num_spectators(&self) -> i32 {
        self.num_spectators.unwrap_or(0)
    }

    fn get_num_spectators_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_spectators
    }

    fn mut_num_spectators_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_spectators
    }

    // optional int32 num_tv_relays = 11;

    pub fn clear_num_tv_relays(&mut self) {
        self.num_tv_relays = ::std::option::Option::None;
    }

    pub fn has_num_tv_relays(&self) -> bool {
        self.num_tv_relays.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_tv_relays(&mut self, v: i32) {
        self.num_tv_relays = ::std::option::Option::Some(v);
    }

    pub fn get_num_tv_relays(&self) -> i32 {
        self.num_tv_relays.unwrap_or(0)
    }

    fn get_num_tv_relays_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_tv_relays
    }

    fn mut_num_tv_relays_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_tv_relays
    }

    // optional float fps = 12;

    pub fn clear_fps(&mut self) {
        self.fps = ::std::option::Option::None;
    }

    pub fn has_fps(&self) -> bool {
        self.fps.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fps(&mut self, v: f32) {
        self.fps = ::std::option::Option::Some(v);
    }

    pub fn get_fps(&self) -> f32 {
        self.fps.unwrap_or(0.)
    }

    fn get_fps_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fps
    }

    fn mut_fps_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fps
    }

    // repeated .dota.CMsgServerNetworkStats.Port ports = 17;

    pub fn clear_ports(&mut self) {
        self.ports.clear();
    }

    // Param is passed by value, moved
    pub fn set_ports(&mut self, v: ::protobuf::RepeatedField<CMsgServerNetworkStats_Port>) {
        self.ports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ports(&mut self) -> &mut ::protobuf::RepeatedField<CMsgServerNetworkStats_Port> {
        &mut self.ports
    }

    // Take field
    pub fn take_ports(&mut self) -> ::protobuf::RepeatedField<CMsgServerNetworkStats_Port> {
        ::std::mem::replace(&mut self.ports, ::protobuf::RepeatedField::new())
    }

    pub fn get_ports(&self) -> &[CMsgServerNetworkStats_Port] {
        &self.ports
    }

    fn get_ports_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgServerNetworkStats_Port> {
        &self.ports
    }

    fn mut_ports_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgServerNetworkStats_Port> {
        &mut self.ports
    }

    // optional float avg_latency_out = 18;

    pub fn clear_avg_latency_out(&mut self) {
        self.avg_latency_out = ::std::option::Option::None;
    }

    pub fn has_avg_latency_out(&self) -> bool {
        self.avg_latency_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_latency_out(&mut self, v: f32) {
        self.avg_latency_out = ::std::option::Option::Some(v);
    }

    pub fn get_avg_latency_out(&self) -> f32 {
        self.avg_latency_out.unwrap_or(0.)
    }

    fn get_avg_latency_out_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.avg_latency_out
    }

    fn mut_avg_latency_out_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.avg_latency_out
    }

    // optional float avg_latency_in = 19;

    pub fn clear_avg_latency_in(&mut self) {
        self.avg_latency_in = ::std::option::Option::None;
    }

    pub fn has_avg_latency_in(&self) -> bool {
        self.avg_latency_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_latency_in(&mut self, v: f32) {
        self.avg_latency_in = ::std::option::Option::Some(v);
    }

    pub fn get_avg_latency_in(&self) -> f32 {
        self.avg_latency_in.unwrap_or(0.)
    }

    fn get_avg_latency_in_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.avg_latency_in
    }

    fn mut_avg_latency_in_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.avg_latency_in
    }

    // optional float avg_packets_out = 20;

    pub fn clear_avg_packets_out(&mut self) {
        self.avg_packets_out = ::std::option::Option::None;
    }

    pub fn has_avg_packets_out(&self) -> bool {
        self.avg_packets_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_packets_out(&mut self, v: f32) {
        self.avg_packets_out = ::std::option::Option::Some(v);
    }

    pub fn get_avg_packets_out(&self) -> f32 {
        self.avg_packets_out.unwrap_or(0.)
    }

    fn get_avg_packets_out_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.avg_packets_out
    }

    fn mut_avg_packets_out_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.avg_packets_out
    }

    // optional float avg_packets_in = 21;

    pub fn clear_avg_packets_in(&mut self) {
        self.avg_packets_in = ::std::option::Option::None;
    }

    pub fn has_avg_packets_in(&self) -> bool {
        self.avg_packets_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_packets_in(&mut self, v: f32) {
        self.avg_packets_in = ::std::option::Option::Some(v);
    }

    pub fn get_avg_packets_in(&self) -> f32 {
        self.avg_packets_in.unwrap_or(0.)
    }

    fn get_avg_packets_in_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.avg_packets_in
    }

    fn mut_avg_packets_in_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.avg_packets_in
    }

    // optional float avg_loss_out = 22;

    pub fn clear_avg_loss_out(&mut self) {
        self.avg_loss_out = ::std::option::Option::None;
    }

    pub fn has_avg_loss_out(&self) -> bool {
        self.avg_loss_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_loss_out(&mut self, v: f32) {
        self.avg_loss_out = ::std::option::Option::Some(v);
    }

    pub fn get_avg_loss_out(&self) -> f32 {
        self.avg_loss_out.unwrap_or(0.)
    }

    fn get_avg_loss_out_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.avg_loss_out
    }

    fn mut_avg_loss_out_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.avg_loss_out
    }

    // optional float avg_loss_in = 23;

    pub fn clear_avg_loss_in(&mut self) {
        self.avg_loss_in = ::std::option::Option::None;
    }

    pub fn has_avg_loss_in(&self) -> bool {
        self.avg_loss_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_loss_in(&mut self, v: f32) {
        self.avg_loss_in = ::std::option::Option::Some(v);
    }

    pub fn get_avg_loss_in(&self) -> f32 {
        self.avg_loss_in.unwrap_or(0.)
    }

    fn get_avg_loss_in_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.avg_loss_in
    }

    fn mut_avg_loss_in_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.avg_loss_in
    }

    // optional float avg_data_out = 24;

    pub fn clear_avg_data_out(&mut self) {
        self.avg_data_out = ::std::option::Option::None;
    }

    pub fn has_avg_data_out(&self) -> bool {
        self.avg_data_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_data_out(&mut self, v: f32) {
        self.avg_data_out = ::std::option::Option::Some(v);
    }

    pub fn get_avg_data_out(&self) -> f32 {
        self.avg_data_out.unwrap_or(0.)
    }

    fn get_avg_data_out_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.avg_data_out
    }

    fn mut_avg_data_out_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.avg_data_out
    }

    // optional float avg_data_in = 25;

    pub fn clear_avg_data_in(&mut self) {
        self.avg_data_in = ::std::option::Option::None;
    }

    pub fn has_avg_data_in(&self) -> bool {
        self.avg_data_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_data_in(&mut self, v: f32) {
        self.avg_data_in = ::std::option::Option::Some(v);
    }

    pub fn get_avg_data_in(&self) -> f32 {
        self.avg_data_in.unwrap_or(0.)
    }

    fn get_avg_data_in_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.avg_data_in
    }

    fn mut_avg_data_in_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.avg_data_in
    }

    // optional uint64 total_data_in = 26;

    pub fn clear_total_data_in(&mut self) {
        self.total_data_in = ::std::option::Option::None;
    }

    pub fn has_total_data_in(&self) -> bool {
        self.total_data_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_data_in(&mut self, v: u64) {
        self.total_data_in = ::std::option::Option::Some(v);
    }

    pub fn get_total_data_in(&self) -> u64 {
        self.total_data_in.unwrap_or(0)
    }

    fn get_total_data_in_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.total_data_in
    }

    fn mut_total_data_in_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.total_data_in
    }

    // optional uint64 total_packets_in = 27;

    pub fn clear_total_packets_in(&mut self) {
        self.total_packets_in = ::std::option::Option::None;
    }

    pub fn has_total_packets_in(&self) -> bool {
        self.total_packets_in.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_packets_in(&mut self, v: u64) {
        self.total_packets_in = ::std::option::Option::Some(v);
    }

    pub fn get_total_packets_in(&self) -> u64 {
        self.total_packets_in.unwrap_or(0)
    }

    fn get_total_packets_in_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.total_packets_in
    }

    fn mut_total_packets_in_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.total_packets_in
    }

    // optional uint64 total_data_out = 28;

    pub fn clear_total_data_out(&mut self) {
        self.total_data_out = ::std::option::Option::None;
    }

    pub fn has_total_data_out(&self) -> bool {
        self.total_data_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_data_out(&mut self, v: u64) {
        self.total_data_out = ::std::option::Option::Some(v);
    }

    pub fn get_total_data_out(&self) -> u64 {
        self.total_data_out.unwrap_or(0)
    }

    fn get_total_data_out_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.total_data_out
    }

    fn mut_total_data_out_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.total_data_out
    }

    // optional uint64 total_packets_out = 29;

    pub fn clear_total_packets_out(&mut self) {
        self.total_packets_out = ::std::option::Option::None;
    }

    pub fn has_total_packets_out(&self) -> bool {
        self.total_packets_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_packets_out(&mut self, v: u64) {
        self.total_packets_out = ::std::option::Option::Some(v);
    }

    pub fn get_total_packets_out(&self) -> u64 {
        self.total_packets_out.unwrap_or(0)
    }

    fn get_total_packets_out_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.total_packets_out
    }

    fn mut_total_packets_out_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.total_packets_out
    }

    // repeated .dota.CMsgServerNetworkStats.Player players = 30;

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: ::protobuf::RepeatedField<CMsgServerNetworkStats_Player>) {
        self.players = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players(&mut self) -> &mut ::protobuf::RepeatedField<CMsgServerNetworkStats_Player> {
        &mut self.players
    }

    // Take field
    pub fn take_players(&mut self) -> ::protobuf::RepeatedField<CMsgServerNetworkStats_Player> {
        ::std::mem::replace(&mut self.players, ::protobuf::RepeatedField::new())
    }

    pub fn get_players(&self) -> &[CMsgServerNetworkStats_Player] {
        &self.players
    }

    fn get_players_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgServerNetworkStats_Player> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgServerNetworkStats_Player> {
        &mut self.players
    }
}

impl ::protobuf::Message for CMsgServerNetworkStats {
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
                    self.dedicated = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.cpu_usage = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.memory_used_mb = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.memory_free_mb = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.uptime = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.spawn_count = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_clients = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_bots = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_spectators = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.num_tv_relays = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.fps = ::std::option::Option::Some(tmp);
                },
                17 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ports)?;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.avg_latency_out = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.avg_latency_in = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.avg_packets_out = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.avg_packets_in = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.avg_loss_out = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.avg_loss_in = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.avg_data_out = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.avg_data_in = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.total_data_in = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.total_packets_in = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.total_data_out = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.total_packets_out = ::std::option::Option::Some(tmp);
                },
                30 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.players)?;
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
        if let Some(v) = self.dedicated {
            my_size += 2;
        };
        if let Some(v) = self.cpu_usage {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.memory_used_mb {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.memory_free_mb {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.uptime {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.spawn_count {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_clients {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_bots {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_spectators {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_tv_relays {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.fps {
            my_size += 5;
        };
        for value in &self.ports {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.avg_latency_out {
            my_size += 6;
        };
        if let Some(v) = self.avg_latency_in {
            my_size += 6;
        };
        if let Some(v) = self.avg_packets_out {
            my_size += 6;
        };
        if let Some(v) = self.avg_packets_in {
            my_size += 6;
        };
        if let Some(v) = self.avg_loss_out {
            my_size += 6;
        };
        if let Some(v) = self.avg_loss_in {
            my_size += 6;
        };
        if let Some(v) = self.avg_data_out {
            my_size += 6;
        };
        if let Some(v) = self.avg_data_in {
            my_size += 6;
        };
        if let Some(v) = self.total_data_in {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.total_packets_in {
            my_size += ::protobuf::rt::value_size(27, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.total_data_out {
            my_size += ::protobuf::rt::value_size(28, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.total_packets_out {
            my_size += ::protobuf::rt::value_size(29, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.players {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dedicated {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.cpu_usage {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.memory_used_mb {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.memory_free_mb {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.uptime {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.spawn_count {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.num_clients {
            os.write_int32(8, v)?;
        };
        if let Some(v) = self.num_bots {
            os.write_int32(9, v)?;
        };
        if let Some(v) = self.num_spectators {
            os.write_int32(10, v)?;
        };
        if let Some(v) = self.num_tv_relays {
            os.write_int32(11, v)?;
        };
        if let Some(v) = self.fps {
            os.write_float(12, v)?;
        };
        for v in &self.ports {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.avg_latency_out {
            os.write_float(18, v)?;
        };
        if let Some(v) = self.avg_latency_in {
            os.write_float(19, v)?;
        };
        if let Some(v) = self.avg_packets_out {
            os.write_float(20, v)?;
        };
        if let Some(v) = self.avg_packets_in {
            os.write_float(21, v)?;
        };
        if let Some(v) = self.avg_loss_out {
            os.write_float(22, v)?;
        };
        if let Some(v) = self.avg_loss_in {
            os.write_float(23, v)?;
        };
        if let Some(v) = self.avg_data_out {
            os.write_float(24, v)?;
        };
        if let Some(v) = self.avg_data_in {
            os.write_float(25, v)?;
        };
        if let Some(v) = self.total_data_in {
            os.write_uint64(26, v)?;
        };
        if let Some(v) = self.total_packets_in {
            os.write_uint64(27, v)?;
        };
        if let Some(v) = self.total_data_out {
            os.write_uint64(28, v)?;
        };
        if let Some(v) = self.total_packets_out {
            os.write_uint64(29, v)?;
        };
        for v in &self.players {
            os.write_tag(30, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgServerNetworkStats {
    fn new() -> CMsgServerNetworkStats {
        CMsgServerNetworkStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgServerNetworkStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "dedicated",
                    CMsgServerNetworkStats::get_dedicated_for_reflect,
                    CMsgServerNetworkStats::mut_dedicated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cpu_usage",
                    CMsgServerNetworkStats::get_cpu_usage_for_reflect,
                    CMsgServerNetworkStats::mut_cpu_usage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "memory_used_mb",
                    CMsgServerNetworkStats::get_memory_used_mb_for_reflect,
                    CMsgServerNetworkStats::mut_memory_used_mb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "memory_free_mb",
                    CMsgServerNetworkStats::get_memory_free_mb_for_reflect,
                    CMsgServerNetworkStats::mut_memory_free_mb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "uptime",
                    CMsgServerNetworkStats::get_uptime_for_reflect,
                    CMsgServerNetworkStats::mut_uptime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "spawn_count",
                    CMsgServerNetworkStats::get_spawn_count_for_reflect,
                    CMsgServerNetworkStats::mut_spawn_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_clients",
                    CMsgServerNetworkStats::get_num_clients_for_reflect,
                    CMsgServerNetworkStats::mut_num_clients_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_bots",
                    CMsgServerNetworkStats::get_num_bots_for_reflect,
                    CMsgServerNetworkStats::mut_num_bots_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_spectators",
                    CMsgServerNetworkStats::get_num_spectators_for_reflect,
                    CMsgServerNetworkStats::mut_num_spectators_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_tv_relays",
                    CMsgServerNetworkStats::get_num_tv_relays_for_reflect,
                    CMsgServerNetworkStats::mut_num_tv_relays_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fps",
                    CMsgServerNetworkStats::get_fps_for_reflect,
                    CMsgServerNetworkStats::mut_fps_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgServerNetworkStats_Port>>(
                    "ports",
                    CMsgServerNetworkStats::get_ports_for_reflect,
                    CMsgServerNetworkStats::mut_ports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "avg_latency_out",
                    CMsgServerNetworkStats::get_avg_latency_out_for_reflect,
                    CMsgServerNetworkStats::mut_avg_latency_out_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "avg_latency_in",
                    CMsgServerNetworkStats::get_avg_latency_in_for_reflect,
                    CMsgServerNetworkStats::mut_avg_latency_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "avg_packets_out",
                    CMsgServerNetworkStats::get_avg_packets_out_for_reflect,
                    CMsgServerNetworkStats::mut_avg_packets_out_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "avg_packets_in",
                    CMsgServerNetworkStats::get_avg_packets_in_for_reflect,
                    CMsgServerNetworkStats::mut_avg_packets_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "avg_loss_out",
                    CMsgServerNetworkStats::get_avg_loss_out_for_reflect,
                    CMsgServerNetworkStats::mut_avg_loss_out_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "avg_loss_in",
                    CMsgServerNetworkStats::get_avg_loss_in_for_reflect,
                    CMsgServerNetworkStats::mut_avg_loss_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "avg_data_out",
                    CMsgServerNetworkStats::get_avg_data_out_for_reflect,
                    CMsgServerNetworkStats::mut_avg_data_out_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "avg_data_in",
                    CMsgServerNetworkStats::get_avg_data_in_for_reflect,
                    CMsgServerNetworkStats::mut_avg_data_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_data_in",
                    CMsgServerNetworkStats::get_total_data_in_for_reflect,
                    CMsgServerNetworkStats::mut_total_data_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_packets_in",
                    CMsgServerNetworkStats::get_total_packets_in_for_reflect,
                    CMsgServerNetworkStats::mut_total_packets_in_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_data_out",
                    CMsgServerNetworkStats::get_total_data_out_for_reflect,
                    CMsgServerNetworkStats::mut_total_data_out_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "total_packets_out",
                    CMsgServerNetworkStats::get_total_packets_out_for_reflect,
                    CMsgServerNetworkStats::mut_total_packets_out_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgServerNetworkStats_Player>>(
                    "players",
                    CMsgServerNetworkStats::get_players_for_reflect,
                    CMsgServerNetworkStats::mut_players_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgServerNetworkStats>(
                    "CMsgServerNetworkStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgServerNetworkStats {
    fn clear(&mut self) {
        self.clear_dedicated();
        self.clear_cpu_usage();
        self.clear_memory_used_mb();
        self.clear_memory_free_mb();
        self.clear_uptime();
        self.clear_spawn_count();
        self.clear_num_clients();
        self.clear_num_bots();
        self.clear_num_spectators();
        self.clear_num_tv_relays();
        self.clear_fps();
        self.clear_ports();
        self.clear_avg_latency_out();
        self.clear_avg_latency_in();
        self.clear_avg_packets_out();
        self.clear_avg_packets_in();
        self.clear_avg_loss_out();
        self.clear_avg_loss_in();
        self.clear_avg_data_out();
        self.clear_avg_data_in();
        self.clear_total_data_in();
        self.clear_total_packets_in();
        self.clear_total_data_out();
        self.clear_total_packets_out();
        self.clear_players();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgServerNetworkStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgServerNetworkStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgServerNetworkStats_Port {
    // message fields
    port: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgServerNetworkStats_Port {}

impl CMsgServerNetworkStats_Port {
    pub fn new() -> CMsgServerNetworkStats_Port {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgServerNetworkStats_Port {
        static mut instance: ::protobuf::lazy::Lazy<CMsgServerNetworkStats_Port> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgServerNetworkStats_Port,
        };
        unsafe {
            instance.get(CMsgServerNetworkStats_Port::new)
        }
    }

    // optional int32 port = 1;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: i32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> i32 {
        self.port.unwrap_or(0)
    }

    fn get_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.port
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for CMsgServerNetworkStats_Port {
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
                    self.port = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.port {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.port {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for CMsgServerNetworkStats_Port {
    fn new() -> CMsgServerNetworkStats_Port {
        CMsgServerNetworkStats_Port::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgServerNetworkStats_Port>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "port",
                    CMsgServerNetworkStats_Port::get_port_for_reflect,
                    CMsgServerNetworkStats_Port::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgServerNetworkStats_Port::get_name_for_reflect,
                    CMsgServerNetworkStats_Port::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgServerNetworkStats_Port>(
                    "CMsgServerNetworkStats_Port",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgServerNetworkStats_Port {
    fn clear(&mut self) {
        self.clear_port();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgServerNetworkStats_Port {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgServerNetworkStats_Port {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgServerNetworkStats_Player {
    // message fields
    steamid: ::std::option::Option<u64>,
    remote_addr: ::protobuf::SingularField<::std::string::String>,
    ping_stddev_ms: ::std::option::Option<i32>,
    ping_avg_ms: ::std::option::Option<i32>,
    packet_loss_pct: ::std::option::Option<f32>,
    is_bot: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgServerNetworkStats_Player {}

impl CMsgServerNetworkStats_Player {
    pub fn new() -> CMsgServerNetworkStats_Player {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgServerNetworkStats_Player {
        static mut instance: ::protobuf::lazy::Lazy<CMsgServerNetworkStats_Player> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgServerNetworkStats_Player,
        };
        unsafe {
            instance.get(CMsgServerNetworkStats_Player::new)
        }
    }

    // optional uint64 steamid = 1;

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

    // optional string remote_addr = 2;

    pub fn clear_remote_addr(&mut self) {
        self.remote_addr.clear();
    }

    pub fn has_remote_addr(&self) -> bool {
        self.remote_addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remote_addr(&mut self, v: ::std::string::String) {
        self.remote_addr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remote_addr(&mut self) -> &mut ::std::string::String {
        if self.remote_addr.is_none() {
            self.remote_addr.set_default();
        };
        self.remote_addr.as_mut().unwrap()
    }

    // Take field
    pub fn take_remote_addr(&mut self) -> ::std::string::String {
        self.remote_addr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_remote_addr(&self) -> &str {
        match self.remote_addr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_remote_addr_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.remote_addr
    }

    fn mut_remote_addr_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.remote_addr
    }

    // optional int32 ping_stddev_ms = 3;

    pub fn clear_ping_stddev_ms(&mut self) {
        self.ping_stddev_ms = ::std::option::Option::None;
    }

    pub fn has_ping_stddev_ms(&self) -> bool {
        self.ping_stddev_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_stddev_ms(&mut self, v: i32) {
        self.ping_stddev_ms = ::std::option::Option::Some(v);
    }

    pub fn get_ping_stddev_ms(&self) -> i32 {
        self.ping_stddev_ms.unwrap_or(0)
    }

    fn get_ping_stddev_ms_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ping_stddev_ms
    }

    fn mut_ping_stddev_ms_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ping_stddev_ms
    }

    // optional int32 ping_avg_ms = 4;

    pub fn clear_ping_avg_ms(&mut self) {
        self.ping_avg_ms = ::std::option::Option::None;
    }

    pub fn has_ping_avg_ms(&self) -> bool {
        self.ping_avg_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_avg_ms(&mut self, v: i32) {
        self.ping_avg_ms = ::std::option::Option::Some(v);
    }

    pub fn get_ping_avg_ms(&self) -> i32 {
        self.ping_avg_ms.unwrap_or(0)
    }

    fn get_ping_avg_ms_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ping_avg_ms
    }

    fn mut_ping_avg_ms_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ping_avg_ms
    }

    // optional float packet_loss_pct = 5;

    pub fn clear_packet_loss_pct(&mut self) {
        self.packet_loss_pct = ::std::option::Option::None;
    }

    pub fn has_packet_loss_pct(&self) -> bool {
        self.packet_loss_pct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packet_loss_pct(&mut self, v: f32) {
        self.packet_loss_pct = ::std::option::Option::Some(v);
    }

    pub fn get_packet_loss_pct(&self) -> f32 {
        self.packet_loss_pct.unwrap_or(0.)
    }

    fn get_packet_loss_pct_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.packet_loss_pct
    }

    fn mut_packet_loss_pct_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.packet_loss_pct
    }

    // optional bool is_bot = 6;

    pub fn clear_is_bot(&mut self) {
        self.is_bot = ::std::option::Option::None;
    }

    pub fn has_is_bot(&self) -> bool {
        self.is_bot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_bot(&mut self, v: bool) {
        self.is_bot = ::std::option::Option::Some(v);
    }

    pub fn get_is_bot(&self) -> bool {
        self.is_bot.unwrap_or(false)
    }

    fn get_is_bot_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_bot
    }

    fn mut_is_bot_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_bot
    }
}

impl ::protobuf::Message for CMsgServerNetworkStats_Player {
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
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.remote_addr)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ping_stddev_ms = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ping_avg_ms = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.packet_loss_pct = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_bot = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.steamid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.remote_addr.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.ping_stddev_ms {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_avg_ms {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packet_loss_pct {
            my_size += 5;
        };
        if let Some(v) = self.is_bot {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.remote_addr.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.ping_stddev_ms {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.ping_avg_ms {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.packet_loss_pct {
            os.write_float(5, v)?;
        };
        if let Some(v) = self.is_bot {
            os.write_bool(6, v)?;
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

impl ::protobuf::MessageStatic for CMsgServerNetworkStats_Player {
    fn new() -> CMsgServerNetworkStats_Player {
        CMsgServerNetworkStats_Player::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgServerNetworkStats_Player>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "steamid",
                    CMsgServerNetworkStats_Player::get_steamid_for_reflect,
                    CMsgServerNetworkStats_Player::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "remote_addr",
                    CMsgServerNetworkStats_Player::get_remote_addr_for_reflect,
                    CMsgServerNetworkStats_Player::mut_remote_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ping_stddev_ms",
                    CMsgServerNetworkStats_Player::get_ping_stddev_ms_for_reflect,
                    CMsgServerNetworkStats_Player::mut_ping_stddev_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ping_avg_ms",
                    CMsgServerNetworkStats_Player::get_ping_avg_ms_for_reflect,
                    CMsgServerNetworkStats_Player::mut_ping_avg_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "packet_loss_pct",
                    CMsgServerNetworkStats_Player::get_packet_loss_pct_for_reflect,
                    CMsgServerNetworkStats_Player::mut_packet_loss_pct_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_bot",
                    CMsgServerNetworkStats_Player::get_is_bot_for_reflect,
                    CMsgServerNetworkStats_Player::mut_is_bot_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgServerNetworkStats_Player>(
                    "CMsgServerNetworkStats_Player",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgServerNetworkStats_Player {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_remote_addr();
        self.clear_ping_stddev_ms();
        self.clear_ping_avg_ms();
        self.clear_packet_loss_pct();
        self.clear_is_bot();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgServerNetworkStats_Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgServerNetworkStats_Player {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CLC_Messages {
    clc_ClientInfo = 20,
    clc_Move = 21,
    clc_VoiceData = 22,
    clc_BaselineAck = 23,
    clc_ListenEvents = 24,
    clc_RespondCvarValue = 25,
    clc_FileCRCCheck = 26,
    clc_LoadingProgress = 27,
    clc_SplitPlayerConnect = 28,
    clc_ClientMessage = 29,
    clc_SplitPlayerDisconnect = 30,
    clc_ServerStatus = 31,
    clc_ServerPing = 32,
    clc_RequestPause = 33,
    clc_CmdKeyValues = 34,
}

impl ::protobuf::ProtobufEnum for CLC_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CLC_Messages> {
        match value {
            20 => ::std::option::Option::Some(CLC_Messages::clc_ClientInfo),
            21 => ::std::option::Option::Some(CLC_Messages::clc_Move),
            22 => ::std::option::Option::Some(CLC_Messages::clc_VoiceData),
            23 => ::std::option::Option::Some(CLC_Messages::clc_BaselineAck),
            24 => ::std::option::Option::Some(CLC_Messages::clc_ListenEvents),
            25 => ::std::option::Option::Some(CLC_Messages::clc_RespondCvarValue),
            26 => ::std::option::Option::Some(CLC_Messages::clc_FileCRCCheck),
            27 => ::std::option::Option::Some(CLC_Messages::clc_LoadingProgress),
            28 => ::std::option::Option::Some(CLC_Messages::clc_SplitPlayerConnect),
            29 => ::std::option::Option::Some(CLC_Messages::clc_ClientMessage),
            30 => ::std::option::Option::Some(CLC_Messages::clc_SplitPlayerDisconnect),
            31 => ::std::option::Option::Some(CLC_Messages::clc_ServerStatus),
            32 => ::std::option::Option::Some(CLC_Messages::clc_ServerPing),
            33 => ::std::option::Option::Some(CLC_Messages::clc_RequestPause),
            34 => ::std::option::Option::Some(CLC_Messages::clc_CmdKeyValues),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CLC_Messages] = &[
            CLC_Messages::clc_ClientInfo,
            CLC_Messages::clc_Move,
            CLC_Messages::clc_VoiceData,
            CLC_Messages::clc_BaselineAck,
            CLC_Messages::clc_ListenEvents,
            CLC_Messages::clc_RespondCvarValue,
            CLC_Messages::clc_FileCRCCheck,
            CLC_Messages::clc_LoadingProgress,
            CLC_Messages::clc_SplitPlayerConnect,
            CLC_Messages::clc_ClientMessage,
            CLC_Messages::clc_SplitPlayerDisconnect,
            CLC_Messages::clc_ServerStatus,
            CLC_Messages::clc_ServerPing,
            CLC_Messages::clc_RequestPause,
            CLC_Messages::clc_CmdKeyValues,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CLC_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CLC_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CLC_Messages {
}

impl ::protobuf::reflect::ProtobufValue for CLC_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SVC_Messages {
    svc_ServerInfo = 40,
    svc_FlattenedSerializer = 41,
    svc_ClassInfo = 42,
    svc_SetPause = 43,
    svc_CreateStringTable = 44,
    svc_UpdateStringTable = 45,
    svc_VoiceInit = 46,
    svc_VoiceData = 47,
    svc_Print = 48,
    svc_Sounds = 49,
    svc_SetView = 50,
    svc_ClearAllStringTables = 51,
    svc_CmdKeyValues = 52,
    svc_BSPDecal = 53,
    svc_SplitScreen = 54,
    svc_PacketEntities = 55,
    svc_Prefetch = 56,
    svc_Menu = 57,
    svc_GetCvarValue = 58,
    svc_StopSound = 59,
    svc_PeerList = 60,
    svc_PacketReliable = 61,
    svc_HLTVStatus = 62,
    svc_ServerSteamID = 63,
    svc_FullFrameSplit = 70,
}

impl ::protobuf::ProtobufEnum for SVC_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SVC_Messages> {
        match value {
            40 => ::std::option::Option::Some(SVC_Messages::svc_ServerInfo),
            41 => ::std::option::Option::Some(SVC_Messages::svc_FlattenedSerializer),
            42 => ::std::option::Option::Some(SVC_Messages::svc_ClassInfo),
            43 => ::std::option::Option::Some(SVC_Messages::svc_SetPause),
            44 => ::std::option::Option::Some(SVC_Messages::svc_CreateStringTable),
            45 => ::std::option::Option::Some(SVC_Messages::svc_UpdateStringTable),
            46 => ::std::option::Option::Some(SVC_Messages::svc_VoiceInit),
            47 => ::std::option::Option::Some(SVC_Messages::svc_VoiceData),
            48 => ::std::option::Option::Some(SVC_Messages::svc_Print),
            49 => ::std::option::Option::Some(SVC_Messages::svc_Sounds),
            50 => ::std::option::Option::Some(SVC_Messages::svc_SetView),
            51 => ::std::option::Option::Some(SVC_Messages::svc_ClearAllStringTables),
            52 => ::std::option::Option::Some(SVC_Messages::svc_CmdKeyValues),
            53 => ::std::option::Option::Some(SVC_Messages::svc_BSPDecal),
            54 => ::std::option::Option::Some(SVC_Messages::svc_SplitScreen),
            55 => ::std::option::Option::Some(SVC_Messages::svc_PacketEntities),
            56 => ::std::option::Option::Some(SVC_Messages::svc_Prefetch),
            57 => ::std::option::Option::Some(SVC_Messages::svc_Menu),
            58 => ::std::option::Option::Some(SVC_Messages::svc_GetCvarValue),
            59 => ::std::option::Option::Some(SVC_Messages::svc_StopSound),
            60 => ::std::option::Option::Some(SVC_Messages::svc_PeerList),
            61 => ::std::option::Option::Some(SVC_Messages::svc_PacketReliable),
            62 => ::std::option::Option::Some(SVC_Messages::svc_HLTVStatus),
            63 => ::std::option::Option::Some(SVC_Messages::svc_ServerSteamID),
            70 => ::std::option::Option::Some(SVC_Messages::svc_FullFrameSplit),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SVC_Messages] = &[
            SVC_Messages::svc_ServerInfo,
            SVC_Messages::svc_FlattenedSerializer,
            SVC_Messages::svc_ClassInfo,
            SVC_Messages::svc_SetPause,
            SVC_Messages::svc_CreateStringTable,
            SVC_Messages::svc_UpdateStringTable,
            SVC_Messages::svc_VoiceInit,
            SVC_Messages::svc_VoiceData,
            SVC_Messages::svc_Print,
            SVC_Messages::svc_Sounds,
            SVC_Messages::svc_SetView,
            SVC_Messages::svc_ClearAllStringTables,
            SVC_Messages::svc_CmdKeyValues,
            SVC_Messages::svc_BSPDecal,
            SVC_Messages::svc_SplitScreen,
            SVC_Messages::svc_PacketEntities,
            SVC_Messages::svc_Prefetch,
            SVC_Messages::svc_Menu,
            SVC_Messages::svc_GetCvarValue,
            SVC_Messages::svc_StopSound,
            SVC_Messages::svc_PeerList,
            SVC_Messages::svc_PacketReliable,
            SVC_Messages::svc_HLTVStatus,
            SVC_Messages::svc_ServerSteamID,
            SVC_Messages::svc_FullFrameSplit,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SVC_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SVC_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SVC_Messages {
}

impl ::protobuf::reflect::ProtobufValue for SVC_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum VoiceDataFormat_t {
    VOICEDATA_FORMAT_STEAM = 0,
    VOICEDATA_FORMAT_ENGINE = 1,
}

impl ::protobuf::ProtobufEnum for VoiceDataFormat_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<VoiceDataFormat_t> {
        match value {
            0 => ::std::option::Option::Some(VoiceDataFormat_t::VOICEDATA_FORMAT_STEAM),
            1 => ::std::option::Option::Some(VoiceDataFormat_t::VOICEDATA_FORMAT_ENGINE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [VoiceDataFormat_t] = &[
            VoiceDataFormat_t::VOICEDATA_FORMAT_STEAM,
            VoiceDataFormat_t::VOICEDATA_FORMAT_ENGINE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<VoiceDataFormat_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("VoiceDataFormat_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for VoiceDataFormat_t {
}

impl ::protobuf::reflect::ProtobufValue for VoiceDataFormat_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RequestPause_t {
    RP_PAUSE = 0,
    RP_UNPAUSE = 1,
    RP_TOGGLEPAUSE = 2,
}

impl ::protobuf::ProtobufEnum for RequestPause_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RequestPause_t> {
        match value {
            0 => ::std::option::Option::Some(RequestPause_t::RP_PAUSE),
            1 => ::std::option::Option::Some(RequestPause_t::RP_UNPAUSE),
            2 => ::std::option::Option::Some(RequestPause_t::RP_TOGGLEPAUSE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RequestPause_t] = &[
            RequestPause_t::RP_PAUSE,
            RequestPause_t::RP_UNPAUSE,
            RequestPause_t::RP_TOGGLEPAUSE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<RequestPause_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RequestPause_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RequestPause_t {
}

impl ::protobuf::reflect::ProtobufValue for RequestPause_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PrefetchType {
    PFT_SOUND = 0,
}

impl ::protobuf::ProtobufEnum for PrefetchType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PrefetchType> {
        match value {
            0 => ::std::option::Option::Some(PrefetchType::PFT_SOUND),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PrefetchType] = &[
            PrefetchType::PFT_SOUND,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<PrefetchType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PrefetchType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PrefetchType {
}

impl ::protobuf::reflect::ProtobufValue for PrefetchType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ESplitScreenMessageType {
    MSG_SPLITSCREEN_ADDUSER = 0,
    MSG_SPLITSCREEN_REMOVEUSER = 1,
}

impl ::protobuf::ProtobufEnum for ESplitScreenMessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ESplitScreenMessageType> {
        match value {
            0 => ::std::option::Option::Some(ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER),
            1 => ::std::option::Option::Some(ESplitScreenMessageType::MSG_SPLITSCREEN_REMOVEUSER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ESplitScreenMessageType] = &[
            ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER,
            ESplitScreenMessageType::MSG_SPLITSCREEN_REMOVEUSER,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ESplitScreenMessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ESplitScreenMessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ESplitScreenMessageType {
}

impl ::protobuf::reflect::ProtobufValue for ESplitScreenMessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EQueryCvarValueStatus {
    eQueryCvarValueStatus_ValueIntact = 0,
    eQueryCvarValueStatus_CvarNotFound = 1,
    eQueryCvarValueStatus_NotACvar = 2,
    eQueryCvarValueStatus_CvarProtected = 3,
}

impl ::protobuf::ProtobufEnum for EQueryCvarValueStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EQueryCvarValueStatus> {
        match value {
            0 => ::std::option::Option::Some(EQueryCvarValueStatus::eQueryCvarValueStatus_ValueIntact),
            1 => ::std::option::Option::Some(EQueryCvarValueStatus::eQueryCvarValueStatus_CvarNotFound),
            2 => ::std::option::Option::Some(EQueryCvarValueStatus::eQueryCvarValueStatus_NotACvar),
            3 => ::std::option::Option::Some(EQueryCvarValueStatus::eQueryCvarValueStatus_CvarProtected),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EQueryCvarValueStatus] = &[
            EQueryCvarValueStatus::eQueryCvarValueStatus_ValueIntact,
            EQueryCvarValueStatus::eQueryCvarValueStatus_CvarNotFound,
            EQueryCvarValueStatus::eQueryCvarValueStatus_NotACvar,
            EQueryCvarValueStatus::eQueryCvarValueStatus_CvarProtected,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EQueryCvarValueStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EQueryCvarValueStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EQueryCvarValueStatus {
}

impl ::protobuf::reflect::ProtobufValue for EQueryCvarValueStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DIALOG_TYPE {
    DIALOG_MSG = 0,
    DIALOG_MENU = 1,
    DIALOG_TEXT = 2,
    DIALOG_ENTRY = 3,
    DIALOG_ASKCONNECT = 4,
}

impl ::protobuf::ProtobufEnum for DIALOG_TYPE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DIALOG_TYPE> {
        match value {
            0 => ::std::option::Option::Some(DIALOG_TYPE::DIALOG_MSG),
            1 => ::std::option::Option::Some(DIALOG_TYPE::DIALOG_MENU),
            2 => ::std::option::Option::Some(DIALOG_TYPE::DIALOG_TEXT),
            3 => ::std::option::Option::Some(DIALOG_TYPE::DIALOG_ENTRY),
            4 => ::std::option::Option::Some(DIALOG_TYPE::DIALOG_ASKCONNECT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DIALOG_TYPE] = &[
            DIALOG_TYPE::DIALOG_MSG,
            DIALOG_TYPE::DIALOG_MENU,
            DIALOG_TYPE::DIALOG_TEXT,
            DIALOG_TYPE::DIALOG_ENTRY,
            DIALOG_TYPE::DIALOG_ASKCONNECT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DIALOG_TYPE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DIALOG_TYPE", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DIALOG_TYPE {
}

impl ::protobuf::reflect::ProtobufValue for DIALOG_TYPE {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SVC_Messages_LowFrequency {
    svc_dummy = 600,
}

impl ::protobuf::ProtobufEnum for SVC_Messages_LowFrequency {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SVC_Messages_LowFrequency> {
        match value {
            600 => ::std::option::Option::Some(SVC_Messages_LowFrequency::svc_dummy),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SVC_Messages_LowFrequency] = &[
            SVC_Messages_LowFrequency::svc_dummy,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SVC_Messages_LowFrequency>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SVC_Messages_LowFrequency", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SVC_Messages_LowFrequency {
}

impl ::protobuf::reflect::ProtobufValue for SVC_Messages_LowFrequency {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Bidirectional_Messages {
    bi_RebroadcastGameEvent = 16,
    bi_RebroadcastSource = 17,
    bi_GameEvent = 18,
}

impl ::protobuf::ProtobufEnum for Bidirectional_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Bidirectional_Messages> {
        match value {
            16 => ::std::option::Option::Some(Bidirectional_Messages::bi_RebroadcastGameEvent),
            17 => ::std::option::Option::Some(Bidirectional_Messages::bi_RebroadcastSource),
            18 => ::std::option::Option::Some(Bidirectional_Messages::bi_GameEvent),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Bidirectional_Messages] = &[
            Bidirectional_Messages::bi_RebroadcastGameEvent,
            Bidirectional_Messages::bi_RebroadcastSource,
            Bidirectional_Messages::bi_GameEvent,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Bidirectional_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Bidirectional_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Bidirectional_Messages {
}

impl ::protobuf::reflect::ProtobufValue for Bidirectional_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Bidirectional_Messages_LowFrequency {
    bi_RelayInfo = 700,
    bi_RelayPacket = 701,
}

impl ::protobuf::ProtobufEnum for Bidirectional_Messages_LowFrequency {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Bidirectional_Messages_LowFrequency> {
        match value {
            700 => ::std::option::Option::Some(Bidirectional_Messages_LowFrequency::bi_RelayInfo),
            701 => ::std::option::Option::Some(Bidirectional_Messages_LowFrequency::bi_RelayPacket),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Bidirectional_Messages_LowFrequency] = &[
            Bidirectional_Messages_LowFrequency::bi_RelayInfo,
            Bidirectional_Messages_LowFrequency::bi_RelayPacket,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Bidirectional_Messages_LowFrequency>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Bidirectional_Messages_LowFrequency", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Bidirectional_Messages_LowFrequency {
}

impl ::protobuf::reflect::ProtobufValue for Bidirectional_Messages_LowFrequency {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x6e, 0x65, 0x74, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x16, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x62, 0x61, 0x73, 0x65, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xd5, 0x01, 0x0a, 0x12, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x65, 0x6e, 0x64,
    0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x0c, 0x73, 0x65, 0x6e, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x72, 0x63, 0x12, 0x21,
    0x0a, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e,
    0x74, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x73, 0x5f, 0x68, 0x6c, 0x74, 0x76, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x06, 0x69, 0x73, 0x48, 0x6c, 0x74, 0x76, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x73,
    0x5f, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x69,
    0x73, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x12, 0x1d, 0x0a, 0x0a, 0x66, 0x72, 0x69, 0x65, 0x6e,
    0x64, 0x73, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x66, 0x72, 0x69,
    0x65, 0x6e, 0x64, 0x73, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x66, 0x72, 0x69, 0x65, 0x6e, 0x64,
    0x73, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x66, 0x72,
    0x69, 0x65, 0x6e, 0x64, 0x73, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x7c, 0x0a, 0x0c, 0x43, 0x43, 0x4c,
    0x43, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x6f, 0x76, 0x65, 0x12, 0x2e, 0x0a, 0x13, 0x6e, 0x75, 0x6d,
    0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x6e, 0x75, 0x6d, 0x42, 0x61, 0x63, 0x6b, 0x75,
    0x70, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x12, 0x28, 0x0a, 0x10, 0x6e, 0x75, 0x6d,
    0x5f, 0x6e, 0x65, 0x77, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0e, 0x6e, 0x75, 0x6d, 0x4e, 0x65, 0x77, 0x43, 0x6f, 0x6d, 0x6d, 0x61,
    0x6e, 0x64, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0xa5, 0x02, 0x0a, 0x0e, 0x43, 0x4d, 0x73, 0x67,
    0x56, 0x6f, 0x69, 0x63, 0x65, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x12, 0x47, 0x0a, 0x06, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61, 0x46, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x5f, 0x74, 0x3a, 0x16, 0x56, 0x4f, 0x49, 0x43, 0x45, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x46,
    0x4f, 0x52, 0x4d, 0x41, 0x54, 0x5f, 0x53, 0x54, 0x45, 0x41, 0x4d, 0x52, 0x06, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x76, 0x6f, 0x69, 0x63, 0x65, 0x5f, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x76, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61,
    0x74, 0x61, 0x12, 0x25, 0x0a, 0x0e, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x73, 0x65, 0x71, 0x75,
    0x65, 0x6e, 0x63, 0x65, 0x42, 0x79, 0x74, 0x65, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x73, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0d, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72,
    0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x52, 0x61, 0x74,
    0x65, 0x12, 0x3c, 0x0a, 0x1a, 0x75, 0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65,
    0x64, 0x5f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x5f, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x18, 0x75, 0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x64, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x22,
    0x67, 0x0a, 0x11, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65,
    0x44, 0x61, 0x74, 0x61, 0x12, 0x2a, 0x0a, 0x05, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56,
    0x6f, 0x69, 0x63, 0x65, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x52, 0x05, 0x61, 0x75, 0x64, 0x69, 0x6f,
    0x12, 0x12, 0x0a, 0x04, 0x78, 0x75, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x04,
    0x78, 0x75, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x22, 0x5b, 0x0a, 0x13, 0x43, 0x43, 0x4c, 0x43,
    0x4d, 0x73, 0x67, 0x5f, 0x42, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x41, 0x63, 0x6b, 0x12,
    0x23, 0x0a, 0x0d, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x5f, 0x74, 0x69, 0x63, 0x6b,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65,
    0x54, 0x69, 0x63, 0x6b, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65,
    0x5f, 0x6e, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x62, 0x61, 0x73, 0x65, 0x6c,
    0x69, 0x6e, 0x65, 0x4e, 0x72, 0x22, 0x35, 0x0a, 0x14, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67,
    0x5f, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x1d, 0x0a,
    0x0a, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x07, 0x52, 0x09, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x4d, 0x61, 0x73, 0x6b, 0x22, 0x7d, 0x0a, 0x18,
    0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x43,
    0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x63, 0x6f, 0x6f, 0x6b,
    0x69, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65,
    0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x64,
    0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x9a, 0x01, 0x0a, 0x14,
    0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x69, 0x6c, 0x65, 0x43, 0x52, 0x43, 0x43,
    0x68, 0x65, 0x63, 0x6b, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6f, 0x64, 0x65, 0x5f, 0x70, 0x61, 0x74,
    0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x63, 0x6f, 0x64, 0x65, 0x50, 0x61, 0x74,
    0x68, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x70, 0x61, 0x74, 0x68, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x64, 0x65, 0x5f, 0x66, 0x69,
    0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x6f,
    0x64, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x66, 0x69,
    0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x66, 0x69,
    0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x63, 0x72, 0x63, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x07, 0x52, 0x03, 0x63, 0x72, 0x63, 0x22, 0x35, 0x0a, 0x17, 0x43, 0x43, 0x4c, 0x43,
    0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x6f, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x6f, 0x67, 0x72,
    0x65, 0x73, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x22,
    0x3c, 0x0a, 0x1a, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74,
    0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x12, 0x1e, 0x0a,
    0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x46, 0x0a,
    0x15, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x6d, 0x73, 0x67, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x33, 0x0a, 0x1d, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67,
    0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x69, 0x73, 0x63,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x22, 0x36, 0x0a, 0x14, 0x43, 0x43,
    0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x12, 0x1e, 0x0a, 0x0a, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x66, 0x69, 0x65, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x66, 0x69,
    0x65, 0x64, 0x22, 0x14, 0x0a, 0x12, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x50, 0x69, 0x6e, 0x67, 0x22, 0x76, 0x0a, 0x14, 0x43, 0x43, 0x4c, 0x43,
    0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x61, 0x75, 0x73, 0x65,
    0x12, 0x3d, 0x0a, 0x0a, 0x70, 0x61, 0x75, 0x73, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x50, 0x61, 0x75, 0x73, 0x65, 0x5f, 0x74, 0x3a, 0x08, 0x52, 0x50, 0x5f, 0x50,
    0x41, 0x55, 0x53, 0x45, 0x52, 0x09, 0x70, 0x61, 0x75, 0x73, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x1f, 0x0a, 0x0b, 0x70, 0x61, 0x75, 0x73, 0x65, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x70, 0x61, 0x75, 0x73, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70,
    0x22, 0x2a, 0x0a, 0x14, 0x43, 0x43, 0x4c, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6d, 0x64, 0x4b,
    0x65, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0xc2, 0x05, 0x0a,
    0x12, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49,
    0x6e, 0x66, 0x6f, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x12,
    0x21, 0x0a, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x43, 0x6f, 0x75,
    0x6e, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x69, 0x73, 0x5f, 0x64, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74,
    0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x69, 0x73, 0x44, 0x65, 0x64, 0x69,
    0x63, 0x61, 0x74, 0x65, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x73, 0x5f, 0x68, 0x6c, 0x74, 0x76,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x69, 0x73, 0x48, 0x6c, 0x74, 0x76, 0x12, 0x1b,
    0x0a, 0x09, 0x69, 0x73, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x08, 0x69, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x12, 0x11, 0x0a, 0x04, 0x63,
    0x5f, 0x6f, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x03, 0x63, 0x4f, 0x73, 0x12, 0x17,
    0x0a, 0x07, 0x6d, 0x61, 0x70, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x07, 0x20, 0x01, 0x28, 0x07, 0x52,
    0x06, 0x6d, 0x61, 0x70, 0x43, 0x72, 0x63, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x08, 0x20, 0x01, 0x28, 0x07, 0x52, 0x09, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x43, 0x72, 0x63, 0x12, 0x28, 0x0a, 0x10, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x09, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x0e, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x72, 0x63,
    0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6d, 0x61, 0x78, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x65, 0x73,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6d, 0x61, 0x78, 0x43, 0x6c, 0x61, 0x73, 0x73,
    0x65, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x73, 0x6c, 0x6f,
    0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x53,
    0x6c, 0x6f, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x74, 0x69, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x76, 0x61, 0x6c, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x74, 0x69, 0x63, 0x6b,
    0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x61, 0x6d, 0x65,
    0x5f, 0x64, 0x69, 0x72, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x67, 0x61, 0x6d, 0x65,
    0x44, 0x69, 0x72, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x61, 0x70, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x0f, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x61, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x19,
    0x0a, 0x08, 0x73, 0x6b, 0x79, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x73, 0x6b, 0x79, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x68, 0x6f, 0x73,
    0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x11, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x68, 0x6f,
    0x73, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x64, 0x64, 0x6f, 0x6e, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x12, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x61, 0x64, 0x64, 0x6f,
    0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x56, 0x0a, 0x13, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x73, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x13, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x26, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x11, 0x67, 0x61, 0x6d, 0x65,
    0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x32, 0x0a,
    0x15, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x61,
    0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x13, 0x67, 0x61,
    0x6d, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x4d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73,
    0x74, 0x22, 0xe5, 0x01, 0x0a, 0x11, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c,
    0x61, 0x73, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x28, 0x0a, 0x10, 0x63, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x5f, 0x6f, 0x6e, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0e, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x4f, 0x6e, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x12, 0x39, 0x0a, 0x07, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x63, 0x6c, 0x61, 0x73,
    0x73, 0x5f, 0x74, 0x52, 0x07, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x65, 0x73, 0x1a, 0x6b, 0x0a, 0x07,
    0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x63, 0x6c, 0x61, 0x73, 0x73,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x63, 0x6c, 0x61, 0x73, 0x73,
    0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x64, 0x61, 0x74,
    0x61, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6c,
    0x61, 0x73, 0x73, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x63, 0x6c, 0x61, 0x73, 0x73, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x2a, 0x0a, 0x10, 0x43, 0x53, 0x56,
    0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x74, 0x50, 0x61, 0x75, 0x73, 0x65, 0x12, 0x16, 0x0a,
    0x06, 0x70, 0x61, 0x75, 0x73, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x70,
    0x61, 0x75, 0x73, 0x65, 0x64, 0x22, 0x60, 0x0a, 0x11, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67,
    0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x71, 0x75,
    0x61, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x71, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x05, 0x63, 0x6f, 0x64, 0x65, 0x63, 0x12, 0x1b, 0x0a, 0x07, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x01, 0x30, 0x52, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x23, 0x0a, 0x0d, 0x43, 0x53, 0x56, 0x43, 0x4d,
    0x73, 0x67, 0x5f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x22, 0xcd, 0x05, 0x0a,
    0x0e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x12,
    0x25, 0x0a, 0x0e, 0x72, 0x65, 0x6c, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x73, 0x6f, 0x75, 0x6e,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x72, 0x65, 0x6c, 0x69, 0x61, 0x62, 0x6c,
    0x65, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x38, 0x0a, 0x06, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53,
    0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x2e, 0x73, 0x6f, 0x75,
    0x6e, 0x64, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x74, 0x52, 0x06, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x73,
    0x1a, 0xd9, 0x04, 0x0a, 0x0b, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x74,
    0x12, 0x19, 0x0a, 0x08, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x78, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x11, 0x52, 0x07, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x58, 0x12, 0x19, 0x0a, 0x08, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x11, 0x52, 0x07, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x59, 0x12, 0x19, 0x0a, 0x08, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x5f, 0x7a, 0x18, 0x03, 0x20, 0x01, 0x28, 0x11, 0x52, 0x07, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x5a, 0x12, 0x16, 0x0a, 0x06, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x06, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x65, 0x6c,
    0x61, 0x79, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a,
    0x64, 0x65, 0x6c, 0x61, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x73, 0x65,
    0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x0e, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x4e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
    0x6c, 0x18, 0x08, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x12, 0x14, 0x0a, 0x05, 0x70, 0x69, 0x74, 0x63, 0x68, 0x18, 0x09, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x05, 0x70, 0x69, 0x74, 0x63, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x1b, 0x0a, 0x09,
    0x73, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x08, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x4e, 0x75, 0x6d, 0x12, 0x28, 0x0a, 0x10, 0x73, 0x6f, 0x75,
    0x6e, 0x64, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x07, 0x52, 0x0e, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x4e, 0x75, 0x6d, 0x48, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x73, 0x70, 0x65, 0x61, 0x6b, 0x65, 0x72, 0x5f, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x73, 0x70, 0x65,
    0x61, 0x6b, 0x65, 0x72, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x61,
    0x6e, 0x64, 0x6f, 0x6d, 0x5f, 0x73, 0x65, 0x65, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0a, 0x72, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x53, 0x65, 0x65, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x73,
    0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0a, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x1f, 0x0a, 0x0b,
    0x69, 0x73, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0a, 0x69, 0x73, 0x53, 0x65, 0x6e, 0x74, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x1d, 0x0a,
    0x0a, 0x69, 0x73, 0x5f, 0x61, 0x6d, 0x62, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x11, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x09, 0x69, 0x73, 0x41, 0x6d, 0x62, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x04,
    0x67, 0x75, 0x69, 0x64, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x67, 0x75, 0x69, 0x64,
    0x12, 0x2a, 0x0a, 0x11, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0f, 0x73, 0x6f, 0x75,
    0x6e, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x49, 0x64, 0x22, 0x77, 0x0a, 0x10,
    0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68,
    0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x49, 0x6e, 0x64, 0x65,
    0x78, 0x12, 0x42, 0x0a, 0x0d, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x50, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x09, 0x50, 0x46,
    0x54, 0x5f, 0x53, 0x4f, 0x55, 0x4e, 0x44, 0x52, 0x0c, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x22, 0x48, 0x0a, 0x0f, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67,
    0x5f, 0x53, 0x65, 0x74, 0x56, 0x69, 0x65, 0x77, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x12, 0x0a, 0x04, 0x73,
    0x6c, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x22,
    0x56, 0x0a, 0x10, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x69, 0x78, 0x41, 0x6e,
    0x67, 0x6c, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x12,
    0x26, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65,
    0x52, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x22, 0x40, 0x0a, 0x16, 0x43, 0x53, 0x56, 0x43, 0x4d,
    0x73, 0x67, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x68, 0x61, 0x69, 0x72, 0x41, 0x6e, 0x67, 0x6c,
    0x65, 0x12, 0x26, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67,
    0x6c, 0x65, 0x52, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x22, 0xcd, 0x01, 0x0a, 0x10, 0x43, 0x53,
    0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x42, 0x53, 0x50, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x12, 0x22,
    0x0a, 0x03, 0x70, 0x6f, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x03, 0x70,
    0x6f, 0x73, 0x12, 0x2e, 0x0a, 0x13, 0x64, 0x65, 0x63, 0x61, 0x6c, 0x5f, 0x74, 0x65, 0x78, 0x74,
    0x75, 0x72, 0x65, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x11, 0x64, 0x65, 0x63, 0x61, 0x6c, 0x54, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6d, 0x6f, 0x64, 0x65,
    0x6c, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x21, 0x0a, 0x0c, 0x6c, 0x6f, 0x77, 0x5f, 0x70, 0x72,
    0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x6c, 0x6f,
    0x77, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x22, 0x98, 0x01, 0x0a, 0x13, 0x43, 0x53,
    0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x53, 0x63, 0x72, 0x65, 0x65,
    0x6e, 0x12, 0x4a, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1d, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x53, 0x63, 0x72,
    0x65, 0x65, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x17,
    0x4d, 0x53, 0x47, 0x5f, 0x53, 0x50, 0x4c, 0x49, 0x54, 0x53, 0x43, 0x52, 0x45, 0x45, 0x4e, 0x5f,
    0x41, 0x44, 0x44, 0x55, 0x53, 0x45, 0x52, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a,
    0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x73, 0x6c, 0x6f,
    0x74, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49,
    0x6e, 0x64, 0x65, 0x78, 0x22, 0x4b, 0x0a, 0x14, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f,
    0x47, 0x65, 0x74, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x16, 0x0a, 0x06,
    0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x63, 0x6f,
    0x6f, 0x6b, 0x69, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x76, 0x61, 0x72, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x76, 0x61, 0x72, 0x4e, 0x61, 0x6d,
    0x65, 0x22, 0x57, 0x0a, 0x0c, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x65, 0x6e,
    0x75, 0x12, 0x1f, 0x0a, 0x0b, 0x64, 0x69, 0x61, 0x6c, 0x6f, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x64, 0x69, 0x61, 0x6c, 0x6f, 0x67, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x6d, 0x65, 0x6e, 0x75, 0x5f, 0x6b, 0x65, 0x79, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0d, 0x6d, 0x65, 0x6e,
    0x75, 0x4b, 0x65, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x22, 0xb2, 0x03, 0x0a, 0x11, 0x43,
    0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65,
    0x12, 0x15, 0x0a, 0x06, 0x69, 0x73, 0x5f, 0x65, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x05, 0x69, 0x73, 0x45, 0x6e, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x6e, 0x65, 0x74, 0x5f, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0c, 0x6e, 0x65, 0x74, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x23, 0x0a,
    0x0d, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x5f, 0x64, 0x65, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x44, 0x65, 0x63, 0x6f, 0x64,
    0x65, 0x72, 0x12, 0x38, 0x0a, 0x05, 0x70, 0x72, 0x6f, 0x70, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x22, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67,
    0x5f, 0x53, 0x65, 0x6e, 0x64, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x73, 0x65, 0x6e, 0x64, 0x70,
    0x72, 0x6f, 0x70, 0x5f, 0x74, 0x52, 0x05, 0x70, 0x72, 0x6f, 0x70, 0x73, 0x1a, 0x80, 0x02, 0x0a,
    0x0a, 0x73, 0x65, 0x6e, 0x64, 0x70, 0x72, 0x6f, 0x70, 0x5f, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12,
    0x19, 0x0a, 0x08, 0x76, 0x61, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x07, 0x76, 0x61, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c,
    0x61, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73,
    0x12, 0x1a, 0x0a, 0x08, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x08, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x12, 0x17, 0x0a, 0x07,
    0x64, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x64,
    0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x6e, 0x75, 0x6d, 0x5f, 0x65, 0x6c, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x6e, 0x75, 0x6d,
    0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x6f, 0x77, 0x5f,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x6c, 0x6f, 0x77,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x68, 0x69, 0x67, 0x68, 0x5f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x68, 0x69, 0x67, 0x68, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x6e, 0x75, 0x6d, 0x5f, 0x62, 0x69, 0x74, 0x73,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x6e, 0x75, 0x6d, 0x42, 0x69, 0x74, 0x73, 0x22,
    0x89, 0x02, 0x0a, 0x15, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x4a, 0x0a, 0x0b, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61,
    0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x2e, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x5f, 0x74, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x73, 0x1a, 0x2f, 0x0a, 0x05, 0x6b, 0x65, 0x79, 0x5f, 0x74, 0x12, 0x12,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x1a, 0x73, 0x0a, 0x0c, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x5f, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x69, 0x64,
    0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x12, 0x35, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x2e,
    0x6b, 0x65, 0x79, 0x5f, 0x74, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x22, 0xb2, 0x03, 0x0a, 0x16,
    0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x45, 0x6e,
    0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x65, 0x6e,
    0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6d, 0x61, 0x78,
    0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x64, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0e, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73,
    0x12, 0x19, 0x0a, 0x08, 0x69, 0x73, 0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x07, 0x69, 0x73, 0x44, 0x65, 0x6c, 0x74, 0x61, 0x12, 0x27, 0x0a, 0x0f, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x42, 0x61, 0x73, 0x65,
    0x6c, 0x69, 0x6e, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x62, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65,
    0x12, 0x1d, 0x0a, 0x0a, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x46, 0x72, 0x6f, 0x6d, 0x12,
    0x1f, 0x0a, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x44, 0x61, 0x74, 0x61,
    0x12, 0x2c, 0x0a, 0x12, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x75, 0x6c, 0x6c,
    0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x10, 0x70, 0x65,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x46, 0x75, 0x6c, 0x6c, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x12, 0x38,
    0x0a, 0x18, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72,
    0x6f, 0x75, 0x70, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x16, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x12, 0x46, 0x0a, 0x1f, 0x6d, 0x61, 0x78, 0x5f,
    0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x63, 0x72, 0x65, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x1d, 0x6d, 0x61, 0x78, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65,
    0x22, 0x74, 0x0a, 0x14, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x54, 0x65, 0x6d, 0x70,
    0x45, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x72, 0x65, 0x6c, 0x69,
    0x61, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x72, 0x65, 0x6c, 0x69,
    0x61, 0x62, 0x6c, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f, 0x65, 0x6e, 0x74, 0x72,
    0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6e, 0x75, 0x6d, 0x45, 0x6e,
    0x74, 0x72, 0x69, 0x65, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x65, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x44, 0x61, 0x74, 0x61, 0x22, 0xe3, 0x02, 0x0a, 0x19, 0x43, 0x53, 0x56, 0x43, 0x4d,
    0x73, 0x67, 0x5f, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54,
    0x61, 0x62, 0x6c, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f,
    0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x6e,
    0x75, 0x6d, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x12, 0x2f, 0x0a, 0x14, 0x75, 0x73, 0x65,
    0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x73, 0x69, 0x7a,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x11, 0x75, 0x73, 0x65, 0x72, 0x44, 0x61, 0x74,
    0x61, 0x46, 0x69, 0x78, 0x65, 0x64, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x75, 0x73,
    0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0c, 0x75, 0x73, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x53, 0x69, 0x7a, 0x65,
    0x12, 0x2d, 0x0a, 0x13, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x73, 0x69,
    0x7a, 0x65, 0x5f, 0x62, 0x69, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x10, 0x75,
    0x73, 0x65, 0x72, 0x44, 0x61, 0x74, 0x61, 0x53, 0x69, 0x7a, 0x65, 0x42, 0x69, 0x74, 0x73, 0x12,
    0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05,
    0x66, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x44, 0x61, 0x74, 0x61, 0x12, 0x2b, 0x0a, 0x11, 0x75, 0x6e, 0x63, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x10, 0x75, 0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x53,
    0x69, 0x7a, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x63, 0x6f, 0x6d, 0x70,
    0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x64, 0x61,
    0x74, 0x61, 0x43, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x22, 0x87, 0x01, 0x0a,
    0x19, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x53,
    0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x49, 0x64, 0x12, 0x2e, 0x0a, 0x13, 0x6e, 0x75, 0x6d, 0x5f, 0x63, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x64, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x11, 0x6e, 0x75, 0x6d, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x45, 0x6e,
    0x74, 0x72, 0x69, 0x65, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x44, 0x61, 0x74, 0x61, 0x22, 0xc0, 0x01, 0x0a, 0x11, 0x43, 0x53, 0x56, 0x43, 0x4d,
    0x73, 0x67, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61, 0x12, 0x2a, 0x0a, 0x05,
    0x61, 0x75, 0x64, 0x69, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x41, 0x75, 0x64, 0x69,
    0x6f, 0x52, 0x05, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x12, 0x16, 0x0a, 0x06, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x12, 0x1c, 0x0a, 0x09, 0x70, 0x72, 0x6f, 0x78, 0x69, 0x6d, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x09, 0x70, 0x72, 0x6f, 0x78, 0x69, 0x6d, 0x69, 0x74, 0x79, 0x12, 0x12,
    0x0a, 0x04, 0x78, 0x75, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x52, 0x04, 0x78, 0x75,
    0x69, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x75, 0x64, 0x69, 0x62, 0x6c, 0x65, 0x5f, 0x6d, 0x61,
    0x73, 0x6b, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x61, 0x75, 0x64, 0x69, 0x62, 0x6c,
    0x65, 0x4d, 0x61, 0x73, 0x6b, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x22, 0x66, 0x0a, 0x16, 0x43, 0x53, 0x56,
    0x43, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x6c, 0x69, 0x61,
    0x62, 0x6c, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x12, 0x22, 0x0a, 0x0c, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x22, 0x70, 0x0a, 0x16, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x75, 0x6c,
    0x6c, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74,
    0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x12,
    0x18, 0x0a, 0x07, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x07, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x74,
    0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x12,
    0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x22, 0x76, 0x0a, 0x12, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x48,
    0x4c, 0x54, 0x56, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x07, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x73,
    0x6c, 0x6f, 0x74, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x73, 0x6c, 0x6f, 0x74,
    0x73, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x72, 0x6f, 0x78, 0x69, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x07, 0x70, 0x72, 0x6f, 0x78, 0x69, 0x65, 0x73, 0x22, 0x32, 0x0a, 0x15, 0x43,
    0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x65,
    0x61, 0x6d, 0x49, 0x44, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x22,
    0x2a, 0x0a, 0x14, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6d, 0x64, 0x4b, 0x65,
    0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x54, 0x0a, 0x0e, 0x43,
    0x4d, 0x73, 0x67, 0x49, 0x50, 0x43, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x23, 0x0a,
    0x0d, 0x63, 0x6f, 0x6d, 0x70, 0x75, 0x74, 0x65, 0x72, 0x5f, 0x67, 0x75, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x0c, 0x63, 0x6f, 0x6d, 0x70, 0x75, 0x74, 0x65, 0x72, 0x47, 0x75,
    0x69, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x49,
    0x64, 0x22, 0xed, 0x01, 0x0a, 0x0e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x50, 0x65, 0x65, 0x72, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x73,
    0x6c, 0x6f, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x53, 0x6c, 0x6f, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x12,
    0x26, 0x0a, 0x03, 0x69, 0x70, 0x63, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x49, 0x50, 0x43, 0x41, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x52, 0x03, 0x69, 0x70, 0x63, 0x12, 0x22, 0x0a, 0x0d, 0x74, 0x68, 0x65, 0x79, 0x5f,
    0x68, 0x65, 0x61, 0x72, 0x5f, 0x79, 0x6f, 0x75, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b,
    0x74, 0x68, 0x65, 0x79, 0x48, 0x65, 0x61, 0x72, 0x59, 0x6f, 0x75, 0x12, 0x22, 0x0a, 0x0d, 0x79,
    0x6f, 0x75, 0x5f, 0x68, 0x65, 0x61, 0x72, 0x5f, 0x74, 0x68, 0x65, 0x6d, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0b, 0x79, 0x6f, 0x75, 0x48, 0x65, 0x61, 0x72, 0x54, 0x68, 0x65, 0x6d, 0x12,
    0x30, 0x0a, 0x14, 0x69, 0x73, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x69,
    0x73, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x48, 0x6f, 0x73,
    0x74, 0x22, 0x3c, 0x0a, 0x10, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x65, 0x65,
    0x72, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x28, 0x0a, 0x04, 0x70, 0x65, 0x65, 0x72, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x50, 0x65, 0x65, 0x72, 0x52, 0x04, 0x70, 0x65, 0x65, 0x72, 0x22,
    0x51, 0x0a, 0x1c, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x65, 0x61, 0x72,
    0x41, 0x6c, 0x6c, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x12,
    0x18, 0x0a, 0x07, 0x6d, 0x61, 0x70, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x6d, 0x61, 0x70, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x6d, 0x61, 0x70,
    0x5f, 0x63, 0x72, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x6d, 0x61, 0x70, 0x43,
    0x72, 0x63, 0x22, 0xa2, 0x03, 0x0a, 0x1f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x46, 0x6c, 0x61, 0x74,
    0x74, 0x65, 0x6e, 0x65, 0x64, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x5f, 0x74, 0x12, 0x20, 0x0a, 0x0c, 0x76, 0x61, 0x72, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x5f, 0x73, 0x79, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x76, 0x61,
    0x72, 0x54, 0x79, 0x70, 0x65, 0x53, 0x79, 0x6d, 0x12, 0x20, 0x0a, 0x0c, 0x76, 0x61, 0x72, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x73, 0x79, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a,
    0x76, 0x61, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x53, 0x79, 0x6d, 0x12, 0x1b, 0x0a, 0x09, 0x62, 0x69,
    0x74, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x62,
    0x69, 0x74, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x6f, 0x77, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x6c, 0x6f, 0x77, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x68, 0x69, 0x67, 0x68, 0x5f, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x68, 0x69, 0x67, 0x68, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x5f, 0x66, 0x6c,
    0x61, 0x67, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x65, 0x6e, 0x63, 0x6f, 0x64,
    0x65, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x39, 0x0a, 0x19, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f,
    0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x5f,
    0x73, 0x79, 0x6d, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x16, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x53, 0x79,
    0x6d, 0x12, 0x38, 0x0a, 0x18, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x72, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x16, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x72, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x22, 0x0a, 0x0d, 0x73,
    0x65, 0x6e, 0x64, 0x5f, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x73, 0x79, 0x6d, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0b, 0x73, 0x65, 0x6e, 0x64, 0x4e, 0x6f, 0x64, 0x65, 0x53, 0x79, 0x6d, 0x12,
    0x26, 0x0a, 0x0f, 0x76, 0x61, 0x72, 0x5f, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x72, 0x5f, 0x73,
    0x79, 0x6d, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x76, 0x61, 0x72, 0x45, 0x6e, 0x63,
    0x6f, 0x64, 0x65, 0x72, 0x53, 0x79, 0x6d, 0x22, 0x9e, 0x01, 0x0a, 0x1a, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x46, 0x6c, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x65, 0x64, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x72, 0x5f, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x73, 0x79, 0x6d, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x11, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x4e,
    0x61, 0x6d, 0x65, 0x53, 0x79, 0x6d, 0x12, 0x2d, 0x0a, 0x12, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x72, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x11, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x5f,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x03, 0x28, 0x05, 0x52, 0x0b, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x73, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x22, 0xba, 0x01, 0x0a, 0x1b, 0x43, 0x53, 0x56,
    0x43, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x6c, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x65, 0x64, 0x53, 0x65,
    0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x12, 0x42, 0x0a, 0x0b, 0x73, 0x65, 0x72, 0x69,
    0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x46, 0x6c, 0x61, 0x74, 0x74, 0x65,
    0x6e, 0x65, 0x64, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x5f, 0x74, 0x52,
    0x0b, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x73, 0x12, 0x18, 0x0a, 0x07,
    0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x07, 0x73,
    0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x73, 0x12, 0x3d, 0x0a, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73,
    0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x46, 0x6c, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x65, 0x64, 0x53, 0x65, 0x72, 0x69,
    0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x74, 0x52, 0x06, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x22, 0x27, 0x0a, 0x11, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67,
    0x5f, 0x53, 0x74, 0x6f, 0x70, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x67, 0x75,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x04, 0x67, 0x75, 0x69, 0x64, 0x22, 0xb2,
    0x01, 0x0a, 0x1e, 0x43, 0x42, 0x69, 0x64, 0x69, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x62,
    0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x12, 0x22, 0x0a, 0x0c, 0x70, 0x6f, 0x73, 0x74, 0x74, 0x6f, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x70, 0x6f, 0x73, 0x74, 0x74, 0x6f, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x62, 0x75, 0x66, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x62, 0x75, 0x66, 0x74, 0x79, 0x70, 0x65, 0x12,
    0x26, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x62, 0x69, 0x74, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x62,
    0x69, 0x74, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x2a, 0x0a, 0x10, 0x72, 0x65, 0x63, 0x65, 0x69,
    0x76, 0x69, 0x6e, 0x67, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x10, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x73, 0x22, 0x3f, 0x0a, 0x1b, 0x43, 0x42, 0x69, 0x64, 0x69, 0x72, 0x4d, 0x73, 0x67,
    0x5f, 0x52, 0x65, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x53, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x22, 0x46, 0x0a, 0x16, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a,
    0x65, 0x64, 0x4e, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x12, 0x2c,
    0x0a, 0x11, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x52, 0x11, 0x73, 0x65, 0x72, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x22, 0xed, 0x02, 0x0a,
    0x13, 0x43, 0x42, 0x69, 0x64, 0x69, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x6c, 0x61, 0x79,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x56, 0x0a, 0x09, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x25, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x42, 0x69, 0x64, 0x69, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x6c, 0x61, 0x79, 0x49, 0x6e,
    0x66, 0x6f, 0x2e, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x3a, 0x11,
    0x52, 0x49, 0x4f, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x5f, 0x52, 0x45, 0x4c, 0x41,
    0x59, 0x52, 0x09, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x56, 0x0a, 0x17,
    0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74,
    0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x4e,
    0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x52, 0x17, 0x73, 0x65, 0x72,
    0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x41, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x12, 0x26, 0x0a, 0x0e, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x48, 0x6f, 0x70, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x61, 0x64,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x48, 0x6f, 0x70, 0x73, 0x22, 0x7e, 0x0a, 0x0b,
    0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x12, 0x15, 0x0a, 0x11, 0x52,
    0x49, 0x4f, 0x5f, 0x52, 0x45, 0x51, 0x55, 0x45, 0x53, 0x54, 0x5f, 0x52, 0x45, 0x4c, 0x41, 0x59,
    0x10, 0x00, 0x12, 0x12, 0x0a, 0x0e, 0x52, 0x49, 0x4f, 0x5f, 0x57, 0x49, 0x4c, 0x4c, 0x5f, 0x52,
    0x45, 0x4c, 0x41, 0x59, 0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c, 0x52, 0x49, 0x4f, 0x5f, 0x4e, 0x4f,
    0x5f, 0x52, 0x4f, 0x55, 0x54, 0x45, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x52, 0x49, 0x4f, 0x5f,
    0x52, 0x45, 0x4a, 0x45, 0x43, 0x54, 0x5f, 0x52, 0x45, 0x4c, 0x41, 0x59, 0x10, 0x03, 0x12, 0x1c,
    0x0a, 0x18, 0x52, 0x49, 0x4f, 0x5f, 0x45, 0x53, 0x54, 0x41, 0x42, 0x4c, 0x49, 0x53, 0x48, 0x5f,
    0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x04, 0x22, 0x7f, 0x0a, 0x0f,
    0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x74, 0x12,
    0x20, 0x0a, 0x0b, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x44, 0x61, 0x74, 0x61, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0c, 0x52, 0x0b, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x44, 0x61, 0x74,
    0x61, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12,
    0x2c, 0x0a, 0x11, 0x62, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x45, 0x6e, 0x63, 0x72, 0x79,
    0x70, 0x74, 0x65, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x52, 0x11, 0x62, 0x50, 0x61, 0x79,
    0x6c, 0x6f, 0x61, 0x64, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x22, 0xd1, 0x03,
    0x0a, 0x15, 0x43, 0x42, 0x69, 0x64, 0x69, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x6c, 0x61,
    0x79, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x22, 0x0a, 0x0c, 0x70, 0x72, 0x65, 0x76, 0x68,
    0x6f, 0x70, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x0c, 0x70,
    0x72, 0x65, 0x76, 0x68, 0x6f, 0x70, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x44, 0x0a, 0x0e, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x53, 0x65, 0x72, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x4e, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f,
    0x74, 0x52, 0x0e, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x53, 0x65, 0x6e, 0x64, 0x65,
    0x72, 0x12, 0x3b, 0x0a, 0x0d, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x61, 0x79, 0x6c, 0x6f,
    0x61, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x74, 0x52,
    0x0d, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x12, 0x5c,
    0x0a, 0x0d, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x36, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x42, 0x69,
    0x64, 0x69, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x6c, 0x61, 0x79, 0x50, 0x61, 0x63, 0x6b,
    0x65, 0x74, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x44, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x52, 0x0d, 0x72,
    0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x1a, 0xb2, 0x01, 0x0a,
    0x1a, 0x53, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x44, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x12, 0x44, 0x0a, 0x0e, 0x73,
    0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x41, 0x64, 0x64, 0x72, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x53, 0x65, 0x72, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x4e, 0x65, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f,
    0x74, 0x52, 0x0e, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x41, 0x64, 0x64,
    0x72, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12,
    0x30, 0x0a, 0x13, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x50, 0x61, 0x79, 0x6c,
    0x6f, 0x61, 0x64, 0x4b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x13, 0x65, 0x6e,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x65, 0x64, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x4b, 0x65,
    0x79, 0x22, 0xa4, 0x09, 0x0a, 0x16, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x73, 0x12, 0x1c, 0x0a, 0x09,
    0x64, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x09, 0x64, 0x65, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x70,
    0x75, 0x5f, 0x75, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x63,
    0x70, 0x75, 0x55, 0x73, 0x61, 0x67, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x6d, 0x65, 0x6d, 0x6f, 0x72,
    0x79, 0x5f, 0x75, 0x73, 0x65, 0x64, 0x5f, 0x6d, 0x62, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0c, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x55, 0x73, 0x65, 0x64, 0x4d, 0x62, 0x12, 0x24, 0x0a,
    0x0e, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x5f, 0x66, 0x72, 0x65, 0x65, 0x5f, 0x6d, 0x62, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x46, 0x72, 0x65,
    0x65, 0x4d, 0x62, 0x12, 0x16, 0x0a, 0x06, 0x75, 0x70, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x06, 0x75, 0x70, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x73,
    0x70, 0x61, 0x77, 0x6e, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0a, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1f, 0x0a, 0x0b,
    0x6e, 0x75, 0x6d, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0a, 0x6e, 0x75, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x19, 0x0a,
    0x08, 0x6e, 0x75, 0x6d, 0x5f, 0x62, 0x6f, 0x74, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x07, 0x6e, 0x75, 0x6d, 0x42, 0x6f, 0x74, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x6e, 0x75, 0x6d, 0x5f,
    0x73, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0d, 0x6e, 0x75, 0x6d, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x12,
    0x22, 0x0a, 0x0d, 0x6e, 0x75, 0x6d, 0x5f, 0x74, 0x76, 0x5f, 0x72, 0x65, 0x6c, 0x61, 0x79, 0x73,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x6e, 0x75, 0x6d, 0x54, 0x76, 0x52, 0x65, 0x6c,
    0x61, 0x79, 0x73, 0x12, 0x10, 0x0a, 0x03, 0x66, 0x70, 0x73, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x03, 0x66, 0x70, 0x73, 0x12, 0x37, 0x0a, 0x05, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x18, 0x11,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x53, 0x74, 0x61,
    0x74, 0x73, 0x2e, 0x50, 0x6f, 0x72, 0x74, 0x52, 0x05, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x12, 0x26,
    0x0a, 0x0f, 0x61, 0x76, 0x67, 0x5f, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x6f, 0x75,
    0x74, 0x18, 0x12, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0d, 0x61, 0x76, 0x67, 0x4c, 0x61, 0x74, 0x65,
    0x6e, 0x63, 0x79, 0x4f, 0x75, 0x74, 0x12, 0x24, 0x0a, 0x0e, 0x61, 0x76, 0x67, 0x5f, 0x6c, 0x61,
    0x74, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x69, 0x6e, 0x18, 0x13, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c,
    0x61, 0x76, 0x67, 0x4c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x49, 0x6e, 0x12, 0x26, 0x0a, 0x0f,
    0x61, 0x76, 0x67, 0x5f, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x6f, 0x75, 0x74, 0x18,
    0x14, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0d, 0x61, 0x76, 0x67, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74,
    0x73, 0x4f, 0x75, 0x74, 0x12, 0x24, 0x0a, 0x0e, 0x61, 0x76, 0x67, 0x5f, 0x70, 0x61, 0x63, 0x6b,
    0x65, 0x74, 0x73, 0x5f, 0x69, 0x6e, 0x18, 0x15, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x61, 0x76,
    0x67, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x49, 0x6e, 0x12, 0x20, 0x0a, 0x0c, 0x61, 0x76,
    0x67, 0x5f, 0x6c, 0x6f, 0x73, 0x73, 0x5f, 0x6f, 0x75, 0x74, 0x18, 0x16, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x0a, 0x61, 0x76, 0x67, 0x4c, 0x6f, 0x73, 0x73, 0x4f, 0x75, 0x74, 0x12, 0x1e, 0x0a, 0x0b,
    0x61, 0x76, 0x67, 0x5f, 0x6c, 0x6f, 0x73, 0x73, 0x5f, 0x69, 0x6e, 0x18, 0x17, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x09, 0x61, 0x76, 0x67, 0x4c, 0x6f, 0x73, 0x73, 0x49, 0x6e, 0x12, 0x20, 0x0a, 0x0c,
    0x61, 0x76, 0x67, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x6f, 0x75, 0x74, 0x18, 0x18, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x0a, 0x61, 0x76, 0x67, 0x44, 0x61, 0x74, 0x61, 0x4f, 0x75, 0x74, 0x12, 0x1e,
    0x0a, 0x0b, 0x61, 0x76, 0x67, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x69, 0x6e, 0x18, 0x19, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x09, 0x61, 0x76, 0x67, 0x44, 0x61, 0x74, 0x61, 0x49, 0x6e, 0x12, 0x22,
    0x0a, 0x0d, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x69, 0x6e, 0x18,
    0x1a, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x44, 0x61, 0x74, 0x61,
    0x49, 0x6e, 0x12, 0x28, 0x0a, 0x10, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x70, 0x61, 0x63, 0x6b,
    0x65, 0x74, 0x73, 0x5f, 0x69, 0x6e, 0x18, 0x1b, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0e, 0x74, 0x6f,
    0x74, 0x61, 0x6c, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x49, 0x6e, 0x12, 0x24, 0x0a, 0x0e,
    0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x6f, 0x75, 0x74, 0x18, 0x1c,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x44, 0x61, 0x74, 0x61, 0x4f,
    0x75, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x5f, 0x70, 0x61, 0x63, 0x6b,
    0x65, 0x74, 0x73, 0x5f, 0x6f, 0x75, 0x74, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0f, 0x74,
    0x6f, 0x74, 0x61, 0x6c, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x4f, 0x75, 0x74, 0x12, 0x3d,
    0x0a, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x18, 0x1e, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x23, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x73, 0x2e, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x52, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x1a, 0x2e, 0x0a,
    0x04, 0x50, 0x6f, 0x72, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x1a, 0xc8, 0x01,
    0x0a, 0x06, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61,
    0x6d, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d,
    0x69, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x61, 0x64, 0x64,
    0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x41,
    0x64, 0x64, 0x72, 0x12, 0x24, 0x0a, 0x0e, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x74, 0x64, 0x64,
    0x65, 0x76, 0x5f, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x70, 0x69, 0x6e,
    0x67, 0x53, 0x74, 0x64, 0x64, 0x65, 0x76, 0x4d, 0x73, 0x12, 0x1e, 0x0a, 0x0b, 0x70, 0x69, 0x6e,
    0x67, 0x5f, 0x61, 0x76, 0x67, 0x5f, 0x6d, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09,
    0x70, 0x69, 0x6e, 0x67, 0x41, 0x76, 0x67, 0x4d, 0x73, 0x12, 0x26, 0x0a, 0x0f, 0x70, 0x61, 0x63,
    0x6b, 0x65, 0x74, 0x5f, 0x6c, 0x6f, 0x73, 0x73, 0x5f, 0x70, 0x63, 0x74, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x0d, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x4c, 0x6f, 0x73, 0x73, 0x50, 0x63,
    0x74, 0x12, 0x15, 0x0a, 0x06, 0x69, 0x73, 0x5f, 0x62, 0x6f, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x05, 0x69, 0x73, 0x42, 0x6f, 0x74, 0x2a, 0xdf, 0x02, 0x0a, 0x0c, 0x43, 0x4c, 0x43,
    0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x0e, 0x63, 0x6c, 0x63,
    0x5f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x14, 0x12, 0x0c, 0x0a,
    0x08, 0x63, 0x6c, 0x63, 0x5f, 0x4d, 0x6f, 0x76, 0x65, 0x10, 0x15, 0x12, 0x11, 0x0a, 0x0d, 0x63,
    0x6c, 0x63, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61, 0x10, 0x16, 0x12, 0x13,
    0x0a, 0x0f, 0x63, 0x6c, 0x63, 0x5f, 0x42, 0x61, 0x73, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x41, 0x63,
    0x6b, 0x10, 0x17, 0x12, 0x14, 0x0a, 0x10, 0x63, 0x6c, 0x63, 0x5f, 0x4c, 0x69, 0x73, 0x74, 0x65,
    0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x10, 0x18, 0x12, 0x18, 0x0a, 0x14, 0x63, 0x6c, 0x63,
    0x5f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x10, 0x19, 0x12, 0x14, 0x0a, 0x10, 0x63, 0x6c, 0x63, 0x5f, 0x46, 0x69, 0x6c, 0x65, 0x43,
    0x52, 0x43, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x10, 0x1a, 0x12, 0x17, 0x0a, 0x13, 0x63, 0x6c, 0x63,
    0x5f, 0x4c, 0x6f, 0x61, 0x64, 0x69, 0x6e, 0x67, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73,
    0x10, 0x1b, 0x12, 0x1a, 0x0a, 0x16, 0x63, 0x6c, 0x63, 0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x50,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x10, 0x1c, 0x12, 0x15,
    0x0a, 0x11, 0x63, 0x6c, 0x63, 0x5f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x10, 0x1d, 0x12, 0x1d, 0x0a, 0x19, 0x63, 0x6c, 0x63, 0x5f, 0x53, 0x70, 0x6c,
    0x69, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x10, 0x1e, 0x12, 0x14, 0x0a, 0x10, 0x63, 0x6c, 0x63, 0x5f, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x10, 0x1f, 0x12, 0x12, 0x0a, 0x0e, 0x63, 0x6c,
    0x63, 0x5f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x50, 0x69, 0x6e, 0x67, 0x10, 0x20, 0x12, 0x14,
    0x0a, 0x10, 0x63, 0x6c, 0x63, 0x5f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x61, 0x75,
    0x73, 0x65, 0x10, 0x21, 0x12, 0x14, 0x0a, 0x10, 0x63, 0x6c, 0x63, 0x5f, 0x43, 0x6d, 0x64, 0x4b,
    0x65, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x10, 0x22, 0x2a, 0x99, 0x04, 0x0a, 0x0c, 0x53,
    0x56, 0x43, 0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x0e, 0x73,
    0x76, 0x63, 0x5f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x28, 0x12,
    0x1b, 0x0a, 0x17, 0x73, 0x76, 0x63, 0x5f, 0x46, 0x6c, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x65, 0x64,
    0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x72, 0x10, 0x29, 0x12, 0x11, 0x0a, 0x0d,
    0x73, 0x76, 0x63, 0x5f, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0x2a, 0x12,
    0x10, 0x0a, 0x0c, 0x73, 0x76, 0x63, 0x5f, 0x53, 0x65, 0x74, 0x50, 0x61, 0x75, 0x73, 0x65, 0x10,
    0x2b, 0x12, 0x19, 0x0a, 0x15, 0x73, 0x76, 0x63, 0x5f, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53,
    0x74, 0x72, 0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x10, 0x2c, 0x12, 0x19, 0x0a, 0x15,
    0x73, 0x76, 0x63, 0x5f, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x10, 0x2d, 0x12, 0x11, 0x0a, 0x0d, 0x73, 0x76, 0x63, 0x5f, 0x56,
    0x6f, 0x69, 0x63, 0x65, 0x49, 0x6e, 0x69, 0x74, 0x10, 0x2e, 0x12, 0x11, 0x0a, 0x0d, 0x73, 0x76,
    0x63, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61, 0x10, 0x2f, 0x12, 0x0d, 0x0a,
    0x09, 0x73, 0x76, 0x63, 0x5f, 0x50, 0x72, 0x69, 0x6e, 0x74, 0x10, 0x30, 0x12, 0x0e, 0x0a, 0x0a,
    0x73, 0x76, 0x63, 0x5f, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x73, 0x10, 0x31, 0x12, 0x0f, 0x0a, 0x0b,
    0x73, 0x76, 0x63, 0x5f, 0x53, 0x65, 0x74, 0x56, 0x69, 0x65, 0x77, 0x10, 0x32, 0x12, 0x1c, 0x0a,
    0x18, 0x73, 0x76, 0x63, 0x5f, 0x43, 0x6c, 0x65, 0x61, 0x72, 0x41, 0x6c, 0x6c, 0x53, 0x74, 0x72,
    0x69, 0x6e, 0x67, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x10, 0x33, 0x12, 0x14, 0x0a, 0x10, 0x73,
    0x76, 0x63, 0x5f, 0x43, 0x6d, 0x64, 0x4b, 0x65, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x10,
    0x34, 0x12, 0x10, 0x0a, 0x0c, 0x73, 0x76, 0x63, 0x5f, 0x42, 0x53, 0x50, 0x44, 0x65, 0x63, 0x61,
    0x6c, 0x10, 0x35, 0x12, 0x13, 0x0a, 0x0f, 0x73, 0x76, 0x63, 0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74,
    0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x10, 0x36, 0x12, 0x16, 0x0a, 0x12, 0x73, 0x76, 0x63, 0x5f,
    0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x69, 0x65, 0x73, 0x10, 0x37,
    0x12, 0x10, 0x0a, 0x0c, 0x73, 0x76, 0x63, 0x5f, 0x50, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68,
    0x10, 0x38, 0x12, 0x0c, 0x0a, 0x08, 0x73, 0x76, 0x63, 0x5f, 0x4d, 0x65, 0x6e, 0x75, 0x10, 0x39,
    0x12, 0x14, 0x0a, 0x10, 0x73, 0x76, 0x63, 0x5f, 0x47, 0x65, 0x74, 0x43, 0x76, 0x61, 0x72, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x10, 0x3a, 0x12, 0x11, 0x0a, 0x0d, 0x73, 0x76, 0x63, 0x5f, 0x53, 0x74,
    0x6f, 0x70, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x10, 0x3b, 0x12, 0x10, 0x0a, 0x0c, 0x73, 0x76, 0x63,
    0x5f, 0x50, 0x65, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x10, 0x3c, 0x12, 0x16, 0x0a, 0x12, 0x73,
    0x76, 0x63, 0x5f, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x6c, 0x69, 0x61, 0x62, 0x6c,
    0x65, 0x10, 0x3d, 0x12, 0x12, 0x0a, 0x0e, 0x73, 0x76, 0x63, 0x5f, 0x48, 0x4c, 0x54, 0x56, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x10, 0x3e, 0x12, 0x15, 0x0a, 0x11, 0x73, 0x76, 0x63, 0x5f, 0x53,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x44, 0x10, 0x3f, 0x12, 0x16,
    0x0a, 0x12, 0x73, 0x76, 0x63, 0x5f, 0x46, 0x75, 0x6c, 0x6c, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x53,
    0x70, 0x6c, 0x69, 0x74, 0x10, 0x46, 0x2a, 0x4c, 0x0a, 0x11, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x44,
    0x61, 0x74, 0x61, 0x46, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x5f, 0x74, 0x12, 0x1a, 0x0a, 0x16, 0x56,
    0x4f, 0x49, 0x43, 0x45, 0x44, 0x41, 0x54, 0x41, 0x5f, 0x46, 0x4f, 0x52, 0x4d, 0x41, 0x54, 0x5f,
    0x53, 0x54, 0x45, 0x41, 0x4d, 0x10, 0x00, 0x12, 0x1b, 0x0a, 0x17, 0x56, 0x4f, 0x49, 0x43, 0x45,
    0x44, 0x41, 0x54, 0x41, 0x5f, 0x46, 0x4f, 0x52, 0x4d, 0x41, 0x54, 0x5f, 0x45, 0x4e, 0x47, 0x49,
    0x4e, 0x45, 0x10, 0x01, 0x2a, 0x42, 0x0a, 0x0e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50,
    0x61, 0x75, 0x73, 0x65, 0x5f, 0x74, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x50, 0x5f, 0x50, 0x41, 0x55,
    0x53, 0x45, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x52, 0x50, 0x5f, 0x55, 0x4e, 0x50, 0x41, 0x55,
    0x53, 0x45, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x52, 0x50, 0x5f, 0x54, 0x4f, 0x47, 0x47, 0x4c,
    0x45, 0x50, 0x41, 0x55, 0x53, 0x45, 0x10, 0x02, 0x2a, 0x1d, 0x0a, 0x0c, 0x50, 0x72, 0x65, 0x66,
    0x65, 0x74, 0x63, 0x68, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x09, 0x50, 0x46, 0x54, 0x5f,
    0x53, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x00, 0x2a, 0x56, 0x0a, 0x17, 0x45, 0x53, 0x70, 0x6c, 0x69,
    0x74, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x1b, 0x0a, 0x17, 0x4d, 0x53, 0x47, 0x5f, 0x53, 0x50, 0x4c, 0x49, 0x54, 0x53,
    0x43, 0x52, 0x45, 0x45, 0x4e, 0x5f, 0x41, 0x44, 0x44, 0x55, 0x53, 0x45, 0x52, 0x10, 0x00, 0x12,
    0x1e, 0x0a, 0x1a, 0x4d, 0x53, 0x47, 0x5f, 0x53, 0x50, 0x4c, 0x49, 0x54, 0x53, 0x43, 0x52, 0x45,
    0x45, 0x4e, 0x5f, 0x52, 0x45, 0x4d, 0x4f, 0x56, 0x45, 0x55, 0x53, 0x45, 0x52, 0x10, 0x01, 0x2a,
    0xb3, 0x01, 0x0a, 0x15, 0x45, 0x51, 0x75, 0x65, 0x72, 0x79, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x25, 0x0a, 0x21, 0x65, 0x51, 0x75,
    0x65, 0x72, 0x79, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x5f, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x49, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x10, 0x00,
    0x12, 0x26, 0x0a, 0x22, 0x65, 0x51, 0x75, 0x65, 0x72, 0x79, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x43, 0x76, 0x61, 0x72, 0x4e, 0x6f,
    0x74, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x10, 0x01, 0x12, 0x22, 0x0a, 0x1e, 0x65, 0x51, 0x75, 0x65,
    0x72, 0x79, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x5f, 0x4e, 0x6f, 0x74, 0x41, 0x43, 0x76, 0x61, 0x72, 0x10, 0x02, 0x12, 0x27, 0x0a, 0x23,
    0x65, 0x51, 0x75, 0x65, 0x72, 0x79, 0x43, 0x76, 0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x43, 0x76, 0x61, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x65, 0x63,
    0x74, 0x65, 0x64, 0x10, 0x03, 0x2a, 0x68, 0x0a, 0x0b, 0x44, 0x49, 0x41, 0x4c, 0x4f, 0x47, 0x5f,
    0x54, 0x59, 0x50, 0x45, 0x12, 0x0e, 0x0a, 0x0a, 0x44, 0x49, 0x41, 0x4c, 0x4f, 0x47, 0x5f, 0x4d,
    0x53, 0x47, 0x10, 0x00, 0x12, 0x0f, 0x0a, 0x0b, 0x44, 0x49, 0x41, 0x4c, 0x4f, 0x47, 0x5f, 0x4d,
    0x45, 0x4e, 0x55, 0x10, 0x01, 0x12, 0x0f, 0x0a, 0x0b, 0x44, 0x49, 0x41, 0x4c, 0x4f, 0x47, 0x5f,
    0x54, 0x45, 0x58, 0x54, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x44, 0x49, 0x41, 0x4c, 0x4f, 0x47,
    0x5f, 0x45, 0x4e, 0x54, 0x52, 0x59, 0x10, 0x03, 0x12, 0x15, 0x0a, 0x11, 0x44, 0x49, 0x41, 0x4c,
    0x4f, 0x47, 0x5f, 0x41, 0x53, 0x4b, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x10, 0x04, 0x2a,
    0x2b, 0x0a, 0x19, 0x53, 0x56, 0x43, 0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f,
    0x4c, 0x6f, 0x77, 0x46, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79, 0x12, 0x0e, 0x0a, 0x09,
    0x73, 0x76, 0x63, 0x5f, 0x64, 0x75, 0x6d, 0x6d, 0x79, 0x10, 0xd8, 0x04, 0x2a, 0x61, 0x0a, 0x16,
    0x42, 0x69, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x1b, 0x0a, 0x17, 0x62, 0x69, 0x5f, 0x52, 0x65, 0x62,
    0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x10, 0x10, 0x12, 0x18, 0x0a, 0x14, 0x62, 0x69, 0x5f, 0x52, 0x65, 0x62, 0x72, 0x6f, 0x61,
    0x64, 0x63, 0x61, 0x73, 0x74, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x10, 0x11, 0x12, 0x10, 0x0a,
    0x0c, 0x62, 0x69, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x12, 0x2a,
    0x4d, 0x0a, 0x23, 0x42, 0x69, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x4c, 0x6f, 0x77, 0x46, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x6e, 0x63, 0x79, 0x12, 0x11, 0x0a, 0x0c, 0x62, 0x69, 0x5f, 0x52, 0x65, 0x6c,
    0x61, 0x79, 0x49, 0x6e, 0x66, 0x6f, 0x10, 0xbc, 0x05, 0x12, 0x13, 0x0a, 0x0e, 0x62, 0x69, 0x5f,
    0x52, 0x65, 0x6c, 0x61, 0x79, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x10, 0xbd, 0x05, 0x42, 0x03,
    0x80, 0x01, 0x00, 0x4a, 0xb9, 0xbd, 0x01, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xad, 0x04, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1f, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x06, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x06, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x06, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x06, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x06, 0x1d,
    0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x08, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x08, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x09, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x09, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x09,
    0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0a, 0x13, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x0b, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x0b, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0c, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x0d, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x0e, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0e, 0x08,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0e, 0x1f, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x06, 0x02, 0x12, 0x03, 0x0f, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07,
    0x12, 0x03, 0x10, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x10, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x10, 0x1e,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x11, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x11, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x11, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x09, 0x12, 0x03, 0x12, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01,
    0x12, 0x03, 0x12, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03,
    0x12, 0x1c, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x13, 0x08, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x13, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x13, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x14, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x0b, 0x01, 0x12, 0x03, 0x14, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02,
    0x12, 0x03, 0x14, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x15,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x15, 0x08, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x15, 0x19, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x16, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x16, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x0d, 0x02, 0x12, 0x03, 0x16, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0e, 0x12,
    0x03, 0x17, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x17,
    0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x17, 0x1b, 0x1d,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x1a, 0x00, 0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x1b, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x19,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1c, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x1d, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1d, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x1d, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x08, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03, 0x1e, 0x17, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x1f, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x05, 0x12, 0x03, 0x20,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x20, 0x08, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x02, 0x12, 0x03, 0x20, 0x20, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x06, 0x12, 0x03, 0x21, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x21, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x06, 0x02, 0x12, 0x03, 0x21, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x07, 0x12,
    0x03, 0x22, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x22,
    0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x02, 0x12, 0x03, 0x22, 0x18, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x08, 0x12, 0x03, 0x23, 0x08, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x23, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x08, 0x02, 0x12, 0x03, 0x23, 0x14, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x09, 0x12, 0x03, 0x24, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x09, 0x01, 0x12,
    0x03, 0x24, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x09, 0x02, 0x12, 0x03, 0x24,
    0x15, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x25, 0x08, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x25, 0x08, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x25, 0x16, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x0b, 0x12, 0x03, 0x26, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0b,
    0x01, 0x12, 0x03, 0x26, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0b, 0x02, 0x12,
    0x03, 0x26, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0c, 0x12, 0x03, 0x27, 0x08,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x27, 0x08, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x27, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x0d, 0x12, 0x03, 0x28, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x0d, 0x01, 0x12, 0x03, 0x28, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0d,
    0x02, 0x12, 0x03, 0x28, 0x17, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0e, 0x12, 0x03,
    0x29, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x29, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x29, 0x1a, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x2a, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x0f, 0x02, 0x12, 0x03, 0x2a, 0x1d, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x10,
    0x12, 0x03, 0x2b, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x10, 0x01, 0x12, 0x03,
    0x2b, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x10, 0x02, 0x12, 0x03, 0x2b, 0x17,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x11, 0x12, 0x03, 0x2c, 0x08, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x11, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x11, 0x02, 0x12, 0x03, 0x2c, 0x13, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x12, 0x12, 0x03, 0x2d, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x12, 0x01,
    0x12, 0x03, 0x2d, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x12, 0x02, 0x12, 0x03,
    0x2d, 0x1b, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x13, 0x12, 0x03, 0x2e, 0x08, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x13, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x13, 0x02, 0x12, 0x03, 0x2e, 0x18, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x14, 0x12, 0x03, 0x2f, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x14, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x14, 0x02,
    0x12, 0x03, 0x2f, 0x17, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x15, 0x12, 0x03, 0x30,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x15, 0x01, 0x12, 0x03, 0x30, 0x08, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x15, 0x02, 0x12, 0x03, 0x30, 0x1d, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x16, 0x12, 0x03, 0x31, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x16, 0x01, 0x12, 0x03, 0x31, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x16, 0x02, 0x12, 0x03, 0x31, 0x19, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x17, 0x12,
    0x03, 0x32, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x17, 0x01, 0x12, 0x03, 0x32,
    0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x17, 0x02, 0x12, 0x03, 0x32, 0x1c, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x18, 0x12, 0x03, 0x33, 0x08, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x18, 0x01, 0x12, 0x03, 0x33, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x18, 0x02, 0x12, 0x03, 0x33, 0x1d, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12,
    0x04, 0x36, 0x00, 0x39, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x36, 0x05,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x03, 0x37, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x37, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x38, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x38, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x38, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x03, 0x12, 0x04, 0x3b, 0x00, 0x3f, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x03, 0x01, 0x12, 0x03, 0x3b, 0x05, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x3c, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x01, 0x02, 0x12, 0x03, 0x3d, 0x15, 0x16, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x03, 0x02, 0x02, 0x12, 0x03, 0x3e, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x3e, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x04, 0x12, 0x04, 0x41, 0x00,
    0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x04, 0x01, 0x12, 0x03, 0x41, 0x05, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x04, 0x02, 0x00, 0x12, 0x03, 0x42, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x42, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x05, 0x12, 0x04, 0x45,
    0x00, 0x48, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x05, 0x01, 0x12, 0x03, 0x45, 0x05, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x00, 0x12, 0x03, 0x46, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x46, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x05, 0x02, 0x01,
    0x12, 0x03, 0x47, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x47, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x05, 0x02, 0x01, 0x02, 0x12, 0x03, 0x47, 0x25,
    0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x06, 0x12, 0x04, 0x4a, 0x00, 0x4f, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x06, 0x01, 0x12, 0x03, 0x4a, 0x05, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x06, 0x02,
    0x00, 0x12, 0x03, 0x4b, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4b, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x00, 0x02, 0x12, 0x03, 0x4b,
    0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x06, 0x02, 0x01, 0x02, 0x12, 0x03, 0x4c, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x06, 0x02, 0x02, 0x12, 0x03, 0x4d, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x4d, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x4d, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x06, 0x02, 0x03, 0x12, 0x03, 0x4e, 0x08,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x06, 0x02, 0x03, 0x02, 0x12, 0x03, 0x4e, 0x2e, 0x2f, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x07, 0x12, 0x04, 0x51, 0x00, 0x57, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x07, 0x01,
    0x12, 0x03, 0x51, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x07, 0x02, 0x00, 0x12, 0x03, 0x52,
    0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x08, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x00, 0x02, 0x12, 0x03, 0x52, 0x15, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x07, 0x02, 0x01, 0x12, 0x03, 0x53, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x53, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x53, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x07, 0x02, 0x02, 0x12,
    0x03, 0x54, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x54,
    0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x02, 0x02, 0x12, 0x03, 0x54, 0x16, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x07, 0x02, 0x03, 0x12, 0x03, 0x55, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x55, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x07, 0x02, 0x03, 0x02, 0x12, 0x03, 0x55, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x07, 0x02,
    0x04, 0x12, 0x03, 0x56, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x56, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x07, 0x02, 0x04, 0x02, 0x12, 0x03, 0x56,
    0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x08, 0x12, 0x04, 0x59, 0x00, 0x5b, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x08, 0x01, 0x12, 0x03, 0x59, 0x05, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x08,
    0x02, 0x00, 0x12, 0x03, 0x5a, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x5a, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x08, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x5a, 0x14, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x09, 0x12, 0x04, 0x5d, 0x00, 0x61, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x09, 0x01, 0x12, 0x03, 0x5d, 0x05, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x09, 0x02, 0x00, 0x12, 0x03, 0x5e, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x5e, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x5e, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x09, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x08,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x01, 0x02, 0x12, 0x03, 0x5f, 0x1f, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x09, 0x02, 0x02, 0x12, 0x03, 0x60, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x60, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x09, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x60, 0x17, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x0a, 0x12, 0x04, 0x63, 0x00,
    0x66, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x0a, 0x01, 0x12, 0x03, 0x63, 0x05, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x64, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x0a, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x64, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x0a, 0x02, 0x01, 0x12,
    0x03, 0x65, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65,
    0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x0a, 0x02, 0x01, 0x02, 0x12, 0x03, 0x65, 0x19, 0x1c,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x68, 0x00, 0x6f, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x68, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x69, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x69, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x69, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x19, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x6a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x6a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x6a, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6a,
    0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x6b, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6b, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x6b, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x6c, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6c,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x6c, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6c, 0x16, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x6c, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x6d, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x6d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x6d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x6d, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x6d, 0x25,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x6e, 0x08, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x6e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x6e, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x6e, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x71, 0x00,
    0x75, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x71, 0x08, 0x14, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x72, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x72, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x72, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x72, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x72, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x73, 0x08, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x73, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x73, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x73, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x74, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x74, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x74, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x74, 0x17, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x74, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x77, 0x00, 0x7e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x77, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x78,
    0x08, 0x51, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x78, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x78, 0x11, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x78, 0x23, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x78, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x78, 0x2e, 0x50, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x07, 0x12, 0x03, 0x78, 0x39, 0x4f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x79, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x79, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x79, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x79, 0x17, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x79, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x7a, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x7a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x7a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7a,
    0x17, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7a, 0x28, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7b, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x7b, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x7b, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7c,
    0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x7c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x7c, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7c, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x7c, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x7d, 0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x7d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x7d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x7d, 0x18,
    0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x7d, 0x35, 0x36, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x06, 0x80, 0x01, 0x00, 0x84, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x04, 0x80, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x04, 0x81, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x81, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x81, 0x01, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x81, 0x01, 0x20, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x81, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01,
    0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x82, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x04, 0x82, 0x01, 0x11, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x01, 0x19, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x04, 0x82, 0x01, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x04, 0x83, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x04, 0x83, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x05, 0x12, 0x04, 0x83, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x83, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x83, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x06,
    0x86, 0x01, 0x00, 0x89, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x04, 0x86,
    0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x04, 0x87, 0x01, 0x08,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x04, 0x87, 0x01, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x01, 0x17, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x88, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x88, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x88, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x06, 0x8b,
    0x01, 0x00, 0x8d, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x04, 0x8b, 0x01,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x06, 0x8f, 0x01, 0x00, 0x94, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12,
    0x04, 0x90, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x90, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x04, 0x90,
    0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x04, 0x90, 0x01,
    0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x04, 0x90, 0x01, 0x20,
    0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x04, 0x91, 0x01, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x91, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x04, 0x91, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x04, 0x91, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x04, 0x91, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x02, 0x12, 0x04, 0x92, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x92, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x05, 0x12, 0x04, 0x92, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x92, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x92, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x04, 0x93,
    0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0x93, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x04, 0x93, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x04, 0x93, 0x01, 0x18, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x04, 0x93, 0x01, 0x20, 0x21, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0x96, 0x01, 0x00, 0x9c, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x00, 0x12, 0x04, 0x97, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x97, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x97, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x97, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x97, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0x98, 0x01,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x98, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x04, 0x98, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x04, 0x98, 0x01, 0x18, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x04, 0x98, 0x01, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x04, 0x99, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x02, 0x05, 0x12, 0x04, 0x99, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x99, 0x01, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x99, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03,
    0x12, 0x04, 0x9a, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x9a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x04,
    0x9a, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9a,
    0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9a, 0x01,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x04, 0x9b, 0x01, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x19, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x08, 0x12, 0x06, 0x9e, 0x01, 0x00, 0xa0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12,
    0x04, 0x9f, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x9f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9f,
    0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x01,
    0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x06, 0xa2, 0x01, 0x00, 0xa4, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xa3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa3, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa3, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0xa6, 0x01,
    0x00, 0xa9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa7, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xa8, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xa8, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xa8, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xab, 0x01, 0x00,
    0xad, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08, 0x25,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0xac, 0x01, 0x08, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xac, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xac, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xac, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xac, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c,
    0x12, 0x06, 0xaf, 0x01, 0x00, 0xb1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12,
    0x04, 0xaf, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xb0,
    0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb0, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x16, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0xb3, 0x01, 0x00, 0xb4, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e,
    0x12, 0x06, 0xb6, 0x01, 0x00, 0xb9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12,
    0x04, 0xb6, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xb7,
    0x01, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb7, 0x01, 0x11,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x20, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x2d, 0x2e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x08, 0x12, 0x04, 0xb7, 0x01, 0x2f, 0x43, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x07, 0x12, 0x04, 0xb7, 0x01, 0x3a, 0x42, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xb8, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xb8, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xbb,
    0x01, 0x00, 0xbd, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xbb, 0x01,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbc, 0x01, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x10, 0x12, 0x06, 0xbf, 0x01, 0x00, 0xd4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10,
    0x01, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12,
    0x04, 0xc0, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xc0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc0,
    0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc0, 0x01,
    0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc1, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x02, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xc2, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xc2, 0x01, 0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xc2, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0xc3,
    0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc3, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x16, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x20, 0x21, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x04, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x04, 0x05, 0x12, 0x04, 0xc4, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02,
    0x05, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x04,
    0x12, 0x04, 0xc5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x05, 0x12,
    0x04, 0xc5, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xc5, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x03, 0x12, 0x04, 0xc5,
    0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x06, 0x12, 0x04, 0xc6, 0x01, 0x08,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x19, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x10, 0x02, 0x07, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x07, 0x04, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x07, 0x05, 0x12, 0x04, 0xc7, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x07, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07,
    0x03, 0x12, 0x04, 0xc7, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x08, 0x12,
    0x04, 0xc8, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x08, 0x04, 0x12, 0x04,
    0xc8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x08, 0x05, 0x12, 0x04, 0xc8,
    0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x08, 0x01, 0x12, 0x04, 0xc8, 0x01,
    0x19, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x08, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x2c,
    0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x09, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x09, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x09, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x09, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x09, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x0a, 0x12, 0x04, 0xca, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x0a, 0x04, 0x12, 0x04, 0xca, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0a,
    0x05, 0x12, 0x04, 0xca, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0a, 0x01,
    0x12, 0x04, 0xca, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0a, 0x03, 0x12,
    0x04, 0xca, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x0b, 0x12, 0x04, 0xcb,
    0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xcb, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0b, 0x05, 0x12, 0x04, 0xcb, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x17, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xcb, 0x01, 0x25, 0x27, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x0c, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x0c, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x0c, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02,
    0x0d, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0d, 0x04,
    0x12, 0x04, 0xcd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0d, 0x05, 0x12,
    0x04, 0xcd, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0d, 0x01, 0x12, 0x04,
    0xcd, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xcd,
    0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x0e, 0x12, 0x04, 0xce, 0x01, 0x08,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xce, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0e, 0x05, 0x12, 0x04, 0xce, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xce, 0x01, 0x18, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xce, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x10, 0x02, 0x0f, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x0f, 0x04, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x0f, 0x05, 0x12, 0x04, 0xcf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x0f, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0f,
    0x03, 0x12, 0x04, 0xcf, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x10, 0x12,
    0x04, 0xd0, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x10, 0x04, 0x12, 0x04,
    0xd0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x10, 0x05, 0x12, 0x04, 0xd0,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x10, 0x01, 0x12, 0x04, 0xd0, 0x01,
    0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x10, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x24,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x11, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x11, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x11, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x11, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x11, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x12, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x12, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x12,
    0x06, 0x12, 0x04, 0xd2, 0x01, 0x11, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x12, 0x01,
    0x12, 0x04, 0xd2, 0x01, 0x32, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x12, 0x03, 0x12,
    0x04, 0xd2, 0x01, 0x48, 0x4a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x13, 0x12, 0x04, 0xd3,
    0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x13, 0x04, 0x12, 0x04, 0xd3, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x13, 0x05, 0x12, 0x04, 0xd3, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x13, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x17, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x13, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x2f, 0x31, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xd6, 0x01, 0x00, 0xdf, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x19, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x11,
    0x03, 0x00, 0x12, 0x06, 0xd7, 0x01, 0x08, 0xdb, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x03, 0x00, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x04, 0xd8, 0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd8, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x11, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x1f, 0x27, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x11, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x10, 0x34, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd9, 0x01, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd9, 0x01, 0x19, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x20, 0x2f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x32,
    0x33, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xda, 0x01, 0x10,
    0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xda, 0x01,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xda,
    0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xda, 0x01, 0x20, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xda, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xdd,
    0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdd, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdd, 0x01, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x16, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x29, 0x2a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xde, 0x01, 0x08, 0x37, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xde, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x06, 0x12, 0x04, 0xde, 0x01, 0x11, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xde, 0x01, 0x2b, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xde, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12,
    0x06, 0xe1, 0x01, 0x00, 0xe3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04,
    0xe1, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xe2, 0x01,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x16, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xe5, 0x01, 0x00, 0xe9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x13, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02,
    0x00, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xe6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xe6, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xe6, 0x01, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe6,
    0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x08,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe7, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x18, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x02, 0x05, 0x12, 0x04, 0xe8, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xe8, 0x01, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x08,
    0x12, 0x04, 0xe8, 0x01, 0x23, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x07, 0x12,
    0x04, 0xe8, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xeb, 0x01, 0x00,
    0xed, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xec, 0x01, 0x08, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xec, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xec, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xec, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xec, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15,
    0x12, 0x06, 0xef, 0x01, 0x00, 0x88, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12,
    0x04, 0xef, 0x01, 0x08, 0x16, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x15, 0x03, 0x00, 0x12, 0x06, 0xf0,
    0x01, 0x08, 0x84, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x03, 0x00, 0x01, 0x12, 0x04,
    0xf0, 0x01, 0x10, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04,
    0xf1, 0x01, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xf1, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xf1, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xf1, 0x01, 0x20, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xf2, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf2, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x20, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf2, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xf3, 0x01, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf3, 0x01, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x20, 0x28, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x2b, 0x2c,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x10, 0x2b,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf4, 0x01,
    0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf4,
    0x01, 0x20, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xf4, 0x01, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x04, 0x12, 0x04,
    0xf5, 0x01, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x04, 0xf5, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x04, 0xf5, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xf5, 0x01, 0x1f, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x04, 0xf5, 0x01, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x05, 0x12, 0x04, 0xf6, 0x01, 0x10, 0x33, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x05, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03,
    0x00, 0x02, 0x05, 0x05, 0x12, 0x04, 0xf6, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x1f, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf6, 0x01, 0x31, 0x32, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x06, 0x12, 0x04, 0xf7, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0xf7, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf7, 0x01, 0x19, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x1f, 0x2b, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x2e, 0x2f,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x07, 0x12, 0x04, 0xf8, 0x01, 0x10, 0x2b,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12, 0x04, 0xf8, 0x01,
    0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0xf8,
    0x01, 0x1f, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x04,
    0xf8, 0x01, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x08, 0x12, 0x04,
    0xf9, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x08, 0x04, 0x12,
    0x04, 0xf9, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x08, 0x05,
    0x12, 0x04, 0xf9, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x08,
    0x01, 0x12, 0x04, 0xf9, 0x01, 0x1f, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x09, 0x12, 0x04, 0xfa, 0x01, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x09, 0x04, 0x12, 0x04, 0xfa, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03,
    0x00, 0x02, 0x09, 0x05, 0x12, 0x04, 0xfa, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x03, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0xfa, 0x01, 0x1f, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x03, 0x00, 0x02, 0x09, 0x03, 0x12, 0x04, 0xfa, 0x01, 0x27, 0x29, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x0a, 0x12, 0x04, 0xfb, 0x01, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xfb, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x04, 0xfb, 0x01, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x20, 0x29, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x2c, 0x2e,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0b, 0x12, 0x04, 0xfc, 0x01, 0x10, 0x37,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x04, 0xfc, 0x01,
    0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xfc,
    0x01, 0x21, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x04,
    0xfc, 0x01, 0x34, 0x36, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0c, 0x12, 0x04,
    0xfd, 0x01, 0x10, 0x33, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0c, 0x04, 0x12,
    0x04, 0xfd, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0c, 0x05,
    0x12, 0x04, 0xfd, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0c,
    0x01, 0x12, 0x04, 0xfd, 0x01, 0x1f, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02,
    0x0c, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x30, 0x32, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x0d, 0x12, 0x04, 0xfe, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x0d, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03,
    0x00, 0x02, 0x0d, 0x05, 0x12, 0x04, 0xfe, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x03, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x1f, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x03, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xfe, 0x01, 0x2d, 0x2f, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x0e, 0x12, 0x04, 0xff, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xff, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0e, 0x05, 0x12, 0x04, 0xff, 0x01, 0x19, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xff, 0x01, 0x1f, 0x2a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xff, 0x01, 0x2d, 0x2f,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0f, 0x12, 0x04, 0x80, 0x02, 0x10, 0x2f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x80, 0x02, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0f, 0x05, 0x12, 0x04, 0x80, 0x02,
    0x19, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x80,
    0x02, 0x1e, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x04,
    0x80, 0x02, 0x2c, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x10, 0x12, 0x04,
    0x81, 0x02, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x10, 0x04, 0x12,
    0x04, 0x81, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x10, 0x05,
    0x12, 0x04, 0x81, 0x02, 0x19, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x10,
    0x01, 0x12, 0x04, 0x81, 0x02, 0x1e, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02,
    0x10, 0x03, 0x12, 0x04, 0x81, 0x02, 0x2b, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x11, 0x12, 0x04, 0x82, 0x02, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x11, 0x04, 0x12, 0x04, 0x82, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03,
    0x00, 0x02, 0x11, 0x05, 0x12, 0x04, 0x82, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x03, 0x00, 0x02, 0x11, 0x01, 0x12, 0x04, 0x82, 0x02, 0x20, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x03, 0x00, 0x02, 0x11, 0x03, 0x12, 0x04, 0x82, 0x02, 0x27, 0x29, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x12, 0x12, 0x04, 0x83, 0x02, 0x10, 0x38, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x12, 0x04, 0x12, 0x04, 0x83, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x12, 0x05, 0x12, 0x04, 0x83, 0x02, 0x19, 0x20, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x12, 0x01, 0x12, 0x04, 0x83, 0x02, 0x21, 0x32, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x12, 0x03, 0x12, 0x04, 0x83, 0x02, 0x35, 0x37,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0x86, 0x02, 0x08, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x02, 0x16, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x01, 0x12, 0x04, 0x87, 0x02, 0x08, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x87, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x06,
    0x12, 0x04, 0x87, 0x02, 0x11, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x87, 0x02, 0x2c, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x87, 0x02, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0x8a, 0x02, 0x00, 0x8d,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x18, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0x8b, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02,
    0x01, 0x12, 0x04, 0x8c, 0x02, 0x08, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x8c, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x06, 0x12,
    0x04, 0x8c, 0x02, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x8c, 0x02, 0x1e, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c,
    0x02, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x08, 0x12, 0x04, 0x8c, 0x02,
    0x30, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x07, 0x12, 0x04, 0x8c, 0x02, 0x3b,
    0x44, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0x8f, 0x02, 0x00, 0x92, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0x8f, 0x02, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0x90, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x90, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x90, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x90, 0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x90, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04,
    0x91, 0x02, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x04, 0x91,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x05, 0x12, 0x04, 0x91, 0x02,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04, 0x91, 0x02, 0x17,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0x91, 0x02, 0x1e, 0x1f,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0x94, 0x02, 0x00, 0x97, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0x94, 0x02, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x18, 0x02, 0x00, 0x12, 0x04, 0x95, 0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x95, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x95, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x95, 0x02, 0x16, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x95, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0x96,
    0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0x96, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x06, 0x12, 0x04, 0x96, 0x02, 0x11,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0x96, 0x02, 0x1c, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0x96, 0x02, 0x24, 0x25, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0x99, 0x02, 0x00, 0x9b, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0x99, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19,
    0x02, 0x00, 0x12, 0x04, 0x9a, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x9a, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x9a, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9a, 0x02, 0x1c, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9a, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0x9d, 0x02, 0x00, 0xa3,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x08, 0x18, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9e, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x02, 0x1c, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x9e, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02,
    0x01, 0x12, 0x04, 0x9f, 0x02, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x9f, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x9f, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x9f, 0x02, 0x17, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9f,
    0x02, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x02, 0x12, 0x04, 0xa0, 0x02, 0x08,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa0, 0x02, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x17, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa0, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1a, 0x02, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xa1, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xa1, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x04, 0x12,
    0x04, 0xa2, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xa2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x05, 0x12, 0x04, 0xa2,
    0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa2, 0x02,
    0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa2, 0x02, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xa5, 0x02, 0x00, 0xa9, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x02, 0x08, 0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa6, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xa6, 0x02, 0x11, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa6, 0x02, 0x29, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa6, 0x02, 0x30, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x08, 0x12,
    0x04, 0xa6, 0x02, 0x32, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x07, 0x12, 0x04,
    0xa6, 0x02, 0x3d, 0x54, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0xa7, 0x02,
    0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa7, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa7, 0x02, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x17, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa7, 0x02, 0x1e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0xa8, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa8, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06,
    0xab, 0x02, 0x00, 0xae, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xab,
    0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xac, 0x02, 0x08,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xac, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xac, 0x02, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xac, 0x02, 0x17, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xac, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1c, 0x02, 0x01, 0x12, 0x04, 0xad, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x01, 0x04, 0x12, 0x04, 0xad, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xad, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xad, 0x02, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xad, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xb0,
    0x02, 0x00, 0xb3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xb0, 0x02,
    0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb1, 0x02, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb1, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb1, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0xb2, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xb2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xb2, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xb2, 0x02, 0x17, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xb2, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xb5, 0x02,
    0x00, 0xc6, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xb5, 0x02, 0x08,
    0x19, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1e, 0x03, 0x00, 0x12, 0x06, 0xb6, 0x02, 0x08, 0xc0, 0x02,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x03, 0x00, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x10, 0x1a,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x02, 0x10, 0x28,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x02, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb7, 0x02,
    0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7,
    0x02, 0x1f, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xb7, 0x02, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04,
    0xb8, 0x02, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xb8, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xb8, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xb8, 0x02, 0x20, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb8, 0x02, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x02, 0x12, 0x04, 0xb9, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xb9, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb9, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x1f, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1e, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x03, 0x12, 0x04, 0xba, 0x02, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0xba, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x04, 0xba, 0x02, 0x19, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xba, 0x02, 0x1f, 0x27, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xba, 0x02, 0x2a, 0x2b,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x02, 0x10, 0x2c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0xbb, 0x02, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x04, 0xbb, 0x02,
    0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xbb,
    0x02, 0x20, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xbb, 0x02, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05, 0x12, 0x04,
    0xbc, 0x02, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xbc, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05, 0x05,
    0x12, 0x04, 0xbc, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xbc, 0x02, 0x1f, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02,
    0x05, 0x03, 0x12, 0x04, 0xbc, 0x02, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x06, 0x12, 0x04, 0xbd, 0x02, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x04, 0xbd, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03,
    0x00, 0x02, 0x06, 0x05, 0x12, 0x04, 0xbd, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e,
    0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x1f, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1e, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x07, 0x12, 0x04, 0xbe, 0x02, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1e, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12, 0x04, 0xbe, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12, 0x04, 0xbe, 0x02, 0x19, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x1f, 0x29, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x04, 0xbe, 0x02, 0x2c, 0x2d,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x12, 0x04, 0xbf, 0x02, 0x10, 0x2c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x04, 0x12, 0x04, 0xbf, 0x02, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x05, 0x12, 0x04, 0xbf, 0x02,
    0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x01, 0x12, 0x04, 0xbf,
    0x02, 0x1f, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x03, 0x00, 0x02, 0x08, 0x03, 0x12, 0x04,
    0xbf, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xc2, 0x02,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc2, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc2, 0x02, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x16, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc2, 0x02, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc3, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc3, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc3, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x02,
    0x12, 0x04, 0xc4, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xc4, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xc4, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc4,
    0x02, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc4, 0x02,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x03, 0x12, 0x04, 0xc5, 0x02, 0x08, 0x38,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc5, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x06, 0x12, 0x04, 0xc5, 0x02, 0x11, 0x2d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x2e, 0x33, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc5, 0x02, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1f, 0x12, 0x06, 0xc8, 0x02, 0x00, 0xd5, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f,
    0x01, 0x12, 0x04, 0xc8, 0x02, 0x08, 0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1f, 0x03, 0x00, 0x12,
    0x06, 0xc9, 0x02, 0x08, 0xcc, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x03, 0x00, 0x01,
    0x12, 0x04, 0xc9, 0x02, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x04, 0xca, 0x02, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xca, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xca, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xca, 0x02, 0x1f, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xca, 0x02, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f,
    0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xcb, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcb, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1f, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcb, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1f, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcb, 0x02, 0x20, 0x24, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1f, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcb, 0x02, 0x27, 0x28, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x1f, 0x03, 0x01, 0x12, 0x06, 0xce, 0x02, 0x08, 0xd2, 0x02, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x03, 0x01, 0x01, 0x12, 0x04, 0xce, 0x02, 0x10, 0x1c, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0xcf, 0x02, 0x10, 0x2b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcf, 0x02, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcf, 0x02, 0x19, 0x1e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcf, 0x02, 0x1f, 0x26,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcf, 0x02, 0x29,
    0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x01, 0x12, 0x04, 0xd0, 0x02, 0x10,
    0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd0, 0x02,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd0,
    0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xd0, 0x02, 0x20, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xd0, 0x02, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x02, 0x12,
    0x04, 0xd1, 0x02, 0x10, 0x3e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x04, 0xd1, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02, 0x02,
    0x06, 0x12, 0x04, 0xd1, 0x02, 0x19, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xd1, 0x02, 0x35, 0x39, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1f, 0x03, 0x01,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xd1, 0x02, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02,
    0x00, 0x12, 0x04, 0xd4, 0x02, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xd4, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xd4, 0x02, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xd4, 0x02, 0x34, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd4,
    0x02, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0xd7, 0x02, 0x00, 0xe2, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x1e, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0xd8, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd8, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xd8, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01,
    0x12, 0x04, 0xd9, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xd9, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xd9, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd9,
    0x02, 0x17, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd9, 0x02,
    0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x02, 0x12, 0x04, 0xda, 0x02, 0x08, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x04, 0x12, 0x04, 0xda, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x05, 0x12, 0x04, 0xda, 0x02, 0x11, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x01, 0x12, 0x04, 0xda, 0x02, 0x16, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x02, 0x03, 0x12, 0x04, 0xda, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x03, 0x12, 0x04, 0xdb, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xdb, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xdb, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xdb, 0x02, 0x16, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xdb, 0x02, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x04, 0x12, 0x04,
    0xdc, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x04, 0x12, 0x04, 0xdc,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x05, 0x12, 0x04, 0xdc, 0x02,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x17,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x03, 0x12, 0x04, 0xdc, 0x02, 0x22, 0x23,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x05, 0x12, 0x04, 0xdd, 0x02, 0x08, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x04, 0x12, 0x04, 0xdd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x05, 0x05, 0x12, 0x04, 0xdd, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x05, 0x01, 0x12, 0x04, 0xdd, 0x02, 0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x05, 0x03, 0x12, 0x04, 0xdd, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20,
    0x02, 0x06, 0x12, 0x04, 0xde, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x06,
    0x04, 0x12, 0x04, 0xde, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x06, 0x05,
    0x12, 0x04, 0xde, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xde, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x06, 0x03, 0x12, 0x04,
    0xde, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x07, 0x12, 0x04, 0xdf, 0x02,
    0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x07, 0x04, 0x12, 0x04, 0xdf, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x07, 0x05, 0x12, 0x04, 0xdf, 0x02, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x07, 0x01, 0x12, 0x04, 0xdf, 0x02, 0x16, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x07, 0x03, 0x12, 0x04, 0xdf, 0x02, 0x2b, 0x2c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x20, 0x02, 0x08, 0x12, 0x04, 0xe0, 0x02, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x08, 0x04, 0x12, 0x04, 0xe0, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x08, 0x05, 0x12, 0x04, 0xe0, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xe0, 0x02, 0x18, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xe0, 0x02, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x09,
    0x12, 0x04, 0xe1, 0x02, 0x08, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x09, 0x04, 0x12,
    0x04, 0xe1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x09, 0x05, 0x12, 0x04,
    0xe1, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x09, 0x01, 0x12, 0x04, 0xe1,
    0x02, 0x18, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x09, 0x03, 0x12, 0x04, 0xe1, 0x02,
    0x3a, 0x3c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xe4, 0x02, 0x00, 0xe8, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xe4, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0xe5, 0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x21, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe5, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xe5, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xe5, 0x02, 0x16, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xe5, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12,
    0x04, 0xe6, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xe6, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe6,
    0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe6, 0x02,
    0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe6, 0x02, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x02, 0x12, 0x04, 0xe7, 0x02, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe7, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe7, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x21, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x21, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe7, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x22, 0x12, 0x06, 0xea, 0x02, 0x00, 0xf4, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01,
    0x12, 0x04, 0xea, 0x02, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x00, 0x12, 0x04,
    0xeb, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x04, 0x12, 0x04, 0xeb,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x05, 0x12, 0x04, 0xeb, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12, 0x04, 0xeb, 0x02, 0x18,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04, 0xeb, 0x02, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01, 0x12, 0x04, 0xec, 0x02, 0x08, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12, 0x04, 0xec, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x22, 0x02, 0x01, 0x05, 0x12, 0x04, 0xec, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x22, 0x02, 0x01, 0x01, 0x12, 0x04, 0xec, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x22, 0x02, 0x01, 0x03, 0x12, 0x04, 0xec, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22,
    0x02, 0x02, 0x12, 0x04, 0xed, 0x02, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xed, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xed, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xed, 0x02, 0x16, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xed, 0x02, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x03, 0x12, 0x04, 0xee, 0x02,
    0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x03, 0x04, 0x12, 0x04, 0xee, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x03, 0x05, 0x12, 0x04, 0xee, 0x02, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x03, 0x01, 0x12, 0x04, 0xee, 0x02, 0x17, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x03, 0x03, 0x12, 0x04, 0xee, 0x02, 0x28, 0x29, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x22, 0x02, 0x04, 0x12, 0x04, 0xef, 0x02, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x22, 0x02, 0x04, 0x04, 0x12, 0x04, 0xef, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x22, 0x02, 0x04, 0x05, 0x12, 0x04, 0xef, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xef, 0x02, 0x17, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02,
    0x04, 0x03, 0x12, 0x04, 0xef, 0x02, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x05,
    0x12, 0x04, 0xf0, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xf0, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x05, 0x05, 0x12, 0x04,
    0xf0, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf0,
    0x02, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf0, 0x02,
    0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x06, 0x12, 0x04, 0xf1, 0x02, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x06, 0x04, 0x12, 0x04, 0xf1, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf1, 0x02, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x22, 0x02, 0x06, 0x01, 0x12, 0x04, 0xf1, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x22, 0x02, 0x06, 0x03, 0x12, 0x04, 0xf1, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x22, 0x02, 0x07, 0x12, 0x04, 0xf2, 0x02, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22,
    0x02, 0x07, 0x04, 0x12, 0x04, 0xf2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02,
    0x07, 0x05, 0x12, 0x04, 0xf2, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x07,
    0x01, 0x12, 0x04, 0xf2, 0x02, 0x17, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x07, 0x03,
    0x12, 0x04, 0xf2, 0x02, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x08, 0x12, 0x04,
    0xf3, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x08, 0x04, 0x12, 0x04, 0xf3,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x08, 0x05, 0x12, 0x04, 0xf3, 0x02,
    0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x08, 0x01, 0x12, 0x04, 0xf3, 0x02, 0x16,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x08, 0x03, 0x12, 0x04, 0xf3, 0x02, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0xf6, 0x02, 0x00, 0xfa, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xf6, 0x02, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x23, 0x02, 0x00, 0x12, 0x04, 0xf7, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xf7, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xf7, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xf7, 0x02, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xf7, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x01, 0x12, 0x04, 0xf8,
    0x02, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf8, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf8, 0x02, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf8, 0x02, 0x17, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf8, 0x02, 0x2d, 0x2e, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x02, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf9, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf9, 0x02, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xf9, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12,
    0x06, 0xfc, 0x02, 0x00, 0x83, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04,
    0xfc, 0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x00, 0x12, 0x04, 0xfd, 0x02,
    0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfd, 0x02, 0x11, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x02, 0x20, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x02, 0x28, 0x29, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x24, 0x02, 0x01, 0x12, 0x04, 0xfe, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x24, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfe, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x24, 0x02, 0x01, 0x05, 0x12, 0x04, 0xfe, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xfe, 0x02, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xfe, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x02,
    0x12, 0x04, 0xff, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xff, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xff, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x01, 0x12, 0x04, 0xff,
    0x02, 0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x02, 0x03, 0x12, 0x04, 0xff, 0x02,
    0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x03, 0x12, 0x04, 0x80, 0x03, 0x08, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x04, 0x12, 0x04, 0x80, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x05, 0x12, 0x04, 0x80, 0x03, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x24, 0x02, 0x03, 0x01, 0x12, 0x04, 0x80, 0x03, 0x19, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x24, 0x02, 0x03, 0x03, 0x12, 0x04, 0x80, 0x03, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x24, 0x02, 0x04, 0x12, 0x04, 0x81, 0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x81, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x81, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x81, 0x03, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x81, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x05, 0x12, 0x04,
    0x82, 0x03, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x05, 0x04, 0x12, 0x04, 0x82,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x05, 0x05, 0x12, 0x04, 0x82, 0x03,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x05, 0x01, 0x12, 0x04, 0x82, 0x03, 0x18,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x05, 0x03, 0x12, 0x04, 0x82, 0x03, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0x85, 0x03, 0x00, 0x89, 0x03, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x25, 0x01, 0x12, 0x04, 0x85, 0x03, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x25, 0x02, 0x00, 0x12, 0x04, 0x86, 0x03, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x86, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x86, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x86, 0x03, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x86, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x01, 0x12, 0x04, 0x87,
    0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x04, 0x12, 0x04, 0x87, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x05, 0x12, 0x04, 0x87, 0x03, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12, 0x04, 0x87, 0x03, 0x17, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04, 0x87, 0x03, 0x26, 0x27, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x02, 0x12, 0x04, 0x88, 0x03, 0x08, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x02, 0x04, 0x12, 0x04, 0x88, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x02, 0x05, 0x12, 0x04, 0x88, 0x03, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x02, 0x01, 0x12, 0x04, 0x88, 0x03, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x88, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12,
    0x06, 0x8b, 0x03, 0x00, 0x90, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04,
    0x8b, 0x03, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x03,
    0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8c, 0x03, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x03, 0x17, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c, 0x03, 0x1e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x26, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x03, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8d, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8d, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x8d, 0x03, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x8d, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x02,
    0x12, 0x04, 0x8e, 0x03, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x8e, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x05, 0x12, 0x04,
    0x8e, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8e,
    0x03, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8e, 0x03,
    0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x03, 0x12, 0x04, 0x8f, 0x03, 0x08, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8f, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8f, 0x03, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8f, 0x03, 0x17, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8f, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x27, 0x12, 0x06, 0x92, 0x03, 0x00, 0x97, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27,
    0x01, 0x12, 0x04, 0x92, 0x03, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12,
    0x04, 0x93, 0x03, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x93, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x05, 0x12, 0x04, 0x93,
    0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93, 0x03,
    0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x03, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01, 0x12, 0x04, 0x94, 0x03, 0x08, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x04, 0x94, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x05, 0x12, 0x04, 0x94, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x03, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x27, 0x02, 0x02, 0x12, 0x04, 0x95, 0x03, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x95, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02,
    0x05, 0x12, 0x04, 0x95, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x95, 0x03, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x95, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x03, 0x12, 0x04, 0x96,
    0x03, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03, 0x04, 0x12, 0x04, 0x96, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03, 0x05, 0x12, 0x04, 0x96, 0x03, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03, 0x01, 0x12, 0x04, 0x96, 0x03, 0x17, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03, 0x03, 0x12, 0x04, 0x96, 0x03, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0x99, 0x03, 0x00, 0x9b, 0x03, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0x99, 0x03, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28,
    0x02, 0x00, 0x12, 0x04, 0x9a, 0x03, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x9a, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x9a, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9a, 0x03, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9a, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x29, 0x12, 0x06, 0x9d, 0x03, 0x00, 0x9f,
    0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29, 0x01, 0x12, 0x04, 0x9d, 0x03, 0x08, 0x1c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x03, 0x08, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9e, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9e, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x03, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x9e, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2a, 0x12,
    0x06, 0xa1, 0x03, 0x00, 0xa4, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01, 0x12, 0x04,
    0xa1, 0x03, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x03,
    0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa2, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa2, 0x03, 0x11, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa2, 0x03, 0x19, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa2, 0x03, 0x29, 0x2a, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2a, 0x02, 0x01, 0x12, 0x04, 0xa3, 0x03, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa3, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2a, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa3, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xa3, 0x03, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xa3, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06,
    0xa6, 0x03, 0x00, 0xad, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04, 0xa6,
    0x03, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x00, 0x12, 0x04, 0xa7, 0x03, 0x08,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa7, 0x03, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa7, 0x03, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa7, 0x03, 0x17, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa7, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x2b, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x03, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa8, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xa8, 0x03, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xa8, 0x03, 0x19, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xa8, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x02, 0x12,
    0x04, 0xa9, 0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xa9, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x06, 0x12, 0x04, 0xa9,
    0x03, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa9, 0x03,
    0x20, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa9, 0x03, 0x26,
    0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x03, 0x12, 0x04, 0xaa, 0x03, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x04, 0x12, 0x04, 0xaa, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2b, 0x02, 0x03, 0x05, 0x12, 0x04, 0xaa, 0x03, 0x11, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xaa, 0x03, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2b, 0x02, 0x03, 0x03, 0x12, 0x04, 0xaa, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x2b, 0x02, 0x04, 0x12, 0x04, 0xab, 0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02,
    0x04, 0x04, 0x12, 0x04, 0xab, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x04,
    0x05, 0x12, 0x04, 0xab, 0x03, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x04, 0x01,
    0x12, 0x04, 0xab, 0x03, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x04, 0x03, 0x12,
    0x04, 0xab, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x05, 0x12, 0x04, 0xac,
    0x03, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x05, 0x04, 0x12, 0x04, 0xac, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x05, 0x05, 0x12, 0x04, 0xac, 0x03, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x05, 0x01, 0x12, 0x04, 0xac, 0x03, 0x16, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x05, 0x03, 0x12, 0x04, 0xac, 0x03, 0x2d, 0x2e, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xaf, 0x03, 0x00, 0xb1, 0x03, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0xaf, 0x03, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c,
    0x02, 0x00, 0x12, 0x04, 0xb0, 0x03, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xb0, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xb0, 0x03, 0x11, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xb0, 0x03, 0x20, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xb0, 0x03, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2d, 0x12, 0x06, 0xb3, 0x03, 0x00, 0xb6,
    0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2d, 0x01, 0x12, 0x04, 0xb3, 0x03, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x00, 0x12, 0x04, 0xb4, 0x03, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb4, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb4, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb4, 0x03, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xb4, 0x03, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02,
    0x01, 0x12, 0x04, 0xb5, 0x03, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xb5, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xb5, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xb5, 0x03, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb5,
    0x03, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2e, 0x12, 0x06, 0xb8, 0x03, 0x00, 0xc3, 0x03,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12, 0x04, 0xb8, 0x03, 0x08, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xb9, 0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb9, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb9, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xb9, 0x03, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xb9, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x01,
    0x12, 0x04, 0xba, 0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xba, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xba, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xba,
    0x03, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xba, 0x03,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x02, 0x12, 0x04, 0xbb, 0x03, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbb, 0x03, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbb, 0x03, 0x17, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2e, 0x02, 0x03, 0x12, 0x04, 0xbc, 0x03, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xbc, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xbc, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xbc, 0x03, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xbc, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x04, 0x12, 0x04,
    0xbd, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x04, 0x04, 0x12, 0x04, 0xbd,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x04, 0x05, 0x12, 0x04, 0xbd, 0x03,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x04, 0x01, 0x12, 0x04, 0xbd, 0x03, 0x17,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x04, 0x03, 0x12, 0x04, 0xbd, 0x03, 0x24, 0x25,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x05, 0x12, 0x04, 0xbe, 0x03, 0x08, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x05, 0x04, 0x12, 0x04, 0xbe, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2e, 0x02, 0x05, 0x05, 0x12, 0x04, 0xbe, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2e, 0x02, 0x05, 0x01, 0x12, 0x04, 0xbe, 0x03, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2e, 0x02, 0x05, 0x03, 0x12, 0x04, 0xbe, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e,
    0x02, 0x06, 0x12, 0x04, 0xbf, 0x03, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x06,
    0x04, 0x12, 0x04, 0xbf, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x06, 0x05,
    0x12, 0x04, 0xbf, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xbf, 0x03, 0x17, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x06, 0x03, 0x12, 0x04,
    0xbf, 0x03, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x07, 0x12, 0x04, 0xc0, 0x03,
    0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x07, 0x04, 0x12, 0x04, 0xc0, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x07, 0x05, 0x12, 0x04, 0xc0, 0x03, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x07, 0x01, 0x12, 0x04, 0xc0, 0x03, 0x17, 0x2f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x07, 0x03, 0x12, 0x04, 0xc0, 0x03, 0x32, 0x33, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2e, 0x02, 0x08, 0x12, 0x04, 0xc1, 0x03, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2e, 0x02, 0x08, 0x04, 0x12, 0x04, 0xc1, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2e, 0x02, 0x08, 0x05, 0x12, 0x04, 0xc1, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xc1, 0x03, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xc1, 0x03, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x09,
    0x12, 0x04, 0xc2, 0x03, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x09, 0x04, 0x12,
    0x04, 0xc2, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x09, 0x05, 0x12, 0x04,
    0xc2, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x09, 0x01, 0x12, 0x04, 0xc2,
    0x03, 0x17, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x09, 0x03, 0x12, 0x04, 0xc2, 0x03,
    0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2f, 0x12, 0x06, 0xc5, 0x03, 0x00, 0xc9, 0x03, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2f, 0x01, 0x12, 0x04, 0xc5, 0x03, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x2f, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x03, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xc6, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc6, 0x03, 0x17, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xc6, 0x03, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02, 0x01, 0x12,
    0x04, 0xc7, 0x03, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xc7, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc7,
    0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc7, 0x03,
    0x17, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc7, 0x03, 0x2c,
    0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02, 0x02, 0x12, 0x04, 0xc8, 0x03, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc8, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2f, 0x02, 0x02, 0x05, 0x12, 0x04, 0xc8, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x03, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2f, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc8, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x30, 0x12, 0x06, 0xcb, 0x03, 0x00, 0xcf, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x30, 0x01,
    0x12, 0x04, 0xcb, 0x03, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x00, 0x12, 0x04,
    0xcc, 0x03, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcc,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x06, 0x12, 0x04, 0xcc, 0x03,
    0x11, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcc, 0x03, 0x2c,
    0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcc, 0x03, 0x3a, 0x3b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x01, 0x12, 0x04, 0xcd, 0x03, 0x08, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcd, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x30, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcd, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x30, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcd, 0x03, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x30, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcd, 0x03, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30,
    0x02, 0x02, 0x12, 0x04, 0xce, 0x03, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xce, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x06,
    0x12, 0x04, 0xce, 0x03, 0x11, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xce, 0x03, 0x31, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xce, 0x03, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x31, 0x12, 0x06, 0xd1, 0x03, 0x00, 0xd3,
    0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x31, 0x01, 0x12, 0x04, 0xd1, 0x03, 0x08, 0x19, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x00, 0x12, 0x04, 0xd2, 0x03, 0x08, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x31, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd2, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd2, 0x03, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd2, 0x03, 0x19, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xd2, 0x03, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x32, 0x12,
    0x06, 0xd5, 0x03, 0x00, 0xda, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x32, 0x01, 0x12, 0x04,
    0xd5, 0x03, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x03,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd6, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd6, 0x03, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x03, 0x16, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd6, 0x03, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x32, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x03, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x32, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd7, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x32, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd7, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xd7, 0x03, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xd7, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x02,
    0x12, 0x04, 0xd8, 0x03, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xd8, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xd8, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd8,
    0x03, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd8, 0x03,
    0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x03, 0x12, 0x04, 0xd9, 0x03, 0x08, 0x2d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd9, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x05, 0x12, 0x04, 0xd9, 0x03, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd9, 0x03, 0x18, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x32, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd9, 0x03, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x33, 0x12, 0x06, 0xdc, 0x03, 0x00, 0xde, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x33,
    0x01, 0x12, 0x04, 0xdc, 0x03, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x00, 0x12,
    0x04, 0xdd, 0x03, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xdd, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdd,
    0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdd, 0x03,
    0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdd, 0x03, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x34, 0x12, 0x06, 0xe0, 0x03, 0x00, 0xe2, 0x03, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x34, 0x01, 0x12, 0x04, 0xe0, 0x03, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x34, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x03, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xe1, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xe1, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xe1, 0x03, 0x17, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xe1, 0x03, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x35, 0x12, 0x06, 0xe4, 0x03,
    0x00, 0xf0, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x35, 0x01, 0x12, 0x04, 0xe4, 0x03, 0x08,
    0x1b, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x35, 0x04, 0x00, 0x12, 0x06, 0xe5, 0x03, 0x08, 0xeb, 0x03,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x04, 0x00, 0x01, 0x12, 0x04, 0xe5, 0x03, 0x0d, 0x18,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xe6, 0x03, 0x10, 0x26,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6, 0x03, 0x10,
    0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xe6, 0x03,
    0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x03,
    0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe7,
    0x03, 0x10, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04,
    0xe7, 0x03, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04,
    0xe8, 0x03, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xe8, 0x03, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x04, 0xe8, 0x03, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x04, 0xe9, 0x03, 0x10, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xe9, 0x03, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00, 0x02,
    0x03, 0x02, 0x12, 0x04, 0xe9, 0x03, 0x23, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x04, 0xea, 0x03, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xea, 0x03, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x04,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xea, 0x03, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35,
    0x02, 0x00, 0x12, 0x04, 0xed, 0x03, 0x08, 0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xed, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xed, 0x03, 0x11, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xed, 0x03, 0x31, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xed, 0x03, 0x3d, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x08, 0x12, 0x04, 0xed,
    0x03, 0x3f, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x07, 0x12, 0x04, 0xed, 0x03,
    0x4a, 0x5b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x01, 0x12, 0x04, 0xee, 0x03, 0x08, 0x44,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x01, 0x06, 0x12, 0x04, 0xee, 0x03, 0x11, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x35, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee, 0x03, 0x28, 0x3f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x35, 0x02, 0x01, 0x03, 0x12, 0x04, 0xee, 0x03, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x35, 0x02, 0x02, 0x12, 0x04, 0xef, 0x03, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xef, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xef, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xef, 0x03, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xef, 0x03, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x36, 0x12, 0x06, 0xf2, 0x03,
    0x00, 0xf6, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x36, 0x01, 0x12, 0x04, 0xf2, 0x03, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x00, 0x12, 0x04, 0xf3, 0x03, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf3, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf3, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x36, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf3, 0x03, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x36, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf3, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x36, 0x02, 0x01, 0x12, 0x04, 0xf4, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xf4, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xf4, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xf4, 0x03, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xf4, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x02, 0x12, 0x04, 0xf5,
    0x03, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf5, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf5, 0x03, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf5, 0x03, 0x16, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf5, 0x03, 0x2a, 0x2b, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x37, 0x12, 0x06, 0xf8, 0x03, 0x00, 0x83, 0x04, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x37, 0x01, 0x12, 0x04, 0xf8, 0x03, 0x08, 0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x37,
    0x03, 0x00, 0x12, 0x06, 0xf9, 0x03, 0x08, 0xfd, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37,
    0x03, 0x00, 0x01, 0x12, 0x04, 0xf9, 0x03, 0x10, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x37, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x03, 0x10, 0x43, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfa, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37,
    0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfa, 0x03, 0x19, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x37, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfa, 0x03, 0x30, 0x3e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x37, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfa, 0x03, 0x41, 0x42, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x37, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xfb, 0x03, 0x10, 0x2e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x37, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfb, 0x03, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x37, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xfb, 0x03, 0x19, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x37, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfb, 0x03, 0x20, 0x29,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfb, 0x03, 0x2c,
    0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x37, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xfc, 0x03, 0x10,
    0x37, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xfc, 0x03,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xfc,
    0x03, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xfc, 0x03, 0x1f, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x37, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xfc, 0x03, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x00, 0x12, 0x04, 0xff,
    0x03, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x04, 0x12, 0x04, 0xff, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x05, 0x12, 0x04, 0xff, 0x03, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x01, 0x12, 0x04, 0xff, 0x03, 0x18, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x03, 0x12, 0x04, 0xff, 0x03, 0x27, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x01, 0x12, 0x04, 0x80, 0x04, 0x08, 0x3b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x37, 0x02, 0x01, 0x04, 0x12, 0x04, 0x80, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x37, 0x02, 0x01, 0x06, 0x12, 0x04, 0x80, 0x04, 0x11, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x37, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x04, 0x28, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x04, 0x39, 0x3a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02,
    0x02, 0x12, 0x04, 0x81, 0x04, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x81, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x06, 0x12,
    0x04, 0x81, 0x04, 0x11, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x81, 0x04, 0x21, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x02, 0x03, 0x12, 0x04, 0x81,
    0x04, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x03, 0x12, 0x04, 0x82, 0x04, 0x08,
    0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x03, 0x04, 0x12, 0x04, 0x82, 0x04, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x03, 0x06, 0x12, 0x04, 0x82, 0x04, 0x11, 0x41, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x03, 0x01, 0x12, 0x04, 0x82, 0x04, 0x42, 0x4f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x37, 0x02, 0x03, 0x03, 0x12, 0x04, 0x82, 0x04, 0x52, 0x53, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x38, 0x12, 0x06, 0x85, 0x04, 0x00, 0xad, 0x04, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x38, 0x01, 0x12, 0x04, 0x85, 0x04, 0x08, 0x1e, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x38, 0x03, 0x00,
    0x12, 0x06, 0x86, 0x04, 0x08, 0x89, 0x04, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x03, 0x00,
    0x01, 0x12, 0x04, 0x86, 0x04, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x38, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x87, 0x04, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x87, 0x04, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x87, 0x04, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x04, 0x1f, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x04, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x38, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x88, 0x04, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x38, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x04, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x38, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x88, 0x04, 0x19, 0x1f, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x38, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x88, 0x04, 0x20, 0x24, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x38, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x88, 0x04, 0x27, 0x28, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x38, 0x03, 0x01, 0x12, 0x06, 0x8b, 0x04, 0x08, 0x92, 0x04, 0x09, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x03, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x04, 0x10, 0x16, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x38, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x04, 0x10, 0x2c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x04, 0x10, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8c, 0x04, 0x19, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x04, 0x20,
    0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c, 0x04,
    0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x38, 0x03, 0x01, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x04,
    0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8d,
    0x04, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x04,
    0x8d, 0x04, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x8d, 0x04, 0x20, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x8d, 0x04, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x38, 0x03, 0x01, 0x02, 0x02,
    0x12, 0x04, 0x8e, 0x04, 0x10, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x8e, 0x04, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02,
    0x02, 0x05, 0x12, 0x04, 0x8e, 0x04, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x8e, 0x04, 0x1f, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03,
    0x01, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8e, 0x04, 0x30, 0x31, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x38,
    0x03, 0x01, 0x02, 0x03, 0x12, 0x04, 0x8f, 0x04, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38,
    0x03, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8f, 0x04, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x38, 0x03, 0x01, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8f, 0x04, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x38, 0x03, 0x01, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8f, 0x04, 0x1f, 0x2a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8f, 0x04, 0x2d, 0x2e, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x38, 0x03, 0x01, 0x02, 0x04, 0x12, 0x04, 0x90, 0x04, 0x10, 0x33, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x04, 0x04, 0x12, 0x04, 0x90, 0x04, 0x10, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x04, 0x05, 0x12, 0x04, 0x90, 0x04, 0x19, 0x1e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x04, 0x01, 0x12, 0x04, 0x90, 0x04, 0x1f,
    0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x04, 0x03, 0x12, 0x04, 0x90, 0x04,
    0x31, 0x32, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x38, 0x03, 0x01, 0x02, 0x05, 0x12, 0x04, 0x91, 0x04,
    0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x05, 0x04, 0x12, 0x04, 0x91,
    0x04, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x05, 0x05, 0x12, 0x04,
    0x91, 0x04, 0x19, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x91, 0x04, 0x1e, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x38, 0x03, 0x01, 0x02, 0x05, 0x03,
    0x12, 0x04, 0x91, 0x04, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x00, 0x12, 0x04,
    0x94, 0x04, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94,
    0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x05, 0x12, 0x04, 0x94, 0x04,
    0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x04, 0x16,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x04, 0x22, 0x23,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x01, 0x12, 0x04, 0x95, 0x04, 0x08, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x01, 0x04, 0x12, 0x04, 0x95, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x01, 0x05, 0x12, 0x04, 0x95, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x01, 0x01, 0x12, 0x04, 0x95, 0x04, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x01, 0x03, 0x12, 0x04, 0x95, 0x04, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38,
    0x02, 0x02, 0x12, 0x04, 0x96, 0x04, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x96, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x05,
    0x12, 0x04, 0x96, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x96, 0x04, 0x17, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x96, 0x04, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x03, 0x12, 0x04, 0x97, 0x04,
    0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x03, 0x04, 0x12, 0x04, 0x97, 0x04, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x03, 0x05, 0x12, 0x04, 0x97, 0x04, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x03, 0x01, 0x12, 0x04, 0x97, 0x04, 0x17, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x03, 0x03, 0x12, 0x04, 0x97, 0x04, 0x28, 0x29, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x38, 0x02, 0x04, 0x12, 0x04, 0x98, 0x04, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x04, 0x04, 0x12, 0x04, 0x98, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x04, 0x05, 0x12, 0x04, 0x98, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x98, 0x04, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x98, 0x04, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x05,
    0x12, 0x04, 0x99, 0x04, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x05, 0x04, 0x12,
    0x04, 0x99, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x05, 0x05, 0x12, 0x04,
    0x99, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x05, 0x01, 0x12, 0x04, 0x99,
    0x04, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x05, 0x03, 0x12, 0x04, 0x99, 0x04,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x06, 0x12, 0x04, 0x9a, 0x04, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x06, 0x04, 0x12, 0x04, 0x9a, 0x04, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x06, 0x05, 0x12, 0x04, 0x9a, 0x04, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x06, 0x01, 0x12, 0x04, 0x9a, 0x04, 0x17, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x06, 0x03, 0x12, 0x04, 0x9a, 0x04, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x38, 0x02, 0x07, 0x12, 0x04, 0x9b, 0x04, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x07, 0x04, 0x12, 0x04, 0x9b, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x07, 0x05, 0x12, 0x04, 0x9b, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x07,
    0x01, 0x12, 0x04, 0x9b, 0x04, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x07, 0x03,
    0x12, 0x04, 0x9b, 0x04, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x08, 0x12, 0x04,
    0x9c, 0x04, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x08, 0x04, 0x12, 0x04, 0x9c,
    0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x08, 0x05, 0x12, 0x04, 0x9c, 0x04,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x08, 0x01, 0x12, 0x04, 0x9c, 0x04, 0x17,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x08, 0x03, 0x12, 0x04, 0x9c, 0x04, 0x28, 0x2a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x09, 0x12, 0x04, 0x9d, 0x04, 0x08, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x09, 0x04, 0x12, 0x04, 0x9d, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x09, 0x05, 0x12, 0x04, 0x9d, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x09, 0x01, 0x12, 0x04, 0x9d, 0x04, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x09, 0x03, 0x12, 0x04, 0x9d, 0x04, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38,
    0x02, 0x0a, 0x12, 0x04, 0x9e, 0x04, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0a,
    0x04, 0x12, 0x04, 0x9e, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0a, 0x05,
    0x12, 0x04, 0x9e, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0a, 0x01, 0x12,
    0x04, 0x9e, 0x04, 0x17, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0a, 0x03, 0x12, 0x04,
    0x9e, 0x04, 0x1d, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x0b, 0x12, 0x04, 0x9f, 0x04,
    0x08, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x9f, 0x04, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0b, 0x06, 0x12, 0x04, 0x9f, 0x04, 0x11, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x9f, 0x04, 0x2d, 0x32, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x9f, 0x04, 0x35, 0x37, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x38, 0x02, 0x0c, 0x12, 0x04, 0xa0, 0x04, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xa0, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x0c, 0x05, 0x12, 0x04, 0xa0, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x0c, 0x01, 0x12, 0x04, 0xa0, 0x04, 0x17, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x0c, 0x03, 0x12, 0x04, 0xa0, 0x04, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x0d,
    0x12, 0x04, 0xa1, 0x04, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0d, 0x04, 0x12,
    0x04, 0xa1, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0d, 0x05, 0x12, 0x04,
    0xa1, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xa1,
    0x04, 0x17, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xa1, 0x04,
    0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x0e, 0x12, 0x04, 0xa2, 0x04, 0x08, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xa2, 0x04, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0e, 0x05, 0x12, 0x04, 0xa2, 0x04, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xa2, 0x04, 0x17, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xa2, 0x04, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x38, 0x02, 0x0f, 0x12, 0x04, 0xa3, 0x04, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x0f, 0x04, 0x12, 0x04, 0xa3, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x0f, 0x05, 0x12, 0x04, 0xa3, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0f,
    0x01, 0x12, 0x04, 0xa3, 0x04, 0x17, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x0f, 0x03,
    0x12, 0x04, 0xa3, 0x04, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x10, 0x12, 0x04,
    0xa4, 0x04, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x10, 0x04, 0x12, 0x04, 0xa4,
    0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x10, 0x05, 0x12, 0x04, 0xa4, 0x04,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x10, 0x01, 0x12, 0x04, 0xa4, 0x04, 0x17,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x10, 0x03, 0x12, 0x04, 0xa4, 0x04, 0x26, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x11, 0x12, 0x04, 0xa5, 0x04, 0x08, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x11, 0x04, 0x12, 0x04, 0xa5, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x11, 0x05, 0x12, 0x04, 0xa5, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x11, 0x01, 0x12, 0x04, 0xa5, 0x04, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x11, 0x03, 0x12, 0x04, 0xa5, 0x04, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38,
    0x02, 0x12, 0x12, 0x04, 0xa6, 0x04, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x12,
    0x04, 0x12, 0x04, 0xa6, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x12, 0x05,
    0x12, 0x04, 0xa6, 0x04, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x12, 0x01, 0x12,
    0x04, 0xa6, 0x04, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x12, 0x03, 0x12, 0x04,
    0xa6, 0x04, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x13, 0x12, 0x04, 0xa7, 0x04,
    0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x13, 0x04, 0x12, 0x04, 0xa7, 0x04, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x13, 0x05, 0x12, 0x04, 0xa7, 0x04, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x13, 0x01, 0x12, 0x04, 0xa7, 0x04, 0x17, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x13, 0x03, 0x12, 0x04, 0xa7, 0x04, 0x25, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x38, 0x02, 0x14, 0x12, 0x04, 0xa8, 0x04, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x38, 0x02, 0x14, 0x04, 0x12, 0x04, 0xa8, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x38, 0x02, 0x14, 0x05, 0x12, 0x04, 0xa8, 0x04, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x14, 0x01, 0x12, 0x04, 0xa8, 0x04, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x14, 0x03, 0x12, 0x04, 0xa8, 0x04, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x15,
    0x12, 0x04, 0xa9, 0x04, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x15, 0x04, 0x12,
    0x04, 0xa9, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x15, 0x05, 0x12, 0x04,
    0xa9, 0x04, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x15, 0x01, 0x12, 0x04, 0xa9,
    0x04, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x15, 0x03, 0x12, 0x04, 0xa9, 0x04,
    0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x16, 0x12, 0x04, 0xaa, 0x04, 0x08, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x16, 0x04, 0x12, 0x04, 0xaa, 0x04, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x16, 0x05, 0x12, 0x04, 0xaa, 0x04, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x38, 0x02, 0x16, 0x01, 0x12, 0x04, 0xaa, 0x04, 0x18, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x38, 0x02, 0x16, 0x03, 0x12, 0x04, 0xaa, 0x04, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x38, 0x02, 0x17, 0x12, 0x04, 0xab, 0x04, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38,
    0x02, 0x17, 0x04, 0x12, 0x04, 0xab, 0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02,
    0x17, 0x05, 0x12, 0x04, 0xab, 0x04, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x17,
    0x01, 0x12, 0x04, 0xab, 0x04, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x17, 0x03,
    0x12, 0x04, 0xab, 0x04, 0x2c, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x38, 0x02, 0x18, 0x12, 0x04,
    0xac, 0x04, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x18, 0x04, 0x12, 0x04, 0xac,
    0x04, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x18, 0x06, 0x12, 0x04, 0xac, 0x04,
    0x11, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x18, 0x01, 0x12, 0x04, 0xac, 0x04, 0x2f,
    0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x38, 0x02, 0x18, 0x03, 0x12, 0x04, 0xac, 0x04, 0x39, 0x3b,
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
