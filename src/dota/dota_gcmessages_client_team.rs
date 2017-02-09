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
pub struct CMsgDOTATeamMemberSDO {
    // message fields
    account_id: ::std::option::Option<u32>,
    team_ids: ::std::vec::Vec<u32>,
    profile_team_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamMemberSDO {}

impl CMsgDOTATeamMemberSDO {
    pub fn new() -> CMsgDOTATeamMemberSDO {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamMemberSDO {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamMemberSDO> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamMemberSDO,
        };
        unsafe {
            instance.get(CMsgDOTATeamMemberSDO::new)
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

    // repeated uint32 team_ids = 2;

    pub fn clear_team_ids(&mut self) {
        self.team_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_team_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.team_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_team_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.team_ids
    }

    // Take field
    pub fn take_team_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.team_ids, ::std::vec::Vec::new())
    }

    pub fn get_team_ids(&self) -> &[u32] {
        &self.team_ids
    }

    fn get_team_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.team_ids
    }

    fn mut_team_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.team_ids
    }

    // optional uint32 profile_team_id = 3;

    pub fn clear_profile_team_id(&mut self) {
        self.profile_team_id = ::std::option::Option::None;
    }

    pub fn has_profile_team_id(&self) -> bool {
        self.profile_team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_profile_team_id(&mut self, v: u32) {
        self.profile_team_id = ::std::option::Option::Some(v);
    }

    pub fn get_profile_team_id(&self) -> u32 {
        self.profile_team_id.unwrap_or(0)
    }

    fn get_profile_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.profile_team_id
    }

    fn mut_profile_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.profile_team_id
    }
}

impl ::protobuf::Message for CMsgDOTATeamMemberSDO {
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
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.team_ids)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.profile_team_id = ::std::option::Option::Some(tmp);
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
        for value in &self.team_ids {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.profile_team_id {
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
        for v in &self.team_ids {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.profile_team_id {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamMemberSDO {
    fn new() -> CMsgDOTATeamMemberSDO {
        CMsgDOTATeamMemberSDO::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamMemberSDO>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTATeamMemberSDO::get_account_id_for_reflect,
                    CMsgDOTATeamMemberSDO::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_ids",
                    CMsgDOTATeamMemberSDO::get_team_ids_for_reflect,
                    CMsgDOTATeamMemberSDO::mut_team_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "profile_team_id",
                    CMsgDOTATeamMemberSDO::get_profile_team_id_for_reflect,
                    CMsgDOTATeamMemberSDO::mut_profile_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamMemberSDO>(
                    "CMsgDOTATeamMemberSDO",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamMemberSDO {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_team_ids();
        self.clear_profile_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamMemberSDO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamMemberSDO {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamAdminSDO {
    // message fields
    account_id: ::std::option::Option<u32>,
    team_ids: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamAdminSDO {}

impl CMsgDOTATeamAdminSDO {
    pub fn new() -> CMsgDOTATeamAdminSDO {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamAdminSDO {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamAdminSDO> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamAdminSDO,
        };
        unsafe {
            instance.get(CMsgDOTATeamAdminSDO::new)
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

    // repeated uint32 team_ids = 2;

    pub fn clear_team_ids(&mut self) {
        self.team_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_team_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.team_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_team_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.team_ids
    }

    // Take field
    pub fn take_team_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.team_ids, ::std::vec::Vec::new())
    }

    pub fn get_team_ids(&self) -> &[u32] {
        &self.team_ids
    }

    fn get_team_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.team_ids
    }

    fn mut_team_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.team_ids
    }
}

impl ::protobuf::Message for CMsgDOTATeamAdminSDO {
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
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.team_ids)?;
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
        for value in &self.team_ids {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        for v in &self.team_ids {
            os.write_uint32(2, *v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATeamAdminSDO {
    fn new() -> CMsgDOTATeamAdminSDO {
        CMsgDOTATeamAdminSDO::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamAdminSDO>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTATeamAdminSDO::get_account_id_for_reflect,
                    CMsgDOTATeamAdminSDO::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_ids",
                    CMsgDOTATeamAdminSDO::get_team_ids_for_reflect,
                    CMsgDOTATeamAdminSDO::mut_team_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamAdminSDO>(
                    "CMsgDOTATeamAdminSDO",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamAdminSDO {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_team_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamAdminSDO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamAdminSDO {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamMember {
    // message fields
    account_id: ::std::option::Option<u32>,
    time_joined: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamMember {}

impl CMsgDOTATeamMember {
    pub fn new() -> CMsgDOTATeamMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamMember {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamMember,
        };
        unsafe {
            instance.get(CMsgDOTATeamMember::new)
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

    // optional uint32 time_joined = 4;

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
}

impl ::protobuf::Message for CMsgDOTATeamMember {
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
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_joined = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
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
            os.write_uint32(4, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATeamMember {
    fn new() -> CMsgDOTATeamMember {
        CMsgDOTATeamMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTATeamMember::get_account_id_for_reflect,
                    CMsgDOTATeamMember::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_joined",
                    CMsgDOTATeamMember::get_time_joined_for_reflect,
                    CMsgDOTATeamMember::mut_time_joined_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamMember>(
                    "CMsgDOTATeamMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamMember {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_time_joined();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeam {
    // message fields
    members: ::protobuf::RepeatedField<CMsgDOTATeamMember>,
    team_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    tag: ::protobuf::SingularField<::std::string::String>,
    admin_id: ::std::option::Option<u32>,
    time_created: ::std::option::Option<u32>,
    disbanded: ::std::option::Option<bool>,
    wins: ::std::option::Option<u32>,
    losses: ::std::option::Option<u32>,
    rank: ::std::option::Option<u32>,
    calibration_games_remaining: ::std::option::Option<u32>,
    logo: ::std::option::Option<u64>,
    base_logo: ::std::option::Option<u64>,
    banner_logo: ::std::option::Option<u64>,
    sponsor_logo: ::std::option::Option<u64>,
    country_code: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    fullgamesplayed: ::std::option::Option<u32>,
    leagues: ::std::vec::Vec<u32>,
    gamesplayed: ::std::option::Option<u32>,
    gamesplayedwithcurrentroster: ::std::option::Option<u32>,
    teammatchmakinggamesplayed: ::std::option::Option<u32>,
    lastplayedgametime: ::std::option::Option<u32>,
    lastrenametime: ::std::option::Option<u32>,
    recent_match_ids: ::std::vec::Vec<u64>,
    top_match_ids: ::std::vec::Vec<u64>,
    pickup_team: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeam {}

impl CMsgDOTATeam {
    pub fn new() -> CMsgDOTATeam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeam {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeam,
        };
        unsafe {
            instance.get(CMsgDOTATeam::new)
        }
    }

    // repeated .dota.CMsgDOTATeamMember members = 1;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATeamMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATeamMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATeamMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CMsgDOTATeamMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATeamMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATeamMember> {
        &mut self.members
    }

    // optional uint32 team_id = 2;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
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

    // optional string tag = 4;

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

    // optional uint32 admin_id = 5;

    pub fn clear_admin_id(&mut self) {
        self.admin_id = ::std::option::Option::None;
    }

    pub fn has_admin_id(&self) -> bool {
        self.admin_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin_id(&mut self, v: u32) {
        self.admin_id = ::std::option::Option::Some(v);
    }

    pub fn get_admin_id(&self) -> u32 {
        self.admin_id.unwrap_or(0)
    }

    fn get_admin_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.admin_id
    }

    fn mut_admin_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.admin_id
    }

    // optional uint32 time_created = 6;

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

    // optional bool disbanded = 7;

    pub fn clear_disbanded(&mut self) {
        self.disbanded = ::std::option::Option::None;
    }

    pub fn has_disbanded(&self) -> bool {
        self.disbanded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disbanded(&mut self, v: bool) {
        self.disbanded = ::std::option::Option::Some(v);
    }

    pub fn get_disbanded(&self) -> bool {
        self.disbanded.unwrap_or(false)
    }

    fn get_disbanded_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.disbanded
    }

    fn mut_disbanded_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.disbanded
    }

    // optional uint32 wins = 8;

    pub fn clear_wins(&mut self) {
        self.wins = ::std::option::Option::None;
    }

    pub fn has_wins(&self) -> bool {
        self.wins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wins(&mut self, v: u32) {
        self.wins = ::std::option::Option::Some(v);
    }

    pub fn get_wins(&self) -> u32 {
        self.wins.unwrap_or(0)
    }

    fn get_wins_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.wins
    }

    fn mut_wins_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.wins
    }

    // optional uint32 losses = 9;

    pub fn clear_losses(&mut self) {
        self.losses = ::std::option::Option::None;
    }

    pub fn has_losses(&self) -> bool {
        self.losses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_losses(&mut self, v: u32) {
        self.losses = ::std::option::Option::Some(v);
    }

    pub fn get_losses(&self) -> u32 {
        self.losses.unwrap_or(0)
    }

    fn get_losses_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.losses
    }

    fn mut_losses_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.losses
    }

    // optional uint32 rank = 10;

    pub fn clear_rank(&mut self) {
        self.rank = ::std::option::Option::None;
    }

    pub fn has_rank(&self) -> bool {
        self.rank.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank(&mut self, v: u32) {
        self.rank = ::std::option::Option::Some(v);
    }

    pub fn get_rank(&self) -> u32 {
        self.rank.unwrap_or(0)
    }

    fn get_rank_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.rank
    }

    fn mut_rank_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.rank
    }

    // optional uint32 calibration_games_remaining = 24;

    pub fn clear_calibration_games_remaining(&mut self) {
        self.calibration_games_remaining = ::std::option::Option::None;
    }

    pub fn has_calibration_games_remaining(&self) -> bool {
        self.calibration_games_remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_calibration_games_remaining(&mut self, v: u32) {
        self.calibration_games_remaining = ::std::option::Option::Some(v);
    }

    pub fn get_calibration_games_remaining(&self) -> u32 {
        self.calibration_games_remaining.unwrap_or(0)
    }

    fn get_calibration_games_remaining_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.calibration_games_remaining
    }

    fn mut_calibration_games_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.calibration_games_remaining
    }

    // optional uint64 logo = 11;

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

    // optional uint64 base_logo = 12;

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

    // optional uint64 banner_logo = 13;

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

    // optional uint64 sponsor_logo = 14;

    pub fn clear_sponsor_logo(&mut self) {
        self.sponsor_logo = ::std::option::Option::None;
    }

    pub fn has_sponsor_logo(&self) -> bool {
        self.sponsor_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sponsor_logo(&mut self, v: u64) {
        self.sponsor_logo = ::std::option::Option::Some(v);
    }

    pub fn get_sponsor_logo(&self) -> u64 {
        self.sponsor_logo.unwrap_or(0)
    }

    fn get_sponsor_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sponsor_logo
    }

    fn mut_sponsor_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sponsor_logo
    }

    // optional string country_code = 15;

    pub fn clear_country_code(&mut self) {
        self.country_code.clear();
    }

    pub fn has_country_code(&self) -> bool {
        self.country_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country_code(&mut self, v: ::std::string::String) {
        self.country_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country_code(&mut self) -> &mut ::std::string::String {
        if self.country_code.is_none() {
            self.country_code.set_default();
        };
        self.country_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_country_code(&mut self) -> ::std::string::String {
        self.country_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country_code(&self) -> &str {
        match self.country_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country_code
    }

    fn mut_country_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country_code
    }

    // optional string url = 16;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }

    // optional uint32 fullgamesplayed = 17;

    pub fn clear_fullgamesplayed(&mut self) {
        self.fullgamesplayed = ::std::option::Option::None;
    }

    pub fn has_fullgamesplayed(&self) -> bool {
        self.fullgamesplayed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullgamesplayed(&mut self, v: u32) {
        self.fullgamesplayed = ::std::option::Option::Some(v);
    }

    pub fn get_fullgamesplayed(&self) -> u32 {
        self.fullgamesplayed.unwrap_or(0)
    }

    fn get_fullgamesplayed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.fullgamesplayed
    }

    fn mut_fullgamesplayed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.fullgamesplayed
    }

    // repeated uint32 leagues = 18;

    pub fn clear_leagues(&mut self) {
        self.leagues.clear();
    }

    // Param is passed by value, moved
    pub fn set_leagues(&mut self, v: ::std::vec::Vec<u32>) {
        self.leagues = v;
    }

    // Mutable pointer to the field.
    pub fn mut_leagues(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.leagues
    }

    // Take field
    pub fn take_leagues(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.leagues, ::std::vec::Vec::new())
    }

    pub fn get_leagues(&self) -> &[u32] {
        &self.leagues
    }

    fn get_leagues_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.leagues
    }

    fn mut_leagues_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.leagues
    }

    // optional uint32 gamesplayed = 19;

    pub fn clear_gamesplayed(&mut self) {
        self.gamesplayed = ::std::option::Option::None;
    }

    pub fn has_gamesplayed(&self) -> bool {
        self.gamesplayed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gamesplayed(&mut self, v: u32) {
        self.gamesplayed = ::std::option::Option::Some(v);
    }

    pub fn get_gamesplayed(&self) -> u32 {
        self.gamesplayed.unwrap_or(0)
    }

    fn get_gamesplayed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gamesplayed
    }

    fn mut_gamesplayed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gamesplayed
    }

    // optional uint32 gamesplayedwithcurrentroster = 20;

    pub fn clear_gamesplayedwithcurrentroster(&mut self) {
        self.gamesplayedwithcurrentroster = ::std::option::Option::None;
    }

    pub fn has_gamesplayedwithcurrentroster(&self) -> bool {
        self.gamesplayedwithcurrentroster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gamesplayedwithcurrentroster(&mut self, v: u32) {
        self.gamesplayedwithcurrentroster = ::std::option::Option::Some(v);
    }

    pub fn get_gamesplayedwithcurrentroster(&self) -> u32 {
        self.gamesplayedwithcurrentroster.unwrap_or(0)
    }

    fn get_gamesplayedwithcurrentroster_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gamesplayedwithcurrentroster
    }

    fn mut_gamesplayedwithcurrentroster_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gamesplayedwithcurrentroster
    }

    // optional uint32 teammatchmakinggamesplayed = 21;

    pub fn clear_teammatchmakinggamesplayed(&mut self) {
        self.teammatchmakinggamesplayed = ::std::option::Option::None;
    }

    pub fn has_teammatchmakinggamesplayed(&self) -> bool {
        self.teammatchmakinggamesplayed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_teammatchmakinggamesplayed(&mut self, v: u32) {
        self.teammatchmakinggamesplayed = ::std::option::Option::Some(v);
    }

    pub fn get_teammatchmakinggamesplayed(&self) -> u32 {
        self.teammatchmakinggamesplayed.unwrap_or(0)
    }

    fn get_teammatchmakinggamesplayed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.teammatchmakinggamesplayed
    }

    fn mut_teammatchmakinggamesplayed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.teammatchmakinggamesplayed
    }

    // optional uint32 lastplayedgametime = 22;

    pub fn clear_lastplayedgametime(&mut self) {
        self.lastplayedgametime = ::std::option::Option::None;
    }

    pub fn has_lastplayedgametime(&self) -> bool {
        self.lastplayedgametime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastplayedgametime(&mut self, v: u32) {
        self.lastplayedgametime = ::std::option::Option::Some(v);
    }

    pub fn get_lastplayedgametime(&self) -> u32 {
        self.lastplayedgametime.unwrap_or(0)
    }

    fn get_lastplayedgametime_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lastplayedgametime
    }

    fn mut_lastplayedgametime_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lastplayedgametime
    }

    // optional uint32 lastrenametime = 23;

    pub fn clear_lastrenametime(&mut self) {
        self.lastrenametime = ::std::option::Option::None;
    }

    pub fn has_lastrenametime(&self) -> bool {
        self.lastrenametime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastrenametime(&mut self, v: u32) {
        self.lastrenametime = ::std::option::Option::Some(v);
    }

    pub fn get_lastrenametime(&self) -> u32 {
        self.lastrenametime.unwrap_or(0)
    }

    fn get_lastrenametime_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lastrenametime
    }

    fn mut_lastrenametime_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lastrenametime
    }

    // repeated uint64 recent_match_ids = 25;

    pub fn clear_recent_match_ids(&mut self) {
        self.recent_match_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_recent_match_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.recent_match_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_recent_match_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.recent_match_ids
    }

    // Take field
    pub fn take_recent_match_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.recent_match_ids, ::std::vec::Vec::new())
    }

    pub fn get_recent_match_ids(&self) -> &[u64] {
        &self.recent_match_ids
    }

    fn get_recent_match_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.recent_match_ids
    }

    fn mut_recent_match_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.recent_match_ids
    }

    // repeated uint64 top_match_ids = 26;

    pub fn clear_top_match_ids(&mut self) {
        self.top_match_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_top_match_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.top_match_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_top_match_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.top_match_ids
    }

    // Take field
    pub fn take_top_match_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.top_match_ids, ::std::vec::Vec::new())
    }

    pub fn get_top_match_ids(&self) -> &[u64] {
        &self.top_match_ids
    }

    fn get_top_match_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.top_match_ids
    }

    fn mut_top_match_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.top_match_ids
    }

    // optional bool pickup_team = 27;

    pub fn clear_pickup_team(&mut self) {
        self.pickup_team = ::std::option::Option::None;
    }

    pub fn has_pickup_team(&self) -> bool {
        self.pickup_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pickup_team(&mut self, v: bool) {
        self.pickup_team = ::std::option::Option::Some(v);
    }

    pub fn get_pickup_team(&self) -> bool {
        self.pickup_team.unwrap_or(false)
    }

    fn get_pickup_team_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.pickup_team
    }

    fn mut_pickup_team_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.pickup_team
    }
}

impl ::protobuf::Message for CMsgDOTATeam {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.admin_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_created = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.disbanded = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.wins = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.losses = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.rank = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.calibration_games_remaining = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.logo = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.base_logo = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.banner_logo = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.sponsor_logo = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country_code)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.fullgamesplayed = ::std::option::Option::Some(tmp);
                },
                18 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.leagues)?;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.gamesplayed = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.gamesplayedwithcurrentroster = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.teammatchmakinggamesplayed = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lastplayedgametime = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lastrenametime = ::std::option::Option::Some(tmp);
                },
                25 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.recent_match_ids)?;
                },
                26 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.top_match_ids)?;
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.pickup_team = ::std::option::Option::Some(tmp);
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
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.admin_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.time_created {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.disbanded {
            my_size += 2;
        };
        if let Some(v) = self.wins {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.losses {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.rank {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.calibration_games_remaining {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.base_logo {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.banner_logo {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sponsor_logo {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.country_code.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        };
        if let Some(v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        };
        if let Some(v) = self.fullgamesplayed {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.leagues {
            my_size += ::protobuf::rt::value_size(18, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gamesplayed {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gamesplayedwithcurrentroster {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.teammatchmakinggamesplayed {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lastplayedgametime {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lastrenametime {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.recent_match_ids {
            my_size += ::protobuf::rt::value_size(25, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.top_match_ids {
            my_size += ::protobuf::rt::value_size(26, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pickup_team {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.members {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.team_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.tag.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.admin_id {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.time_created {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.disbanded {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.wins {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.losses {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.rank {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.calibration_games_remaining {
            os.write_uint32(24, v)?;
        };
        if let Some(v) = self.logo {
            os.write_uint64(11, v)?;
        };
        if let Some(v) = self.base_logo {
            os.write_uint64(12, v)?;
        };
        if let Some(v) = self.banner_logo {
            os.write_uint64(13, v)?;
        };
        if let Some(v) = self.sponsor_logo {
            os.write_uint64(14, v)?;
        };
        if let Some(v) = self.country_code.as_ref() {
            os.write_string(15, &v)?;
        };
        if let Some(v) = self.url.as_ref() {
            os.write_string(16, &v)?;
        };
        if let Some(v) = self.fullgamesplayed {
            os.write_uint32(17, v)?;
        };
        for v in &self.leagues {
            os.write_uint32(18, *v)?;
        };
        if let Some(v) = self.gamesplayed {
            os.write_uint32(19, v)?;
        };
        if let Some(v) = self.gamesplayedwithcurrentroster {
            os.write_uint32(20, v)?;
        };
        if let Some(v) = self.teammatchmakinggamesplayed {
            os.write_uint32(21, v)?;
        };
        if let Some(v) = self.lastplayedgametime {
            os.write_uint32(22, v)?;
        };
        if let Some(v) = self.lastrenametime {
            os.write_uint32(23, v)?;
        };
        for v in &self.recent_match_ids {
            os.write_uint64(25, *v)?;
        };
        for v in &self.top_match_ids {
            os.write_uint64(26, *v)?;
        };
        if let Some(v) = self.pickup_team {
            os.write_bool(27, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATeam {
    fn new() -> CMsgDOTATeam {
        CMsgDOTATeam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATeamMember>>(
                    "members",
                    CMsgDOTATeam::get_members_for_reflect,
                    CMsgDOTATeam::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTATeam::get_team_id_for_reflect,
                    CMsgDOTATeam::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgDOTATeam::get_name_for_reflect,
                    CMsgDOTATeam::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    CMsgDOTATeam::get_tag_for_reflect,
                    CMsgDOTATeam::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "admin_id",
                    CMsgDOTATeam::get_admin_id_for_reflect,
                    CMsgDOTATeam::mut_admin_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_created",
                    CMsgDOTATeam::get_time_created_for_reflect,
                    CMsgDOTATeam::mut_time_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "disbanded",
                    CMsgDOTATeam::get_disbanded_for_reflect,
                    CMsgDOTATeam::mut_disbanded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "wins",
                    CMsgDOTATeam::get_wins_for_reflect,
                    CMsgDOTATeam::mut_wins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "losses",
                    CMsgDOTATeam::get_losses_for_reflect,
                    CMsgDOTATeam::mut_losses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "rank",
                    CMsgDOTATeam::get_rank_for_reflect,
                    CMsgDOTATeam::mut_rank_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "calibration_games_remaining",
                    CMsgDOTATeam::get_calibration_games_remaining_for_reflect,
                    CMsgDOTATeam::mut_calibration_games_remaining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTATeam::get_logo_for_reflect,
                    CMsgDOTATeam::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "base_logo",
                    CMsgDOTATeam::get_base_logo_for_reflect,
                    CMsgDOTATeam::mut_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "banner_logo",
                    CMsgDOTATeam::get_banner_logo_for_reflect,
                    CMsgDOTATeam::mut_banner_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sponsor_logo",
                    CMsgDOTATeam::get_sponsor_logo_for_reflect,
                    CMsgDOTATeam::mut_sponsor_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country_code",
                    CMsgDOTATeam::get_country_code_for_reflect,
                    CMsgDOTATeam::mut_country_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    CMsgDOTATeam::get_url_for_reflect,
                    CMsgDOTATeam::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "fullgamesplayed",
                    CMsgDOTATeam::get_fullgamesplayed_for_reflect,
                    CMsgDOTATeam::mut_fullgamesplayed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leagues",
                    CMsgDOTATeam::get_leagues_for_reflect,
                    CMsgDOTATeam::mut_leagues_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gamesplayed",
                    CMsgDOTATeam::get_gamesplayed_for_reflect,
                    CMsgDOTATeam::mut_gamesplayed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gamesplayedwithcurrentroster",
                    CMsgDOTATeam::get_gamesplayedwithcurrentroster_for_reflect,
                    CMsgDOTATeam::mut_gamesplayedwithcurrentroster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "teammatchmakinggamesplayed",
                    CMsgDOTATeam::get_teammatchmakinggamesplayed_for_reflect,
                    CMsgDOTATeam::mut_teammatchmakinggamesplayed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lastplayedgametime",
                    CMsgDOTATeam::get_lastplayedgametime_for_reflect,
                    CMsgDOTATeam::mut_lastplayedgametime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lastrenametime",
                    CMsgDOTATeam::get_lastrenametime_for_reflect,
                    CMsgDOTATeam::mut_lastrenametime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "recent_match_ids",
                    CMsgDOTATeam::get_recent_match_ids_for_reflect,
                    CMsgDOTATeam::mut_recent_match_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "top_match_ids",
                    CMsgDOTATeam::get_top_match_ids_for_reflect,
                    CMsgDOTATeam::mut_top_match_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "pickup_team",
                    CMsgDOTATeam::get_pickup_team_for_reflect,
                    CMsgDOTATeam::mut_pickup_team_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeam>(
                    "CMsgDOTATeam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeam {
    fn clear(&mut self) {
        self.clear_members();
        self.clear_team_id();
        self.clear_name();
        self.clear_tag();
        self.clear_admin_id();
        self.clear_time_created();
        self.clear_disbanded();
        self.clear_wins();
        self.clear_losses();
        self.clear_rank();
        self.clear_calibration_games_remaining();
        self.clear_logo();
        self.clear_base_logo();
        self.clear_banner_logo();
        self.clear_sponsor_logo();
        self.clear_country_code();
        self.clear_url();
        self.clear_fullgamesplayed();
        self.clear_leagues();
        self.clear_gamesplayed();
        self.clear_gamesplayedwithcurrentroster();
        self.clear_teammatchmakinggamesplayed();
        self.clear_lastplayedgametime();
        self.clear_lastrenametime();
        self.clear_recent_match_ids();
        self.clear_top_match_ids();
        self.clear_pickup_team();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamInfo {
    // message fields
    members: ::protobuf::RepeatedField<CMsgDOTATeamInfo_Member>,
    team_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    tag: ::protobuf::SingularField<::std::string::String>,
    time_created: ::std::option::Option<u32>,
    pro: ::std::option::Option<bool>,
    locked: ::std::option::Option<bool>,
    pickup_team: ::std::option::Option<bool>,
    ugc_logo: ::std::option::Option<u64>,
    ugc_base_logo: ::std::option::Option<u64>,
    ugc_banner_logo: ::std::option::Option<u64>,
    ugc_sponsor_logo: ::std::option::Option<u64>,
    country_code: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    wins: ::std::option::Option<u32>,
    losses: ::std::option::Option<u32>,
    rank: ::std::option::Option<u32>,
    calibration_games_remaining: ::std::option::Option<u32>,
    games_played_total: ::std::option::Option<u32>,
    games_played_matchmaking: ::std::option::Option<u32>,
    leagues_participated: ::std::vec::Vec<u32>,
    top_match_ids: ::std::vec::Vec<u64>,
    recent_match_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamInfo {}

impl CMsgDOTATeamInfo {
    pub fn new() -> CMsgDOTATeamInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamInfo,
        };
        unsafe {
            instance.get(CMsgDOTATeamInfo::new)
        }
    }

    // repeated .dota.CMsgDOTATeamInfo.Member members = 1;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATeamInfo_Member>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATeamInfo_Member> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATeamInfo_Member> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CMsgDOTATeamInfo_Member] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATeamInfo_Member> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATeamInfo_Member> {
        &mut self.members
    }

    // optional uint32 team_id = 2;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
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

    // optional string tag = 4;

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

    // optional uint32 time_created = 5;

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

    // optional bool pro = 6;

    pub fn clear_pro(&mut self) {
        self.pro = ::std::option::Option::None;
    }

    pub fn has_pro(&self) -> bool {
        self.pro.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pro(&mut self, v: bool) {
        self.pro = ::std::option::Option::Some(v);
    }

    pub fn get_pro(&self) -> bool {
        self.pro.unwrap_or(false)
    }

    fn get_pro_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.pro
    }

    fn mut_pro_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.pro
    }

    // optional bool locked = 7;

    pub fn clear_locked(&mut self) {
        self.locked = ::std::option::Option::None;
    }

    pub fn has_locked(&self) -> bool {
        self.locked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locked(&mut self, v: bool) {
        self.locked = ::std::option::Option::Some(v);
    }

    pub fn get_locked(&self) -> bool {
        self.locked.unwrap_or(false)
    }

    fn get_locked_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.locked
    }

    fn mut_locked_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.locked
    }

    // optional bool pickup_team = 8;

    pub fn clear_pickup_team(&mut self) {
        self.pickup_team = ::std::option::Option::None;
    }

    pub fn has_pickup_team(&self) -> bool {
        self.pickup_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pickup_team(&mut self, v: bool) {
        self.pickup_team = ::std::option::Option::Some(v);
    }

    pub fn get_pickup_team(&self) -> bool {
        self.pickup_team.unwrap_or(false)
    }

    fn get_pickup_team_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.pickup_team
    }

    fn mut_pickup_team_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.pickup_team
    }

    // optional uint64 ugc_logo = 9;

    pub fn clear_ugc_logo(&mut self) {
        self.ugc_logo = ::std::option::Option::None;
    }

    pub fn has_ugc_logo(&self) -> bool {
        self.ugc_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ugc_logo(&mut self, v: u64) {
        self.ugc_logo = ::std::option::Option::Some(v);
    }

    pub fn get_ugc_logo(&self) -> u64 {
        self.ugc_logo.unwrap_or(0)
    }

    fn get_ugc_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ugc_logo
    }

    fn mut_ugc_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ugc_logo
    }

    // optional uint64 ugc_base_logo = 10;

    pub fn clear_ugc_base_logo(&mut self) {
        self.ugc_base_logo = ::std::option::Option::None;
    }

    pub fn has_ugc_base_logo(&self) -> bool {
        self.ugc_base_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ugc_base_logo(&mut self, v: u64) {
        self.ugc_base_logo = ::std::option::Option::Some(v);
    }

    pub fn get_ugc_base_logo(&self) -> u64 {
        self.ugc_base_logo.unwrap_or(0)
    }

    fn get_ugc_base_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ugc_base_logo
    }

    fn mut_ugc_base_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ugc_base_logo
    }

    // optional uint64 ugc_banner_logo = 11;

    pub fn clear_ugc_banner_logo(&mut self) {
        self.ugc_banner_logo = ::std::option::Option::None;
    }

    pub fn has_ugc_banner_logo(&self) -> bool {
        self.ugc_banner_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ugc_banner_logo(&mut self, v: u64) {
        self.ugc_banner_logo = ::std::option::Option::Some(v);
    }

    pub fn get_ugc_banner_logo(&self) -> u64 {
        self.ugc_banner_logo.unwrap_or(0)
    }

    fn get_ugc_banner_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ugc_banner_logo
    }

    fn mut_ugc_banner_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ugc_banner_logo
    }

    // optional uint64 ugc_sponsor_logo = 12;

    pub fn clear_ugc_sponsor_logo(&mut self) {
        self.ugc_sponsor_logo = ::std::option::Option::None;
    }

    pub fn has_ugc_sponsor_logo(&self) -> bool {
        self.ugc_sponsor_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ugc_sponsor_logo(&mut self, v: u64) {
        self.ugc_sponsor_logo = ::std::option::Option::Some(v);
    }

    pub fn get_ugc_sponsor_logo(&self) -> u64 {
        self.ugc_sponsor_logo.unwrap_or(0)
    }

    fn get_ugc_sponsor_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ugc_sponsor_logo
    }

    fn mut_ugc_sponsor_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ugc_sponsor_logo
    }

    // optional string country_code = 13;

    pub fn clear_country_code(&mut self) {
        self.country_code.clear();
    }

    pub fn has_country_code(&self) -> bool {
        self.country_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country_code(&mut self, v: ::std::string::String) {
        self.country_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country_code(&mut self) -> &mut ::std::string::String {
        if self.country_code.is_none() {
            self.country_code.set_default();
        };
        self.country_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_country_code(&mut self) -> ::std::string::String {
        self.country_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country_code(&self) -> &str {
        match self.country_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country_code
    }

    fn mut_country_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country_code
    }

    // optional string url = 14;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }

    // optional uint32 wins = 15;

    pub fn clear_wins(&mut self) {
        self.wins = ::std::option::Option::None;
    }

    pub fn has_wins(&self) -> bool {
        self.wins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wins(&mut self, v: u32) {
        self.wins = ::std::option::Option::Some(v);
    }

    pub fn get_wins(&self) -> u32 {
        self.wins.unwrap_or(0)
    }

    fn get_wins_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.wins
    }

    fn mut_wins_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.wins
    }

    // optional uint32 losses = 16;

    pub fn clear_losses(&mut self) {
        self.losses = ::std::option::Option::None;
    }

    pub fn has_losses(&self) -> bool {
        self.losses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_losses(&mut self, v: u32) {
        self.losses = ::std::option::Option::Some(v);
    }

    pub fn get_losses(&self) -> u32 {
        self.losses.unwrap_or(0)
    }

    fn get_losses_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.losses
    }

    fn mut_losses_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.losses
    }

    // optional uint32 rank = 17;

    pub fn clear_rank(&mut self) {
        self.rank = ::std::option::Option::None;
    }

    pub fn has_rank(&self) -> bool {
        self.rank.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank(&mut self, v: u32) {
        self.rank = ::std::option::Option::Some(v);
    }

    pub fn get_rank(&self) -> u32 {
        self.rank.unwrap_or(0)
    }

    fn get_rank_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.rank
    }

    fn mut_rank_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.rank
    }

    // optional uint32 calibration_games_remaining = 18;

    pub fn clear_calibration_games_remaining(&mut self) {
        self.calibration_games_remaining = ::std::option::Option::None;
    }

    pub fn has_calibration_games_remaining(&self) -> bool {
        self.calibration_games_remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_calibration_games_remaining(&mut self, v: u32) {
        self.calibration_games_remaining = ::std::option::Option::Some(v);
    }

    pub fn get_calibration_games_remaining(&self) -> u32 {
        self.calibration_games_remaining.unwrap_or(0)
    }

    fn get_calibration_games_remaining_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.calibration_games_remaining
    }

    fn mut_calibration_games_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.calibration_games_remaining
    }

    // optional uint32 games_played_total = 19;

    pub fn clear_games_played_total(&mut self) {
        self.games_played_total = ::std::option::Option::None;
    }

    pub fn has_games_played_total(&self) -> bool {
        self.games_played_total.is_some()
    }

    // Param is passed by value, moved
    pub fn set_games_played_total(&mut self, v: u32) {
        self.games_played_total = ::std::option::Option::Some(v);
    }

    pub fn get_games_played_total(&self) -> u32 {
        self.games_played_total.unwrap_or(0)
    }

    fn get_games_played_total_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.games_played_total
    }

    fn mut_games_played_total_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.games_played_total
    }

    // optional uint32 games_played_matchmaking = 20;

    pub fn clear_games_played_matchmaking(&mut self) {
        self.games_played_matchmaking = ::std::option::Option::None;
    }

    pub fn has_games_played_matchmaking(&self) -> bool {
        self.games_played_matchmaking.is_some()
    }

    // Param is passed by value, moved
    pub fn set_games_played_matchmaking(&mut self, v: u32) {
        self.games_played_matchmaking = ::std::option::Option::Some(v);
    }

    pub fn get_games_played_matchmaking(&self) -> u32 {
        self.games_played_matchmaking.unwrap_or(0)
    }

    fn get_games_played_matchmaking_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.games_played_matchmaking
    }

    fn mut_games_played_matchmaking_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.games_played_matchmaking
    }

    // repeated uint32 leagues_participated = 21;

    pub fn clear_leagues_participated(&mut self) {
        self.leagues_participated.clear();
    }

    // Param is passed by value, moved
    pub fn set_leagues_participated(&mut self, v: ::std::vec::Vec<u32>) {
        self.leagues_participated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_leagues_participated(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.leagues_participated
    }

    // Take field
    pub fn take_leagues_participated(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.leagues_participated, ::std::vec::Vec::new())
    }

    pub fn get_leagues_participated(&self) -> &[u32] {
        &self.leagues_participated
    }

    fn get_leagues_participated_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.leagues_participated
    }

    fn mut_leagues_participated_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.leagues_participated
    }

    // repeated uint64 top_match_ids = 22;

    pub fn clear_top_match_ids(&mut self) {
        self.top_match_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_top_match_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.top_match_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_top_match_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.top_match_ids
    }

    // Take field
    pub fn take_top_match_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.top_match_ids, ::std::vec::Vec::new())
    }

    pub fn get_top_match_ids(&self) -> &[u64] {
        &self.top_match_ids
    }

    fn get_top_match_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.top_match_ids
    }

    fn mut_top_match_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.top_match_ids
    }

    // repeated uint64 recent_match_ids = 23;

    pub fn clear_recent_match_ids(&mut self) {
        self.recent_match_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_recent_match_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.recent_match_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_recent_match_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.recent_match_ids
    }

    // Take field
    pub fn take_recent_match_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.recent_match_ids, ::std::vec::Vec::new())
    }

    pub fn get_recent_match_ids(&self) -> &[u64] {
        &self.recent_match_ids
    }

    fn get_recent_match_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.recent_match_ids
    }

    fn mut_recent_match_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.recent_match_ids
    }
}

impl ::protobuf::Message for CMsgDOTATeamInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_created = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.pro = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.locked = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.pickup_team = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.ugc_logo = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.ugc_base_logo = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.ugc_banner_logo = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.ugc_sponsor_logo = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country_code)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.wins = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.losses = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.rank = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.calibration_games_remaining = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.games_played_total = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.games_played_matchmaking = ::std::option::Option::Some(tmp);
                },
                21 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.leagues_participated)?;
                },
                22 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.top_match_ids)?;
                },
                23 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.recent_match_ids)?;
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
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.time_created {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pro {
            my_size += 2;
        };
        if let Some(v) = self.locked {
            my_size += 2;
        };
        if let Some(v) = self.pickup_team {
            my_size += 2;
        };
        if let Some(v) = self.ugc_logo {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ugc_base_logo {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ugc_banner_logo {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ugc_sponsor_logo {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.country_code.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        };
        if let Some(v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        };
        if let Some(v) = self.wins {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.losses {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.rank {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.calibration_games_remaining {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.games_played_total {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.games_played_matchmaking {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.leagues_participated {
            my_size += ::protobuf::rt::value_size(21, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.top_match_ids {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.recent_match_ids {
            my_size += ::protobuf::rt::value_size(23, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.members {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.team_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.tag.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.time_created {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.pro {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.locked {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.pickup_team {
            os.write_bool(8, v)?;
        };
        if let Some(v) = self.ugc_logo {
            os.write_uint64(9, v)?;
        };
        if let Some(v) = self.ugc_base_logo {
            os.write_uint64(10, v)?;
        };
        if let Some(v) = self.ugc_banner_logo {
            os.write_uint64(11, v)?;
        };
        if let Some(v) = self.ugc_sponsor_logo {
            os.write_uint64(12, v)?;
        };
        if let Some(v) = self.country_code.as_ref() {
            os.write_string(13, &v)?;
        };
        if let Some(v) = self.url.as_ref() {
            os.write_string(14, &v)?;
        };
        if let Some(v) = self.wins {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.losses {
            os.write_uint32(16, v)?;
        };
        if let Some(v) = self.rank {
            os.write_uint32(17, v)?;
        };
        if let Some(v) = self.calibration_games_remaining {
            os.write_uint32(18, v)?;
        };
        if let Some(v) = self.games_played_total {
            os.write_uint32(19, v)?;
        };
        if let Some(v) = self.games_played_matchmaking {
            os.write_uint32(20, v)?;
        };
        for v in &self.leagues_participated {
            os.write_uint32(21, *v)?;
        };
        for v in &self.top_match_ids {
            os.write_uint64(22, *v)?;
        };
        for v in &self.recent_match_ids {
            os.write_uint64(23, *v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATeamInfo {
    fn new() -> CMsgDOTATeamInfo {
        CMsgDOTATeamInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATeamInfo_Member>>(
                    "members",
                    CMsgDOTATeamInfo::get_members_for_reflect,
                    CMsgDOTATeamInfo::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTATeamInfo::get_team_id_for_reflect,
                    CMsgDOTATeamInfo::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgDOTATeamInfo::get_name_for_reflect,
                    CMsgDOTATeamInfo::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    CMsgDOTATeamInfo::get_tag_for_reflect,
                    CMsgDOTATeamInfo::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_created",
                    CMsgDOTATeamInfo::get_time_created_for_reflect,
                    CMsgDOTATeamInfo::mut_time_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "pro",
                    CMsgDOTATeamInfo::get_pro_for_reflect,
                    CMsgDOTATeamInfo::mut_pro_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "locked",
                    CMsgDOTATeamInfo::get_locked_for_reflect,
                    CMsgDOTATeamInfo::mut_locked_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "pickup_team",
                    CMsgDOTATeamInfo::get_pickup_team_for_reflect,
                    CMsgDOTATeamInfo::mut_pickup_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ugc_logo",
                    CMsgDOTATeamInfo::get_ugc_logo_for_reflect,
                    CMsgDOTATeamInfo::mut_ugc_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ugc_base_logo",
                    CMsgDOTATeamInfo::get_ugc_base_logo_for_reflect,
                    CMsgDOTATeamInfo::mut_ugc_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ugc_banner_logo",
                    CMsgDOTATeamInfo::get_ugc_banner_logo_for_reflect,
                    CMsgDOTATeamInfo::mut_ugc_banner_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ugc_sponsor_logo",
                    CMsgDOTATeamInfo::get_ugc_sponsor_logo_for_reflect,
                    CMsgDOTATeamInfo::mut_ugc_sponsor_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country_code",
                    CMsgDOTATeamInfo::get_country_code_for_reflect,
                    CMsgDOTATeamInfo::mut_country_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    CMsgDOTATeamInfo::get_url_for_reflect,
                    CMsgDOTATeamInfo::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "wins",
                    CMsgDOTATeamInfo::get_wins_for_reflect,
                    CMsgDOTATeamInfo::mut_wins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "losses",
                    CMsgDOTATeamInfo::get_losses_for_reflect,
                    CMsgDOTATeamInfo::mut_losses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "rank",
                    CMsgDOTATeamInfo::get_rank_for_reflect,
                    CMsgDOTATeamInfo::mut_rank_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "calibration_games_remaining",
                    CMsgDOTATeamInfo::get_calibration_games_remaining_for_reflect,
                    CMsgDOTATeamInfo::mut_calibration_games_remaining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "games_played_total",
                    CMsgDOTATeamInfo::get_games_played_total_for_reflect,
                    CMsgDOTATeamInfo::mut_games_played_total_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "games_played_matchmaking",
                    CMsgDOTATeamInfo::get_games_played_matchmaking_for_reflect,
                    CMsgDOTATeamInfo::mut_games_played_matchmaking_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leagues_participated",
                    CMsgDOTATeamInfo::get_leagues_participated_for_reflect,
                    CMsgDOTATeamInfo::mut_leagues_participated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "top_match_ids",
                    CMsgDOTATeamInfo::get_top_match_ids_for_reflect,
                    CMsgDOTATeamInfo::mut_top_match_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "recent_match_ids",
                    CMsgDOTATeamInfo::get_recent_match_ids_for_reflect,
                    CMsgDOTATeamInfo::mut_recent_match_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamInfo>(
                    "CMsgDOTATeamInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamInfo {
    fn clear(&mut self) {
        self.clear_members();
        self.clear_team_id();
        self.clear_name();
        self.clear_tag();
        self.clear_time_created();
        self.clear_pro();
        self.clear_locked();
        self.clear_pickup_team();
        self.clear_ugc_logo();
        self.clear_ugc_base_logo();
        self.clear_ugc_banner_logo();
        self.clear_ugc_sponsor_logo();
        self.clear_country_code();
        self.clear_url();
        self.clear_wins();
        self.clear_losses();
        self.clear_rank();
        self.clear_calibration_games_remaining();
        self.clear_games_played_total();
        self.clear_games_played_matchmaking();
        self.clear_leagues_participated();
        self.clear_top_match_ids();
        self.clear_recent_match_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamInfo_Member {
    // message fields
    account_id: ::std::option::Option<u32>,
    time_joined: ::std::option::Option<u32>,
    admin: ::std::option::Option<bool>,
    sub: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamInfo_Member {}

impl CMsgDOTATeamInfo_Member {
    pub fn new() -> CMsgDOTATeamInfo_Member {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamInfo_Member {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamInfo_Member> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamInfo_Member,
        };
        unsafe {
            instance.get(CMsgDOTATeamInfo_Member::new)
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

    // optional bool admin = 3;

    pub fn clear_admin(&mut self) {
        self.admin = ::std::option::Option::None;
    }

    pub fn has_admin(&self) -> bool {
        self.admin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin(&mut self, v: bool) {
        self.admin = ::std::option::Option::Some(v);
    }

    pub fn get_admin(&self) -> bool {
        self.admin.unwrap_or(false)
    }

    fn get_admin_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.admin
    }

    fn mut_admin_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.admin
    }

    // optional bool sub = 4;

    pub fn clear_sub(&mut self) {
        self.sub = ::std::option::Option::None;
    }

    pub fn has_sub(&self) -> bool {
        self.sub.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sub(&mut self, v: bool) {
        self.sub = ::std::option::Option::Some(v);
    }

    pub fn get_sub(&self) -> bool {
        self.sub.unwrap_or(false)
    }

    fn get_sub_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.sub
    }

    fn mut_sub_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.sub
    }
}

impl ::protobuf::Message for CMsgDOTATeamInfo_Member {
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
                    let tmp = is.read_bool()?;
                    self.admin = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.sub = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.admin {
            my_size += 2;
        };
        if let Some(v) = self.sub {
            my_size += 2;
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
        if let Some(v) = self.admin {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.sub {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTATeamInfo_Member {
    fn new() -> CMsgDOTATeamInfo_Member {
        CMsgDOTATeamInfo_Member::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamInfo_Member>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTATeamInfo_Member::get_account_id_for_reflect,
                    CMsgDOTATeamInfo_Member::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_joined",
                    CMsgDOTATeamInfo_Member::get_time_joined_for_reflect,
                    CMsgDOTATeamInfo_Member::mut_time_joined_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "admin",
                    CMsgDOTATeamInfo_Member::get_admin_for_reflect,
                    CMsgDOTATeamInfo_Member::mut_admin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sub",
                    CMsgDOTATeamInfo_Member::get_sub_for_reflect,
                    CMsgDOTATeamInfo_Member::mut_sub_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamInfo_Member>(
                    "CMsgDOTATeamInfo_Member",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamInfo_Member {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_time_joined();
        self.clear_admin();
        self.clear_sub();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamInfo_Member {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamInfo_Member {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamsInfo {
    // message fields
    league_id: ::std::option::Option<u32>,
    teams: ::protobuf::RepeatedField<CMsgDOTATeamInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamsInfo {}

impl CMsgDOTATeamsInfo {
    pub fn new() -> CMsgDOTATeamsInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamsInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamsInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamsInfo,
        };
        unsafe {
            instance.get(CMsgDOTATeamsInfo::new)
        }
    }

    // optional uint32 league_id = 1;

    pub fn clear_league_id(&mut self) {
        self.league_id = ::std::option::Option::None;
    }

    pub fn has_league_id(&self) -> bool {
        self.league_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_id(&mut self, v: u32) {
        self.league_id = ::std::option::Option::Some(v);
    }

    pub fn get_league_id(&self) -> u32 {
        self.league_id.unwrap_or(0)
    }

    fn get_league_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_id
    }

    fn mut_league_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_id
    }

    // repeated .dota.CMsgDOTATeamInfo teams = 2;

    pub fn clear_teams(&mut self) {
        self.teams.clear();
    }

    // Param is passed by value, moved
    pub fn set_teams(&mut self, v: ::protobuf::RepeatedField<CMsgDOTATeamInfo>) {
        self.teams = v;
    }

    // Mutable pointer to the field.
    pub fn mut_teams(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATeamInfo> {
        &mut self.teams
    }

    // Take field
    pub fn take_teams(&mut self) -> ::protobuf::RepeatedField<CMsgDOTATeamInfo> {
        ::std::mem::replace(&mut self.teams, ::protobuf::RepeatedField::new())
    }

    pub fn get_teams(&self) -> &[CMsgDOTATeamInfo] {
        &self.teams
    }

    fn get_teams_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTATeamInfo> {
        &self.teams
    }

    fn mut_teams_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTATeamInfo> {
        &mut self.teams
    }
}

impl ::protobuf::Message for CMsgDOTATeamsInfo {
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
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.teams)?;
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
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.teams {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.league_id {
            os.write_uint32(1, v)?;
        };
        for v in &self.teams {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamsInfo {
    fn new() -> CMsgDOTATeamsInfo {
        CMsgDOTATeamsInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamsInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgDOTATeamsInfo::get_league_id_for_reflect,
                    CMsgDOTATeamsInfo::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATeamInfo>>(
                    "teams",
                    CMsgDOTATeamsInfo::get_teams_for_reflect,
                    CMsgDOTATeamsInfo::mut_teams_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamsInfo>(
                    "CMsgDOTATeamsInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamsInfo {
    fn clear(&mut self) {
        self.clear_league_id();
        self.clear_teams();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamsInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamsInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAMyTeamInfoRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAMyTeamInfoRequest {}

impl CMsgDOTAMyTeamInfoRequest {
    pub fn new() -> CMsgDOTAMyTeamInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAMyTeamInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAMyTeamInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAMyTeamInfoRequest,
        };
        unsafe {
            instance.get(CMsgDOTAMyTeamInfoRequest::new)
        }
    }
}

impl ::protobuf::Message for CMsgDOTAMyTeamInfoRequest {
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

impl ::protobuf::MessageStatic for CMsgDOTAMyTeamInfoRequest {
    fn new() -> CMsgDOTAMyTeamInfoRequest {
        CMsgDOTAMyTeamInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAMyTeamInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAMyTeamInfoRequest>(
                    "CMsgDOTAMyTeamInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAMyTeamInfoRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAMyTeamInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAMyTeamInfoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTACreateTeam {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    tag: ::protobuf::SingularField<::std::string::String>,
    logo: ::std::option::Option<u64>,
    base_logo: ::std::option::Option<u64>,
    banner_logo: ::std::option::Option<u64>,
    sponsor_logo: ::std::option::Option<u64>,
    country_code: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    pickup_team: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTACreateTeam {}

impl CMsgDOTACreateTeam {
    pub fn new() -> CMsgDOTACreateTeam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTACreateTeam {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTACreateTeam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTACreateTeam,
        };
        unsafe {
            instance.get(CMsgDOTACreateTeam::new)
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

    // optional uint64 sponsor_logo = 6;

    pub fn clear_sponsor_logo(&mut self) {
        self.sponsor_logo = ::std::option::Option::None;
    }

    pub fn has_sponsor_logo(&self) -> bool {
        self.sponsor_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sponsor_logo(&mut self, v: u64) {
        self.sponsor_logo = ::std::option::Option::Some(v);
    }

    pub fn get_sponsor_logo(&self) -> u64 {
        self.sponsor_logo.unwrap_or(0)
    }

    fn get_sponsor_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sponsor_logo
    }

    fn mut_sponsor_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sponsor_logo
    }

    // optional string country_code = 7;

    pub fn clear_country_code(&mut self) {
        self.country_code.clear();
    }

    pub fn has_country_code(&self) -> bool {
        self.country_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country_code(&mut self, v: ::std::string::String) {
        self.country_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country_code(&mut self) -> &mut ::std::string::String {
        if self.country_code.is_none() {
            self.country_code.set_default();
        };
        self.country_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_country_code(&mut self) -> ::std::string::String {
        self.country_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country_code(&self) -> &str {
        match self.country_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country_code
    }

    fn mut_country_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country_code
    }

    // optional string url = 8;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }

    // optional bool pickup_team = 9;

    pub fn clear_pickup_team(&mut self) {
        self.pickup_team = ::std::option::Option::None;
    }

    pub fn has_pickup_team(&self) -> bool {
        self.pickup_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pickup_team(&mut self, v: bool) {
        self.pickup_team = ::std::option::Option::Some(v);
    }

    pub fn get_pickup_team(&self) -> bool {
        self.pickup_team.unwrap_or(false)
    }

    fn get_pickup_team_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.pickup_team
    }

    fn mut_pickup_team_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.pickup_team
    }
}

impl ::protobuf::Message for CMsgDOTACreateTeam {
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
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.sponsor_logo = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country_code)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.pickup_team = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.sponsor_logo {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.country_code.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let Some(v) = self.pickup_team {
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
        if let Some(v) = self.sponsor_logo {
            os.write_uint64(6, v)?;
        };
        if let Some(v) = self.country_code.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.url.as_ref() {
            os.write_string(8, &v)?;
        };
        if let Some(v) = self.pickup_team {
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

impl ::protobuf::MessageStatic for CMsgDOTACreateTeam {
    fn new() -> CMsgDOTACreateTeam {
        CMsgDOTACreateTeam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTACreateTeam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgDOTACreateTeam::get_name_for_reflect,
                    CMsgDOTACreateTeam::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    CMsgDOTACreateTeam::get_tag_for_reflect,
                    CMsgDOTACreateTeam::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTACreateTeam::get_logo_for_reflect,
                    CMsgDOTACreateTeam::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "base_logo",
                    CMsgDOTACreateTeam::get_base_logo_for_reflect,
                    CMsgDOTACreateTeam::mut_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "banner_logo",
                    CMsgDOTACreateTeam::get_banner_logo_for_reflect,
                    CMsgDOTACreateTeam::mut_banner_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sponsor_logo",
                    CMsgDOTACreateTeam::get_sponsor_logo_for_reflect,
                    CMsgDOTACreateTeam::mut_sponsor_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country_code",
                    CMsgDOTACreateTeam::get_country_code_for_reflect,
                    CMsgDOTACreateTeam::mut_country_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    CMsgDOTACreateTeam::get_url_for_reflect,
                    CMsgDOTACreateTeam::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "pickup_team",
                    CMsgDOTACreateTeam::get_pickup_team_for_reflect,
                    CMsgDOTACreateTeam::mut_pickup_team_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTACreateTeam>(
                    "CMsgDOTACreateTeam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTACreateTeam {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_tag();
        self.clear_logo();
        self.clear_base_logo();
        self.clear_banner_logo();
        self.clear_sponsor_logo();
        self.clear_country_code();
        self.clear_url();
        self.clear_pickup_team();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTACreateTeam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTACreateTeam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTACreateTeamResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTACreateTeamResponse_Result>,
    team_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTACreateTeamResponse {}

impl CMsgDOTACreateTeamResponse {
    pub fn new() -> CMsgDOTACreateTeamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTACreateTeamResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTACreateTeamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTACreateTeamResponse,
        };
        unsafe {
            instance.get(CMsgDOTACreateTeamResponse::new)
        }
    }

    // optional .dota.CMsgDOTACreateTeamResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTACreateTeamResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTACreateTeamResponse_Result {
        self.result.unwrap_or(CMsgDOTACreateTeamResponse_Result::INVALID)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTACreateTeamResponse_Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTACreateTeamResponse_Result> {
        &mut self.result
    }

    // optional uint32 team_id = 2;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }
}

impl ::protobuf::Message for CMsgDOTACreateTeamResponse {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.team_id {
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

impl ::protobuf::MessageStatic for CMsgDOTACreateTeamResponse {
    fn new() -> CMsgDOTACreateTeamResponse {
        CMsgDOTACreateTeamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTACreateTeamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTACreateTeamResponse_Result>>(
                    "result",
                    CMsgDOTACreateTeamResponse::get_result_for_reflect,
                    CMsgDOTACreateTeamResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTACreateTeamResponse::get_team_id_for_reflect,
                    CMsgDOTACreateTeamResponse::mut_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTACreateTeamResponse>(
                    "CMsgDOTACreateTeamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTACreateTeamResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTACreateTeamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTACreateTeamResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTACreateTeamResponse_Result {
    INVALID = -1,
    SUCCESS = 0,
    NAME_EMPTY = 1,
    NAME_BAD_CHARACTERS = 2,
    NAME_TAKEN = 3,
    NAME_TOO_LONG = 4,
    TAG_EMPTY = 5,
    TAG_BAD_CHARACTERS = 6,
    TAG_TAKEN = 7,
    TAG_TOO_LONG = 8,
    CREATOR_BUSY = 9,
    UNSPECIFIED_ERROR = 10,
    CREATOR_TEAM_LIMIT_REACHED = 11,
    NO_LOGO = 12,
    CREATOR_TEAM_CREATION_COOLDOWN = 13,
    LOGO_UPLOAD_FAILED = 14,
    NAME_CHANGED_TOO_RECENTLY = 15,
    CREATOR_INSUFFICIENT_LEVEL = 16,
    INVALID_ACCOUNT_TYPE = 17,
}

impl ::protobuf::ProtobufEnum for CMsgDOTACreateTeamResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTACreateTeamResponse_Result> {
        match value {
            -1 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::INVALID),
            0 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::NAME_EMPTY),
            2 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::NAME_BAD_CHARACTERS),
            3 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::NAME_TAKEN),
            4 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::NAME_TOO_LONG),
            5 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::TAG_EMPTY),
            6 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::TAG_BAD_CHARACTERS),
            7 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::TAG_TAKEN),
            8 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::TAG_TOO_LONG),
            9 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::CREATOR_BUSY),
            10 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::UNSPECIFIED_ERROR),
            11 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::CREATOR_TEAM_LIMIT_REACHED),
            12 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::NO_LOGO),
            13 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::CREATOR_TEAM_CREATION_COOLDOWN),
            14 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::LOGO_UPLOAD_FAILED),
            15 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::NAME_CHANGED_TOO_RECENTLY),
            16 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::CREATOR_INSUFFICIENT_LEVEL),
            17 => ::std::option::Option::Some(CMsgDOTACreateTeamResponse_Result::INVALID_ACCOUNT_TYPE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTACreateTeamResponse_Result] = &[
            CMsgDOTACreateTeamResponse_Result::INVALID,
            CMsgDOTACreateTeamResponse_Result::SUCCESS,
            CMsgDOTACreateTeamResponse_Result::NAME_EMPTY,
            CMsgDOTACreateTeamResponse_Result::NAME_BAD_CHARACTERS,
            CMsgDOTACreateTeamResponse_Result::NAME_TAKEN,
            CMsgDOTACreateTeamResponse_Result::NAME_TOO_LONG,
            CMsgDOTACreateTeamResponse_Result::TAG_EMPTY,
            CMsgDOTACreateTeamResponse_Result::TAG_BAD_CHARACTERS,
            CMsgDOTACreateTeamResponse_Result::TAG_TAKEN,
            CMsgDOTACreateTeamResponse_Result::TAG_TOO_LONG,
            CMsgDOTACreateTeamResponse_Result::CREATOR_BUSY,
            CMsgDOTACreateTeamResponse_Result::UNSPECIFIED_ERROR,
            CMsgDOTACreateTeamResponse_Result::CREATOR_TEAM_LIMIT_REACHED,
            CMsgDOTACreateTeamResponse_Result::NO_LOGO,
            CMsgDOTACreateTeamResponse_Result::CREATOR_TEAM_CREATION_COOLDOWN,
            CMsgDOTACreateTeamResponse_Result::LOGO_UPLOAD_FAILED,
            CMsgDOTACreateTeamResponse_Result::NAME_CHANGED_TOO_RECENTLY,
            CMsgDOTACreateTeamResponse_Result::CREATOR_INSUFFICIENT_LEVEL,
            CMsgDOTACreateTeamResponse_Result::INVALID_ACCOUNT_TYPE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTACreateTeamResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTACreateTeamResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTACreateTeamResponse_Result {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTACreateTeamResponse_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAEditTeamDetails {
    // message fields
    team_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    tag: ::protobuf::SingularField<::std::string::String>,
    logo: ::std::option::Option<u64>,
    base_logo: ::std::option::Option<u64>,
    banner_logo: ::std::option::Option<u64>,
    sponsor_logo: ::std::option::Option<u64>,
    country_code: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    in_use_by_party: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAEditTeamDetails {}

impl CMsgDOTAEditTeamDetails {
    pub fn new() -> CMsgDOTAEditTeamDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAEditTeamDetails {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAEditTeamDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAEditTeamDetails,
        };
        unsafe {
            instance.get(CMsgDOTAEditTeamDetails::new)
        }
    }

    // optional uint32 team_id = 1;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
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

    // optional uint64 logo = 4;

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

    // optional uint64 base_logo = 5;

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

    // optional uint64 banner_logo = 6;

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

    // optional uint64 sponsor_logo = 7;

    pub fn clear_sponsor_logo(&mut self) {
        self.sponsor_logo = ::std::option::Option::None;
    }

    pub fn has_sponsor_logo(&self) -> bool {
        self.sponsor_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sponsor_logo(&mut self, v: u64) {
        self.sponsor_logo = ::std::option::Option::Some(v);
    }

    pub fn get_sponsor_logo(&self) -> u64 {
        self.sponsor_logo.unwrap_or(0)
    }

    fn get_sponsor_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sponsor_logo
    }

    fn mut_sponsor_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sponsor_logo
    }

    // optional string country_code = 8;

    pub fn clear_country_code(&mut self) {
        self.country_code.clear();
    }

    pub fn has_country_code(&self) -> bool {
        self.country_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country_code(&mut self, v: ::std::string::String) {
        self.country_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country_code(&mut self) -> &mut ::std::string::String {
        if self.country_code.is_none() {
            self.country_code.set_default();
        };
        self.country_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_country_code(&mut self) -> ::std::string::String {
        self.country_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country_code(&self) -> &str {
        match self.country_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country_code
    }

    fn mut_country_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country_code
    }

    // optional string url = 9;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        };
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }

    // optional bool in_use_by_party = 10;

    pub fn clear_in_use_by_party(&mut self) {
        self.in_use_by_party = ::std::option::Option::None;
    }

    pub fn has_in_use_by_party(&self) -> bool {
        self.in_use_by_party.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_use_by_party(&mut self, v: bool) {
        self.in_use_by_party = ::std::option::Option::Some(v);
    }

    pub fn get_in_use_by_party(&self) -> bool {
        self.in_use_by_party.unwrap_or(false)
    }

    fn get_in_use_by_party_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.in_use_by_party
    }

    fn mut_in_use_by_party_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.in_use_by_party
    }
}

impl ::protobuf::Message for CMsgDOTAEditTeamDetails {
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
                    self.team_id = ::std::option::Option::Some(tmp);
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
                    let tmp = is.read_uint64()?;
                    self.logo = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.base_logo = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.banner_logo = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.sponsor_logo = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country_code)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.in_use_by_party = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.base_logo {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.banner_logo {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sponsor_logo {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.country_code.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let Some(v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        };
        if let Some(v) = self.in_use_by_party {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.tag.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.logo {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.base_logo {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.banner_logo {
            os.write_uint64(6, v)?;
        };
        if let Some(v) = self.sponsor_logo {
            os.write_uint64(7, v)?;
        };
        if let Some(v) = self.country_code.as_ref() {
            os.write_string(8, &v)?;
        };
        if let Some(v) = self.url.as_ref() {
            os.write_string(9, &v)?;
        };
        if let Some(v) = self.in_use_by_party {
            os.write_bool(10, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTAEditTeamDetails {
    fn new() -> CMsgDOTAEditTeamDetails {
        CMsgDOTAEditTeamDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAEditTeamDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTAEditTeamDetails::get_team_id_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgDOTAEditTeamDetails::get_name_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    CMsgDOTAEditTeamDetails::get_tag_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTAEditTeamDetails::get_logo_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "base_logo",
                    CMsgDOTAEditTeamDetails::get_base_logo_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "banner_logo",
                    CMsgDOTAEditTeamDetails::get_banner_logo_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_banner_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sponsor_logo",
                    CMsgDOTAEditTeamDetails::get_sponsor_logo_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_sponsor_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country_code",
                    CMsgDOTAEditTeamDetails::get_country_code_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_country_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    CMsgDOTAEditTeamDetails::get_url_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "in_use_by_party",
                    CMsgDOTAEditTeamDetails::get_in_use_by_party_for_reflect,
                    CMsgDOTAEditTeamDetails::mut_in_use_by_party_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAEditTeamDetails>(
                    "CMsgDOTAEditTeamDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAEditTeamDetails {
    fn clear(&mut self) {
        self.clear_team_id();
        self.clear_name();
        self.clear_tag();
        self.clear_logo();
        self.clear_base_logo();
        self.clear_banner_logo();
        self.clear_sponsor_logo();
        self.clear_country_code();
        self.clear_url();
        self.clear_in_use_by_party();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAEditTeamDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAEditTeamDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAEditTeamDetailsResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAEditTeamDetailsResponse_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAEditTeamDetailsResponse {}

impl CMsgDOTAEditTeamDetailsResponse {
    pub fn new() -> CMsgDOTAEditTeamDetailsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAEditTeamDetailsResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAEditTeamDetailsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAEditTeamDetailsResponse,
        };
        unsafe {
            instance.get(CMsgDOTAEditTeamDetailsResponse::new)
        }
    }

    // optional .dota.CMsgDOTAEditTeamDetailsResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAEditTeamDetailsResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAEditTeamDetailsResponse_Result {
        self.result.unwrap_or(CMsgDOTAEditTeamDetailsResponse_Result::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAEditTeamDetailsResponse_Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAEditTeamDetailsResponse_Result> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAEditTeamDetailsResponse {
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

impl ::protobuf::MessageStatic for CMsgDOTAEditTeamDetailsResponse {
    fn new() -> CMsgDOTAEditTeamDetailsResponse {
        CMsgDOTAEditTeamDetailsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAEditTeamDetailsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAEditTeamDetailsResponse_Result>>(
                    "result",
                    CMsgDOTAEditTeamDetailsResponse::get_result_for_reflect,
                    CMsgDOTAEditTeamDetailsResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAEditTeamDetailsResponse>(
                    "CMsgDOTAEditTeamDetailsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAEditTeamDetailsResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAEditTeamDetailsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAEditTeamDetailsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAEditTeamDetailsResponse_Result {
    SUCCESS = 0,
    FAILURE_INVALID_ACCOUNT_TYPE = 1,
    FAILURE_NOT_MEMBER = 2,
    FAILURE_TEAM_LOCKED = 3,
    FAILURE_UNSPECIFIED_ERROR = 4,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAEditTeamDetailsResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAEditTeamDetailsResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAEditTeamDetailsResponse_Result::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAEditTeamDetailsResponse_Result::FAILURE_INVALID_ACCOUNT_TYPE),
            2 => ::std::option::Option::Some(CMsgDOTAEditTeamDetailsResponse_Result::FAILURE_NOT_MEMBER),
            3 => ::std::option::Option::Some(CMsgDOTAEditTeamDetailsResponse_Result::FAILURE_TEAM_LOCKED),
            4 => ::std::option::Option::Some(CMsgDOTAEditTeamDetailsResponse_Result::FAILURE_UNSPECIFIED_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAEditTeamDetailsResponse_Result] = &[
            CMsgDOTAEditTeamDetailsResponse_Result::SUCCESS,
            CMsgDOTAEditTeamDetailsResponse_Result::FAILURE_INVALID_ACCOUNT_TYPE,
            CMsgDOTAEditTeamDetailsResponse_Result::FAILURE_NOT_MEMBER,
            CMsgDOTAEditTeamDetailsResponse_Result::FAILURE_TEAM_LOCKED,
            CMsgDOTAEditTeamDetailsResponse_Result::FAILURE_UNSPECIFIED_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAEditTeamDetailsResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAEditTeamDetailsResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAEditTeamDetailsResponse_Result {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAEditTeamDetailsResponse_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamProfileResponse {
    // message fields
    eresult: ::std::option::Option<u32>,
    team: ::protobuf::SingularPtrField<CMsgDOTATeam>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamProfileResponse {}

impl CMsgDOTATeamProfileResponse {
    pub fn new() -> CMsgDOTATeamProfileResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamProfileResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamProfileResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamProfileResponse,
        };
        unsafe {
            instance.get(CMsgDOTATeamProfileResponse::new)
        }
    }

    // optional uint32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: u32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> u32 {
        self.eresult.unwrap_or(0)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.eresult
    }

    // optional .dota.CMsgDOTATeam team = 2;

    pub fn clear_team(&mut self) {
        self.team.clear();
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: CMsgDOTATeam) {
        self.team = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team(&mut self) -> &mut CMsgDOTATeam {
        if self.team.is_none() {
            self.team.set_default();
        };
        self.team.as_mut().unwrap()
    }

    // Take field
    pub fn take_team(&mut self) -> CMsgDOTATeam {
        self.team.take().unwrap_or_else(|| CMsgDOTATeam::new())
    }

    pub fn get_team(&self) -> &CMsgDOTATeam {
        self.team.as_ref().unwrap_or_else(|| CMsgDOTATeam::default_instance())
    }

    fn get_team_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgDOTATeam> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgDOTATeam> {
        &mut self.team
    }
}

impl ::protobuf::Message for CMsgDOTATeamProfileResponse {
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
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.team)?;
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
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.team.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamProfileResponse {
    fn new() -> CMsgDOTATeamProfileResponse {
        CMsgDOTATeamProfileResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamProfileResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "eresult",
                    CMsgDOTATeamProfileResponse::get_eresult_for_reflect,
                    CMsgDOTATeamProfileResponse::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTATeam>>(
                    "team",
                    CMsgDOTATeamProfileResponse::get_team_for_reflect,
                    CMsgDOTATeamProfileResponse::mut_team_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamProfileResponse>(
                    "CMsgDOTATeamProfileResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamProfileResponse {
    fn clear(&mut self) {
        self.clear_eresult();
        self.clear_team();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamProfileResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamProfileResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAProTeamListRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAProTeamListRequest {}

impl CMsgDOTAProTeamListRequest {
    pub fn new() -> CMsgDOTAProTeamListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAProTeamListRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAProTeamListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAProTeamListRequest,
        };
        unsafe {
            instance.get(CMsgDOTAProTeamListRequest::new)
        }
    }
}

impl ::protobuf::Message for CMsgDOTAProTeamListRequest {
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

impl ::protobuf::MessageStatic for CMsgDOTAProTeamListRequest {
    fn new() -> CMsgDOTAProTeamListRequest {
        CMsgDOTAProTeamListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAProTeamListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAProTeamListRequest>(
                    "CMsgDOTAProTeamListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAProTeamListRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAProTeamListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAProTeamListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAProTeamListResponse {
    // message fields
    teams: ::protobuf::RepeatedField<CMsgDOTAProTeamListResponse_TeamEntry>,
    eresult: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAProTeamListResponse {}

impl CMsgDOTAProTeamListResponse {
    pub fn new() -> CMsgDOTAProTeamListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAProTeamListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAProTeamListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAProTeamListResponse,
        };
        unsafe {
            instance.get(CMsgDOTAProTeamListResponse::new)
        }
    }

    // repeated .dota.CMsgDOTAProTeamListResponse.TeamEntry teams = 1;

    pub fn clear_teams(&mut self) {
        self.teams.clear();
    }

    // Param is passed by value, moved
    pub fn set_teams(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAProTeamListResponse_TeamEntry>) {
        self.teams = v;
    }

    // Mutable pointer to the field.
    pub fn mut_teams(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAProTeamListResponse_TeamEntry> {
        &mut self.teams
    }

    // Take field
    pub fn take_teams(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAProTeamListResponse_TeamEntry> {
        ::std::mem::replace(&mut self.teams, ::protobuf::RepeatedField::new())
    }

    pub fn get_teams(&self) -> &[CMsgDOTAProTeamListResponse_TeamEntry] {
        &self.teams
    }

    fn get_teams_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAProTeamListResponse_TeamEntry> {
        &self.teams
    }

    fn mut_teams_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAProTeamListResponse_TeamEntry> {
        &mut self.teams
    }

    // optional uint32 eresult = 2;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: u32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> u32 {
        self.eresult.unwrap_or(0)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.eresult
    }
}

impl ::protobuf::Message for CMsgDOTAProTeamListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.teams)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
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
        for value in &self.teams {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.teams {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.eresult {
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

impl ::protobuf::MessageStatic for CMsgDOTAProTeamListResponse {
    fn new() -> CMsgDOTAProTeamListResponse {
        CMsgDOTAProTeamListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAProTeamListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAProTeamListResponse_TeamEntry>>(
                    "teams",
                    CMsgDOTAProTeamListResponse::get_teams_for_reflect,
                    CMsgDOTAProTeamListResponse::mut_teams_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "eresult",
                    CMsgDOTAProTeamListResponse::get_eresult_for_reflect,
                    CMsgDOTAProTeamListResponse::mut_eresult_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAProTeamListResponse>(
                    "CMsgDOTAProTeamListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAProTeamListResponse {
    fn clear(&mut self) {
        self.clear_teams();
        self.clear_eresult();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAProTeamListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAProTeamListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAProTeamListResponse_TeamEntry {
    // message fields
    team_id: ::std::option::Option<u32>,
    tag: ::protobuf::SingularField<::std::string::String>,
    time_created: ::std::option::Option<u32>,
    logo: ::std::option::Option<u64>,
    country_code: ::protobuf::SingularField<::std::string::String>,
    member_count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAProTeamListResponse_TeamEntry {}

impl CMsgDOTAProTeamListResponse_TeamEntry {
    pub fn new() -> CMsgDOTAProTeamListResponse_TeamEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAProTeamListResponse_TeamEntry {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAProTeamListResponse_TeamEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAProTeamListResponse_TeamEntry,
        };
        unsafe {
            instance.get(CMsgDOTAProTeamListResponse_TeamEntry::new)
        }
    }

    // optional uint32 team_id = 1;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
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

    // optional uint64 logo = 4;

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

    // optional string country_code = 5;

    pub fn clear_country_code(&mut self) {
        self.country_code.clear();
    }

    pub fn has_country_code(&self) -> bool {
        self.country_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country_code(&mut self, v: ::std::string::String) {
        self.country_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country_code(&mut self) -> &mut ::std::string::String {
        if self.country_code.is_none() {
            self.country_code.set_default();
        };
        self.country_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_country_code(&mut self) -> ::std::string::String {
        self.country_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country_code(&self) -> &str {
        match self.country_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country_code
    }

    fn mut_country_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country_code
    }

    // optional uint32 member_count = 6;

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

impl ::protobuf::Message for CMsgDOTAProTeamListResponse_TeamEntry {
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
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.tag)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.time_created = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.logo = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country_code)?;
                },
                6 => {
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tag.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.time_created {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.country_code.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.member_count {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.tag.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.time_created {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.logo {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.country_code.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.member_count {
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

impl ::protobuf::MessageStatic for CMsgDOTAProTeamListResponse_TeamEntry {
    fn new() -> CMsgDOTAProTeamListResponse_TeamEntry {
        CMsgDOTAProTeamListResponse_TeamEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAProTeamListResponse_TeamEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTAProTeamListResponse_TeamEntry::get_team_id_for_reflect,
                    CMsgDOTAProTeamListResponse_TeamEntry::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tag",
                    CMsgDOTAProTeamListResponse_TeamEntry::get_tag_for_reflect,
                    CMsgDOTAProTeamListResponse_TeamEntry::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_created",
                    CMsgDOTAProTeamListResponse_TeamEntry::get_time_created_for_reflect,
                    CMsgDOTAProTeamListResponse_TeamEntry::mut_time_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTAProTeamListResponse_TeamEntry::get_logo_for_reflect,
                    CMsgDOTAProTeamListResponse_TeamEntry::mut_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country_code",
                    CMsgDOTAProTeamListResponse_TeamEntry::get_country_code_for_reflect,
                    CMsgDOTAProTeamListResponse_TeamEntry::mut_country_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "member_count",
                    CMsgDOTAProTeamListResponse_TeamEntry::get_member_count_for_reflect,
                    CMsgDOTAProTeamListResponse_TeamEntry::mut_member_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAProTeamListResponse_TeamEntry>(
                    "CMsgDOTAProTeamListResponse_TeamEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAProTeamListResponse_TeamEntry {
    fn clear(&mut self) {
        self.clear_team_id();
        self.clear_tag();
        self.clear_time_created();
        self.clear_logo();
        self.clear_country_code();
        self.clear_member_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAProTeamListResponse_TeamEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAProTeamListResponse_TeamEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamInvite_InviterToGC {
    // message fields
    account_id: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamInvite_InviterToGC {}

impl CMsgDOTATeamInvite_InviterToGC {
    pub fn new() -> CMsgDOTATeamInvite_InviterToGC {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamInvite_InviterToGC {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamInvite_InviterToGC> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamInvite_InviterToGC,
        };
        unsafe {
            instance.get(CMsgDOTATeamInvite_InviterToGC::new)
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

    // optional uint32 team_id = 2;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }
}

impl ::protobuf::Message for CMsgDOTATeamInvite_InviterToGC {
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
                    self.team_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.team_id {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamInvite_InviterToGC {
    fn new() -> CMsgDOTATeamInvite_InviterToGC {
        CMsgDOTATeamInvite_InviterToGC::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamInvite_InviterToGC>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTATeamInvite_InviterToGC::get_account_id_for_reflect,
                    CMsgDOTATeamInvite_InviterToGC::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTATeamInvite_InviterToGC::get_team_id_for_reflect,
                    CMsgDOTATeamInvite_InviterToGC::mut_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamInvite_InviterToGC>(
                    "CMsgDOTATeamInvite_InviterToGC",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamInvite_InviterToGC {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamInvite_InviterToGC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamInvite_InviterToGC {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamInvite_GCImmediateResponseToInviter {
    // message fields
    result: ::std::option::Option<ETeamInviteResult>,
    invitee_name: ::protobuf::SingularField<::std::string::String>,
    required_badge_level: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamInvite_GCImmediateResponseToInviter {}

impl CMsgDOTATeamInvite_GCImmediateResponseToInviter {
    pub fn new() -> CMsgDOTATeamInvite_GCImmediateResponseToInviter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamInvite_GCImmediateResponseToInviter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamInvite_GCImmediateResponseToInviter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamInvite_GCImmediateResponseToInviter,
        };
        unsafe {
            instance.get(CMsgDOTATeamInvite_GCImmediateResponseToInviter::new)
        }
    }

    // optional .dota.ETeamInviteResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ETeamInviteResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ETeamInviteResult {
        self.result.unwrap_or(ETeamInviteResult::TEAM_INVITE_SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ETeamInviteResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ETeamInviteResult> {
        &mut self.result
    }

    // optional string invitee_name = 2;

    pub fn clear_invitee_name(&mut self) {
        self.invitee_name.clear();
    }

    pub fn has_invitee_name(&self) -> bool {
        self.invitee_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invitee_name(&mut self, v: ::std::string::String) {
        self.invitee_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_invitee_name(&mut self) -> &mut ::std::string::String {
        if self.invitee_name.is_none() {
            self.invitee_name.set_default();
        };
        self.invitee_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_invitee_name(&mut self) -> ::std::string::String {
        self.invitee_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_invitee_name(&self) -> &str {
        match self.invitee_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_invitee_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.invitee_name
    }

    fn mut_invitee_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.invitee_name
    }

    // optional uint32 required_badge_level = 3;

    pub fn clear_required_badge_level(&mut self) {
        self.required_badge_level = ::std::option::Option::None;
    }

    pub fn has_required_badge_level(&self) -> bool {
        self.required_badge_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_required_badge_level(&mut self, v: u32) {
        self.required_badge_level = ::std::option::Option::Some(v);
    }

    pub fn get_required_badge_level(&self) -> u32 {
        self.required_badge_level.unwrap_or(0)
    }

    fn get_required_badge_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.required_badge_level
    }

    fn mut_required_badge_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.required_badge_level
    }
}

impl ::protobuf::Message for CMsgDOTATeamInvite_GCImmediateResponseToInviter {
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
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.invitee_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.required_badge_level = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.invitee_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.required_badge_level {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.invitee_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.required_badge_level {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamInvite_GCImmediateResponseToInviter {
    fn new() -> CMsgDOTATeamInvite_GCImmediateResponseToInviter {
        CMsgDOTATeamInvite_GCImmediateResponseToInviter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamInvite_GCImmediateResponseToInviter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ETeamInviteResult>>(
                    "result",
                    CMsgDOTATeamInvite_GCImmediateResponseToInviter::get_result_for_reflect,
                    CMsgDOTATeamInvite_GCImmediateResponseToInviter::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "invitee_name",
                    CMsgDOTATeamInvite_GCImmediateResponseToInviter::get_invitee_name_for_reflect,
                    CMsgDOTATeamInvite_GCImmediateResponseToInviter::mut_invitee_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "required_badge_level",
                    CMsgDOTATeamInvite_GCImmediateResponseToInviter::get_required_badge_level_for_reflect,
                    CMsgDOTATeamInvite_GCImmediateResponseToInviter::mut_required_badge_level_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamInvite_GCImmediateResponseToInviter>(
                    "CMsgDOTATeamInvite_GCImmediateResponseToInviter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamInvite_GCImmediateResponseToInviter {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_invitee_name();
        self.clear_required_badge_level();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamInvite_GCImmediateResponseToInviter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamInvite_GCImmediateResponseToInviter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamInvite_GCRequestToInvitee {
    // message fields
    inviter_account_id: ::std::option::Option<u32>,
    team_name: ::protobuf::SingularField<::std::string::String>,
    team_tag: ::protobuf::SingularField<::std::string::String>,
    logo: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamInvite_GCRequestToInvitee {}

impl CMsgDOTATeamInvite_GCRequestToInvitee {
    pub fn new() -> CMsgDOTATeamInvite_GCRequestToInvitee {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamInvite_GCRequestToInvitee {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamInvite_GCRequestToInvitee> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamInvite_GCRequestToInvitee,
        };
        unsafe {
            instance.get(CMsgDOTATeamInvite_GCRequestToInvitee::new)
        }
    }

    // optional uint32 inviter_account_id = 1;

    pub fn clear_inviter_account_id(&mut self) {
        self.inviter_account_id = ::std::option::Option::None;
    }

    pub fn has_inviter_account_id(&self) -> bool {
        self.inviter_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inviter_account_id(&mut self, v: u32) {
        self.inviter_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_inviter_account_id(&self) -> u32 {
        self.inviter_account_id.unwrap_or(0)
    }

    fn get_inviter_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.inviter_account_id
    }

    fn mut_inviter_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.inviter_account_id
    }

    // optional string team_name = 2;

    pub fn clear_team_name(&mut self) {
        self.team_name.clear();
    }

    pub fn has_team_name(&self) -> bool {
        self.team_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name(&mut self, v: ::std::string::String) {
        self.team_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name(&mut self) -> &mut ::std::string::String {
        if self.team_name.is_none() {
            self.team_name.set_default();
        };
        self.team_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name(&mut self) -> ::std::string::String {
        self.team_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name(&self) -> &str {
        match self.team_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name
    }

    fn mut_team_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name
    }

    // optional string team_tag = 3;

    pub fn clear_team_tag(&mut self) {
        self.team_tag.clear();
    }

    pub fn has_team_tag(&self) -> bool {
        self.team_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_tag(&mut self, v: ::std::string::String) {
        self.team_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_tag(&mut self) -> &mut ::std::string::String {
        if self.team_tag.is_none() {
            self.team_tag.set_default();
        };
        self.team_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_tag(&mut self) -> ::std::string::String {
        self.team_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_tag(&self) -> &str {
        match self.team_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_tag
    }

    fn mut_team_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_tag
    }

    // optional uint64 logo = 4;

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

impl ::protobuf::Message for CMsgDOTATeamInvite_GCRequestToInvitee {
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
                    self.inviter_account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_tag)?;
                },
                4 => {
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
        if let Some(v) = self.inviter_account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.team_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.logo {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.inviter_account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.team_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.team_tag.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.logo {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamInvite_GCRequestToInvitee {
    fn new() -> CMsgDOTATeamInvite_GCRequestToInvitee {
        CMsgDOTATeamInvite_GCRequestToInvitee::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamInvite_GCRequestToInvitee>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "inviter_account_id",
                    CMsgDOTATeamInvite_GCRequestToInvitee::get_inviter_account_id_for_reflect,
                    CMsgDOTATeamInvite_GCRequestToInvitee::mut_inviter_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name",
                    CMsgDOTATeamInvite_GCRequestToInvitee::get_team_name_for_reflect,
                    CMsgDOTATeamInvite_GCRequestToInvitee::mut_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_tag",
                    CMsgDOTATeamInvite_GCRequestToInvitee::get_team_tag_for_reflect,
                    CMsgDOTATeamInvite_GCRequestToInvitee::mut_team_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "logo",
                    CMsgDOTATeamInvite_GCRequestToInvitee::get_logo_for_reflect,
                    CMsgDOTATeamInvite_GCRequestToInvitee::mut_logo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamInvite_GCRequestToInvitee>(
                    "CMsgDOTATeamInvite_GCRequestToInvitee",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamInvite_GCRequestToInvitee {
    fn clear(&mut self) {
        self.clear_inviter_account_id();
        self.clear_team_name();
        self.clear_team_tag();
        self.clear_logo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamInvite_GCRequestToInvitee {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamInvite_GCRequestToInvitee {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamInvite_InviteeResponseToGC {
    // message fields
    result: ::std::option::Option<ETeamInviteResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamInvite_InviteeResponseToGC {}

impl CMsgDOTATeamInvite_InviteeResponseToGC {
    pub fn new() -> CMsgDOTATeamInvite_InviteeResponseToGC {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamInvite_InviteeResponseToGC {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamInvite_InviteeResponseToGC> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamInvite_InviteeResponseToGC,
        };
        unsafe {
            instance.get(CMsgDOTATeamInvite_InviteeResponseToGC::new)
        }
    }

    // optional .dota.ETeamInviteResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ETeamInviteResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ETeamInviteResult {
        self.result.unwrap_or(ETeamInviteResult::TEAM_INVITE_SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ETeamInviteResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ETeamInviteResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTATeamInvite_InviteeResponseToGC {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamInvite_InviteeResponseToGC {
    fn new() -> CMsgDOTATeamInvite_InviteeResponseToGC {
        CMsgDOTATeamInvite_InviteeResponseToGC::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamInvite_InviteeResponseToGC>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ETeamInviteResult>>(
                    "result",
                    CMsgDOTATeamInvite_InviteeResponseToGC::get_result_for_reflect,
                    CMsgDOTATeamInvite_InviteeResponseToGC::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamInvite_InviteeResponseToGC>(
                    "CMsgDOTATeamInvite_InviteeResponseToGC",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamInvite_InviteeResponseToGC {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamInvite_InviteeResponseToGC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamInvite_InviteeResponseToGC {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamInvite_GCResponseToInviter {
    // message fields
    result: ::std::option::Option<ETeamInviteResult>,
    invitee_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamInvite_GCResponseToInviter {}

impl CMsgDOTATeamInvite_GCResponseToInviter {
    pub fn new() -> CMsgDOTATeamInvite_GCResponseToInviter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamInvite_GCResponseToInviter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamInvite_GCResponseToInviter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamInvite_GCResponseToInviter,
        };
        unsafe {
            instance.get(CMsgDOTATeamInvite_GCResponseToInviter::new)
        }
    }

    // optional .dota.ETeamInviteResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ETeamInviteResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ETeamInviteResult {
        self.result.unwrap_or(ETeamInviteResult::TEAM_INVITE_SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ETeamInviteResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ETeamInviteResult> {
        &mut self.result
    }

    // optional string invitee_name = 2;

    pub fn clear_invitee_name(&mut self) {
        self.invitee_name.clear();
    }

    pub fn has_invitee_name(&self) -> bool {
        self.invitee_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invitee_name(&mut self, v: ::std::string::String) {
        self.invitee_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_invitee_name(&mut self) -> &mut ::std::string::String {
        if self.invitee_name.is_none() {
            self.invitee_name.set_default();
        };
        self.invitee_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_invitee_name(&mut self) -> ::std::string::String {
        self.invitee_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_invitee_name(&self) -> &str {
        match self.invitee_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_invitee_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.invitee_name
    }

    fn mut_invitee_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.invitee_name
    }
}

impl ::protobuf::Message for CMsgDOTATeamInvite_GCResponseToInviter {
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
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.invitee_name)?;
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
        if let Some(v) = self.invitee_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.invitee_name.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamInvite_GCResponseToInviter {
    fn new() -> CMsgDOTATeamInvite_GCResponseToInviter {
        CMsgDOTATeamInvite_GCResponseToInviter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamInvite_GCResponseToInviter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ETeamInviteResult>>(
                    "result",
                    CMsgDOTATeamInvite_GCResponseToInviter::get_result_for_reflect,
                    CMsgDOTATeamInvite_GCResponseToInviter::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "invitee_name",
                    CMsgDOTATeamInvite_GCResponseToInviter::get_invitee_name_for_reflect,
                    CMsgDOTATeamInvite_GCResponseToInviter::mut_invitee_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamInvite_GCResponseToInviter>(
                    "CMsgDOTATeamInvite_GCResponseToInviter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamInvite_GCResponseToInviter {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_invitee_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamInvite_GCResponseToInviter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamInvite_GCResponseToInviter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATeamInvite_GCResponseToInvitee {
    // message fields
    result: ::std::option::Option<ETeamInviteResult>,
    team_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATeamInvite_GCResponseToInvitee {}

impl CMsgDOTATeamInvite_GCResponseToInvitee {
    pub fn new() -> CMsgDOTATeamInvite_GCResponseToInvitee {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATeamInvite_GCResponseToInvitee {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATeamInvite_GCResponseToInvitee> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATeamInvite_GCResponseToInvitee,
        };
        unsafe {
            instance.get(CMsgDOTATeamInvite_GCResponseToInvitee::new)
        }
    }

    // optional .dota.ETeamInviteResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ETeamInviteResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> ETeamInviteResult {
        self.result.unwrap_or(ETeamInviteResult::TEAM_INVITE_SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<ETeamInviteResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<ETeamInviteResult> {
        &mut self.result
    }

    // optional string team_name = 2;

    pub fn clear_team_name(&mut self) {
        self.team_name.clear();
    }

    pub fn has_team_name(&self) -> bool {
        self.team_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name(&mut self, v: ::std::string::String) {
        self.team_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name(&mut self) -> &mut ::std::string::String {
        if self.team_name.is_none() {
            self.team_name.set_default();
        };
        self.team_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name(&mut self) -> ::std::string::String {
        self.team_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name(&self) -> &str {
        match self.team_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name
    }

    fn mut_team_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name
    }
}

impl ::protobuf::Message for CMsgDOTATeamInvite_GCResponseToInvitee {
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
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name)?;
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
        if let Some(v) = self.team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.team_name.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgDOTATeamInvite_GCResponseToInvitee {
    fn new() -> CMsgDOTATeamInvite_GCResponseToInvitee {
        CMsgDOTATeamInvite_GCResponseToInvitee::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATeamInvite_GCResponseToInvitee>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ETeamInviteResult>>(
                    "result",
                    CMsgDOTATeamInvite_GCResponseToInvitee::get_result_for_reflect,
                    CMsgDOTATeamInvite_GCResponseToInvitee::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name",
                    CMsgDOTATeamInvite_GCResponseToInvitee::get_team_name_for_reflect,
                    CMsgDOTATeamInvite_GCResponseToInvitee::mut_team_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATeamInvite_GCResponseToInvitee>(
                    "CMsgDOTATeamInvite_GCResponseToInvitee",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATeamInvite_GCResponseToInvitee {
    fn clear(&mut self) {
        self.clear_result();
        self.clear_team_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATeamInvite_GCResponseToInvitee {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATeamInvite_GCResponseToInvitee {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAKickTeamMember {
    // message fields
    account_id: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAKickTeamMember {}

impl CMsgDOTAKickTeamMember {
    pub fn new() -> CMsgDOTAKickTeamMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAKickTeamMember {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAKickTeamMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAKickTeamMember,
        };
        unsafe {
            instance.get(CMsgDOTAKickTeamMember::new)
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

    // optional uint32 team_id = 2;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }
}

impl ::protobuf::Message for CMsgDOTAKickTeamMember {
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
                    self.team_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.team_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAKickTeamMember {
    fn new() -> CMsgDOTAKickTeamMember {
        CMsgDOTAKickTeamMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAKickTeamMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAKickTeamMember::get_account_id_for_reflect,
                    CMsgDOTAKickTeamMember::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTAKickTeamMember::get_team_id_for_reflect,
                    CMsgDOTAKickTeamMember::mut_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAKickTeamMember>(
                    "CMsgDOTAKickTeamMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAKickTeamMember {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAKickTeamMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAKickTeamMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAKickTeamMemberResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAKickTeamMemberResponse_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAKickTeamMemberResponse {}

impl CMsgDOTAKickTeamMemberResponse {
    pub fn new() -> CMsgDOTAKickTeamMemberResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAKickTeamMemberResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAKickTeamMemberResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAKickTeamMemberResponse,
        };
        unsafe {
            instance.get(CMsgDOTAKickTeamMemberResponse::new)
        }
    }

    // optional .dota.CMsgDOTAKickTeamMemberResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAKickTeamMemberResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAKickTeamMemberResponse_Result {
        self.result.unwrap_or(CMsgDOTAKickTeamMemberResponse_Result::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAKickTeamMemberResponse_Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAKickTeamMemberResponse_Result> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAKickTeamMemberResponse {
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

impl ::protobuf::MessageStatic for CMsgDOTAKickTeamMemberResponse {
    fn new() -> CMsgDOTAKickTeamMemberResponse {
        CMsgDOTAKickTeamMemberResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAKickTeamMemberResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAKickTeamMemberResponse_Result>>(
                    "result",
                    CMsgDOTAKickTeamMemberResponse::get_result_for_reflect,
                    CMsgDOTAKickTeamMemberResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAKickTeamMemberResponse>(
                    "CMsgDOTAKickTeamMemberResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAKickTeamMemberResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAKickTeamMemberResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAKickTeamMemberResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAKickTeamMemberResponse_Result {
    SUCCESS = 0,
    FAILURE_INVALID_ACCOUNT_TYPE = 1,
    FAILURE_KICKER_NOT_ADMIN = 2,
    FAILURE_KICKEE_NOT_MEMBER = 3,
    FAILURE_TEAM_LOCKED = 4,
    FAILURE_UNSPECIFIED_ERROR = 5,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAKickTeamMemberResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAKickTeamMemberResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAKickTeamMemberResponse_Result::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAKickTeamMemberResponse_Result::FAILURE_INVALID_ACCOUNT_TYPE),
            2 => ::std::option::Option::Some(CMsgDOTAKickTeamMemberResponse_Result::FAILURE_KICKER_NOT_ADMIN),
            3 => ::std::option::Option::Some(CMsgDOTAKickTeamMemberResponse_Result::FAILURE_KICKEE_NOT_MEMBER),
            4 => ::std::option::Option::Some(CMsgDOTAKickTeamMemberResponse_Result::FAILURE_TEAM_LOCKED),
            5 => ::std::option::Option::Some(CMsgDOTAKickTeamMemberResponse_Result::FAILURE_UNSPECIFIED_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAKickTeamMemberResponse_Result] = &[
            CMsgDOTAKickTeamMemberResponse_Result::SUCCESS,
            CMsgDOTAKickTeamMemberResponse_Result::FAILURE_INVALID_ACCOUNT_TYPE,
            CMsgDOTAKickTeamMemberResponse_Result::FAILURE_KICKER_NOT_ADMIN,
            CMsgDOTAKickTeamMemberResponse_Result::FAILURE_KICKEE_NOT_MEMBER,
            CMsgDOTAKickTeamMemberResponse_Result::FAILURE_TEAM_LOCKED,
            CMsgDOTAKickTeamMemberResponse_Result::FAILURE_UNSPECIFIED_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAKickTeamMemberResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAKickTeamMemberResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAKickTeamMemberResponse_Result {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAKickTeamMemberResponse_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATransferTeamAdmin {
    // message fields
    new_admin_account_id: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATransferTeamAdmin {}

impl CMsgDOTATransferTeamAdmin {
    pub fn new() -> CMsgDOTATransferTeamAdmin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATransferTeamAdmin {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATransferTeamAdmin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATransferTeamAdmin,
        };
        unsafe {
            instance.get(CMsgDOTATransferTeamAdmin::new)
        }
    }

    // optional uint32 new_admin_account_id = 1;

    pub fn clear_new_admin_account_id(&mut self) {
        self.new_admin_account_id = ::std::option::Option::None;
    }

    pub fn has_new_admin_account_id(&self) -> bool {
        self.new_admin_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_admin_account_id(&mut self, v: u32) {
        self.new_admin_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_new_admin_account_id(&self) -> u32 {
        self.new_admin_account_id.unwrap_or(0)
    }

    fn get_new_admin_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.new_admin_account_id
    }

    fn mut_new_admin_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.new_admin_account_id
    }

    // optional uint32 team_id = 2;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }
}

impl ::protobuf::Message for CMsgDOTATransferTeamAdmin {
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
                    self.new_admin_account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.new_admin_account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_admin_account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.team_id {
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

impl ::protobuf::MessageStatic for CMsgDOTATransferTeamAdmin {
    fn new() -> CMsgDOTATransferTeamAdmin {
        CMsgDOTATransferTeamAdmin::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATransferTeamAdmin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "new_admin_account_id",
                    CMsgDOTATransferTeamAdmin::get_new_admin_account_id_for_reflect,
                    CMsgDOTATransferTeamAdmin::mut_new_admin_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTATransferTeamAdmin::get_team_id_for_reflect,
                    CMsgDOTATransferTeamAdmin::mut_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATransferTeamAdmin>(
                    "CMsgDOTATransferTeamAdmin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATransferTeamAdmin {
    fn clear(&mut self) {
        self.clear_new_admin_account_id();
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATransferTeamAdmin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATransferTeamAdmin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTATransferTeamAdminResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTATransferTeamAdminResponse_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTATransferTeamAdminResponse {}

impl CMsgDOTATransferTeamAdminResponse {
    pub fn new() -> CMsgDOTATransferTeamAdminResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTATransferTeamAdminResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTATransferTeamAdminResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTATransferTeamAdminResponse,
        };
        unsafe {
            instance.get(CMsgDOTATransferTeamAdminResponse::new)
        }
    }

    // optional .dota.CMsgDOTATransferTeamAdminResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTATransferTeamAdminResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTATransferTeamAdminResponse_Result {
        self.result.unwrap_or(CMsgDOTATransferTeamAdminResponse_Result::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTATransferTeamAdminResponse_Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTATransferTeamAdminResponse_Result> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTATransferTeamAdminResponse {
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

impl ::protobuf::MessageStatic for CMsgDOTATransferTeamAdminResponse {
    fn new() -> CMsgDOTATransferTeamAdminResponse {
        CMsgDOTATransferTeamAdminResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTATransferTeamAdminResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTATransferTeamAdminResponse_Result>>(
                    "result",
                    CMsgDOTATransferTeamAdminResponse::get_result_for_reflect,
                    CMsgDOTATransferTeamAdminResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTATransferTeamAdminResponse>(
                    "CMsgDOTATransferTeamAdminResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTATransferTeamAdminResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTATransferTeamAdminResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATransferTeamAdminResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTATransferTeamAdminResponse_Result {
    SUCCESS = 0,
    FAILURE_INVALID_ACCOUNT_TYPE = 1,
    FAILURE_NOT_ADMIN = 2,
    FAILURE_SAME_ACCOUNT = 3,
    FAILURE_NOT_MEMBER = 4,
    FAILURE_UNSPECIFIED_ERROR = 5,
}

impl ::protobuf::ProtobufEnum for CMsgDOTATransferTeamAdminResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTATransferTeamAdminResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTATransferTeamAdminResponse_Result::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTATransferTeamAdminResponse_Result::FAILURE_INVALID_ACCOUNT_TYPE),
            2 => ::std::option::Option::Some(CMsgDOTATransferTeamAdminResponse_Result::FAILURE_NOT_ADMIN),
            3 => ::std::option::Option::Some(CMsgDOTATransferTeamAdminResponse_Result::FAILURE_SAME_ACCOUNT),
            4 => ::std::option::Option::Some(CMsgDOTATransferTeamAdminResponse_Result::FAILURE_NOT_MEMBER),
            5 => ::std::option::Option::Some(CMsgDOTATransferTeamAdminResponse_Result::FAILURE_UNSPECIFIED_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTATransferTeamAdminResponse_Result] = &[
            CMsgDOTATransferTeamAdminResponse_Result::SUCCESS,
            CMsgDOTATransferTeamAdminResponse_Result::FAILURE_INVALID_ACCOUNT_TYPE,
            CMsgDOTATransferTeamAdminResponse_Result::FAILURE_NOT_ADMIN,
            CMsgDOTATransferTeamAdminResponse_Result::FAILURE_SAME_ACCOUNT,
            CMsgDOTATransferTeamAdminResponse_Result::FAILURE_NOT_MEMBER,
            CMsgDOTATransferTeamAdminResponse_Result::FAILURE_UNSPECIFIED_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTATransferTeamAdminResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTATransferTeamAdminResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTATransferTeamAdminResponse_Result {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTATransferTeamAdminResponse_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChangeTeamSub {
    // message fields
    team_id: ::std::option::Option<u32>,
    member_account_id: ::std::option::Option<u32>,
    sub_account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChangeTeamSub {}

impl CMsgDOTAChangeTeamSub {
    pub fn new() -> CMsgDOTAChangeTeamSub {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChangeTeamSub {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChangeTeamSub> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChangeTeamSub,
        };
        unsafe {
            instance.get(CMsgDOTAChangeTeamSub::new)
        }
    }

    // optional uint32 team_id = 1;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }

    // optional uint32 member_account_id = 2;

    pub fn clear_member_account_id(&mut self) {
        self.member_account_id = ::std::option::Option::None;
    }

    pub fn has_member_account_id(&self) -> bool {
        self.member_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_member_account_id(&mut self, v: u32) {
        self.member_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_member_account_id(&self) -> u32 {
        self.member_account_id.unwrap_or(0)
    }

    fn get_member_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.member_account_id
    }

    fn mut_member_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.member_account_id
    }

    // optional uint32 sub_account_id = 3;

    pub fn clear_sub_account_id(&mut self) {
        self.sub_account_id = ::std::option::Option::None;
    }

    pub fn has_sub_account_id(&self) -> bool {
        self.sub_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sub_account_id(&mut self, v: u32) {
        self.sub_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_sub_account_id(&self) -> u32 {
        self.sub_account_id.unwrap_or(0)
    }

    fn get_sub_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sub_account_id
    }

    fn mut_sub_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sub_account_id
    }
}

impl ::protobuf::Message for CMsgDOTAChangeTeamSub {
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
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.member_account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.sub_account_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.member_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sub_account_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.member_account_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.sub_account_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAChangeTeamSub {
    fn new() -> CMsgDOTAChangeTeamSub {
        CMsgDOTAChangeTeamSub::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChangeTeamSub>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTAChangeTeamSub::get_team_id_for_reflect,
                    CMsgDOTAChangeTeamSub::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "member_account_id",
                    CMsgDOTAChangeTeamSub::get_member_account_id_for_reflect,
                    CMsgDOTAChangeTeamSub::mut_member_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sub_account_id",
                    CMsgDOTAChangeTeamSub::get_sub_account_id_for_reflect,
                    CMsgDOTAChangeTeamSub::mut_sub_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChangeTeamSub>(
                    "CMsgDOTAChangeTeamSub",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChangeTeamSub {
    fn clear(&mut self) {
        self.clear_team_id();
        self.clear_member_account_id();
        self.clear_sub_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChangeTeamSub {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChangeTeamSub {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChangeTeamSubResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTAChangeTeamSubResponse_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChangeTeamSubResponse {}

impl CMsgDOTAChangeTeamSubResponse {
    pub fn new() -> CMsgDOTAChangeTeamSubResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChangeTeamSubResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChangeTeamSubResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChangeTeamSubResponse,
        };
        unsafe {
            instance.get(CMsgDOTAChangeTeamSubResponse::new)
        }
    }

    // optional .dota.CMsgDOTAChangeTeamSubResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAChangeTeamSubResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAChangeTeamSubResponse_Result {
        self.result.unwrap_or(CMsgDOTAChangeTeamSubResponse_Result::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAChangeTeamSubResponse_Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAChangeTeamSubResponse_Result> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAChangeTeamSubResponse {
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

impl ::protobuf::MessageStatic for CMsgDOTAChangeTeamSubResponse {
    fn new() -> CMsgDOTAChangeTeamSubResponse {
        CMsgDOTAChangeTeamSubResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChangeTeamSubResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAChangeTeamSubResponse_Result>>(
                    "result",
                    CMsgDOTAChangeTeamSubResponse::get_result_for_reflect,
                    CMsgDOTAChangeTeamSubResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChangeTeamSubResponse>(
                    "CMsgDOTAChangeTeamSubResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChangeTeamSubResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChangeTeamSubResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChangeTeamSubResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAChangeTeamSubResponse_Result {
    SUCCESS = 0,
    FAILURE_INVALID_ACCOUNT_TYPE = 1,
    FAILURE_SAME_ACCOUNT = 2,
    FAILURE_NOT_ADMIN = 3,
    FAILURE_NOT_MEMBER = 4,
    FAILURE_NOT_SUB = 5,
    FAILURE_ALREADY_SUB = 6,
    FAILURE_UNSPECIFIED_ERROR = 7,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAChangeTeamSubResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAChangeTeamSubResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAChangeTeamSubResponse_Result::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAChangeTeamSubResponse_Result::FAILURE_INVALID_ACCOUNT_TYPE),
            2 => ::std::option::Option::Some(CMsgDOTAChangeTeamSubResponse_Result::FAILURE_SAME_ACCOUNT),
            3 => ::std::option::Option::Some(CMsgDOTAChangeTeamSubResponse_Result::FAILURE_NOT_ADMIN),
            4 => ::std::option::Option::Some(CMsgDOTAChangeTeamSubResponse_Result::FAILURE_NOT_MEMBER),
            5 => ::std::option::Option::Some(CMsgDOTAChangeTeamSubResponse_Result::FAILURE_NOT_SUB),
            6 => ::std::option::Option::Some(CMsgDOTAChangeTeamSubResponse_Result::FAILURE_ALREADY_SUB),
            7 => ::std::option::Option::Some(CMsgDOTAChangeTeamSubResponse_Result::FAILURE_UNSPECIFIED_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAChangeTeamSubResponse_Result] = &[
            CMsgDOTAChangeTeamSubResponse_Result::SUCCESS,
            CMsgDOTAChangeTeamSubResponse_Result::FAILURE_INVALID_ACCOUNT_TYPE,
            CMsgDOTAChangeTeamSubResponse_Result::FAILURE_SAME_ACCOUNT,
            CMsgDOTAChangeTeamSubResponse_Result::FAILURE_NOT_ADMIN,
            CMsgDOTAChangeTeamSubResponse_Result::FAILURE_NOT_MEMBER,
            CMsgDOTAChangeTeamSubResponse_Result::FAILURE_NOT_SUB,
            CMsgDOTAChangeTeamSubResponse_Result::FAILURE_ALREADY_SUB,
            CMsgDOTAChangeTeamSubResponse_Result::FAILURE_UNSPECIFIED_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAChangeTeamSubResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAChangeTeamSubResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAChangeTeamSubResponse_Result {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChangeTeamSubResponse_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTALeaveTeam {
    // message fields
    team_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTALeaveTeam {}

impl CMsgDOTALeaveTeam {
    pub fn new() -> CMsgDOTALeaveTeam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTALeaveTeam {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTALeaveTeam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTALeaveTeam,
        };
        unsafe {
            instance.get(CMsgDOTALeaveTeam::new)
        }
    }

    // optional uint32 team_id = 1;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }
}

impl ::protobuf::Message for CMsgDOTALeaveTeam {
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
                    self.team_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTALeaveTeam {
    fn new() -> CMsgDOTALeaveTeam {
        CMsgDOTALeaveTeam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTALeaveTeam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgDOTALeaveTeam::get_team_id_for_reflect,
                    CMsgDOTALeaveTeam::mut_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTALeaveTeam>(
                    "CMsgDOTALeaveTeam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTALeaveTeam {
    fn clear(&mut self) {
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTALeaveTeam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTALeaveTeam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTALeaveTeamResponse {
    // message fields
    result: ::std::option::Option<CMsgDOTALeaveTeamResponse_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTALeaveTeamResponse {}

impl CMsgDOTALeaveTeamResponse {
    pub fn new() -> CMsgDOTALeaveTeamResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTALeaveTeamResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTALeaveTeamResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTALeaveTeamResponse,
        };
        unsafe {
            instance.get(CMsgDOTALeaveTeamResponse::new)
        }
    }

    // optional .dota.CMsgDOTALeaveTeamResponse.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTALeaveTeamResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTALeaveTeamResponse_Result {
        self.result.unwrap_or(CMsgDOTALeaveTeamResponse_Result::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTALeaveTeamResponse_Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTALeaveTeamResponse_Result> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTALeaveTeamResponse {
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

impl ::protobuf::MessageStatic for CMsgDOTALeaveTeamResponse {
    fn new() -> CMsgDOTALeaveTeamResponse {
        CMsgDOTALeaveTeamResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTALeaveTeamResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTALeaveTeamResponse_Result>>(
                    "result",
                    CMsgDOTALeaveTeamResponse::get_result_for_reflect,
                    CMsgDOTALeaveTeamResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTALeaveTeamResponse>(
                    "CMsgDOTALeaveTeamResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTALeaveTeamResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTALeaveTeamResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTALeaveTeamResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTALeaveTeamResponse_Result {
    SUCCESS = 0,
    FAILURE_NOT_MEMBER = 1,
    FAILURE_TEAM_LOCKED = 2,
    FAILURE_UNSPECIFIED_ERROR = 3,
}

impl ::protobuf::ProtobufEnum for CMsgDOTALeaveTeamResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTALeaveTeamResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTALeaveTeamResponse_Result::SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTALeaveTeamResponse_Result::FAILURE_NOT_MEMBER),
            2 => ::std::option::Option::Some(CMsgDOTALeaveTeamResponse_Result::FAILURE_TEAM_LOCKED),
            3 => ::std::option::Option::Some(CMsgDOTALeaveTeamResponse_Result::FAILURE_UNSPECIFIED_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTALeaveTeamResponse_Result] = &[
            CMsgDOTALeaveTeamResponse_Result::SUCCESS,
            CMsgDOTALeaveTeamResponse_Result::FAILURE_NOT_MEMBER,
            CMsgDOTALeaveTeamResponse_Result::FAILURE_TEAM_LOCKED,
            CMsgDOTALeaveTeamResponse_Result::FAILURE_UNSPECIFIED_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTALeaveTeamResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTALeaveTeamResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTALeaveTeamResponse_Result {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTALeaveTeamResponse_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTABetaParticipation {
    // message fields
    access_rights: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTABetaParticipation {}

impl CMsgDOTABetaParticipation {
    pub fn new() -> CMsgDOTABetaParticipation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTABetaParticipation {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTABetaParticipation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTABetaParticipation,
        };
        unsafe {
            instance.get(CMsgDOTABetaParticipation::new)
        }
    }

    // optional uint32 access_rights = 1;

    pub fn clear_access_rights(&mut self) {
        self.access_rights = ::std::option::Option::None;
    }

    pub fn has_access_rights(&self) -> bool {
        self.access_rights.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_rights(&mut self, v: u32) {
        self.access_rights = ::std::option::Option::Some(v);
    }

    pub fn get_access_rights(&self) -> u32 {
        self.access_rights.unwrap_or(0)
    }

    fn get_access_rights_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.access_rights
    }

    fn mut_access_rights_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.access_rights
    }
}

impl ::protobuf::Message for CMsgDOTABetaParticipation {
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
                    self.access_rights = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.access_rights {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.access_rights {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTABetaParticipation {
    fn new() -> CMsgDOTABetaParticipation {
        CMsgDOTABetaParticipation::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTABetaParticipation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "access_rights",
                    CMsgDOTABetaParticipation::get_access_rights_for_reflect,
                    CMsgDOTABetaParticipation::mut_access_rights_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTABetaParticipation>(
                    "CMsgDOTABetaParticipation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTABetaParticipation {
    fn clear(&mut self) {
        self.clear_access_rights();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTABetaParticipation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTABetaParticipation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ETeamInviteResult {
    TEAM_INVITE_SUCCESS = 0,
    TEAM_INVITE_FAILURE_INVITE_REJECTED = 1,
    TEAM_INVITE_FAILURE_INVITE_TIMEOUT = 2,
    TEAM_INVITE_ERROR_TEAM_AT_MEMBER_LIMIT = 3,
    TEAM_INVITE_ERROR_TEAM_LOCKED = 4,
    TEAM_INVITE_ERROR_INVITEE_NOT_AVAILABLE = 5,
    TEAM_INVITE_ERROR_INVITEE_BUSY = 6,
    TEAM_INVITE_ERROR_INVITEE_ALREADY_MEMBER = 7,
    TEAM_INVITE_ERROR_INVITEE_AT_TEAM_LIMIT = 8,
    TEAM_INVITE_ERROR_INVITEE_INSUFFICIENT_LEVEL = 9,
    TEAM_INVITE_ERROR_INVITER_INVALID_ACCOUNT_TYPE = 10,
    TEAM_INVITE_ERROR_INVITER_NOT_ADMIN = 11,
    TEAM_INVITE_ERROR_INCORRECT_USER_RESPONDED = 12,
    TEAM_INVITE_ERROR_UNSPECIFIED = 13,
}

impl ::protobuf::ProtobufEnum for ETeamInviteResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETeamInviteResult> {
        match value {
            0 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_SUCCESS),
            1 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_FAILURE_INVITE_REJECTED),
            2 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_FAILURE_INVITE_TIMEOUT),
            3 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_TEAM_AT_MEMBER_LIMIT),
            4 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_TEAM_LOCKED),
            5 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_NOT_AVAILABLE),
            6 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_BUSY),
            7 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_ALREADY_MEMBER),
            8 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_AT_TEAM_LIMIT),
            9 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_INSUFFICIENT_LEVEL),
            10 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_INVITER_INVALID_ACCOUNT_TYPE),
            11 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_INVITER_NOT_ADMIN),
            12 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_INCORRECT_USER_RESPONDED),
            13 => ::std::option::Option::Some(ETeamInviteResult::TEAM_INVITE_ERROR_UNSPECIFIED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETeamInviteResult] = &[
            ETeamInviteResult::TEAM_INVITE_SUCCESS,
            ETeamInviteResult::TEAM_INVITE_FAILURE_INVITE_REJECTED,
            ETeamInviteResult::TEAM_INVITE_FAILURE_INVITE_TIMEOUT,
            ETeamInviteResult::TEAM_INVITE_ERROR_TEAM_AT_MEMBER_LIMIT,
            ETeamInviteResult::TEAM_INVITE_ERROR_TEAM_LOCKED,
            ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_NOT_AVAILABLE,
            ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_BUSY,
            ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_ALREADY_MEMBER,
            ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_AT_TEAM_LIMIT,
            ETeamInviteResult::TEAM_INVITE_ERROR_INVITEE_INSUFFICIENT_LEVEL,
            ETeamInviteResult::TEAM_INVITE_ERROR_INVITER_INVALID_ACCOUNT_TYPE,
            ETeamInviteResult::TEAM_INVITE_ERROR_INVITER_NOT_ADMIN,
            ETeamInviteResult::TEAM_INVITE_ERROR_INCORRECT_USER_RESPONDED,
            ETeamInviteResult::TEAM_INVITE_ERROR_UNSPECIFIED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ETeamInviteResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETeamInviteResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETeamInviteResult {
}

impl ::protobuf::reflect::ProtobufValue for ETeamInviteResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x21, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x22, 0x79, 0x0a, 0x15, 0x43, 0x4d, 0x73,
    0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x53,
    0x44, 0x4f, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49,
    0x64, 0x12, 0x19, 0x0a, 0x08, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0d, 0x52, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x73, 0x12, 0x26, 0x0a, 0x0f,
    0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x54, 0x65,
    0x61, 0x6d, 0x49, 0x64, 0x22, 0x50, 0x0a, 0x14, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x54, 0x65, 0x61, 0x6d, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x53, 0x44, 0x4f, 0x12, 0x1d, 0x0a, 0x0a,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x74,
    0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x07, 0x74,
    0x65, 0x61, 0x6d, 0x49, 0x64, 0x73, 0x22, 0x54, 0x0a, 0x12, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f,
    0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x74,
    0x69, 0x6d, 0x65, 0x5f, 0x6a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x4a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x22, 0xb8, 0x07, 0x0a,
    0x0c, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x32, 0x0a,
    0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65,
    0x61, 0x6d, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x10,
    0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x74, 0x61, 0x67,
    0x12, 0x19, 0x0a, 0x08, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x07, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x74,
    0x69, 0x6d, 0x65, 0x5f, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0b, 0x74, 0x69, 0x6d, 0x65, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x12, 0x1c,
    0x0a, 0x09, 0x64, 0x69, 0x73, 0x62, 0x61, 0x6e, 0x64, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x09, 0x64, 0x69, 0x73, 0x62, 0x61, 0x6e, 0x64, 0x65, 0x64, 0x12, 0x12, 0x0a, 0x04,
    0x77, 0x69, 0x6e, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x77, 0x69, 0x6e, 0x73,
    0x12, 0x16, 0x0a, 0x06, 0x6c, 0x6f, 0x73, 0x73, 0x65, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x06, 0x6c, 0x6f, 0x73, 0x73, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x72, 0x61, 0x6e, 0x6b,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x72, 0x61, 0x6e, 0x6b, 0x12, 0x3e, 0x0a, 0x1b,
    0x63, 0x61, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x67, 0x61, 0x6d, 0x65,
    0x73, 0x5f, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x18, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x19, 0x63, 0x61, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x47, 0x61,
    0x6d, 0x65, 0x73, 0x52, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x12, 0x12, 0x0a, 0x04,
    0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x6f,
    0x12, 0x1b, 0x0a, 0x09, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x08, 0x62, 0x61, 0x73, 0x65, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x1f, 0x0a,
    0x0b, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0d, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0a, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x21,
    0x0a, 0x0c, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0e,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x4c, 0x6f, 0x67,
    0x6f, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x63, 0x6f, 0x64,
    0x65, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79,
    0x43, 0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x10, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x03, 0x75, 0x72, 0x6c, 0x12, 0x28, 0x0a, 0x0f, 0x66, 0x75, 0x6c, 0x6c, 0x67, 0x61,
    0x6d, 0x65, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0f, 0x66, 0x75, 0x6c, 0x6c, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64,
    0x12, 0x18, 0x0a, 0x07, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x73, 0x18, 0x12, 0x20, 0x03, 0x28,
    0x0d, 0x52, 0x07, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x73, 0x12, 0x20, 0x0a, 0x0b, 0x67, 0x61,
    0x6d, 0x65, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0b, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x12, 0x42, 0x0a, 0x1c,
    0x67, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x77, 0x69, 0x74, 0x68, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x72, 0x6f, 0x73, 0x74, 0x65, 0x72, 0x18, 0x14, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x1c, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x77,
    0x69, 0x74, 0x68, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x72, 0x6f, 0x73, 0x74, 0x65, 0x72,
    0x12, 0x3e, 0x0a, 0x1a, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x6d, 0x61, 0x6b,
    0x69, 0x6e, 0x67, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x18, 0x15,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x1a, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x6d,
    0x61, 0x6b, 0x69, 0x6e, 0x67, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64,
    0x12, 0x2e, 0x0a, 0x12, 0x6c, 0x61, 0x73, 0x74, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x67, 0x61,
    0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x6c, 0x61,
    0x73, 0x74, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x67, 0x61, 0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65,
    0x12, 0x26, 0x0a, 0x0e, 0x6c, 0x61, 0x73, 0x74, 0x72, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x17, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x6c, 0x61, 0x73, 0x74, 0x72, 0x65,
    0x6e, 0x61, 0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x72, 0x65, 0x63, 0x65,
    0x6e, 0x74, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x19, 0x20, 0x03,
    0x28, 0x04, 0x52, 0x0e, 0x72, 0x65, 0x63, 0x65, 0x6e, 0x74, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x49,
    0x64, 0x73, 0x12, 0x22, 0x0a, 0x0d, 0x74, 0x6f, 0x70, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f,
    0x69, 0x64, 0x73, 0x18, 0x1a, 0x20, 0x03, 0x28, 0x04, 0x52, 0x0b, 0x74, 0x6f, 0x70, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x49, 0x64, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x69, 0x63, 0x6b, 0x75, 0x70,
    0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x1b, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x70, 0x69, 0x63,
    0x6b, 0x75, 0x70, 0x54, 0x65, 0x61, 0x6d, 0x22, 0x99, 0x07, 0x0a, 0x10, 0x43, 0x4d, 0x73, 0x67,
    0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x37, 0x0a, 0x07,
    0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61,
    0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x73, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x12,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x03, 0x74, 0x61, 0x67, 0x12, 0x21, 0x0a, 0x0c, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x63, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x74, 0x69, 0x6d, 0x65,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x12, 0x10, 0x0a, 0x03, 0x70, 0x72, 0x6f, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x03, 0x70, 0x72, 0x6f, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x6f, 0x63,
    0x6b, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x6c, 0x6f, 0x63, 0x6b, 0x65,
    0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x69, 0x63, 0x6b, 0x75, 0x70, 0x5f, 0x74, 0x65, 0x61, 0x6d,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x70, 0x69, 0x63, 0x6b, 0x75, 0x70, 0x54, 0x65,
    0x61, 0x6d, 0x12, 0x19, 0x0a, 0x08, 0x75, 0x67, 0x63, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x75, 0x67, 0x63, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x22, 0x0a,
    0x0d, 0x75, 0x67, 0x63, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x75, 0x67, 0x63, 0x42, 0x61, 0x73, 0x65, 0x4c, 0x6f, 0x67,
    0x6f, 0x12, 0x26, 0x0a, 0x0f, 0x75, 0x67, 0x63, 0x5f, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5f,
    0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x75, 0x67, 0x63, 0x42,
    0x61, 0x6e, 0x6e, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x28, 0x0a, 0x10, 0x75, 0x67, 0x63,
    0x5f, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x0e, 0x75, 0x67, 0x63, 0x53, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x4c,
    0x6f, 0x67, 0x6f, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x63,
    0x6f, 0x64, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x72, 0x79, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x77, 0x69, 0x6e, 0x73,
    0x18, 0x0f, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x77, 0x69, 0x6e, 0x73, 0x12, 0x16, 0x0a, 0x06,
    0x6c, 0x6f, 0x73, 0x73, 0x65, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x6c, 0x6f,
    0x73, 0x73, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x72, 0x61, 0x6e, 0x6b, 0x18, 0x11, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x04, 0x72, 0x61, 0x6e, 0x6b, 0x12, 0x3e, 0x0a, 0x1b, 0x63, 0x61, 0x6c, 0x69,
    0x62, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x5f, 0x72, 0x65,
    0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x19, 0x63,
    0x61, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x52,
    0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x12, 0x2c, 0x0a, 0x12, 0x67, 0x61, 0x6d, 0x65,
    0x73, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18, 0x13,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x50, 0x6c, 0x61, 0x79, 0x65,
    0x64, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x12, 0x38, 0x0a, 0x18, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x5f,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x6d, 0x61, 0x6b, 0x69,
    0x6e, 0x67, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x16, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x50,
    0x6c, 0x61, 0x79, 0x65, 0x64, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x6d, 0x61, 0x6b, 0x69, 0x6e, 0x67,
    0x12, 0x31, 0x0a, 0x14, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x63, 0x69, 0x70, 0x61, 0x74, 0x65, 0x64, 0x18, 0x15, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x13,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x73, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x69, 0x70, 0x61,
    0x74, 0x65, 0x64, 0x12, 0x22, 0x0a, 0x0d, 0x74, 0x6f, 0x70, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x5f, 0x69, 0x64, 0x73, 0x18, 0x16, 0x20, 0x03, 0x28, 0x04, 0x52, 0x0b, 0x74, 0x6f, 0x70, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x73, 0x12, 0x28, 0x0a, 0x10, 0x72, 0x65, 0x63, 0x65, 0x6e,
    0x74, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x17, 0x20, 0x03, 0x28,
    0x04, 0x52, 0x0e, 0x72, 0x65, 0x63, 0x65, 0x6e, 0x74, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64,
    0x73, 0x1a, 0x70, 0x0a, 0x06, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x74, 0x69,
    0x6d, 0x65, 0x5f, 0x6a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0a, 0x74, 0x69, 0x6d, 0x65, 0x4a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x61,
    0x64, 0x6d, 0x69, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x61, 0x64, 0x6d, 0x69,
    0x6e, 0x12, 0x10, 0x0a, 0x03, 0x73, 0x75, 0x62, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x03,
    0x73, 0x75, 0x62, 0x22, 0x5e, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54,
    0x65, 0x61, 0x6d, 0x73, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x65, 0x61, 0x67,
    0x75, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x6c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x05, 0x74, 0x65, 0x61, 0x6d, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x05, 0x74, 0x65,
    0x61, 0x6d, 0x73, 0x22, 0x1b, 0x0a, 0x19, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4d,
    0x79, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x22, 0x85, 0x02, 0x0a, 0x12, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x74,
    0x61, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x74, 0x61, 0x67, 0x12, 0x12, 0x0a,
    0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f, 0x67,
    0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x62, 0x61, 0x73, 0x65, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x1f,
    0x0a, 0x0b, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x0a, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x12,
    0x21, 0x0a, 0x0c, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x4c, 0x6f,
    0x67, 0x6f, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x63, 0x6f,
    0x64, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72,
    0x79, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x6c, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x69, 0x63, 0x6b, 0x75,
    0x70, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x70, 0x69,
    0x63, 0x6b, 0x75, 0x70, 0x54, 0x65, 0x61, 0x6d, 0x22, 0xac, 0x04, 0x0a, 0x1a, 0x43, 0x4d, 0x73,
    0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x48, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x27, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x65, 0x61,
    0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x3a, 0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x22, 0xaa, 0x03, 0x0a, 0x06, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x14, 0x0a, 0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44,
    0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x53,
    0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x4e, 0x41, 0x4d, 0x45,
    0x5f, 0x45, 0x4d, 0x50, 0x54, 0x59, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x4e, 0x41, 0x4d, 0x45,
    0x5f, 0x42, 0x41, 0x44, 0x5f, 0x43, 0x48, 0x41, 0x52, 0x41, 0x43, 0x54, 0x45, 0x52, 0x53, 0x10,
    0x02, 0x12, 0x0e, 0x0a, 0x0a, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x54, 0x41, 0x4b, 0x45, 0x4e, 0x10,
    0x03, 0x12, 0x11, 0x0a, 0x0d, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x4c, 0x4f,
    0x4e, 0x47, 0x10, 0x04, 0x12, 0x0d, 0x0a, 0x09, 0x54, 0x41, 0x47, 0x5f, 0x45, 0x4d, 0x50, 0x54,
    0x59, 0x10, 0x05, 0x12, 0x16, 0x0a, 0x12, 0x54, 0x41, 0x47, 0x5f, 0x42, 0x41, 0x44, 0x5f, 0x43,
    0x48, 0x41, 0x52, 0x41, 0x43, 0x54, 0x45, 0x52, 0x53, 0x10, 0x06, 0x12, 0x0d, 0x0a, 0x09, 0x54,
    0x41, 0x47, 0x5f, 0x54, 0x41, 0x4b, 0x45, 0x4e, 0x10, 0x07, 0x12, 0x10, 0x0a, 0x0c, 0x54, 0x41,
    0x47, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x4c, 0x4f, 0x4e, 0x47, 0x10, 0x08, 0x12, 0x10, 0x0a, 0x0c,
    0x43, 0x52, 0x45, 0x41, 0x54, 0x4f, 0x52, 0x5f, 0x42, 0x55, 0x53, 0x59, 0x10, 0x09, 0x12, 0x15,
    0x0a, 0x11, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x5f, 0x45, 0x52,
    0x52, 0x4f, 0x52, 0x10, 0x0a, 0x12, 0x1e, 0x0a, 0x1a, 0x43, 0x52, 0x45, 0x41, 0x54, 0x4f, 0x52,
    0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x4c, 0x49, 0x4d, 0x49, 0x54, 0x5f, 0x52, 0x45, 0x41, 0x43,
    0x48, 0x45, 0x44, 0x10, 0x0b, 0x12, 0x0b, 0x0a, 0x07, 0x4e, 0x4f, 0x5f, 0x4c, 0x4f, 0x47, 0x4f,
    0x10, 0x0c, 0x12, 0x22, 0x0a, 0x1e, 0x43, 0x52, 0x45, 0x41, 0x54, 0x4f, 0x52, 0x5f, 0x54, 0x45,
    0x41, 0x4d, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x4f, 0x4f, 0x4c,
    0x44, 0x4f, 0x57, 0x4e, 0x10, 0x0d, 0x12, 0x16, 0x0a, 0x12, 0x4c, 0x4f, 0x47, 0x4f, 0x5f, 0x55,
    0x50, 0x4c, 0x4f, 0x41, 0x44, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x0e, 0x12, 0x1d,
    0x0a, 0x19, 0x4e, 0x41, 0x4d, 0x45, 0x5f, 0x43, 0x48, 0x41, 0x4e, 0x47, 0x45, 0x44, 0x5f, 0x54,
    0x4f, 0x4f, 0x5f, 0x52, 0x45, 0x43, 0x45, 0x4e, 0x54, 0x4c, 0x59, 0x10, 0x0f, 0x12, 0x1e, 0x0a,
    0x1a, 0x43, 0x52, 0x45, 0x41, 0x54, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x53, 0x55, 0x46, 0x46, 0x49,
    0x43, 0x49, 0x45, 0x4e, 0x54, 0x5f, 0x4c, 0x45, 0x56, 0x45, 0x4c, 0x10, 0x10, 0x12, 0x18, 0x0a,
    0x14, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54,
    0x5f, 0x54, 0x59, 0x50, 0x45, 0x10, 0x11, 0x22, 0xa9, 0x02, 0x0a, 0x17, 0x43, 0x4d, 0x73, 0x67,
    0x44, 0x4f, 0x54, 0x41, 0x45, 0x64, 0x69, 0x74, 0x54, 0x65, 0x61, 0x6d, 0x44, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x12, 0x10, 0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x74,
    0x61, 0x67, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c,
    0x6f, 0x67, 0x6f, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x62, 0x61, 0x73, 0x65, 0x4c,
    0x6f, 0x67, 0x6f, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5f, 0x6c, 0x6f,
    0x67, 0x6f, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72,
    0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x6f, 0x72, 0x5f,
    0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x6f, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x72, 0x79, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72,
    0x6c, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x6c, 0x12, 0x25, 0x0a, 0x0f,
    0x69, 0x6e, 0x5f, 0x75, 0x73, 0x65, 0x5f, 0x62, 0x79, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x79, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x69, 0x6e, 0x55, 0x73, 0x65, 0x42, 0x79, 0x50, 0x61,
    0x72, 0x74, 0x79, 0x22, 0xfa, 0x01, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x45, 0x64, 0x69, 0x74, 0x54, 0x65, 0x61, 0x6d, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4d, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x45, 0x64, 0x69, 0x74, 0x54, 0x65, 0x61, 0x6d, 0x44,
    0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x87, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12, 0x20,
    0x0a, 0x1c, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49,
    0x44, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x10, 0x01,
    0x12, 0x16, 0x0a, 0x12, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f,
    0x4d, 0x45, 0x4d, 0x42, 0x45, 0x52, 0x10, 0x02, 0x12, 0x17, 0x0a, 0x13, 0x46, 0x41, 0x49, 0x4c,
    0x55, 0x52, 0x45, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x4c, 0x4f, 0x43, 0x4b, 0x45, 0x44, 0x10,
    0x03, 0x12, 0x1d, 0x0a, 0x19, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x55, 0x4e, 0x53,
    0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x04,
    0x22, 0x5f, 0x0a, 0x1b, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d,
    0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x18, 0x0a, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x26, 0x0a, 0x04, 0x74, 0x65, 0x61,
    0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x52, 0x04, 0x74, 0x65, 0x61,
    0x6d, 0x22, 0x1c, 0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x72, 0x6f,
    0x54, 0x65, 0x61, 0x6d, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22,
    0xb0, 0x02, 0x0a, 0x1b, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x72, 0x6f, 0x54,
    0x65, 0x61, 0x6d, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x41, 0x0a, 0x05, 0x74, 0x65, 0x61, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2b,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x72,
    0x6f, 0x54, 0x65, 0x61, 0x6d, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x05, 0x74, 0x65, 0x61,
    0x6d, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x1a, 0xb3, 0x01, 0x0a,
    0x09, 0x54, 0x65, 0x61, 0x6d, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65,
    0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61,
    0x6d, 0x49, 0x64, 0x12, 0x10, 0x0a, 0x03, 0x74, 0x61, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x74, 0x61, 0x67, 0x12, 0x21, 0x0a, 0x0c, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x63, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x74, 0x69, 0x6d,
    0x65, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x6f, 0x67, 0x6f,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x12, 0x21, 0x0a, 0x0c,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x43, 0x6f, 0x64, 0x65, 0x12,
    0x21, 0x0a, 0x0c, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x43, 0x6f, 0x75,
    0x6e, 0x74, 0x22, 0x58, 0x0a, 0x1e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65,
    0x61, 0x6d, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x72,
    0x54, 0x6f, 0x47, 0x43, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x22, 0xcc, 0x01, 0x0a,
    0x2f, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x76,
    0x69, 0x74, 0x65, 0x5f, 0x47, 0x43, 0x49, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x54, 0x6f, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x72,
    0x12, 0x44, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x76,
    0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x13, 0x54, 0x45, 0x41, 0x4d, 0x5f,
    0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65,
    0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x69, 0x6e,
    0x76, 0x69, 0x74, 0x65, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x30, 0x0a, 0x14, 0x72, 0x65, 0x71,
    0x75, 0x69, 0x72, 0x65, 0x64, 0x5f, 0x62, 0x61, 0x64, 0x67, 0x65, 0x5f, 0x6c, 0x65, 0x76, 0x65,
    0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65,
    0x64, 0x42, 0x61, 0x64, 0x67, 0x65, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x22, 0xa1, 0x01, 0x0a, 0x25,
    0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x76, 0x69,
    0x74, 0x65, 0x5f, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x54, 0x6f, 0x49, 0x6e,
    0x76, 0x69, 0x74, 0x65, 0x65, 0x12, 0x2c, 0x0a, 0x12, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x72,
    0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x10, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x74, 0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65,
    0x12, 0x19, 0x0a, 0x08, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x74, 0x61, 0x67, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x54, 0x61, 0x67, 0x12, 0x12, 0x0a, 0x04, 0x6c,
    0x6f, 0x67, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x6c, 0x6f, 0x67, 0x6f, 0x22,
    0x6e, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d, 0x49,
    0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x54, 0x6f, 0x47, 0x43, 0x12, 0x44, 0x0a, 0x06, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x45, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x3a, 0x13, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f,
    0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22,
    0x91, 0x01, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x65, 0x61, 0x6d,
    0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x47, 0x43, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x54, 0x6f, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x72, 0x12, 0x44, 0x0a, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x45, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x3a, 0x13, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45,
    0x5f, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x12, 0x21, 0x0a, 0x0c, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x65, 0x4e,
    0x61, 0x6d, 0x65, 0x22, 0x8b, 0x01, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x47, 0x43, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x54, 0x6f, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x65, 0x12, 0x44,
    0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x6e, 0x76, 0x69, 0x74,
    0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x13, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e,
    0x56, 0x49, 0x54, 0x45, 0x5f, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x74, 0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d,
    0x65, 0x22, 0x50, 0x0a, 0x16, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4b, 0x69, 0x63,
    0x6b, 0x54, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65,
    0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61,
    0x6d, 0x49, 0x64, 0x22, 0x9d, 0x02, 0x0a, 0x1e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x4b, 0x69, 0x63, 0x6b, 0x54, 0x65, 0x61, 0x6d, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4c, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4b, 0x69, 0x63, 0x6b, 0x54, 0x65, 0x61, 0x6d, 0x4d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x22, 0xac, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12, 0x20, 0x0a, 0x1c,
    0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f,
    0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x10, 0x01, 0x12, 0x1c,
    0x0a, 0x18, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4b, 0x49, 0x43, 0x4b, 0x45, 0x52,
    0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x44, 0x4d, 0x49, 0x4e, 0x10, 0x02, 0x12, 0x1d, 0x0a, 0x19,
    0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4b, 0x49, 0x43, 0x4b, 0x45, 0x45, 0x5f, 0x4e,
    0x4f, 0x54, 0x5f, 0x4d, 0x45, 0x4d, 0x42, 0x45, 0x52, 0x10, 0x03, 0x12, 0x17, 0x0a, 0x13, 0x46,
    0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x4c, 0x4f, 0x43, 0x4b,
    0x45, 0x44, 0x10, 0x04, 0x12, 0x1d, 0x0a, 0x19, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f,
    0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x5f, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x10, 0x05, 0x22, 0x65, 0x0a, 0x19, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x54, 0x65, 0x61, 0x6d, 0x41, 0x64, 0x6d, 0x69, 0x6e,
    0x12, 0x2f, 0x0a, 0x14, 0x6e, 0x65, 0x77, 0x5f, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x5f, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11,
    0x6e, 0x65, 0x77, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49,
    0x64, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x22, 0x96, 0x02, 0x0a, 0x21, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x54,
    0x65, 0x61, 0x6d, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x4f, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x2e, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x54, 0x65, 0x61, 0x6d, 0x41, 0x64, 0x6d, 0x69,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x22, 0x9f, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07,
    0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12, 0x20, 0x0a, 0x1c, 0x46, 0x41, 0x49,
    0x4c, 0x55, 0x52, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x41, 0x43, 0x43,
    0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x10, 0x01, 0x12, 0x15, 0x0a, 0x11, 0x46,
    0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x44, 0x4d, 0x49, 0x4e,
    0x10, 0x02, 0x12, 0x18, 0x0a, 0x14, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x53, 0x41,
    0x4d, 0x45, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x10, 0x03, 0x12, 0x16, 0x0a, 0x12,
    0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x4d, 0x45, 0x4d, 0x42,
    0x45, 0x52, 0x10, 0x04, 0x12, 0x1d, 0x0a, 0x19, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f,
    0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x5f, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x10, 0x05, 0x22, 0x82, 0x01, 0x0a, 0x15, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41,
    0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x53, 0x75, 0x62, 0x12, 0x17, 0x0a,
    0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06,
    0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x2a, 0x0a, 0x11, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0f, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x49, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x75, 0x62, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x73, 0x75, 0x62, 0x41,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x22, 0xbc, 0x02, 0x0a, 0x1d, 0x43, 0x4d, 0x73,
    0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x53,
    0x75, 0x62, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4b, 0x0a, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2a, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x54, 0x65, 0x61, 0x6d, 0x53, 0x75, 0x62, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52,
    0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0xcd, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12,
    0x20, 0x0a, 0x1c, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c,
    0x49, 0x44, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x10,
    0x01, 0x12, 0x18, 0x0a, 0x14, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x53, 0x41, 0x4d,
    0x45, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x10, 0x02, 0x12, 0x15, 0x0a, 0x11, 0x46,
    0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x44, 0x4d, 0x49, 0x4e,
    0x10, 0x03, 0x12, 0x16, 0x0a, 0x12, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f,
    0x54, 0x5f, 0x4d, 0x45, 0x4d, 0x42, 0x45, 0x52, 0x10, 0x04, 0x12, 0x13, 0x0a, 0x0f, 0x46, 0x41,
    0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x53, 0x55, 0x42, 0x10, 0x05, 0x12,
    0x17, 0x0a, 0x13, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41,
    0x44, 0x59, 0x5f, 0x53, 0x55, 0x42, 0x10, 0x06, 0x12, 0x1d, 0x0a, 0x19, 0x46, 0x41, 0x49, 0x4c,
    0x55, 0x52, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x5f,
    0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x07, 0x22, 0x2c, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x4c, 0x65, 0x61, 0x76, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x17, 0x0a, 0x07,
    0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74,
    0x65, 0x61, 0x6d, 0x49, 0x64, 0x22, 0xcb, 0x01, 0x0a, 0x19, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f,
    0x54, 0x41, 0x4c, 0x65, 0x61, 0x76, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x47, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x26, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x4c, 0x65, 0x61, 0x76, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07, 0x53, 0x55, 0x43,
    0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x65, 0x0a, 0x06,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53,
    0x53, 0x10, 0x00, 0x12, 0x16, 0x0a, 0x12, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e,
    0x4f, 0x54, 0x5f, 0x4d, 0x45, 0x4d, 0x42, 0x45, 0x52, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x46,
    0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x4c, 0x4f, 0x43, 0x4b,
    0x45, 0x44, 0x10, 0x02, 0x12, 0x1d, 0x0a, 0x19, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f,
    0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x5f, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x10, 0x03, 0x22, 0x40, 0x0a, 0x19, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x42,
    0x65, 0x74, 0x61, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x69, 0x70, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x12, 0x23, 0x0a, 0x0d, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x72, 0x69, 0x67, 0x68, 0x74,
    0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x52,
    0x69, 0x67, 0x68, 0x74, 0x73, 0x2a, 0xda, 0x04, 0x0a, 0x11, 0x45, 0x54, 0x65, 0x61, 0x6d, 0x49,
    0x6e, 0x76, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x17, 0x0a, 0x13, 0x54,
    0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f, 0x53, 0x55, 0x43, 0x43, 0x45,
    0x53, 0x53, 0x10, 0x00, 0x12, 0x27, 0x0a, 0x23, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56,
    0x49, 0x54, 0x45, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x49,
    0x54, 0x45, 0x5f, 0x52, 0x45, 0x4a, 0x45, 0x43, 0x54, 0x45, 0x44, 0x10, 0x01, 0x12, 0x26, 0x0a,
    0x22, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f, 0x46, 0x41, 0x49,
    0x4c, 0x55, 0x52, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f, 0x54, 0x49, 0x4d, 0x45,
    0x4f, 0x55, 0x54, 0x10, 0x02, 0x12, 0x2a, 0x0a, 0x26, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e,
    0x56, 0x49, 0x54, 0x45, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f,
    0x41, 0x54, 0x5f, 0x4d, 0x45, 0x4d, 0x42, 0x45, 0x52, 0x5f, 0x4c, 0x49, 0x4d, 0x49, 0x54, 0x10,
    0x03, 0x12, 0x21, 0x0a, 0x1d, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45,
    0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x4c, 0x4f, 0x43, 0x4b,
    0x45, 0x44, 0x10, 0x04, 0x12, 0x2b, 0x0a, 0x27, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56,
    0x49, 0x54, 0x45, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45,
    0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10,
    0x05, 0x12, 0x22, 0x0a, 0x1e, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45,
    0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x45, 0x5f, 0x42,
    0x55, 0x53, 0x59, 0x10, 0x06, 0x12, 0x2c, 0x0a, 0x28, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e,
    0x56, 0x49, 0x54, 0x45, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54,
    0x45, 0x45, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f, 0x4d, 0x45, 0x4d, 0x42, 0x45,
    0x52, 0x10, 0x07, 0x12, 0x2b, 0x0a, 0x27, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49,
    0x54, 0x45, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x45,
    0x5f, 0x41, 0x54, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x4c, 0x49, 0x4d, 0x49, 0x54, 0x10, 0x08,
    0x12, 0x30, 0x0a, 0x2c, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f,
    0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x45, 0x5f, 0x49, 0x4e,
    0x53, 0x55, 0x46, 0x46, 0x49, 0x43, 0x49, 0x45, 0x4e, 0x54, 0x5f, 0x4c, 0x45, 0x56, 0x45, 0x4c,
    0x10, 0x09, 0x12, 0x32, 0x0a, 0x2e, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54,
    0x45, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x52, 0x5f,
    0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x5f, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f,
    0x54, 0x59, 0x50, 0x45, 0x10, 0x0a, 0x12, 0x27, 0x0a, 0x23, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49,
    0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x49,
    0x54, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x44, 0x4d, 0x49, 0x4e, 0x10, 0x0b, 0x12,
    0x2e, 0x0a, 0x2a, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f, 0x45,
    0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x43, 0x4f, 0x52, 0x52, 0x45, 0x43, 0x54, 0x5f, 0x55,
    0x53, 0x45, 0x52, 0x5f, 0x52, 0x45, 0x53, 0x50, 0x4f, 0x4e, 0x44, 0x45, 0x44, 0x10, 0x0c, 0x12,
    0x21, 0x0a, 0x1d, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x49, 0x4e, 0x56, 0x49, 0x54, 0x45, 0x5f, 0x45,
    0x52, 0x52, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44,
    0x10, 0x0d, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xc2, 0x67, 0x0a, 0x07, 0x12, 0x05,
    0x00, 0x00, 0xaf, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x04, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x04, 0x00, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x04, 0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x05, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x05,
    0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x05, 0x07, 0x1a,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x05, 0x07, 0x1a, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x07, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x05, 0x1d, 0x22, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x00, 0x12, 0x04, 0x07, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x03, 0x07, 0x05, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x08, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x09, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0a, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x2d, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x08, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0b, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x0c, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x0c, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x0c,
    0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0d, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x06, 0x12, 0x03, 0x0e, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x0e, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12,
    0x03, 0x0e, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0f, 0x08,
    0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x0f, 0x33, 0x34, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x10, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x10, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08,
    0x02, 0x12, 0x03, 0x10, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03,
    0x11, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x11, 0x08,
    0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x11, 0x37, 0x38, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x12, 0x08, 0x3c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x12, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0a, 0x02, 0x12, 0x03, 0x12, 0x39, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b,
    0x12, 0x03, 0x13, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x13, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x13, 0x2e,
    0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x14, 0x08, 0x38, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x14, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x14, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x0d, 0x12, 0x03, 0x15, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x01,
    0x12, 0x03, 0x15, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03,
    0x15, 0x28, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x18, 0x00, 0x1c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x19, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x19, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19,
    0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1a, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1a, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1b,
    0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x1e, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1e,
    0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x08, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1f, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x20, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x20,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x18, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x23, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x23, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x08,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x25, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x25, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x25,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x18, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x26, 0x27, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x28, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x28, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x29, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x11, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x24, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a,
    0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x08, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2b, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x2b, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x2c,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2c, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x2d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2d, 0x18,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2d, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x2e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x2e, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x2e, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x2f, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2f, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x07, 0x12, 0x03, 0x30, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x30, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05, 0x12, 0x03, 0x30,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x30, 0x18, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x03, 0x12, 0x03, 0x30, 0x1f, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x08, 0x12, 0x03, 0x31, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x08, 0x04, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x08, 0x05, 0x12, 0x03, 0x31, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x31, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x31, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x09, 0x12, 0x03, 0x32, 0x08, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x09, 0x05, 0x12, 0x03, 0x32, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x09, 0x01, 0x12, 0x03, 0x32, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x32, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0a,
    0x12, 0x03, 0x33, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x04, 0x12, 0x03,
    0x33, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x33, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x33, 0x18, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x33, 0x36, 0x38, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x0b, 0x12, 0x03, 0x34, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x0b, 0x04, 0x12, 0x03, 0x34, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b,
    0x05, 0x12, 0x03, 0x34, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x34, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x34,
    0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0c, 0x12, 0x03, 0x35, 0x08, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x35, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x35, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x35, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x35, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0d, 0x12,
    0x03, 0x36, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x36,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x36, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x36, 0x18, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x36, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x0e, 0x12, 0x03, 0x37, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x0e, 0x04, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0e, 0x05,
    0x12, 0x03, 0x37, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0e, 0x01, 0x12, 0x03,
    0x37, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x37, 0x27,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x0f, 0x12, 0x03, 0x38, 0x08, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x38, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x38, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x0f, 0x01, 0x12, 0x03, 0x38, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x0f,
    0x03, 0x12, 0x03, 0x38, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x10, 0x12, 0x03,
    0x39, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x10, 0x04, 0x12, 0x03, 0x39, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x10, 0x05, 0x12, 0x03, 0x39, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x10, 0x01, 0x12, 0x03, 0x39, 0x18, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x10, 0x03, 0x12, 0x03, 0x39, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x11, 0x12, 0x03, 0x3a, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x11,
    0x04, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x11, 0x05, 0x12,
    0x03, 0x3a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x11, 0x01, 0x12, 0x03, 0x3a,
    0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x11, 0x03, 0x12, 0x03, 0x3a, 0x2a, 0x2c,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x12, 0x12, 0x03, 0x3b, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x12, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x12, 0x05, 0x12, 0x03, 0x3b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x12, 0x01, 0x12, 0x03, 0x3b, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x12, 0x03,
    0x12, 0x03, 0x3b, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x13, 0x12, 0x03, 0x3c,
    0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x13, 0x04, 0x12, 0x03, 0x3c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x13, 0x05, 0x12, 0x03, 0x3c, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x13, 0x01, 0x12, 0x03, 0x3c, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x13, 0x03, 0x12, 0x03, 0x3c, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x14, 0x12, 0x03, 0x3d, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x14, 0x04,
    0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x14, 0x05, 0x12, 0x03,
    0x3d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x14, 0x01, 0x12, 0x03, 0x3d, 0x18,
    0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x14, 0x03, 0x12, 0x03, 0x3d, 0x37, 0x39, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x15, 0x12, 0x03, 0x3e, 0x08, 0x38, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x15, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x15, 0x05, 0x12, 0x03, 0x3e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x15,
    0x01, 0x12, 0x03, 0x3e, 0x18, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x15, 0x03, 0x12,
    0x03, 0x3e, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x16, 0x12, 0x03, 0x3f, 0x08,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x16, 0x04, 0x12, 0x03, 0x3f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x16, 0x05, 0x12, 0x03, 0x3f, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x16, 0x01, 0x12, 0x03, 0x3f, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x16, 0x03, 0x12, 0x03, 0x3f, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x17, 0x12, 0x03, 0x40, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x17, 0x04, 0x12,
    0x03, 0x40, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x17, 0x05, 0x12, 0x03, 0x40,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x17, 0x01, 0x12, 0x03, 0x40, 0x18, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x17, 0x03, 0x12, 0x03, 0x40, 0x29, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x18, 0x12, 0x03, 0x41, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x18, 0x04, 0x12, 0x03, 0x41, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x18, 0x05, 0x12, 0x03, 0x41, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x18, 0x01,
    0x12, 0x03, 0x41, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x18, 0x03, 0x12, 0x03,
    0x41, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x19, 0x12, 0x03, 0x42, 0x08, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x19, 0x04, 0x12, 0x03, 0x42, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x19, 0x05, 0x12, 0x03, 0x42, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x19, 0x01, 0x12, 0x03, 0x42, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x19, 0x03, 0x12, 0x03, 0x42, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x1a,
    0x12, 0x03, 0x43, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x1a, 0x04, 0x12, 0x03,
    0x43, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x1a, 0x05, 0x12, 0x03, 0x43, 0x11,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x43, 0x16, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x1a, 0x03, 0x12, 0x03, 0x43, 0x24, 0x26, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x46, 0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x46, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x47,
    0x08, 0x4c, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x47, 0x10,
    0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x48, 0x10, 0x2f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x48, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x48, 0x19, 0x1f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x20, 0x2a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x48, 0x2d, 0x2e,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x49, 0x10, 0x30, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x49, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x49, 0x19, 0x1f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x49, 0x20, 0x2b, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x49, 0x2e, 0x2f, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x4a, 0x10, 0x28, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4a, 0x10, 0x18, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4a, 0x19, 0x1d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x1e, 0x23, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x26, 0x27, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x4b, 0x10, 0x26, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4b, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4b, 0x19, 0x1d, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4b, 0x1e, 0x21, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4b, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x4e, 0x11, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4e, 0x29, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e,
    0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4f, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4f, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x4f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12,
    0x03, 0x50, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x50,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x50, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x50, 0x18, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x50, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x51, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x51, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x51, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x51, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x51, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x52, 0x08, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x52, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x52, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x52, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03,
    0x53, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x53, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x53, 0x11, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x53, 0x16, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x53, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x06, 0x12, 0x03, 0x54, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x54, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x54, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x54,
    0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x54, 0x1f, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x55, 0x08, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x03, 0x55, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x07, 0x05, 0x12, 0x03, 0x55, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x55, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x55, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x56,
    0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x03, 0x56, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x05, 0x12, 0x03, 0x56, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x56, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x08, 0x03, 0x12, 0x03, 0x56, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x09, 0x12, 0x03, 0x57, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x04,
    0x12, 0x03, 0x57, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x57, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x57, 0x18,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x03, 0x12, 0x03, 0x57, 0x28, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0a, 0x12, 0x03, 0x58, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x58, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0a, 0x05, 0x12, 0x03, 0x58, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x58, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x03, 0x12,
    0x03, 0x58, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0b, 0x12, 0x03, 0x59, 0x08,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x59, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x59, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x59, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x59, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x0c, 0x12, 0x03, 0x5a, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x04, 0x12,
    0x03, 0x5a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x5a,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x5a, 0x18, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x5a, 0x27, 0x29, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x0d, 0x12, 0x03, 0x5b, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x5b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0d, 0x05, 0x12, 0x03, 0x5b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x01,
    0x12, 0x03, 0x5b, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0d, 0x03, 0x12, 0x03,
    0x5b, 0x1e, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0e, 0x12, 0x03, 0x5c, 0x08, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x5c, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x5c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x5c, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0e, 0x03, 0x12, 0x03, 0x5c, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0f,
    0x12, 0x03, 0x5d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x04, 0x12, 0x03,
    0x5d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x5d, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x5d, 0x18, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x5d, 0x21, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x10, 0x12, 0x03, 0x5e, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x10, 0x04, 0x12, 0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x10,
    0x05, 0x12, 0x03, 0x5e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x10, 0x01, 0x12,
    0x03, 0x5e, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x10, 0x03, 0x12, 0x03, 0x5e,
    0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x11, 0x12, 0x03, 0x5f, 0x08, 0x39, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x11, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x11, 0x05, 0x12, 0x03, 0x5f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x11, 0x01, 0x12, 0x03, 0x5f, 0x18, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x11, 0x03, 0x12, 0x03, 0x5f, 0x36, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x12, 0x12,
    0x03, 0x60, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x12, 0x04, 0x12, 0x03, 0x60,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x12, 0x05, 0x12, 0x03, 0x60, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x12, 0x01, 0x12, 0x03, 0x60, 0x18, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x12, 0x03, 0x12, 0x03, 0x60, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x13, 0x12, 0x03, 0x61, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x13, 0x04, 0x12, 0x03, 0x61, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x13, 0x05,
    0x12, 0x03, 0x61, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x13, 0x01, 0x12, 0x03,
    0x61, 0x18, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x13, 0x03, 0x12, 0x03, 0x61, 0x33,
    0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x14, 0x12, 0x03, 0x62, 0x08, 0x32, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x14, 0x04, 0x12, 0x03, 0x62, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x14, 0x05, 0x12, 0x03, 0x62, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x14, 0x01, 0x12, 0x03, 0x62, 0x18, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x14,
    0x03, 0x12, 0x03, 0x62, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x15, 0x12, 0x03,
    0x63, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x15, 0x04, 0x12, 0x03, 0x63, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x15, 0x05, 0x12, 0x03, 0x63, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x15, 0x01, 0x12, 0x03, 0x63, 0x18, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x15, 0x03, 0x12, 0x03, 0x63, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x16, 0x12, 0x03, 0x64, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x16,
    0x04, 0x12, 0x03, 0x64, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x16, 0x05, 0x12,
    0x03, 0x64, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x16, 0x01, 0x12, 0x03, 0x64,
    0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x16, 0x03, 0x12, 0x03, 0x64, 0x2b, 0x2d,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x67, 0x00, 0x6a, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x67, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x68, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x68, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x68, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x68, 0x18, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x68, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x69, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x69, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x69, 0x11, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x69, 0x22, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x69,
    0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x6c, 0x00, 0x6d, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x6c, 0x08, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x04, 0x6f, 0x00, 0x79, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x6f,
    0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x70, 0x08, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x70, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x70, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x03, 0x71, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x71,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x71, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x18, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x71, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x72, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x72, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x72, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x72, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x72, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x73, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x03, 0x73, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x73, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x73, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03,
    0x74, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x74, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x03, 0x74, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x74, 0x18, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x03, 0x74, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x05, 0x12, 0x03, 0x75, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x75, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x75, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x03, 0x75,
    0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x03, 0x12, 0x03, 0x75, 0x27, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x06, 0x12, 0x03, 0x76, 0x08, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x06, 0x04, 0x12, 0x03, 0x76, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x06, 0x05, 0x12, 0x03, 0x76, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x76, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x76, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x07, 0x12, 0x03, 0x77,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x04, 0x12, 0x03, 0x77, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x05, 0x12, 0x03, 0x77, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x01, 0x12, 0x03, 0x77, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x07, 0x03, 0x12, 0x03, 0x77, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x08, 0x12, 0x03, 0x78, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x04,
    0x12, 0x03, 0x78, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x78, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x01, 0x12, 0x03, 0x78, 0x16,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x03, 0x12, 0x03, 0x78, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x05, 0x7b, 0x00, 0x94, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x08, 0x01, 0x12, 0x03, 0x7b, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x04, 0x04, 0x08, 0x04, 0x00,
    0x12, 0x05, 0x7c, 0x08, 0x90, 0x01, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x7c, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x7d, 0x10, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x7d, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x7d, 0x1a, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x7e, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x7e, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x7e, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x7f,
    0x10, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7f,
    0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x7f,
    0x1d, 0x1e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x80, 0x01,
    0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x80,
    0x01, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04,
    0x80, 0x01, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04,
    0x81, 0x01, 0x10, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x81, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x81, 0x01, 0x1d, 0x1e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x05,
    0x12, 0x04, 0x82, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x04, 0x82, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02,
    0x05, 0x02, 0x12, 0x04, 0x82, 0x01, 0x20, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x04, 0x83, 0x01, 0x10, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00,
    0x02, 0x06, 0x01, 0x12, 0x04, 0x83, 0x01, 0x10, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04,
    0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0x83, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08,
    0x04, 0x00, 0x02, 0x07, 0x12, 0x04, 0x84, 0x01, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08,
    0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0x84, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x08, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0x84, 0x01, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x08, 0x04, 0x00, 0x02, 0x08, 0x12, 0x04, 0x85, 0x01, 0x10, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x08, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x04, 0x85, 0x01, 0x10, 0x19, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x04, 0x85, 0x01, 0x1c, 0x1d, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x09, 0x12, 0x04, 0x86, 0x01, 0x10, 0x21, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0x86, 0x01, 0x10, 0x1c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0x86, 0x01, 0x1f, 0x20,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x04, 0x87, 0x01, 0x10, 0x21,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x87, 0x01, 0x10,
    0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x04, 0x87, 0x01,
    0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x04, 0x88, 0x01,
    0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x88,
    0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x04,
    0x88, 0x01, 0x24, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x04,
    0x89, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12,
    0x04, 0x89, 0x01, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0c, 0x02,
    0x12, 0x04, 0x89, 0x01, 0x2d, 0x2f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0d,
    0x12, 0x04, 0x8a, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x0d,
    0x01, 0x12, 0x04, 0x8a, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02,
    0x0d, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x1a, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00,
    0x02, 0x0e, 0x12, 0x04, 0x8b, 0x01, 0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00,
    0x02, 0x0e, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04,
    0x00, 0x02, 0x0e, 0x02, 0x12, 0x04, 0x8b, 0x01, 0x31, 0x33, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08,
    0x04, 0x00, 0x02, 0x0f, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08,
    0x04, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x08, 0x04, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x25, 0x27, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x08, 0x04, 0x00, 0x02, 0x10, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x08, 0x04, 0x00, 0x02, 0x10, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x10, 0x02, 0x12, 0x04, 0x8d, 0x01, 0x2c, 0x2e, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x11, 0x12, 0x04, 0x8e, 0x01, 0x10, 0x30, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x10, 0x2a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x11, 0x02, 0x12, 0x04, 0x8e, 0x01, 0x2d, 0x2f,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x04, 0x00, 0x02, 0x12, 0x12, 0x04, 0x8f, 0x01, 0x10, 0x2a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x12, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x10,
    0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x04, 0x00, 0x02, 0x12, 0x02, 0x12, 0x04, 0x8f, 0x01,
    0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0x92, 0x01, 0x08, 0x52,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x92, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x04, 0x92, 0x01, 0x11, 0x32, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0x92, 0x01, 0x33, 0x39, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0x92, 0x01, 0x3c, 0x3d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x08, 0x12, 0x04, 0x92, 0x01, 0x3e, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x07, 0x12, 0x04, 0x92, 0x01, 0x49, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x01, 0x12, 0x04, 0x93, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x93, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x93, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x93, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x93, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x06, 0x96, 0x01, 0x00, 0xa1,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x1f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0x97, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x97, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x04, 0x97, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x97, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x97, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x01, 0x12, 0x04, 0x98, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x98, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x98, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x98, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x98,
    0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x04, 0x99, 0x01, 0x08,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x04, 0x99, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0x99, 0x01, 0x18, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0x99, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x03, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x03, 0x05, 0x12, 0x04, 0x9a, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x9a, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04, 0x12,
    0x04, 0x9b, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x9b, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x05, 0x12, 0x04, 0x9b,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x04, 0x9b, 0x01,
    0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x04, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x05, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x05, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x06, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x06, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06,
    0x05, 0x12, 0x04, 0x9d, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x01,
    0x12, 0x04, 0x9d, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x03, 0x12,
    0x04, 0x9d, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x07, 0x12, 0x04, 0x9e,
    0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x04, 0x12, 0x04, 0x9e, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x05, 0x12, 0x04, 0x9e, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x18, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x27, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x08, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x08, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x08, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x08, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x18, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x08, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x09, 0x12, 0x04, 0xa0, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x04,
    0x12, 0x04, 0xa0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x05, 0x12,
    0x04, 0xa0, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x01, 0x12, 0x04,
    0xa0, 0x01, 0x16, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x03, 0x12, 0x04, 0xa0,
    0x01, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0xa3, 0x01, 0x00, 0xad, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x27, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x0a, 0x04, 0x00, 0x12, 0x06, 0xa4, 0x01, 0x08, 0xaa, 0x01, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x04, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xa5, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x10, 0x17, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xa5, 0x01, 0x1a, 0x1b, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x10, 0x31, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x10, 0x2c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xa6, 0x01, 0x2f,
    0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xa7, 0x01, 0x10,
    0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa7, 0x01,
    0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xa7,
    0x01, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xa8,
    0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xa8, 0x01, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12,
    0x04, 0xa8, 0x01, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x04, 0xa9, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x04, 0xa9, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0a, 0x04, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x04, 0xa9, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12,
    0x04, 0xac, 0x01, 0x08, 0x57, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xac, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xac,
    0x01, 0x11, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xac, 0x01,
    0x38, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xac, 0x01, 0x41,
    0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x08, 0x12, 0x04, 0xac, 0x01, 0x43, 0x56,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x07, 0x12, 0x04, 0xac, 0x01, 0x4e, 0x55, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xaf, 0x01, 0x00, 0xb2, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x00, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xb0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xb0, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xb0, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x04, 0xb1, 0x01,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x1e, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0xb4, 0x01, 0x00, 0xb5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0c, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12,
    0x06, 0xb7, 0x01, 0x00, 0xc3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04,
    0xb7, 0x01, 0x08, 0x23, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0d, 0x03, 0x00, 0x12, 0x06, 0xb8, 0x01,
    0x08, 0xbf, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x03, 0x00, 0x01, 0x12, 0x04, 0xb8,
    0x01, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb9,
    0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xb9, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xb9, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xb9, 0x01, 0x20, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xb9, 0x01, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x03, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xba, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xba, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xba, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xba, 0x01, 0x20, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xba, 0x01, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x0d, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xbb, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0d, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0d, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbb, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x20, 0x2c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x2f, 0x30, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x10, 0x29, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x10, 0x18,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x04, 0xbc, 0x01, 0x19,
    0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbc, 0x01,
    0x20, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xbc,
    0x01, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x04, 0x12, 0x04, 0xbd,
    0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xbd, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12,
    0x04, 0xbd, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x04, 0xbd, 0x01, 0x20, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x04, 0xbd, 0x01, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x03, 0x00, 0x02,
    0x05, 0x12, 0x04, 0xbe, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02,
    0x05, 0x04, 0x12, 0x04, 0xbe, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00,
    0x02, 0x05, 0x05, 0x12, 0x04, 0xbe, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x03,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x20, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d,
    0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x00, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x06, 0x12, 0x04, 0xc1, 0x01, 0x11, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xc1, 0x01, 0x37, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xc1, 0x01, 0x3f, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0xc2,
    0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc2, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc2, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x18, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xc5, 0x01, 0x00, 0xc8, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x00, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xc6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xc6, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xc6, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0xc7, 0x01,
    0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc7, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc7, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x18, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc7, 0x01, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xca, 0x01, 0x00, 0xce, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0f, 0x01, 0x12, 0x04, 0xca, 0x01, 0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x00, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xcb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xcb, 0x01, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xcb, 0x01, 0x23, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcb,
    0x01, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x08, 0x12, 0x04, 0xcb, 0x01,
    0x2e, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x07, 0x12, 0x04, 0xcb, 0x01, 0x39,
    0x4c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xcd, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xcd, 0x01, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xcd, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xd0, 0x01, 0x00,
    0xd5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x2d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x2f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xd2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xd2, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xd2, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x01,
    0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd3, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd3, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x18, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x23, 0x24, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x03, 0x05, 0x12, 0x04, 0xd4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06,
    0xd7, 0x01, 0x00, 0xd9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xd7,
    0x01, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xd8, 0x01, 0x08,
    0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd8, 0x01, 0x11, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x23, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x08, 0x12, 0x04, 0xd8, 0x01, 0x2e, 0x4d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x07, 0x12, 0x04, 0xd8, 0x01, 0x39, 0x4c, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x12, 0x12, 0x06, 0xdb, 0x01, 0x00, 0xde, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01,
    0x12, 0x04, 0xdb, 0x01, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04,
    0xdc, 0x01, 0x08, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdc,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x04, 0xdc, 0x01,
    0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x23,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdc, 0x01, 0x2c, 0x2d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x08, 0x12, 0x04, 0xdc, 0x01, 0x2e, 0x4d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x07, 0x12, 0x04, 0xdc, 0x01, 0x39, 0x4c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0xdd, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06,
    0xe0, 0x01, 0x00, 0xe3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xe0,
    0x01, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x01, 0x08,
    0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe1, 0x01, 0x11, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x23, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x08, 0x12, 0x04, 0xe1, 0x01, 0x2e, 0x4d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x07, 0x12, 0x04, 0xe1, 0x01, 0x39, 0x4c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xe2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xe2, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xe2, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xe5, 0x01, 0x00,
    0xe8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xe7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xe7, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xe7, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xea, 0x01, 0x00, 0xf5,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xea, 0x01, 0x08, 0x26, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x15, 0x04, 0x00, 0x12, 0x06, 0xeb, 0x01, 0x08, 0xf2, 0x01, 0x09, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x04, 0x00, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x0d, 0x13, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xec, 0x01, 0x10, 0x1c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xec, 0x01, 0x10, 0x17, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xec, 0x01, 0x1a, 0x1b,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xed, 0x01, 0x10, 0x31,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xed, 0x01, 0x10,
    0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xed, 0x01,
    0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xee, 0x01,
    0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xee,
    0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04,
    0xee, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04,
    0xef, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xef, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x03, 0x02,
    0x12, 0x04, 0xef, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x04, 0xf0, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xf0, 0x01, 0x10, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02,
    0x04, 0x02, 0x12, 0x04, 0xf0, 0x01, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00,
    0x02, 0x05, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04,
    0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xf1, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x00, 0x12, 0x04, 0xf4, 0x01, 0x08, 0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xf4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xf4, 0x01, 0x11, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xf4, 0x01, 0x37, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xf4, 0x01, 0x40, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x08, 0x12, 0x04, 0xf4,
    0x01, 0x42, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x07, 0x12, 0x04, 0xf4, 0x01,
    0x4d, 0x54, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xf7, 0x01, 0x00, 0xfa, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xf8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xf8, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12,
    0x04, 0xf9, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xf9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf9,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf9, 0x01,
    0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xfc, 0x01, 0x00, 0x87, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x08, 0x29, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x17, 0x04, 0x00, 0x12, 0x06, 0xfd, 0x01, 0x08, 0x84, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x04, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x17, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xfe, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x17, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x17, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xfe, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xff, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xff, 0x01, 0x10, 0x2c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xff, 0x01, 0x2f, 0x30, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x80, 0x02, 0x10, 0x26, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x80, 0x02, 0x10, 0x21,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x80, 0x02, 0x24,
    0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x81, 0x02, 0x10,
    0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x81, 0x02,
    0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x81,
    0x02, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0x82,
    0x02, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x82, 0x02, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x82, 0x02, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x04, 0x00, 0x02, 0x05, 0x12,
    0x04, 0x83, 0x02, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x83, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x04, 0x00, 0x02, 0x05,
    0x02, 0x12, 0x04, 0x83, 0x02, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12,
    0x04, 0x86, 0x02, 0x08, 0x59, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x86, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x06, 0x12, 0x04, 0x86,
    0x02, 0x11, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x02,
    0x3a, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x02, 0x43,
    0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x08, 0x12, 0x04, 0x86, 0x02, 0x45, 0x58,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x07, 0x12, 0x04, 0x86, 0x02, 0x50, 0x57, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0x89, 0x02, 0x00, 0x8d, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0x89, 0x02, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x00, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x8a, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x8a, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x8a, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x02,
    0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x18, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x2c, 0x2d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8c, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06,
    0x8f, 0x02, 0x00, 0x9c, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0x8f,
    0x02, 0x08, 0x25, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x19, 0x04, 0x00, 0x12, 0x06, 0x90, 0x02, 0x08,
    0x99, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x04, 0x00, 0x01, 0x12, 0x04, 0x90, 0x02,
    0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x91, 0x02,
    0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x91,
    0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04,
    0x91, 0x02, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04,
    0x92, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x92, 0x02, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x04, 0x92, 0x02, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x04, 0x93, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x93, 0x02, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x04, 0x93, 0x02, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x04, 0x94, 0x02, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x94, 0x02, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x94, 0x02, 0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19,
    0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0x95, 0x02, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19,
    0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0x95, 0x02, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x19, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0x95, 0x02, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x19, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0x96, 0x02, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x19, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0x96, 0x02, 0x10, 0x1f, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0x96, 0x02, 0x22, 0x23, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x19, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0x97, 0x02, 0x10, 0x28, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0x97, 0x02, 0x10, 0x23, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0x97, 0x02, 0x26, 0x27,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19, 0x04, 0x00, 0x02, 0x07, 0x12, 0x04, 0x98, 0x02, 0x10, 0x2e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0x98, 0x02, 0x10,
    0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0x98, 0x02,
    0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0x9b, 0x02, 0x08, 0x55,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9b, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9b, 0x02, 0x11, 0x35, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x36, 0x3c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9b, 0x02, 0x3f, 0x40, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x08, 0x12, 0x04, 0x9b, 0x02, 0x41, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x07, 0x12, 0x04, 0x9b, 0x02, 0x4c, 0x53, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a,
    0x12, 0x06, 0x9e, 0x02, 0x00, 0xa0, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12,
    0x04, 0x9e, 0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0x9f,
    0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9f, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9f, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x02, 0x18, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f, 0x02, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xa2, 0x02, 0x00, 0xab, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xa2, 0x02, 0x08, 0x21, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x1b,
    0x04, 0x00, 0x12, 0x06, 0xa3, 0x02, 0x08, 0xa8, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x04, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x04, 0xa4, 0x02, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b,
    0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xa4, 0x02, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x1b, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1b, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1b, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xa5, 0x02, 0x25, 0x26, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xa6, 0x02, 0x10, 0x28, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x10, 0x23, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xa6, 0x02, 0x26, 0x27, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xa7, 0x02, 0x10, 0x2e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x10, 0x29,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1b, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xa7, 0x02, 0x2c,
    0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x02, 0x08, 0x51, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaa, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xaa, 0x02, 0x11, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x02, 0x32, 0x38, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaa, 0x02, 0x3b, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x00, 0x08, 0x12, 0x04, 0xaa, 0x02, 0x3d, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x00, 0x07, 0x12, 0x04, 0xaa, 0x02, 0x48, 0x4f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12,
    0x06, 0xad, 0x02, 0x00, 0xaf, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04,
    0xad, 0x02, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xae, 0x02,
    0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xae, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xae, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xae, 0x02, 0x18, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xae, 0x02, 0x28, 0x29,
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
