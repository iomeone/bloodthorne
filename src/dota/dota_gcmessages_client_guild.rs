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
pub struct CMsgDOTAGuildSDO {
    // message fields
    guild_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    tag: ::protobuf::SingularField<::std::string::String>,
    time_created: ::std::option::Option<u32>,
    time_disbanded: ::std::option::Option<u32>,
    logo: ::std::option::Option<u64>,
    base_logo: ::std::option::Option<u64>,
    banner_logo: ::std::option::Option<u64>,
    members: ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Member>,
    invitations: ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Invitation>,
    message: ::protobuf::SingularField<::std::string::String>,
    incremental: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildSDO {}

impl CMsgDOTAGuildSDO {
    pub fn new() -> CMsgDOTAGuildSDO {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildSDO {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildSDO> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildSDO,
        };
        unsafe {
            instance.get(CMsgDOTAGuildSDO::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
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

    // optional string tag = 3;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::string::String) {
        self.tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::string::String {
        if self.tag.is_none() {
            self.tag.set_default();
        };
        self.tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::string::String {
        self.tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_tag(&self) -> &str {
        match self.tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.tag
    }

    // optional uint32 time_created = 4;

    pub fn clear_time_created(&mut self) {
        self.time_created = ::std::option::Option::None;
    }

    pub fn has_time_created(&self) -> bool {
        self.time_created.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_created(&mut self, v: u32) {
        self.time_created = ::std::option::Option::Some(v);
    }

    pub fn get_time_created(&self) -> u32 {
        self.time_created.unwrap_or(0)
    }

    fn get_time_created_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_created
    }

    fn mut_time_created_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_created
    }

    // optional uint32 time_disbanded = 5;

    pub fn clear_time_disbanded(&mut self) {
        self.time_disbanded = ::std::option::Option::None;
    }

    pub fn has_time_disbanded(&self) -> bool {
        self.time_disbanded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_disbanded(&mut self, v: u32) {
        self.time_disbanded = ::std::option::Option::Some(v);
    }

    pub fn get_time_disbanded(&self) -> u32 {
        self.time_disbanded.unwrap_or(0)
    }

    fn get_time_disbanded_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_disbanded
    }

    fn mut_time_disbanded_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_disbanded
    }

    // optional uint64 logo = 6;

    pub fn clear_logo(&mut self) {
        self.logo = ::std::option::Option::None;
    }

    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo(&mut self, v: u64) {
        self.logo = ::std::option::Option::Some(v);
    }

    pub fn get_logo(&self) -> u64 {
        self.logo.unwrap_or(0)
    }

    fn get_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.logo
    }

    fn mut_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.logo
    }

    // optional uint64 base_logo = 7;

    pub fn clear_base_logo(&mut self) {
        self.base_logo = ::std::option::Option::None;
    }

    pub fn has_base_logo(&self) -> bool {
        self.base_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_logo(&mut self, v: u64) {
        self.base_logo = ::std::option::Option::Some(v);
    }

    pub fn get_base_logo(&self) -> u64 {
        self.base_logo.unwrap_or(0)
    }

    fn get_base_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.base_logo
    }

    fn mut_base_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.base_logo
    }

    // optional uint64 banner_logo = 8;

    pub fn clear_banner_logo(&mut self) {
        self.banner_logo = ::std::option::Option::None;
    }

    pub fn has_banner_logo(&self) -> bool {
        self.banner_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_banner_logo(&mut self, v: u64) {
        self.banner_logo = ::std::option::Option::Some(v);
    }

    pub fn get_banner_logo(&self) -> u64 {
        self.banner_logo.unwrap_or(0)
    }

    fn get_banner_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.banner_logo
    }

    fn mut_banner_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.banner_logo
    }

    // repeated .dota.CMsgDOTAGuildSDO.Member members = 9;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Member>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Member> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Member> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CMsgDOTAGuildSDO_Member] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAGuildSDO_Member> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Member> {
        &mut self.members
    }

    // repeated .dota.CMsgDOTAGuildSDO.Invitation invitations = 10;

    pub fn clear_invitations(&mut self) {
        self.invitations.clear();
    }

    // Param is passed by value, moved
    pub fn set_invitations(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Invitation>) {
        self.invitations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invitations(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Invitation> {
        &mut self.invitations
    }

    // Take field
    pub fn take_invitations(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Invitation> {
        ::std::mem::replace(&mut self.invitations, ::protobuf::RepeatedField::new())
    }

    pub fn get_invitations(&self) -> &[CMsgDOTAGuildSDO_Invitation] {
        &self.invitations
    }

    fn get_invitations_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAGuildSDO_Invitation> {
        &self.invitations
    }

    fn mut_invitations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAGuildSDO_Invitation> {
        &mut self.invitations
    }

    // optional string message = 11;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // optional bool incremental = 12;

    pub fn clear_incremental(&mut self) {
        self.incremental = ::std::option::Option::None;
    }

    pub fn has_incremental(&self) -> bool {
        self.incremental.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incremental(&mut self, v: bool) {
        self.incremental = ::std::option::Option::Some(v);
    }

    pub fn get_incremental(&self) -> bool {
        self.incremental.unwrap_or(false)
    }

    fn get_incremental_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.incremental
    }

    fn mut_incremental_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.incremental
    }
}

impl ::protobuf::Message for CMsgDOTAGuildSDO {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_created = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_disbanded = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.logo = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.base_logo = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.banner_logo = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invitations)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.incremental = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.time_created {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.time_disbanded {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.base_logo {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.banner_logo {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.invitations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        };
        if let Some(v) = self.incremental {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.tag.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.time_created {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.time_disbanded {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.logo {
            os.write_uint64(6, v)?;
        };
        if let Some(v) = self.base_logo {
            os.write_uint64(7, v)?;
        };
        if let Some(v) = self.banner_logo {
            os.write_uint64(8, v)?;
        };
        for v in &self.members {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.invitations {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.message.as_ref() {
            os.write_string(11, &v)?;
        };
        if let Some(v) = self.incremental {
            os.write_bool(12, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildSDO {
    fn new() -> CMsgDOTAGuildSDO {
        CMsgDOTAGuildSDO::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildSDO>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildSDO::get_guild_id_for_reflect,
                    CMsgDOTAGuildSDO::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgDOTAGuildSDO::get_name_for_reflect,
                    CMsgDOTAGuildSDO::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    CMsgDOTAGuildSDO::get_tag_for_reflect,
                    CMsgDOTAGuildSDO::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_created",
                    CMsgDOTAGuildSDO::get_time_created_for_reflect,
                    CMsgDOTAGuildSDO::mut_time_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_disbanded",
                    CMsgDOTAGuildSDO::get_time_disbanded_for_reflect,
                    CMsgDOTAGuildSDO::mut_time_disbanded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTAGuildSDO::get_logo_for_reflect,
                    CMsgDOTAGuildSDO::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "base_logo",
                    CMsgDOTAGuildSDO::get_base_logo_for_reflect,
                    CMsgDOTAGuildSDO::mut_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "banner_logo",
                    CMsgDOTAGuildSDO::get_banner_logo_for_reflect,
                    CMsgDOTAGuildSDO::mut_banner_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAGuildSDO_Member>>(
                    "members",
                    CMsgDOTAGuildSDO::get_members_for_reflect,
                    CMsgDOTAGuildSDO::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAGuildSDO_Invitation>>(
                    "invitations",
                    CMsgDOTAGuildSDO::get_invitations_for_reflect,
                    CMsgDOTAGuildSDO::mut_invitations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CMsgDOTAGuildSDO::get_message_for_reflect,
                    CMsgDOTAGuildSDO::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "incremental",
                    CMsgDOTAGuildSDO::get_incremental_for_reflect,
                    CMsgDOTAGuildSDO::mut_incremental_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildSDO>(
                    "CMsgDOTAGuildSDO",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildSDO {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_name();
        self.clear_tag();
        self.clear_time_created();
        self.clear_time_disbanded();
        self.clear_logo();
        self.clear_base_logo();
        self.clear_banner_logo();
        self.clear_members();
        self.clear_invitations();
        self.clear_message();
        self.clear_incremental();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildSDO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildSDO {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildSDO_Member {
    // message fields
    account_id: ::std::option::Option<u32>,
    time_joined: ::std::option::Option<u32>,
    role: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildSDO_Member {}

impl CMsgDOTAGuildSDO_Member {
    pub fn new() -> CMsgDOTAGuildSDO_Member {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildSDO_Member {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildSDO_Member> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildSDO_Member,
        };
        unsafe {
            instance.get(CMsgDOTAGuildSDO_Member::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 time_joined = 2;

    pub fn clear_time_joined(&mut self) {
        self.time_joined = ::std::option::Option::None;
    }

    pub fn has_time_joined(&self) -> bool {
        self.time_joined.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_joined(&mut self, v: u32) {
        self.time_joined = ::std::option::Option::Some(v);
    }

    pub fn get_time_joined(&self) -> u32 {
        self.time_joined.unwrap_or(0)
    }

    fn get_time_joined_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_joined
    }

    fn mut_time_joined_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_joined
    }

    // optional uint32 role = 3;

    pub fn clear_role(&mut self) {
        self.role = ::std::option::Option::None;
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: u32) {
        self.role = ::std::option::Option::Some(v);
    }

    pub fn get_role(&self) -> u32 {
        self.role.unwrap_or(0)
    }

    fn get_role_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.role
    }
}

impl ::protobuf::Message for CMsgDOTAGuildSDO_Member {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_joined = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.role = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.time_joined {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.role {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.time_joined {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.role {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildSDO_Member {
    fn new() -> CMsgDOTAGuildSDO_Member {
        CMsgDOTAGuildSDO_Member::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildSDO_Member>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAGuildSDO_Member::get_account_id_for_reflect,
                    CMsgDOTAGuildSDO_Member::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_joined",
                    CMsgDOTAGuildSDO_Member::get_time_joined_for_reflect,
                    CMsgDOTAGuildSDO_Member::mut_time_joined_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "role",
                    CMsgDOTAGuildSDO_Member::get_role_for_reflect,
                    CMsgDOTAGuildSDO_Member::mut_role_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildSDO_Member>(
                    "CMsgDOTAGuildSDO_Member",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildSDO_Member {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_time_joined();
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildSDO_Member {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildSDO_Member {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildSDO_Invitation {
    // message fields
    account_id: ::std::option::Option<u32>,
    time_sent: ::std::option::Option<u32>,
    account_id_sender: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildSDO_Invitation {}

impl CMsgDOTAGuildSDO_Invitation {
    pub fn new() -> CMsgDOTAGuildSDO_Invitation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildSDO_Invitation {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildSDO_Invitation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildSDO_Invitation,
        };
        unsafe {
            instance.get(CMsgDOTAGuildSDO_Invitation::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 time_sent = 2;

    pub fn clear_time_sent(&mut self) {
        self.time_sent = ::std::option::Option::None;
    }

    pub fn has_time_sent(&self) -> bool {
        self.time_sent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_sent(&mut self, v: u32) {
        self.time_sent = ::std::option::Option::Some(v);
    }

    pub fn get_time_sent(&self) -> u32 {
        self.time_sent.unwrap_or(0)
    }

    fn get_time_sent_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_sent
    }

    fn mut_time_sent_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_sent
    }

    // optional uint32 account_id_sender = 3;

    pub fn clear_account_id_sender(&mut self) {
        self.account_id_sender = ::std::option::Option::None;
    }

    pub fn has_account_id_sender(&self) -> bool {
        self.account_id_sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id_sender(&mut self, v: u32) {
        self.account_id_sender = ::std::option::Option::Some(v);
    }

    pub fn get_account_id_sender(&self) -> u32 {
        self.account_id_sender.unwrap_or(0)
    }

    fn get_account_id_sender_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id_sender
    }

    fn mut_account_id_sender_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id_sender
    }
}

impl ::protobuf::Message for CMsgDOTAGuildSDO_Invitation {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_sent = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.account_id_sender = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.time_sent {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_id_sender {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.time_sent {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.account_id_sender {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildSDO_Invitation {
    fn new() -> CMsgDOTAGuildSDO_Invitation {
        CMsgDOTAGuildSDO_Invitation::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildSDO_Invitation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAGuildSDO_Invitation::get_account_id_for_reflect,
                    CMsgDOTAGuildSDO_Invitation::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_sent",
                    CMsgDOTAGuildSDO_Invitation::get_time_sent_for_reflect,
                    CMsgDOTAGuildSDO_Invitation::mut_time_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id_sender",
                    CMsgDOTAGuildSDO_Invitation::get_account_id_sender_for_reflect,
                    CMsgDOTAGuildSDO_Invitation::mut_account_id_sender_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildSDO_Invitation>(
                    "CMsgDOTAGuildSDO_Invitation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildSDO_Invitation {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_time_sent();
        self.clear_account_id_sender();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildSDO_Invitation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildSDO_Invitation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildAuditSDO {
    // message fields
    guild_id: ::std::option::Option<u32>,
    entries: ::protobuf::RepeatedField<CMsgDOTAGuildAuditSDO_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildAuditSDO {}

impl CMsgDOTAGuildAuditSDO {
    pub fn new() -> CMsgDOTAGuildAuditSDO {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildAuditSDO {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildAuditSDO> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildAuditSDO,
        };
        unsafe {
            instance.get(CMsgDOTAGuildAuditSDO::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // repeated .dota.CMsgDOTAGuildAuditSDO.Entry entries = 2;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAGuildAuditSDO_Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAGuildAuditSDO_Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAGuildAuditSDO_Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[CMsgDOTAGuildAuditSDO_Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAGuildAuditSDO_Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAGuildAuditSDO_Entry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for CMsgDOTAGuildAuditSDO {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        for v in &self.entries {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildAuditSDO {
    fn new() -> CMsgDOTAGuildAuditSDO {
        CMsgDOTAGuildAuditSDO::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildAuditSDO>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildAuditSDO::get_guild_id_for_reflect,
                    CMsgDOTAGuildAuditSDO::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAGuildAuditSDO_Entry>>(
                    "entries",
                    CMsgDOTAGuildAuditSDO::get_entries_for_reflect,
                    CMsgDOTAGuildAuditSDO::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildAuditSDO>(
                    "CMsgDOTAGuildAuditSDO",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildAuditSDO {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildAuditSDO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildAuditSDO {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildAuditSDO_Entry {
    // message fields
    event_index: ::std::option::Option<u32>,
    timestamp: ::std::option::Option<u32>,
    action: ::std::option::Option<u32>,
    account_id_requestor: ::std::option::Option<u32>,
    account_id_target: ::std::option::Option<u32>,
    reference_data_a: ::std::option::Option<u32>,
    reference_data_b: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildAuditSDO_Entry {}

impl CMsgDOTAGuildAuditSDO_Entry {
    pub fn new() -> CMsgDOTAGuildAuditSDO_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildAuditSDO_Entry {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildAuditSDO_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildAuditSDO_Entry,
        };
        unsafe {
            instance.get(CMsgDOTAGuildAuditSDO_Entry::new)
        }
    }

    // optional uint32 event_index = 1;

    pub fn clear_event_index(&mut self) {
        self.event_index = ::std::option::Option::None;
    }

    pub fn has_event_index(&self) -> bool {
        self.event_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_index(&mut self, v: u32) {
        self.event_index = ::std::option::Option::Some(v);
    }

    pub fn get_event_index(&self) -> u32 {
        self.event_index.unwrap_or(0)
    }

    fn get_event_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.event_index
    }

    fn mut_event_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.event_index
    }

    // optional uint32 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u32) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u32 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.timestamp
    }

    // optional uint32 action = 3;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: u32) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action(&self) -> u32 {
        self.action.unwrap_or(0)
    }

    fn get_action_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.action
    }

    // optional uint32 account_id_requestor = 4;

    pub fn clear_account_id_requestor(&mut self) {
        self.account_id_requestor = ::std::option::Option::None;
    }

    pub fn has_account_id_requestor(&self) -> bool {
        self.account_id_requestor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id_requestor(&mut self, v: u32) {
        self.account_id_requestor = ::std::option::Option::Some(v);
    }

    pub fn get_account_id_requestor(&self) -> u32 {
        self.account_id_requestor.unwrap_or(0)
    }

    fn get_account_id_requestor_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id_requestor
    }

    fn mut_account_id_requestor_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id_requestor
    }

    // optional uint32 account_id_target = 5;

    pub fn clear_account_id_target(&mut self) {
        self.account_id_target = ::std::option::Option::None;
    }

    pub fn has_account_id_target(&self) -> bool {
        self.account_id_target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id_target(&mut self, v: u32) {
        self.account_id_target = ::std::option::Option::Some(v);
    }

    pub fn get_account_id_target(&self) -> u32 {
        self.account_id_target.unwrap_or(0)
    }

    fn get_account_id_target_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id_target
    }

    fn mut_account_id_target_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id_target
    }

    // optional uint32 reference_data_a = 6;

    pub fn clear_reference_data_a(&mut self) {
        self.reference_data_a = ::std::option::Option::None;
    }

    pub fn has_reference_data_a(&self) -> bool {
        self.reference_data_a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reference_data_a(&mut self, v: u32) {
        self.reference_data_a = ::std::option::Option::Some(v);
    }

    pub fn get_reference_data_a(&self) -> u32 {
        self.reference_data_a.unwrap_or(0)
    }

    fn get_reference_data_a_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.reference_data_a
    }

    fn mut_reference_data_a_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.reference_data_a
    }

    // optional uint32 reference_data_b = 7;

    pub fn clear_reference_data_b(&mut self) {
        self.reference_data_b = ::std::option::Option::None;
    }

    pub fn has_reference_data_b(&self) -> bool {
        self.reference_data_b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reference_data_b(&mut self, v: u32) {
        self.reference_data_b = ::std::option::Option::Some(v);
    }

    pub fn get_reference_data_b(&self) -> u32 {
        self.reference_data_b.unwrap_or(0)
    }

    fn get_reference_data_b_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.reference_data_b
    }

    fn mut_reference_data_b_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.reference_data_b
    }
}

impl ::protobuf::Message for CMsgDOTAGuildAuditSDO_Entry {
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
                    self.event_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.action = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.account_id_requestor = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.account_id_target = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.reference_data_a = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.reference_data_b = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.event_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_id_requestor {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_id_target {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.reference_data_a {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.reference_data_b {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event_index {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.action {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.account_id_requestor {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.account_id_target {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.reference_data_a {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.reference_data_b {
            os.write_uint32(7, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildAuditSDO_Entry {
    fn new() -> CMsgDOTAGuildAuditSDO_Entry {
        CMsgDOTAGuildAuditSDO_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildAuditSDO_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "event_index",
                    CMsgDOTAGuildAuditSDO_Entry::get_event_index_for_reflect,
                    CMsgDOTAGuildAuditSDO_Entry::mut_event_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timestamp",
                    CMsgDOTAGuildAuditSDO_Entry::get_timestamp_for_reflect,
                    CMsgDOTAGuildAuditSDO_Entry::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "action",
                    CMsgDOTAGuildAuditSDO_Entry::get_action_for_reflect,
                    CMsgDOTAGuildAuditSDO_Entry::mut_action_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id_requestor",
                    CMsgDOTAGuildAuditSDO_Entry::get_account_id_requestor_for_reflect,
                    CMsgDOTAGuildAuditSDO_Entry::mut_account_id_requestor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id_target",
                    CMsgDOTAGuildAuditSDO_Entry::get_account_id_target_for_reflect,
                    CMsgDOTAGuildAuditSDO_Entry::mut_account_id_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "reference_data_a",
                    CMsgDOTAGuildAuditSDO_Entry::get_reference_data_a_for_reflect,
                    CMsgDOTAGuildAuditSDO_Entry::mut_reference_data_a_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "reference_data_b",
                    CMsgDOTAGuildAuditSDO_Entry::get_reference_data_b_for_reflect,
                    CMsgDOTAGuildAuditSDO_Entry::mut_reference_data_b_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildAuditSDO_Entry>(
                    "CMsgDOTAGuildAuditSDO_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildAuditSDO_Entry {
    fn clear(&mut self) {
        self.clear_event_index();
        self.clear_timestamp();
        self.clear_action();
        self.clear_account_id_requestor();
        self.clear_account_id_target();
        self.clear_reference_data_a();
        self.clear_reference_data_b();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildAuditSDO_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildAuditSDO_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAAccountGuildMembershipsSDO {
    // message fields
    account_id: ::std::option::Option<u32>,
    memberships: ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Membership>,
    invitations: ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Invitation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAAccountGuildMembershipsSDO {}

impl CMsgDOTAAccountGuildMembershipsSDO {
    pub fn new() -> CMsgDOTAAccountGuildMembershipsSDO {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAAccountGuildMembershipsSDO {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAAccountGuildMembershipsSDO> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAAccountGuildMembershipsSDO,
        };
        unsafe {
            instance.get(CMsgDOTAAccountGuildMembershipsSDO::new)
        }
    }

    // optional uint32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // repeated .dota.CMsgDOTAAccountGuildMembershipsSDO.Membership memberships = 2;

    pub fn clear_memberships(&mut self) {
        self.memberships.clear();
    }

    // Param is passed by value, moved
    pub fn set_memberships(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Membership>) {
        self.memberships = v;
    }

    // Mutable pointer to the field.
    pub fn mut_memberships(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Membership> {
        &mut self.memberships
    }

    // Take field
    pub fn take_memberships(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Membership> {
        ::std::mem::replace(&mut self.memberships, ::protobuf::RepeatedField::new())
    }

    pub fn get_memberships(&self) -> &[CMsgDOTAAccountGuildMembershipsSDO_Membership] {
        &self.memberships
    }

    fn get_memberships_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Membership> {
        &self.memberships
    }

    fn mut_memberships_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Membership> {
        &mut self.memberships
    }

    // repeated .dota.CMsgDOTAAccountGuildMembershipsSDO.Invitation invitations = 3;

    pub fn clear_invitations(&mut self) {
        self.invitations.clear();
    }

    // Param is passed by value, moved
    pub fn set_invitations(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Invitation>) {
        self.invitations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invitations(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Invitation> {
        &mut self.invitations
    }

    // Take field
    pub fn take_invitations(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Invitation> {
        ::std::mem::replace(&mut self.invitations, ::protobuf::RepeatedField::new())
    }

    pub fn get_invitations(&self) -> &[CMsgDOTAAccountGuildMembershipsSDO_Invitation] {
        &self.invitations
    }

    fn get_invitations_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Invitation> {
        &self.invitations
    }

    fn mut_invitations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAAccountGuildMembershipsSDO_Invitation> {
        &mut self.invitations
    }
}

impl ::protobuf::Message for CMsgDOTAAccountGuildMembershipsSDO {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.memberships)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.invitations)?;
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.memberships {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.invitations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        for v in &self.memberships {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.invitations {
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

impl ::protobuf::MessageStatic for CMsgDOTAAccountGuildMembershipsSDO {
    fn new() -> CMsgDOTAAccountGuildMembershipsSDO {
        CMsgDOTAAccountGuildMembershipsSDO::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAAccountGuildMembershipsSDO>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAAccountGuildMembershipsSDO::get_account_id_for_reflect,
                    CMsgDOTAAccountGuildMembershipsSDO::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAAccountGuildMembershipsSDO_Membership>>(
                    "memberships",
                    CMsgDOTAAccountGuildMembershipsSDO::get_memberships_for_reflect,
                    CMsgDOTAAccountGuildMembershipsSDO::mut_memberships_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAAccountGuildMembershipsSDO_Invitation>>(
                    "invitations",
                    CMsgDOTAAccountGuildMembershipsSDO::get_invitations_for_reflect,
                    CMsgDOTAAccountGuildMembershipsSDO::mut_invitations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAAccountGuildMembershipsSDO>(
                    "CMsgDOTAAccountGuildMembershipsSDO",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAAccountGuildMembershipsSDO {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_memberships();
        self.clear_invitations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAAccountGuildMembershipsSDO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAAccountGuildMembershipsSDO {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAAccountGuildMembershipsSDO_Membership {
    // message fields
    guild_id: ::std::option::Option<u32>,
    role: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAAccountGuildMembershipsSDO_Membership {}

impl CMsgDOTAAccountGuildMembershipsSDO_Membership {
    pub fn new() -> CMsgDOTAAccountGuildMembershipsSDO_Membership {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAAccountGuildMembershipsSDO_Membership {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAAccountGuildMembershipsSDO_Membership> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAAccountGuildMembershipsSDO_Membership,
        };
        unsafe {
            instance.get(CMsgDOTAAccountGuildMembershipsSDO_Membership::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional uint32 role = 2;

    pub fn clear_role(&mut self) {
        self.role = ::std::option::Option::None;
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: u32) {
        self.role = ::std::option::Option::Some(v);
    }

    pub fn get_role(&self) -> u32 {
        self.role.unwrap_or(0)
    }

    fn get_role_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.role
    }
}

impl ::protobuf::Message for CMsgDOTAAccountGuildMembershipsSDO_Membership {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.role = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.role {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.role {
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

impl ::protobuf::MessageStatic for CMsgDOTAAccountGuildMembershipsSDO_Membership {
    fn new() -> CMsgDOTAAccountGuildMembershipsSDO_Membership {
        CMsgDOTAAccountGuildMembershipsSDO_Membership::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAAccountGuildMembershipsSDO_Membership>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAAccountGuildMembershipsSDO_Membership::get_guild_id_for_reflect,
                    CMsgDOTAAccountGuildMembershipsSDO_Membership::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "role",
                    CMsgDOTAAccountGuildMembershipsSDO_Membership::get_role_for_reflect,
                    CMsgDOTAAccountGuildMembershipsSDO_Membership::mut_role_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAAccountGuildMembershipsSDO_Membership>(
                    "CMsgDOTAAccountGuildMembershipsSDO_Membership",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAAccountGuildMembershipsSDO_Membership {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAAccountGuildMembershipsSDO_Membership {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAAccountGuildMembershipsSDO_Membership {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAAccountGuildMembershipsSDO_Invitation {
    // message fields
    guild_id: ::std::option::Option<u32>,
    time_sent: ::std::option::Option<u32>,
    account_id_sender: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAAccountGuildMembershipsSDO_Invitation {}

impl CMsgDOTAAccountGuildMembershipsSDO_Invitation {
    pub fn new() -> CMsgDOTAAccountGuildMembershipsSDO_Invitation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAAccountGuildMembershipsSDO_Invitation {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAAccountGuildMembershipsSDO_Invitation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAAccountGuildMembershipsSDO_Invitation,
        };
        unsafe {
            instance.get(CMsgDOTAAccountGuildMembershipsSDO_Invitation::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional uint32 time_sent = 2;

    pub fn clear_time_sent(&mut self) {
        self.time_sent = ::std::option::Option::None;
    }

    pub fn has_time_sent(&self) -> bool {
        self.time_sent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_sent(&mut self, v: u32) {
        self.time_sent = ::std::option::Option::Some(v);
    }

    pub fn get_time_sent(&self) -> u32 {
        self.time_sent.unwrap_or(0)
    }

    fn get_time_sent_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_sent
    }

    fn mut_time_sent_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_sent
    }

    // optional uint32 account_id_sender = 3;

    pub fn clear_account_id_sender(&mut self) {
        self.account_id_sender = ::std::option::Option::None;
    }

    pub fn has_account_id_sender(&self) -> bool {
        self.account_id_sender.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id_sender(&mut self, v: u32) {
        self.account_id_sender = ::std::option::Option::Some(v);
    }

    pub fn get_account_id_sender(&self) -> u32 {
        self.account_id_sender.unwrap_or(0)
    }

    fn get_account_id_sender_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id_sender
    }

    fn mut_account_id_sender_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id_sender
    }
}

impl ::protobuf::Message for CMsgDOTAAccountGuildMembershipsSDO_Invitation {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_sent = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.account_id_sender = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.time_sent {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.account_id_sender {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.time_sent {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.account_id_sender {
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

impl ::protobuf::MessageStatic for CMsgDOTAAccountGuildMembershipsSDO_Invitation {
    fn new() -> CMsgDOTAAccountGuildMembershipsSDO_Invitation {
        CMsgDOTAAccountGuildMembershipsSDO_Invitation::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAAccountGuildMembershipsSDO_Invitation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAAccountGuildMembershipsSDO_Invitation::get_guild_id_for_reflect,
                    CMsgDOTAAccountGuildMembershipsSDO_Invitation::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_sent",
                    CMsgDOTAAccountGuildMembershipsSDO_Invitation::get_time_sent_for_reflect,
                    CMsgDOTAAccountGuildMembershipsSDO_Invitation::mut_time_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id_sender",
                    CMsgDOTAAccountGuildMembershipsSDO_Invitation::get_account_id_sender_for_reflect,
                    CMsgDOTAAccountGuildMembershipsSDO_Invitation::mut_account_id_sender_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAAccountGuildMembershipsSDO_Invitation>(
                    "CMsgDOTAAccountGuildMembershipsSDO_Invitation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAAccountGuildMembershipsSDO_Invitation {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_time_sent();
        self.clear_account_id_sender();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAAccountGuildMembershipsSDO_Invitation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAAccountGuildMembershipsSDO_Invitation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildCreateRequest {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    tag: ::protobuf::SingularField<::std::string::String>,
    logo: ::std::option::Option<u64>,
    base_logo: ::std::option::Option<u64>,
    banner_logo: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildCreateRequest {}

impl CMsgDOTAGuildCreateRequest {
    pub fn new() -> CMsgDOTAGuildCreateRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildCreateRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildCreateRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildCreateRequest,
        };
        unsafe {
            instance.get(CMsgDOTAGuildCreateRequest::new)
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

    // optional string tag = 2;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::string::String) {
        self.tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tag(&mut self) -> &mut ::std::string::String {
        if self.tag.is_none() {
            self.tag.set_default();
        };
        self.tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::string::String {
        self.tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_tag(&self) -> &str {
        match self.tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.tag
    }

    // optional uint64 logo = 3;

    pub fn clear_logo(&mut self) {
        self.logo = ::std::option::Option::None;
    }

    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo(&mut self, v: u64) {
        self.logo = ::std::option::Option::Some(v);
    }

    pub fn get_logo(&self) -> u64 {
        self.logo.unwrap_or(0)
    }

    fn get_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.logo
    }

    fn mut_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.logo
    }

    // optional uint64 base_logo = 4;

    pub fn clear_base_logo(&mut self) {
        self.base_logo = ::std::option::Option::None;
    }

    pub fn has_base_logo(&self) -> bool {
        self.base_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_logo(&mut self, v: u64) {
        self.base_logo = ::std::option::Option::Some(v);
    }

    pub fn get_base_logo(&self) -> u64 {
        self.base_logo.unwrap_or(0)
    }

    fn get_base_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.base_logo
    }

    fn mut_base_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.base_logo
    }

    // optional uint64 banner_logo = 5;

    pub fn clear_banner_logo(&mut self) {
        self.banner_logo = ::std::option::Option::None;
    }

    pub fn has_banner_logo(&self) -> bool {
        self.banner_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_banner_logo(&mut self, v: u64) {
        self.banner_logo = ::std::option::Option::Some(v);
    }

    pub fn get_banner_logo(&self) -> u64 {
        self.banner_logo.unwrap_or(0)
    }

    fn get_banner_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.banner_logo
    }

    fn mut_banner_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.banner_logo
    }
}

impl ::protobuf::Message for CMsgDOTAGuildCreateRequest {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.logo = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.base_logo = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.banner_logo = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.base_logo {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.banner_logo {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.tag.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.logo {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.base_logo {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.banner_logo {
            os.write_uint64(5, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildCreateRequest {
    fn new() -> CMsgDOTAGuildCreateRequest {
        CMsgDOTAGuildCreateRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildCreateRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgDOTAGuildCreateRequest::get_name_for_reflect,
                    CMsgDOTAGuildCreateRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    CMsgDOTAGuildCreateRequest::get_tag_for_reflect,
                    CMsgDOTAGuildCreateRequest::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTAGuildCreateRequest::get_logo_for_reflect,
                    CMsgDOTAGuildCreateRequest::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "base_logo",
                    CMsgDOTAGuildCreateRequest::get_base_logo_for_reflect,
                    CMsgDOTAGuildCreateRequest::mut_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "banner_logo",
                    CMsgDOTAGuildCreateRequest::get_banner_logo_for_reflect,
                    CMsgDOTAGuildCreateRequest::mut_banner_logo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildCreateRequest>(
                    "CMsgDOTAGuildCreateRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildCreateRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_tag();
        self.clear_logo();
        self.clear_base_logo();
        self.clear_banner_logo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildCreateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildCreateRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildCreateResponse {
    // message fields
    guild_id: ::std::option::Option<u32>,
    errors: ::std::vec::Vec<CMsgDOTAGuildCreateResponse_EError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildCreateResponse {}

impl CMsgDOTAGuildCreateResponse {
    pub fn new() -> CMsgDOTAGuildCreateResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildCreateResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildCreateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildCreateResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGuildCreateResponse::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // repeated .dota.CMsgDOTAGuildCreateResponse.EError errors = 2;

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ::std::vec::Vec<CMsgDOTAGuildCreateResponse_EError>) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<CMsgDOTAGuildCreateResponse_EError> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::std::vec::Vec<CMsgDOTAGuildCreateResponse_EError> {
        ::std::mem::replace(&mut self.errors, ::std::vec::Vec::new())
    }

    pub fn get_errors(&self) -> &[CMsgDOTAGuildCreateResponse_EError] {
        &self.errors
    }

    fn get_errors_for_reflect(&self) -> &::std::vec::Vec<CMsgDOTAGuildCreateResponse_EError> {
        &self.errors
    }

    fn mut_errors_for_reflect(&mut self) -> &mut ::std::vec::Vec<CMsgDOTAGuildCreateResponse_EError> {
        &mut self.errors
    }
}

impl ::protobuf::Message for CMsgDOTAGuildCreateResponse {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.errors)?;
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.errors {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        for v in &self.errors {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildCreateResponse {
    fn new() -> CMsgDOTAGuildCreateResponse {
        CMsgDOTAGuildCreateResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildCreateResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildCreateResponse::get_guild_id_for_reflect,
                    CMsgDOTAGuildCreateResponse::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAGuildCreateResponse_EError>>(
                    "errors",
                    CMsgDOTAGuildCreateResponse::get_errors_for_reflect,
                    CMsgDOTAGuildCreateResponse::mut_errors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildCreateResponse>(
                    "CMsgDOTAGuildCreateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildCreateResponse {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_errors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildCreateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildCreateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAGuildCreateResponse_EError {
    UNSPECIFIED = 0,
    NAME_EMPTY = 1,
    NAME_BAD_CHARACTERS = 2,
    NAME_TOO_LONG = 3,
    NAME_TAKEN = 4,
    TAG_EMPTY = 5,
    TAG_BAD_CHARACTERS = 6,
    TAG_TOO_LONG = 7,
    ACCOUNT_TOO_MANY_GUILDS = 8,
    LOGO_UPLOAD_FAILED = 9,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAGuildCreateResponse_EError {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAGuildCreateResponse_EError> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::UNSPECIFIED),
            1 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::NAME_EMPTY),
            2 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::NAME_BAD_CHARACTERS),
            3 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::NAME_TOO_LONG),
            4 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::NAME_TAKEN),
            5 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::TAG_EMPTY),
            6 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::TAG_BAD_CHARACTERS),
            7 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::TAG_TOO_LONG),
            8 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::ACCOUNT_TOO_MANY_GUILDS),
            9 => ::std::option::Option::Some(CMsgDOTAGuildCreateResponse_EError::LOGO_UPLOAD_FAILED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAGuildCreateResponse_EError] = &[
            CMsgDOTAGuildCreateResponse_EError::UNSPECIFIED,
            CMsgDOTAGuildCreateResponse_EError::NAME_EMPTY,
            CMsgDOTAGuildCreateResponse_EError::NAME_BAD_CHARACTERS,
            CMsgDOTAGuildCreateResponse_EError::NAME_TOO_LONG,
            CMsgDOTAGuildCreateResponse_EError::NAME_TAKEN,
            CMsgDOTAGuildCreateResponse_EError::TAG_EMPTY,
            CMsgDOTAGuildCreateResponse_EError::TAG_BAD_CHARACTERS,
            CMsgDOTAGuildCreateResponse_EError::TAG_TOO_LONG,
            CMsgDOTAGuildCreateResponse_EError::ACCOUNT_TOO_MANY_GUILDS,
            CMsgDOTAGuildCreateResponse_EError::LOGO_UPLOAD_FAILED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAGuildCreateResponse_EError>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAGuildCreateResponse_EError", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAGuildCreateResponse_EError {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildCreateResponse_EError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildSetAccountRoleRequest {
    // message fields
    guild_id: ::std::option::Option<u32>,
    target_account_id: ::std::option::Option<u32>,
    target_role: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildSetAccountRoleRequest {}

impl CMsgDOTAGuildSetAccountRoleRequest {
    pub fn new() -> CMsgDOTAGuildSetAccountRoleRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildSetAccountRoleRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildSetAccountRoleRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildSetAccountRoleRequest,
        };
        unsafe {
            instance.get(CMsgDOTAGuildSetAccountRoleRequest::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional uint32 target_account_id = 2;

    pub fn clear_target_account_id(&mut self) {
        self.target_account_id = ::std::option::Option::None;
    }

    pub fn has_target_account_id(&self) -> bool {
        self.target_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_account_id(&mut self, v: u32) {
        self.target_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_target_account_id(&self) -> u32 {
        self.target_account_id.unwrap_or(0)
    }

    fn get_target_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_account_id
    }

    fn mut_target_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_account_id
    }

    // optional uint32 target_role = 3;

    pub fn clear_target_role(&mut self) {
        self.target_role = ::std::option::Option::None;
    }

    pub fn has_target_role(&self) -> bool {
        self.target_role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_role(&mut self, v: u32) {
        self.target_role = ::std::option::Option::Some(v);
    }

    pub fn get_target_role(&self) -> u32 {
        self.target_role.unwrap_or(0)
    }

    fn get_target_role_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_role
    }

    fn mut_target_role_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_role
    }
}

impl ::protobuf::Message for CMsgDOTAGuildSetAccountRoleRequest {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.target_account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.target_role = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target_role {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.target_account_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.target_role {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildSetAccountRoleRequest {
    fn new() -> CMsgDOTAGuildSetAccountRoleRequest {
        CMsgDOTAGuildSetAccountRoleRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildSetAccountRoleRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildSetAccountRoleRequest::get_guild_id_for_reflect,
                    CMsgDOTAGuildSetAccountRoleRequest::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_account_id",
                    CMsgDOTAGuildSetAccountRoleRequest::get_target_account_id_for_reflect,
                    CMsgDOTAGuildSetAccountRoleRequest::mut_target_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_role",
                    CMsgDOTAGuildSetAccountRoleRequest::get_target_role_for_reflect,
                    CMsgDOTAGuildSetAccountRoleRequest::mut_target_role_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildSetAccountRoleRequest>(
                    "CMsgDOTAGuildSetAccountRoleRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildSetAccountRoleRequest {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_target_account_id();
        self.clear_target_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildSetAccountRoleRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildSetAccountRoleRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildSetAccountRoleResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAGuildSetAccountRoleResponse_EResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildSetAccountRoleResponse {}

impl CMsgDOTAGuildSetAccountRoleResponse {
    pub fn new() -> CMsgDOTAGuildSetAccountRoleResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildSetAccountRoleResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildSetAccountRoleResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildSetAccountRoleResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGuildSetAccountRoleResponse::new)
        }
    }

    // optional .dota.CMsgDOTAGuildSetAccountRoleResponse.EResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAGuildSetAccountRoleResponse_EResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAGuildSetAccountRoleResponse_EResult {
        self.result.unwrap_or(CMsgDOTAGuildSetAccountRoleResponse_EResult::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAGuildSetAccountRoleResponse_EResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAGuildSetAccountRoleResponse_EResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAGuildSetAccountRoleResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildSetAccountRoleResponse {
    fn new() -> CMsgDOTAGuildSetAccountRoleResponse {
        CMsgDOTAGuildSetAccountRoleResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildSetAccountRoleResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAGuildSetAccountRoleResponse_EResult>>(
                    "result",
                    CMsgDOTAGuildSetAccountRoleResponse::get_result_for_reflect,
                    CMsgDOTAGuildSetAccountRoleResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildSetAccountRoleResponse>(
                    "CMsgDOTAGuildSetAccountRoleResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildSetAccountRoleResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildSetAccountRoleResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildSetAccountRoleResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAGuildSetAccountRoleResponse_EResult {
    SUCCESS = 0,
    ERROR_UNSPECIFIED = 1,
    ERROR_NO_PERMISSION = 2,
    ERROR_NO_OTHER_LEADER = 3,
    ERROR_ACCOUNT_TOO_MANY_GUILDS = 4,
    ERROR_GUILD_TOO_MANY_MEMBERS = 5,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAGuildSetAccountRoleResponse_EResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAGuildSetAccountRoleResponse_EResult> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAGuildSetAccountRoleResponse_EResult::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_UNSPECIFIED),
            2 => ::std::option::Option::Some(CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_NO_PERMISSION),
            3 => ::std::option::Option::Some(CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_NO_OTHER_LEADER),
            4 => ::std::option::Option::Some(CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_ACCOUNT_TOO_MANY_GUILDS),
            5 => ::std::option::Option::Some(CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_GUILD_TOO_MANY_MEMBERS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAGuildSetAccountRoleResponse_EResult] = &[
            CMsgDOTAGuildSetAccountRoleResponse_EResult::SUCCESS,
            CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_UNSPECIFIED,
            CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_NO_PERMISSION,
            CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_NO_OTHER_LEADER,
            CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_ACCOUNT_TOO_MANY_GUILDS,
            CMsgDOTAGuildSetAccountRoleResponse_EResult::ERROR_GUILD_TOO_MANY_MEMBERS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAGuildSetAccountRoleResponse_EResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAGuildSetAccountRoleResponse_EResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAGuildSetAccountRoleResponse_EResult {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildSetAccountRoleResponse_EResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildInviteAccountRequest {
    // message fields
    guild_id: ::std::option::Option<u32>,
    target_account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildInviteAccountRequest {}

impl CMsgDOTAGuildInviteAccountRequest {
    pub fn new() -> CMsgDOTAGuildInviteAccountRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildInviteAccountRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildInviteAccountRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildInviteAccountRequest,
        };
        unsafe {
            instance.get(CMsgDOTAGuildInviteAccountRequest::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional uint32 target_account_id = 2;

    pub fn clear_target_account_id(&mut self) {
        self.target_account_id = ::std::option::Option::None;
    }

    pub fn has_target_account_id(&self) -> bool {
        self.target_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_account_id(&mut self, v: u32) {
        self.target_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_target_account_id(&self) -> u32 {
        self.target_account_id.unwrap_or(0)
    }

    fn get_target_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_account_id
    }

    fn mut_target_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_account_id
    }
}

impl ::protobuf::Message for CMsgDOTAGuildInviteAccountRequest {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.target_account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.target_account_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildInviteAccountRequest {
    fn new() -> CMsgDOTAGuildInviteAccountRequest {
        CMsgDOTAGuildInviteAccountRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildInviteAccountRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildInviteAccountRequest::get_guild_id_for_reflect,
                    CMsgDOTAGuildInviteAccountRequest::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_account_id",
                    CMsgDOTAGuildInviteAccountRequest::get_target_account_id_for_reflect,
                    CMsgDOTAGuildInviteAccountRequest::mut_target_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildInviteAccountRequest>(
                    "CMsgDOTAGuildInviteAccountRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildInviteAccountRequest {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_target_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildInviteAccountRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildInviteAccountRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildInviteAccountResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAGuildInviteAccountResponse_EResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildInviteAccountResponse {}

impl CMsgDOTAGuildInviteAccountResponse {
    pub fn new() -> CMsgDOTAGuildInviteAccountResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildInviteAccountResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildInviteAccountResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildInviteAccountResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGuildInviteAccountResponse::new)
        }
    }

    // optional .dota.CMsgDOTAGuildInviteAccountResponse.EResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAGuildInviteAccountResponse_EResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAGuildInviteAccountResponse_EResult {
        self.result.unwrap_or(CMsgDOTAGuildInviteAccountResponse_EResult::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAGuildInviteAccountResponse_EResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAGuildInviteAccountResponse_EResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAGuildInviteAccountResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildInviteAccountResponse {
    fn new() -> CMsgDOTAGuildInviteAccountResponse {
        CMsgDOTAGuildInviteAccountResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildInviteAccountResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAGuildInviteAccountResponse_EResult>>(
                    "result",
                    CMsgDOTAGuildInviteAccountResponse::get_result_for_reflect,
                    CMsgDOTAGuildInviteAccountResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildInviteAccountResponse>(
                    "CMsgDOTAGuildInviteAccountResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildInviteAccountResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildInviteAccountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildInviteAccountResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAGuildInviteAccountResponse_EResult {
    SUCCESS = 0,
    ERROR_UNSPECIFIED = 1,
    ERROR_NO_PERMISSION = 2,
    ERROR_ACCOUNT_ALREADY_INVITED = 3,
    ERROR_ACCOUNT_ALREADY_IN_GUILD = 4,
    ERROR_ACCOUNT_TOO_MANY_INVITES = 5,
    ERROR_GUILD_TOO_MANY_INVITES = 6,
    ERROR_ACCOUNT_TOO_MANY_GUILDS = 7,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAGuildInviteAccountResponse_EResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAGuildInviteAccountResponse_EResult> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAGuildInviteAccountResponse_EResult::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_UNSPECIFIED),
            2 => ::std::option::Option::Some(CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_NO_PERMISSION),
            3 => ::std::option::Option::Some(CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_ACCOUNT_ALREADY_INVITED),
            4 => ::std::option::Option::Some(CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_ACCOUNT_ALREADY_IN_GUILD),
            5 => ::std::option::Option::Some(CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_ACCOUNT_TOO_MANY_INVITES),
            6 => ::std::option::Option::Some(CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_GUILD_TOO_MANY_INVITES),
            7 => ::std::option::Option::Some(CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_ACCOUNT_TOO_MANY_GUILDS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAGuildInviteAccountResponse_EResult] = &[
            CMsgDOTAGuildInviteAccountResponse_EResult::SUCCESS,
            CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_UNSPECIFIED,
            CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_NO_PERMISSION,
            CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_ACCOUNT_ALREADY_INVITED,
            CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_ACCOUNT_ALREADY_IN_GUILD,
            CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_ACCOUNT_TOO_MANY_INVITES,
            CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_GUILD_TOO_MANY_INVITES,
            CMsgDOTAGuildInviteAccountResponse_EResult::ERROR_ACCOUNT_TOO_MANY_GUILDS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAGuildInviteAccountResponse_EResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAGuildInviteAccountResponse_EResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAGuildInviteAccountResponse_EResult {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildInviteAccountResponse_EResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildCancelInviteRequest {
    // message fields
    guild_id: ::std::option::Option<u32>,
    target_account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildCancelInviteRequest {}

impl CMsgDOTAGuildCancelInviteRequest {
    pub fn new() -> CMsgDOTAGuildCancelInviteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildCancelInviteRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildCancelInviteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildCancelInviteRequest,
        };
        unsafe {
            instance.get(CMsgDOTAGuildCancelInviteRequest::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional uint32 target_account_id = 2;

    pub fn clear_target_account_id(&mut self) {
        self.target_account_id = ::std::option::Option::None;
    }

    pub fn has_target_account_id(&self) -> bool {
        self.target_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_account_id(&mut self, v: u32) {
        self.target_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_target_account_id(&self) -> u32 {
        self.target_account_id.unwrap_or(0)
    }

    fn get_target_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_account_id
    }

    fn mut_target_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_account_id
    }
}

impl ::protobuf::Message for CMsgDOTAGuildCancelInviteRequest {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.target_account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.target_account_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildCancelInviteRequest {
    fn new() -> CMsgDOTAGuildCancelInviteRequest {
        CMsgDOTAGuildCancelInviteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildCancelInviteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildCancelInviteRequest::get_guild_id_for_reflect,
                    CMsgDOTAGuildCancelInviteRequest::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_account_id",
                    CMsgDOTAGuildCancelInviteRequest::get_target_account_id_for_reflect,
                    CMsgDOTAGuildCancelInviteRequest::mut_target_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildCancelInviteRequest>(
                    "CMsgDOTAGuildCancelInviteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildCancelInviteRequest {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_target_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildCancelInviteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildCancelInviteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildCancelInviteResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAGuildCancelInviteResponse_EResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildCancelInviteResponse {}

impl CMsgDOTAGuildCancelInviteResponse {
    pub fn new() -> CMsgDOTAGuildCancelInviteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildCancelInviteResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildCancelInviteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildCancelInviteResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGuildCancelInviteResponse::new)
        }
    }

    // optional .dota.CMsgDOTAGuildCancelInviteResponse.EResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAGuildCancelInviteResponse_EResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAGuildCancelInviteResponse_EResult {
        self.result.unwrap_or(CMsgDOTAGuildCancelInviteResponse_EResult::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAGuildCancelInviteResponse_EResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAGuildCancelInviteResponse_EResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAGuildCancelInviteResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildCancelInviteResponse {
    fn new() -> CMsgDOTAGuildCancelInviteResponse {
        CMsgDOTAGuildCancelInviteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildCancelInviteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAGuildCancelInviteResponse_EResult>>(
                    "result",
                    CMsgDOTAGuildCancelInviteResponse::get_result_for_reflect,
                    CMsgDOTAGuildCancelInviteResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildCancelInviteResponse>(
                    "CMsgDOTAGuildCancelInviteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildCancelInviteResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildCancelInviteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildCancelInviteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAGuildCancelInviteResponse_EResult {
    SUCCESS = 0,
    ERROR_UNSPECIFIED = 1,
    ERROR_NO_PERMISSION = 2,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAGuildCancelInviteResponse_EResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAGuildCancelInviteResponse_EResult> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAGuildCancelInviteResponse_EResult::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAGuildCancelInviteResponse_EResult::ERROR_UNSPECIFIED),
            2 => ::std::option::Option::Some(CMsgDOTAGuildCancelInviteResponse_EResult::ERROR_NO_PERMISSION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAGuildCancelInviteResponse_EResult] = &[
            CMsgDOTAGuildCancelInviteResponse_EResult::SUCCESS,
            CMsgDOTAGuildCancelInviteResponse_EResult::ERROR_UNSPECIFIED,
            CMsgDOTAGuildCancelInviteResponse_EResult::ERROR_NO_PERMISSION,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAGuildCancelInviteResponse_EResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAGuildCancelInviteResponse_EResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAGuildCancelInviteResponse_EResult {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildCancelInviteResponse_EResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildUpdateDetailsRequest {
    // message fields
    guild_id: ::std::option::Option<u32>,
    logo: ::std::option::Option<u64>,
    base_logo: ::std::option::Option<u64>,
    banner_logo: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildUpdateDetailsRequest {}

impl CMsgDOTAGuildUpdateDetailsRequest {
    pub fn new() -> CMsgDOTAGuildUpdateDetailsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildUpdateDetailsRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildUpdateDetailsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildUpdateDetailsRequest,
        };
        unsafe {
            instance.get(CMsgDOTAGuildUpdateDetailsRequest::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional uint64 logo = 2;

    pub fn clear_logo(&mut self) {
        self.logo = ::std::option::Option::None;
    }

    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo(&mut self, v: u64) {
        self.logo = ::std::option::Option::Some(v);
    }

    pub fn get_logo(&self) -> u64 {
        self.logo.unwrap_or(0)
    }

    fn get_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.logo
    }

    fn mut_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.logo
    }

    // optional uint64 base_logo = 3;

    pub fn clear_base_logo(&mut self) {
        self.base_logo = ::std::option::Option::None;
    }

    pub fn has_base_logo(&self) -> bool {
        self.base_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_logo(&mut self, v: u64) {
        self.base_logo = ::std::option::Option::Some(v);
    }

    pub fn get_base_logo(&self) -> u64 {
        self.base_logo.unwrap_or(0)
    }

    fn get_base_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.base_logo
    }

    fn mut_base_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.base_logo
    }

    // optional uint64 banner_logo = 4;

    pub fn clear_banner_logo(&mut self) {
        self.banner_logo = ::std::option::Option::None;
    }

    pub fn has_banner_logo(&self) -> bool {
        self.banner_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_banner_logo(&mut self, v: u64) {
        self.banner_logo = ::std::option::Option::Some(v);
    }

    pub fn get_banner_logo(&self) -> u64 {
        self.banner_logo.unwrap_or(0)
    }

    fn get_banner_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.banner_logo
    }

    fn mut_banner_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.banner_logo
    }
}

impl ::protobuf::Message for CMsgDOTAGuildUpdateDetailsRequest {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.logo = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.base_logo = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.banner_logo = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.base_logo {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.banner_logo {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.logo {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.base_logo {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.banner_logo {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildUpdateDetailsRequest {
    fn new() -> CMsgDOTAGuildUpdateDetailsRequest {
        CMsgDOTAGuildUpdateDetailsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildUpdateDetailsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildUpdateDetailsRequest::get_guild_id_for_reflect,
                    CMsgDOTAGuildUpdateDetailsRequest::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTAGuildUpdateDetailsRequest::get_logo_for_reflect,
                    CMsgDOTAGuildUpdateDetailsRequest::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "base_logo",
                    CMsgDOTAGuildUpdateDetailsRequest::get_base_logo_for_reflect,
                    CMsgDOTAGuildUpdateDetailsRequest::mut_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "banner_logo",
                    CMsgDOTAGuildUpdateDetailsRequest::get_banner_logo_for_reflect,
                    CMsgDOTAGuildUpdateDetailsRequest::mut_banner_logo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildUpdateDetailsRequest>(
                    "CMsgDOTAGuildUpdateDetailsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildUpdateDetailsRequest {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_logo();
        self.clear_base_logo();
        self.clear_banner_logo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildUpdateDetailsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildUpdateDetailsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildUpdateDetailsResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAGuildUpdateDetailsResponse_EResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildUpdateDetailsResponse {}

impl CMsgDOTAGuildUpdateDetailsResponse {
    pub fn new() -> CMsgDOTAGuildUpdateDetailsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildUpdateDetailsResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildUpdateDetailsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildUpdateDetailsResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGuildUpdateDetailsResponse::new)
        }
    }

    // optional .dota.CMsgDOTAGuildUpdateDetailsResponse.EResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAGuildUpdateDetailsResponse_EResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAGuildUpdateDetailsResponse_EResult {
        self.result.unwrap_or(CMsgDOTAGuildUpdateDetailsResponse_EResult::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAGuildUpdateDetailsResponse_EResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAGuildUpdateDetailsResponse_EResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAGuildUpdateDetailsResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildUpdateDetailsResponse {
    fn new() -> CMsgDOTAGuildUpdateDetailsResponse {
        CMsgDOTAGuildUpdateDetailsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildUpdateDetailsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAGuildUpdateDetailsResponse_EResult>>(
                    "result",
                    CMsgDOTAGuildUpdateDetailsResponse::get_result_for_reflect,
                    CMsgDOTAGuildUpdateDetailsResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildUpdateDetailsResponse>(
                    "CMsgDOTAGuildUpdateDetailsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildUpdateDetailsResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildUpdateDetailsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildUpdateDetailsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAGuildUpdateDetailsResponse_EResult {
    SUCCESS = 0,
    ERROR_UNSPECIFIED = 1,
    ERROR_NO_PERMISSION = 2,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAGuildUpdateDetailsResponse_EResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAGuildUpdateDetailsResponse_EResult> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAGuildUpdateDetailsResponse_EResult::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAGuildUpdateDetailsResponse_EResult::ERROR_UNSPECIFIED),
            2 => ::std::option::Option::Some(CMsgDOTAGuildUpdateDetailsResponse_EResult::ERROR_NO_PERMISSION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAGuildUpdateDetailsResponse_EResult] = &[
            CMsgDOTAGuildUpdateDetailsResponse_EResult::SUCCESS,
            CMsgDOTAGuildUpdateDetailsResponse_EResult::ERROR_UNSPECIFIED,
            CMsgDOTAGuildUpdateDetailsResponse_EResult::ERROR_NO_PERMISSION,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAGuildUpdateDetailsResponse_EResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAGuildUpdateDetailsResponse_EResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAGuildUpdateDetailsResponse_EResult {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildUpdateDetailsResponse_EResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
    // message fields
    party_id: ::std::option::Option<u64>,
    guild_id: ::std::option::Option<u32>,
    member_account_ids: ::std::vec::Vec<u32>,
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {}

impl CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
    pub fn new() -> CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGCToGCUpdateOpenGuildPartyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGCToGCUpdateOpenGuildPartyRequest,
        };
        unsafe {
            instance.get(CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::new)
        }
    }

    // optional uint64 party_id = 1;

    pub fn clear_party_id(&mut self) {
        self.party_id = ::std::option::Option::None;
    }

    pub fn has_party_id(&self) -> bool {
        self.party_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_id(&mut self, v: u64) {
        self.party_id = ::std::option::Option::Some(v);
    }

    pub fn get_party_id(&self) -> u64 {
        self.party_id.unwrap_or(0)
    }

    fn get_party_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.party_id
    }

    fn mut_party_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.party_id
    }

    // optional uint32 guild_id = 2;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // repeated uint32 member_account_ids = 3;

    pub fn clear_member_account_ids(&mut self) {
        self.member_account_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_member_account_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.member_account_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_member_account_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.member_account_ids
    }

    // Take field
    pub fn take_member_account_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.member_account_ids, ::std::vec::Vec::new())
    }

    pub fn get_member_account_ids(&self) -> &[u32] {
        &self.member_account_ids
    }

    fn get_member_account_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.member_account_ids
    }

    fn mut_member_account_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.member_account_ids
    }

    // optional string description = 4;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.description
    }
}

impl ::protobuf::Message for CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
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
                    self.party_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.member_account_ids)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
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
        if let Some(v) = self.party_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.member_account_ids {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.party_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.guild_id {
            os.write_uint32(2, v)?;
        };
        for v in &self.member_account_ids {
            os.write_uint32(3, *v)?;
        };
        if let Some(v) = self.description.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
    fn new() -> CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
        CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGCToGCUpdateOpenGuildPartyRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "party_id",
                    CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::get_party_id_for_reflect,
                    CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::mut_party_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::get_guild_id_for_reflect,
                    CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "member_account_ids",
                    CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::get_member_account_ids_for_reflect,
                    CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::mut_member_account_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::get_description_for_reflect,
                    CMsgDOTAGCToGCUpdateOpenGuildPartyRequest::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGCToGCUpdateOpenGuildPartyRequest>(
                    "CMsgDOTAGCToGCUpdateOpenGuildPartyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
    fn clear(&mut self) {
        self.clear_party_id();
        self.clear_guild_id();
        self.clear_member_account_ids();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGCToGCUpdateOpenGuildPartyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
    // message fields
    maintain_association: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {}

impl CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
    pub fn new() -> CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGCToGCUpdateOpenGuildPartyResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGCToGCUpdateOpenGuildPartyResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGCToGCUpdateOpenGuildPartyResponse::new)
        }
    }

    // optional bool maintain_association = 1;

    pub fn clear_maintain_association(&mut self) {
        self.maintain_association = ::std::option::Option::None;
    }

    pub fn has_maintain_association(&self) -> bool {
        self.maintain_association.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maintain_association(&mut self, v: bool) {
        self.maintain_association = ::std::option::Option::Some(v);
    }

    pub fn get_maintain_association(&self) -> bool {
        self.maintain_association.unwrap_or(false)
    }

    fn get_maintain_association_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.maintain_association
    }

    fn mut_maintain_association_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.maintain_association
    }
}

impl ::protobuf::Message for CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
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
                    self.maintain_association = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.maintain_association {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.maintain_association {
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

impl ::protobuf::MessageStatic for CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
    fn new() -> CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
        CMsgDOTAGCToGCUpdateOpenGuildPartyResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGCToGCUpdateOpenGuildPartyResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "maintain_association",
                    CMsgDOTAGCToGCUpdateOpenGuildPartyResponse::get_maintain_association_for_reflect,
                    CMsgDOTAGCToGCUpdateOpenGuildPartyResponse::mut_maintain_association_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGCToGCUpdateOpenGuildPartyResponse>(
                    "CMsgDOTAGCToGCUpdateOpenGuildPartyResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
    fn clear(&mut self) {
        self.clear_maintain_association();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGCToGCUpdateOpenGuildPartyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
    // message fields
    party_id: ::std::option::Option<u64>,
    guild_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {}

impl CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
    pub fn new() -> CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGCToGCDestroyOpenGuildPartyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGCToGCDestroyOpenGuildPartyRequest,
        };
        unsafe {
            instance.get(CMsgDOTAGCToGCDestroyOpenGuildPartyRequest::new)
        }
    }

    // optional uint64 party_id = 1;

    pub fn clear_party_id(&mut self) {
        self.party_id = ::std::option::Option::None;
    }

    pub fn has_party_id(&self) -> bool {
        self.party_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_id(&mut self, v: u64) {
        self.party_id = ::std::option::Option::Some(v);
    }

    pub fn get_party_id(&self) -> u64 {
        self.party_id.unwrap_or(0)
    }

    fn get_party_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.party_id
    }

    fn mut_party_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.party_id
    }

    // optional uint32 guild_id = 2;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }
}

impl ::protobuf::Message for CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
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
                    self.party_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.guild_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.party_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.party_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.guild_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
    fn new() -> CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
        CMsgDOTAGCToGCDestroyOpenGuildPartyRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGCToGCDestroyOpenGuildPartyRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "party_id",
                    CMsgDOTAGCToGCDestroyOpenGuildPartyRequest::get_party_id_for_reflect,
                    CMsgDOTAGCToGCDestroyOpenGuildPartyRequest::mut_party_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGCToGCDestroyOpenGuildPartyRequest::get_guild_id_for_reflect,
                    CMsgDOTAGCToGCDestroyOpenGuildPartyRequest::mut_guild_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGCToGCDestroyOpenGuildPartyRequest>(
                    "CMsgDOTAGCToGCDestroyOpenGuildPartyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
    fn clear(&mut self) {
        self.clear_party_id();
        self.clear_guild_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGCToGCDestroyOpenGuildPartyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {}

impl CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
    pub fn new() -> CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGCToGCDestroyOpenGuildPartyResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGCToGCDestroyOpenGuildPartyResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGCToGCDestroyOpenGuildPartyResponse::new)
        }
    }
}

impl ::protobuf::Message for CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
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

impl ::protobuf::MessageStatic for CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
    fn new() -> CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
        CMsgDOTAGCToGCDestroyOpenGuildPartyResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGCToGCDestroyOpenGuildPartyResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGCToGCDestroyOpenGuildPartyResponse>(
                    "CMsgDOTAGCToGCDestroyOpenGuildPartyResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGCToGCDestroyOpenGuildPartyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAPartySetOpenGuildRequest {
    // message fields
    guild_id: ::std::option::Option<u32>,
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAPartySetOpenGuildRequest {}

impl CMsgDOTAPartySetOpenGuildRequest {
    pub fn new() -> CMsgDOTAPartySetOpenGuildRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAPartySetOpenGuildRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAPartySetOpenGuildRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAPartySetOpenGuildRequest,
        };
        unsafe {
            instance.get(CMsgDOTAPartySetOpenGuildRequest::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.description
    }
}

impl ::protobuf::Message for CMsgDOTAPartySetOpenGuildRequest {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.description.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgDOTAPartySetOpenGuildRequest {
    fn new() -> CMsgDOTAPartySetOpenGuildRequest {
        CMsgDOTAPartySetOpenGuildRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAPartySetOpenGuildRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAPartySetOpenGuildRequest::get_guild_id_for_reflect,
                    CMsgDOTAPartySetOpenGuildRequest::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    CMsgDOTAPartySetOpenGuildRequest::get_description_for_reflect,
                    CMsgDOTAPartySetOpenGuildRequest::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAPartySetOpenGuildRequest>(
                    "CMsgDOTAPartySetOpenGuildRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAPartySetOpenGuildRequest {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAPartySetOpenGuildRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAPartySetOpenGuildRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAPartySetOpenGuildResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAPartySetOpenGuildResponse_EResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAPartySetOpenGuildResponse {}

impl CMsgDOTAPartySetOpenGuildResponse {
    pub fn new() -> CMsgDOTAPartySetOpenGuildResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAPartySetOpenGuildResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAPartySetOpenGuildResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAPartySetOpenGuildResponse,
        };
        unsafe {
            instance.get(CMsgDOTAPartySetOpenGuildResponse::new)
        }
    }

    // optional .dota.CMsgDOTAPartySetOpenGuildResponse.EResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAPartySetOpenGuildResponse_EResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAPartySetOpenGuildResponse_EResult {
        self.result.unwrap_or(CMsgDOTAPartySetOpenGuildResponse_EResult::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAPartySetOpenGuildResponse_EResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAPartySetOpenGuildResponse_EResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAPartySetOpenGuildResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTAPartySetOpenGuildResponse {
    fn new() -> CMsgDOTAPartySetOpenGuildResponse {
        CMsgDOTAPartySetOpenGuildResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAPartySetOpenGuildResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAPartySetOpenGuildResponse_EResult>>(
                    "result",
                    CMsgDOTAPartySetOpenGuildResponse::get_result_for_reflect,
                    CMsgDOTAPartySetOpenGuildResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAPartySetOpenGuildResponse>(
                    "CMsgDOTAPartySetOpenGuildResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAPartySetOpenGuildResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAPartySetOpenGuildResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAPartySetOpenGuildResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAPartySetOpenGuildResponse_EResult {
    SUCCESS = 0,
    ERROR_UNSPECIFIED = 1,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAPartySetOpenGuildResponse_EResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAPartySetOpenGuildResponse_EResult> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAPartySetOpenGuildResponse_EResult::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAPartySetOpenGuildResponse_EResult::ERROR_UNSPECIFIED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAPartySetOpenGuildResponse_EResult] = &[
            CMsgDOTAPartySetOpenGuildResponse_EResult::SUCCESS,
            CMsgDOTAPartySetOpenGuildResponse_EResult::ERROR_UNSPECIFIED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAPartySetOpenGuildResponse_EResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAPartySetOpenGuildResponse_EResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAPartySetOpenGuildResponse_EResult {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAPartySetOpenGuildResponse_EResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAJoinOpenGuildPartyRequest {
    // message fields
    party_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAJoinOpenGuildPartyRequest {}

impl CMsgDOTAJoinOpenGuildPartyRequest {
    pub fn new() -> CMsgDOTAJoinOpenGuildPartyRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAJoinOpenGuildPartyRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAJoinOpenGuildPartyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAJoinOpenGuildPartyRequest,
        };
        unsafe {
            instance.get(CMsgDOTAJoinOpenGuildPartyRequest::new)
        }
    }

    // optional uint64 party_id = 1;

    pub fn clear_party_id(&mut self) {
        self.party_id = ::std::option::Option::None;
    }

    pub fn has_party_id(&self) -> bool {
        self.party_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_id(&mut self, v: u64) {
        self.party_id = ::std::option::Option::Some(v);
    }

    pub fn get_party_id(&self) -> u64 {
        self.party_id.unwrap_or(0)
    }

    fn get_party_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.party_id
    }

    fn mut_party_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.party_id
    }
}

impl ::protobuf::Message for CMsgDOTAJoinOpenGuildPartyRequest {
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
                    self.party_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.party_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.party_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAJoinOpenGuildPartyRequest {
    fn new() -> CMsgDOTAJoinOpenGuildPartyRequest {
        CMsgDOTAJoinOpenGuildPartyRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAJoinOpenGuildPartyRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "party_id",
                    CMsgDOTAJoinOpenGuildPartyRequest::get_party_id_for_reflect,
                    CMsgDOTAJoinOpenGuildPartyRequest::mut_party_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAJoinOpenGuildPartyRequest>(
                    "CMsgDOTAJoinOpenGuildPartyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAJoinOpenGuildPartyRequest {
    fn clear(&mut self) {
        self.clear_party_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAJoinOpenGuildPartyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAJoinOpenGuildPartyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAJoinOpenGuildPartyResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAJoinOpenGuildPartyResponse_EResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAJoinOpenGuildPartyResponse {}

impl CMsgDOTAJoinOpenGuildPartyResponse {
    pub fn new() -> CMsgDOTAJoinOpenGuildPartyResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAJoinOpenGuildPartyResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAJoinOpenGuildPartyResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAJoinOpenGuildPartyResponse,
        };
        unsafe {
            instance.get(CMsgDOTAJoinOpenGuildPartyResponse::new)
        }
    }

    // optional .dota.CMsgDOTAJoinOpenGuildPartyResponse.EResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAJoinOpenGuildPartyResponse_EResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAJoinOpenGuildPartyResponse_EResult {
        self.result.unwrap_or(CMsgDOTAJoinOpenGuildPartyResponse_EResult::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAJoinOpenGuildPartyResponse_EResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAJoinOpenGuildPartyResponse_EResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAJoinOpenGuildPartyResponse {
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
                    self.result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTAJoinOpenGuildPartyResponse {
    fn new() -> CMsgDOTAJoinOpenGuildPartyResponse {
        CMsgDOTAJoinOpenGuildPartyResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAJoinOpenGuildPartyResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAJoinOpenGuildPartyResponse_EResult>>(
                    "result",
                    CMsgDOTAJoinOpenGuildPartyResponse::get_result_for_reflect,
                    CMsgDOTAJoinOpenGuildPartyResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAJoinOpenGuildPartyResponse>(
                    "CMsgDOTAJoinOpenGuildPartyResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAJoinOpenGuildPartyResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAJoinOpenGuildPartyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAJoinOpenGuildPartyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAJoinOpenGuildPartyResponse_EResult {
    SUCCESS = 0,
    ERROR_UNSPECIFIED = 1,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAJoinOpenGuildPartyResponse_EResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAJoinOpenGuildPartyResponse_EResult> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAJoinOpenGuildPartyResponse_EResult::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAJoinOpenGuildPartyResponse_EResult::ERROR_UNSPECIFIED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAJoinOpenGuildPartyResponse_EResult] = &[
            CMsgDOTAJoinOpenGuildPartyResponse_EResult::SUCCESS,
            CMsgDOTAJoinOpenGuildPartyResponse_EResult::ERROR_UNSPECIFIED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAJoinOpenGuildPartyResponse_EResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAJoinOpenGuildPartyResponse_EResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAJoinOpenGuildPartyResponse_EResult {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAJoinOpenGuildPartyResponse_EResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildOpenPartyRefresh {
    // message fields
    guild_id: ::std::option::Option<u32>,
    open_parties: ::protobuf::RepeatedField<CMsgDOTAGuildOpenPartyRefresh_OpenParty>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildOpenPartyRefresh {}

impl CMsgDOTAGuildOpenPartyRefresh {
    pub fn new() -> CMsgDOTAGuildOpenPartyRefresh {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildOpenPartyRefresh {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildOpenPartyRefresh> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildOpenPartyRefresh,
        };
        unsafe {
            instance.get(CMsgDOTAGuildOpenPartyRefresh::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // repeated .dota.CMsgDOTAGuildOpenPartyRefresh.OpenParty open_parties = 2;

    pub fn clear_open_parties(&mut self) {
        self.open_parties.clear();
    }

    // Param is passed by value, moved
    pub fn set_open_parties(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAGuildOpenPartyRefresh_OpenParty>) {
        self.open_parties = v;
    }

    // Mutable pointer to the field.
    pub fn mut_open_parties(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAGuildOpenPartyRefresh_OpenParty> {
        &mut self.open_parties
    }

    // Take field
    pub fn take_open_parties(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAGuildOpenPartyRefresh_OpenParty> {
        ::std::mem::replace(&mut self.open_parties, ::protobuf::RepeatedField::new())
    }

    pub fn get_open_parties(&self) -> &[CMsgDOTAGuildOpenPartyRefresh_OpenParty] {
        &self.open_parties
    }

    fn get_open_parties_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAGuildOpenPartyRefresh_OpenParty> {
        &self.open_parties
    }

    fn mut_open_parties_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAGuildOpenPartyRefresh_OpenParty> {
        &mut self.open_parties
    }
}

impl ::protobuf::Message for CMsgDOTAGuildOpenPartyRefresh {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.open_parties)?;
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.open_parties {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        for v in &self.open_parties {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildOpenPartyRefresh {
    fn new() -> CMsgDOTAGuildOpenPartyRefresh {
        CMsgDOTAGuildOpenPartyRefresh::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildOpenPartyRefresh>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildOpenPartyRefresh::get_guild_id_for_reflect,
                    CMsgDOTAGuildOpenPartyRefresh::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAGuildOpenPartyRefresh_OpenParty>>(
                    "open_parties",
                    CMsgDOTAGuildOpenPartyRefresh::get_open_parties_for_reflect,
                    CMsgDOTAGuildOpenPartyRefresh::mut_open_parties_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildOpenPartyRefresh>(
                    "CMsgDOTAGuildOpenPartyRefresh",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildOpenPartyRefresh {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_open_parties();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildOpenPartyRefresh {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildOpenPartyRefresh {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildOpenPartyRefresh_OpenParty {
    // message fields
    party_id: ::std::option::Option<u64>,
    member_account_ids: ::std::vec::Vec<u32>,
    time_created: ::std::option::Option<u32>,
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildOpenPartyRefresh_OpenParty {}

impl CMsgDOTAGuildOpenPartyRefresh_OpenParty {
    pub fn new() -> CMsgDOTAGuildOpenPartyRefresh_OpenParty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildOpenPartyRefresh_OpenParty {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildOpenPartyRefresh_OpenParty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildOpenPartyRefresh_OpenParty,
        };
        unsafe {
            instance.get(CMsgDOTAGuildOpenPartyRefresh_OpenParty::new)
        }
    }

    // optional uint64 party_id = 1;

    pub fn clear_party_id(&mut self) {
        self.party_id = ::std::option::Option::None;
    }

    pub fn has_party_id(&self) -> bool {
        self.party_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_id(&mut self, v: u64) {
        self.party_id = ::std::option::Option::Some(v);
    }

    pub fn get_party_id(&self) -> u64 {
        self.party_id.unwrap_or(0)
    }

    fn get_party_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.party_id
    }

    fn mut_party_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.party_id
    }

    // repeated uint32 member_account_ids = 2;

    pub fn clear_member_account_ids(&mut self) {
        self.member_account_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_member_account_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.member_account_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_member_account_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.member_account_ids
    }

    // Take field
    pub fn take_member_account_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.member_account_ids, ::std::vec::Vec::new())
    }

    pub fn get_member_account_ids(&self) -> &[u32] {
        &self.member_account_ids
    }

    fn get_member_account_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.member_account_ids
    }

    fn mut_member_account_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.member_account_ids
    }

    // optional uint32 time_created = 3;

    pub fn clear_time_created(&mut self) {
        self.time_created = ::std::option::Option::None;
    }

    pub fn has_time_created(&self) -> bool {
        self.time_created.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_created(&mut self, v: u32) {
        self.time_created = ::std::option::Option::Some(v);
    }

    pub fn get_time_created(&self) -> u32 {
        self.time_created.unwrap_or(0)
    }

    fn get_time_created_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_created
    }

    fn mut_time_created_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_created
    }

    // optional string description = 4;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.description
    }
}

impl ::protobuf::Message for CMsgDOTAGuildOpenPartyRefresh_OpenParty {
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
                    self.party_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.member_account_ids)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_created = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
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
        if let Some(v) = self.party_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.member_account_ids {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.time_created {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.party_id {
            os.write_uint64(1, v)?;
        };
        for v in &self.member_account_ids {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.time_created {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.description.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildOpenPartyRefresh_OpenParty {
    fn new() -> CMsgDOTAGuildOpenPartyRefresh_OpenParty {
        CMsgDOTAGuildOpenPartyRefresh_OpenParty::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildOpenPartyRefresh_OpenParty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "party_id",
                    CMsgDOTAGuildOpenPartyRefresh_OpenParty::get_party_id_for_reflect,
                    CMsgDOTAGuildOpenPartyRefresh_OpenParty::mut_party_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "member_account_ids",
                    CMsgDOTAGuildOpenPartyRefresh_OpenParty::get_member_account_ids_for_reflect,
                    CMsgDOTAGuildOpenPartyRefresh_OpenParty::mut_member_account_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_created",
                    CMsgDOTAGuildOpenPartyRefresh_OpenParty::get_time_created_for_reflect,
                    CMsgDOTAGuildOpenPartyRefresh_OpenParty::mut_time_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    CMsgDOTAGuildOpenPartyRefresh_OpenParty::get_description_for_reflect,
                    CMsgDOTAGuildOpenPartyRefresh_OpenParty::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildOpenPartyRefresh_OpenParty>(
                    "CMsgDOTAGuildOpenPartyRefresh_OpenParty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildOpenPartyRefresh_OpenParty {
    fn clear(&mut self) {
        self.clear_party_id();
        self.clear_member_account_ids();
        self.clear_time_created();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildOpenPartyRefresh_OpenParty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildOpenPartyRefresh_OpenParty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTARequestGuildData {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTARequestGuildData {}

impl CMsgDOTARequestGuildData {
    pub fn new() -> CMsgDOTARequestGuildData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTARequestGuildData {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTARequestGuildData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTARequestGuildData,
        };
        unsafe {
            instance.get(CMsgDOTARequestGuildData::new)
        }
    }
}

impl ::protobuf::Message for CMsgDOTARequestGuildData {
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

impl ::protobuf::MessageStatic for CMsgDOTARequestGuildData {
    fn new() -> CMsgDOTARequestGuildData {
        CMsgDOTARequestGuildData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTARequestGuildData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTARequestGuildData>(
                    "CMsgDOTARequestGuildData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTARequestGuildData {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTARequestGuildData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTARequestGuildData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildInviteData {
    // message fields
    invited_to_guild: ::std::option::Option<bool>,
    guild_id: ::std::option::Option<u32>,
    guild_name: ::protobuf::SingularField<::std::string::String>,
    guild_tag: ::protobuf::SingularField<::std::string::String>,
    logo: ::std::option::Option<u64>,
    inviter: ::std::option::Option<u32>,
    inviter_name: ::protobuf::SingularField<::std::string::String>,
    member_count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildInviteData {}

impl CMsgDOTAGuildInviteData {
    pub fn new() -> CMsgDOTAGuildInviteData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildInviteData {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildInviteData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildInviteData,
        };
        unsafe {
            instance.get(CMsgDOTAGuildInviteData::new)
        }
    }

    // optional bool invited_to_guild = 1;

    pub fn clear_invited_to_guild(&mut self) {
        self.invited_to_guild = ::std::option::Option::None;
    }

    pub fn has_invited_to_guild(&self) -> bool {
        self.invited_to_guild.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invited_to_guild(&mut self, v: bool) {
        self.invited_to_guild = ::std::option::Option::Some(v);
    }

    pub fn get_invited_to_guild(&self) -> bool {
        self.invited_to_guild.unwrap_or(false)
    }

    fn get_invited_to_guild_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.invited_to_guild
    }

    fn mut_invited_to_guild_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.invited_to_guild
    }

    // optional uint32 guild_id = 2;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional string guild_name = 3;

    pub fn clear_guild_name(&mut self) {
        self.guild_name.clear();
    }

    pub fn has_guild_name(&self) -> bool {
        self.guild_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_name(&mut self, v: ::std::string::String) {
        self.guild_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guild_name(&mut self) -> &mut ::std::string::String {
        if self.guild_name.is_none() {
            self.guild_name.set_default();
        };
        self.guild_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_guild_name(&mut self) -> ::std::string::String {
        self.guild_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_guild_name(&self) -> &str {
        match self.guild_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_guild_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.guild_name
    }

    fn mut_guild_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.guild_name
    }

    // optional string guild_tag = 4;

    pub fn clear_guild_tag(&mut self) {
        self.guild_tag.clear();
    }

    pub fn has_guild_tag(&self) -> bool {
        self.guild_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_tag(&mut self, v: ::std::string::String) {
        self.guild_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guild_tag(&mut self) -> &mut ::std::string::String {
        if self.guild_tag.is_none() {
            self.guild_tag.set_default();
        };
        self.guild_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_guild_tag(&mut self) -> ::std::string::String {
        self.guild_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_guild_tag(&self) -> &str {
        match self.guild_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_guild_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.guild_tag
    }

    fn mut_guild_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.guild_tag
    }

    // optional uint64 logo = 5;

    pub fn clear_logo(&mut self) {
        self.logo = ::std::option::Option::None;
    }

    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo(&mut self, v: u64) {
        self.logo = ::std::option::Option::Some(v);
    }

    pub fn get_logo(&self) -> u64 {
        self.logo.unwrap_or(0)
    }

    fn get_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.logo
    }

    fn mut_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.logo
    }

    // optional uint32 inviter = 6;

    pub fn clear_inviter(&mut self) {
        self.inviter = ::std::option::Option::None;
    }

    pub fn has_inviter(&self) -> bool {
        self.inviter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inviter(&mut self, v: u32) {
        self.inviter = ::std::option::Option::Some(v);
    }

    pub fn get_inviter(&self) -> u32 {
        self.inviter.unwrap_or(0)
    }

    fn get_inviter_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.inviter
    }

    fn mut_inviter_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.inviter
    }

    // optional string inviter_name = 7;

    pub fn clear_inviter_name(&mut self) {
        self.inviter_name.clear();
    }

    pub fn has_inviter_name(&self) -> bool {
        self.inviter_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inviter_name(&mut self, v: ::std::string::String) {
        self.inviter_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inviter_name(&mut self) -> &mut ::std::string::String {
        if self.inviter_name.is_none() {
            self.inviter_name.set_default();
        };
        self.inviter_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_inviter_name(&mut self) -> ::std::string::String {
        self.inviter_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_inviter_name(&self) -> &str {
        match self.inviter_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_inviter_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.inviter_name
    }

    fn mut_inviter_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.inviter_name
    }

    // optional uint32 member_count = 8;

    pub fn clear_member_count(&mut self) {
        self.member_count = ::std::option::Option::None;
    }

    pub fn has_member_count(&self) -> bool {
        self.member_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member_count(&mut self, v: u32) {
        self.member_count = ::std::option::Option::Some(v);
    }

    pub fn get_member_count(&self) -> u32 {
        self.member_count.unwrap_or(0)
    }

    fn get_member_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.member_count
    }

    fn mut_member_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.member_count
    }
}

impl ::protobuf::Message for CMsgDOTAGuildInviteData {
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
                    self.invited_to_guild = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.guild_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.guild_tag)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.logo = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.inviter = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.inviter_name)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.member_count = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.invited_to_guild {
            my_size += 2;
        };
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.guild_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.guild_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.inviter {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.inviter_name.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.member_count {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.invited_to_guild {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.guild_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.guild_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.guild_tag.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.logo {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.inviter {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.inviter_name.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.member_count {
            os.write_uint32(8, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildInviteData {
    fn new() -> CMsgDOTAGuildInviteData {
        CMsgDOTAGuildInviteData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildInviteData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "invited_to_guild",
                    CMsgDOTAGuildInviteData::get_invited_to_guild_for_reflect,
                    CMsgDOTAGuildInviteData::mut_invited_to_guild_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildInviteData::get_guild_id_for_reflect,
                    CMsgDOTAGuildInviteData::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "guild_name",
                    CMsgDOTAGuildInviteData::get_guild_name_for_reflect,
                    CMsgDOTAGuildInviteData::mut_guild_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "guild_tag",
                    CMsgDOTAGuildInviteData::get_guild_tag_for_reflect,
                    CMsgDOTAGuildInviteData::mut_guild_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTAGuildInviteData::get_logo_for_reflect,
                    CMsgDOTAGuildInviteData::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "inviter",
                    CMsgDOTAGuildInviteData::get_inviter_for_reflect,
                    CMsgDOTAGuildInviteData::mut_inviter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "inviter_name",
                    CMsgDOTAGuildInviteData::get_inviter_name_for_reflect,
                    CMsgDOTAGuildInviteData::mut_inviter_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "member_count",
                    CMsgDOTAGuildInviteData::get_member_count_for_reflect,
                    CMsgDOTAGuildInviteData::mut_member_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildInviteData>(
                    "CMsgDOTAGuildInviteData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildInviteData {
    fn clear(&mut self) {
        self.clear_invited_to_guild();
        self.clear_guild_id();
        self.clear_guild_name();
        self.clear_guild_tag();
        self.clear_logo();
        self.clear_inviter();
        self.clear_inviter_name();
        self.clear_member_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildInviteData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildInviteData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildUpdateMessage {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    guild_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildUpdateMessage {}

impl CMsgDOTAGuildUpdateMessage {
    pub fn new() -> CMsgDOTAGuildUpdateMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildUpdateMessage {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildUpdateMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildUpdateMessage,
        };
        unsafe {
            instance.get(CMsgDOTAGuildUpdateMessage::new)
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // optional uint32 guild_id = 2;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }
}

impl ::protobuf::Message for CMsgDOTAGuildUpdateMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.guild_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.guild_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildUpdateMessage {
    fn new() -> CMsgDOTAGuildUpdateMessage {
        CMsgDOTAGuildUpdateMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildUpdateMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CMsgDOTAGuildUpdateMessage::get_message_for_reflect,
                    CMsgDOTAGuildUpdateMessage::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildUpdateMessage::get_guild_id_for_reflect,
                    CMsgDOTAGuildUpdateMessage::mut_guild_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildUpdateMessage>(
                    "CMsgDOTAGuildUpdateMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildUpdateMessage {
    fn clear(&mut self) {
        self.clear_message();
        self.clear_guild_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildUpdateMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildUpdateMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildEditLogoRequest {
    // message fields
    guild_id: ::std::option::Option<u32>,
    logo: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildEditLogoRequest {}

impl CMsgDOTAGuildEditLogoRequest {
    pub fn new() -> CMsgDOTAGuildEditLogoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildEditLogoRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildEditLogoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildEditLogoRequest,
        };
        unsafe {
            instance.get(CMsgDOTAGuildEditLogoRequest::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional uint64 logo = 2;

    pub fn clear_logo(&mut self) {
        self.logo = ::std::option::Option::None;
    }

    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo(&mut self, v: u64) {
        self.logo = ::std::option::Option::Some(v);
    }

    pub fn get_logo(&self) -> u64 {
        self.logo.unwrap_or(0)
    }

    fn get_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.logo
    }

    fn mut_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.logo
    }
}

impl ::protobuf::Message for CMsgDOTAGuildEditLogoRequest {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.logo = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.logo {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildEditLogoRequest {
    fn new() -> CMsgDOTAGuildEditLogoRequest {
        CMsgDOTAGuildEditLogoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildEditLogoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildEditLogoRequest::get_guild_id_for_reflect,
                    CMsgDOTAGuildEditLogoRequest::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTAGuildEditLogoRequest::get_logo_for_reflect,
                    CMsgDOTAGuildEditLogoRequest::mut_logo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildEditLogoRequest>(
                    "CMsgDOTAGuildEditLogoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildEditLogoRequest {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_logo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildEditLogoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildEditLogoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGuildEditLogoResponse {
    // message fields
    guild_id: ::std::option::Option<u32>,
    result: ::std::option::Option<CMsgDOTAGuildEditLogoResponse_EResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGuildEditLogoResponse {}

impl CMsgDOTAGuildEditLogoResponse {
    pub fn new() -> CMsgDOTAGuildEditLogoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGuildEditLogoResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGuildEditLogoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGuildEditLogoResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGuildEditLogoResponse::new)
        }
    }

    // optional uint32 guild_id = 1;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional .dota.CMsgDOTAGuildEditLogoResponse.EResult result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAGuildEditLogoResponse_EResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAGuildEditLogoResponse_EResult {
        self.result.unwrap_or(CMsgDOTAGuildEditLogoResponse_EResult::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAGuildEditLogoResponse_EResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAGuildEditLogoResponse_EResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAGuildEditLogoResponse {
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
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.guild_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.result {
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

impl ::protobuf::MessageStatic for CMsgDOTAGuildEditLogoResponse {
    fn new() -> CMsgDOTAGuildEditLogoResponse {
        CMsgDOTAGuildEditLogoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGuildEditLogoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgDOTAGuildEditLogoResponse::get_guild_id_for_reflect,
                    CMsgDOTAGuildEditLogoResponse::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAGuildEditLogoResponse_EResult>>(
                    "result",
                    CMsgDOTAGuildEditLogoResponse::get_result_for_reflect,
                    CMsgDOTAGuildEditLogoResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGuildEditLogoResponse>(
                    "CMsgDOTAGuildEditLogoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGuildEditLogoResponse {
    fn clear(&mut self) {
        self.clear_guild_id();
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGuildEditLogoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildEditLogoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAGuildEditLogoResponse_EResult {
    SUCCESS = 0,
    NO_PERMISSION = 1,
    LOGO_UPLOAD_FAILED = 2,
    UNSPECIFIED_ERROR = 3,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAGuildEditLogoResponse_EResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAGuildEditLogoResponse_EResult> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAGuildEditLogoResponse_EResult::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAGuildEditLogoResponse_EResult::NO_PERMISSION),
            2 => ::std::option::Option::Some(CMsgDOTAGuildEditLogoResponse_EResult::LOGO_UPLOAD_FAILED),
            3 => ::std::option::Option::Some(CMsgDOTAGuildEditLogoResponse_EResult::UNSPECIFIED_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAGuildEditLogoResponse_EResult] = &[
            CMsgDOTAGuildEditLogoResponse_EResult::SUCCESS,
            CMsgDOTAGuildEditLogoResponse_EResult::NO_PERMISSION,
            CMsgDOTAGuildEditLogoResponse_EResult::LOGO_UPLOAD_FAILED,
            CMsgDOTAGuildEditLogoResponse_EResult::UNSPECIFIED_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAGuildEditLogoResponse_EResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAGuildEditLogoResponse_EResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAGuildEditLogoResponse_EResult {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGuildEditLogoResponse_EResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x22, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x22, 0xfd, 0x04, 0x0a, 0x10, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x53, 0x44, 0x4f, 0x12,
    0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x10,
    0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x74, 0x61, 0x67,
    0x12, 0x21, 0x0a, 0x0c, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x74, 0x69, 0x6d, 0x65, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x64, 0x69, 0x73, 0x62,
    0x61, 0x6e, 0x64, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x74, 0x69, 0x6d,
    0x65, 0x44, 0x69, 0x73, 0x62, 0x61, 0x6e, 0x64, 0x65, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x6f,
    0x67, 0x6f, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x12, 0x1b,
    0x0a, 0x09, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x08, 0x62, 0x61, 0x73, 0x65, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x1f, 0x0a, 0x0b, 0x62,
    0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x08, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x0a, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x37, 0x0a, 0x07,
    0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x09, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69,
    0x6c, 0x64, 0x53, 0x44, 0x4f, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x73, 0x12, 0x43, 0x0a, 0x0b, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x53,
    0x44, 0x4f, 0x2e, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x69,
    0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x61, 0x6c, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x69, 0x6e, 0x63, 0x72, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x1a, 0x5c, 0x0a, 0x06, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12,
    0x1f, 0x0a, 0x0b, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x6a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x4a, 0x6f, 0x69, 0x6e, 0x65, 0x64,
    0x12, 0x12, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04,
    0x72, 0x6f, 0x6c, 0x65, 0x1a, 0x74, 0x0a, 0x0a, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49,
    0x64, 0x12, 0x1b, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x74, 0x69, 0x6d, 0x65, 0x53, 0x65, 0x6e, 0x74, 0x12, 0x2a,
    0x0a, 0x11, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x5f, 0x73, 0x65, 0x6e,
    0x64, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x49, 0x64, 0x53, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x22, 0x82, 0x03, 0x0a, 0x15, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x41, 0x75, 0x64, 0x69,
    0x74, 0x53, 0x44, 0x4f, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12,
    0x3b, 0x0a, 0x07, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x47, 0x75, 0x69, 0x6c, 0x64, 0x41, 0x75, 0x64, 0x69, 0x74, 0x53, 0x44, 0x4f, 0x2e, 0x45, 0x6e,
    0x74, 0x72, 0x79, 0x52, 0x07, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x1a, 0x90, 0x02, 0x0a,
    0x05, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x30, 0x0a,
    0x14, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x5f, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x6f, 0x72, 0x12,
    0x2a, 0x0a, 0x11, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x5f, 0x74, 0x61,
    0x72, 0x67, 0x65, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x49, 0x64, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x12, 0x28, 0x0a, 0x10, 0x72,
    0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x61, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65,
    0x44, 0x61, 0x74, 0x61, 0x41, 0x12, 0x28, 0x0a, 0x10, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e,
    0x63, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x62, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0e, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61, 0x42, 0x22,
    0xa0, 0x03, 0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x41, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68,
    0x69, 0x70, 0x73, 0x53, 0x44, 0x4f, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x55, 0x0a, 0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x68, 0x69, 0x70, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70,
    0x73, 0x53, 0x44, 0x4f, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x52,
    0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x73, 0x12, 0x55, 0x0a, 0x0b,
    0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x33, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54,
    0x41, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x4d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x73, 0x53, 0x44, 0x4f, 0x2e, 0x49, 0x6e, 0x76, 0x69,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x1a, 0x3b, 0x0a, 0x0a, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69,
    0x70, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04,
    0x72, 0x6f, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x72, 0x6f, 0x6c, 0x65,
    0x1a, 0x70, 0x0a, 0x0a, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x19,
    0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x74, 0x69, 0x6d,
    0x65, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x74, 0x69,
    0x6d, 0x65, 0x53, 0x65, 0x6e, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x5f, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x53, 0x65, 0x6e, 0x64,
    0x65, 0x72, 0x22, 0x94, 0x01, 0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47,
    0x75, 0x69, 0x6c, 0x64, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x03, 0x74, 0x61, 0x67, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x62,
    0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08,
    0x62, 0x61, 0x73, 0x65, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x61, 0x6e, 0x6e,
    0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x62,
    0x61, 0x6e, 0x6e, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x22, 0xd0, 0x02, 0x0a, 0x1b, 0x43, 0x4d,
    0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69,
    0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69,
    0x6c, 0x64, 0x49, 0x64, 0x12, 0x40, 0x0a, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0e, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x45, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x06,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x22, 0xd3, 0x01, 0x0a, 0x06, 0x45, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44,
    0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x45, 0x4d, 0x50, 0x54, 0x59,
    0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x42, 0x41, 0x44, 0x5f, 0x43,
    0x48, 0x41, 0x52, 0x41, 0x43, 0x54, 0x45, 0x52, 0x53, 0x10, 0x02, 0x12, 0x11, 0x0a, 0x0d, 0x4e,
    0x41, 0x4d, 0x45, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x4c, 0x4f, 0x4e, 0x47, 0x10, 0x03, 0x12, 0x0e,
    0x0a, 0x0a, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x54, 0x41, 0x4b, 0x45, 0x4e, 0x10, 0x04, 0x12, 0x0d,
    0x0a, 0x09, 0x54, 0x41, 0x47, 0x5f, 0x45, 0x4d, 0x50, 0x54, 0x59, 0x10, 0x05, 0x12, 0x16, 0x0a,
    0x12, 0x54, 0x41, 0x47, 0x5f, 0x42, 0x41, 0x44, 0x5f, 0x43, 0x48, 0x41, 0x52, 0x41, 0x43, 0x54,
    0x45, 0x52, 0x53, 0x10, 0x06, 0x12, 0x10, 0x0a, 0x0c, 0x54, 0x41, 0x47, 0x5f, 0x54, 0x4f, 0x4f,
    0x5f, 0x4c, 0x4f, 0x4e, 0x47, 0x10, 0x07, 0x12, 0x1b, 0x0a, 0x17, 0x41, 0x43, 0x43, 0x4f, 0x55,
    0x4e, 0x54, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x47, 0x55, 0x49, 0x4c,
    0x44, 0x53, 0x10, 0x08, 0x12, 0x16, 0x0a, 0x12, 0x4c, 0x4f, 0x47, 0x4f, 0x5f, 0x55, 0x50, 0x4c,
    0x4f, 0x41, 0x44, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x09, 0x22, 0x8c, 0x01, 0x0a,
    0x22, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x53, 0x65,
    0x74, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x2a,
    0x0a, 0x11, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x74, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x74, 0x61,
    0x72, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0a, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x22, 0xa2, 0x02, 0x0a, 0x23,
    0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x53, 0x65, 0x74,
    0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x52, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x31, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x53, 0x65, 0x74, 0x41, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x45,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52,
    0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0xa6, 0x01, 0x0a, 0x07, 0x45, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00,
    0x12, 0x15, 0x0a, 0x11, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43,
    0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x45, 0x52, 0x52, 0x4f, 0x52,
    0x5f, 0x4e, 0x4f, 0x5f, 0x50, 0x45, 0x52, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x02,
    0x12, 0x19, 0x0a, 0x15, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x5f, 0x4f, 0x54, 0x48,
    0x45, 0x52, 0x5f, 0x4c, 0x45, 0x41, 0x44, 0x45, 0x52, 0x10, 0x03, 0x12, 0x21, 0x0a, 0x1d, 0x45,
    0x52, 0x52, 0x4f, 0x52, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x54, 0x4f, 0x4f,
    0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x47, 0x55, 0x49, 0x4c, 0x44, 0x53, 0x10, 0x04, 0x12, 0x20,
    0x0a, 0x1c, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x47, 0x55, 0x49, 0x4c, 0x44, 0x5f, 0x54, 0x4f,
    0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x4d, 0x45, 0x4d, 0x42, 0x45, 0x52, 0x53, 0x10, 0x05,
    0x22, 0x6a, 0x0a, 0x21, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c,
    0x64, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64,
    0x12, 0x2a, 0x0a, 0x11, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x74, 0x61, 0x72,
    0x67, 0x65, 0x74, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x22, 0xf0, 0x02, 0x0a,
    0x22, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x6e,
    0x76, 0x69, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x51, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x30, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x41, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x45, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0xf6, 0x01, 0x0a, 0x07, 0x45, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12,
    0x15, 0x0a, 0x11, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49,
    0x46, 0x49, 0x45, 0x44, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x4e, 0x4f, 0x5f, 0x50, 0x45, 0x52, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x02, 0x12,
    0x21, 0x0a, 0x1d, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54,
    0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x44,
    0x10, 0x03, 0x12, 0x22, 0x0a, 0x1e, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x41, 0x43, 0x43, 0x4f,
    0x55, 0x4e, 0x54, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x49, 0x4e, 0x5f, 0x47,
    0x55, 0x49, 0x4c, 0x44, 0x10, 0x04, 0x12, 0x22, 0x0a, 0x1e, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59,
    0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x53, 0x10, 0x05, 0x12, 0x20, 0x0a, 0x1c, 0x45, 0x52,
    0x52, 0x4f, 0x52, 0x5f, 0x47, 0x55, 0x49, 0x4c, 0x44, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x4d, 0x41,
    0x4e, 0x59, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x53, 0x10, 0x06, 0x12, 0x21, 0x0a, 0x1d,
    0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x54, 0x4f,
    0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x47, 0x55, 0x49, 0x4c, 0x44, 0x53, 0x10, 0x07, 0x22,
    0x69, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64,
    0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x2a,
    0x0a, 0x11, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x74, 0x61, 0x72, 0x67, 0x65,
    0x74, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x22, 0xbd, 0x01, 0x0a, 0x21, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x43, 0x61, 0x6e, 0x63,
    0x65, 0x6c, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x50, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x2f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x47, 0x75, 0x69, 0x6c, 0x64, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x49, 0x6e, 0x76, 0x69, 0x74,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x45, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x22, 0x46, 0x0a, 0x07, 0x45, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a,
    0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12, 0x15, 0x0a, 0x11, 0x45, 0x52,
    0x52, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10,
    0x01, 0x12, 0x17, 0x0a, 0x13, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x4e, 0x4f, 0x5f, 0x50, 0x45,
    0x52, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x02, 0x22, 0x90, 0x01, 0x0a, 0x21, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6c,
    0x6f, 0x67, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x12,
    0x1b, 0x0a, 0x09, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x08, 0x62, 0x61, 0x73, 0x65, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x1f, 0x0a, 0x0b,
    0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x0a, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x22, 0xbf, 0x01,
    0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x51, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x30, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x44,
    0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x45,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52,
    0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x46, 0x0a, 0x07, 0x45, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12,
    0x15, 0x0a, 0x11, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49,
    0x46, 0x49, 0x45, 0x44, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f,
    0x4e, 0x4f, 0x5f, 0x50, 0x45, 0x52, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x02, 0x22,
    0xb1, 0x01, 0x0a, 0x29, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x43, 0x54, 0x6f,
    0x47, 0x43, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4f, 0x70, 0x65, 0x6e, 0x47, 0x75, 0x69, 0x6c,
    0x64, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x19, 0x0a,
    0x08, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x07, 0x70, 0x61, 0x72, 0x74, 0x79, 0x49, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c,
    0x64, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c,
    0x64, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x12, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0d, 0x52,
    0x10, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64,
    0x73, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x22, 0x5f, 0x0a, 0x2a, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47,
    0x43, 0x54, 0x6f, 0x47, 0x43, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4f, 0x70, 0x65, 0x6e, 0x47,
    0x75, 0x69, 0x6c, 0x64, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x31, 0x0a, 0x14, 0x6d, 0x61, 0x69, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x5f, 0x61, 0x73,
    0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x13, 0x6d, 0x61, 0x69, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x41, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x22, 0x62, 0x0a, 0x2a, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x4f, 0x70, 0x65,
    0x6e, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x70, 0x61, 0x72, 0x74, 0x79, 0x49, 0x64, 0x12, 0x19, 0x0a,
    0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x22, 0x2d, 0x0a, 0x2b, 0x43, 0x4d, 0x73, 0x67,
    0x44, 0x4f, 0x54, 0x41, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f,
    0x79, 0x4f, 0x70, 0x65, 0x6e, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x5f, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79, 0x53, 0x65, 0x74, 0x4f, 0x70, 0x65, 0x6e, 0x47,
    0x75, 0x69, 0x6c, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x67,
    0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67,
    0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xa4, 0x01, 0x0a, 0x21, 0x43, 0x4d, 0x73,
    0x67, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79, 0x53, 0x65, 0x74, 0x4f, 0x70, 0x65,
    0x6e, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x50,
    0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2f,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61,
    0x72, 0x74, 0x79, 0x53, 0x65, 0x74, 0x4f, 0x70, 0x65, 0x6e, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x45, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a,
    0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x22, 0x2d, 0x0a, 0x07, 0x45, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53,
    0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12, 0x15, 0x0a, 0x11, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x01, 0x22,
    0x3e, 0x0a, 0x21, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4a, 0x6f, 0x69, 0x6e, 0x4f,
    0x70, 0x65, 0x6e, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x70, 0x61, 0x72, 0x74, 0x79, 0x49, 0x64, 0x22,
    0xa6, 0x01, 0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4a, 0x6f, 0x69, 0x6e,
    0x4f, 0x70, 0x65, 0x6e, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x51, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x30, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4a, 0x6f, 0x69, 0x6e, 0x4f, 0x70, 0x65, 0x6e, 0x47, 0x75,
    0x69, 0x6c, 0x64, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x45, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53,
    0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x2d, 0x0a, 0x07, 0x45, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10,
    0x00, 0x12, 0x15, 0x0a, 0x11, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45,
    0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x01, 0x22, 0xa8, 0x02, 0x0a, 0x1d, 0x43, 0x4d, 0x73,
    0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x4f, 0x70, 0x65, 0x6e, 0x50, 0x61,
    0x72, 0x74, 0x79, 0x52, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75,
    0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75,
    0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x50, 0x0a, 0x0c, 0x6f, 0x70, 0x65, 0x6e, 0x5f, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c, 0x64,
    0x4f, 0x70, 0x65, 0x6e, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68,
    0x2e, 0x4f, 0x70, 0x65, 0x6e, 0x50, 0x61, 0x72, 0x74, 0x79, 0x52, 0x0b, 0x6f, 0x70, 0x65, 0x6e,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x65, 0x73, 0x1a, 0x99, 0x01, 0x0a, 0x09, 0x4f, 0x70, 0x65, 0x6e,
    0x50, 0x61, 0x72, 0x74, 0x79, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x70, 0x61, 0x72, 0x74, 0x79, 0x49, 0x64,
    0x12, 0x2c, 0x0a, 0x12, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x10, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x73, 0x12, 0x21,
    0x0a, 0x0c, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x74, 0x69, 0x6d, 0x65, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x22, 0x1a, 0x0a, 0x18, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x44, 0x61, 0x74, 0x61, 0x22,
    0x8e, 0x02, 0x0a, 0x17, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c,
    0x64, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x44, 0x61, 0x74, 0x61, 0x12, 0x28, 0x0a, 0x10, 0x69,
    0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x5f, 0x74, 0x6f, 0x5f, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x54, 0x6f,
    0x47, 0x75, 0x69, 0x6c, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64,
    0x12, 0x1d, 0x0a, 0x0a, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x4e, 0x61, 0x6d, 0x65, 0x12,
    0x1b, 0x0a, 0x09, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x74, 0x61, 0x67, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x54, 0x61, 0x67, 0x12, 0x12, 0x0a, 0x04,
    0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x6f,
    0x12, 0x18, 0x0a, 0x07, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x07, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x72, 0x12, 0x21, 0x0a, 0x0c, 0x69, 0x6e,
    0x76, 0x69, 0x74, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0b, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a,
    0x0c, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e, 0x74,
    0x22, 0x51, 0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x75, 0x69, 0x6c,
    0x64, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x18,
    0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c,
    0x64, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c,
    0x64, 0x49, 0x64, 0x22, 0x4d, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47,
    0x75, 0x69, 0x6c, 0x64, 0x45, 0x64, 0x69, 0x74, 0x4c, 0x6f, 0x67, 0x6f, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x12,
    0x0a, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f,
    0x67, 0x6f, 0x22, 0xe2, 0x01, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47,
    0x75, 0x69, 0x6c, 0x64, 0x45, 0x64, 0x69, 0x74, 0x4c, 0x6f, 0x67, 0x6f, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12,
    0x4c, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x2b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47,
    0x75, 0x69, 0x6c, 0x64, 0x45, 0x64, 0x69, 0x74, 0x4c, 0x6f, 0x67, 0x6f, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x45, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55,
    0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x58, 0x0a,
    0x07, 0x45, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43,
    0x45, 0x53, 0x53, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x4e, 0x4f, 0x5f, 0x50, 0x45, 0x52, 0x4d,
    0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x12, 0x16, 0x0a, 0x12, 0x4c, 0x4f, 0x47, 0x4f,
    0x5f, 0x55, 0x50, 0x4c, 0x4f, 0x41, 0x44, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x02,
    0x12, 0x15, 0x0a, 0x11, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x5f,
    0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x03, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xb4,
    0x4d, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xff, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x04, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x04, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04,
    0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04,
    0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x04, 0x16, 0x1b,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x01, 0x12, 0x03, 0x05, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x12, 0x03, 0x05, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x05, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x05, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x05,
    0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x20, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00,
    0x03, 0x00, 0x12, 0x04, 0x08, 0x08, 0x0c, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x08, 0x10, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x09, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x09, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x09, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x09, 0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x09, 0x2d, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x0a, 0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x0a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0a, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0a, 0x20, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x0a, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x0b, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x0b, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0b, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x0b, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x0b, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x01, 0x12, 0x04, 0x0e, 0x08, 0x12,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x10, 0x1a, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x10, 0x2f, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x10, 0x18, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x19, 0x1f, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x20, 0x2a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x2d, 0x2e, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x10, 0x10, 0x2e, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x10, 0x19, 0x1f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x20, 0x29, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x12, 0x03, 0x11, 0x10, 0x36, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x11, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x11, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x20, 0x31, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x11, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x14, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x14, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x15, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x15, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x16, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x16, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x16, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x18, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x17, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x17, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x17,
    0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x17, 0x27, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x18, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x18, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x18, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x18, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x18, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x19,
    0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x19, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x19, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x19, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x19, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x03, 0x1a, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x1a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1a, 0x18,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x1a, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x1b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x1b, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12,
    0x03, 0x1b, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x1c, 0x08,
    0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x1c, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x1c, 0x11, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1c, 0x29, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x1c, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x09, 0x12, 0x03, 0x1d, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12,
    0x03, 0x1d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03, 0x1d,
    0x11, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x1d, 0x2d, 0x38,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x1d, 0x3b, 0x3d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x1e, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0a, 0x05, 0x12, 0x03, 0x1e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01,
    0x12, 0x03, 0x1e, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03,
    0x1e, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x1f, 0x08, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x1f, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x1f, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0b, 0x03, 0x12, 0x03, 0x1f, 0x24, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x22, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x22, 0x08, 0x1d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x23, 0x08, 0x2b, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x23, 0x10, 0x15, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x20, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x25, 0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x25, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x25, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x20, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x26, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x26, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x20, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x27, 0x10, 0x39, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x27, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x27, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x27, 0x20, 0x34, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x27, 0x37, 0x38, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x28, 0x10, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x28, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x28, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x28, 0x20, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x28, 0x34, 0x35, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x29, 0x10, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x29, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x29, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x29, 0x20, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x29, 0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x2a, 0x10, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x2a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x05,
    0x12, 0x03, 0x2a, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x2a, 0x20, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x2a, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2d,
    0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x2e, 0x11, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x2d,
    0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x37, 0x38, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x31, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x31, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x00, 0x12,
    0x04, 0x32, 0x08, 0x35, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x00, 0x01, 0x12, 0x03,
    0x32, 0x10, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x33,
    0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x33,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x33,
    0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33,
    0x20, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33,
    0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x34, 0x10,
    0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x34, 0x10,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x34, 0x19,
    0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x34, 0x20,
    0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x34, 0x27,
    0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x04, 0x37, 0x08, 0x3b, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x01, 0x01, 0x12, 0x03, 0x37, 0x10, 0x1a, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x38, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x20, 0x28, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x02, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x39, 0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x39, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x39, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x20, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x02, 0x03, 0x01, 0x02, 0x02, 0x12, 0x03, 0x3a, 0x10, 0x36, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3a, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x20, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x3d, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x3d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x18,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x4f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x3e, 0x11, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x3e, 0x3f, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x3e, 0x4d, 0x4e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x3f, 0x08,
    0x4f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x3f, 0x11, 0x3e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x3f, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3f, 0x4d, 0x4e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x42, 0x00, 0x48, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x42, 0x08,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x43, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x43, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x43, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03,
    0x44, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x44, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x44, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x18, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x02, 0x12, 0x03, 0x45, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x45, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x45,
    0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x45, 0x1f, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x46, 0x08, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x46, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x46, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x46, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x46, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x47,
    0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x47, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x47, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x47, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x4a, 0x00, 0x5a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4a,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x04, 0x4b, 0x08, 0x56, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x4b, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x10, 0x20, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x10, 0x1b, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x4c, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x10, 0x1f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4d, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x4d, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x4e, 0x10, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4e, 0x10, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x4e, 0x26, 0x27, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x10, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4f, 0x10, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x4f, 0x20, 0x21, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x50, 0x10, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x50, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x50, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x05, 0x12, 0x03, 0x51, 0x10, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x51, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x02, 0x12, 0x03, 0x51, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x03, 0x52, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x52, 0x10, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x06, 0x02, 0x12, 0x03, 0x52, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x07, 0x12, 0x03, 0x53, 0x10, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x53, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x07,
    0x02, 0x12, 0x03, 0x53, 0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x08,
    0x12, 0x03, 0x54, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x54, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x08, 0x02,
    0x12, 0x03, 0x54, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12,
    0x03, 0x55, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12,
    0x03, 0x55, 0x10, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x09, 0x02, 0x12,
    0x03, 0x55, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x58, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x58, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x58, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x58, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x59, 0x08, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x59, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x59,
    0x11, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x59, 0x34, 0x3a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x59, 0x3d, 0x3e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5c, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x5c, 0x08, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03,
    0x5d, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5d, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5d, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x18, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x5e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5e,
    0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5e, 0x2c, 0x2d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x5f, 0x08, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x5f, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x5f, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x62, 0x00, 0x6d,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x62, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x06, 0x04, 0x00, 0x12, 0x04, 0x63, 0x08, 0x6a, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x04, 0x00, 0x01, 0x12, 0x03, 0x63, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x64, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x64, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x65, 0x10, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x65, 0x10, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x65, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x66, 0x10, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x66, 0x10, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x66, 0x26, 0x27, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x67, 0x10, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x67, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x67, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x68, 0x10, 0x32, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x68, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x68, 0x30, 0x31, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x69, 0x10, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x69, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03,
    0x69, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x6c, 0x08, 0x5c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6c, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6c, 0x11, 0x3c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6c, 0x3d, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x6c, 0x46, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x08, 0x12, 0x03, 0x6c, 0x48, 0x5b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x07, 0x12,
    0x03, 0x6c, 0x53, 0x5a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x6f, 0x00, 0x72, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x6f, 0x08, 0x29, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x70, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x70, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x70, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x70, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x71, 0x08, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x71, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x71, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x71, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x05, 0x74, 0x00,
    0x81, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x74, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x08, 0x04, 0x00, 0x12, 0x04, 0x75, 0x08, 0x7e, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x04, 0x00, 0x01, 0x12, 0x03, 0x75, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x08, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x76, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x76, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08,
    0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x76, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x77, 0x10, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x77, 0x10, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x77, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x78, 0x10, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x78, 0x10, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x78, 0x26, 0x27, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x79, 0x10, 0x32, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x79, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x79, 0x30, 0x31, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x7a, 0x10, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x7a, 0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x7a, 0x31, 0x32, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x7b, 0x10, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x7b, 0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x05, 0x02,
    0x12, 0x03, 0x7b, 0x31, 0x32, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x7c, 0x10, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x7c, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12,
    0x03, 0x7c, 0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03,
    0x7d, 0x10, 0x32, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x7d, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03,
    0x7d, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0x80, 0x01, 0x08,
    0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x80, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x04, 0x80, 0x01, 0x11, 0x3b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01, 0x3c, 0x42, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x01, 0x45, 0x46, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x08, 0x12, 0x04, 0x80, 0x01, 0x47, 0x5a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x07, 0x12, 0x04, 0x80, 0x01, 0x52, 0x59, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x06, 0x83, 0x01, 0x00, 0x86, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01,
    0x12, 0x04, 0x83, 0x01, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04,
    0x84, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x84,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x04, 0x84, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x84, 0x01, 0x18,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x84, 0x01, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x85, 0x01, 0x08, 0x2e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x85, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x04, 0x85, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0x85, 0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x85, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a,
    0x12, 0x06, 0x88, 0x01, 0x00, 0x90, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12,
    0x04, 0x88, 0x01, 0x08, 0x29, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0a, 0x04, 0x00, 0x12, 0x06, 0x89,
    0x01, 0x08, 0x8d, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x04, 0x00, 0x01, 0x12, 0x04,
    0x89, 0x01, 0x0d, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x8a, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x8a, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x04, 0x8a, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x04, 0x8b, 0x01, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x8b, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x04, 0x8b, 0x01, 0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0a, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x8f, 0x01, 0x11, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x8f, 0x01, 0x3b, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x8f, 0x01, 0x44, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x08, 0x12, 0x04, 0x8f,
    0x01, 0x46, 0x59, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x07, 0x12, 0x04, 0x8f, 0x01,
    0x51, 0x58, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x92, 0x01, 0x00, 0x97, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x92, 0x01, 0x08, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x93, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x93, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x93, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x93, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x93, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12,
    0x04, 0x94, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x04, 0x94,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x01,
    0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x01, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0x95, 0x01, 0x08, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0x95, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x04, 0x95, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x04, 0x95, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x04, 0x95, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x03, 0x12, 0x04, 0x96, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x96, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03,
    0x05, 0x12, 0x04, 0x96, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x96, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12,
    0x04, 0x96, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x99, 0x01, 0x00,
    0xa1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x99, 0x01, 0x08, 0x2a,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0c, 0x04, 0x00, 0x12, 0x06, 0x9a, 0x01, 0x08, 0x9e, 0x01, 0x09,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x04, 0x00, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x0d, 0x14, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x9b, 0x01, 0x10, 0x1c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x10, 0x17,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0x9b, 0x01, 0x1a,
    0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x10,
    0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9c, 0x01,
    0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x9c,
    0x01, 0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x9d,
    0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x9d, 0x01, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x04, 0x9d, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xa0,
    0x01, 0x08, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa0, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa0, 0x01, 0x11,
    0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x3c, 0x42,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x45, 0x46, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x08, 0x12, 0x04, 0xa0, 0x01, 0x47, 0x5a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x07, 0x12, 0x04, 0xa0, 0x01, 0x52, 0x59, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x0d, 0x12, 0x06, 0xa3, 0x01, 0x00, 0xa8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x0d, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00,
    0x12, 0x04, 0xa4, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xa4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4,
    0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x01,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa5, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xa6, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xa6, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x04,
    0xa7, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa7,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa7, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x18,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xaa, 0x01, 0x00, 0xac, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0e, 0x02, 0x00, 0x12, 0x04, 0xab, 0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xab, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xab, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xab, 0x01, 0x16, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xab, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xae, 0x01, 0x00,
    0xb1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xae, 0x01, 0x08, 0x32,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xb0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xb0, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xb0, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xb3, 0x01, 0x00, 0xb4,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x33, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xb6, 0x01, 0x00, 0xb9, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x00, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xb7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xb7, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xb7, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x01,
    0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb8, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb8, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x18, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb8, 0x01, 0x26, 0x27, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xbb, 0x01, 0x00, 0xc2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x12, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x29, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x12, 0x04,
    0x00, 0x12, 0x06, 0xbc, 0x01, 0x08, 0xbf, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x04,
    0x00, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x0d, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xbd, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xbd, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x12, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xbe, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xc1, 0x01, 0x11, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xc1, 0x01, 0x3b, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xc1, 0x01, 0x44, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x08, 0x12,
    0x04, 0xc1, 0x01, 0x46, 0x59, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x07, 0x12, 0x04,
    0xc1, 0x01, 0x51, 0x58, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xc4, 0x01, 0x00, 0xc6,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xc5, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12,
    0x06, 0xc8, 0x01, 0x00, 0xcf, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04,
    0xc8, 0x01, 0x08, 0x2a, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x14, 0x04, 0x00, 0x12, 0x06, 0xc9, 0x01,
    0x08, 0xcc, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x04, 0x00, 0x01, 0x12, 0x04, 0xc9,
    0x01, 0x0d, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x14, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xca,
    0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x14, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xca, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x14, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x04, 0xca, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x14, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x04, 0xcb, 0x01, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x14, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xcb, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x14, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x04, 0xcb, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12,
    0x04, 0xce, 0x01, 0x08, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xce, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0xce,
    0x01, 0x11, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xce, 0x01,
    0x3c, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xce, 0x01, 0x45,
    0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x08, 0x12, 0x04, 0xce, 0x01, 0x47, 0x5a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x07, 0x12, 0x04, 0xce, 0x01, 0x52, 0x59, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xd1, 0x01, 0x00, 0xdb, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x25, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x15,
    0x03, 0x00, 0x12, 0x06, 0xd2, 0x01, 0x08, 0xd7, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x03, 0x00, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x04, 0xd3, 0x01, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd3, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd3, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x20, 0x28, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x10, 0x37, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd4, 0x01, 0x19, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x20, 0x32,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x35,
    0x36, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xd5, 0x01, 0x10,
    0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x01,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd5,
    0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xd5, 0x01, 0x20, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xd5, 0x01, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03, 0x12,
    0x04, 0xd6, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x04, 0xd6, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x04, 0xd6, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x20, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x00, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xd9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xd9, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xd9, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd9,
    0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xda, 0x01, 0x08,
    0x4a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xda, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x06, 0x12, 0x04, 0xda, 0x01, 0x11, 0x38, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xda, 0x01, 0x39, 0x45, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xda, 0x01, 0x48, 0x49, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x16, 0x12, 0x06, 0xdd, 0x01, 0x00, 0xde, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x16, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06,
    0xe0, 0x01, 0x00, 0xe9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xe0,
    0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x01, 0x08,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe1, 0x01, 0x11, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x16, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xe2, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12,
    0x04, 0xe3, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xe3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe3,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe3, 0x01,
    0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe3, 0x01, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x05, 0x12, 0x04, 0xe4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe4, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x04, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x04, 0x04, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04,
    0x05, 0x12, 0x04, 0xe5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x01,
    0x12, 0x04, 0xe5, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x03, 0x12,
    0x04, 0xe5, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x05, 0x12, 0x04, 0xe6,
    0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x04, 0x12, 0x04, 0xe6, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x18, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x06, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x06, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x06, 0x05, 0x12, 0x04, 0xe7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x06, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x06, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x07, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x04,
    0x12, 0x04, 0xe8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x05, 0x12,
    0x04, 0xe8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x01, 0x12, 0x04,
    0xe8, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x03, 0x12, 0x04, 0xe8,
    0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xeb, 0x01, 0x00, 0xee, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xec, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xec, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0xec, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xec, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xec, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01,
    0x12, 0x04, 0xed, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xed, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xed, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xed,
    0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xed, 0x01,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xf0, 0x01, 0x00, 0xf3, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xf1, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12,
    0x04, 0xf2, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xf2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf2,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf2, 0x01,
    0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf2, 0x01, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xf5, 0x01, 0x00, 0xff, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x25, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x1a, 0x04, 0x00, 0x12, 0x06, 0xf6, 0x01, 0x08, 0xfb, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x04, 0x00, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x0d, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x1a, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xf7, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1a, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1a, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xf7, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x10, 0x1d, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xf8, 0x01, 0x20, 0x21, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xf9, 0x01, 0x10, 0x27, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x10, 0x22,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xf9, 0x01, 0x25,
    0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xfa, 0x01, 0x10,
    0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xfa, 0x01,
    0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1a, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xfa,
    0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xfd, 0x01, 0x08,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xfd, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x18, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x08, 0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xfe, 0x01, 0x11, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x37, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xfe, 0x01, 0x40, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x08,
    0x12, 0x04, 0xfe, 0x01, 0x42, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x07, 0x12,
    0x04, 0xfe, 0x01, 0x4d, 0x54,
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
