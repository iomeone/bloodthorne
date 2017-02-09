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
pub struct CMsgVDebugGameSessionIDEvent {
    // message fields
    clientid: ::std::option::Option<i32>,
    gamesessionid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgVDebugGameSessionIDEvent {}

impl CMsgVDebugGameSessionIDEvent {
    pub fn new() -> CMsgVDebugGameSessionIDEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgVDebugGameSessionIDEvent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgVDebugGameSessionIDEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgVDebugGameSessionIDEvent,
        };
        unsafe {
            instance.get(CMsgVDebugGameSessionIDEvent::new)
        }
    }

    // optional int32 clientid = 1;

    pub fn clear_clientid(&mut self) {
        self.clientid = ::std::option::Option::None;
    }

    pub fn has_clientid(&self) -> bool {
        self.clientid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientid(&mut self, v: i32) {
        self.clientid = ::std::option::Option::Some(v);
    }

    pub fn get_clientid(&self) -> i32 {
        self.clientid.unwrap_or(0)
    }

    fn get_clientid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.clientid
    }

    fn mut_clientid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.clientid
    }

    // optional string gamesessionid = 2;

    pub fn clear_gamesessionid(&mut self) {
        self.gamesessionid.clear();
    }

    pub fn has_gamesessionid(&self) -> bool {
        self.gamesessionid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gamesessionid(&mut self, v: ::std::string::String) {
        self.gamesessionid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gamesessionid(&mut self) -> &mut ::std::string::String {
        if self.gamesessionid.is_none() {
            self.gamesessionid.set_default();
        };
        self.gamesessionid.as_mut().unwrap()
    }

    // Take field
    pub fn take_gamesessionid(&mut self) -> ::std::string::String {
        self.gamesessionid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gamesessionid(&self) -> &str {
        match self.gamesessionid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_gamesessionid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.gamesessionid
    }

    fn mut_gamesessionid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.gamesessionid
    }
}

impl ::protobuf::Message for CMsgVDebugGameSessionIDEvent {
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
                    self.clientid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gamesessionid)?;
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
        if let Some(v) = self.clientid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.gamesessionid.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.clientid {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.gamesessionid.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgVDebugGameSessionIDEvent {
    fn new() -> CMsgVDebugGameSessionIDEvent {
        CMsgVDebugGameSessionIDEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgVDebugGameSessionIDEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "clientid",
                    CMsgVDebugGameSessionIDEvent::get_clientid_for_reflect,
                    CMsgVDebugGameSessionIDEvent::mut_clientid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gamesessionid",
                    CMsgVDebugGameSessionIDEvent::get_gamesessionid_for_reflect,
                    CMsgVDebugGameSessionIDEvent::mut_gamesessionid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgVDebugGameSessionIDEvent>(
                    "CMsgVDebugGameSessionIDEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgVDebugGameSessionIDEvent {
    fn clear(&mut self) {
        self.clear_clientid();
        self.clear_gamesessionid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgVDebugGameSessionIDEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgVDebugGameSessionIDEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPlaceDecalEvent {
    // message fields
    position: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    normal: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    saxis: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    decalmaterialindex: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    color: ::std::option::Option<u32>,
    width: ::std::option::Option<f32>,
    height: ::std::option::Option<f32>,
    depth: ::std::option::Option<f32>,
    entityhandleindex: ::std::option::Option<u32>,
    skeletoninstancehash: ::std::option::Option<u32>,
    boneindex: ::std::option::Option<i32>,
    translucenthit: ::std::option::Option<bool>,
    is_adjacent: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPlaceDecalEvent {}

impl CMsgPlaceDecalEvent {
    pub fn new() -> CMsgPlaceDecalEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPlaceDecalEvent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPlaceDecalEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPlaceDecalEvent,
        };
        unsafe {
            instance.get(CMsgPlaceDecalEvent::new)
        }
    }

    // optional .dota.CMsgVector position = 1;

    pub fn clear_position(&mut self) {
        self.position.clear();
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.position = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_position(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.position.is_none() {
            self.position.set_default();
        };
        self.position.as_mut().unwrap()
    }

    // Take field
    pub fn take_position(&mut self) -> super::networkbasetypes::CMsgVector {
        self.position.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_position(&self) -> &super::networkbasetypes::CMsgVector {
        self.position.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_position_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.position
    }

    fn mut_position_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.position
    }

    // optional .dota.CMsgVector normal = 2;

    pub fn clear_normal(&mut self) {
        self.normal.clear();
    }

    pub fn has_normal(&self) -> bool {
        self.normal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normal(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.normal = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_normal(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.normal.is_none() {
            self.normal.set_default();
        };
        self.normal.as_mut().unwrap()
    }

    // Take field
    pub fn take_normal(&mut self) -> super::networkbasetypes::CMsgVector {
        self.normal.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_normal(&self) -> &super::networkbasetypes::CMsgVector {
        self.normal.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_normal_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.normal
    }

    fn mut_normal_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.normal
    }

    // optional .dota.CMsgVector saxis = 3;

    pub fn clear_saxis(&mut self) {
        self.saxis.clear();
    }

    pub fn has_saxis(&self) -> bool {
        self.saxis.is_some()
    }

    // Param is passed by value, moved
    pub fn set_saxis(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.saxis = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_saxis(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.saxis.is_none() {
            self.saxis.set_default();
        };
        self.saxis.as_mut().unwrap()
    }

    // Take field
    pub fn take_saxis(&mut self) -> super::networkbasetypes::CMsgVector {
        self.saxis.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_saxis(&self) -> &super::networkbasetypes::CMsgVector {
        self.saxis.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_saxis_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.saxis
    }

    fn mut_saxis_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.saxis
    }

    // optional uint32 decalmaterialindex = 4;

    pub fn clear_decalmaterialindex(&mut self) {
        self.decalmaterialindex = ::std::option::Option::None;
    }

    pub fn has_decalmaterialindex(&self) -> bool {
        self.decalmaterialindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decalmaterialindex(&mut self, v: u32) {
        self.decalmaterialindex = ::std::option::Option::Some(v);
    }

    pub fn get_decalmaterialindex(&self) -> u32 {
        self.decalmaterialindex.unwrap_or(0)
    }

    fn get_decalmaterialindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.decalmaterialindex
    }

    fn mut_decalmaterialindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.decalmaterialindex
    }

    // optional uint32 flags = 5;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: u32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> u32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flags
    }

    // optional fixed32 color = 6;

    pub fn clear_color(&mut self) {
        self.color = ::std::option::Option::None;
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: u32) {
        self.color = ::std::option::Option::Some(v);
    }

    pub fn get_color(&self) -> u32 {
        self.color.unwrap_or(0)
    }

    fn get_color_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.color
    }

    fn mut_color_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.color
    }

    // optional float width = 7;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: f32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> f32 {
        self.width.unwrap_or(0.)
    }

    fn get_width_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.width
    }

    fn mut_width_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.width
    }

    // optional float height = 8;

    pub fn clear_height(&mut self) {
        self.height = ::std::option::Option::None;
    }

    pub fn has_height(&self) -> bool {
        self.height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: f32) {
        self.height = ::std::option::Option::Some(v);
    }

    pub fn get_height(&self) -> f32 {
        self.height.unwrap_or(0.)
    }

    fn get_height_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.height
    }

    fn mut_height_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.height
    }

    // optional float depth = 9;

    pub fn clear_depth(&mut self) {
        self.depth = ::std::option::Option::None;
    }

    pub fn has_depth(&self) -> bool {
        self.depth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_depth(&mut self, v: f32) {
        self.depth = ::std::option::Option::Some(v);
    }

    pub fn get_depth(&self) -> f32 {
        self.depth.unwrap_or(0.)
    }

    fn get_depth_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.depth
    }

    fn mut_depth_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.depth
    }

    // optional uint32 entityhandleindex = 10;

    pub fn clear_entityhandleindex(&mut self) {
        self.entityhandleindex = ::std::option::Option::None;
    }

    pub fn has_entityhandleindex(&self) -> bool {
        self.entityhandleindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entityhandleindex(&mut self, v: u32) {
        self.entityhandleindex = ::std::option::Option::Some(v);
    }

    pub fn get_entityhandleindex(&self) -> u32 {
        self.entityhandleindex.unwrap_or(0)
    }

    fn get_entityhandleindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entityhandleindex
    }

    fn mut_entityhandleindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entityhandleindex
    }

    // optional fixed32 skeletoninstancehash = 11;

    pub fn clear_skeletoninstancehash(&mut self) {
        self.skeletoninstancehash = ::std::option::Option::None;
    }

    pub fn has_skeletoninstancehash(&self) -> bool {
        self.skeletoninstancehash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_skeletoninstancehash(&mut self, v: u32) {
        self.skeletoninstancehash = ::std::option::Option::Some(v);
    }

    pub fn get_skeletoninstancehash(&self) -> u32 {
        self.skeletoninstancehash.unwrap_or(0)
    }

    fn get_skeletoninstancehash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.skeletoninstancehash
    }

    fn mut_skeletoninstancehash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.skeletoninstancehash
    }

    // optional int32 boneindex = 12;

    pub fn clear_boneindex(&mut self) {
        self.boneindex = ::std::option::Option::None;
    }

    pub fn has_boneindex(&self) -> bool {
        self.boneindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_boneindex(&mut self, v: i32) {
        self.boneindex = ::std::option::Option::Some(v);
    }

    pub fn get_boneindex(&self) -> i32 {
        self.boneindex.unwrap_or(0)
    }

    fn get_boneindex_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.boneindex
    }

    fn mut_boneindex_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.boneindex
    }

    // optional bool translucenthit = 13;

    pub fn clear_translucenthit(&mut self) {
        self.translucenthit = ::std::option::Option::None;
    }

    pub fn has_translucenthit(&self) -> bool {
        self.translucenthit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_translucenthit(&mut self, v: bool) {
        self.translucenthit = ::std::option::Option::Some(v);
    }

    pub fn get_translucenthit(&self) -> bool {
        self.translucenthit.unwrap_or(false)
    }

    fn get_translucenthit_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.translucenthit
    }

    fn mut_translucenthit_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.translucenthit
    }

    // optional bool is_adjacent = 14;

    pub fn clear_is_adjacent(&mut self) {
        self.is_adjacent = ::std::option::Option::None;
    }

    pub fn has_is_adjacent(&self) -> bool {
        self.is_adjacent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_adjacent(&mut self, v: bool) {
        self.is_adjacent = ::std::option::Option::Some(v);
    }

    pub fn get_is_adjacent(&self) -> bool {
        self.is_adjacent.unwrap_or(false)
    }

    fn get_is_adjacent_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_adjacent
    }

    fn mut_is_adjacent_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_adjacent
    }
}

impl ::protobuf::Message for CMsgPlaceDecalEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.position)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.normal)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.saxis)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.decalmaterialindex = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.color = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.width = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.height = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.depth = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.entityhandleindex = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.skeletoninstancehash = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.boneindex = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.translucenthit = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_adjacent = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.position.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.normal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.saxis.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.decalmaterialindex {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.color {
            my_size += 5;
        };
        if let Some(v) = self.width {
            my_size += 5;
        };
        if let Some(v) = self.height {
            my_size += 5;
        };
        if let Some(v) = self.depth {
            my_size += 5;
        };
        if let Some(v) = self.entityhandleindex {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.skeletoninstancehash {
            my_size += 5;
        };
        if let Some(v) = self.boneindex {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.translucenthit {
            my_size += 2;
        };
        if let Some(v) = self.is_adjacent {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.position.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.normal.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.saxis.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.decalmaterialindex {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.color {
            os.write_fixed32(6, v)?;
        };
        if let Some(v) = self.width {
            os.write_float(7, v)?;
        };
        if let Some(v) = self.height {
            os.write_float(8, v)?;
        };
        if let Some(v) = self.depth {
            os.write_float(9, v)?;
        };
        if let Some(v) = self.entityhandleindex {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.skeletoninstancehash {
            os.write_fixed32(11, v)?;
        };
        if let Some(v) = self.boneindex {
            os.write_int32(12, v)?;
        };
        if let Some(v) = self.translucenthit {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.is_adjacent {
            os.write_bool(14, v)?;
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

impl ::protobuf::MessageStatic for CMsgPlaceDecalEvent {
    fn new() -> CMsgPlaceDecalEvent {
        CMsgPlaceDecalEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPlaceDecalEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "position",
                    CMsgPlaceDecalEvent::get_position_for_reflect,
                    CMsgPlaceDecalEvent::mut_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "normal",
                    CMsgPlaceDecalEvent::get_normal_for_reflect,
                    CMsgPlaceDecalEvent::mut_normal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "saxis",
                    CMsgPlaceDecalEvent::get_saxis_for_reflect,
                    CMsgPlaceDecalEvent::mut_saxis_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "decalmaterialindex",
                    CMsgPlaceDecalEvent::get_decalmaterialindex_for_reflect,
                    CMsgPlaceDecalEvent::mut_decalmaterialindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgPlaceDecalEvent::get_flags_for_reflect,
                    CMsgPlaceDecalEvent::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "color",
                    CMsgPlaceDecalEvent::get_color_for_reflect,
                    CMsgPlaceDecalEvent::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "width",
                    CMsgPlaceDecalEvent::get_width_for_reflect,
                    CMsgPlaceDecalEvent::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "height",
                    CMsgPlaceDecalEvent::get_height_for_reflect,
                    CMsgPlaceDecalEvent::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "depth",
                    CMsgPlaceDecalEvent::get_depth_for_reflect,
                    CMsgPlaceDecalEvent::mut_depth_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "entityhandleindex",
                    CMsgPlaceDecalEvent::get_entityhandleindex_for_reflect,
                    CMsgPlaceDecalEvent::mut_entityhandleindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "skeletoninstancehash",
                    CMsgPlaceDecalEvent::get_skeletoninstancehash_for_reflect,
                    CMsgPlaceDecalEvent::mut_skeletoninstancehash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "boneindex",
                    CMsgPlaceDecalEvent::get_boneindex_for_reflect,
                    CMsgPlaceDecalEvent::mut_boneindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "translucenthit",
                    CMsgPlaceDecalEvent::get_translucenthit_for_reflect,
                    CMsgPlaceDecalEvent::mut_translucenthit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_adjacent",
                    CMsgPlaceDecalEvent::get_is_adjacent_for_reflect,
                    CMsgPlaceDecalEvent::mut_is_adjacent_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPlaceDecalEvent>(
                    "CMsgPlaceDecalEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPlaceDecalEvent {
    fn clear(&mut self) {
        self.clear_position();
        self.clear_normal();
        self.clear_saxis();
        self.clear_decalmaterialindex();
        self.clear_flags();
        self.clear_color();
        self.clear_width();
        self.clear_height();
        self.clear_depth();
        self.clear_entityhandleindex();
        self.clear_skeletoninstancehash();
        self.clear_boneindex();
        self.clear_translucenthit();
        self.clear_is_adjacent();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPlaceDecalEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPlaceDecalEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClearWorldDecalsEvent {
    // message fields
    flagstoclear: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClearWorldDecalsEvent {}

impl CMsgClearWorldDecalsEvent {
    pub fn new() -> CMsgClearWorldDecalsEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClearWorldDecalsEvent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClearWorldDecalsEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClearWorldDecalsEvent,
        };
        unsafe {
            instance.get(CMsgClearWorldDecalsEvent::new)
        }
    }

    // optional uint32 flagstoclear = 1;

    pub fn clear_flagstoclear(&mut self) {
        self.flagstoclear = ::std::option::Option::None;
    }

    pub fn has_flagstoclear(&self) -> bool {
        self.flagstoclear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flagstoclear(&mut self, v: u32) {
        self.flagstoclear = ::std::option::Option::Some(v);
    }

    pub fn get_flagstoclear(&self) -> u32 {
        self.flagstoclear.unwrap_or(0)
    }

    fn get_flagstoclear_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flagstoclear
    }

    fn mut_flagstoclear_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flagstoclear
    }
}

impl ::protobuf::Message for CMsgClearWorldDecalsEvent {
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
                    self.flagstoclear = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.flagstoclear {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.flagstoclear {
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

impl ::protobuf::MessageStatic for CMsgClearWorldDecalsEvent {
    fn new() -> CMsgClearWorldDecalsEvent {
        CMsgClearWorldDecalsEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClearWorldDecalsEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flagstoclear",
                    CMsgClearWorldDecalsEvent::get_flagstoclear_for_reflect,
                    CMsgClearWorldDecalsEvent::mut_flagstoclear_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClearWorldDecalsEvent>(
                    "CMsgClearWorldDecalsEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClearWorldDecalsEvent {
    fn clear(&mut self) {
        self.clear_flagstoclear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClearWorldDecalsEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClearWorldDecalsEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClearEntityDecalsEvent {
    // message fields
    flagstoclear: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClearEntityDecalsEvent {}

impl CMsgClearEntityDecalsEvent {
    pub fn new() -> CMsgClearEntityDecalsEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClearEntityDecalsEvent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClearEntityDecalsEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClearEntityDecalsEvent,
        };
        unsafe {
            instance.get(CMsgClearEntityDecalsEvent::new)
        }
    }

    // optional uint32 flagstoclear = 1;

    pub fn clear_flagstoclear(&mut self) {
        self.flagstoclear = ::std::option::Option::None;
    }

    pub fn has_flagstoclear(&self) -> bool {
        self.flagstoclear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flagstoclear(&mut self, v: u32) {
        self.flagstoclear = ::std::option::Option::Some(v);
    }

    pub fn get_flagstoclear(&self) -> u32 {
        self.flagstoclear.unwrap_or(0)
    }

    fn get_flagstoclear_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flagstoclear
    }

    fn mut_flagstoclear_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flagstoclear
    }
}

impl ::protobuf::Message for CMsgClearEntityDecalsEvent {
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
                    self.flagstoclear = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.flagstoclear {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.flagstoclear {
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

impl ::protobuf::MessageStatic for CMsgClearEntityDecalsEvent {
    fn new() -> CMsgClearEntityDecalsEvent {
        CMsgClearEntityDecalsEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClearEntityDecalsEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flagstoclear",
                    CMsgClearEntityDecalsEvent::get_flagstoclear_for_reflect,
                    CMsgClearEntityDecalsEvent::mut_flagstoclear_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClearEntityDecalsEvent>(
                    "CMsgClearEntityDecalsEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClearEntityDecalsEvent {
    fn clear(&mut self) {
        self.clear_flagstoclear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClearEntityDecalsEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClearEntityDecalsEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClearDecalsForSkeletonInstanceEvent {
    // message fields
    flagstoclear: ::std::option::Option<u32>,
    entityhandleindex: ::std::option::Option<u32>,
    skeletoninstancehash: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClearDecalsForSkeletonInstanceEvent {}

impl CMsgClearDecalsForSkeletonInstanceEvent {
    pub fn new() -> CMsgClearDecalsForSkeletonInstanceEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClearDecalsForSkeletonInstanceEvent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClearDecalsForSkeletonInstanceEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClearDecalsForSkeletonInstanceEvent,
        };
        unsafe {
            instance.get(CMsgClearDecalsForSkeletonInstanceEvent::new)
        }
    }

    // optional uint32 flagstoclear = 1;

    pub fn clear_flagstoclear(&mut self) {
        self.flagstoclear = ::std::option::Option::None;
    }

    pub fn has_flagstoclear(&self) -> bool {
        self.flagstoclear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flagstoclear(&mut self, v: u32) {
        self.flagstoclear = ::std::option::Option::Some(v);
    }

    pub fn get_flagstoclear(&self) -> u32 {
        self.flagstoclear.unwrap_or(0)
    }

    fn get_flagstoclear_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flagstoclear
    }

    fn mut_flagstoclear_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flagstoclear
    }

    // optional uint32 entityhandleindex = 2;

    pub fn clear_entityhandleindex(&mut self) {
        self.entityhandleindex = ::std::option::Option::None;
    }

    pub fn has_entityhandleindex(&self) -> bool {
        self.entityhandleindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entityhandleindex(&mut self, v: u32) {
        self.entityhandleindex = ::std::option::Option::Some(v);
    }

    pub fn get_entityhandleindex(&self) -> u32 {
        self.entityhandleindex.unwrap_or(0)
    }

    fn get_entityhandleindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entityhandleindex
    }

    fn mut_entityhandleindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entityhandleindex
    }

    // optional uint32 skeletoninstancehash = 3;

    pub fn clear_skeletoninstancehash(&mut self) {
        self.skeletoninstancehash = ::std::option::Option::None;
    }

    pub fn has_skeletoninstancehash(&self) -> bool {
        self.skeletoninstancehash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_skeletoninstancehash(&mut self, v: u32) {
        self.skeletoninstancehash = ::std::option::Option::Some(v);
    }

    pub fn get_skeletoninstancehash(&self) -> u32 {
        self.skeletoninstancehash.unwrap_or(0)
    }

    fn get_skeletoninstancehash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.skeletoninstancehash
    }

    fn mut_skeletoninstancehash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.skeletoninstancehash
    }
}

impl ::protobuf::Message for CMsgClearDecalsForSkeletonInstanceEvent {
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
                    self.flagstoclear = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.entityhandleindex = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.skeletoninstancehash = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.flagstoclear {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entityhandleindex {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.skeletoninstancehash {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.flagstoclear {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.entityhandleindex {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.skeletoninstancehash {
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

impl ::protobuf::MessageStatic for CMsgClearDecalsForSkeletonInstanceEvent {
    fn new() -> CMsgClearDecalsForSkeletonInstanceEvent {
        CMsgClearDecalsForSkeletonInstanceEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClearDecalsForSkeletonInstanceEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flagstoclear",
                    CMsgClearDecalsForSkeletonInstanceEvent::get_flagstoclear_for_reflect,
                    CMsgClearDecalsForSkeletonInstanceEvent::mut_flagstoclear_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "entityhandleindex",
                    CMsgClearDecalsForSkeletonInstanceEvent::get_entityhandleindex_for_reflect,
                    CMsgClearDecalsForSkeletonInstanceEvent::mut_entityhandleindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "skeletoninstancehash",
                    CMsgClearDecalsForSkeletonInstanceEvent::get_skeletoninstancehash_for_reflect,
                    CMsgClearDecalsForSkeletonInstanceEvent::mut_skeletoninstancehash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClearDecalsForSkeletonInstanceEvent>(
                    "CMsgClearDecalsForSkeletonInstanceEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClearDecalsForSkeletonInstanceEvent {
    fn clear(&mut self) {
        self.clear_flagstoclear();
        self.clear_entityhandleindex();
        self.clear_skeletoninstancehash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClearDecalsForSkeletonInstanceEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClearDecalsForSkeletonInstanceEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSource1LegacyGameEventList {
    // message fields
    descriptors: ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_descriptor_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSource1LegacyGameEventList {}

impl CMsgSource1LegacyGameEventList {
    pub fn new() -> CMsgSource1LegacyGameEventList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSource1LegacyGameEventList {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSource1LegacyGameEventList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSource1LegacyGameEventList,
        };
        unsafe {
            instance.get(CMsgSource1LegacyGameEventList::new)
        }
    }

    // repeated .dota.CMsgSource1LegacyGameEventList.descriptor_t descriptors = 1;

    pub fn clear_descriptors(&mut self) {
        self.descriptors.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptors(&mut self, v: ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_descriptor_t>) {
        self.descriptors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_descriptors(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_descriptor_t> {
        &mut self.descriptors
    }

    // Take field
    pub fn take_descriptors(&mut self) -> ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_descriptor_t> {
        ::std::mem::replace(&mut self.descriptors, ::protobuf::RepeatedField::new())
    }

    pub fn get_descriptors(&self) -> &[CMsgSource1LegacyGameEventList_descriptor_t] {
        &self.descriptors
    }

    fn get_descriptors_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_descriptor_t> {
        &self.descriptors
    }

    fn mut_descriptors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_descriptor_t> {
        &mut self.descriptors
    }
}

impl ::protobuf::Message for CMsgSource1LegacyGameEventList {
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

impl ::protobuf::MessageStatic for CMsgSource1LegacyGameEventList {
    fn new() -> CMsgSource1LegacyGameEventList {
        CMsgSource1LegacyGameEventList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSource1LegacyGameEventList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSource1LegacyGameEventList_descriptor_t>>(
                    "descriptors",
                    CMsgSource1LegacyGameEventList::get_descriptors_for_reflect,
                    CMsgSource1LegacyGameEventList::mut_descriptors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSource1LegacyGameEventList>(
                    "CMsgSource1LegacyGameEventList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSource1LegacyGameEventList {
    fn clear(&mut self) {
        self.clear_descriptors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSource1LegacyGameEventList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSource1LegacyGameEventList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSource1LegacyGameEventList_key_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSource1LegacyGameEventList_key_t {}

impl CMsgSource1LegacyGameEventList_key_t {
    pub fn new() -> CMsgSource1LegacyGameEventList_key_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSource1LegacyGameEventList_key_t {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSource1LegacyGameEventList_key_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSource1LegacyGameEventList_key_t,
        };
        unsafe {
            instance.get(CMsgSource1LegacyGameEventList_key_t::new)
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

impl ::protobuf::Message for CMsgSource1LegacyGameEventList_key_t {
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

impl ::protobuf::MessageStatic for CMsgSource1LegacyGameEventList_key_t {
    fn new() -> CMsgSource1LegacyGameEventList_key_t {
        CMsgSource1LegacyGameEventList_key_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSource1LegacyGameEventList_key_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CMsgSource1LegacyGameEventList_key_t::get_field_type_for_reflect,
                    CMsgSource1LegacyGameEventList_key_t::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgSource1LegacyGameEventList_key_t::get_name_for_reflect,
                    CMsgSource1LegacyGameEventList_key_t::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSource1LegacyGameEventList_key_t>(
                    "CMsgSource1LegacyGameEventList_key_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSource1LegacyGameEventList_key_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSource1LegacyGameEventList_key_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSource1LegacyGameEventList_key_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSource1LegacyGameEventList_descriptor_t {
    // message fields
    eventid: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    keys: ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_key_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSource1LegacyGameEventList_descriptor_t {}

impl CMsgSource1LegacyGameEventList_descriptor_t {
    pub fn new() -> CMsgSource1LegacyGameEventList_descriptor_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSource1LegacyGameEventList_descriptor_t {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSource1LegacyGameEventList_descriptor_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSource1LegacyGameEventList_descriptor_t,
        };
        unsafe {
            instance.get(CMsgSource1LegacyGameEventList_descriptor_t::new)
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

    // repeated .dota.CMsgSource1LegacyGameEventList.key_t keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_key_t>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_key_t> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_key_t> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CMsgSource1LegacyGameEventList_key_t] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_key_t> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSource1LegacyGameEventList_key_t> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CMsgSource1LegacyGameEventList_descriptor_t {
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

impl ::protobuf::MessageStatic for CMsgSource1LegacyGameEventList_descriptor_t {
    fn new() -> CMsgSource1LegacyGameEventList_descriptor_t {
        CMsgSource1LegacyGameEventList_descriptor_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSource1LegacyGameEventList_descriptor_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eventid",
                    CMsgSource1LegacyGameEventList_descriptor_t::get_eventid_for_reflect,
                    CMsgSource1LegacyGameEventList_descriptor_t::mut_eventid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgSource1LegacyGameEventList_descriptor_t::get_name_for_reflect,
                    CMsgSource1LegacyGameEventList_descriptor_t::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSource1LegacyGameEventList_key_t>>(
                    "keys",
                    CMsgSource1LegacyGameEventList_descriptor_t::get_keys_for_reflect,
                    CMsgSource1LegacyGameEventList_descriptor_t::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSource1LegacyGameEventList_descriptor_t>(
                    "CMsgSource1LegacyGameEventList_descriptor_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSource1LegacyGameEventList_descriptor_t {
    fn clear(&mut self) {
        self.clear_eventid();
        self.clear_name();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSource1LegacyGameEventList_descriptor_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSource1LegacyGameEventList_descriptor_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSource1LegacyListenEvents {
    // message fields
    playerslot: ::std::option::Option<i32>,
    eventarraybits: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSource1LegacyListenEvents {}

impl CMsgSource1LegacyListenEvents {
    pub fn new() -> CMsgSource1LegacyListenEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSource1LegacyListenEvents {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSource1LegacyListenEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSource1LegacyListenEvents,
        };
        unsafe {
            instance.get(CMsgSource1LegacyListenEvents::new)
        }
    }

    // optional int32 playerslot = 1;

    pub fn clear_playerslot(&mut self) {
        self.playerslot = ::std::option::Option::None;
    }

    pub fn has_playerslot(&self) -> bool {
        self.playerslot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerslot(&mut self, v: i32) {
        self.playerslot = ::std::option::Option::Some(v);
    }

    pub fn get_playerslot(&self) -> i32 {
        self.playerslot.unwrap_or(0)
    }

    fn get_playerslot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.playerslot
    }

    fn mut_playerslot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.playerslot
    }

    // repeated uint32 eventarraybits = 2;

    pub fn clear_eventarraybits(&mut self) {
        self.eventarraybits.clear();
    }

    // Param is passed by value, moved
    pub fn set_eventarraybits(&mut self, v: ::std::vec::Vec<u32>) {
        self.eventarraybits = v;
    }

    // Mutable pointer to the field.
    pub fn mut_eventarraybits(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.eventarraybits
    }

    // Take field
    pub fn take_eventarraybits(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.eventarraybits, ::std::vec::Vec::new())
    }

    pub fn get_eventarraybits(&self) -> &[u32] {
        &self.eventarraybits
    }

    fn get_eventarraybits_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.eventarraybits
    }

    fn mut_eventarraybits_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.eventarraybits
    }
}

impl ::protobuf::Message for CMsgSource1LegacyListenEvents {
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
                    self.playerslot = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.eventarraybits)?;
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
        if let Some(v) = self.playerslot {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.eventarraybits {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.playerslot {
            os.write_int32(1, v)?;
        };
        for v in &self.eventarraybits {
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

impl ::protobuf::MessageStatic for CMsgSource1LegacyListenEvents {
    fn new() -> CMsgSource1LegacyListenEvents {
        CMsgSource1LegacyListenEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSource1LegacyListenEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "playerslot",
                    CMsgSource1LegacyListenEvents::get_playerslot_for_reflect,
                    CMsgSource1LegacyListenEvents::mut_playerslot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "eventarraybits",
                    CMsgSource1LegacyListenEvents::get_eventarraybits_for_reflect,
                    CMsgSource1LegacyListenEvents::mut_eventarraybits_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSource1LegacyListenEvents>(
                    "CMsgSource1LegacyListenEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSource1LegacyListenEvents {
    fn clear(&mut self) {
        self.clear_playerslot();
        self.clear_eventarraybits();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSource1LegacyListenEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSource1LegacyListenEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSource1LegacyGameEvent {
    // message fields
    event_name: ::protobuf::SingularField<::std::string::String>,
    eventid: ::std::option::Option<i32>,
    keys: ::protobuf::RepeatedField<CMsgSource1LegacyGameEvent_key_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSource1LegacyGameEvent {}

impl CMsgSource1LegacyGameEvent {
    pub fn new() -> CMsgSource1LegacyGameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSource1LegacyGameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSource1LegacyGameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSource1LegacyGameEvent,
        };
        unsafe {
            instance.get(CMsgSource1LegacyGameEvent::new)
        }
    }

    // optional string event_name = 1;

    pub fn clear_event_name(&mut self) {
        self.event_name.clear();
    }

    pub fn has_event_name(&self) -> bool {
        self.event_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_name(&mut self, v: ::std::string::String) {
        self.event_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_name(&mut self) -> &mut ::std::string::String {
        if self.event_name.is_none() {
            self.event_name.set_default();
        };
        self.event_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_name(&mut self) -> ::std::string::String {
        self.event_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_event_name(&self) -> &str {
        match self.event_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_event_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.event_name
    }

    fn mut_event_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.event_name
    }

    // optional int32 eventid = 2;

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

    // repeated .dota.CMsgSource1LegacyGameEvent.key_t keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CMsgSource1LegacyGameEvent_key_t>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSource1LegacyGameEvent_key_t> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CMsgSource1LegacyGameEvent_key_t> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CMsgSource1LegacyGameEvent_key_t] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSource1LegacyGameEvent_key_t> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSource1LegacyGameEvent_key_t> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CMsgSource1LegacyGameEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.event_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.eventid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.event_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.eventid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.event_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.eventid {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for CMsgSource1LegacyGameEvent {
    fn new() -> CMsgSource1LegacyGameEvent {
        CMsgSource1LegacyGameEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSource1LegacyGameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event_name",
                    CMsgSource1LegacyGameEvent::get_event_name_for_reflect,
                    CMsgSource1LegacyGameEvent::mut_event_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eventid",
                    CMsgSource1LegacyGameEvent::get_eventid_for_reflect,
                    CMsgSource1LegacyGameEvent::mut_eventid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSource1LegacyGameEvent_key_t>>(
                    "keys",
                    CMsgSource1LegacyGameEvent::get_keys_for_reflect,
                    CMsgSource1LegacyGameEvent::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSource1LegacyGameEvent>(
                    "CMsgSource1LegacyGameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSource1LegacyGameEvent {
    fn clear(&mut self) {
        self.clear_event_name();
        self.clear_eventid();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSource1LegacyGameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSource1LegacyGameEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSource1LegacyGameEvent_key_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    val_string: ::protobuf::SingularField<::std::string::String>,
    val_float: ::std::option::Option<f32>,
    val_long: ::std::option::Option<i32>,
    val_short: ::std::option::Option<i32>,
    val_byte: ::std::option::Option<i32>,
    val_bool: ::std::option::Option<bool>,
    val_uint64: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSource1LegacyGameEvent_key_t {}

impl CMsgSource1LegacyGameEvent_key_t {
    pub fn new() -> CMsgSource1LegacyGameEvent_key_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSource1LegacyGameEvent_key_t {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSource1LegacyGameEvent_key_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSource1LegacyGameEvent_key_t,
        };
        unsafe {
            instance.get(CMsgSource1LegacyGameEvent_key_t::new)
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

    // optional string val_string = 2;

    pub fn clear_val_string(&mut self) {
        self.val_string.clear();
    }

    pub fn has_val_string(&self) -> bool {
        self.val_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_string(&mut self, v: ::std::string::String) {
        self.val_string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val_string(&mut self) -> &mut ::std::string::String {
        if self.val_string.is_none() {
            self.val_string.set_default();
        };
        self.val_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_val_string(&mut self) -> ::std::string::String {
        self.val_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_val_string(&self) -> &str {
        match self.val_string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_val_string_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.val_string
    }

    fn mut_val_string_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.val_string
    }

    // optional float val_float = 3;

    pub fn clear_val_float(&mut self) {
        self.val_float = ::std::option::Option::None;
    }

    pub fn has_val_float(&self) -> bool {
        self.val_float.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_float(&mut self, v: f32) {
        self.val_float = ::std::option::Option::Some(v);
    }

    pub fn get_val_float(&self) -> f32 {
        self.val_float.unwrap_or(0.)
    }

    fn get_val_float_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.val_float
    }

    fn mut_val_float_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.val_float
    }

    // optional int32 val_long = 4;

    pub fn clear_val_long(&mut self) {
        self.val_long = ::std::option::Option::None;
    }

    pub fn has_val_long(&self) -> bool {
        self.val_long.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_long(&mut self, v: i32) {
        self.val_long = ::std::option::Option::Some(v);
    }

    pub fn get_val_long(&self) -> i32 {
        self.val_long.unwrap_or(0)
    }

    fn get_val_long_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.val_long
    }

    fn mut_val_long_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.val_long
    }

    // optional int32 val_short = 5;

    pub fn clear_val_short(&mut self) {
        self.val_short = ::std::option::Option::None;
    }

    pub fn has_val_short(&self) -> bool {
        self.val_short.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_short(&mut self, v: i32) {
        self.val_short = ::std::option::Option::Some(v);
    }

    pub fn get_val_short(&self) -> i32 {
        self.val_short.unwrap_or(0)
    }

    fn get_val_short_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.val_short
    }

    fn mut_val_short_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.val_short
    }

    // optional int32 val_byte = 6;

    pub fn clear_val_byte(&mut self) {
        self.val_byte = ::std::option::Option::None;
    }

    pub fn has_val_byte(&self) -> bool {
        self.val_byte.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_byte(&mut self, v: i32) {
        self.val_byte = ::std::option::Option::Some(v);
    }

    pub fn get_val_byte(&self) -> i32 {
        self.val_byte.unwrap_or(0)
    }

    fn get_val_byte_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.val_byte
    }

    fn mut_val_byte_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.val_byte
    }

    // optional bool val_bool = 7;

    pub fn clear_val_bool(&mut self) {
        self.val_bool = ::std::option::Option::None;
    }

    pub fn has_val_bool(&self) -> bool {
        self.val_bool.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_bool(&mut self, v: bool) {
        self.val_bool = ::std::option::Option::Some(v);
    }

    pub fn get_val_bool(&self) -> bool {
        self.val_bool.unwrap_or(false)
    }

    fn get_val_bool_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.val_bool
    }

    fn mut_val_bool_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.val_bool
    }

    // optional uint64 val_uint64 = 8;

    pub fn clear_val_uint64(&mut self) {
        self.val_uint64 = ::std::option::Option::None;
    }

    pub fn has_val_uint64(&self) -> bool {
        self.val_uint64.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_uint64(&mut self, v: u64) {
        self.val_uint64 = ::std::option::Option::Some(v);
    }

    pub fn get_val_uint64(&self) -> u64 {
        self.val_uint64.unwrap_or(0)
    }

    fn get_val_uint64_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.val_uint64
    }

    fn mut_val_uint64_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.val_uint64
    }
}

impl ::protobuf::Message for CMsgSource1LegacyGameEvent_key_t {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.val_string)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.val_float = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.val_long = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.val_short = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.val_byte = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.val_bool = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.val_uint64 = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.val_string.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.val_float {
            my_size += 5;
        };
        if let Some(v) = self.val_long {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.val_short {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.val_byte {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.val_bool {
            my_size += 2;
        };
        if let Some(v) = self.val_uint64 {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.val_string.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.val_float {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.val_long {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.val_short {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.val_byte {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.val_bool {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.val_uint64 {
            os.write_uint64(8, v)?;
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

impl ::protobuf::MessageStatic for CMsgSource1LegacyGameEvent_key_t {
    fn new() -> CMsgSource1LegacyGameEvent_key_t {
        CMsgSource1LegacyGameEvent_key_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSource1LegacyGameEvent_key_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CMsgSource1LegacyGameEvent_key_t::get_field_type_for_reflect,
                    CMsgSource1LegacyGameEvent_key_t::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "val_string",
                    CMsgSource1LegacyGameEvent_key_t::get_val_string_for_reflect,
                    CMsgSource1LegacyGameEvent_key_t::mut_val_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "val_float",
                    CMsgSource1LegacyGameEvent_key_t::get_val_float_for_reflect,
                    CMsgSource1LegacyGameEvent_key_t::mut_val_float_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_long",
                    CMsgSource1LegacyGameEvent_key_t::get_val_long_for_reflect,
                    CMsgSource1LegacyGameEvent_key_t::mut_val_long_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_short",
                    CMsgSource1LegacyGameEvent_key_t::get_val_short_for_reflect,
                    CMsgSource1LegacyGameEvent_key_t::mut_val_short_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_byte",
                    CMsgSource1LegacyGameEvent_key_t::get_val_byte_for_reflect,
                    CMsgSource1LegacyGameEvent_key_t::mut_val_byte_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "val_bool",
                    CMsgSource1LegacyGameEvent_key_t::get_val_bool_for_reflect,
                    CMsgSource1LegacyGameEvent_key_t::mut_val_bool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "val_uint64",
                    CMsgSource1LegacyGameEvent_key_t::get_val_uint64_for_reflect,
                    CMsgSource1LegacyGameEvent_key_t::mut_val_uint64_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSource1LegacyGameEvent_key_t>(
                    "CMsgSource1LegacyGameEvent_key_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSource1LegacyGameEvent_key_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_val_string();
        self.clear_val_float();
        self.clear_val_long();
        self.clear_val_short();
        self.clear_val_byte();
        self.clear_val_bool();
        self.clear_val_uint64();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSource1LegacyGameEvent_key_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSource1LegacyGameEvent_key_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSosStartSoundEvent {
    // message fields
    soundevent_guid: ::std::option::Option<i32>,
    soundevent_hash: ::std::option::Option<u32>,
    source_entity_index: ::std::option::Option<i32>,
    seed: ::std::option::Option<i32>,
    packed_params: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_time: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSosStartSoundEvent {}

impl CMsgSosStartSoundEvent {
    pub fn new() -> CMsgSosStartSoundEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSosStartSoundEvent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSosStartSoundEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSosStartSoundEvent,
        };
        unsafe {
            instance.get(CMsgSosStartSoundEvent::new)
        }
    }

    // optional int32 soundevent_guid = 1;

    pub fn clear_soundevent_guid(&mut self) {
        self.soundevent_guid = ::std::option::Option::None;
    }

    pub fn has_soundevent_guid(&self) -> bool {
        self.soundevent_guid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soundevent_guid(&mut self, v: i32) {
        self.soundevent_guid = ::std::option::Option::Some(v);
    }

    pub fn get_soundevent_guid(&self) -> i32 {
        self.soundevent_guid.unwrap_or(0)
    }

    fn get_soundevent_guid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.soundevent_guid
    }

    fn mut_soundevent_guid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.soundevent_guid
    }

    // optional fixed32 soundevent_hash = 2;

    pub fn clear_soundevent_hash(&mut self) {
        self.soundevent_hash = ::std::option::Option::None;
    }

    pub fn has_soundevent_hash(&self) -> bool {
        self.soundevent_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soundevent_hash(&mut self, v: u32) {
        self.soundevent_hash = ::std::option::Option::Some(v);
    }

    pub fn get_soundevent_hash(&self) -> u32 {
        self.soundevent_hash.unwrap_or(0)
    }

    fn get_soundevent_hash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.soundevent_hash
    }

    fn mut_soundevent_hash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.soundevent_hash
    }

    // optional int32 source_entity_index = 3;

    pub fn clear_source_entity_index(&mut self) {
        self.source_entity_index = ::std::option::Option::None;
    }

    pub fn has_source_entity_index(&self) -> bool {
        self.source_entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_entity_index(&mut self, v: i32) {
        self.source_entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_source_entity_index(&self) -> i32 {
        self.source_entity_index.unwrap_or(0)
    }

    fn get_source_entity_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.source_entity_index
    }

    fn mut_source_entity_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.source_entity_index
    }

    // optional int32 seed = 4;

    pub fn clear_seed(&mut self) {
        self.seed = ::std::option::Option::None;
    }

    pub fn has_seed(&self) -> bool {
        self.seed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seed(&mut self, v: i32) {
        self.seed = ::std::option::Option::Some(v);
    }

    pub fn get_seed(&self) -> i32 {
        self.seed.unwrap_or(0)
    }

    fn get_seed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.seed
    }

    fn mut_seed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.seed
    }

    // optional bytes packed_params = 5;

    pub fn clear_packed_params(&mut self) {
        self.packed_params.clear();
    }

    pub fn has_packed_params(&self) -> bool {
        self.packed_params.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packed_params(&mut self, v: ::std::vec::Vec<u8>) {
        self.packed_params = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packed_params(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.packed_params.is_none() {
            self.packed_params.set_default();
        };
        self.packed_params.as_mut().unwrap()
    }

    // Take field
    pub fn take_packed_params(&mut self) -> ::std::vec::Vec<u8> {
        self.packed_params.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_packed_params(&self) -> &[u8] {
        match self.packed_params.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_packed_params_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.packed_params
    }

    fn mut_packed_params_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.packed_params
    }

    // optional float start_time = 6;

    pub fn clear_start_time(&mut self) {
        self.start_time = ::std::option::Option::None;
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: f32) {
        self.start_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_time(&self) -> f32 {
        self.start_time.unwrap_or(0.)
    }

    fn get_start_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.start_time
    }

    fn mut_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.start_time
    }
}

impl ::protobuf::Message for CMsgSosStartSoundEvent {
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
                    self.soundevent_guid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.soundevent_hash = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.source_entity_index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.seed = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.packed_params)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.start_time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.soundevent_guid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.soundevent_hash {
            my_size += 5;
        };
        if let Some(v) = self.source_entity_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seed {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packed_params.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        if let Some(v) = self.start_time {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.soundevent_guid {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.soundevent_hash {
            os.write_fixed32(2, v)?;
        };
        if let Some(v) = self.source_entity_index {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.seed {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.packed_params.as_ref() {
            os.write_bytes(5, &v)?;
        };
        if let Some(v) = self.start_time {
            os.write_float(6, v)?;
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

impl ::protobuf::MessageStatic for CMsgSosStartSoundEvent {
    fn new() -> CMsgSosStartSoundEvent {
        CMsgSosStartSoundEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSosStartSoundEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "soundevent_guid",
                    CMsgSosStartSoundEvent::get_soundevent_guid_for_reflect,
                    CMsgSosStartSoundEvent::mut_soundevent_guid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "soundevent_hash",
                    CMsgSosStartSoundEvent::get_soundevent_hash_for_reflect,
                    CMsgSosStartSoundEvent::mut_soundevent_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "source_entity_index",
                    CMsgSosStartSoundEvent::get_source_entity_index_for_reflect,
                    CMsgSosStartSoundEvent::mut_source_entity_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "seed",
                    CMsgSosStartSoundEvent::get_seed_for_reflect,
                    CMsgSosStartSoundEvent::mut_seed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "packed_params",
                    CMsgSosStartSoundEvent::get_packed_params_for_reflect,
                    CMsgSosStartSoundEvent::mut_packed_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "start_time",
                    CMsgSosStartSoundEvent::get_start_time_for_reflect,
                    CMsgSosStartSoundEvent::mut_start_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSosStartSoundEvent>(
                    "CMsgSosStartSoundEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSosStartSoundEvent {
    fn clear(&mut self) {
        self.clear_soundevent_guid();
        self.clear_soundevent_hash();
        self.clear_source_entity_index();
        self.clear_seed();
        self.clear_packed_params();
        self.clear_start_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSosStartSoundEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSosStartSoundEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSosStopSoundEvent {
    // message fields
    soundevent_guid: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSosStopSoundEvent {}

impl CMsgSosStopSoundEvent {
    pub fn new() -> CMsgSosStopSoundEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSosStopSoundEvent {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSosStopSoundEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSosStopSoundEvent,
        };
        unsafe {
            instance.get(CMsgSosStopSoundEvent::new)
        }
    }

    // optional int32 soundevent_guid = 1;

    pub fn clear_soundevent_guid(&mut self) {
        self.soundevent_guid = ::std::option::Option::None;
    }

    pub fn has_soundevent_guid(&self) -> bool {
        self.soundevent_guid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soundevent_guid(&mut self, v: i32) {
        self.soundevent_guid = ::std::option::Option::Some(v);
    }

    pub fn get_soundevent_guid(&self) -> i32 {
        self.soundevent_guid.unwrap_or(0)
    }

    fn get_soundevent_guid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.soundevent_guid
    }

    fn mut_soundevent_guid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.soundevent_guid
    }
}

impl ::protobuf::Message for CMsgSosStopSoundEvent {
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
                    self.soundevent_guid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.soundevent_guid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.soundevent_guid {
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

impl ::protobuf::MessageStatic for CMsgSosStopSoundEvent {
    fn new() -> CMsgSosStopSoundEvent {
        CMsgSosStopSoundEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSosStopSoundEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "soundevent_guid",
                    CMsgSosStopSoundEvent::get_soundevent_guid_for_reflect,
                    CMsgSosStopSoundEvent::mut_soundevent_guid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSosStopSoundEvent>(
                    "CMsgSosStopSoundEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSosStopSoundEvent {
    fn clear(&mut self) {
        self.clear_soundevent_guid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSosStopSoundEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSosStopSoundEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSosStopSoundEventHash {
    // message fields
    soundevent_hash: ::std::option::Option<u32>,
    source_entity_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSosStopSoundEventHash {}

impl CMsgSosStopSoundEventHash {
    pub fn new() -> CMsgSosStopSoundEventHash {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSosStopSoundEventHash {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSosStopSoundEventHash> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSosStopSoundEventHash,
        };
        unsafe {
            instance.get(CMsgSosStopSoundEventHash::new)
        }
    }

    // optional fixed32 soundevent_hash = 1;

    pub fn clear_soundevent_hash(&mut self) {
        self.soundevent_hash = ::std::option::Option::None;
    }

    pub fn has_soundevent_hash(&self) -> bool {
        self.soundevent_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soundevent_hash(&mut self, v: u32) {
        self.soundevent_hash = ::std::option::Option::Some(v);
    }

    pub fn get_soundevent_hash(&self) -> u32 {
        self.soundevent_hash.unwrap_or(0)
    }

    fn get_soundevent_hash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.soundevent_hash
    }

    fn mut_soundevent_hash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.soundevent_hash
    }

    // optional int32 source_entity_index = 2;

    pub fn clear_source_entity_index(&mut self) {
        self.source_entity_index = ::std::option::Option::None;
    }

    pub fn has_source_entity_index(&self) -> bool {
        self.source_entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_entity_index(&mut self, v: i32) {
        self.source_entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_source_entity_index(&self) -> i32 {
        self.source_entity_index.unwrap_or(0)
    }

    fn get_source_entity_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.source_entity_index
    }

    fn mut_source_entity_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.source_entity_index
    }
}

impl ::protobuf::Message for CMsgSosStopSoundEventHash {
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
                    self.soundevent_hash = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.source_entity_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.soundevent_hash {
            my_size += 5;
        };
        if let Some(v) = self.source_entity_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.soundevent_hash {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.source_entity_index {
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

impl ::protobuf::MessageStatic for CMsgSosStopSoundEventHash {
    fn new() -> CMsgSosStopSoundEventHash {
        CMsgSosStopSoundEventHash::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSosStopSoundEventHash>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "soundevent_hash",
                    CMsgSosStopSoundEventHash::get_soundevent_hash_for_reflect,
                    CMsgSosStopSoundEventHash::mut_soundevent_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "source_entity_index",
                    CMsgSosStopSoundEventHash::get_source_entity_index_for_reflect,
                    CMsgSosStopSoundEventHash::mut_source_entity_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSosStopSoundEventHash>(
                    "CMsgSosStopSoundEventHash",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSosStopSoundEventHash {
    fn clear(&mut self) {
        self.clear_soundevent_hash();
        self.clear_source_entity_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSosStopSoundEventHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSosStopSoundEventHash {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSosSetSoundEventParams {
    // message fields
    soundevent_guid: ::std::option::Option<i32>,
    packed_params: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSosSetSoundEventParams {}

impl CMsgSosSetSoundEventParams {
    pub fn new() -> CMsgSosSetSoundEventParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSosSetSoundEventParams {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSosSetSoundEventParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSosSetSoundEventParams,
        };
        unsafe {
            instance.get(CMsgSosSetSoundEventParams::new)
        }
    }

    // optional int32 soundevent_guid = 1;

    pub fn clear_soundevent_guid(&mut self) {
        self.soundevent_guid = ::std::option::Option::None;
    }

    pub fn has_soundevent_guid(&self) -> bool {
        self.soundevent_guid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soundevent_guid(&mut self, v: i32) {
        self.soundevent_guid = ::std::option::Option::Some(v);
    }

    pub fn get_soundevent_guid(&self) -> i32 {
        self.soundevent_guid.unwrap_or(0)
    }

    fn get_soundevent_guid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.soundevent_guid
    }

    fn mut_soundevent_guid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.soundevent_guid
    }

    // optional bytes packed_params = 5;

    pub fn clear_packed_params(&mut self) {
        self.packed_params.clear();
    }

    pub fn has_packed_params(&self) -> bool {
        self.packed_params.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packed_params(&mut self, v: ::std::vec::Vec<u8>) {
        self.packed_params = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packed_params(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.packed_params.is_none() {
            self.packed_params.set_default();
        };
        self.packed_params.as_mut().unwrap()
    }

    // Take field
    pub fn take_packed_params(&mut self) -> ::std::vec::Vec<u8> {
        self.packed_params.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_packed_params(&self) -> &[u8] {
        match self.packed_params.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_packed_params_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.packed_params
    }

    fn mut_packed_params_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.packed_params
    }
}

impl ::protobuf::Message for CMsgSosSetSoundEventParams {
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
                    self.soundevent_guid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.packed_params)?;
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
        if let Some(v) = self.soundevent_guid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packed_params.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.soundevent_guid {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.packed_params.as_ref() {
            os.write_bytes(5, &v)?;
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

impl ::protobuf::MessageStatic for CMsgSosSetSoundEventParams {
    fn new() -> CMsgSosSetSoundEventParams {
        CMsgSosSetSoundEventParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSosSetSoundEventParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "soundevent_guid",
                    CMsgSosSetSoundEventParams::get_soundevent_guid_for_reflect,
                    CMsgSosSetSoundEventParams::mut_soundevent_guid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "packed_params",
                    CMsgSosSetSoundEventParams::get_packed_params_for_reflect,
                    CMsgSosSetSoundEventParams::mut_packed_params_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSosSetSoundEventParams>(
                    "CMsgSosSetSoundEventParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSosSetSoundEventParams {
    fn clear(&mut self) {
        self.clear_soundevent_guid();
        self.clear_packed_params();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSosSetSoundEventParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSosSetSoundEventParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSosSetLibraryStackFields {
    // message fields
    stack_hash: ::std::option::Option<u32>,
    packed_fields: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSosSetLibraryStackFields {}

impl CMsgSosSetLibraryStackFields {
    pub fn new() -> CMsgSosSetLibraryStackFields {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSosSetLibraryStackFields {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSosSetLibraryStackFields> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSosSetLibraryStackFields,
        };
        unsafe {
            instance.get(CMsgSosSetLibraryStackFields::new)
        }
    }

    // optional fixed32 stack_hash = 1;

    pub fn clear_stack_hash(&mut self) {
        self.stack_hash = ::std::option::Option::None;
    }

    pub fn has_stack_hash(&self) -> bool {
        self.stack_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stack_hash(&mut self, v: u32) {
        self.stack_hash = ::std::option::Option::Some(v);
    }

    pub fn get_stack_hash(&self) -> u32 {
        self.stack_hash.unwrap_or(0)
    }

    fn get_stack_hash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.stack_hash
    }

    fn mut_stack_hash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.stack_hash
    }

    // optional bytes packed_fields = 5;

    pub fn clear_packed_fields(&mut self) {
        self.packed_fields.clear();
    }

    pub fn has_packed_fields(&self) -> bool {
        self.packed_fields.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packed_fields(&mut self, v: ::std::vec::Vec<u8>) {
        self.packed_fields = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_packed_fields(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.packed_fields.is_none() {
            self.packed_fields.set_default();
        };
        self.packed_fields.as_mut().unwrap()
    }

    // Take field
    pub fn take_packed_fields(&mut self) -> ::std::vec::Vec<u8> {
        self.packed_fields.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_packed_fields(&self) -> &[u8] {
        match self.packed_fields.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_packed_fields_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.packed_fields
    }

    fn mut_packed_fields_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.packed_fields
    }
}

impl ::protobuf::Message for CMsgSosSetLibraryStackFields {
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
                    self.stack_hash = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.packed_fields)?;
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
        if let Some(v) = self.stack_hash {
            my_size += 5;
        };
        if let Some(v) = self.packed_fields.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stack_hash {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.packed_fields.as_ref() {
            os.write_bytes(5, &v)?;
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

impl ::protobuf::MessageStatic for CMsgSosSetLibraryStackFields {
    fn new() -> CMsgSosSetLibraryStackFields {
        CMsgSosSetLibraryStackFields::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSosSetLibraryStackFields>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "stack_hash",
                    CMsgSosSetLibraryStackFields::get_stack_hash_for_reflect,
                    CMsgSosSetLibraryStackFields::mut_stack_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "packed_fields",
                    CMsgSosSetLibraryStackFields::get_packed_fields_for_reflect,
                    CMsgSosSetLibraryStackFields::mut_packed_fields_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSosSetLibraryStackFields>(
                    "CMsgSosSetLibraryStackFields",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSosSetLibraryStackFields {
    fn clear(&mut self) {
        self.clear_stack_hash();
        self.clear_packed_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSosSetLibraryStackFields {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSosSetLibraryStackFields {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EBaseGameEvents {
    GE_VDebugGameSessionIDEvent = 200,
    GE_PlaceDecalEvent = 201,
    GE_ClearWorldDecalsEvent = 202,
    GE_ClearEntityDecalsEvent = 203,
    GE_ClearDecalsForSkeletonInstanceEvent = 204,
    GE_Source1LegacyGameEventList = 205,
    GE_Source1LegacyListenEvents = 206,
    GE_Source1LegacyGameEvent = 207,
    GE_SosStartSoundEvent = 208,
    GE_SosStopSoundEvent = 209,
    GE_SosSetSoundEventParams = 210,
    GE_SosSetLibraryStackFields = 211,
    GE_SosStopSoundEventHash = 212,
}

impl ::protobuf::ProtobufEnum for EBaseGameEvents {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EBaseGameEvents> {
        match value {
            200 => ::std::option::Option::Some(EBaseGameEvents::GE_VDebugGameSessionIDEvent),
            201 => ::std::option::Option::Some(EBaseGameEvents::GE_PlaceDecalEvent),
            202 => ::std::option::Option::Some(EBaseGameEvents::GE_ClearWorldDecalsEvent),
            203 => ::std::option::Option::Some(EBaseGameEvents::GE_ClearEntityDecalsEvent),
            204 => ::std::option::Option::Some(EBaseGameEvents::GE_ClearDecalsForSkeletonInstanceEvent),
            205 => ::std::option::Option::Some(EBaseGameEvents::GE_Source1LegacyGameEventList),
            206 => ::std::option::Option::Some(EBaseGameEvents::GE_Source1LegacyListenEvents),
            207 => ::std::option::Option::Some(EBaseGameEvents::GE_Source1LegacyGameEvent),
            208 => ::std::option::Option::Some(EBaseGameEvents::GE_SosStartSoundEvent),
            209 => ::std::option::Option::Some(EBaseGameEvents::GE_SosStopSoundEvent),
            210 => ::std::option::Option::Some(EBaseGameEvents::GE_SosSetSoundEventParams),
            211 => ::std::option::Option::Some(EBaseGameEvents::GE_SosSetLibraryStackFields),
            212 => ::std::option::Option::Some(EBaseGameEvents::GE_SosStopSoundEventHash),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EBaseGameEvents] = &[
            EBaseGameEvents::GE_VDebugGameSessionIDEvent,
            EBaseGameEvents::GE_PlaceDecalEvent,
            EBaseGameEvents::GE_ClearWorldDecalsEvent,
            EBaseGameEvents::GE_ClearEntityDecalsEvent,
            EBaseGameEvents::GE_ClearDecalsForSkeletonInstanceEvent,
            EBaseGameEvents::GE_Source1LegacyGameEventList,
            EBaseGameEvents::GE_Source1LegacyListenEvents,
            EBaseGameEvents::GE_Source1LegacyGameEvent,
            EBaseGameEvents::GE_SosStartSoundEvent,
            EBaseGameEvents::GE_SosStopSoundEvent,
            EBaseGameEvents::GE_SosSetSoundEventParams,
            EBaseGameEvents::GE_SosSetLibraryStackFields,
            EBaseGameEvents::GE_SosStopSoundEventHash,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EBaseGameEvents>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EBaseGameEvents", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EBaseGameEvents {
}

impl ::protobuf::reflect::ProtobufValue for EBaseGameEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x67, 0x61, 0x6d, 0x65, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x16, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x62, 0x61, 0x73, 0x65, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x60, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x44, 0x65, 0x62, 0x75, 0x67, 0x47, 0x61,
    0x6d, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x44, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x69, 0x64, 0x12, 0x24, 0x0a, 0x0d,
    0x67, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0d, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x69, 0x64, 0x22, 0xfe, 0x03, 0x0a, 0x13, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x6c, 0x61, 0x63, 0x65,
    0x44, 0x65, 0x63, 0x61, 0x6c, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x2c, 0x0a, 0x08, 0x70, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x08,
    0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x06, 0x6e, 0x6f, 0x72, 0x6d,
    0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6e, 0x6f, 0x72, 0x6d,
    0x61, 0x6c, 0x12, 0x26, 0x0a, 0x05, 0x73, 0x61, 0x78, 0x69, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x52, 0x05, 0x73, 0x61, 0x78, 0x69, 0x73, 0x12, 0x2e, 0x0a, 0x12, 0x64, 0x65,
    0x63, 0x61, 0x6c, 0x6d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x64, 0x65, 0x63, 0x61, 0x6c, 0x6d, 0x61, 0x74,
    0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c,
    0x61, 0x67, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73,
    0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x07, 0x52,
    0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x12, 0x16, 0x0a, 0x06,
    0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x52, 0x06, 0x68, 0x65,
    0x69, 0x67, 0x68, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x70, 0x74, 0x68, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x05, 0x64, 0x65, 0x70, 0x74, 0x68, 0x12, 0x2c, 0x0a, 0x11, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x68, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x32, 0x0a, 0x14, 0x73, 0x6b, 0x65, 0x6c,
    0x65, 0x74, 0x6f, 0x6e, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x68, 0x61, 0x73, 0x68,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x07, 0x52, 0x14, 0x73, 0x6b, 0x65, 0x6c, 0x65, 0x74, 0x6f, 0x6e,
    0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x68, 0x61, 0x73, 0x68, 0x12, 0x1c, 0x0a, 0x09,
    0x62, 0x6f, 0x6e, 0x65, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x09, 0x62, 0x6f, 0x6e, 0x65, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x26, 0x0a, 0x0e, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x6c, 0x75, 0x63, 0x65, 0x6e, 0x74, 0x68, 0x69, 0x74, 0x18, 0x0d, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x6c, 0x75, 0x63, 0x65, 0x6e, 0x74, 0x68,
    0x69, 0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x73, 0x5f, 0x61, 0x64, 0x6a, 0x61, 0x63, 0x65, 0x6e,
    0x74, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x41, 0x64, 0x6a, 0x61, 0x63,
    0x65, 0x6e, 0x74, 0x22, 0x3f, 0x0a, 0x19, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x65, 0x61, 0x72,
    0x57, 0x6f, 0x72, 0x6c, 0x64, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x12, 0x22, 0x0a, 0x0c, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x74, 0x6f, 0x63, 0x6c, 0x65, 0x61, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x74, 0x6f, 0x63,
    0x6c, 0x65, 0x61, 0x72, 0x22, 0x40, 0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x65, 0x61,
    0x72, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x12, 0x22, 0x0a, 0x0c, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x74, 0x6f, 0x63, 0x6c, 0x65,
    0x61, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x74,
    0x6f, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x22, 0xaf, 0x01, 0x0a, 0x27, 0x43, 0x4d, 0x73, 0x67, 0x43,
    0x6c, 0x65, 0x61, 0x72, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x46, 0x6f, 0x72, 0x53, 0x6b, 0x65,
    0x6c, 0x65, 0x74, 0x6f, 0x6e, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x12, 0x22, 0x0a, 0x0c, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x74, 0x6f, 0x63, 0x6c, 0x65,
    0x61, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x74,
    0x6f, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x12, 0x2c, 0x0a, 0x11, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x11, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x12, 0x32, 0x0a, 0x14, 0x73, 0x6b, 0x65, 0x6c, 0x65, 0x74, 0x6f, 0x6e,
    0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x68, 0x61, 0x73, 0x68, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x14, 0x73, 0x6b, 0x65, 0x6c, 0x65, 0x74, 0x6f, 0x6e, 0x69, 0x6e, 0x73, 0x74,
    0x61, 0x6e, 0x63, 0x65, 0x68, 0x61, 0x73, 0x68, 0x22, 0xa4, 0x02, 0x0a, 0x1e, 0x43, 0x4d, 0x73,
    0x67, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x31, 0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x47, 0x61,
    0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x53, 0x0a, 0x0b, 0x64,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x31, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x31, 0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x2e, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f,
    0x72, 0x5f, 0x74, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73,
    0x1a, 0x2f, 0x0a, 0x05, 0x6b, 0x65, 0x79, 0x5f, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x1a, 0x7c, 0x0a, 0x0c, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x5f,
    0x74, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x69, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12,
    0x3e, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2a, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x31,
    0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4c,
    0x69, 0x73, 0x74, 0x2e, 0x6b, 0x65, 0x79, 0x5f, 0x74, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x22,
    0x67, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x31, 0x4c, 0x65,
    0x67, 0x61, 0x63, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73,
    0x12, 0x1e, 0x0a, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x6c, 0x6f, 0x74,
    0x12, 0x26, 0x0a, 0x0e, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x61, 0x72, 0x72, 0x61, 0x79, 0x62, 0x69,
    0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0e, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x61,
    0x72, 0x72, 0x61, 0x79, 0x62, 0x69, 0x74, 0x73, 0x22, 0xf8, 0x02, 0x0a, 0x1a, 0x43, 0x4d, 0x73,
    0x67, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x31, 0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x47, 0x61,
    0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x69, 0x64,
    0x12, 0x3a, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x26,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65,
    0x31, 0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x2e, 0x6b, 0x65, 0x79, 0x5f, 0x74, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x1a, 0xe4, 0x01, 0x0a,
    0x05, 0x6b, 0x65, 0x79, 0x5f, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x76, 0x61,
    0x6c, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x76, 0x61, 0x6c, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x09, 0x76, 0x61, 0x6c,
    0x5f, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x76, 0x61,
    0x6c, 0x46, 0x6c, 0x6f, 0x61, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x76, 0x61, 0x6c, 0x5f, 0x6c, 0x6f,
    0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x76, 0x61, 0x6c, 0x4c, 0x6f, 0x6e,
    0x67, 0x12, 0x1b, 0x0a, 0x09, 0x76, 0x61, 0x6c, 0x5f, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x76, 0x61, 0x6c, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x12, 0x19,
    0x0a, 0x08, 0x76, 0x61, 0x6c, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x07, 0x76, 0x61, 0x6c, 0x42, 0x79, 0x74, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x76, 0x61, 0x6c,
    0x5f, 0x62, 0x6f, 0x6f, 0x6c, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x76, 0x61, 0x6c,
    0x42, 0x6f, 0x6f, 0x6c, 0x12, 0x1d, 0x0a, 0x0a, 0x76, 0x61, 0x6c, 0x5f, 0x75, 0x69, 0x6e, 0x74,
    0x36, 0x34, 0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x76, 0x61, 0x6c, 0x55, 0x69, 0x6e,
    0x74, 0x36, 0x34, 0x22, 0xf2, 0x01, 0x0a, 0x16, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x6f, 0x73, 0x53,
    0x74, 0x61, 0x72, 0x74, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x27,
    0x0a, 0x0f, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x67, 0x75, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x47, 0x75, 0x69, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x73, 0x6f, 0x75, 0x6e, 0x64,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x0e, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x48, 0x61, 0x73, 0x68,
    0x12, 0x2e, 0x0a, 0x13, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x12, 0x12, 0x0a, 0x04, 0x73, 0x65, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04,
    0x73, 0x65, 0x65, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x5f, 0x70,
    0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x70, 0x61, 0x63,
    0x6b, 0x65, 0x64, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x22, 0x40, 0x0a, 0x15, 0x43, 0x4d, 0x73, 0x67,
    0x53, 0x6f, 0x73, 0x53, 0x74, 0x6f, 0x70, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x12, 0x27, 0x0a, 0x0f, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f,
    0x67, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x73, 0x6f, 0x75, 0x6e,
    0x64, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x47, 0x75, 0x69, 0x64, 0x22, 0x74, 0x0a, 0x19, 0x43, 0x4d,
    0x73, 0x67, 0x53, 0x6f, 0x73, 0x53, 0x74, 0x6f, 0x70, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x48, 0x61, 0x73, 0x68, 0x12, 0x27, 0x0a, 0x0f, 0x73, 0x6f, 0x75, 0x6e, 0x64,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x0e, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x48, 0x61, 0x73, 0x68,
    0x12, 0x2e, 0x0a, 0x13, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x11, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x22, 0x6a, 0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x6f, 0x73, 0x53, 0x65, 0x74, 0x53, 0x6f,
    0x75, 0x6e, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x27,
    0x0a, 0x0f, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x67, 0x75, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x73, 0x6f, 0x75, 0x6e, 0x64, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x47, 0x75, 0x69, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x70, 0x61, 0x63, 0x6b, 0x65,
    0x64, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c,
    0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x22, 0x62, 0x0a, 0x1c,
    0x43, 0x4d, 0x73, 0x67, 0x53, 0x6f, 0x73, 0x53, 0x65, 0x74, 0x4c, 0x69, 0x62, 0x72, 0x61, 0x72,
    0x79, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x12, 0x1d, 0x0a, 0x0a,
    0x73, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x09, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x48, 0x61, 0x73, 0x68, 0x12, 0x23, 0x0a, 0x0d, 0x70,
    0x61, 0x63, 0x6b, 0x65, 0x64, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x0c, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73,
    0x2a, 0xb7, 0x03, 0x0a, 0x0f, 0x45, 0x42, 0x61, 0x73, 0x65, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x73, 0x12, 0x20, 0x0a, 0x1b, 0x47, 0x45, 0x5f, 0x56, 0x44, 0x65, 0x62, 0x75,
    0x67, 0x47, 0x61, 0x6d, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x44, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x10, 0xc8, 0x01, 0x12, 0x17, 0x0a, 0x12, 0x47, 0x45, 0x5f, 0x50, 0x6c, 0x61,
    0x63, 0x65, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0xc9, 0x01, 0x12,
    0x1d, 0x0a, 0x18, 0x47, 0x45, 0x5f, 0x43, 0x6c, 0x65, 0x61, 0x72, 0x57, 0x6f, 0x72, 0x6c, 0x64,
    0x44, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0xca, 0x01, 0x12, 0x1e,
    0x0a, 0x19, 0x47, 0x45, 0x5f, 0x43, 0x6c, 0x65, 0x61, 0x72, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x44, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0xcb, 0x01, 0x12, 0x2b,
    0x0a, 0x26, 0x47, 0x45, 0x5f, 0x43, 0x6c, 0x65, 0x61, 0x72, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x73,
    0x46, 0x6f, 0x72, 0x53, 0x6b, 0x65, 0x6c, 0x65, 0x74, 0x6f, 0x6e, 0x49, 0x6e, 0x73, 0x74, 0x61,
    0x6e, 0x63, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0xcc, 0x01, 0x12, 0x22, 0x0a, 0x1d, 0x47,
    0x45, 0x5f, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x31, 0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x47,
    0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x73, 0x74, 0x10, 0xcd, 0x01, 0x12,
    0x21, 0x0a, 0x1c, 0x47, 0x45, 0x5f, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x31, 0x4c, 0x65, 0x67,
    0x61, 0x63, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x10,
    0xce, 0x01, 0x12, 0x1e, 0x0a, 0x19, 0x47, 0x45, 0x5f, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x31,
    0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10,
    0xcf, 0x01, 0x12, 0x1a, 0x0a, 0x15, 0x47, 0x45, 0x5f, 0x53, 0x6f, 0x73, 0x53, 0x74, 0x61, 0x72,
    0x74, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0xd0, 0x01, 0x12, 0x19,
    0x0a, 0x14, 0x47, 0x45, 0x5f, 0x53, 0x6f, 0x73, 0x53, 0x74, 0x6f, 0x70, 0x53, 0x6f, 0x75, 0x6e,
    0x64, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0xd1, 0x01, 0x12, 0x1e, 0x0a, 0x19, 0x47, 0x45, 0x5f,
    0x53, 0x6f, 0x73, 0x53, 0x65, 0x74, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x10, 0xd2, 0x01, 0x12, 0x20, 0x0a, 0x1b, 0x47, 0x45, 0x5f,
    0x53, 0x6f, 0x73, 0x53, 0x65, 0x74, 0x4c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x53, 0x74, 0x61,
    0x63, 0x6b, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x10, 0xd3, 0x01, 0x12, 0x1d, 0x0a, 0x18, 0x47,
    0x45, 0x5f, 0x53, 0x6f, 0x73, 0x53, 0x74, 0x6f, 0x70, 0x53, 0x6f, 0x75, 0x6e, 0x64, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x48, 0x61, 0x73, 0x68, 0x10, 0xd4, 0x01, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01,
    0x00, 0x4a, 0xcf, 0x26, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x7c, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1f, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x06,
    0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x13,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x1b, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12,
    0x03, 0x07, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x07,
    0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07,
    0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x07,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1d, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x00, 0x01, 0x12, 0x03, 0x09, 0x05, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0a, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x26, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0c, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x0c, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0c,
    0x23, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0d, 0x24, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0e, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x0e, 0x31, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x08,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0f, 0x28, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06,
    0x02, 0x12, 0x03, 0x10, 0x27, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03,
    0x11, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x11, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x11, 0x24, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x12, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x08, 0x02, 0x12, 0x03, 0x12, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09,
    0x12, 0x03, 0x13, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x13, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x13, 0x1f,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x14, 0x08, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x14, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x14, 0x24, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x0b, 0x12, 0x03, 0x15, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01,
    0x12, 0x03, 0x15, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03,
    0x15, 0x26, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x16, 0x08, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x16, 0x08, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x16, 0x23, 0x26, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x19, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x19, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x1b, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x1b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1b,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x18, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1b, 0x28, 0x29, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1e, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x1e, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x1f, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1f, 0x11, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x1c, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1f, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x20, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x20, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20,
    0x1c, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x21, 0x08, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x21, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x21, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x21, 0x1c, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x21, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x22,
    0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x22, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x22, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x22, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x22, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x04, 0x12, 0x03, 0x23, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x23, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x23, 0x18,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x23, 0x20, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x24, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x05, 0x05, 0x12, 0x03, 0x24, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x24, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12,
    0x03, 0x24, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x25, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x25, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x25, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x25, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x25, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x07, 0x12, 0x03, 0x26, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x26,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x26, 0x17, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x26, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12, 0x03, 0x27, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x08, 0x04, 0x12, 0x03, 0x27, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x08, 0x05, 0x12, 0x03, 0x27, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x27, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x27, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x28, 0x08, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x28, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03, 0x28, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x28, 0x2c, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a,
    0x12, 0x03, 0x29, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x04, 0x12, 0x03,
    0x29, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x29, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x29, 0x19, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x29, 0x30, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x03, 0x2a, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0b, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b,
    0x05, 0x12, 0x03, 0x2a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x2a, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x2a,
    0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0c, 0x12, 0x03, 0x2b, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x2b, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x2b, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x2b, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0d, 0x12,
    0x03, 0x2c, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x2c,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x2c, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x2c, 0x16, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x2c, 0x24, 0x26, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x2f, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x2f, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x30, 0x08,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x30, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x33, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x33, 0x08,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x34, 0x08, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x34, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x34, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x34, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x37, 0x00,
    0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x37, 0x08, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x38, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x38, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x38, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x38, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x39, 0x08, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x39, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02,
    0x12, 0x03, 0x3a, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3a, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x18, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x2f, 0x30, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x3d, 0x00, 0x4a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x3d, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x00, 0x12, 0x04, 0x3e,
    0x08, 0x41, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x03, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x10,
    0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x10, 0x28,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3f, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3f, 0x19, 0x1e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x1f, 0x23,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x26, 0x27,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x40, 0x10, 0x29, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x40, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x40, 0x19, 0x1f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x40, 0x20, 0x24, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x27, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x01, 0x12, 0x04, 0x43, 0x08, 0x47, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x03, 0x01, 0x01, 0x12, 0x03, 0x43, 0x10, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x05, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x44, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x44, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x44, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x1f, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05,
    0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05,
    0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x45, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x45, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x45, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x45, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x05, 0x03,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x46, 0x10, 0x47, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x46, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x46, 0x19, 0x3d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x46, 0x3e, 0x42, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x01,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x46, 0x45, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x49, 0x08, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x49, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x49, 0x11,
    0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x49, 0x3d, 0x48, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x4b, 0x4c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x04, 0x4c, 0x00, 0x4f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x4c, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x4d,
    0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4d, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4d, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4d, 0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x4e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4e, 0x18,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4e, 0x29, 0x2a, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x51, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x07, 0x01, 0x12, 0x03, 0x51, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x03, 0x00, 0x12,
    0x04, 0x52, 0x08, 0x5b, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x03, 0x00, 0x01, 0x12, 0x03,
    0x52, 0x10, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x53,
    0x10, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x53,
    0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53,
    0x1f, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53,
    0x26, 0x27, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x54, 0x10,
    0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x54, 0x10,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x54, 0x19,
    0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x20,
    0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x2d,
    0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x55, 0x10, 0x2d,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x55, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x55, 0x19, 0x1e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x55, 0x1f, 0x28,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x55, 0x2b, 0x2c,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x56, 0x10, 0x2c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x56, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x56, 0x19, 0x1e, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x56, 0x1f, 0x27, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x56, 0x2a, 0x2b, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x57, 0x10, 0x2d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x57, 0x10, 0x18, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x57, 0x19, 0x1e, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x57, 0x1f, 0x28, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x57, 0x2b, 0x2c, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x58, 0x10, 0x2c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x58, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x58, 0x19, 0x1e, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x58, 0x1f, 0x27, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x58, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x06, 0x12, 0x03, 0x59, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x59, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x59, 0x19, 0x1d, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x59, 0x1e, 0x26, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x59, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x07, 0x12, 0x03, 0x5a, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x5a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x5a, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x5a, 0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x5a, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x00, 0x12, 0x03, 0x5d, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x5d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x5d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d,
    0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x5e, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x5e, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x5f,
    0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x5f, 0x11, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x32, 0x36, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x39, 0x3a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x62, 0x00, 0x69, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x62,
    0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x63, 0x08, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x17, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x63, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12,
    0x03, 0x64, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x64,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x64, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x64, 0x19, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x64, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x65, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x65, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x65, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x65, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x65, 0x2d,
    0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x66, 0x08, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x66, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x66, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x66, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x66, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03,
    0x67, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x67, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x03, 0x67, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x67, 0x17, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x67, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x05, 0x12, 0x03, 0x68, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x68, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x68, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x68,
    0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x68, 0x24, 0x25,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x6b, 0x00, 0x6d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x03, 0x6b, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x03, 0x6c, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x6c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6c, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6c, 0x17, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6c, 0x29, 0x2a, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0a, 0x12, 0x04, 0x6f, 0x00, 0x72, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01,
    0x12, 0x03, 0x6f, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x70,
    0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x70, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x19, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x01, 0x12, 0x03, 0x71, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x71, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x71, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x17,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x71, 0x2d, 0x2e, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x74, 0x00, 0x77, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0b, 0x01, 0x12, 0x03, 0x74, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12,
    0x03, 0x75, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x75,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x75, 0x11, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x75, 0x17, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x75, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x76, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x76, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x76, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x76, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x76, 0x27,
    0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x79, 0x00, 0x7c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x79, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x00, 0x12, 0x03, 0x7a, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x7a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7a,
    0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7a, 0x19, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7a, 0x26, 0x27, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x7b, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x7b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x7b, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x7b, 0x27, 0x28,
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
