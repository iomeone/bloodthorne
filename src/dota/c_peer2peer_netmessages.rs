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
pub struct CP2P_TextMessage {
    // message fields
    text: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_TextMessage {}

impl CP2P_TextMessage {
    pub fn new() -> CP2P_TextMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_TextMessage {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_TextMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_TextMessage,
        };
        unsafe {
            instance.get(CP2P_TextMessage::new)
        }
    }

    // optional bytes text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::vec::Vec<u8>) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::vec::Vec<u8> {
        self.text.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_text(&self) -> &[u8] {
        match self.text.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.text
    }
}

impl ::protobuf::Message for CP2P_TextMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.text)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.text.as_ref() {
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

impl ::protobuf::MessageStatic for CP2P_TextMessage {
    fn new() -> CP2P_TextMessage {
        CP2P_TextMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_TextMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "text",
                    CP2P_TextMessage::get_text_for_reflect,
                    CP2P_TextMessage::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_TextMessage>(
                    "CP2P_TextMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_TextMessage {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_TextMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_TextMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSteam_Voice_Encoding {
    // message fields
    voice_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSteam_Voice_Encoding {}

impl CSteam_Voice_Encoding {
    pub fn new() -> CSteam_Voice_Encoding {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSteam_Voice_Encoding {
        static mut instance: ::protobuf::lazy::Lazy<CSteam_Voice_Encoding> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSteam_Voice_Encoding,
        };
        unsafe {
            instance.get(CSteam_Voice_Encoding::new)
        }
    }

    // optional bytes voice_data = 1;

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
}

impl ::protobuf::Message for CSteam_Voice_Encoding {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.voice_data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.voice_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.voice_data.as_ref() {
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

impl ::protobuf::MessageStatic for CSteam_Voice_Encoding {
    fn new() -> CSteam_Voice_Encoding {
        CSteam_Voice_Encoding::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSteam_Voice_Encoding>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "voice_data",
                    CSteam_Voice_Encoding::get_voice_data_for_reflect,
                    CSteam_Voice_Encoding::mut_voice_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSteam_Voice_Encoding>(
                    "CSteam_Voice_Encoding",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSteam_Voice_Encoding {
    fn clear(&mut self) {
        self.clear_voice_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSteam_Voice_Encoding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSteam_Voice_Encoding {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_Voice {
    // message fields
    audio: ::protobuf::SingularPtrField<super::netmessages::CMsgVoiceAudio>,
    broadcast_group: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_Voice {}

impl CP2P_Voice {
    pub fn new() -> CP2P_Voice {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_Voice {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_Voice> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_Voice,
        };
        unsafe {
            instance.get(CP2P_Voice::new)
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
    pub fn set_audio(&mut self, v: super::netmessages::CMsgVoiceAudio) {
        self.audio = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_audio(&mut self) -> &mut super::netmessages::CMsgVoiceAudio {
        if self.audio.is_none() {
            self.audio.set_default();
        };
        self.audio.as_mut().unwrap()
    }

    // Take field
    pub fn take_audio(&mut self) -> super::netmessages::CMsgVoiceAudio {
        self.audio.take().unwrap_or_else(|| super::netmessages::CMsgVoiceAudio::new())
    }

    pub fn get_audio(&self) -> &super::netmessages::CMsgVoiceAudio {
        self.audio.as_ref().unwrap_or_else(|| super::netmessages::CMsgVoiceAudio::default_instance())
    }

    fn get_audio_for_reflect(&self) -> &::protobuf::SingularPtrField<super::netmessages::CMsgVoiceAudio> {
        &self.audio
    }

    fn mut_audio_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::netmessages::CMsgVoiceAudio> {
        &mut self.audio
    }

    // optional uint32 broadcast_group = 2;

    pub fn clear_broadcast_group(&mut self) {
        self.broadcast_group = ::std::option::Option::None;
    }

    pub fn has_broadcast_group(&self) -> bool {
        self.broadcast_group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcast_group(&mut self, v: u32) {
        self.broadcast_group = ::std::option::Option::Some(v);
    }

    pub fn get_broadcast_group(&self) -> u32 {
        self.broadcast_group.unwrap_or(0)
    }

    fn get_broadcast_group_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.broadcast_group
    }

    fn mut_broadcast_group_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.broadcast_group
    }
}

impl ::protobuf::Message for CP2P_Voice {
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
                    let tmp = is.read_uint32()?;
                    self.broadcast_group = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.broadcast_group {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.broadcast_group {
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

impl ::protobuf::MessageStatic for CP2P_Voice {
    fn new() -> CP2P_Voice {
        CP2P_Voice::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_Voice>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::netmessages::CMsgVoiceAudio>>(
                    "audio",
                    CP2P_Voice::get_audio_for_reflect,
                    CP2P_Voice::mut_audio_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "broadcast_group",
                    CP2P_Voice::get_broadcast_group_for_reflect,
                    CP2P_Voice::mut_broadcast_group_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_Voice>(
                    "CP2P_Voice",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_Voice {
    fn clear(&mut self) {
        self.clear_audio();
        self.clear_broadcast_group();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_Voice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_Voice {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CP2P_Voice_Handler_Flags {
    Played_Audio = 1,
}

impl ::protobuf::ProtobufEnum for CP2P_Voice_Handler_Flags {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CP2P_Voice_Handler_Flags> {
        match value {
            1 => ::std::option::Option::Some(CP2P_Voice_Handler_Flags::Played_Audio),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CP2P_Voice_Handler_Flags] = &[
            CP2P_Voice_Handler_Flags::Played_Audio,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CP2P_Voice_Handler_Flags>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CP2P_Voice_Handler_Flags", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CP2P_Voice_Handler_Flags {
}

impl ::protobuf::reflect::ProtobufValue for CP2P_Voice_Handler_Flags {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_Ping {
    // message fields
    send_time: ::std::option::Option<u64>,
    is_reply: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_Ping {}

impl CP2P_Ping {
    pub fn new() -> CP2P_Ping {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_Ping {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_Ping> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_Ping,
        };
        unsafe {
            instance.get(CP2P_Ping::new)
        }
    }

    // required uint64 send_time = 1;

    pub fn clear_send_time(&mut self) {
        self.send_time = ::std::option::Option::None;
    }

    pub fn has_send_time(&self) -> bool {
        self.send_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_time(&mut self, v: u64) {
        self.send_time = ::std::option::Option::Some(v);
    }

    pub fn get_send_time(&self) -> u64 {
        self.send_time.unwrap_or(0)
    }

    fn get_send_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.send_time
    }

    fn mut_send_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.send_time
    }

    // required bool is_reply = 2;

    pub fn clear_is_reply(&mut self) {
        self.is_reply = ::std::option::Option::None;
    }

    pub fn has_is_reply(&self) -> bool {
        self.is_reply.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_reply(&mut self, v: bool) {
        self.is_reply = ::std::option::Option::Some(v);
    }

    pub fn get_is_reply(&self) -> bool {
        self.is_reply.unwrap_or(false)
    }

    fn get_is_reply_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_reply
    }

    fn mut_is_reply_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_reply
    }
}

impl ::protobuf::Message for CP2P_Ping {
    fn is_initialized(&self) -> bool {
        if self.send_time.is_none() {
            return false;
        };
        if self.is_reply.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.send_time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_reply = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.send_time {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.is_reply {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.send_time {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.is_reply {
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

impl ::protobuf::MessageStatic for CP2P_Ping {
    fn new() -> CP2P_Ping {
        CP2P_Ping::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_Ping>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "send_time",
                    CP2P_Ping::get_send_time_for_reflect,
                    CP2P_Ping::mut_send_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_reply",
                    CP2P_Ping::get_is_reply_for_reflect,
                    CP2P_Ping::mut_is_reply_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_Ping>(
                    "CP2P_Ping",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_Ping {
    fn clear(&mut self) {
        self.clear_send_time();
        self.clear_is_reply();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_Ping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_Ping {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_VRAvatarPosition {
    // message fields
    body_parts: ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation>,
    hat_id: ::std::option::Option<i32>,
    scene_id: ::std::option::Option<i32>,
    world_scale: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_VRAvatarPosition {}

impl CP2P_VRAvatarPosition {
    pub fn new() -> CP2P_VRAvatarPosition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_VRAvatarPosition {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_VRAvatarPosition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_VRAvatarPosition,
        };
        unsafe {
            instance.get(CP2P_VRAvatarPosition::new)
        }
    }

    // repeated .dota.CP2P_VRAvatarPosition.COrientation body_parts = 1;

    pub fn clear_body_parts(&mut self) {
        self.body_parts.clear();
    }

    // Param is passed by value, moved
    pub fn set_body_parts(&mut self, v: ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation>) {
        self.body_parts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_body_parts(&mut self) -> &mut ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation> {
        &mut self.body_parts
    }

    // Take field
    pub fn take_body_parts(&mut self) -> ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation> {
        ::std::mem::replace(&mut self.body_parts, ::protobuf::RepeatedField::new())
    }

    pub fn get_body_parts(&self) -> &[CP2P_VRAvatarPosition_COrientation] {
        &self.body_parts
    }

    fn get_body_parts_for_reflect(&self) -> &::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation> {
        &self.body_parts
    }

    fn mut_body_parts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CP2P_VRAvatarPosition_COrientation> {
        &mut self.body_parts
    }

    // optional int32 hat_id = 2;

    pub fn clear_hat_id(&mut self) {
        self.hat_id = ::std::option::Option::None;
    }

    pub fn has_hat_id(&self) -> bool {
        self.hat_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hat_id(&mut self, v: i32) {
        self.hat_id = ::std::option::Option::Some(v);
    }

    pub fn get_hat_id(&self) -> i32 {
        self.hat_id.unwrap_or(0)
    }

    fn get_hat_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.hat_id
    }

    fn mut_hat_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.hat_id
    }

    // optional int32 scene_id = 3;

    pub fn clear_scene_id(&mut self) {
        self.scene_id = ::std::option::Option::None;
    }

    pub fn has_scene_id(&self) -> bool {
        self.scene_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scene_id(&mut self, v: i32) {
        self.scene_id = ::std::option::Option::Some(v);
    }

    pub fn get_scene_id(&self) -> i32 {
        self.scene_id.unwrap_or(0)
    }

    fn get_scene_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.scene_id
    }

    fn mut_scene_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.scene_id
    }

    // optional int32 world_scale = 4;

    pub fn clear_world_scale(&mut self) {
        self.world_scale = ::std::option::Option::None;
    }

    pub fn has_world_scale(&self) -> bool {
        self.world_scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_scale(&mut self, v: i32) {
        self.world_scale = ::std::option::Option::Some(v);
    }

    pub fn get_world_scale(&self) -> i32 {
        self.world_scale.unwrap_or(0)
    }

    fn get_world_scale_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.world_scale
    }

    fn mut_world_scale_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.world_scale
    }
}

impl ::protobuf::Message for CP2P_VRAvatarPosition {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.body_parts)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.hat_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.scene_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.world_scale = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.body_parts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hat_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.scene_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.world_scale {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.body_parts {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hat_id {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.scene_id {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.world_scale {
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

impl ::protobuf::MessageStatic for CP2P_VRAvatarPosition {
    fn new() -> CP2P_VRAvatarPosition {
        CP2P_VRAvatarPosition::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_VRAvatarPosition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CP2P_VRAvatarPosition_COrientation>>(
                    "body_parts",
                    CP2P_VRAvatarPosition::get_body_parts_for_reflect,
                    CP2P_VRAvatarPosition::mut_body_parts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "hat_id",
                    CP2P_VRAvatarPosition::get_hat_id_for_reflect,
                    CP2P_VRAvatarPosition::mut_hat_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "scene_id",
                    CP2P_VRAvatarPosition::get_scene_id_for_reflect,
                    CP2P_VRAvatarPosition::mut_scene_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "world_scale",
                    CP2P_VRAvatarPosition::get_world_scale_for_reflect,
                    CP2P_VRAvatarPosition::mut_world_scale_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_VRAvatarPosition>(
                    "CP2P_VRAvatarPosition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_VRAvatarPosition {
    fn clear(&mut self) {
        self.clear_body_parts();
        self.clear_hat_id();
        self.clear_scene_id();
        self.clear_world_scale();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_VRAvatarPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_VRAvatarPosition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_VRAvatarPosition_COrientation {
    // message fields
    pos: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    ang: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_VRAvatarPosition_COrientation {}

impl CP2P_VRAvatarPosition_COrientation {
    pub fn new() -> CP2P_VRAvatarPosition_COrientation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_VRAvatarPosition_COrientation {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_VRAvatarPosition_COrientation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_VRAvatarPosition_COrientation,
        };
        unsafe {
            instance.get(CP2P_VRAvatarPosition_COrientation::new)
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

    // optional .dota.CMsgQAngle ang = 2;

    pub fn clear_ang(&mut self) {
        self.ang.clear();
    }

    pub fn has_ang(&self) -> bool {
        self.ang.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ang(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.ang = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ang(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.ang.is_none() {
            self.ang.set_default();
        };
        self.ang.as_mut().unwrap()
    }

    // Take field
    pub fn take_ang(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.ang.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_ang(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.ang.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_ang_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.ang
    }

    fn mut_ang_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.ang
    }
}

impl ::protobuf::Message for CP2P_VRAvatarPosition_COrientation {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ang)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.ang.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.ang.as_ref() {
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

impl ::protobuf::MessageStatic for CP2P_VRAvatarPosition_COrientation {
    fn new() -> CP2P_VRAvatarPosition_COrientation {
        CP2P_VRAvatarPosition_COrientation::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_VRAvatarPosition_COrientation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "pos",
                    CP2P_VRAvatarPosition_COrientation::get_pos_for_reflect,
                    CP2P_VRAvatarPosition_COrientation::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "ang",
                    CP2P_VRAvatarPosition_COrientation::get_ang_for_reflect,
                    CP2P_VRAvatarPosition_COrientation::mut_ang_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_VRAvatarPosition_COrientation>(
                    "CP2P_VRAvatarPosition_COrientation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_VRAvatarPosition_COrientation {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_ang();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_VRAvatarPosition_COrientation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_VRAvatarPosition_COrientation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CP2P_WatchSynchronization {
    // message fields
    demo_tick: ::std::option::Option<i32>,
    paused: ::std::option::Option<bool>,
    tv_listen_voice_indices: ::std::option::Option<i32>,
    dota_spectator_mode: ::std::option::Option<i32>,
    dota_spectator_watching_broadcaster: ::std::option::Option<i32>,
    dota_spectator_hero_index: ::std::option::Option<i32>,
    dota_spectator_autospeed: ::std::option::Option<i32>,
    dota_replay_speed: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CP2P_WatchSynchronization {}

impl CP2P_WatchSynchronization {
    pub fn new() -> CP2P_WatchSynchronization {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CP2P_WatchSynchronization {
        static mut instance: ::protobuf::lazy::Lazy<CP2P_WatchSynchronization> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CP2P_WatchSynchronization,
        };
        unsafe {
            instance.get(CP2P_WatchSynchronization::new)
        }
    }

    // optional int32 demo_tick = 1;

    pub fn clear_demo_tick(&mut self) {
        self.demo_tick = ::std::option::Option::None;
    }

    pub fn has_demo_tick(&self) -> bool {
        self.demo_tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_demo_tick(&mut self, v: i32) {
        self.demo_tick = ::std::option::Option::Some(v);
    }

    pub fn get_demo_tick(&self) -> i32 {
        self.demo_tick.unwrap_or(0)
    }

    fn get_demo_tick_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.demo_tick
    }

    fn mut_demo_tick_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.demo_tick
    }

    // optional bool paused = 2;

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

    // optional int32 tv_listen_voice_indices = 3;

    pub fn clear_tv_listen_voice_indices(&mut self) {
        self.tv_listen_voice_indices = ::std::option::Option::None;
    }

    pub fn has_tv_listen_voice_indices(&self) -> bool {
        self.tv_listen_voice_indices.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tv_listen_voice_indices(&mut self, v: i32) {
        self.tv_listen_voice_indices = ::std::option::Option::Some(v);
    }

    pub fn get_tv_listen_voice_indices(&self) -> i32 {
        self.tv_listen_voice_indices.unwrap_or(0)
    }

    fn get_tv_listen_voice_indices_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.tv_listen_voice_indices
    }

    fn mut_tv_listen_voice_indices_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.tv_listen_voice_indices
    }

    // optional int32 dota_spectator_mode = 4;

    pub fn clear_dota_spectator_mode(&mut self) {
        self.dota_spectator_mode = ::std::option::Option::None;
    }

    pub fn has_dota_spectator_mode(&self) -> bool {
        self.dota_spectator_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_spectator_mode(&mut self, v: i32) {
        self.dota_spectator_mode = ::std::option::Option::Some(v);
    }

    pub fn get_dota_spectator_mode(&self) -> i32 {
        self.dota_spectator_mode.unwrap_or(0)
    }

    fn get_dota_spectator_mode_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_spectator_mode
    }

    fn mut_dota_spectator_mode_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_spectator_mode
    }

    // optional int32 dota_spectator_watching_broadcaster = 5;

    pub fn clear_dota_spectator_watching_broadcaster(&mut self) {
        self.dota_spectator_watching_broadcaster = ::std::option::Option::None;
    }

    pub fn has_dota_spectator_watching_broadcaster(&self) -> bool {
        self.dota_spectator_watching_broadcaster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_spectator_watching_broadcaster(&mut self, v: i32) {
        self.dota_spectator_watching_broadcaster = ::std::option::Option::Some(v);
    }

    pub fn get_dota_spectator_watching_broadcaster(&self) -> i32 {
        self.dota_spectator_watching_broadcaster.unwrap_or(0)
    }

    fn get_dota_spectator_watching_broadcaster_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_spectator_watching_broadcaster
    }

    fn mut_dota_spectator_watching_broadcaster_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_spectator_watching_broadcaster
    }

    // optional int32 dota_spectator_hero_index = 6;

    pub fn clear_dota_spectator_hero_index(&mut self) {
        self.dota_spectator_hero_index = ::std::option::Option::None;
    }

    pub fn has_dota_spectator_hero_index(&self) -> bool {
        self.dota_spectator_hero_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_spectator_hero_index(&mut self, v: i32) {
        self.dota_spectator_hero_index = ::std::option::Option::Some(v);
    }

    pub fn get_dota_spectator_hero_index(&self) -> i32 {
        self.dota_spectator_hero_index.unwrap_or(0)
    }

    fn get_dota_spectator_hero_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_spectator_hero_index
    }

    fn mut_dota_spectator_hero_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_spectator_hero_index
    }

    // optional int32 dota_spectator_autospeed = 7;

    pub fn clear_dota_spectator_autospeed(&mut self) {
        self.dota_spectator_autospeed = ::std::option::Option::None;
    }

    pub fn has_dota_spectator_autospeed(&self) -> bool {
        self.dota_spectator_autospeed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_spectator_autospeed(&mut self, v: i32) {
        self.dota_spectator_autospeed = ::std::option::Option::Some(v);
    }

    pub fn get_dota_spectator_autospeed(&self) -> i32 {
        self.dota_spectator_autospeed.unwrap_or(0)
    }

    fn get_dota_spectator_autospeed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_spectator_autospeed
    }

    fn mut_dota_spectator_autospeed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_spectator_autospeed
    }

    // optional int32 dota_replay_speed = 8;

    pub fn clear_dota_replay_speed(&mut self) {
        self.dota_replay_speed = ::std::option::Option::None;
    }

    pub fn has_dota_replay_speed(&self) -> bool {
        self.dota_replay_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_replay_speed(&mut self, v: i32) {
        self.dota_replay_speed = ::std::option::Option::Some(v);
    }

    pub fn get_dota_replay_speed(&self) -> i32 {
        self.dota_replay_speed.unwrap_or(0)
    }

    fn get_dota_replay_speed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dota_replay_speed
    }

    fn mut_dota_replay_speed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dota_replay_speed
    }
}

impl ::protobuf::Message for CP2P_WatchSynchronization {
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
                    self.demo_tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.paused = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.tv_listen_voice_indices = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.dota_spectator_mode = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.dota_spectator_watching_broadcaster = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.dota_spectator_hero_index = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.dota_spectator_autospeed = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.dota_replay_speed = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.demo_tick {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.paused {
            my_size += 2;
        };
        if let Some(v) = self.tv_listen_voice_indices {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dota_spectator_mode {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dota_spectator_watching_broadcaster {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dota_spectator_hero_index {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dota_spectator_autospeed {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dota_replay_speed {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.demo_tick {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.paused {
            os.write_bool(2, v)?;
        };
        if let Some(v) = self.tv_listen_voice_indices {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.dota_spectator_mode {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.dota_spectator_watching_broadcaster {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.dota_spectator_hero_index {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.dota_spectator_autospeed {
            os.write_int32(7, v)?;
        };
        if let Some(v) = self.dota_replay_speed {
            os.write_int32(8, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CP2P_WatchSynchronization {
    fn new() -> CP2P_WatchSynchronization {
        CP2P_WatchSynchronization::new()
    }

    fn descriptor_static(_: ::std::option::Option<CP2P_WatchSynchronization>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "demo_tick",
                    CP2P_WatchSynchronization::get_demo_tick_for_reflect,
                    CP2P_WatchSynchronization::mut_demo_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "paused",
                    CP2P_WatchSynchronization::get_paused_for_reflect,
                    CP2P_WatchSynchronization::mut_paused_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tv_listen_voice_indices",
                    CP2P_WatchSynchronization::get_tv_listen_voice_indices_for_reflect,
                    CP2P_WatchSynchronization::mut_tv_listen_voice_indices_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_spectator_mode",
                    CP2P_WatchSynchronization::get_dota_spectator_mode_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_spectator_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_spectator_watching_broadcaster",
                    CP2P_WatchSynchronization::get_dota_spectator_watching_broadcaster_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_spectator_watching_broadcaster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_spectator_hero_index",
                    CP2P_WatchSynchronization::get_dota_spectator_hero_index_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_spectator_hero_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_spectator_autospeed",
                    CP2P_WatchSynchronization::get_dota_spectator_autospeed_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_spectator_autospeed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dota_replay_speed",
                    CP2P_WatchSynchronization::get_dota_replay_speed_for_reflect,
                    CP2P_WatchSynchronization::mut_dota_replay_speed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CP2P_WatchSynchronization>(
                    "CP2P_WatchSynchronization",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CP2P_WatchSynchronization {
    fn clear(&mut self) {
        self.clear_demo_tick();
        self.clear_paused();
        self.clear_tv_listen_voice_indices();
        self.clear_dota_spectator_mode();
        self.clear_dota_spectator_watching_broadcaster();
        self.clear_dota_spectator_hero_index();
        self.clear_dota_spectator_autospeed();
        self.clear_dota_replay_speed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CP2P_WatchSynchronization {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CP2P_WatchSynchronization {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum P2P_Messages {
    p2p_TextMessage = 256,
    p2p_Voice = 257,
    p2p_Ping = 258,
    p2p_VRAvatarPosition = 259,
    p2p_WatchSynchronization = 260,
}

impl ::protobuf::ProtobufEnum for P2P_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<P2P_Messages> {
        match value {
            256 => ::std::option::Option::Some(P2P_Messages::p2p_TextMessage),
            257 => ::std::option::Option::Some(P2P_Messages::p2p_Voice),
            258 => ::std::option::Option::Some(P2P_Messages::p2p_Ping),
            259 => ::std::option::Option::Some(P2P_Messages::p2p_VRAvatarPosition),
            260 => ::std::option::Option::Some(P2P_Messages::p2p_WatchSynchronization),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [P2P_Messages] = &[
            P2P_Messages::p2p_TextMessage,
            P2P_Messages::p2p_Voice,
            P2P_Messages::p2p_Ping,
            P2P_Messages::p2p_VRAvatarPosition,
            P2P_Messages::p2p_WatchSynchronization,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<P2P_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("P2P_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for P2P_Messages {
}

impl ::protobuf::reflect::ProtobufValue for P2P_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1d, 0x63, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x32, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x6e, 0x65,
    0x74, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x11, 0x6e, 0x65, 0x74, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
    0x6b, 0x62, 0x61, 0x73, 0x65, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x26, 0x0a, 0x10, 0x43, 0x50, 0x32, 0x50, 0x5f, 0x54, 0x65, 0x78, 0x74, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x22, 0x36, 0x0a, 0x15, 0x43, 0x53, 0x74, 0x65,
    0x61, 0x6d, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x5f, 0x45, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e,
    0x67, 0x12, 0x1d, 0x0a, 0x0a, 0x76, 0x6f, 0x69, 0x63, 0x65, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x76, 0x6f, 0x69, 0x63, 0x65, 0x44, 0x61, 0x74, 0x61,
    0x22, 0x84, 0x01, 0x0a, 0x0a, 0x43, 0x50, 0x32, 0x50, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x12,
    0x2a, 0x0a, 0x05, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x41,
    0x75, 0x64, 0x69, 0x6f, 0x52, 0x05, 0x61, 0x75, 0x64, 0x69, 0x6f, 0x12, 0x27, 0x0a, 0x0f, 0x62,
    0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x47,
    0x72, 0x6f, 0x75, 0x70, 0x22, 0x21, 0x0a, 0x0d, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x5f,
    0x46, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x10, 0x0a, 0x0c, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x64, 0x5f,
    0x41, 0x75, 0x64, 0x69, 0x6f, 0x10, 0x01, 0x22, 0x43, 0x0a, 0x09, 0x43, 0x50, 0x32, 0x50, 0x5f,
    0x50, 0x69, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x52, 0x08, 0x73, 0x65, 0x6e, 0x64, 0x54, 0x69, 0x6d,
    0x65, 0x12, 0x19, 0x0a, 0x08, 0x69, 0x73, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x08, 0x52, 0x07, 0x69, 0x73, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x22, 0x8b, 0x02, 0x0a,
    0x15, 0x43, 0x50, 0x32, 0x50, 0x5f, 0x56, 0x52, 0x41, 0x76, 0x61, 0x74, 0x61, 0x72, 0x50, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x47, 0x0a, 0x0a, 0x62, 0x6f, 0x64, 0x79, 0x5f, 0x70,
    0x61, 0x72, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x50, 0x32, 0x50, 0x5f, 0x56, 0x52, 0x41, 0x76, 0x61, 0x74, 0x61, 0x72, 0x50,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x43, 0x4f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x09, 0x62, 0x6f, 0x64, 0x79, 0x50, 0x61, 0x72, 0x74, 0x73, 0x12,
    0x15, 0x0a, 0x06, 0x68, 0x61, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x05, 0x68, 0x61, 0x74, 0x49, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x63, 0x65, 0x6e, 0x65, 0x5f,
    0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x73, 0x63, 0x65, 0x6e, 0x65, 0x49,
    0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x73, 0x63, 0x61, 0x6c, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x63, 0x61,
    0x6c, 0x65, 0x1a, 0x56, 0x0a, 0x0c, 0x43, 0x4f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x22, 0x0a, 0x03, 0x70, 0x6f, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x52, 0x03, 0x70, 0x6f, 0x73, 0x12, 0x22, 0x0a, 0x03, 0x61, 0x6e, 0x67, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51,
    0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x03, 0x61, 0x6e, 0x67, 0x22, 0xa7, 0x03, 0x0a, 0x19, 0x43,
    0x50, 0x32, 0x50, 0x5f, 0x57, 0x61, 0x74, 0x63, 0x68, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f,
    0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x65, 0x6d, 0x6f,
    0x5f, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x64, 0x65, 0x6d,
    0x6f, 0x54, 0x69, 0x63, 0x6b, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x61, 0x75, 0x73, 0x65, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x70, 0x61, 0x75, 0x73, 0x65, 0x64, 0x12, 0x35, 0x0a,
    0x17, 0x74, 0x76, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x5f, 0x76, 0x6f, 0x69, 0x63, 0x65,
    0x5f, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x14,
    0x74, 0x76, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x49, 0x6e, 0x64,
    0x69, 0x63, 0x65, 0x73, 0x12, 0x2e, 0x0a, 0x13, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x73, 0x70, 0x65,
    0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x11, 0x64, 0x6f, 0x74, 0x61, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72,
    0x4d, 0x6f, 0x64, 0x65, 0x12, 0x4d, 0x0a, 0x23, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x73, 0x70, 0x65,
    0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x77, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x5f,
    0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x20, 0x64, 0x6f, 0x74, 0x61, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72,
    0x57, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x12, 0x39, 0x0a, 0x19, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x73, 0x70, 0x65, 0x63,
    0x74, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x16, 0x64, 0x6f, 0x74, 0x61, 0x53, 0x70, 0x65, 0x63,
    0x74, 0x61, 0x74, 0x6f, 0x72, 0x48, 0x65, 0x72, 0x6f, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x38,
    0x0a, 0x18, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x73, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72,
    0x5f, 0x61, 0x75, 0x74, 0x6f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x16, 0x64, 0x6f, 0x74, 0x61, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x41,
    0x75, 0x74, 0x6f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x12, 0x2a, 0x0a, 0x11, 0x64, 0x6f, 0x74, 0x61,
    0x5f, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x5f, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x0f, 0x64, 0x6f, 0x74, 0x61, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x53,
    0x70, 0x65, 0x65, 0x64, 0x2a, 0x7d, 0x0a, 0x0c, 0x50, 0x32, 0x50, 0x5f, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x12, 0x14, 0x0a, 0x0f, 0x70, 0x32, 0x70, 0x5f, 0x54, 0x65, 0x78, 0x74,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x10, 0x80, 0x02, 0x12, 0x0e, 0x0a, 0x09, 0x70, 0x32,
    0x70, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x10, 0x81, 0x02, 0x12, 0x0d, 0x0a, 0x08, 0x70, 0x32,
    0x70, 0x5f, 0x50, 0x69, 0x6e, 0x67, 0x10, 0x82, 0x02, 0x12, 0x19, 0x0a, 0x14, 0x70, 0x32, 0x70,
    0x5f, 0x56, 0x52, 0x41, 0x76, 0x61, 0x74, 0x61, 0x72, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x10, 0x83, 0x02, 0x12, 0x1d, 0x0a, 0x18, 0x70, 0x32, 0x70, 0x5f, 0x57, 0x61, 0x74, 0x63,
    0x68, 0x53, 0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x10, 0x84, 0x02, 0x42, 0x03, 0x80, 0x01, 0x00, 0x4a, 0xd8, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x3c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04,
    0x07, 0x1a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x07, 0x1f, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x07, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x07,
    0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07,
    0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x07,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x07, 0x1d, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x00, 0x01, 0x12, 0x03, 0x09, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x0a, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x1a, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0c, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0c,
    0x13, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0d, 0x1f, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x0e, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x0e, 0x23, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x11, 0x00, 0x13, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x11, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x12, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x12, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x12, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x1e,
    0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x15, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x16, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x16, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x17, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x24, 0x25, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x19, 0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x19, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x1a, 0x08, 0x1c, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1a,
    0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x10,
    0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x10,
    0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x08, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1e, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x20, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1e, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x1f, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1f, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1f, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x18, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x03, 0x12, 0x04, 0x22, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x22, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x23, 0x08, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x23, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x24, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x24, 0x11,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x24, 0x16, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x24, 0x21, 0x22, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x27, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x27, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x28,
    0x08, 0x2b, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x28, 0x10,
    0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x10, 0x2c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x19, 0x23,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x24, 0x27,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x2a, 0x2b,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x10, 0x2c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2a, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2a, 0x19, 0x23, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x24, 0x27, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x08, 0x43, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x2d, 0x11, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x2d, 0x34, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x2d, 0x41, 0x42, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x08,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x02, 0x12, 0x03, 0x2f, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x2f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2f,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x17, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x30, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x30, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x30, 0x17, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x30, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x33, 0x00, 0x3c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x33, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x00, 0x12, 0x03, 0x34, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x34, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x34, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34,
    0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x34, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x35, 0x08, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x35, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x35, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x35, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x35, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x36,
    0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x36, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x36, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x36, 0x17, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x36, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x03, 0x12, 0x03, 0x37, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x37, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x37, 0x17,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x37, 0x2d, 0x2e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x38, 0x08, 0x3f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x04, 0x04, 0x12, 0x03, 0x38, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x38, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x38, 0x17, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x38, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x03, 0x39, 0x08,
    0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x03, 0x39, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x03, 0x39, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x39, 0x17, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x39, 0x33, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x06, 0x12, 0x03, 0x3a, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x05, 0x12, 0x03, 0x3a,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x01, 0x12, 0x03, 0x3a, 0x17, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03, 0x12, 0x03, 0x3a, 0x32, 0x33, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x07, 0x12, 0x03, 0x3b, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x07, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x3b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x3b, 0x17, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x3b, 0x2b, 0x2c,
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
