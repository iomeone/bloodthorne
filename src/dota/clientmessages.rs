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
pub struct CClientMsg_CustomGameEvent {
    // message fields
    event_name: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_CustomGameEvent {}

impl CClientMsg_CustomGameEvent {
    pub fn new() -> CClientMsg_CustomGameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_CustomGameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_CustomGameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_CustomGameEvent,
        };
        unsafe {
            instance.get(CClientMsg_CustomGameEvent::new)
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

impl ::protobuf::Message for CClientMsg_CustomGameEvent {
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
        if let Some(v) = self.event_name.as_ref() {
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
        if let Some(v) = self.event_name.as_ref() {
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

impl ::protobuf::MessageStatic for CClientMsg_CustomGameEvent {
    fn new() -> CClientMsg_CustomGameEvent {
        CClientMsg_CustomGameEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_CustomGameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event_name",
                    CClientMsg_CustomGameEvent::get_event_name_for_reflect,
                    CClientMsg_CustomGameEvent::mut_event_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CClientMsg_CustomGameEvent::get_data_for_reflect,
                    CClientMsg_CustomGameEvent::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_CustomGameEvent>(
                    "CClientMsg_CustomGameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_CustomGameEvent {
    fn clear(&mut self) {
        self.clear_event_name();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_CustomGameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_CustomGameEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_ClientUIEvent {
    // message fields
    event: ::std::option::Option<EClientUIEvent>,
    ent_ehandle: ::std::option::Option<u32>,
    client_ehandle: ::std::option::Option<u32>,
    data1: ::protobuf::SingularField<::std::string::String>,
    data2: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_ClientUIEvent {}

impl CClientMsg_ClientUIEvent {
    pub fn new() -> CClientMsg_ClientUIEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_ClientUIEvent {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_ClientUIEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_ClientUIEvent,
        };
        unsafe {
            instance.get(CClientMsg_ClientUIEvent::new)
        }
    }

    // optional .dota.EClientUIEvent event = 1;

    pub fn clear_event(&mut self) {
        self.event = ::std::option::Option::None;
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: EClientUIEvent) {
        self.event = ::std::option::Option::Some(v);
    }

    pub fn get_event(&self) -> EClientUIEvent {
        self.event.unwrap_or(EClientUIEvent::EClientUIEvent_Invalid)
    }

    fn get_event_for_reflect(&self) -> &::std::option::Option<EClientUIEvent> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::std::option::Option<EClientUIEvent> {
        &mut self.event
    }

    // optional uint32 ent_ehandle = 2;

    pub fn clear_ent_ehandle(&mut self) {
        self.ent_ehandle = ::std::option::Option::None;
    }

    pub fn has_ent_ehandle(&self) -> bool {
        self.ent_ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_ehandle(&mut self, v: u32) {
        self.ent_ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_ent_ehandle(&self) -> u32 {
        self.ent_ehandle.unwrap_or(0)
    }

    fn get_ent_ehandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ent_ehandle
    }

    fn mut_ent_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ent_ehandle
    }

    // optional uint32 client_ehandle = 3;

    pub fn clear_client_ehandle(&mut self) {
        self.client_ehandle = ::std::option::Option::None;
    }

    pub fn has_client_ehandle(&self) -> bool {
        self.client_ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_ehandle(&mut self, v: u32) {
        self.client_ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_client_ehandle(&self) -> u32 {
        self.client_ehandle.unwrap_or(0)
    }

    fn get_client_ehandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_ehandle
    }

    fn mut_client_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_ehandle
    }

    // optional string data1 = 4;

    pub fn clear_data1(&mut self) {
        self.data1.clear();
    }

    pub fn has_data1(&self) -> bool {
        self.data1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data1(&mut self, v: ::std::string::String) {
        self.data1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data1(&mut self) -> &mut ::std::string::String {
        if self.data1.is_none() {
            self.data1.set_default();
        };
        self.data1.as_mut().unwrap()
    }

    // Take field
    pub fn take_data1(&mut self) -> ::std::string::String {
        self.data1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data1(&self) -> &str {
        match self.data1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data1
    }

    fn mut_data1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data1
    }

    // optional string data2 = 5;

    pub fn clear_data2(&mut self) {
        self.data2.clear();
    }

    pub fn has_data2(&self) -> bool {
        self.data2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data2(&mut self, v: ::std::string::String) {
        self.data2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data2(&mut self) -> &mut ::std::string::String {
        if self.data2.is_none() {
            self.data2.set_default();
        };
        self.data2.as_mut().unwrap()
    }

    // Take field
    pub fn take_data2(&mut self) -> ::std::string::String {
        self.data2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data2(&self) -> &str {
        match self.data2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data2
    }

    fn mut_data2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data2
    }
}

impl ::protobuf::Message for CClientMsg_ClientUIEvent {
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
                    self.event = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ent_ehandle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_ehandle = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data1)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data2)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.event {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.ent_ehandle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_ehandle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.data1.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.data2.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.ent_ehandle {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.client_ehandle {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.data1.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.data2.as_ref() {
            os.write_string(5, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CClientMsg_ClientUIEvent {
    fn new() -> CClientMsg_ClientUIEvent {
        CClientMsg_ClientUIEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_ClientUIEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EClientUIEvent>>(
                    "event",
                    CClientMsg_ClientUIEvent::get_event_for_reflect,
                    CClientMsg_ClientUIEvent::mut_event_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ent_ehandle",
                    CClientMsg_ClientUIEvent::get_ent_ehandle_for_reflect,
                    CClientMsg_ClientUIEvent::mut_ent_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_ehandle",
                    CClientMsg_ClientUIEvent::get_client_ehandle_for_reflect,
                    CClientMsg_ClientUIEvent::mut_client_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data1",
                    CClientMsg_ClientUIEvent::get_data1_for_reflect,
                    CClientMsg_ClientUIEvent::mut_data1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data2",
                    CClientMsg_ClientUIEvent::get_data2_for_reflect,
                    CClientMsg_ClientUIEvent::mut_data2_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_ClientUIEvent>(
                    "CClientMsg_ClientUIEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_ClientUIEvent {
    fn clear(&mut self) {
        self.clear_event();
        self.clear_ent_ehandle();
        self.clear_client_ehandle();
        self.clear_data1();
        self.clear_data2();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_ClientUIEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_ClientUIEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_DevPaletteVisibilityChangedEvent {
    // message fields
    visible: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_DevPaletteVisibilityChangedEvent {}

impl CClientMsg_DevPaletteVisibilityChangedEvent {
    pub fn new() -> CClientMsg_DevPaletteVisibilityChangedEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_DevPaletteVisibilityChangedEvent {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_DevPaletteVisibilityChangedEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_DevPaletteVisibilityChangedEvent,
        };
        unsafe {
            instance.get(CClientMsg_DevPaletteVisibilityChangedEvent::new)
        }
    }

    // optional bool visible = 1;

    pub fn clear_visible(&mut self) {
        self.visible = ::std::option::Option::None;
    }

    pub fn has_visible(&self) -> bool {
        self.visible.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visible(&mut self, v: bool) {
        self.visible = ::std::option::Option::Some(v);
    }

    pub fn get_visible(&self) -> bool {
        self.visible.unwrap_or(false)
    }

    fn get_visible_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.visible
    }

    fn mut_visible_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.visible
    }
}

impl ::protobuf::Message for CClientMsg_DevPaletteVisibilityChangedEvent {
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
                    self.visible = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.visible {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.visible {
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

impl ::protobuf::MessageStatic for CClientMsg_DevPaletteVisibilityChangedEvent {
    fn new() -> CClientMsg_DevPaletteVisibilityChangedEvent {
        CClientMsg_DevPaletteVisibilityChangedEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_DevPaletteVisibilityChangedEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "visible",
                    CClientMsg_DevPaletteVisibilityChangedEvent::get_visible_for_reflect,
                    CClientMsg_DevPaletteVisibilityChangedEvent::mut_visible_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_DevPaletteVisibilityChangedEvent>(
                    "CClientMsg_DevPaletteVisibilityChangedEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_DevPaletteVisibilityChangedEvent {
    fn clear(&mut self) {
        self.clear_visible();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_DevPaletteVisibilityChangedEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_DevPaletteVisibilityChangedEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_WorldUIControllerHasPanelChangedEvent {
    // message fields
    has_panel: ::std::option::Option<bool>,
    client_ehandle: ::std::option::Option<u32>,
    literal_hand_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_WorldUIControllerHasPanelChangedEvent {}

impl CClientMsg_WorldUIControllerHasPanelChangedEvent {
    pub fn new() -> CClientMsg_WorldUIControllerHasPanelChangedEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_WorldUIControllerHasPanelChangedEvent {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_WorldUIControllerHasPanelChangedEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_WorldUIControllerHasPanelChangedEvent,
        };
        unsafe {
            instance.get(CClientMsg_WorldUIControllerHasPanelChangedEvent::new)
        }
    }

    // optional bool has_panel = 1;

    pub fn clear_has_panel(&mut self) {
        self.has_panel = ::std::option::Option::None;
    }

    pub fn has_has_panel(&self) -> bool {
        self.has_panel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_panel(&mut self, v: bool) {
        self.has_panel = ::std::option::Option::Some(v);
    }

    pub fn get_has_panel(&self) -> bool {
        self.has_panel.unwrap_or(false)
    }

    fn get_has_panel_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_panel
    }

    fn mut_has_panel_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_panel
    }

    // optional uint32 client_ehandle = 2;

    pub fn clear_client_ehandle(&mut self) {
        self.client_ehandle = ::std::option::Option::None;
    }

    pub fn has_client_ehandle(&self) -> bool {
        self.client_ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_ehandle(&mut self, v: u32) {
        self.client_ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_client_ehandle(&self) -> u32 {
        self.client_ehandle.unwrap_or(0)
    }

    fn get_client_ehandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_ehandle
    }

    fn mut_client_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_ehandle
    }

    // optional uint32 literal_hand_type = 3;

    pub fn clear_literal_hand_type(&mut self) {
        self.literal_hand_type = ::std::option::Option::None;
    }

    pub fn has_literal_hand_type(&self) -> bool {
        self.literal_hand_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_literal_hand_type(&mut self, v: u32) {
        self.literal_hand_type = ::std::option::Option::Some(v);
    }

    pub fn get_literal_hand_type(&self) -> u32 {
        self.literal_hand_type.unwrap_or(0)
    }

    fn get_literal_hand_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.literal_hand_type
    }

    fn mut_literal_hand_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.literal_hand_type
    }
}

impl ::protobuf::Message for CClientMsg_WorldUIControllerHasPanelChangedEvent {
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
                    self.has_panel = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_ehandle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.literal_hand_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.has_panel {
            my_size += 2;
        };
        if let Some(v) = self.client_ehandle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.literal_hand_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.has_panel {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.client_ehandle {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.literal_hand_type {
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

impl ::protobuf::MessageStatic for CClientMsg_WorldUIControllerHasPanelChangedEvent {
    fn new() -> CClientMsg_WorldUIControllerHasPanelChangedEvent {
        CClientMsg_WorldUIControllerHasPanelChangedEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_WorldUIControllerHasPanelChangedEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_panel",
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::get_has_panel_for_reflect,
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::mut_has_panel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_ehandle",
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::get_client_ehandle_for_reflect,
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::mut_client_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "literal_hand_type",
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::get_literal_hand_type_for_reflect,
                    CClientMsg_WorldUIControllerHasPanelChangedEvent::mut_literal_hand_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_WorldUIControllerHasPanelChangedEvent>(
                    "CClientMsg_WorldUIControllerHasPanelChangedEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_WorldUIControllerHasPanelChangedEvent {
    fn clear(&mut self) {
        self.clear_has_panel();
        self.clear_client_ehandle();
        self.clear_literal_hand_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_WorldUIControllerHasPanelChangedEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_WorldUIControllerHasPanelChangedEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CClientMsg_RotateAnchor {
    // message fields
    angle: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CClientMsg_RotateAnchor {}

impl CClientMsg_RotateAnchor {
    pub fn new() -> CClientMsg_RotateAnchor {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CClientMsg_RotateAnchor {
        static mut instance: ::protobuf::lazy::Lazy<CClientMsg_RotateAnchor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CClientMsg_RotateAnchor,
        };
        unsafe {
            instance.get(CClientMsg_RotateAnchor::new)
        }
    }

    // optional float angle = 1;

    pub fn clear_angle(&mut self) {
        self.angle = ::std::option::Option::None;
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: f32) {
        self.angle = ::std::option::Option::Some(v);
    }

    pub fn get_angle(&self) -> f32 {
        self.angle.unwrap_or(0.)
    }

    fn get_angle_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.angle
    }
}

impl ::protobuf::Message for CClientMsg_RotateAnchor {
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
                    self.angle = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.angle {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.angle {
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

impl ::protobuf::MessageStatic for CClientMsg_RotateAnchor {
    fn new() -> CClientMsg_RotateAnchor {
        CClientMsg_RotateAnchor::new()
    }

    fn descriptor_static(_: ::std::option::Option<CClientMsg_RotateAnchor>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "angle",
                    CClientMsg_RotateAnchor::get_angle_for_reflect,
                    CClientMsg_RotateAnchor::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CClientMsg_RotateAnchor>(
                    "CClientMsg_RotateAnchor",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CClientMsg_RotateAnchor {
    fn clear(&mut self) {
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CClientMsg_RotateAnchor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CClientMsg_RotateAnchor {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EBaseClientMessages {
    CM_CustomGameEvent = 280,
    CM_ClientUIEvent = 282,
    CM_DevPaletteVisibilityChanged = 283,
    CM_WorldUIControllerHasPanelChanged = 284,
    CM_RotateAnchor = 285,
    CM_MAX_BASE = 300,
}

impl ::protobuf::ProtobufEnum for EBaseClientMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EBaseClientMessages> {
        match value {
            280 => ::std::option::Option::Some(EBaseClientMessages::CM_CustomGameEvent),
            282 => ::std::option::Option::Some(EBaseClientMessages::CM_ClientUIEvent),
            283 => ::std::option::Option::Some(EBaseClientMessages::CM_DevPaletteVisibilityChanged),
            284 => ::std::option::Option::Some(EBaseClientMessages::CM_WorldUIControllerHasPanelChanged),
            285 => ::std::option::Option::Some(EBaseClientMessages::CM_RotateAnchor),
            300 => ::std::option::Option::Some(EBaseClientMessages::CM_MAX_BASE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EBaseClientMessages] = &[
            EBaseClientMessages::CM_CustomGameEvent,
            EBaseClientMessages::CM_ClientUIEvent,
            EBaseClientMessages::CM_DevPaletteVisibilityChanged,
            EBaseClientMessages::CM_WorldUIControllerHasPanelChanged,
            EBaseClientMessages::CM_RotateAnchor,
            EBaseClientMessages::CM_MAX_BASE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EBaseClientMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EBaseClientMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EBaseClientMessages {
}

impl ::protobuf::reflect::ProtobufValue for EBaseClientMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EClientUIEvent {
    EClientUIEvent_Invalid = 0,
    EClientUIEvent_DialogFinished = 1,
    EClientUIEvent_FireOutput = 2,
}

impl ::protobuf::ProtobufEnum for EClientUIEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EClientUIEvent> {
        match value {
            0 => ::std::option::Option::Some(EClientUIEvent::EClientUIEvent_Invalid),
            1 => ::std::option::Option::Some(EClientUIEvent::EClientUIEvent_DialogFinished),
            2 => ::std::option::Option::Some(EClientUIEvent::EClientUIEvent_FireOutput),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EClientUIEvent] = &[
            EClientUIEvent::EClientUIEvent_Invalid,
            EClientUIEvent::EClientUIEvent_DialogFinished,
            EClientUIEvent::EClientUIEvent_FireOutput,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EClientUIEvent>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EClientUIEvent", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EClientUIEvent {
}

impl ::protobuf::reflect::ProtobufValue for EClientUIEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x22, 0x4f, 0x0a, 0x1a,
    0x43, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0xd2, 0x01,
    0x0a, 0x18, 0x43, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x55, 0x49, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x42, 0x0a, 0x05, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x45, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x55, 0x49, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x3a,
    0x16, 0x45, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x55, 0x49, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x5f,
    0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x52, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x1f,
    0x0a, 0x0b, 0x65, 0x6e, 0x74, 0x5f, 0x65, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0a, 0x65, 0x6e, 0x74, 0x45, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x12,
    0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x65, 0x68, 0x61, 0x6e, 0x64, 0x6c,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x45,
    0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x61, 0x74, 0x61, 0x31, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x64, 0x61, 0x74, 0x61, 0x31, 0x12, 0x14, 0x0a, 0x05,
    0x64, 0x61, 0x74, 0x61, 0x32, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x64, 0x61, 0x74,
    0x61, 0x32, 0x22, 0x47, 0x0a, 0x2b, 0x43, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67,
    0x5f, 0x44, 0x65, 0x76, 0x50, 0x61, 0x6c, 0x65, 0x74, 0x74, 0x65, 0x56, 0x69, 0x73, 0x69, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x07, 0x76, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x22, 0xa2, 0x01, 0x0a, 0x30,
    0x43, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x57, 0x6f, 0x72, 0x6c, 0x64,
    0x55, 0x49, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x48, 0x61, 0x73, 0x50,
    0x61, 0x6e, 0x65, 0x6c, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x12, 0x1b, 0x0a, 0x09, 0x68, 0x61, 0x73, 0x5f, 0x70, 0x61, 0x6e, 0x65, 0x6c, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x08, 0x68, 0x61, 0x73, 0x50, 0x61, 0x6e, 0x65, 0x6c, 0x12, 0x25, 0x0a,
    0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x65, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x45, 0x68, 0x61,
    0x6e, 0x64, 0x6c, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x6c, 0x69, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x5f,
    0x68, 0x61, 0x6e, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0f, 0x6c, 0x69, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x48, 0x61, 0x6e, 0x64, 0x54, 0x79, 0x70, 0x65,
    0x22, 0x2f, 0x0a, 0x17, 0x43, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x52,
    0x6f, 0x74, 0x61, 0x74, 0x65, 0x41, 0x6e, 0x63, 0x68, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x61,
    0x6e, 0x67, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x61, 0x6e, 0x67, 0x6c,
    0x65, 0x2a, 0xbc, 0x01, 0x0a, 0x13, 0x45, 0x42, 0x61, 0x73, 0x65, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x17, 0x0a, 0x12, 0x43, 0x4d, 0x5f,
    0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10,
    0x98, 0x02, 0x12, 0x15, 0x0a, 0x10, 0x43, 0x4d, 0x5f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x55,
    0x49, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x9a, 0x02, 0x12, 0x23, 0x0a, 0x1e, 0x43, 0x4d, 0x5f,
    0x44, 0x65, 0x76, 0x50, 0x61, 0x6c, 0x65, 0x74, 0x74, 0x65, 0x56, 0x69, 0x73, 0x69, 0x62, 0x69,
    0x6c, 0x69, 0x74, 0x79, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x10, 0x9b, 0x02, 0x12, 0x28,
    0x0a, 0x23, 0x43, 0x4d, 0x5f, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x55, 0x49, 0x43, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x48, 0x61, 0x73, 0x50, 0x61, 0x6e, 0x65, 0x6c, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x64, 0x10, 0x9c, 0x02, 0x12, 0x14, 0x0a, 0x0f, 0x43, 0x4d, 0x5f, 0x52,
    0x6f, 0x74, 0x61, 0x74, 0x65, 0x41, 0x6e, 0x63, 0x68, 0x6f, 0x72, 0x10, 0x9d, 0x02, 0x12, 0x10,
    0x0a, 0x0b, 0x43, 0x4d, 0x5f, 0x4d, 0x41, 0x58, 0x5f, 0x42, 0x41, 0x53, 0x45, 0x10, 0xac, 0x02,
    0x2a, 0x6e, 0x0a, 0x0e, 0x45, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x55, 0x49, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x12, 0x1a, 0x0a, 0x16, 0x45, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x55, 0x49, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x5f, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x10, 0x00, 0x12, 0x21,
    0x0a, 0x1d, 0x45, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x55, 0x49, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x5f, 0x44, 0x69, 0x61, 0x6c, 0x6f, 0x67, 0x46, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x10,
    0x01, 0x12, 0x1d, 0x0a, 0x19, 0x45, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x55, 0x49, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x5f, 0x46, 0x69, 0x72, 0x65, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x10, 0x02,
    0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xb1, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x2f, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x1c,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x04, 0x00, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x04, 0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05,
    0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x05, 0x00, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x05, 0x07, 0x1a, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x05, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x05, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00,
    0x12, 0x04, 0x07, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x07,
    0x05, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x08, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x08, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x09, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x08,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x29, 0x2c, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x0b, 0x2e, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x0c, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0c, 0x08,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x0c, 0x1a, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x05, 0x02, 0x12, 0x03, 0x0d, 0x16, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04,
    0x10, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x10, 0x05, 0x13,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x08, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x11, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x01, 0x12, 0x03, 0x12, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x12, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x12,
    0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x13, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x13, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x16, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x16, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x17, 0x08, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x18, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x18, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x17, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x1b, 0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x1b, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1c,
    0x08, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1c, 0x11, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x20, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x1c, 0x2a, 0x4c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x07, 0x12, 0x03, 0x1c, 0x35, 0x4b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x1d, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1d, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1d, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x18, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x1e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1e,
    0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x29, 0x2a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1f, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x1f, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x20,
    0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x20, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x20, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x20, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x20, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x23, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23,
    0x08, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x24, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x27,
    0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x27, 0x08, 0x38, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x28, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x28, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x28, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x29, 0x08,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x29, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x02, 0x12, 0x03, 0x2a, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2a,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x18, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x2c, 0x2d, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2d, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x2d, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x2e, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x17, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x1f, 0x20,
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
