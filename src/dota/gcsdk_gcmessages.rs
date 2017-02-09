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
pub struct CMsgSHA1Digest {
    // message fields
    block1: ::std::option::Option<u64>,
    block2: ::std::option::Option<u64>,
    block3: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSHA1Digest {}

impl CMsgSHA1Digest {
    pub fn new() -> CMsgSHA1Digest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSHA1Digest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSHA1Digest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSHA1Digest,
        };
        unsafe {
            instance.get(CMsgSHA1Digest::new)
        }
    }

    // required fixed64 block1 = 1;

    pub fn clear_block1(&mut self) {
        self.block1 = ::std::option::Option::None;
    }

    pub fn has_block1(&self) -> bool {
        self.block1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block1(&mut self, v: u64) {
        self.block1 = ::std::option::Option::Some(v);
    }

    pub fn get_block1(&self) -> u64 {
        self.block1.unwrap_or(0)
    }

    fn get_block1_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.block1
    }

    fn mut_block1_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.block1
    }

    // required fixed64 block2 = 2;

    pub fn clear_block2(&mut self) {
        self.block2 = ::std::option::Option::None;
    }

    pub fn has_block2(&self) -> bool {
        self.block2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block2(&mut self, v: u64) {
        self.block2 = ::std::option::Option::Some(v);
    }

    pub fn get_block2(&self) -> u64 {
        self.block2.unwrap_or(0)
    }

    fn get_block2_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.block2
    }

    fn mut_block2_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.block2
    }

    // required fixed32 block3 = 3;

    pub fn clear_block3(&mut self) {
        self.block3 = ::std::option::Option::None;
    }

    pub fn has_block3(&self) -> bool {
        self.block3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block3(&mut self, v: u32) {
        self.block3 = ::std::option::Option::Some(v);
    }

    pub fn get_block3(&self) -> u32 {
        self.block3.unwrap_or(0)
    }

    fn get_block3_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.block3
    }

    fn mut_block3_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.block3
    }
}

impl ::protobuf::Message for CMsgSHA1Digest {
    fn is_initialized(&self) -> bool {
        if self.block1.is_none() {
            return false;
        };
        if self.block2.is_none() {
            return false;
        };
        if self.block3.is_none() {
            return false;
        };
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
                    self.block1 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.block2 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.block3 = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.block1 {
            my_size += 9;
        };
        if let Some(v) = self.block2 {
            my_size += 9;
        };
        if let Some(v) = self.block3 {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.block1 {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.block2 {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.block3 {
            os.write_fixed32(3, v)?;
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

impl ::protobuf::MessageStatic for CMsgSHA1Digest {
    fn new() -> CMsgSHA1Digest {
        CMsgSHA1Digest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSHA1Digest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "block1",
                    CMsgSHA1Digest::get_block1_for_reflect,
                    CMsgSHA1Digest::mut_block1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "block2",
                    CMsgSHA1Digest::get_block2_for_reflect,
                    CMsgSHA1Digest::mut_block2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "block3",
                    CMsgSHA1Digest::get_block3_for_reflect,
                    CMsgSHA1Digest::mut_block3_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSHA1Digest>(
                    "CMsgSHA1Digest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSHA1Digest {
    fn clear(&mut self) {
        self.clear_block1();
        self.clear_block2();
        self.clear_block3();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSHA1Digest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSHA1Digest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOIDOwner {
    // message fields
    field_type: ::std::option::Option<u32>,
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOIDOwner {}

impl CMsgSOIDOwner {
    pub fn new() -> CMsgSOIDOwner {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOIDOwner {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOIDOwner> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOIDOwner,
        };
        unsafe {
            instance.get(CMsgSOIDOwner::new)
        }
    }

    // optional uint32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: u32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> u32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.field_type
    }

    // optional uint64 id = 2;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }
}

impl ::protobuf::Message for CMsgSOIDOwner {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.id {
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

impl ::protobuf::MessageStatic for CMsgSOIDOwner {
    fn new() -> CMsgSOIDOwner {
        CMsgSOIDOwner::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOIDOwner>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "type",
                    CMsgSOIDOwner::get_field_type_for_reflect,
                    CMsgSOIDOwner::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CMsgSOIDOwner::get_id_for_reflect,
                    CMsgSOIDOwner::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOIDOwner>(
                    "CMsgSOIDOwner",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOIDOwner {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOIDOwner {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOIDOwner {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOSingleObject {
    // message fields
    type_id: ::std::option::Option<i32>,
    object_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    version: ::std::option::Option<u64>,
    owner_soid: ::protobuf::SingularPtrField<CMsgSOIDOwner>,
    service_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOSingleObject {}

impl CMsgSOSingleObject {
    pub fn new() -> CMsgSOSingleObject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOSingleObject {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOSingleObject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOSingleObject,
        };
        unsafe {
            instance.get(CMsgSOSingleObject::new)
        }
    }

    // optional int32 type_id = 2;

    pub fn clear_type_id(&mut self) {
        self.type_id = ::std::option::Option::None;
    }

    pub fn has_type_id(&self) -> bool {
        self.type_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type_id(&mut self, v: i32) {
        self.type_id = ::std::option::Option::Some(v);
    }

    pub fn get_type_id(&self) -> i32 {
        self.type_id.unwrap_or(0)
    }

    fn get_type_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.type_id
    }

    fn mut_type_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.type_id
    }

    // optional bytes object_data = 3;

    pub fn clear_object_data(&mut self) {
        self.object_data.clear();
    }

    pub fn has_object_data(&self) -> bool {
        self.object_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.object_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.object_data.is_none() {
            self.object_data.set_default();
        };
        self.object_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_object_data(&mut self) -> ::std::vec::Vec<u8> {
        self.object_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_object_data(&self) -> &[u8] {
        match self.object_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_object_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.object_data
    }

    fn mut_object_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.object_data
    }

    // optional fixed64 version = 4;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }

    // optional .dota.CMsgSOIDOwner owner_soid = 5;

    pub fn clear_owner_soid(&mut self) {
        self.owner_soid.clear();
    }

    pub fn has_owner_soid(&self) -> bool {
        self.owner_soid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_soid(&mut self, v: CMsgSOIDOwner) {
        self.owner_soid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_soid(&mut self) -> &mut CMsgSOIDOwner {
        if self.owner_soid.is_none() {
            self.owner_soid.set_default();
        };
        self.owner_soid.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_soid(&mut self) -> CMsgSOIDOwner {
        self.owner_soid.take().unwrap_or_else(|| CMsgSOIDOwner::new())
    }

    pub fn get_owner_soid(&self) -> &CMsgSOIDOwner {
        self.owner_soid.as_ref().unwrap_or_else(|| CMsgSOIDOwner::default_instance())
    }

    fn get_owner_soid_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &self.owner_soid
    }

    fn mut_owner_soid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &mut self.owner_soid
    }

    // optional uint32 service_id = 6;

    pub fn clear_service_id(&mut self) {
        self.service_id = ::std::option::Option::None;
    }

    pub fn has_service_id(&self) -> bool {
        self.service_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: u32) {
        self.service_id = ::std::option::Option::Some(v);
    }

    pub fn get_service_id(&self) -> u32 {
        self.service_id.unwrap_or(0)
    }

    fn get_service_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service_id
    }

    fn mut_service_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service_id
    }
}

impl ::protobuf::Message for CMsgSOSingleObject {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.type_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.object_data)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.owner_soid)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.service_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.type_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.object_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        if let Some(v) = self.version {
            my_size += 9;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.service_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.type_id {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.object_data.as_ref() {
            os.write_bytes(3, &v)?;
        };
        if let Some(v) = self.version {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.service_id {
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

impl ::protobuf::MessageStatic for CMsgSOSingleObject {
    fn new() -> CMsgSOSingleObject {
        CMsgSOSingleObject::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOSingleObject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type_id",
                    CMsgSOSingleObject::get_type_id_for_reflect,
                    CMsgSOSingleObject::mut_type_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "object_data",
                    CMsgSOSingleObject::get_object_data_for_reflect,
                    CMsgSOSingleObject::mut_object_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "version",
                    CMsgSOSingleObject::get_version_for_reflect,
                    CMsgSOSingleObject::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOIDOwner>>(
                    "owner_soid",
                    CMsgSOSingleObject::get_owner_soid_for_reflect,
                    CMsgSOSingleObject::mut_owner_soid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_id",
                    CMsgSOSingleObject::get_service_id_for_reflect,
                    CMsgSOSingleObject::mut_service_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOSingleObject>(
                    "CMsgSOSingleObject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOSingleObject {
    fn clear(&mut self) {
        self.clear_type_id();
        self.clear_object_data();
        self.clear_version();
        self.clear_owner_soid();
        self.clear_service_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOSingleObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOSingleObject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOMultipleObjects {
    // message fields
    objects_modified: ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject>,
    version: ::std::option::Option<u64>,
    objects_added: ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject>,
    objects_removed: ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject>,
    owner_soid: ::protobuf::SingularPtrField<CMsgSOIDOwner>,
    service_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOMultipleObjects {}

impl CMsgSOMultipleObjects {
    pub fn new() -> CMsgSOMultipleObjects {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOMultipleObjects {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOMultipleObjects> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOMultipleObjects,
        };
        unsafe {
            instance.get(CMsgSOMultipleObjects::new)
        }
    }

    // repeated .dota.CMsgSOMultipleObjects.SingleObject objects_modified = 2;

    pub fn clear_objects_modified(&mut self) {
        self.objects_modified.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects_modified(&mut self, v: ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject>) {
        self.objects_modified = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects_modified(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &mut self.objects_modified
    }

    // Take field
    pub fn take_objects_modified(&mut self) -> ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        ::std::mem::replace(&mut self.objects_modified, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects_modified(&self) -> &[CMsgSOMultipleObjects_SingleObject] {
        &self.objects_modified
    }

    fn get_objects_modified_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &self.objects_modified
    }

    fn mut_objects_modified_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &mut self.objects_modified
    }

    // optional fixed64 version = 3;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }

    // repeated .dota.CMsgSOMultipleObjects.SingleObject objects_added = 4;

    pub fn clear_objects_added(&mut self) {
        self.objects_added.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects_added(&mut self, v: ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject>) {
        self.objects_added = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects_added(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &mut self.objects_added
    }

    // Take field
    pub fn take_objects_added(&mut self) -> ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        ::std::mem::replace(&mut self.objects_added, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects_added(&self) -> &[CMsgSOMultipleObjects_SingleObject] {
        &self.objects_added
    }

    fn get_objects_added_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &self.objects_added
    }

    fn mut_objects_added_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &mut self.objects_added
    }

    // repeated .dota.CMsgSOMultipleObjects.SingleObject objects_removed = 5;

    pub fn clear_objects_removed(&mut self) {
        self.objects_removed.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects_removed(&mut self, v: ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject>) {
        self.objects_removed = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects_removed(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &mut self.objects_removed
    }

    // Take field
    pub fn take_objects_removed(&mut self) -> ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        ::std::mem::replace(&mut self.objects_removed, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects_removed(&self) -> &[CMsgSOMultipleObjects_SingleObject] {
        &self.objects_removed
    }

    fn get_objects_removed_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &self.objects_removed
    }

    fn mut_objects_removed_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOMultipleObjects_SingleObject> {
        &mut self.objects_removed
    }

    // optional .dota.CMsgSOIDOwner owner_soid = 6;

    pub fn clear_owner_soid(&mut self) {
        self.owner_soid.clear();
    }

    pub fn has_owner_soid(&self) -> bool {
        self.owner_soid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_soid(&mut self, v: CMsgSOIDOwner) {
        self.owner_soid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_soid(&mut self) -> &mut CMsgSOIDOwner {
        if self.owner_soid.is_none() {
            self.owner_soid.set_default();
        };
        self.owner_soid.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_soid(&mut self) -> CMsgSOIDOwner {
        self.owner_soid.take().unwrap_or_else(|| CMsgSOIDOwner::new())
    }

    pub fn get_owner_soid(&self) -> &CMsgSOIDOwner {
        self.owner_soid.as_ref().unwrap_or_else(|| CMsgSOIDOwner::default_instance())
    }

    fn get_owner_soid_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &self.owner_soid
    }

    fn mut_owner_soid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &mut self.owner_soid
    }

    // optional uint32 service_id = 7;

    pub fn clear_service_id(&mut self) {
        self.service_id = ::std::option::Option::None;
    }

    pub fn has_service_id(&self) -> bool {
        self.service_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: u32) {
        self.service_id = ::std::option::Option::Some(v);
    }

    pub fn get_service_id(&self) -> u32 {
        self.service_id.unwrap_or(0)
    }

    fn get_service_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service_id
    }

    fn mut_service_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service_id
    }
}

impl ::protobuf::Message for CMsgSOMultipleObjects {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects_modified)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects_added)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects_removed)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.owner_soid)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.service_id = ::std::option::Option::Some(tmp);
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
        for value in &self.objects_modified {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.version {
            my_size += 9;
        };
        for value in &self.objects_added {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.objects_removed {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.service_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.objects_modified {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.version {
            os.write_fixed64(3, v)?;
        };
        for v in &self.objects_added {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.objects_removed {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.service_id {
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

impl ::protobuf::MessageStatic for CMsgSOMultipleObjects {
    fn new() -> CMsgSOMultipleObjects {
        CMsgSOMultipleObjects::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOMultipleObjects>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOMultipleObjects_SingleObject>>(
                    "objects_modified",
                    CMsgSOMultipleObjects::get_objects_modified_for_reflect,
                    CMsgSOMultipleObjects::mut_objects_modified_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "version",
                    CMsgSOMultipleObjects::get_version_for_reflect,
                    CMsgSOMultipleObjects::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOMultipleObjects_SingleObject>>(
                    "objects_added",
                    CMsgSOMultipleObjects::get_objects_added_for_reflect,
                    CMsgSOMultipleObjects::mut_objects_added_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOMultipleObjects_SingleObject>>(
                    "objects_removed",
                    CMsgSOMultipleObjects::get_objects_removed_for_reflect,
                    CMsgSOMultipleObjects::mut_objects_removed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOIDOwner>>(
                    "owner_soid",
                    CMsgSOMultipleObjects::get_owner_soid_for_reflect,
                    CMsgSOMultipleObjects::mut_owner_soid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_id",
                    CMsgSOMultipleObjects::get_service_id_for_reflect,
                    CMsgSOMultipleObjects::mut_service_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOMultipleObjects>(
                    "CMsgSOMultipleObjects",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOMultipleObjects {
    fn clear(&mut self) {
        self.clear_objects_modified();
        self.clear_version();
        self.clear_objects_added();
        self.clear_objects_removed();
        self.clear_owner_soid();
        self.clear_service_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOMultipleObjects {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOMultipleObjects {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOMultipleObjects_SingleObject {
    // message fields
    type_id: ::std::option::Option<i32>,
    object_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOMultipleObjects_SingleObject {}

impl CMsgSOMultipleObjects_SingleObject {
    pub fn new() -> CMsgSOMultipleObjects_SingleObject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOMultipleObjects_SingleObject {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOMultipleObjects_SingleObject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOMultipleObjects_SingleObject,
        };
        unsafe {
            instance.get(CMsgSOMultipleObjects_SingleObject::new)
        }
    }

    // optional int32 type_id = 1;

    pub fn clear_type_id(&mut self) {
        self.type_id = ::std::option::Option::None;
    }

    pub fn has_type_id(&self) -> bool {
        self.type_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type_id(&mut self, v: i32) {
        self.type_id = ::std::option::Option::Some(v);
    }

    pub fn get_type_id(&self) -> i32 {
        self.type_id.unwrap_or(0)
    }

    fn get_type_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.type_id
    }

    fn mut_type_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.type_id
    }

    // optional bytes object_data = 2;

    pub fn clear_object_data(&mut self) {
        self.object_data.clear();
    }

    pub fn has_object_data(&self) -> bool {
        self.object_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.object_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.object_data.is_none() {
            self.object_data.set_default();
        };
        self.object_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_object_data(&mut self) -> ::std::vec::Vec<u8> {
        self.object_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_object_data(&self) -> &[u8] {
        match self.object_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_object_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.object_data
    }

    fn mut_object_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.object_data
    }
}

impl ::protobuf::Message for CMsgSOMultipleObjects_SingleObject {
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
                    self.type_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.object_data)?;
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
        if let Some(v) = self.type_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.object_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.type_id {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.object_data.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSOMultipleObjects_SingleObject {
    fn new() -> CMsgSOMultipleObjects_SingleObject {
        CMsgSOMultipleObjects_SingleObject::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOMultipleObjects_SingleObject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type_id",
                    CMsgSOMultipleObjects_SingleObject::get_type_id_for_reflect,
                    CMsgSOMultipleObjects_SingleObject::mut_type_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "object_data",
                    CMsgSOMultipleObjects_SingleObject::get_object_data_for_reflect,
                    CMsgSOMultipleObjects_SingleObject::mut_object_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOMultipleObjects_SingleObject>(
                    "CMsgSOMultipleObjects_SingleObject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOMultipleObjects_SingleObject {
    fn clear(&mut self) {
        self.clear_type_id();
        self.clear_object_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOMultipleObjects_SingleObject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOMultipleObjects_SingleObject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOCacheSubscribed {
    // message fields
    objects: ::protobuf::RepeatedField<CMsgSOCacheSubscribed_SubscribedType>,
    version: ::std::option::Option<u64>,
    owner_soid: ::protobuf::SingularPtrField<CMsgSOIDOwner>,
    service_id: ::std::option::Option<u32>,
    service_list: ::std::vec::Vec<u32>,
    sync_version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOCacheSubscribed {}

impl CMsgSOCacheSubscribed {
    pub fn new() -> CMsgSOCacheSubscribed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOCacheSubscribed {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOCacheSubscribed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOCacheSubscribed,
        };
        unsafe {
            instance.get(CMsgSOCacheSubscribed::new)
        }
    }

    // repeated .dota.CMsgSOCacheSubscribed.SubscribedType objects = 2;

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects(&mut self, v: ::protobuf::RepeatedField<CMsgSOCacheSubscribed_SubscribedType>) {
        self.objects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOCacheSubscribed_SubscribedType> {
        &mut self.objects
    }

    // Take field
    pub fn take_objects(&mut self) -> ::protobuf::RepeatedField<CMsgSOCacheSubscribed_SubscribedType> {
        ::std::mem::replace(&mut self.objects, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects(&self) -> &[CMsgSOCacheSubscribed_SubscribedType] {
        &self.objects
    }

    fn get_objects_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSOCacheSubscribed_SubscribedType> {
        &self.objects
    }

    fn mut_objects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOCacheSubscribed_SubscribedType> {
        &mut self.objects
    }

    // optional fixed64 version = 3;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }

    // optional .dota.CMsgSOIDOwner owner_soid = 4;

    pub fn clear_owner_soid(&mut self) {
        self.owner_soid.clear();
    }

    pub fn has_owner_soid(&self) -> bool {
        self.owner_soid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_soid(&mut self, v: CMsgSOIDOwner) {
        self.owner_soid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_soid(&mut self) -> &mut CMsgSOIDOwner {
        if self.owner_soid.is_none() {
            self.owner_soid.set_default();
        };
        self.owner_soid.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_soid(&mut self) -> CMsgSOIDOwner {
        self.owner_soid.take().unwrap_or_else(|| CMsgSOIDOwner::new())
    }

    pub fn get_owner_soid(&self) -> &CMsgSOIDOwner {
        self.owner_soid.as_ref().unwrap_or_else(|| CMsgSOIDOwner::default_instance())
    }

    fn get_owner_soid_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &self.owner_soid
    }

    fn mut_owner_soid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &mut self.owner_soid
    }

    // optional uint32 service_id = 5;

    pub fn clear_service_id(&mut self) {
        self.service_id = ::std::option::Option::None;
    }

    pub fn has_service_id(&self) -> bool {
        self.service_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: u32) {
        self.service_id = ::std::option::Option::Some(v);
    }

    pub fn get_service_id(&self) -> u32 {
        self.service_id.unwrap_or(0)
    }

    fn get_service_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service_id
    }

    fn mut_service_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service_id
    }

    // repeated uint32 service_list = 6;

    pub fn clear_service_list(&mut self) {
        self.service_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_list(&mut self, v: ::std::vec::Vec<u32>) {
        self.service_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_service_list(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.service_list
    }

    // Take field
    pub fn take_service_list(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.service_list, ::std::vec::Vec::new())
    }

    pub fn get_service_list(&self) -> &[u32] {
        &self.service_list
    }

    fn get_service_list_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.service_list
    }

    fn mut_service_list_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.service_list
    }

    // optional fixed64 sync_version = 7;

    pub fn clear_sync_version(&mut self) {
        self.sync_version = ::std::option::Option::None;
    }

    pub fn has_sync_version(&self) -> bool {
        self.sync_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sync_version(&mut self, v: u64) {
        self.sync_version = ::std::option::Option::Some(v);
    }

    pub fn get_sync_version(&self) -> u64 {
        self.sync_version.unwrap_or(0)
    }

    fn get_sync_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sync_version
    }

    fn mut_sync_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sync_version
    }
}

impl ::protobuf::Message for CMsgSOCacheSubscribed {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.objects)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.owner_soid)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.service_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.service_list)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.sync_version = ::std::option::Option::Some(tmp);
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
        for value in &self.objects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.version {
            my_size += 9;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.service_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.service_list {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sync_version {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.objects {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.version {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.service_id {
            os.write_uint32(5, v)?;
        };
        for v in &self.service_list {
            os.write_uint32(6, *v)?;
        };
        if let Some(v) = self.sync_version {
            os.write_fixed64(7, v)?;
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

impl ::protobuf::MessageStatic for CMsgSOCacheSubscribed {
    fn new() -> CMsgSOCacheSubscribed {
        CMsgSOCacheSubscribed::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOCacheSubscribed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOCacheSubscribed_SubscribedType>>(
                    "objects",
                    CMsgSOCacheSubscribed::get_objects_for_reflect,
                    CMsgSOCacheSubscribed::mut_objects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "version",
                    CMsgSOCacheSubscribed::get_version_for_reflect,
                    CMsgSOCacheSubscribed::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOIDOwner>>(
                    "owner_soid",
                    CMsgSOCacheSubscribed::get_owner_soid_for_reflect,
                    CMsgSOCacheSubscribed::mut_owner_soid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_id",
                    CMsgSOCacheSubscribed::get_service_id_for_reflect,
                    CMsgSOCacheSubscribed::mut_service_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_list",
                    CMsgSOCacheSubscribed::get_service_list_for_reflect,
                    CMsgSOCacheSubscribed::mut_service_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "sync_version",
                    CMsgSOCacheSubscribed::get_sync_version_for_reflect,
                    CMsgSOCacheSubscribed::mut_sync_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOCacheSubscribed>(
                    "CMsgSOCacheSubscribed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOCacheSubscribed {
    fn clear(&mut self) {
        self.clear_objects();
        self.clear_version();
        self.clear_owner_soid();
        self.clear_service_id();
        self.clear_service_list();
        self.clear_sync_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOCacheSubscribed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOCacheSubscribed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOCacheSubscribed_SubscribedType {
    // message fields
    type_id: ::std::option::Option<i32>,
    object_data: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOCacheSubscribed_SubscribedType {}

impl CMsgSOCacheSubscribed_SubscribedType {
    pub fn new() -> CMsgSOCacheSubscribed_SubscribedType {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOCacheSubscribed_SubscribedType {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOCacheSubscribed_SubscribedType> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOCacheSubscribed_SubscribedType,
        };
        unsafe {
            instance.get(CMsgSOCacheSubscribed_SubscribedType::new)
        }
    }

    // optional int32 type_id = 1;

    pub fn clear_type_id(&mut self) {
        self.type_id = ::std::option::Option::None;
    }

    pub fn has_type_id(&self) -> bool {
        self.type_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type_id(&mut self, v: i32) {
        self.type_id = ::std::option::Option::Some(v);
    }

    pub fn get_type_id(&self) -> i32 {
        self.type_id.unwrap_or(0)
    }

    fn get_type_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.type_id
    }

    fn mut_type_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.type_id
    }

    // repeated bytes object_data = 2;

    pub fn clear_object_data(&mut self) {
        self.object_data.clear();
    }

    // Param is passed by value, moved
    pub fn set_object_data(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.object_data = v;
    }

    // Mutable pointer to the field.
    pub fn mut_object_data(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.object_data
    }

    // Take field
    pub fn take_object_data(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.object_data, ::protobuf::RepeatedField::new())
    }

    pub fn get_object_data(&self) -> &[::std::vec::Vec<u8>] {
        &self.object_data
    }

    fn get_object_data_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.object_data
    }

    fn mut_object_data_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.object_data
    }
}

impl ::protobuf::Message for CMsgSOCacheSubscribed_SubscribedType {
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
                    self.type_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.object_data)?;
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
        if let Some(v) = self.type_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.object_data {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.type_id {
            os.write_int32(1, v)?;
        };
        for v in &self.object_data {
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

impl ::protobuf::MessageStatic for CMsgSOCacheSubscribed_SubscribedType {
    fn new() -> CMsgSOCacheSubscribed_SubscribedType {
        CMsgSOCacheSubscribed_SubscribedType::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOCacheSubscribed_SubscribedType>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type_id",
                    CMsgSOCacheSubscribed_SubscribedType::get_type_id_for_reflect,
                    CMsgSOCacheSubscribed_SubscribedType::mut_type_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "object_data",
                    CMsgSOCacheSubscribed_SubscribedType::get_object_data_for_reflect,
                    CMsgSOCacheSubscribed_SubscribedType::mut_object_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOCacheSubscribed_SubscribedType>(
                    "CMsgSOCacheSubscribed_SubscribedType",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOCacheSubscribed_SubscribedType {
    fn clear(&mut self) {
        self.clear_type_id();
        self.clear_object_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOCacheSubscribed_SubscribedType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOCacheSubscribed_SubscribedType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOCacheSubscribedUpToDate {
    // message fields
    version: ::std::option::Option<u64>,
    owner_soid: ::protobuf::SingularPtrField<CMsgSOIDOwner>,
    service_id: ::std::option::Option<u32>,
    service_list: ::std::vec::Vec<u32>,
    sync_version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOCacheSubscribedUpToDate {}

impl CMsgSOCacheSubscribedUpToDate {
    pub fn new() -> CMsgSOCacheSubscribedUpToDate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOCacheSubscribedUpToDate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOCacheSubscribedUpToDate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOCacheSubscribedUpToDate,
        };
        unsafe {
            instance.get(CMsgSOCacheSubscribedUpToDate::new)
        }
    }

    // optional fixed64 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }

    // optional .dota.CMsgSOIDOwner owner_soid = 2;

    pub fn clear_owner_soid(&mut self) {
        self.owner_soid.clear();
    }

    pub fn has_owner_soid(&self) -> bool {
        self.owner_soid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_soid(&mut self, v: CMsgSOIDOwner) {
        self.owner_soid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_soid(&mut self) -> &mut CMsgSOIDOwner {
        if self.owner_soid.is_none() {
            self.owner_soid.set_default();
        };
        self.owner_soid.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_soid(&mut self) -> CMsgSOIDOwner {
        self.owner_soid.take().unwrap_or_else(|| CMsgSOIDOwner::new())
    }

    pub fn get_owner_soid(&self) -> &CMsgSOIDOwner {
        self.owner_soid.as_ref().unwrap_or_else(|| CMsgSOIDOwner::default_instance())
    }

    fn get_owner_soid_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &self.owner_soid
    }

    fn mut_owner_soid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &mut self.owner_soid
    }

    // optional uint32 service_id = 3;

    pub fn clear_service_id(&mut self) {
        self.service_id = ::std::option::Option::None;
    }

    pub fn has_service_id(&self) -> bool {
        self.service_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: u32) {
        self.service_id = ::std::option::Option::Some(v);
    }

    pub fn get_service_id(&self) -> u32 {
        self.service_id.unwrap_or(0)
    }

    fn get_service_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service_id
    }

    fn mut_service_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service_id
    }

    // repeated uint32 service_list = 4;

    pub fn clear_service_list(&mut self) {
        self.service_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_list(&mut self, v: ::std::vec::Vec<u32>) {
        self.service_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_service_list(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.service_list
    }

    // Take field
    pub fn take_service_list(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.service_list, ::std::vec::Vec::new())
    }

    pub fn get_service_list(&self) -> &[u32] {
        &self.service_list
    }

    fn get_service_list_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.service_list
    }

    fn mut_service_list_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.service_list
    }

    // optional fixed64 sync_version = 5;

    pub fn clear_sync_version(&mut self) {
        self.sync_version = ::std::option::Option::None;
    }

    pub fn has_sync_version(&self) -> bool {
        self.sync_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sync_version(&mut self, v: u64) {
        self.sync_version = ::std::option::Option::Some(v);
    }

    pub fn get_sync_version(&self) -> u64 {
        self.sync_version.unwrap_or(0)
    }

    fn get_sync_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sync_version
    }

    fn mut_sync_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sync_version
    }
}

impl ::protobuf::Message for CMsgSOCacheSubscribedUpToDate {
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
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.owner_soid)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.service_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.service_list)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.sync_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.version {
            my_size += 9;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.service_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.service_list {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sync_version {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.service_id {
            os.write_uint32(3, v)?;
        };
        for v in &self.service_list {
            os.write_uint32(4, *v)?;
        };
        if let Some(v) = self.sync_version {
            os.write_fixed64(5, v)?;
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

impl ::protobuf::MessageStatic for CMsgSOCacheSubscribedUpToDate {
    fn new() -> CMsgSOCacheSubscribedUpToDate {
        CMsgSOCacheSubscribedUpToDate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOCacheSubscribedUpToDate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "version",
                    CMsgSOCacheSubscribedUpToDate::get_version_for_reflect,
                    CMsgSOCacheSubscribedUpToDate::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOIDOwner>>(
                    "owner_soid",
                    CMsgSOCacheSubscribedUpToDate::get_owner_soid_for_reflect,
                    CMsgSOCacheSubscribedUpToDate::mut_owner_soid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_id",
                    CMsgSOCacheSubscribedUpToDate::get_service_id_for_reflect,
                    CMsgSOCacheSubscribedUpToDate::mut_service_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_list",
                    CMsgSOCacheSubscribedUpToDate::get_service_list_for_reflect,
                    CMsgSOCacheSubscribedUpToDate::mut_service_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "sync_version",
                    CMsgSOCacheSubscribedUpToDate::get_sync_version_for_reflect,
                    CMsgSOCacheSubscribedUpToDate::mut_sync_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOCacheSubscribedUpToDate>(
                    "CMsgSOCacheSubscribedUpToDate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOCacheSubscribedUpToDate {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_owner_soid();
        self.clear_service_id();
        self.clear_service_list();
        self.clear_sync_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOCacheSubscribedUpToDate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOCacheSubscribedUpToDate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOCacheUnsubscribed {
    // message fields
    owner_soid: ::protobuf::SingularPtrField<CMsgSOIDOwner>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOCacheUnsubscribed {}

impl CMsgSOCacheUnsubscribed {
    pub fn new() -> CMsgSOCacheUnsubscribed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOCacheUnsubscribed {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOCacheUnsubscribed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOCacheUnsubscribed,
        };
        unsafe {
            instance.get(CMsgSOCacheUnsubscribed::new)
        }
    }

    // optional .dota.CMsgSOIDOwner owner_soid = 2;

    pub fn clear_owner_soid(&mut self) {
        self.owner_soid.clear();
    }

    pub fn has_owner_soid(&self) -> bool {
        self.owner_soid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_soid(&mut self, v: CMsgSOIDOwner) {
        self.owner_soid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_soid(&mut self) -> &mut CMsgSOIDOwner {
        if self.owner_soid.is_none() {
            self.owner_soid.set_default();
        };
        self.owner_soid.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_soid(&mut self) -> CMsgSOIDOwner {
        self.owner_soid.take().unwrap_or_else(|| CMsgSOIDOwner::new())
    }

    pub fn get_owner_soid(&self) -> &CMsgSOIDOwner {
        self.owner_soid.as_ref().unwrap_or_else(|| CMsgSOIDOwner::default_instance())
    }

    fn get_owner_soid_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &self.owner_soid
    }

    fn mut_owner_soid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &mut self.owner_soid
    }
}

impl ::protobuf::Message for CMsgSOCacheUnsubscribed {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.owner_soid)?;
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
        if let Some(v) = self.owner_soid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_soid.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSOCacheUnsubscribed {
    fn new() -> CMsgSOCacheUnsubscribed {
        CMsgSOCacheUnsubscribed::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOCacheUnsubscribed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOIDOwner>>(
                    "owner_soid",
                    CMsgSOCacheUnsubscribed::get_owner_soid_for_reflect,
                    CMsgSOCacheUnsubscribed::mut_owner_soid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOCacheUnsubscribed>(
                    "CMsgSOCacheUnsubscribed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOCacheUnsubscribed {
    fn clear(&mut self) {
        self.clear_owner_soid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOCacheUnsubscribed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOCacheUnsubscribed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOCacheSubscriptionCheck {
    // message fields
    version: ::std::option::Option<u64>,
    owner_soid: ::protobuf::SingularPtrField<CMsgSOIDOwner>,
    service_id: ::std::option::Option<u32>,
    service_list: ::std::vec::Vec<u32>,
    sync_version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOCacheSubscriptionCheck {}

impl CMsgSOCacheSubscriptionCheck {
    pub fn new() -> CMsgSOCacheSubscriptionCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOCacheSubscriptionCheck {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOCacheSubscriptionCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOCacheSubscriptionCheck,
        };
        unsafe {
            instance.get(CMsgSOCacheSubscriptionCheck::new)
        }
    }

    // optional fixed64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }

    // optional .dota.CMsgSOIDOwner owner_soid = 3;

    pub fn clear_owner_soid(&mut self) {
        self.owner_soid.clear();
    }

    pub fn has_owner_soid(&self) -> bool {
        self.owner_soid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_soid(&mut self, v: CMsgSOIDOwner) {
        self.owner_soid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_soid(&mut self) -> &mut CMsgSOIDOwner {
        if self.owner_soid.is_none() {
            self.owner_soid.set_default();
        };
        self.owner_soid.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_soid(&mut self) -> CMsgSOIDOwner {
        self.owner_soid.take().unwrap_or_else(|| CMsgSOIDOwner::new())
    }

    pub fn get_owner_soid(&self) -> &CMsgSOIDOwner {
        self.owner_soid.as_ref().unwrap_or_else(|| CMsgSOIDOwner::default_instance())
    }

    fn get_owner_soid_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &self.owner_soid
    }

    fn mut_owner_soid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &mut self.owner_soid
    }

    // optional uint32 service_id = 4;

    pub fn clear_service_id(&mut self) {
        self.service_id = ::std::option::Option::None;
    }

    pub fn has_service_id(&self) -> bool {
        self.service_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: u32) {
        self.service_id = ::std::option::Option::Some(v);
    }

    pub fn get_service_id(&self) -> u32 {
        self.service_id.unwrap_or(0)
    }

    fn get_service_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service_id
    }

    fn mut_service_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service_id
    }

    // repeated uint32 service_list = 5;

    pub fn clear_service_list(&mut self) {
        self.service_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_service_list(&mut self, v: ::std::vec::Vec<u32>) {
        self.service_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_service_list(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.service_list
    }

    // Take field
    pub fn take_service_list(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.service_list, ::std::vec::Vec::new())
    }

    pub fn get_service_list(&self) -> &[u32] {
        &self.service_list
    }

    fn get_service_list_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.service_list
    }

    fn mut_service_list_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.service_list
    }

    // optional fixed64 sync_version = 6;

    pub fn clear_sync_version(&mut self) {
        self.sync_version = ::std::option::Option::None;
    }

    pub fn has_sync_version(&self) -> bool {
        self.sync_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sync_version(&mut self, v: u64) {
        self.sync_version = ::std::option::Option::Some(v);
    }

    pub fn get_sync_version(&self) -> u64 {
        self.sync_version.unwrap_or(0)
    }

    fn get_sync_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sync_version
    }

    fn mut_sync_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sync_version
    }
}

impl ::protobuf::Message for CMsgSOCacheSubscriptionCheck {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.owner_soid)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.service_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.service_list)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.sync_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.version {
            my_size += 9;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.service_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.service_list {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sync_version {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.owner_soid.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.service_id {
            os.write_uint32(4, v)?;
        };
        for v in &self.service_list {
            os.write_uint32(5, *v)?;
        };
        if let Some(v) = self.sync_version {
            os.write_fixed64(6, v)?;
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

impl ::protobuf::MessageStatic for CMsgSOCacheSubscriptionCheck {
    fn new() -> CMsgSOCacheSubscriptionCheck {
        CMsgSOCacheSubscriptionCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOCacheSubscriptionCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "version",
                    CMsgSOCacheSubscriptionCheck::get_version_for_reflect,
                    CMsgSOCacheSubscriptionCheck::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOIDOwner>>(
                    "owner_soid",
                    CMsgSOCacheSubscriptionCheck::get_owner_soid_for_reflect,
                    CMsgSOCacheSubscriptionCheck::mut_owner_soid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_id",
                    CMsgSOCacheSubscriptionCheck::get_service_id_for_reflect,
                    CMsgSOCacheSubscriptionCheck::mut_service_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_list",
                    CMsgSOCacheSubscriptionCheck::get_service_list_for_reflect,
                    CMsgSOCacheSubscriptionCheck::mut_service_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "sync_version",
                    CMsgSOCacheSubscriptionCheck::get_sync_version_for_reflect,
                    CMsgSOCacheSubscriptionCheck::mut_sync_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOCacheSubscriptionCheck>(
                    "CMsgSOCacheSubscriptionCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOCacheSubscriptionCheck {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_owner_soid();
        self.clear_service_id();
        self.clear_service_list();
        self.clear_sync_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOCacheSubscriptionCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOCacheSubscriptionCheck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOCacheSubscriptionRefresh {
    // message fields
    owner_soid: ::protobuf::SingularPtrField<CMsgSOIDOwner>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOCacheSubscriptionRefresh {}

impl CMsgSOCacheSubscriptionRefresh {
    pub fn new() -> CMsgSOCacheSubscriptionRefresh {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOCacheSubscriptionRefresh {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOCacheSubscriptionRefresh> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOCacheSubscriptionRefresh,
        };
        unsafe {
            instance.get(CMsgSOCacheSubscriptionRefresh::new)
        }
    }

    // optional .dota.CMsgSOIDOwner owner_soid = 2;

    pub fn clear_owner_soid(&mut self) {
        self.owner_soid.clear();
    }

    pub fn has_owner_soid(&self) -> bool {
        self.owner_soid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_soid(&mut self, v: CMsgSOIDOwner) {
        self.owner_soid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner_soid(&mut self) -> &mut CMsgSOIDOwner {
        if self.owner_soid.is_none() {
            self.owner_soid.set_default();
        };
        self.owner_soid.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner_soid(&mut self) -> CMsgSOIDOwner {
        self.owner_soid.take().unwrap_or_else(|| CMsgSOIDOwner::new())
    }

    pub fn get_owner_soid(&self) -> &CMsgSOIDOwner {
        self.owner_soid.as_ref().unwrap_or_else(|| CMsgSOIDOwner::default_instance())
    }

    fn get_owner_soid_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &self.owner_soid
    }

    fn mut_owner_soid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &mut self.owner_soid
    }
}

impl ::protobuf::Message for CMsgSOCacheSubscriptionRefresh {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.owner_soid)?;
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
        if let Some(v) = self.owner_soid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_soid.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSOCacheSubscriptionRefresh {
    fn new() -> CMsgSOCacheSubscriptionRefresh {
        CMsgSOCacheSubscriptionRefresh::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOCacheSubscriptionRefresh>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOIDOwner>>(
                    "owner_soid",
                    CMsgSOCacheSubscriptionRefresh::get_owner_soid_for_reflect,
                    CMsgSOCacheSubscriptionRefresh::mut_owner_soid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOCacheSubscriptionRefresh>(
                    "CMsgSOCacheSubscriptionRefresh",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOCacheSubscriptionRefresh {
    fn clear(&mut self) {
        self.clear_owner_soid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOCacheSubscriptionRefresh {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOCacheSubscriptionRefresh {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOCacheVersion {
    // message fields
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOCacheVersion {}

impl CMsgSOCacheVersion {
    pub fn new() -> CMsgSOCacheVersion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOCacheVersion {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOCacheVersion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOCacheVersion,
        };
        unsafe {
            instance.get(CMsgSOCacheVersion::new)
        }
    }

    // optional fixed64 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for CMsgSOCacheVersion {
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
        if let Some(v) = self.version {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_fixed64(1, v)?;
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

impl ::protobuf::MessageStatic for CMsgSOCacheVersion {
    fn new() -> CMsgSOCacheVersion {
        CMsgSOCacheVersion::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOCacheVersion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "version",
                    CMsgSOCacheVersion::get_version_for_reflect,
                    CMsgSOCacheVersion::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOCacheVersion>(
                    "CMsgSOCacheVersion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOCacheVersion {
    fn clear(&mut self) {
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOCacheVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOCacheVersion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMultiplexMessage {
    // message fields
    msgtype: ::std::option::Option<u32>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    steamids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMultiplexMessage {}

impl CMsgGCMultiplexMessage {
    pub fn new() -> CMsgGCMultiplexMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMultiplexMessage {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMultiplexMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMultiplexMessage,
        };
        unsafe {
            instance.get(CMsgGCMultiplexMessage::new)
        }
    }

    // optional uint32 msgtype = 1;

    pub fn clear_msgtype(&mut self) {
        self.msgtype = ::std::option::Option::None;
    }

    pub fn has_msgtype(&self) -> bool {
        self.msgtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msgtype(&mut self, v: u32) {
        self.msgtype = ::std::option::Option::Some(v);
    }

    pub fn get_msgtype(&self) -> u32 {
        self.msgtype.unwrap_or(0)
    }

    fn get_msgtype_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.msgtype
    }

    fn mut_msgtype_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.msgtype
    }

    // optional bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.payload
    }

    // repeated fixed64 steamids = 3;

    pub fn clear_steamids(&mut self) {
        self.steamids.clear();
    }

    // Param is passed by value, moved
    pub fn set_steamids(&mut self, v: ::std::vec::Vec<u64>) {
        self.steamids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_steamids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.steamids
    }

    // Take field
    pub fn take_steamids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.steamids, ::std::vec::Vec::new())
    }

    pub fn get_steamids(&self) -> &[u64] {
        &self.steamids
    }

    fn get_steamids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.steamids
    }

    fn mut_steamids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.steamids
    }
}

impl ::protobuf::Message for CMsgGCMultiplexMessage {
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
                    self.msgtype = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.steamids)?;
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
        if let Some(v) = self.msgtype {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += 9 * self.steamids.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msgtype {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.payload.as_ref() {
            os.write_bytes(2, &v)?;
        };
        for v in &self.steamids {
            os.write_fixed64(3, *v)?;
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

impl ::protobuf::MessageStatic for CMsgGCMultiplexMessage {
    fn new() -> CMsgGCMultiplexMessage {
        CMsgGCMultiplexMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMultiplexMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "msgtype",
                    CMsgGCMultiplexMessage::get_msgtype_for_reflect,
                    CMsgGCMultiplexMessage::mut_msgtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    CMsgGCMultiplexMessage::get_payload_for_reflect,
                    CMsgGCMultiplexMessage::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamids",
                    CMsgGCMultiplexMessage::get_steamids_for_reflect,
                    CMsgGCMultiplexMessage::mut_steamids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMultiplexMessage>(
                    "CMsgGCMultiplexMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMultiplexMessage {
    fn clear(&mut self) {
        self.clear_msgtype();
        self.clear_payload();
        self.clear_steamids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMultiplexMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMultiplexMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCToGCMsgMasterAck {
    // message fields
    dir_index: ::std::option::Option<u32>,
    machine_name: ::protobuf::SingularField<::std::string::String>,
    process_name: ::protobuf::SingularField<::std::string::String>,
    directory: ::protobuf::RepeatedField<CGCToGCMsgMasterAck_Process>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCToGCMsgMasterAck {}

impl CGCToGCMsgMasterAck {
    pub fn new() -> CGCToGCMsgMasterAck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCToGCMsgMasterAck {
        static mut instance: ::protobuf::lazy::Lazy<CGCToGCMsgMasterAck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCToGCMsgMasterAck,
        };
        unsafe {
            instance.get(CGCToGCMsgMasterAck::new)
        }
    }

    // optional uint32 dir_index = 1;

    pub fn clear_dir_index(&mut self) {
        self.dir_index = ::std::option::Option::None;
    }

    pub fn has_dir_index(&self) -> bool {
        self.dir_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir_index(&mut self, v: u32) {
        self.dir_index = ::std::option::Option::Some(v);
    }

    pub fn get_dir_index(&self) -> u32 {
        self.dir_index.unwrap_or(0)
    }

    fn get_dir_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dir_index
    }

    fn mut_dir_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dir_index
    }

    // optional string machine_name = 3;

    pub fn clear_machine_name(&mut self) {
        self.machine_name.clear();
    }

    pub fn has_machine_name(&self) -> bool {
        self.machine_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_machine_name(&mut self, v: ::std::string::String) {
        self.machine_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_machine_name(&mut self) -> &mut ::std::string::String {
        if self.machine_name.is_none() {
            self.machine_name.set_default();
        };
        self.machine_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_machine_name(&mut self) -> ::std::string::String {
        self.machine_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_machine_name(&self) -> &str {
        match self.machine_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_machine_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.machine_name
    }

    fn mut_machine_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.machine_name
    }

    // optional string process_name = 4;

    pub fn clear_process_name(&mut self) {
        self.process_name.clear();
    }

    pub fn has_process_name(&self) -> bool {
        self.process_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_process_name(&mut self, v: ::std::string::String) {
        self.process_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_process_name(&mut self) -> &mut ::std::string::String {
        if self.process_name.is_none() {
            self.process_name.set_default();
        };
        self.process_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_process_name(&mut self) -> ::std::string::String {
        self.process_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_process_name(&self) -> &str {
        match self.process_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_process_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.process_name
    }

    fn mut_process_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.process_name
    }

    // repeated .dota.CGCToGCMsgMasterAck.Process directory = 6;

    pub fn clear_directory(&mut self) {
        self.directory.clear();
    }

    // Param is passed by value, moved
    pub fn set_directory(&mut self, v: ::protobuf::RepeatedField<CGCToGCMsgMasterAck_Process>) {
        self.directory = v;
    }

    // Mutable pointer to the field.
    pub fn mut_directory(&mut self) -> &mut ::protobuf::RepeatedField<CGCToGCMsgMasterAck_Process> {
        &mut self.directory
    }

    // Take field
    pub fn take_directory(&mut self) -> ::protobuf::RepeatedField<CGCToGCMsgMasterAck_Process> {
        ::std::mem::replace(&mut self.directory, ::protobuf::RepeatedField::new())
    }

    pub fn get_directory(&self) -> &[CGCToGCMsgMasterAck_Process] {
        &self.directory
    }

    fn get_directory_for_reflect(&self) -> &::protobuf::RepeatedField<CGCToGCMsgMasterAck_Process> {
        &self.directory
    }

    fn mut_directory_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CGCToGCMsgMasterAck_Process> {
        &mut self.directory
    }
}

impl ::protobuf::Message for CGCToGCMsgMasterAck {
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
                    self.dir_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.machine_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.process_name)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.directory)?;
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
        if let Some(v) = self.dir_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.machine_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.process_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        for value in &self.directory {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dir_index {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.machine_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.process_name.as_ref() {
            os.write_string(4, &v)?;
        };
        for v in &self.directory {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CGCToGCMsgMasterAck {
    fn new() -> CGCToGCMsgMasterAck {
        CGCToGCMsgMasterAck::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCToGCMsgMasterAck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dir_index",
                    CGCToGCMsgMasterAck::get_dir_index_for_reflect,
                    CGCToGCMsgMasterAck::mut_dir_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "machine_name",
                    CGCToGCMsgMasterAck::get_machine_name_for_reflect,
                    CGCToGCMsgMasterAck::mut_machine_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "process_name",
                    CGCToGCMsgMasterAck::get_process_name_for_reflect,
                    CGCToGCMsgMasterAck::mut_process_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGCToGCMsgMasterAck_Process>>(
                    "directory",
                    CGCToGCMsgMasterAck::get_directory_for_reflect,
                    CGCToGCMsgMasterAck::mut_directory_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCToGCMsgMasterAck>(
                    "CGCToGCMsgMasterAck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCToGCMsgMasterAck {
    fn clear(&mut self) {
        self.clear_dir_index();
        self.clear_machine_name();
        self.clear_process_name();
        self.clear_directory();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCToGCMsgMasterAck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCToGCMsgMasterAck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCToGCMsgMasterAck_Process {
    // message fields
    dir_index: ::std::option::Option<u32>,
    type_instances: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCToGCMsgMasterAck_Process {}

impl CGCToGCMsgMasterAck_Process {
    pub fn new() -> CGCToGCMsgMasterAck_Process {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCToGCMsgMasterAck_Process {
        static mut instance: ::protobuf::lazy::Lazy<CGCToGCMsgMasterAck_Process> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCToGCMsgMasterAck_Process,
        };
        unsafe {
            instance.get(CGCToGCMsgMasterAck_Process::new)
        }
    }

    // optional uint32 dir_index = 1;

    pub fn clear_dir_index(&mut self) {
        self.dir_index = ::std::option::Option::None;
    }

    pub fn has_dir_index(&self) -> bool {
        self.dir_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir_index(&mut self, v: u32) {
        self.dir_index = ::std::option::Option::Some(v);
    }

    pub fn get_dir_index(&self) -> u32 {
        self.dir_index.unwrap_or(0)
    }

    fn get_dir_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dir_index
    }

    fn mut_dir_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dir_index
    }

    // repeated uint32 type_instances = 2;

    pub fn clear_type_instances(&mut self) {
        self.type_instances.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_instances(&mut self, v: ::std::vec::Vec<u32>) {
        self.type_instances = v;
    }

    // Mutable pointer to the field.
    pub fn mut_type_instances(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.type_instances
    }

    // Take field
    pub fn take_type_instances(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.type_instances, ::std::vec::Vec::new())
    }

    pub fn get_type_instances(&self) -> &[u32] {
        &self.type_instances
    }

    fn get_type_instances_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.type_instances
    }

    fn mut_type_instances_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.type_instances
    }
}

impl ::protobuf::Message for CGCToGCMsgMasterAck_Process {
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
                    self.dir_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.type_instances)?;
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
        if let Some(v) = self.dir_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.type_instances {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dir_index {
            os.write_uint32(1, v)?;
        };
        for v in &self.type_instances {
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

impl ::protobuf::MessageStatic for CGCToGCMsgMasterAck_Process {
    fn new() -> CGCToGCMsgMasterAck_Process {
        CGCToGCMsgMasterAck_Process::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCToGCMsgMasterAck_Process>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dir_index",
                    CGCToGCMsgMasterAck_Process::get_dir_index_for_reflect,
                    CGCToGCMsgMasterAck_Process::mut_dir_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "type_instances",
                    CGCToGCMsgMasterAck_Process::get_type_instances_for_reflect,
                    CGCToGCMsgMasterAck_Process::mut_type_instances_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCToGCMsgMasterAck_Process>(
                    "CGCToGCMsgMasterAck_Process",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCToGCMsgMasterAck_Process {
    fn clear(&mut self) {
        self.clear_dir_index();
        self.clear_type_instances();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCToGCMsgMasterAck_Process {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCToGCMsgMasterAck_Process {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCToGCMsgMasterAck_Response {
    // message fields
    eresult: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCToGCMsgMasterAck_Response {}

impl CGCToGCMsgMasterAck_Response {
    pub fn new() -> CGCToGCMsgMasterAck_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCToGCMsgMasterAck_Response {
        static mut instance: ::protobuf::lazy::Lazy<CGCToGCMsgMasterAck_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCToGCMsgMasterAck_Response,
        };
        unsafe {
            instance.get(CGCToGCMsgMasterAck_Response::new)
        }
    }

    // optional int32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }
}

impl ::protobuf::Message for CGCToGCMsgMasterAck_Response {
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
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
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

impl ::protobuf::MessageStatic for CGCToGCMsgMasterAck_Response {
    fn new() -> CGCToGCMsgMasterAck_Response {
        CGCToGCMsgMasterAck_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCToGCMsgMasterAck_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CGCToGCMsgMasterAck_Response::get_eresult_for_reflect,
                    CGCToGCMsgMasterAck_Response::mut_eresult_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCToGCMsgMasterAck_Response>(
                    "CGCToGCMsgMasterAck_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCToGCMsgMasterAck_Response {
    fn clear(&mut self) {
        self.clear_eresult();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCToGCMsgMasterAck_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCToGCMsgMasterAck_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCUniverseStartup {
    // message fields
    is_initial_startup: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCUniverseStartup {}

impl CMsgGCToGCUniverseStartup {
    pub fn new() -> CMsgGCToGCUniverseStartup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCUniverseStartup {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCUniverseStartup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCUniverseStartup,
        };
        unsafe {
            instance.get(CMsgGCToGCUniverseStartup::new)
        }
    }

    // optional bool is_initial_startup = 1;

    pub fn clear_is_initial_startup(&mut self) {
        self.is_initial_startup = ::std::option::Option::None;
    }

    pub fn has_is_initial_startup(&self) -> bool {
        self.is_initial_startup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_initial_startup(&mut self, v: bool) {
        self.is_initial_startup = ::std::option::Option::Some(v);
    }

    pub fn get_is_initial_startup(&self) -> bool {
        self.is_initial_startup.unwrap_or(false)
    }

    fn get_is_initial_startup_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_initial_startup
    }

    fn mut_is_initial_startup_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_initial_startup
    }
}

impl ::protobuf::Message for CMsgGCToGCUniverseStartup {
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
                    self.is_initial_startup = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.is_initial_startup {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_initial_startup {
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

impl ::protobuf::MessageStatic for CMsgGCToGCUniverseStartup {
    fn new() -> CMsgGCToGCUniverseStartup {
        CMsgGCToGCUniverseStartup::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCUniverseStartup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_initial_startup",
                    CMsgGCToGCUniverseStartup::get_is_initial_startup_for_reflect,
                    CMsgGCToGCUniverseStartup::mut_is_initial_startup_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCUniverseStartup>(
                    "CMsgGCToGCUniverseStartup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCUniverseStartup {
    fn clear(&mut self) {
        self.clear_is_initial_startup();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCUniverseStartup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCUniverseStartup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCUniverseStartupResponse {
    // message fields
    eresult: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCUniverseStartupResponse {}

impl CMsgGCToGCUniverseStartupResponse {
    pub fn new() -> CMsgGCToGCUniverseStartupResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCUniverseStartupResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCUniverseStartupResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCUniverseStartupResponse,
        };
        unsafe {
            instance.get(CMsgGCToGCUniverseStartupResponse::new)
        }
    }

    // optional int32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(0)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }
}

impl ::protobuf::Message for CMsgGCToGCUniverseStartupResponse {
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
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
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

impl ::protobuf::MessageStatic for CMsgGCToGCUniverseStartupResponse {
    fn new() -> CMsgGCToGCUniverseStartupResponse {
        CMsgGCToGCUniverseStartupResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCUniverseStartupResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgGCToGCUniverseStartupResponse::get_eresult_for_reflect,
                    CMsgGCToGCUniverseStartupResponse::mut_eresult_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCUniverseStartupResponse>(
                    "CMsgGCToGCUniverseStartupResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCUniverseStartupResponse {
    fn clear(&mut self) {
        self.clear_eresult();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCUniverseStartupResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCUniverseStartupResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCToGCMsgMasterStartupComplete {
    // message fields
    gc_info: ::protobuf::RepeatedField<CGCToGCMsgMasterStartupComplete_GCInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCToGCMsgMasterStartupComplete {}

impl CGCToGCMsgMasterStartupComplete {
    pub fn new() -> CGCToGCMsgMasterStartupComplete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCToGCMsgMasterStartupComplete {
        static mut instance: ::protobuf::lazy::Lazy<CGCToGCMsgMasterStartupComplete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCToGCMsgMasterStartupComplete,
        };
        unsafe {
            instance.get(CGCToGCMsgMasterStartupComplete::new)
        }
    }

    // repeated .dota.CGCToGCMsgMasterStartupComplete.GCInfo gc_info = 1;

    pub fn clear_gc_info(&mut self) {
        self.gc_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_gc_info(&mut self, v: ::protobuf::RepeatedField<CGCToGCMsgMasterStartupComplete_GCInfo>) {
        self.gc_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_gc_info(&mut self) -> &mut ::protobuf::RepeatedField<CGCToGCMsgMasterStartupComplete_GCInfo> {
        &mut self.gc_info
    }

    // Take field
    pub fn take_gc_info(&mut self) -> ::protobuf::RepeatedField<CGCToGCMsgMasterStartupComplete_GCInfo> {
        ::std::mem::replace(&mut self.gc_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_gc_info(&self) -> &[CGCToGCMsgMasterStartupComplete_GCInfo] {
        &self.gc_info
    }

    fn get_gc_info_for_reflect(&self) -> &::protobuf::RepeatedField<CGCToGCMsgMasterStartupComplete_GCInfo> {
        &self.gc_info
    }

    fn mut_gc_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CGCToGCMsgMasterStartupComplete_GCInfo> {
        &mut self.gc_info
    }
}

impl ::protobuf::Message for CGCToGCMsgMasterStartupComplete {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.gc_info)?;
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
        for value in &self.gc_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.gc_info {
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

impl ::protobuf::MessageStatic for CGCToGCMsgMasterStartupComplete {
    fn new() -> CGCToGCMsgMasterStartupComplete {
        CGCToGCMsgMasterStartupComplete::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCToGCMsgMasterStartupComplete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGCToGCMsgMasterStartupComplete_GCInfo>>(
                    "gc_info",
                    CGCToGCMsgMasterStartupComplete::get_gc_info_for_reflect,
                    CGCToGCMsgMasterStartupComplete::mut_gc_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCToGCMsgMasterStartupComplete>(
                    "CGCToGCMsgMasterStartupComplete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCToGCMsgMasterStartupComplete {
    fn clear(&mut self) {
        self.clear_gc_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCToGCMsgMasterStartupComplete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCToGCMsgMasterStartupComplete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCToGCMsgMasterStartupComplete_GCInfo {
    // message fields
    dir_index: ::std::option::Option<u32>,
    machine_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCToGCMsgMasterStartupComplete_GCInfo {}

impl CGCToGCMsgMasterStartupComplete_GCInfo {
    pub fn new() -> CGCToGCMsgMasterStartupComplete_GCInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCToGCMsgMasterStartupComplete_GCInfo {
        static mut instance: ::protobuf::lazy::Lazy<CGCToGCMsgMasterStartupComplete_GCInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCToGCMsgMasterStartupComplete_GCInfo,
        };
        unsafe {
            instance.get(CGCToGCMsgMasterStartupComplete_GCInfo::new)
        }
    }

    // optional uint32 dir_index = 1;

    pub fn clear_dir_index(&mut self) {
        self.dir_index = ::std::option::Option::None;
    }

    pub fn has_dir_index(&self) -> bool {
        self.dir_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir_index(&mut self, v: u32) {
        self.dir_index = ::std::option::Option::Some(v);
    }

    pub fn get_dir_index(&self) -> u32 {
        self.dir_index.unwrap_or(0)
    }

    fn get_dir_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dir_index
    }

    fn mut_dir_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dir_index
    }

    // optional string machine_name = 2;

    pub fn clear_machine_name(&mut self) {
        self.machine_name.clear();
    }

    pub fn has_machine_name(&self) -> bool {
        self.machine_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_machine_name(&mut self, v: ::std::string::String) {
        self.machine_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_machine_name(&mut self) -> &mut ::std::string::String {
        if self.machine_name.is_none() {
            self.machine_name.set_default();
        };
        self.machine_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_machine_name(&mut self) -> ::std::string::String {
        self.machine_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_machine_name(&self) -> &str {
        match self.machine_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_machine_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.machine_name
    }

    fn mut_machine_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.machine_name
    }
}

impl ::protobuf::Message for CGCToGCMsgMasterStartupComplete_GCInfo {
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
                    self.dir_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.machine_name)?;
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
        if let Some(v) = self.dir_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.machine_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dir_index {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.machine_name.as_ref() {
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

impl ::protobuf::MessageStatic for CGCToGCMsgMasterStartupComplete_GCInfo {
    fn new() -> CGCToGCMsgMasterStartupComplete_GCInfo {
        CGCToGCMsgMasterStartupComplete_GCInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCToGCMsgMasterStartupComplete_GCInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dir_index",
                    CGCToGCMsgMasterStartupComplete_GCInfo::get_dir_index_for_reflect,
                    CGCToGCMsgMasterStartupComplete_GCInfo::mut_dir_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "machine_name",
                    CGCToGCMsgMasterStartupComplete_GCInfo::get_machine_name_for_reflect,
                    CGCToGCMsgMasterStartupComplete_GCInfo::mut_machine_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCToGCMsgMasterStartupComplete_GCInfo>(
                    "CGCToGCMsgMasterStartupComplete_GCInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCToGCMsgMasterStartupComplete_GCInfo {
    fn clear(&mut self) {
        self.clear_dir_index();
        self.clear_machine_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCToGCMsgMasterStartupComplete_GCInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCToGCMsgMasterStartupComplete_GCInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCToGCMsgRouted {
    // message fields
    msg_type: ::std::option::Option<u32>,
    sender_id: ::std::option::Option<u64>,
    net_message: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCToGCMsgRouted {}

impl CGCToGCMsgRouted {
    pub fn new() -> CGCToGCMsgRouted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCToGCMsgRouted {
        static mut instance: ::protobuf::lazy::Lazy<CGCToGCMsgRouted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCToGCMsgRouted,
        };
        unsafe {
            instance.get(CGCToGCMsgRouted::new)
        }
    }

    // optional uint32 msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: u32) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> u32 {
        self.msg_type.unwrap_or(0)
    }

    fn get_msg_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.msg_type
    }

    fn mut_msg_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.msg_type
    }

    // optional fixed64 sender_id = 2;

    pub fn clear_sender_id(&mut self) {
        self.sender_id = ::std::option::Option::None;
    }

    pub fn has_sender_id(&self) -> bool {
        self.sender_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender_id(&mut self, v: u64) {
        self.sender_id = ::std::option::Option::Some(v);
    }

    pub fn get_sender_id(&self) -> u64 {
        self.sender_id.unwrap_or(0)
    }

    fn get_sender_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sender_id
    }

    fn mut_sender_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sender_id
    }

    // optional bytes net_message = 3;

    pub fn clear_net_message(&mut self) {
        self.net_message.clear();
    }

    pub fn has_net_message(&self) -> bool {
        self.net_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_message(&mut self, v: ::std::vec::Vec<u8>) {
        self.net_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_net_message(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.net_message.is_none() {
            self.net_message.set_default();
        };
        self.net_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_net_message(&mut self) -> ::std::vec::Vec<u8> {
        self.net_message.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_net_message(&self) -> &[u8] {
        match self.net_message.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_net_message_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.net_message
    }

    fn mut_net_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.net_message
    }
}

impl ::protobuf::Message for CGCToGCMsgRouted {
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
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.sender_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.net_message)?;
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
        if let Some(v) = self.sender_id {
            my_size += 9;
        };
        if let Some(v) = self.net_message.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_type {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.sender_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.net_message.as_ref() {
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

impl ::protobuf::MessageStatic for CGCToGCMsgRouted {
    fn new() -> CGCToGCMsgRouted {
        CGCToGCMsgRouted::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCToGCMsgRouted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "msg_type",
                    CGCToGCMsgRouted::get_msg_type_for_reflect,
                    CGCToGCMsgRouted::mut_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "sender_id",
                    CGCToGCMsgRouted::get_sender_id_for_reflect,
                    CGCToGCMsgRouted::mut_sender_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "net_message",
                    CGCToGCMsgRouted::get_net_message_for_reflect,
                    CGCToGCMsgRouted::mut_net_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCToGCMsgRouted>(
                    "CGCToGCMsgRouted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCToGCMsgRouted {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_sender_id();
        self.clear_net_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCToGCMsgRouted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCToGCMsgRouted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCToGCMsgRoutedReply {
    // message fields
    msg_type: ::std::option::Option<u32>,
    net_message: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCToGCMsgRoutedReply {}

impl CGCToGCMsgRoutedReply {
    pub fn new() -> CGCToGCMsgRoutedReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCToGCMsgRoutedReply {
        static mut instance: ::protobuf::lazy::Lazy<CGCToGCMsgRoutedReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCToGCMsgRoutedReply,
        };
        unsafe {
            instance.get(CGCToGCMsgRoutedReply::new)
        }
    }

    // optional uint32 msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: u32) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> u32 {
        self.msg_type.unwrap_or(0)
    }

    fn get_msg_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.msg_type
    }

    fn mut_msg_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.msg_type
    }

    // optional bytes net_message = 2;

    pub fn clear_net_message(&mut self) {
        self.net_message.clear();
    }

    pub fn has_net_message(&self) -> bool {
        self.net_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_message(&mut self, v: ::std::vec::Vec<u8>) {
        self.net_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_net_message(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.net_message.is_none() {
            self.net_message.set_default();
        };
        self.net_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_net_message(&mut self) -> ::std::vec::Vec<u8> {
        self.net_message.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_net_message(&self) -> &[u8] {
        match self.net_message.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_net_message_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.net_message
    }

    fn mut_net_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.net_message
    }
}

impl ::protobuf::Message for CGCToGCMsgRoutedReply {
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
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.net_message)?;
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
        if let Some(v) = self.net_message.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_type {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.net_message.as_ref() {
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

impl ::protobuf::MessageStatic for CGCToGCMsgRoutedReply {
    fn new() -> CGCToGCMsgRoutedReply {
        CGCToGCMsgRoutedReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCToGCMsgRoutedReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "msg_type",
                    CGCToGCMsgRoutedReply::get_msg_type_for_reflect,
                    CGCToGCMsgRoutedReply::mut_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "net_message",
                    CGCToGCMsgRoutedReply::get_net_message_for_reflect,
                    CGCToGCMsgRoutedReply::mut_net_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCToGCMsgRoutedReply>(
                    "CGCToGCMsgRoutedReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCToGCMsgRoutedReply {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_net_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCToGCMsgRoutedReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCToGCMsgRoutedReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCUpdateSubGCSessionInfo {
    // message fields
    updates: ::protobuf::RepeatedField<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCUpdateSubGCSessionInfo {}

impl CMsgGCUpdateSubGCSessionInfo {
    pub fn new() -> CMsgGCUpdateSubGCSessionInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCUpdateSubGCSessionInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCUpdateSubGCSessionInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCUpdateSubGCSessionInfo,
        };
        unsafe {
            instance.get(CMsgGCUpdateSubGCSessionInfo::new)
        }
    }

    // repeated .dota.CMsgGCUpdateSubGCSessionInfo.CMsgUpdate updates = 1;

    pub fn clear_updates(&mut self) {
        self.updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_updates(&mut self, v: ::protobuf::RepeatedField<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate>) {
        self.updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updates(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate> {
        &mut self.updates
    }

    // Take field
    pub fn take_updates(&mut self) -> ::protobuf::RepeatedField<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate> {
        ::std::mem::replace(&mut self.updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_updates(&self) -> &[CMsgGCUpdateSubGCSessionInfo_CMsgUpdate] {
        &self.updates
    }

    fn get_updates_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate> {
        &self.updates
    }

    fn mut_updates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate> {
        &mut self.updates
    }
}

impl ::protobuf::Message for CMsgGCUpdateSubGCSessionInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.updates)?;
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
        for value in &self.updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.updates {
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

impl ::protobuf::MessageStatic for CMsgGCUpdateSubGCSessionInfo {
    fn new() -> CMsgGCUpdateSubGCSessionInfo {
        CMsgGCUpdateSubGCSessionInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCUpdateSubGCSessionInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate>>(
                    "updates",
                    CMsgGCUpdateSubGCSessionInfo::get_updates_for_reflect,
                    CMsgGCUpdateSubGCSessionInfo::mut_updates_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCUpdateSubGCSessionInfo>(
                    "CMsgGCUpdateSubGCSessionInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCUpdateSubGCSessionInfo {
    fn clear(&mut self) {
        self.clear_updates();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCUpdateSubGCSessionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCUpdateSubGCSessionInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
    // message fields
    steamid: ::std::option::Option<u64>,
    ip: ::std::option::Option<u32>,
    trusted: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {}

impl CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
    pub fn new() -> CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCUpdateSubGCSessionInfo_CMsgUpdate,
        };
        unsafe {
            instance.get(CMsgGCUpdateSubGCSessionInfo_CMsgUpdate::new)
        }
    }

    // optional fixed64 steamid = 1;

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

    // optional fixed32 ip = 2;

    pub fn clear_ip(&mut self) {
        self.ip = ::std::option::Option::None;
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: u32) {
        self.ip = ::std::option::Option::Some(v);
    }

    pub fn get_ip(&self) -> u32 {
        self.ip.unwrap_or(0)
    }

    fn get_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ip
    }

    // optional bool trusted = 3;

    pub fn clear_trusted(&mut self) {
        self.trusted = ::std::option::Option::None;
    }

    pub fn has_trusted(&self) -> bool {
        self.trusted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trusted(&mut self, v: bool) {
        self.trusted = ::std::option::Option::Some(v);
    }

    pub fn get_trusted(&self) -> bool {
        self.trusted.unwrap_or(false)
    }

    fn get_trusted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.trusted
    }

    fn mut_trusted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.trusted
    }
}

impl ::protobuf::Message for CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
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
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.ip = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.trusted = ::std::option::Option::Some(tmp);
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
            my_size += 9;
        };
        if let Some(v) = self.ip {
            my_size += 5;
        };
        if let Some(v) = self.trusted {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.ip {
            os.write_fixed32(2, v)?;
        };
        if let Some(v) = self.trusted {
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

impl ::protobuf::MessageStatic for CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
    fn new() -> CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
        CMsgGCUpdateSubGCSessionInfo_CMsgUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgGCUpdateSubGCSessionInfo_CMsgUpdate::get_steamid_for_reflect,
                    CMsgGCUpdateSubGCSessionInfo_CMsgUpdate::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ip",
                    CMsgGCUpdateSubGCSessionInfo_CMsgUpdate::get_ip_for_reflect,
                    CMsgGCUpdateSubGCSessionInfo_CMsgUpdate::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "trusted",
                    CMsgGCUpdateSubGCSessionInfo_CMsgUpdate::get_trusted_for_reflect,
                    CMsgGCUpdateSubGCSessionInfo_CMsgUpdate::mut_trusted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCUpdateSubGCSessionInfo_CMsgUpdate>(
                    "CMsgGCUpdateSubGCSessionInfo_CMsgUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_ip();
        self.clear_trusted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCUpdateSubGCSessionInfo_CMsgUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCRequestSubGCSessionInfo {
    // message fields
    steamid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCRequestSubGCSessionInfo {}

impl CMsgGCRequestSubGCSessionInfo {
    pub fn new() -> CMsgGCRequestSubGCSessionInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCRequestSubGCSessionInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCRequestSubGCSessionInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCRequestSubGCSessionInfo,
        };
        unsafe {
            instance.get(CMsgGCRequestSubGCSessionInfo::new)
        }
    }

    // optional fixed64 steamid = 1;

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
}

impl ::protobuf::Message for CMsgGCRequestSubGCSessionInfo {
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
                    self.steamid = ::std::option::Option::Some(tmp);
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
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
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

impl ::protobuf::MessageStatic for CMsgGCRequestSubGCSessionInfo {
    fn new() -> CMsgGCRequestSubGCSessionInfo {
        CMsgGCRequestSubGCSessionInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCRequestSubGCSessionInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgGCRequestSubGCSessionInfo::get_steamid_for_reflect,
                    CMsgGCRequestSubGCSessionInfo::mut_steamid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCRequestSubGCSessionInfo>(
                    "CMsgGCRequestSubGCSessionInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCRequestSubGCSessionInfo {
    fn clear(&mut self) {
        self.clear_steamid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCRequestSubGCSessionInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCRequestSubGCSessionInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCRequestSubGCSessionInfoResponse {
    // message fields
    ip: ::std::option::Option<u32>,
    trusted: ::std::option::Option<bool>,
    port: ::std::option::Option<u32>,
    success: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCRequestSubGCSessionInfoResponse {}

impl CMsgGCRequestSubGCSessionInfoResponse {
    pub fn new() -> CMsgGCRequestSubGCSessionInfoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCRequestSubGCSessionInfoResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCRequestSubGCSessionInfoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCRequestSubGCSessionInfoResponse,
        };
        unsafe {
            instance.get(CMsgGCRequestSubGCSessionInfoResponse::new)
        }
    }

    // optional fixed32 ip = 1;

    pub fn clear_ip(&mut self) {
        self.ip = ::std::option::Option::None;
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: u32) {
        self.ip = ::std::option::Option::Some(v);
    }

    pub fn get_ip(&self) -> u32 {
        self.ip.unwrap_or(0)
    }

    fn get_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ip
    }

    // optional bool trusted = 2;

    pub fn clear_trusted(&mut self) {
        self.trusted = ::std::option::Option::None;
    }

    pub fn has_trusted(&self) -> bool {
        self.trusted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trusted(&mut self, v: bool) {
        self.trusted = ::std::option::Option::Some(v);
    }

    pub fn get_trusted(&self) -> bool {
        self.trusted.unwrap_or(false)
    }

    fn get_trusted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.trusted
    }

    fn mut_trusted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.trusted
    }

    // optional uint32 port = 3;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }

    fn get_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.port
    }

    // optional bool success = 4;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    fn get_success_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.success
    }
}

impl ::protobuf::Message for CMsgGCRequestSubGCSessionInfoResponse {
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
                    self.ip = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.trusted = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.port = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.success = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ip {
            my_size += 5;
        };
        if let Some(v) = self.trusted {
            my_size += 2;
        };
        if let Some(v) = self.port {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.success {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ip {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.trusted {
            os.write_bool(2, v)?;
        };
        if let Some(v) = self.port {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.success {
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

impl ::protobuf::MessageStatic for CMsgGCRequestSubGCSessionInfoResponse {
    fn new() -> CMsgGCRequestSubGCSessionInfoResponse {
        CMsgGCRequestSubGCSessionInfoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCRequestSubGCSessionInfoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ip",
                    CMsgGCRequestSubGCSessionInfoResponse::get_ip_for_reflect,
                    CMsgGCRequestSubGCSessionInfoResponse::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "trusted",
                    CMsgGCRequestSubGCSessionInfoResponse::get_trusted_for_reflect,
                    CMsgGCRequestSubGCSessionInfoResponse::mut_trusted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "port",
                    CMsgGCRequestSubGCSessionInfoResponse::get_port_for_reflect,
                    CMsgGCRequestSubGCSessionInfoResponse::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    CMsgGCRequestSubGCSessionInfoResponse::get_success_for_reflect,
                    CMsgGCRequestSubGCSessionInfoResponse::mut_success_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCRequestSubGCSessionInfoResponse>(
                    "CMsgGCRequestSubGCSessionInfoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCRequestSubGCSessionInfoResponse {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_trusted();
        self.clear_port();
        self.clear_success();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCRequestSubGCSessionInfoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCRequestSubGCSessionInfoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSOCacheHaveVersion {
    // message fields
    soid: ::protobuf::SingularPtrField<CMsgSOIDOwner>,
    version: ::std::option::Option<u64>,
    service_id: ::std::option::Option<u32>,
    cached_file_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSOCacheHaveVersion {}

impl CMsgSOCacheHaveVersion {
    pub fn new() -> CMsgSOCacheHaveVersion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSOCacheHaveVersion {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSOCacheHaveVersion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSOCacheHaveVersion,
        };
        unsafe {
            instance.get(CMsgSOCacheHaveVersion::new)
        }
    }

    // optional .dota.CMsgSOIDOwner soid = 1;

    pub fn clear_soid(&mut self) {
        self.soid.clear();
    }

    pub fn has_soid(&self) -> bool {
        self.soid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soid(&mut self, v: CMsgSOIDOwner) {
        self.soid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_soid(&mut self) -> &mut CMsgSOIDOwner {
        if self.soid.is_none() {
            self.soid.set_default();
        };
        self.soid.as_mut().unwrap()
    }

    // Take field
    pub fn take_soid(&mut self) -> CMsgSOIDOwner {
        self.soid.take().unwrap_or_else(|| CMsgSOIDOwner::new())
    }

    pub fn get_soid(&self) -> &CMsgSOIDOwner {
        self.soid.as_ref().unwrap_or_else(|| CMsgSOIDOwner::default_instance())
    }

    fn get_soid_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &self.soid
    }

    fn mut_soid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSOIDOwner> {
        &mut self.soid
    }

    // optional fixed64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }

    // optional uint32 service_id = 3;

    pub fn clear_service_id(&mut self) {
        self.service_id = ::std::option::Option::None;
    }

    pub fn has_service_id(&self) -> bool {
        self.service_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: u32) {
        self.service_id = ::std::option::Option::Some(v);
    }

    pub fn get_service_id(&self) -> u32 {
        self.service_id.unwrap_or(0)
    }

    fn get_service_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service_id
    }

    fn mut_service_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service_id
    }

    // optional uint32 cached_file_version = 4;

    pub fn clear_cached_file_version(&mut self) {
        self.cached_file_version = ::std::option::Option::None;
    }

    pub fn has_cached_file_version(&self) -> bool {
        self.cached_file_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cached_file_version(&mut self, v: u32) {
        self.cached_file_version = ::std::option::Option::Some(v);
    }

    pub fn get_cached_file_version(&self) -> u32 {
        self.cached_file_version.unwrap_or(0)
    }

    fn get_cached_file_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cached_file_version
    }

    fn mut_cached_file_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cached_file_version
    }
}

impl ::protobuf::Message for CMsgSOCacheHaveVersion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.soid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.service_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.cached_file_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.soid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.version {
            my_size += 9;
        };
        if let Some(v) = self.service_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.cached_file_version {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.soid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.version {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.service_id {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.cached_file_version {
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

impl ::protobuf::MessageStatic for CMsgSOCacheHaveVersion {
    fn new() -> CMsgSOCacheHaveVersion {
        CMsgSOCacheHaveVersion::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSOCacheHaveVersion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOIDOwner>>(
                    "soid",
                    CMsgSOCacheHaveVersion::get_soid_for_reflect,
                    CMsgSOCacheHaveVersion::mut_soid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "version",
                    CMsgSOCacheHaveVersion::get_version_for_reflect,
                    CMsgSOCacheHaveVersion::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_id",
                    CMsgSOCacheHaveVersion::get_service_id_for_reflect,
                    CMsgSOCacheHaveVersion::mut_service_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cached_file_version",
                    CMsgSOCacheHaveVersion::get_cached_file_version_for_reflect,
                    CMsgSOCacheHaveVersion::mut_cached_file_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSOCacheHaveVersion>(
                    "CMsgSOCacheHaveVersion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSOCacheHaveVersion {
    fn clear(&mut self) {
        self.clear_soid();
        self.clear_version();
        self.clear_service_id();
        self.clear_cached_file_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSOCacheHaveVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSOCacheHaveVersion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientHello {
    // message fields
    version: ::std::option::Option<u32>,
    socache_have_versions: ::protobuf::RepeatedField<CMsgSOCacheHaveVersion>,
    client_session_need: ::std::option::Option<u32>,
    client_launcher: ::std::option::Option<PartnerAccountType>,
    secret_key: ::protobuf::SingularField<::std::string::String>,
    client_language: ::std::option::Option<u32>,
    engine: ::std::option::Option<ESourceEngine>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientHello {}

impl CMsgClientHello {
    pub fn new() -> CMsgClientHello {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientHello {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientHello> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientHello,
        };
        unsafe {
            instance.get(CMsgClientHello::new)
        }
    }

    // optional uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
    }

    // repeated .dota.CMsgSOCacheHaveVersion socache_have_versions = 2;

    pub fn clear_socache_have_versions(&mut self) {
        self.socache_have_versions.clear();
    }

    // Param is passed by value, moved
    pub fn set_socache_have_versions(&mut self, v: ::protobuf::RepeatedField<CMsgSOCacheHaveVersion>) {
        self.socache_have_versions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_socache_have_versions(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOCacheHaveVersion> {
        &mut self.socache_have_versions
    }

    // Take field
    pub fn take_socache_have_versions(&mut self) -> ::protobuf::RepeatedField<CMsgSOCacheHaveVersion> {
        ::std::mem::replace(&mut self.socache_have_versions, ::protobuf::RepeatedField::new())
    }

    pub fn get_socache_have_versions(&self) -> &[CMsgSOCacheHaveVersion] {
        &self.socache_have_versions
    }

    fn get_socache_have_versions_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSOCacheHaveVersion> {
        &self.socache_have_versions
    }

    fn mut_socache_have_versions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOCacheHaveVersion> {
        &mut self.socache_have_versions
    }

    // optional uint32 client_session_need = 3;

    pub fn clear_client_session_need(&mut self) {
        self.client_session_need = ::std::option::Option::None;
    }

    pub fn has_client_session_need(&self) -> bool {
        self.client_session_need.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_session_need(&mut self, v: u32) {
        self.client_session_need = ::std::option::Option::Some(v);
    }

    pub fn get_client_session_need(&self) -> u32 {
        self.client_session_need.unwrap_or(0)
    }

    fn get_client_session_need_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_session_need
    }

    fn mut_client_session_need_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_session_need
    }

    // optional .dota.PartnerAccountType client_launcher = 4;

    pub fn clear_client_launcher(&mut self) {
        self.client_launcher = ::std::option::Option::None;
    }

    pub fn has_client_launcher(&self) -> bool {
        self.client_launcher.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_launcher(&mut self, v: PartnerAccountType) {
        self.client_launcher = ::std::option::Option::Some(v);
    }

    pub fn get_client_launcher(&self) -> PartnerAccountType {
        self.client_launcher.unwrap_or(PartnerAccountType::PARTNER_NONE)
    }

    fn get_client_launcher_for_reflect(&self) -> &::std::option::Option<PartnerAccountType> {
        &self.client_launcher
    }

    fn mut_client_launcher_for_reflect(&mut self) -> &mut ::std::option::Option<PartnerAccountType> {
        &mut self.client_launcher
    }

    // optional string secret_key = 5;

    pub fn clear_secret_key(&mut self) {
        self.secret_key.clear();
    }

    pub fn has_secret_key(&self) -> bool {
        self.secret_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secret_key(&mut self, v: ::std::string::String) {
        self.secret_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret_key(&mut self) -> &mut ::std::string::String {
        if self.secret_key.is_none() {
            self.secret_key.set_default();
        };
        self.secret_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_secret_key(&mut self) -> ::std::string::String {
        self.secret_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_secret_key(&self) -> &str {
        match self.secret_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_secret_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.secret_key
    }

    fn mut_secret_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.secret_key
    }

    // optional uint32 client_language = 6;

    pub fn clear_client_language(&mut self) {
        self.client_language = ::std::option::Option::None;
    }

    pub fn has_client_language(&self) -> bool {
        self.client_language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_language(&mut self, v: u32) {
        self.client_language = ::std::option::Option::Some(v);
    }

    pub fn get_client_language(&self) -> u32 {
        self.client_language.unwrap_or(0)
    }

    fn get_client_language_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_language
    }

    fn mut_client_language_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_language
    }

    // optional .dota.ESourceEngine engine = 7;

    pub fn clear_engine(&mut self) {
        self.engine = ::std::option::Option::None;
    }

    pub fn has_engine(&self) -> bool {
        self.engine.is_some()
    }

    // Param is passed by value, moved
    pub fn set_engine(&mut self, v: ESourceEngine) {
        self.engine = ::std::option::Option::Some(v);
    }

    pub fn get_engine(&self) -> ESourceEngine {
        self.engine.unwrap_or(ESourceEngine::k_ESE_Source1)
    }

    fn get_engine_for_reflect(&self) -> &::std::option::Option<ESourceEngine> {
        &self.engine
    }

    fn mut_engine_for_reflect(&mut self) -> &mut ::std::option::Option<ESourceEngine> {
        &mut self.engine
    }
}

impl ::protobuf::Message for CMsgClientHello {
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
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.socache_have_versions)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_session_need = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.client_launcher = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.secret_key)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_language = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.engine = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.socache_have_versions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.client_session_need {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_launcher {
            my_size += ::protobuf::rt::enum_size(4, v);
        };
        if let Some(v) = self.secret_key.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.client_language {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.engine {
            my_size += ::protobuf::rt::enum_size(7, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint32(1, v)?;
        };
        for v in &self.socache_have_versions {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.client_session_need {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.client_launcher {
            os.write_enum(4, v.value())?;
        };
        if let Some(v) = self.secret_key.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.client_language {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.engine {
            os.write_enum(7, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgClientHello {
    fn new() -> CMsgClientHello {
        CMsgClientHello::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientHello>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    CMsgClientHello::get_version_for_reflect,
                    CMsgClientHello::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOCacheHaveVersion>>(
                    "socache_have_versions",
                    CMsgClientHello::get_socache_have_versions_for_reflect,
                    CMsgClientHello::mut_socache_have_versions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_session_need",
                    CMsgClientHello::get_client_session_need_for_reflect,
                    CMsgClientHello::mut_client_session_need_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PartnerAccountType>>(
                    "client_launcher",
                    CMsgClientHello::get_client_launcher_for_reflect,
                    CMsgClientHello::mut_client_launcher_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "secret_key",
                    CMsgClientHello::get_secret_key_for_reflect,
                    CMsgClientHello::mut_secret_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_language",
                    CMsgClientHello::get_client_language_for_reflect,
                    CMsgClientHello::mut_client_language_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ESourceEngine>>(
                    "engine",
                    CMsgClientHello::get_engine_for_reflect,
                    CMsgClientHello::mut_engine_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientHello>(
                    "CMsgClientHello",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientHello {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_socache_have_versions();
        self.clear_client_session_need();
        self.clear_client_launcher();
        self.clear_secret_key();
        self.clear_client_language();
        self.clear_engine();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientHello {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientHello {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientWelcome {
    // message fields
    version: ::std::option::Option<u32>,
    game_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    outofdate_subscribed_caches: ::protobuf::RepeatedField<CMsgSOCacheSubscribed>,
    uptodate_subscribed_caches: ::protobuf::RepeatedField<CMsgSOCacheSubscriptionCheck>,
    location: ::protobuf::SingularPtrField<CMsgClientWelcome_Location>,
    save_game_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    item_schema_crc: ::std::option::Option<u32>,
    items_game_url: ::protobuf::SingularField<::std::string::String>,
    gc_socache_file_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientWelcome {}

impl CMsgClientWelcome {
    pub fn new() -> CMsgClientWelcome {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientWelcome {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientWelcome> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientWelcome,
        };
        unsafe {
            instance.get(CMsgClientWelcome::new)
        }
    }

    // optional uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
    }

    // optional bytes game_data = 2;

    pub fn clear_game_data(&mut self) {
        self.game_data.clear();
    }

    pub fn has_game_data(&self) -> bool {
        self.game_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.game_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.game_data.is_none() {
            self.game_data.set_default();
        };
        self.game_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_data(&mut self) -> ::std::vec::Vec<u8> {
        self.game_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_game_data(&self) -> &[u8] {
        match self.game_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_game_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.game_data
    }

    fn mut_game_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.game_data
    }

    // repeated .dota.CMsgSOCacheSubscribed outofdate_subscribed_caches = 3;

    pub fn clear_outofdate_subscribed_caches(&mut self) {
        self.outofdate_subscribed_caches.clear();
    }

    // Param is passed by value, moved
    pub fn set_outofdate_subscribed_caches(&mut self, v: ::protobuf::RepeatedField<CMsgSOCacheSubscribed>) {
        self.outofdate_subscribed_caches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_outofdate_subscribed_caches(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOCacheSubscribed> {
        &mut self.outofdate_subscribed_caches
    }

    // Take field
    pub fn take_outofdate_subscribed_caches(&mut self) -> ::protobuf::RepeatedField<CMsgSOCacheSubscribed> {
        ::std::mem::replace(&mut self.outofdate_subscribed_caches, ::protobuf::RepeatedField::new())
    }

    pub fn get_outofdate_subscribed_caches(&self) -> &[CMsgSOCacheSubscribed] {
        &self.outofdate_subscribed_caches
    }

    fn get_outofdate_subscribed_caches_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSOCacheSubscribed> {
        &self.outofdate_subscribed_caches
    }

    fn mut_outofdate_subscribed_caches_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOCacheSubscribed> {
        &mut self.outofdate_subscribed_caches
    }

    // repeated .dota.CMsgSOCacheSubscriptionCheck uptodate_subscribed_caches = 4;

    pub fn clear_uptodate_subscribed_caches(&mut self) {
        self.uptodate_subscribed_caches.clear();
    }

    // Param is passed by value, moved
    pub fn set_uptodate_subscribed_caches(&mut self, v: ::protobuf::RepeatedField<CMsgSOCacheSubscriptionCheck>) {
        self.uptodate_subscribed_caches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uptodate_subscribed_caches(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOCacheSubscriptionCheck> {
        &mut self.uptodate_subscribed_caches
    }

    // Take field
    pub fn take_uptodate_subscribed_caches(&mut self) -> ::protobuf::RepeatedField<CMsgSOCacheSubscriptionCheck> {
        ::std::mem::replace(&mut self.uptodate_subscribed_caches, ::protobuf::RepeatedField::new())
    }

    pub fn get_uptodate_subscribed_caches(&self) -> &[CMsgSOCacheSubscriptionCheck] {
        &self.uptodate_subscribed_caches
    }

    fn get_uptodate_subscribed_caches_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSOCacheSubscriptionCheck> {
        &self.uptodate_subscribed_caches
    }

    fn mut_uptodate_subscribed_caches_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSOCacheSubscriptionCheck> {
        &mut self.uptodate_subscribed_caches
    }

    // optional .dota.CMsgClientWelcome.Location location = 5;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: CMsgClientWelcome_Location) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut CMsgClientWelcome_Location {
        if self.location.is_none() {
            self.location.set_default();
        };
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> CMsgClientWelcome_Location {
        self.location.take().unwrap_or_else(|| CMsgClientWelcome_Location::new())
    }

    pub fn get_location(&self) -> &CMsgClientWelcome_Location {
        self.location.as_ref().unwrap_or_else(|| CMsgClientWelcome_Location::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgClientWelcome_Location> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgClientWelcome_Location> {
        &mut self.location
    }

    // optional bytes save_game_key = 6;

    pub fn clear_save_game_key(&mut self) {
        self.save_game_key.clear();
    }

    pub fn has_save_game_key(&self) -> bool {
        self.save_game_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_game_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.save_game_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_save_game_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.save_game_key.is_none() {
            self.save_game_key.set_default();
        };
        self.save_game_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_save_game_key(&mut self) -> ::std::vec::Vec<u8> {
        self.save_game_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_save_game_key(&self) -> &[u8] {
        match self.save_game_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_save_game_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.save_game_key
    }

    fn mut_save_game_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.save_game_key
    }

    // optional fixed32 item_schema_crc = 7;

    pub fn clear_item_schema_crc(&mut self) {
        self.item_schema_crc = ::std::option::Option::None;
    }

    pub fn has_item_schema_crc(&self) -> bool {
        self.item_schema_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_schema_crc(&mut self, v: u32) {
        self.item_schema_crc = ::std::option::Option::Some(v);
    }

    pub fn get_item_schema_crc(&self) -> u32 {
        self.item_schema_crc.unwrap_or(0)
    }

    fn get_item_schema_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_schema_crc
    }

    fn mut_item_schema_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_schema_crc
    }

    // optional string items_game_url = 8;

    pub fn clear_items_game_url(&mut self) {
        self.items_game_url.clear();
    }

    pub fn has_items_game_url(&self) -> bool {
        self.items_game_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_items_game_url(&mut self, v: ::std::string::String) {
        self.items_game_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_items_game_url(&mut self) -> &mut ::std::string::String {
        if self.items_game_url.is_none() {
            self.items_game_url.set_default();
        };
        self.items_game_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_items_game_url(&mut self) -> ::std::string::String {
        self.items_game_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_items_game_url(&self) -> &str {
        match self.items_game_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_items_game_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.items_game_url
    }

    fn mut_items_game_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.items_game_url
    }

    // optional uint32 gc_socache_file_version = 9;

    pub fn clear_gc_socache_file_version(&mut self) {
        self.gc_socache_file_version = ::std::option::Option::None;
    }

    pub fn has_gc_socache_file_version(&self) -> bool {
        self.gc_socache_file_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_socache_file_version(&mut self, v: u32) {
        self.gc_socache_file_version = ::std::option::Option::Some(v);
    }

    pub fn get_gc_socache_file_version(&self) -> u32 {
        self.gc_socache_file_version.unwrap_or(0)
    }

    fn get_gc_socache_file_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gc_socache_file_version
    }

    fn mut_gc_socache_file_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gc_socache_file_version
    }
}

impl ::protobuf::Message for CMsgClientWelcome {
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
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.game_data)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.outofdate_subscribed_caches)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uptodate_subscribed_caches)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.save_game_key)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.item_schema_crc = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.items_game_url)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.gc_socache_file_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        for value in &self.outofdate_subscribed_caches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.uptodate_subscribed_caches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.save_game_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        };
        if let Some(v) = self.item_schema_crc {
            my_size += 5;
        };
        if let Some(v) = self.items_game_url.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let Some(v) = self.gc_socache_file_version {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.game_data.as_ref() {
            os.write_bytes(2, &v)?;
        };
        for v in &self.outofdate_subscribed_caches {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.uptodate_subscribed_caches {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.location.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.save_game_key.as_ref() {
            os.write_bytes(6, &v)?;
        };
        if let Some(v) = self.item_schema_crc {
            os.write_fixed32(7, v)?;
        };
        if let Some(v) = self.items_game_url.as_ref() {
            os.write_string(8, &v)?;
        };
        if let Some(v) = self.gc_socache_file_version {
            os.write_uint32(9, v)?;
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

impl ::protobuf::MessageStatic for CMsgClientWelcome {
    fn new() -> CMsgClientWelcome {
        CMsgClientWelcome::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientWelcome>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    CMsgClientWelcome::get_version_for_reflect,
                    CMsgClientWelcome::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "game_data",
                    CMsgClientWelcome::get_game_data_for_reflect,
                    CMsgClientWelcome::mut_game_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOCacheSubscribed>>(
                    "outofdate_subscribed_caches",
                    CMsgClientWelcome::get_outofdate_subscribed_caches_for_reflect,
                    CMsgClientWelcome::mut_outofdate_subscribed_caches_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSOCacheSubscriptionCheck>>(
                    "uptodate_subscribed_caches",
                    CMsgClientWelcome::get_uptodate_subscribed_caches_for_reflect,
                    CMsgClientWelcome::mut_uptodate_subscribed_caches_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgClientWelcome_Location>>(
                    "location",
                    CMsgClientWelcome::get_location_for_reflect,
                    CMsgClientWelcome::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "save_game_key",
                    CMsgClientWelcome::get_save_game_key_for_reflect,
                    CMsgClientWelcome::mut_save_game_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "item_schema_crc",
                    CMsgClientWelcome::get_item_schema_crc_for_reflect,
                    CMsgClientWelcome::mut_item_schema_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "items_game_url",
                    CMsgClientWelcome::get_items_game_url_for_reflect,
                    CMsgClientWelcome::mut_items_game_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gc_socache_file_version",
                    CMsgClientWelcome::get_gc_socache_file_version_for_reflect,
                    CMsgClientWelcome::mut_gc_socache_file_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientWelcome>(
                    "CMsgClientWelcome",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientWelcome {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_game_data();
        self.clear_outofdate_subscribed_caches();
        self.clear_uptodate_subscribed_caches();
        self.clear_location();
        self.clear_save_game_key();
        self.clear_item_schema_crc();
        self.clear_items_game_url();
        self.clear_gc_socache_file_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientWelcome {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientWelcome {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientWelcome_Location {
    // message fields
    latitude: ::std::option::Option<f32>,
    longitude: ::std::option::Option<f32>,
    country: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientWelcome_Location {}

impl CMsgClientWelcome_Location {
    pub fn new() -> CMsgClientWelcome_Location {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientWelcome_Location {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientWelcome_Location> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientWelcome_Location,
        };
        unsafe {
            instance.get(CMsgClientWelcome_Location::new)
        }
    }

    // optional float latitude = 1;

    pub fn clear_latitude(&mut self) {
        self.latitude = ::std::option::Option::None;
    }

    pub fn has_latitude(&self) -> bool {
        self.latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latitude(&mut self, v: f32) {
        self.latitude = ::std::option::Option::Some(v);
    }

    pub fn get_latitude(&self) -> f32 {
        self.latitude.unwrap_or(0.)
    }

    fn get_latitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.latitude
    }

    fn mut_latitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.latitude
    }

    // optional float longitude = 2;

    pub fn clear_longitude(&mut self) {
        self.longitude = ::std::option::Option::None;
    }

    pub fn has_longitude(&self) -> bool {
        self.longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_longitude(&mut self, v: f32) {
        self.longitude = ::std::option::Option::Some(v);
    }

    pub fn get_longitude(&self) -> f32 {
        self.longitude.unwrap_or(0.)
    }

    fn get_longitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.longitude
    }

    fn mut_longitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.longitude
    }

    // optional string country = 3;

    pub fn clear_country(&mut self) {
        self.country.clear();
    }

    pub fn has_country(&self) -> bool {
        self.country.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country(&mut self, v: ::std::string::String) {
        self.country = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country(&mut self) -> &mut ::std::string::String {
        if self.country.is_none() {
            self.country.set_default();
        };
        self.country.as_mut().unwrap()
    }

    // Take field
    pub fn take_country(&mut self) -> ::std::string::String {
        self.country.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country(&self) -> &str {
        match self.country.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country
    }

    fn mut_country_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country
    }
}

impl ::protobuf::Message for CMsgClientWelcome_Location {
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
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.longitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country)?;
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
        if let Some(v) = self.latitude {
            my_size += 5;
        };
        if let Some(v) = self.longitude {
            my_size += 5;
        };
        if let Some(v) = self.country.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.latitude {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.longitude {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.country.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgClientWelcome_Location {
    fn new() -> CMsgClientWelcome_Location {
        CMsgClientWelcome_Location::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientWelcome_Location>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "latitude",
                    CMsgClientWelcome_Location::get_latitude_for_reflect,
                    CMsgClientWelcome_Location::mut_latitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "longitude",
                    CMsgClientWelcome_Location::get_longitude_for_reflect,
                    CMsgClientWelcome_Location::mut_longitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country",
                    CMsgClientWelcome_Location::get_country_for_reflect,
                    CMsgClientWelcome_Location::mut_country_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientWelcome_Location>(
                    "CMsgClientWelcome_Location",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientWelcome_Location {
    fn clear(&mut self) {
        self.clear_latitude();
        self.clear_longitude();
        self.clear_country();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientWelcome_Location {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientWelcome_Location {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgConnectionStatus {
    // message fields
    status: ::std::option::Option<GCConnectionStatus>,
    client_session_need: ::std::option::Option<u32>,
    queue_position: ::std::option::Option<i32>,
    queue_size: ::std::option::Option<i32>,
    wait_seconds: ::std::option::Option<i32>,
    estimated_wait_seconds_remaining: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgConnectionStatus {}

impl CMsgConnectionStatus {
    pub fn new() -> CMsgConnectionStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgConnectionStatus {
        static mut instance: ::protobuf::lazy::Lazy<CMsgConnectionStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgConnectionStatus,
        };
        unsafe {
            instance.get(CMsgConnectionStatus::new)
        }
    }

    // optional .dota.GCConnectionStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: GCConnectionStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> GCConnectionStatus {
        self.status.unwrap_or(GCConnectionStatus::GCConnectionStatus_HAVE_SESSION)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<GCConnectionStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<GCConnectionStatus> {
        &mut self.status
    }

    // optional uint32 client_session_need = 2;

    pub fn clear_client_session_need(&mut self) {
        self.client_session_need = ::std::option::Option::None;
    }

    pub fn has_client_session_need(&self) -> bool {
        self.client_session_need.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_session_need(&mut self, v: u32) {
        self.client_session_need = ::std::option::Option::Some(v);
    }

    pub fn get_client_session_need(&self) -> u32 {
        self.client_session_need.unwrap_or(0)
    }

    fn get_client_session_need_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_session_need
    }

    fn mut_client_session_need_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_session_need
    }

    // optional int32 queue_position = 3;

    pub fn clear_queue_position(&mut self) {
        self.queue_position = ::std::option::Option::None;
    }

    pub fn has_queue_position(&self) -> bool {
        self.queue_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queue_position(&mut self, v: i32) {
        self.queue_position = ::std::option::Option::Some(v);
    }

    pub fn get_queue_position(&self) -> i32 {
        self.queue_position.unwrap_or(0)
    }

    fn get_queue_position_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.queue_position
    }

    fn mut_queue_position_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.queue_position
    }

    // optional int32 queue_size = 4;

    pub fn clear_queue_size(&mut self) {
        self.queue_size = ::std::option::Option::None;
    }

    pub fn has_queue_size(&self) -> bool {
        self.queue_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queue_size(&mut self, v: i32) {
        self.queue_size = ::std::option::Option::Some(v);
    }

    pub fn get_queue_size(&self) -> i32 {
        self.queue_size.unwrap_or(0)
    }

    fn get_queue_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.queue_size
    }

    fn mut_queue_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.queue_size
    }

    // optional int32 wait_seconds = 5;

    pub fn clear_wait_seconds(&mut self) {
        self.wait_seconds = ::std::option::Option::None;
    }

    pub fn has_wait_seconds(&self) -> bool {
        self.wait_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wait_seconds(&mut self, v: i32) {
        self.wait_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_wait_seconds(&self) -> i32 {
        self.wait_seconds.unwrap_or(0)
    }

    fn get_wait_seconds_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.wait_seconds
    }

    fn mut_wait_seconds_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.wait_seconds
    }

    // optional int32 estimated_wait_seconds_remaining = 6;

    pub fn clear_estimated_wait_seconds_remaining(&mut self) {
        self.estimated_wait_seconds_remaining = ::std::option::Option::None;
    }

    pub fn has_estimated_wait_seconds_remaining(&self) -> bool {
        self.estimated_wait_seconds_remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_estimated_wait_seconds_remaining(&mut self, v: i32) {
        self.estimated_wait_seconds_remaining = ::std::option::Option::Some(v);
    }

    pub fn get_estimated_wait_seconds_remaining(&self) -> i32 {
        self.estimated_wait_seconds_remaining.unwrap_or(0)
    }

    fn get_estimated_wait_seconds_remaining_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.estimated_wait_seconds_remaining
    }

    fn mut_estimated_wait_seconds_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.estimated_wait_seconds_remaining
    }
}

impl ::protobuf::Message for CMsgConnectionStatus {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_session_need = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.queue_position = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.queue_size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.wait_seconds = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.estimated_wait_seconds_remaining = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.client_session_need {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.queue_position {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.queue_size {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.wait_seconds {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.estimated_wait_seconds_remaining {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.client_session_need {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.queue_position {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.queue_size {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.wait_seconds {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.estimated_wait_seconds_remaining {
            os.write_int32(6, v)?;
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

impl ::protobuf::MessageStatic for CMsgConnectionStatus {
    fn new() -> CMsgConnectionStatus {
        CMsgConnectionStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgConnectionStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<GCConnectionStatus>>(
                    "status",
                    CMsgConnectionStatus::get_status_for_reflect,
                    CMsgConnectionStatus::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_session_need",
                    CMsgConnectionStatus::get_client_session_need_for_reflect,
                    CMsgConnectionStatus::mut_client_session_need_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "queue_position",
                    CMsgConnectionStatus::get_queue_position_for_reflect,
                    CMsgConnectionStatus::mut_queue_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "queue_size",
                    CMsgConnectionStatus::get_queue_size_for_reflect,
                    CMsgConnectionStatus::mut_queue_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "wait_seconds",
                    CMsgConnectionStatus::get_wait_seconds_for_reflect,
                    CMsgConnectionStatus::mut_wait_seconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "estimated_wait_seconds_remaining",
                    CMsgConnectionStatus::get_estimated_wait_seconds_remaining_for_reflect,
                    CMsgConnectionStatus::mut_estimated_wait_seconds_remaining_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgConnectionStatus>(
                    "CMsgConnectionStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgConnectionStatus {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_client_session_need();
        self.clear_queue_position();
        self.clear_queue_size();
        self.clear_wait_seconds();
        self.clear_estimated_wait_seconds_remaining();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgConnectionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgConnectionStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCSOCacheSubscribe {
    // message fields
    subscriber: ::std::option::Option<u64>,
    subscribe_to_id: ::std::option::Option<u64>,
    sync_version: ::std::option::Option<u64>,
    have_versions: ::protobuf::RepeatedField<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions>,
    subscribe_to_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCSOCacheSubscribe {}

impl CMsgGCToGCSOCacheSubscribe {
    pub fn new() -> CMsgGCToGCSOCacheSubscribe {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCSOCacheSubscribe {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCSOCacheSubscribe> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCSOCacheSubscribe,
        };
        unsafe {
            instance.get(CMsgGCToGCSOCacheSubscribe::new)
        }
    }

    // optional fixed64 subscriber = 1;

    pub fn clear_subscriber(&mut self) {
        self.subscriber = ::std::option::Option::None;
    }

    pub fn has_subscriber(&self) -> bool {
        self.subscriber.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscriber(&mut self, v: u64) {
        self.subscriber = ::std::option::Option::Some(v);
    }

    pub fn get_subscriber(&self) -> u64 {
        self.subscriber.unwrap_or(0)
    }

    fn get_subscriber_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.subscriber
    }

    fn mut_subscriber_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.subscriber
    }

    // optional fixed64 subscribe_to_id = 2;

    pub fn clear_subscribe_to_id(&mut self) {
        self.subscribe_to_id = ::std::option::Option::None;
    }

    pub fn has_subscribe_to_id(&self) -> bool {
        self.subscribe_to_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscribe_to_id(&mut self, v: u64) {
        self.subscribe_to_id = ::std::option::Option::Some(v);
    }

    pub fn get_subscribe_to_id(&self) -> u64 {
        self.subscribe_to_id.unwrap_or(0)
    }

    fn get_subscribe_to_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.subscribe_to_id
    }

    fn mut_subscribe_to_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.subscribe_to_id
    }

    // optional fixed64 sync_version = 3;

    pub fn clear_sync_version(&mut self) {
        self.sync_version = ::std::option::Option::None;
    }

    pub fn has_sync_version(&self) -> bool {
        self.sync_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sync_version(&mut self, v: u64) {
        self.sync_version = ::std::option::Option::Some(v);
    }

    pub fn get_sync_version(&self) -> u64 {
        self.sync_version.unwrap_or(0)
    }

    fn get_sync_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sync_version
    }

    fn mut_sync_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sync_version
    }

    // repeated .dota.CMsgGCToGCSOCacheSubscribe.CMsgHaveVersions have_versions = 4;

    pub fn clear_have_versions(&mut self) {
        self.have_versions.clear();
    }

    // Param is passed by value, moved
    pub fn set_have_versions(&mut self, v: ::protobuf::RepeatedField<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions>) {
        self.have_versions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_have_versions(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions> {
        &mut self.have_versions
    }

    // Take field
    pub fn take_have_versions(&mut self) -> ::protobuf::RepeatedField<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions> {
        ::std::mem::replace(&mut self.have_versions, ::protobuf::RepeatedField::new())
    }

    pub fn get_have_versions(&self) -> &[CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions] {
        &self.have_versions
    }

    fn get_have_versions_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions> {
        &self.have_versions
    }

    fn mut_have_versions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions> {
        &mut self.have_versions
    }

    // optional uint32 subscribe_to_type = 5;

    pub fn clear_subscribe_to_type(&mut self) {
        self.subscribe_to_type = ::std::option::Option::None;
    }

    pub fn has_subscribe_to_type(&self) -> bool {
        self.subscribe_to_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscribe_to_type(&mut self, v: u32) {
        self.subscribe_to_type = ::std::option::Option::Some(v);
    }

    pub fn get_subscribe_to_type(&self) -> u32 {
        self.subscribe_to_type.unwrap_or(0)
    }

    fn get_subscribe_to_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.subscribe_to_type
    }

    fn mut_subscribe_to_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.subscribe_to_type
    }
}

impl ::protobuf::Message for CMsgGCToGCSOCacheSubscribe {
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
                    self.subscriber = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.subscribe_to_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.sync_version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.have_versions)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.subscribe_to_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.subscriber {
            my_size += 9;
        };
        if let Some(v) = self.subscribe_to_id {
            my_size += 9;
        };
        if let Some(v) = self.sync_version {
            my_size += 9;
        };
        for value in &self.have_versions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.subscribe_to_type {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.subscriber {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.subscribe_to_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.sync_version {
            os.write_fixed64(3, v)?;
        };
        for v in &self.have_versions {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.subscribe_to_type {
            os.write_uint32(5, v)?;
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

impl ::protobuf::MessageStatic for CMsgGCToGCSOCacheSubscribe {
    fn new() -> CMsgGCToGCSOCacheSubscribe {
        CMsgGCToGCSOCacheSubscribe::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCSOCacheSubscribe>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "subscriber",
                    CMsgGCToGCSOCacheSubscribe::get_subscriber_for_reflect,
                    CMsgGCToGCSOCacheSubscribe::mut_subscriber_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "subscribe_to_id",
                    CMsgGCToGCSOCacheSubscribe::get_subscribe_to_id_for_reflect,
                    CMsgGCToGCSOCacheSubscribe::mut_subscribe_to_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "sync_version",
                    CMsgGCToGCSOCacheSubscribe::get_sync_version_for_reflect,
                    CMsgGCToGCSOCacheSubscribe::mut_sync_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions>>(
                    "have_versions",
                    CMsgGCToGCSOCacheSubscribe::get_have_versions_for_reflect,
                    CMsgGCToGCSOCacheSubscribe::mut_have_versions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "subscribe_to_type",
                    CMsgGCToGCSOCacheSubscribe::get_subscribe_to_type_for_reflect,
                    CMsgGCToGCSOCacheSubscribe::mut_subscribe_to_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCSOCacheSubscribe>(
                    "CMsgGCToGCSOCacheSubscribe",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCSOCacheSubscribe {
    fn clear(&mut self) {
        self.clear_subscriber();
        self.clear_subscribe_to_id();
        self.clear_sync_version();
        self.clear_have_versions();
        self.clear_subscribe_to_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCSOCacheSubscribe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCSOCacheSubscribe {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
    // message fields
    service_id: ::std::option::Option<u32>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {}

impl CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
    pub fn new() -> CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions,
        };
        unsafe {
            instance.get(CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions::new)
        }
    }

    // optional uint32 service_id = 1;

    pub fn clear_service_id(&mut self) {
        self.service_id = ::std::option::Option::None;
    }

    pub fn has_service_id(&self) -> bool {
        self.service_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: u32) {
        self.service_id = ::std::option::Option::Some(v);
    }

    pub fn get_service_id(&self) -> u32 {
        self.service_id.unwrap_or(0)
    }

    fn get_service_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service_id
    }

    fn mut_service_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service_id
    }

    // optional uint64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
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
                    self.service_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
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
        if let Some(v) = self.service_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.service_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.version {
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

impl ::protobuf::MessageStatic for CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
    fn new() -> CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
        CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_id",
                    CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions::get_service_id_for_reflect,
                    CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions::mut_service_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions::get_version_for_reflect,
                    CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions>(
                    "CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
    fn clear(&mut self) {
        self.clear_service_id();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCSOCacheSubscribe_CMsgHaveVersions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCSOCacheUnsubscribe {
    // message fields
    subscriber: ::std::option::Option<u64>,
    unsubscribe_from_id: ::std::option::Option<u64>,
    unsubscribe_from_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCSOCacheUnsubscribe {}

impl CMsgGCToGCSOCacheUnsubscribe {
    pub fn new() -> CMsgGCToGCSOCacheUnsubscribe {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCSOCacheUnsubscribe {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCSOCacheUnsubscribe> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCSOCacheUnsubscribe,
        };
        unsafe {
            instance.get(CMsgGCToGCSOCacheUnsubscribe::new)
        }
    }

    // optional fixed64 subscriber = 1;

    pub fn clear_subscriber(&mut self) {
        self.subscriber = ::std::option::Option::None;
    }

    pub fn has_subscriber(&self) -> bool {
        self.subscriber.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subscriber(&mut self, v: u64) {
        self.subscriber = ::std::option::Option::Some(v);
    }

    pub fn get_subscriber(&self) -> u64 {
        self.subscriber.unwrap_or(0)
    }

    fn get_subscriber_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.subscriber
    }

    fn mut_subscriber_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.subscriber
    }

    // optional fixed64 unsubscribe_from_id = 2;

    pub fn clear_unsubscribe_from_id(&mut self) {
        self.unsubscribe_from_id = ::std::option::Option::None;
    }

    pub fn has_unsubscribe_from_id(&self) -> bool {
        self.unsubscribe_from_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unsubscribe_from_id(&mut self, v: u64) {
        self.unsubscribe_from_id = ::std::option::Option::Some(v);
    }

    pub fn get_unsubscribe_from_id(&self) -> u64 {
        self.unsubscribe_from_id.unwrap_or(0)
    }

    fn get_unsubscribe_from_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.unsubscribe_from_id
    }

    fn mut_unsubscribe_from_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.unsubscribe_from_id
    }

    // optional uint32 unsubscribe_from_type = 3;

    pub fn clear_unsubscribe_from_type(&mut self) {
        self.unsubscribe_from_type = ::std::option::Option::None;
    }

    pub fn has_unsubscribe_from_type(&self) -> bool {
        self.unsubscribe_from_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unsubscribe_from_type(&mut self, v: u32) {
        self.unsubscribe_from_type = ::std::option::Option::Some(v);
    }

    pub fn get_unsubscribe_from_type(&self) -> u32 {
        self.unsubscribe_from_type.unwrap_or(0)
    }

    fn get_unsubscribe_from_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unsubscribe_from_type
    }

    fn mut_unsubscribe_from_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unsubscribe_from_type
    }
}

impl ::protobuf::Message for CMsgGCToGCSOCacheUnsubscribe {
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
                    self.subscriber = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.unsubscribe_from_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.unsubscribe_from_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.subscriber {
            my_size += 9;
        };
        if let Some(v) = self.unsubscribe_from_id {
            my_size += 9;
        };
        if let Some(v) = self.unsubscribe_from_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.subscriber {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.unsubscribe_from_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.unsubscribe_from_type {
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

impl ::protobuf::MessageStatic for CMsgGCToGCSOCacheUnsubscribe {
    fn new() -> CMsgGCToGCSOCacheUnsubscribe {
        CMsgGCToGCSOCacheUnsubscribe::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCSOCacheUnsubscribe>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "subscriber",
                    CMsgGCToGCSOCacheUnsubscribe::get_subscriber_for_reflect,
                    CMsgGCToGCSOCacheUnsubscribe::mut_subscriber_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "unsubscribe_from_id",
                    CMsgGCToGCSOCacheUnsubscribe::get_unsubscribe_from_id_for_reflect,
                    CMsgGCToGCSOCacheUnsubscribe::mut_unsubscribe_from_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unsubscribe_from_type",
                    CMsgGCToGCSOCacheUnsubscribe::get_unsubscribe_from_type_for_reflect,
                    CMsgGCToGCSOCacheUnsubscribe::mut_unsubscribe_from_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCSOCacheUnsubscribe>(
                    "CMsgGCToGCSOCacheUnsubscribe",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCSOCacheUnsubscribe {
    fn clear(&mut self) {
        self.clear_subscriber();
        self.clear_unsubscribe_from_id();
        self.clear_unsubscribe_from_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCSOCacheUnsubscribe {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCSOCacheUnsubscribe {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCClientPing {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCClientPing {}

impl CMsgGCClientPing {
    pub fn new() -> CMsgGCClientPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCClientPing {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCClientPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCClientPing,
        };
        unsafe {
            instance.get(CMsgGCClientPing::new)
        }
    }
}

impl ::protobuf::Message for CMsgGCClientPing {
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

impl ::protobuf::MessageStatic for CMsgGCClientPing {
    fn new() -> CMsgGCClientPing {
        CMsgGCClientPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCClientPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCClientPing>(
                    "CMsgGCClientPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCClientPing {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCClientPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCClientPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCForwardAccountDetails {
    // message fields
    steamid: ::std::option::Option<u64>,
    account_details: ::protobuf::SingularPtrField<super::steammessages::CGCSystemMsg_GetAccountDetails_Response>,
    age_seconds: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCForwardAccountDetails {}

impl CMsgGCToGCForwardAccountDetails {
    pub fn new() -> CMsgGCToGCForwardAccountDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCForwardAccountDetails {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCForwardAccountDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCForwardAccountDetails,
        };
        unsafe {
            instance.get(CMsgGCToGCForwardAccountDetails::new)
        }
    }

    // optional fixed64 steamid = 1;

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

    // optional .dota.CGCSystemMsg_GetAccountDetails_Response account_details = 2;

    pub fn clear_account_details(&mut self) {
        self.account_details.clear();
    }

    pub fn has_account_details(&self) -> bool {
        self.account_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_details(&mut self, v: super::steammessages::CGCSystemMsg_GetAccountDetails_Response) {
        self.account_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_details(&mut self) -> &mut super::steammessages::CGCSystemMsg_GetAccountDetails_Response {
        if self.account_details.is_none() {
            self.account_details.set_default();
        };
        self.account_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_details(&mut self) -> super::steammessages::CGCSystemMsg_GetAccountDetails_Response {
        self.account_details.take().unwrap_or_else(|| super::steammessages::CGCSystemMsg_GetAccountDetails_Response::new())
    }

    pub fn get_account_details(&self) -> &super::steammessages::CGCSystemMsg_GetAccountDetails_Response {
        self.account_details.as_ref().unwrap_or_else(|| super::steammessages::CGCSystemMsg_GetAccountDetails_Response::default_instance())
    }

    fn get_account_details_for_reflect(&self) -> &::protobuf::SingularPtrField<super::steammessages::CGCSystemMsg_GetAccountDetails_Response> {
        &self.account_details
    }

    fn mut_account_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::steammessages::CGCSystemMsg_GetAccountDetails_Response> {
        &mut self.account_details
    }

    // optional uint32 age_seconds = 3;

    pub fn clear_age_seconds(&mut self) {
        self.age_seconds = ::std::option::Option::None;
    }

    pub fn has_age_seconds(&self) -> bool {
        self.age_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_age_seconds(&mut self, v: u32) {
        self.age_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_age_seconds(&self) -> u32 {
        self.age_seconds.unwrap_or(0)
    }

    fn get_age_seconds_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.age_seconds
    }

    fn mut_age_seconds_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.age_seconds
    }
}

impl ::protobuf::Message for CMsgGCToGCForwardAccountDetails {
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
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.account_details)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.age_seconds = ::std::option::Option::Some(tmp);
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
            my_size += 9;
        };
        if let Some(v) = self.account_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.age_seconds {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.account_details.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.age_seconds {
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

impl ::protobuf::MessageStatic for CMsgGCToGCForwardAccountDetails {
    fn new() -> CMsgGCToGCForwardAccountDetails {
        CMsgGCToGCForwardAccountDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCForwardAccountDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgGCToGCForwardAccountDetails::get_steamid_for_reflect,
                    CMsgGCToGCForwardAccountDetails::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::steammessages::CGCSystemMsg_GetAccountDetails_Response>>(
                    "account_details",
                    CMsgGCToGCForwardAccountDetails::get_account_details_for_reflect,
                    CMsgGCToGCForwardAccountDetails::mut_account_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "age_seconds",
                    CMsgGCToGCForwardAccountDetails::get_age_seconds_for_reflect,
                    CMsgGCToGCForwardAccountDetails::mut_age_seconds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCForwardAccountDetails>(
                    "CMsgGCToGCForwardAccountDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCForwardAccountDetails {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_account_details();
        self.clear_age_seconds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCForwardAccountDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCForwardAccountDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCLoadSessionSOCache {
    // message fields
    account_id: ::std::option::Option<u32>,
    forward_account_details: ::protobuf::SingularPtrField<CMsgGCToGCForwardAccountDetails>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCLoadSessionSOCache {}

impl CMsgGCToGCLoadSessionSOCache {
    pub fn new() -> CMsgGCToGCLoadSessionSOCache {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCLoadSessionSOCache {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCLoadSessionSOCache> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCLoadSessionSOCache,
        };
        unsafe {
            instance.get(CMsgGCToGCLoadSessionSOCache::new)
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

    // optional .dota.CMsgGCToGCForwardAccountDetails forward_account_details = 2;

    pub fn clear_forward_account_details(&mut self) {
        self.forward_account_details.clear();
    }

    pub fn has_forward_account_details(&self) -> bool {
        self.forward_account_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forward_account_details(&mut self, v: CMsgGCToGCForwardAccountDetails) {
        self.forward_account_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forward_account_details(&mut self) -> &mut CMsgGCToGCForwardAccountDetails {
        if self.forward_account_details.is_none() {
            self.forward_account_details.set_default();
        };
        self.forward_account_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_forward_account_details(&mut self) -> CMsgGCToGCForwardAccountDetails {
        self.forward_account_details.take().unwrap_or_else(|| CMsgGCToGCForwardAccountDetails::new())
    }

    pub fn get_forward_account_details(&self) -> &CMsgGCToGCForwardAccountDetails {
        self.forward_account_details.as_ref().unwrap_or_else(|| CMsgGCToGCForwardAccountDetails::default_instance())
    }

    fn get_forward_account_details_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgGCToGCForwardAccountDetails> {
        &self.forward_account_details
    }

    fn mut_forward_account_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgGCToGCForwardAccountDetails> {
        &mut self.forward_account_details
    }
}

impl ::protobuf::Message for CMsgGCToGCLoadSessionSOCache {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.forward_account_details)?;
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
        if let Some(v) = self.forward_account_details.as_ref() {
            let len = v.compute_size();
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
        if let Some(v) = self.forward_account_details.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgGCToGCLoadSessionSOCache {
    fn new() -> CMsgGCToGCLoadSessionSOCache {
        CMsgGCToGCLoadSessionSOCache::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCLoadSessionSOCache>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgGCToGCLoadSessionSOCache::get_account_id_for_reflect,
                    CMsgGCToGCLoadSessionSOCache::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCToGCForwardAccountDetails>>(
                    "forward_account_details",
                    CMsgGCToGCLoadSessionSOCache::get_forward_account_details_for_reflect,
                    CMsgGCToGCLoadSessionSOCache::mut_forward_account_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCLoadSessionSOCache>(
                    "CMsgGCToGCLoadSessionSOCache",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCLoadSessionSOCache {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_forward_account_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCLoadSessionSOCache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCLoadSessionSOCache {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCLoadSessionSOCacheResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCLoadSessionSOCacheResponse {}

impl CMsgGCToGCLoadSessionSOCacheResponse {
    pub fn new() -> CMsgGCToGCLoadSessionSOCacheResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCLoadSessionSOCacheResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCLoadSessionSOCacheResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCLoadSessionSOCacheResponse,
        };
        unsafe {
            instance.get(CMsgGCToGCLoadSessionSOCacheResponse::new)
        }
    }
}

impl ::protobuf::Message for CMsgGCToGCLoadSessionSOCacheResponse {
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

impl ::protobuf::MessageStatic for CMsgGCToGCLoadSessionSOCacheResponse {
    fn new() -> CMsgGCToGCLoadSessionSOCacheResponse {
        CMsgGCToGCLoadSessionSOCacheResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCLoadSessionSOCacheResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCLoadSessionSOCacheResponse>(
                    "CMsgGCToGCLoadSessionSOCacheResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCLoadSessionSOCacheResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCLoadSessionSOCacheResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCLoadSessionSOCacheResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToGCUpdateSessionStats {
    // message fields
    user_sessions: ::std::option::Option<u32>,
    server_sessions: ::std::option::Option<u32>,
    in_logon_surge: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToGCUpdateSessionStats {}

impl CMsgGCToGCUpdateSessionStats {
    pub fn new() -> CMsgGCToGCUpdateSessionStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToGCUpdateSessionStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToGCUpdateSessionStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToGCUpdateSessionStats,
        };
        unsafe {
            instance.get(CMsgGCToGCUpdateSessionStats::new)
        }
    }

    // optional uint32 user_sessions = 1;

    pub fn clear_user_sessions(&mut self) {
        self.user_sessions = ::std::option::Option::None;
    }

    pub fn has_user_sessions(&self) -> bool {
        self.user_sessions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_sessions(&mut self, v: u32) {
        self.user_sessions = ::std::option::Option::Some(v);
    }

    pub fn get_user_sessions(&self) -> u32 {
        self.user_sessions.unwrap_or(0)
    }

    fn get_user_sessions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.user_sessions
    }

    fn mut_user_sessions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.user_sessions
    }

    // optional uint32 server_sessions = 2;

    pub fn clear_server_sessions(&mut self) {
        self.server_sessions = ::std::option::Option::None;
    }

    pub fn has_server_sessions(&self) -> bool {
        self.server_sessions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_sessions(&mut self, v: u32) {
        self.server_sessions = ::std::option::Option::Some(v);
    }

    pub fn get_server_sessions(&self) -> u32 {
        self.server_sessions.unwrap_or(0)
    }

    fn get_server_sessions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_sessions
    }

    fn mut_server_sessions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_sessions
    }

    // optional bool in_logon_surge = 3;

    pub fn clear_in_logon_surge(&mut self) {
        self.in_logon_surge = ::std::option::Option::None;
    }

    pub fn has_in_logon_surge(&self) -> bool {
        self.in_logon_surge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_logon_surge(&mut self, v: bool) {
        self.in_logon_surge = ::std::option::Option::Some(v);
    }

    pub fn get_in_logon_surge(&self) -> bool {
        self.in_logon_surge.unwrap_or(false)
    }

    fn get_in_logon_surge_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.in_logon_surge
    }

    fn mut_in_logon_surge_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.in_logon_surge
    }
}

impl ::protobuf::Message for CMsgGCToGCUpdateSessionStats {
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
                    self.user_sessions = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.server_sessions = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.in_logon_surge = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.user_sessions {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.server_sessions {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.in_logon_surge {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.user_sessions {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.server_sessions {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.in_logon_surge {
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

impl ::protobuf::MessageStatic for CMsgGCToGCUpdateSessionStats {
    fn new() -> CMsgGCToGCUpdateSessionStats {
        CMsgGCToGCUpdateSessionStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToGCUpdateSessionStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "user_sessions",
                    CMsgGCToGCUpdateSessionStats::get_user_sessions_for_reflect,
                    CMsgGCToGCUpdateSessionStats::mut_user_sessions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_sessions",
                    CMsgGCToGCUpdateSessionStats::get_server_sessions_for_reflect,
                    CMsgGCToGCUpdateSessionStats::mut_server_sessions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "in_logon_surge",
                    CMsgGCToGCUpdateSessionStats::get_in_logon_surge_for_reflect,
                    CMsgGCToGCUpdateSessionStats::mut_in_logon_surge_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToGCUpdateSessionStats>(
                    "CMsgGCToGCUpdateSessionStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToGCUpdateSessionStats {
    fn clear(&mut self) {
        self.clear_user_sessions();
        self.clear_server_sessions();
        self.clear_in_logon_surge();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToGCUpdateSessionStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToGCUpdateSessionStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_PopulateItemDescriptions_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    languages: ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_PopulateItemDescriptions_Request {}

impl CWorkshop_PopulateItemDescriptions_Request {
    pub fn new() -> CWorkshop_PopulateItemDescriptions_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_PopulateItemDescriptions_Request {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_PopulateItemDescriptions_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_PopulateItemDescriptions_Request,
        };
        unsafe {
            instance.get(CWorkshop_PopulateItemDescriptions_Request::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // repeated .dota.CWorkshop_PopulateItemDescriptions_Request.ItemDescriptionsLanguageBlock languages = 2;

    pub fn clear_languages(&mut self) {
        self.languages.clear();
    }

    // Param is passed by value, moved
    pub fn set_languages(&mut self, v: ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock>) {
        self.languages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_languages(&mut self) -> &mut ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock> {
        &mut self.languages
    }

    // Take field
    pub fn take_languages(&mut self) -> ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock> {
        ::std::mem::replace(&mut self.languages, ::protobuf::RepeatedField::new())
    }

    pub fn get_languages(&self) -> &[CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock] {
        &self.languages
    }

    fn get_languages_for_reflect(&self) -> &::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock> {
        &self.languages
    }

    fn mut_languages_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock> {
        &mut self.languages
    }
}

impl ::protobuf::Message for CWorkshop_PopulateItemDescriptions_Request {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.languages)?;
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.languages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        };
        for v in &self.languages {
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

impl ::protobuf::MessageStatic for CWorkshop_PopulateItemDescriptions_Request {
    fn new() -> CWorkshop_PopulateItemDescriptions_Request {
        CWorkshop_PopulateItemDescriptions_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_PopulateItemDescriptions_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CWorkshop_PopulateItemDescriptions_Request::get_appid_for_reflect,
                    CWorkshop_PopulateItemDescriptions_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock>>(
                    "languages",
                    CWorkshop_PopulateItemDescriptions_Request::get_languages_for_reflect,
                    CWorkshop_PopulateItemDescriptions_Request::mut_languages_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_PopulateItemDescriptions_Request>(
                    "CWorkshop_PopulateItemDescriptions_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_PopulateItemDescriptions_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_languages();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_PopulateItemDescriptions_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_PopulateItemDescriptions_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
    // message fields
    gameitemid: ::std::option::Option<u32>,
    item_description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {}

impl CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
    pub fn new() -> CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription,
        };
        unsafe {
            instance.get(CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription::new)
        }
    }

    // optional uint32 gameitemid = 1;

    pub fn clear_gameitemid(&mut self) {
        self.gameitemid = ::std::option::Option::None;
    }

    pub fn has_gameitemid(&self) -> bool {
        self.gameitemid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameitemid(&mut self, v: u32) {
        self.gameitemid = ::std::option::Option::Some(v);
    }

    pub fn get_gameitemid(&self) -> u32 {
        self.gameitemid.unwrap_or(0)
    }

    fn get_gameitemid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gameitemid
    }

    fn mut_gameitemid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gameitemid
    }

    // optional string item_description = 2;

    pub fn clear_item_description(&mut self) {
        self.item_description.clear();
    }

    pub fn has_item_description(&self) -> bool {
        self.item_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_description(&mut self, v: ::std::string::String) {
        self.item_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_item_description(&mut self) -> &mut ::std::string::String {
        if self.item_description.is_none() {
            self.item_description.set_default();
        };
        self.item_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_item_description(&mut self) -> ::std::string::String {
        self.item_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_item_description(&self) -> &str {
        match self.item_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_item_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.item_description
    }

    fn mut_item_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.item_description
    }
}

impl ::protobuf::Message for CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
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
                    self.gameitemid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.item_description)?;
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
        if let Some(v) = self.gameitemid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.item_description.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gameitemid {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.item_description.as_ref() {
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

impl ::protobuf::MessageStatic for CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
    fn new() -> CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
        CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gameitemid",
                    CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription::get_gameitemid_for_reflect,
                    CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription::mut_gameitemid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "item_description",
                    CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription::get_item_description_for_reflect,
                    CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription::mut_item_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription>(
                    "CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
    fn clear(&mut self) {
        self.clear_gameitemid();
        self.clear_item_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
    // message fields
    language: ::protobuf::SingularField<::std::string::String>,
    descriptions: ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {}

impl CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
    pub fn new() -> CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock,
        };
        unsafe {
            instance.get(CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock::new)
        }
    }

    // optional string language = 1;

    pub fn clear_language(&mut self) {
        self.language.clear();
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: ::std::string::String) {
        self.language = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_language(&mut self) -> &mut ::std::string::String {
        if self.language.is_none() {
            self.language.set_default();
        };
        self.language.as_mut().unwrap()
    }

    // Take field
    pub fn take_language(&mut self) -> ::std::string::String {
        self.language.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_language(&self) -> &str {
        match self.language.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_language_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.language
    }

    fn mut_language_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.language
    }

    // repeated .dota.CWorkshop_PopulateItemDescriptions_Request.SingleItemDescription descriptions = 2;

    pub fn clear_descriptions(&mut self) {
        self.descriptions.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptions(&mut self, v: ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription>) {
        self.descriptions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_descriptions(&mut self) -> &mut ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription> {
        &mut self.descriptions
    }

    // Take field
    pub fn take_descriptions(&mut self) -> ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription> {
        ::std::mem::replace(&mut self.descriptions, ::protobuf::RepeatedField::new())
    }

    pub fn get_descriptions(&self) -> &[CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription] {
        &self.descriptions
    }

    fn get_descriptions_for_reflect(&self) -> &::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription> {
        &self.descriptions
    }

    fn mut_descriptions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription> {
        &mut self.descriptions
    }
}

impl ::protobuf::Message for CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.language)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.descriptions)?;
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
        if let Some(v) = self.language.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        for value in &self.descriptions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.language.as_ref() {
            os.write_string(1, &v)?;
        };
        for v in &self.descriptions {
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

impl ::protobuf::MessageStatic for CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
    fn new() -> CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
        CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "language",
                    CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock::get_language_for_reflect,
                    CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock::mut_language_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CWorkshop_PopulateItemDescriptions_Request_SingleItemDescription>>(
                    "descriptions",
                    CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock::get_descriptions_for_reflect,
                    CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock::mut_descriptions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock>(
                    "CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
    fn clear(&mut self) {
        self.clear_language();
        self.clear_descriptions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_PopulateItemDescriptions_Request_ItemDescriptionsLanguageBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_GetContributors_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    gameitemid: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_GetContributors_Request {}

impl CWorkshop_GetContributors_Request {
    pub fn new() -> CWorkshop_GetContributors_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_GetContributors_Request {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_GetContributors_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_GetContributors_Request,
        };
        unsafe {
            instance.get(CWorkshop_GetContributors_Request::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // optional uint32 gameitemid = 2;

    pub fn clear_gameitemid(&mut self) {
        self.gameitemid = ::std::option::Option::None;
    }

    pub fn has_gameitemid(&self) -> bool {
        self.gameitemid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameitemid(&mut self, v: u32) {
        self.gameitemid = ::std::option::Option::Some(v);
    }

    pub fn get_gameitemid(&self) -> u32 {
        self.gameitemid.unwrap_or(0)
    }

    fn get_gameitemid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gameitemid
    }

    fn mut_gameitemid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gameitemid
    }
}

impl ::protobuf::Message for CWorkshop_GetContributors_Request {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.gameitemid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gameitemid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.gameitemid {
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

impl ::protobuf::MessageStatic for CWorkshop_GetContributors_Request {
    fn new() -> CWorkshop_GetContributors_Request {
        CWorkshop_GetContributors_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_GetContributors_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CWorkshop_GetContributors_Request::get_appid_for_reflect,
                    CWorkshop_GetContributors_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gameitemid",
                    CWorkshop_GetContributors_Request::get_gameitemid_for_reflect,
                    CWorkshop_GetContributors_Request::mut_gameitemid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_GetContributors_Request>(
                    "CWorkshop_GetContributors_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_GetContributors_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_gameitemid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_GetContributors_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_GetContributors_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_GetContributors_Response {
    // message fields
    contributors: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_GetContributors_Response {}

impl CWorkshop_GetContributors_Response {
    pub fn new() -> CWorkshop_GetContributors_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_GetContributors_Response {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_GetContributors_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_GetContributors_Response,
        };
        unsafe {
            instance.get(CWorkshop_GetContributors_Response::new)
        }
    }

    // repeated fixed64 contributors = 1;

    pub fn clear_contributors(&mut self) {
        self.contributors.clear();
    }

    // Param is passed by value, moved
    pub fn set_contributors(&mut self, v: ::std::vec::Vec<u64>) {
        self.contributors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_contributors(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.contributors
    }

    // Take field
    pub fn take_contributors(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.contributors, ::std::vec::Vec::new())
    }

    pub fn get_contributors(&self) -> &[u64] {
        &self.contributors
    }

    fn get_contributors_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.contributors
    }

    fn mut_contributors_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.contributors
    }
}

impl ::protobuf::Message for CWorkshop_GetContributors_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.contributors)?;
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
        my_size += 9 * self.contributors.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.contributors {
            os.write_fixed64(1, *v)?;
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

impl ::protobuf::MessageStatic for CWorkshop_GetContributors_Response {
    fn new() -> CWorkshop_GetContributors_Response {
        CWorkshop_GetContributors_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_GetContributors_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "contributors",
                    CWorkshop_GetContributors_Response::get_contributors_for_reflect,
                    CWorkshop_GetContributors_Response::mut_contributors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_GetContributors_Response>(
                    "CWorkshop_GetContributors_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_GetContributors_Response {
    fn clear(&mut self) {
        self.clear_contributors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_GetContributors_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_GetContributors_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_SetItemPaymentRules_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    gameitemid: ::std::option::Option<u32>,
    associated_workshop_files: ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule>,
    partner_accounts: ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_SetItemPaymentRules_Request {}

impl CWorkshop_SetItemPaymentRules_Request {
    pub fn new() -> CWorkshop_SetItemPaymentRules_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_SetItemPaymentRules_Request {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_SetItemPaymentRules_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_SetItemPaymentRules_Request,
        };
        unsafe {
            instance.get(CWorkshop_SetItemPaymentRules_Request::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // optional uint32 gameitemid = 2;

    pub fn clear_gameitemid(&mut self) {
        self.gameitemid = ::std::option::Option::None;
    }

    pub fn has_gameitemid(&self) -> bool {
        self.gameitemid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameitemid(&mut self, v: u32) {
        self.gameitemid = ::std::option::Option::Some(v);
    }

    pub fn get_gameitemid(&self) -> u32 {
        self.gameitemid.unwrap_or(0)
    }

    fn get_gameitemid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gameitemid
    }

    fn mut_gameitemid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gameitemid
    }

    // repeated .dota.CWorkshop_SetItemPaymentRules_Request.WorkshopItemPaymentRule associated_workshop_files = 3;

    pub fn clear_associated_workshop_files(&mut self) {
        self.associated_workshop_files.clear();
    }

    // Param is passed by value, moved
    pub fn set_associated_workshop_files(&mut self, v: ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule>) {
        self.associated_workshop_files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_associated_workshop_files(&mut self) -> &mut ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule> {
        &mut self.associated_workshop_files
    }

    // Take field
    pub fn take_associated_workshop_files(&mut self) -> ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule> {
        ::std::mem::replace(&mut self.associated_workshop_files, ::protobuf::RepeatedField::new())
    }

    pub fn get_associated_workshop_files(&self) -> &[CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule] {
        &self.associated_workshop_files
    }

    fn get_associated_workshop_files_for_reflect(&self) -> &::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule> {
        &self.associated_workshop_files
    }

    fn mut_associated_workshop_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule> {
        &mut self.associated_workshop_files
    }

    // repeated .dota.CWorkshop_SetItemPaymentRules_Request.PartnerItemPaymentRule partner_accounts = 4;

    pub fn clear_partner_accounts(&mut self) {
        self.partner_accounts.clear();
    }

    // Param is passed by value, moved
    pub fn set_partner_accounts(&mut self, v: ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule>) {
        self.partner_accounts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_partner_accounts(&mut self) -> &mut ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule> {
        &mut self.partner_accounts
    }

    // Take field
    pub fn take_partner_accounts(&mut self) -> ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule> {
        ::std::mem::replace(&mut self.partner_accounts, ::protobuf::RepeatedField::new())
    }

    pub fn get_partner_accounts(&self) -> &[CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule] {
        &self.partner_accounts
    }

    fn get_partner_accounts_for_reflect(&self) -> &::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule> {
        &self.partner_accounts
    }

    fn mut_partner_accounts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule> {
        &mut self.partner_accounts
    }
}

impl ::protobuf::Message for CWorkshop_SetItemPaymentRules_Request {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.gameitemid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.associated_workshop_files)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.partner_accounts)?;
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gameitemid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.associated_workshop_files {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.partner_accounts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.gameitemid {
            os.write_uint32(2, v)?;
        };
        for v in &self.associated_workshop_files {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.partner_accounts {
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

impl ::protobuf::MessageStatic for CWorkshop_SetItemPaymentRules_Request {
    fn new() -> CWorkshop_SetItemPaymentRules_Request {
        CWorkshop_SetItemPaymentRules_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_SetItemPaymentRules_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CWorkshop_SetItemPaymentRules_Request::get_appid_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gameitemid",
                    CWorkshop_SetItemPaymentRules_Request::get_gameitemid_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request::mut_gameitemid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule>>(
                    "associated_workshop_files",
                    CWorkshop_SetItemPaymentRules_Request::get_associated_workshop_files_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request::mut_associated_workshop_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule>>(
                    "partner_accounts",
                    CWorkshop_SetItemPaymentRules_Request::get_partner_accounts_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request::mut_partner_accounts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_SetItemPaymentRules_Request>(
                    "CWorkshop_SetItemPaymentRules_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_SetItemPaymentRules_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_gameitemid();
        self.clear_associated_workshop_files();
        self.clear_partner_accounts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_SetItemPaymentRules_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_SetItemPaymentRules_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
    // message fields
    workshop_file_id: ::std::option::Option<u64>,
    revenue_percentage: ::std::option::Option<f32>,
    rule_description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {}

impl CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
    pub fn new() -> CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule,
        };
        unsafe {
            instance.get(CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule::new)
        }
    }

    // optional uint64 workshop_file_id = 1;

    pub fn clear_workshop_file_id(&mut self) {
        self.workshop_file_id = ::std::option::Option::None;
    }

    pub fn has_workshop_file_id(&self) -> bool {
        self.workshop_file_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_workshop_file_id(&mut self, v: u64) {
        self.workshop_file_id = ::std::option::Option::Some(v);
    }

    pub fn get_workshop_file_id(&self) -> u64 {
        self.workshop_file_id.unwrap_or(0)
    }

    fn get_workshop_file_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.workshop_file_id
    }

    fn mut_workshop_file_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.workshop_file_id
    }

    // optional float revenue_percentage = 2;

    pub fn clear_revenue_percentage(&mut self) {
        self.revenue_percentage = ::std::option::Option::None;
    }

    pub fn has_revenue_percentage(&self) -> bool {
        self.revenue_percentage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revenue_percentage(&mut self, v: f32) {
        self.revenue_percentage = ::std::option::Option::Some(v);
    }

    pub fn get_revenue_percentage(&self) -> f32 {
        self.revenue_percentage.unwrap_or(0.)
    }

    fn get_revenue_percentage_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.revenue_percentage
    }

    fn mut_revenue_percentage_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.revenue_percentage
    }

    // optional string rule_description = 3;

    pub fn clear_rule_description(&mut self) {
        self.rule_description.clear();
    }

    pub fn has_rule_description(&self) -> bool {
        self.rule_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rule_description(&mut self, v: ::std::string::String) {
        self.rule_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rule_description(&mut self) -> &mut ::std::string::String {
        if self.rule_description.is_none() {
            self.rule_description.set_default();
        };
        self.rule_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_rule_description(&mut self) -> ::std::string::String {
        self.rule_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rule_description(&self) -> &str {
        match self.rule_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rule_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rule_description
    }

    fn mut_rule_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rule_description
    }
}

impl ::protobuf::Message for CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
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
                    self.workshop_file_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.revenue_percentage = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rule_description)?;
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
        if let Some(v) = self.workshop_file_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.revenue_percentage {
            my_size += 5;
        };
        if let Some(v) = self.rule_description.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.workshop_file_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.revenue_percentage {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.rule_description.as_ref() {
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

impl ::protobuf::MessageStatic for CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
    fn new() -> CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
        CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "workshop_file_id",
                    CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule::get_workshop_file_id_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule::mut_workshop_file_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "revenue_percentage",
                    CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule::get_revenue_percentage_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule::mut_revenue_percentage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rule_description",
                    CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule::get_rule_description_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule::mut_rule_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule>(
                    "CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
    fn clear(&mut self) {
        self.clear_workshop_file_id();
        self.clear_revenue_percentage();
        self.clear_rule_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_SetItemPaymentRules_Request_WorkshopItemPaymentRule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
    // message fields
    account_id: ::std::option::Option<u32>,
    revenue_percentage: ::std::option::Option<f32>,
    rule_description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {}

impl CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
    pub fn new() -> CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule,
        };
        unsafe {
            instance.get(CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule::new)
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

    // optional float revenue_percentage = 2;

    pub fn clear_revenue_percentage(&mut self) {
        self.revenue_percentage = ::std::option::Option::None;
    }

    pub fn has_revenue_percentage(&self) -> bool {
        self.revenue_percentage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revenue_percentage(&mut self, v: f32) {
        self.revenue_percentage = ::std::option::Option::Some(v);
    }

    pub fn get_revenue_percentage(&self) -> f32 {
        self.revenue_percentage.unwrap_or(0.)
    }

    fn get_revenue_percentage_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.revenue_percentage
    }

    fn mut_revenue_percentage_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.revenue_percentage
    }

    // optional string rule_description = 3;

    pub fn clear_rule_description(&mut self) {
        self.rule_description.clear();
    }

    pub fn has_rule_description(&self) -> bool {
        self.rule_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rule_description(&mut self, v: ::std::string::String) {
        self.rule_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rule_description(&mut self) -> &mut ::std::string::String {
        if self.rule_description.is_none() {
            self.rule_description.set_default();
        };
        self.rule_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_rule_description(&mut self) -> ::std::string::String {
        self.rule_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rule_description(&self) -> &str {
        match self.rule_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rule_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rule_description
    }

    fn mut_rule_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rule_description
    }
}

impl ::protobuf::Message for CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.revenue_percentage = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rule_description)?;
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
        if let Some(v) = self.revenue_percentage {
            my_size += 5;
        };
        if let Some(v) = self.rule_description.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.revenue_percentage {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.rule_description.as_ref() {
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

impl ::protobuf::MessageStatic for CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
    fn new() -> CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
        CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule::get_account_id_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "revenue_percentage",
                    CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule::get_revenue_percentage_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule::mut_revenue_percentage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rule_description",
                    CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule::get_rule_description_for_reflect,
                    CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule::mut_rule_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule>(
                    "CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_revenue_percentage();
        self.clear_rule_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_SetItemPaymentRules_Request_PartnerItemPaymentRule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CWorkshop_SetItemPaymentRules_Response {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CWorkshop_SetItemPaymentRules_Response {}

impl CWorkshop_SetItemPaymentRules_Response {
    pub fn new() -> CWorkshop_SetItemPaymentRules_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CWorkshop_SetItemPaymentRules_Response {
        static mut instance: ::protobuf::lazy::Lazy<CWorkshop_SetItemPaymentRules_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CWorkshop_SetItemPaymentRules_Response,
        };
        unsafe {
            instance.get(CWorkshop_SetItemPaymentRules_Response::new)
        }
    }
}

impl ::protobuf::Message for CWorkshop_SetItemPaymentRules_Response {
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

impl ::protobuf::MessageStatic for CWorkshop_SetItemPaymentRules_Response {
    fn new() -> CWorkshop_SetItemPaymentRules_Response {
        CWorkshop_SetItemPaymentRules_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CWorkshop_SetItemPaymentRules_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CWorkshop_SetItemPaymentRules_Response>(
                    "CWorkshop_SetItemPaymentRules_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CWorkshop_SetItemPaymentRules_Response {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CWorkshop_SetItemPaymentRules_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CWorkshop_SetItemPaymentRules_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CBroadcast_PostGameDataFrame_Request {
    // message fields
    appid: ::std::option::Option<u32>,
    steamid: ::std::option::Option<u64>,
    broadcast_id: ::std::option::Option<u64>,
    frame_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CBroadcast_PostGameDataFrame_Request {}

impl CBroadcast_PostGameDataFrame_Request {
    pub fn new() -> CBroadcast_PostGameDataFrame_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CBroadcast_PostGameDataFrame_Request {
        static mut instance: ::protobuf::lazy::Lazy<CBroadcast_PostGameDataFrame_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CBroadcast_PostGameDataFrame_Request,
        };
        unsafe {
            instance.get(CBroadcast_PostGameDataFrame_Request::new)
        }
    }

    // optional uint32 appid = 1;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
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

    // optional fixed64 broadcast_id = 3;

    pub fn clear_broadcast_id(&mut self) {
        self.broadcast_id = ::std::option::Option::None;
    }

    pub fn has_broadcast_id(&self) -> bool {
        self.broadcast_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcast_id(&mut self, v: u64) {
        self.broadcast_id = ::std::option::Option::Some(v);
    }

    pub fn get_broadcast_id(&self) -> u64 {
        self.broadcast_id.unwrap_or(0)
    }

    fn get_broadcast_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.broadcast_id
    }

    fn mut_broadcast_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.broadcast_id
    }

    // optional bytes frame_data = 4;

    pub fn clear_frame_data(&mut self) {
        self.frame_data.clear();
    }

    pub fn has_frame_data(&self) -> bool {
        self.frame_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frame_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.frame_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_frame_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.frame_data.is_none() {
            self.frame_data.set_default();
        };
        self.frame_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_frame_data(&mut self) -> ::std::vec::Vec<u8> {
        self.frame_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_frame_data(&self) -> &[u8] {
        match self.frame_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_frame_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.frame_data
    }

    fn mut_frame_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.frame_data
    }
}

impl ::protobuf::Message for CBroadcast_PostGameDataFrame_Request {
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
                    self.appid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.broadcast_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.frame_data)?;
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
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.steamid {
            my_size += 9;
        };
        if let Some(v) = self.broadcast_id {
            my_size += 9;
        };
        if let Some(v) = self.frame_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.appid {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.steamid {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.broadcast_id {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.frame_data.as_ref() {
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

impl ::protobuf::MessageStatic for CBroadcast_PostGameDataFrame_Request {
    fn new() -> CBroadcast_PostGameDataFrame_Request {
        CBroadcast_PostGameDataFrame_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CBroadcast_PostGameDataFrame_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CBroadcast_PostGameDataFrame_Request::get_appid_for_reflect,
                    CBroadcast_PostGameDataFrame_Request::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CBroadcast_PostGameDataFrame_Request::get_steamid_for_reflect,
                    CBroadcast_PostGameDataFrame_Request::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "broadcast_id",
                    CBroadcast_PostGameDataFrame_Request::get_broadcast_id_for_reflect,
                    CBroadcast_PostGameDataFrame_Request::mut_broadcast_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "frame_data",
                    CBroadcast_PostGameDataFrame_Request::get_frame_data_for_reflect,
                    CBroadcast_PostGameDataFrame_Request::mut_frame_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CBroadcast_PostGameDataFrame_Request>(
                    "CBroadcast_PostGameDataFrame_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CBroadcast_PostGameDataFrame_Request {
    fn clear(&mut self) {
        self.clear_appid();
        self.clear_steamid();
        self.clear_broadcast_id();
        self.clear_frame_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CBroadcast_PostGameDataFrame_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CBroadcast_PostGameDataFrame_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSerializedSOCache {
    // message fields
    file_version: ::std::option::Option<u32>,
    caches: ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache>,
    gc_socache_file_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSerializedSOCache {}

impl CMsgSerializedSOCache {
    pub fn new() -> CMsgSerializedSOCache {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSerializedSOCache {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSerializedSOCache> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSerializedSOCache,
        };
        unsafe {
            instance.get(CMsgSerializedSOCache::new)
        }
    }

    // optional uint32 file_version = 1;

    pub fn clear_file_version(&mut self) {
        self.file_version = ::std::option::Option::None;
    }

    pub fn has_file_version(&self) -> bool {
        self.file_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_version(&mut self, v: u32) {
        self.file_version = ::std::option::Option::Some(v);
    }

    pub fn get_file_version(&self) -> u32 {
        self.file_version.unwrap_or(0)
    }

    fn get_file_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.file_version
    }

    fn mut_file_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.file_version
    }

    // repeated .dota.CMsgSerializedSOCache.Cache caches = 2;

    pub fn clear_caches(&mut self) {
        self.caches.clear();
    }

    // Param is passed by value, moved
    pub fn set_caches(&mut self, v: ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache>) {
        self.caches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_caches(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache> {
        &mut self.caches
    }

    // Take field
    pub fn take_caches(&mut self) -> ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache> {
        ::std::mem::replace(&mut self.caches, ::protobuf::RepeatedField::new())
    }

    pub fn get_caches(&self) -> &[CMsgSerializedSOCache_Cache] {
        &self.caches
    }

    fn get_caches_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSerializedSOCache_Cache> {
        &self.caches
    }

    fn mut_caches_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache> {
        &mut self.caches
    }

    // optional uint32 gc_socache_file_version = 3;

    pub fn clear_gc_socache_file_version(&mut self) {
        self.gc_socache_file_version = ::std::option::Option::None;
    }

    pub fn has_gc_socache_file_version(&self) -> bool {
        self.gc_socache_file_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_socache_file_version(&mut self, v: u32) {
        self.gc_socache_file_version = ::std::option::Option::Some(v);
    }

    pub fn get_gc_socache_file_version(&self) -> u32 {
        self.gc_socache_file_version.unwrap_or(0)
    }

    fn get_gc_socache_file_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gc_socache_file_version
    }

    fn mut_gc_socache_file_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gc_socache_file_version
    }
}

impl ::protobuf::Message for CMsgSerializedSOCache {
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
                    self.file_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.caches)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.gc_socache_file_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.file_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.caches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.gc_socache_file_version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.file_version {
            os.write_uint32(1, v)?;
        };
        for v in &self.caches {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.gc_socache_file_version {
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

impl ::protobuf::MessageStatic for CMsgSerializedSOCache {
    fn new() -> CMsgSerializedSOCache {
        CMsgSerializedSOCache::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSerializedSOCache>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "file_version",
                    CMsgSerializedSOCache::get_file_version_for_reflect,
                    CMsgSerializedSOCache::mut_file_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSerializedSOCache_Cache>>(
                    "caches",
                    CMsgSerializedSOCache::get_caches_for_reflect,
                    CMsgSerializedSOCache::mut_caches_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gc_socache_file_version",
                    CMsgSerializedSOCache::get_gc_socache_file_version_for_reflect,
                    CMsgSerializedSOCache::mut_gc_socache_file_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSerializedSOCache>(
                    "CMsgSerializedSOCache",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSerializedSOCache {
    fn clear(&mut self) {
        self.clear_file_version();
        self.clear_caches();
        self.clear_gc_socache_file_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSerializedSOCache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSerializedSOCache {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSerializedSOCache_TypeCache {
    // message fields
    field_type: ::std::option::Option<u32>,
    objects: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    service_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSerializedSOCache_TypeCache {}

impl CMsgSerializedSOCache_TypeCache {
    pub fn new() -> CMsgSerializedSOCache_TypeCache {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSerializedSOCache_TypeCache {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSerializedSOCache_TypeCache> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSerializedSOCache_TypeCache,
        };
        unsafe {
            instance.get(CMsgSerializedSOCache_TypeCache::new)
        }
    }

    // optional uint32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: u32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> u32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.field_type
    }

    // repeated bytes objects = 2;

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    // Param is passed by value, moved
    pub fn set_objects(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.objects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_objects(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.objects
    }

    // Take field
    pub fn take_objects(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.objects, ::protobuf::RepeatedField::new())
    }

    pub fn get_objects(&self) -> &[::std::vec::Vec<u8>] {
        &self.objects
    }

    fn get_objects_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.objects
    }

    fn mut_objects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.objects
    }

    // optional uint32 service_id = 3;

    pub fn clear_service_id(&mut self) {
        self.service_id = ::std::option::Option::None;
    }

    pub fn has_service_id(&self) -> bool {
        self.service_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service_id(&mut self, v: u32) {
        self.service_id = ::std::option::Option::Some(v);
    }

    pub fn get_service_id(&self) -> u32 {
        self.service_id.unwrap_or(0)
    }

    fn get_service_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service_id
    }

    fn mut_service_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service_id
    }
}

impl ::protobuf::Message for CMsgSerializedSOCache_TypeCache {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.objects)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.service_id = ::std::option::Option::Some(tmp);
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
        for value in &self.objects {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if let Some(v) = self.service_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_uint32(1, v)?;
        };
        for v in &self.objects {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.service_id {
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

impl ::protobuf::MessageStatic for CMsgSerializedSOCache_TypeCache {
    fn new() -> CMsgSerializedSOCache_TypeCache {
        CMsgSerializedSOCache_TypeCache::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSerializedSOCache_TypeCache>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "type",
                    CMsgSerializedSOCache_TypeCache::get_field_type_for_reflect,
                    CMsgSerializedSOCache_TypeCache::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "objects",
                    CMsgSerializedSOCache_TypeCache::get_objects_for_reflect,
                    CMsgSerializedSOCache_TypeCache::mut_objects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service_id",
                    CMsgSerializedSOCache_TypeCache::get_service_id_for_reflect,
                    CMsgSerializedSOCache_TypeCache::mut_service_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSerializedSOCache_TypeCache>(
                    "CMsgSerializedSOCache_TypeCache",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSerializedSOCache_TypeCache {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_objects();
        self.clear_service_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSerializedSOCache_TypeCache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSerializedSOCache_TypeCache {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSerializedSOCache_Cache {
    // message fields
    field_type: ::std::option::Option<u32>,
    id: ::std::option::Option<u64>,
    versions: ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache_Version>,
    type_caches: ::protobuf::RepeatedField<CMsgSerializedSOCache_TypeCache>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSerializedSOCache_Cache {}

impl CMsgSerializedSOCache_Cache {
    pub fn new() -> CMsgSerializedSOCache_Cache {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSerializedSOCache_Cache {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSerializedSOCache_Cache> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSerializedSOCache_Cache,
        };
        unsafe {
            instance.get(CMsgSerializedSOCache_Cache::new)
        }
    }

    // optional uint32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: u32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> u32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.field_type
    }

    // optional uint64 id = 2;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // repeated .dota.CMsgSerializedSOCache.Cache.Version versions = 3;

    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }

    // Param is passed by value, moved
    pub fn set_versions(&mut self, v: ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache_Version>) {
        self.versions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_versions(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache_Version> {
        &mut self.versions
    }

    // Take field
    pub fn take_versions(&mut self) -> ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache_Version> {
        ::std::mem::replace(&mut self.versions, ::protobuf::RepeatedField::new())
    }

    pub fn get_versions(&self) -> &[CMsgSerializedSOCache_Cache_Version] {
        &self.versions
    }

    fn get_versions_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSerializedSOCache_Cache_Version> {
        &self.versions
    }

    fn mut_versions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSerializedSOCache_Cache_Version> {
        &mut self.versions
    }

    // repeated .dota.CMsgSerializedSOCache.TypeCache type_caches = 4;

    pub fn clear_type_caches(&mut self) {
        self.type_caches.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_caches(&mut self, v: ::protobuf::RepeatedField<CMsgSerializedSOCache_TypeCache>) {
        self.type_caches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_type_caches(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSerializedSOCache_TypeCache> {
        &mut self.type_caches
    }

    // Take field
    pub fn take_type_caches(&mut self) -> ::protobuf::RepeatedField<CMsgSerializedSOCache_TypeCache> {
        ::std::mem::replace(&mut self.type_caches, ::protobuf::RepeatedField::new())
    }

    pub fn get_type_caches(&self) -> &[CMsgSerializedSOCache_TypeCache] {
        &self.type_caches
    }

    fn get_type_caches_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSerializedSOCache_TypeCache> {
        &self.type_caches
    }

    fn mut_type_caches_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSerializedSOCache_TypeCache> {
        &mut self.type_caches
    }
}

impl ::protobuf::Message for CMsgSerializedSOCache_Cache {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.versions)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.type_caches)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.versions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.type_caches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.id {
            os.write_uint64(2, v)?;
        };
        for v in &self.versions {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.type_caches {
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

impl ::protobuf::MessageStatic for CMsgSerializedSOCache_Cache {
    fn new() -> CMsgSerializedSOCache_Cache {
        CMsgSerializedSOCache_Cache::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSerializedSOCache_Cache>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "type",
                    CMsgSerializedSOCache_Cache::get_field_type_for_reflect,
                    CMsgSerializedSOCache_Cache::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CMsgSerializedSOCache_Cache::get_id_for_reflect,
                    CMsgSerializedSOCache_Cache::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSerializedSOCache_Cache_Version>>(
                    "versions",
                    CMsgSerializedSOCache_Cache::get_versions_for_reflect,
                    CMsgSerializedSOCache_Cache::mut_versions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSerializedSOCache_TypeCache>>(
                    "type_caches",
                    CMsgSerializedSOCache_Cache::get_type_caches_for_reflect,
                    CMsgSerializedSOCache_Cache::mut_type_caches_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSerializedSOCache_Cache>(
                    "CMsgSerializedSOCache_Cache",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSerializedSOCache_Cache {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.clear_versions();
        self.clear_type_caches();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSerializedSOCache_Cache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSerializedSOCache_Cache {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSerializedSOCache_Cache_Version {
    // message fields
    service: ::std::option::Option<u32>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSerializedSOCache_Cache_Version {}

impl CMsgSerializedSOCache_Cache_Version {
    pub fn new() -> CMsgSerializedSOCache_Cache_Version {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSerializedSOCache_Cache_Version {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSerializedSOCache_Cache_Version> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSerializedSOCache_Cache_Version,
        };
        unsafe {
            instance.get(CMsgSerializedSOCache_Cache_Version::new)
        }
    }

    // optional uint32 service = 1;

    pub fn clear_service(&mut self) {
        self.service = ::std::option::Option::None;
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: u32) {
        self.service = ::std::option::Option::Some(v);
    }

    pub fn get_service(&self) -> u32 {
        self.service.unwrap_or(0)
    }

    fn get_service_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.service
    }

    // optional uint64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u64 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.version
    }
}

impl ::protobuf::Message for CMsgSerializedSOCache_Cache_Version {
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
                    self.service = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
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
        if let Some(v) = self.service {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.service {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.version {
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

impl ::protobuf::MessageStatic for CMsgSerializedSOCache_Cache_Version {
    fn new() -> CMsgSerializedSOCache_Cache_Version {
        CMsgSerializedSOCache_Cache_Version::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSerializedSOCache_Cache_Version>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "service",
                    CMsgSerializedSOCache_Cache_Version::get_service_for_reflect,
                    CMsgSerializedSOCache_Cache_Version::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    CMsgSerializedSOCache_Cache_Version::get_version_for_reflect,
                    CMsgSerializedSOCache_Cache_Version::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSerializedSOCache_Cache_Version>(
                    "CMsgSerializedSOCache_Cache_Version",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSerializedSOCache_Cache_Version {
    fn clear(&mut self) {
        self.clear_service();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSerializedSOCache_Cache_Version {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSerializedSOCache_Cache_Version {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientPollConvarRequest {
    // message fields
    convar_name: ::protobuf::SingularField<::std::string::String>,
    poll_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientPollConvarRequest {}

impl CMsgGCToClientPollConvarRequest {
    pub fn new() -> CMsgGCToClientPollConvarRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientPollConvarRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientPollConvarRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientPollConvarRequest,
        };
        unsafe {
            instance.get(CMsgGCToClientPollConvarRequest::new)
        }
    }

    // optional string convar_name = 1;

    pub fn clear_convar_name(&mut self) {
        self.convar_name.clear();
    }

    pub fn has_convar_name(&self) -> bool {
        self.convar_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_convar_name(&mut self, v: ::std::string::String) {
        self.convar_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_convar_name(&mut self) -> &mut ::std::string::String {
        if self.convar_name.is_none() {
            self.convar_name.set_default();
        };
        self.convar_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_convar_name(&mut self) -> ::std::string::String {
        self.convar_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_convar_name(&self) -> &str {
        match self.convar_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_convar_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.convar_name
    }

    fn mut_convar_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.convar_name
    }

    // optional uint32 poll_id = 2;

    pub fn clear_poll_id(&mut self) {
        self.poll_id = ::std::option::Option::None;
    }

    pub fn has_poll_id(&self) -> bool {
        self.poll_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poll_id(&mut self, v: u32) {
        self.poll_id = ::std::option::Option::Some(v);
    }

    pub fn get_poll_id(&self) -> u32 {
        self.poll_id.unwrap_or(0)
    }

    fn get_poll_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.poll_id
    }

    fn mut_poll_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.poll_id
    }
}

impl ::protobuf::Message for CMsgGCToClientPollConvarRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.convar_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.poll_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.convar_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.poll_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.convar_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.poll_id {
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

impl ::protobuf::MessageStatic for CMsgGCToClientPollConvarRequest {
    fn new() -> CMsgGCToClientPollConvarRequest {
        CMsgGCToClientPollConvarRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientPollConvarRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "convar_name",
                    CMsgGCToClientPollConvarRequest::get_convar_name_for_reflect,
                    CMsgGCToClientPollConvarRequest::mut_convar_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "poll_id",
                    CMsgGCToClientPollConvarRequest::get_poll_id_for_reflect,
                    CMsgGCToClientPollConvarRequest::mut_poll_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientPollConvarRequest>(
                    "CMsgGCToClientPollConvarRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientPollConvarRequest {
    fn clear(&mut self) {
        self.clear_convar_name();
        self.clear_poll_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientPollConvarRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientPollConvarRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientPollConvarResponse {
    // message fields
    poll_id: ::std::option::Option<u32>,
    convar_value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientPollConvarResponse {}

impl CMsgGCToClientPollConvarResponse {
    pub fn new() -> CMsgGCToClientPollConvarResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientPollConvarResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientPollConvarResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientPollConvarResponse,
        };
        unsafe {
            instance.get(CMsgGCToClientPollConvarResponse::new)
        }
    }

    // optional uint32 poll_id = 1;

    pub fn clear_poll_id(&mut self) {
        self.poll_id = ::std::option::Option::None;
    }

    pub fn has_poll_id(&self) -> bool {
        self.poll_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poll_id(&mut self, v: u32) {
        self.poll_id = ::std::option::Option::Some(v);
    }

    pub fn get_poll_id(&self) -> u32 {
        self.poll_id.unwrap_or(0)
    }

    fn get_poll_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.poll_id
    }

    fn mut_poll_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.poll_id
    }

    // optional string convar_value = 2;

    pub fn clear_convar_value(&mut self) {
        self.convar_value.clear();
    }

    pub fn has_convar_value(&self) -> bool {
        self.convar_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_convar_value(&mut self, v: ::std::string::String) {
        self.convar_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_convar_value(&mut self) -> &mut ::std::string::String {
        if self.convar_value.is_none() {
            self.convar_value.set_default();
        };
        self.convar_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_convar_value(&mut self) -> ::std::string::String {
        self.convar_value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_convar_value(&self) -> &str {
        match self.convar_value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_convar_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.convar_value
    }

    fn mut_convar_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.convar_value
    }
}

impl ::protobuf::Message for CMsgGCToClientPollConvarResponse {
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
                    self.poll_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.convar_value)?;
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
        if let Some(v) = self.poll_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.convar_value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.poll_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.convar_value.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgGCToClientPollConvarResponse {
    fn new() -> CMsgGCToClientPollConvarResponse {
        CMsgGCToClientPollConvarResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientPollConvarResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "poll_id",
                    CMsgGCToClientPollConvarResponse::get_poll_id_for_reflect,
                    CMsgGCToClientPollConvarResponse::mut_poll_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "convar_value",
                    CMsgGCToClientPollConvarResponse::get_convar_value_for_reflect,
                    CMsgGCToClientPollConvarResponse::mut_convar_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientPollConvarResponse>(
                    "CMsgGCToClientPollConvarResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientPollConvarResponse {
    fn clear(&mut self) {
        self.clear_poll_id();
        self.clear_convar_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientPollConvarResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientPollConvarResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ESourceEngine {
    k_ESE_Source1 = 0,
    k_ESE_Source2 = 1,
}

impl ::protobuf::ProtobufEnum for ESourceEngine {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ESourceEngine> {
        match value {
            0 => ::std::option::Option::Some(ESourceEngine::k_ESE_Source1),
            1 => ::std::option::Option::Some(ESourceEngine::k_ESE_Source2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ESourceEngine] = &[
            ESourceEngine::k_ESE_Source1,
            ESourceEngine::k_ESE_Source2,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ESourceEngine>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ESourceEngine", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ESourceEngine {
}

impl ::protobuf::reflect::ProtobufValue for ESourceEngine {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PartnerAccountType {
    PARTNER_NONE = 0,
    PARTNER_PERFECT_WORLD = 1,
    PARTNER_NEXON = 2,
    PARTNER_INVALID = 3,
}

impl ::protobuf::ProtobufEnum for PartnerAccountType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PartnerAccountType> {
        match value {
            0 => ::std::option::Option::Some(PartnerAccountType::PARTNER_NONE),
            1 => ::std::option::Option::Some(PartnerAccountType::PARTNER_PERFECT_WORLD),
            2 => ::std::option::Option::Some(PartnerAccountType::PARTNER_NEXON),
            3 => ::std::option::Option::Some(PartnerAccountType::PARTNER_INVALID),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PartnerAccountType] = &[
            PartnerAccountType::PARTNER_NONE,
            PartnerAccountType::PARTNER_PERFECT_WORLD,
            PartnerAccountType::PARTNER_NEXON,
            PartnerAccountType::PARTNER_INVALID,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<PartnerAccountType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PartnerAccountType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PartnerAccountType {
}

impl ::protobuf::reflect::ProtobufValue for PartnerAccountType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GCConnectionStatus {
    GCConnectionStatus_HAVE_SESSION = 0,
    GCConnectionStatus_GC_GOING_DOWN = 1,
    GCConnectionStatus_NO_SESSION = 2,
    GCConnectionStatus_NO_SESSION_IN_LOGON_QUEUE = 3,
    GCConnectionStatus_NO_STEAM = 4,
    GCConnectionStatus_SUSPENDED = 5,
    GCConnectionStatus_STEAM_GOING_DOWN = 6,
}

impl ::protobuf::ProtobufEnum for GCConnectionStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GCConnectionStatus> {
        match value {
            0 => ::std::option::Option::Some(GCConnectionStatus::GCConnectionStatus_HAVE_SESSION),
            1 => ::std::option::Option::Some(GCConnectionStatus::GCConnectionStatus_GC_GOING_DOWN),
            2 => ::std::option::Option::Some(GCConnectionStatus::GCConnectionStatus_NO_SESSION),
            3 => ::std::option::Option::Some(GCConnectionStatus::GCConnectionStatus_NO_SESSION_IN_LOGON_QUEUE),
            4 => ::std::option::Option::Some(GCConnectionStatus::GCConnectionStatus_NO_STEAM),
            5 => ::std::option::Option::Some(GCConnectionStatus::GCConnectionStatus_SUSPENDED),
            6 => ::std::option::Option::Some(GCConnectionStatus::GCConnectionStatus_STEAM_GOING_DOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GCConnectionStatus] = &[
            GCConnectionStatus::GCConnectionStatus_HAVE_SESSION,
            GCConnectionStatus::GCConnectionStatus_GC_GOING_DOWN,
            GCConnectionStatus::GCConnectionStatus_NO_SESSION,
            GCConnectionStatus::GCConnectionStatus_NO_SESSION_IN_LOGON_QUEUE,
            GCConnectionStatus::GCConnectionStatus_NO_STEAM,
            GCConnectionStatus::GCConnectionStatus_SUSPENDED,
            GCConnectionStatus::GCConnectionStatus_STEAM_GOING_DOWN,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<GCConnectionStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GCConnectionStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GCConnectionStatus {
}

impl ::protobuf::reflect::ProtobufValue for GCConnectionStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x16, 0x67, 0x63, 0x73, 0x64, 0x6b, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x13,
    0x73, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x58, 0x0a, 0x0e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x48, 0x41, 0x31, 0x44,
    0x69, 0x67, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x31, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x06, 0x52, 0x06, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x31, 0x12, 0x16, 0x0a,
    0x06, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x32, 0x18, 0x02, 0x20, 0x02, 0x28, 0x06, 0x52, 0x06, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x32, 0x12, 0x16, 0x0a, 0x06, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x33, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x07, 0x52, 0x06, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x33, 0x22, 0x33, 0x0a,
    0x0d, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x49, 0x44, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x12, 0x12,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02,
    0x69, 0x64, 0x22, 0xbb, 0x01, 0x0a, 0x12, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x53, 0x69, 0x6e,
    0x67, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x79, 0x70,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x74, 0x79, 0x70, 0x65,
    0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x5f, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44,
    0x61, 0x74, 0x61, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x32, 0x0a,
    0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x73, 0x6f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x49,
    0x44, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x52, 0x09, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x53, 0x6f, 0x69,
    0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64,
    0x22, 0xd1, 0x03, 0x0a, 0x15, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x4d, 0x75, 0x6c, 0x74, 0x69,
    0x70, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x12, 0x53, 0x0a, 0x10, 0x6f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x73, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x53, 0x4f, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74,
    0x73, 0x2e, 0x53, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x0f,
    0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x12,
    0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06,
    0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x4d, 0x0a, 0x0d, 0x6f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x73, 0x5f, 0x61, 0x64, 0x64, 0x65, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x4d, 0x75,
    0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x53, 0x69,
    0x6e, 0x67, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x0c, 0x6f, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x73, 0x41, 0x64, 0x64, 0x65, 0x64, 0x12, 0x51, 0x0a, 0x0f, 0x6f, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x73, 0x5f, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x18, 0x05, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x4d,
    0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x53,
    0x69, 0x6e, 0x67, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x52, 0x0e, 0x6f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x73, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x12, 0x32, 0x0a, 0x0a, 0x6f,
    0x77, 0x6e, 0x65, 0x72, 0x5f, 0x73, 0x6f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x49, 0x44, 0x4f,
    0x77, 0x6e, 0x65, 0x72, 0x52, 0x09, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x53, 0x6f, 0x69, 0x64, 0x12,
    0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64, 0x1a, 0x54,
    0x0a, 0x0c, 0x53, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x17,
    0x0a, 0x07, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x06, 0x74, 0x79, 0x70, 0x65, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x6f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x44, 0x61, 0x74, 0x61, 0x3a, 0x0a, 0x80, 0xa6, 0x1d, 0x80, 0x02, 0x88,
    0xa6, 0x1d, 0x80, 0x08, 0x22, 0xdc, 0x02, 0x0a, 0x15, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x43,
    0x61, 0x63, 0x68, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x12, 0x44,
    0x0a, 0x07, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x2a, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x43, 0x61, 0x63,
    0x68, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x2e, 0x53, 0x75, 0x62,
    0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07, 0x6f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x32,
    0x0a, 0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x73, 0x6f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f,
    0x49, 0x44, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x52, 0x09, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x53, 0x6f,
    0x69, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x49,
    0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6c, 0x69, 0x73,
    0x74, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x4c, 0x69, 0x73, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x79, 0x6e, 0x63, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x73, 0x79, 0x6e, 0x63,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x1a, 0x4a, 0x0a, 0x0e, 0x53, 0x75, 0x62, 0x73, 0x63,
    0x72, 0x69, 0x62, 0x65, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x79, 0x70,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x74, 0x79, 0x70, 0x65,
    0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x5f, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x0a, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44,
    0x61, 0x74, 0x61, 0x22, 0xd2, 0x01, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x43, 0x61,
    0x63, 0x68, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x55, 0x70, 0x54,
    0x6f, 0x44, 0x61, 0x74, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x32, 0x0a, 0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x73, 0x6f, 0x69, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53,
    0x4f, 0x49, 0x44, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x52, 0x09, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x53,
    0x6f, 0x69, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x6c, 0x69,
    0x73, 0x74, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x79, 0x6e, 0x63, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x73, 0x79, 0x6e,
    0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x4d, 0x0a, 0x17, 0x43, 0x4d, 0x73, 0x67,
    0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x55, 0x6e, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x64, 0x12, 0x32, 0x0a, 0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x73, 0x6f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x53, 0x4f, 0x49, 0x44, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x52, 0x09, 0x6f, 0x77,
    0x6e, 0x65, 0x72, 0x53, 0x6f, 0x69, 0x64, 0x22, 0xd1, 0x01, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67,
    0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x12, 0x32, 0x0a, 0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x73, 0x6f, 0x69, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x53, 0x4f, 0x49, 0x44, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x52, 0x09, 0x6f, 0x77, 0x6e,
    0x65, 0x72, 0x53, 0x6f, 0x69, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x72, 0x76,
    0x69, 0x63, 0x65, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0b, 0x73, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x79, 0x6e, 0x63,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b,
    0x73, 0x79, 0x6e, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x54, 0x0a, 0x1e, 0x43,
    0x4d, 0x73, 0x67, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x12, 0x32, 0x0a,
    0x0a, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x73, 0x6f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x49,
    0x44, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x52, 0x09, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x53, 0x6f, 0x69,
    0x64, 0x22, 0x2e, 0x0a, 0x12, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x22, 0x68, 0x0a, 0x16, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x4d, 0x75, 0x6c, 0x74, 0x69,
    0x70, 0x6c, 0x65, 0x78, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x6d,
    0x73, 0x67, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x6d, 0x73,
    0x67, 0x74, 0x79, 0x70, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x12,
    0x1a, 0x0a, 0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x06, 0x52, 0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x73, 0x22, 0x88, 0x02, 0x0a, 0x13,
    0x43, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x4d, 0x73, 0x67, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x41, 0x63, 0x6b, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x64, 0x69, 0x72, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x4e,
    0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x70, 0x72, 0x6f, 0x63, 0x65,
    0x73, 0x73, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x3f, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x4d, 0x73, 0x67, 0x4d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x41, 0x63, 0x6b, 0x2e, 0x50, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x52, 0x09, 0x64, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x1a, 0x4d, 0x0a, 0x07, 0x50, 0x72, 0x6f, 0x63, 0x65,
    0x73, 0x73, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x64, 0x69, 0x72, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12,
    0x25, 0x0a, 0x0e, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0d, 0x74, 0x79, 0x70, 0x65, 0x49, 0x6e, 0x73,
    0x74, 0x61, 0x6e, 0x63, 0x65, 0x73, 0x22, 0x3b, 0x0a, 0x1c, 0x43, 0x47, 0x43, 0x54, 0x6f, 0x47,
    0x43, 0x4d, 0x73, 0x67, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x41, 0x63, 0x6b, 0x5f, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1b, 0x0a, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x01, 0x32, 0x52, 0x07, 0x65, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x22, 0x49, 0x0a, 0x19, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x47,
    0x43, 0x55, 0x6e, 0x69, 0x76, 0x65, 0x72, 0x73, 0x65, 0x53, 0x74, 0x61, 0x72, 0x74, 0x75, 0x70,
    0x12, 0x2c, 0x0a, 0x12, 0x69, 0x73, 0x5f, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x5f, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x75, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x10, 0x69, 0x73,
    0x49, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x72, 0x74, 0x75, 0x70, 0x22, 0x3d,
    0x0a, 0x21, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x55, 0x6e, 0x69, 0x76,
    0x65, 0x72, 0x73, 0x65, 0x53, 0x74, 0x61, 0x72, 0x74, 0x75, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x65, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0xb2, 0x01,
    0x0a, 0x1f, 0x43, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x4d, 0x73, 0x67, 0x4d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x53, 0x74, 0x61, 0x72, 0x74, 0x75, 0x70, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74,
    0x65, 0x12, 0x45, 0x0a, 0x07, 0x67, 0x63, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x47, 0x43, 0x54, 0x6f, 0x47,
    0x43, 0x4d, 0x73, 0x67, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x53, 0x74, 0x61, 0x72, 0x74, 0x75,
    0x70, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x2e, 0x47, 0x43, 0x49, 0x6e, 0x66, 0x6f,
    0x52, 0x06, 0x67, 0x63, 0x49, 0x6e, 0x66, 0x6f, 0x1a, 0x48, 0x0a, 0x06, 0x47, 0x43, 0x49, 0x6e,
    0x66, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x64, 0x69, 0x72, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12,
    0x21, 0x0a, 0x0c, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x4e, 0x61,
    0x6d, 0x65, 0x22, 0x6b, 0x0a, 0x10, 0x43, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x4d, 0x73, 0x67,
    0x52, 0x6f, 0x75, 0x74, 0x65, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x6d, 0x73, 0x67, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x08, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1f,
    0x0a, 0x0b, 0x6e, 0x65, 0x74, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x0a, 0x6e, 0x65, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22,
    0x53, 0x0a, 0x15, 0x43, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x6f, 0x75,
    0x74, 0x65, 0x64, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x6d, 0x73, 0x67, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x6e, 0x65, 0x74, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0a, 0x6e, 0x65, 0x74, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x22, 0xb9, 0x01, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x53, 0x75, 0x62, 0x47, 0x43, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x47, 0x0a, 0x07, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x47, 0x43, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x53, 0x75, 0x62, 0x47, 0x43, 0x53,
    0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x07, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x1a, 0x50,
    0x0a, 0x0a, 0x43, 0x4d, 0x73, 0x67, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x12, 0x18, 0x0a, 0x07,
    0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73,
    0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x70, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x07, 0x52, 0x02, 0x69, 0x70, 0x12, 0x18, 0x0a, 0x07, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64,
    0x22, 0x39, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x53, 0x75, 0x62, 0x47, 0x43, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x22, 0x7f, 0x0a, 0x25, 0x43,
    0x4d, 0x73, 0x67, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x53, 0x75, 0x62, 0x47,
    0x43, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x02, 0x69, 0x70, 0x12, 0x18, 0x0a, 0x07, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x74, 0x72, 0x75, 0x73, 0x74, 0x65, 0x64, 0x12, 0x12,
    0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x70, 0x6f,
    0x72, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x07, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x22, 0xaa, 0x01, 0x0a,
    0x16, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x48, 0x61, 0x76, 0x65,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x27, 0x0a, 0x04, 0x73, 0x6f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x53, 0x4f, 0x49, 0x44, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x52, 0x04, 0x73, 0x6f, 0x69, 0x64,
    0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x06, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64, 0x12, 0x2e, 0x0a, 0x13, 0x63, 0x61, 0x63,
    0x68, 0x65, 0x64, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x63, 0x61, 0x63, 0x68, 0x65, 0x64, 0x46, 0x69,
    0x6c, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x82, 0x03, 0x0a, 0x0f, 0x43, 0x4d,
    0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x12, 0x18, 0x0a,
    0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x50, 0x0a, 0x15, 0x73, 0x6f, 0x63, 0x61, 0x63,
    0x68, 0x65, 0x5f, 0x68, 0x61, 0x76, 0x65, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x48, 0x61, 0x76, 0x65, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x52, 0x13, 0x73, 0x6f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x48, 0x61, 0x76,
    0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x2e, 0x0a, 0x13, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x65, 0x65, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x4e, 0x65, 0x65, 0x64, 0x12, 0x4f, 0x0a, 0x0f, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x6c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x6e, 0x65,
    0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x0c, 0x50, 0x41,
    0x52, 0x54, 0x4e, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x52, 0x0e, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65,
    0x63, 0x72, 0x65, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x27, 0x0a, 0x0f, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61,
    0x67, 0x65, 0x12, 0x3a, 0x0a, 0x06, 0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x53, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x45, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x3a, 0x0d, 0x6b, 0x5f, 0x45, 0x53, 0x45, 0x5f, 0x53,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x31, 0x52, 0x06, 0x65, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x22, 0xd0,
    0x04, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x57, 0x65, 0x6c,
    0x63, 0x6f, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1b,
    0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x44, 0x61, 0x74, 0x61, 0x12, 0x5b, 0x0a, 0x1b, 0x6f,
    0x75, 0x74, 0x6f, 0x66, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x64, 0x5f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x43, 0x61,
    0x63, 0x68, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x52, 0x19, 0x6f,
    0x75, 0x74, 0x6f, 0x66, 0x64, 0x61, 0x74, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62,
    0x65, 0x64, 0x43, 0x61, 0x63, 0x68, 0x65, 0x73, 0x12, 0x60, 0x0a, 0x1a, 0x75, 0x70, 0x74, 0x6f,
    0x64, 0x61, 0x74, 0x65, 0x5f, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x5f,
    0x63, 0x61, 0x63, 0x68, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x53,
    0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x52, 0x18, 0x75, 0x70, 0x74, 0x6f, 0x64, 0x61, 0x74, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72,
    0x69, 0x62, 0x65, 0x64, 0x43, 0x61, 0x63, 0x68, 0x65, 0x73, 0x12, 0x3c, 0x0a, 0x08, 0x6c, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x57, 0x65,
    0x6c, 0x63, 0x6f, 0x6d, 0x65, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x08,
    0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x22, 0x0a, 0x0d, 0x73, 0x61, 0x76, 0x65,
    0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x0b, 0x73, 0x61, 0x76, 0x65, 0x47, 0x61, 0x6d, 0x65, 0x4b, 0x65, 0x79, 0x12, 0x26, 0x0a, 0x0f,
    0x69, 0x74, 0x65, 0x6d, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x5f, 0x63, 0x72, 0x63, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0d, 0x69, 0x74, 0x65, 0x6d, 0x53, 0x63, 0x68, 0x65, 0x6d,
    0x61, 0x43, 0x72, 0x63, 0x12, 0x24, 0x0a, 0x0e, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x5f, 0x67, 0x61,
    0x6d, 0x65, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x47, 0x61, 0x6d, 0x65, 0x55, 0x72, 0x6c, 0x12, 0x35, 0x0a, 0x17, 0x67, 0x63,
    0x5f, 0x73, 0x6f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x67, 0x63, 0x53,
    0x6f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x1a, 0x5e, 0x0a, 0x08, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1a, 0x0a,
    0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x08, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6c, 0x6f, 0x6e,
    0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x6c, 0x6f,
    0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x72, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72,
    0x79, 0x22, 0xcb, 0x02, 0x0a, 0x14, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x51, 0x0a, 0x06, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x47, 0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x3a, 0x1f, 0x47, 0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x48, 0x41, 0x56, 0x45, 0x5f, 0x53, 0x45,
    0x53, 0x53, 0x49, 0x4f, 0x4e, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x2e, 0x0a,
    0x13, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f,
    0x6e, 0x65, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x4e, 0x65, 0x65, 0x64, 0x12, 0x25, 0x0a,
    0x0e, 0x71, 0x75, 0x65, 0x75, 0x65, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x71, 0x75, 0x65, 0x75, 0x65, 0x50, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x71, 0x75, 0x65, 0x75, 0x65, 0x5f, 0x73, 0x69,
    0x7a, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x71, 0x75, 0x65, 0x75, 0x65, 0x53,
    0x69, 0x7a, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x77, 0x61, 0x69, 0x74, 0x5f, 0x73, 0x65, 0x63, 0x6f,
    0x6e, 0x64, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x77, 0x61, 0x69, 0x74, 0x53,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x12, 0x47, 0x0a, 0x20, 0x65, 0x73, 0x74, 0x69, 0x6d, 0x61,
    0x74, 0x65, 0x64, 0x5f, 0x77, 0x61, 0x69, 0x74, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73,
    0x5f, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x1d, 0x65, 0x73, 0x74, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x64, 0x57, 0x61, 0x69, 0x74, 0x53,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x52, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x22,
    0xd8, 0x02, 0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x53, 0x4f,
    0x43, 0x61, 0x63, 0x68, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x12, 0x1e,
    0x0a, 0x0a, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x06, 0x52, 0x0a, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x72, 0x12, 0x26,
    0x0a, 0x0f, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x5f, 0x74, 0x6f, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x54, 0x6f, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x79, 0x6e, 0x63, 0x5f, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x73, 0x79,
    0x6e, 0x63, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x56, 0x0a, 0x0d, 0x68, 0x61, 0x76,
    0x65, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x31, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f,
    0x47, 0x43, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x48, 0x61, 0x76, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x52, 0x0c, 0x68, 0x61, 0x76, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x12, 0x2a, 0x0a, 0x11, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x5f, 0x74,
    0x6f, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x73, 0x75,
    0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x54, 0x6f, 0x54, 0x79, 0x70, 0x65, 0x1a, 0x4b, 0x0a,
    0x10, 0x43, 0x4d, 0x73, 0x67, 0x48, 0x61, 0x76, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64,
    0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0xa2, 0x01, 0x0a, 0x1c, 0x43,
    0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65,
    0x55, 0x6e, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x73,
    0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x0a, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x72, 0x12, 0x2e, 0x0a, 0x13, 0x75,
    0x6e, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x11, 0x75, 0x6e, 0x73, 0x75, 0x62, 0x73,
    0x63, 0x72, 0x69, 0x62, 0x65, 0x46, 0x72, 0x6f, 0x6d, 0x49, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x75,
    0x6e, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x13, 0x75, 0x6e, 0x73, 0x75,
    0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x46, 0x72, 0x6f, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x22,
    0x12, 0x0a, 0x10, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x50,
    0x69, 0x6e, 0x67, 0x22, 0xb4, 0x01, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f,
    0x47, 0x43, 0x46, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69,
    0x64, 0x12, 0x56, 0x0a, 0x0f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x64, 0x65, 0x74,
    0x61, 0x69, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x47, 0x43, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x47,
    0x65, 0x74, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73,
    0x5f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0e, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x61, 0x67, 0x65,
    0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a,
    0x61, 0x67, 0x65, 0x53, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x22, 0x9c, 0x01, 0x0a, 0x1c, 0x43,
    0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x4c, 0x6f, 0x61, 0x64, 0x53, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x5d, 0x0a, 0x17, 0x66, 0x6f,
    0x72, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x64, 0x65,
    0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x46, 0x6f, 0x72,
    0x77, 0x61, 0x72, 0x64, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69,
    0x6c, 0x73, 0x52, 0x15, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x41, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x22, 0x26, 0x0a, 0x24, 0x43, 0x4d, 0x73,
    0x67, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43, 0x4c, 0x6f, 0x61, 0x64, 0x53, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x92, 0x01, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x47, 0x43,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61,
    0x74, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x75, 0x73, 0x65, 0x72, 0x53,
    0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x12, 0x24, 0x0a, 0x0e, 0x69, 0x6e, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x6e, 0x5f, 0x73, 0x75, 0x72,
    0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x69, 0x6e, 0x4c, 0x6f, 0x67, 0x6f,
    0x6e, 0x53, 0x75, 0x72, 0x67, 0x65, 0x22, 0xbe, 0x03, 0x0a, 0x2a, 0x43, 0x57, 0x6f, 0x72, 0x6b,
    0x73, 0x68, 0x6f, 0x70, 0x5f, 0x50, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x49, 0x74, 0x65,
    0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5f, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x70, 0x70, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x61, 0x70, 0x70, 0x69, 0x64, 0x12, 0x6c, 0x0a, 0x09, 0x6c,
    0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x4e,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70, 0x5f,
    0x50, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x2e, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x09,
    0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x1a, 0x62, 0x0a, 0x15, 0x53, 0x69, 0x6e,
    0x67, 0x6c, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x69, 0x74, 0x65, 0x6d, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x69, 0x74, 0x65, 0x6d,
    0x69, 0x64, 0x12, 0x29, 0x0a, 0x10, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x69, 0x74,
    0x65, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x1a, 0xa7, 0x01,
    0x0a, 0x1d, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x12,
    0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x12, 0x6a, 0x0a, 0x0c, 0x64,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x46, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68,
    0x6f, 0x70, 0x5f, 0x50, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x44,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5f, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x2e, 0x53, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x44, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0c, 0x64, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0x59, 0x0a, 0x21, 0x43, 0x57, 0x6f, 0x72, 0x6b,
    0x73, 0x68, 0x6f, 0x70, 0x5f, 0x47, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x6f, 0x72, 0x73, 0x5f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05,
    0x61, 0x70, 0x70, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x61, 0x70, 0x70,
    0x69, 0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x69, 0x74, 0x65, 0x6d, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x69, 0x74, 0x65, 0x6d,
    0x69, 0x64, 0x22, 0x48, 0x0a, 0x22, 0x43, 0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70, 0x5f,
    0x47, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x6f, 0x72, 0x73, 0x5f,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x22, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x06, 0x52, 0x0c,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x6f, 0x72, 0x73, 0x22, 0x81, 0x05, 0x0a,
    0x25, 0x43, 0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70, 0x5f, 0x53, 0x65, 0x74, 0x49, 0x74,
    0x65, 0x6d, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x75, 0x6c, 0x65, 0x73, 0x5f, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x70, 0x70, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x61, 0x70, 0x70, 0x69, 0x64, 0x12, 0x1e, 0x0a, 0x0a,
    0x67, 0x61, 0x6d, 0x65, 0x69, 0x74, 0x65, 0x6d, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x69, 0x74, 0x65, 0x6d, 0x69, 0x64, 0x12, 0x7f, 0x0a, 0x19,
    0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x77, 0x6f, 0x72, 0x6b, 0x73,
    0x68, 0x6f, 0x70, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x43, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70,
    0x5f, 0x53, 0x65, 0x74, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52,
    0x75, 0x6c, 0x65, 0x73, 0x5f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x57, 0x6f, 0x72,
    0x6b, 0x73, 0x68, 0x6f, 0x70, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74,
    0x52, 0x75, 0x6c, 0x65, 0x52, 0x17, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64,
    0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70, 0x46, 0x69, 0x6c, 0x65, 0x73, 0x12, 0x6d, 0x0a,
    0x10, 0x70, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x42, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70, 0x5f, 0x53, 0x65, 0x74, 0x49, 0x74, 0x65, 0x6d,
    0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x75, 0x6c, 0x65, 0x73, 0x5f, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72, 0x49, 0x74, 0x65, 0x6d,
    0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x75, 0x6c, 0x65, 0x52, 0x0f, 0x70, 0x61, 0x72,
    0x74, 0x6e, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x73, 0x1a, 0x9d, 0x01, 0x0a,
    0x17, 0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x61, 0x79,
    0x6d, 0x65, 0x6e, 0x74, 0x52, 0x75, 0x6c, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x77, 0x6f, 0x72, 0x6b,
    0x73, 0x68, 0x6f, 0x70, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0e, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70, 0x46, 0x69, 0x6c, 0x65,
    0x49, 0x64, 0x12, 0x2d, 0x0a, 0x12, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x75, 0x65, 0x5f, 0x70, 0x65,
    0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x11,
    0x72, 0x65, 0x76, 0x65, 0x6e, 0x75, 0x65, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67,
    0x65, 0x12, 0x29, 0x0a, 0x10, 0x72, 0x75, 0x6c, 0x65, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x72, 0x75, 0x6c,
    0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x1a, 0x91, 0x01, 0x0a,
    0x16, 0x50, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x61, 0x79, 0x6d,
    0x65, 0x6e, 0x74, 0x52, 0x75, 0x6c, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x2d, 0x0a, 0x12, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x75,
    0x65, 0x5f, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x11, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x75, 0x65, 0x50, 0x65, 0x72, 0x63, 0x65,
    0x6e, 0x74, 0x61, 0x67, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x72, 0x75, 0x6c, 0x65, 0x5f, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0f, 0x72, 0x75, 0x6c, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x22, 0x28, 0x0a, 0x26, 0x43, 0x57, 0x6f, 0x72, 0x6b, 0x73, 0x68, 0x6f, 0x70, 0x5f, 0x53, 0x65,
    0x74, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x61, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x75, 0x6c, 0x65,
    0x73, 0x5f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x98, 0x01, 0x0a, 0x24, 0x43,
    0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x5f, 0x50, 0x6f, 0x73, 0x74, 0x47, 0x61,
    0x6d, 0x65, 0x44, 0x61, 0x74, 0x61, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x5f, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x70, 0x70, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x05, 0x61, 0x70, 0x70, 0x69, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x65,
    0x61, 0x6d, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61,
    0x6d, 0x69, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0b, 0x62, 0x72, 0x6f, 0x61, 0x64,
    0x63, 0x61, 0x73, 0x74, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x5f,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x66, 0x72, 0x61, 0x6d,
    0x65, 0x44, 0x61, 0x74, 0x61, 0x22, 0x82, 0x04, 0x0a, 0x15, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x65,
    0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x12,
    0x21, 0x0a, 0x0c, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x66, 0x69, 0x6c, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x12, 0x39, 0x0a, 0x06, 0x63, 0x61, 0x63, 0x68, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x65,
    0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x2e,
    0x43, 0x61, 0x63, 0x68, 0x65, 0x52, 0x06, 0x63, 0x61, 0x63, 0x68, 0x65, 0x73, 0x12, 0x35, 0x0a,
    0x17, 0x67, 0x63, 0x5f, 0x73, 0x6f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x66, 0x69, 0x6c, 0x65,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14,
    0x67, 0x63, 0x53, 0x6f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x46, 0x69, 0x6c, 0x65, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x1a, 0x58, 0x0a, 0x09, 0x54, 0x79, 0x70, 0x65, 0x43, 0x61, 0x63, 0x68,
    0x65, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x07, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x12,
    0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x49, 0x64, 0x1a, 0xf9,
    0x01, 0x0a, 0x05, 0x43, 0x61, 0x63, 0x68, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x0e, 0x0a, 0x02,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x45, 0x0a, 0x08,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x29,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x64, 0x53, 0x4f, 0x43, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x43, 0x61, 0x63, 0x68,
    0x65, 0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x08, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x12, 0x46, 0x0a, 0x0b, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x63, 0x61, 0x63, 0x68,
    0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x53, 0x4f,
    0x43, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x43, 0x61, 0x63, 0x68, 0x65, 0x52,
    0x0a, 0x74, 0x79, 0x70, 0x65, 0x43, 0x61, 0x63, 0x68, 0x65, 0x73, 0x1a, 0x3d, 0x0a, 0x07, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x5b, 0x0a, 0x1f, 0x43, 0x4d,
    0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x50, 0x6f, 0x6c, 0x6c,
    0x43, 0x6f, 0x6e, 0x76, 0x61, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1f, 0x0a,
    0x0b, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x17,
    0x0a, 0x07, 0x70, 0x6f, 0x6c, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x06, 0x70, 0x6f, 0x6c, 0x6c, 0x49, 0x64, 0x22, 0x5e, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x47,
    0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x50, 0x6f, 0x6c, 0x6c, 0x43, 0x6f, 0x6e,
    0x76, 0x61, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x70,
    0x6f, 0x6c, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x70, 0x6f,
    0x6c, 0x6c, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x72, 0x5f, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f, 0x6e, 0x76,
    0x61, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2a, 0x35, 0x0a, 0x0d, 0x45, 0x53, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x45, 0x6e, 0x67, 0x69, 0x6e, 0x65, 0x12, 0x11, 0x0a, 0x0d, 0x6b, 0x5f, 0x45, 0x53,
    0x45, 0x5f, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x31, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x6b,
    0x5f, 0x45, 0x53, 0x45, 0x5f, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x32, 0x10, 0x01, 0x2a, 0x69,
    0x0a, 0x12, 0x50, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x0c, 0x50, 0x41, 0x52, 0x54, 0x4e, 0x45, 0x52, 0x5f,
    0x4e, 0x4f, 0x4e, 0x45, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x15, 0x50, 0x41, 0x52, 0x54, 0x4e, 0x45,
    0x52, 0x5f, 0x50, 0x45, 0x52, 0x46, 0x45, 0x43, 0x54, 0x5f, 0x57, 0x4f, 0x52, 0x4c, 0x44, 0x10,
    0x01, 0x12, 0x11, 0x0a, 0x0d, 0x50, 0x41, 0x52, 0x54, 0x4e, 0x45, 0x52, 0x5f, 0x4e, 0x45, 0x58,
    0x4f, 0x4e, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x50, 0x41, 0x52, 0x54, 0x4e, 0x45, 0x52, 0x5f,
    0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x10, 0x03, 0x2a, 0xa0, 0x02, 0x0a, 0x12, 0x47, 0x43,
    0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x12, 0x23, 0x0a, 0x1f, 0x47, 0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x48, 0x41, 0x56, 0x45, 0x5f, 0x53, 0x45, 0x53, 0x53,
    0x49, 0x4f, 0x4e, 0x10, 0x00, 0x12, 0x24, 0x0a, 0x20, 0x47, 0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x47, 0x43, 0x5f, 0x47,
    0x4f, 0x49, 0x4e, 0x47, 0x5f, 0x44, 0x4f, 0x57, 0x4e, 0x10, 0x01, 0x12, 0x21, 0x0a, 0x1d, 0x47,
    0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x5f, 0x4e, 0x4f, 0x5f, 0x53, 0x45, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x02, 0x12, 0x30,
    0x0a, 0x2c, 0x47, 0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x5f, 0x4e, 0x4f, 0x5f, 0x53, 0x45, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f,
    0x49, 0x4e, 0x5f, 0x4c, 0x4f, 0x47, 0x4f, 0x4e, 0x5f, 0x51, 0x55, 0x45, 0x55, 0x45, 0x10, 0x03,
    0x12, 0x1f, 0x0a, 0x1b, 0x47, 0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x4e, 0x4f, 0x5f, 0x53, 0x54, 0x45, 0x41, 0x4d, 0x10,
    0x04, 0x12, 0x20, 0x0a, 0x1c, 0x47, 0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x53, 0x55, 0x53, 0x50, 0x45, 0x4e, 0x44, 0x45,
    0x44, 0x10, 0x05, 0x12, 0x27, 0x0a, 0x23, 0x47, 0x43, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x53, 0x54, 0x45, 0x41, 0x4d, 0x5f,
    0x47, 0x4f, 0x49, 0x4e, 0x47, 0x5f, 0x44, 0x4f, 0x57, 0x4e, 0x10, 0x06, 0x42, 0x05, 0x48, 0x01,
    0x80, 0x01, 0x00, 0x4a, 0xce, 0x6e, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xde, 0x02, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1c, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x06, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06,
    0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06,
    0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x1b,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x01, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x07,
    0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0c, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x09, 0x05, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x0a, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0a, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x0a, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x0e, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12,
    0x03, 0x0e, 0x05, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x08,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x17, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x10, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x11, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x08,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x11, 0x18, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12, 0x03, 0x12, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x12, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x12, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04,
    0x15, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x15, 0x05, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x03, 0x16, 0x08, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x16, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x17, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x17, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x17,
    0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x18, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x18, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x18, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x19, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x19, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x08,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x02, 0x12, 0x03, 0x1a, 0x26, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1b, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05,
    0x02, 0x12, 0x03, 0x1b, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x1c, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1c, 0x08,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x06, 0x02, 0x12, 0x03, 0x1c, 0x2e, 0x2f, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1f, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x20, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x20,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x19, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x20, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x21, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x21, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x21, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x21, 0x19, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x21, 0x22,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x22, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x22, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x22, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x19, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x22, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x25, 0x00,
    0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x25, 0x08, 0x15, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x26, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x26, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x26, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x26, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x27, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x27, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x2a, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x2b, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x2b, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2c,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2c, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x2d, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x2d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x19,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x06, 0x12, 0x03, 0x2e, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x2e, 0x1f, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x2e, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x08,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2f, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2f, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2f, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x32, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x32, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x03, 0x00, 0x12, 0x04, 0x33, 0x08, 0x38, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x03, 0x00, 0x01, 0x12, 0x03, 0x33, 0x10, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x03, 0x00, 0x07, 0x12, 0x03, 0x34, 0x10, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x03, 0x03, 0x00, 0x07, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x34, 0x10, 0x32, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x03, 0x03, 0x00, 0x07, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x34, 0x17, 0x2b, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x03, 0x03, 0x00, 0x07, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x34, 0x17,
    0x2b, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03, 0x03, 0x00, 0x07, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x34, 0x18, 0x2a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x07, 0xe7, 0x07,
    0x00, 0x04, 0x12, 0x03, 0x34, 0x2e, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x03, 0x00, 0x07,
    0x12, 0x03, 0x35, 0x10, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x03, 0x00, 0x07, 0xe7, 0x07,
    0x01, 0x12, 0x03, 0x35, 0x10, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x07, 0xe7,
    0x07, 0x01, 0x02, 0x12, 0x03, 0x35, 0x17, 0x2b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x03, 0x00,
    0x07, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x35, 0x17, 0x2b, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x03, 0x03, 0x00, 0x07, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x18, 0x2a, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x03, 0x03, 0x00, 0x07, 0xe7, 0x07, 0x01, 0x04, 0x12, 0x03, 0x35, 0x2e,
    0x32, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x36, 0x10, 0x2b,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x36, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x36, 0x19, 0x1e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x1f, 0x26,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36, 0x29, 0x2a,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x37, 0x10, 0x2f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x37, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x37, 0x19, 0x1e, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x1f, 0x2a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x03, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x2d, 0x2e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3a, 0x08, 0x49, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x3a, 0x11, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x3a, 0x34, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x3a, 0x47, 0x48, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3b, 0x11, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3b, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x02, 0x12, 0x03, 0x3c, 0x08, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x3c,
    0x11, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x34, 0x41,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3c, 0x44, 0x45, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x3d, 0x08, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x3d, 0x11, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x3d, 0x34, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x3d, 0x46, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x06, 0x12, 0x03, 0x3e, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3e, 0x1f, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x3e, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05,
    0x12, 0x03, 0x3f, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x3f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x3f, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12, 0x03, 0x3f, 0x18, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x3f, 0x25, 0x26, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x42, 0x00, 0x4e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x42, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x43,
    0x08, 0x46, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x43, 0x10,
    0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x44, 0x10, 0x2b,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x44, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x44, 0x19, 0x1e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x1f, 0x26,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x29, 0x2a,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x45, 0x10, 0x2f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x45, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x45, 0x19, 0x1e, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x45, 0x1f, 0x2a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45, 0x2d, 0x2e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x48, 0x08, 0x42, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x48, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x48, 0x11, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x48, 0x36, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x48, 0x40, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x49, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x49, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x49, 0x11, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x49, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x49, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x02, 0x12, 0x03, 0x4a, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4a,
    0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x1f, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x2c, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x4b, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x4b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x4b, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x4b, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x4c, 0x08, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x4c, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x4c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4c, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x4c, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05,
    0x12, 0x03, 0x4d, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x4d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x4d, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4d, 0x19, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x4d, 0x28, 0x29, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x50, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x50, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x51,
    0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x51, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x51, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x51, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x01, 0x12, 0x03, 0x52, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x52, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x52, 0x1f,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x52, 0x2c, 0x2d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x53, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x53, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x53, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x53, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x53, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x54, 0x08,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x54, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x54, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x54, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x54, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x04, 0x12, 0x03, 0x55, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x55, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12, 0x03, 0x55,
    0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x55, 0x19, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x55, 0x28, 0x29, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x58, 0x00, 0x5a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x58, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03,
    0x59, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x59, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x59, 0x11, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x1f, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x59, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x07, 0x12, 0x04, 0x5c, 0x00, 0x62, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03,
    0x5c, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x5d, 0x08, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5d, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01,
    0x12, 0x03, 0x5e, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5e, 0x11,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5e, 0x1f, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5e, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x5f, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x5f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x5f, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5f,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x60, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x03, 0x60, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x03, 0x60, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x60, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x60, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12,
    0x03, 0x61, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x61,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x03, 0x61, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x61, 0x19, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x03, 0x61, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x08, 0x12, 0x04, 0x64, 0x00, 0x66, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12,
    0x03, 0x64, 0x08, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x65, 0x08,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x65, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x65, 0x11, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x65, 0x1f, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x65, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12,
    0x04, 0x68, 0x00, 0x6a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x68, 0x08,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x69, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x69, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x69, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x69, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x6c, 0x00,
    0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x6c, 0x08, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x6d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x6d, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x6d, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6e, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x6e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02,
    0x12, 0x03, 0x6f, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x6f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6f, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6f, 0x19, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6f, 0x24, 0x25, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0b, 0x12, 0x04, 0x72, 0x00, 0x7c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01,
    0x12, 0x03, 0x72, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x03, 0x00, 0x12, 0x04, 0x73,
    0x08, 0x76, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x03, 0x00, 0x01, 0x12, 0x03, 0x73, 0x10,
    0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x74, 0x10, 0x2e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x74, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x74, 0x19, 0x1f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x74, 0x20, 0x29,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x74, 0x2c, 0x2d,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x75, 0x10, 0x33, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x75, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x75, 0x19, 0x1f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x75, 0x20, 0x2e, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x75, 0x31, 0x32, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x78, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x78, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x78, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x78, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x78, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x79, 0x08,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x79, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x79, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x79, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x79, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x02, 0x12, 0x03, 0x7a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x7a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7a,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x7a, 0x18, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7a, 0x27, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x7b, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x7b, 0x11, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x7b, 0x2d, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x7b, 0x39, 0x3a, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x05, 0x7e, 0x00, 0x80, 0x01, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x7f, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x7f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x7f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x7f, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7f, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x08, 0x12, 0x03, 0x7f, 0x23, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x07, 0x12, 0x03, 0x7f, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x0d, 0x12, 0x06, 0x82, 0x01, 0x00, 0x84, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x0d, 0x01, 0x12, 0x04, 0x82, 0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00,
    0x12, 0x04, 0x83, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x83, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x83, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x83,
    0x01, 0x16, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x83, 0x01,
    0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0x86, 0x01, 0x00, 0x88, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0x86, 0x01, 0x08, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0x87, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x87, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x87, 0x01, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x87, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x8a,
    0x01, 0x00, 0x91, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x8a, 0x01,
    0x08, 0x27, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0f, 0x03, 0x00, 0x12, 0x06, 0x8b, 0x01, 0x08, 0x8e,
    0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x03, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x10,
    0x16, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x01, 0x10,
    0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x01,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8c,
    0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x8c, 0x01, 0x20, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x8c, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x04, 0x8d, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x8d, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x8d, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x20, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x00, 0x12, 0x04, 0x90, 0x01, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x90, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12,
    0x04, 0x90, 0x01, 0x11, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x90, 0x01, 0x38, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x90,
    0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x93, 0x01, 0x00, 0x97, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x93, 0x01, 0x08, 0x18, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x94, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0x94, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x94, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01,
    0x12, 0x04, 0x95, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x95, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04,
    0x95, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0x95,
    0x01, 0x19, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0x95, 0x01,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0x96, 0x01, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0x96, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x05, 0x12, 0x04, 0x96, 0x01, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0x96, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x11, 0x12, 0x06, 0x99, 0x01, 0x00, 0x9c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11,
    0x01, 0x12, 0x04, 0x99, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12,
    0x04, 0x9a, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x9a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9a,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9a, 0x01,
    0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x23,
    0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9b, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x12, 0x12, 0x06, 0x9e, 0x01, 0x00, 0xa6, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01,
    0x12, 0x04, 0x9e, 0x01, 0x08, 0x24, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x12, 0x03, 0x00, 0x12, 0x06,
    0x9f, 0x01, 0x08, 0xa3, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x03, 0x00, 0x01, 0x12,
    0x04, 0x9f, 0x01, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xa0, 0x01, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xa0, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xa0, 0x01, 0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x21, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12,
    0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x12, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x21, 0x23, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x12, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x26, 0x27, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x12, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xa2, 0x01, 0x10, 0x2a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x19, 0x1d, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x1e, 0x25,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x28,
    0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x45, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa5, 0x01, 0x11, 0x38, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x39, 0x40, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x43, 0x44, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x13, 0x12, 0x06, 0xa8, 0x01, 0x00, 0xaa, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01,
    0x12, 0x04, 0xa8, 0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04,
    0xa9, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa9,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa9, 0x01,
    0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x19,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xac, 0x01, 0x00, 0xb1, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xac, 0x01, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x00, 0x12, 0x04, 0xad, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xad, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xad, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xad, 0x01, 0x19, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xad, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xae,
    0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xae, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0xae, 0x01, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xae, 0x01, 0x16, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xae, 0x01, 0x20, 0x21, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02,
    0x03, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04,
    0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x05, 0x12,
    0x04, 0xb0, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xb0, 0x01, 0x16, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb0,
    0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xb3, 0x01, 0x00, 0xb8, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x1e, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb4, 0x01, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x1f, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xb4, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01,
    0x12, 0x04, 0xb5, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xb5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xb5, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb5,
    0x01, 0x19, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb5, 0x01,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb6, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xb7, 0x01, 0x18, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xb7, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xba, 0x01,
    0x00, 0xc2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbb, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xbc, 0x01, 0x11, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xbc, 0x01, 0x28, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xbc, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x02, 0x12, 0x04, 0xbd,
    0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbd, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbd, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x18, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbd, 0x01, 0x2e, 0x2f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x51, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x03, 0x06, 0x12, 0x04, 0xbe, 0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x24, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x36, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x03, 0x08, 0x12, 0x04, 0xbe, 0x01, 0x38, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03,
    0x07, 0x12, 0x04, 0xbe, 0x01, 0x43, 0x4f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x04, 0x12,
    0x04, 0xbf, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xbf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x05, 0x12, 0x04, 0xbf,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x01, 0x12, 0x04, 0xbf, 0x01,
    0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x05, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x04, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x05, 0x12, 0x04, 0xc0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x05, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x05, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x06, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x06, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x06,
    0x06, 0x12, 0x04, 0xc1, 0x01, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x01,
    0x12, 0x04, 0xc1, 0x01, 0x1f, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x03, 0x12,
    0x04, 0xc1, 0x01, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x08, 0x12, 0x04,
    0xc1, 0x01, 0x2a, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x07, 0x12, 0x04, 0xc1,
    0x01, 0x35, 0x42, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xc4, 0x01, 0x00, 0xd4, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x19, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x17, 0x03, 0x00, 0x12, 0x06, 0xc5, 0x01, 0x08, 0xc9, 0x01, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x03, 0x00, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x17, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x19, 0x1e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x1f, 0x27,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x2a,
    0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x10,
    0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc7, 0x01,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc7,
    0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xc7, 0x01, 0x1f, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xc7, 0x01, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x03, 0x00, 0x02, 0x02, 0x12,
    0x04, 0xc8, 0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x04, 0xc8, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xc8, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x20, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x00, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xcb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xcb, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xcb, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcb,
    0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x17, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x02, 0x06, 0x12, 0x04, 0xcd, 0x01, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x27, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xcd, 0x01, 0x45, 0x46, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12,
    0x04, 0xce, 0x01, 0x08, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xce, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x06, 0x12, 0x04, 0xce,
    0x01, 0x11, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0xce, 0x01,
    0x2e, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0xce, 0x01, 0x4b,
    0x4c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x04, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x39, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x04, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x04, 0x06, 0x12, 0x04, 0xcf, 0x01, 0x11, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x04, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x2c, 0x34, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x04, 0x03, 0x12, 0x04, 0xcf, 0x01, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x05, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x05, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05,
    0x05, 0x12, 0x04, 0xd0, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x01,
    0x12, 0x04, 0xd0, 0x01, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x03, 0x12,
    0x04, 0xd0, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x06, 0x12, 0x04, 0xd1,
    0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x06, 0x04, 0x12, 0x04, 0xd1, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x06, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x06, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x19, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x06, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x2b, 0x2c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x07, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x07, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x07, 0x05, 0x12, 0x04, 0xd2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x07, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x07, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x08, 0x12, 0x04, 0xd3, 0x01, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x08, 0x04,
    0x12, 0x04, 0xd3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x08, 0x05, 0x12,
    0x04, 0xd3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x08, 0x01, 0x12, 0x04,
    0xd3, 0x01, 0x18, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x08, 0x03, 0x12, 0x04, 0xd3,
    0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xd6, 0x01, 0x00, 0xdd, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x1c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x5b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd7, 0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x24, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x2d, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x08, 0x12, 0x04, 0xd7, 0x01, 0x2f, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x07,
    0x12, 0x04, 0xd7, 0x01, 0x3a, 0x59, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04,
    0xd8, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd8,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd8, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x18,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x2e, 0x2f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x02, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd9, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x17, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x03, 0x12, 0x04, 0xda, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xda, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x05,
    0x12, 0x04, 0xda, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xda, 0x01, 0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xda, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x04, 0x12, 0x04, 0xdb, 0x01,
    0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x17, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x26, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x05, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x05, 0x04, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x05, 0x05, 0x12, 0x04, 0xdc, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x05, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x17, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x05, 0x03, 0x12, 0x04, 0xdc, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06,
    0xdf, 0x01, 0x00, 0xea, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xdf,
    0x01, 0x08, 0x22, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x19, 0x03, 0x00, 0x12, 0x06, 0xe0, 0x01, 0x08,
    0xe3, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x03, 0x00, 0x01, 0x12, 0x04, 0xe0, 0x01,
    0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x01,
    0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe1,
    0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xe1, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xe1, 0x01, 0x20, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xe1, 0x01, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x19, 0x03, 0x00, 0x02, 0x01,
    0x12, 0x04, 0xe2, 0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xe2, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x20, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x19, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19,
    0x02, 0x00, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xe5, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xe5, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xe5, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xe6, 0x01,
    0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x11, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x19, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x2b, 0x2c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe7, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x19, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x03,
    0x12, 0x04, 0xe8, 0x01, 0x08, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xe8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x06, 0x12, 0x04,
    0xe8, 0x01, 0x11, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe8,
    0x01, 0x3d, 0x4a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe8, 0x01,
    0x4d, 0x4e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x04, 0x12, 0x04, 0xe9, 0x01, 0x08, 0x2e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x04, 0x12, 0x04, 0xe9, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x05, 0x12, 0x04, 0xe9, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x01, 0x12, 0x04, 0xe9, 0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x04, 0x03, 0x12, 0x04, 0xe9, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1a, 0x12, 0x06, 0xec, 0x01, 0x00, 0xf0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a,
    0x01, 0x12, 0x04, 0xec, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12,
    0x04, 0xed, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xed, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xed,
    0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xed, 0x01,
    0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xed, 0x01, 0x26,
    0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0xee, 0x01, 0x08, 0x31, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x05, 0x12, 0x04, 0xee, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee, 0x01, 0x19, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xee, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1a, 0x02, 0x02, 0x12, 0x04, 0xef, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xef, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xef, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xef, 0x01, 0x18, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xef, 0x01, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xf2, 0x01, 0x00,
    0xf3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x18,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xf5, 0x01, 0x00, 0xf9, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1c, 0x02, 0x00, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xf6, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xf6, 0x01, 0x19, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xf6, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01, 0x12, 0x04, 0xf7,
    0x01, 0x08, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf7, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf7, 0x01, 0x11,
    0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x39, 0x48,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x4b, 0x4c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12,
    0x06, 0xfb, 0x01, 0x00, 0xfe, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04,
    0xfb, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xfc, 0x01,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xfc, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x18, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x4d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x01, 0x06, 0x12, 0x04, 0xfd, 0x01, 0x11, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x31, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x4b, 0x4c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06,
    0x80, 0x02, 0x00, 0x81, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0x80,
    0x02, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0x83, 0x02, 0x00, 0x87, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0x83, 0x02, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0x84, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x84, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x84, 0x02, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x84, 0x02, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x01,
    0x12, 0x04, 0x85, 0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x85, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x05, 0x12, 0x04,
    0x85, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x01, 0x12, 0x04, 0x85,
    0x02, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03, 0x12, 0x04, 0x85, 0x02,
    0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x02, 0x12, 0x04, 0x86, 0x02, 0x08, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x04, 0x12, 0x04, 0x86, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x05, 0x12, 0x04, 0x86, 0x02, 0x11, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x01, 0x12, 0x04, 0x86, 0x02, 0x16, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x02, 0x03, 0x12, 0x04, 0x86, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x20, 0x12, 0x06, 0x89, 0x02, 0x00, 0x96, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20,
    0x01, 0x12, 0x04, 0x89, 0x02, 0x08, 0x32, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x20, 0x03, 0x00, 0x12,
    0x06, 0x8a, 0x02, 0x08, 0x8d, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x03, 0x00, 0x01,
    0x12, 0x04, 0x8a, 0x02, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x20, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x04, 0x8b, 0x02, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x8b, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x20, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x20,
    0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x10, 0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8c, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x20, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8c, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x20, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x20, 0x30, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x20, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x33, 0x34, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x20, 0x03, 0x01, 0x12, 0x06, 0x8f, 0x02, 0x08, 0x92, 0x02, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x03, 0x01, 0x01, 0x12, 0x04, 0x8f, 0x02, 0x10, 0x2d, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x20, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0x90, 0x02, 0x10, 0x2d, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x20, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x90, 0x02, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x20, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0x90, 0x02, 0x19, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0x90, 0x02, 0x20, 0x28,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04, 0x90, 0x02, 0x2b,
    0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x20, 0x03, 0x01, 0x02, 0x01, 0x12, 0x04, 0x91, 0x02, 0x10,
    0x6b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0x91, 0x02,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x01, 0x02, 0x01, 0x06, 0x12, 0x04, 0x91,
    0x02, 0x19, 0x59, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x91, 0x02, 0x5a, 0x66, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x91, 0x02, 0x69, 0x6a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0x94,
    0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x05, 0x12, 0x04, 0x94, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x02, 0x18, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x02, 0x20, 0x21, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04, 0x95, 0x02, 0x08, 0x68, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04, 0x95, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x01, 0x06, 0x12, 0x04, 0x95, 0x02, 0x11, 0x59, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0x95, 0x02, 0x5a, 0x63, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x95, 0x02, 0x66, 0x67, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12,
    0x06, 0x98, 0x02, 0x00, 0x9b, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04,
    0x98, 0x02, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0x99, 0x02,
    0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x04, 0x12, 0x04, 0x99, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x05, 0x12, 0x04, 0x99, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99, 0x02, 0x18, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03, 0x12, 0x04, 0x99, 0x02, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9a, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x21, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9a, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x9a, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06,
    0x9d, 0x02, 0x00, 0x9f, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0x9d,
    0x02, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x02, 0x08,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9e, 0x02, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x02, 0x19, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9e, 0x02, 0x28, 0x29, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x23, 0x12, 0x06, 0xa1, 0x02, 0x00, 0xb2, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x23, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x2d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x23, 0x03, 0x00,
    0x12, 0x06, 0xa2, 0x02, 0x08, 0xa6, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x03, 0x00,
    0x01, 0x12, 0x04, 0xa2, 0x02, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xa3, 0x02, 0x10, 0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xa3, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xa3, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x20, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x02, 0x33, 0x34, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x23, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x10, 0x36, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x23, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa4, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x23, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa4, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x1f, 0x31, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x34, 0x35, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x23, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xa5, 0x02, 0x10, 0x35, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa5, 0x02, 0x10, 0x18,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa5, 0x02, 0x19,
    0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x02,
    0x20, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa5,
    0x02, 0x33, 0x34, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x23, 0x03, 0x01, 0x12, 0x06, 0xa8, 0x02, 0x08,
    0xac, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x03, 0x01, 0x01, 0x12, 0x04, 0xa8, 0x02,
    0x10, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0xa9, 0x02,
    0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa9,
    0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xa9, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xa9, 0x02, 0x20, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa9, 0x02, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23, 0x03, 0x01, 0x02, 0x01,
    0x12, 0x04, 0xaa, 0x02, 0x10, 0x36, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xaa, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x01, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xaa, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xaa, 0x02, 0x1f, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23, 0x03,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa, 0x02, 0x34, 0x35, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x23,
    0x03, 0x01, 0x02, 0x02, 0x12, 0x04, 0xab, 0x02, 0x10, 0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x23,
    0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0xab, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x23, 0x03, 0x01, 0x02, 0x02, 0x05, 0x12, 0x04, 0xab, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x23, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0xab, 0x02, 0x20, 0x30, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x23, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x04, 0xab, 0x02, 0x33, 0x34, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x23, 0x02, 0x00, 0x12, 0x04, 0xae, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x00, 0x04, 0x12, 0x04, 0xae, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x00, 0x05, 0x12, 0x04, 0xae, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xae, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xae, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x01,
    0x12, 0x04, 0xaf, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xaf, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xaf, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaf,
    0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaf, 0x02,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x02, 0x12, 0x04, 0xb0, 0x02, 0x08, 0x6d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb0, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x06, 0x12, 0x04, 0xb0, 0x02, 0x11, 0x4e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb0, 0x02, 0x4f, 0x68, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb0, 0x02, 0x6b, 0x6c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x23, 0x02, 0x03, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x63, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x03, 0x06, 0x12, 0x04, 0xb1, 0x02, 0x11, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xb1, 0x02, 0x4e, 0x5e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xb1, 0x02, 0x61, 0x62, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xb4, 0x02,
    0x00, 0xb5, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xb4, 0x02, 0x08,
    0x2e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0xb7, 0x02, 0x00, 0xbc, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x25, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x25, 0x02, 0x00, 0x12, 0x04, 0xb8, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xb8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xb8, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xb8, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xb8, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x01, 0x12, 0x04,
    0xb9, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb9,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb9, 0x02,
    0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x19,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x02, 0x12, 0x04, 0xba, 0x02, 0x08, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x04, 0x12, 0x04, 0xba, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x02, 0x05, 0x12, 0x04, 0xba, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x02, 0x01, 0x12, 0x04, 0xba, 0x02, 0x19, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x02, 0x03, 0x12, 0x04, 0xba, 0x02, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25,
    0x02, 0x03, 0x12, 0x04, 0xbb, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xbb, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x05,
    0x12, 0x04, 0xbb, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xbb, 0x02, 0x17, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xbb, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0xbe, 0x02, 0x00, 0xd4,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x08, 0x1d, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x26, 0x03, 0x00, 0x12, 0x06, 0xbf, 0x02, 0x08, 0xc3, 0x02, 0x09, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x03, 0x00, 0x01, 0x12, 0x04, 0xbf, 0x02, 0x10, 0x19, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x26, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc0, 0x02, 0x10, 0x29, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc0, 0x02, 0x10, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc0, 0x02, 0x19, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc0, 0x02, 0x20,
    0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc0, 0x02,
    0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xc1, 0x02,
    0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc1,
    0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xc1, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xc1, 0x02, 0x1f, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xc1, 0x02, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x03, 0x00, 0x02, 0x02,
    0x12, 0x04, 0xc2, 0x02, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xc2, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xc2, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x20, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc2, 0x02, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x26,
    0x03, 0x01, 0x12, 0x06, 0xc5, 0x02, 0x08, 0xcf, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x03, 0x01, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x10, 0x15, 0x0a, 0x10, 0x0a, 0x06, 0x04, 0x26, 0x03,
    0x01, 0x03, 0x00, 0x12, 0x06, 0xc6, 0x02, 0x10, 0xc9, 0x02, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x26, 0x03, 0x01, 0x03, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x02, 0x18, 0x1f, 0x0a, 0x10, 0x0a, 0x08,
    0x04, 0x26, 0x03, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc7, 0x02, 0x18, 0x34, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x26, 0x03, 0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc7, 0x02, 0x18,
    0x20, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x26, 0x03, 0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xc7, 0x02, 0x21, 0x27, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x26, 0x03, 0x01, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xc7, 0x02, 0x28, 0x2f, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x26, 0x03, 0x01, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc7, 0x02, 0x32, 0x33, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x26,
    0x03, 0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x02, 0x18, 0x34, 0x0a, 0x11, 0x0a, 0x09,
    0x04, 0x26, 0x03, 0x01, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc8, 0x02, 0x18, 0x20, 0x0a,
    0x11, 0x0a, 0x09, 0x04, 0x26, 0x03, 0x01, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc8, 0x02,
    0x21, 0x27, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x26, 0x03, 0x01, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xc8, 0x02, 0x28, 0x2f, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x26, 0x03, 0x01, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc8, 0x02, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x03, 0x01,
    0x02, 0x00, 0x12, 0x04, 0xcb, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xcb, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcb, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26,
    0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x02, 0x20, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x26, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcb, 0x02, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x26, 0x03, 0x01, 0x02, 0x01, 0x12, 0x04, 0xcc, 0x02, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x26, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcc, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcc, 0x02, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc, 0x02, 0x20, 0x22, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc, 0x02, 0x25, 0x26,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x03, 0x01, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x02, 0x10, 0x4a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0xcd, 0x02, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x02, 0x06, 0x12, 0x04, 0xcd, 0x02,
    0x19, 0x3c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcd,
    0x02, 0x3d, 0x45, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xcd, 0x02, 0x48, 0x49, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x26, 0x03, 0x01, 0x02, 0x03, 0x12, 0x04,
    0xce, 0x02, 0x10, 0x49, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xce, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x03, 0x06,
    0x12, 0x04, 0xce, 0x02, 0x19, 0x38, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xce, 0x02, 0x39, 0x44, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x26, 0x03, 0x01, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xce, 0x02, 0x47, 0x48, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00,
    0x12, 0x04, 0xd1, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xd1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xd1, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd1,
    0x02, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd1, 0x02,
    0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x02, 0x08, 0x38,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd2, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd2, 0x02, 0x11, 0x2c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd2, 0x02, 0x2d, 0x33, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2, 0x02, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x26, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x02, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xd3, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xd3, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xd3, 0x02, 0x18, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xd3, 0x02, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27, 0x12, 0x06, 0xd6, 0x02,
    0x00, 0xd9, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12, 0x04, 0xd6, 0x02, 0x08,
    0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd7, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd7, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x27, 0x02, 0x01, 0x12, 0x04, 0xd8, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xd8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xd8, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xd8, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xd8, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0xdb, 0x02, 0x00,
    0xde, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0xdb, 0x02, 0x08, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04, 0xdc, 0x02, 0x08, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdc, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdc, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdc, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28,
    0x02, 0x01, 0x12, 0x04, 0xdd, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xdd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xdd, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xdd, 0x02, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xdd, 0x02, 0x27, 0x28,
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
