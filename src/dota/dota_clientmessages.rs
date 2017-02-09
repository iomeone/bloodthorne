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
pub struct CDOTAClientMsg_MapPing {
    // message fields
    location_ping: ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_LocationPing>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_MapPing {}

impl CDOTAClientMsg_MapPing {
    pub fn new() -> CDOTAClientMsg_MapPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_MapPing {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_MapPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_MapPing,
        };
        unsafe {
            instance.get(CDOTAClientMsg_MapPing::new)
        }
    }

    // optional .dota.CDOTAMsg_LocationPing location_ping = 1;

    pub fn clear_location_ping(&mut self) {
        self.location_ping.clear();
    }

    pub fn has_location_ping(&self) -> bool {
        self.location_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location_ping(&mut self, v: super::dota_commonmessages::CDOTAMsg_LocationPing) {
        self.location_ping = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location_ping(&mut self) -> &mut super::dota_commonmessages::CDOTAMsg_LocationPing {
        if self.location_ping.is_none() {
            self.location_ping.set_default();
        };
        self.location_ping.as_mut().unwrap()
    }

    // Take field
    pub fn take_location_ping(&mut self) -> super::dota_commonmessages::CDOTAMsg_LocationPing {
        self.location_ping.take().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_LocationPing::new())
    }

    pub fn get_location_ping(&self) -> &super::dota_commonmessages::CDOTAMsg_LocationPing {
        self.location_ping.as_ref().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_LocationPing::default_instance())
    }

    fn get_location_ping_for_reflect(&self) -> &::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_LocationPing> {
        &self.location_ping
    }

    fn mut_location_ping_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_LocationPing> {
        &mut self.location_ping
    }
}

impl ::protobuf::Message for CDOTAClientMsg_MapPing {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location_ping)?;
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
        if let Some(v) = self.location_ping.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.location_ping.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_MapPing {
    fn new() -> CDOTAClientMsg_MapPing {
        CDOTAClientMsg_MapPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_MapPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_commonmessages::CDOTAMsg_LocationPing>>(
                    "location_ping",
                    CDOTAClientMsg_MapPing::get_location_ping_for_reflect,
                    CDOTAClientMsg_MapPing::mut_location_ping_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_MapPing>(
                    "CDOTAClientMsg_MapPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_MapPing {
    fn clear(&mut self) {
        self.clear_location_ping();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_MapPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_MapPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ItemAlert {
    // message fields
    item_alert: ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_ItemAlert>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ItemAlert {}

impl CDOTAClientMsg_ItemAlert {
    pub fn new() -> CDOTAClientMsg_ItemAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ItemAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ItemAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ItemAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ItemAlert::new)
        }
    }

    // optional .dota.CDOTAMsg_ItemAlert item_alert = 1;

    pub fn clear_item_alert(&mut self) {
        self.item_alert.clear();
    }

    pub fn has_item_alert(&self) -> bool {
        self.item_alert.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_alert(&mut self, v: super::dota_commonmessages::CDOTAMsg_ItemAlert) {
        self.item_alert = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_item_alert(&mut self) -> &mut super::dota_commonmessages::CDOTAMsg_ItemAlert {
        if self.item_alert.is_none() {
            self.item_alert.set_default();
        };
        self.item_alert.as_mut().unwrap()
    }

    // Take field
    pub fn take_item_alert(&mut self) -> super::dota_commonmessages::CDOTAMsg_ItemAlert {
        self.item_alert.take().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_ItemAlert::new())
    }

    pub fn get_item_alert(&self) -> &super::dota_commonmessages::CDOTAMsg_ItemAlert {
        self.item_alert.as_ref().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_ItemAlert::default_instance())
    }

    fn get_item_alert_for_reflect(&self) -> &::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_ItemAlert> {
        &self.item_alert
    }

    fn mut_item_alert_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_ItemAlert> {
        &mut self.item_alert
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ItemAlert {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.item_alert)?;
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
        if let Some(v) = self.item_alert.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_alert.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ItemAlert {
    fn new() -> CDOTAClientMsg_ItemAlert {
        CDOTAClientMsg_ItemAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ItemAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_commonmessages::CDOTAMsg_ItemAlert>>(
                    "item_alert",
                    CDOTAClientMsg_ItemAlert::get_item_alert_for_reflect,
                    CDOTAClientMsg_ItemAlert::mut_item_alert_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ItemAlert>(
                    "CDOTAClientMsg_ItemAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ItemAlert {
    fn clear(&mut self) {
        self.clear_item_alert();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ItemAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ItemAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_EnemyItemAlert {
    // message fields
    item_entindex: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_EnemyItemAlert {}

impl CDOTAClientMsg_EnemyItemAlert {
    pub fn new() -> CDOTAClientMsg_EnemyItemAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_EnemyItemAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_EnemyItemAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_EnemyItemAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_EnemyItemAlert::new)
        }
    }

    // optional uint32 item_entindex = 1;

    pub fn clear_item_entindex(&mut self) {
        self.item_entindex = ::std::option::Option::None;
    }

    pub fn has_item_entindex(&self) -> bool {
        self.item_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_entindex(&mut self, v: u32) {
        self.item_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_item_entindex(&self) -> u32 {
        self.item_entindex.unwrap_or(0)
    }

    fn get_item_entindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_entindex
    }

    fn mut_item_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_entindex
    }
}

impl ::protobuf::Message for CDOTAClientMsg_EnemyItemAlert {
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
                    self.item_entindex = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.item_entindex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_entindex {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_EnemyItemAlert {
    fn new() -> CDOTAClientMsg_EnemyItemAlert {
        CDOTAClientMsg_EnemyItemAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_EnemyItemAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_entindex",
                    CDOTAClientMsg_EnemyItemAlert::get_item_entindex_for_reflect,
                    CDOTAClientMsg_EnemyItemAlert::mut_item_entindex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_EnemyItemAlert>(
                    "CDOTAClientMsg_EnemyItemAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_EnemyItemAlert {
    fn clear(&mut self) {
        self.clear_item_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_EnemyItemAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_EnemyItemAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ModifierAlert {
    // message fields
    buff_internal_index: ::std::option::Option<i32>,
    target_entindex: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ModifierAlert {}

impl CDOTAClientMsg_ModifierAlert {
    pub fn new() -> CDOTAClientMsg_ModifierAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ModifierAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ModifierAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ModifierAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ModifierAlert::new)
        }
    }

    // optional int32 buff_internal_index = 1;

    pub fn clear_buff_internal_index(&mut self) {
        self.buff_internal_index = ::std::option::Option::None;
    }

    pub fn has_buff_internal_index(&self) -> bool {
        self.buff_internal_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buff_internal_index(&mut self, v: i32) {
        self.buff_internal_index = ::std::option::Option::Some(v);
    }

    pub fn get_buff_internal_index(&self) -> i32 {
        self.buff_internal_index.unwrap_or(0)
    }

    fn get_buff_internal_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.buff_internal_index
    }

    fn mut_buff_internal_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.buff_internal_index
    }

    // optional uint32 target_entindex = 2;

    pub fn clear_target_entindex(&mut self) {
        self.target_entindex = ::std::option::Option::None;
    }

    pub fn has_target_entindex(&self) -> bool {
        self.target_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_entindex(&mut self, v: u32) {
        self.target_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_target_entindex(&self) -> u32 {
        self.target_entindex.unwrap_or(0)
    }

    fn get_target_entindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_entindex
    }

    fn mut_target_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_entindex
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ModifierAlert {
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
                    self.buff_internal_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.target_entindex = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.buff_internal_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target_entindex {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.buff_internal_index {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.target_entindex {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ModifierAlert {
    fn new() -> CDOTAClientMsg_ModifierAlert {
        CDOTAClientMsg_ModifierAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ModifierAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "buff_internal_index",
                    CDOTAClientMsg_ModifierAlert::get_buff_internal_index_for_reflect,
                    CDOTAClientMsg_ModifierAlert::mut_buff_internal_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_entindex",
                    CDOTAClientMsg_ModifierAlert::get_target_entindex_for_reflect,
                    CDOTAClientMsg_ModifierAlert::mut_target_entindex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ModifierAlert>(
                    "CDOTAClientMsg_ModifierAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ModifierAlert {
    fn clear(&mut self) {
        self.clear_buff_internal_index();
        self.clear_target_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ModifierAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ModifierAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ClickedBuff {
    // message fields
    buff_internal_index: ::std::option::Option<i32>,
    target_entindex: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ClickedBuff {}

impl CDOTAClientMsg_ClickedBuff {
    pub fn new() -> CDOTAClientMsg_ClickedBuff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ClickedBuff {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ClickedBuff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ClickedBuff,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ClickedBuff::new)
        }
    }

    // optional int32 buff_internal_index = 1;

    pub fn clear_buff_internal_index(&mut self) {
        self.buff_internal_index = ::std::option::Option::None;
    }

    pub fn has_buff_internal_index(&self) -> bool {
        self.buff_internal_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buff_internal_index(&mut self, v: i32) {
        self.buff_internal_index = ::std::option::Option::Some(v);
    }

    pub fn get_buff_internal_index(&self) -> i32 {
        self.buff_internal_index.unwrap_or(0)
    }

    fn get_buff_internal_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.buff_internal_index
    }

    fn mut_buff_internal_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.buff_internal_index
    }

    // optional uint32 target_entindex = 2;

    pub fn clear_target_entindex(&mut self) {
        self.target_entindex = ::std::option::Option::None;
    }

    pub fn has_target_entindex(&self) -> bool {
        self.target_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_entindex(&mut self, v: u32) {
        self.target_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_target_entindex(&self) -> u32 {
        self.target_entindex.unwrap_or(0)
    }

    fn get_target_entindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_entindex
    }

    fn mut_target_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_entindex
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ClickedBuff {
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
                    self.buff_internal_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.target_entindex = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.buff_internal_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target_entindex {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.buff_internal_index {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.target_entindex {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ClickedBuff {
    fn new() -> CDOTAClientMsg_ClickedBuff {
        CDOTAClientMsg_ClickedBuff::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ClickedBuff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "buff_internal_index",
                    CDOTAClientMsg_ClickedBuff::get_buff_internal_index_for_reflect,
                    CDOTAClientMsg_ClickedBuff::mut_buff_internal_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_entindex",
                    CDOTAClientMsg_ClickedBuff::get_target_entindex_for_reflect,
                    CDOTAClientMsg_ClickedBuff::mut_target_entindex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ClickedBuff>(
                    "CDOTAClientMsg_ClickedBuff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ClickedBuff {
    fn clear(&mut self) {
        self.clear_buff_internal_index();
        self.clear_target_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ClickedBuff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ClickedBuff {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_HPManaAlert {
    // message fields
    target_entindex: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_HPManaAlert {}

impl CDOTAClientMsg_HPManaAlert {
    pub fn new() -> CDOTAClientMsg_HPManaAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_HPManaAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_HPManaAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_HPManaAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_HPManaAlert::new)
        }
    }

    // optional uint32 target_entindex = 1;

    pub fn clear_target_entindex(&mut self) {
        self.target_entindex = ::std::option::Option::None;
    }

    pub fn has_target_entindex(&self) -> bool {
        self.target_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_entindex(&mut self, v: u32) {
        self.target_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_target_entindex(&self) -> u32 {
        self.target_entindex.unwrap_or(0)
    }

    fn get_target_entindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_entindex
    }

    fn mut_target_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_entindex
    }
}

impl ::protobuf::Message for CDOTAClientMsg_HPManaAlert {
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
                    self.target_entindex = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.target_entindex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.target_entindex {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_HPManaAlert {
    fn new() -> CDOTAClientMsg_HPManaAlert {
        CDOTAClientMsg_HPManaAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_HPManaAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_entindex",
                    CDOTAClientMsg_HPManaAlert::get_target_entindex_for_reflect,
                    CDOTAClientMsg_HPManaAlert::mut_target_entindex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_HPManaAlert>(
                    "CDOTAClientMsg_HPManaAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_HPManaAlert {
    fn clear(&mut self) {
        self.clear_target_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_HPManaAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_HPManaAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_GlyphAlert {
    // message fields
    negative: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_GlyphAlert {}

impl CDOTAClientMsg_GlyphAlert {
    pub fn new() -> CDOTAClientMsg_GlyphAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_GlyphAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_GlyphAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_GlyphAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_GlyphAlert::new)
        }
    }

    // optional bool negative = 1;

    pub fn clear_negative(&mut self) {
        self.negative = ::std::option::Option::None;
    }

    pub fn has_negative(&self) -> bool {
        self.negative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_negative(&mut self, v: bool) {
        self.negative = ::std::option::Option::Some(v);
    }

    pub fn get_negative(&self) -> bool {
        self.negative.unwrap_or(false)
    }

    fn get_negative_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.negative
    }

    fn mut_negative_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.negative
    }
}

impl ::protobuf::Message for CDOTAClientMsg_GlyphAlert {
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
                    self.negative = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.negative {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.negative {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_GlyphAlert {
    fn new() -> CDOTAClientMsg_GlyphAlert {
        CDOTAClientMsg_GlyphAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_GlyphAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "negative",
                    CDOTAClientMsg_GlyphAlert::get_negative_for_reflect,
                    CDOTAClientMsg_GlyphAlert::mut_negative_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_GlyphAlert>(
                    "CDOTAClientMsg_GlyphAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_GlyphAlert {
    fn clear(&mut self) {
        self.clear_negative();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_GlyphAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_GlyphAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_MapLine {
    // message fields
    mapline: ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_MapLine>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_MapLine {}

impl CDOTAClientMsg_MapLine {
    pub fn new() -> CDOTAClientMsg_MapLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_MapLine {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_MapLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_MapLine,
        };
        unsafe {
            instance.get(CDOTAClientMsg_MapLine::new)
        }
    }

    // optional .dota.CDOTAMsg_MapLine mapline = 1;

    pub fn clear_mapline(&mut self) {
        self.mapline.clear();
    }

    pub fn has_mapline(&self) -> bool {
        self.mapline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mapline(&mut self, v: super::dota_commonmessages::CDOTAMsg_MapLine) {
        self.mapline = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mapline(&mut self) -> &mut super::dota_commonmessages::CDOTAMsg_MapLine {
        if self.mapline.is_none() {
            self.mapline.set_default();
        };
        self.mapline.as_mut().unwrap()
    }

    // Take field
    pub fn take_mapline(&mut self) -> super::dota_commonmessages::CDOTAMsg_MapLine {
        self.mapline.take().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_MapLine::new())
    }

    pub fn get_mapline(&self) -> &super::dota_commonmessages::CDOTAMsg_MapLine {
        self.mapline.as_ref().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_MapLine::default_instance())
    }

    fn get_mapline_for_reflect(&self) -> &::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_MapLine> {
        &self.mapline
    }

    fn mut_mapline_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_MapLine> {
        &mut self.mapline
    }
}

impl ::protobuf::Message for CDOTAClientMsg_MapLine {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mapline)?;
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
        if let Some(v) = self.mapline.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mapline.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_MapLine {
    fn new() -> CDOTAClientMsg_MapLine {
        CDOTAClientMsg_MapLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_MapLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_commonmessages::CDOTAMsg_MapLine>>(
                    "mapline",
                    CDOTAClientMsg_MapLine::get_mapline_for_reflect,
                    CDOTAClientMsg_MapLine::mut_mapline_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_MapLine>(
                    "CDOTAClientMsg_MapLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_MapLine {
    fn clear(&mut self) {
        self.clear_mapline();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_MapLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_MapLine {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_AspectRatio {
    // message fields
    ratio: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_AspectRatio {}

impl CDOTAClientMsg_AspectRatio {
    pub fn new() -> CDOTAClientMsg_AspectRatio {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_AspectRatio {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_AspectRatio> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_AspectRatio,
        };
        unsafe {
            instance.get(CDOTAClientMsg_AspectRatio::new)
        }
    }

    // optional float ratio = 1;

    pub fn clear_ratio(&mut self) {
        self.ratio = ::std::option::Option::None;
    }

    pub fn has_ratio(&self) -> bool {
        self.ratio.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ratio(&mut self, v: f32) {
        self.ratio = ::std::option::Option::Some(v);
    }

    pub fn get_ratio(&self) -> f32 {
        self.ratio.unwrap_or(0.)
    }

    fn get_ratio_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.ratio
    }

    fn mut_ratio_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.ratio
    }
}

impl ::protobuf::Message for CDOTAClientMsg_AspectRatio {
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
                    self.ratio = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ratio {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ratio {
            os.write_float(1, v)?;
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_AspectRatio {
    fn new() -> CDOTAClientMsg_AspectRatio {
        CDOTAClientMsg_AspectRatio::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_AspectRatio>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "ratio",
                    CDOTAClientMsg_AspectRatio::get_ratio_for_reflect,
                    CDOTAClientMsg_AspectRatio::mut_ratio_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_AspectRatio>(
                    "CDOTAClientMsg_AspectRatio",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_AspectRatio {
    fn clear(&mut self) {
        self.clear_ratio();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_AspectRatio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_AspectRatio {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_UnitsAutoAttackMode {
    // message fields
    mode: ::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode_EMode>,
    unit_type: ::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode_EUnitType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_UnitsAutoAttackMode {}

impl CDOTAClientMsg_UnitsAutoAttackMode {
    pub fn new() -> CDOTAClientMsg_UnitsAutoAttackMode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_UnitsAutoAttackMode {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_UnitsAutoAttackMode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_UnitsAutoAttackMode,
        };
        unsafe {
            instance.get(CDOTAClientMsg_UnitsAutoAttackMode::new)
        }
    }

    // optional .dota.CDOTAClientMsg_UnitsAutoAttackMode.EMode mode = 1;

    pub fn clear_mode(&mut self) {
        self.mode = ::std::option::Option::None;
    }

    pub fn has_mode(&self) -> bool {
        self.mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode(&mut self, v: CDOTAClientMsg_UnitsAutoAttackMode_EMode) {
        self.mode = ::std::option::Option::Some(v);
    }

    pub fn get_mode(&self) -> CDOTAClientMsg_UnitsAutoAttackMode_EMode {
        self.mode.unwrap_or(CDOTAClientMsg_UnitsAutoAttackMode_EMode::INVALID)
    }

    fn get_mode_for_reflect(&self) -> &::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode_EMode> {
        &self.mode
    }

    fn mut_mode_for_reflect(&mut self) -> &mut ::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode_EMode> {
        &mut self.mode
    }

    // optional .dota.CDOTAClientMsg_UnitsAutoAttackMode.EUnitType unit_type = 2;

    pub fn clear_unit_type(&mut self) {
        self.unit_type = ::std::option::Option::None;
    }

    pub fn has_unit_type(&self) -> bool {
        self.unit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_type(&mut self, v: CDOTAClientMsg_UnitsAutoAttackMode_EUnitType) {
        self.unit_type = ::std::option::Option::Some(v);
    }

    pub fn get_unit_type(&self) -> CDOTAClientMsg_UnitsAutoAttackMode_EUnitType {
        self.unit_type.unwrap_or(CDOTAClientMsg_UnitsAutoAttackMode_EUnitType::NORMAL)
    }

    fn get_unit_type_for_reflect(&self) -> &::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode_EUnitType> {
        &self.unit_type
    }

    fn mut_unit_type_for_reflect(&mut self) -> &mut ::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode_EUnitType> {
        &mut self.unit_type
    }
}

impl ::protobuf::Message for CDOTAClientMsg_UnitsAutoAttackMode {
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
                    self.mode = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.unit_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.mode {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.unit_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.unit_type {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_UnitsAutoAttackMode {
    fn new() -> CDOTAClientMsg_UnitsAutoAttackMode {
        CDOTAClientMsg_UnitsAutoAttackMode::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CDOTAClientMsg_UnitsAutoAttackMode_EMode>>(
                    "mode",
                    CDOTAClientMsg_UnitsAutoAttackMode::get_mode_for_reflect,
                    CDOTAClientMsg_UnitsAutoAttackMode::mut_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CDOTAClientMsg_UnitsAutoAttackMode_EUnitType>>(
                    "unit_type",
                    CDOTAClientMsg_UnitsAutoAttackMode::get_unit_type_for_reflect,
                    CDOTAClientMsg_UnitsAutoAttackMode::mut_unit_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_UnitsAutoAttackMode>(
                    "CDOTAClientMsg_UnitsAutoAttackMode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_UnitsAutoAttackMode {
    fn clear(&mut self) {
        self.clear_mode();
        self.clear_unit_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_UnitsAutoAttackMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_UnitsAutoAttackMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CDOTAClientMsg_UnitsAutoAttackMode_EMode {
    INVALID = -1,
    NEVER = 0,
    AFTER_SPELLCAST = 1,
    ALWAYS = 2,
}

impl ::protobuf::ProtobufEnum for CDOTAClientMsg_UnitsAutoAttackMode_EMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode_EMode> {
        match value {
            -1 => ::std::option::Option::Some(CDOTAClientMsg_UnitsAutoAttackMode_EMode::INVALID),
            0 => ::std::option::Option::Some(CDOTAClientMsg_UnitsAutoAttackMode_EMode::NEVER),
            1 => ::std::option::Option::Some(CDOTAClientMsg_UnitsAutoAttackMode_EMode::AFTER_SPELLCAST),
            2 => ::std::option::Option::Some(CDOTAClientMsg_UnitsAutoAttackMode_EMode::ALWAYS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CDOTAClientMsg_UnitsAutoAttackMode_EMode] = &[
            CDOTAClientMsg_UnitsAutoAttackMode_EMode::INVALID,
            CDOTAClientMsg_UnitsAutoAttackMode_EMode::NEVER,
            CDOTAClientMsg_UnitsAutoAttackMode_EMode::AFTER_SPELLCAST,
            CDOTAClientMsg_UnitsAutoAttackMode_EMode::ALWAYS,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CDOTAClientMsg_UnitsAutoAttackMode_EMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CDOTAClientMsg_UnitsAutoAttackMode_EMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CDOTAClientMsg_UnitsAutoAttackMode_EMode {
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_UnitsAutoAttackMode_EMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CDOTAClientMsg_UnitsAutoAttackMode_EUnitType {
    NORMAL = 0,
    SUMMONED = 1,
}

impl ::protobuf::ProtobufEnum for CDOTAClientMsg_UnitsAutoAttackMode_EUnitType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CDOTAClientMsg_UnitsAutoAttackMode_EUnitType> {
        match value {
            0 => ::std::option::Option::Some(CDOTAClientMsg_UnitsAutoAttackMode_EUnitType::NORMAL),
            1 => ::std::option::Option::Some(CDOTAClientMsg_UnitsAutoAttackMode_EUnitType::SUMMONED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CDOTAClientMsg_UnitsAutoAttackMode_EUnitType] = &[
            CDOTAClientMsg_UnitsAutoAttackMode_EUnitType::NORMAL,
            CDOTAClientMsg_UnitsAutoAttackMode_EUnitType::SUMMONED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CDOTAClientMsg_UnitsAutoAttackMode_EUnitType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CDOTAClientMsg_UnitsAutoAttackMode_EUnitType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CDOTAClientMsg_UnitsAutoAttackMode_EUnitType {
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_UnitsAutoAttackMode_EUnitType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_UnitsAutoAttackAfterSpell {
    // message fields
    enabled: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_UnitsAutoAttackAfterSpell {}

impl CDOTAClientMsg_UnitsAutoAttackAfterSpell {
    pub fn new() -> CDOTAClientMsg_UnitsAutoAttackAfterSpell {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_UnitsAutoAttackAfterSpell {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_UnitsAutoAttackAfterSpell> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_UnitsAutoAttackAfterSpell,
        };
        unsafe {
            instance.get(CDOTAClientMsg_UnitsAutoAttackAfterSpell::new)
        }
    }

    // optional bool enabled = 1;

    pub fn clear_enabled(&mut self) {
        self.enabled = ::std::option::Option::None;
    }

    pub fn has_enabled(&self) -> bool {
        self.enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = ::std::option::Option::Some(v);
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.unwrap_or(false)
    }

    fn get_enabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }

    fn mut_enabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.enabled
    }
}

impl ::protobuf::Message for CDOTAClientMsg_UnitsAutoAttackAfterSpell {
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
                    self.enabled = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.enabled {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.enabled {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_UnitsAutoAttackAfterSpell {
    fn new() -> CDOTAClientMsg_UnitsAutoAttackAfterSpell {
        CDOTAClientMsg_UnitsAutoAttackAfterSpell::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_UnitsAutoAttackAfterSpell>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "enabled",
                    CDOTAClientMsg_UnitsAutoAttackAfterSpell::get_enabled_for_reflect,
                    CDOTAClientMsg_UnitsAutoAttackAfterSpell::mut_enabled_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_UnitsAutoAttackAfterSpell>(
                    "CDOTAClientMsg_UnitsAutoAttackAfterSpell",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_UnitsAutoAttackAfterSpell {
    fn clear(&mut self) {
        self.clear_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_UnitsAutoAttackAfterSpell {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_UnitsAutoAttackAfterSpell {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_TeleportRequiresHalt {
    // message fields
    enabled: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_TeleportRequiresHalt {}

impl CDOTAClientMsg_TeleportRequiresHalt {
    pub fn new() -> CDOTAClientMsg_TeleportRequiresHalt {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_TeleportRequiresHalt {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_TeleportRequiresHalt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_TeleportRequiresHalt,
        };
        unsafe {
            instance.get(CDOTAClientMsg_TeleportRequiresHalt::new)
        }
    }

    // optional bool enabled = 1;

    pub fn clear_enabled(&mut self) {
        self.enabled = ::std::option::Option::None;
    }

    pub fn has_enabled(&self) -> bool {
        self.enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = ::std::option::Option::Some(v);
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.unwrap_or(false)
    }

    fn get_enabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }

    fn mut_enabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.enabled
    }
}

impl ::protobuf::Message for CDOTAClientMsg_TeleportRequiresHalt {
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
                    self.enabled = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.enabled {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.enabled {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_TeleportRequiresHalt {
    fn new() -> CDOTAClientMsg_TeleportRequiresHalt {
        CDOTAClientMsg_TeleportRequiresHalt::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_TeleportRequiresHalt>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "enabled",
                    CDOTAClientMsg_TeleportRequiresHalt::get_enabled_for_reflect,
                    CDOTAClientMsg_TeleportRequiresHalt::mut_enabled_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_TeleportRequiresHalt>(
                    "CDOTAClientMsg_TeleportRequiresHalt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_TeleportRequiresHalt {
    fn clear(&mut self) {
        self.clear_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_TeleportRequiresHalt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_TeleportRequiresHalt {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_SearchString {
    // message fields
    search: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_SearchString {}

impl CDOTAClientMsg_SearchString {
    pub fn new() -> CDOTAClientMsg_SearchString {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_SearchString {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_SearchString> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_SearchString,
        };
        unsafe {
            instance.get(CDOTAClientMsg_SearchString::new)
        }
    }

    // optional string search = 1;

    pub fn clear_search(&mut self) {
        self.search.clear();
    }

    pub fn has_search(&self) -> bool {
        self.search.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search(&mut self, v: ::std::string::String) {
        self.search = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search(&mut self) -> &mut ::std::string::String {
        if self.search.is_none() {
            self.search.set_default();
        };
        self.search.as_mut().unwrap()
    }

    // Take field
    pub fn take_search(&mut self) -> ::std::string::String {
        self.search.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_search(&self) -> &str {
        match self.search.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_search_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.search
    }

    fn mut_search_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.search
    }
}

impl ::protobuf::Message for CDOTAClientMsg_SearchString {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.search)?;
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
        if let Some(v) = self.search.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.search.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_SearchString {
    fn new() -> CDOTAClientMsg_SearchString {
        CDOTAClientMsg_SearchString::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_SearchString>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "search",
                    CDOTAClientMsg_SearchString::get_search_for_reflect,
                    CDOTAClientMsg_SearchString::mut_search_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_SearchString>(
                    "CDOTAClientMsg_SearchString",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_SearchString {
    fn clear(&mut self) {
        self.clear_search();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_SearchString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_SearchString {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_Pause {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_Pause {}

impl CDOTAClientMsg_Pause {
    pub fn new() -> CDOTAClientMsg_Pause {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_Pause {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_Pause> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_Pause,
        };
        unsafe {
            instance.get(CDOTAClientMsg_Pause::new)
        }
    }
}

impl ::protobuf::Message for CDOTAClientMsg_Pause {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_Pause {
    fn new() -> CDOTAClientMsg_Pause {
        CDOTAClientMsg_Pause::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_Pause>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_Pause>(
                    "CDOTAClientMsg_Pause",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_Pause {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_Pause {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_Pause {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ShopViewMode {
    // message fields
    mode: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ShopViewMode {}

impl CDOTAClientMsg_ShopViewMode {
    pub fn new() -> CDOTAClientMsg_ShopViewMode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ShopViewMode {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ShopViewMode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ShopViewMode,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ShopViewMode::new)
        }
    }

    // optional uint32 mode = 1;

    pub fn clear_mode(&mut self) {
        self.mode = ::std::option::Option::None;
    }

    pub fn has_mode(&self) -> bool {
        self.mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode(&mut self, v: u32) {
        self.mode = ::std::option::Option::Some(v);
    }

    pub fn get_mode(&self) -> u32 {
        self.mode.unwrap_or(0)
    }

    fn get_mode_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.mode
    }

    fn mut_mode_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.mode
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ShopViewMode {
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
                    self.mode = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.mode {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ShopViewMode {
    fn new() -> CDOTAClientMsg_ShopViewMode {
        CDOTAClientMsg_ShopViewMode::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ShopViewMode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mode",
                    CDOTAClientMsg_ShopViewMode::get_mode_for_reflect,
                    CDOTAClientMsg_ShopViewMode::mut_mode_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ShopViewMode>(
                    "CDOTAClientMsg_ShopViewMode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ShopViewMode {
    fn clear(&mut self) {
        self.clear_mode();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ShopViewMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ShopViewMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_SetUnitShareFlag {
    // message fields
    playerID: ::std::option::Option<u32>,
    flag: ::std::option::Option<u32>,
    state: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_SetUnitShareFlag {}

impl CDOTAClientMsg_SetUnitShareFlag {
    pub fn new() -> CDOTAClientMsg_SetUnitShareFlag {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_SetUnitShareFlag {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_SetUnitShareFlag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_SetUnitShareFlag,
        };
        unsafe {
            instance.get(CDOTAClientMsg_SetUnitShareFlag::new)
        }
    }

    // optional uint32 playerID = 1;

    pub fn clear_playerID(&mut self) {
        self.playerID = ::std::option::Option::None;
    }

    pub fn has_playerID(&self) -> bool {
        self.playerID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerID(&mut self, v: u32) {
        self.playerID = ::std::option::Option::Some(v);
    }

    pub fn get_playerID(&self) -> u32 {
        self.playerID.unwrap_or(0)
    }

    fn get_playerID_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.playerID
    }

    fn mut_playerID_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.playerID
    }

    // optional uint32 flag = 2;

    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None;
    }

    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: u32) {
        self.flag = ::std::option::Option::Some(v);
    }

    pub fn get_flag(&self) -> u32 {
        self.flag.unwrap_or(0)
    }

    fn get_flag_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flag
    }

    fn mut_flag_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flag
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

impl ::protobuf::Message for CDOTAClientMsg_SetUnitShareFlag {
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
                    self.playerID = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flag = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.playerID {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flag {
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
        if let Some(v) = self.playerID {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.flag {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_SetUnitShareFlag {
    fn new() -> CDOTAClientMsg_SetUnitShareFlag {
        CDOTAClientMsg_SetUnitShareFlag::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_SetUnitShareFlag>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "playerID",
                    CDOTAClientMsg_SetUnitShareFlag::get_playerID_for_reflect,
                    CDOTAClientMsg_SetUnitShareFlag::mut_playerID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flag",
                    CDOTAClientMsg_SetUnitShareFlag::get_flag_for_reflect,
                    CDOTAClientMsg_SetUnitShareFlag::mut_flag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "state",
                    CDOTAClientMsg_SetUnitShareFlag::get_state_for_reflect,
                    CDOTAClientMsg_SetUnitShareFlag::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_SetUnitShareFlag>(
                    "CDOTAClientMsg_SetUnitShareFlag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_SetUnitShareFlag {
    fn clear(&mut self) {
        self.clear_playerID();
        self.clear_flag();
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_SetUnitShareFlag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_SetUnitShareFlag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_SwapRequest {
    // message fields
    player_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_SwapRequest {}

impl CDOTAClientMsg_SwapRequest {
    pub fn new() -> CDOTAClientMsg_SwapRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_SwapRequest {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_SwapRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_SwapRequest,
        };
        unsafe {
            instance.get(CDOTAClientMsg_SwapRequest::new)
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }
}

impl ::protobuf::Message for CDOTAClientMsg_SwapRequest {
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
                    self.player_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_SwapRequest {
    fn new() -> CDOTAClientMsg_SwapRequest {
        CDOTAClientMsg_SwapRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_SwapRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    CDOTAClientMsg_SwapRequest::get_player_id_for_reflect,
                    CDOTAClientMsg_SwapRequest::mut_player_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_SwapRequest>(
                    "CDOTAClientMsg_SwapRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_SwapRequest {
    fn clear(&mut self) {
        self.clear_player_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_SwapRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_SwapRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_SwapAccept {
    // message fields
    player_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_SwapAccept {}

impl CDOTAClientMsg_SwapAccept {
    pub fn new() -> CDOTAClientMsg_SwapAccept {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_SwapAccept {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_SwapAccept> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_SwapAccept,
        };
        unsafe {
            instance.get(CDOTAClientMsg_SwapAccept::new)
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }
}

impl ::protobuf::Message for CDOTAClientMsg_SwapAccept {
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
                    self.player_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_SwapAccept {
    fn new() -> CDOTAClientMsg_SwapAccept {
        CDOTAClientMsg_SwapAccept::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_SwapAccept>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    CDOTAClientMsg_SwapAccept::get_player_id_for_reflect,
                    CDOTAClientMsg_SwapAccept::mut_player_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_SwapAccept>(
                    "CDOTAClientMsg_SwapAccept",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_SwapAccept {
    fn clear(&mut self) {
        self.clear_player_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_SwapAccept {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_SwapAccept {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_WorldLine {
    // message fields
    worldline: ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_WorldLine>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_WorldLine {}

impl CDOTAClientMsg_WorldLine {
    pub fn new() -> CDOTAClientMsg_WorldLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_WorldLine {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_WorldLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_WorldLine,
        };
        unsafe {
            instance.get(CDOTAClientMsg_WorldLine::new)
        }
    }

    // optional .dota.CDOTAMsg_WorldLine worldline = 1;

    pub fn clear_worldline(&mut self) {
        self.worldline.clear();
    }

    pub fn has_worldline(&self) -> bool {
        self.worldline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_worldline(&mut self, v: super::dota_commonmessages::CDOTAMsg_WorldLine) {
        self.worldline = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_worldline(&mut self) -> &mut super::dota_commonmessages::CDOTAMsg_WorldLine {
        if self.worldline.is_none() {
            self.worldline.set_default();
        };
        self.worldline.as_mut().unwrap()
    }

    // Take field
    pub fn take_worldline(&mut self) -> super::dota_commonmessages::CDOTAMsg_WorldLine {
        self.worldline.take().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_WorldLine::new())
    }

    pub fn get_worldline(&self) -> &super::dota_commonmessages::CDOTAMsg_WorldLine {
        self.worldline.as_ref().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_WorldLine::default_instance())
    }

    fn get_worldline_for_reflect(&self) -> &::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_WorldLine> {
        &self.worldline
    }

    fn mut_worldline_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_WorldLine> {
        &mut self.worldline
    }
}

impl ::protobuf::Message for CDOTAClientMsg_WorldLine {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.worldline)?;
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
        if let Some(v) = self.worldline.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.worldline.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_WorldLine {
    fn new() -> CDOTAClientMsg_WorldLine {
        CDOTAClientMsg_WorldLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_WorldLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_commonmessages::CDOTAMsg_WorldLine>>(
                    "worldline",
                    CDOTAClientMsg_WorldLine::get_worldline_for_reflect,
                    CDOTAClientMsg_WorldLine::mut_worldline_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_WorldLine>(
                    "CDOTAClientMsg_WorldLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_WorldLine {
    fn clear(&mut self) {
        self.clear_worldline();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_WorldLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_WorldLine {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_RequestGraphUpdate {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_RequestGraphUpdate {}

impl CDOTAClientMsg_RequestGraphUpdate {
    pub fn new() -> CDOTAClientMsg_RequestGraphUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_RequestGraphUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_RequestGraphUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_RequestGraphUpdate,
        };
        unsafe {
            instance.get(CDOTAClientMsg_RequestGraphUpdate::new)
        }
    }
}

impl ::protobuf::Message for CDOTAClientMsg_RequestGraphUpdate {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_RequestGraphUpdate {
    fn new() -> CDOTAClientMsg_RequestGraphUpdate {
        CDOTAClientMsg_RequestGraphUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_RequestGraphUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_RequestGraphUpdate>(
                    "CDOTAClientMsg_RequestGraphUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_RequestGraphUpdate {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_RequestGraphUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_RequestGraphUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ChatWheel {
    // message fields
    chat_message: ::std::option::Option<super::dota_commonmessages::EDOTAChatWheelMessage>,
    param_hero_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ChatWheel {}

impl CDOTAClientMsg_ChatWheel {
    pub fn new() -> CDOTAClientMsg_ChatWheel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ChatWheel {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ChatWheel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ChatWheel,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ChatWheel::new)
        }
    }

    // optional .dota.EDOTAChatWheelMessage chat_message = 1;

    pub fn clear_chat_message(&mut self) {
        self.chat_message = ::std::option::Option::None;
    }

    pub fn has_chat_message(&self) -> bool {
        self.chat_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chat_message(&mut self, v: super::dota_commonmessages::EDOTAChatWheelMessage) {
        self.chat_message = ::std::option::Option::Some(v);
    }

    pub fn get_chat_message(&self) -> super::dota_commonmessages::EDOTAChatWheelMessage {
        self.chat_message.unwrap_or(super::dota_commonmessages::EDOTAChatWheelMessage::k_EDOTA_CW_Ok)
    }

    fn get_chat_message_for_reflect(&self) -> &::std::option::Option<super::dota_commonmessages::EDOTAChatWheelMessage> {
        &self.chat_message
    }

    fn mut_chat_message_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_commonmessages::EDOTAChatWheelMessage> {
        &mut self.chat_message
    }

    // optional uint32 param_hero_id = 2;

    pub fn clear_param_hero_id(&mut self) {
        self.param_hero_id = ::std::option::Option::None;
    }

    pub fn has_param_hero_id(&self) -> bool {
        self.param_hero_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_param_hero_id(&mut self, v: u32) {
        self.param_hero_id = ::std::option::Option::Some(v);
    }

    pub fn get_param_hero_id(&self) -> u32 {
        self.param_hero_id.unwrap_or(0)
    }

    fn get_param_hero_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.param_hero_id
    }

    fn mut_param_hero_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.param_hero_id
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ChatWheel {
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
                    self.chat_message = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.param_hero_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.chat_message {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.param_hero_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.chat_message {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.param_hero_id {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ChatWheel {
    fn new() -> CDOTAClientMsg_ChatWheel {
        CDOTAClientMsg_ChatWheel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ChatWheel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_commonmessages::EDOTAChatWheelMessage>>(
                    "chat_message",
                    CDOTAClientMsg_ChatWheel::get_chat_message_for_reflect,
                    CDOTAClientMsg_ChatWheel::mut_chat_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "param_hero_id",
                    CDOTAClientMsg_ChatWheel::get_param_hero_id_for_reflect,
                    CDOTAClientMsg_ChatWheel::mut_param_hero_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ChatWheel>(
                    "CDOTAClientMsg_ChatWheel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ChatWheel {
    fn clear(&mut self) {
        self.clear_chat_message();
        self.clear_param_hero_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ChatWheel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ChatWheel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_SendStatPopup {
    // message fields
    statpopup: ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_SendStatPopup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_SendStatPopup {}

impl CDOTAClientMsg_SendStatPopup {
    pub fn new() -> CDOTAClientMsg_SendStatPopup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_SendStatPopup {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_SendStatPopup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_SendStatPopup,
        };
        unsafe {
            instance.get(CDOTAClientMsg_SendStatPopup::new)
        }
    }

    // optional .dota.CDOTAMsg_SendStatPopup statpopup = 1;

    pub fn clear_statpopup(&mut self) {
        self.statpopup.clear();
    }

    pub fn has_statpopup(&self) -> bool {
        self.statpopup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_statpopup(&mut self, v: super::dota_commonmessages::CDOTAMsg_SendStatPopup) {
        self.statpopup = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_statpopup(&mut self) -> &mut super::dota_commonmessages::CDOTAMsg_SendStatPopup {
        if self.statpopup.is_none() {
            self.statpopup.set_default();
        };
        self.statpopup.as_mut().unwrap()
    }

    // Take field
    pub fn take_statpopup(&mut self) -> super::dota_commonmessages::CDOTAMsg_SendStatPopup {
        self.statpopup.take().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_SendStatPopup::new())
    }

    pub fn get_statpopup(&self) -> &super::dota_commonmessages::CDOTAMsg_SendStatPopup {
        self.statpopup.as_ref().unwrap_or_else(|| super::dota_commonmessages::CDOTAMsg_SendStatPopup::default_instance())
    }

    fn get_statpopup_for_reflect(&self) -> &::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_SendStatPopup> {
        &self.statpopup
    }

    fn mut_statpopup_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::dota_commonmessages::CDOTAMsg_SendStatPopup> {
        &mut self.statpopup
    }
}

impl ::protobuf::Message for CDOTAClientMsg_SendStatPopup {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.statpopup)?;
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
        if let Some(v) = self.statpopup.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.statpopup.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_SendStatPopup {
    fn new() -> CDOTAClientMsg_SendStatPopup {
        CDOTAClientMsg_SendStatPopup::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_SendStatPopup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_commonmessages::CDOTAMsg_SendStatPopup>>(
                    "statpopup",
                    CDOTAClientMsg_SendStatPopup::get_statpopup_for_reflect,
                    CDOTAClientMsg_SendStatPopup::mut_statpopup_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_SendStatPopup>(
                    "CDOTAClientMsg_SendStatPopup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_SendStatPopup {
    fn clear(&mut self) {
        self.clear_statpopup();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_SendStatPopup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_SendStatPopup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_BeginLastHitChallenge {
    // message fields
    chosen_lane: ::std::option::Option<u32>,
    helper_enabled: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_BeginLastHitChallenge {}

impl CDOTAClientMsg_BeginLastHitChallenge {
    pub fn new() -> CDOTAClientMsg_BeginLastHitChallenge {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_BeginLastHitChallenge {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_BeginLastHitChallenge> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_BeginLastHitChallenge,
        };
        unsafe {
            instance.get(CDOTAClientMsg_BeginLastHitChallenge::new)
        }
    }

    // optional uint32 chosen_lane = 1;

    pub fn clear_chosen_lane(&mut self) {
        self.chosen_lane = ::std::option::Option::None;
    }

    pub fn has_chosen_lane(&self) -> bool {
        self.chosen_lane.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chosen_lane(&mut self, v: u32) {
        self.chosen_lane = ::std::option::Option::Some(v);
    }

    pub fn get_chosen_lane(&self) -> u32 {
        self.chosen_lane.unwrap_or(0)
    }

    fn get_chosen_lane_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.chosen_lane
    }

    fn mut_chosen_lane_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.chosen_lane
    }

    // optional bool helper_enabled = 2;

    pub fn clear_helper_enabled(&mut self) {
        self.helper_enabled = ::std::option::Option::None;
    }

    pub fn has_helper_enabled(&self) -> bool {
        self.helper_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_helper_enabled(&mut self, v: bool) {
        self.helper_enabled = ::std::option::Option::Some(v);
    }

    pub fn get_helper_enabled(&self) -> bool {
        self.helper_enabled.unwrap_or(false)
    }

    fn get_helper_enabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.helper_enabled
    }

    fn mut_helper_enabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.helper_enabled
    }
}

impl ::protobuf::Message for CDOTAClientMsg_BeginLastHitChallenge {
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
                    self.chosen_lane = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.helper_enabled = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.chosen_lane {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.helper_enabled {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.chosen_lane {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.helper_enabled {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_BeginLastHitChallenge {
    fn new() -> CDOTAClientMsg_BeginLastHitChallenge {
        CDOTAClientMsg_BeginLastHitChallenge::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_BeginLastHitChallenge>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "chosen_lane",
                    CDOTAClientMsg_BeginLastHitChallenge::get_chosen_lane_for_reflect,
                    CDOTAClientMsg_BeginLastHitChallenge::mut_chosen_lane_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "helper_enabled",
                    CDOTAClientMsg_BeginLastHitChallenge::get_helper_enabled_for_reflect,
                    CDOTAClientMsg_BeginLastHitChallenge::mut_helper_enabled_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_BeginLastHitChallenge>(
                    "CDOTAClientMsg_BeginLastHitChallenge",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_BeginLastHitChallenge {
    fn clear(&mut self) {
        self.clear_chosen_lane();
        self.clear_helper_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_BeginLastHitChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_BeginLastHitChallenge {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_UpdateQuickBuyItem {
    // message fields
    item_type: ::std::option::Option<i32>,
    purchasable: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_UpdateQuickBuyItem {}

impl CDOTAClientMsg_UpdateQuickBuyItem {
    pub fn new() -> CDOTAClientMsg_UpdateQuickBuyItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_UpdateQuickBuyItem {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_UpdateQuickBuyItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_UpdateQuickBuyItem,
        };
        unsafe {
            instance.get(CDOTAClientMsg_UpdateQuickBuyItem::new)
        }
    }

    // optional int32 item_type = 1;

    pub fn clear_item_type(&mut self) {
        self.item_type = ::std::option::Option::None;
    }

    pub fn has_item_type(&self) -> bool {
        self.item_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_type(&mut self, v: i32) {
        self.item_type = ::std::option::Option::Some(v);
    }

    pub fn get_item_type(&self) -> i32 {
        self.item_type.unwrap_or(0)
    }

    fn get_item_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.item_type
    }

    fn mut_item_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.item_type
    }

    // optional bool purchasable = 2;

    pub fn clear_purchasable(&mut self) {
        self.purchasable = ::std::option::Option::None;
    }

    pub fn has_purchasable(&self) -> bool {
        self.purchasable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_purchasable(&mut self, v: bool) {
        self.purchasable = ::std::option::Option::Some(v);
    }

    pub fn get_purchasable(&self) -> bool {
        self.purchasable.unwrap_or(false)
    }

    fn get_purchasable_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.purchasable
    }

    fn mut_purchasable_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.purchasable
    }
}

impl ::protobuf::Message for CDOTAClientMsg_UpdateQuickBuyItem {
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
                    self.item_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.purchasable = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.item_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.purchasable {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_type {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.purchasable {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_UpdateQuickBuyItem {
    fn new() -> CDOTAClientMsg_UpdateQuickBuyItem {
        CDOTAClientMsg_UpdateQuickBuyItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_UpdateQuickBuyItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "item_type",
                    CDOTAClientMsg_UpdateQuickBuyItem::get_item_type_for_reflect,
                    CDOTAClientMsg_UpdateQuickBuyItem::mut_item_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "purchasable",
                    CDOTAClientMsg_UpdateQuickBuyItem::get_purchasable_for_reflect,
                    CDOTAClientMsg_UpdateQuickBuyItem::mut_purchasable_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_UpdateQuickBuyItem>(
                    "CDOTAClientMsg_UpdateQuickBuyItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_UpdateQuickBuyItem {
    fn clear(&mut self) {
        self.clear_item_type();
        self.clear_purchasable();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_UpdateQuickBuyItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_UpdateQuickBuyItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_UpdateQuickBuy {
    // message fields
    items: ::protobuf::RepeatedField<CDOTAClientMsg_UpdateQuickBuyItem>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_UpdateQuickBuy {}

impl CDOTAClientMsg_UpdateQuickBuy {
    pub fn new() -> CDOTAClientMsg_UpdateQuickBuy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_UpdateQuickBuy {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_UpdateQuickBuy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_UpdateQuickBuy,
        };
        unsafe {
            instance.get(CDOTAClientMsg_UpdateQuickBuy::new)
        }
    }

    // repeated .dota.CDOTAClientMsg_UpdateQuickBuyItem items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CDOTAClientMsg_UpdateQuickBuyItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAClientMsg_UpdateQuickBuyItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CDOTAClientMsg_UpdateQuickBuyItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CDOTAClientMsg_UpdateQuickBuyItem] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAClientMsg_UpdateQuickBuyItem> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAClientMsg_UpdateQuickBuyItem> {
        &mut self.items
    }
}

impl ::protobuf::Message for CDOTAClientMsg_UpdateQuickBuy {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_UpdateQuickBuy {
    fn new() -> CDOTAClientMsg_UpdateQuickBuy {
        CDOTAClientMsg_UpdateQuickBuy::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_UpdateQuickBuy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAClientMsg_UpdateQuickBuyItem>>(
                    "items",
                    CDOTAClientMsg_UpdateQuickBuy::get_items_for_reflect,
                    CDOTAClientMsg_UpdateQuickBuy::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_UpdateQuickBuy>(
                    "CDOTAClientMsg_UpdateQuickBuy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_UpdateQuickBuy {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_UpdateQuickBuy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_UpdateQuickBuy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_RecordVote {
    // message fields
    choice_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_RecordVote {}

impl CDOTAClientMsg_RecordVote {
    pub fn new() -> CDOTAClientMsg_RecordVote {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_RecordVote {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_RecordVote> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_RecordVote,
        };
        unsafe {
            instance.get(CDOTAClientMsg_RecordVote::new)
        }
    }

    // optional int32 choice_index = 1;

    pub fn clear_choice_index(&mut self) {
        self.choice_index = ::std::option::Option::None;
    }

    pub fn has_choice_index(&self) -> bool {
        self.choice_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_choice_index(&mut self, v: i32) {
        self.choice_index = ::std::option::Option::Some(v);
    }

    pub fn get_choice_index(&self) -> i32 {
        self.choice_index.unwrap_or(0)
    }

    fn get_choice_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.choice_index
    }

    fn mut_choice_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.choice_index
    }
}

impl ::protobuf::Message for CDOTAClientMsg_RecordVote {
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
                    self.choice_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.choice_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.choice_index {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_RecordVote {
    fn new() -> CDOTAClientMsg_RecordVote {
        CDOTAClientMsg_RecordVote::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_RecordVote>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "choice_index",
                    CDOTAClientMsg_RecordVote::get_choice_index_for_reflect,
                    CDOTAClientMsg_RecordVote::mut_choice_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_RecordVote>(
                    "CDOTAClientMsg_RecordVote",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_RecordVote {
    fn clear(&mut self) {
        self.clear_choice_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_RecordVote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_RecordVote {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_WillPurchaseAlert {
    // message fields
    itemid: ::std::option::Option<i32>,
    gold_remaining: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_WillPurchaseAlert {}

impl CDOTAClientMsg_WillPurchaseAlert {
    pub fn new() -> CDOTAClientMsg_WillPurchaseAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_WillPurchaseAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_WillPurchaseAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_WillPurchaseAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_WillPurchaseAlert::new)
        }
    }

    // optional int32 itemid = 1;

    pub fn clear_itemid(&mut self) {
        self.itemid = ::std::option::Option::None;
    }

    pub fn has_itemid(&self) -> bool {
        self.itemid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itemid(&mut self, v: i32) {
        self.itemid = ::std::option::Option::Some(v);
    }

    pub fn get_itemid(&self) -> i32 {
        self.itemid.unwrap_or(0)
    }

    fn get_itemid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.itemid
    }

    fn mut_itemid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.itemid
    }

    // optional uint32 gold_remaining = 2;

    pub fn clear_gold_remaining(&mut self) {
        self.gold_remaining = ::std::option::Option::None;
    }

    pub fn has_gold_remaining(&self) -> bool {
        self.gold_remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gold_remaining(&mut self, v: u32) {
        self.gold_remaining = ::std::option::Option::Some(v);
    }

    pub fn get_gold_remaining(&self) -> u32 {
        self.gold_remaining.unwrap_or(0)
    }

    fn get_gold_remaining_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gold_remaining
    }

    fn mut_gold_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gold_remaining
    }
}

impl ::protobuf::Message for CDOTAClientMsg_WillPurchaseAlert {
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
                    self.itemid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.gold_remaining = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.itemid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gold_remaining {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.itemid {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.gold_remaining {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_WillPurchaseAlert {
    fn new() -> CDOTAClientMsg_WillPurchaseAlert {
        CDOTAClientMsg_WillPurchaseAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_WillPurchaseAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "itemid",
                    CDOTAClientMsg_WillPurchaseAlert::get_itemid_for_reflect,
                    CDOTAClientMsg_WillPurchaseAlert::mut_itemid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gold_remaining",
                    CDOTAClientMsg_WillPurchaseAlert::get_gold_remaining_for_reflect,
                    CDOTAClientMsg_WillPurchaseAlert::mut_gold_remaining_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_WillPurchaseAlert>(
                    "CDOTAClientMsg_WillPurchaseAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_WillPurchaseAlert {
    fn clear(&mut self) {
        self.clear_itemid();
        self.clear_gold_remaining();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_WillPurchaseAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_WillPurchaseAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_BuyBackStateAlert {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_BuyBackStateAlert {}

impl CDOTAClientMsg_BuyBackStateAlert {
    pub fn new() -> CDOTAClientMsg_BuyBackStateAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_BuyBackStateAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_BuyBackStateAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_BuyBackStateAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_BuyBackStateAlert::new)
        }
    }
}

impl ::protobuf::Message for CDOTAClientMsg_BuyBackStateAlert {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_BuyBackStateAlert {
    fn new() -> CDOTAClientMsg_BuyBackStateAlert {
        CDOTAClientMsg_BuyBackStateAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_BuyBackStateAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_BuyBackStateAlert>(
                    "CDOTAClientMsg_BuyBackStateAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_BuyBackStateAlert {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_BuyBackStateAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_BuyBackStateAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_QuickBuyAlert {
    // message fields
    itemid: ::std::option::Option<i32>,
    gold_required: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_QuickBuyAlert {}

impl CDOTAClientMsg_QuickBuyAlert {
    pub fn new() -> CDOTAClientMsg_QuickBuyAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_QuickBuyAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_QuickBuyAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_QuickBuyAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_QuickBuyAlert::new)
        }
    }

    // optional int32 itemid = 1;

    pub fn clear_itemid(&mut self) {
        self.itemid = ::std::option::Option::None;
    }

    pub fn has_itemid(&self) -> bool {
        self.itemid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itemid(&mut self, v: i32) {
        self.itemid = ::std::option::Option::Some(v);
    }

    pub fn get_itemid(&self) -> i32 {
        self.itemid.unwrap_or(0)
    }

    fn get_itemid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.itemid
    }

    fn mut_itemid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.itemid
    }

    // optional int32 gold_required = 2;

    pub fn clear_gold_required(&mut self) {
        self.gold_required = ::std::option::Option::None;
    }

    pub fn has_gold_required(&self) -> bool {
        self.gold_required.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gold_required(&mut self, v: i32) {
        self.gold_required = ::std::option::Option::Some(v);
    }

    pub fn get_gold_required(&self) -> i32 {
        self.gold_required.unwrap_or(0)
    }

    fn get_gold_required_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.gold_required
    }

    fn mut_gold_required_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.gold_required
    }
}

impl ::protobuf::Message for CDOTAClientMsg_QuickBuyAlert {
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
                    self.itemid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.gold_required = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.itemid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gold_required {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.itemid {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.gold_required {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_QuickBuyAlert {
    fn new() -> CDOTAClientMsg_QuickBuyAlert {
        CDOTAClientMsg_QuickBuyAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_QuickBuyAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "itemid",
                    CDOTAClientMsg_QuickBuyAlert::get_itemid_for_reflect,
                    CDOTAClientMsg_QuickBuyAlert::mut_itemid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "gold_required",
                    CDOTAClientMsg_QuickBuyAlert::get_gold_required_for_reflect,
                    CDOTAClientMsg_QuickBuyAlert::mut_gold_required_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_QuickBuyAlert>(
                    "CDOTAClientMsg_QuickBuyAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_QuickBuyAlert {
    fn clear(&mut self) {
        self.clear_itemid();
        self.clear_gold_required();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_QuickBuyAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_QuickBuyAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_PlayerShowCase {
    // message fields
    showcase: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_PlayerShowCase {}

impl CDOTAClientMsg_PlayerShowCase {
    pub fn new() -> CDOTAClientMsg_PlayerShowCase {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_PlayerShowCase {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_PlayerShowCase> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_PlayerShowCase,
        };
        unsafe {
            instance.get(CDOTAClientMsg_PlayerShowCase::new)
        }
    }

    // optional bool showcase = 1;

    pub fn clear_showcase(&mut self) {
        self.showcase = ::std::option::Option::None;
    }

    pub fn has_showcase(&self) -> bool {
        self.showcase.is_some()
    }

    // Param is passed by value, moved
    pub fn set_showcase(&mut self, v: bool) {
        self.showcase = ::std::option::Option::Some(v);
    }

    pub fn get_showcase(&self) -> bool {
        self.showcase.unwrap_or(false)
    }

    fn get_showcase_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.showcase
    }

    fn mut_showcase_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.showcase
    }
}

impl ::protobuf::Message for CDOTAClientMsg_PlayerShowCase {
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
                    self.showcase = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.showcase {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.showcase {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_PlayerShowCase {
    fn new() -> CDOTAClientMsg_PlayerShowCase {
        CDOTAClientMsg_PlayerShowCase::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_PlayerShowCase>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "showcase",
                    CDOTAClientMsg_PlayerShowCase::get_showcase_for_reflect,
                    CDOTAClientMsg_PlayerShowCase::mut_showcase_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_PlayerShowCase>(
                    "CDOTAClientMsg_PlayerShowCase",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_PlayerShowCase {
    fn clear(&mut self) {
        self.clear_showcase();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_PlayerShowCase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_PlayerShowCase {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_CameraZoomAmount {
    // message fields
    zoom_amount: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_CameraZoomAmount {}

impl CDOTAClientMsg_CameraZoomAmount {
    pub fn new() -> CDOTAClientMsg_CameraZoomAmount {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_CameraZoomAmount {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_CameraZoomAmount> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_CameraZoomAmount,
        };
        unsafe {
            instance.get(CDOTAClientMsg_CameraZoomAmount::new)
        }
    }

    // optional float zoom_amount = 1;

    pub fn clear_zoom_amount(&mut self) {
        self.zoom_amount = ::std::option::Option::None;
    }

    pub fn has_zoom_amount(&self) -> bool {
        self.zoom_amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zoom_amount(&mut self, v: f32) {
        self.zoom_amount = ::std::option::Option::Some(v);
    }

    pub fn get_zoom_amount(&self) -> f32 {
        self.zoom_amount.unwrap_or(0.)
    }

    fn get_zoom_amount_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.zoom_amount
    }

    fn mut_zoom_amount_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.zoom_amount
    }
}

impl ::protobuf::Message for CDOTAClientMsg_CameraZoomAmount {
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
                    self.zoom_amount = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.zoom_amount {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.zoom_amount {
            os.write_float(1, v)?;
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_CameraZoomAmount {
    fn new() -> CDOTAClientMsg_CameraZoomAmount {
        CDOTAClientMsg_CameraZoomAmount::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_CameraZoomAmount>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "zoom_amount",
                    CDOTAClientMsg_CameraZoomAmount::get_zoom_amount_for_reflect,
                    CDOTAClientMsg_CameraZoomAmount::mut_zoom_amount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_CameraZoomAmount>(
                    "CDOTAClientMsg_CameraZoomAmount",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_CameraZoomAmount {
    fn clear(&mut self) {
        self.clear_zoom_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_CameraZoomAmount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_CameraZoomAmount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_BroadcasterUsingCameraman {
    // message fields
    cameraman: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_BroadcasterUsingCameraman {}

impl CDOTAClientMsg_BroadcasterUsingCameraman {
    pub fn new() -> CDOTAClientMsg_BroadcasterUsingCameraman {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_BroadcasterUsingCameraman {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_BroadcasterUsingCameraman> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_BroadcasterUsingCameraman,
        };
        unsafe {
            instance.get(CDOTAClientMsg_BroadcasterUsingCameraman::new)
        }
    }

    // optional bool cameraman = 1;

    pub fn clear_cameraman(&mut self) {
        self.cameraman = ::std::option::Option::None;
    }

    pub fn has_cameraman(&self) -> bool {
        self.cameraman.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cameraman(&mut self, v: bool) {
        self.cameraman = ::std::option::Option::Some(v);
    }

    pub fn get_cameraman(&self) -> bool {
        self.cameraman.unwrap_or(false)
    }

    fn get_cameraman_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.cameraman
    }

    fn mut_cameraman_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.cameraman
    }
}

impl ::protobuf::Message for CDOTAClientMsg_BroadcasterUsingCameraman {
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
                    self.cameraman = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.cameraman {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cameraman {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_BroadcasterUsingCameraman {
    fn new() -> CDOTAClientMsg_BroadcasterUsingCameraman {
        CDOTAClientMsg_BroadcasterUsingCameraman::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_BroadcasterUsingCameraman>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "cameraman",
                    CDOTAClientMsg_BroadcasterUsingCameraman::get_cameraman_for_reflect,
                    CDOTAClientMsg_BroadcasterUsingCameraman::mut_cameraman_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_BroadcasterUsingCameraman>(
                    "CDOTAClientMsg_BroadcasterUsingCameraman",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_BroadcasterUsingCameraman {
    fn clear(&mut self) {
        self.clear_cameraman();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_BroadcasterUsingCameraman {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_BroadcasterUsingCameraman {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
    // message fields
    enabled: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {}

impl CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
    pub fn new() -> CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator,
        };
        unsafe {
            instance.get(CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator::new)
        }
    }

    // optional bool enabled = 1;

    pub fn clear_enabled(&mut self) {
        self.enabled = ::std::option::Option::None;
    }

    pub fn has_enabled(&self) -> bool {
        self.enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enabled(&mut self, v: bool) {
        self.enabled = ::std::option::Option::Some(v);
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.unwrap_or(false)
    }

    fn get_enabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }

    fn mut_enabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.enabled
    }
}

impl ::protobuf::Message for CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
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
                    self.enabled = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.enabled {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.enabled {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
    fn new() -> CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
        CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "enabled",
                    CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator::get_enabled_for_reflect,
                    CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator::mut_enabled_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator>(
                    "CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
    fn clear(&mut self) {
        self.clear_enabled();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_BroadcasterUsingAssistedCameraOperator {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CAdditionalEquipSlotClientMsg {
    // message fields
    class_id: ::std::option::Option<u32>,
    slot_id: ::std::option::Option<u32>,
    def_index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CAdditionalEquipSlotClientMsg {}

impl CAdditionalEquipSlotClientMsg {
    pub fn new() -> CAdditionalEquipSlotClientMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CAdditionalEquipSlotClientMsg {
        static mut instance: ::protobuf::lazy::Lazy<CAdditionalEquipSlotClientMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CAdditionalEquipSlotClientMsg,
        };
        unsafe {
            instance.get(CAdditionalEquipSlotClientMsg::new)
        }
    }

    // optional uint32 class_id = 1;

    pub fn clear_class_id(&mut self) {
        self.class_id = ::std::option::Option::None;
    }

    pub fn has_class_id(&self) -> bool {
        self.class_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_class_id(&mut self, v: u32) {
        self.class_id = ::std::option::Option::Some(v);
    }

    pub fn get_class_id(&self) -> u32 {
        self.class_id.unwrap_or(0)
    }

    fn get_class_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.class_id
    }

    fn mut_class_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.class_id
    }

    // optional uint32 slot_id = 2;

    pub fn clear_slot_id(&mut self) {
        self.slot_id = ::std::option::Option::None;
    }

    pub fn has_slot_id(&self) -> bool {
        self.slot_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot_id(&mut self, v: u32) {
        self.slot_id = ::std::option::Option::Some(v);
    }

    pub fn get_slot_id(&self) -> u32 {
        self.slot_id.unwrap_or(0)
    }

    fn get_slot_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.slot_id
    }

    fn mut_slot_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.slot_id
    }

    // optional uint32 def_index = 3;

    pub fn clear_def_index(&mut self) {
        self.def_index = ::std::option::Option::None;
    }

    pub fn has_def_index(&self) -> bool {
        self.def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_def_index(&mut self, v: u32) {
        self.def_index = ::std::option::Option::Some(v);
    }

    pub fn get_def_index(&self) -> u32 {
        self.def_index.unwrap_or(0)
    }

    fn get_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.def_index
    }

    fn mut_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.def_index
    }
}

impl ::protobuf::Message for CAdditionalEquipSlotClientMsg {
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
                    self.class_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.slot_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.def_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.slot_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.def_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.class_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.slot_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.def_index {
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

impl ::protobuf::MessageStatic for CAdditionalEquipSlotClientMsg {
    fn new() -> CAdditionalEquipSlotClientMsg {
        CAdditionalEquipSlotClientMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CAdditionalEquipSlotClientMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "class_id",
                    CAdditionalEquipSlotClientMsg::get_class_id_for_reflect,
                    CAdditionalEquipSlotClientMsg::mut_class_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "slot_id",
                    CAdditionalEquipSlotClientMsg::get_slot_id_for_reflect,
                    CAdditionalEquipSlotClientMsg::mut_slot_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "def_index",
                    CAdditionalEquipSlotClientMsg::get_def_index_for_reflect,
                    CAdditionalEquipSlotClientMsg::mut_def_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CAdditionalEquipSlotClientMsg>(
                    "CAdditionalEquipSlotClientMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CAdditionalEquipSlotClientMsg {
    fn clear(&mut self) {
        self.clear_class_id();
        self.clear_slot_id();
        self.clear_def_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CAdditionalEquipSlotClientMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CAdditionalEquipSlotClientMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_FreeInventory {
    // message fields
    equips: ::protobuf::RepeatedField<CAdditionalEquipSlotClientMsg>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_FreeInventory {}

impl CDOTAClientMsg_FreeInventory {
    pub fn new() -> CDOTAClientMsg_FreeInventory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_FreeInventory {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_FreeInventory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_FreeInventory,
        };
        unsafe {
            instance.get(CDOTAClientMsg_FreeInventory::new)
        }
    }

    // repeated .dota.CAdditionalEquipSlotClientMsg equips = 1;

    pub fn clear_equips(&mut self) {
        self.equips.clear();
    }

    // Param is passed by value, moved
    pub fn set_equips(&mut self, v: ::protobuf::RepeatedField<CAdditionalEquipSlotClientMsg>) {
        self.equips = v;
    }

    // Mutable pointer to the field.
    pub fn mut_equips(&mut self) -> &mut ::protobuf::RepeatedField<CAdditionalEquipSlotClientMsg> {
        &mut self.equips
    }

    // Take field
    pub fn take_equips(&mut self) -> ::protobuf::RepeatedField<CAdditionalEquipSlotClientMsg> {
        ::std::mem::replace(&mut self.equips, ::protobuf::RepeatedField::new())
    }

    pub fn get_equips(&self) -> &[CAdditionalEquipSlotClientMsg] {
        &self.equips
    }

    fn get_equips_for_reflect(&self) -> &::protobuf::RepeatedField<CAdditionalEquipSlotClientMsg> {
        &self.equips
    }

    fn mut_equips_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CAdditionalEquipSlotClientMsg> {
        &mut self.equips
    }
}

impl ::protobuf::Message for CDOTAClientMsg_FreeInventory {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.equips)?;
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
        for value in &self.equips {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.equips {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_FreeInventory {
    fn new() -> CDOTAClientMsg_FreeInventory {
        CDOTAClientMsg_FreeInventory::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_FreeInventory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CAdditionalEquipSlotClientMsg>>(
                    "equips",
                    CDOTAClientMsg_FreeInventory::get_equips_for_reflect,
                    CDOTAClientMsg_FreeInventory::mut_equips_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_FreeInventory>(
                    "CDOTAClientMsg_FreeInventory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_FreeInventory {
    fn clear(&mut self) {
        self.clear_equips();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_FreeInventory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_FreeInventory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_FillEmptySlotsWithBots {
    // message fields
    fillwithbots: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_FillEmptySlotsWithBots {}

impl CDOTAClientMsg_FillEmptySlotsWithBots {
    pub fn new() -> CDOTAClientMsg_FillEmptySlotsWithBots {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_FillEmptySlotsWithBots {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_FillEmptySlotsWithBots> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_FillEmptySlotsWithBots,
        };
        unsafe {
            instance.get(CDOTAClientMsg_FillEmptySlotsWithBots::new)
        }
    }

    // optional bool fillwithbots = 1;

    pub fn clear_fillwithbots(&mut self) {
        self.fillwithbots = ::std::option::Option::None;
    }

    pub fn has_fillwithbots(&self) -> bool {
        self.fillwithbots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fillwithbots(&mut self, v: bool) {
        self.fillwithbots = ::std::option::Option::Some(v);
    }

    pub fn get_fillwithbots(&self) -> bool {
        self.fillwithbots.unwrap_or(false)
    }

    fn get_fillwithbots_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.fillwithbots
    }

    fn mut_fillwithbots_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.fillwithbots
    }
}

impl ::protobuf::Message for CDOTAClientMsg_FillEmptySlotsWithBots {
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
                    self.fillwithbots = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.fillwithbots {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fillwithbots {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_FillEmptySlotsWithBots {
    fn new() -> CDOTAClientMsg_FillEmptySlotsWithBots {
        CDOTAClientMsg_FillEmptySlotsWithBots::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_FillEmptySlotsWithBots>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "fillwithbots",
                    CDOTAClientMsg_FillEmptySlotsWithBots::get_fillwithbots_for_reflect,
                    CDOTAClientMsg_FillEmptySlotsWithBots::mut_fillwithbots_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_FillEmptySlotsWithBots>(
                    "CDOTAClientMsg_FillEmptySlotsWithBots",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_FillEmptySlotsWithBots {
    fn clear(&mut self) {
        self.clear_fillwithbots();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_FillEmptySlotsWithBots {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_FillEmptySlotsWithBots {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_HeroStatueLike {
    // message fields
    owner_player_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_HeroStatueLike {}

impl CDOTAClientMsg_HeroStatueLike {
    pub fn new() -> CDOTAClientMsg_HeroStatueLike {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_HeroStatueLike {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_HeroStatueLike> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_HeroStatueLike,
        };
        unsafe {
            instance.get(CDOTAClientMsg_HeroStatueLike::new)
        }
    }

    // optional uint32 owner_player_id = 1;

    pub fn clear_owner_player_id(&mut self) {
        self.owner_player_id = ::std::option::Option::None;
    }

    pub fn has_owner_player_id(&self) -> bool {
        self.owner_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_player_id(&mut self, v: u32) {
        self.owner_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_player_id(&self) -> u32 {
        self.owner_player_id.unwrap_or(0)
    }

    fn get_owner_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.owner_player_id
    }

    fn mut_owner_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.owner_player_id
    }
}

impl ::protobuf::Message for CDOTAClientMsg_HeroStatueLike {
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
                    self.owner_player_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.owner_player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner_player_id {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_HeroStatueLike {
    fn new() -> CDOTAClientMsg_HeroStatueLike {
        CDOTAClientMsg_HeroStatueLike::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_HeroStatueLike>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "owner_player_id",
                    CDOTAClientMsg_HeroStatueLike::get_owner_player_id_for_reflect,
                    CDOTAClientMsg_HeroStatueLike::mut_owner_player_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_HeroStatueLike>(
                    "CDOTAClientMsg_HeroStatueLike",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_HeroStatueLike {
    fn clear(&mut self) {
        self.clear_owner_player_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_HeroStatueLike {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_HeroStatueLike {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_EventCNY2015Cmd {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_EventCNY2015Cmd {}

impl CDOTAClientMsg_EventCNY2015Cmd {
    pub fn new() -> CDOTAClientMsg_EventCNY2015Cmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_EventCNY2015Cmd {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_EventCNY2015Cmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_EventCNY2015Cmd,
        };
        unsafe {
            instance.get(CDOTAClientMsg_EventCNY2015Cmd::new)
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

impl ::protobuf::Message for CDOTAClientMsg_EventCNY2015Cmd {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_EventCNY2015Cmd {
    fn new() -> CDOTAClientMsg_EventCNY2015Cmd {
        CDOTAClientMsg_EventCNY2015Cmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_EventCNY2015Cmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CDOTAClientMsg_EventCNY2015Cmd::get_data_for_reflect,
                    CDOTAClientMsg_EventCNY2015Cmd::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_EventCNY2015Cmd>(
                    "CDOTAClientMsg_EventCNY2015Cmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_EventCNY2015Cmd {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_EventCNY2015Cmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_EventCNY2015Cmd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_DemoHero {
    // message fields
    hero_id: ::std::option::Option<i32>,
    hero_id_to_spawn: ::std::option::Option<i32>,
    item_defs: ::std::vec::Vec<u32>,
    item_ids: ::std::vec::Vec<u64>,
    style_index: ::std::option::Option<u32>,
    keep_existing_demohero: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_DemoHero {}

impl CDOTAClientMsg_DemoHero {
    pub fn new() -> CDOTAClientMsg_DemoHero {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_DemoHero {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_DemoHero> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_DemoHero,
        };
        unsafe {
            instance.get(CDOTAClientMsg_DemoHero::new)
        }
    }

    // optional int32 hero_id = 1;

    pub fn clear_hero_id(&mut self) {
        self.hero_id = ::std::option::Option::None;
    }

    pub fn has_hero_id(&self) -> bool {
        self.hero_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_id(&mut self, v: i32) {
        self.hero_id = ::std::option::Option::Some(v);
    }

    pub fn get_hero_id(&self) -> i32 {
        self.hero_id.unwrap_or(0)
    }

    fn get_hero_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.hero_id
    }

    fn mut_hero_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.hero_id
    }

    // optional int32 hero_id_to_spawn = 2;

    pub fn clear_hero_id_to_spawn(&mut self) {
        self.hero_id_to_spawn = ::std::option::Option::None;
    }

    pub fn has_hero_id_to_spawn(&self) -> bool {
        self.hero_id_to_spawn.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_id_to_spawn(&mut self, v: i32) {
        self.hero_id_to_spawn = ::std::option::Option::Some(v);
    }

    pub fn get_hero_id_to_spawn(&self) -> i32 {
        self.hero_id_to_spawn.unwrap_or(0)
    }

    fn get_hero_id_to_spawn_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.hero_id_to_spawn
    }

    fn mut_hero_id_to_spawn_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.hero_id_to_spawn
    }

    // repeated uint32 item_defs = 3;

    pub fn clear_item_defs(&mut self) {
        self.item_defs.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_defs(&mut self, v: ::std::vec::Vec<u32>) {
        self.item_defs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_defs(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.item_defs
    }

    // Take field
    pub fn take_item_defs(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.item_defs, ::std::vec::Vec::new())
    }

    pub fn get_item_defs(&self) -> &[u32] {
        &self.item_defs
    }

    fn get_item_defs_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.item_defs
    }

    fn mut_item_defs_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.item_defs
    }

    // repeated uint64 item_ids = 4;

    pub fn clear_item_ids(&mut self) {
        self.item_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.item_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.item_ids
    }

    // Take field
    pub fn take_item_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.item_ids, ::std::vec::Vec::new())
    }

    pub fn get_item_ids(&self) -> &[u64] {
        &self.item_ids
    }

    fn get_item_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.item_ids
    }

    fn mut_item_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.item_ids
    }

    // optional uint32 style_index = 5;

    pub fn clear_style_index(&mut self) {
        self.style_index = ::std::option::Option::None;
    }

    pub fn has_style_index(&self) -> bool {
        self.style_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_style_index(&mut self, v: u32) {
        self.style_index = ::std::option::Option::Some(v);
    }

    pub fn get_style_index(&self) -> u32 {
        self.style_index.unwrap_or(0)
    }

    fn get_style_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.style_index
    }

    fn mut_style_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.style_index
    }

    // optional bool keep_existing_demohero = 6;

    pub fn clear_keep_existing_demohero(&mut self) {
        self.keep_existing_demohero = ::std::option::Option::None;
    }

    pub fn has_keep_existing_demohero(&self) -> bool {
        self.keep_existing_demohero.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keep_existing_demohero(&mut self, v: bool) {
        self.keep_existing_demohero = ::std::option::Option::Some(v);
    }

    pub fn get_keep_existing_demohero(&self) -> bool {
        self.keep_existing_demohero.unwrap_or(false)
    }

    fn get_keep_existing_demohero_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.keep_existing_demohero
    }

    fn mut_keep_existing_demohero_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.keep_existing_demohero
    }
}

impl ::protobuf::Message for CDOTAClientMsg_DemoHero {
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
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.hero_id_to_spawn = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.item_defs)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.item_ids)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.style_index = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.keep_existing_demohero = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hero_id_to_spawn {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.item_defs {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.item_ids {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.style_index {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.keep_existing_demohero {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hero_id {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.hero_id_to_spawn {
            os.write_int32(2, v)?;
        };
        for v in &self.item_defs {
            os.write_uint32(3, *v)?;
        };
        for v in &self.item_ids {
            os.write_uint64(4, *v)?;
        };
        if let Some(v) = self.style_index {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.keep_existing_demohero {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_DemoHero {
    fn new() -> CDOTAClientMsg_DemoHero {
        CDOTAClientMsg_DemoHero::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_DemoHero>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "hero_id",
                    CDOTAClientMsg_DemoHero::get_hero_id_for_reflect,
                    CDOTAClientMsg_DemoHero::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "hero_id_to_spawn",
                    CDOTAClientMsg_DemoHero::get_hero_id_to_spawn_for_reflect,
                    CDOTAClientMsg_DemoHero::mut_hero_id_to_spawn_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_defs",
                    CDOTAClientMsg_DemoHero::get_item_defs_for_reflect,
                    CDOTAClientMsg_DemoHero::mut_item_defs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "item_ids",
                    CDOTAClientMsg_DemoHero::get_item_ids_for_reflect,
                    CDOTAClientMsg_DemoHero::mut_item_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "style_index",
                    CDOTAClientMsg_DemoHero::get_style_index_for_reflect,
                    CDOTAClientMsg_DemoHero::mut_style_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "keep_existing_demohero",
                    CDOTAClientMsg_DemoHero::get_keep_existing_demohero_for_reflect,
                    CDOTAClientMsg_DemoHero::mut_keep_existing_demohero_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_DemoHero>(
                    "CDOTAClientMsg_DemoHero",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_DemoHero {
    fn clear(&mut self) {
        self.clear_hero_id();
        self.clear_hero_id_to_spawn();
        self.clear_item_defs();
        self.clear_item_ids();
        self.clear_style_index();
        self.clear_keep_existing_demohero();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_DemoHero {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_DemoHero {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ChallengeSelect {
    // message fields
    event_id: ::std::option::Option<u32>,
    slot_id: ::std::option::Option<u32>,
    sequence_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ChallengeSelect {}

impl CDOTAClientMsg_ChallengeSelect {
    pub fn new() -> CDOTAClientMsg_ChallengeSelect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ChallengeSelect {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ChallengeSelect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ChallengeSelect,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ChallengeSelect::new)
        }
    }

    // optional uint32 event_id = 1;

    pub fn clear_event_id(&mut self) {
        self.event_id = ::std::option::Option::None;
    }

    pub fn has_event_id(&self) -> bool {
        self.event_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_id(&mut self, v: u32) {
        self.event_id = ::std::option::Option::Some(v);
    }

    pub fn get_event_id(&self) -> u32 {
        self.event_id.unwrap_or(0)
    }

    fn get_event_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.event_id
    }

    fn mut_event_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.event_id
    }

    // optional uint32 slot_id = 2;

    pub fn clear_slot_id(&mut self) {
        self.slot_id = ::std::option::Option::None;
    }

    pub fn has_slot_id(&self) -> bool {
        self.slot_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot_id(&mut self, v: u32) {
        self.slot_id = ::std::option::Option::Some(v);
    }

    pub fn get_slot_id(&self) -> u32 {
        self.slot_id.unwrap_or(0)
    }

    fn get_slot_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.slot_id
    }

    fn mut_slot_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.slot_id
    }

    // optional uint32 sequence_id = 3;

    pub fn clear_sequence_id(&mut self) {
        self.sequence_id = ::std::option::Option::None;
    }

    pub fn has_sequence_id(&self) -> bool {
        self.sequence_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_id(&mut self, v: u32) {
        self.sequence_id = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_id(&self) -> u32 {
        self.sequence_id.unwrap_or(0)
    }

    fn get_sequence_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sequence_id
    }

    fn mut_sequence_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sequence_id
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ChallengeSelect {
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
                    self.event_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.slot_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.sequence_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.event_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.slot_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sequence_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.slot_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.sequence_id {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ChallengeSelect {
    fn new() -> CDOTAClientMsg_ChallengeSelect {
        CDOTAClientMsg_ChallengeSelect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ChallengeSelect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "event_id",
                    CDOTAClientMsg_ChallengeSelect::get_event_id_for_reflect,
                    CDOTAClientMsg_ChallengeSelect::mut_event_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "slot_id",
                    CDOTAClientMsg_ChallengeSelect::get_slot_id_for_reflect,
                    CDOTAClientMsg_ChallengeSelect::mut_slot_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sequence_id",
                    CDOTAClientMsg_ChallengeSelect::get_sequence_id_for_reflect,
                    CDOTAClientMsg_ChallengeSelect::mut_sequence_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ChallengeSelect>(
                    "CDOTAClientMsg_ChallengeSelect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ChallengeSelect {
    fn clear(&mut self) {
        self.clear_event_id();
        self.clear_slot_id();
        self.clear_sequence_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ChallengeSelect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ChallengeSelect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ChallengeReroll {
    // message fields
    event_id: ::std::option::Option<u32>,
    slot_id: ::std::option::Option<u32>,
    sequence_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ChallengeReroll {}

impl CDOTAClientMsg_ChallengeReroll {
    pub fn new() -> CDOTAClientMsg_ChallengeReroll {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ChallengeReroll {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ChallengeReroll> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ChallengeReroll,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ChallengeReroll::new)
        }
    }

    // optional uint32 event_id = 1;

    pub fn clear_event_id(&mut self) {
        self.event_id = ::std::option::Option::None;
    }

    pub fn has_event_id(&self) -> bool {
        self.event_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_id(&mut self, v: u32) {
        self.event_id = ::std::option::Option::Some(v);
    }

    pub fn get_event_id(&self) -> u32 {
        self.event_id.unwrap_or(0)
    }

    fn get_event_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.event_id
    }

    fn mut_event_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.event_id
    }

    // optional uint32 slot_id = 2;

    pub fn clear_slot_id(&mut self) {
        self.slot_id = ::std::option::Option::None;
    }

    pub fn has_slot_id(&self) -> bool {
        self.slot_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot_id(&mut self, v: u32) {
        self.slot_id = ::std::option::Option::Some(v);
    }

    pub fn get_slot_id(&self) -> u32 {
        self.slot_id.unwrap_or(0)
    }

    fn get_slot_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.slot_id
    }

    fn mut_slot_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.slot_id
    }

    // optional uint32 sequence_id = 3;

    pub fn clear_sequence_id(&mut self) {
        self.sequence_id = ::std::option::Option::None;
    }

    pub fn has_sequence_id(&self) -> bool {
        self.sequence_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_id(&mut self, v: u32) {
        self.sequence_id = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_id(&self) -> u32 {
        self.sequence_id.unwrap_or(0)
    }

    fn get_sequence_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sequence_id
    }

    fn mut_sequence_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sequence_id
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ChallengeReroll {
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
                    self.event_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.slot_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.sequence_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.event_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.slot_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sequence_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.slot_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.sequence_id {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ChallengeReroll {
    fn new() -> CDOTAClientMsg_ChallengeReroll {
        CDOTAClientMsg_ChallengeReroll::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ChallengeReroll>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "event_id",
                    CDOTAClientMsg_ChallengeReroll::get_event_id_for_reflect,
                    CDOTAClientMsg_ChallengeReroll::mut_event_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "slot_id",
                    CDOTAClientMsg_ChallengeReroll::get_slot_id_for_reflect,
                    CDOTAClientMsg_ChallengeReroll::mut_slot_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sequence_id",
                    CDOTAClientMsg_ChallengeReroll::get_sequence_id_for_reflect,
                    CDOTAClientMsg_ChallengeReroll::mut_sequence_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ChallengeReroll>(
                    "CDOTAClientMsg_ChallengeReroll",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ChallengeReroll {
    fn clear(&mut self) {
        self.clear_event_id();
        self.clear_slot_id();
        self.clear_sequence_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ChallengeReroll {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ChallengeReroll {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_CoinWager {
    // message fields
    wager_amount: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_CoinWager {}

impl CDOTAClientMsg_CoinWager {
    pub fn new() -> CDOTAClientMsg_CoinWager {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_CoinWager {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_CoinWager> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_CoinWager,
        };
        unsafe {
            instance.get(CDOTAClientMsg_CoinWager::new)
        }
    }

    // optional uint32 wager_amount = 1;

    pub fn clear_wager_amount(&mut self) {
        self.wager_amount = ::std::option::Option::None;
    }

    pub fn has_wager_amount(&self) -> bool {
        self.wager_amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wager_amount(&mut self, v: u32) {
        self.wager_amount = ::std::option::Option::Some(v);
    }

    pub fn get_wager_amount(&self) -> u32 {
        self.wager_amount.unwrap_or(0)
    }

    fn get_wager_amount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.wager_amount
    }

    fn mut_wager_amount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.wager_amount
    }
}

impl ::protobuf::Message for CDOTAClientMsg_CoinWager {
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
                    self.wager_amount = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.wager_amount {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.wager_amount {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_CoinWager {
    fn new() -> CDOTAClientMsg_CoinWager {
        CDOTAClientMsg_CoinWager::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_CoinWager>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "wager_amount",
                    CDOTAClientMsg_CoinWager::get_wager_amount_for_reflect,
                    CDOTAClientMsg_CoinWager::mut_wager_amount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_CoinWager>(
                    "CDOTAClientMsg_CoinWager",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_CoinWager {
    fn clear(&mut self) {
        self.clear_wager_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_CoinWager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_CoinWager {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_EventPointsTip {
    // message fields
    recipient_player_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_EventPointsTip {}

impl CDOTAClientMsg_EventPointsTip {
    pub fn new() -> CDOTAClientMsg_EventPointsTip {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_EventPointsTip {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_EventPointsTip> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_EventPointsTip,
        };
        unsafe {
            instance.get(CDOTAClientMsg_EventPointsTip::new)
        }
    }

    // optional uint32 recipient_player_id = 1;

    pub fn clear_recipient_player_id(&mut self) {
        self.recipient_player_id = ::std::option::Option::None;
    }

    pub fn has_recipient_player_id(&self) -> bool {
        self.recipient_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_recipient_player_id(&mut self, v: u32) {
        self.recipient_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_recipient_player_id(&self) -> u32 {
        self.recipient_player_id.unwrap_or(0)
    }

    fn get_recipient_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.recipient_player_id
    }

    fn mut_recipient_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.recipient_player_id
    }
}

impl ::protobuf::Message for CDOTAClientMsg_EventPointsTip {
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
                    self.recipient_player_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.recipient_player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.recipient_player_id {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_EventPointsTip {
    fn new() -> CDOTAClientMsg_EventPointsTip {
        CDOTAClientMsg_EventPointsTip::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_EventPointsTip>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "recipient_player_id",
                    CDOTAClientMsg_EventPointsTip::get_recipient_player_id_for_reflect,
                    CDOTAClientMsg_EventPointsTip::mut_recipient_player_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_EventPointsTip>(
                    "CDOTAClientMsg_EventPointsTip",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_EventPointsTip {
    fn clear(&mut self) {
        self.clear_recipient_player_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_EventPointsTip {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_EventPointsTip {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ExecuteOrders {
    // message fields
    orders: ::protobuf::RepeatedField<super::dota_commonmessages::CDOTAMsg_UnitOrder>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ExecuteOrders {}

impl CDOTAClientMsg_ExecuteOrders {
    pub fn new() -> CDOTAClientMsg_ExecuteOrders {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ExecuteOrders {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ExecuteOrders> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ExecuteOrders,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ExecuteOrders::new)
        }
    }

    // repeated .dota.CDOTAMsg_UnitOrder orders = 1;

    pub fn clear_orders(&mut self) {
        self.orders.clear();
    }

    // Param is passed by value, moved
    pub fn set_orders(&mut self, v: ::protobuf::RepeatedField<super::dota_commonmessages::CDOTAMsg_UnitOrder>) {
        self.orders = v;
    }

    // Mutable pointer to the field.
    pub fn mut_orders(&mut self) -> &mut ::protobuf::RepeatedField<super::dota_commonmessages::CDOTAMsg_UnitOrder> {
        &mut self.orders
    }

    // Take field
    pub fn take_orders(&mut self) -> ::protobuf::RepeatedField<super::dota_commonmessages::CDOTAMsg_UnitOrder> {
        ::std::mem::replace(&mut self.orders, ::protobuf::RepeatedField::new())
    }

    pub fn get_orders(&self) -> &[super::dota_commonmessages::CDOTAMsg_UnitOrder] {
        &self.orders
    }

    fn get_orders_for_reflect(&self) -> &::protobuf::RepeatedField<super::dota_commonmessages::CDOTAMsg_UnitOrder> {
        &self.orders
    }

    fn mut_orders_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::dota_commonmessages::CDOTAMsg_UnitOrder> {
        &mut self.orders
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ExecuteOrders {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.orders)?;
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
        for value in &self.orders {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.orders {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ExecuteOrders {
    fn new() -> CDOTAClientMsg_ExecuteOrders {
        CDOTAClientMsg_ExecuteOrders::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ExecuteOrders>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_commonmessages::CDOTAMsg_UnitOrder>>(
                    "orders",
                    CDOTAClientMsg_ExecuteOrders::get_orders_for_reflect,
                    CDOTAClientMsg_ExecuteOrders::mut_orders_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ExecuteOrders>(
                    "CDOTAClientMsg_ExecuteOrders",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ExecuteOrders {
    fn clear(&mut self) {
        self.clear_orders();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ExecuteOrders {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ExecuteOrders {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_XPAlert {
    // message fields
    target_entindex: ::std::option::Option<u32>,
    damage_taken: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_XPAlert {}

impl CDOTAClientMsg_XPAlert {
    pub fn new() -> CDOTAClientMsg_XPAlert {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_XPAlert {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_XPAlert> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_XPAlert,
        };
        unsafe {
            instance.get(CDOTAClientMsg_XPAlert::new)
        }
    }

    // optional uint32 target_entindex = 1;

    pub fn clear_target_entindex(&mut self) {
        self.target_entindex = ::std::option::Option::None;
    }

    pub fn has_target_entindex(&self) -> bool {
        self.target_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_entindex(&mut self, v: u32) {
        self.target_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_target_entindex(&self) -> u32 {
        self.target_entindex.unwrap_or(0)
    }

    fn get_target_entindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_entindex
    }

    fn mut_target_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_entindex
    }

    // optional uint32 damage_taken = 2;

    pub fn clear_damage_taken(&mut self) {
        self.damage_taken = ::std::option::Option::None;
    }

    pub fn has_damage_taken(&self) -> bool {
        self.damage_taken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_taken(&mut self, v: u32) {
        self.damage_taken = ::std::option::Option::Some(v);
    }

    pub fn get_damage_taken(&self) -> u32 {
        self.damage_taken.unwrap_or(0)
    }

    fn get_damage_taken_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.damage_taken
    }

    fn mut_damage_taken_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.damage_taken
    }
}

impl ::protobuf::Message for CDOTAClientMsg_XPAlert {
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
                    self.target_entindex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.damage_taken = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.target_entindex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.damage_taken {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.target_entindex {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.damage_taken {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_XPAlert {
    fn new() -> CDOTAClientMsg_XPAlert {
        CDOTAClientMsg_XPAlert::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_XPAlert>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_entindex",
                    CDOTAClientMsg_XPAlert::get_target_entindex_for_reflect,
                    CDOTAClientMsg_XPAlert::mut_target_entindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "damage_taken",
                    CDOTAClientMsg_XPAlert::get_damage_taken_for_reflect,
                    CDOTAClientMsg_XPAlert::mut_damage_taken_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_XPAlert>(
                    "CDOTAClientMsg_XPAlert",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_XPAlert {
    fn clear(&mut self) {
        self.clear_target_entindex();
        self.clear_damage_taken();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_XPAlert {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_XPAlert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_KillcamDamageTaken {
    // message fields
    target_entindex: ::std::option::Option<u32>,
    damage_taken: ::std::option::Option<u32>,
    item_type: ::std::option::Option<u32>,
    item_id: ::std::option::Option<u32>,
    hero_name: ::protobuf::SingularField<::std::string::String>,
    damage_color: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_KillcamDamageTaken {}

impl CDOTAClientMsg_KillcamDamageTaken {
    pub fn new() -> CDOTAClientMsg_KillcamDamageTaken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_KillcamDamageTaken {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_KillcamDamageTaken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_KillcamDamageTaken,
        };
        unsafe {
            instance.get(CDOTAClientMsg_KillcamDamageTaken::new)
        }
    }

    // optional uint32 target_entindex = 1;

    pub fn clear_target_entindex(&mut self) {
        self.target_entindex = ::std::option::Option::None;
    }

    pub fn has_target_entindex(&self) -> bool {
        self.target_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_entindex(&mut self, v: u32) {
        self.target_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_target_entindex(&self) -> u32 {
        self.target_entindex.unwrap_or(0)
    }

    fn get_target_entindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_entindex
    }

    fn mut_target_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_entindex
    }

    // optional uint32 damage_taken = 2;

    pub fn clear_damage_taken(&mut self) {
        self.damage_taken = ::std::option::Option::None;
    }

    pub fn has_damage_taken(&self) -> bool {
        self.damage_taken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_taken(&mut self, v: u32) {
        self.damage_taken = ::std::option::Option::Some(v);
    }

    pub fn get_damage_taken(&self) -> u32 {
        self.damage_taken.unwrap_or(0)
    }

    fn get_damage_taken_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.damage_taken
    }

    fn mut_damage_taken_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.damage_taken
    }

    // optional uint32 item_type = 3;

    pub fn clear_item_type(&mut self) {
        self.item_type = ::std::option::Option::None;
    }

    pub fn has_item_type(&self) -> bool {
        self.item_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_type(&mut self, v: u32) {
        self.item_type = ::std::option::Option::Some(v);
    }

    pub fn get_item_type(&self) -> u32 {
        self.item_type.unwrap_or(0)
    }

    fn get_item_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_type
    }

    fn mut_item_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_type
    }

    // optional uint32 item_id = 4;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u32) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u32 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_id
    }

    // optional string hero_name = 5;

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

    // optional string damage_color = 6;

    pub fn clear_damage_color(&mut self) {
        self.damage_color.clear();
    }

    pub fn has_damage_color(&self) -> bool {
        self.damage_color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_color(&mut self, v: ::std::string::String) {
        self.damage_color = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_damage_color(&mut self) -> &mut ::std::string::String {
        if self.damage_color.is_none() {
            self.damage_color.set_default();
        };
        self.damage_color.as_mut().unwrap()
    }

    // Take field
    pub fn take_damage_color(&mut self) -> ::std::string::String {
        self.damage_color.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_damage_color(&self) -> &str {
        match self.damage_color.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_damage_color_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.damage_color
    }

    fn mut_damage_color_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.damage_color
    }
}

impl ::protobuf::Message for CDOTAClientMsg_KillcamDamageTaken {
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
                    self.target_entindex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.damage_taken = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.item_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hero_name)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.damage_color)?;
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
        if let Some(v) = self.target_entindex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.damage_taken {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.item_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hero_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.damage_color.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.target_entindex {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.damage_taken {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.item_type {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.item_id {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.hero_name.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.damage_color.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_KillcamDamageTaken {
    fn new() -> CDOTAClientMsg_KillcamDamageTaken {
        CDOTAClientMsg_KillcamDamageTaken::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_KillcamDamageTaken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_entindex",
                    CDOTAClientMsg_KillcamDamageTaken::get_target_entindex_for_reflect,
                    CDOTAClientMsg_KillcamDamageTaken::mut_target_entindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "damage_taken",
                    CDOTAClientMsg_KillcamDamageTaken::get_damage_taken_for_reflect,
                    CDOTAClientMsg_KillcamDamageTaken::mut_damage_taken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_type",
                    CDOTAClientMsg_KillcamDamageTaken::get_item_type_for_reflect,
                    CDOTAClientMsg_KillcamDamageTaken::mut_item_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_id",
                    CDOTAClientMsg_KillcamDamageTaken::get_item_id_for_reflect,
                    CDOTAClientMsg_KillcamDamageTaken::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hero_name",
                    CDOTAClientMsg_KillcamDamageTaken::get_hero_name_for_reflect,
                    CDOTAClientMsg_KillcamDamageTaken::mut_hero_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "damage_color",
                    CDOTAClientMsg_KillcamDamageTaken::get_damage_color_for_reflect,
                    CDOTAClientMsg_KillcamDamageTaken::mut_damage_color_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_KillcamDamageTaken>(
                    "CDOTAClientMsg_KillcamDamageTaken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_KillcamDamageTaken {
    fn clear(&mut self) {
        self.clear_target_entindex();
        self.clear_damage_taken();
        self.clear_item_type();
        self.clear_item_id();
        self.clear_hero_name();
        self.clear_damage_color();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_KillcamDamageTaken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_KillcamDamageTaken {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_MatchMetadata {
    // message fields
    match_id: ::std::option::Option<u64>,
    metadata: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_MatchMetadata {}

impl CDOTAClientMsg_MatchMetadata {
    pub fn new() -> CDOTAClientMsg_MatchMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_MatchMetadata {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_MatchMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_MatchMetadata,
        };
        unsafe {
            instance.get(CDOTAClientMsg_MatchMetadata::new)
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

    // optional bytes metadata = 2;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: ::std::vec::Vec<u8>) {
        self.metadata = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.metadata.is_none() {
            self.metadata.set_default();
        };
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> ::std::vec::Vec<u8> {
        self.metadata.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_metadata(&self) -> &[u8] {
        match self.metadata.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.metadata
    }
}

impl ::protobuf::Message for CDOTAClientMsg_MatchMetadata {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.metadata)?;
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
        if let Some(v) = self.metadata.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.match_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.metadata.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_MatchMetadata {
    fn new() -> CDOTAClientMsg_MatchMetadata {
        CDOTAClientMsg_MatchMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_MatchMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CDOTAClientMsg_MatchMetadata::get_match_id_for_reflect,
                    CDOTAClientMsg_MatchMetadata::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "metadata",
                    CDOTAClientMsg_MatchMetadata::get_metadata_for_reflect,
                    CDOTAClientMsg_MatchMetadata::mut_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_MatchMetadata>(
                    "CDOTAClientMsg_MatchMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_MatchMetadata {
    fn clear(&mut self) {
        self.clear_match_id();
        self.clear_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_MatchMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_MatchMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_KillMyHero {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_KillMyHero {}

impl CDOTAClientMsg_KillMyHero {
    pub fn new() -> CDOTAClientMsg_KillMyHero {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_KillMyHero {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_KillMyHero> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_KillMyHero,
        };
        unsafe {
            instance.get(CDOTAClientMsg_KillMyHero::new)
        }
    }
}

impl ::protobuf::Message for CDOTAClientMsg_KillMyHero {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_KillMyHero {
    fn new() -> CDOTAClientMsg_KillMyHero {
        CDOTAClientMsg_KillMyHero::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_KillMyHero>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_KillMyHero>(
                    "CDOTAClientMsg_KillMyHero",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_KillMyHero {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_KillMyHero {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_KillMyHero {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_QuestStatus {
    // message fields
    quest_id: ::std::option::Option<u32>,
    challenge_id: ::std::option::Option<u32>,
    progress: ::std::option::Option<u32>,
    goal: ::std::option::Option<u32>,
    query: ::std::option::Option<u32>,
    fail_gametime: ::std::option::Option<f32>,
    item_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_QuestStatus {}

impl CDOTAClientMsg_QuestStatus {
    pub fn new() -> CDOTAClientMsg_QuestStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_QuestStatus {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_QuestStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_QuestStatus,
        };
        unsafe {
            instance.get(CDOTAClientMsg_QuestStatus::new)
        }
    }

    // optional uint32 quest_id = 1;

    pub fn clear_quest_id(&mut self) {
        self.quest_id = ::std::option::Option::None;
    }

    pub fn has_quest_id(&self) -> bool {
        self.quest_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quest_id(&mut self, v: u32) {
        self.quest_id = ::std::option::Option::Some(v);
    }

    pub fn get_quest_id(&self) -> u32 {
        self.quest_id.unwrap_or(0)
    }

    fn get_quest_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quest_id
    }

    fn mut_quest_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quest_id
    }

    // optional uint32 challenge_id = 2;

    pub fn clear_challenge_id(&mut self) {
        self.challenge_id = ::std::option::Option::None;
    }

    pub fn has_challenge_id(&self) -> bool {
        self.challenge_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge_id(&mut self, v: u32) {
        self.challenge_id = ::std::option::Option::Some(v);
    }

    pub fn get_challenge_id(&self) -> u32 {
        self.challenge_id.unwrap_or(0)
    }

    fn get_challenge_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.challenge_id
    }

    fn mut_challenge_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.challenge_id
    }

    // optional uint32 progress = 3;

    pub fn clear_progress(&mut self) {
        self.progress = ::std::option::Option::None;
    }

    pub fn has_progress(&self) -> bool {
        self.progress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_progress(&mut self, v: u32) {
        self.progress = ::std::option::Option::Some(v);
    }

    pub fn get_progress(&self) -> u32 {
        self.progress.unwrap_or(0)
    }

    fn get_progress_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.progress
    }

    fn mut_progress_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.progress
    }

    // optional uint32 goal = 4;

    pub fn clear_goal(&mut self) {
        self.goal = ::std::option::Option::None;
    }

    pub fn has_goal(&self) -> bool {
        self.goal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_goal(&mut self, v: u32) {
        self.goal = ::std::option::Option::Some(v);
    }

    pub fn get_goal(&self) -> u32 {
        self.goal.unwrap_or(0)
    }

    fn get_goal_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.goal
    }

    fn mut_goal_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.goal
    }

    // optional uint32 query = 5;

    pub fn clear_query(&mut self) {
        self.query = ::std::option::Option::None;
    }

    pub fn has_query(&self) -> bool {
        self.query.is_some()
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: u32) {
        self.query = ::std::option::Option::Some(v);
    }

    pub fn get_query(&self) -> u32 {
        self.query.unwrap_or(0)
    }

    fn get_query_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.query
    }

    fn mut_query_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.query
    }

    // optional float fail_gametime = 6;

    pub fn clear_fail_gametime(&mut self) {
        self.fail_gametime = ::std::option::Option::None;
    }

    pub fn has_fail_gametime(&self) -> bool {
        self.fail_gametime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fail_gametime(&mut self, v: f32) {
        self.fail_gametime = ::std::option::Option::Some(v);
    }

    pub fn get_fail_gametime(&self) -> f32 {
        self.fail_gametime.unwrap_or(0.)
    }

    fn get_fail_gametime_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fail_gametime
    }

    fn mut_fail_gametime_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fail_gametime
    }

    // optional uint32 item_id = 7;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u32) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u32 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_id
    }
}

impl ::protobuf::Message for CDOTAClientMsg_QuestStatus {
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
                    self.quest_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.challenge_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.progress = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.goal = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.query = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.fail_gametime = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.item_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.quest_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.challenge_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.progress {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.goal {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.query {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.fail_gametime {
            my_size += 5;
        };
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quest_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.challenge_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.progress {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.goal {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.query {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.fail_gametime {
            os.write_float(6, v)?;
        };
        if let Some(v) = self.item_id {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_QuestStatus {
    fn new() -> CDOTAClientMsg_QuestStatus {
        CDOTAClientMsg_QuestStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_QuestStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quest_id",
                    CDOTAClientMsg_QuestStatus::get_quest_id_for_reflect,
                    CDOTAClientMsg_QuestStatus::mut_quest_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "challenge_id",
                    CDOTAClientMsg_QuestStatus::get_challenge_id_for_reflect,
                    CDOTAClientMsg_QuestStatus::mut_challenge_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "progress",
                    CDOTAClientMsg_QuestStatus::get_progress_for_reflect,
                    CDOTAClientMsg_QuestStatus::mut_progress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "goal",
                    CDOTAClientMsg_QuestStatus::get_goal_for_reflect,
                    CDOTAClientMsg_QuestStatus::mut_goal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "query",
                    CDOTAClientMsg_QuestStatus::get_query_for_reflect,
                    CDOTAClientMsg_QuestStatus::mut_query_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fail_gametime",
                    CDOTAClientMsg_QuestStatus::get_fail_gametime_for_reflect,
                    CDOTAClientMsg_QuestStatus::mut_fail_gametime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_id",
                    CDOTAClientMsg_QuestStatus::get_item_id_for_reflect,
                    CDOTAClientMsg_QuestStatus::mut_item_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_QuestStatus>(
                    "CDOTAClientMsg_QuestStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_QuestStatus {
    fn clear(&mut self) {
        self.clear_quest_id();
        self.clear_challenge_id();
        self.clear_progress();
        self.clear_goal();
        self.clear_query();
        self.clear_fail_gametime();
        self.clear_item_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_QuestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_QuestStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_ToggleAutoattack {
    // message fields
    mode: ::std::option::Option<i32>,
    show_message: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_ToggleAutoattack {}

impl CDOTAClientMsg_ToggleAutoattack {
    pub fn new() -> CDOTAClientMsg_ToggleAutoattack {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_ToggleAutoattack {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_ToggleAutoattack> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_ToggleAutoattack,
        };
        unsafe {
            instance.get(CDOTAClientMsg_ToggleAutoattack::new)
        }
    }

    // optional int32 mode = 1;

    pub fn clear_mode(&mut self) {
        self.mode = ::std::option::Option::None;
    }

    pub fn has_mode(&self) -> bool {
        self.mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode(&mut self, v: i32) {
        self.mode = ::std::option::Option::Some(v);
    }

    pub fn get_mode(&self) -> i32 {
        self.mode.unwrap_or(0)
    }

    fn get_mode_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.mode
    }

    fn mut_mode_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.mode
    }

    // optional bool show_message = 2;

    pub fn clear_show_message(&mut self) {
        self.show_message = ::std::option::Option::None;
    }

    pub fn has_show_message(&self) -> bool {
        self.show_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show_message(&mut self, v: bool) {
        self.show_message = ::std::option::Option::Some(v);
    }

    pub fn get_show_message(&self) -> bool {
        self.show_message.unwrap_or(false)
    }

    fn get_show_message_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.show_message
    }

    fn mut_show_message_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.show_message
    }
}

impl ::protobuf::Message for CDOTAClientMsg_ToggleAutoattack {
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
                    self.mode = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.show_message = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.mode {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.show_message {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.show_message {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_ToggleAutoattack {
    fn new() -> CDOTAClientMsg_ToggleAutoattack {
        CDOTAClientMsg_ToggleAutoattack::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_ToggleAutoattack>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "mode",
                    CDOTAClientMsg_ToggleAutoattack::get_mode_for_reflect,
                    CDOTAClientMsg_ToggleAutoattack::mut_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "show_message",
                    CDOTAClientMsg_ToggleAutoattack::get_show_message_for_reflect,
                    CDOTAClientMsg_ToggleAutoattack::mut_show_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_ToggleAutoattack>(
                    "CDOTAClientMsg_ToggleAutoattack",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_ToggleAutoattack {
    fn clear(&mut self) {
        self.clear_mode();
        self.clear_show_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_ToggleAutoattack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_ToggleAutoattack {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_SpecialAbility {
    // message fields
    ability_index: ::std::option::Option<u32>,
    target_entindex: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_SpecialAbility {}

impl CDOTAClientMsg_SpecialAbility {
    pub fn new() -> CDOTAClientMsg_SpecialAbility {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_SpecialAbility {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_SpecialAbility> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_SpecialAbility,
        };
        unsafe {
            instance.get(CDOTAClientMsg_SpecialAbility::new)
        }
    }

    // optional uint32 ability_index = 1;

    pub fn clear_ability_index(&mut self) {
        self.ability_index = ::std::option::Option::None;
    }

    pub fn has_ability_index(&self) -> bool {
        self.ability_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_index(&mut self, v: u32) {
        self.ability_index = ::std::option::Option::Some(v);
    }

    pub fn get_ability_index(&self) -> u32 {
        self.ability_index.unwrap_or(0)
    }

    fn get_ability_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_index
    }

    fn mut_ability_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_index
    }

    // optional uint32 target_entindex = 2;

    pub fn clear_target_entindex(&mut self) {
        self.target_entindex = ::std::option::Option::None;
    }

    pub fn has_target_entindex(&self) -> bool {
        self.target_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_entindex(&mut self, v: u32) {
        self.target_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_target_entindex(&self) -> u32 {
        self.target_entindex.unwrap_or(0)
    }

    fn get_target_entindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.target_entindex
    }

    fn mut_target_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.target_entindex
    }
}

impl ::protobuf::Message for CDOTAClientMsg_SpecialAbility {
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
                    self.ability_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.target_entindex = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ability_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.target_entindex {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_index {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.target_entindex {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_SpecialAbility {
    fn new() -> CDOTAClientMsg_SpecialAbility {
        CDOTAClientMsg_SpecialAbility::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_SpecialAbility>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_index",
                    CDOTAClientMsg_SpecialAbility::get_ability_index_for_reflect,
                    CDOTAClientMsg_SpecialAbility::mut_ability_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "target_entindex",
                    CDOTAClientMsg_SpecialAbility::get_target_entindex_for_reflect,
                    CDOTAClientMsg_SpecialAbility::mut_target_entindex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_SpecialAbility>(
                    "CDOTAClientMsg_SpecialAbility",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_SpecialAbility {
    fn clear(&mut self) {
        self.clear_ability_index();
        self.clear_target_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_SpecialAbility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_SpecialAbility {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_SetEnemyStartingPosition {
    // message fields
    enemy_player_id: ::std::option::Option<u32>,
    enemy_starting_position: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_SetEnemyStartingPosition {}

impl CDOTAClientMsg_SetEnemyStartingPosition {
    pub fn new() -> CDOTAClientMsg_SetEnemyStartingPosition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_SetEnemyStartingPosition {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_SetEnemyStartingPosition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_SetEnemyStartingPosition,
        };
        unsafe {
            instance.get(CDOTAClientMsg_SetEnemyStartingPosition::new)
        }
    }

    // optional uint32 enemy_player_id = 1;

    pub fn clear_enemy_player_id(&mut self) {
        self.enemy_player_id = ::std::option::Option::None;
    }

    pub fn has_enemy_player_id(&self) -> bool {
        self.enemy_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enemy_player_id(&mut self, v: u32) {
        self.enemy_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_enemy_player_id(&self) -> u32 {
        self.enemy_player_id.unwrap_or(0)
    }

    fn get_enemy_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.enemy_player_id
    }

    fn mut_enemy_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.enemy_player_id
    }

    // optional uint32 enemy_starting_position = 2;

    pub fn clear_enemy_starting_position(&mut self) {
        self.enemy_starting_position = ::std::option::Option::None;
    }

    pub fn has_enemy_starting_position(&self) -> bool {
        self.enemy_starting_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enemy_starting_position(&mut self, v: u32) {
        self.enemy_starting_position = ::std::option::Option::Some(v);
    }

    pub fn get_enemy_starting_position(&self) -> u32 {
        self.enemy_starting_position.unwrap_or(0)
    }

    fn get_enemy_starting_position_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.enemy_starting_position
    }

    fn mut_enemy_starting_position_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.enemy_starting_position
    }
}

impl ::protobuf::Message for CDOTAClientMsg_SetEnemyStartingPosition {
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
                    self.enemy_player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.enemy_starting_position = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.enemy_player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.enemy_starting_position {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.enemy_player_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.enemy_starting_position {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_SetEnemyStartingPosition {
    fn new() -> CDOTAClientMsg_SetEnemyStartingPosition {
        CDOTAClientMsg_SetEnemyStartingPosition::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_SetEnemyStartingPosition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "enemy_player_id",
                    CDOTAClientMsg_SetEnemyStartingPosition::get_enemy_player_id_for_reflect,
                    CDOTAClientMsg_SetEnemyStartingPosition::mut_enemy_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "enemy_starting_position",
                    CDOTAClientMsg_SetEnemyStartingPosition::get_enemy_starting_position_for_reflect,
                    CDOTAClientMsg_SetEnemyStartingPosition::mut_enemy_starting_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_SetEnemyStartingPosition>(
                    "CDOTAClientMsg_SetEnemyStartingPosition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_SetEnemyStartingPosition {
    fn clear(&mut self) {
        self.clear_enemy_player_id();
        self.clear_enemy_starting_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_SetEnemyStartingPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_SetEnemyStartingPosition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_SetDesiredWardPlacement {
    // message fields
    ward_index: ::std::option::Option<u32>,
    ward_x: ::std::option::Option<f32>,
    ward_y: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_SetDesiredWardPlacement {}

impl CDOTAClientMsg_SetDesiredWardPlacement {
    pub fn new() -> CDOTAClientMsg_SetDesiredWardPlacement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_SetDesiredWardPlacement {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_SetDesiredWardPlacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_SetDesiredWardPlacement,
        };
        unsafe {
            instance.get(CDOTAClientMsg_SetDesiredWardPlacement::new)
        }
    }

    // optional uint32 ward_index = 1;

    pub fn clear_ward_index(&mut self) {
        self.ward_index = ::std::option::Option::None;
    }

    pub fn has_ward_index(&self) -> bool {
        self.ward_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ward_index(&mut self, v: u32) {
        self.ward_index = ::std::option::Option::Some(v);
    }

    pub fn get_ward_index(&self) -> u32 {
        self.ward_index.unwrap_or(0)
    }

    fn get_ward_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ward_index
    }

    fn mut_ward_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ward_index
    }

    // optional float ward_x = 2;

    pub fn clear_ward_x(&mut self) {
        self.ward_x = ::std::option::Option::None;
    }

    pub fn has_ward_x(&self) -> bool {
        self.ward_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ward_x(&mut self, v: f32) {
        self.ward_x = ::std::option::Option::Some(v);
    }

    pub fn get_ward_x(&self) -> f32 {
        self.ward_x.unwrap_or(0.)
    }

    fn get_ward_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.ward_x
    }

    fn mut_ward_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.ward_x
    }

    // optional float ward_y = 3;

    pub fn clear_ward_y(&mut self) {
        self.ward_y = ::std::option::Option::None;
    }

    pub fn has_ward_y(&self) -> bool {
        self.ward_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ward_y(&mut self, v: f32) {
        self.ward_y = ::std::option::Option::Some(v);
    }

    pub fn get_ward_y(&self) -> f32 {
        self.ward_y.unwrap_or(0.)
    }

    fn get_ward_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.ward_y
    }

    fn mut_ward_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.ward_y
    }
}

impl ::protobuf::Message for CDOTAClientMsg_SetDesiredWardPlacement {
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
                    self.ward_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.ward_x = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.ward_y = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ward_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ward_x {
            my_size += 5;
        };
        if let Some(v) = self.ward_y {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ward_index {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.ward_x {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.ward_y {
            os.write_float(3, v)?;
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_SetDesiredWardPlacement {
    fn new() -> CDOTAClientMsg_SetDesiredWardPlacement {
        CDOTAClientMsg_SetDesiredWardPlacement::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_SetDesiredWardPlacement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ward_index",
                    CDOTAClientMsg_SetDesiredWardPlacement::get_ward_index_for_reflect,
                    CDOTAClientMsg_SetDesiredWardPlacement::mut_ward_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "ward_x",
                    CDOTAClientMsg_SetDesiredWardPlacement::get_ward_x_for_reflect,
                    CDOTAClientMsg_SetDesiredWardPlacement::mut_ward_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "ward_y",
                    CDOTAClientMsg_SetDesiredWardPlacement::get_ward_y_for_reflect,
                    CDOTAClientMsg_SetDesiredWardPlacement::mut_ward_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_SetDesiredWardPlacement>(
                    "CDOTAClientMsg_SetDesiredWardPlacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_SetDesiredWardPlacement {
    fn clear(&mut self) {
        self.clear_ward_index();
        self.clear_ward_x();
        self.clear_ward_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_SetDesiredWardPlacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_SetDesiredWardPlacement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_RollDice {
    // message fields
    channel_type: ::std::option::Option<u32>,
    roll_min: ::std::option::Option<u32>,
    roll_max: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_RollDice {}

impl CDOTAClientMsg_RollDice {
    pub fn new() -> CDOTAClientMsg_RollDice {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_RollDice {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_RollDice> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_RollDice,
        };
        unsafe {
            instance.get(CDOTAClientMsg_RollDice::new)
        }
    }

    // optional uint32 channel_type = 1;

    pub fn clear_channel_type(&mut self) {
        self.channel_type = ::std::option::Option::None;
    }

    pub fn has_channel_type(&self) -> bool {
        self.channel_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_type(&mut self, v: u32) {
        self.channel_type = ::std::option::Option::Some(v);
    }

    pub fn get_channel_type(&self) -> u32 {
        self.channel_type.unwrap_or(0)
    }

    fn get_channel_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_type
    }

    fn mut_channel_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_type
    }

    // optional uint32 roll_min = 2;

    pub fn clear_roll_min(&mut self) {
        self.roll_min = ::std::option::Option::None;
    }

    pub fn has_roll_min(&self) -> bool {
        self.roll_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_roll_min(&mut self, v: u32) {
        self.roll_min = ::std::option::Option::Some(v);
    }

    pub fn get_roll_min(&self) -> u32 {
        self.roll_min.unwrap_or(0)
    }

    fn get_roll_min_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.roll_min
    }

    fn mut_roll_min_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.roll_min
    }

    // optional uint32 roll_max = 3;

    pub fn clear_roll_max(&mut self) {
        self.roll_max = ::std::option::Option::None;
    }

    pub fn has_roll_max(&self) -> bool {
        self.roll_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_roll_max(&mut self, v: u32) {
        self.roll_max = ::std::option::Option::Some(v);
    }

    pub fn get_roll_max(&self) -> u32 {
        self.roll_max.unwrap_or(0)
    }

    fn get_roll_max_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.roll_max
    }

    fn mut_roll_max_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.roll_max
    }
}

impl ::protobuf::Message for CDOTAClientMsg_RollDice {
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
                    self.channel_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.roll_min = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.roll_max = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.channel_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.roll_min {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.roll_max {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_type {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.roll_min {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.roll_max {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_RollDice {
    fn new() -> CDOTAClientMsg_RollDice {
        CDOTAClientMsg_RollDice::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_RollDice>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_type",
                    CDOTAClientMsg_RollDice::get_channel_type_for_reflect,
                    CDOTAClientMsg_RollDice::mut_channel_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "roll_min",
                    CDOTAClientMsg_RollDice::get_roll_min_for_reflect,
                    CDOTAClientMsg_RollDice::mut_roll_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "roll_max",
                    CDOTAClientMsg_RollDice::get_roll_max_for_reflect,
                    CDOTAClientMsg_RollDice::mut_roll_max_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_RollDice>(
                    "CDOTAClientMsg_RollDice",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_RollDice {
    fn clear(&mut self) {
        self.clear_channel_type();
        self.clear_roll_min();
        self.clear_roll_max();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_RollDice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_RollDice {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientMsg_FlipCoin {
    // message fields
    channel_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientMsg_FlipCoin {}

impl CDOTAClientMsg_FlipCoin {
    pub fn new() -> CDOTAClientMsg_FlipCoin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientMsg_FlipCoin {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientMsg_FlipCoin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientMsg_FlipCoin,
        };
        unsafe {
            instance.get(CDOTAClientMsg_FlipCoin::new)
        }
    }

    // optional uint32 channel_type = 1;

    pub fn clear_channel_type(&mut self) {
        self.channel_type = ::std::option::Option::None;
    }

    pub fn has_channel_type(&self) -> bool {
        self.channel_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_type(&mut self, v: u32) {
        self.channel_type = ::std::option::Option::Some(v);
    }

    pub fn get_channel_type(&self) -> u32 {
        self.channel_type.unwrap_or(0)
    }

    fn get_channel_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_type
    }

    fn mut_channel_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_type
    }
}

impl ::protobuf::Message for CDOTAClientMsg_FlipCoin {
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
                    self.channel_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.channel_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_type {
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

impl ::protobuf::MessageStatic for CDOTAClientMsg_FlipCoin {
    fn new() -> CDOTAClientMsg_FlipCoin {
        CDOTAClientMsg_FlipCoin::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientMsg_FlipCoin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_type",
                    CDOTAClientMsg_FlipCoin::get_channel_type_for_reflect,
                    CDOTAClientMsg_FlipCoin::mut_channel_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientMsg_FlipCoin>(
                    "CDOTAClientMsg_FlipCoin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientMsg_FlipCoin {
    fn clear(&mut self) {
        self.clear_channel_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientMsg_FlipCoin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientMsg_FlipCoin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDotaClientMessages {
    DOTA_CM_MapLine = 301,
    DOTA_CM_AspectRatio = 302,
    DOTA_CM_MapPing = 303,
    DOTA_CM_UnitsAutoAttack = 304,
    DOTA_CM_SearchString = 307,
    DOTA_CM_Pause = 308,
    DOTA_CM_ShopViewMode = 309,
    DOTA_CM_SetUnitShareFlag = 310,
    DOTA_CM_SwapRequest = 311,
    DOTA_CM_SwapAccept = 312,
    DOTA_CM_WorldLine = 313,
    DOTA_CM_RequestGraphUpdate = 314,
    DOTA_CM_ItemAlert = 315,
    DOTA_CM_ChatWheel = 316,
    DOTA_CM_SendStatPopup = 317,
    DOTA_CM_BeginLastHitChallenge = 318,
    DOTA_CM_UpdateQuickBuy = 319,
    DOTA_CM_UpdateCoachListen = 320,
    DOTA_CM_CoachHUDPing = 321,
    DOTA_CM_RecordVote = 322,
    DOTA_CM_UnitsAutoAttackAfterSpell = 323,
    DOTA_CM_WillPurchaseAlert = 324,
    DOTA_CM_PlayerShowCase = 325,
    DOTA_CM_TeleportRequiresHalt = 326,
    DOTA_CM_CameraZoomAmount = 327,
    DOTA_CM_BroadcasterUsingCamerman = 328,
    DOTA_CM_BroadcasterUsingAssistedCameraOperator = 329,
    DOTA_CM_EnemyItemAlert = 330,
    DOTA_CM_FreeInventory = 331,
    DOTA_CM_BuyBackStateAlert = 332,
    DOTA_CM_QuickBuyAlert = 333,
    DOTA_CM_HeroStatueLike = 334,
    DOTA_CM_ModifierAlert = 335,
    DOTA_CM_TeamShowcaseEditor = 336,
    DOTA_CM_HPManaAlert = 337,
    DOTA_CM_GlyphAlert = 338,
    DOTA_CM_TeamShowcaseClientData = 339,
    DOTA_CM_PlayTeamShowcase = 340,
    DOTA_CM_EventCNY2015Cmd = 341,
    DOTA_CM_FillEmptySlotsWithBots = 342,
    DOTA_CM_DemoHero = 343,
    DOTA_CM_AbilityLearnModeToggled = 344,
    DOTA_CM_AbilityStartUse = 345,
    DOTA_CM_ChallengeSelect = 346,
    DOTA_CM_ChallengeReroll = 347,
    DOTA_CM_ClickedBuff = 348,
    DOTA_CM_CoinWager = 349,
    DOTA_CM_ExecuteOrders = 350,
    DOTA_CM_XPAlert = 351,
    DOTA_CM_EventPointsTip = 353,
    DOTA_CM_MatchMetadata = 354,
    DOTA_CM_KillMyHero = 355,
    DOTA_CM_QuestStatus = 356,
    DOTA_CM_ToggleAutoattack = 357,
    DOTA_CM_SpecialAbility = 358,
    DOTA_CM_KillcamDamageTaken = 359,
    DOTA_CM_SetEnemyStartingPosition = 360,
    DOTA_CM_SetDesiredWardPlacement = 361,
    DOTA_CM_RollDice = 362,
    DOTA_CM_FlipCoin = 363,
}

impl ::protobuf::ProtobufEnum for EDotaClientMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDotaClientMessages> {
        match value {
            301 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_MapLine),
            302 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_AspectRatio),
            303 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_MapPing),
            304 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_UnitsAutoAttack),
            307 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_SearchString),
            308 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_Pause),
            309 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ShopViewMode),
            310 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_SetUnitShareFlag),
            311 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_SwapRequest),
            312 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_SwapAccept),
            313 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_WorldLine),
            314 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_RequestGraphUpdate),
            315 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ItemAlert),
            316 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ChatWheel),
            317 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_SendStatPopup),
            318 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_BeginLastHitChallenge),
            319 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_UpdateQuickBuy),
            320 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_UpdateCoachListen),
            321 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_CoachHUDPing),
            322 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_RecordVote),
            323 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_UnitsAutoAttackAfterSpell),
            324 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_WillPurchaseAlert),
            325 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_PlayerShowCase),
            326 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_TeleportRequiresHalt),
            327 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_CameraZoomAmount),
            328 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_BroadcasterUsingCamerman),
            329 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_BroadcasterUsingAssistedCameraOperator),
            330 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_EnemyItemAlert),
            331 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_FreeInventory),
            332 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_BuyBackStateAlert),
            333 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_QuickBuyAlert),
            334 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_HeroStatueLike),
            335 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ModifierAlert),
            336 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_TeamShowcaseEditor),
            337 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_HPManaAlert),
            338 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_GlyphAlert),
            339 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_TeamShowcaseClientData),
            340 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_PlayTeamShowcase),
            341 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_EventCNY2015Cmd),
            342 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_FillEmptySlotsWithBots),
            343 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_DemoHero),
            344 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_AbilityLearnModeToggled),
            345 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_AbilityStartUse),
            346 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ChallengeSelect),
            347 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ChallengeReroll),
            348 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ClickedBuff),
            349 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_CoinWager),
            350 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ExecuteOrders),
            351 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_XPAlert),
            353 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_EventPointsTip),
            354 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_MatchMetadata),
            355 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_KillMyHero),
            356 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_QuestStatus),
            357 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_ToggleAutoattack),
            358 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_SpecialAbility),
            359 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_KillcamDamageTaken),
            360 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_SetEnemyStartingPosition),
            361 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_SetDesiredWardPlacement),
            362 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_RollDice),
            363 => ::std::option::Option::Some(EDotaClientMessages::DOTA_CM_FlipCoin),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDotaClientMessages] = &[
            EDotaClientMessages::DOTA_CM_MapLine,
            EDotaClientMessages::DOTA_CM_AspectRatio,
            EDotaClientMessages::DOTA_CM_MapPing,
            EDotaClientMessages::DOTA_CM_UnitsAutoAttack,
            EDotaClientMessages::DOTA_CM_SearchString,
            EDotaClientMessages::DOTA_CM_Pause,
            EDotaClientMessages::DOTA_CM_ShopViewMode,
            EDotaClientMessages::DOTA_CM_SetUnitShareFlag,
            EDotaClientMessages::DOTA_CM_SwapRequest,
            EDotaClientMessages::DOTA_CM_SwapAccept,
            EDotaClientMessages::DOTA_CM_WorldLine,
            EDotaClientMessages::DOTA_CM_RequestGraphUpdate,
            EDotaClientMessages::DOTA_CM_ItemAlert,
            EDotaClientMessages::DOTA_CM_ChatWheel,
            EDotaClientMessages::DOTA_CM_SendStatPopup,
            EDotaClientMessages::DOTA_CM_BeginLastHitChallenge,
            EDotaClientMessages::DOTA_CM_UpdateQuickBuy,
            EDotaClientMessages::DOTA_CM_UpdateCoachListen,
            EDotaClientMessages::DOTA_CM_CoachHUDPing,
            EDotaClientMessages::DOTA_CM_RecordVote,
            EDotaClientMessages::DOTA_CM_UnitsAutoAttackAfterSpell,
            EDotaClientMessages::DOTA_CM_WillPurchaseAlert,
            EDotaClientMessages::DOTA_CM_PlayerShowCase,
            EDotaClientMessages::DOTA_CM_TeleportRequiresHalt,
            EDotaClientMessages::DOTA_CM_CameraZoomAmount,
            EDotaClientMessages::DOTA_CM_BroadcasterUsingCamerman,
            EDotaClientMessages::DOTA_CM_BroadcasterUsingAssistedCameraOperator,
            EDotaClientMessages::DOTA_CM_EnemyItemAlert,
            EDotaClientMessages::DOTA_CM_FreeInventory,
            EDotaClientMessages::DOTA_CM_BuyBackStateAlert,
            EDotaClientMessages::DOTA_CM_QuickBuyAlert,
            EDotaClientMessages::DOTA_CM_HeroStatueLike,
            EDotaClientMessages::DOTA_CM_ModifierAlert,
            EDotaClientMessages::DOTA_CM_TeamShowcaseEditor,
            EDotaClientMessages::DOTA_CM_HPManaAlert,
            EDotaClientMessages::DOTA_CM_GlyphAlert,
            EDotaClientMessages::DOTA_CM_TeamShowcaseClientData,
            EDotaClientMessages::DOTA_CM_PlayTeamShowcase,
            EDotaClientMessages::DOTA_CM_EventCNY2015Cmd,
            EDotaClientMessages::DOTA_CM_FillEmptySlotsWithBots,
            EDotaClientMessages::DOTA_CM_DemoHero,
            EDotaClientMessages::DOTA_CM_AbilityLearnModeToggled,
            EDotaClientMessages::DOTA_CM_AbilityStartUse,
            EDotaClientMessages::DOTA_CM_ChallengeSelect,
            EDotaClientMessages::DOTA_CM_ChallengeReroll,
            EDotaClientMessages::DOTA_CM_ClickedBuff,
            EDotaClientMessages::DOTA_CM_CoinWager,
            EDotaClientMessages::DOTA_CM_ExecuteOrders,
            EDotaClientMessages::DOTA_CM_XPAlert,
            EDotaClientMessages::DOTA_CM_EventPointsTip,
            EDotaClientMessages::DOTA_CM_MatchMetadata,
            EDotaClientMessages::DOTA_CM_KillMyHero,
            EDotaClientMessages::DOTA_CM_QuestStatus,
            EDotaClientMessages::DOTA_CM_ToggleAutoattack,
            EDotaClientMessages::DOTA_CM_SpecialAbility,
            EDotaClientMessages::DOTA_CM_KillcamDamageTaken,
            EDotaClientMessages::DOTA_CM_SetEnemyStartingPosition,
            EDotaClientMessages::DOTA_CM_SetDesiredWardPlacement,
            EDotaClientMessages::DOTA_CM_RollDice,
            EDotaClientMessages::DOTA_CM_FlipCoin,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EDotaClientMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDotaClientMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDotaClientMessages {
}

impl ::protobuf::reflect::ProtobufValue for EDotaClientMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74,
    0x61, 0x1a, 0x19, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5a, 0x0a, 0x16,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x4d,
    0x61, 0x70, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x40, 0x0a, 0x0d, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x0c, 0x6c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x22, 0x53, 0x0a, 0x18, 0x43, 0x44, 0x4f, 0x54,
    0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x49, 0x74, 0x65, 0x6d, 0x41,
    0x6c, 0x65, 0x72, 0x74, 0x12, 0x37, 0x0a, 0x0a, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x61, 0x6c, 0x65,
    0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x6c, 0x65,
    0x72, 0x74, 0x52, 0x09, 0x69, 0x74, 0x65, 0x6d, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x22, 0x44, 0x0a,
    0x1d, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f,
    0x45, 0x6e, 0x65, 0x6d, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x12, 0x23,
    0x0a, 0x0d, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x69, 0x74, 0x65, 0x6d, 0x45, 0x6e, 0x74, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x22, 0x77, 0x0a, 0x1c, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x41, 0x6c,
    0x65, 0x72, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x62, 0x75, 0x66, 0x66, 0x5f, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x11, 0x62, 0x75, 0x66, 0x66, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x49, 0x6e,
    0x64, 0x65, 0x78, 0x12, 0x27, 0x0a, 0x0f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x65, 0x6e,
    0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x74, 0x61,
    0x72, 0x67, 0x65, 0x74, 0x45, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x22, 0x75, 0x0a, 0x1a,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x43,
    0x6c, 0x69, 0x63, 0x6b, 0x65, 0x64, 0x42, 0x75, 0x66, 0x66, 0x12, 0x2e, 0x0a, 0x13, 0x62, 0x75,
    0x66, 0x66, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x5f, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11, 0x62, 0x75, 0x66, 0x66, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x27, 0x0a, 0x0f, 0x74, 0x61,
    0x72, 0x67, 0x65, 0x74, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0e, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x45, 0x6e, 0x74, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x22, 0x45, 0x0a, 0x1a, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x48, 0x50, 0x4d, 0x61, 0x6e, 0x61, 0x41, 0x6c, 0x65, 0x72,
    0x74, 0x12, 0x27, 0x0a, 0x0f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x65, 0x6e, 0x74, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x74, 0x61, 0x72, 0x67,
    0x65, 0x74, 0x45, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x22, 0x37, 0x0a, 0x19, 0x43, 0x44,
    0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x6c, 0x79,
    0x70, 0x68, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x12, 0x1a, 0x0a, 0x08, 0x6e, 0x65, 0x67, 0x61, 0x74,
    0x69, 0x76, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x6e, 0x65, 0x67, 0x61, 0x74,
    0x69, 0x76, 0x65, 0x22, 0x4a, 0x0a, 0x16, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x61, 0x70, 0x4c, 0x69, 0x6e, 0x65, 0x12, 0x30, 0x0a,
    0x07, 0x6d, 0x61, 0x70, 0x6c, 0x69, 0x6e, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x4d,
    0x61, 0x70, 0x4c, 0x69, 0x6e, 0x65, 0x52, 0x07, 0x6d, 0x61, 0x70, 0x6c, 0x69, 0x6e, 0x65, 0x22,
    0x32, 0x0a, 0x1a, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73,
    0x67, 0x5f, 0x41, 0x73, 0x70, 0x65, 0x63, 0x74, 0x52, 0x61, 0x74, 0x69, 0x6f, 0x12, 0x14, 0x0a,
    0x05, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x22, 0xbc, 0x02, 0x0a, 0x22, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x73, 0x41, 0x75, 0x74, 0x6f,
    0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x4b, 0x0a, 0x04, 0x6d, 0x6f,
    0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2e, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x55,
    0x6e, 0x69, 0x74, 0x73, 0x41, 0x75, 0x74, 0x6f, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x4d, 0x6f,
    0x64, 0x65, 0x2e, 0x45, 0x4d, 0x6f, 0x64, 0x65, 0x3a, 0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49,
    0x44, 0x52, 0x04, 0x6d, 0x6f, 0x64, 0x65, 0x12, 0x57, 0x0a, 0x09, 0x75, 0x6e, 0x69, 0x74, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x32, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67,
    0x5f, 0x55, 0x6e, 0x69, 0x74, 0x73, 0x41, 0x75, 0x74, 0x6f, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b,
    0x4d, 0x6f, 0x64, 0x65, 0x2e, 0x45, 0x55, 0x6e, 0x69, 0x74, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x06,
    0x4e, 0x4f, 0x52, 0x4d, 0x41, 0x4c, 0x52, 0x08, 0x75, 0x6e, 0x69, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x22, 0x49, 0x0a, 0x05, 0x45, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x14, 0x0a, 0x07, 0x49, 0x4e, 0x56,
    0x41, 0x4c, 0x49, 0x44, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x12,
    0x09, 0x0a, 0x05, 0x4e, 0x45, 0x56, 0x45, 0x52, 0x10, 0x00, 0x12, 0x13, 0x0a, 0x0f, 0x41, 0x46,
    0x54, 0x45, 0x52, 0x5f, 0x53, 0x50, 0x45, 0x4c, 0x4c, 0x43, 0x41, 0x53, 0x54, 0x10, 0x01, 0x12,
    0x0a, 0x0a, 0x06, 0x41, 0x4c, 0x57, 0x41, 0x59, 0x53, 0x10, 0x02, 0x22, 0x25, 0x0a, 0x09, 0x45,
    0x55, 0x6e, 0x69, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x4e, 0x4f, 0x52, 0x4d,
    0x41, 0x4c, 0x10, 0x00, 0x12, 0x0c, 0x0a, 0x08, 0x53, 0x55, 0x4d, 0x4d, 0x4f, 0x4e, 0x45, 0x44,
    0x10, 0x01, 0x22, 0x44, 0x0a, 0x28, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x73, 0x41, 0x75, 0x74, 0x6f, 0x41, 0x74,
    0x74, 0x61, 0x63, 0x6b, 0x41, 0x66, 0x74, 0x65, 0x72, 0x53, 0x70, 0x65, 0x6c, 0x6c, 0x12, 0x18,
    0x0a, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x22, 0x3f, 0x0a, 0x23, 0x43, 0x44, 0x4f, 0x54,
    0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x54, 0x65, 0x6c, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x48, 0x61, 0x6c, 0x74, 0x12,
    0x18, 0x0a, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x07, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x22, 0x35, 0x0a, 0x1b, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x61, 0x72,
    0x63, 0x68, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x65, 0x61, 0x72,
    0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68,
    0x22, 0x16, 0x0a, 0x14, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d,
    0x73, 0x67, 0x5f, 0x50, 0x61, 0x75, 0x73, 0x65, 0x22, 0x31, 0x0a, 0x1b, 0x43, 0x44, 0x4f, 0x54,
    0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x68, 0x6f, 0x70, 0x56,
    0x69, 0x65, 0x77, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x6f, 0x64, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x6d, 0x6f, 0x64, 0x65, 0x22, 0x67, 0x0a, 0x1f, 0x43,
    0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65,
    0x74, 0x55, 0x6e, 0x69, 0x74, 0x53, 0x68, 0x61, 0x72, 0x65, 0x46, 0x6c, 0x61, 0x67, 0x12, 0x1a,
    0x0a, 0x08, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x44, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x08, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x44, 0x12, 0x12, 0x0a, 0x04, 0x66, 0x6c,
    0x61, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x66, 0x6c, 0x61, 0x67, 0x12, 0x14,
    0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x05, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x22, 0x39, 0x0a, 0x1a, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x77, 0x61, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x64, 0x22,
    0x38, 0x0a, 0x19, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73,
    0x67, 0x5f, 0x53, 0x77, 0x61, 0x70, 0x41, 0x63, 0x63, 0x65, 0x70, 0x74, 0x12, 0x1b, 0x0a, 0x09,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x08, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x64, 0x22, 0x52, 0x0a, 0x18, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x57, 0x6f, 0x72, 0x6c,
    0x64, 0x4c, 0x69, 0x6e, 0x65, 0x12, 0x36, 0x0a, 0x09, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x6c, 0x69,
    0x6e, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x4c, 0x69,
    0x6e, 0x65, 0x52, 0x09, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x6c, 0x69, 0x6e, 0x65, 0x22, 0x23, 0x0a,
    0x21, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x47, 0x72, 0x61, 0x70, 0x68, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x22, 0x8d, 0x01, 0x0a, 0x18, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x68, 0x61, 0x74, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x12,
    0x4d, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x74, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x3a, 0x0d, 0x6b, 0x5f, 0x45, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x57, 0x5f, 0x4f,
    0x6b, 0x52, 0x0b, 0x63, 0x68, 0x61, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x22,
    0x0a, 0x0d, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x5f, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x48, 0x65, 0x72, 0x6f,
    0x49, 0x64, 0x22, 0x5a, 0x0a, 0x1c, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x53, 0x74, 0x61, 0x74, 0x50, 0x6f, 0x70,
    0x75, 0x70, 0x12, 0x3a, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x74, 0x70, 0x6f, 0x70, 0x75, 0x70, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x53, 0x74, 0x61, 0x74, 0x50, 0x6f,
    0x70, 0x75, 0x70, 0x52, 0x09, 0x73, 0x74, 0x61, 0x74, 0x70, 0x6f, 0x70, 0x75, 0x70, 0x22, 0x6e,
    0x0a, 0x24, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67,
    0x5f, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x4c, 0x61, 0x73, 0x74, 0x48, 0x69, 0x74, 0x43, 0x68, 0x61,
    0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x63, 0x68, 0x6f, 0x73, 0x65, 0x6e,
    0x5f, 0x6c, 0x61, 0x6e, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x63, 0x68, 0x6f,
    0x73, 0x65, 0x6e, 0x4c, 0x61, 0x6e, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x68, 0x65, 0x6c, 0x70, 0x65,
    0x72, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x0d, 0x68, 0x65, 0x6c, 0x70, 0x65, 0x72, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x22, 0x62,
    0x0a, 0x21, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67,
    0x5f, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x51, 0x75, 0x69, 0x63, 0x6b, 0x42, 0x75, 0x79, 0x49,
    0x74, 0x65, 0x6d, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x69, 0x74, 0x65, 0x6d, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x20, 0x0a, 0x0b, 0x70, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x70, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x61, 0x62,
    0x6c, 0x65, 0x22, 0x5e, 0x0a, 0x1d, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x51, 0x75, 0x69, 0x63, 0x6b,
    0x42, 0x75, 0x79, 0x12, 0x3d, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x27, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x51,
    0x75, 0x69, 0x63, 0x6b, 0x42, 0x75, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69, 0x74, 0x65,
    0x6d, 0x73, 0x22, 0x3e, 0x0a, 0x19, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x56, 0x6f, 0x74, 0x65, 0x12,
    0x21, 0x0a, 0x0c, 0x63, 0x68, 0x6f, 0x69, 0x63, 0x65, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x63, 0x68, 0x6f, 0x69, 0x63, 0x65, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x22, 0x61, 0x0a, 0x20, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x4d, 0x73, 0x67, 0x5f, 0x57, 0x69, 0x6c, 0x6c, 0x50, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73,
    0x65, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x69, 0x64, 0x12, 0x25,
    0x0a, 0x0e, 0x67, 0x6f, 0x6c, 0x64, 0x5f, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x67, 0x6f, 0x6c, 0x64, 0x52, 0x65, 0x6d, 0x61,
    0x69, 0x6e, 0x69, 0x6e, 0x67, 0x22, 0x22, 0x0a, 0x20, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x42, 0x75, 0x79, 0x42, 0x61, 0x63, 0x6b, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x22, 0x5b, 0x0a, 0x1c, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x51, 0x75, 0x69, 0x63,
    0x6b, 0x42, 0x75, 0x79, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x69, 0x74, 0x65,
    0x6d, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x69,
    0x64, 0x12, 0x23, 0x0a, 0x0d, 0x67, 0x6f, 0x6c, 0x64, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72,
    0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x67, 0x6f, 0x6c, 0x64, 0x52, 0x65,
    0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x22, 0x3b, 0x0a, 0x1d, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x53,
    0x68, 0x6f, 0x77, 0x43, 0x61, 0x73, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x68, 0x6f, 0x77, 0x63,
    0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x73, 0x68, 0x6f, 0x77, 0x63,
    0x61, 0x73, 0x65, 0x22, 0x42, 0x0a, 0x1f, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5a, 0x6f, 0x6f, 0x6d,
    0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x7a, 0x6f, 0x6f, 0x6d, 0x5f, 0x61,
    0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x7a, 0x6f, 0x6f,
    0x6d, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x48, 0x0a, 0x28, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x55, 0x73, 0x69, 0x6e, 0x67, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x6d, 0x61, 0x6e, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x6d, 0x61, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x6d, 0x61,
    0x6e, 0x22, 0x51, 0x0a, 0x35, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x4d, 0x73, 0x67, 0x5f, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x65, 0x72, 0x55,
    0x73, 0x69, 0x6e, 0x67, 0x41, 0x73, 0x73, 0x69, 0x73, 0x74, 0x65, 0x64, 0x43, 0x61, 0x6d, 0x65,
    0x72, 0x61, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x6e,
    0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x65, 0x6e, 0x61,
    0x62, 0x6c, 0x65, 0x64, 0x22, 0x70, 0x0a, 0x1d, 0x43, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x45, 0x71, 0x75, 0x69, 0x70, 0x53, 0x6c, 0x6f, 0x74, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x19, 0x0a, 0x08, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x49, 0x64,
    0x12, 0x17, 0x0a, 0x07, 0x73, 0x6c, 0x6f, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x06, 0x73, 0x6c, 0x6f, 0x74, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x65, 0x66,
    0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x64, 0x65,
    0x66, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x22, 0x5b, 0x0a, 0x1c, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x72, 0x65, 0x65, 0x49, 0x6e, 0x76,
    0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x12, 0x3b, 0x0a, 0x06, 0x65, 0x71, 0x75, 0x69, 0x70, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x41,
    0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x45, 0x71, 0x75, 0x69, 0x70, 0x53, 0x6c,
    0x6f, 0x74, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x52, 0x06, 0x65, 0x71, 0x75,
    0x69, 0x70, 0x73, 0x22, 0x4b, 0x0a, 0x25, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x46, 0x69, 0x6c, 0x6c, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x53,
    0x6c, 0x6f, 0x74, 0x73, 0x57, 0x69, 0x74, 0x68, 0x42, 0x6f, 0x74, 0x73, 0x12, 0x22, 0x0a, 0x0c,
    0x66, 0x69, 0x6c, 0x6c, 0x77, 0x69, 0x74, 0x68, 0x62, 0x6f, 0x74, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0c, 0x66, 0x69, 0x6c, 0x6c, 0x77, 0x69, 0x74, 0x68, 0x62, 0x6f, 0x74, 0x73,
    0x22, 0x47, 0x0a, 0x1d, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d,
    0x73, 0x67, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x74, 0x61, 0x74, 0x75, 0x65, 0x4c, 0x69, 0x6b,
    0x65, 0x12, 0x26, 0x0a, 0x0f, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x64, 0x22, 0x34, 0x0a, 0x1e, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x43, 0x4e, 0x59, 0x32, 0x30, 0x31, 0x35, 0x43, 0x6d, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22,
    0xea, 0x01, 0x0a, 0x17, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d,
    0x73, 0x67, 0x5f, 0x44, 0x65, 0x6d, 0x6f, 0x48, 0x65, 0x72, 0x6f, 0x12, 0x17, 0x0a, 0x07, 0x68,
    0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x68, 0x65,
    0x72, 0x6f, 0x49, 0x64, 0x12, 0x27, 0x0a, 0x10, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64, 0x5f,
    0x74, 0x6f, 0x5f, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d,
    0x68, 0x65, 0x72, 0x6f, 0x49, 0x64, 0x54, 0x6f, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x12, 0x1b, 0x0a,
    0x09, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x64, 0x65, 0x66, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0d,
    0x52, 0x08, 0x69, 0x74, 0x65, 0x6d, 0x44, 0x65, 0x66, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x69, 0x74,
    0x65, 0x6d, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x04, 0x52, 0x07, 0x69, 0x74,
    0x65, 0x6d, 0x49, 0x64, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x74, 0x79, 0x6c, 0x65, 0x5f, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x73, 0x74, 0x79, 0x6c,
    0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x34, 0x0a, 0x16, 0x6b, 0x65, 0x65, 0x70, 0x5f, 0x65,
    0x78, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x64, 0x65, 0x6d, 0x6f, 0x68, 0x65, 0x72, 0x6f,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x14, 0x6b, 0x65, 0x65, 0x70, 0x45, 0x78, 0x69, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x44, 0x65, 0x6d, 0x6f, 0x68, 0x65, 0x72, 0x6f, 0x22, 0x75, 0x0a, 0x1e,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x43,
    0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x12, 0x19,
    0x0a, 0x08, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x73, 0x6c, 0x6f,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x73, 0x6c, 0x6f, 0x74,
    0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63,
    0x65, 0x49, 0x64, 0x22, 0x75, 0x0a, 0x1e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52,
    0x65, 0x72, 0x6f, 0x6c, 0x6c, 0x12, 0x19, 0x0a, 0x08, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x49, 0x64,
    0x12, 0x17, 0x0a, 0x07, 0x73, 0x6c, 0x6f, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x06, 0x73, 0x6c, 0x6f, 0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x71,
    0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a,
    0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x49, 0x64, 0x22, 0x3d, 0x0a, 0x18, 0x43, 0x44,
    0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6f, 0x69,
    0x6e, 0x57, 0x61, 0x67, 0x65, 0x72, 0x12, 0x21, 0x0a, 0x0c, 0x77, 0x61, 0x67, 0x65, 0x72, 0x5f,
    0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x77, 0x61,
    0x67, 0x65, 0x72, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x4f, 0x0a, 0x1d, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x54, 0x69, 0x70, 0x12, 0x2e, 0x0a, 0x13, 0x72, 0x65,
    0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65,
    0x6e, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x64, 0x22, 0x50, 0x0a, 0x1c, 0x43, 0x44,
    0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x12, 0x30, 0x0a, 0x06, 0x6f, 0x72,
    0x64, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x73, 0x67, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x4f,
    0x72, 0x64, 0x65, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x22, 0x64, 0x0a, 0x16,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x58,
    0x50, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x12, 0x27, 0x0a, 0x0f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74,
    0x5f, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0e, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x45, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12,
    0x21, 0x0a, 0x0c, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x61, 0x6b, 0x65, 0x6e, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x54, 0x61, 0x6b,
    0x65, 0x6e, 0x22, 0xe5, 0x01, 0x0a, 0x21, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x4b, 0x69, 0x6c, 0x6c, 0x63, 0x61, 0x6d, 0x44, 0x61, 0x6d,
    0x61, 0x67, 0x65, 0x54, 0x61, 0x6b, 0x65, 0x6e, 0x12, 0x27, 0x0a, 0x0f, 0x74, 0x61, 0x72, 0x67,
    0x65, 0x74, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0e, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x45, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x5f, 0x74, 0x61, 0x6b, 0x65,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x54,
    0x61, 0x6b, 0x65, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x69, 0x74, 0x65, 0x6d, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x68, 0x65,
    0x72, 0x6f, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x68,
    0x65, 0x72, 0x6f, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x61, 0x6d, 0x61, 0x67,
    0x65, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64,
    0x61, 0x6d, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x22, 0x55, 0x0a, 0x1c, 0x43, 0x44,
    0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0x22, 0x1b, 0x0a, 0x19, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x4d, 0x73, 0x67, 0x5f, 0x4b, 0x69, 0x6c, 0x6c, 0x4d, 0x79, 0x48, 0x65, 0x72, 0x6f, 0x22, 0xde,
    0x01, 0x0a, 0x1a, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73,
    0x67, 0x5f, 0x51, 0x75, 0x65, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x19, 0x0a,
    0x08, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x07, 0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6c,
    0x6c, 0x65, 0x6e, 0x67, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b,
    0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x49, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x70,
    0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x70,
    0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x67, 0x6f, 0x61, 0x6c, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x67, 0x6f, 0x61, 0x6c, 0x12, 0x14, 0x0a, 0x05, 0x71,
    0x75, 0x65, 0x72, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x71, 0x75, 0x65, 0x72,
    0x79, 0x12, 0x23, 0x0a, 0x0d, 0x66, 0x61, 0x69, 0x6c, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x66, 0x61, 0x69, 0x6c, 0x47, 0x61,
    0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x69,
    0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64, 0x22,
    0x58, 0x0a, 0x1f, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73,
    0x67, 0x5f, 0x54, 0x6f, 0x67, 0x67, 0x6c, 0x65, 0x41, 0x75, 0x74, 0x6f, 0x61, 0x74, 0x74, 0x61,
    0x63, 0x6b, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x04, 0x6d, 0x6f, 0x64, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x68, 0x6f, 0x77, 0x5f, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x73, 0x68,
    0x6f, 0x77, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x6d, 0x0a, 0x1d, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x65, 0x63,
    0x69, 0x61, 0x6c, 0x41, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x23, 0x0a, 0x0d, 0x61, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0c, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12,
    0x27, 0x0a, 0x0f, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74,
    0x45, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x22, 0x89, 0x01, 0x0a, 0x27, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x74, 0x45,
    0x6e, 0x65, 0x6d, 0x79, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x50, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x26, 0x0a, 0x0f, 0x65, 0x6e, 0x65, 0x6d, 0x79, 0x5f, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x65,
    0x6e, 0x65, 0x6d, 0x79, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x64, 0x12, 0x36, 0x0a, 0x17,
    0x65, 0x6e, 0x65, 0x6d, 0x79, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x70,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x15, 0x65,
    0x6e, 0x65, 0x6d, 0x79, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x50, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x22, 0x75, 0x0a, 0x26, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x74, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65,
    0x64, 0x57, 0x61, 0x72, 0x64, 0x50, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x1d,
    0x0a, 0x0a, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x09, 0x77, 0x61, 0x72, 0x64, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x15, 0x0a,
    0x06, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x77,
    0x61, 0x72, 0x64, 0x58, 0x12, 0x15, 0x0a, 0x06, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x77, 0x61, 0x72, 0x64, 0x59, 0x22, 0x72, 0x0a, 0x17, 0x43,
    0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x6f,
    0x6c, 0x6c, 0x44, 0x69, 0x63, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
    0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x63, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x72, 0x6f, 0x6c,
    0x6c, 0x5f, 0x6d, 0x69, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x72, 0x6f, 0x6c,
    0x6c, 0x4d, 0x69, 0x6e, 0x12, 0x19, 0x0a, 0x08, 0x72, 0x6f, 0x6c, 0x6c, 0x5f, 0x6d, 0x61, 0x78,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x72, 0x6f, 0x6c, 0x6c, 0x4d, 0x61, 0x78, 0x22,
    0x3c, 0x0a, 0x17, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73,
    0x67, 0x5f, 0x46, 0x6c, 0x69, 0x70, 0x43, 0x6f, 0x69, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x2a, 0xfa, 0x0d,
    0x0a, 0x13, 0x45, 0x44, 0x6f, 0x74, 0x61, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x14, 0x0a, 0x0f, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d,
    0x5f, 0x4d, 0x61, 0x70, 0x4c, 0x69, 0x6e, 0x65, 0x10, 0xad, 0x02, 0x12, 0x18, 0x0a, 0x13, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x41, 0x73, 0x70, 0x65, 0x63, 0x74, 0x52, 0x61, 0x74,
    0x69, 0x6f, 0x10, 0xae, 0x02, 0x12, 0x14, 0x0a, 0x0f, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d,
    0x5f, 0x4d, 0x61, 0x70, 0x50, 0x69, 0x6e, 0x67, 0x10, 0xaf, 0x02, 0x12, 0x1c, 0x0a, 0x17, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x73, 0x41, 0x75, 0x74, 0x6f,
    0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x10, 0xb0, 0x02, 0x12, 0x19, 0x0a, 0x14, 0x44, 0x4f, 0x54,
    0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x53, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x10, 0xb3, 0x02, 0x12, 0x12, 0x0a, 0x0d, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f,
    0x50, 0x61, 0x75, 0x73, 0x65, 0x10, 0xb4, 0x02, 0x12, 0x19, 0x0a, 0x14, 0x44, 0x4f, 0x54, 0x41,
    0x5f, 0x43, 0x4d, 0x5f, 0x53, 0x68, 0x6f, 0x70, 0x56, 0x69, 0x65, 0x77, 0x4d, 0x6f, 0x64, 0x65,
    0x10, 0xb5, 0x02, 0x12, 0x1d, 0x0a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x53,
    0x65, 0x74, 0x55, 0x6e, 0x69, 0x74, 0x53, 0x68, 0x61, 0x72, 0x65, 0x46, 0x6c, 0x61, 0x67, 0x10,
    0xb6, 0x02, 0x12, 0x18, 0x0a, 0x13, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x53, 0x77,
    0x61, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0xb7, 0x02, 0x12, 0x17, 0x0a, 0x12,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x53, 0x77, 0x61, 0x70, 0x41, 0x63, 0x63, 0x65,
    0x70, 0x74, 0x10, 0xb8, 0x02, 0x12, 0x16, 0x0a, 0x11, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d,
    0x5f, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x4c, 0x69, 0x6e, 0x65, 0x10, 0xb9, 0x02, 0x12, 0x1f, 0x0a,
    0x1a, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x47, 0x72, 0x61, 0x70, 0x68, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x10, 0xba, 0x02, 0x12, 0x16,
    0x0a, 0x11, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x6c,
    0x65, 0x72, 0x74, 0x10, 0xbb, 0x02, 0x12, 0x16, 0x0a, 0x11, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43,
    0x4d, 0x5f, 0x43, 0x68, 0x61, 0x74, 0x57, 0x68, 0x65, 0x65, 0x6c, 0x10, 0xbc, 0x02, 0x12, 0x1a,
    0x0a, 0x15, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x53, 0x74,
    0x61, 0x74, 0x50, 0x6f, 0x70, 0x75, 0x70, 0x10, 0xbd, 0x02, 0x12, 0x22, 0x0a, 0x1d, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x4c, 0x61, 0x73, 0x74, 0x48,
    0x69, 0x74, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x10, 0xbe, 0x02, 0x12, 0x1b,
    0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x51, 0x75, 0x69, 0x63, 0x6b, 0x42, 0x75, 0x79, 0x10, 0xbf, 0x02, 0x12, 0x1e, 0x0a, 0x19, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x43, 0x6f, 0x61,
    0x63, 0x68, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x10, 0xc0, 0x02, 0x12, 0x19, 0x0a, 0x14, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x43, 0x6f, 0x61, 0x63, 0x68, 0x48, 0x55, 0x44, 0x50,
    0x69, 0x6e, 0x67, 0x10, 0xc1, 0x02, 0x12, 0x17, 0x0a, 0x12, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43,
    0x4d, 0x5f, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x56, 0x6f, 0x74, 0x65, 0x10, 0xc2, 0x02, 0x12,
    0x26, 0x0a, 0x21, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x55, 0x6e, 0x69, 0x74, 0x73,
    0x41, 0x75, 0x74, 0x6f, 0x41, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x41, 0x66, 0x74, 0x65, 0x72, 0x53,
    0x70, 0x65, 0x6c, 0x6c, 0x10, 0xc3, 0x02, 0x12, 0x1e, 0x0a, 0x19, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x43, 0x4d, 0x5f, 0x57, 0x69, 0x6c, 0x6c, 0x50, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x65, 0x41,
    0x6c, 0x65, 0x72, 0x74, 0x10, 0xc4, 0x02, 0x12, 0x1b, 0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x43, 0x4d, 0x5f, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x53, 0x68, 0x6f, 0x77, 0x43, 0x61, 0x73,
    0x65, 0x10, 0xc5, 0x02, 0x12, 0x21, 0x0a, 0x1c, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f,
    0x54, 0x65, 0x6c, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73,
    0x48, 0x61, 0x6c, 0x74, 0x10, 0xc6, 0x02, 0x12, 0x1d, 0x0a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x43, 0x4d, 0x5f, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5a, 0x6f, 0x6f, 0x6d, 0x41, 0x6d, 0x6f,
    0x75, 0x6e, 0x74, 0x10, 0xc7, 0x02, 0x12, 0x25, 0x0a, 0x20, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43,
    0x4d, 0x5f, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x65, 0x72, 0x55, 0x73, 0x69,
    0x6e, 0x67, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x6d, 0x61, 0x6e, 0x10, 0xc8, 0x02, 0x12, 0x33, 0x0a,
    0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x55, 0x73, 0x69, 0x6e, 0x67, 0x41, 0x73, 0x73, 0x69, 0x73, 0x74, 0x65,
    0x64, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x10,
    0xc9, 0x02, 0x12, 0x1b, 0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x45, 0x6e,
    0x65, 0x6d, 0x79, 0x49, 0x74, 0x65, 0x6d, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x10, 0xca, 0x02, 0x12,
    0x1a, 0x0a, 0x15, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x46, 0x72, 0x65, 0x65, 0x49,
    0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x10, 0xcb, 0x02, 0x12, 0x1e, 0x0a, 0x19, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x42, 0x75, 0x79, 0x42, 0x61, 0x63, 0x6b, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x10, 0xcc, 0x02, 0x12, 0x1a, 0x0a, 0x15, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x51, 0x75, 0x69, 0x63, 0x6b, 0x42, 0x75, 0x79, 0x41,
    0x6c, 0x65, 0x72, 0x74, 0x10, 0xcd, 0x02, 0x12, 0x1b, 0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x43, 0x4d, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x74, 0x61, 0x74, 0x75, 0x65, 0x4c, 0x69, 0x6b,
    0x65, 0x10, 0xce, 0x02, 0x12, 0x1a, 0x0a, 0x15, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f,
    0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x10, 0xcf, 0x02,
    0x12, 0x1f, 0x0a, 0x1a, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x54, 0x65, 0x61, 0x6d,
    0x53, 0x68, 0x6f, 0x77, 0x63, 0x61, 0x73, 0x65, 0x45, 0x64, 0x69, 0x74, 0x6f, 0x72, 0x10, 0xd0,
    0x02, 0x12, 0x18, 0x0a, 0x13, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x48, 0x50, 0x4d,
    0x61, 0x6e, 0x61, 0x41, 0x6c, 0x65, 0x72, 0x74, 0x10, 0xd1, 0x02, 0x12, 0x17, 0x0a, 0x12, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x47, 0x6c, 0x79, 0x70, 0x68, 0x41, 0x6c, 0x65, 0x72,
    0x74, 0x10, 0xd2, 0x02, 0x12, 0x23, 0x0a, 0x1e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f,
    0x54, 0x65, 0x61, 0x6d, 0x53, 0x68, 0x6f, 0x77, 0x63, 0x61, 0x73, 0x65, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x44, 0x61, 0x74, 0x61, 0x10, 0xd3, 0x02, 0x12, 0x1d, 0x0a, 0x18, 0x44, 0x4f, 0x54,
    0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x50, 0x6c, 0x61, 0x79, 0x54, 0x65, 0x61, 0x6d, 0x53, 0x68, 0x6f,
    0x77, 0x63, 0x61, 0x73, 0x65, 0x10, 0xd4, 0x02, 0x12, 0x1c, 0x0a, 0x17, 0x44, 0x4f, 0x54, 0x41,
    0x5f, 0x43, 0x4d, 0x5f, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x43, 0x4e, 0x59, 0x32, 0x30, 0x31, 0x35,
    0x43, 0x6d, 0x64, 0x10, 0xd5, 0x02, 0x12, 0x23, 0x0a, 0x1e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43,
    0x4d, 0x5f, 0x46, 0x69, 0x6c, 0x6c, 0x45, 0x6d, 0x70, 0x74, 0x79, 0x53, 0x6c, 0x6f, 0x74, 0x73,
    0x57, 0x69, 0x74, 0x68, 0x42, 0x6f, 0x74, 0x73, 0x10, 0xd6, 0x02, 0x12, 0x15, 0x0a, 0x10, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x44, 0x65, 0x6d, 0x6f, 0x48, 0x65, 0x72, 0x6f, 0x10,
    0xd7, 0x02, 0x12, 0x24, 0x0a, 0x1f, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x41, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x79, 0x4c, 0x65, 0x61, 0x72, 0x6e, 0x4d, 0x6f, 0x64, 0x65, 0x54, 0x6f,
    0x67, 0x67, 0x6c, 0x65, 0x64, 0x10, 0xd8, 0x02, 0x12, 0x1c, 0x0a, 0x17, 0x44, 0x4f, 0x54, 0x41,
    0x5f, 0x43, 0x4d, 0x5f, 0x41, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x55, 0x73, 0x65, 0x10, 0xd9, 0x02, 0x12, 0x1c, 0x0a, 0x17, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43,
    0x4d, 0x5f, 0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x53, 0x65, 0x6c, 0x65, 0x63,
    0x74, 0x10, 0xda, 0x02, 0x12, 0x1c, 0x0a, 0x17, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f,
    0x43, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x52, 0x65, 0x72, 0x6f, 0x6c, 0x6c, 0x10,
    0xdb, 0x02, 0x12, 0x18, 0x0a, 0x13, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x43, 0x6c,
    0x69, 0x63, 0x6b, 0x65, 0x64, 0x42, 0x75, 0x66, 0x66, 0x10, 0xdc, 0x02, 0x12, 0x16, 0x0a, 0x11,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x43, 0x6f, 0x69, 0x6e, 0x57, 0x61, 0x67, 0x65,
    0x72, 0x10, 0xdd, 0x02, 0x12, 0x1a, 0x0a, 0x15, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x73, 0x10, 0xde, 0x02,
    0x12, 0x14, 0x0a, 0x0f, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x58, 0x50, 0x41, 0x6c,
    0x65, 0x72, 0x74, 0x10, 0xdf, 0x02, 0x12, 0x1b, 0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43,
    0x4d, 0x5f, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x54, 0x69, 0x70,
    0x10, 0xe1, 0x02, 0x12, 0x1a, 0x0a, 0x15, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x10, 0xe2, 0x02, 0x12,
    0x17, 0x0a, 0x12, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x4b, 0x69, 0x6c, 0x6c, 0x4d,
    0x79, 0x48, 0x65, 0x72, 0x6f, 0x10, 0xe3, 0x02, 0x12, 0x18, 0x0a, 0x13, 0x44, 0x4f, 0x54, 0x41,
    0x5f, 0x43, 0x4d, 0x5f, 0x51, 0x75, 0x65, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x10,
    0xe4, 0x02, 0x12, 0x1d, 0x0a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x54, 0x6f,
    0x67, 0x67, 0x6c, 0x65, 0x41, 0x75, 0x74, 0x6f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6b, 0x10, 0xe5,
    0x02, 0x12, 0x1b, 0x0a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x53, 0x70, 0x65,
    0x63, 0x69, 0x61, 0x6c, 0x41, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x10, 0xe6, 0x02, 0x12, 0x1f,
    0x0a, 0x1a, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x4b, 0x69, 0x6c, 0x6c, 0x63, 0x61,
    0x6d, 0x44, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x54, 0x61, 0x6b, 0x65, 0x6e, 0x10, 0xe7, 0x02, 0x12,
    0x25, 0x0a, 0x20, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x53, 0x65, 0x74, 0x45, 0x6e,
    0x65, 0x6d, 0x79, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x10, 0xe8, 0x02, 0x12, 0x24, 0x0a, 0x1f, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43,
    0x4d, 0x5f, 0x53, 0x65, 0x74, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x57, 0x61, 0x72, 0x64,
    0x50, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x10, 0xe9, 0x02, 0x12, 0x15, 0x0a, 0x10,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x52, 0x6f, 0x6c, 0x6c, 0x44, 0x69, 0x63, 0x65,
    0x10, 0xea, 0x02, 0x12, 0x15, 0x0a, 0x10, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x46,
    0x6c, 0x69, 0x70, 0x43, 0x6f, 0x69, 0x6e, 0x10, 0xeb, 0x02, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01,
    0x00, 0x4a, 0xac, 0x58, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xd3, 0x02, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08,
    0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x22, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x06, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x06, 0x07,
    0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x07, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x1b, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01,
    0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03,
    0x07, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07,
    0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07,
    0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1d, 0x22,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x46, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x09, 0x05, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0a, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x1a,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x0c, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x0c, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x0c, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0d, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x0e, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0f,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0f, 0x18, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x06, 0x02, 0x12, 0x03, 0x10, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x11, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x11,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x11, 0x23, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x12, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x12, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x09, 0x12, 0x03, 0x13, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12,
    0x03, 0x13, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x13,
    0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x14, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x14, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x14, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x0b, 0x12, 0x03, 0x15, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b,
    0x01, 0x12, 0x03, 0x15, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12,
    0x03, 0x15, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x16, 0x08,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x16, 0x08, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x16, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x17, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0d, 0x01, 0x12, 0x03, 0x17, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d,
    0x02, 0x12, 0x03, 0x17, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0e, 0x12, 0x03,
    0x18, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x18, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x18, 0x20, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x19, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x19, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0f, 0x02, 0x12, 0x03, 0x19, 0x28, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x10,
    0x12, 0x03, 0x1a, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03,
    0x1a, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x02, 0x12, 0x03, 0x1a, 0x21,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x11, 0x12, 0x03, 0x1b, 0x08, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x11, 0x02, 0x12, 0x03, 0x1b, 0x24, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x12, 0x12, 0x03, 0x1c, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x12, 0x01,
    0x12, 0x03, 0x1c, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x12, 0x02, 0x12, 0x03,
    0x1c, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x13, 0x12, 0x03, 0x1d, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x02, 0x12, 0x03, 0x1d, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x14, 0x12, 0x03, 0x1e, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x14, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x14, 0x02,
    0x12, 0x03, 0x1e, 0x2c, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x15, 0x12, 0x03, 0x1f,
    0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x15, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x15, 0x02, 0x12, 0x03, 0x1f, 0x24, 0x27, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x16, 0x12, 0x03, 0x20, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x16, 0x01, 0x12, 0x03, 0x20, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x16, 0x02, 0x12, 0x03, 0x20, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x17, 0x12,
    0x03, 0x21, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x17, 0x01, 0x12, 0x03, 0x21,
    0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x17, 0x02, 0x12, 0x03, 0x21, 0x27, 0x2a,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x18, 0x12, 0x03, 0x22, 0x08, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x18, 0x01, 0x12, 0x03, 0x22, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x18, 0x02, 0x12, 0x03, 0x22, 0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x19, 0x12, 0x03, 0x23, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x19, 0x01, 0x12,
    0x03, 0x23, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x19, 0x02, 0x12, 0x03, 0x23,
    0x2b, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1a, 0x12, 0x03, 0x24, 0x08, 0x3d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x24, 0x08, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x1a, 0x02, 0x12, 0x03, 0x24, 0x39, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x1b, 0x12, 0x03, 0x25, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1b,
    0x01, 0x12, 0x03, 0x25, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1b, 0x02, 0x12,
    0x03, 0x25, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1c, 0x12, 0x03, 0x26, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1c, 0x01, 0x12, 0x03, 0x26, 0x08, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1c, 0x02, 0x12, 0x03, 0x26, 0x20, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x1d, 0x12, 0x03, 0x27, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x1d, 0x01, 0x12, 0x03, 0x27, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1d,
    0x02, 0x12, 0x03, 0x27, 0x24, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1e, 0x12, 0x03,
    0x28, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1e, 0x01, 0x12, 0x03, 0x28, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1e, 0x02, 0x12, 0x03, 0x28, 0x20, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1f, 0x12, 0x03, 0x29, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x1f, 0x01, 0x12, 0x03, 0x29, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x1f, 0x02, 0x12, 0x03, 0x29, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x20,
    0x12, 0x03, 0x2a, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x20, 0x01, 0x12, 0x03,
    0x2a, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x20, 0x02, 0x12, 0x03, 0x2a, 0x20,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x21, 0x12, 0x03, 0x2b, 0x08, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x21, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x21, 0x02, 0x12, 0x03, 0x2b, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x22, 0x12, 0x03, 0x2c, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x22, 0x01,
    0x12, 0x03, 0x2c, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x22, 0x02, 0x12, 0x03,
    0x2c, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x23, 0x12, 0x03, 0x2d, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x23, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x23, 0x02, 0x12, 0x03, 0x2d, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x24, 0x12, 0x03, 0x2e, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x24, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x24, 0x02,
    0x12, 0x03, 0x2e, 0x29, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x25, 0x12, 0x03, 0x2f,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x25, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x25, 0x02, 0x12, 0x03, 0x2f, 0x23, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x26, 0x12, 0x03, 0x30, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x26, 0x01, 0x12, 0x03, 0x30, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x26, 0x02, 0x12, 0x03, 0x30, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x27, 0x12,
    0x03, 0x31, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x27, 0x01, 0x12, 0x03, 0x31,
    0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x27, 0x02, 0x12, 0x03, 0x31, 0x29, 0x2c,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x28, 0x12, 0x03, 0x32, 0x08, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x28, 0x01, 0x12, 0x03, 0x32, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x28, 0x02, 0x12, 0x03, 0x32, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x29, 0x12, 0x03, 0x33, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x29, 0x01, 0x12,
    0x03, 0x33, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x29, 0x02, 0x12, 0x03, 0x33,
    0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x2a, 0x12, 0x03, 0x34, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x2a, 0x01, 0x12, 0x03, 0x34, 0x08, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x2a, 0x02, 0x12, 0x03, 0x34, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x2b, 0x12, 0x03, 0x35, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x2b,
    0x01, 0x12, 0x03, 0x35, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x2b, 0x02, 0x12,
    0x03, 0x35, 0x22, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x2c, 0x12, 0x03, 0x36, 0x08,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x2c, 0x01, 0x12, 0x03, 0x36, 0x08, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x2c, 0x02, 0x12, 0x03, 0x36, 0x22, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x2d, 0x12, 0x03, 0x37, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x2d, 0x01, 0x12, 0x03, 0x37, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x2d,
    0x02, 0x12, 0x03, 0x37, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x2e, 0x12, 0x03,
    0x38, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x2e, 0x01, 0x12, 0x03, 0x38, 0x08,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x2e, 0x02, 0x12, 0x03, 0x38, 0x1c, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x2f, 0x12, 0x03, 0x39, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x2f, 0x01, 0x12, 0x03, 0x39, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x2f, 0x02, 0x12, 0x03, 0x39, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x30,
    0x12, 0x03, 0x3a, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x30, 0x01, 0x12, 0x03,
    0x3a, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x30, 0x02, 0x12, 0x03, 0x3a, 0x1a,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x31, 0x12, 0x03, 0x3b, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x31, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x31, 0x02, 0x12, 0x03, 0x3b, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x32, 0x12, 0x03, 0x3c, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x32, 0x01,
    0x12, 0x03, 0x3c, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x32, 0x02, 0x12, 0x03,
    0x3c, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x33, 0x12, 0x03, 0x3d, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x33, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x33, 0x02, 0x12, 0x03, 0x3d, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x34, 0x12, 0x03, 0x3e, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x34, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x34, 0x02,
    0x12, 0x03, 0x3e, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x35, 0x12, 0x03, 0x3f,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x35, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x35, 0x02, 0x12, 0x03, 0x3f, 0x23, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x36, 0x12, 0x03, 0x40, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x36, 0x01, 0x12, 0x03, 0x40, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x36, 0x02, 0x12, 0x03, 0x40, 0x21, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x37, 0x12,
    0x03, 0x41, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x37, 0x01, 0x12, 0x03, 0x41,
    0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x37, 0x02, 0x12, 0x03, 0x41, 0x25, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x38, 0x12, 0x03, 0x42, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x38, 0x01, 0x12, 0x03, 0x42, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x38, 0x02, 0x12, 0x03, 0x42, 0x2b, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x39, 0x12, 0x03, 0x43, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x39, 0x01, 0x12,
    0x03, 0x43, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x39, 0x02, 0x12, 0x03, 0x43,
    0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x3a, 0x12, 0x03, 0x44, 0x08, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x3a, 0x01, 0x12, 0x03, 0x44, 0x08, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x3a, 0x02, 0x12, 0x03, 0x44, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x3b, 0x12, 0x03, 0x45, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x3b,
    0x01, 0x12, 0x03, 0x45, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x3b, 0x02, 0x12,
    0x03, 0x45, 0x1b, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x48, 0x00, 0x4a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x48, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x49, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x49, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x49, 0x11, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x49, 0x27, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x37,
    0x38, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x4c, 0x00, 0x4e, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x4d, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x4d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4d,
    0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d, 0x24, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x31, 0x32, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x50, 0x00, 0x52, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x50, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x51, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x51, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x51, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x18, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x51, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x03, 0x12, 0x04, 0x54, 0x00, 0x57, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x54, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x55, 0x08, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x55, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x55, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x55, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x56, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x56, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x56, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x56, 0x18, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x56, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x59, 0x00, 0x5c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x59, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x5a,
    0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5a, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5a, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5a, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x5b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x5b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5b, 0x18,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5b, 0x2a, 0x2b, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5e, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x5e, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12,
    0x03, 0x5f, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5f, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x18, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5f, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x62, 0x00, 0x64, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x62, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x63, 0x08,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x63, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x66, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x66, 0x08,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x67, 0x08, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x67, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x67, 0x11, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x22, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x67, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x6a, 0x00,
    0x6c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x6b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x6b, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x6b, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x6e, 0x00, 0x7d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x04, 0x00, 0x12, 0x04, 0x6f, 0x08, 0x74, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x6f, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x70, 0x10, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x70, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x70, 0x1a, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x71, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x71, 0x10, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x71, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x72, 0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x72, 0x10, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x72, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x73, 0x10, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x73, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x73, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x04, 0x01, 0x12, 0x04, 0x76, 0x08, 0x79,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x04, 0x01, 0x01, 0x12, 0x03, 0x76, 0x0d, 0x16, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x77, 0x10, 0x1b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x10, 0x16, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x77, 0x19, 0x1a, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x78, 0x10, 0x1d, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x78, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x78, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x7b, 0x08, 0x57, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x7b, 0x11, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x7b, 0x3a, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7b,
    0x41, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x08, 0x12, 0x03, 0x7b, 0x43, 0x56,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x07, 0x12, 0x03, 0x7b, 0x4e, 0x55, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x5f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x7c, 0x11, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x7c, 0x3e, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x7c, 0x4a, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x08, 0x12, 0x03, 0x7c, 0x4c,
    0x5e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x07, 0x12, 0x03, 0x7c, 0x57, 0x5d, 0x0a,
    0x0b, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x05, 0x7f, 0x00, 0x81, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0a, 0x01, 0x12, 0x03, 0x7f, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00,
    0x12, 0x04, 0x80, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x80, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x80, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80,
    0x01, 0x16, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x01,
    0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x83, 0x01, 0x00, 0x85, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x83, 0x01, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x84, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x84, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x84, 0x01, 0x16, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x84, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x87,
    0x01, 0x00, 0x89, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x87, 0x01,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x88, 0x01, 0x08, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x88, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x04, 0x88, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x06, 0x8b, 0x01, 0x00, 0x8c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0x8e,
    0x01, 0x00, 0x90, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0x8e, 0x01,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0f, 0x12, 0x06, 0x92, 0x01, 0x00, 0x96, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f,
    0x01, 0x12, 0x04, 0x92, 0x01, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12,
    0x04, 0x93, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x93, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x93,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93, 0x01,
    0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x01, 0x23,
    0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0x94, 0x01, 0x08, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0x94, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x02, 0x12, 0x04, 0x95, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x95, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02,
    0x05, 0x12, 0x04, 0x95, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x95, 0x01, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x95, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x98, 0x01, 0x00,
    0x9a, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x98, 0x01, 0x08, 0x22,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x99, 0x01, 0x08, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0x99, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x99, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0x99, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11,
    0x12, 0x06, 0x9c, 0x01, 0x00, 0x9e, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12,
    0x04, 0x9c, 0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0x9d,
    0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x18, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x24, 0x25, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xa0, 0x01, 0x00, 0xa2, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12,
    0x02, 0x00, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xa1, 0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xa1, 0x01, 0x24, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xa1, 0x01, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xa4, 0x01, 0x00, 0xa5,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xa7, 0x01, 0x00, 0xaa, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x00, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x52, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xa8, 0x01, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xa8, 0x01, 0x27, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xa8, 0x01, 0x36, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x08, 0x12, 0x04, 0xa8,
    0x01, 0x38, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x07, 0x12, 0x04, 0xa8, 0x01,
    0x43, 0x50, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x18, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x15, 0x12, 0x06, 0xac, 0x01, 0x00, 0xae, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15,
    0x01, 0x12, 0x04, 0xac, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12,
    0x04, 0xad, 0x01, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xad, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xad,
    0x01, 0x11, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xad, 0x01,
    0x28, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xad, 0x01, 0x34,
    0x35, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xb0, 0x01, 0x00, 0xb3, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xb1, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xb1, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04,
    0xb2, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb2,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb2, 0x01,
    0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x16,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x27, 0x28,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xb5, 0x01, 0x00, 0xb8, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x00, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xb6, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xb6, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xb6, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xb7,
    0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb7, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x16, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x24, 0x25, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xba, 0x01, 0x00, 0xbc, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x00, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xbb, 0x01, 0x11, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xbb, 0x01, 0x33, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xbb, 0x01, 0x3b, 0x3c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xbe, 0x01, 0x00, 0xc0,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x21, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12,
    0x06, 0xc2, 0x01, 0x00, 0xc5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04,
    0xc2, 0x01, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xc3, 0x01,
    0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc3, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc3, 0x01, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x17, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06,
    0xc7, 0x01, 0x00, 0xc8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xc7,
    0x01, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xca, 0x01, 0x00, 0xcd, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xca, 0x01, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcb, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xcb, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01,
    0x12, 0x04, 0xcc, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xcc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xcc, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc,
    0x01, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc, 0x01,
    0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xcf, 0x01, 0x00, 0xd1, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xd0, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x16, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xd0, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xd3,
    0x01, 0x00, 0xd5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xd3, 0x01,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd4, 0x01, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1f, 0x12, 0x06, 0xd7, 0x01, 0x00, 0xd9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f,
    0x01, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12,
    0x04, 0xd8, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xd8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd8,
    0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x01,
    0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0xdb, 0x01, 0x00, 0xdd, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xdc, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xdc, 0x01, 0x16, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xdc, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xdf, 0x01,
    0x00, 0xe3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x21, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x21, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x21, 0x02, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xe1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xe1, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xe1, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x02, 0x12, 0x04, 0xe2,
    0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe2, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x18, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x24, 0x25, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0xe5, 0x01, 0x00, 0xe7, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22,
    0x02, 0x00, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xe6, 0x01, 0x11, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xe6, 0x01, 0x2f, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xe6, 0x01, 0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06, 0xe9, 0x01, 0x00, 0xeb,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xe9, 0x01, 0x08, 0x2d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x00, 0x12, 0x04, 0xea, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x00, 0x04, 0x12, 0x04, 0xea, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x00, 0x05, 0x12, 0x04, 0xea, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x00, 0x01, 0x12, 0x04, 0xea, 0x01, 0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xea, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12,
    0x06, 0xed, 0x01, 0x00, 0xef, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04,
    0xed, 0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x00, 0x12, 0x04, 0xee, 0x01,
    0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x04, 0x12, 0x04, 0xee, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x05, 0x12, 0x04, 0xee, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01, 0x12, 0x04, 0xee, 0x01, 0x18, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12, 0x04, 0xee, 0x01, 0x2a, 0x2b, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0xf1, 0x01, 0x00, 0xf3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x25, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02,
    0x00, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xf2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xf2, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xf2, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf2,
    0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0xf5, 0x01, 0x00, 0xfc, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf6, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xf6, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x01,
    0x12, 0x04, 0xf7, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xf7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xf7, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf7,
    0x01, 0x17, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf7, 0x01,
    0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x02, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf8, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x26, 0x02, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xf9, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xf9, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xf9, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x04, 0x12, 0x04,
    0xfa, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x04, 0x04, 0x12, 0x04, 0xfa,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x04, 0x05, 0x12, 0x04, 0xfa, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x04, 0x01, 0x12, 0x04, 0xfa, 0x01, 0x18,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x04, 0x03, 0x12, 0x04, 0xfa, 0x01, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x05, 0x12, 0x04, 0xfb, 0x01, 0x08, 0x31, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x05, 0x04, 0x12, 0x04, 0xfb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x05, 0x05, 0x12, 0x04, 0xfb, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x05, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x16, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x05, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27,
    0x12, 0x06, 0xfe, 0x01, 0x00, 0x82, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12,
    0x04, 0xfe, 0x01, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0xff,
    0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0xff, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x05, 0x12, 0x04, 0xff, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0xff, 0x01, 0x18, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0xff, 0x01, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01, 0x12, 0x04, 0x80, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x04, 0x80, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x01, 0x05, 0x12, 0x04, 0x80, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02,
    0x02, 0x12, 0x04, 0x81, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x81, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x05, 0x12,
    0x04, 0x81, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x81, 0x02, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x03, 0x12, 0x04, 0x81,
    0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0x84, 0x02, 0x00, 0x88, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0x84, 0x02, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04, 0x85, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x04, 0x85, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x00, 0x05, 0x12, 0x04, 0x85, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x85, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x85, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x01,
    0x12, 0x04, 0x86, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x86, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x05, 0x12, 0x04,
    0x86, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x01, 0x12, 0x04, 0x86,
    0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x03, 0x12, 0x04, 0x86, 0x02,
    0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x02, 0x12, 0x04, 0x87, 0x02, 0x08, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x04, 0x12, 0x04, 0x87, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x05, 0x12, 0x04, 0x87, 0x02, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x01, 0x12, 0x04, 0x87, 0x02, 0x18, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x02, 0x03, 0x12, 0x04, 0x87, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x29, 0x12, 0x06, 0x8a, 0x02, 0x00, 0x8c, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29,
    0x01, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00, 0x12,
    0x04, 0x8b, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x8b, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8b,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x02,
    0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x27,
    0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2a, 0x12, 0x06, 0x8e, 0x02, 0x00, 0x90, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2a, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x02, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8f, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8f, 0x02, 0x18, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8f, 0x02, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06, 0x92, 0x02,
    0x00, 0x94, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04, 0x92, 0x02, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x00, 0x12, 0x04, 0x93, 0x02, 0x08, 0x2f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x93, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x06, 0x12, 0x04, 0x93, 0x02, 0x11, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93, 0x02, 0x24, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x02, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x2c, 0x12, 0x06, 0x96, 0x02, 0x00, 0x99, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01,
    0x12, 0x04, 0x96, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x00, 0x12, 0x04,
    0x97, 0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x97,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x05, 0x12, 0x04, 0x97, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x97, 0x02, 0x18,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x97, 0x02, 0x2a, 0x2b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x01, 0x12, 0x04, 0x98, 0x02, 0x08, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x98, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2c, 0x02, 0x01, 0x05, 0x12, 0x04, 0x98, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x98, 0x02, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x98, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2d,
    0x12, 0x06, 0x9b, 0x02, 0x00, 0xa2, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2d, 0x01, 0x12,
    0x04, 0x9b, 0x02, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x00, 0x12, 0x04, 0x9c,
    0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9c, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9c, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9c, 0x02, 0x18, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x02, 0x2a, 0x2b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2d, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9d, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02,
    0x02, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x9e, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02, 0x05, 0x12,
    0x04, 0x9e, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x9e, 0x02, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9e,
    0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x02, 0x08,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x04, 0x12, 0x04, 0x9f, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x05, 0x12, 0x04, 0x9f, 0x02, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9f, 0x02, 0x18, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9f, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x2d, 0x02, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x04, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d,
    0x02, 0x04, 0x05, 0x12, 0x04, 0xa0, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02,
    0x04, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x04,
    0x03, 0x12, 0x04, 0xa0, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x05, 0x12,
    0x04, 0xa1, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x05, 0x04, 0x12, 0x04,
    0xa1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x05, 0x05, 0x12, 0x04, 0xa1,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x05, 0x01, 0x12, 0x04, 0xa1, 0x02,
    0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x05, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x27,
    0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2e, 0x12, 0x06, 0xa4, 0x02, 0x00, 0xa7, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xa5, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa5, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xa5, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa5, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa5, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x01, 0x12, 0x04,
    0xa6, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa6,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa6, 0x02,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x17,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa6, 0x02, 0x22, 0x23,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2f, 0x12, 0x06, 0xa9, 0x02, 0x00, 0xaa, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x2f, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x30, 0x12, 0x06, 0xac, 0x02, 0x00, 0xb4, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x30, 0x01,
    0x12, 0x04, 0xac, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x00, 0x12, 0x04,
    0xad, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x04, 0x12, 0x04, 0xad,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x05, 0x12, 0x04, 0xad, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x01, 0x12, 0x04, 0xad, 0x02, 0x18,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x03, 0x12, 0x04, 0xad, 0x02, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x01, 0x12, 0x04, 0xae, 0x02, 0x08, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x30, 0x02, 0x01, 0x04, 0x12, 0x04, 0xae, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x30, 0x02, 0x01, 0x05, 0x12, 0x04, 0xae, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x30, 0x02, 0x01, 0x01, 0x12, 0x04, 0xae, 0x02, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x30, 0x02, 0x01, 0x03, 0x12, 0x04, 0xae, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30,
    0x02, 0x02, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xaf, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xaf, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xaf, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x03, 0x12, 0x04, 0xb0, 0x02,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x03, 0x04, 0x12, 0x04, 0xb0, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x03, 0x05, 0x12, 0x04, 0xb0, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb0, 0x02, 0x18, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb0, 0x02, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x30, 0x02, 0x04, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x30, 0x02, 0x04, 0x04, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x30, 0x02, 0x04, 0x05, 0x12, 0x04, 0xb1, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xb1, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02,
    0x04, 0x03, 0x12, 0x04, 0xb1, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x05,
    0x12, 0x04, 0xb2, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xb2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x05, 0x05, 0x12, 0x04,
    0xb2, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb2,
    0x02, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x05, 0x03, 0x12, 0x04, 0xb2, 0x02,
    0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x06, 0x12, 0x04, 0xb3, 0x02, 0x08, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x06, 0x04, 0x12, 0x04, 0xb3, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x06, 0x05, 0x12, 0x04, 0xb3, 0x02, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x30, 0x02, 0x06, 0x01, 0x12, 0x04, 0xb3, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x30, 0x02, 0x06, 0x03, 0x12, 0x04, 0xb3, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x31, 0x12, 0x06, 0xb6, 0x02, 0x00, 0xb9, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x31,
    0x01, 0x12, 0x04, 0xb6, 0x02, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x00, 0x12,
    0x04, 0xb7, 0x02, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xb7, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb7,
    0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x02,
    0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x1e,
    0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb8, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x31, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb8, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x31, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb8, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x32, 0x12, 0x06, 0xbb, 0x02, 0x00, 0xbe, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x32, 0x01,
    0x12, 0x04, 0xbb, 0x02, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x00, 0x12, 0x04,
    0xbc, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbc,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbc, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x02, 0x18,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x02, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x08, 0x2c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x32, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x32, 0x02, 0x01, 0x05, 0x12, 0x04, 0xbd, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x32, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x32, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x33,
    0x12, 0x06, 0xc0, 0x02, 0x00, 0xc3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x33, 0x01, 0x12,
    0x04, 0xc0, 0x02, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x00, 0x12, 0x04, 0xc1,
    0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc1, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc1, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc1, 0x02, 0x18, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc1, 0x02, 0x2a, 0x2b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x34, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x33, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x33, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc2, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x33, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x18, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xc2, 0x02, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x34, 0x12,
    0x06, 0xc5, 0x02, 0x00, 0xc9, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x34, 0x01, 0x12, 0x04,
    0xc5, 0x02, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x02,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc6, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x02, 0x18, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc6, 0x02, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x34, 0x02, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x34, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc7, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x34, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc7, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc7, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x34, 0x02, 0x02,
    0x12, 0x04, 0xc8, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xc8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xc8, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc8,
    0x02, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x34, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc8, 0x02,
    0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x35, 0x12, 0x06, 0xcb, 0x02, 0x00, 0xcf, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x35, 0x01, 0x12, 0x04, 0xcb, 0x02, 0x08, 0x1f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x35, 0x02, 0x00, 0x12, 0x04, 0xcc, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x35, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcc, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xcc, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xcc, 0x02, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xcc, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x01, 0x12,
    0x04, 0xcd, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xcd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcd,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcd, 0x02,
    0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcd, 0x02, 0x23,
    0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x02, 0x12, 0x04, 0xce, 0x02, 0x08, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x02, 0x04, 0x12, 0x04, 0xce, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x35, 0x02, 0x02, 0x05, 0x12, 0x04, 0xce, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x35, 0x02, 0x02, 0x01, 0x12, 0x04, 0xce, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x35, 0x02, 0x02, 0x03, 0x12, 0x04, 0xce, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x36, 0x12, 0x06, 0xd1, 0x02, 0x00, 0xd3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x36, 0x01,
    0x12, 0x04, 0xd1, 0x02, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x36, 0x02, 0x00, 0x12, 0x04,
    0xd2, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd2,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd2, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd2, 0x02, 0x18,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd2, 0x02, 0x27, 0x28,
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
