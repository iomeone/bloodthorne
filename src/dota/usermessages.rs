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
pub struct CUserMessageAchievementEvent {
    // message fields
    achievement: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageAchievementEvent {}

impl CUserMessageAchievementEvent {
    pub fn new() -> CUserMessageAchievementEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageAchievementEvent {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageAchievementEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageAchievementEvent,
        };
        unsafe {
            instance.get(CUserMessageAchievementEvent::new)
        }
    }

    // optional uint32 achievement = 1;

    pub fn clear_achievement(&mut self) {
        self.achievement = ::std::option::Option::None;
    }

    pub fn has_achievement(&self) -> bool {
        self.achievement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_achievement(&mut self, v: u32) {
        self.achievement = ::std::option::Option::Some(v);
    }

    pub fn get_achievement(&self) -> u32 {
        self.achievement.unwrap_or(0)
    }

    fn get_achievement_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.achievement
    }

    fn mut_achievement_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.achievement
    }
}

impl ::protobuf::Message for CUserMessageAchievementEvent {
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
                    self.achievement = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.achievement {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.achievement {
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

impl ::protobuf::MessageStatic for CUserMessageAchievementEvent {
    fn new() -> CUserMessageAchievementEvent {
        CUserMessageAchievementEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageAchievementEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "achievement",
                    CUserMessageAchievementEvent::get_achievement_for_reflect,
                    CUserMessageAchievementEvent::mut_achievement_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageAchievementEvent>(
                    "CUserMessageAchievementEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageAchievementEvent {
    fn clear(&mut self) {
        self.clear_achievement();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageAchievementEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageAchievementEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageCloseCaption {
    // message fields
    hash: ::std::option::Option<u32>,
    duration: ::std::option::Option<f32>,
    from_player: ::std::option::Option<bool>,
    ent_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageCloseCaption {}

impl CUserMessageCloseCaption {
    pub fn new() -> CUserMessageCloseCaption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageCloseCaption {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageCloseCaption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageCloseCaption,
        };
        unsafe {
            instance.get(CUserMessageCloseCaption::new)
        }
    }

    // optional fixed32 hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u32) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> u32 {
        self.hash.unwrap_or(0)
    }

    fn get_hash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hash
    }

    // optional float duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }

    // optional bool from_player = 3;

    pub fn clear_from_player(&mut self) {
        self.from_player = ::std::option::Option::None;
    }

    pub fn has_from_player(&self) -> bool {
        self.from_player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_player(&mut self, v: bool) {
        self.from_player = ::std::option::Option::Some(v);
    }

    pub fn get_from_player(&self) -> bool {
        self.from_player.unwrap_or(false)
    }

    fn get_from_player_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.from_player
    }

    fn mut_from_player_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.from_player
    }

    // optional int32 ent_index = 4;

    pub fn clear_ent_index(&mut self) {
        self.ent_index = ::std::option::Option::None;
    }

    pub fn has_ent_index(&self) -> bool {
        self.ent_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_index(&mut self, v: i32) {
        self.ent_index = ::std::option::Option::Some(v);
    }

    pub fn get_ent_index(&self) -> i32 {
        self.ent_index.unwrap_or(0)
    }

    fn get_ent_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ent_index
    }

    fn mut_ent_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ent_index
    }
}

impl ::protobuf::Message for CUserMessageCloseCaption {
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
                    self.hash = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.from_player = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ent_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.hash {
            my_size += 5;
        };
        if let Some(v) = self.duration {
            my_size += 5;
        };
        if let Some(v) = self.from_player {
            my_size += 2;
        };
        if let Some(v) = self.ent_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.duration {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.from_player {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.ent_index {
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

impl ::protobuf::MessageStatic for CUserMessageCloseCaption {
    fn new() -> CUserMessageCloseCaption {
        CUserMessageCloseCaption::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageCloseCaption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "hash",
                    CUserMessageCloseCaption::get_hash_for_reflect,
                    CUserMessageCloseCaption::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CUserMessageCloseCaption::get_duration_for_reflect,
                    CUserMessageCloseCaption::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "from_player",
                    CUserMessageCloseCaption::get_from_player_for_reflect,
                    CUserMessageCloseCaption::mut_from_player_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ent_index",
                    CUserMessageCloseCaption::get_ent_index_for_reflect,
                    CUserMessageCloseCaption::mut_ent_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageCloseCaption>(
                    "CUserMessageCloseCaption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageCloseCaption {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_duration();
        self.clear_from_player();
        self.clear_ent_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageCloseCaption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageCloseCaption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageCloseCaptionDirect {
    // message fields
    hash: ::std::option::Option<u32>,
    duration: ::std::option::Option<f32>,
    from_player: ::std::option::Option<bool>,
    ent_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageCloseCaptionDirect {}

impl CUserMessageCloseCaptionDirect {
    pub fn new() -> CUserMessageCloseCaptionDirect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageCloseCaptionDirect {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageCloseCaptionDirect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageCloseCaptionDirect,
        };
        unsafe {
            instance.get(CUserMessageCloseCaptionDirect::new)
        }
    }

    // optional fixed32 hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u32) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> u32 {
        self.hash.unwrap_or(0)
    }

    fn get_hash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hash
    }

    // optional float duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }

    // optional bool from_player = 3;

    pub fn clear_from_player(&mut self) {
        self.from_player = ::std::option::Option::None;
    }

    pub fn has_from_player(&self) -> bool {
        self.from_player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_player(&mut self, v: bool) {
        self.from_player = ::std::option::Option::Some(v);
    }

    pub fn get_from_player(&self) -> bool {
        self.from_player.unwrap_or(false)
    }

    fn get_from_player_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.from_player
    }

    fn mut_from_player_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.from_player
    }

    // optional int32 ent_index = 4;

    pub fn clear_ent_index(&mut self) {
        self.ent_index = ::std::option::Option::None;
    }

    pub fn has_ent_index(&self) -> bool {
        self.ent_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_index(&mut self, v: i32) {
        self.ent_index = ::std::option::Option::Some(v);
    }

    pub fn get_ent_index(&self) -> i32 {
        self.ent_index.unwrap_or(0)
    }

    fn get_ent_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ent_index
    }

    fn mut_ent_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ent_index
    }
}

impl ::protobuf::Message for CUserMessageCloseCaptionDirect {
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
                    self.hash = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.from_player = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ent_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.hash {
            my_size += 5;
        };
        if let Some(v) = self.duration {
            my_size += 5;
        };
        if let Some(v) = self.from_player {
            my_size += 2;
        };
        if let Some(v) = self.ent_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.duration {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.from_player {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.ent_index {
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

impl ::protobuf::MessageStatic for CUserMessageCloseCaptionDirect {
    fn new() -> CUserMessageCloseCaptionDirect {
        CUserMessageCloseCaptionDirect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageCloseCaptionDirect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "hash",
                    CUserMessageCloseCaptionDirect::get_hash_for_reflect,
                    CUserMessageCloseCaptionDirect::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CUserMessageCloseCaptionDirect::get_duration_for_reflect,
                    CUserMessageCloseCaptionDirect::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "from_player",
                    CUserMessageCloseCaptionDirect::get_from_player_for_reflect,
                    CUserMessageCloseCaptionDirect::mut_from_player_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ent_index",
                    CUserMessageCloseCaptionDirect::get_ent_index_for_reflect,
                    CUserMessageCloseCaptionDirect::mut_ent_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageCloseCaptionDirect>(
                    "CUserMessageCloseCaptionDirect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageCloseCaptionDirect {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_duration();
        self.clear_from_player();
        self.clear_ent_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageCloseCaptionDirect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageCloseCaptionDirect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageCloseCaptionPlaceholder {
    // message fields
    string: ::protobuf::SingularField<::std::string::String>,
    duration: ::std::option::Option<f32>,
    from_player: ::std::option::Option<bool>,
    ent_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageCloseCaptionPlaceholder {}

impl CUserMessageCloseCaptionPlaceholder {
    pub fn new() -> CUserMessageCloseCaptionPlaceholder {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageCloseCaptionPlaceholder {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageCloseCaptionPlaceholder> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageCloseCaptionPlaceholder,
        };
        unsafe {
            instance.get(CUserMessageCloseCaptionPlaceholder::new)
        }
    }

    // optional string string = 1;

    pub fn clear_string(&mut self) {
        self.string.clear();
    }

    pub fn has_string(&self) -> bool {
        self.string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string(&mut self, v: ::std::string::String) {
        self.string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string(&mut self) -> &mut ::std::string::String {
        if self.string.is_none() {
            self.string.set_default();
        };
        self.string.as_mut().unwrap()
    }

    // Take field
    pub fn take_string(&mut self) -> ::std::string::String {
        self.string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_string(&self) -> &str {
        match self.string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_string_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.string
    }

    fn mut_string_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.string
    }

    // optional float duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }

    // optional bool from_player = 3;

    pub fn clear_from_player(&mut self) {
        self.from_player = ::std::option::Option::None;
    }

    pub fn has_from_player(&self) -> bool {
        self.from_player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_player(&mut self, v: bool) {
        self.from_player = ::std::option::Option::Some(v);
    }

    pub fn get_from_player(&self) -> bool {
        self.from_player.unwrap_or(false)
    }

    fn get_from_player_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.from_player
    }

    fn mut_from_player_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.from_player
    }

    // optional int32 ent_index = 4;

    pub fn clear_ent_index(&mut self) {
        self.ent_index = ::std::option::Option::None;
    }

    pub fn has_ent_index(&self) -> bool {
        self.ent_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_index(&mut self, v: i32) {
        self.ent_index = ::std::option::Option::Some(v);
    }

    pub fn get_ent_index(&self) -> i32 {
        self.ent_index.unwrap_or(0)
    }

    fn get_ent_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ent_index
    }

    fn mut_ent_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ent_index
    }
}

impl ::protobuf::Message for CUserMessageCloseCaptionPlaceholder {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.string)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.from_player = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ent_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.string.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.duration {
            my_size += 5;
        };
        if let Some(v) = self.from_player {
            my_size += 2;
        };
        if let Some(v) = self.ent_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.string.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.duration {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.from_player {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.ent_index {
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

impl ::protobuf::MessageStatic for CUserMessageCloseCaptionPlaceholder {
    fn new() -> CUserMessageCloseCaptionPlaceholder {
        CUserMessageCloseCaptionPlaceholder::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageCloseCaptionPlaceholder>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string",
                    CUserMessageCloseCaptionPlaceholder::get_string_for_reflect,
                    CUserMessageCloseCaptionPlaceholder::mut_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CUserMessageCloseCaptionPlaceholder::get_duration_for_reflect,
                    CUserMessageCloseCaptionPlaceholder::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "from_player",
                    CUserMessageCloseCaptionPlaceholder::get_from_player_for_reflect,
                    CUserMessageCloseCaptionPlaceholder::mut_from_player_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ent_index",
                    CUserMessageCloseCaptionPlaceholder::get_ent_index_for_reflect,
                    CUserMessageCloseCaptionPlaceholder::mut_ent_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageCloseCaptionPlaceholder>(
                    "CUserMessageCloseCaptionPlaceholder",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageCloseCaptionPlaceholder {
    fn clear(&mut self) {
        self.clear_string();
        self.clear_duration();
        self.clear_from_player();
        self.clear_ent_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageCloseCaptionPlaceholder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageCloseCaptionPlaceholder {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageCurrentTimescale {
    // message fields
    current: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageCurrentTimescale {}

impl CUserMessageCurrentTimescale {
    pub fn new() -> CUserMessageCurrentTimescale {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageCurrentTimescale {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageCurrentTimescale> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageCurrentTimescale,
        };
        unsafe {
            instance.get(CUserMessageCurrentTimescale::new)
        }
    }

    // optional float current = 1;

    pub fn clear_current(&mut self) {
        self.current = ::std::option::Option::None;
    }

    pub fn has_current(&self) -> bool {
        self.current.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current(&mut self, v: f32) {
        self.current = ::std::option::Option::Some(v);
    }

    pub fn get_current(&self) -> f32 {
        self.current.unwrap_or(0.)
    }

    fn get_current_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.current
    }

    fn mut_current_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.current
    }
}

impl ::protobuf::Message for CUserMessageCurrentTimescale {
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
                    self.current = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.current {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.current {
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

impl ::protobuf::MessageStatic for CUserMessageCurrentTimescale {
    fn new() -> CUserMessageCurrentTimescale {
        CUserMessageCurrentTimescale::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageCurrentTimescale>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "current",
                    CUserMessageCurrentTimescale::get_current_for_reflect,
                    CUserMessageCurrentTimescale::mut_current_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageCurrentTimescale>(
                    "CUserMessageCurrentTimescale",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageCurrentTimescale {
    fn clear(&mut self) {
        self.clear_current();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageCurrentTimescale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageCurrentTimescale {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageDesiredTimescale {
    // message fields
    desired: ::std::option::Option<f32>,
    acceleration: ::std::option::Option<f32>,
    minblendrate: ::std::option::Option<f32>,
    blenddeltamultiplier: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageDesiredTimescale {}

impl CUserMessageDesiredTimescale {
    pub fn new() -> CUserMessageDesiredTimescale {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageDesiredTimescale {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageDesiredTimescale> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageDesiredTimescale,
        };
        unsafe {
            instance.get(CUserMessageDesiredTimescale::new)
        }
    }

    // optional float desired = 1;

    pub fn clear_desired(&mut self) {
        self.desired = ::std::option::Option::None;
    }

    pub fn has_desired(&self) -> bool {
        self.desired.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desired(&mut self, v: f32) {
        self.desired = ::std::option::Option::Some(v);
    }

    pub fn get_desired(&self) -> f32 {
        self.desired.unwrap_or(0.)
    }

    fn get_desired_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.desired
    }

    fn mut_desired_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.desired
    }

    // optional float acceleration = 2;

    pub fn clear_acceleration(&mut self) {
        self.acceleration = ::std::option::Option::None;
    }

    pub fn has_acceleration(&self) -> bool {
        self.acceleration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_acceleration(&mut self, v: f32) {
        self.acceleration = ::std::option::Option::Some(v);
    }

    pub fn get_acceleration(&self) -> f32 {
        self.acceleration.unwrap_or(0.)
    }

    fn get_acceleration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.acceleration
    }

    fn mut_acceleration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.acceleration
    }

    // optional float minblendrate = 3;

    pub fn clear_minblendrate(&mut self) {
        self.minblendrate = ::std::option::Option::None;
    }

    pub fn has_minblendrate(&self) -> bool {
        self.minblendrate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minblendrate(&mut self, v: f32) {
        self.minblendrate = ::std::option::Option::Some(v);
    }

    pub fn get_minblendrate(&self) -> f32 {
        self.minblendrate.unwrap_or(0.)
    }

    fn get_minblendrate_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.minblendrate
    }

    fn mut_minblendrate_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.minblendrate
    }

    // optional float blenddeltamultiplier = 4;

    pub fn clear_blenddeltamultiplier(&mut self) {
        self.blenddeltamultiplier = ::std::option::Option::None;
    }

    pub fn has_blenddeltamultiplier(&self) -> bool {
        self.blenddeltamultiplier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blenddeltamultiplier(&mut self, v: f32) {
        self.blenddeltamultiplier = ::std::option::Option::Some(v);
    }

    pub fn get_blenddeltamultiplier(&self) -> f32 {
        self.blenddeltamultiplier.unwrap_or(0.)
    }

    fn get_blenddeltamultiplier_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.blenddeltamultiplier
    }

    fn mut_blenddeltamultiplier_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.blenddeltamultiplier
    }
}

impl ::protobuf::Message for CUserMessageDesiredTimescale {
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
                    self.desired = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.acceleration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.minblendrate = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.blenddeltamultiplier = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.desired {
            my_size += 5;
        };
        if let Some(v) = self.acceleration {
            my_size += 5;
        };
        if let Some(v) = self.minblendrate {
            my_size += 5;
        };
        if let Some(v) = self.blenddeltamultiplier {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.desired {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.acceleration {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.minblendrate {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.blenddeltamultiplier {
            os.write_float(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMessageDesiredTimescale {
    fn new() -> CUserMessageDesiredTimescale {
        CUserMessageDesiredTimescale::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageDesiredTimescale>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "desired",
                    CUserMessageDesiredTimescale::get_desired_for_reflect,
                    CUserMessageDesiredTimescale::mut_desired_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "acceleration",
                    CUserMessageDesiredTimescale::get_acceleration_for_reflect,
                    CUserMessageDesiredTimescale::mut_acceleration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "minblendrate",
                    CUserMessageDesiredTimescale::get_minblendrate_for_reflect,
                    CUserMessageDesiredTimescale::mut_minblendrate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "blenddeltamultiplier",
                    CUserMessageDesiredTimescale::get_blenddeltamultiplier_for_reflect,
                    CUserMessageDesiredTimescale::mut_blenddeltamultiplier_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageDesiredTimescale>(
                    "CUserMessageDesiredTimescale",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageDesiredTimescale {
    fn clear(&mut self) {
        self.clear_desired();
        self.clear_acceleration();
        self.clear_minblendrate();
        self.clear_blenddeltamultiplier();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageDesiredTimescale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageDesiredTimescale {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageFade {
    // message fields
    duration: ::std::option::Option<u32>,
    hold_time: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    color: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageFade {}

impl CUserMessageFade {
    pub fn new() -> CUserMessageFade {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageFade {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageFade> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageFade,
        };
        unsafe {
            instance.get(CUserMessageFade::new)
        }
    }

    // optional uint32 duration = 1;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: u32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> u32 {
        self.duration.unwrap_or(0)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.duration
    }

    // optional uint32 hold_time = 2;

    pub fn clear_hold_time(&mut self) {
        self.hold_time = ::std::option::Option::None;
    }

    pub fn has_hold_time(&self) -> bool {
        self.hold_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hold_time(&mut self, v: u32) {
        self.hold_time = ::std::option::Option::Some(v);
    }

    pub fn get_hold_time(&self) -> u32 {
        self.hold_time.unwrap_or(0)
    }

    fn get_hold_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hold_time
    }

    fn mut_hold_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hold_time
    }

    // optional uint32 flags = 3;

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

    // optional fixed32 color = 4;

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
}

impl ::protobuf::Message for CUserMessageFade {
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
                    self.duration = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hold_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.color = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.duration {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hold_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.color {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.duration {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.hold_time {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.color {
            os.write_fixed32(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMessageFade {
    fn new() -> CUserMessageFade {
        CUserMessageFade::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageFade>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "duration",
                    CUserMessageFade::get_duration_for_reflect,
                    CUserMessageFade::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hold_time",
                    CUserMessageFade::get_hold_time_for_reflect,
                    CUserMessageFade::mut_hold_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CUserMessageFade::get_flags_for_reflect,
                    CUserMessageFade::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "color",
                    CUserMessageFade::get_color_for_reflect,
                    CUserMessageFade::mut_color_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageFade>(
                    "CUserMessageFade",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageFade {
    fn clear(&mut self) {
        self.clear_duration();
        self.clear_hold_time();
        self.clear_flags();
        self.clear_color();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageFade {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageFade {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageShake {
    // message fields
    command: ::std::option::Option<u32>,
    amplitude: ::std::option::Option<f32>,
    frequency: ::std::option::Option<f32>,
    duration: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageShake {}

impl CUserMessageShake {
    pub fn new() -> CUserMessageShake {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageShake {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageShake> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageShake,
        };
        unsafe {
            instance.get(CUserMessageShake::new)
        }
    }

    // optional uint32 command = 1;

    pub fn clear_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: u32) {
        self.command = ::std::option::Option::Some(v);
    }

    pub fn get_command(&self) -> u32 {
        self.command.unwrap_or(0)
    }

    fn get_command_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.command
    }

    // optional float amplitude = 2;

    pub fn clear_amplitude(&mut self) {
        self.amplitude = ::std::option::Option::None;
    }

    pub fn has_amplitude(&self) -> bool {
        self.amplitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amplitude(&mut self, v: f32) {
        self.amplitude = ::std::option::Option::Some(v);
    }

    pub fn get_amplitude(&self) -> f32 {
        self.amplitude.unwrap_or(0.)
    }

    fn get_amplitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.amplitude
    }

    fn mut_amplitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.amplitude
    }

    // optional float frequency = 3;

    pub fn clear_frequency(&mut self) {
        self.frequency = ::std::option::Option::None;
    }

    pub fn has_frequency(&self) -> bool {
        self.frequency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frequency(&mut self, v: f32) {
        self.frequency = ::std::option::Option::Some(v);
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency.unwrap_or(0.)
    }

    fn get_frequency_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.frequency
    }

    fn mut_frequency_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.frequency
    }

    // optional float duration = 4;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }
}

impl ::protobuf::Message for CUserMessageShake {
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
                    self.command = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.amplitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.frequency = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.command {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.amplitude {
            my_size += 5;
        };
        if let Some(v) = self.frequency {
            my_size += 5;
        };
        if let Some(v) = self.duration {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.command {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.amplitude {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.frequency {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.duration {
            os.write_float(4, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMessageShake {
    fn new() -> CUserMessageShake {
        CUserMessageShake::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageShake>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "command",
                    CUserMessageShake::get_command_for_reflect,
                    CUserMessageShake::mut_command_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "amplitude",
                    CUserMessageShake::get_amplitude_for_reflect,
                    CUserMessageShake::mut_amplitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "frequency",
                    CUserMessageShake::get_frequency_for_reflect,
                    CUserMessageShake::mut_frequency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CUserMessageShake::get_duration_for_reflect,
                    CUserMessageShake::mut_duration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageShake>(
                    "CUserMessageShake",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageShake {
    fn clear(&mut self) {
        self.clear_command();
        self.clear_amplitude();
        self.clear_frequency();
        self.clear_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageShake {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageShake {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageShakeDir {
    // message fields
    shake: ::protobuf::SingularPtrField<CUserMessageShake>,
    direction: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageShakeDir {}

impl CUserMessageShakeDir {
    pub fn new() -> CUserMessageShakeDir {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageShakeDir {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageShakeDir> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageShakeDir,
        };
        unsafe {
            instance.get(CUserMessageShakeDir::new)
        }
    }

    // optional .dota.CUserMessageShake shake = 1;

    pub fn clear_shake(&mut self) {
        self.shake.clear();
    }

    pub fn has_shake(&self) -> bool {
        self.shake.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shake(&mut self, v: CUserMessageShake) {
        self.shake = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shake(&mut self) -> &mut CUserMessageShake {
        if self.shake.is_none() {
            self.shake.set_default();
        };
        self.shake.as_mut().unwrap()
    }

    // Take field
    pub fn take_shake(&mut self) -> CUserMessageShake {
        self.shake.take().unwrap_or_else(|| CUserMessageShake::new())
    }

    pub fn get_shake(&self) -> &CUserMessageShake {
        self.shake.as_ref().unwrap_or_else(|| CUserMessageShake::default_instance())
    }

    fn get_shake_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMessageShake> {
        &self.shake
    }

    fn mut_shake_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMessageShake> {
        &mut self.shake
    }

    // optional .dota.CMsgVector direction = 2;

    pub fn clear_direction(&mut self) {
        self.direction.clear();
    }

    pub fn has_direction(&self) -> bool {
        self.direction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_direction(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.direction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_direction(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.direction.is_none() {
            self.direction.set_default();
        };
        self.direction.as_mut().unwrap()
    }

    // Take field
    pub fn take_direction(&mut self) -> super::networkbasetypes::CMsgVector {
        self.direction.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_direction(&self) -> &super::networkbasetypes::CMsgVector {
        self.direction.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_direction_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.direction
    }

    fn mut_direction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.direction
    }
}

impl ::protobuf::Message for CUserMessageShakeDir {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shake)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.direction)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.shake.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.direction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.shake.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.direction.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageShakeDir {
    fn new() -> CUserMessageShakeDir {
        CUserMessageShakeDir::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageShakeDir>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMessageShake>>(
                    "shake",
                    CUserMessageShakeDir::get_shake_for_reflect,
                    CUserMessageShakeDir::mut_shake_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "direction",
                    CUserMessageShakeDir::get_direction_for_reflect,
                    CUserMessageShakeDir::mut_direction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageShakeDir>(
                    "CUserMessageShakeDir",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageShakeDir {
    fn clear(&mut self) {
        self.clear_shake();
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageShakeDir {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageShakeDir {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageScreenTilt {
    // message fields
    command: ::std::option::Option<u32>,
    ease_in_out: ::std::option::Option<bool>,
    angle: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    duration: ::std::option::Option<f32>,
    time: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageScreenTilt {}

impl CUserMessageScreenTilt {
    pub fn new() -> CUserMessageScreenTilt {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageScreenTilt {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageScreenTilt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageScreenTilt,
        };
        unsafe {
            instance.get(CUserMessageScreenTilt::new)
        }
    }

    // optional uint32 command = 1;

    pub fn clear_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: u32) {
        self.command = ::std::option::Option::Some(v);
    }

    pub fn get_command(&self) -> u32 {
        self.command.unwrap_or(0)
    }

    fn get_command_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.command
    }

    // optional bool ease_in_out = 2;

    pub fn clear_ease_in_out(&mut self) {
        self.ease_in_out = ::std::option::Option::None;
    }

    pub fn has_ease_in_out(&self) -> bool {
        self.ease_in_out.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ease_in_out(&mut self, v: bool) {
        self.ease_in_out = ::std::option::Option::Some(v);
    }

    pub fn get_ease_in_out(&self) -> bool {
        self.ease_in_out.unwrap_or(false)
    }

    fn get_ease_in_out_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ease_in_out
    }

    fn mut_ease_in_out_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ease_in_out
    }

    // optional .dota.CMsgVector angle = 3;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angle(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.angle.is_none() {
            self.angle.set_default();
        };
        self.angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_angle(&mut self) -> super::networkbasetypes::CMsgVector {
        self.angle.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_angle(&self) -> &super::networkbasetypes::CMsgVector {
        self.angle.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_angle_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.angle
    }

    // optional float duration = 4;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }

    // optional float time = 5;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: f32) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time(&self) -> f32 {
        self.time.unwrap_or(0.)
    }

    fn get_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.time
    }
}

impl ::protobuf::Message for CUserMessageScreenTilt {
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
                    self.command = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.ease_in_out = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angle)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.time = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.command {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ease_in_out {
            my_size += 2;
        };
        if let Some(v) = self.angle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.duration {
            my_size += 5;
        };
        if let Some(v) = self.time {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.command {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.ease_in_out {
            os.write_bool(2, v)?;
        };
        if let Some(v) = self.angle.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.duration {
            os.write_float(4, v)?;
        };
        if let Some(v) = self.time {
            os.write_float(5, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMessageScreenTilt {
    fn new() -> CUserMessageScreenTilt {
        CUserMessageScreenTilt::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageScreenTilt>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "command",
                    CUserMessageScreenTilt::get_command_for_reflect,
                    CUserMessageScreenTilt::mut_command_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ease_in_out",
                    CUserMessageScreenTilt::get_ease_in_out_for_reflect,
                    CUserMessageScreenTilt::mut_ease_in_out_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "angle",
                    CUserMessageScreenTilt::get_angle_for_reflect,
                    CUserMessageScreenTilt::mut_angle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CUserMessageScreenTilt::get_duration_for_reflect,
                    CUserMessageScreenTilt::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "time",
                    CUserMessageScreenTilt::get_time_for_reflect,
                    CUserMessageScreenTilt::mut_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageScreenTilt>(
                    "CUserMessageScreenTilt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageScreenTilt {
    fn clear(&mut self) {
        self.clear_command();
        self.clear_ease_in_out();
        self.clear_angle();
        self.clear_duration();
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageScreenTilt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageScreenTilt {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageSayText {
    // message fields
    playerindex: ::std::option::Option<u32>,
    text: ::protobuf::SingularField<::std::string::String>,
    chat: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageSayText {}

impl CUserMessageSayText {
    pub fn new() -> CUserMessageSayText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageSayText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageSayText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageSayText,
        };
        unsafe {
            instance.get(CUserMessageSayText::new)
        }
    }

    // optional uint32 playerindex = 1;

    pub fn clear_playerindex(&mut self) {
        self.playerindex = ::std::option::Option::None;
    }

    pub fn has_playerindex(&self) -> bool {
        self.playerindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playerindex(&mut self, v: u32) {
        self.playerindex = ::std::option::Option::Some(v);
    }

    pub fn get_playerindex(&self) -> u32 {
        self.playerindex.unwrap_or(0)
    }

    fn get_playerindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.playerindex
    }

    fn mut_playerindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.playerindex
    }

    // optional string text = 2;

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

    // optional bool chat = 3;

    pub fn clear_chat(&mut self) {
        self.chat = ::std::option::Option::None;
    }

    pub fn has_chat(&self) -> bool {
        self.chat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: bool) {
        self.chat = ::std::option::Option::Some(v);
    }

    pub fn get_chat(&self) -> bool {
        self.chat.unwrap_or(false)
    }

    fn get_chat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.chat
    }

    fn mut_chat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.chat
    }
}

impl ::protobuf::Message for CUserMessageSayText {
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
                    self.playerindex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.chat = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.playerindex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.chat {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.playerindex {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.text.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.chat {
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

impl ::protobuf::MessageStatic for CUserMessageSayText {
    fn new() -> CUserMessageSayText {
        CUserMessageSayText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageSayText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "playerindex",
                    CUserMessageSayText::get_playerindex_for_reflect,
                    CUserMessageSayText::mut_playerindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CUserMessageSayText::get_text_for_reflect,
                    CUserMessageSayText::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "chat",
                    CUserMessageSayText::get_chat_for_reflect,
                    CUserMessageSayText::mut_chat_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageSayText>(
                    "CUserMessageSayText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageSayText {
    fn clear(&mut self) {
        self.clear_playerindex();
        self.clear_text();
        self.clear_chat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageSayText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageSayText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageSayText2 {
    // message fields
    entityindex: ::std::option::Option<u32>,
    chat: ::std::option::Option<bool>,
    messagename: ::protobuf::SingularField<::std::string::String>,
    param1: ::protobuf::SingularField<::std::string::String>,
    param2: ::protobuf::SingularField<::std::string::String>,
    param3: ::protobuf::SingularField<::std::string::String>,
    param4: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageSayText2 {}

impl CUserMessageSayText2 {
    pub fn new() -> CUserMessageSayText2 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageSayText2 {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageSayText2> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageSayText2,
        };
        unsafe {
            instance.get(CUserMessageSayText2::new)
        }
    }

    // optional uint32 entityindex = 1;

    pub fn clear_entityindex(&mut self) {
        self.entityindex = ::std::option::Option::None;
    }

    pub fn has_entityindex(&self) -> bool {
        self.entityindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entityindex(&mut self, v: u32) {
        self.entityindex = ::std::option::Option::Some(v);
    }

    pub fn get_entityindex(&self) -> u32 {
        self.entityindex.unwrap_or(0)
    }

    fn get_entityindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entityindex
    }

    fn mut_entityindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entityindex
    }

    // optional bool chat = 2;

    pub fn clear_chat(&mut self) {
        self.chat = ::std::option::Option::None;
    }

    pub fn has_chat(&self) -> bool {
        self.chat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: bool) {
        self.chat = ::std::option::Option::Some(v);
    }

    pub fn get_chat(&self) -> bool {
        self.chat.unwrap_or(false)
    }

    fn get_chat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.chat
    }

    fn mut_chat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.chat
    }

    // optional string messagename = 3;

    pub fn clear_messagename(&mut self) {
        self.messagename.clear();
    }

    pub fn has_messagename(&self) -> bool {
        self.messagename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_messagename(&mut self, v: ::std::string::String) {
        self.messagename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_messagename(&mut self) -> &mut ::std::string::String {
        if self.messagename.is_none() {
            self.messagename.set_default();
        };
        self.messagename.as_mut().unwrap()
    }

    // Take field
    pub fn take_messagename(&mut self) -> ::std::string::String {
        self.messagename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_messagename(&self) -> &str {
        match self.messagename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_messagename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.messagename
    }

    fn mut_messagename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.messagename
    }

    // optional string param1 = 4;

    pub fn clear_param1(&mut self) {
        self.param1.clear();
    }

    pub fn has_param1(&self) -> bool {
        self.param1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_param1(&mut self, v: ::std::string::String) {
        self.param1 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_param1(&mut self) -> &mut ::std::string::String {
        if self.param1.is_none() {
            self.param1.set_default();
        };
        self.param1.as_mut().unwrap()
    }

    // Take field
    pub fn take_param1(&mut self) -> ::std::string::String {
        self.param1.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_param1(&self) -> &str {
        match self.param1.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_param1_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.param1
    }

    fn mut_param1_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.param1
    }

    // optional string param2 = 5;

    pub fn clear_param2(&mut self) {
        self.param2.clear();
    }

    pub fn has_param2(&self) -> bool {
        self.param2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_param2(&mut self, v: ::std::string::String) {
        self.param2 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_param2(&mut self) -> &mut ::std::string::String {
        if self.param2.is_none() {
            self.param2.set_default();
        };
        self.param2.as_mut().unwrap()
    }

    // Take field
    pub fn take_param2(&mut self) -> ::std::string::String {
        self.param2.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_param2(&self) -> &str {
        match self.param2.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_param2_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.param2
    }

    fn mut_param2_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.param2
    }

    // optional string param3 = 6;

    pub fn clear_param3(&mut self) {
        self.param3.clear();
    }

    pub fn has_param3(&self) -> bool {
        self.param3.is_some()
    }

    // Param is passed by value, moved
    pub fn set_param3(&mut self, v: ::std::string::String) {
        self.param3 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_param3(&mut self) -> &mut ::std::string::String {
        if self.param3.is_none() {
            self.param3.set_default();
        };
        self.param3.as_mut().unwrap()
    }

    // Take field
    pub fn take_param3(&mut self) -> ::std::string::String {
        self.param3.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_param3(&self) -> &str {
        match self.param3.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_param3_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.param3
    }

    fn mut_param3_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.param3
    }

    // optional string param4 = 7;

    pub fn clear_param4(&mut self) {
        self.param4.clear();
    }

    pub fn has_param4(&self) -> bool {
        self.param4.is_some()
    }

    // Param is passed by value, moved
    pub fn set_param4(&mut self, v: ::std::string::String) {
        self.param4 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_param4(&mut self) -> &mut ::std::string::String {
        if self.param4.is_none() {
            self.param4.set_default();
        };
        self.param4.as_mut().unwrap()
    }

    // Take field
    pub fn take_param4(&mut self) -> ::std::string::String {
        self.param4.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_param4(&self) -> &str {
        match self.param4.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_param4_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.param4
    }

    fn mut_param4_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.param4
    }
}

impl ::protobuf::Message for CUserMessageSayText2 {
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
                    self.entityindex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.chat = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.messagename)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.param1)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.param2)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.param3)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.param4)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.entityindex {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.chat {
            my_size += 2;
        };
        if let Some(v) = self.messagename.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.param1.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.param2.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.param3.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.param4.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entityindex {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.chat {
            os.write_bool(2, v)?;
        };
        if let Some(v) = self.messagename.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.param1.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.param2.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.param3.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.param4.as_ref() {
            os.write_string(7, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMessageSayText2 {
    fn new() -> CUserMessageSayText2 {
        CUserMessageSayText2::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageSayText2>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "entityindex",
                    CUserMessageSayText2::get_entityindex_for_reflect,
                    CUserMessageSayText2::mut_entityindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "chat",
                    CUserMessageSayText2::get_chat_for_reflect,
                    CUserMessageSayText2::mut_chat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "messagename",
                    CUserMessageSayText2::get_messagename_for_reflect,
                    CUserMessageSayText2::mut_messagename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "param1",
                    CUserMessageSayText2::get_param1_for_reflect,
                    CUserMessageSayText2::mut_param1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "param2",
                    CUserMessageSayText2::get_param2_for_reflect,
                    CUserMessageSayText2::mut_param2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "param3",
                    CUserMessageSayText2::get_param3_for_reflect,
                    CUserMessageSayText2::mut_param3_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "param4",
                    CUserMessageSayText2::get_param4_for_reflect,
                    CUserMessageSayText2::mut_param4_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageSayText2>(
                    "CUserMessageSayText2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageSayText2 {
    fn clear(&mut self) {
        self.clear_entityindex();
        self.clear_chat();
        self.clear_messagename();
        self.clear_param1();
        self.clear_param2();
        self.clear_param3();
        self.clear_param4();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageSayText2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageSayText2 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageHudMsg {
    // message fields
    channel: ::std::option::Option<u32>,
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    color1: ::std::option::Option<u32>,
    color2: ::std::option::Option<u32>,
    effect: ::std::option::Option<u32>,
    fade_in_time: ::std::option::Option<f32>,
    fade_out_time: ::std::option::Option<f32>,
    hold_time: ::std::option::Option<f32>,
    fx_time: ::std::option::Option<f32>,
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageHudMsg {}

impl CUserMessageHudMsg {
    pub fn new() -> CUserMessageHudMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageHudMsg {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageHudMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageHudMsg,
        };
        unsafe {
            instance.get(CUserMessageHudMsg::new)
        }
    }

    // optional uint32 channel = 1;

    pub fn clear_channel(&mut self) {
        self.channel = ::std::option::Option::None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: u32) {
        self.channel = ::std::option::Option::Some(v);
    }

    pub fn get_channel(&self) -> u32 {
        self.channel.unwrap_or(0)
    }

    fn get_channel_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel
    }

    // optional float x = 2;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.x
    }

    // optional float y = 3;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.y
    }

    // optional fixed32 color1 = 4;

    pub fn clear_color1(&mut self) {
        self.color1 = ::std::option::Option::None;
    }

    pub fn has_color1(&self) -> bool {
        self.color1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color1(&mut self, v: u32) {
        self.color1 = ::std::option::Option::Some(v);
    }

    pub fn get_color1(&self) -> u32 {
        self.color1.unwrap_or(0)
    }

    fn get_color1_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.color1
    }

    fn mut_color1_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.color1
    }

    // optional fixed32 color2 = 5;

    pub fn clear_color2(&mut self) {
        self.color2 = ::std::option::Option::None;
    }

    pub fn has_color2(&self) -> bool {
        self.color2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color2(&mut self, v: u32) {
        self.color2 = ::std::option::Option::Some(v);
    }

    pub fn get_color2(&self) -> u32 {
        self.color2.unwrap_or(0)
    }

    fn get_color2_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.color2
    }

    fn mut_color2_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.color2
    }

    // optional uint32 effect = 6;

    pub fn clear_effect(&mut self) {
        self.effect = ::std::option::Option::None;
    }

    pub fn has_effect(&self) -> bool {
        self.effect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effect(&mut self, v: u32) {
        self.effect = ::std::option::Option::Some(v);
    }

    pub fn get_effect(&self) -> u32 {
        self.effect.unwrap_or(0)
    }

    fn get_effect_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.effect
    }

    fn mut_effect_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.effect
    }

    // optional float fade_in_time = 7;

    pub fn clear_fade_in_time(&mut self) {
        self.fade_in_time = ::std::option::Option::None;
    }

    pub fn has_fade_in_time(&self) -> bool {
        self.fade_in_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_in_time(&mut self, v: f32) {
        self.fade_in_time = ::std::option::Option::Some(v);
    }

    pub fn get_fade_in_time(&self) -> f32 {
        self.fade_in_time.unwrap_or(0.)
    }

    fn get_fade_in_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fade_in_time
    }

    fn mut_fade_in_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fade_in_time
    }

    // optional float fade_out_time = 8;

    pub fn clear_fade_out_time(&mut self) {
        self.fade_out_time = ::std::option::Option::None;
    }

    pub fn has_fade_out_time(&self) -> bool {
        self.fade_out_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_out_time(&mut self, v: f32) {
        self.fade_out_time = ::std::option::Option::Some(v);
    }

    pub fn get_fade_out_time(&self) -> f32 {
        self.fade_out_time.unwrap_or(0.)
    }

    fn get_fade_out_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fade_out_time
    }

    fn mut_fade_out_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fade_out_time
    }

    // optional float hold_time = 9;

    pub fn clear_hold_time(&mut self) {
        self.hold_time = ::std::option::Option::None;
    }

    pub fn has_hold_time(&self) -> bool {
        self.hold_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hold_time(&mut self, v: f32) {
        self.hold_time = ::std::option::Option::Some(v);
    }

    pub fn get_hold_time(&self) -> f32 {
        self.hold_time.unwrap_or(0.)
    }

    fn get_hold_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.hold_time
    }

    fn mut_hold_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.hold_time
    }

    // optional float fx_time = 10;

    pub fn clear_fx_time(&mut self) {
        self.fx_time = ::std::option::Option::None;
    }

    pub fn has_fx_time(&self) -> bool {
        self.fx_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fx_time(&mut self, v: f32) {
        self.fx_time = ::std::option::Option::Some(v);
    }

    pub fn get_fx_time(&self) -> f32 {
        self.fx_time.unwrap_or(0.)
    }

    fn get_fx_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fx_time
    }

    fn mut_fx_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fx_time
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
}

impl ::protobuf::Message for CUserMessageHudMsg {
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
                    self.channel = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.x = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.color1 = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.color2 = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.effect = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.fade_in_time = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.fade_out_time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.hold_time = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.fx_time = ::std::option::Option::Some(tmp);
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.x {
            my_size += 5;
        };
        if let Some(v) = self.y {
            my_size += 5;
        };
        if let Some(v) = self.color1 {
            my_size += 5;
        };
        if let Some(v) = self.color2 {
            my_size += 5;
        };
        if let Some(v) = self.effect {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.fade_in_time {
            my_size += 5;
        };
        if let Some(v) = self.fade_out_time {
            my_size += 5;
        };
        if let Some(v) = self.hold_time {
            my_size += 5;
        };
        if let Some(v) = self.fx_time {
            my_size += 5;
        };
        if let Some(v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.x {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.y {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.color1 {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.color2 {
            os.write_fixed32(5, v)?;
        };
        if let Some(v) = self.effect {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.fade_in_time {
            os.write_float(7, v)?;
        };
        if let Some(v) = self.fade_out_time {
            os.write_float(8, v)?;
        };
        if let Some(v) = self.hold_time {
            os.write_float(9, v)?;
        };
        if let Some(v) = self.fx_time {
            os.write_float(10, v)?;
        };
        if let Some(v) = self.message.as_ref() {
            os.write_string(11, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMessageHudMsg {
    fn new() -> CUserMessageHudMsg {
        CUserMessageHudMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageHudMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel",
                    CUserMessageHudMsg::get_channel_for_reflect,
                    CUserMessageHudMsg::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    CUserMessageHudMsg::get_x_for_reflect,
                    CUserMessageHudMsg::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    CUserMessageHudMsg::get_y_for_reflect,
                    CUserMessageHudMsg::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "color1",
                    CUserMessageHudMsg::get_color1_for_reflect,
                    CUserMessageHudMsg::mut_color1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "color2",
                    CUserMessageHudMsg::get_color2_for_reflect,
                    CUserMessageHudMsg::mut_color2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "effect",
                    CUserMessageHudMsg::get_effect_for_reflect,
                    CUserMessageHudMsg::mut_effect_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fade_in_time",
                    CUserMessageHudMsg::get_fade_in_time_for_reflect,
                    CUserMessageHudMsg::mut_fade_in_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fade_out_time",
                    CUserMessageHudMsg::get_fade_out_time_for_reflect,
                    CUserMessageHudMsg::mut_fade_out_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "hold_time",
                    CUserMessageHudMsg::get_hold_time_for_reflect,
                    CUserMessageHudMsg::mut_hold_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fx_time",
                    CUserMessageHudMsg::get_fx_time_for_reflect,
                    CUserMessageHudMsg::mut_fx_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CUserMessageHudMsg::get_message_for_reflect,
                    CUserMessageHudMsg::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageHudMsg>(
                    "CUserMessageHudMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageHudMsg {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_x();
        self.clear_y();
        self.clear_color1();
        self.clear_color2();
        self.clear_effect();
        self.clear_fade_in_time();
        self.clear_fade_out_time();
        self.clear_hold_time();
        self.clear_fx_time();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageHudMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageHudMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageHudText {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageHudText {}

impl CUserMessageHudText {
    pub fn new() -> CUserMessageHudText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageHudText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageHudText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageHudText,
        };
        unsafe {
            instance.get(CUserMessageHudText::new)
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
}

impl ::protobuf::Message for CUserMessageHudText {
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
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageHudText {
    fn new() -> CUserMessageHudText {
        CUserMessageHudText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageHudText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CUserMessageHudText::get_message_for_reflect,
                    CUserMessageHudText::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageHudText>(
                    "CUserMessageHudText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageHudText {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageHudText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageHudText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageTextMsg {
    // message fields
    dest: ::std::option::Option<u32>,
    param: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageTextMsg {}

impl CUserMessageTextMsg {
    pub fn new() -> CUserMessageTextMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageTextMsg {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageTextMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageTextMsg,
        };
        unsafe {
            instance.get(CUserMessageTextMsg::new)
        }
    }

    // optional uint32 dest = 1;

    pub fn clear_dest(&mut self) {
        self.dest = ::std::option::Option::None;
    }

    pub fn has_dest(&self) -> bool {
        self.dest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dest(&mut self, v: u32) {
        self.dest = ::std::option::Option::Some(v);
    }

    pub fn get_dest(&self) -> u32 {
        self.dest.unwrap_or(0)
    }

    fn get_dest_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dest
    }

    fn mut_dest_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dest
    }

    // repeated string param = 2;

    pub fn clear_param(&mut self) {
        self.param.clear();
    }

    // Param is passed by value, moved
    pub fn set_param(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.param = v;
    }

    // Mutable pointer to the field.
    pub fn mut_param(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.param
    }

    // Take field
    pub fn take_param(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.param, ::protobuf::RepeatedField::new())
    }

    pub fn get_param(&self) -> &[::std::string::String] {
        &self.param
    }

    fn get_param_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.param
    }

    fn mut_param_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.param
    }
}

impl ::protobuf::Message for CUserMessageTextMsg {
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
                    self.dest = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.param)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.dest {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.param {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dest {
            os.write_uint32(1, v)?;
        };
        for v in &self.param {
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

impl ::protobuf::MessageStatic for CUserMessageTextMsg {
    fn new() -> CUserMessageTextMsg {
        CUserMessageTextMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageTextMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dest",
                    CUserMessageTextMsg::get_dest_for_reflect,
                    CUserMessageTextMsg::mut_dest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "param",
                    CUserMessageTextMsg::get_param_for_reflect,
                    CUserMessageTextMsg::mut_param_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageTextMsg>(
                    "CUserMessageTextMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageTextMsg {
    fn clear(&mut self) {
        self.clear_dest();
        self.clear_param();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageTextMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageTextMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageGameTitle {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageGameTitle {}

impl CUserMessageGameTitle {
    pub fn new() -> CUserMessageGameTitle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageGameTitle {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageGameTitle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageGameTitle,
        };
        unsafe {
            instance.get(CUserMessageGameTitle::new)
        }
    }
}

impl ::protobuf::Message for CUserMessageGameTitle {
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

impl ::protobuf::MessageStatic for CUserMessageGameTitle {
    fn new() -> CUserMessageGameTitle {
        CUserMessageGameTitle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageGameTitle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageGameTitle>(
                    "CUserMessageGameTitle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageGameTitle {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageGameTitle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageGameTitle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageResetHUD {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageResetHUD {}

impl CUserMessageResetHUD {
    pub fn new() -> CUserMessageResetHUD {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageResetHUD {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageResetHUD> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageResetHUD,
        };
        unsafe {
            instance.get(CUserMessageResetHUD::new)
        }
    }
}

impl ::protobuf::Message for CUserMessageResetHUD {
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

impl ::protobuf::MessageStatic for CUserMessageResetHUD {
    fn new() -> CUserMessageResetHUD {
        CUserMessageResetHUD::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageResetHUD>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageResetHUD>(
                    "CUserMessageResetHUD",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageResetHUD {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageResetHUD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageResetHUD {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageSendAudio {
    // message fields
    soundname: ::protobuf::SingularField<::std::string::String>,
    stop: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageSendAudio {}

impl CUserMessageSendAudio {
    pub fn new() -> CUserMessageSendAudio {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageSendAudio {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageSendAudio> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageSendAudio,
        };
        unsafe {
            instance.get(CUserMessageSendAudio::new)
        }
    }

    // optional string soundname = 1;

    pub fn clear_soundname(&mut self) {
        self.soundname.clear();
    }

    pub fn has_soundname(&self) -> bool {
        self.soundname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_soundname(&mut self, v: ::std::string::String) {
        self.soundname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_soundname(&mut self) -> &mut ::std::string::String {
        if self.soundname.is_none() {
            self.soundname.set_default();
        };
        self.soundname.as_mut().unwrap()
    }

    // Take field
    pub fn take_soundname(&mut self) -> ::std::string::String {
        self.soundname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_soundname(&self) -> &str {
        match self.soundname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_soundname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.soundname
    }

    fn mut_soundname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.soundname
    }

    // optional bool stop = 2;

    pub fn clear_stop(&mut self) {
        self.stop = ::std::option::Option::None;
    }

    pub fn has_stop(&self) -> bool {
        self.stop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stop(&mut self, v: bool) {
        self.stop = ::std::option::Option::Some(v);
    }

    pub fn get_stop(&self) -> bool {
        self.stop.unwrap_or(false)
    }

    fn get_stop_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.stop
    }

    fn mut_stop_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.stop
    }
}

impl ::protobuf::Message for CUserMessageSendAudio {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.soundname)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.stop = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.soundname.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.stop {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.soundname.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.stop {
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

impl ::protobuf::MessageStatic for CUserMessageSendAudio {
    fn new() -> CUserMessageSendAudio {
        CUserMessageSendAudio::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageSendAudio>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "soundname",
                    CUserMessageSendAudio::get_soundname_for_reflect,
                    CUserMessageSendAudio::mut_soundname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "stop",
                    CUserMessageSendAudio::get_stop_for_reflect,
                    CUserMessageSendAudio::mut_stop_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageSendAudio>(
                    "CUserMessageSendAudio",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageSendAudio {
    fn clear(&mut self) {
        self.clear_soundname();
        self.clear_stop();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageSendAudio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageSendAudio {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageAudioParameter {
    // message fields
    parameter_type: ::std::option::Option<u32>,
    name_hash_code: ::std::option::Option<u32>,
    value: ::std::option::Option<f32>,
    int_value: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageAudioParameter {}

impl CUserMessageAudioParameter {
    pub fn new() -> CUserMessageAudioParameter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageAudioParameter {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageAudioParameter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageAudioParameter,
        };
        unsafe {
            instance.get(CUserMessageAudioParameter::new)
        }
    }

    // optional uint32 parameter_type = 1;

    pub fn clear_parameter_type(&mut self) {
        self.parameter_type = ::std::option::Option::None;
    }

    pub fn has_parameter_type(&self) -> bool {
        self.parameter_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parameter_type(&mut self, v: u32) {
        self.parameter_type = ::std::option::Option::Some(v);
    }

    pub fn get_parameter_type(&self) -> u32 {
        self.parameter_type.unwrap_or(0)
    }

    fn get_parameter_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.parameter_type
    }

    fn mut_parameter_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.parameter_type
    }

    // optional uint32 name_hash_code = 2;

    pub fn clear_name_hash_code(&mut self) {
        self.name_hash_code = ::std::option::Option::None;
    }

    pub fn has_name_hash_code(&self) -> bool {
        self.name_hash_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name_hash_code(&mut self, v: u32) {
        self.name_hash_code = ::std::option::Option::Some(v);
    }

    pub fn get_name_hash_code(&self) -> u32 {
        self.name_hash_code.unwrap_or(0)
    }

    fn get_name_hash_code_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.name_hash_code
    }

    fn mut_name_hash_code_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.name_hash_code
    }

    // optional float value = 3;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> f32 {
        self.value.unwrap_or(0.)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.value
    }

    // optional uint32 int_value = 4;

    pub fn clear_int_value(&mut self) {
        self.int_value = ::std::option::Option::None;
    }

    pub fn has_int_value(&self) -> bool {
        self.int_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int_value(&mut self, v: u32) {
        self.int_value = ::std::option::Option::Some(v);
    }

    pub fn get_int_value(&self) -> u32 {
        self.int_value.unwrap_or(0)
    }

    fn get_int_value_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.int_value
    }

    fn mut_int_value_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.int_value
    }
}

impl ::protobuf::Message for CUserMessageAudioParameter {
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
                    self.parameter_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.name_hash_code = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.int_value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.parameter_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name_hash_code {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.value {
            my_size += 5;
        };
        if let Some(v) = self.int_value {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.parameter_type {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.name_hash_code {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.value {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.int_value {
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

impl ::protobuf::MessageStatic for CUserMessageAudioParameter {
    fn new() -> CUserMessageAudioParameter {
        CUserMessageAudioParameter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageAudioParameter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "parameter_type",
                    CUserMessageAudioParameter::get_parameter_type_for_reflect,
                    CUserMessageAudioParameter::mut_parameter_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "name_hash_code",
                    CUserMessageAudioParameter::get_name_hash_code_for_reflect,
                    CUserMessageAudioParameter::mut_name_hash_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "value",
                    CUserMessageAudioParameter::get_value_for_reflect,
                    CUserMessageAudioParameter::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "int_value",
                    CUserMessageAudioParameter::get_int_value_for_reflect,
                    CUserMessageAudioParameter::mut_int_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageAudioParameter>(
                    "CUserMessageAudioParameter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageAudioParameter {
    fn clear(&mut self) {
        self.clear_parameter_type();
        self.clear_name_hash_code();
        self.clear_value();
        self.clear_int_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageAudioParameter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageAudioParameter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageVoiceMask {
    // message fields
    gamerules_masks: ::std::vec::Vec<u32>,
    ban_masks: ::std::vec::Vec<u32>,
    mod_enable: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageVoiceMask {}

impl CUserMessageVoiceMask {
    pub fn new() -> CUserMessageVoiceMask {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageVoiceMask {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageVoiceMask> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageVoiceMask,
        };
        unsafe {
            instance.get(CUserMessageVoiceMask::new)
        }
    }

    // repeated uint32 gamerules_masks = 1;

    pub fn clear_gamerules_masks(&mut self) {
        self.gamerules_masks.clear();
    }

    // Param is passed by value, moved
    pub fn set_gamerules_masks(&mut self, v: ::std::vec::Vec<u32>) {
        self.gamerules_masks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_gamerules_masks(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.gamerules_masks
    }

    // Take field
    pub fn take_gamerules_masks(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.gamerules_masks, ::std::vec::Vec::new())
    }

    pub fn get_gamerules_masks(&self) -> &[u32] {
        &self.gamerules_masks
    }

    fn get_gamerules_masks_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.gamerules_masks
    }

    fn mut_gamerules_masks_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.gamerules_masks
    }

    // repeated uint32 ban_masks = 2;

    pub fn clear_ban_masks(&mut self) {
        self.ban_masks.clear();
    }

    // Param is passed by value, moved
    pub fn set_ban_masks(&mut self, v: ::std::vec::Vec<u32>) {
        self.ban_masks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ban_masks(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ban_masks
    }

    // Take field
    pub fn take_ban_masks(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ban_masks, ::std::vec::Vec::new())
    }

    pub fn get_ban_masks(&self) -> &[u32] {
        &self.ban_masks
    }

    fn get_ban_masks_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ban_masks
    }

    fn mut_ban_masks_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ban_masks
    }

    // optional bool mod_enable = 3;

    pub fn clear_mod_enable(&mut self) {
        self.mod_enable = ::std::option::Option::None;
    }

    pub fn has_mod_enable(&self) -> bool {
        self.mod_enable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mod_enable(&mut self, v: bool) {
        self.mod_enable = ::std::option::Option::Some(v);
    }

    pub fn get_mod_enable(&self) -> bool {
        self.mod_enable.unwrap_or(false)
    }

    fn get_mod_enable_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.mod_enable
    }

    fn mut_mod_enable_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.mod_enable
    }
}

impl ::protobuf::Message for CUserMessageVoiceMask {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.gamerules_masks)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.ban_masks)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.mod_enable = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.gamerules_masks {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.ban_masks {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.mod_enable {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.gamerules_masks {
            os.write_uint32(1, *v)?;
        };
        for v in &self.ban_masks {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.mod_enable {
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

impl ::protobuf::MessageStatic for CUserMessageVoiceMask {
    fn new() -> CUserMessageVoiceMask {
        CUserMessageVoiceMask::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageVoiceMask>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gamerules_masks",
                    CUserMessageVoiceMask::get_gamerules_masks_for_reflect,
                    CUserMessageVoiceMask::mut_gamerules_masks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ban_masks",
                    CUserMessageVoiceMask::get_ban_masks_for_reflect,
                    CUserMessageVoiceMask::mut_ban_masks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "mod_enable",
                    CUserMessageVoiceMask::get_mod_enable_for_reflect,
                    CUserMessageVoiceMask::mut_mod_enable_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageVoiceMask>(
                    "CUserMessageVoiceMask",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageVoiceMask {
    fn clear(&mut self) {
        self.clear_gamerules_masks();
        self.clear_ban_masks();
        self.clear_mod_enable();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageVoiceMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageVoiceMask {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageRequestState {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageRequestState {}

impl CUserMessageRequestState {
    pub fn new() -> CUserMessageRequestState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageRequestState {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageRequestState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageRequestState,
        };
        unsafe {
            instance.get(CUserMessageRequestState::new)
        }
    }
}

impl ::protobuf::Message for CUserMessageRequestState {
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

impl ::protobuf::MessageStatic for CUserMessageRequestState {
    fn new() -> CUserMessageRequestState {
        CUserMessageRequestState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageRequestState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageRequestState>(
                    "CUserMessageRequestState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageRequestState {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageRequestState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageRequestState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageHintText {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageHintText {}

impl CUserMessageHintText {
    pub fn new() -> CUserMessageHintText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageHintText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageHintText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageHintText,
        };
        unsafe {
            instance.get(CUserMessageHintText::new)
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
}

impl ::protobuf::Message for CUserMessageHintText {
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
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageHintText {
    fn new() -> CUserMessageHintText {
        CUserMessageHintText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageHintText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CUserMessageHintText::get_message_for_reflect,
                    CUserMessageHintText::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageHintText>(
                    "CUserMessageHintText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageHintText {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageHintText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageHintText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageKeyHintText {
    // message fields
    messages: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageKeyHintText {}

impl CUserMessageKeyHintText {
    pub fn new() -> CUserMessageKeyHintText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageKeyHintText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageKeyHintText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageKeyHintText,
        };
        unsafe {
            instance.get(CUserMessageKeyHintText::new)
        }
    }

    // repeated string messages = 1;

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.messages
    }

    // Take field
    pub fn take_messages(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.messages, ::protobuf::RepeatedField::new())
    }

    pub fn get_messages(&self) -> &[::std::string::String] {
        &self.messages
    }

    fn get_messages_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.messages
    }

    fn mut_messages_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.messages
    }
}

impl ::protobuf::Message for CUserMessageKeyHintText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.messages)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.messages {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.messages {
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

impl ::protobuf::MessageStatic for CUserMessageKeyHintText {
    fn new() -> CUserMessageKeyHintText {
        CUserMessageKeyHintText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageKeyHintText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "messages",
                    CUserMessageKeyHintText::get_messages_for_reflect,
                    CUserMessageKeyHintText::mut_messages_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageKeyHintText>(
                    "CUserMessageKeyHintText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageKeyHintText {
    fn clear(&mut self) {
        self.clear_messages();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageKeyHintText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageKeyHintText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageVoiceSubtitle {
    // message fields
    player: ::std::option::Option<i32>,
    menu: ::std::option::Option<i32>,
    item: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageVoiceSubtitle {}

impl CUserMessageVoiceSubtitle {
    pub fn new() -> CUserMessageVoiceSubtitle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageVoiceSubtitle {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageVoiceSubtitle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageVoiceSubtitle,
        };
        unsafe {
            instance.get(CUserMessageVoiceSubtitle::new)
        }
    }

    // optional int32 player = 1;

    pub fn clear_player(&mut self) {
        self.player = ::std::option::Option::None;
    }

    pub fn has_player(&self) -> bool {
        self.player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player(&mut self, v: i32) {
        self.player = ::std::option::Option::Some(v);
    }

    pub fn get_player(&self) -> i32 {
        self.player.unwrap_or(0)
    }

    fn get_player_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player
    }

    fn mut_player_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player
    }

    // optional int32 menu = 2;

    pub fn clear_menu(&mut self) {
        self.menu = ::std::option::Option::None;
    }

    pub fn has_menu(&self) -> bool {
        self.menu.is_some()
    }

    // Param is passed by value, moved
    pub fn set_menu(&mut self, v: i32) {
        self.menu = ::std::option::Option::Some(v);
    }

    pub fn get_menu(&self) -> i32 {
        self.menu.unwrap_or(0)
    }

    fn get_menu_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.menu
    }

    fn mut_menu_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.menu
    }

    // optional int32 item = 3;

    pub fn clear_item(&mut self) {
        self.item = ::std::option::Option::None;
    }

    pub fn has_item(&self) -> bool {
        self.item.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item(&mut self, v: i32) {
        self.item = ::std::option::Option::Some(v);
    }

    pub fn get_item(&self) -> i32 {
        self.item.unwrap_or(0)
    }

    fn get_item_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.item
    }

    fn mut_item_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.item
    }
}

impl ::protobuf::Message for CUserMessageVoiceSubtitle {
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
                    self.player = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.menu = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.item = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.player {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.menu {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.item {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.menu {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.item {
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

impl ::protobuf::MessageStatic for CUserMessageVoiceSubtitle {
    fn new() -> CUserMessageVoiceSubtitle {
        CUserMessageVoiceSubtitle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageVoiceSubtitle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player",
                    CUserMessageVoiceSubtitle::get_player_for_reflect,
                    CUserMessageVoiceSubtitle::mut_player_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "menu",
                    CUserMessageVoiceSubtitle::get_menu_for_reflect,
                    CUserMessageVoiceSubtitle::mut_menu_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "item",
                    CUserMessageVoiceSubtitle::get_item_for_reflect,
                    CUserMessageVoiceSubtitle::mut_item_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageVoiceSubtitle>(
                    "CUserMessageVoiceSubtitle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageVoiceSubtitle {
    fn clear(&mut self) {
        self.clear_player();
        self.clear_menu();
        self.clear_item();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageVoiceSubtitle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageVoiceSubtitle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageVGUIMenu {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    show: ::std::option::Option<bool>,
    keys: ::protobuf::RepeatedField<CUserMessageVGUIMenu_Keys>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageVGUIMenu {}

impl CUserMessageVGUIMenu {
    pub fn new() -> CUserMessageVGUIMenu {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageVGUIMenu {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageVGUIMenu> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageVGUIMenu,
        };
        unsafe {
            instance.get(CUserMessageVGUIMenu::new)
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

    // optional bool show = 2;

    pub fn clear_show(&mut self) {
        self.show = ::std::option::Option::None;
    }

    pub fn has_show(&self) -> bool {
        self.show.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show(&mut self, v: bool) {
        self.show = ::std::option::Option::Some(v);
    }

    pub fn get_show(&self) -> bool {
        self.show.unwrap_or(false)
    }

    fn get_show_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.show
    }

    fn mut_show_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.show
    }

    // repeated .dota.CUserMessageVGUIMenu.Keys keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CUserMessageVGUIMenu_Keys>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CUserMessageVGUIMenu_Keys> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CUserMessageVGUIMenu_Keys> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CUserMessageVGUIMenu_Keys] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<CUserMessageVGUIMenu_Keys> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CUserMessageVGUIMenu_Keys> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CUserMessageVGUIMenu {
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
                    let tmp = is.read_bool()?;
                    self.show = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.show {
            my_size += 2;
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
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.show {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for CUserMessageVGUIMenu {
    fn new() -> CUserMessageVGUIMenu {
        CUserMessageVGUIMenu::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageVGUIMenu>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CUserMessageVGUIMenu::get_name_for_reflect,
                    CUserMessageVGUIMenu::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "show",
                    CUserMessageVGUIMenu::get_show_for_reflect,
                    CUserMessageVGUIMenu::mut_show_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMessageVGUIMenu_Keys>>(
                    "keys",
                    CUserMessageVGUIMenu::get_keys_for_reflect,
                    CUserMessageVGUIMenu::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageVGUIMenu>(
                    "CUserMessageVGUIMenu",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageVGUIMenu {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_show();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageVGUIMenu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageVGUIMenu {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageVGUIMenu_Keys {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageVGUIMenu_Keys {}

impl CUserMessageVGUIMenu_Keys {
    pub fn new() -> CUserMessageVGUIMenu_Keys {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageVGUIMenu_Keys {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageVGUIMenu_Keys> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageVGUIMenu_Keys,
        };
        unsafe {
            instance.get(CUserMessageVGUIMenu_Keys::new)
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

    // optional string value = 2;

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

impl ::protobuf::Message for CUserMessageVGUIMenu_Keys {
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageVGUIMenu_Keys {
    fn new() -> CUserMessageVGUIMenu_Keys {
        CUserMessageVGUIMenu_Keys::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageVGUIMenu_Keys>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CUserMessageVGUIMenu_Keys::get_name_for_reflect,
                    CUserMessageVGUIMenu_Keys::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CUserMessageVGUIMenu_Keys::get_value_for_reflect,
                    CUserMessageVGUIMenu_Keys::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageVGUIMenu_Keys>(
                    "CUserMessageVGUIMenu_Keys",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageVGUIMenu_Keys {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageVGUIMenu_Keys {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageVGUIMenu_Keys {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageRumble {
    // message fields
    index: ::std::option::Option<i32>,
    data: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageRumble {}

impl CUserMessageRumble {
    pub fn new() -> CUserMessageRumble {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageRumble {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageRumble> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageRumble,
        };
        unsafe {
            instance.get(CUserMessageRumble::new)
        }
    }

    // optional int32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> i32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.index
    }

    // optional int32 data = 2;

    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: i32) {
        self.data = ::std::option::Option::Some(v);
    }

    pub fn get_data(&self) -> i32 {
        self.data.unwrap_or(0)
    }

    fn get_data_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.data
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
}

impl ::protobuf::Message for CUserMessageRumble {
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
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.data = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.data {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.data {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.flags {
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

impl ::protobuf::MessageStatic for CUserMessageRumble {
    fn new() -> CUserMessageRumble {
        CUserMessageRumble::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageRumble>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "index",
                    CUserMessageRumble::get_index_for_reflect,
                    CUserMessageRumble::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "data",
                    CUserMessageRumble::get_data_for_reflect,
                    CUserMessageRumble::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CUserMessageRumble::get_flags_for_reflect,
                    CUserMessageRumble::mut_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageRumble>(
                    "CUserMessageRumble",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageRumble {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_data();
        self.clear_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageRumble {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageRumble {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageTrain {
    // message fields
    position: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageTrain {}

impl CUserMessageTrain {
    pub fn new() -> CUserMessageTrain {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageTrain {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageTrain> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageTrain,
        };
        unsafe {
            instance.get(CUserMessageTrain::new)
        }
    }

    // optional uint32 position = 1;

    pub fn clear_position(&mut self) {
        self.position = ::std::option::Option::None;
    }

    pub fn has_position(&self) -> bool {
        self.position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position(&mut self, v: u32) {
        self.position = ::std::option::Option::Some(v);
    }

    pub fn get_position(&self) -> u32 {
        self.position.unwrap_or(0)
    }

    fn get_position_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.position
    }

    fn mut_position_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.position
    }
}

impl ::protobuf::Message for CUserMessageTrain {
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
                    self.position = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.position {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.position {
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

impl ::protobuf::MessageStatic for CUserMessageTrain {
    fn new() -> CUserMessageTrain {
        CUserMessageTrain::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageTrain>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position",
                    CUserMessageTrain::get_position_for_reflect,
                    CUserMessageTrain::mut_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageTrain>(
                    "CUserMessageTrain",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageTrain {
    fn clear(&mut self) {
        self.clear_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageTrain {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageTrain {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageSayTextChannel {
    // message fields
    player: ::std::option::Option<i32>,
    channel: ::std::option::Option<i32>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageSayTextChannel {}

impl CUserMessageSayTextChannel {
    pub fn new() -> CUserMessageSayTextChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageSayTextChannel {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageSayTextChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageSayTextChannel,
        };
        unsafe {
            instance.get(CUserMessageSayTextChannel::new)
        }
    }

    // optional int32 player = 1;

    pub fn clear_player(&mut self) {
        self.player = ::std::option::Option::None;
    }

    pub fn has_player(&self) -> bool {
        self.player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player(&mut self, v: i32) {
        self.player = ::std::option::Option::Some(v);
    }

    pub fn get_player(&self) -> i32 {
        self.player.unwrap_or(0)
    }

    fn get_player_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player
    }

    fn mut_player_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player
    }

    // optional int32 channel = 2;

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

    // optional string text = 3;

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

impl ::protobuf::Message for CUserMessageSayTextChannel {
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
                    self.player = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.channel = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(v) = self.player {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.channel {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.channel {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.text.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageSayTextChannel {
    fn new() -> CUserMessageSayTextChannel {
        CUserMessageSayTextChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageSayTextChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player",
                    CUserMessageSayTextChannel::get_player_for_reflect,
                    CUserMessageSayTextChannel::mut_player_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "channel",
                    CUserMessageSayTextChannel::get_channel_for_reflect,
                    CUserMessageSayTextChannel::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CUserMessageSayTextChannel::get_text_for_reflect,
                    CUserMessageSayTextChannel::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageSayTextChannel>(
                    "CUserMessageSayTextChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageSayTextChannel {
    fn clear(&mut self) {
        self.clear_player();
        self.clear_channel();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageSayTextChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageSayTextChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageColoredText {
    // message fields
    color: ::std::option::Option<u32>,
    text: ::protobuf::SingularField<::std::string::String>,
    reset: ::std::option::Option<bool>,
    context_player_id: ::std::option::Option<i32>,
    context_value: ::std::option::Option<i32>,
    context_team_id: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageColoredText {}

impl CUserMessageColoredText {
    pub fn new() -> CUserMessageColoredText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageColoredText {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageColoredText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageColoredText,
        };
        unsafe {
            instance.get(CUserMessageColoredText::new)
        }
    }

    // optional uint32 color = 1;

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

    // optional string text = 2;

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

    // optional bool reset = 3;

    pub fn clear_reset(&mut self) {
        self.reset = ::std::option::Option::None;
    }

    pub fn has_reset(&self) -> bool {
        self.reset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: bool) {
        self.reset = ::std::option::Option::Some(v);
    }

    pub fn get_reset(&self) -> bool {
        self.reset.unwrap_or(false)
    }

    fn get_reset_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.reset
    }

    fn mut_reset_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.reset
    }

    // optional int32 context_player_id = 4;

    pub fn clear_context_player_id(&mut self) {
        self.context_player_id = ::std::option::Option::None;
    }

    pub fn has_context_player_id(&self) -> bool {
        self.context_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_player_id(&mut self, v: i32) {
        self.context_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_context_player_id(&self) -> i32 {
        self.context_player_id.unwrap_or(0)
    }

    fn get_context_player_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.context_player_id
    }

    fn mut_context_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.context_player_id
    }

    // optional int32 context_value = 5;

    pub fn clear_context_value(&mut self) {
        self.context_value = ::std::option::Option::None;
    }

    pub fn has_context_value(&self) -> bool {
        self.context_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_value(&mut self, v: i32) {
        self.context_value = ::std::option::Option::Some(v);
    }

    pub fn get_context_value(&self) -> i32 {
        self.context_value.unwrap_or(0)
    }

    fn get_context_value_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.context_value
    }

    fn mut_context_value_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.context_value
    }

    // optional int32 context_team_id = 6;

    pub fn clear_context_team_id(&mut self) {
        self.context_team_id = ::std::option::Option::None;
    }

    pub fn has_context_team_id(&self) -> bool {
        self.context_team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_team_id(&mut self, v: i32) {
        self.context_team_id = ::std::option::Option::Some(v);
    }

    pub fn get_context_team_id(&self) -> i32 {
        self.context_team_id.unwrap_or(0)
    }

    fn get_context_team_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.context_team_id
    }

    fn mut_context_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.context_team_id
    }
}

impl ::protobuf::Message for CUserMessageColoredText {
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
                    self.color = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.reset = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.context_player_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.context_value = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.context_team_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.color {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.reset {
            my_size += 2;
        };
        if let Some(v) = self.context_player_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.context_value {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.context_team_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.color {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.text.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.reset {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.context_player_id {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.context_value {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.context_team_id {
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

impl ::protobuf::MessageStatic for CUserMessageColoredText {
    fn new() -> CUserMessageColoredText {
        CUserMessageColoredText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageColoredText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "color",
                    CUserMessageColoredText::get_color_for_reflect,
                    CUserMessageColoredText::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CUserMessageColoredText::get_text_for_reflect,
                    CUserMessageColoredText::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reset",
                    CUserMessageColoredText::get_reset_for_reflect,
                    CUserMessageColoredText::mut_reset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "context_player_id",
                    CUserMessageColoredText::get_context_player_id_for_reflect,
                    CUserMessageColoredText::mut_context_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "context_value",
                    CUserMessageColoredText::get_context_value_for_reflect,
                    CUserMessageColoredText::mut_context_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "context_team_id",
                    CUserMessageColoredText::get_context_team_id_for_reflect,
                    CUserMessageColoredText::mut_context_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageColoredText>(
                    "CUserMessageColoredText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageColoredText {
    fn clear(&mut self) {
        self.clear_color();
        self.clear_text();
        self.clear_reset();
        self.clear_context_player_id();
        self.clear_context_value();
        self.clear_context_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageColoredText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageColoredText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageItemPickup {
    // message fields
    itemname: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageItemPickup {}

impl CUserMessageItemPickup {
    pub fn new() -> CUserMessageItemPickup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageItemPickup {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageItemPickup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageItemPickup,
        };
        unsafe {
            instance.get(CUserMessageItemPickup::new)
        }
    }

    // optional string itemname = 1;

    pub fn clear_itemname(&mut self) {
        self.itemname.clear();
    }

    pub fn has_itemname(&self) -> bool {
        self.itemname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itemname(&mut self, v: ::std::string::String) {
        self.itemname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_itemname(&mut self) -> &mut ::std::string::String {
        if self.itemname.is_none() {
            self.itemname.set_default();
        };
        self.itemname.as_mut().unwrap()
    }

    // Take field
    pub fn take_itemname(&mut self) -> ::std::string::String {
        self.itemname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_itemname(&self) -> &str {
        match self.itemname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_itemname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.itemname
    }

    fn mut_itemname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.itemname
    }
}

impl ::protobuf::Message for CUserMessageItemPickup {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.itemname)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.itemname.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.itemname.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageItemPickup {
    fn new() -> CUserMessageItemPickup {
        CUserMessageItemPickup::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageItemPickup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "itemname",
                    CUserMessageItemPickup::get_itemname_for_reflect,
                    CUserMessageItemPickup::mut_itemname_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageItemPickup>(
                    "CUserMessageItemPickup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageItemPickup {
    fn clear(&mut self) {
        self.clear_itemname();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageItemPickup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageItemPickup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageAmmoDenied {
    // message fields
    ammo_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageAmmoDenied {}

impl CUserMessageAmmoDenied {
    pub fn new() -> CUserMessageAmmoDenied {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageAmmoDenied {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageAmmoDenied> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageAmmoDenied,
        };
        unsafe {
            instance.get(CUserMessageAmmoDenied::new)
        }
    }

    // optional uint32 ammo_id = 1;

    pub fn clear_ammo_id(&mut self) {
        self.ammo_id = ::std::option::Option::None;
    }

    pub fn has_ammo_id(&self) -> bool {
        self.ammo_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ammo_id(&mut self, v: u32) {
        self.ammo_id = ::std::option::Option::Some(v);
    }

    pub fn get_ammo_id(&self) -> u32 {
        self.ammo_id.unwrap_or(0)
    }

    fn get_ammo_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ammo_id
    }

    fn mut_ammo_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ammo_id
    }
}

impl ::protobuf::Message for CUserMessageAmmoDenied {
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
                    self.ammo_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ammo_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ammo_id {
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

impl ::protobuf::MessageStatic for CUserMessageAmmoDenied {
    fn new() -> CUserMessageAmmoDenied {
        CUserMessageAmmoDenied::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageAmmoDenied>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ammo_id",
                    CUserMessageAmmoDenied::get_ammo_id_for_reflect,
                    CUserMessageAmmoDenied::mut_ammo_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageAmmoDenied>(
                    "CUserMessageAmmoDenied",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageAmmoDenied {
    fn clear(&mut self) {
        self.clear_ammo_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageAmmoDenied {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageAmmoDenied {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageCrosshairAngle {
    // message fields
    angcrosshair: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageCrosshairAngle {}

impl CUserMessageCrosshairAngle {
    pub fn new() -> CUserMessageCrosshairAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageCrosshairAngle {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageCrosshairAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageCrosshairAngle,
        };
        unsafe {
            instance.get(CUserMessageCrosshairAngle::new)
        }
    }

    // optional .dota.CMsgQAngle angcrosshair = 1;

    pub fn clear_angcrosshair(&mut self) {
        self.angcrosshair.clear();
    }

    pub fn has_angcrosshair(&self) -> bool {
        self.angcrosshair.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angcrosshair(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angcrosshair = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angcrosshair(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angcrosshair.is_none() {
            self.angcrosshair.set_default();
        };
        self.angcrosshair.as_mut().unwrap()
    }

    // Take field
    pub fn take_angcrosshair(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angcrosshair.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angcrosshair(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angcrosshair.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angcrosshair_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angcrosshair
    }

    fn mut_angcrosshair_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angcrosshair
    }
}

impl ::protobuf::Message for CUserMessageCrosshairAngle {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angcrosshair)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.angcrosshair.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.angcrosshair.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageCrosshairAngle {
    fn new() -> CUserMessageCrosshairAngle {
        CUserMessageCrosshairAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageCrosshairAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angcrosshair",
                    CUserMessageCrosshairAngle::get_angcrosshair_for_reflect,
                    CUserMessageCrosshairAngle::mut_angcrosshair_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageCrosshairAngle>(
                    "CUserMessageCrosshairAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageCrosshairAngle {
    fn clear(&mut self) {
        self.clear_angcrosshair();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageCrosshairAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageCrosshairAngle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageShowMenu {
    // message fields
    validslots: ::std::option::Option<u32>,
    displaytime: ::std::option::Option<u32>,
    needmore: ::std::option::Option<bool>,
    menustring: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageShowMenu {}

impl CUserMessageShowMenu {
    pub fn new() -> CUserMessageShowMenu {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageShowMenu {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageShowMenu> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageShowMenu,
        };
        unsafe {
            instance.get(CUserMessageShowMenu::new)
        }
    }

    // optional uint32 validslots = 1;

    pub fn clear_validslots(&mut self) {
        self.validslots = ::std::option::Option::None;
    }

    pub fn has_validslots(&self) -> bool {
        self.validslots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_validslots(&mut self, v: u32) {
        self.validslots = ::std::option::Option::Some(v);
    }

    pub fn get_validslots(&self) -> u32 {
        self.validslots.unwrap_or(0)
    }

    fn get_validslots_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.validslots
    }

    fn mut_validslots_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.validslots
    }

    // optional uint32 displaytime = 2;

    pub fn clear_displaytime(&mut self) {
        self.displaytime = ::std::option::Option::None;
    }

    pub fn has_displaytime(&self) -> bool {
        self.displaytime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_displaytime(&mut self, v: u32) {
        self.displaytime = ::std::option::Option::Some(v);
    }

    pub fn get_displaytime(&self) -> u32 {
        self.displaytime.unwrap_or(0)
    }

    fn get_displaytime_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.displaytime
    }

    fn mut_displaytime_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.displaytime
    }

    // optional bool needmore = 3;

    pub fn clear_needmore(&mut self) {
        self.needmore = ::std::option::Option::None;
    }

    pub fn has_needmore(&self) -> bool {
        self.needmore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needmore(&mut self, v: bool) {
        self.needmore = ::std::option::Option::Some(v);
    }

    pub fn get_needmore(&self) -> bool {
        self.needmore.unwrap_or(false)
    }

    fn get_needmore_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.needmore
    }

    fn mut_needmore_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.needmore
    }

    // optional string menustring = 4;

    pub fn clear_menustring(&mut self) {
        self.menustring.clear();
    }

    pub fn has_menustring(&self) -> bool {
        self.menustring.is_some()
    }

    // Param is passed by value, moved
    pub fn set_menustring(&mut self, v: ::std::string::String) {
        self.menustring = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_menustring(&mut self) -> &mut ::std::string::String {
        if self.menustring.is_none() {
            self.menustring.set_default();
        };
        self.menustring.as_mut().unwrap()
    }

    // Take field
    pub fn take_menustring(&mut self) -> ::std::string::String {
        self.menustring.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_menustring(&self) -> &str {
        match self.menustring.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_menustring_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.menustring
    }

    fn mut_menustring_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.menustring
    }
}

impl ::protobuf::Message for CUserMessageShowMenu {
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
                    self.validslots = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.displaytime = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.needmore = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.menustring)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.validslots {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.displaytime {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.needmore {
            my_size += 2;
        };
        if let Some(v) = self.menustring.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.validslots {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.displaytime {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.needmore {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.menustring.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageShowMenu {
    fn new() -> CUserMessageShowMenu {
        CUserMessageShowMenu::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageShowMenu>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "validslots",
                    CUserMessageShowMenu::get_validslots_for_reflect,
                    CUserMessageShowMenu::mut_validslots_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "displaytime",
                    CUserMessageShowMenu::get_displaytime_for_reflect,
                    CUserMessageShowMenu::mut_displaytime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "needmore",
                    CUserMessageShowMenu::get_needmore_for_reflect,
                    CUserMessageShowMenu::mut_needmore_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "menustring",
                    CUserMessageShowMenu::get_menustring_for_reflect,
                    CUserMessageShowMenu::mut_menustring_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageShowMenu>(
                    "CUserMessageShowMenu",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageShowMenu {
    fn clear(&mut self) {
        self.clear_validslots();
        self.clear_displaytime();
        self.clear_needmore();
        self.clear_menustring();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageShowMenu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageShowMenu {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageCreditsMsg {
    // message fields
    rolltype: ::std::option::Option<eRollType>,
    logo_length: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageCreditsMsg {}

impl CUserMessageCreditsMsg {
    pub fn new() -> CUserMessageCreditsMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageCreditsMsg {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageCreditsMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageCreditsMsg,
        };
        unsafe {
            instance.get(CUserMessageCreditsMsg::new)
        }
    }

    // optional .dota.eRollType rolltype = 1;

    pub fn clear_rolltype(&mut self) {
        self.rolltype = ::std::option::Option::None;
    }

    pub fn has_rolltype(&self) -> bool {
        self.rolltype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rolltype(&mut self, v: eRollType) {
        self.rolltype = ::std::option::Option::Some(v);
    }

    pub fn get_rolltype(&self) -> eRollType {
        self.rolltype.unwrap_or(eRollType::ROLL_NONE)
    }

    fn get_rolltype_for_reflect(&self) -> &::std::option::Option<eRollType> {
        &self.rolltype
    }

    fn mut_rolltype_for_reflect(&mut self) -> &mut ::std::option::Option<eRollType> {
        &mut self.rolltype
    }

    // optional float logo_length = 2;

    pub fn clear_logo_length(&mut self) {
        self.logo_length = ::std::option::Option::None;
    }

    pub fn has_logo_length(&self) -> bool {
        self.logo_length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logo_length(&mut self, v: f32) {
        self.logo_length = ::std::option::Option::Some(v);
    }

    pub fn get_logo_length(&self) -> f32 {
        self.logo_length.unwrap_or(0.)
    }

    fn get_logo_length_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.logo_length
    }

    fn mut_logo_length_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.logo_length
    }
}

impl ::protobuf::Message for CUserMessageCreditsMsg {
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
                    self.rolltype = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.logo_length = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.rolltype {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.logo_length {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.rolltype {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.logo_length {
            os.write_float(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CUserMessageCreditsMsg {
    fn new() -> CUserMessageCreditsMsg {
        CUserMessageCreditsMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageCreditsMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<eRollType>>(
                    "rolltype",
                    CUserMessageCreditsMsg::get_rolltype_for_reflect,
                    CUserMessageCreditsMsg::mut_rolltype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "logo_length",
                    CUserMessageCreditsMsg::get_logo_length_for_reflect,
                    CUserMessageCreditsMsg::mut_logo_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageCreditsMsg>(
                    "CUserMessageCreditsMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageCreditsMsg {
    fn clear(&mut self) {
        self.clear_rolltype();
        self.clear_logo_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageCreditsMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageCreditsMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CEntityMessagePlayJingle {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CEntityMessagePlayJingle {}

impl CEntityMessagePlayJingle {
    pub fn new() -> CEntityMessagePlayJingle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CEntityMessagePlayJingle {
        static mut instance: ::protobuf::lazy::Lazy<CEntityMessagePlayJingle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CEntityMessagePlayJingle,
        };
        unsafe {
            instance.get(CEntityMessagePlayJingle::new)
        }
    }
}

impl ::protobuf::Message for CEntityMessagePlayJingle {
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

impl ::protobuf::MessageStatic for CEntityMessagePlayJingle {
    fn new() -> CEntityMessagePlayJingle {
        CEntityMessagePlayJingle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CEntityMessagePlayJingle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CEntityMessagePlayJingle>(
                    "CEntityMessagePlayJingle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CEntityMessagePlayJingle {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEntityMessagePlayJingle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEntityMessagePlayJingle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CEntityMessageScreenOverlay {
    // message fields
    start_effect: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CEntityMessageScreenOverlay {}

impl CEntityMessageScreenOverlay {
    pub fn new() -> CEntityMessageScreenOverlay {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CEntityMessageScreenOverlay {
        static mut instance: ::protobuf::lazy::Lazy<CEntityMessageScreenOverlay> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CEntityMessageScreenOverlay,
        };
        unsafe {
            instance.get(CEntityMessageScreenOverlay::new)
        }
    }

    // optional bool start_effect = 1;

    pub fn clear_start_effect(&mut self) {
        self.start_effect = ::std::option::Option::None;
    }

    pub fn has_start_effect(&self) -> bool {
        self.start_effect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_effect(&mut self, v: bool) {
        self.start_effect = ::std::option::Option::Some(v);
    }

    pub fn get_start_effect(&self) -> bool {
        self.start_effect.unwrap_or(false)
    }

    fn get_start_effect_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.start_effect
    }

    fn mut_start_effect_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.start_effect
    }
}

impl ::protobuf::Message for CEntityMessageScreenOverlay {
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
                    self.start_effect = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start_effect {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_effect {
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

impl ::protobuf::MessageStatic for CEntityMessageScreenOverlay {
    fn new() -> CEntityMessageScreenOverlay {
        CEntityMessageScreenOverlay::new()
    }

    fn descriptor_static(_: ::std::option::Option<CEntityMessageScreenOverlay>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "start_effect",
                    CEntityMessageScreenOverlay::get_start_effect_for_reflect,
                    CEntityMessageScreenOverlay::mut_start_effect_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CEntityMessageScreenOverlay>(
                    "CEntityMessageScreenOverlay",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CEntityMessageScreenOverlay {
    fn clear(&mut self) {
        self.clear_start_effect();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEntityMessageScreenOverlay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEntityMessageScreenOverlay {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CEntityMessageRemoveAllDecals {
    // message fields
    remove_decals: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CEntityMessageRemoveAllDecals {}

impl CEntityMessageRemoveAllDecals {
    pub fn new() -> CEntityMessageRemoveAllDecals {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CEntityMessageRemoveAllDecals {
        static mut instance: ::protobuf::lazy::Lazy<CEntityMessageRemoveAllDecals> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CEntityMessageRemoveAllDecals,
        };
        unsafe {
            instance.get(CEntityMessageRemoveAllDecals::new)
        }
    }

    // optional bool remove_decals = 1;

    pub fn clear_remove_decals(&mut self) {
        self.remove_decals = ::std::option::Option::None;
    }

    pub fn has_remove_decals(&self) -> bool {
        self.remove_decals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remove_decals(&mut self, v: bool) {
        self.remove_decals = ::std::option::Option::Some(v);
    }

    pub fn get_remove_decals(&self) -> bool {
        self.remove_decals.unwrap_or(false)
    }

    fn get_remove_decals_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.remove_decals
    }

    fn mut_remove_decals_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.remove_decals
    }
}

impl ::protobuf::Message for CEntityMessageRemoveAllDecals {
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
                    self.remove_decals = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.remove_decals {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.remove_decals {
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

impl ::protobuf::MessageStatic for CEntityMessageRemoveAllDecals {
    fn new() -> CEntityMessageRemoveAllDecals {
        CEntityMessageRemoveAllDecals::new()
    }

    fn descriptor_static(_: ::std::option::Option<CEntityMessageRemoveAllDecals>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "remove_decals",
                    CEntityMessageRemoveAllDecals::get_remove_decals_for_reflect,
                    CEntityMessageRemoveAllDecals::mut_remove_decals_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CEntityMessageRemoveAllDecals>(
                    "CEntityMessageRemoveAllDecals",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CEntityMessageRemoveAllDecals {
    fn clear(&mut self) {
        self.clear_remove_decals();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEntityMessageRemoveAllDecals {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEntityMessageRemoveAllDecals {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CEntityMessagePropagateForce {
    // message fields
    impulse: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CEntityMessagePropagateForce {}

impl CEntityMessagePropagateForce {
    pub fn new() -> CEntityMessagePropagateForce {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CEntityMessagePropagateForce {
        static mut instance: ::protobuf::lazy::Lazy<CEntityMessagePropagateForce> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CEntityMessagePropagateForce,
        };
        unsafe {
            instance.get(CEntityMessagePropagateForce::new)
        }
    }

    // optional .dota.CMsgVector impulse = 1;

    pub fn clear_impulse(&mut self) {
        self.impulse.clear();
    }

    pub fn has_impulse(&self) -> bool {
        self.impulse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_impulse(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.impulse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_impulse(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.impulse.is_none() {
            self.impulse.set_default();
        };
        self.impulse.as_mut().unwrap()
    }

    // Take field
    pub fn take_impulse(&mut self) -> super::networkbasetypes::CMsgVector {
        self.impulse.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_impulse(&self) -> &super::networkbasetypes::CMsgVector {
        self.impulse.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_impulse_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.impulse
    }

    fn mut_impulse_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.impulse
    }
}

impl ::protobuf::Message for CEntityMessagePropagateForce {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.impulse)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.impulse.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.impulse.as_ref() {
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

impl ::protobuf::MessageStatic for CEntityMessagePropagateForce {
    fn new() -> CEntityMessagePropagateForce {
        CEntityMessagePropagateForce::new()
    }

    fn descriptor_static(_: ::std::option::Option<CEntityMessagePropagateForce>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "impulse",
                    CEntityMessagePropagateForce::get_impulse_for_reflect,
                    CEntityMessagePropagateForce::mut_impulse_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CEntityMessagePropagateForce>(
                    "CEntityMessagePropagateForce",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CEntityMessagePropagateForce {
    fn clear(&mut self) {
        self.clear_impulse();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEntityMessagePropagateForce {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEntityMessagePropagateForce {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CEntityMessageDoSpark {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    entityindex: ::std::option::Option<u32>,
    radius: ::std::option::Option<f32>,
    color: ::std::option::Option<u32>,
    beams: ::std::option::Option<u32>,
    thick: ::std::option::Option<f32>,
    duration: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CEntityMessageDoSpark {}

impl CEntityMessageDoSpark {
    pub fn new() -> CEntityMessageDoSpark {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CEntityMessageDoSpark {
        static mut instance: ::protobuf::lazy::Lazy<CEntityMessageDoSpark> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CEntityMessageDoSpark,
        };
        unsafe {
            instance.get(CEntityMessageDoSpark::new)
        }
    }

    // optional .dota.CMsgVector origin = 1;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.origin.is_none() {
            self.origin.set_default();
        };
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> super::networkbasetypes::CMsgVector {
        self.origin.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_origin(&self) -> &super::networkbasetypes::CMsgVector {
        self.origin.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_origin_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.origin
    }

    // optional uint32 entityindex = 2;

    pub fn clear_entityindex(&mut self) {
        self.entityindex = ::std::option::Option::None;
    }

    pub fn has_entityindex(&self) -> bool {
        self.entityindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entityindex(&mut self, v: u32) {
        self.entityindex = ::std::option::Option::Some(v);
    }

    pub fn get_entityindex(&self) -> u32 {
        self.entityindex.unwrap_or(0)
    }

    fn get_entityindex_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entityindex
    }

    fn mut_entityindex_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entityindex
    }

    // optional float radius = 3;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: f32) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius(&self) -> f32 {
        self.radius.unwrap_or(0.)
    }

    fn get_radius_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.radius
    }

    fn mut_radius_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.radius
    }

    // optional fixed32 color = 4;

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

    // optional uint32 beams = 5;

    pub fn clear_beams(&mut self) {
        self.beams = ::std::option::Option::None;
    }

    pub fn has_beams(&self) -> bool {
        self.beams.is_some()
    }

    // Param is passed by value, moved
    pub fn set_beams(&mut self, v: u32) {
        self.beams = ::std::option::Option::Some(v);
    }

    pub fn get_beams(&self) -> u32 {
        self.beams.unwrap_or(0)
    }

    fn get_beams_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.beams
    }

    fn mut_beams_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.beams
    }

    // optional float thick = 6;

    pub fn clear_thick(&mut self) {
        self.thick = ::std::option::Option::None;
    }

    pub fn has_thick(&self) -> bool {
        self.thick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_thick(&mut self, v: f32) {
        self.thick = ::std::option::Option::Some(v);
    }

    pub fn get_thick(&self) -> f32 {
        self.thick.unwrap_or(0.)
    }

    fn get_thick_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.thick
    }

    fn mut_thick_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.thick
    }

    // optional float duration = 7;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }
}

impl ::protobuf::Message for CEntityMessageDoSpark {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.entityindex = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.radius = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.color = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.beams = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.thick = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.origin.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.entityindex {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radius {
            my_size += 5;
        };
        if let Some(v) = self.color {
            my_size += 5;
        };
        if let Some(v) = self.beams {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.thick {
            my_size += 5;
        };
        if let Some(v) = self.duration {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.entityindex {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.radius {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.color {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.beams {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.thick {
            os.write_float(6, v)?;
        };
        if let Some(v) = self.duration {
            os.write_float(7, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CEntityMessageDoSpark {
    fn new() -> CEntityMessageDoSpark {
        CEntityMessageDoSpark::new()
    }

    fn descriptor_static(_: ::std::option::Option<CEntityMessageDoSpark>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CEntityMessageDoSpark::get_origin_for_reflect,
                    CEntityMessageDoSpark::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "entityindex",
                    CEntityMessageDoSpark::get_entityindex_for_reflect,
                    CEntityMessageDoSpark::mut_entityindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "radius",
                    CEntityMessageDoSpark::get_radius_for_reflect,
                    CEntityMessageDoSpark::mut_radius_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "color",
                    CEntityMessageDoSpark::get_color_for_reflect,
                    CEntityMessageDoSpark::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "beams",
                    CEntityMessageDoSpark::get_beams_for_reflect,
                    CEntityMessageDoSpark::mut_beams_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "thick",
                    CEntityMessageDoSpark::get_thick_for_reflect,
                    CEntityMessageDoSpark::mut_thick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CEntityMessageDoSpark::get_duration_for_reflect,
                    CEntityMessageDoSpark::mut_duration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CEntityMessageDoSpark>(
                    "CEntityMessageDoSpark",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CEntityMessageDoSpark {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_entityindex();
        self.clear_radius();
        self.clear_color();
        self.clear_beams();
        self.clear_thick();
        self.clear_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEntityMessageDoSpark {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEntityMessageDoSpark {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CEntityMessageFixAngle {
    // message fields
    relative: ::std::option::Option<bool>,
    angle: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CEntityMessageFixAngle {}

impl CEntityMessageFixAngle {
    pub fn new() -> CEntityMessageFixAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CEntityMessageFixAngle {
        static mut instance: ::protobuf::lazy::Lazy<CEntityMessageFixAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CEntityMessageFixAngle,
        };
        unsafe {
            instance.get(CEntityMessageFixAngle::new)
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

impl ::protobuf::Message for CEntityMessageFixAngle {
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

impl ::protobuf::MessageStatic for CEntityMessageFixAngle {
    fn new() -> CEntityMessageFixAngle {
        CEntityMessageFixAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CEntityMessageFixAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "relative",
                    CEntityMessageFixAngle::get_relative_for_reflect,
                    CEntityMessageFixAngle::mut_relative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angle",
                    CEntityMessageFixAngle::get_angle_for_reflect,
                    CEntityMessageFixAngle::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CEntityMessageFixAngle>(
                    "CEntityMessageFixAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CEntityMessageFixAngle {
    fn clear(&mut self) {
        self.clear_relative();
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CEntityMessageFixAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CEntityMessageFixAngle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageCameraTransition {
    // message fields
    camera_type: ::std::option::Option<u32>,
    duration: ::std::option::Option<f32>,
    params_data_driven: ::protobuf::SingularPtrField<CUserMessageCameraTransition_Transition_DataDriven>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageCameraTransition {}

impl CUserMessageCameraTransition {
    pub fn new() -> CUserMessageCameraTransition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageCameraTransition {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageCameraTransition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageCameraTransition,
        };
        unsafe {
            instance.get(CUserMessageCameraTransition::new)
        }
    }

    // optional uint32 camera_type = 1;

    pub fn clear_camera_type(&mut self) {
        self.camera_type = ::std::option::Option::None;
    }

    pub fn has_camera_type(&self) -> bool {
        self.camera_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera_type(&mut self, v: u32) {
        self.camera_type = ::std::option::Option::Some(v);
    }

    pub fn get_camera_type(&self) -> u32 {
        self.camera_type.unwrap_or(0)
    }

    fn get_camera_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.camera_type
    }

    fn mut_camera_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.camera_type
    }

    // optional float duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }

    // optional .dota.CUserMessageCameraTransition.Transition_DataDriven params_data_driven = 3;

    pub fn clear_params_data_driven(&mut self) {
        self.params_data_driven.clear();
    }

    pub fn has_params_data_driven(&self) -> bool {
        self.params_data_driven.is_some()
    }

    // Param is passed by value, moved
    pub fn set_params_data_driven(&mut self, v: CUserMessageCameraTransition_Transition_DataDriven) {
        self.params_data_driven = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_params_data_driven(&mut self) -> &mut CUserMessageCameraTransition_Transition_DataDriven {
        if self.params_data_driven.is_none() {
            self.params_data_driven.set_default();
        };
        self.params_data_driven.as_mut().unwrap()
    }

    // Take field
    pub fn take_params_data_driven(&mut self) -> CUserMessageCameraTransition_Transition_DataDriven {
        self.params_data_driven.take().unwrap_or_else(|| CUserMessageCameraTransition_Transition_DataDriven::new())
    }

    pub fn get_params_data_driven(&self) -> &CUserMessageCameraTransition_Transition_DataDriven {
        self.params_data_driven.as_ref().unwrap_or_else(|| CUserMessageCameraTransition_Transition_DataDriven::default_instance())
    }

    fn get_params_data_driven_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMessageCameraTransition_Transition_DataDriven> {
        &self.params_data_driven
    }

    fn mut_params_data_driven_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMessageCameraTransition_Transition_DataDriven> {
        &mut self.params_data_driven
    }
}

impl ::protobuf::Message for CUserMessageCameraTransition {
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
                    self.camera_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.params_data_driven)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.camera_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.duration {
            my_size += 5;
        };
        if let Some(v) = self.params_data_driven.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.camera_type {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.duration {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.params_data_driven.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMessageCameraTransition {
    fn new() -> CUserMessageCameraTransition {
        CUserMessageCameraTransition::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageCameraTransition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "camera_type",
                    CUserMessageCameraTransition::get_camera_type_for_reflect,
                    CUserMessageCameraTransition::mut_camera_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CUserMessageCameraTransition::get_duration_for_reflect,
                    CUserMessageCameraTransition::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMessageCameraTransition_Transition_DataDriven>>(
                    "params_data_driven",
                    CUserMessageCameraTransition::get_params_data_driven_for_reflect,
                    CUserMessageCameraTransition::mut_params_data_driven_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageCameraTransition>(
                    "CUserMessageCameraTransition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageCameraTransition {
    fn clear(&mut self) {
        self.clear_camera_type();
        self.clear_duration();
        self.clear_params_data_driven();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageCameraTransition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageCameraTransition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageCameraTransition_Transition_DataDriven {
    // message fields
    filename: ::protobuf::SingularField<::std::string::String>,
    attach_ent_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageCameraTransition_Transition_DataDriven {}

impl CUserMessageCameraTransition_Transition_DataDriven {
    pub fn new() -> CUserMessageCameraTransition_Transition_DataDriven {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageCameraTransition_Transition_DataDriven {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageCameraTransition_Transition_DataDriven> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageCameraTransition_Transition_DataDriven,
        };
        unsafe {
            instance.get(CUserMessageCameraTransition_Transition_DataDriven::new)
        }
    }

    // optional string filename = 1;

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

    // optional int32 attach_ent_index = 2;

    pub fn clear_attach_ent_index(&mut self) {
        self.attach_ent_index = ::std::option::Option::None;
    }

    pub fn has_attach_ent_index(&self) -> bool {
        self.attach_ent_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attach_ent_index(&mut self, v: i32) {
        self.attach_ent_index = ::std::option::Option::Some(v);
    }

    pub fn get_attach_ent_index(&self) -> i32 {
        self.attach_ent_index.unwrap_or(0)
    }

    fn get_attach_ent_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.attach_ent_index
    }

    fn mut_attach_ent_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.attach_ent_index
    }
}

impl ::protobuf::Message for CUserMessageCameraTransition_Transition_DataDriven {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.attach_ent_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.attach_ent_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.filename.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.attach_ent_index {
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

impl ::protobuf::MessageStatic for CUserMessageCameraTransition_Transition_DataDriven {
    fn new() -> CUserMessageCameraTransition_Transition_DataDriven {
        CUserMessageCameraTransition_Transition_DataDriven::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageCameraTransition_Transition_DataDriven>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    CUserMessageCameraTransition_Transition_DataDriven::get_filename_for_reflect,
                    CUserMessageCameraTransition_Transition_DataDriven::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "attach_ent_index",
                    CUserMessageCameraTransition_Transition_DataDriven::get_attach_ent_index_for_reflect,
                    CUserMessageCameraTransition_Transition_DataDriven::mut_attach_ent_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageCameraTransition_Transition_DataDriven>(
                    "CUserMessageCameraTransition_Transition_DataDriven",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageCameraTransition_Transition_DataDriven {
    fn clear(&mut self) {
        self.clear_filename();
        self.clear_attach_ent_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageCameraTransition_Transition_DataDriven {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageCameraTransition_Transition_DataDriven {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager {
    // message fields
    field_type: ::std::option::Option<PARTICLE_MESSAGE>,
    index: ::std::option::Option<u32>,
    release_particle_index: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_ReleaseParticleIndex>,
    create_particle: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_CreateParticle>,
    destroy_particle: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_DestroyParticle>,
    destroy_particle_involving: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_DestroyParticleInvolving>,
    update_particle: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticle>,
    update_particle_fwd: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleFwd>,
    update_particle_orient: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleOrient>,
    update_particle_fallback: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleFallback>,
    update_particle_offset: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleOffset>,
    update_particle_ent: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleEnt>,
    update_particle_should_draw: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleShouldDraw>,
    update_particle_set_frozen: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleSetFrozen>,
    change_control_point_attachment: ::protobuf::SingularPtrField<CUserMsg_ParticleManager_ChangeControlPointAttachment>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager {}

impl CUserMsg_ParticleManager {
    pub fn new() -> CUserMsg_ParticleManager {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager::new)
        }
    }

    // required .dota.PARTICLE_MESSAGE type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: PARTICLE_MESSAGE) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> PARTICLE_MESSAGE {
        self.field_type.unwrap_or(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_CREATE)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<PARTICLE_MESSAGE> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<PARTICLE_MESSAGE> {
        &mut self.field_type
    }

    // required uint32 index = 2;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.index
    }

    // optional .dota.CUserMsg_ParticleManager.ReleaseParticleIndex release_particle_index = 3;

    pub fn clear_release_particle_index(&mut self) {
        self.release_particle_index.clear();
    }

    pub fn has_release_particle_index(&self) -> bool {
        self.release_particle_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_release_particle_index(&mut self, v: CUserMsg_ParticleManager_ReleaseParticleIndex) {
        self.release_particle_index = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_release_particle_index(&mut self) -> &mut CUserMsg_ParticleManager_ReleaseParticleIndex {
        if self.release_particle_index.is_none() {
            self.release_particle_index.set_default();
        };
        self.release_particle_index.as_mut().unwrap()
    }

    // Take field
    pub fn take_release_particle_index(&mut self) -> CUserMsg_ParticleManager_ReleaseParticleIndex {
        self.release_particle_index.take().unwrap_or_else(|| CUserMsg_ParticleManager_ReleaseParticleIndex::new())
    }

    pub fn get_release_particle_index(&self) -> &CUserMsg_ParticleManager_ReleaseParticleIndex {
        self.release_particle_index.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_ReleaseParticleIndex::default_instance())
    }

    fn get_release_particle_index_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_ReleaseParticleIndex> {
        &self.release_particle_index
    }

    fn mut_release_particle_index_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_ReleaseParticleIndex> {
        &mut self.release_particle_index
    }

    // optional .dota.CUserMsg_ParticleManager.CreateParticle create_particle = 4;

    pub fn clear_create_particle(&mut self) {
        self.create_particle.clear();
    }

    pub fn has_create_particle(&self) -> bool {
        self.create_particle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_particle(&mut self, v: CUserMsg_ParticleManager_CreateParticle) {
        self.create_particle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_particle(&mut self) -> &mut CUserMsg_ParticleManager_CreateParticle {
        if self.create_particle.is_none() {
            self.create_particle.set_default();
        };
        self.create_particle.as_mut().unwrap()
    }

    // Take field
    pub fn take_create_particle(&mut self) -> CUserMsg_ParticleManager_CreateParticle {
        self.create_particle.take().unwrap_or_else(|| CUserMsg_ParticleManager_CreateParticle::new())
    }

    pub fn get_create_particle(&self) -> &CUserMsg_ParticleManager_CreateParticle {
        self.create_particle.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_CreateParticle::default_instance())
    }

    fn get_create_particle_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_CreateParticle> {
        &self.create_particle
    }

    fn mut_create_particle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_CreateParticle> {
        &mut self.create_particle
    }

    // optional .dota.CUserMsg_ParticleManager.DestroyParticle destroy_particle = 5;

    pub fn clear_destroy_particle(&mut self) {
        self.destroy_particle.clear();
    }

    pub fn has_destroy_particle(&self) -> bool {
        self.destroy_particle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroy_particle(&mut self, v: CUserMsg_ParticleManager_DestroyParticle) {
        self.destroy_particle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destroy_particle(&mut self) -> &mut CUserMsg_ParticleManager_DestroyParticle {
        if self.destroy_particle.is_none() {
            self.destroy_particle.set_default();
        };
        self.destroy_particle.as_mut().unwrap()
    }

    // Take field
    pub fn take_destroy_particle(&mut self) -> CUserMsg_ParticleManager_DestroyParticle {
        self.destroy_particle.take().unwrap_or_else(|| CUserMsg_ParticleManager_DestroyParticle::new())
    }

    pub fn get_destroy_particle(&self) -> &CUserMsg_ParticleManager_DestroyParticle {
        self.destroy_particle.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_DestroyParticle::default_instance())
    }

    fn get_destroy_particle_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_DestroyParticle> {
        &self.destroy_particle
    }

    fn mut_destroy_particle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_DestroyParticle> {
        &mut self.destroy_particle
    }

    // optional .dota.CUserMsg_ParticleManager.DestroyParticleInvolving destroy_particle_involving = 6;

    pub fn clear_destroy_particle_involving(&mut self) {
        self.destroy_particle_involving.clear();
    }

    pub fn has_destroy_particle_involving(&self) -> bool {
        self.destroy_particle_involving.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroy_particle_involving(&mut self, v: CUserMsg_ParticleManager_DestroyParticleInvolving) {
        self.destroy_particle_involving = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destroy_particle_involving(&mut self) -> &mut CUserMsg_ParticleManager_DestroyParticleInvolving {
        if self.destroy_particle_involving.is_none() {
            self.destroy_particle_involving.set_default();
        };
        self.destroy_particle_involving.as_mut().unwrap()
    }

    // Take field
    pub fn take_destroy_particle_involving(&mut self) -> CUserMsg_ParticleManager_DestroyParticleInvolving {
        self.destroy_particle_involving.take().unwrap_or_else(|| CUserMsg_ParticleManager_DestroyParticleInvolving::new())
    }

    pub fn get_destroy_particle_involving(&self) -> &CUserMsg_ParticleManager_DestroyParticleInvolving {
        self.destroy_particle_involving.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_DestroyParticleInvolving::default_instance())
    }

    fn get_destroy_particle_involving_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_DestroyParticleInvolving> {
        &self.destroy_particle_involving
    }

    fn mut_destroy_particle_involving_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_DestroyParticleInvolving> {
        &mut self.destroy_particle_involving
    }

    // optional .dota.CUserMsg_ParticleManager.UpdateParticle update_particle = 7;

    pub fn clear_update_particle(&mut self) {
        self.update_particle.clear();
    }

    pub fn has_update_particle(&self) -> bool {
        self.update_particle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle(&mut self, v: CUserMsg_ParticleManager_UpdateParticle) {
        self.update_particle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle(&mut self) -> &mut CUserMsg_ParticleManager_UpdateParticle {
        if self.update_particle.is_none() {
            self.update_particle.set_default();
        };
        self.update_particle.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle(&mut self) -> CUserMsg_ParticleManager_UpdateParticle {
        self.update_particle.take().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticle::new())
    }

    pub fn get_update_particle(&self) -> &CUserMsg_ParticleManager_UpdateParticle {
        self.update_particle.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticle::default_instance())
    }

    fn get_update_particle_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticle> {
        &self.update_particle
    }

    fn mut_update_particle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticle> {
        &mut self.update_particle
    }

    // optional .dota.CUserMsg_ParticleManager.UpdateParticleFwd update_particle_fwd = 8;

    pub fn clear_update_particle_fwd(&mut self) {
        self.update_particle_fwd.clear();
    }

    pub fn has_update_particle_fwd(&self) -> bool {
        self.update_particle_fwd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_fwd(&mut self, v: CUserMsg_ParticleManager_UpdateParticleFwd) {
        self.update_particle_fwd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_fwd(&mut self) -> &mut CUserMsg_ParticleManager_UpdateParticleFwd {
        if self.update_particle_fwd.is_none() {
            self.update_particle_fwd.set_default();
        };
        self.update_particle_fwd.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_fwd(&mut self) -> CUserMsg_ParticleManager_UpdateParticleFwd {
        self.update_particle_fwd.take().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleFwd::new())
    }

    pub fn get_update_particle_fwd(&self) -> &CUserMsg_ParticleManager_UpdateParticleFwd {
        self.update_particle_fwd.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleFwd::default_instance())
    }

    fn get_update_particle_fwd_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleFwd> {
        &self.update_particle_fwd
    }

    fn mut_update_particle_fwd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleFwd> {
        &mut self.update_particle_fwd
    }

    // optional .dota.CUserMsg_ParticleManager.UpdateParticleOrient update_particle_orient = 9;

    pub fn clear_update_particle_orient(&mut self) {
        self.update_particle_orient.clear();
    }

    pub fn has_update_particle_orient(&self) -> bool {
        self.update_particle_orient.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_orient(&mut self, v: CUserMsg_ParticleManager_UpdateParticleOrient) {
        self.update_particle_orient = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_orient(&mut self) -> &mut CUserMsg_ParticleManager_UpdateParticleOrient {
        if self.update_particle_orient.is_none() {
            self.update_particle_orient.set_default();
        };
        self.update_particle_orient.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_orient(&mut self) -> CUserMsg_ParticleManager_UpdateParticleOrient {
        self.update_particle_orient.take().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleOrient::new())
    }

    pub fn get_update_particle_orient(&self) -> &CUserMsg_ParticleManager_UpdateParticleOrient {
        self.update_particle_orient.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleOrient::default_instance())
    }

    fn get_update_particle_orient_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleOrient> {
        &self.update_particle_orient
    }

    fn mut_update_particle_orient_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleOrient> {
        &mut self.update_particle_orient
    }

    // optional .dota.CUserMsg_ParticleManager.UpdateParticleFallback update_particle_fallback = 10;

    pub fn clear_update_particle_fallback(&mut self) {
        self.update_particle_fallback.clear();
    }

    pub fn has_update_particle_fallback(&self) -> bool {
        self.update_particle_fallback.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_fallback(&mut self, v: CUserMsg_ParticleManager_UpdateParticleFallback) {
        self.update_particle_fallback = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_fallback(&mut self) -> &mut CUserMsg_ParticleManager_UpdateParticleFallback {
        if self.update_particle_fallback.is_none() {
            self.update_particle_fallback.set_default();
        };
        self.update_particle_fallback.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_fallback(&mut self) -> CUserMsg_ParticleManager_UpdateParticleFallback {
        self.update_particle_fallback.take().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleFallback::new())
    }

    pub fn get_update_particle_fallback(&self) -> &CUserMsg_ParticleManager_UpdateParticleFallback {
        self.update_particle_fallback.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleFallback::default_instance())
    }

    fn get_update_particle_fallback_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleFallback> {
        &self.update_particle_fallback
    }

    fn mut_update_particle_fallback_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleFallback> {
        &mut self.update_particle_fallback
    }

    // optional .dota.CUserMsg_ParticleManager.UpdateParticleOffset update_particle_offset = 11;

    pub fn clear_update_particle_offset(&mut self) {
        self.update_particle_offset.clear();
    }

    pub fn has_update_particle_offset(&self) -> bool {
        self.update_particle_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_offset(&mut self, v: CUserMsg_ParticleManager_UpdateParticleOffset) {
        self.update_particle_offset = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_offset(&mut self) -> &mut CUserMsg_ParticleManager_UpdateParticleOffset {
        if self.update_particle_offset.is_none() {
            self.update_particle_offset.set_default();
        };
        self.update_particle_offset.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_offset(&mut self) -> CUserMsg_ParticleManager_UpdateParticleOffset {
        self.update_particle_offset.take().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleOffset::new())
    }

    pub fn get_update_particle_offset(&self) -> &CUserMsg_ParticleManager_UpdateParticleOffset {
        self.update_particle_offset.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleOffset::default_instance())
    }

    fn get_update_particle_offset_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleOffset> {
        &self.update_particle_offset
    }

    fn mut_update_particle_offset_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleOffset> {
        &mut self.update_particle_offset
    }

    // optional .dota.CUserMsg_ParticleManager.UpdateParticleEnt update_particle_ent = 12;

    pub fn clear_update_particle_ent(&mut self) {
        self.update_particle_ent.clear();
    }

    pub fn has_update_particle_ent(&self) -> bool {
        self.update_particle_ent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_ent(&mut self, v: CUserMsg_ParticleManager_UpdateParticleEnt) {
        self.update_particle_ent = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_ent(&mut self) -> &mut CUserMsg_ParticleManager_UpdateParticleEnt {
        if self.update_particle_ent.is_none() {
            self.update_particle_ent.set_default();
        };
        self.update_particle_ent.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_ent(&mut self) -> CUserMsg_ParticleManager_UpdateParticleEnt {
        self.update_particle_ent.take().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleEnt::new())
    }

    pub fn get_update_particle_ent(&self) -> &CUserMsg_ParticleManager_UpdateParticleEnt {
        self.update_particle_ent.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleEnt::default_instance())
    }

    fn get_update_particle_ent_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleEnt> {
        &self.update_particle_ent
    }

    fn mut_update_particle_ent_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleEnt> {
        &mut self.update_particle_ent
    }

    // optional .dota.CUserMsg_ParticleManager.UpdateParticleShouldDraw update_particle_should_draw = 14;

    pub fn clear_update_particle_should_draw(&mut self) {
        self.update_particle_should_draw.clear();
    }

    pub fn has_update_particle_should_draw(&self) -> bool {
        self.update_particle_should_draw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_should_draw(&mut self, v: CUserMsg_ParticleManager_UpdateParticleShouldDraw) {
        self.update_particle_should_draw = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_should_draw(&mut self) -> &mut CUserMsg_ParticleManager_UpdateParticleShouldDraw {
        if self.update_particle_should_draw.is_none() {
            self.update_particle_should_draw.set_default();
        };
        self.update_particle_should_draw.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_should_draw(&mut self) -> CUserMsg_ParticleManager_UpdateParticleShouldDraw {
        self.update_particle_should_draw.take().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleShouldDraw::new())
    }

    pub fn get_update_particle_should_draw(&self) -> &CUserMsg_ParticleManager_UpdateParticleShouldDraw {
        self.update_particle_should_draw.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleShouldDraw::default_instance())
    }

    fn get_update_particle_should_draw_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleShouldDraw> {
        &self.update_particle_should_draw
    }

    fn mut_update_particle_should_draw_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleShouldDraw> {
        &mut self.update_particle_should_draw
    }

    // optional .dota.CUserMsg_ParticleManager.UpdateParticleSetFrozen update_particle_set_frozen = 15;

    pub fn clear_update_particle_set_frozen(&mut self) {
        self.update_particle_set_frozen.clear();
    }

    pub fn has_update_particle_set_frozen(&self) -> bool {
        self.update_particle_set_frozen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_particle_set_frozen(&mut self, v: CUserMsg_ParticleManager_UpdateParticleSetFrozen) {
        self.update_particle_set_frozen = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update_particle_set_frozen(&mut self) -> &mut CUserMsg_ParticleManager_UpdateParticleSetFrozen {
        if self.update_particle_set_frozen.is_none() {
            self.update_particle_set_frozen.set_default();
        };
        self.update_particle_set_frozen.as_mut().unwrap()
    }

    // Take field
    pub fn take_update_particle_set_frozen(&mut self) -> CUserMsg_ParticleManager_UpdateParticleSetFrozen {
        self.update_particle_set_frozen.take().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleSetFrozen::new())
    }

    pub fn get_update_particle_set_frozen(&self) -> &CUserMsg_ParticleManager_UpdateParticleSetFrozen {
        self.update_particle_set_frozen.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_UpdateParticleSetFrozen::default_instance())
    }

    fn get_update_particle_set_frozen_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleSetFrozen> {
        &self.update_particle_set_frozen
    }

    fn mut_update_particle_set_frozen_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_UpdateParticleSetFrozen> {
        &mut self.update_particle_set_frozen
    }

    // optional .dota.CUserMsg_ParticleManager.ChangeControlPointAttachment change_control_point_attachment = 16;

    pub fn clear_change_control_point_attachment(&mut self) {
        self.change_control_point_attachment.clear();
    }

    pub fn has_change_control_point_attachment(&self) -> bool {
        self.change_control_point_attachment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_change_control_point_attachment(&mut self, v: CUserMsg_ParticleManager_ChangeControlPointAttachment) {
        self.change_control_point_attachment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_change_control_point_attachment(&mut self) -> &mut CUserMsg_ParticleManager_ChangeControlPointAttachment {
        if self.change_control_point_attachment.is_none() {
            self.change_control_point_attachment.set_default();
        };
        self.change_control_point_attachment.as_mut().unwrap()
    }

    // Take field
    pub fn take_change_control_point_attachment(&mut self) -> CUserMsg_ParticleManager_ChangeControlPointAttachment {
        self.change_control_point_attachment.take().unwrap_or_else(|| CUserMsg_ParticleManager_ChangeControlPointAttachment::new())
    }

    pub fn get_change_control_point_attachment(&self) -> &CUserMsg_ParticleManager_ChangeControlPointAttachment {
        self.change_control_point_attachment.as_ref().unwrap_or_else(|| CUserMsg_ParticleManager_ChangeControlPointAttachment::default_instance())
    }

    fn get_change_control_point_attachment_for_reflect(&self) -> &::protobuf::SingularPtrField<CUserMsg_ParticleManager_ChangeControlPointAttachment> {
        &self.change_control_point_attachment
    }

    fn mut_change_control_point_attachment_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CUserMsg_ParticleManager_ChangeControlPointAttachment> {
        &mut self.change_control_point_attachment
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.index.is_none() {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.release_particle_index)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.create_particle)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.destroy_particle)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.destroy_particle_involving)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_fwd)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_orient)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_fallback)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_offset)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_ent)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_should_draw)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.update_particle_set_frozen)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.change_control_point_attachment)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.release_particle_index.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.create_particle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.destroy_particle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.destroy_particle_involving.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_particle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_particle_fwd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_particle_orient.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_particle_fallback.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_particle_offset.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_particle_ent.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_particle_should_draw.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.update_particle_set_frozen.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.change_control_point_attachment.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.index {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.release_particle_index.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.create_particle.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.destroy_particle.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.destroy_particle_involving.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_particle.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_particle_fwd.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_particle_orient.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_particle_fallback.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_particle_offset.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_particle_ent.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_particle_should_draw.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.update_particle_set_frozen.as_ref() {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.change_control_point_attachment.as_ref() {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager {
    fn new() -> CUserMsg_ParticleManager {
        CUserMsg_ParticleManager::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PARTICLE_MESSAGE>>(
                    "type",
                    CUserMsg_ParticleManager::get_field_type_for_reflect,
                    CUserMsg_ParticleManager::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    CUserMsg_ParticleManager::get_index_for_reflect,
                    CUserMsg_ParticleManager::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_ReleaseParticleIndex>>(
                    "release_particle_index",
                    CUserMsg_ParticleManager::get_release_particle_index_for_reflect,
                    CUserMsg_ParticleManager::mut_release_particle_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_CreateParticle>>(
                    "create_particle",
                    CUserMsg_ParticleManager::get_create_particle_for_reflect,
                    CUserMsg_ParticleManager::mut_create_particle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_DestroyParticle>>(
                    "destroy_particle",
                    CUserMsg_ParticleManager::get_destroy_particle_for_reflect,
                    CUserMsg_ParticleManager::mut_destroy_particle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_DestroyParticleInvolving>>(
                    "destroy_particle_involving",
                    CUserMsg_ParticleManager::get_destroy_particle_involving_for_reflect,
                    CUserMsg_ParticleManager::mut_destroy_particle_involving_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_UpdateParticle>>(
                    "update_particle",
                    CUserMsg_ParticleManager::get_update_particle_for_reflect,
                    CUserMsg_ParticleManager::mut_update_particle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_UpdateParticleFwd>>(
                    "update_particle_fwd",
                    CUserMsg_ParticleManager::get_update_particle_fwd_for_reflect,
                    CUserMsg_ParticleManager::mut_update_particle_fwd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_UpdateParticleOrient>>(
                    "update_particle_orient",
                    CUserMsg_ParticleManager::get_update_particle_orient_for_reflect,
                    CUserMsg_ParticleManager::mut_update_particle_orient_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_UpdateParticleFallback>>(
                    "update_particle_fallback",
                    CUserMsg_ParticleManager::get_update_particle_fallback_for_reflect,
                    CUserMsg_ParticleManager::mut_update_particle_fallback_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_UpdateParticleOffset>>(
                    "update_particle_offset",
                    CUserMsg_ParticleManager::get_update_particle_offset_for_reflect,
                    CUserMsg_ParticleManager::mut_update_particle_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_UpdateParticleEnt>>(
                    "update_particle_ent",
                    CUserMsg_ParticleManager::get_update_particle_ent_for_reflect,
                    CUserMsg_ParticleManager::mut_update_particle_ent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_UpdateParticleShouldDraw>>(
                    "update_particle_should_draw",
                    CUserMsg_ParticleManager::get_update_particle_should_draw_for_reflect,
                    CUserMsg_ParticleManager::mut_update_particle_should_draw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_UpdateParticleSetFrozen>>(
                    "update_particle_set_frozen",
                    CUserMsg_ParticleManager::get_update_particle_set_frozen_for_reflect,
                    CUserMsg_ParticleManager::mut_update_particle_set_frozen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CUserMsg_ParticleManager_ChangeControlPointAttachment>>(
                    "change_control_point_attachment",
                    CUserMsg_ParticleManager::get_change_control_point_attachment_for_reflect,
                    CUserMsg_ParticleManager::mut_change_control_point_attachment_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager>(
                    "CUserMsg_ParticleManager",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_index();
        self.clear_release_particle_index();
        self.clear_create_particle();
        self.clear_destroy_particle();
        self.clear_destroy_particle_involving();
        self.clear_update_particle();
        self.clear_update_particle_fwd();
        self.clear_update_particle_orient();
        self.clear_update_particle_fallback();
        self.clear_update_particle_offset();
        self.clear_update_particle_ent();
        self.clear_update_particle_should_draw();
        self.clear_update_particle_set_frozen();
        self.clear_change_control_point_attachment();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_ReleaseParticleIndex {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_ReleaseParticleIndex {}

impl CUserMsg_ParticleManager_ReleaseParticleIndex {
    pub fn new() -> CUserMsg_ParticleManager_ReleaseParticleIndex {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_ReleaseParticleIndex {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_ReleaseParticleIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_ReleaseParticleIndex,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_ReleaseParticleIndex::new)
        }
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_ReleaseParticleIndex {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_ReleaseParticleIndex {
    fn new() -> CUserMsg_ParticleManager_ReleaseParticleIndex {
        CUserMsg_ParticleManager_ReleaseParticleIndex::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_ReleaseParticleIndex>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_ReleaseParticleIndex>(
                    "CUserMsg_ParticleManager_ReleaseParticleIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_ReleaseParticleIndex {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_ReleaseParticleIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_ReleaseParticleIndex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_CreateParticle {
    // message fields
    particle_name_index: ::std::option::Option<u64>,
    attach_type: ::std::option::Option<i32>,
    entity_handle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_CreateParticle {}

impl CUserMsg_ParticleManager_CreateParticle {
    pub fn new() -> CUserMsg_ParticleManager_CreateParticle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_CreateParticle {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_CreateParticle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_CreateParticle,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_CreateParticle::new)
        }
    }

    // optional fixed64 particle_name_index = 1;

    pub fn clear_particle_name_index(&mut self) {
        self.particle_name_index = ::std::option::Option::None;
    }

    pub fn has_particle_name_index(&self) -> bool {
        self.particle_name_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_particle_name_index(&mut self, v: u64) {
        self.particle_name_index = ::std::option::Option::Some(v);
    }

    pub fn get_particle_name_index(&self) -> u64 {
        self.particle_name_index.unwrap_or(0)
    }

    fn get_particle_name_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.particle_name_index
    }

    fn mut_particle_name_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.particle_name_index
    }

    // optional int32 attach_type = 2;

    pub fn clear_attach_type(&mut self) {
        self.attach_type = ::std::option::Option::None;
    }

    pub fn has_attach_type(&self) -> bool {
        self.attach_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attach_type(&mut self, v: i32) {
        self.attach_type = ::std::option::Option::Some(v);
    }

    pub fn get_attach_type(&self) -> i32 {
        self.attach_type.unwrap_or(0)
    }

    fn get_attach_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.attach_type
    }

    fn mut_attach_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.attach_type
    }

    // optional int32 entity_handle = 3;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }

    fn get_entity_handle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_handle
    }

    fn mut_entity_handle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_handle
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_CreateParticle {
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
                    self.particle_name_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.attach_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.entity_handle = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.particle_name_index {
            my_size += 9;
        };
        if let Some(v) = self.attach_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entity_handle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.particle_name_index {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.attach_type {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.entity_handle {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_CreateParticle {
    fn new() -> CUserMsg_ParticleManager_CreateParticle {
        CUserMsg_ParticleManager_CreateParticle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_CreateParticle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "particle_name_index",
                    CUserMsg_ParticleManager_CreateParticle::get_particle_name_index_for_reflect,
                    CUserMsg_ParticleManager_CreateParticle::mut_particle_name_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "attach_type",
                    CUserMsg_ParticleManager_CreateParticle::get_attach_type_for_reflect,
                    CUserMsg_ParticleManager_CreateParticle::mut_attach_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_handle",
                    CUserMsg_ParticleManager_CreateParticle::get_entity_handle_for_reflect,
                    CUserMsg_ParticleManager_CreateParticle::mut_entity_handle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_CreateParticle>(
                    "CUserMsg_ParticleManager_CreateParticle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_CreateParticle {
    fn clear(&mut self) {
        self.clear_particle_name_index();
        self.clear_attach_type();
        self.clear_entity_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_CreateParticle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_CreateParticle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_DestroyParticle {
    // message fields
    destroy_immediately: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_DestroyParticle {}

impl CUserMsg_ParticleManager_DestroyParticle {
    pub fn new() -> CUserMsg_ParticleManager_DestroyParticle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_DestroyParticle {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_DestroyParticle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_DestroyParticle,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_DestroyParticle::new)
        }
    }

    // optional bool destroy_immediately = 1;

    pub fn clear_destroy_immediately(&mut self) {
        self.destroy_immediately = ::std::option::Option::None;
    }

    pub fn has_destroy_immediately(&self) -> bool {
        self.destroy_immediately.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroy_immediately(&mut self, v: bool) {
        self.destroy_immediately = ::std::option::Option::Some(v);
    }

    pub fn get_destroy_immediately(&self) -> bool {
        self.destroy_immediately.unwrap_or(false)
    }

    fn get_destroy_immediately_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.destroy_immediately
    }

    fn mut_destroy_immediately_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.destroy_immediately
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_DestroyParticle {
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
                    self.destroy_immediately = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.destroy_immediately {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.destroy_immediately {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_DestroyParticle {
    fn new() -> CUserMsg_ParticleManager_DestroyParticle {
        CUserMsg_ParticleManager_DestroyParticle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_DestroyParticle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "destroy_immediately",
                    CUserMsg_ParticleManager_DestroyParticle::get_destroy_immediately_for_reflect,
                    CUserMsg_ParticleManager_DestroyParticle::mut_destroy_immediately_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_DestroyParticle>(
                    "CUserMsg_ParticleManager_DestroyParticle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_DestroyParticle {
    fn clear(&mut self) {
        self.clear_destroy_immediately();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_DestroyParticle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_DestroyParticle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_DestroyParticleInvolving {
    // message fields
    destroy_immediately: ::std::option::Option<bool>,
    entity_handle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_DestroyParticleInvolving {}

impl CUserMsg_ParticleManager_DestroyParticleInvolving {
    pub fn new() -> CUserMsg_ParticleManager_DestroyParticleInvolving {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_DestroyParticleInvolving {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_DestroyParticleInvolving> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_DestroyParticleInvolving,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_DestroyParticleInvolving::new)
        }
    }

    // optional bool destroy_immediately = 1;

    pub fn clear_destroy_immediately(&mut self) {
        self.destroy_immediately = ::std::option::Option::None;
    }

    pub fn has_destroy_immediately(&self) -> bool {
        self.destroy_immediately.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destroy_immediately(&mut self, v: bool) {
        self.destroy_immediately = ::std::option::Option::Some(v);
    }

    pub fn get_destroy_immediately(&self) -> bool {
        self.destroy_immediately.unwrap_or(false)
    }

    fn get_destroy_immediately_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.destroy_immediately
    }

    fn mut_destroy_immediately_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.destroy_immediately
    }

    // optional int32 entity_handle = 3;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }

    fn get_entity_handle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_handle
    }

    fn mut_entity_handle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_handle
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_DestroyParticleInvolving {
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
                    self.destroy_immediately = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.entity_handle = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.destroy_immediately {
            my_size += 2;
        };
        if let Some(v) = self.entity_handle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.destroy_immediately {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.entity_handle {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_DestroyParticleInvolving {
    fn new() -> CUserMsg_ParticleManager_DestroyParticleInvolving {
        CUserMsg_ParticleManager_DestroyParticleInvolving::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_DestroyParticleInvolving>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "destroy_immediately",
                    CUserMsg_ParticleManager_DestroyParticleInvolving::get_destroy_immediately_for_reflect,
                    CUserMsg_ParticleManager_DestroyParticleInvolving::mut_destroy_immediately_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_handle",
                    CUserMsg_ParticleManager_DestroyParticleInvolving::get_entity_handle_for_reflect,
                    CUserMsg_ParticleManager_DestroyParticleInvolving::mut_entity_handle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_DestroyParticleInvolving>(
                    "CUserMsg_ParticleManager_DestroyParticleInvolving",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_DestroyParticleInvolving {
    fn clear(&mut self) {
        self.clear_destroy_immediately();
        self.clear_entity_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_DestroyParticleInvolving {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_DestroyParticleInvolving {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_UpdateParticle {
    // message fields
    control_point: ::std::option::Option<i32>,
    position: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_UpdateParticle {}

impl CUserMsg_ParticleManager_UpdateParticle {
    pub fn new() -> CUserMsg_ParticleManager_UpdateParticle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_UpdateParticle {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_UpdateParticle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_UpdateParticle,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_UpdateParticle::new)
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    fn get_control_point_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.control_point
    }

    fn mut_control_point_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.control_point
    }

    // optional .dota.CMsgVector position = 2;

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
}

impl ::protobuf::Message for CUserMsg_ParticleManager_UpdateParticle {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.position)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.control_point {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.position.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.position.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_UpdateParticle {
    fn new() -> CUserMsg_ParticleManager_UpdateParticle {
        CUserMsg_ParticleManager_UpdateParticle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_UpdateParticle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "control_point",
                    CUserMsg_ParticleManager_UpdateParticle::get_control_point_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticle::mut_control_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "position",
                    CUserMsg_ParticleManager_UpdateParticle::get_position_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticle::mut_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_UpdateParticle>(
                    "CUserMsg_ParticleManager_UpdateParticle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_UpdateParticle {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_UpdateParticle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_UpdateParticle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_UpdateParticleFwd {
    // message fields
    control_point: ::std::option::Option<i32>,
    forward: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_UpdateParticleFwd {}

impl CUserMsg_ParticleManager_UpdateParticleFwd {
    pub fn new() -> CUserMsg_ParticleManager_UpdateParticleFwd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_UpdateParticleFwd {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_UpdateParticleFwd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_UpdateParticleFwd,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_UpdateParticleFwd::new)
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    fn get_control_point_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.control_point
    }

    fn mut_control_point_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.control_point
    }

    // optional .dota.CMsgVector forward = 2;

    pub fn clear_forward(&mut self) {
        self.forward.clear();
    }

    pub fn has_forward(&self) -> bool {
        self.forward.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forward(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.forward = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forward(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.forward.is_none() {
            self.forward.set_default();
        };
        self.forward.as_mut().unwrap()
    }

    // Take field
    pub fn take_forward(&mut self) -> super::networkbasetypes::CMsgVector {
        self.forward.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_forward(&self) -> &super::networkbasetypes::CMsgVector {
        self.forward.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_forward_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.forward
    }

    fn mut_forward_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.forward
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_UpdateParticleFwd {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.forward)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.control_point {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.forward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.forward.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_UpdateParticleFwd {
    fn new() -> CUserMsg_ParticleManager_UpdateParticleFwd {
        CUserMsg_ParticleManager_UpdateParticleFwd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_UpdateParticleFwd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "control_point",
                    CUserMsg_ParticleManager_UpdateParticleFwd::get_control_point_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleFwd::mut_control_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "forward",
                    CUserMsg_ParticleManager_UpdateParticleFwd::get_forward_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleFwd::mut_forward_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_UpdateParticleFwd>(
                    "CUserMsg_ParticleManager_UpdateParticleFwd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_UpdateParticleFwd {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_forward();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_UpdateParticleFwd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_UpdateParticleFwd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_UpdateParticleOrient {
    // message fields
    control_point: ::std::option::Option<i32>,
    forward: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    right: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    up: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_UpdateParticleOrient {}

impl CUserMsg_ParticleManager_UpdateParticleOrient {
    pub fn new() -> CUserMsg_ParticleManager_UpdateParticleOrient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_UpdateParticleOrient {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_UpdateParticleOrient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_UpdateParticleOrient,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_UpdateParticleOrient::new)
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    fn get_control_point_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.control_point
    }

    fn mut_control_point_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.control_point
    }

    // optional .dota.CMsgVector forward = 2;

    pub fn clear_forward(&mut self) {
        self.forward.clear();
    }

    pub fn has_forward(&self) -> bool {
        self.forward.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forward(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.forward = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forward(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.forward.is_none() {
            self.forward.set_default();
        };
        self.forward.as_mut().unwrap()
    }

    // Take field
    pub fn take_forward(&mut self) -> super::networkbasetypes::CMsgVector {
        self.forward.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_forward(&self) -> &super::networkbasetypes::CMsgVector {
        self.forward.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_forward_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.forward
    }

    fn mut_forward_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.forward
    }

    // optional .dota.CMsgVector right = 3;

    pub fn clear_right(&mut self) {
        self.right.clear();
    }

    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.right = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_right(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.right.is_none() {
            self.right.set_default();
        };
        self.right.as_mut().unwrap()
    }

    // Take field
    pub fn take_right(&mut self) -> super::networkbasetypes::CMsgVector {
        self.right.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_right(&self) -> &super::networkbasetypes::CMsgVector {
        self.right.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_right_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.right
    }

    fn mut_right_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.right
    }

    // optional .dota.CMsgVector up = 4;

    pub fn clear_up(&mut self) {
        self.up.clear();
    }

    pub fn has_up(&self) -> bool {
        self.up.is_some()
    }

    // Param is passed by value, moved
    pub fn set_up(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.up = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_up(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.up.is_none() {
            self.up.set_default();
        };
        self.up.as_mut().unwrap()
    }

    // Take field
    pub fn take_up(&mut self) -> super::networkbasetypes::CMsgVector {
        self.up.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_up(&self) -> &super::networkbasetypes::CMsgVector {
        self.up.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_up_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.up
    }

    fn mut_up_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.up
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_UpdateParticleOrient {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.forward)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.right)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.up)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.control_point {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.forward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.right.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.up.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.forward.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.right.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.up.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_UpdateParticleOrient {
    fn new() -> CUserMsg_ParticleManager_UpdateParticleOrient {
        CUserMsg_ParticleManager_UpdateParticleOrient::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_UpdateParticleOrient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "control_point",
                    CUserMsg_ParticleManager_UpdateParticleOrient::get_control_point_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleOrient::mut_control_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "forward",
                    CUserMsg_ParticleManager_UpdateParticleOrient::get_forward_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleOrient::mut_forward_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "right",
                    CUserMsg_ParticleManager_UpdateParticleOrient::get_right_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleOrient::mut_right_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "up",
                    CUserMsg_ParticleManager_UpdateParticleOrient::get_up_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleOrient::mut_up_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_UpdateParticleOrient>(
                    "CUserMsg_ParticleManager_UpdateParticleOrient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_UpdateParticleOrient {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_forward();
        self.clear_right();
        self.clear_up();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_UpdateParticleOrient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_UpdateParticleOrient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_UpdateParticleFallback {
    // message fields
    control_point: ::std::option::Option<i32>,
    position: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_UpdateParticleFallback {}

impl CUserMsg_ParticleManager_UpdateParticleFallback {
    pub fn new() -> CUserMsg_ParticleManager_UpdateParticleFallback {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_UpdateParticleFallback {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_UpdateParticleFallback> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_UpdateParticleFallback,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_UpdateParticleFallback::new)
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    fn get_control_point_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.control_point
    }

    fn mut_control_point_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.control_point
    }

    // optional .dota.CMsgVector position = 2;

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
}

impl ::protobuf::Message for CUserMsg_ParticleManager_UpdateParticleFallback {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.position)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.control_point {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.position.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.position.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_UpdateParticleFallback {
    fn new() -> CUserMsg_ParticleManager_UpdateParticleFallback {
        CUserMsg_ParticleManager_UpdateParticleFallback::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_UpdateParticleFallback>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "control_point",
                    CUserMsg_ParticleManager_UpdateParticleFallback::get_control_point_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleFallback::mut_control_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "position",
                    CUserMsg_ParticleManager_UpdateParticleFallback::get_position_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleFallback::mut_position_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_UpdateParticleFallback>(
                    "CUserMsg_ParticleManager_UpdateParticleFallback",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_UpdateParticleFallback {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_position();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_UpdateParticleFallback {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_UpdateParticleFallback {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_UpdateParticleOffset {
    // message fields
    control_point: ::std::option::Option<i32>,
    origin_offset: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_UpdateParticleOffset {}

impl CUserMsg_ParticleManager_UpdateParticleOffset {
    pub fn new() -> CUserMsg_ParticleManager_UpdateParticleOffset {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_UpdateParticleOffset {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_UpdateParticleOffset> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_UpdateParticleOffset,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_UpdateParticleOffset::new)
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    fn get_control_point_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.control_point
    }

    fn mut_control_point_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.control_point
    }

    // optional .dota.CMsgVector origin_offset = 2;

    pub fn clear_origin_offset(&mut self) {
        self.origin_offset.clear();
    }

    pub fn has_origin_offset(&self) -> bool {
        self.origin_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_offset(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.origin_offset = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin_offset(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.origin_offset.is_none() {
            self.origin_offset.set_default();
        };
        self.origin_offset.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin_offset(&mut self) -> super::networkbasetypes::CMsgVector {
        self.origin_offset.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_origin_offset(&self) -> &super::networkbasetypes::CMsgVector {
        self.origin_offset.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_origin_offset_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.origin_offset
    }

    fn mut_origin_offset_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.origin_offset
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_UpdateParticleOffset {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin_offset)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.control_point {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin_offset.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.origin_offset.as_ref() {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_UpdateParticleOffset {
    fn new() -> CUserMsg_ParticleManager_UpdateParticleOffset {
        CUserMsg_ParticleManager_UpdateParticleOffset::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_UpdateParticleOffset>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "control_point",
                    CUserMsg_ParticleManager_UpdateParticleOffset::get_control_point_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleOffset::mut_control_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin_offset",
                    CUserMsg_ParticleManager_UpdateParticleOffset::get_origin_offset_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleOffset::mut_origin_offset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_UpdateParticleOffset>(
                    "CUserMsg_ParticleManager_UpdateParticleOffset",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_UpdateParticleOffset {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_origin_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_UpdateParticleOffset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_UpdateParticleOffset {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_UpdateParticleEnt {
    // message fields
    control_point: ::std::option::Option<i32>,
    entity_handle: ::std::option::Option<i32>,
    attach_type: ::std::option::Option<i32>,
    attachment: ::std::option::Option<i32>,
    fallback_position: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    include_wearables: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_UpdateParticleEnt {}

impl CUserMsg_ParticleManager_UpdateParticleEnt {
    pub fn new() -> CUserMsg_ParticleManager_UpdateParticleEnt {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_UpdateParticleEnt {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_UpdateParticleEnt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_UpdateParticleEnt,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_UpdateParticleEnt::new)
        }
    }

    // optional int32 control_point = 1;

    pub fn clear_control_point(&mut self) {
        self.control_point = ::std::option::Option::None;
    }

    pub fn has_control_point(&self) -> bool {
        self.control_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_point(&mut self, v: i32) {
        self.control_point = ::std::option::Option::Some(v);
    }

    pub fn get_control_point(&self) -> i32 {
        self.control_point.unwrap_or(0)
    }

    fn get_control_point_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.control_point
    }

    fn mut_control_point_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.control_point
    }

    // optional int32 entity_handle = 2;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }

    fn get_entity_handle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_handle
    }

    fn mut_entity_handle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_handle
    }

    // optional int32 attach_type = 3;

    pub fn clear_attach_type(&mut self) {
        self.attach_type = ::std::option::Option::None;
    }

    pub fn has_attach_type(&self) -> bool {
        self.attach_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attach_type(&mut self, v: i32) {
        self.attach_type = ::std::option::Option::Some(v);
    }

    pub fn get_attach_type(&self) -> i32 {
        self.attach_type.unwrap_or(0)
    }

    fn get_attach_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.attach_type
    }

    fn mut_attach_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.attach_type
    }

    // optional int32 attachment = 4;

    pub fn clear_attachment(&mut self) {
        self.attachment = ::std::option::Option::None;
    }

    pub fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attachment(&mut self, v: i32) {
        self.attachment = ::std::option::Option::Some(v);
    }

    pub fn get_attachment(&self) -> i32 {
        self.attachment.unwrap_or(0)
    }

    fn get_attachment_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.attachment
    }

    fn mut_attachment_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.attachment
    }

    // optional .dota.CMsgVector fallback_position = 5;

    pub fn clear_fallback_position(&mut self) {
        self.fallback_position.clear();
    }

    pub fn has_fallback_position(&self) -> bool {
        self.fallback_position.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fallback_position(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.fallback_position = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fallback_position(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.fallback_position.is_none() {
            self.fallback_position.set_default();
        };
        self.fallback_position.as_mut().unwrap()
    }

    // Take field
    pub fn take_fallback_position(&mut self) -> super::networkbasetypes::CMsgVector {
        self.fallback_position.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_fallback_position(&self) -> &super::networkbasetypes::CMsgVector {
        self.fallback_position.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_fallback_position_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.fallback_position
    }

    fn mut_fallback_position_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.fallback_position
    }

    // optional bool include_wearables = 6;

    pub fn clear_include_wearables(&mut self) {
        self.include_wearables = ::std::option::Option::None;
    }

    pub fn has_include_wearables(&self) -> bool {
        self.include_wearables.is_some()
    }

    // Param is passed by value, moved
    pub fn set_include_wearables(&mut self, v: bool) {
        self.include_wearables = ::std::option::Option::Some(v);
    }

    pub fn get_include_wearables(&self) -> bool {
        self.include_wearables.unwrap_or(false)
    }

    fn get_include_wearables_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.include_wearables
    }

    fn mut_include_wearables_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.include_wearables
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_UpdateParticleEnt {
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
                    self.control_point = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.entity_handle = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.attach_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.attachment = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fallback_position)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.include_wearables = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.control_point {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entity_handle {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attach_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attachment {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.fallback_position.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.include_wearables {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_point {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.entity_handle {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.attach_type {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.attachment {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.fallback_position.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.include_wearables {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_UpdateParticleEnt {
    fn new() -> CUserMsg_ParticleManager_UpdateParticleEnt {
        CUserMsg_ParticleManager_UpdateParticleEnt::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_UpdateParticleEnt>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "control_point",
                    CUserMsg_ParticleManager_UpdateParticleEnt::get_control_point_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleEnt::mut_control_point_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_handle",
                    CUserMsg_ParticleManager_UpdateParticleEnt::get_entity_handle_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleEnt::mut_entity_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "attach_type",
                    CUserMsg_ParticleManager_UpdateParticleEnt::get_attach_type_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleEnt::mut_attach_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "attachment",
                    CUserMsg_ParticleManager_UpdateParticleEnt::get_attachment_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleEnt::mut_attachment_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "fallback_position",
                    CUserMsg_ParticleManager_UpdateParticleEnt::get_fallback_position_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleEnt::mut_fallback_position_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "include_wearables",
                    CUserMsg_ParticleManager_UpdateParticleEnt::get_include_wearables_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleEnt::mut_include_wearables_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_UpdateParticleEnt>(
                    "CUserMsg_ParticleManager_UpdateParticleEnt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_UpdateParticleEnt {
    fn clear(&mut self) {
        self.clear_control_point();
        self.clear_entity_handle();
        self.clear_attach_type();
        self.clear_attachment();
        self.clear_fallback_position();
        self.clear_include_wearables();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_UpdateParticleEnt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_UpdateParticleEnt {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_UpdateParticleSetFrozen {
    // message fields
    set_frozen: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_UpdateParticleSetFrozen {}

impl CUserMsg_ParticleManager_UpdateParticleSetFrozen {
    pub fn new() -> CUserMsg_ParticleManager_UpdateParticleSetFrozen {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_UpdateParticleSetFrozen {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_UpdateParticleSetFrozen> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_UpdateParticleSetFrozen,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_UpdateParticleSetFrozen::new)
        }
    }

    // optional bool set_frozen = 1;

    pub fn clear_set_frozen(&mut self) {
        self.set_frozen = ::std::option::Option::None;
    }

    pub fn has_set_frozen(&self) -> bool {
        self.set_frozen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set_frozen(&mut self, v: bool) {
        self.set_frozen = ::std::option::Option::Some(v);
    }

    pub fn get_set_frozen(&self) -> bool {
        self.set_frozen.unwrap_or(false)
    }

    fn get_set_frozen_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.set_frozen
    }

    fn mut_set_frozen_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.set_frozen
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_UpdateParticleSetFrozen {
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
                    self.set_frozen = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.set_frozen {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.set_frozen {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_UpdateParticleSetFrozen {
    fn new() -> CUserMsg_ParticleManager_UpdateParticleSetFrozen {
        CUserMsg_ParticleManager_UpdateParticleSetFrozen::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_UpdateParticleSetFrozen>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "set_frozen",
                    CUserMsg_ParticleManager_UpdateParticleSetFrozen::get_set_frozen_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleSetFrozen::mut_set_frozen_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_UpdateParticleSetFrozen>(
                    "CUserMsg_ParticleManager_UpdateParticleSetFrozen",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_UpdateParticleSetFrozen {
    fn clear(&mut self) {
        self.clear_set_frozen();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_UpdateParticleSetFrozen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_UpdateParticleSetFrozen {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_UpdateParticleShouldDraw {
    // message fields
    should_draw: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_UpdateParticleShouldDraw {}

impl CUserMsg_ParticleManager_UpdateParticleShouldDraw {
    pub fn new() -> CUserMsg_ParticleManager_UpdateParticleShouldDraw {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_UpdateParticleShouldDraw {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_UpdateParticleShouldDraw> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_UpdateParticleShouldDraw,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_UpdateParticleShouldDraw::new)
        }
    }

    // optional bool should_draw = 1;

    pub fn clear_should_draw(&mut self) {
        self.should_draw = ::std::option::Option::None;
    }

    pub fn has_should_draw(&self) -> bool {
        self.should_draw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_should_draw(&mut self, v: bool) {
        self.should_draw = ::std::option::Option::Some(v);
    }

    pub fn get_should_draw(&self) -> bool {
        self.should_draw.unwrap_or(false)
    }

    fn get_should_draw_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.should_draw
    }

    fn mut_should_draw_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.should_draw
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_UpdateParticleShouldDraw {
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
                    self.should_draw = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.should_draw {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.should_draw {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_UpdateParticleShouldDraw {
    fn new() -> CUserMsg_ParticleManager_UpdateParticleShouldDraw {
        CUserMsg_ParticleManager_UpdateParticleShouldDraw::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_UpdateParticleShouldDraw>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "should_draw",
                    CUserMsg_ParticleManager_UpdateParticleShouldDraw::get_should_draw_for_reflect,
                    CUserMsg_ParticleManager_UpdateParticleShouldDraw::mut_should_draw_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_UpdateParticleShouldDraw>(
                    "CUserMsg_ParticleManager_UpdateParticleShouldDraw",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_UpdateParticleShouldDraw {
    fn clear(&mut self) {
        self.clear_should_draw();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_UpdateParticleShouldDraw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_UpdateParticleShouldDraw {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_ParticleManager_ChangeControlPointAttachment {
    // message fields
    attachment_old: ::std::option::Option<i32>,
    attachment_new: ::std::option::Option<i32>,
    entity_handle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_ParticleManager_ChangeControlPointAttachment {}

impl CUserMsg_ParticleManager_ChangeControlPointAttachment {
    pub fn new() -> CUserMsg_ParticleManager_ChangeControlPointAttachment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_ParticleManager_ChangeControlPointAttachment {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_ParticleManager_ChangeControlPointAttachment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_ParticleManager_ChangeControlPointAttachment,
        };
        unsafe {
            instance.get(CUserMsg_ParticleManager_ChangeControlPointAttachment::new)
        }
    }

    // optional int32 attachment_old = 1;

    pub fn clear_attachment_old(&mut self) {
        self.attachment_old = ::std::option::Option::None;
    }

    pub fn has_attachment_old(&self) -> bool {
        self.attachment_old.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attachment_old(&mut self, v: i32) {
        self.attachment_old = ::std::option::Option::Some(v);
    }

    pub fn get_attachment_old(&self) -> i32 {
        self.attachment_old.unwrap_or(0)
    }

    fn get_attachment_old_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.attachment_old
    }

    fn mut_attachment_old_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.attachment_old
    }

    // optional int32 attachment_new = 2;

    pub fn clear_attachment_new(&mut self) {
        self.attachment_new = ::std::option::Option::None;
    }

    pub fn has_attachment_new(&self) -> bool {
        self.attachment_new.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attachment_new(&mut self, v: i32) {
        self.attachment_new = ::std::option::Option::Some(v);
    }

    pub fn get_attachment_new(&self) -> i32 {
        self.attachment_new.unwrap_or(0)
    }

    fn get_attachment_new_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.attachment_new
    }

    fn mut_attachment_new_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.attachment_new
    }

    // optional int32 entity_handle = 3;

    pub fn clear_entity_handle(&mut self) {
        self.entity_handle = ::std::option::Option::None;
    }

    pub fn has_entity_handle(&self) -> bool {
        self.entity_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_handle(&mut self, v: i32) {
        self.entity_handle = ::std::option::Option::Some(v);
    }

    pub fn get_entity_handle(&self) -> i32 {
        self.entity_handle.unwrap_or(0)
    }

    fn get_entity_handle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_handle
    }

    fn mut_entity_handle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_handle
    }
}

impl ::protobuf::Message for CUserMsg_ParticleManager_ChangeControlPointAttachment {
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
                    self.attachment_old = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.attachment_new = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.entity_handle = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.attachment_old {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attachment_new {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entity_handle {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.attachment_old {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.attachment_new {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.entity_handle {
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

impl ::protobuf::MessageStatic for CUserMsg_ParticleManager_ChangeControlPointAttachment {
    fn new() -> CUserMsg_ParticleManager_ChangeControlPointAttachment {
        CUserMsg_ParticleManager_ChangeControlPointAttachment::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_ParticleManager_ChangeControlPointAttachment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "attachment_old",
                    CUserMsg_ParticleManager_ChangeControlPointAttachment::get_attachment_old_for_reflect,
                    CUserMsg_ParticleManager_ChangeControlPointAttachment::mut_attachment_old_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "attachment_new",
                    CUserMsg_ParticleManager_ChangeControlPointAttachment::get_attachment_new_for_reflect,
                    CUserMsg_ParticleManager_ChangeControlPointAttachment::mut_attachment_new_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_handle",
                    CUserMsg_ParticleManager_ChangeControlPointAttachment::get_entity_handle_for_reflect,
                    CUserMsg_ParticleManager_ChangeControlPointAttachment::mut_entity_handle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_ParticleManager_ChangeControlPointAttachment>(
                    "CUserMsg_ParticleManager_ChangeControlPointAttachment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_ParticleManager_ChangeControlPointAttachment {
    fn clear(&mut self) {
        self.clear_attachment_old();
        self.clear_attachment_new();
        self.clear_entity_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_ParticleManager_ChangeControlPointAttachment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_ParticleManager_ChangeControlPointAttachment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_HudError {
    // message fields
    order_id: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_HudError {}

impl CUserMsg_HudError {
    pub fn new() -> CUserMsg_HudError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_HudError {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_HudError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_HudError,
        };
        unsafe {
            instance.get(CUserMsg_HudError::new)
        }
    }

    // optional int32 order_id = 1;

    pub fn clear_order_id(&mut self) {
        self.order_id = ::std::option::Option::None;
    }

    pub fn has_order_id(&self) -> bool {
        self.order_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_order_id(&mut self, v: i32) {
        self.order_id = ::std::option::Option::Some(v);
    }

    pub fn get_order_id(&self) -> i32 {
        self.order_id.unwrap_or(0)
    }

    fn get_order_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.order_id
    }

    fn mut_order_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.order_id
    }
}

impl ::protobuf::Message for CUserMsg_HudError {
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
                    self.order_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.order_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.order_id {
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

impl ::protobuf::MessageStatic for CUserMsg_HudError {
    fn new() -> CUserMsg_HudError {
        CUserMsg_HudError::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_HudError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "order_id",
                    CUserMsg_HudError::get_order_id_for_reflect,
                    CUserMsg_HudError::mut_order_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_HudError>(
                    "CUserMsg_HudError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_HudError {
    fn clear(&mut self) {
        self.clear_order_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_HudError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_HudError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMsg_CustomGameEvent {
    // message fields
    event_name: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMsg_CustomGameEvent {}

impl CUserMsg_CustomGameEvent {
    pub fn new() -> CUserMsg_CustomGameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMsg_CustomGameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CUserMsg_CustomGameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMsg_CustomGameEvent,
        };
        unsafe {
            instance.get(CUserMsg_CustomGameEvent::new)
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

impl ::protobuf::Message for CUserMsg_CustomGameEvent {
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

impl ::protobuf::MessageStatic for CUserMsg_CustomGameEvent {
    fn new() -> CUserMsg_CustomGameEvent {
        CUserMsg_CustomGameEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMsg_CustomGameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event_name",
                    CUserMsg_CustomGameEvent::get_event_name_for_reflect,
                    CUserMsg_CustomGameEvent::mut_event_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CUserMsg_CustomGameEvent::get_data_for_reflect,
                    CUserMsg_CustomGameEvent::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMsg_CustomGameEvent>(
                    "CUserMsg_CustomGameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMsg_CustomGameEvent {
    fn clear(&mut self) {
        self.clear_event_name();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMsg_CustomGameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMsg_CustomGameEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CUserMessageHapticPulse {
    // message fields
    hand_id: ::std::option::Option<i32>,
    pulse_type: ::std::option::Option<EHapticPulseType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CUserMessageHapticPulse {}

impl CUserMessageHapticPulse {
    pub fn new() -> CUserMessageHapticPulse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CUserMessageHapticPulse {
        static mut instance: ::protobuf::lazy::Lazy<CUserMessageHapticPulse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CUserMessageHapticPulse,
        };
        unsafe {
            instance.get(CUserMessageHapticPulse::new)
        }
    }

    // optional int32 hand_id = 1;

    pub fn clear_hand_id(&mut self) {
        self.hand_id = ::std::option::Option::None;
    }

    pub fn has_hand_id(&self) -> bool {
        self.hand_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hand_id(&mut self, v: i32) {
        self.hand_id = ::std::option::Option::Some(v);
    }

    pub fn get_hand_id(&self) -> i32 {
        self.hand_id.unwrap_or(0)
    }

    fn get_hand_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.hand_id
    }

    fn mut_hand_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.hand_id
    }

    // optional .dota.EHapticPulseType pulse_type = 2;

    pub fn clear_pulse_type(&mut self) {
        self.pulse_type = ::std::option::Option::None;
    }

    pub fn has_pulse_type(&self) -> bool {
        self.pulse_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pulse_type(&mut self, v: EHapticPulseType) {
        self.pulse_type = ::std::option::Option::Some(v);
    }

    pub fn get_pulse_type(&self) -> EHapticPulseType {
        self.pulse_type.unwrap_or(EHapticPulseType::VR_HAND_HAPTIC_PULSE_LIGHT)
    }

    fn get_pulse_type_for_reflect(&self) -> &::std::option::Option<EHapticPulseType> {
        &self.pulse_type
    }

    fn mut_pulse_type_for_reflect(&mut self) -> &mut ::std::option::Option<EHapticPulseType> {
        &mut self.pulse_type
    }
}

impl ::protobuf::Message for CUserMessageHapticPulse {
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
                    self.hand_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.pulse_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.hand_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pulse_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hand_id {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.pulse_type {
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

impl ::protobuf::MessageStatic for CUserMessageHapticPulse {
    fn new() -> CUserMessageHapticPulse {
        CUserMessageHapticPulse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CUserMessageHapticPulse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "hand_id",
                    CUserMessageHapticPulse::get_hand_id_for_reflect,
                    CUserMessageHapticPulse::mut_hand_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EHapticPulseType>>(
                    "pulse_type",
                    CUserMessageHapticPulse::get_pulse_type_for_reflect,
                    CUserMessageHapticPulse::mut_pulse_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CUserMessageHapticPulse>(
                    "CUserMessageHapticPulse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CUserMessageHapticPulse {
    fn clear(&mut self) {
        self.clear_hand_id();
        self.clear_pulse_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CUserMessageHapticPulse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CUserMessageHapticPulse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EBaseUserMessages {
    UM_AchievementEvent = 101,
    UM_CloseCaption = 102,
    UM_CloseCaptionDirect = 103,
    UM_CurrentTimescale = 104,
    UM_DesiredTimescale = 105,
    UM_Fade = 106,
    UM_GameTitle = 107,
    UM_HintText = 109,
    UM_HudMsg = 110,
    UM_HudText = 111,
    UM_KeyHintText = 112,
    UM_ColoredText = 113,
    UM_RequestState = 114,
    UM_ResetHUD = 115,
    UM_Rumble = 116,
    UM_SayText = 117,
    UM_SayText2 = 118,
    UM_SayTextChannel = 119,
    UM_Shake = 120,
    UM_ShakeDir = 121,
    UM_TextMsg = 124,
    UM_ScreenTilt = 125,
    UM_Train = 126,
    UM_VGUIMenu = 127,
    UM_VoiceMask = 128,
    UM_VoiceSubtitle = 129,
    UM_SendAudio = 130,
    UM_ItemPickup = 131,
    UM_AmmoDenied = 132,
    UM_CrosshairAngle = 133,
    UM_ShowMenu = 134,
    UM_CreditsMsg = 135,
    UM_CloseCaptionPlaceholder = 142,
    UM_CameraTransition = 143,
    UM_AudioParameter = 144,
    UM_ParticleManager = 145,
    UM_HudError = 146,
    UM_CustomGameEvent = 148,
    UM_HandHapticPulse = 149,
    UM_MAX_BASE = 200,
}

impl ::protobuf::ProtobufEnum for EBaseUserMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EBaseUserMessages> {
        match value {
            101 => ::std::option::Option::Some(EBaseUserMessages::UM_AchievementEvent),
            102 => ::std::option::Option::Some(EBaseUserMessages::UM_CloseCaption),
            103 => ::std::option::Option::Some(EBaseUserMessages::UM_CloseCaptionDirect),
            104 => ::std::option::Option::Some(EBaseUserMessages::UM_CurrentTimescale),
            105 => ::std::option::Option::Some(EBaseUserMessages::UM_DesiredTimescale),
            106 => ::std::option::Option::Some(EBaseUserMessages::UM_Fade),
            107 => ::std::option::Option::Some(EBaseUserMessages::UM_GameTitle),
            109 => ::std::option::Option::Some(EBaseUserMessages::UM_HintText),
            110 => ::std::option::Option::Some(EBaseUserMessages::UM_HudMsg),
            111 => ::std::option::Option::Some(EBaseUserMessages::UM_HudText),
            112 => ::std::option::Option::Some(EBaseUserMessages::UM_KeyHintText),
            113 => ::std::option::Option::Some(EBaseUserMessages::UM_ColoredText),
            114 => ::std::option::Option::Some(EBaseUserMessages::UM_RequestState),
            115 => ::std::option::Option::Some(EBaseUserMessages::UM_ResetHUD),
            116 => ::std::option::Option::Some(EBaseUserMessages::UM_Rumble),
            117 => ::std::option::Option::Some(EBaseUserMessages::UM_SayText),
            118 => ::std::option::Option::Some(EBaseUserMessages::UM_SayText2),
            119 => ::std::option::Option::Some(EBaseUserMessages::UM_SayTextChannel),
            120 => ::std::option::Option::Some(EBaseUserMessages::UM_Shake),
            121 => ::std::option::Option::Some(EBaseUserMessages::UM_ShakeDir),
            124 => ::std::option::Option::Some(EBaseUserMessages::UM_TextMsg),
            125 => ::std::option::Option::Some(EBaseUserMessages::UM_ScreenTilt),
            126 => ::std::option::Option::Some(EBaseUserMessages::UM_Train),
            127 => ::std::option::Option::Some(EBaseUserMessages::UM_VGUIMenu),
            128 => ::std::option::Option::Some(EBaseUserMessages::UM_VoiceMask),
            129 => ::std::option::Option::Some(EBaseUserMessages::UM_VoiceSubtitle),
            130 => ::std::option::Option::Some(EBaseUserMessages::UM_SendAudio),
            131 => ::std::option::Option::Some(EBaseUserMessages::UM_ItemPickup),
            132 => ::std::option::Option::Some(EBaseUserMessages::UM_AmmoDenied),
            133 => ::std::option::Option::Some(EBaseUserMessages::UM_CrosshairAngle),
            134 => ::std::option::Option::Some(EBaseUserMessages::UM_ShowMenu),
            135 => ::std::option::Option::Some(EBaseUserMessages::UM_CreditsMsg),
            142 => ::std::option::Option::Some(EBaseUserMessages::UM_CloseCaptionPlaceholder),
            143 => ::std::option::Option::Some(EBaseUserMessages::UM_CameraTransition),
            144 => ::std::option::Option::Some(EBaseUserMessages::UM_AudioParameter),
            145 => ::std::option::Option::Some(EBaseUserMessages::UM_ParticleManager),
            146 => ::std::option::Option::Some(EBaseUserMessages::UM_HudError),
            148 => ::std::option::Option::Some(EBaseUserMessages::UM_CustomGameEvent),
            149 => ::std::option::Option::Some(EBaseUserMessages::UM_HandHapticPulse),
            200 => ::std::option::Option::Some(EBaseUserMessages::UM_MAX_BASE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EBaseUserMessages] = &[
            EBaseUserMessages::UM_AchievementEvent,
            EBaseUserMessages::UM_CloseCaption,
            EBaseUserMessages::UM_CloseCaptionDirect,
            EBaseUserMessages::UM_CurrentTimescale,
            EBaseUserMessages::UM_DesiredTimescale,
            EBaseUserMessages::UM_Fade,
            EBaseUserMessages::UM_GameTitle,
            EBaseUserMessages::UM_HintText,
            EBaseUserMessages::UM_HudMsg,
            EBaseUserMessages::UM_HudText,
            EBaseUserMessages::UM_KeyHintText,
            EBaseUserMessages::UM_ColoredText,
            EBaseUserMessages::UM_RequestState,
            EBaseUserMessages::UM_ResetHUD,
            EBaseUserMessages::UM_Rumble,
            EBaseUserMessages::UM_SayText,
            EBaseUserMessages::UM_SayText2,
            EBaseUserMessages::UM_SayTextChannel,
            EBaseUserMessages::UM_Shake,
            EBaseUserMessages::UM_ShakeDir,
            EBaseUserMessages::UM_TextMsg,
            EBaseUserMessages::UM_ScreenTilt,
            EBaseUserMessages::UM_Train,
            EBaseUserMessages::UM_VGUIMenu,
            EBaseUserMessages::UM_VoiceMask,
            EBaseUserMessages::UM_VoiceSubtitle,
            EBaseUserMessages::UM_SendAudio,
            EBaseUserMessages::UM_ItemPickup,
            EBaseUserMessages::UM_AmmoDenied,
            EBaseUserMessages::UM_CrosshairAngle,
            EBaseUserMessages::UM_ShowMenu,
            EBaseUserMessages::UM_CreditsMsg,
            EBaseUserMessages::UM_CloseCaptionPlaceholder,
            EBaseUserMessages::UM_CameraTransition,
            EBaseUserMessages::UM_AudioParameter,
            EBaseUserMessages::UM_ParticleManager,
            EBaseUserMessages::UM_HudError,
            EBaseUserMessages::UM_CustomGameEvent,
            EBaseUserMessages::UM_HandHapticPulse,
            EBaseUserMessages::UM_MAX_BASE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EBaseUserMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EBaseUserMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EBaseUserMessages {
}

impl ::protobuf::reflect::ProtobufValue for EBaseUserMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EBaseEntityMessages {
    EM_PlayJingle = 136,
    EM_ScreenOverlay = 137,
    EM_RemoveAllDecals = 138,
    EM_PropagateForce = 139,
    EM_DoSpark = 140,
    EM_FixAngle = 141,
}

impl ::protobuf::ProtobufEnum for EBaseEntityMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EBaseEntityMessages> {
        match value {
            136 => ::std::option::Option::Some(EBaseEntityMessages::EM_PlayJingle),
            137 => ::std::option::Option::Some(EBaseEntityMessages::EM_ScreenOverlay),
            138 => ::std::option::Option::Some(EBaseEntityMessages::EM_RemoveAllDecals),
            139 => ::std::option::Option::Some(EBaseEntityMessages::EM_PropagateForce),
            140 => ::std::option::Option::Some(EBaseEntityMessages::EM_DoSpark),
            141 => ::std::option::Option::Some(EBaseEntityMessages::EM_FixAngle),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EBaseEntityMessages] = &[
            EBaseEntityMessages::EM_PlayJingle,
            EBaseEntityMessages::EM_ScreenOverlay,
            EBaseEntityMessages::EM_RemoveAllDecals,
            EBaseEntityMessages::EM_PropagateForce,
            EBaseEntityMessages::EM_DoSpark,
            EBaseEntityMessages::EM_FixAngle,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EBaseEntityMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EBaseEntityMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EBaseEntityMessages {
}

impl ::protobuf::reflect::ProtobufValue for EBaseEntityMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum eRollType {
    ROLL_NONE = -1,
    ROLL_STATS = 0,
    ROLL_CREDITS = 1,
    ROLL_LATE_JOIN_LOGO = 2,
    ROLL_OUTTRO = 3,
}

impl ::protobuf::ProtobufEnum for eRollType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<eRollType> {
        match value {
            -1 => ::std::option::Option::Some(eRollType::ROLL_NONE),
            0 => ::std::option::Option::Some(eRollType::ROLL_STATS),
            1 => ::std::option::Option::Some(eRollType::ROLL_CREDITS),
            2 => ::std::option::Option::Some(eRollType::ROLL_LATE_JOIN_LOGO),
            3 => ::std::option::Option::Some(eRollType::ROLL_OUTTRO),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [eRollType] = &[
            eRollType::ROLL_NONE,
            eRollType::ROLL_STATS,
            eRollType::ROLL_CREDITS,
            eRollType::ROLL_LATE_JOIN_LOGO,
            eRollType::ROLL_OUTTRO,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<eRollType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("eRollType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for eRollType {
}

impl ::protobuf::reflect::ProtobufValue for eRollType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PARTICLE_MESSAGE {
    GAME_PARTICLE_MANAGER_EVENT_CREATE = 0,
    GAME_PARTICLE_MANAGER_EVENT_UPDATE = 1,
    GAME_PARTICLE_MANAGER_EVENT_UPDATE_FORWARD = 2,
    GAME_PARTICLE_MANAGER_EVENT_UPDATE_ORIENTATION = 3,
    GAME_PARTICLE_MANAGER_EVENT_UPDATE_FALLBACK = 4,
    GAME_PARTICLE_MANAGER_EVENT_UPDATE_ENT = 5,
    GAME_PARTICLE_MANAGER_EVENT_UPDATE_OFFSET = 6,
    GAME_PARTICLE_MANAGER_EVENT_DESTROY = 7,
    GAME_PARTICLE_MANAGER_EVENT_DESTROY_INVOLVING = 8,
    GAME_PARTICLE_MANAGER_EVENT_RELEASE = 9,
    GAME_PARTICLE_MANAGER_EVENT_LATENCY = 10,
    GAME_PARTICLE_MANAGER_EVENT_SHOULD_DRAW = 11,
    GAME_PARTICLE_MANAGER_EVENT_FROZEN = 12,
    GAME_PARTICLE_MANAGER_EVENT_CHANGE_CONTROL_POINT_ATTACHMENT = 13,
}

impl ::protobuf::ProtobufEnum for PARTICLE_MESSAGE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PARTICLE_MESSAGE> {
        match value {
            0 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_CREATE),
            1 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE),
            2 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_FORWARD),
            3 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_ORIENTATION),
            4 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_FALLBACK),
            5 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_ENT),
            6 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_OFFSET),
            7 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_DESTROY),
            8 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_DESTROY_INVOLVING),
            9 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_RELEASE),
            10 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_LATENCY),
            11 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_SHOULD_DRAW),
            12 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_FROZEN),
            13 => ::std::option::Option::Some(PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_CHANGE_CONTROL_POINT_ATTACHMENT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PARTICLE_MESSAGE] = &[
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_CREATE,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_FORWARD,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_ORIENTATION,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_FALLBACK,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_ENT,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_UPDATE_OFFSET,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_DESTROY,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_DESTROY_INVOLVING,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_RELEASE,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_LATENCY,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_SHOULD_DRAW,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_FROZEN,
            PARTICLE_MESSAGE::GAME_PARTICLE_MANAGER_EVENT_CHANGE_CONTROL_POINT_ATTACHMENT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<PARTICLE_MESSAGE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PARTICLE_MESSAGE", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PARTICLE_MESSAGE {
}

impl ::protobuf::reflect::ProtobufValue for PARTICLE_MESSAGE {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EHapticPulseType {
    VR_HAND_HAPTIC_PULSE_LIGHT = 0,
    VR_HAND_HAPTIC_PULSE_MEDIUM = 1,
    VR_HAND_HAPTIC_PULSE_STRONG = 2,
}

impl ::protobuf::ProtobufEnum for EHapticPulseType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EHapticPulseType> {
        match value {
            0 => ::std::option::Option::Some(EHapticPulseType::VR_HAND_HAPTIC_PULSE_LIGHT),
            1 => ::std::option::Option::Some(EHapticPulseType::VR_HAND_HAPTIC_PULSE_MEDIUM),
            2 => ::std::option::Option::Some(EHapticPulseType::VR_HAND_HAPTIC_PULSE_STRONG),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EHapticPulseType] = &[
            EHapticPulseType::VR_HAND_HAPTIC_PULSE_LIGHT,
            EHapticPulseType::VR_HAND_HAPTIC_PULSE_MEDIUM,
            EHapticPulseType::VR_HAND_HAPTIC_PULSE_STRONG,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EHapticPulseType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EHapticPulseType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EHapticPulseType {
}

impl ::protobuf::reflect::ProtobufValue for EHapticPulseType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x16, 0x6e, 0x65, 0x74, 0x77,
    0x6f, 0x72, 0x6b, 0x62, 0x61, 0x73, 0x65, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x40, 0x0a, 0x1c, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x41, 0x63, 0x68, 0x69, 0x65, 0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x12, 0x20, 0x0a, 0x0b, 0x61, 0x63, 0x68, 0x69, 0x65, 0x76, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x61, 0x63, 0x68, 0x69, 0x65, 0x76, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x22, 0x88, 0x01, 0x0a, 0x18, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x43, 0x61, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52,
    0x04, 0x68, 0x61, 0x73, 0x68, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x66, 0x72, 0x6f, 0x6d, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x12, 0x1b, 0x0a, 0x09, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x22,
    0x8e, 0x01, 0x0a, 0x1e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x43, 0x6c, 0x6f, 0x73, 0x65, 0x43, 0x61, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x69, 0x72, 0x65,
    0x63, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x66, 0x72, 0x6f, 0x6d, 0x50, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x12, 0x1b, 0x0a, 0x09, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x22, 0x97, 0x01, 0x0a, 0x23, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x43, 0x61, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x6c, 0x61,
    0x63, 0x65, 0x68, 0x6f, 0x6c, 0x64, 0x65, 0x72, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x12, 0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b,
    0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0a, 0x66, 0x72, 0x6f, 0x6d, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x12, 0x1b, 0x0a,
    0x09, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x08, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x22, 0x38, 0x0a, 0x1c, 0x43, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x07, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x74, 0x22, 0xb4, 0x01, 0x0a, 0x1c, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x54, 0x69, 0x6d, 0x65,
    0x73, 0x63, 0x61, 0x6c, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x64, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x07, 0x64, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x12,
    0x22, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x61, 0x63, 0x63, 0x65, 0x6c, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x22, 0x0a, 0x0c, 0x6d, 0x69, 0x6e, 0x62, 0x6c, 0x65, 0x6e, 0x64, 0x72,
    0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x6d, 0x69, 0x6e, 0x62, 0x6c,
    0x65, 0x6e, 0x64, 0x72, 0x61, 0x74, 0x65, 0x12, 0x32, 0x0a, 0x14, 0x62, 0x6c, 0x65, 0x6e, 0x64,
    0x64, 0x65, 0x6c, 0x74, 0x61, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x14, 0x62, 0x6c, 0x65, 0x6e, 0x64, 0x64, 0x65, 0x6c, 0x74,
    0x61, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x69, 0x65, 0x72, 0x22, 0x77, 0x0a, 0x10, 0x43,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x46, 0x61, 0x64, 0x65, 0x12,
    0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x68,
    0x6f, 0x6c, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08,
    0x68, 0x6f, 0x6c, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67,
    0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x14,
    0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52, 0x05, 0x63,
    0x6f, 0x6c, 0x6f, 0x72, 0x22, 0x85, 0x01, 0x0a, 0x11, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x12, 0x1c, 0x0a, 0x09, 0x61, 0x6d, 0x70, 0x6c, 0x69, 0x74, 0x75, 0x64,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x61, 0x6d, 0x70, 0x6c, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x66, 0x72, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x79,
    0x12, 0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x75, 0x0a, 0x14,
    0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x68, 0x61, 0x6b,
    0x65, 0x44, 0x69, 0x72, 0x12, 0x2d, 0x0a, 0x05, 0x73, 0x68, 0x61, 0x6b, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x52, 0x05, 0x73, 0x68,
    0x61, 0x6b, 0x65, 0x12, 0x2e, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x22, 0xaa, 0x01, 0x0a, 0x16, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x54, 0x69, 0x6c, 0x74, 0x12, 0x18,
    0x0a, 0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x12, 0x1e, 0x0a, 0x0b, 0x65, 0x61, 0x73, 0x65,
    0x5f, 0x69, 0x6e, 0x5f, 0x6f, 0x75, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x65,
    0x61, 0x73, 0x65, 0x49, 0x6e, 0x4f, 0x75, 0x74, 0x12, 0x26, 0x0a, 0x05, 0x61, 0x6e, 0x67, 0x6c,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x05, 0x61, 0x6e, 0x67, 0x6c, 0x65,
    0x12, 0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04,
    0x74, 0x69, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x04, 0x74, 0x69, 0x6d, 0x65,
    0x22, 0x5f, 0x0a, 0x13, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x53, 0x61, 0x79, 0x54, 0x65, 0x78, 0x74, 0x12, 0x20, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78,
    0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x12, 0x12, 0x0a,
    0x04, 0x63, 0x68, 0x61, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x63, 0x68, 0x61,
    0x74, 0x22, 0xce, 0x01, 0x0a, 0x14, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x53, 0x61, 0x79, 0x54, 0x65, 0x78, 0x74, 0x32, 0x12, 0x20, 0x0a, 0x0b, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x12, 0x0a, 0x04,
    0x63, 0x68, 0x61, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x63, 0x68, 0x61, 0x74,
    0x12, 0x20, 0x0a, 0x0b, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x31, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x31, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x61,
    0x72, 0x61, 0x6d, 0x32, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x70, 0x61, 0x72, 0x61,
    0x6d, 0x32, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x33, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x33, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x61,
    0x72, 0x61, 0x6d, 0x34, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x70, 0x61, 0x72, 0x61,
    0x6d, 0x34, 0x22, 0xa8, 0x02, 0x0a, 0x12, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x48, 0x75, 0x64, 0x4d, 0x73, 0x67, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x12, 0x0c, 0x0a, 0x01, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01,
    0x78, 0x12, 0x0c, 0x0a, 0x01, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01, 0x79, 0x12,
    0x16, 0x0a, 0x06, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x31, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52,
    0x06, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x31, 0x12, 0x16, 0x0a, 0x06, 0x63, 0x6f, 0x6c, 0x6f, 0x72,
    0x32, 0x18, 0x05, 0x20, 0x01, 0x28, 0x07, 0x52, 0x06, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x32, 0x12,
    0x16, 0x0a, 0x06, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x06, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x12, 0x20, 0x0a, 0x0c, 0x66, 0x61, 0x64, 0x65, 0x5f,
    0x69, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x66,
    0x61, 0x64, 0x65, 0x49, 0x6e, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x22, 0x0a, 0x0d, 0x66, 0x61, 0x64,
    0x65, 0x5f, 0x6f, 0x75, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x0b, 0x66, 0x61, 0x64, 0x65, 0x4f, 0x75, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x1b, 0x0a,
    0x09, 0x68, 0x6f, 0x6c, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x08, 0x68, 0x6f, 0x6c, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x78,
    0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x02, 0x52, 0x06, 0x66, 0x78, 0x54,
    0x69, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x0b,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x2f, 0x0a,
    0x13, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x48, 0x75, 0x64,
    0x54, 0x65, 0x78, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x3f,
    0x0a, 0x13, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x65,
    0x78, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x65, 0x73, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x04, 0x64, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x72,
    0x61, 0x6d, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x05, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x22,
    0x17, 0x0a, 0x15, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x47,
    0x61, 0x6d, 0x65, 0x54, 0x69, 0x74, 0x6c, 0x65, 0x22, 0x16, 0x0a, 0x14, 0x43, 0x55, 0x73, 0x65,
    0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x65, 0x74, 0x48, 0x55, 0x44,
    0x22, 0x49, 0x0a, 0x15, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x53, 0x65, 0x6e, 0x64, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x6f, 0x75,
    0x6e, 0x64, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x6f,
    0x75, 0x6e, 0x64, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x74, 0x6f, 0x70, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x73, 0x74, 0x6f, 0x70, 0x22, 0x9c, 0x01, 0x0a, 0x1a,
    0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x41, 0x75, 0x64, 0x69,
    0x6f, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x0e, 0x70, 0x61,
    0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0d, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x24, 0x0a, 0x0e, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x5f, 0x63,
    0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x6e, 0x61, 0x6d, 0x65, 0x48,
    0x61, 0x73, 0x68, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x1b, 0x0a,
    0x09, 0x69, 0x6e, 0x74, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x08, 0x69, 0x6e, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x7c, 0x0a, 0x15, 0x43, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x4d,
    0x61, 0x73, 0x6b, 0x12, 0x27, 0x0a, 0x0f, 0x67, 0x61, 0x6d, 0x65, 0x72, 0x75, 0x6c, 0x65, 0x73,
    0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0e, 0x67, 0x61,
    0x6d, 0x65, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x4d, 0x61, 0x73, 0x6b, 0x73, 0x12, 0x1b, 0x0a, 0x09,
    0x62, 0x61, 0x6e, 0x5f, 0x6d, 0x61, 0x73, 0x6b, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0d, 0x52,
    0x08, 0x62, 0x61, 0x6e, 0x4d, 0x61, 0x73, 0x6b, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x6f, 0x64,
    0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x6d,
    0x6f, 0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x22, 0x1a, 0x0a, 0x18, 0x43, 0x55, 0x73, 0x65,
    0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x22, 0x30, 0x0a, 0x14, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x48, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x78, 0x74, 0x12, 0x18, 0x0a, 0x07,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x35, 0x0a, 0x17, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4b, 0x65, 0x79, 0x48, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x78,
    0x74, 0x12, 0x1a, 0x0a, 0x08, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x09, 0x52, 0x08, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x22, 0x5b, 0x0a,
    0x19, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x56, 0x6f, 0x69,
    0x63, 0x65, 0x53, 0x75, 0x62, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x65, 0x6e, 0x75, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x04, 0x6d, 0x65, 0x6e, 0x75, 0x12, 0x12, 0x0a, 0x04, 0x69, 0x74, 0x65, 0x6d, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x69, 0x74, 0x65, 0x6d, 0x22, 0xa5, 0x01, 0x0a, 0x14, 0x43,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x56, 0x47, 0x55, 0x49, 0x4d,
    0x65, 0x6e, 0x75, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x68, 0x6f, 0x77, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x73, 0x68, 0x6f, 0x77, 0x12, 0x33, 0x0a, 0x04, 0x6b,
    0x65, 0x79, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x56, 0x47, 0x55,
    0x49, 0x4d, 0x65, 0x6e, 0x75, 0x2e, 0x4b, 0x65, 0x79, 0x73, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73,
    0x1a, 0x30, 0x0a, 0x04, 0x4b, 0x65, 0x79, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x22, 0x54, 0x0a, 0x12, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x52, 0x75, 0x6d, 0x62, 0x6c, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x12,
    0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x64, 0x61,
    0x74, 0x61, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x22, 0x2f, 0x0a, 0x11, 0x43, 0x55, 0x73, 0x65,
    0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x12, 0x1a, 0x0a,
    0x08, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x08, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x62, 0x0a, 0x1a, 0x43, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x61, 0x79, 0x54, 0x65, 0x78, 0x74,
    0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x12,
    0x18, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78,
    0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x22, 0xd2, 0x01,
    0x0a, 0x17, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x6f,
    0x6c, 0x6f, 0x72, 0x65, 0x64, 0x54, 0x65, 0x78, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x6c,
    0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x12,
    0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74,
    0x65, 0x78, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x72, 0x65, 0x73, 0x65, 0x74, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x05, 0x72, 0x65, 0x73, 0x65, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x78, 0x74, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x50, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x78, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x78, 0x74, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x54, 0x65, 0x61, 0x6d,
    0x49, 0x64, 0x22, 0x34, 0x0a, 0x16, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x69, 0x63, 0x6b, 0x75, 0x70, 0x12, 0x1a, 0x0a, 0x08,
    0x69, 0x74, 0x65, 0x6d, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08,
    0x69, 0x74, 0x65, 0x6d, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x31, 0x0a, 0x16, 0x43, 0x55, 0x73, 0x65,
    0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x41, 0x6d, 0x6d, 0x6f, 0x44, 0x65, 0x6e, 0x69,
    0x65, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x61, 0x6d, 0x6d, 0x6f, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x06, 0x61, 0x6d, 0x6d, 0x6f, 0x49, 0x64, 0x22, 0x52, 0x0a, 0x1a, 0x43,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x72, 0x6f, 0x73, 0x73,
    0x68, 0x61, 0x69, 0x72, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x12, 0x34, 0x0a, 0x0c, 0x61, 0x6e, 0x67,
    0x63, 0x72, 0x6f, 0x73, 0x73, 0x68, 0x61, 0x69, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c,
    0x65, 0x52, 0x0c, 0x61, 0x6e, 0x67, 0x63, 0x72, 0x6f, 0x73, 0x73, 0x68, 0x61, 0x69, 0x72, 0x22,
    0x94, 0x01, 0x0a, 0x14, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x53, 0x68, 0x6f, 0x77, 0x4d, 0x65, 0x6e, 0x75, 0x12, 0x1e, 0x0a, 0x0a, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x73, 0x6c, 0x6f, 0x74, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x73, 0x6c, 0x6f, 0x74, 0x73, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x69, 0x73, 0x70,
    0x6c, 0x61, 0x79, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x64,
    0x69, 0x73, 0x70, 0x6c, 0x61, 0x79, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x6e, 0x65,
    0x65, 0x64, 0x6d, 0x6f, 0x72, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x6e, 0x65,
    0x65, 0x64, 0x6d, 0x6f, 0x72, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x6d, 0x65, 0x6e, 0x75, 0x73, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x6d, 0x65, 0x6e, 0x75,
    0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x22, 0x71, 0x0a, 0x16, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x72, 0x65, 0x64, 0x69, 0x74, 0x73, 0x4d, 0x73, 0x67,
    0x12, 0x36, 0x0a, 0x08, 0x72, 0x6f, 0x6c, 0x6c, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x65, 0x52, 0x6f, 0x6c, 0x6c, 0x54,
    0x79, 0x70, 0x65, 0x3a, 0x09, 0x52, 0x4f, 0x4c, 0x4c, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x52, 0x08,
    0x72, 0x6f, 0x6c, 0x6c, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x6c, 0x6f, 0x67, 0x6f,
    0x5f, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x6c,
    0x6f, 0x67, 0x6f, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x22, 0x1a, 0x0a, 0x18, 0x43, 0x45, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x50, 0x6c, 0x61, 0x79, 0x4a,
    0x69, 0x6e, 0x67, 0x6c, 0x65, 0x22, 0x40, 0x0a, 0x1b, 0x43, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x4f, 0x76, 0x65,
    0x72, 0x6c, 0x61, 0x79, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x65, 0x66,
    0x66, 0x65, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x22, 0x44, 0x0a, 0x1d, 0x43, 0x45, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x41,
    0x6c, 0x6c, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x72, 0x65, 0x6d, 0x6f,
    0x76, 0x65, 0x5f, 0x64, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x0c, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x22, 0x4a, 0x0a,
    0x1c, 0x43, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x50,
    0x72, 0x6f, 0x70, 0x61, 0x67, 0x61, 0x74, 0x65, 0x46, 0x6f, 0x72, 0x63, 0x65, 0x12, 0x2a, 0x0a,
    0x07, 0x69, 0x6d, 0x70, 0x75, 0x6c, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x52, 0x07, 0x69, 0x6d, 0x70, 0x75, 0x6c, 0x73, 0x65, 0x22, 0xd9, 0x01, 0x0a, 0x15, 0x43, 0x45,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x44, 0x6f, 0x53, 0x70,
    0x61, 0x72, 0x6b, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x20, 0x0a,
    0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0b, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12,
    0x16, 0x0a, 0x06, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x06, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x14, 0x0a,
    0x05, 0x62, 0x65, 0x61, 0x6d, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x62, 0x65,
    0x61, 0x6d, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x68, 0x69, 0x63, 0x6b, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x05, 0x74, 0x68, 0x69, 0x63, 0x6b, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x64, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x5c, 0x0a, 0x16, 0x43, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x46, 0x69, 0x78, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x12,
    0x1a, 0x0a, 0x08, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x08, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x12, 0x26, 0x0a, 0x05, 0x61,
    0x6e, 0x67, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x05, 0x61, 0x6e,
    0x67, 0x6c, 0x65, 0x22, 0xa2, 0x02, 0x0a, 0x1c, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x63, 0x61, 0x6d, 0x65, 0x72,
    0x61, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x66, 0x0a, 0x12, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5f, 0x64, 0x61, 0x74, 0x61,
    0x5f, 0x64, 0x72, 0x69, 0x76, 0x65, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x38, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x43, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x44, 0x61, 0x74,
    0x61, 0x44, 0x72, 0x69, 0x76, 0x65, 0x6e, 0x52, 0x10, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x44,
    0x61, 0x74, 0x61, 0x44, 0x72, 0x69, 0x76, 0x65, 0x6e, 0x1a, 0x5d, 0x0a, 0x15, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x44, 0x61, 0x74, 0x61, 0x44, 0x72, 0x69, 0x76,
    0x65, 0x6e, 0x12, 0x1a, 0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x28,
    0x0a, 0x10, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x5f, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68,
    0x45, 0x6e, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x22, 0xb9, 0x17, 0x0a, 0x18, 0x43, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61,
    0x6e, 0x61, 0x67, 0x65, 0x72, 0x12, 0x4e, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x50, 0x41, 0x52, 0x54, 0x49,
    0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x3a, 0x22, 0x47, 0x41, 0x4d,
    0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47,
    0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x52,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0d, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x69, 0x0a, 0x16, 0x72,
    0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74,
    0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x52, 0x65, 0x6c, 0x65,
    0x61, 0x73, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x52, 0x14, 0x72, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x56, 0x0a, 0x0f, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x2d, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e,
    0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x52, 0x0e,
    0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x59,
    0x0a, 0x10, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63,
    0x6c, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x52, 0x0f, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f,
    0x79, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x75, 0x0a, 0x1a, 0x64, 0x65, 0x73,
    0x74, 0x72, 0x6f, 0x79, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x69, 0x6e,
    0x76, 0x6f, 0x6c, 0x76, 0x69, 0x6e, 0x67, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x44, 0x65,
    0x73, 0x74, 0x72, 0x6f, 0x79, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x76,
    0x6f, 0x6c, 0x76, 0x69, 0x6e, 0x67, 0x52, 0x18, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x50,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x76, 0x6f, 0x6c, 0x76, 0x69, 0x6e, 0x67,
    0x12, 0x56, 0x0a, 0x0f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69,
    0x63, 0x6c, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63,
    0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x52, 0x0e, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x60, 0x0a, 0x13, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x66, 0x77, 0x64, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x55, 0x73,
    0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61,
    0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74,
    0x69, 0x63, 0x6c, 0x65, 0x46, 0x77, 0x64, 0x52, 0x11, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x46, 0x77, 0x64, 0x12, 0x69, 0x0a, 0x16, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x6f, 0x72,
    0x69, 0x65, 0x6e, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69,
    0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x52,
    0x14, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f,
    0x72, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x6f, 0x0a, 0x18, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f,
    0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x66, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63,
    0x6b, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x35, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65,
    0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x46, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x16,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x46, 0x61,
    0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x12, 0x69, 0x0a, 0x16, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x55,
    0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d,
    0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72,
    0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x52, 0x14, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x66, 0x66, 0x73, 0x65,
    0x74, 0x12, 0x60, 0x0a, 0x13, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x63, 0x6c, 0x65, 0x5f, 0x65, 0x6e, 0x74, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x30,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x45, 0x6e, 0x74,
    0x52, 0x11, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65,
    0x45, 0x6e, 0x74, 0x12, 0x76, 0x0a, 0x1b, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x5f, 0x64, 0x72,
    0x61, 0x77, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x53, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x44, 0x72, 0x61,
    0x77, 0x52, 0x18, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c,
    0x65, 0x53, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x44, 0x72, 0x61, 0x77, 0x12, 0x73, 0x0a, 0x1a, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x73,
    0x65, 0x74, 0x5f, 0x66, 0x72, 0x6f, 0x7a, 0x65, 0x6e, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x36, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x53, 0x65,
    0x74, 0x46, 0x72, 0x6f, 0x7a, 0x65, 0x6e, 0x52, 0x17, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x46, 0x72, 0x6f, 0x7a, 0x65, 0x6e,
    0x12, 0x82, 0x01, 0x0a, 0x1f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68,
    0x6d, 0x65, 0x6e, 0x74, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x3b, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67, 0x5f, 0x50, 0x61, 0x72, 0x74, 0x69,
    0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x41, 0x74, 0x74,
    0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x52, 0x1c, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x41, 0x74, 0x74, 0x61, 0x63,
    0x68, 0x6d, 0x65, 0x6e, 0x74, 0x1a, 0x16, 0x0a, 0x14, 0x52, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65,
    0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x1a, 0x86, 0x01,
    0x0a, 0x0e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65,
    0x12, 0x2e, 0x0a, 0x13, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x11, 0x70,
    0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x12, 0x1f, 0x0a, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0a, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64,
    0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x1a, 0x42, 0x0a, 0x0f, 0x44, 0x65, 0x73, 0x74, 0x72, 0x6f,
    0x79, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x2f, 0x0a, 0x13, 0x64, 0x65, 0x73,
    0x74, 0x72, 0x6f, 0x79, 0x5f, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x49,
    0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x1a, 0x70, 0x0a, 0x18, 0x44, 0x65,
    0x73, 0x74, 0x72, 0x6f, 0x79, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x49, 0x6e, 0x76,
    0x6f, 0x6c, 0x76, 0x69, 0x6e, 0x67, 0x12, 0x2f, 0x0a, 0x13, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f,
    0x79, 0x5f, 0x69, 0x6d, 0x6d, 0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x12, 0x64, 0x65, 0x73, 0x74, 0x72, 0x6f, 0x79, 0x49, 0x6d, 0x6d, 0x65,
    0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x1a, 0x63, 0x0a, 0x0e,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x12, 0x23,
    0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x50, 0x6f,
    0x69, 0x6e, 0x74, 0x12, 0x2c, 0x0a, 0x08, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x08, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x1a, 0x64, 0x0a, 0x11, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69,
    0x63, 0x6c, 0x65, 0x46, 0x77, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x2a, 0x0a, 0x07, 0x66,
    0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x07,
    0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x1a, 0xb1, 0x01, 0x0a, 0x14, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x72, 0x69, 0x65, 0x6e, 0x74,
    0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f, 0x70, 0x6f, 0x69, 0x6e,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x50, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x2a, 0x0a, 0x07, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x07, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72,
    0x64, 0x12, 0x26, 0x0a, 0x05, 0x72, 0x69, 0x67, 0x68, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x52, 0x05, 0x72, 0x69, 0x67, 0x68, 0x74, 0x12, 0x20, 0x0a, 0x02, 0x75, 0x70, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x02, 0x75, 0x70, 0x1a, 0x6b, 0x0a, 0x16, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x46, 0x61, 0x6c,
    0x6c, 0x62, 0x61, 0x63, 0x6b, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x2c, 0x0a, 0x08, 0x70, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x08,
    0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x1a, 0x72, 0x0a, 0x14, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74,
    0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f, 0x70, 0x6f, 0x69, 0x6e,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x50, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x35, 0x0a, 0x0d, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x5f,
    0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x0c,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x1a, 0x8a, 0x02, 0x0a,
    0x11, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x45,
    0x6e, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x5f, 0x70, 0x6f,
    0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x12, 0x1f, 0x0a, 0x0b,
    0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0a, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1e, 0x0a,
    0x0a, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0a, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x3d, 0x0a,
    0x11, 0x66, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x10, 0x66, 0x61, 0x6c, 0x6c,
    0x62, 0x61, 0x63, 0x6b, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x0a, 0x11,
    0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x5f, 0x77, 0x65, 0x61, 0x72, 0x61, 0x62, 0x6c, 0x65,
    0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x10, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65,
    0x57, 0x65, 0x61, 0x72, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x1a, 0x38, 0x0a, 0x17, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x46, 0x72,
    0x6f, 0x7a, 0x65, 0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x74, 0x5f, 0x66, 0x72, 0x6f, 0x7a,
    0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x73, 0x65, 0x74, 0x46, 0x72, 0x6f,
    0x7a, 0x65, 0x6e, 0x1a, 0x3b, 0x0a, 0x18, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72,
    0x74, 0x69, 0x63, 0x6c, 0x65, 0x53, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x44, 0x72, 0x61, 0x77, 0x12,
    0x1f, 0x0a, 0x0b, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x5f, 0x64, 0x72, 0x61, 0x77, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x44, 0x72, 0x61, 0x77,
    0x1a, 0x91, 0x01, 0x0a, 0x1c, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e,
    0x74, 0x12, 0x25, 0x0a, 0x0e, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x5f,
    0x6f, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x61, 0x74, 0x74, 0x61, 0x63,
    0x68, 0x6d, 0x65, 0x6e, 0x74, 0x4f, 0x6c, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x61, 0x74, 0x74, 0x61,
    0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x65, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x0d, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x4e, 0x65, 0x77, 0x12,
    0x23, 0x0a, 0x0d, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x48, 0x61,
    0x6e, 0x64, 0x6c, 0x65, 0x22, 0x2e, 0x0a, 0x11, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67,
    0x5f, 0x48, 0x75, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x19, 0x0a, 0x08, 0x6f, 0x72, 0x64,
    0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x6f, 0x72, 0x64,
    0x65, 0x72, 0x49, 0x64, 0x22, 0x4d, 0x0a, 0x18, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x73, 0x67,
    0x5f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x12, 0x1d, 0x0a, 0x0a, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12,
    0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x22, 0x85, 0x01, 0x0a, 0x17, 0x43, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x48, 0x61, 0x70, 0x74, 0x69, 0x63, 0x50, 0x75, 0x6c, 0x73, 0x65, 0x12,
    0x17, 0x0a, 0x07, 0x68, 0x61, 0x6e, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x06, 0x68, 0x61, 0x6e, 0x64, 0x49, 0x64, 0x12, 0x51, 0x0a, 0x0a, 0x70, 0x75, 0x6c, 0x73,
    0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x45, 0x48, 0x61, 0x70, 0x74, 0x69, 0x63, 0x50, 0x75, 0x6c, 0x73, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x3a, 0x1a, 0x56, 0x52, 0x5f, 0x48, 0x41, 0x4e, 0x44, 0x5f, 0x48, 0x41,
    0x50, 0x54, 0x49, 0x43, 0x5f, 0x50, 0x55, 0x4c, 0x53, 0x45, 0x5f, 0x4c, 0x49, 0x47, 0x48, 0x54,
    0x52, 0x09, 0x70, 0x75, 0x6c, 0x73, 0x65, 0x54, 0x79, 0x70, 0x65, 0x2a, 0xb8, 0x06, 0x0a, 0x11,
    0x45, 0x42, 0x61, 0x73, 0x65, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x12, 0x17, 0x0a, 0x13, 0x55, 0x4d, 0x5f, 0x41, 0x63, 0x68, 0x69, 0x65, 0x76, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x65, 0x12, 0x13, 0x0a, 0x0f, 0x55, 0x4d,
    0x5f, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x43, 0x61, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x66, 0x12,
    0x19, 0x0a, 0x15, 0x55, 0x4d, 0x5f, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x43, 0x61, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x10, 0x67, 0x12, 0x17, 0x0a, 0x13, 0x55, 0x4d,
    0x5f, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x63, 0x61, 0x6c,
    0x65, 0x10, 0x68, 0x12, 0x17, 0x0a, 0x13, 0x55, 0x4d, 0x5f, 0x44, 0x65, 0x73, 0x69, 0x72, 0x65,
    0x64, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x10, 0x69, 0x12, 0x0b, 0x0a, 0x07,
    0x55, 0x4d, 0x5f, 0x46, 0x61, 0x64, 0x65, 0x10, 0x6a, 0x12, 0x10, 0x0a, 0x0c, 0x55, 0x4d, 0x5f,
    0x47, 0x61, 0x6d, 0x65, 0x54, 0x69, 0x74, 0x6c, 0x65, 0x10, 0x6b, 0x12, 0x0f, 0x0a, 0x0b, 0x55,
    0x4d, 0x5f, 0x48, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x78, 0x74, 0x10, 0x6d, 0x12, 0x0d, 0x0a, 0x09,
    0x55, 0x4d, 0x5f, 0x48, 0x75, 0x64, 0x4d, 0x73, 0x67, 0x10, 0x6e, 0x12, 0x0e, 0x0a, 0x0a, 0x55,
    0x4d, 0x5f, 0x48, 0x75, 0x64, 0x54, 0x65, 0x78, 0x74, 0x10, 0x6f, 0x12, 0x12, 0x0a, 0x0e, 0x55,
    0x4d, 0x5f, 0x4b, 0x65, 0x79, 0x48, 0x69, 0x6e, 0x74, 0x54, 0x65, 0x78, 0x74, 0x10, 0x70, 0x12,
    0x12, 0x0a, 0x0e, 0x55, 0x4d, 0x5f, 0x43, 0x6f, 0x6c, 0x6f, 0x72, 0x65, 0x64, 0x54, 0x65, 0x78,
    0x74, 0x10, 0x71, 0x12, 0x13, 0x0a, 0x0f, 0x55, 0x4d, 0x5f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x53, 0x74, 0x61, 0x74, 0x65, 0x10, 0x72, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x52,
    0x65, 0x73, 0x65, 0x74, 0x48, 0x55, 0x44, 0x10, 0x73, 0x12, 0x0d, 0x0a, 0x09, 0x55, 0x4d, 0x5f,
    0x52, 0x75, 0x6d, 0x62, 0x6c, 0x65, 0x10, 0x74, 0x12, 0x0e, 0x0a, 0x0a, 0x55, 0x4d, 0x5f, 0x53,
    0x61, 0x79, 0x54, 0x65, 0x78, 0x74, 0x10, 0x75, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x53,
    0x61, 0x79, 0x54, 0x65, 0x78, 0x74, 0x32, 0x10, 0x76, 0x12, 0x15, 0x0a, 0x11, 0x55, 0x4d, 0x5f,
    0x53, 0x61, 0x79, 0x54, 0x65, 0x78, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x10, 0x77,
    0x12, 0x0c, 0x0a, 0x08, 0x55, 0x4d, 0x5f, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x10, 0x78, 0x12, 0x0f,
    0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x53, 0x68, 0x61, 0x6b, 0x65, 0x44, 0x69, 0x72, 0x10, 0x79, 0x12,
    0x0e, 0x0a, 0x0a, 0x55, 0x4d, 0x5f, 0x54, 0x65, 0x78, 0x74, 0x4d, 0x73, 0x67, 0x10, 0x7c, 0x12,
    0x11, 0x0a, 0x0d, 0x55, 0x4d, 0x5f, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x54, 0x69, 0x6c, 0x74,
    0x10, 0x7d, 0x12, 0x0c, 0x0a, 0x08, 0x55, 0x4d, 0x5f, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x10, 0x7e,
    0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x56, 0x47, 0x55, 0x49, 0x4d, 0x65, 0x6e, 0x75, 0x10,
    0x7f, 0x12, 0x11, 0x0a, 0x0c, 0x55, 0x4d, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65, 0x4d, 0x61, 0x73,
    0x6b, 0x10, 0x80, 0x01, 0x12, 0x15, 0x0a, 0x10, 0x55, 0x4d, 0x5f, 0x56, 0x6f, 0x69, 0x63, 0x65,
    0x53, 0x75, 0x62, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x10, 0x81, 0x01, 0x12, 0x11, 0x0a, 0x0c, 0x55,
    0x4d, 0x5f, 0x53, 0x65, 0x6e, 0x64, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x10, 0x82, 0x01, 0x12, 0x12,
    0x0a, 0x0d, 0x55, 0x4d, 0x5f, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x69, 0x63, 0x6b, 0x75, 0x70, 0x10,
    0x83, 0x01, 0x12, 0x12, 0x0a, 0x0d, 0x55, 0x4d, 0x5f, 0x41, 0x6d, 0x6d, 0x6f, 0x44, 0x65, 0x6e,
    0x69, 0x65, 0x64, 0x10, 0x84, 0x01, 0x12, 0x16, 0x0a, 0x11, 0x55, 0x4d, 0x5f, 0x43, 0x72, 0x6f,
    0x73, 0x73, 0x68, 0x61, 0x69, 0x72, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x10, 0x85, 0x01, 0x12, 0x10,
    0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x53, 0x68, 0x6f, 0x77, 0x4d, 0x65, 0x6e, 0x75, 0x10, 0x86, 0x01,
    0x12, 0x12, 0x0a, 0x0d, 0x55, 0x4d, 0x5f, 0x43, 0x72, 0x65, 0x64, 0x69, 0x74, 0x73, 0x4d, 0x73,
    0x67, 0x10, 0x87, 0x01, 0x12, 0x1f, 0x0a, 0x1a, 0x55, 0x4d, 0x5f, 0x43, 0x6c, 0x6f, 0x73, 0x65,
    0x43, 0x61, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x6c, 0x61, 0x63, 0x65, 0x68, 0x6f, 0x6c, 0x64,
    0x65, 0x72, 0x10, 0x8e, 0x01, 0x12, 0x18, 0x0a, 0x13, 0x55, 0x4d, 0x5f, 0x43, 0x61, 0x6d, 0x65,
    0x72, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x10, 0x8f, 0x01, 0x12,
    0x16, 0x0a, 0x11, 0x55, 0x4d, 0x5f, 0x41, 0x75, 0x64, 0x69, 0x6f, 0x50, 0x61, 0x72, 0x61, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x10, 0x90, 0x01, 0x12, 0x17, 0x0a, 0x12, 0x55, 0x4d, 0x5f, 0x50, 0x61,
    0x72, 0x74, 0x69, 0x63, 0x6c, 0x65, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x10, 0x91, 0x01,
    0x12, 0x10, 0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x48, 0x75, 0x64, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x10,
    0x92, 0x01, 0x12, 0x17, 0x0a, 0x12, 0x55, 0x4d, 0x5f, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47,
    0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x10, 0x94, 0x01, 0x12, 0x17, 0x0a, 0x12, 0x55,
    0x4d, 0x5f, 0x48, 0x61, 0x6e, 0x64, 0x48, 0x61, 0x70, 0x74, 0x69, 0x63, 0x50, 0x75, 0x6c, 0x73,
    0x65, 0x10, 0x95, 0x01, 0x12, 0x10, 0x0a, 0x0b, 0x55, 0x4d, 0x5f, 0x4d, 0x41, 0x58, 0x5f, 0x42,
    0x41, 0x53, 0x45, 0x10, 0xc8, 0x01, 0x2a, 0x94, 0x01, 0x0a, 0x13, 0x45, 0x42, 0x61, 0x73, 0x65,
    0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x12,
    0x0a, 0x0d, 0x45, 0x4d, 0x5f, 0x50, 0x6c, 0x61, 0x79, 0x4a, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x10,
    0x88, 0x01, 0x12, 0x15, 0x0a, 0x10, 0x45, 0x4d, 0x5f, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x4f,
    0x76, 0x65, 0x72, 0x6c, 0x61, 0x79, 0x10, 0x89, 0x01, 0x12, 0x17, 0x0a, 0x12, 0x45, 0x4d, 0x5f,
    0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x41, 0x6c, 0x6c, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x73, 0x10,
    0x8a, 0x01, 0x12, 0x16, 0x0a, 0x11, 0x45, 0x4d, 0x5f, 0x50, 0x72, 0x6f, 0x70, 0x61, 0x67, 0x61,
    0x74, 0x65, 0x46, 0x6f, 0x72, 0x63, 0x65, 0x10, 0x8b, 0x01, 0x12, 0x0f, 0x0a, 0x0a, 0x45, 0x4d,
    0x5f, 0x44, 0x6f, 0x53, 0x70, 0x61, 0x72, 0x6b, 0x10, 0x8c, 0x01, 0x12, 0x10, 0x0a, 0x0b, 0x45,
    0x4d, 0x5f, 0x46, 0x69, 0x78, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x10, 0x8d, 0x01, 0x2a, 0x6f, 0x0a,
    0x09, 0x65, 0x52, 0x6f, 0x6c, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x09, 0x52, 0x4f,
    0x4c, 0x4c, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x01, 0x12, 0x0e, 0x0a, 0x0a, 0x52, 0x4f, 0x4c, 0x4c, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x53,
    0x10, 0x00, 0x12, 0x10, 0x0a, 0x0c, 0x52, 0x4f, 0x4c, 0x4c, 0x5f, 0x43, 0x52, 0x45, 0x44, 0x49,
    0x54, 0x53, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x52, 0x4f, 0x4c, 0x4c, 0x5f, 0x4c, 0x41, 0x54,
    0x45, 0x5f, 0x4a, 0x4f, 0x49, 0x4e, 0x5f, 0x4c, 0x4f, 0x47, 0x4f, 0x10, 0x02, 0x12, 0x0f, 0x0a,
    0x0b, 0x52, 0x4f, 0x4c, 0x4c, 0x5f, 0x4f, 0x55, 0x54, 0x54, 0x52, 0x4f, 0x10, 0x03, 0x2a, 0x96,
    0x05, 0x0a, 0x10, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x45, 0x53, 0x53,
    0x41, 0x47, 0x45, 0x12, 0x26, 0x0a, 0x22, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54,
    0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45,
    0x4e, 0x54, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x10, 0x00, 0x12, 0x26, 0x0a, 0x22, 0x47,
    0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e,
    0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54,
    0x45, 0x10, 0x01, 0x12, 0x2e, 0x0a, 0x2a, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54,
    0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45,
    0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x46, 0x4f, 0x52, 0x57, 0x41, 0x52,
    0x44, 0x10, 0x02, 0x12, 0x32, 0x0a, 0x2e, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54,
    0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45,
    0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x4f, 0x52, 0x49, 0x45, 0x4e, 0x54,
    0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x03, 0x12, 0x2f, 0x0a, 0x2b, 0x47, 0x41, 0x4d, 0x45, 0x5f,
    0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52,
    0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x46, 0x41,
    0x4c, 0x4c, 0x42, 0x41, 0x43, 0x4b, 0x10, 0x04, 0x12, 0x2a, 0x0a, 0x26, 0x47, 0x41, 0x4d, 0x45,
    0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45,
    0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x45,
    0x4e, 0x54, 0x10, 0x05, 0x12, 0x2d, 0x0a, 0x29, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52,
    0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56,
    0x45, 0x4e, 0x54, 0x5f, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x4f, 0x46, 0x46, 0x53, 0x45,
    0x54, 0x10, 0x06, 0x12, 0x27, 0x0a, 0x23, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54,
    0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45,
    0x4e, 0x54, 0x5f, 0x44, 0x45, 0x53, 0x54, 0x52, 0x4f, 0x59, 0x10, 0x07, 0x12, 0x31, 0x0a, 0x2d,
    0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41,
    0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x44, 0x45, 0x53, 0x54,
    0x52, 0x4f, 0x59, 0x5f, 0x49, 0x4e, 0x56, 0x4f, 0x4c, 0x56, 0x49, 0x4e, 0x47, 0x10, 0x08, 0x12,
    0x27, 0x0a, 0x23, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45,
    0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x52,
    0x45, 0x4c, 0x45, 0x41, 0x53, 0x45, 0x10, 0x09, 0x12, 0x27, 0x0a, 0x23, 0x47, 0x41, 0x4d, 0x45,
    0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45,
    0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x4c, 0x41, 0x54, 0x45, 0x4e, 0x43, 0x59, 0x10,
    0x0a, 0x12, 0x2b, 0x0a, 0x27, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43,
    0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54,
    0x5f, 0x53, 0x48, 0x4f, 0x55, 0x4c, 0x44, 0x5f, 0x44, 0x52, 0x41, 0x57, 0x10, 0x0b, 0x12, 0x26,
    0x0a, 0x22, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f,
    0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x46, 0x52,
    0x4f, 0x5a, 0x45, 0x4e, 0x10, 0x0c, 0x12, 0x3f, 0x0a, 0x3b, 0x47, 0x41, 0x4d, 0x45, 0x5f, 0x50,
    0x41, 0x52, 0x54, 0x49, 0x43, 0x4c, 0x45, 0x5f, 0x4d, 0x41, 0x4e, 0x41, 0x47, 0x45, 0x52, 0x5f,
    0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x43, 0x48, 0x41, 0x4e, 0x47, 0x45, 0x5f, 0x43, 0x4f, 0x4e,
    0x54, 0x52, 0x4f, 0x4c, 0x5f, 0x50, 0x4f, 0x49, 0x4e, 0x54, 0x5f, 0x41, 0x54, 0x54, 0x41, 0x43,
    0x48, 0x4d, 0x45, 0x4e, 0x54, 0x10, 0x0d, 0x2a, 0x74, 0x0a, 0x10, 0x45, 0x48, 0x61, 0x70, 0x74,
    0x69, 0x63, 0x50, 0x75, 0x6c, 0x73, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1e, 0x0a, 0x1a, 0x56,
    0x52, 0x5f, 0x48, 0x41, 0x4e, 0x44, 0x5f, 0x48, 0x41, 0x50, 0x54, 0x49, 0x43, 0x5f, 0x50, 0x55,
    0x4c, 0x53, 0x45, 0x5f, 0x4c, 0x49, 0x47, 0x48, 0x54, 0x10, 0x00, 0x12, 0x1f, 0x0a, 0x1b, 0x56,
    0x52, 0x5f, 0x48, 0x41, 0x4e, 0x44, 0x5f, 0x48, 0x41, 0x50, 0x54, 0x49, 0x43, 0x5f, 0x50, 0x55,
    0x4c, 0x53, 0x45, 0x5f, 0x4d, 0x45, 0x44, 0x49, 0x55, 0x4d, 0x10, 0x01, 0x12, 0x1f, 0x0a, 0x1b,
    0x56, 0x52, 0x5f, 0x48, 0x41, 0x4e, 0x44, 0x5f, 0x48, 0x41, 0x50, 0x54, 0x49, 0x43, 0x5f, 0x50,
    0x55, 0x4c, 0x53, 0x45, 0x5f, 0x53, 0x54, 0x52, 0x4f, 0x4e, 0x47, 0x10, 0x02, 0x42, 0x05, 0x48,
    0x01, 0x80, 0x01, 0x00, 0x4a, 0xd8, 0x86, 0x01, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xb2, 0x03,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1f,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x06, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x06, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x06,
    0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12,
    0x03, 0x07, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x32, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x09, 0x05, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x0a, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b,
    0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x1a, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x0c, 0x20, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x0d, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d,
    0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0d, 0x1e, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x0e, 0x1e, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x0f, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x0f, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0f,
    0x12, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x08, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x08, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x10, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x11, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x11, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12,
    0x03, 0x11, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x12, 0x08,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x08, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x12, 0x14, 0x17, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x13, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x13, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09,
    0x02, 0x12, 0x03, 0x13, 0x15, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03,
    0x14, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x14, 0x08,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x14, 0x19, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x15, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x15, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0b, 0x02, 0x12, 0x03, 0x15, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c,
    0x12, 0x03, 0x16, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x16, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x16, 0x1a,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x17, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x17, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x17, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x0e, 0x12, 0x03, 0x18, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x01,
    0x12, 0x03, 0x18, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03,
    0x18, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x19, 0x08, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x19, 0x08, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x19, 0x15, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x10, 0x12, 0x03, 0x1a, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x10, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x02,
    0x12, 0x03, 0x1a, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x11, 0x12, 0x03, 0x1b,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x02, 0x12, 0x03, 0x1b, 0x1c, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x12, 0x12, 0x03, 0x1c, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x12, 0x02, 0x12, 0x03, 0x1c, 0x13, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x13, 0x12,
    0x03, 0x1d, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03, 0x1d,
    0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x02, 0x12, 0x03, 0x1d, 0x16, 0x19,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x14, 0x12, 0x03, 0x1e, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x14, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x14, 0x02, 0x12, 0x03, 0x1e, 0x15, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x15, 0x12, 0x03, 0x1f, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x15, 0x01, 0x12,
    0x03, 0x1f, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x15, 0x02, 0x12, 0x03, 0x1f,
    0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x16, 0x12, 0x03, 0x20, 0x08, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x16, 0x01, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x16, 0x02, 0x12, 0x03, 0x20, 0x13, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x17, 0x12, 0x03, 0x21, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x17,
    0x01, 0x12, 0x03, 0x21, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x17, 0x02, 0x12,
    0x03, 0x21, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x18, 0x12, 0x03, 0x22, 0x08,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x18, 0x01, 0x12, 0x03, 0x22, 0x08, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x18, 0x02, 0x12, 0x03, 0x22, 0x17, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x19, 0x12, 0x03, 0x23, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x19, 0x01, 0x12, 0x03, 0x23, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x19,
    0x02, 0x12, 0x03, 0x23, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1a, 0x12, 0x03,
    0x24, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x24, 0x08,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1a, 0x02, 0x12, 0x03, 0x24, 0x17, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1b, 0x12, 0x03, 0x25, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x1b, 0x01, 0x12, 0x03, 0x25, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x1b, 0x02, 0x12, 0x03, 0x25, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1c,
    0x12, 0x03, 0x26, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1c, 0x01, 0x12, 0x03,
    0x26, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1c, 0x02, 0x12, 0x03, 0x26, 0x18,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1d, 0x12, 0x03, 0x27, 0x08, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x1d, 0x01, 0x12, 0x03, 0x27, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x1d, 0x02, 0x12, 0x03, 0x27, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x1e, 0x12, 0x03, 0x28, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1e, 0x01,
    0x12, 0x03, 0x28, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1e, 0x02, 0x12, 0x03,
    0x28, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1f, 0x12, 0x03, 0x29, 0x08, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x1f, 0x01, 0x12, 0x03, 0x29, 0x08, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x1f, 0x02, 0x12, 0x03, 0x29, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x20, 0x12, 0x03, 0x2a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x20, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x20, 0x02,
    0x12, 0x03, 0x2a, 0x25, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x21, 0x12, 0x03, 0x2b,
    0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x21, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x21, 0x02, 0x12, 0x03, 0x2b, 0x1e, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x22, 0x12, 0x03, 0x2c, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x22, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x22, 0x02, 0x12, 0x03, 0x2c, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x23, 0x12,
    0x03, 0x2d, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x23, 0x01, 0x12, 0x03, 0x2d,
    0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x23, 0x02, 0x12, 0x03, 0x2d, 0x1d, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x24, 0x12, 0x03, 0x2e, 0x08, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x24, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x24, 0x02, 0x12, 0x03, 0x2e, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x25, 0x12, 0x03, 0x2f, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x25, 0x01, 0x12,
    0x03, 0x2f, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x25, 0x02, 0x12, 0x03, 0x2f,
    0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x26, 0x12, 0x03, 0x30, 0x08, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x26, 0x01, 0x12, 0x03, 0x30, 0x08, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x26, 0x02, 0x12, 0x03, 0x30, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x27, 0x12, 0x03, 0x31, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x27,
    0x01, 0x12, 0x03, 0x31, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x27, 0x02, 0x12,
    0x03, 0x31, 0x16, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x34, 0x00, 0x3b, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x34, 0x05, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x35, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x35, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x35, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x36,
    0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x08, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x36, 0x1b, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x37, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x37, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12,
    0x03, 0x38, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x38,
    0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03, 0x38, 0x1c, 0x1f,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12, 0x03, 0x39, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x39, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x04, 0x02, 0x12, 0x03, 0x39, 0x15, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x05, 0x12, 0x03, 0x3a, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x3a, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x02, 0x12, 0x03, 0x3a,
    0x16, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x3d, 0x00, 0x43, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x05, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x3e, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x3e, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x3e, 0x14, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x3f, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x02, 0x02, 0x02, 0x12, 0x03, 0x40, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x40, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x40, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x41,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x41, 0x08, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x41, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x42, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x42, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x04, 0x02, 0x12, 0x03, 0x42, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x03, 0x12, 0x04, 0x45,
    0x00, 0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x03, 0x01, 0x12, 0x03, 0x45, 0x05, 0x15, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x00, 0x12, 0x03, 0x46, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x46, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x47, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x47, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x01, 0x02, 0x12, 0x03, 0x47, 0x2d,
    0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x02, 0x12, 0x03, 0x48, 0x08, 0x37, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x48, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x48, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03,
    0x02, 0x03, 0x12, 0x03, 0x49, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x49, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x49, 0x39, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x04, 0x12, 0x03, 0x4a, 0x08, 0x38,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x33, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x03, 0x02, 0x04, 0x02, 0x12, 0x03, 0x4a, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x03, 0x02, 0x05, 0x12, 0x03, 0x4b, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x05, 0x02,
    0x12, 0x03, 0x4b, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x06, 0x12, 0x03, 0x4c,
    0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x06, 0x02, 0x12, 0x03, 0x4c, 0x34, 0x35, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x03, 0x02, 0x07, 0x12, 0x03, 0x4d, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02,
    0x07, 0x02, 0x12, 0x03, 0x4d, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x08, 0x12,
    0x03, 0x4e, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x08, 0x01, 0x12, 0x03, 0x4e,
    0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x08, 0x02, 0x12, 0x03, 0x4e, 0x38, 0x39,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x09, 0x12, 0x03, 0x4f, 0x08, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x03, 0x02, 0x09, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x03, 0x02, 0x09, 0x02, 0x12, 0x03, 0x4f, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02,
    0x0a, 0x12, 0x03, 0x50, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x0a, 0x01, 0x12,
    0x03, 0x50, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x50,
    0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x0b, 0x12, 0x03, 0x51, 0x08, 0x35, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x51, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x03, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x51, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x03, 0x02, 0x0c, 0x12, 0x03, 0x52, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x52, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x0c, 0x02, 0x12,
    0x03, 0x52, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x0d, 0x12, 0x03, 0x53, 0x08,
    0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x53, 0x08, 0x43, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x53, 0x46, 0x48, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x04, 0x12, 0x04, 0x56, 0x00, 0x5a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x04, 0x01,
    0x12, 0x03, 0x56, 0x05, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x00, 0x12, 0x03, 0x57,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x08, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x02, 0x12, 0x03, 0x57, 0x25, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x04, 0x02, 0x01, 0x12, 0x03, 0x58, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x58, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x58, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x02, 0x12,
    0x03, 0x59, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x59,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x59, 0x26, 0x27,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x5c, 0x00, 0x5e, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x5d, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x5d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5d, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x18, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x26, 0x27, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x60, 0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x60, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x61,
    0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x61, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x61, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x61, 0x19, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x61, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x62, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x62, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x62, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x17,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x62, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x63, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x63, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x63, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x63, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x63, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x64, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x64, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x64, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x64, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x64, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x67, 0x00, 0x6c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x67, 0x08,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x68, 0x08, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x68, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x68, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x68, 0x19, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x68, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x69, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x69, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x69, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x69, 0x17, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x69, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x6a, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x6a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x6a, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6a,
    0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6a, 0x24, 0x25,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6b, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x6b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x6b, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x6b, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x6e, 0x00, 0x73,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x6f, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x6f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x6f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x6f, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6f,
    0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x70, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x70, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x70, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x70, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x70, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12,
    0x03, 0x71, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x71,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x71, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x71, 0x16, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x71, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x72, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x72, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x72, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x72, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x72, 0x23,
    0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x75, 0x00, 0x77, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x75, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x76, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x76, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x76,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x76, 0x17, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x76, 0x21, 0x22, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x79, 0x00, 0x7e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x79, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03,
    0x7a, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7a, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7a, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7a, 0x17, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x7b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x7b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7b,
    0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7b, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x7c, 0x08, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7c, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x7c, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x7c, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x7d,
    0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7d, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7d, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7d, 0x17, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7d, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x06,
    0x12, 0x06, 0x80, 0x01, 0x00, 0x85, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x04, 0x80, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x04, 0x81,
    0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x04, 0x81, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x18, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x01, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x82, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x04, 0x82, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x82, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x02, 0x12, 0x04, 0x83, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x83, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12,
    0x04, 0x83, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x83, 0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x04, 0x83,
    0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x04, 0x84, 0x01, 0x08,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0x84, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x04, 0x84, 0x01, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x04, 0x84, 0x01, 0x19, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x04, 0x84, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x07, 0x12, 0x06, 0x87, 0x01, 0x00, 0x8c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x07, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00,
    0x12, 0x04, 0x88, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x88, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x04,
    0x88, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88,
    0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x01,
    0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0x89, 0x01, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x89, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x04, 0x89, 0x01, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x04, 0x89, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x04, 0x89, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x02, 0x05, 0x12, 0x04, 0x8a, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x8a, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x8a, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x04,
    0x8b, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8b,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8b, 0x01,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x17,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x22, 0x23,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x8e, 0x01, 0x00, 0x91, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x8f, 0x01, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x8f, 0x01, 0x23, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x8f, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x04, 0x90,
    0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x90, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x04, 0x90, 0x01, 0x11,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0x90, 0x01, 0x1c, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0x90, 0x01, 0x28, 0x29, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x06, 0x93, 0x01, 0x00, 0x99, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x04, 0x93, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x00, 0x12, 0x04, 0x94, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x94, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x94, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x94, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x95, 0x01,
    0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x95, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x04, 0x95, 0x01, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0x95, 0x01, 0x16, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x95, 0x01, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x04, 0x96, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0x96, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x02, 0x06, 0x12, 0x04, 0x96, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x1c, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x96, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03,
    0x12, 0x04, 0x97, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x97, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x04,
    0x97, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x04, 0x97,
    0x01, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x04, 0x97, 0x01,
    0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04, 0x12, 0x04, 0x98, 0x01, 0x08, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x04, 0x98, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x05, 0x12, 0x04, 0x98, 0x01, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x04, 0x98, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x04, 0x98, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x06, 0x9b, 0x01, 0x00, 0x9f, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12,
    0x04, 0x9c, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x9c, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9c,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9c, 0x01,
    0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x26,
    0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x02, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02,
    0x05, 0x12, 0x04, 0x9e, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x9e, 0x01, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x9e, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xa1, 0x01, 0x00,
    0xa9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x1c,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xa3, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xa3, 0x01, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xa3, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0xa4, 0x01,
    0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x18, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x26, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04,
    0x12, 0x04, 0xa6, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12,
    0x04, 0xa6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x04,
    0xa6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa6,
    0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa6, 0x01,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x05, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x05, 0x12, 0x04, 0xa7, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x06, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x06, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x06, 0x05, 0x12, 0x04, 0xa8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06,
    0x01, 0x12, 0x04, 0xa8, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x03,
    0x12, 0x04, 0xa8, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0xab, 0x01,
    0x00, 0xb7, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0xab, 0x01, 0x08,
    0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xac, 0x01, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xac, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xac, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xac, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xac, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0c, 0x02, 0x01, 0x12, 0x04, 0xad, 0x01, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xad, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xad, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xad, 0x01, 0x17, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xad, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x04, 0xae,
    0x01, 0x08, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0xae, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x04, 0xae, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xae, 0x01, 0x17, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xae, 0x01, 0x1b, 0x1c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x03, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x03, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x19, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x04, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x04,
    0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x05, 0x12,
    0x04, 0xb0, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xb0, 0x01, 0x19, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x03, 0x12, 0x04, 0xb0,
    0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x08,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x18, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x06, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x06, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x06, 0x05, 0x12, 0x04, 0xb2, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x06, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06,
    0x03, 0x12, 0x04, 0xb2, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x07, 0x12,
    0x04, 0xb3, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x04, 0x12, 0x04,
    0xb3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x05, 0x12, 0x04, 0xb3,
    0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x01, 0x12, 0x04, 0xb3, 0x01,
    0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x27,
    0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x08, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x04, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x05, 0x12, 0x04, 0xb4, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x08, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x08, 0x03, 0x12, 0x04, 0xb4, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0c, 0x02, 0x09, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x09, 0x04, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09,
    0x05, 0x12, 0x04, 0xb5, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x01,
    0x12, 0x04, 0xb5, 0x01, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x09, 0x03, 0x12,
    0x04, 0xb5, 0x01, 0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0a, 0x12, 0x04, 0xb6,
    0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xb6, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x05, 0x12, 0x04, 0xb6, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x18, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x22, 0x24, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0xb9, 0x01, 0x00, 0xbb, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x00, 0x12, 0x04, 0xba, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xba, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xba, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xba, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xba, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xbd, 0x01, 0x00, 0xc0,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x1b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbe, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x01, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xbf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xbf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xbf, 0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbf,
    0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xc2, 0x01, 0x00, 0xc3, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x1d, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xc5, 0x01, 0x00, 0xc6, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x10, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12,
    0x06, 0xc8, 0x01, 0x00, 0xcb, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04,
    0xc8, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xc9, 0x01,
    0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x18, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xca, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xca, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0xca, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xca, 0x01, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xca, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06,
    0xcd, 0x01, 0x00, 0xd2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xcd,
    0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xce, 0x01, 0x08,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xce, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0xce, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xce, 0x01, 0x18, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xce, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xcf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xcf, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02, 0x12,
    0x04, 0xd0, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xd0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd0,
    0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd0, 0x01,
    0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x13, 0x12, 0x06, 0xd4, 0x01, 0x00, 0xd8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01,
    0x12, 0x04, 0xd4, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04,
    0xd5, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd5,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd5, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x18,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x2a, 0x2b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13,
    0x02, 0x02, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xd7, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xd7, 0x01, 0x16, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xd7, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xda, 0x01, 0x00, 0xdb,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xda, 0x01, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xdd, 0x01, 0x00, 0xdf, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x00, 0x12, 0x04, 0xde, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xde, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xde, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xde, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xde, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xe1, 0x01, 0x00, 0xe3,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x1f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12,
    0x06, 0xe5, 0x01, 0x00, 0xe9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04,
    0xe5, 0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xe6, 0x01,
    0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x17, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe7, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02,
    0x12, 0x04, 0xe8, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xe8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xe8, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe8,
    0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe8, 0x01,
    0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xeb, 0x01, 0x00, 0xf4, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x1c, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x18, 0x03, 0x00, 0x12, 0x06, 0xec, 0x01, 0x08, 0xef, 0x01, 0x09, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x03, 0x00, 0x01, 0x12, 0x04, 0xec, 0x01, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xed, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xed, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xed, 0x01, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xed, 0x01, 0x20, 0x24, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xed, 0x01, 0x27, 0x28,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xee, 0x01, 0x10, 0x2a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xee, 0x01,
    0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee,
    0x01, 0x20, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xee, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xf1, 0x01,
    0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x18, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf2, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xf2, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x02,
    0x12, 0x04, 0xf3, 0x01, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xf3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x06, 0x12, 0x04,
    0xf3, 0x01, 0x11, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf3,
    0x01, 0x2b, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf3, 0x01,
    0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xf6, 0x01, 0x00, 0xfa, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xf7, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xf7, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xf7, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12,
    0x04, 0xf8, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xf8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf8,
    0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf8, 0x01,
    0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x1e,
    0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf9, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x1a, 0x12, 0x06, 0xfc, 0x01, 0x00, 0xfe, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01,
    0x12, 0x04, 0xfc, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04,
    0xfd, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xfd, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x18,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0x80, 0x02, 0x00, 0x84, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0x80, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1b, 0x02, 0x00, 0x12, 0x04, 0x81, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x81, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x81, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x81, 0x02, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x81, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0x82,
    0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x82, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x05, 0x12, 0x04, 0x82, 0x02, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x02, 0x17, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0x82, 0x02, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0x83, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0x83, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x02, 0x05, 0x12, 0x04, 0x83, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x02, 0x01, 0x12, 0x04, 0x83, 0x02, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x83, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12,
    0x06, 0x86, 0x02, 0x00, 0x8d, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04,
    0x86, 0x02, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0x87, 0x02,
    0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x05, 0x12, 0x04, 0x87, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x02, 0x18, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x02, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01, 0x12, 0x04, 0x88, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x01, 0x05, 0x12, 0x04, 0x88, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x88, 0x02, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x88, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02,
    0x12, 0x04, 0x89, 0x02, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x89, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x05, 0x12, 0x04,
    0x89, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x89,
    0x02, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x03, 0x12, 0x04, 0x89, 0x02,
    0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x2d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8a, 0x02, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x17, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8a, 0x02, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1c, 0x02, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x8b, 0x02, 0x17, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x8b, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x05, 0x12, 0x04,
    0x8c, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x05, 0x04, 0x12, 0x04, 0x8c,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x05, 0x05, 0x12, 0x04, 0x8c, 0x02,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x05, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x17,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x05, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x29, 0x2a,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0x8f, 0x02, 0x00, 0x91, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0x8f, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1d, 0x02, 0x00, 0x12, 0x04, 0x90, 0x02, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x90, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x90, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x90, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x90, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0x93, 0x02, 0x00,
    0x95, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0x93, 0x02, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0x94, 0x02, 0x08, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0x94, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f,
    0x12, 0x06, 0x97, 0x02, 0x00, 0x99, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12,
    0x04, 0x97, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0x98,
    0x02, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0x98, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x06, 0x12, 0x04, 0x98, 0x02, 0x11,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x98, 0x02, 0x1c, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x98, 0x02, 0x2b, 0x2c, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0x9b, 0x02, 0x00, 0xa0, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20,
    0x02, 0x00, 0x12, 0x04, 0x9c, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x9c, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x9c, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9c, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9c, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x02,
    0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9d, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x18, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x26, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x20, 0x02, 0x02, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9e, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x02, 0x16, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x9e, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x03,
    0x12, 0x04, 0x9f, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x9f, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x05, 0x12, 0x04,
    0x9f, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9f,
    0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9f, 0x02,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xa2, 0x02, 0x00, 0xa5, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xa2, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0xa3, 0x02, 0x08, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x21, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa3, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xa3, 0x02, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x1b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xa3, 0x02, 0x26, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x08,
    0x12, 0x04, 0xa3, 0x02, 0x28, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x07, 0x12,
    0x04, 0xa3, 0x02, 0x33, 0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04, 0xa4,
    0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa4, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa4, 0x02, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x17, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x25, 0x26, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0xa7, 0x02, 0x00, 0xa8, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x23,
    0x12, 0x06, 0xaa, 0x02, 0x00, 0xac, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x23, 0x01, 0x12,
    0x04, 0xaa, 0x02, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x00, 0x12, 0x04, 0xab,
    0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x04, 0x12, 0x04, 0xab, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x05, 0x12, 0x04, 0xab, 0x02, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01, 0x12, 0x04, 0xab, 0x02, 0x16, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12, 0x04, 0xab, 0x02, 0x25, 0x26, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xae, 0x02, 0x00, 0xb0, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xae, 0x02, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24,
    0x02, 0x00, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xaf, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xaf, 0x02, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xaf, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0xb2, 0x02, 0x00, 0xb4,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x25, 0x01, 0x12, 0x04, 0xb2, 0x02, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x00, 0x12, 0x04, 0xb3, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb3, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb3, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb3, 0x02, 0x1c, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xb3, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12,
    0x06, 0xb6, 0x02, 0x00, 0xbe, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04,
    0xb6, 0x02, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x02,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb7, 0x02, 0x11, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x1c, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x26, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb8, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb8, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x02,
    0x12, 0x04, 0xb9, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xb9, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xb9, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb9,
    0x02, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x02,
    0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x03, 0x12, 0x04, 0xba, 0x02, 0x08, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x04, 0x12, 0x04, 0xba, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x05, 0x12, 0x04, 0xba, 0x02, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x01, 0x12, 0x04, 0xba, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x03, 0x03, 0x12, 0x04, 0xba, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x26, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x02, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xbb, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xbb, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xbb, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xbb, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x05, 0x12, 0x04,
    0xbc, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x05, 0x04, 0x12, 0x04, 0xbc,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x05, 0x05, 0x12, 0x04, 0xbc, 0x02,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x05, 0x01, 0x12, 0x04, 0xbc, 0x02, 0x17,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x05, 0x03, 0x12, 0x04, 0xbc, 0x02, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x06, 0x12, 0x04, 0xbd, 0x02, 0x08, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x06, 0x04, 0x12, 0x04, 0xbd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x06, 0x05, 0x12, 0x04, 0xbd, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x06, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x06, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27,
    0x12, 0x06, 0xc0, 0x02, 0x00, 0xc3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12,
    0x04, 0xc0, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0xc1,
    0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc1, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc1, 0x02, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc1, 0x02, 0x16, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc1, 0x02, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc2, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x1c, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xc2, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12,
    0x06, 0xc5, 0x02, 0x00, 0xce, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04,
    0xc5, 0x02, 0x08, 0x24, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x28, 0x03, 0x00, 0x12, 0x06, 0xc6, 0x02,
    0x08, 0xc9, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x03, 0x00, 0x01, 0x12, 0x04, 0xc6,
    0x02, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x28, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc7,
    0x02, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xc7, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xc7, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xc7, 0x02, 0x20, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xc7, 0x02, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x28, 0x03, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xc8, 0x02, 0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xc8, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xc8, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc8, 0x02, 0x1f, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x28,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc8, 0x02, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x28, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xcb, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xcb, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xcb, 0x02, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xcb, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x01, 0x12, 0x04, 0xcc,
    0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcc, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcc, 0x02, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc, 0x02, 0x17, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcc, 0x02, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x5b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x02, 0x04, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x02, 0x06, 0x12, 0x04, 0xcd, 0x02, 0x11, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcd, 0x02, 0x44, 0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xcd, 0x02, 0x59, 0x5a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x29, 0x12,
    0x06, 0xd0, 0x02, 0x00, 0xa4, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29, 0x01, 0x12, 0x04,
    0xd0, 0x02, 0x08, 0x20, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x00, 0x12, 0x06, 0xd1, 0x02,
    0x08, 0xd2, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x00, 0x01, 0x12, 0x04, 0xd1,
    0x02, 0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x01, 0x12, 0x06, 0xd4, 0x02, 0x08,
    0xd8, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x01, 0x01, 0x12, 0x04, 0xd4, 0x02,
    0x10, 0x1e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0xd5, 0x02,
    0x10, 0x39, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd5,
    0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xd5, 0x02, 0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd5, 0x02, 0x21, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xd5, 0x02, 0x37, 0x38, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x01, 0x02, 0x01,
    0x12, 0x04, 0xd6, 0x02, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xd6, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x01, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xd6, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xd6, 0x02, 0x1f, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd6, 0x02, 0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29,
    0x03, 0x01, 0x02, 0x02, 0x12, 0x04, 0xd7, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29,
    0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd7, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x29, 0x03, 0x01, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd7, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x29, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x29, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd7, 0x02, 0x2f, 0x30, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x29, 0x03, 0x02, 0x12, 0x06, 0xda, 0x02, 0x08, 0xdc, 0x02, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x03, 0x02, 0x01, 0x12, 0x04, 0xda, 0x02, 0x10, 0x1f, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x29, 0x03, 0x02, 0x02, 0x00, 0x12, 0x04, 0xdb, 0x02, 0x10, 0x36, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x29, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdb, 0x02, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x29, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdb, 0x02, 0x19, 0x1d, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdb, 0x02, 0x1e, 0x31,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdb, 0x02, 0x34,
    0x35, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x03, 0x12, 0x06, 0xde, 0x02, 0x08, 0xe1, 0x02,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x03, 0x01, 0x12, 0x04, 0xde, 0x02, 0x10, 0x28,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x03, 0x02, 0x00, 0x12, 0x04, 0xdf, 0x02, 0x10, 0x36,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdf, 0x02, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x03, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdf, 0x02,
    0x19, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x03, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdf,
    0x02, 0x1e, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x03, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xdf, 0x02, 0x34, 0x35, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x03, 0x02, 0x01, 0x12, 0x04,
    0xe0, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x03, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xe0, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x03, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xe0, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x03, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xe0, 0x02, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x03, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xe0, 0x02, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x04,
    0x12, 0x06, 0xe3, 0x02, 0x08, 0xe6, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x04,
    0x01, 0x12, 0x04, 0xe3, 0x02, 0x10, 0x1e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x04, 0x02,
    0x00, 0x12, 0x04, 0xe4, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xe4, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x04,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xe4, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe4, 0x02, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29,
    0x03, 0x04, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe4, 0x02, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x29, 0x03, 0x04, 0x02, 0x01, 0x12, 0x04, 0xe5, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x29, 0x03, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe5, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x29, 0x03, 0x04, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe5, 0x02, 0x19, 0x23, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x29, 0x03, 0x04, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe5, 0x02, 0x24, 0x2c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x29, 0x03, 0x04, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe5, 0x02, 0x2f, 0x30, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x05, 0x12, 0x06, 0xe8, 0x02, 0x08, 0xeb, 0x02, 0x09, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x05, 0x01, 0x12, 0x04, 0xe8, 0x02, 0x10, 0x21, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x29, 0x03, 0x05, 0x02, 0x00, 0x12, 0x04, 0xe9, 0x02, 0x10, 0x31, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x29, 0x03, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe9, 0x02, 0x10, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x05, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe9, 0x02, 0x19, 0x1e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x05, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe9, 0x02, 0x1f,
    0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x05, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe9, 0x02,
    0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x05, 0x02, 0x01, 0x12, 0x04, 0xea, 0x02,
    0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x05, 0x02, 0x01, 0x04, 0x12, 0x04, 0xea,
    0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x05, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xea, 0x02, 0x19, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x05, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xea, 0x02, 0x24, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xea, 0x02, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x06, 0x12, 0x06,
    0xed, 0x02, 0x08, 0xf2, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x06, 0x01, 0x12,
    0x04, 0xed, 0x02, 0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x06, 0x02, 0x00, 0x12,
    0x04, 0xee, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xee, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xee, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xee, 0x02, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xee, 0x02, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03,
    0x06, 0x02, 0x01, 0x12, 0x04, 0xef, 0x02, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03,
    0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0xef, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29,
    0x03, 0x06, 0x02, 0x01, 0x06, 0x12, 0x04, 0xef, 0x02, 0x19, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x29, 0x03, 0x06, 0x02, 0x01, 0x01, 0x12, 0x04, 0xef, 0x02, 0x24, 0x2b, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x29, 0x03, 0x06, 0x02, 0x01, 0x03, 0x12, 0x04, 0xef, 0x02, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x29, 0x03, 0x06, 0x02, 0x02, 0x12, 0x04, 0xf0, 0x02, 0x10, 0x2e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf0, 0x02, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x02, 0x06, 0x12, 0x04, 0xf0, 0x02, 0x19, 0x23, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf0, 0x02, 0x24, 0x29,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf0, 0x02, 0x2c,
    0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x06, 0x02, 0x03, 0x12, 0x04, 0xf1, 0x02, 0x10,
    0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf1, 0x02,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x03, 0x06, 0x12, 0x04, 0xf1,
    0x02, 0x19, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xf1, 0x02, 0x24, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x06, 0x02, 0x03, 0x03, 0x12,
    0x04, 0xf1, 0x02, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x07, 0x12, 0x06, 0xf4,
    0x02, 0x08, 0xf7, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x07, 0x01, 0x12, 0x04,
    0xf4, 0x02, 0x10, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x07, 0x02, 0x00, 0x12, 0x04,
    0xf5, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xf5, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x07, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xf5, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x07, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xf5, 0x02, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xf5, 0x02, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x07,
    0x02, 0x01, 0x12, 0x04, 0xf6, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x07,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xf6, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03,
    0x07, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf6, 0x02, 0x19, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29,
    0x03, 0x07, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf6, 0x02, 0x24, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x29, 0x03, 0x07, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf6, 0x02, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x29, 0x03, 0x08, 0x12, 0x06, 0xf9, 0x02, 0x08, 0xfc, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x03, 0x08, 0x01, 0x12, 0x04, 0xf9, 0x02, 0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x29, 0x03, 0x08, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x29, 0x03, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfa, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x29, 0x03, 0x08, 0x02, 0x00, 0x05, 0x12, 0x04, 0xfa, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x29, 0x03, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfa, 0x02, 0x1f, 0x2c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x29, 0x03, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfa, 0x02, 0x2f, 0x30, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x08, 0x02, 0x01, 0x12, 0x04, 0xfb, 0x02, 0x10, 0x36, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfb, 0x02, 0x10, 0x18,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x08, 0x02, 0x01, 0x06, 0x12, 0x04, 0xfb, 0x02, 0x19,
    0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfb, 0x02,
    0x24, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfb,
    0x02, 0x34, 0x35, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x09, 0x12, 0x06, 0xfe, 0x02, 0x08,
    0x85, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x09, 0x01, 0x12, 0x04, 0xfe, 0x02,
    0x10, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x09, 0x02, 0x00, 0x12, 0x04, 0xff, 0x02,
    0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0xff,
    0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xff, 0x02, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xff, 0x02, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xff, 0x02, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x09, 0x02, 0x01,
    0x12, 0x04, 0x80, 0x03, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x80, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x80, 0x03, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x03, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03,
    0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x03, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29,
    0x03, 0x09, 0x02, 0x02, 0x12, 0x04, 0x81, 0x03, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29,
    0x03, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0x81, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x29, 0x03, 0x09, 0x02, 0x02, 0x05, 0x12, 0x04, 0x81, 0x03, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x29, 0x03, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0x81, 0x03, 0x1f, 0x2a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0x81, 0x03, 0x2d, 0x2e, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x29, 0x03, 0x09, 0x02, 0x03, 0x12, 0x04, 0x82, 0x03, 0x10, 0x2e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x03, 0x04, 0x12, 0x04, 0x82, 0x03, 0x10, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x03, 0x05, 0x12, 0x04, 0x82, 0x03, 0x19, 0x1e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x03, 0x01, 0x12, 0x04, 0x82, 0x03, 0x1f,
    0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x03, 0x03, 0x12, 0x04, 0x82, 0x03,
    0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x09, 0x02, 0x04, 0x12, 0x04, 0x83, 0x03,
    0x10, 0x3a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x04, 0x04, 0x12, 0x04, 0x83,
    0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x83, 0x03, 0x19, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x83, 0x03, 0x24, 0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x83, 0x03, 0x38, 0x39, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x09, 0x02, 0x05,
    0x12, 0x04, 0x84, 0x03, 0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x84, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09, 0x02,
    0x05, 0x05, 0x12, 0x04, 0x84, 0x03, 0x19, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x09,
    0x02, 0x05, 0x01, 0x12, 0x04, 0x84, 0x03, 0x1e, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03,
    0x09, 0x02, 0x05, 0x03, 0x12, 0x04, 0x84, 0x03, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29,
    0x03, 0x0a, 0x12, 0x06, 0x87, 0x03, 0x08, 0x89, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x03, 0x0a, 0x01, 0x12, 0x04, 0x87, 0x03, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03,
    0x0a, 0x02, 0x00, 0x12, 0x04, 0x88, 0x03, 0x10, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03,
    0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x88, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29,
    0x03, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0x88, 0x03, 0x19, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x29, 0x03, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x88, 0x03, 0x1e, 0x28, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x29, 0x03, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x88, 0x03, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x29, 0x03, 0x0b, 0x12, 0x06, 0x8b, 0x03, 0x08, 0x8d, 0x03, 0x09, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x03, 0x0b, 0x01, 0x12, 0x04, 0x8b, 0x03, 0x10, 0x28, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x29, 0x03, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x03, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x29, 0x03, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x29, 0x03, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8c, 0x03, 0x19, 0x1d, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x29, 0x03, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x03, 0x1e, 0x29, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c, 0x03, 0x2c, 0x2d,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x29, 0x03, 0x0c, 0x12, 0x06, 0x8f, 0x03, 0x08, 0x93, 0x03, 0x09,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x03, 0x0c, 0x01, 0x12, 0x04, 0x8f, 0x03, 0x10, 0x2c, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x90, 0x03, 0x10, 0x32, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x90, 0x03, 0x10, 0x18,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x04, 0x90, 0x03, 0x19,
    0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x90, 0x03,
    0x1f, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x90,
    0x03, 0x30, 0x31, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x01, 0x12, 0x04, 0x91,
    0x03, 0x10, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x91, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x91, 0x03, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x91, 0x03, 0x1f, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x91, 0x03, 0x30, 0x31, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x29, 0x03, 0x0c, 0x02,
    0x02, 0x12, 0x04, 0x92, 0x03, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x92, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03, 0x0c,
    0x02, 0x02, 0x05, 0x12, 0x04, 0x92, 0x03, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29, 0x03,
    0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x92, 0x03, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x29,
    0x03, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0x92, 0x03, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x29, 0x02, 0x00, 0x12, 0x04, 0x95, 0x03, 0x08, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x95, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x95, 0x03, 0x11, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x95, 0x03, 0x22, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x95, 0x03, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x08, 0x12, 0x04,
    0x95, 0x03, 0x2b, 0x59, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x07, 0x12, 0x04, 0x95,
    0x03, 0x36, 0x58, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x01, 0x12, 0x04, 0x96, 0x03, 0x08,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x04, 0x12, 0x04, 0x96, 0x03, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x05, 0x12, 0x04, 0x96, 0x03, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x01, 0x12, 0x04, 0x96, 0x03, 0x18, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x03, 0x12, 0x04, 0x96, 0x03, 0x20, 0x21, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x29, 0x02, 0x02, 0x12, 0x04, 0x97, 0x03, 0x08, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x02, 0x04, 0x12, 0x04, 0x97, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x02, 0x06, 0x12, 0x04, 0x97, 0x03, 0x11, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x02, 0x01, 0x12, 0x04, 0x97, 0x03, 0x3f, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02,
    0x03, 0x12, 0x04, 0x97, 0x03, 0x58, 0x59, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x03, 0x12,
    0x04, 0x98, 0x03, 0x08, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x98, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x06, 0x12, 0x04, 0x98,
    0x03, 0x11, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x01, 0x12, 0x04, 0x98, 0x03,
    0x39, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x03, 0x12, 0x04, 0x98, 0x03, 0x4b,
    0x4c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x04, 0x12, 0x04, 0x99, 0x03, 0x08, 0x4f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x04, 0x12, 0x04, 0x99, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x06, 0x12, 0x04, 0x99, 0x03, 0x11, 0x39, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x04, 0x01, 0x12, 0x04, 0x99, 0x03, 0x3a, 0x4a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x04, 0x03, 0x12, 0x04, 0x99, 0x03, 0x4d, 0x4e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x29, 0x02, 0x05, 0x12, 0x04, 0x9a, 0x03, 0x08, 0x62, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x9a, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x05,
    0x06, 0x12, 0x04, 0x9a, 0x03, 0x11, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x9a, 0x03, 0x43, 0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x05, 0x03, 0x12,
    0x04, 0x9a, 0x03, 0x60, 0x61, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x06, 0x12, 0x04, 0x9b,
    0x03, 0x08, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x06, 0x04, 0x12, 0x04, 0x9b, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x06, 0x06, 0x12, 0x04, 0x9b, 0x03, 0x11,
    0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x06, 0x01, 0x12, 0x04, 0x9b, 0x03, 0x39, 0x48,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x06, 0x03, 0x12, 0x04, 0x9b, 0x03, 0x4b, 0x4c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x07, 0x12, 0x04, 0x9c, 0x03, 0x08, 0x54, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x07, 0x04, 0x12, 0x04, 0x9c, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x07, 0x06, 0x12, 0x04, 0x9c, 0x03, 0x11, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x07, 0x01, 0x12, 0x04, 0x9c, 0x03, 0x3c, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x07, 0x03, 0x12, 0x04, 0x9c, 0x03, 0x52, 0x53, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02,
    0x08, 0x12, 0x04, 0x9d, 0x03, 0x08, 0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x08, 0x04,
    0x12, 0x04, 0x9d, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x08, 0x06, 0x12,
    0x04, 0x9d, 0x03, 0x11, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x08, 0x01, 0x12, 0x04,
    0x9d, 0x03, 0x3f, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x08, 0x03, 0x12, 0x04, 0x9d,
    0x03, 0x58, 0x59, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x09, 0x12, 0x04, 0x9e, 0x03, 0x08,
    0x5f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x09, 0x04, 0x12, 0x04, 0x9e, 0x03, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x09, 0x06, 0x12, 0x04, 0x9e, 0x03, 0x11, 0x40, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x09, 0x01, 0x12, 0x04, 0x9e, 0x03, 0x41, 0x59, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x09, 0x03, 0x12, 0x04, 0x9e, 0x03, 0x5c, 0x5e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x29, 0x02, 0x0a, 0x12, 0x04, 0x9f, 0x03, 0x08, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x9f, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x0a, 0x06, 0x12, 0x04, 0x9f, 0x03, 0x11, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x0a, 0x01, 0x12, 0x04, 0x9f, 0x03, 0x3f, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0a,
    0x03, 0x12, 0x04, 0x9f, 0x03, 0x58, 0x5a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x0b, 0x12,
    0x04, 0xa0, 0x03, 0x08, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0b, 0x04, 0x12, 0x04,
    0xa0, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0b, 0x06, 0x12, 0x04, 0xa0,
    0x03, 0x11, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xa0, 0x03,
    0x3c, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xa0, 0x03, 0x52,
    0x54, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x0c, 0x12, 0x04, 0xa1, 0x03, 0x08, 0x64, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xa1, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x0c, 0x06, 0x12, 0x04, 0xa1, 0x03, 0x11, 0x42, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xa1, 0x03, 0x43, 0x5e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xa1, 0x03, 0x61, 0x63, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x29, 0x02, 0x0d, 0x12, 0x04, 0xa2, 0x03, 0x08, 0x62, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x0d, 0x04, 0x12, 0x04, 0xa2, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0d,
    0x06, 0x12, 0x04, 0xa2, 0x03, 0x11, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0d, 0x01,
    0x12, 0x04, 0xa2, 0x03, 0x42, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0d, 0x03, 0x12,
    0x04, 0xa2, 0x03, 0x5f, 0x61, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x0e, 0x12, 0x04, 0xa3,
    0x03, 0x08, 0x6c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xa3, 0x03,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0e, 0x06, 0x12, 0x04, 0xa3, 0x03, 0x11,
    0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xa3, 0x03, 0x47, 0x66,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xa3, 0x03, 0x69, 0x6b, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x2a, 0x12, 0x06, 0xa6, 0x03, 0x00, 0xa8, 0x03, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x2a, 0x01, 0x12, 0x04, 0xa6, 0x03, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a,
    0x02, 0x00, 0x12, 0x04, 0xa7, 0x03, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xa7, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xa7, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xa7, 0x03, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xa7, 0x03, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06, 0xaa, 0x03, 0x00, 0xad,
    0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04, 0xaa, 0x03, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x00, 0x12, 0x04, 0xab, 0x03, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xab, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xab, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xab, 0x03, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xab, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02,
    0x01, 0x12, 0x04, 0xac, 0x03, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xac, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xac, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xac, 0x03, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xac,
    0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xaf, 0x03, 0x00, 0xb2, 0x03,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0xaf, 0x03, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2c, 0x02, 0x00, 0x12, 0x04, 0xb0, 0x03, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb0, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb0, 0x03, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x03, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xb0, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x01,
    0x12, 0x04, 0xb1, 0x03, 0x08, 0x58, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xb1, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xb1, 0x03, 0x11, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb1,
    0x03, 0x22, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb1, 0x03,
    0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x08, 0x12, 0x04, 0xb1, 0x03, 0x31,
    0x57, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x07, 0x12, 0x04, 0xb1, 0x03, 0x3c, 0x56,
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
