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
pub struct CMsgTEArmorRicochet {
    // message fields
    pos: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    dir: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEArmorRicochet {}

impl CMsgTEArmorRicochet {
    pub fn new() -> CMsgTEArmorRicochet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEArmorRicochet {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEArmorRicochet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEArmorRicochet,
        };
        unsafe {
            instance.get(CMsgTEArmorRicochet::new)
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

    // optional .dota.CMsgVector dir = 2;

    pub fn clear_dir(&mut self) {
        self.dir.clear();
    }

    pub fn has_dir(&self) -> bool {
        self.dir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.dir = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dir(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.dir.is_none() {
            self.dir.set_default();
        };
        self.dir.as_mut().unwrap()
    }

    // Take field
    pub fn take_dir(&mut self) -> super::networkbasetypes::CMsgVector {
        self.dir.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_dir(&self) -> &super::networkbasetypes::CMsgVector {
        self.dir.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_dir_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.dir
    }

    fn mut_dir_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.dir
    }
}

impl ::protobuf::Message for CMsgTEArmorRicochet {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dir)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.dir.as_ref() {
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
        if let Some(v) = self.dir.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgTEArmorRicochet {
    fn new() -> CMsgTEArmorRicochet {
        CMsgTEArmorRicochet::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEArmorRicochet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "pos",
                    CMsgTEArmorRicochet::get_pos_for_reflect,
                    CMsgTEArmorRicochet::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "dir",
                    CMsgTEArmorRicochet::get_dir_for_reflect,
                    CMsgTEArmorRicochet::mut_dir_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEArmorRicochet>(
                    "CMsgTEArmorRicochet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEArmorRicochet {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_dir();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEArmorRicochet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEArmorRicochet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBaseBeam {
    // message fields
    modelindex: ::std::option::Option<u64>,
    haloindex: ::std::option::Option<u64>,
    startframe: ::std::option::Option<u32>,
    framerate: ::std::option::Option<u32>,
    life: ::std::option::Option<f32>,
    width: ::std::option::Option<f32>,
    endwidth: ::std::option::Option<f32>,
    fadelength: ::std::option::Option<u32>,
    amplitude: ::std::option::Option<f32>,
    color: ::std::option::Option<u32>,
    speed: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBaseBeam {}

impl CMsgTEBaseBeam {
    pub fn new() -> CMsgTEBaseBeam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBaseBeam {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBaseBeam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBaseBeam,
        };
        unsafe {
            instance.get(CMsgTEBaseBeam::new)
        }
    }

    // optional fixed64 modelindex = 1;

    pub fn clear_modelindex(&mut self) {
        self.modelindex = ::std::option::Option::None;
    }

    pub fn has_modelindex(&self) -> bool {
        self.modelindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modelindex(&mut self, v: u64) {
        self.modelindex = ::std::option::Option::Some(v);
    }

    pub fn get_modelindex(&self) -> u64 {
        self.modelindex.unwrap_or(0)
    }

    fn get_modelindex_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.modelindex
    }

    fn mut_modelindex_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.modelindex
    }

    // optional fixed64 haloindex = 2;

    pub fn clear_haloindex(&mut self) {
        self.haloindex = ::std::option::Option::None;
    }

    pub fn has_haloindex(&self) -> bool {
        self.haloindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_haloindex(&mut self, v: u64) {
        self.haloindex = ::std::option::Option::Some(v);
    }

    pub fn get_haloindex(&self) -> u64 {
        self.haloindex.unwrap_or(0)
    }

    fn get_haloindex_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.haloindex
    }

    fn mut_haloindex_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.haloindex
    }

    // optional uint32 startframe = 3;

    pub fn clear_startframe(&mut self) {
        self.startframe = ::std::option::Option::None;
    }

    pub fn has_startframe(&self) -> bool {
        self.startframe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startframe(&mut self, v: u32) {
        self.startframe = ::std::option::Option::Some(v);
    }

    pub fn get_startframe(&self) -> u32 {
        self.startframe.unwrap_or(0)
    }

    fn get_startframe_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.startframe
    }

    fn mut_startframe_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.startframe
    }

    // optional uint32 framerate = 4;

    pub fn clear_framerate(&mut self) {
        self.framerate = ::std::option::Option::None;
    }

    pub fn has_framerate(&self) -> bool {
        self.framerate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framerate(&mut self, v: u32) {
        self.framerate = ::std::option::Option::Some(v);
    }

    pub fn get_framerate(&self) -> u32 {
        self.framerate.unwrap_or(0)
    }

    fn get_framerate_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.framerate
    }

    fn mut_framerate_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.framerate
    }

    // optional float life = 5;

    pub fn clear_life(&mut self) {
        self.life = ::std::option::Option::None;
    }

    pub fn has_life(&self) -> bool {
        self.life.is_some()
    }

    // Param is passed by value, moved
    pub fn set_life(&mut self, v: f32) {
        self.life = ::std::option::Option::Some(v);
    }

    pub fn get_life(&self) -> f32 {
        self.life.unwrap_or(0.)
    }

    fn get_life_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.life
    }

    fn mut_life_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.life
    }

    // optional float width = 6;

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

    // optional float endwidth = 7;

    pub fn clear_endwidth(&mut self) {
        self.endwidth = ::std::option::Option::None;
    }

    pub fn has_endwidth(&self) -> bool {
        self.endwidth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endwidth(&mut self, v: f32) {
        self.endwidth = ::std::option::Option::Some(v);
    }

    pub fn get_endwidth(&self) -> f32 {
        self.endwidth.unwrap_or(0.)
    }

    fn get_endwidth_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.endwidth
    }

    fn mut_endwidth_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.endwidth
    }

    // optional uint32 fadelength = 8;

    pub fn clear_fadelength(&mut self) {
        self.fadelength = ::std::option::Option::None;
    }

    pub fn has_fadelength(&self) -> bool {
        self.fadelength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fadelength(&mut self, v: u32) {
        self.fadelength = ::std::option::Option::Some(v);
    }

    pub fn get_fadelength(&self) -> u32 {
        self.fadelength.unwrap_or(0)
    }

    fn get_fadelength_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.fadelength
    }

    fn mut_fadelength_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.fadelength
    }

    // optional float amplitude = 9;

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

    // optional fixed32 color = 10;

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

    // optional uint32 speed = 11;

    pub fn clear_speed(&mut self) {
        self.speed = ::std::option::Option::None;
    }

    pub fn has_speed(&self) -> bool {
        self.speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speed(&mut self, v: u32) {
        self.speed = ::std::option::Option::Some(v);
    }

    pub fn get_speed(&self) -> u32 {
        self.speed.unwrap_or(0)
    }

    fn get_speed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.speed
    }

    fn mut_speed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.speed
    }

    // optional uint32 flags = 12;

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
}

impl ::protobuf::Message for CMsgTEBaseBeam {
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
                    self.modelindex = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.haloindex = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.startframe = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.framerate = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.life = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.width = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.endwidth = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.fadelength = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.amplitude = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.color = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.speed = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
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
        if let Some(v) = self.modelindex {
            my_size += 9;
        };
        if let Some(v) = self.haloindex {
            my_size += 9;
        };
        if let Some(v) = self.startframe {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.framerate {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.life {
            my_size += 5;
        };
        if let Some(v) = self.width {
            my_size += 5;
        };
        if let Some(v) = self.endwidth {
            my_size += 5;
        };
        if let Some(v) = self.fadelength {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.amplitude {
            my_size += 5;
        };
        if let Some(v) = self.color {
            my_size += 5;
        };
        if let Some(v) = self.speed {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.modelindex {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.haloindex {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.startframe {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.framerate {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.life {
            os.write_float(5, v)?;
        };
        if let Some(v) = self.width {
            os.write_float(6, v)?;
        };
        if let Some(v) = self.endwidth {
            os.write_float(7, v)?;
        };
        if let Some(v) = self.fadelength {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.amplitude {
            os.write_float(9, v)?;
        };
        if let Some(v) = self.color {
            os.write_fixed32(10, v)?;
        };
        if let Some(v) = self.speed {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(12, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgTEBaseBeam {
    fn new() -> CMsgTEBaseBeam {
        CMsgTEBaseBeam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBaseBeam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "modelindex",
                    CMsgTEBaseBeam::get_modelindex_for_reflect,
                    CMsgTEBaseBeam::mut_modelindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "haloindex",
                    CMsgTEBaseBeam::get_haloindex_for_reflect,
                    CMsgTEBaseBeam::mut_haloindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "startframe",
                    CMsgTEBaseBeam::get_startframe_for_reflect,
                    CMsgTEBaseBeam::mut_startframe_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "framerate",
                    CMsgTEBaseBeam::get_framerate_for_reflect,
                    CMsgTEBaseBeam::mut_framerate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "life",
                    CMsgTEBaseBeam::get_life_for_reflect,
                    CMsgTEBaseBeam::mut_life_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "width",
                    CMsgTEBaseBeam::get_width_for_reflect,
                    CMsgTEBaseBeam::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "endwidth",
                    CMsgTEBaseBeam::get_endwidth_for_reflect,
                    CMsgTEBaseBeam::mut_endwidth_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "fadelength",
                    CMsgTEBaseBeam::get_fadelength_for_reflect,
                    CMsgTEBaseBeam::mut_fadelength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "amplitude",
                    CMsgTEBaseBeam::get_amplitude_for_reflect,
                    CMsgTEBaseBeam::mut_amplitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "color",
                    CMsgTEBaseBeam::get_color_for_reflect,
                    CMsgTEBaseBeam::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "speed",
                    CMsgTEBaseBeam::get_speed_for_reflect,
                    CMsgTEBaseBeam::mut_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgTEBaseBeam::get_flags_for_reflect,
                    CMsgTEBaseBeam::mut_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBaseBeam>(
                    "CMsgTEBaseBeam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBaseBeam {
    fn clear(&mut self) {
        self.clear_modelindex();
        self.clear_haloindex();
        self.clear_startframe();
        self.clear_framerate();
        self.clear_life();
        self.clear_width();
        self.clear_endwidth();
        self.clear_fadelength();
        self.clear_amplitude();
        self.clear_color();
        self.clear_speed();
        self.clear_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBaseBeam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBaseBeam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBeamEntPoint {
    // message fields
    base: ::protobuf::SingularPtrField<CMsgTEBaseBeam>,
    startentity: ::std::option::Option<u32>,
    endentity: ::std::option::Option<u32>,
    start: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    end: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBeamEntPoint {}

impl CMsgTEBeamEntPoint {
    pub fn new() -> CMsgTEBeamEntPoint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBeamEntPoint {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBeamEntPoint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBeamEntPoint,
        };
        unsafe {
            instance.get(CMsgTEBeamEntPoint::new)
        }
    }

    // optional .dota.CMsgTEBaseBeam base = 1;

    pub fn clear_base(&mut self) {
        self.base.clear();
    }

    pub fn has_base(&self) -> bool {
        self.base.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base(&mut self, v: CMsgTEBaseBeam) {
        self.base = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base(&mut self) -> &mut CMsgTEBaseBeam {
        if self.base.is_none() {
            self.base.set_default();
        };
        self.base.as_mut().unwrap()
    }

    // Take field
    pub fn take_base(&mut self) -> CMsgTEBaseBeam {
        self.base.take().unwrap_or_else(|| CMsgTEBaseBeam::new())
    }

    pub fn get_base(&self) -> &CMsgTEBaseBeam {
        self.base.as_ref().unwrap_or_else(|| CMsgTEBaseBeam::default_instance())
    }

    fn get_base_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgTEBaseBeam> {
        &self.base
    }

    fn mut_base_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgTEBaseBeam> {
        &mut self.base
    }

    // optional uint32 startentity = 2;

    pub fn clear_startentity(&mut self) {
        self.startentity = ::std::option::Option::None;
    }

    pub fn has_startentity(&self) -> bool {
        self.startentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startentity(&mut self, v: u32) {
        self.startentity = ::std::option::Option::Some(v);
    }

    pub fn get_startentity(&self) -> u32 {
        self.startentity.unwrap_or(0)
    }

    fn get_startentity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.startentity
    }

    fn mut_startentity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.startentity
    }

    // optional uint32 endentity = 3;

    pub fn clear_endentity(&mut self) {
        self.endentity = ::std::option::Option::None;
    }

    pub fn has_endentity(&self) -> bool {
        self.endentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endentity(&mut self, v: u32) {
        self.endentity = ::std::option::Option::Some(v);
    }

    pub fn get_endentity(&self) -> u32 {
        self.endentity.unwrap_or(0)
    }

    fn get_endentity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.endentity
    }

    fn mut_endentity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.endentity
    }

    // optional .dota.CMsgVector start = 4;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.start.is_none() {
            self.start.set_default();
        };
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> super::networkbasetypes::CMsgVector {
        self.start.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_start(&self) -> &super::networkbasetypes::CMsgVector {
        self.start.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_start_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.start
    }

    // optional .dota.CMsgVector end = 5;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.end.is_none() {
            self.end.set_default();
        };
        self.end.as_mut().unwrap()
    }

    // Take field
    pub fn take_end(&mut self) -> super::networkbasetypes::CMsgVector {
        self.end.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_end(&self) -> &super::networkbasetypes::CMsgVector {
        self.end.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_end_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.end
    }
}

impl ::protobuf::Message for CMsgTEBeamEntPoint {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.base)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.startentity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.endentity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.base.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.startentity {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.endentity {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.end.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.startentity {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.endentity {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.start.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.end.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgTEBeamEntPoint {
    fn new() -> CMsgTEBeamEntPoint {
        CMsgTEBeamEntPoint::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBeamEntPoint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgTEBaseBeam>>(
                    "base",
                    CMsgTEBeamEntPoint::get_base_for_reflect,
                    CMsgTEBeamEntPoint::mut_base_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "startentity",
                    CMsgTEBeamEntPoint::get_startentity_for_reflect,
                    CMsgTEBeamEntPoint::mut_startentity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "endentity",
                    CMsgTEBeamEntPoint::get_endentity_for_reflect,
                    CMsgTEBeamEntPoint::mut_endentity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "start",
                    CMsgTEBeamEntPoint::get_start_for_reflect,
                    CMsgTEBeamEntPoint::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "end",
                    CMsgTEBeamEntPoint::get_end_for_reflect,
                    CMsgTEBeamEntPoint::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBeamEntPoint>(
                    "CMsgTEBeamEntPoint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBeamEntPoint {
    fn clear(&mut self) {
        self.clear_base();
        self.clear_startentity();
        self.clear_endentity();
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBeamEntPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBeamEntPoint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBeamEnts {
    // message fields
    base: ::protobuf::SingularPtrField<CMsgTEBaseBeam>,
    startentity: ::std::option::Option<u32>,
    endentity: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBeamEnts {}

impl CMsgTEBeamEnts {
    pub fn new() -> CMsgTEBeamEnts {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBeamEnts {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBeamEnts> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBeamEnts,
        };
        unsafe {
            instance.get(CMsgTEBeamEnts::new)
        }
    }

    // optional .dota.CMsgTEBaseBeam base = 1;

    pub fn clear_base(&mut self) {
        self.base.clear();
    }

    pub fn has_base(&self) -> bool {
        self.base.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base(&mut self, v: CMsgTEBaseBeam) {
        self.base = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base(&mut self) -> &mut CMsgTEBaseBeam {
        if self.base.is_none() {
            self.base.set_default();
        };
        self.base.as_mut().unwrap()
    }

    // Take field
    pub fn take_base(&mut self) -> CMsgTEBaseBeam {
        self.base.take().unwrap_or_else(|| CMsgTEBaseBeam::new())
    }

    pub fn get_base(&self) -> &CMsgTEBaseBeam {
        self.base.as_ref().unwrap_or_else(|| CMsgTEBaseBeam::default_instance())
    }

    fn get_base_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgTEBaseBeam> {
        &self.base
    }

    fn mut_base_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgTEBaseBeam> {
        &mut self.base
    }

    // optional uint32 startentity = 2;

    pub fn clear_startentity(&mut self) {
        self.startentity = ::std::option::Option::None;
    }

    pub fn has_startentity(&self) -> bool {
        self.startentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startentity(&mut self, v: u32) {
        self.startentity = ::std::option::Option::Some(v);
    }

    pub fn get_startentity(&self) -> u32 {
        self.startentity.unwrap_or(0)
    }

    fn get_startentity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.startentity
    }

    fn mut_startentity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.startentity
    }

    // optional uint32 endentity = 3;

    pub fn clear_endentity(&mut self) {
        self.endentity = ::std::option::Option::None;
    }

    pub fn has_endentity(&self) -> bool {
        self.endentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endentity(&mut self, v: u32) {
        self.endentity = ::std::option::Option::Some(v);
    }

    pub fn get_endentity(&self) -> u32 {
        self.endentity.unwrap_or(0)
    }

    fn get_endentity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.endentity
    }

    fn mut_endentity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.endentity
    }
}

impl ::protobuf::Message for CMsgTEBeamEnts {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.base)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.startentity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.endentity = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.base.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.startentity {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.endentity {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.startentity {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.endentity {
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

impl ::protobuf::MessageStatic for CMsgTEBeamEnts {
    fn new() -> CMsgTEBeamEnts {
        CMsgTEBeamEnts::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBeamEnts>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgTEBaseBeam>>(
                    "base",
                    CMsgTEBeamEnts::get_base_for_reflect,
                    CMsgTEBeamEnts::mut_base_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "startentity",
                    CMsgTEBeamEnts::get_startentity_for_reflect,
                    CMsgTEBeamEnts::mut_startentity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "endentity",
                    CMsgTEBeamEnts::get_endentity_for_reflect,
                    CMsgTEBeamEnts::mut_endentity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBeamEnts>(
                    "CMsgTEBeamEnts",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBeamEnts {
    fn clear(&mut self) {
        self.clear_base();
        self.clear_startentity();
        self.clear_endentity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBeamEnts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBeamEnts {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBeamPoints {
    // message fields
    base: ::protobuf::SingularPtrField<CMsgTEBaseBeam>,
    start: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    end: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBeamPoints {}

impl CMsgTEBeamPoints {
    pub fn new() -> CMsgTEBeamPoints {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBeamPoints {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBeamPoints> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBeamPoints,
        };
        unsafe {
            instance.get(CMsgTEBeamPoints::new)
        }
    }

    // optional .dota.CMsgTEBaseBeam base = 1;

    pub fn clear_base(&mut self) {
        self.base.clear();
    }

    pub fn has_base(&self) -> bool {
        self.base.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base(&mut self, v: CMsgTEBaseBeam) {
        self.base = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base(&mut self) -> &mut CMsgTEBaseBeam {
        if self.base.is_none() {
            self.base.set_default();
        };
        self.base.as_mut().unwrap()
    }

    // Take field
    pub fn take_base(&mut self) -> CMsgTEBaseBeam {
        self.base.take().unwrap_or_else(|| CMsgTEBaseBeam::new())
    }

    pub fn get_base(&self) -> &CMsgTEBaseBeam {
        self.base.as_ref().unwrap_or_else(|| CMsgTEBaseBeam::default_instance())
    }

    fn get_base_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgTEBaseBeam> {
        &self.base
    }

    fn mut_base_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgTEBaseBeam> {
        &mut self.base
    }

    // optional .dota.CMsgVector start = 2;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.start.is_none() {
            self.start.set_default();
        };
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> super::networkbasetypes::CMsgVector {
        self.start.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_start(&self) -> &super::networkbasetypes::CMsgVector {
        self.start.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_start_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.start
    }

    // optional .dota.CMsgVector end = 3;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.end.is_none() {
            self.end.set_default();
        };
        self.end.as_mut().unwrap()
    }

    // Take field
    pub fn take_end(&mut self) -> super::networkbasetypes::CMsgVector {
        self.end.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_end(&self) -> &super::networkbasetypes::CMsgVector {
        self.end.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_end_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.end
    }
}

impl ::protobuf::Message for CMsgTEBeamPoints {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.base)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.base.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.end.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.start.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.end.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgTEBeamPoints {
    fn new() -> CMsgTEBeamPoints {
        CMsgTEBeamPoints::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBeamPoints>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgTEBaseBeam>>(
                    "base",
                    CMsgTEBeamPoints::get_base_for_reflect,
                    CMsgTEBeamPoints::mut_base_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "start",
                    CMsgTEBeamPoints::get_start_for_reflect,
                    CMsgTEBeamPoints::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "end",
                    CMsgTEBeamPoints::get_end_for_reflect,
                    CMsgTEBeamPoints::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBeamPoints>(
                    "CMsgTEBeamPoints",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBeamPoints {
    fn clear(&mut self) {
        self.clear_base();
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBeamPoints {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBeamPoints {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBeamRing {
    // message fields
    base: ::protobuf::SingularPtrField<CMsgTEBaseBeam>,
    startentity: ::std::option::Option<u32>,
    endentity: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBeamRing {}

impl CMsgTEBeamRing {
    pub fn new() -> CMsgTEBeamRing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBeamRing {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBeamRing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBeamRing,
        };
        unsafe {
            instance.get(CMsgTEBeamRing::new)
        }
    }

    // optional .dota.CMsgTEBaseBeam base = 1;

    pub fn clear_base(&mut self) {
        self.base.clear();
    }

    pub fn has_base(&self) -> bool {
        self.base.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base(&mut self, v: CMsgTEBaseBeam) {
        self.base = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_base(&mut self) -> &mut CMsgTEBaseBeam {
        if self.base.is_none() {
            self.base.set_default();
        };
        self.base.as_mut().unwrap()
    }

    // Take field
    pub fn take_base(&mut self) -> CMsgTEBaseBeam {
        self.base.take().unwrap_or_else(|| CMsgTEBaseBeam::new())
    }

    pub fn get_base(&self) -> &CMsgTEBaseBeam {
        self.base.as_ref().unwrap_or_else(|| CMsgTEBaseBeam::default_instance())
    }

    fn get_base_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgTEBaseBeam> {
        &self.base
    }

    fn mut_base_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgTEBaseBeam> {
        &mut self.base
    }

    // optional uint32 startentity = 2;

    pub fn clear_startentity(&mut self) {
        self.startentity = ::std::option::Option::None;
    }

    pub fn has_startentity(&self) -> bool {
        self.startentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startentity(&mut self, v: u32) {
        self.startentity = ::std::option::Option::Some(v);
    }

    pub fn get_startentity(&self) -> u32 {
        self.startentity.unwrap_or(0)
    }

    fn get_startentity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.startentity
    }

    fn mut_startentity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.startentity
    }

    // optional uint32 endentity = 3;

    pub fn clear_endentity(&mut self) {
        self.endentity = ::std::option::Option::None;
    }

    pub fn has_endentity(&self) -> bool {
        self.endentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endentity(&mut self, v: u32) {
        self.endentity = ::std::option::Option::Some(v);
    }

    pub fn get_endentity(&self) -> u32 {
        self.endentity.unwrap_or(0)
    }

    fn get_endentity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.endentity
    }

    fn mut_endentity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.endentity
    }
}

impl ::protobuf::Message for CMsgTEBeamRing {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.base)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.startentity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.endentity = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.base.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.startentity {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.endentity {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.base.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.startentity {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.endentity {
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

impl ::protobuf::MessageStatic for CMsgTEBeamRing {
    fn new() -> CMsgTEBeamRing {
        CMsgTEBeamRing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBeamRing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgTEBaseBeam>>(
                    "base",
                    CMsgTEBeamRing::get_base_for_reflect,
                    CMsgTEBeamRing::mut_base_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "startentity",
                    CMsgTEBeamRing::get_startentity_for_reflect,
                    CMsgTEBeamRing::mut_startentity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "endentity",
                    CMsgTEBeamRing::get_endentity_for_reflect,
                    CMsgTEBeamRing::mut_endentity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBeamRing>(
                    "CMsgTEBeamRing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBeamRing {
    fn clear(&mut self) {
        self.clear_base();
        self.clear_startentity();
        self.clear_endentity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBeamRing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBeamRing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBreakModel {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    angles: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    size: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    velocity: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    randomization: ::std::option::Option<u32>,
    modelindex: ::std::option::Option<u64>,
    count: ::std::option::Option<u32>,
    time: ::std::option::Option<f32>,
    flags: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBreakModel {}

impl CMsgTEBreakModel {
    pub fn new() -> CMsgTEBreakModel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBreakModel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBreakModel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBreakModel,
        };
        unsafe {
            instance.get(CMsgTEBreakModel::new)
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

    // optional .dota.CMsgQAngle angles = 2;

    pub fn clear_angles(&mut self) {
        self.angles.clear();
    }

    pub fn has_angles(&self) -> bool {
        self.angles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angles(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angles = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angles(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angles.is_none() {
            self.angles.set_default();
        };
        self.angles.as_mut().unwrap()
    }

    // Take field
    pub fn take_angles(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angles.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angles(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angles.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angles_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angles
    }

    fn mut_angles_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angles
    }

    // optional .dota.CMsgVector size = 3;

    pub fn clear_size(&mut self) {
        self.size.clear();
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_size(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.size.is_none() {
            self.size.set_default();
        };
        self.size.as_mut().unwrap()
    }

    // Take field
    pub fn take_size(&mut self) -> super::networkbasetypes::CMsgVector {
        self.size.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_size(&self) -> &super::networkbasetypes::CMsgVector {
        self.size.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_size_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.size
    }

    // optional .dota.CMsgVector velocity = 4;

    pub fn clear_velocity(&mut self) {
        self.velocity.clear();
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.velocity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_velocity(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.velocity.is_none() {
            self.velocity.set_default();
        };
        self.velocity.as_mut().unwrap()
    }

    // Take field
    pub fn take_velocity(&mut self) -> super::networkbasetypes::CMsgVector {
        self.velocity.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_velocity(&self) -> &super::networkbasetypes::CMsgVector {
        self.velocity.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_velocity_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.velocity
    }

    fn mut_velocity_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.velocity
    }

    // optional uint32 randomization = 5;

    pub fn clear_randomization(&mut self) {
        self.randomization = ::std::option::Option::None;
    }

    pub fn has_randomization(&self) -> bool {
        self.randomization.is_some()
    }

    // Param is passed by value, moved
    pub fn set_randomization(&mut self, v: u32) {
        self.randomization = ::std::option::Option::Some(v);
    }

    pub fn get_randomization(&self) -> u32 {
        self.randomization.unwrap_or(0)
    }

    fn get_randomization_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.randomization
    }

    fn mut_randomization_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.randomization
    }

    // optional fixed64 modelindex = 6;

    pub fn clear_modelindex(&mut self) {
        self.modelindex = ::std::option::Option::None;
    }

    pub fn has_modelindex(&self) -> bool {
        self.modelindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modelindex(&mut self, v: u64) {
        self.modelindex = ::std::option::Option::Some(v);
    }

    pub fn get_modelindex(&self) -> u64 {
        self.modelindex.unwrap_or(0)
    }

    fn get_modelindex_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.modelindex
    }

    fn mut_modelindex_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.modelindex
    }

    // optional uint32 count = 7;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.count
    }

    // optional float time = 8;

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

    // optional uint32 flags = 9;

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
}

impl ::protobuf::Message for CMsgTEBreakModel {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angles)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.size)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.velocity)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.randomization = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.modelindex = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.count = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
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
        if let Some(v) = self.origin.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.angles.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.velocity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.randomization {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.modelindex {
            my_size += 9;
        };
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.time {
            my_size += 5;
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.angles.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.size.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.velocity.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.randomization {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.modelindex {
            os.write_fixed64(6, v)?;
        };
        if let Some(v) = self.count {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.time {
            os.write_float(8, v)?;
        };
        if let Some(v) = self.flags {
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

impl ::protobuf::MessageStatic for CMsgTEBreakModel {
    fn new() -> CMsgTEBreakModel {
        CMsgTEBreakModel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBreakModel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEBreakModel::get_origin_for_reflect,
                    CMsgTEBreakModel::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angles",
                    CMsgTEBreakModel::get_angles_for_reflect,
                    CMsgTEBreakModel::mut_angles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "size",
                    CMsgTEBreakModel::get_size_for_reflect,
                    CMsgTEBreakModel::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "velocity",
                    CMsgTEBreakModel::get_velocity_for_reflect,
                    CMsgTEBreakModel::mut_velocity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "randomization",
                    CMsgTEBreakModel::get_randomization_for_reflect,
                    CMsgTEBreakModel::mut_randomization_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "modelindex",
                    CMsgTEBreakModel::get_modelindex_for_reflect,
                    CMsgTEBreakModel::mut_modelindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    CMsgTEBreakModel::get_count_for_reflect,
                    CMsgTEBreakModel::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "time",
                    CMsgTEBreakModel::get_time_for_reflect,
                    CMsgTEBreakModel::mut_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgTEBreakModel::get_flags_for_reflect,
                    CMsgTEBreakModel::mut_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBreakModel>(
                    "CMsgTEBreakModel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBreakModel {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_angles();
        self.clear_size();
        self.clear_velocity();
        self.clear_randomization();
        self.clear_modelindex();
        self.clear_count();
        self.clear_time();
        self.clear_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBreakModel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBreakModel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBSPDecal {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    normal: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    saxis: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    entity: ::std::option::Option<u32>,
    index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBSPDecal {}

impl CMsgTEBSPDecal {
    pub fn new() -> CMsgTEBSPDecal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBSPDecal {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBSPDecal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBSPDecal,
        };
        unsafe {
            instance.get(CMsgTEBSPDecal::new)
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

    // optional uint32 entity = 4;

    pub fn clear_entity(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_entity(&self) -> bool {
        self.entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(v);
    }

    pub fn get_entity(&self) -> u32 {
        self.entity.unwrap_or(0)
    }

    fn get_entity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entity
    }

    fn mut_entity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entity
    }

    // optional uint32 index = 5;

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
}

impl ::protobuf::Message for CMsgTEBSPDecal {
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
                    self.entity = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.normal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.saxis.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.entity {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.entity {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.index {
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

impl ::protobuf::MessageStatic for CMsgTEBSPDecal {
    fn new() -> CMsgTEBSPDecal {
        CMsgTEBSPDecal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBSPDecal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEBSPDecal::get_origin_for_reflect,
                    CMsgTEBSPDecal::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "normal",
                    CMsgTEBSPDecal::get_normal_for_reflect,
                    CMsgTEBSPDecal::mut_normal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "saxis",
                    CMsgTEBSPDecal::get_saxis_for_reflect,
                    CMsgTEBSPDecal::mut_saxis_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "entity",
                    CMsgTEBSPDecal::get_entity_for_reflect,
                    CMsgTEBSPDecal::mut_entity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    CMsgTEBSPDecal::get_index_for_reflect,
                    CMsgTEBSPDecal::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBSPDecal>(
                    "CMsgTEBSPDecal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBSPDecal {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_normal();
        self.clear_saxis();
        self.clear_entity();
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBSPDecal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBSPDecal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBubbles {
    // message fields
    mins: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    maxs: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    height: ::std::option::Option<f32>,
    count: ::std::option::Option<u32>,
    speed: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBubbles {}

impl CMsgTEBubbles {
    pub fn new() -> CMsgTEBubbles {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBubbles {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBubbles> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBubbles,
        };
        unsafe {
            instance.get(CMsgTEBubbles::new)
        }
    }

    // optional .dota.CMsgVector mins = 1;

    pub fn clear_mins(&mut self) {
        self.mins.clear();
    }

    pub fn has_mins(&self) -> bool {
        self.mins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mins(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.mins = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mins(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.mins.is_none() {
            self.mins.set_default();
        };
        self.mins.as_mut().unwrap()
    }

    // Take field
    pub fn take_mins(&mut self) -> super::networkbasetypes::CMsgVector {
        self.mins.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_mins(&self) -> &super::networkbasetypes::CMsgVector {
        self.mins.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_mins_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.mins
    }

    fn mut_mins_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.mins
    }

    // optional .dota.CMsgVector maxs = 2;

    pub fn clear_maxs(&mut self) {
        self.maxs.clear();
    }

    pub fn has_maxs(&self) -> bool {
        self.maxs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxs(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.maxs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maxs(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.maxs.is_none() {
            self.maxs.set_default();
        };
        self.maxs.as_mut().unwrap()
    }

    // Take field
    pub fn take_maxs(&mut self) -> super::networkbasetypes::CMsgVector {
        self.maxs.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_maxs(&self) -> &super::networkbasetypes::CMsgVector {
        self.maxs.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_maxs_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.maxs
    }

    fn mut_maxs_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.maxs
    }

    // optional float height = 3;

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

    // optional uint32 count = 4;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.count
    }

    // optional float speed = 5;

    pub fn clear_speed(&mut self) {
        self.speed = ::std::option::Option::None;
    }

    pub fn has_speed(&self) -> bool {
        self.speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speed(&mut self, v: f32) {
        self.speed = ::std::option::Option::Some(v);
    }

    pub fn get_speed(&self) -> f32 {
        self.speed.unwrap_or(0.)
    }

    fn get_speed_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.speed
    }

    fn mut_speed_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.speed
    }
}

impl ::protobuf::Message for CMsgTEBubbles {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mins)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.maxs)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.height = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.count = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.speed = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.mins.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.maxs.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.height {
            my_size += 5;
        };
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.speed {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mins.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.maxs.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.height {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.count {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.speed {
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

impl ::protobuf::MessageStatic for CMsgTEBubbles {
    fn new() -> CMsgTEBubbles {
        CMsgTEBubbles::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBubbles>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "mins",
                    CMsgTEBubbles::get_mins_for_reflect,
                    CMsgTEBubbles::mut_mins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "maxs",
                    CMsgTEBubbles::get_maxs_for_reflect,
                    CMsgTEBubbles::mut_maxs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "height",
                    CMsgTEBubbles::get_height_for_reflect,
                    CMsgTEBubbles::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    CMsgTEBubbles::get_count_for_reflect,
                    CMsgTEBubbles::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "speed",
                    CMsgTEBubbles::get_speed_for_reflect,
                    CMsgTEBubbles::mut_speed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBubbles>(
                    "CMsgTEBubbles",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBubbles {
    fn clear(&mut self) {
        self.clear_mins();
        self.clear_maxs();
        self.clear_height();
        self.clear_count();
        self.clear_speed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBubbles {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBubbles {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBubbleTrail {
    // message fields
    mins: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    maxs: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    waterz: ::std::option::Option<f32>,
    count: ::std::option::Option<u32>,
    speed: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBubbleTrail {}

impl CMsgTEBubbleTrail {
    pub fn new() -> CMsgTEBubbleTrail {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBubbleTrail {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBubbleTrail> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBubbleTrail,
        };
        unsafe {
            instance.get(CMsgTEBubbleTrail::new)
        }
    }

    // optional .dota.CMsgVector mins = 1;

    pub fn clear_mins(&mut self) {
        self.mins.clear();
    }

    pub fn has_mins(&self) -> bool {
        self.mins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mins(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.mins = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mins(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.mins.is_none() {
            self.mins.set_default();
        };
        self.mins.as_mut().unwrap()
    }

    // Take field
    pub fn take_mins(&mut self) -> super::networkbasetypes::CMsgVector {
        self.mins.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_mins(&self) -> &super::networkbasetypes::CMsgVector {
        self.mins.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_mins_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.mins
    }

    fn mut_mins_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.mins
    }

    // optional .dota.CMsgVector maxs = 2;

    pub fn clear_maxs(&mut self) {
        self.maxs.clear();
    }

    pub fn has_maxs(&self) -> bool {
        self.maxs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxs(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.maxs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maxs(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.maxs.is_none() {
            self.maxs.set_default();
        };
        self.maxs.as_mut().unwrap()
    }

    // Take field
    pub fn take_maxs(&mut self) -> super::networkbasetypes::CMsgVector {
        self.maxs.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_maxs(&self) -> &super::networkbasetypes::CMsgVector {
        self.maxs.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_maxs_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.maxs
    }

    fn mut_maxs_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.maxs
    }

    // optional float waterz = 3;

    pub fn clear_waterz(&mut self) {
        self.waterz = ::std::option::Option::None;
    }

    pub fn has_waterz(&self) -> bool {
        self.waterz.is_some()
    }

    // Param is passed by value, moved
    pub fn set_waterz(&mut self, v: f32) {
        self.waterz = ::std::option::Option::Some(v);
    }

    pub fn get_waterz(&self) -> f32 {
        self.waterz.unwrap_or(0.)
    }

    fn get_waterz_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.waterz
    }

    fn mut_waterz_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.waterz
    }

    // optional uint32 count = 4;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.count
    }

    // optional float speed = 5;

    pub fn clear_speed(&mut self) {
        self.speed = ::std::option::Option::None;
    }

    pub fn has_speed(&self) -> bool {
        self.speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speed(&mut self, v: f32) {
        self.speed = ::std::option::Option::Some(v);
    }

    pub fn get_speed(&self) -> f32 {
        self.speed.unwrap_or(0.)
    }

    fn get_speed_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.speed
    }

    fn mut_speed_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.speed
    }
}

impl ::protobuf::Message for CMsgTEBubbleTrail {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mins)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.maxs)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.waterz = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.count = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.speed = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.mins.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.maxs.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.waterz {
            my_size += 5;
        };
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.speed {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mins.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.maxs.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.waterz {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.count {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.speed {
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

impl ::protobuf::MessageStatic for CMsgTEBubbleTrail {
    fn new() -> CMsgTEBubbleTrail {
        CMsgTEBubbleTrail::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBubbleTrail>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "mins",
                    CMsgTEBubbleTrail::get_mins_for_reflect,
                    CMsgTEBubbleTrail::mut_mins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "maxs",
                    CMsgTEBubbleTrail::get_maxs_for_reflect,
                    CMsgTEBubbleTrail::mut_maxs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "waterz",
                    CMsgTEBubbleTrail::get_waterz_for_reflect,
                    CMsgTEBubbleTrail::mut_waterz_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    CMsgTEBubbleTrail::get_count_for_reflect,
                    CMsgTEBubbleTrail::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "speed",
                    CMsgTEBubbleTrail::get_speed_for_reflect,
                    CMsgTEBubbleTrail::mut_speed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBubbleTrail>(
                    "CMsgTEBubbleTrail",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBubbleTrail {
    fn clear(&mut self) {
        self.clear_mins();
        self.clear_maxs();
        self.clear_waterz();
        self.clear_count();
        self.clear_speed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBubbleTrail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBubbleTrail {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEDecal {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    start: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    entity: ::std::option::Option<u32>,
    hitbox: ::std::option::Option<u32>,
    index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEDecal {}

impl CMsgTEDecal {
    pub fn new() -> CMsgTEDecal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEDecal {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEDecal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEDecal,
        };
        unsafe {
            instance.get(CMsgTEDecal::new)
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

    // optional .dota.CMsgVector start = 2;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.start.is_none() {
            self.start.set_default();
        };
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> super::networkbasetypes::CMsgVector {
        self.start.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_start(&self) -> &super::networkbasetypes::CMsgVector {
        self.start.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_start_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.start
    }

    // optional uint32 entity = 3;

    pub fn clear_entity(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_entity(&self) -> bool {
        self.entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(v);
    }

    pub fn get_entity(&self) -> u32 {
        self.entity.unwrap_or(0)
    }

    fn get_entity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entity
    }

    fn mut_entity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entity
    }

    // optional uint32 hitbox = 4;

    pub fn clear_hitbox(&mut self) {
        self.hitbox = ::std::option::Option::None;
    }

    pub fn has_hitbox(&self) -> bool {
        self.hitbox.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hitbox(&mut self, v: u32) {
        self.hitbox = ::std::option::Option::Some(v);
    }

    pub fn get_hitbox(&self) -> u32 {
        self.hitbox.unwrap_or(0)
    }

    fn get_hitbox_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hitbox
    }

    fn mut_hitbox_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hitbox
    }

    // optional uint32 index = 5;

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
}

impl ::protobuf::Message for CMsgTEDecal {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.entity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hitbox = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.entity {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hitbox {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.start.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.entity {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.hitbox {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.index {
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

impl ::protobuf::MessageStatic for CMsgTEDecal {
    fn new() -> CMsgTEDecal {
        CMsgTEDecal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEDecal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEDecal::get_origin_for_reflect,
                    CMsgTEDecal::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "start",
                    CMsgTEDecal::get_start_for_reflect,
                    CMsgTEDecal::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "entity",
                    CMsgTEDecal::get_entity_for_reflect,
                    CMsgTEDecal::mut_entity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hitbox",
                    CMsgTEDecal::get_hitbox_for_reflect,
                    CMsgTEDecal::mut_hitbox_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    CMsgTEDecal::get_index_for_reflect,
                    CMsgTEDecal::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEDecal>(
                    "CMsgTEDecal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEDecal {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_start();
        self.clear_entity();
        self.clear_hitbox();
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEDecal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEDecal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgEffectData {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    start: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    normal: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    angles: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    entity: ::std::option::Option<u32>,
    otherentity: ::std::option::Option<u32>,
    scale: ::std::option::Option<f32>,
    magnitude: ::std::option::Option<f32>,
    radius: ::std::option::Option<f32>,
    surfaceprop: ::std::option::Option<u32>,
    effectindex: ::std::option::Option<u64>,
    damagetype: ::std::option::Option<u32>,
    material: ::std::option::Option<u32>,
    hitbox: ::std::option::Option<u32>,
    color: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    attachmentindex: ::std::option::Option<i32>,
    effectname: ::std::option::Option<u32>,
    attachmentname: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgEffectData {}

impl CMsgEffectData {
    pub fn new() -> CMsgEffectData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgEffectData {
        static mut instance: ::protobuf::lazy::Lazy<CMsgEffectData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgEffectData,
        };
        unsafe {
            instance.get(CMsgEffectData::new)
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

    // optional .dota.CMsgVector start = 2;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.start.is_none() {
            self.start.set_default();
        };
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> super::networkbasetypes::CMsgVector {
        self.start.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_start(&self) -> &super::networkbasetypes::CMsgVector {
        self.start.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_start_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.start
    }

    // optional .dota.CMsgVector normal = 3;

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

    // optional .dota.CMsgQAngle angles = 4;

    pub fn clear_angles(&mut self) {
        self.angles.clear();
    }

    pub fn has_angles(&self) -> bool {
        self.angles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angles(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angles = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angles(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angles.is_none() {
            self.angles.set_default();
        };
        self.angles.as_mut().unwrap()
    }

    // Take field
    pub fn take_angles(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angles.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angles(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angles.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angles_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angles
    }

    fn mut_angles_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angles
    }

    // optional fixed32 entity = 5;

    pub fn clear_entity(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_entity(&self) -> bool {
        self.entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(v);
    }

    pub fn get_entity(&self) -> u32 {
        self.entity.unwrap_or(0)
    }

    fn get_entity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entity
    }

    fn mut_entity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entity
    }

    // optional fixed32 otherentity = 6;

    pub fn clear_otherentity(&mut self) {
        self.otherentity = ::std::option::Option::None;
    }

    pub fn has_otherentity(&self) -> bool {
        self.otherentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_otherentity(&mut self, v: u32) {
        self.otherentity = ::std::option::Option::Some(v);
    }

    pub fn get_otherentity(&self) -> u32 {
        self.otherentity.unwrap_or(0)
    }

    fn get_otherentity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.otherentity
    }

    fn mut_otherentity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.otherentity
    }

    // optional float scale = 7;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: f32) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale.unwrap_or(0.)
    }

    fn get_scale_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.scale
    }

    fn mut_scale_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.scale
    }

    // optional float magnitude = 8;

    pub fn clear_magnitude(&mut self) {
        self.magnitude = ::std::option::Option::None;
    }

    pub fn has_magnitude(&self) -> bool {
        self.magnitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magnitude(&mut self, v: f32) {
        self.magnitude = ::std::option::Option::Some(v);
    }

    pub fn get_magnitude(&self) -> f32 {
        self.magnitude.unwrap_or(0.)
    }

    fn get_magnitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.magnitude
    }

    fn mut_magnitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.magnitude
    }

    // optional float radius = 9;

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

    // optional fixed32 surfaceprop = 10;

    pub fn clear_surfaceprop(&mut self) {
        self.surfaceprop = ::std::option::Option::None;
    }

    pub fn has_surfaceprop(&self) -> bool {
        self.surfaceprop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_surfaceprop(&mut self, v: u32) {
        self.surfaceprop = ::std::option::Option::Some(v);
    }

    pub fn get_surfaceprop(&self) -> u32 {
        self.surfaceprop.unwrap_or(0)
    }

    fn get_surfaceprop_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.surfaceprop
    }

    fn mut_surfaceprop_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.surfaceprop
    }

    // optional fixed64 effectindex = 11;

    pub fn clear_effectindex(&mut self) {
        self.effectindex = ::std::option::Option::None;
    }

    pub fn has_effectindex(&self) -> bool {
        self.effectindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effectindex(&mut self, v: u64) {
        self.effectindex = ::std::option::Option::Some(v);
    }

    pub fn get_effectindex(&self) -> u64 {
        self.effectindex.unwrap_or(0)
    }

    fn get_effectindex_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.effectindex
    }

    fn mut_effectindex_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.effectindex
    }

    // optional uint32 damagetype = 12;

    pub fn clear_damagetype(&mut self) {
        self.damagetype = ::std::option::Option::None;
    }

    pub fn has_damagetype(&self) -> bool {
        self.damagetype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damagetype(&mut self, v: u32) {
        self.damagetype = ::std::option::Option::Some(v);
    }

    pub fn get_damagetype(&self) -> u32 {
        self.damagetype.unwrap_or(0)
    }

    fn get_damagetype_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.damagetype
    }

    fn mut_damagetype_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.damagetype
    }

    // optional uint32 material = 13;

    pub fn clear_material(&mut self) {
        self.material = ::std::option::Option::None;
    }

    pub fn has_material(&self) -> bool {
        self.material.is_some()
    }

    // Param is passed by value, moved
    pub fn set_material(&mut self, v: u32) {
        self.material = ::std::option::Option::Some(v);
    }

    pub fn get_material(&self) -> u32 {
        self.material.unwrap_or(0)
    }

    fn get_material_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.material
    }

    fn mut_material_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.material
    }

    // optional uint32 hitbox = 14;

    pub fn clear_hitbox(&mut self) {
        self.hitbox = ::std::option::Option::None;
    }

    pub fn has_hitbox(&self) -> bool {
        self.hitbox.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hitbox(&mut self, v: u32) {
        self.hitbox = ::std::option::Option::Some(v);
    }

    pub fn get_hitbox(&self) -> u32 {
        self.hitbox.unwrap_or(0)
    }

    fn get_hitbox_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hitbox
    }

    fn mut_hitbox_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hitbox
    }

    // optional uint32 color = 15;

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

    // optional uint32 flags = 16;

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

    // optional int32 attachmentindex = 17;

    pub fn clear_attachmentindex(&mut self) {
        self.attachmentindex = ::std::option::Option::None;
    }

    pub fn has_attachmentindex(&self) -> bool {
        self.attachmentindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attachmentindex(&mut self, v: i32) {
        self.attachmentindex = ::std::option::Option::Some(v);
    }

    pub fn get_attachmentindex(&self) -> i32 {
        self.attachmentindex.unwrap_or(0)
    }

    fn get_attachmentindex_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.attachmentindex
    }

    fn mut_attachmentindex_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.attachmentindex
    }

    // optional uint32 effectname = 18;

    pub fn clear_effectname(&mut self) {
        self.effectname = ::std::option::Option::None;
    }

    pub fn has_effectname(&self) -> bool {
        self.effectname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effectname(&mut self, v: u32) {
        self.effectname = ::std::option::Option::Some(v);
    }

    pub fn get_effectname(&self) -> u32 {
        self.effectname.unwrap_or(0)
    }

    fn get_effectname_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.effectname
    }

    fn mut_effectname_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.effectname
    }

    // optional uint32 attachmentname = 19;

    pub fn clear_attachmentname(&mut self) {
        self.attachmentname = ::std::option::Option::None;
    }

    pub fn has_attachmentname(&self) -> bool {
        self.attachmentname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attachmentname(&mut self, v: u32) {
        self.attachmentname = ::std::option::Option::Some(v);
    }

    pub fn get_attachmentname(&self) -> u32 {
        self.attachmentname.unwrap_or(0)
    }

    fn get_attachmentname_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attachmentname
    }

    fn mut_attachmentname_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attachmentname
    }
}

impl ::protobuf::Message for CMsgEffectData {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.normal)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angles)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.entity = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.otherentity = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.scale = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.magnitude = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.radius = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.surfaceprop = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.effectindex = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.damagetype = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.material = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hitbox = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.color = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.attachmentindex = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.effectname = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.attachmentname = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.normal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.angles.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.entity {
            my_size += 5;
        };
        if let Some(v) = self.otherentity {
            my_size += 5;
        };
        if let Some(v) = self.scale {
            my_size += 5;
        };
        if let Some(v) = self.magnitude {
            my_size += 5;
        };
        if let Some(v) = self.radius {
            my_size += 5;
        };
        if let Some(v) = self.surfaceprop {
            my_size += 5;
        };
        if let Some(v) = self.effectindex {
            my_size += 9;
        };
        if let Some(v) = self.damagetype {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.material {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hitbox {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.color {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attachmentindex {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.effectname {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attachmentname {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.start.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.normal.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.angles.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.entity {
            os.write_fixed32(5, v)?;
        };
        if let Some(v) = self.otherentity {
            os.write_fixed32(6, v)?;
        };
        if let Some(v) = self.scale {
            os.write_float(7, v)?;
        };
        if let Some(v) = self.magnitude {
            os.write_float(8, v)?;
        };
        if let Some(v) = self.radius {
            os.write_float(9, v)?;
        };
        if let Some(v) = self.surfaceprop {
            os.write_fixed32(10, v)?;
        };
        if let Some(v) = self.effectindex {
            os.write_fixed64(11, v)?;
        };
        if let Some(v) = self.damagetype {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.material {
            os.write_uint32(13, v)?;
        };
        if let Some(v) = self.hitbox {
            os.write_uint32(14, v)?;
        };
        if let Some(v) = self.color {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(16, v)?;
        };
        if let Some(v) = self.attachmentindex {
            os.write_int32(17, v)?;
        };
        if let Some(v) = self.effectname {
            os.write_uint32(18, v)?;
        };
        if let Some(v) = self.attachmentname {
            os.write_uint32(19, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgEffectData {
    fn new() -> CMsgEffectData {
        CMsgEffectData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgEffectData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgEffectData::get_origin_for_reflect,
                    CMsgEffectData::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "start",
                    CMsgEffectData::get_start_for_reflect,
                    CMsgEffectData::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "normal",
                    CMsgEffectData::get_normal_for_reflect,
                    CMsgEffectData::mut_normal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angles",
                    CMsgEffectData::get_angles_for_reflect,
                    CMsgEffectData::mut_angles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "entity",
                    CMsgEffectData::get_entity_for_reflect,
                    CMsgEffectData::mut_entity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "otherentity",
                    CMsgEffectData::get_otherentity_for_reflect,
                    CMsgEffectData::mut_otherentity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scale",
                    CMsgEffectData::get_scale_for_reflect,
                    CMsgEffectData::mut_scale_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "magnitude",
                    CMsgEffectData::get_magnitude_for_reflect,
                    CMsgEffectData::mut_magnitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "radius",
                    CMsgEffectData::get_radius_for_reflect,
                    CMsgEffectData::mut_radius_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "surfaceprop",
                    CMsgEffectData::get_surfaceprop_for_reflect,
                    CMsgEffectData::mut_surfaceprop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "effectindex",
                    CMsgEffectData::get_effectindex_for_reflect,
                    CMsgEffectData::mut_effectindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "damagetype",
                    CMsgEffectData::get_damagetype_for_reflect,
                    CMsgEffectData::mut_damagetype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "material",
                    CMsgEffectData::get_material_for_reflect,
                    CMsgEffectData::mut_material_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hitbox",
                    CMsgEffectData::get_hitbox_for_reflect,
                    CMsgEffectData::mut_hitbox_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "color",
                    CMsgEffectData::get_color_for_reflect,
                    CMsgEffectData::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgEffectData::get_flags_for_reflect,
                    CMsgEffectData::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "attachmentindex",
                    CMsgEffectData::get_attachmentindex_for_reflect,
                    CMsgEffectData::mut_attachmentindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "effectname",
                    CMsgEffectData::get_effectname_for_reflect,
                    CMsgEffectData::mut_effectname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attachmentname",
                    CMsgEffectData::get_attachmentname_for_reflect,
                    CMsgEffectData::mut_attachmentname_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgEffectData>(
                    "CMsgEffectData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgEffectData {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_start();
        self.clear_normal();
        self.clear_angles();
        self.clear_entity();
        self.clear_otherentity();
        self.clear_scale();
        self.clear_magnitude();
        self.clear_radius();
        self.clear_surfaceprop();
        self.clear_effectindex();
        self.clear_damagetype();
        self.clear_material();
        self.clear_hitbox();
        self.clear_color();
        self.clear_flags();
        self.clear_attachmentindex();
        self.clear_effectname();
        self.clear_attachmentname();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgEffectData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgEffectData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEEffectDispatch {
    // message fields
    effectdata: ::protobuf::SingularPtrField<CMsgEffectData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEEffectDispatch {}

impl CMsgTEEffectDispatch {
    pub fn new() -> CMsgTEEffectDispatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEEffectDispatch {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEEffectDispatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEEffectDispatch,
        };
        unsafe {
            instance.get(CMsgTEEffectDispatch::new)
        }
    }

    // optional .dota.CMsgEffectData effectdata = 1;

    pub fn clear_effectdata(&mut self) {
        self.effectdata.clear();
    }

    pub fn has_effectdata(&self) -> bool {
        self.effectdata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effectdata(&mut self, v: CMsgEffectData) {
        self.effectdata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_effectdata(&mut self) -> &mut CMsgEffectData {
        if self.effectdata.is_none() {
            self.effectdata.set_default();
        };
        self.effectdata.as_mut().unwrap()
    }

    // Take field
    pub fn take_effectdata(&mut self) -> CMsgEffectData {
        self.effectdata.take().unwrap_or_else(|| CMsgEffectData::new())
    }

    pub fn get_effectdata(&self) -> &CMsgEffectData {
        self.effectdata.as_ref().unwrap_or_else(|| CMsgEffectData::default_instance())
    }

    fn get_effectdata_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgEffectData> {
        &self.effectdata
    }

    fn mut_effectdata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgEffectData> {
        &mut self.effectdata
    }
}

impl ::protobuf::Message for CMsgTEEffectDispatch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.effectdata)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.effectdata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.effectdata.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgTEEffectDispatch {
    fn new() -> CMsgTEEffectDispatch {
        CMsgTEEffectDispatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEEffectDispatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgEffectData>>(
                    "effectdata",
                    CMsgTEEffectDispatch::get_effectdata_for_reflect,
                    CMsgTEEffectDispatch::mut_effectdata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEEffectDispatch>(
                    "CMsgTEEffectDispatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEEffectDispatch {
    fn clear(&mut self) {
        self.clear_effectdata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEEffectDispatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEEffectDispatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEEnergySplash {
    // message fields
    pos: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    dir: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    explosive: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEEnergySplash {}

impl CMsgTEEnergySplash {
    pub fn new() -> CMsgTEEnergySplash {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEEnergySplash {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEEnergySplash> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEEnergySplash,
        };
        unsafe {
            instance.get(CMsgTEEnergySplash::new)
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

    // optional .dota.CMsgVector dir = 2;

    pub fn clear_dir(&mut self) {
        self.dir.clear();
    }

    pub fn has_dir(&self) -> bool {
        self.dir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.dir = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dir(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.dir.is_none() {
            self.dir.set_default();
        };
        self.dir.as_mut().unwrap()
    }

    // Take field
    pub fn take_dir(&mut self) -> super::networkbasetypes::CMsgVector {
        self.dir.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_dir(&self) -> &super::networkbasetypes::CMsgVector {
        self.dir.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_dir_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.dir
    }

    fn mut_dir_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.dir
    }

    // optional bool explosive = 3;

    pub fn clear_explosive(&mut self) {
        self.explosive = ::std::option::Option::None;
    }

    pub fn has_explosive(&self) -> bool {
        self.explosive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_explosive(&mut self, v: bool) {
        self.explosive = ::std::option::Option::Some(v);
    }

    pub fn get_explosive(&self) -> bool {
        self.explosive.unwrap_or(false)
    }

    fn get_explosive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.explosive
    }

    fn mut_explosive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.explosive
    }
}

impl ::protobuf::Message for CMsgTEEnergySplash {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dir)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.explosive = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.dir.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.explosive {
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
        if let Some(v) = self.dir.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.explosive {
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

impl ::protobuf::MessageStatic for CMsgTEEnergySplash {
    fn new() -> CMsgTEEnergySplash {
        CMsgTEEnergySplash::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEEnergySplash>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "pos",
                    CMsgTEEnergySplash::get_pos_for_reflect,
                    CMsgTEEnergySplash::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "dir",
                    CMsgTEEnergySplash::get_dir_for_reflect,
                    CMsgTEEnergySplash::mut_dir_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "explosive",
                    CMsgTEEnergySplash::get_explosive_for_reflect,
                    CMsgTEEnergySplash::mut_explosive_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEEnergySplash>(
                    "CMsgTEEnergySplash",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEEnergySplash {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_dir();
        self.clear_explosive();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEEnergySplash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEEnergySplash {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEFizz {
    // message fields
    entity: ::std::option::Option<u32>,
    density: ::std::option::Option<u32>,
    current: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEFizz {}

impl CMsgTEFizz {
    pub fn new() -> CMsgTEFizz {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEFizz {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEFizz> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEFizz,
        };
        unsafe {
            instance.get(CMsgTEFizz::new)
        }
    }

    // optional uint32 entity = 1;

    pub fn clear_entity(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_entity(&self) -> bool {
        self.entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(v);
    }

    pub fn get_entity(&self) -> u32 {
        self.entity.unwrap_or(0)
    }

    fn get_entity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entity
    }

    fn mut_entity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entity
    }

    // optional uint32 density = 2;

    pub fn clear_density(&mut self) {
        self.density = ::std::option::Option::None;
    }

    pub fn has_density(&self) -> bool {
        self.density.is_some()
    }

    // Param is passed by value, moved
    pub fn set_density(&mut self, v: u32) {
        self.density = ::std::option::Option::Some(v);
    }

    pub fn get_density(&self) -> u32 {
        self.density.unwrap_or(0)
    }

    fn get_density_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.density
    }

    fn mut_density_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.density
    }

    // optional int32 current = 3;

    pub fn clear_current(&mut self) {
        self.current = ::std::option::Option::None;
    }

    pub fn has_current(&self) -> bool {
        self.current.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current(&mut self, v: i32) {
        self.current = ::std::option::Option::Some(v);
    }

    pub fn get_current(&self) -> i32 {
        self.current.unwrap_or(0)
    }

    fn get_current_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.current
    }

    fn mut_current_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.current
    }
}

impl ::protobuf::Message for CMsgTEFizz {
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
                    self.entity = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.density = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
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
        if let Some(v) = self.entity {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.density {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.current {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entity {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.density {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.current {
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

impl ::protobuf::MessageStatic for CMsgTEFizz {
    fn new() -> CMsgTEFizz {
        CMsgTEFizz::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEFizz>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "entity",
                    CMsgTEFizz::get_entity_for_reflect,
                    CMsgTEFizz::mut_entity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "density",
                    CMsgTEFizz::get_density_for_reflect,
                    CMsgTEFizz::mut_density_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "current",
                    CMsgTEFizz::get_current_for_reflect,
                    CMsgTEFizz::mut_current_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEFizz>(
                    "CMsgTEFizz",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEFizz {
    fn clear(&mut self) {
        self.clear_entity();
        self.clear_density();
        self.clear_current();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEFizz {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEFizz {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEShatterSurface {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    angles: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    force: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    forcepos: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    width: ::std::option::Option<f32>,
    height: ::std::option::Option<f32>,
    shardsize: ::std::option::Option<f32>,
    surfacetype: ::std::option::Option<u32>,
    frontcolor: ::std::option::Option<u32>,
    backcolor: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEShatterSurface {}

impl CMsgTEShatterSurface {
    pub fn new() -> CMsgTEShatterSurface {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEShatterSurface {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEShatterSurface> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEShatterSurface,
        };
        unsafe {
            instance.get(CMsgTEShatterSurface::new)
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

    // optional .dota.CMsgQAngle angles = 2;

    pub fn clear_angles(&mut self) {
        self.angles.clear();
    }

    pub fn has_angles(&self) -> bool {
        self.angles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angles(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angles = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angles(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angles.is_none() {
            self.angles.set_default();
        };
        self.angles.as_mut().unwrap()
    }

    // Take field
    pub fn take_angles(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angles.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angles(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angles.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angles_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angles
    }

    fn mut_angles_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angles
    }

    // optional .dota.CMsgVector force = 3;

    pub fn clear_force(&mut self) {
        self.force.clear();
    }

    pub fn has_force(&self) -> bool {
        self.force.is_some()
    }

    // Param is passed by value, moved
    pub fn set_force(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.force = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_force(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.force.is_none() {
            self.force.set_default();
        };
        self.force.as_mut().unwrap()
    }

    // Take field
    pub fn take_force(&mut self) -> super::networkbasetypes::CMsgVector {
        self.force.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_force(&self) -> &super::networkbasetypes::CMsgVector {
        self.force.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_force_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.force
    }

    fn mut_force_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.force
    }

    // optional .dota.CMsgVector forcepos = 4;

    pub fn clear_forcepos(&mut self) {
        self.forcepos.clear();
    }

    pub fn has_forcepos(&self) -> bool {
        self.forcepos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forcepos(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.forcepos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_forcepos(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.forcepos.is_none() {
            self.forcepos.set_default();
        };
        self.forcepos.as_mut().unwrap()
    }

    // Take field
    pub fn take_forcepos(&mut self) -> super::networkbasetypes::CMsgVector {
        self.forcepos.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_forcepos(&self) -> &super::networkbasetypes::CMsgVector {
        self.forcepos.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_forcepos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.forcepos
    }

    fn mut_forcepos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.forcepos
    }

    // optional float width = 5;

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

    // optional float height = 6;

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

    // optional float shardsize = 7;

    pub fn clear_shardsize(&mut self) {
        self.shardsize = ::std::option::Option::None;
    }

    pub fn has_shardsize(&self) -> bool {
        self.shardsize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shardsize(&mut self, v: f32) {
        self.shardsize = ::std::option::Option::Some(v);
    }

    pub fn get_shardsize(&self) -> f32 {
        self.shardsize.unwrap_or(0.)
    }

    fn get_shardsize_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.shardsize
    }

    fn mut_shardsize_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.shardsize
    }

    // optional uint32 surfacetype = 8;

    pub fn clear_surfacetype(&mut self) {
        self.surfacetype = ::std::option::Option::None;
    }

    pub fn has_surfacetype(&self) -> bool {
        self.surfacetype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_surfacetype(&mut self, v: u32) {
        self.surfacetype = ::std::option::Option::Some(v);
    }

    pub fn get_surfacetype(&self) -> u32 {
        self.surfacetype.unwrap_or(0)
    }

    fn get_surfacetype_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.surfacetype
    }

    fn mut_surfacetype_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.surfacetype
    }

    // optional fixed32 frontcolor = 9;

    pub fn clear_frontcolor(&mut self) {
        self.frontcolor = ::std::option::Option::None;
    }

    pub fn has_frontcolor(&self) -> bool {
        self.frontcolor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frontcolor(&mut self, v: u32) {
        self.frontcolor = ::std::option::Option::Some(v);
    }

    pub fn get_frontcolor(&self) -> u32 {
        self.frontcolor.unwrap_or(0)
    }

    fn get_frontcolor_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.frontcolor
    }

    fn mut_frontcolor_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.frontcolor
    }

    // optional fixed32 backcolor = 10;

    pub fn clear_backcolor(&mut self) {
        self.backcolor = ::std::option::Option::None;
    }

    pub fn has_backcolor(&self) -> bool {
        self.backcolor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_backcolor(&mut self, v: u32) {
        self.backcolor = ::std::option::Option::Some(v);
    }

    pub fn get_backcolor(&self) -> u32 {
        self.backcolor.unwrap_or(0)
    }

    fn get_backcolor_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.backcolor
    }

    fn mut_backcolor_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.backcolor
    }
}

impl ::protobuf::Message for CMsgTEShatterSurface {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angles)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.force)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.forcepos)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.width = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.height = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.shardsize = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.surfacetype = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.frontcolor = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.backcolor = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.angles.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.force.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.forcepos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.width {
            my_size += 5;
        };
        if let Some(v) = self.height {
            my_size += 5;
        };
        if let Some(v) = self.shardsize {
            my_size += 5;
        };
        if let Some(v) = self.surfacetype {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.frontcolor {
            my_size += 5;
        };
        if let Some(v) = self.backcolor {
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
        if let Some(v) = self.angles.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.force.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.forcepos.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.width {
            os.write_float(5, v)?;
        };
        if let Some(v) = self.height {
            os.write_float(6, v)?;
        };
        if let Some(v) = self.shardsize {
            os.write_float(7, v)?;
        };
        if let Some(v) = self.surfacetype {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.frontcolor {
            os.write_fixed32(9, v)?;
        };
        if let Some(v) = self.backcolor {
            os.write_fixed32(10, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgTEShatterSurface {
    fn new() -> CMsgTEShatterSurface {
        CMsgTEShatterSurface::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEShatterSurface>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEShatterSurface::get_origin_for_reflect,
                    CMsgTEShatterSurface::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angles",
                    CMsgTEShatterSurface::get_angles_for_reflect,
                    CMsgTEShatterSurface::mut_angles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "force",
                    CMsgTEShatterSurface::get_force_for_reflect,
                    CMsgTEShatterSurface::mut_force_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "forcepos",
                    CMsgTEShatterSurface::get_forcepos_for_reflect,
                    CMsgTEShatterSurface::mut_forcepos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "width",
                    CMsgTEShatterSurface::get_width_for_reflect,
                    CMsgTEShatterSurface::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "height",
                    CMsgTEShatterSurface::get_height_for_reflect,
                    CMsgTEShatterSurface::mut_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "shardsize",
                    CMsgTEShatterSurface::get_shardsize_for_reflect,
                    CMsgTEShatterSurface::mut_shardsize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "surfacetype",
                    CMsgTEShatterSurface::get_surfacetype_for_reflect,
                    CMsgTEShatterSurface::mut_surfacetype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "frontcolor",
                    CMsgTEShatterSurface::get_frontcolor_for_reflect,
                    CMsgTEShatterSurface::mut_frontcolor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "backcolor",
                    CMsgTEShatterSurface::get_backcolor_for_reflect,
                    CMsgTEShatterSurface::mut_backcolor_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEShatterSurface>(
                    "CMsgTEShatterSurface",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEShatterSurface {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_angles();
        self.clear_force();
        self.clear_forcepos();
        self.clear_width();
        self.clear_height();
        self.clear_shardsize();
        self.clear_surfacetype();
        self.clear_frontcolor();
        self.clear_backcolor();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEShatterSurface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEShatterSurface {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEGlowSprite {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    scale: ::std::option::Option<f32>,
    life: ::std::option::Option<f32>,
    brightness: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEGlowSprite {}

impl CMsgTEGlowSprite {
    pub fn new() -> CMsgTEGlowSprite {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEGlowSprite {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEGlowSprite> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEGlowSprite,
        };
        unsafe {
            instance.get(CMsgTEGlowSprite::new)
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

    // optional float scale = 2;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: f32) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale.unwrap_or(0.)
    }

    fn get_scale_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.scale
    }

    fn mut_scale_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.scale
    }

    // optional float life = 3;

    pub fn clear_life(&mut self) {
        self.life = ::std::option::Option::None;
    }

    pub fn has_life(&self) -> bool {
        self.life.is_some()
    }

    // Param is passed by value, moved
    pub fn set_life(&mut self, v: f32) {
        self.life = ::std::option::Option::Some(v);
    }

    pub fn get_life(&self) -> f32 {
        self.life.unwrap_or(0.)
    }

    fn get_life_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.life
    }

    fn mut_life_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.life
    }

    // optional uint32 brightness = 4;

    pub fn clear_brightness(&mut self) {
        self.brightness = ::std::option::Option::None;
    }

    pub fn has_brightness(&self) -> bool {
        self.brightness.is_some()
    }

    // Param is passed by value, moved
    pub fn set_brightness(&mut self, v: u32) {
        self.brightness = ::std::option::Option::Some(v);
    }

    pub fn get_brightness(&self) -> u32 {
        self.brightness.unwrap_or(0)
    }

    fn get_brightness_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.brightness
    }

    fn mut_brightness_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.brightness
    }
}

impl ::protobuf::Message for CMsgTEGlowSprite {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.scale = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.life = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.brightness = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.scale {
            my_size += 5;
        };
        if let Some(v) = self.life {
            my_size += 5;
        };
        if let Some(v) = self.brightness {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.scale {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.life {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.brightness {
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

impl ::protobuf::MessageStatic for CMsgTEGlowSprite {
    fn new() -> CMsgTEGlowSprite {
        CMsgTEGlowSprite::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEGlowSprite>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEGlowSprite::get_origin_for_reflect,
                    CMsgTEGlowSprite::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scale",
                    CMsgTEGlowSprite::get_scale_for_reflect,
                    CMsgTEGlowSprite::mut_scale_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "life",
                    CMsgTEGlowSprite::get_life_for_reflect,
                    CMsgTEGlowSprite::mut_life_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "brightness",
                    CMsgTEGlowSprite::get_brightness_for_reflect,
                    CMsgTEGlowSprite::mut_brightness_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEGlowSprite>(
                    "CMsgTEGlowSprite",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEGlowSprite {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_scale();
        self.clear_life();
        self.clear_brightness();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEGlowSprite {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEGlowSprite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEImpact {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    normal: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    field_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEImpact {}

impl CMsgTEImpact {
    pub fn new() -> CMsgTEImpact {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEImpact {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEImpact> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEImpact,
        };
        unsafe {
            instance.get(CMsgTEImpact::new)
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

    // optional uint32 type = 3;

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
}

impl ::protobuf::Message for CMsgTEImpact {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.normal)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.normal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.normal.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.field_type {
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

impl ::protobuf::MessageStatic for CMsgTEImpact {
    fn new() -> CMsgTEImpact {
        CMsgTEImpact::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEImpact>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEImpact::get_origin_for_reflect,
                    CMsgTEImpact::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "normal",
                    CMsgTEImpact::get_normal_for_reflect,
                    CMsgTEImpact::mut_normal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "type",
                    CMsgTEImpact::get_field_type_for_reflect,
                    CMsgTEImpact::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEImpact>(
                    "CMsgTEImpact",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEImpact {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_normal();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEImpact {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEImpact {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEMuzzleFlash {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    angles: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    scale: ::std::option::Option<f32>,
    field_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEMuzzleFlash {}

impl CMsgTEMuzzleFlash {
    pub fn new() -> CMsgTEMuzzleFlash {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEMuzzleFlash {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEMuzzleFlash> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEMuzzleFlash,
        };
        unsafe {
            instance.get(CMsgTEMuzzleFlash::new)
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

    // optional .dota.CMsgQAngle angles = 2;

    pub fn clear_angles(&mut self) {
        self.angles.clear();
    }

    pub fn has_angles(&self) -> bool {
        self.angles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angles(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angles = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angles(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angles.is_none() {
            self.angles.set_default();
        };
        self.angles.as_mut().unwrap()
    }

    // Take field
    pub fn take_angles(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angles.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angles(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angles.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angles_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angles
    }

    fn mut_angles_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angles
    }

    // optional float scale = 3;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: f32) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale.unwrap_or(0.)
    }

    fn get_scale_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.scale
    }

    fn mut_scale_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.scale
    }

    // optional uint32 type = 4;

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
}

impl ::protobuf::Message for CMsgTEMuzzleFlash {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angles)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.scale = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.angles.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.scale {
            my_size += 5;
        };
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.angles.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.scale {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.field_type {
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

impl ::protobuf::MessageStatic for CMsgTEMuzzleFlash {
    fn new() -> CMsgTEMuzzleFlash {
        CMsgTEMuzzleFlash::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEMuzzleFlash>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEMuzzleFlash::get_origin_for_reflect,
                    CMsgTEMuzzleFlash::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angles",
                    CMsgTEMuzzleFlash::get_angles_for_reflect,
                    CMsgTEMuzzleFlash::mut_angles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scale",
                    CMsgTEMuzzleFlash::get_scale_for_reflect,
                    CMsgTEMuzzleFlash::mut_scale_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "type",
                    CMsgTEMuzzleFlash::get_field_type_for_reflect,
                    CMsgTEMuzzleFlash::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEMuzzleFlash>(
                    "CMsgTEMuzzleFlash",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEMuzzleFlash {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_angles();
        self.clear_scale();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEMuzzleFlash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEMuzzleFlash {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEBloodStream {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    direction: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    color: ::std::option::Option<u32>,
    amount: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEBloodStream {}

impl CMsgTEBloodStream {
    pub fn new() -> CMsgTEBloodStream {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEBloodStream {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEBloodStream> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEBloodStream,
        };
        unsafe {
            instance.get(CMsgTEBloodStream::new)
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

    // optional fixed32 color = 3;

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

    // optional uint32 amount = 4;

    pub fn clear_amount(&mut self) {
        self.amount = ::std::option::Option::None;
    }

    pub fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: u32) {
        self.amount = ::std::option::Option::Some(v);
    }

    pub fn get_amount(&self) -> u32 {
        self.amount.unwrap_or(0)
    }

    fn get_amount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.amount
    }
}

impl ::protobuf::Message for CMsgTEBloodStream {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.direction)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.color = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.amount = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.direction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.color {
            my_size += 5;
        };
        if let Some(v) = self.amount {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.direction.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.color {
            os.write_fixed32(3, v)?;
        };
        if let Some(v) = self.amount {
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

impl ::protobuf::MessageStatic for CMsgTEBloodStream {
    fn new() -> CMsgTEBloodStream {
        CMsgTEBloodStream::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEBloodStream>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEBloodStream::get_origin_for_reflect,
                    CMsgTEBloodStream::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "direction",
                    CMsgTEBloodStream::get_direction_for_reflect,
                    CMsgTEBloodStream::mut_direction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "color",
                    CMsgTEBloodStream::get_color_for_reflect,
                    CMsgTEBloodStream::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "amount",
                    CMsgTEBloodStream::get_amount_for_reflect,
                    CMsgTEBloodStream::mut_amount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEBloodStream>(
                    "CMsgTEBloodStream",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEBloodStream {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_direction();
        self.clear_color();
        self.clear_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEBloodStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEBloodStream {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEExplosion {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    framerate: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    normal: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    materialtype: ::std::option::Option<u32>,
    radius: ::std::option::Option<u32>,
    magnitude: ::std::option::Option<u32>,
    scale: ::std::option::Option<f32>,
    affect_ragdolls: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEExplosion {}

impl CMsgTEExplosion {
    pub fn new() -> CMsgTEExplosion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEExplosion {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEExplosion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEExplosion,
        };
        unsafe {
            instance.get(CMsgTEExplosion::new)
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

    // optional uint32 framerate = 2;

    pub fn clear_framerate(&mut self) {
        self.framerate = ::std::option::Option::None;
    }

    pub fn has_framerate(&self) -> bool {
        self.framerate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framerate(&mut self, v: u32) {
        self.framerate = ::std::option::Option::Some(v);
    }

    pub fn get_framerate(&self) -> u32 {
        self.framerate.unwrap_or(0)
    }

    fn get_framerate_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.framerate
    }

    fn mut_framerate_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.framerate
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

    // optional .dota.CMsgVector normal = 4;

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

    // optional uint32 materialtype = 5;

    pub fn clear_materialtype(&mut self) {
        self.materialtype = ::std::option::Option::None;
    }

    pub fn has_materialtype(&self) -> bool {
        self.materialtype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_materialtype(&mut self, v: u32) {
        self.materialtype = ::std::option::Option::Some(v);
    }

    pub fn get_materialtype(&self) -> u32 {
        self.materialtype.unwrap_or(0)
    }

    fn get_materialtype_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.materialtype
    }

    fn mut_materialtype_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.materialtype
    }

    // optional uint32 radius = 6;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: u32) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius(&self) -> u32 {
        self.radius.unwrap_or(0)
    }

    fn get_radius_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.radius
    }

    fn mut_radius_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.radius
    }

    // optional uint32 magnitude = 7;

    pub fn clear_magnitude(&mut self) {
        self.magnitude = ::std::option::Option::None;
    }

    pub fn has_magnitude(&self) -> bool {
        self.magnitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magnitude(&mut self, v: u32) {
        self.magnitude = ::std::option::Option::Some(v);
    }

    pub fn get_magnitude(&self) -> u32 {
        self.magnitude.unwrap_or(0)
    }

    fn get_magnitude_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.magnitude
    }

    fn mut_magnitude_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.magnitude
    }

    // optional float scale = 8;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: f32) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale.unwrap_or(0.)
    }

    fn get_scale_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.scale
    }

    fn mut_scale_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.scale
    }

    // optional bool affect_ragdolls = 9;

    pub fn clear_affect_ragdolls(&mut self) {
        self.affect_ragdolls = ::std::option::Option::None;
    }

    pub fn has_affect_ragdolls(&self) -> bool {
        self.affect_ragdolls.is_some()
    }

    // Param is passed by value, moved
    pub fn set_affect_ragdolls(&mut self, v: bool) {
        self.affect_ragdolls = ::std::option::Option::Some(v);
    }

    pub fn get_affect_ragdolls(&self) -> bool {
        self.affect_ragdolls.unwrap_or(false)
    }

    fn get_affect_ragdolls_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.affect_ragdolls
    }

    fn mut_affect_ragdolls_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.affect_ragdolls
    }
}

impl ::protobuf::Message for CMsgTEExplosion {
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
                    self.framerate = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.normal)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.materialtype = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.radius = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.magnitude = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.scale = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.affect_ragdolls = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.framerate {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.normal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.materialtype {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radius {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.magnitude {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.scale {
            my_size += 5;
        };
        if let Some(v) = self.affect_ragdolls {
            my_size += 2;
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
        if let Some(v) = self.framerate {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.normal.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.materialtype {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.radius {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.magnitude {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.scale {
            os.write_float(8, v)?;
        };
        if let Some(v) = self.affect_ragdolls {
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

impl ::protobuf::MessageStatic for CMsgTEExplosion {
    fn new() -> CMsgTEExplosion {
        CMsgTEExplosion::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEExplosion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEExplosion::get_origin_for_reflect,
                    CMsgTEExplosion::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "framerate",
                    CMsgTEExplosion::get_framerate_for_reflect,
                    CMsgTEExplosion::mut_framerate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgTEExplosion::get_flags_for_reflect,
                    CMsgTEExplosion::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "normal",
                    CMsgTEExplosion::get_normal_for_reflect,
                    CMsgTEExplosion::mut_normal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "materialtype",
                    CMsgTEExplosion::get_materialtype_for_reflect,
                    CMsgTEExplosion::mut_materialtype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "radius",
                    CMsgTEExplosion::get_radius_for_reflect,
                    CMsgTEExplosion::mut_radius_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "magnitude",
                    CMsgTEExplosion::get_magnitude_for_reflect,
                    CMsgTEExplosion::mut_magnitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scale",
                    CMsgTEExplosion::get_scale_for_reflect,
                    CMsgTEExplosion::mut_scale_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "affect_ragdolls",
                    CMsgTEExplosion::get_affect_ragdolls_for_reflect,
                    CMsgTEExplosion::mut_affect_ragdolls_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEExplosion>(
                    "CMsgTEExplosion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEExplosion {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_framerate();
        self.clear_flags();
        self.clear_normal();
        self.clear_materialtype();
        self.clear_radius();
        self.clear_magnitude();
        self.clear_scale();
        self.clear_affect_ragdolls();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEExplosion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEExplosion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEDust {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    size: ::std::option::Option<f32>,
    speed: ::std::option::Option<f32>,
    direction: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEDust {}

impl CMsgTEDust {
    pub fn new() -> CMsgTEDust {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEDust {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEDust> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEDust,
        };
        unsafe {
            instance.get(CMsgTEDust::new)
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

    // optional float size = 2;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: f32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> f32 {
        self.size.unwrap_or(0.)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.size
    }

    // optional float speed = 3;

    pub fn clear_speed(&mut self) {
        self.speed = ::std::option::Option::None;
    }

    pub fn has_speed(&self) -> bool {
        self.speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speed(&mut self, v: f32) {
        self.speed = ::std::option::Option::Some(v);
    }

    pub fn get_speed(&self) -> f32 {
        self.speed.unwrap_or(0.)
    }

    fn get_speed_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.speed
    }

    fn mut_speed_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.speed
    }

    // optional .dota.CMsgVector direction = 4;

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

impl ::protobuf::Message for CMsgTEDust {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.speed = ::std::option::Option::Some(tmp);
                },
                4 => {
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
        if let Some(v) = self.origin.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.size {
            my_size += 5;
        };
        if let Some(v) = self.speed {
            my_size += 5;
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
        if let Some(v) = self.origin.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.size {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.speed {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.direction.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgTEDust {
    fn new() -> CMsgTEDust {
        CMsgTEDust::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEDust>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEDust::get_origin_for_reflect,
                    CMsgTEDust::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "size",
                    CMsgTEDust::get_size_for_reflect,
                    CMsgTEDust::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "speed",
                    CMsgTEDust::get_speed_for_reflect,
                    CMsgTEDust::mut_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "direction",
                    CMsgTEDust::get_direction_for_reflect,
                    CMsgTEDust::mut_direction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEDust>(
                    "CMsgTEDust",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEDust {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_size();
        self.clear_speed();
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEDust {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEDust {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTELargeFunnel {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    reversed: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTELargeFunnel {}

impl CMsgTELargeFunnel {
    pub fn new() -> CMsgTELargeFunnel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTELargeFunnel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTELargeFunnel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTELargeFunnel,
        };
        unsafe {
            instance.get(CMsgTELargeFunnel::new)
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

    // optional uint32 reversed = 2;

    pub fn clear_reversed(&mut self) {
        self.reversed = ::std::option::Option::None;
    }

    pub fn has_reversed(&self) -> bool {
        self.reversed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reversed(&mut self, v: u32) {
        self.reversed = ::std::option::Option::Some(v);
    }

    pub fn get_reversed(&self) -> u32 {
        self.reversed.unwrap_or(0)
    }

    fn get_reversed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.reversed
    }

    fn mut_reversed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.reversed
    }
}

impl ::protobuf::Message for CMsgTELargeFunnel {
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
                    self.reversed = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.reversed {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.reversed {
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

impl ::protobuf::MessageStatic for CMsgTELargeFunnel {
    fn new() -> CMsgTELargeFunnel {
        CMsgTELargeFunnel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTELargeFunnel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTELargeFunnel::get_origin_for_reflect,
                    CMsgTELargeFunnel::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "reversed",
                    CMsgTELargeFunnel::get_reversed_for_reflect,
                    CMsgTELargeFunnel::mut_reversed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTELargeFunnel>(
                    "CMsgTELargeFunnel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTELargeFunnel {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_reversed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTELargeFunnel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTELargeFunnel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTESparks {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    magnitude: ::std::option::Option<u32>,
    length: ::std::option::Option<u32>,
    direction: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTESparks {}

impl CMsgTESparks {
    pub fn new() -> CMsgTESparks {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTESparks {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTESparks> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTESparks,
        };
        unsafe {
            instance.get(CMsgTESparks::new)
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

    // optional uint32 magnitude = 2;

    pub fn clear_magnitude(&mut self) {
        self.magnitude = ::std::option::Option::None;
    }

    pub fn has_magnitude(&self) -> bool {
        self.magnitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magnitude(&mut self, v: u32) {
        self.magnitude = ::std::option::Option::Some(v);
    }

    pub fn get_magnitude(&self) -> u32 {
        self.magnitude.unwrap_or(0)
    }

    fn get_magnitude_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.magnitude
    }

    fn mut_magnitude_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.magnitude
    }

    // optional uint32 length = 3;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u32) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u32 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.length
    }

    // optional .dota.CMsgVector direction = 4;

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

impl ::protobuf::Message for CMsgTESparks {
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
                    self.magnitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                4 => {
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
        if let Some(v) = self.origin.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.magnitude {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.origin.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.magnitude {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.length {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.direction.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgTESparks {
    fn new() -> CMsgTESparks {
        CMsgTESparks::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTESparks>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTESparks::get_origin_for_reflect,
                    CMsgTESparks::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "magnitude",
                    CMsgTESparks::get_magnitude_for_reflect,
                    CMsgTESparks::mut_magnitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "length",
                    CMsgTESparks::get_length_for_reflect,
                    CMsgTESparks::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "direction",
                    CMsgTESparks::get_direction_for_reflect,
                    CMsgTESparks::mut_direction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTESparks>(
                    "CMsgTESparks",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTESparks {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_magnitude();
        self.clear_length();
        self.clear_direction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTESparks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTESparks {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEPhysicsProp {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    velocity: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    angles: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    skin: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    effects: ::std::option::Option<u32>,
    color: ::std::option::Option<u32>,
    modelindex: ::std::option::Option<u64>,
    breakmodelsnottomake: ::std::option::Option<u32>,
    scale: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEPhysicsProp {}

impl CMsgTEPhysicsProp {
    pub fn new() -> CMsgTEPhysicsProp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEPhysicsProp {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEPhysicsProp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEPhysicsProp,
        };
        unsafe {
            instance.get(CMsgTEPhysicsProp::new)
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

    // optional .dota.CMsgVector velocity = 2;

    pub fn clear_velocity(&mut self) {
        self.velocity.clear();
    }

    pub fn has_velocity(&self) -> bool {
        self.velocity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_velocity(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.velocity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_velocity(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.velocity.is_none() {
            self.velocity.set_default();
        };
        self.velocity.as_mut().unwrap()
    }

    // Take field
    pub fn take_velocity(&mut self) -> super::networkbasetypes::CMsgVector {
        self.velocity.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_velocity(&self) -> &super::networkbasetypes::CMsgVector {
        self.velocity.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_velocity_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.velocity
    }

    fn mut_velocity_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.velocity
    }

    // optional .dota.CMsgQAngle angles = 3;

    pub fn clear_angles(&mut self) {
        self.angles.clear();
    }

    pub fn has_angles(&self) -> bool {
        self.angles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angles(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angles = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angles(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angles.is_none() {
            self.angles.set_default();
        };
        self.angles.as_mut().unwrap()
    }

    // Take field
    pub fn take_angles(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angles.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angles(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angles.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angles_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angles
    }

    fn mut_angles_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angles
    }

    // optional fixed32 skin = 4;

    pub fn clear_skin(&mut self) {
        self.skin = ::std::option::Option::None;
    }

    pub fn has_skin(&self) -> bool {
        self.skin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_skin(&mut self, v: u32) {
        self.skin = ::std::option::Option::Some(v);
    }

    pub fn get_skin(&self) -> u32 {
        self.skin.unwrap_or(0)
    }

    fn get_skin_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.skin
    }

    fn mut_skin_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.skin
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

    // optional uint32 effects = 6;

    pub fn clear_effects(&mut self) {
        self.effects = ::std::option::Option::None;
    }

    pub fn has_effects(&self) -> bool {
        self.effects.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effects(&mut self, v: u32) {
        self.effects = ::std::option::Option::Some(v);
    }

    pub fn get_effects(&self) -> u32 {
        self.effects.unwrap_or(0)
    }

    fn get_effects_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.effects
    }

    fn mut_effects_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.effects
    }

    // optional fixed32 color = 7;

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

    // optional fixed64 modelindex = 8;

    pub fn clear_modelindex(&mut self) {
        self.modelindex = ::std::option::Option::None;
    }

    pub fn has_modelindex(&self) -> bool {
        self.modelindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modelindex(&mut self, v: u64) {
        self.modelindex = ::std::option::Option::Some(v);
    }

    pub fn get_modelindex(&self) -> u64 {
        self.modelindex.unwrap_or(0)
    }

    fn get_modelindex_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.modelindex
    }

    fn mut_modelindex_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.modelindex
    }

    // optional uint32 breakmodelsnottomake = 9;

    pub fn clear_breakmodelsnottomake(&mut self) {
        self.breakmodelsnottomake = ::std::option::Option::None;
    }

    pub fn has_breakmodelsnottomake(&self) -> bool {
        self.breakmodelsnottomake.is_some()
    }

    // Param is passed by value, moved
    pub fn set_breakmodelsnottomake(&mut self, v: u32) {
        self.breakmodelsnottomake = ::std::option::Option::Some(v);
    }

    pub fn get_breakmodelsnottomake(&self) -> u32 {
        self.breakmodelsnottomake.unwrap_or(0)
    }

    fn get_breakmodelsnottomake_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.breakmodelsnottomake
    }

    fn mut_breakmodelsnottomake_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.breakmodelsnottomake
    }

    // optional float scale = 10;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: f32) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale.unwrap_or(0.)
    }

    fn get_scale_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.scale
    }

    fn mut_scale_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.scale
    }
}

impl ::protobuf::Message for CMsgTEPhysicsProp {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.velocity)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angles)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.skin = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.effects = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.color = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.modelindex = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.breakmodelsnottomake = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.scale = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.velocity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.angles.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.skin {
            my_size += 5;
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.effects {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.color {
            my_size += 5;
        };
        if let Some(v) = self.modelindex {
            my_size += 9;
        };
        if let Some(v) = self.breakmodelsnottomake {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.scale {
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
        if let Some(v) = self.velocity.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.angles.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.skin {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.effects {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.color {
            os.write_fixed32(7, v)?;
        };
        if let Some(v) = self.modelindex {
            os.write_fixed64(8, v)?;
        };
        if let Some(v) = self.breakmodelsnottomake {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.scale {
            os.write_float(10, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgTEPhysicsProp {
    fn new() -> CMsgTEPhysicsProp {
        CMsgTEPhysicsProp::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEPhysicsProp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEPhysicsProp::get_origin_for_reflect,
                    CMsgTEPhysicsProp::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "velocity",
                    CMsgTEPhysicsProp::get_velocity_for_reflect,
                    CMsgTEPhysicsProp::mut_velocity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angles",
                    CMsgTEPhysicsProp::get_angles_for_reflect,
                    CMsgTEPhysicsProp::mut_angles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "skin",
                    CMsgTEPhysicsProp::get_skin_for_reflect,
                    CMsgTEPhysicsProp::mut_skin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CMsgTEPhysicsProp::get_flags_for_reflect,
                    CMsgTEPhysicsProp::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "effects",
                    CMsgTEPhysicsProp::get_effects_for_reflect,
                    CMsgTEPhysicsProp::mut_effects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "color",
                    CMsgTEPhysicsProp::get_color_for_reflect,
                    CMsgTEPhysicsProp::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "modelindex",
                    CMsgTEPhysicsProp::get_modelindex_for_reflect,
                    CMsgTEPhysicsProp::mut_modelindex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "breakmodelsnottomake",
                    CMsgTEPhysicsProp::get_breakmodelsnottomake_for_reflect,
                    CMsgTEPhysicsProp::mut_breakmodelsnottomake_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scale",
                    CMsgTEPhysicsProp::get_scale_for_reflect,
                    CMsgTEPhysicsProp::mut_scale_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEPhysicsProp>(
                    "CMsgTEPhysicsProp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEPhysicsProp {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_velocity();
        self.clear_angles();
        self.clear_skin();
        self.clear_flags();
        self.clear_effects();
        self.clear_color();
        self.clear_modelindex();
        self.clear_breakmodelsnottomake();
        self.clear_scale();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEPhysicsProp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEPhysicsProp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEPlayerDecal {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    player: ::std::option::Option<u32>,
    entity: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEPlayerDecal {}

impl CMsgTEPlayerDecal {
    pub fn new() -> CMsgTEPlayerDecal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEPlayerDecal {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEPlayerDecal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEPlayerDecal,
        };
        unsafe {
            instance.get(CMsgTEPlayerDecal::new)
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

    // optional uint32 player = 2;

    pub fn clear_player(&mut self) {
        self.player = ::std::option::Option::None;
    }

    pub fn has_player(&self) -> bool {
        self.player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player(&mut self, v: u32) {
        self.player = ::std::option::Option::Some(v);
    }

    pub fn get_player(&self) -> u32 {
        self.player.unwrap_or(0)
    }

    fn get_player_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player
    }

    fn mut_player_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player
    }

    // optional uint32 entity = 3;

    pub fn clear_entity(&mut self) {
        self.entity = ::std::option::Option::None;
    }

    pub fn has_entity(&self) -> bool {
        self.entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity(&mut self, v: u32) {
        self.entity = ::std::option::Option::Some(v);
    }

    pub fn get_entity(&self) -> u32 {
        self.entity.unwrap_or(0)
    }

    fn get_entity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.entity
    }

    fn mut_entity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.entity
    }
}

impl ::protobuf::Message for CMsgTEPlayerDecal {
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
                    self.player = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.entity = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.player {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.entity {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.player {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.entity {
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

impl ::protobuf::MessageStatic for CMsgTEPlayerDecal {
    fn new() -> CMsgTEPlayerDecal {
        CMsgTEPlayerDecal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEPlayerDecal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEPlayerDecal::get_origin_for_reflect,
                    CMsgTEPlayerDecal::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player",
                    CMsgTEPlayerDecal::get_player_for_reflect,
                    CMsgTEPlayerDecal::mut_player_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "entity",
                    CMsgTEPlayerDecal::get_entity_for_reflect,
                    CMsgTEPlayerDecal::mut_entity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEPlayerDecal>(
                    "CMsgTEPlayerDecal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEPlayerDecal {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_player();
        self.clear_entity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEPlayerDecal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEPlayerDecal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEProjectedDecal {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    angles: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle>,
    index: ::std::option::Option<u32>,
    distance: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEProjectedDecal {}

impl CMsgTEProjectedDecal {
    pub fn new() -> CMsgTEProjectedDecal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEProjectedDecal {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEProjectedDecal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEProjectedDecal,
        };
        unsafe {
            instance.get(CMsgTEProjectedDecal::new)
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

    // optional .dota.CMsgQAngle angles = 2;

    pub fn clear_angles(&mut self) {
        self.angles.clear();
    }

    pub fn has_angles(&self) -> bool {
        self.angles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angles(&mut self, v: super::networkbasetypes::CMsgQAngle) {
        self.angles = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angles(&mut self) -> &mut super::networkbasetypes::CMsgQAngle {
        if self.angles.is_none() {
            self.angles.set_default();
        };
        self.angles.as_mut().unwrap()
    }

    // Take field
    pub fn take_angles(&mut self) -> super::networkbasetypes::CMsgQAngle {
        self.angles.take().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::new())
    }

    pub fn get_angles(&self) -> &super::networkbasetypes::CMsgQAngle {
        self.angles.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgQAngle::default_instance())
    }

    fn get_angles_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &self.angles
    }

    fn mut_angles_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgQAngle> {
        &mut self.angles
    }

    // optional uint32 index = 3;

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

    // optional float distance = 4;

    pub fn clear_distance(&mut self) {
        self.distance = ::std::option::Option::None;
    }

    pub fn has_distance(&self) -> bool {
        self.distance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distance(&mut self, v: f32) {
        self.distance = ::std::option::Option::Some(v);
    }

    pub fn get_distance(&self) -> f32 {
        self.distance.unwrap_or(0.)
    }

    fn get_distance_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.distance
    }

    fn mut_distance_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.distance
    }
}

impl ::protobuf::Message for CMsgTEProjectedDecal {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angles)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.distance = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.angles.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.distance {
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
        if let Some(v) = self.angles.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.index {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.distance {
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

impl ::protobuf::MessageStatic for CMsgTEProjectedDecal {
    fn new() -> CMsgTEProjectedDecal {
        CMsgTEProjectedDecal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEProjectedDecal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEProjectedDecal::get_origin_for_reflect,
                    CMsgTEProjectedDecal::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgQAngle>>(
                    "angles",
                    CMsgTEProjectedDecal::get_angles_for_reflect,
                    CMsgTEProjectedDecal::mut_angles_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    CMsgTEProjectedDecal::get_index_for_reflect,
                    CMsgTEProjectedDecal::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "distance",
                    CMsgTEProjectedDecal::get_distance_for_reflect,
                    CMsgTEProjectedDecal::mut_distance_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEProjectedDecal>(
                    "CMsgTEProjectedDecal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEProjectedDecal {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_angles();
        self.clear_index();
        self.clear_distance();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEProjectedDecal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEProjectedDecal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTESmoke {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    scale: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTESmoke {}

impl CMsgTESmoke {
    pub fn new() -> CMsgTESmoke {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTESmoke {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTESmoke> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTESmoke,
        };
        unsafe {
            instance.get(CMsgTESmoke::new)
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

    // optional float scale = 2;

    pub fn clear_scale(&mut self) {
        self.scale = ::std::option::Option::None;
    }

    pub fn has_scale(&self) -> bool {
        self.scale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scale(&mut self, v: f32) {
        self.scale = ::std::option::Option::Some(v);
    }

    pub fn get_scale(&self) -> f32 {
        self.scale.unwrap_or(0.)
    }

    fn get_scale_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.scale
    }

    fn mut_scale_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.scale
    }
}

impl ::protobuf::Message for CMsgTESmoke {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.scale = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.scale {
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
        if let Some(v) = self.scale {
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

impl ::protobuf::MessageStatic for CMsgTESmoke {
    fn new() -> CMsgTESmoke {
        CMsgTESmoke::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTESmoke>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTESmoke::get_origin_for_reflect,
                    CMsgTESmoke::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "scale",
                    CMsgTESmoke::get_scale_for_reflect,
                    CMsgTESmoke::mut_scale_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTESmoke>(
                    "CMsgTESmoke",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTESmoke {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_scale();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTESmoke {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTESmoke {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgTEWorldDecal {
    // message fields
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    normal: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgTEWorldDecal {}

impl CMsgTEWorldDecal {
    pub fn new() -> CMsgTEWorldDecal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgTEWorldDecal {
        static mut instance: ::protobuf::lazy::Lazy<CMsgTEWorldDecal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgTEWorldDecal,
        };
        unsafe {
            instance.get(CMsgTEWorldDecal::new)
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

    // optional uint32 index = 3;

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
}

impl ::protobuf::Message for CMsgTEWorldDecal {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.normal)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.normal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.normal.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.index {
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

impl ::protobuf::MessageStatic for CMsgTEWorldDecal {
    fn new() -> CMsgTEWorldDecal {
        CMsgTEWorldDecal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgTEWorldDecal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    CMsgTEWorldDecal::get_origin_for_reflect,
                    CMsgTEWorldDecal::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "normal",
                    CMsgTEWorldDecal::get_normal_for_reflect,
                    CMsgTEWorldDecal::mut_normal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    CMsgTEWorldDecal::get_index_for_reflect,
                    CMsgTEWorldDecal::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgTEWorldDecal>(
                    "CMsgTEWorldDecal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgTEWorldDecal {
    fn clear(&mut self) {
        self.clear_origin();
        self.clear_normal();
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgTEWorldDecal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgTEWorldDecal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ETEProtobufIds {
    TE_EffectDispatchId = 400,
    TE_ArmorRicochetId = 401,
    TE_BeamEntPointId = 402,
    TE_BeamEntsId = 403,
    TE_BeamPointsId = 404,
    TE_BeamRingId = 405,
    TE_BreakModelId = 406,
    TE_BSPDecalId = 407,
    TE_BubblesId = 408,
    TE_BubbleTrailId = 409,
    TE_DecalId = 410,
    TE_WorldDecalId = 411,
    TE_EnergySplashId = 412,
    TE_FizzId = 413,
    TE_ShatterSurfaceId = 414,
    TE_GlowSpriteId = 415,
    TE_ImpactId = 416,
    TE_MuzzleFlashId = 417,
    TE_BloodStreamId = 418,
    TE_ExplosionId = 419,
    TE_DustId = 420,
    TE_LargeFunnelId = 421,
    TE_SparksId = 422,
    TE_PhysicsPropId = 423,
    TE_PlayerDecalId = 424,
    TE_ProjectedDecalId = 425,
    TE_SmokeId = 426,
}

impl ::protobuf::ProtobufEnum for ETEProtobufIds {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETEProtobufIds> {
        match value {
            400 => ::std::option::Option::Some(ETEProtobufIds::TE_EffectDispatchId),
            401 => ::std::option::Option::Some(ETEProtobufIds::TE_ArmorRicochetId),
            402 => ::std::option::Option::Some(ETEProtobufIds::TE_BeamEntPointId),
            403 => ::std::option::Option::Some(ETEProtobufIds::TE_BeamEntsId),
            404 => ::std::option::Option::Some(ETEProtobufIds::TE_BeamPointsId),
            405 => ::std::option::Option::Some(ETEProtobufIds::TE_BeamRingId),
            406 => ::std::option::Option::Some(ETEProtobufIds::TE_BreakModelId),
            407 => ::std::option::Option::Some(ETEProtobufIds::TE_BSPDecalId),
            408 => ::std::option::Option::Some(ETEProtobufIds::TE_BubblesId),
            409 => ::std::option::Option::Some(ETEProtobufIds::TE_BubbleTrailId),
            410 => ::std::option::Option::Some(ETEProtobufIds::TE_DecalId),
            411 => ::std::option::Option::Some(ETEProtobufIds::TE_WorldDecalId),
            412 => ::std::option::Option::Some(ETEProtobufIds::TE_EnergySplashId),
            413 => ::std::option::Option::Some(ETEProtobufIds::TE_FizzId),
            414 => ::std::option::Option::Some(ETEProtobufIds::TE_ShatterSurfaceId),
            415 => ::std::option::Option::Some(ETEProtobufIds::TE_GlowSpriteId),
            416 => ::std::option::Option::Some(ETEProtobufIds::TE_ImpactId),
            417 => ::std::option::Option::Some(ETEProtobufIds::TE_MuzzleFlashId),
            418 => ::std::option::Option::Some(ETEProtobufIds::TE_BloodStreamId),
            419 => ::std::option::Option::Some(ETEProtobufIds::TE_ExplosionId),
            420 => ::std::option::Option::Some(ETEProtobufIds::TE_DustId),
            421 => ::std::option::Option::Some(ETEProtobufIds::TE_LargeFunnelId),
            422 => ::std::option::Option::Some(ETEProtobufIds::TE_SparksId),
            423 => ::std::option::Option::Some(ETEProtobufIds::TE_PhysicsPropId),
            424 => ::std::option::Option::Some(ETEProtobufIds::TE_PlayerDecalId),
            425 => ::std::option::Option::Some(ETEProtobufIds::TE_ProjectedDecalId),
            426 => ::std::option::Option::Some(ETEProtobufIds::TE_SmokeId),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETEProtobufIds] = &[
            ETEProtobufIds::TE_EffectDispatchId,
            ETEProtobufIds::TE_ArmorRicochetId,
            ETEProtobufIds::TE_BeamEntPointId,
            ETEProtobufIds::TE_BeamEntsId,
            ETEProtobufIds::TE_BeamPointsId,
            ETEProtobufIds::TE_BeamRingId,
            ETEProtobufIds::TE_BreakModelId,
            ETEProtobufIds::TE_BSPDecalId,
            ETEProtobufIds::TE_BubblesId,
            ETEProtobufIds::TE_BubbleTrailId,
            ETEProtobufIds::TE_DecalId,
            ETEProtobufIds::TE_WorldDecalId,
            ETEProtobufIds::TE_EnergySplashId,
            ETEProtobufIds::TE_FizzId,
            ETEProtobufIds::TE_ShatterSurfaceId,
            ETEProtobufIds::TE_GlowSpriteId,
            ETEProtobufIds::TE_ImpactId,
            ETEProtobufIds::TE_MuzzleFlashId,
            ETEProtobufIds::TE_BloodStreamId,
            ETEProtobufIds::TE_ExplosionId,
            ETEProtobufIds::TE_DustId,
            ETEProtobufIds::TE_LargeFunnelId,
            ETEProtobufIds::TE_SparksId,
            ETEProtobufIds::TE_PhysicsPropId,
            ETEProtobufIds::TE_PlayerDecalId,
            ETEProtobufIds::TE_ProjectedDecalId,
            ETEProtobufIds::TE_SmokeId,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ETEProtobufIds>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETEProtobufIds", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETEProtobufIds {
}

impl ::protobuf::reflect::ProtobufValue for ETEProtobufIds {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x08, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61,
    0x1a, 0x16, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x62, 0x61, 0x73, 0x65, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5d, 0x0a, 0x13, 0x43, 0x4d, 0x73, 0x67,
    0x54, 0x45, 0x41, 0x72, 0x6d, 0x6f, 0x72, 0x52, 0x69, 0x63, 0x6f, 0x63, 0x68, 0x65, 0x74, 0x12,
    0x22, 0x0a, 0x03, 0x70, 0x6f, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x03,
    0x70, 0x6f, 0x73, 0x12, 0x22, 0x0a, 0x03, 0x64, 0x69, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x52, 0x03, 0x64, 0x69, 0x72, 0x22, 0xd2, 0x02, 0x0a, 0x0e, 0x43, 0x4d, 0x73, 0x67,
    0x54, 0x45, 0x42, 0x61, 0x73, 0x65, 0x42, 0x65, 0x61, 0x6d, 0x12, 0x1e, 0x0a, 0x0a, 0x6d, 0x6f,
    0x64, 0x65, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0a,
    0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1c, 0x0a, 0x09, 0x68, 0x61,
    0x6c, 0x6f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x68,
    0x61, 0x6c, 0x6f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1e, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x66, 0x72, 0x61, 0x6d,
    0x65, 0x72, 0x61, 0x74, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x66, 0x72, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x74, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x69, 0x66, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x04, 0x6c, 0x69, 0x66, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x77, 0x69,
    0x64, 0x74, 0x68, 0x18, 0x06, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68,
    0x12, 0x1a, 0x0a, 0x08, 0x65, 0x6e, 0x64, 0x77, 0x69, 0x64, 0x74, 0x68, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x08, 0x65, 0x6e, 0x64, 0x77, 0x69, 0x64, 0x74, 0x68, 0x12, 0x1e, 0x0a, 0x0a,
    0x66, 0x61, 0x64, 0x65, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0a, 0x66, 0x61, 0x64, 0x65, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x12, 0x1c, 0x0a, 0x09,
    0x61, 0x6d, 0x70, 0x6c, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x09, 0x61, 0x6d, 0x70, 0x6c, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f,
    0x6c, 0x6f, 0x72, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x07, 0x52, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72,
    0x12, 0x14, 0x0a, 0x05, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x05, 0x73, 0x70, 0x65, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18,
    0x0c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x22, 0xca, 0x01, 0x0a,
    0x12, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x65, 0x61, 0x6d, 0x45, 0x6e, 0x74, 0x50, 0x6f,
    0x69, 0x6e, 0x74, 0x12, 0x28, 0x0a, 0x04, 0x62, 0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42,
    0x61, 0x73, 0x65, 0x42, 0x65, 0x61, 0x6d, 0x52, 0x04, 0x62, 0x61, 0x73, 0x65, 0x12, 0x20, 0x0a,
    0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12,
    0x1c, 0x0a, 0x09, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x09, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x26, 0x0a,
    0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x05,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x12, 0x22, 0x0a, 0x03, 0x65, 0x6e, 0x64, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x03, 0x65, 0x6e, 0x64, 0x22, 0x7a, 0x0a, 0x0e, 0x43, 0x4d, 0x73,
    0x67, 0x54, 0x45, 0x42, 0x65, 0x61, 0x6d, 0x45, 0x6e, 0x74, 0x73, 0x12, 0x28, 0x0a, 0x04, 0x62,
    0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x61, 0x73, 0x65, 0x42, 0x65, 0x61, 0x6d, 0x52,
    0x04, 0x62, 0x61, 0x73, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x1c, 0x0a, 0x09, 0x65, 0x6e, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x65, 0x6e, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x22, 0x88, 0x01, 0x0a, 0x10, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45,
    0x42, 0x65, 0x61, 0x6d, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x12, 0x28, 0x0a, 0x04, 0x62, 0x61,
    0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x61, 0x73, 0x65, 0x42, 0x65, 0x61, 0x6d, 0x52, 0x04,
    0x62, 0x61, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x12, 0x22, 0x0a, 0x03,
    0x65, 0x6e, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x03, 0x65, 0x6e, 0x64,
    0x22, 0x7a, 0x0a, 0x0e, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x65, 0x61, 0x6d, 0x52, 0x69,
    0x6e, 0x67, 0x12, 0x28, 0x0a, 0x04, 0x62, 0x61, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x61,
    0x73, 0x65, 0x42, 0x65, 0x61, 0x6d, 0x52, 0x04, 0x62, 0x61, 0x73, 0x65, 0x12, 0x20, 0x0a, 0x0b,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x1c,
    0x0a, 0x09, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x09, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x22, 0xc0, 0x02, 0x0a,
    0x10, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x72, 0x65, 0x61, 0x6b, 0x4d, 0x6f, 0x64, 0x65,
    0x6c, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x28, 0x0a, 0x06, 0x61,
    0x6e, 0x67, 0x6c, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x06, 0x61,
    0x6e, 0x67, 0x6c, 0x65, 0x73, 0x12, 0x24, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x2c, 0x0a, 0x08, 0x76,
    0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52,
    0x08, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x12, 0x24, 0x0a, 0x0d, 0x72, 0x61, 0x6e,
    0x64, 0x6f, 0x6d, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0d, 0x72, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x1e, 0x0a, 0x0a, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x0a, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12,
    0x14, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61,
    0x67, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x22,
    0xba, 0x01, 0x0a, 0x0e, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x53, 0x50, 0x44, 0x65, 0x63,
    0x61, 0x6c, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x28, 0x0a, 0x06,
    0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06,
    0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x12, 0x26, 0x0a, 0x05, 0x73, 0x61, 0x78, 0x69, 0x73, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x05, 0x73, 0x61, 0x78, 0x69, 0x73, 0x12, 0x16,
    0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x22, 0x9f, 0x01, 0x0a,
    0x0d, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x75, 0x62, 0x62, 0x6c, 0x65, 0x73, 0x12, 0x24,
    0x0a, 0x04, 0x6d, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x04,
    0x6d, 0x69, 0x6e, 0x73, 0x12, 0x24, 0x0a, 0x04, 0x6d, 0x61, 0x78, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x04, 0x6d, 0x61, 0x78, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x68, 0x65,
    0x69, 0x67, 0x68, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x06, 0x68, 0x65, 0x69, 0x67,
    0x68, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x70, 0x65, 0x65,
    0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x73, 0x70, 0x65, 0x65, 0x64, 0x22, 0xa3,
    0x01, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x42, 0x75, 0x62, 0x62, 0x6c, 0x65, 0x54,
    0x72, 0x61, 0x69, 0x6c, 0x12, 0x24, 0x0a, 0x04, 0x6d, 0x69, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x04, 0x6d, 0x69, 0x6e, 0x73, 0x12, 0x24, 0x0a, 0x04, 0x6d, 0x61,
    0x78, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x04, 0x6d, 0x61, 0x78, 0x73,
    0x12, 0x16, 0x0a, 0x06, 0x77, 0x61, 0x74, 0x65, 0x72, 0x7a, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x06, 0x77, 0x61, 0x74, 0x65, 0x72, 0x7a, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x14,
    0x0a, 0x05, 0x73, 0x70, 0x65, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x73,
    0x70, 0x65, 0x65, 0x64, 0x22, 0xa5, 0x01, 0x0a, 0x0b, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x44,
    0x65, 0x63, 0x61, 0x6c, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x26,
    0x0a, 0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52,
    0x05, 0x73, 0x74, 0x61, 0x72, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x16,
    0x0a, 0x06, 0x68, 0x69, 0x74, 0x62, 0x6f, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06,
    0x68, 0x69, 0x74, 0x62, 0x6f, 0x78, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x22, 0xf2, 0x04, 0x0a,
    0x0e, 0x43, 0x4d, 0x73, 0x67, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x44, 0x61, 0x74, 0x61, 0x12,
    0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x26, 0x0a, 0x05, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x05, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x12, 0x28, 0x0a, 0x06, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x52, 0x06, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x12, 0x28, 0x0a, 0x06, 0x61,
    0x6e, 0x67, 0x6c, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x06, 0x61,
    0x6e, 0x67, 0x6c, 0x65, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x07, 0x52, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x20, 0x0a,
    0x0b, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x07, 0x52, 0x0b, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12,
    0x14, 0x0a, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05,
    0x73, 0x63, 0x61, 0x6c, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x6d, 0x61, 0x67, 0x6e, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x6d, 0x61, 0x67, 0x6e, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x06, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x12, 0x20, 0x0a, 0x0b, 0x73,
    0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x70, 0x72, 0x6f, 0x70, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x0b, 0x73, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x70, 0x72, 0x6f, 0x70, 0x12, 0x20, 0x0a,
    0x0b, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x06, 0x52, 0x0b, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12,
    0x1e, 0x0a, 0x0a, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0c, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0a, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65, 0x74, 0x79, 0x70, 0x65, 0x12,
    0x1a, 0x0a, 0x08, 0x6d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x18, 0x0d, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x08, 0x6d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x68,
    0x69, 0x74, 0x62, 0x6f, 0x78, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x69, 0x74,
    0x62, 0x6f, 0x78, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x0f, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61,
    0x67, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x12,
    0x28, 0x0a, 0x0f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x11, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68,
    0x6d, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1e, 0x0a, 0x0a, 0x65, 0x66, 0x66,
    0x65, 0x63, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x65,
    0x66, 0x66, 0x65, 0x63, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x0e, 0x61, 0x74, 0x74,
    0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x13, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0e, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x6e, 0x61, 0x6d,
    0x65, 0x22, 0x4c, 0x0a, 0x14, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x45, 0x66, 0x66, 0x65, 0x63,
    0x74, 0x44, 0x69, 0x73, 0x70, 0x61, 0x74, 0x63, 0x68, 0x12, 0x34, 0x0a, 0x0a, 0x65, 0x66, 0x66,
    0x65, 0x63, 0x74, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x45, 0x66, 0x66, 0x65, 0x63, 0x74, 0x44,
    0x61, 0x74, 0x61, 0x52, 0x0a, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x64, 0x61, 0x74, 0x61, 0x22,
    0x7a, 0x0a, 0x12, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x45, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x53,
    0x70, 0x6c, 0x61, 0x73, 0x68, 0x12, 0x22, 0x0a, 0x03, 0x70, 0x6f, 0x73, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x03, 0x70, 0x6f, 0x73, 0x12, 0x22, 0x0a, 0x03, 0x64, 0x69, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x03, 0x64, 0x69, 0x72, 0x12, 0x1c, 0x0a,
    0x09, 0x65, 0x78, 0x70, 0x6c, 0x6f, 0x73, 0x69, 0x76, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x09, 0x65, 0x78, 0x70, 0x6c, 0x6f, 0x73, 0x69, 0x76, 0x65, 0x22, 0x58, 0x0a, 0x0a, 0x43,
    0x4d, 0x73, 0x67, 0x54, 0x45, 0x46, 0x69, 0x7a, 0x7a, 0x12, 0x16, 0x0a, 0x06, 0x65, 0x6e, 0x74,
    0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x12, 0x18, 0x0a, 0x07, 0x64, 0x65, 0x6e, 0x73, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x07, 0x64, 0x65, 0x6e, 0x73, 0x69, 0x74, 0x79, 0x12, 0x18, 0x0a, 0x07, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x74, 0x22, 0xec, 0x02, 0x0a, 0x14, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45,
    0x53, 0x68, 0x61, 0x74, 0x74, 0x65, 0x72, 0x53, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x12, 0x28,
    0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x28, 0x0a, 0x06, 0x61, 0x6e, 0x67, 0x6c,
    0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x06, 0x61, 0x6e, 0x67, 0x6c,
    0x65, 0x73, 0x12, 0x26, 0x0a, 0x05, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x52, 0x05, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x12, 0x2c, 0x0a, 0x08, 0x66, 0x6f,
    0x72, 0x63, 0x65, 0x70, 0x6f, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x08,
    0x66, 0x6f, 0x72, 0x63, 0x65, 0x70, 0x6f, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x77, 0x69, 0x64, 0x74,
    0x68, 0x18, 0x05, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x77, 0x69, 0x64, 0x74, 0x68, 0x12, 0x16,
    0x0a, 0x06, 0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x02, 0x52, 0x06,
    0x68, 0x65, 0x69, 0x67, 0x68, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x68, 0x61, 0x72, 0x64, 0x73,
    0x69, 0x7a, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x73, 0x68, 0x61, 0x72, 0x64,
    0x73, 0x69, 0x7a, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x73, 0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x73, 0x75, 0x72, 0x66, 0x61,
    0x63, 0x65, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x66, 0x72, 0x6f, 0x6e, 0x74, 0x63,
    0x6f, 0x6c, 0x6f, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0a, 0x66, 0x72, 0x6f, 0x6e,
    0x74, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x1c, 0x0a, 0x09, 0x62, 0x61, 0x63, 0x6b, 0x63, 0x6f,
    0x6c, 0x6f, 0x72, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x07, 0x52, 0x09, 0x62, 0x61, 0x63, 0x6b, 0x63,
    0x6f, 0x6c, 0x6f, 0x72, 0x22, 0x86, 0x01, 0x0a, 0x10, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x47,
    0x6c, 0x6f, 0x77, 0x53, 0x70, 0x72, 0x69, 0x74, 0x65, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69,
    0x67, 0x69, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x69, 0x66,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x04, 0x6c, 0x69, 0x66, 0x65, 0x12, 0x1e, 0x0a,
    0x0a, 0x62, 0x72, 0x69, 0x67, 0x68, 0x74, 0x6e, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0a, 0x62, 0x72, 0x69, 0x67, 0x68, 0x74, 0x6e, 0x65, 0x73, 0x73, 0x22, 0x76, 0x0a,
    0x0c, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x49, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x12, 0x28, 0x0a,
    0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52,
    0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x28, 0x0a, 0x06, 0x6e, 0x6f, 0x72, 0x6d, 0x61,
    0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6e, 0x6f, 0x72, 0x6d, 0x61,
    0x6c, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x91, 0x01, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45,
    0x4d, 0x75, 0x7a, 0x7a, 0x6c, 0x65, 0x46, 0x6c, 0x61, 0x73, 0x68, 0x12, 0x28, 0x0a, 0x06, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x28, 0x0a, 0x06, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x73, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x06, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x73, 0x12,
    0x14, 0x0a, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05,
    0x73, 0x63, 0x61, 0x6c, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x22, 0x9b, 0x01, 0x0a, 0x11, 0x43, 0x4d,
    0x73, 0x67, 0x54, 0x45, 0x42, 0x6c, 0x6f, 0x6f, 0x64, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x12,
    0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x2e, 0x0a, 0x09, 0x64, 0x69, 0x72,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x09,
    0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x6c,
    0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x07, 0x52, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x12,
    0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0xb2, 0x02, 0x0a, 0x0f, 0x43, 0x4d, 0x73, 0x67,
    0x54, 0x45, 0x45, 0x78, 0x70, 0x6c, 0x6f, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x06, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x1c, 0x0a, 0x09, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x72,
    0x61, 0x74, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x28, 0x0a, 0x06, 0x6e, 0x6f, 0x72,
    0x6d, 0x61, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6e, 0x6f, 0x72,
    0x6d, 0x61, 0x6c, 0x12, 0x22, 0x0a, 0x0c, 0x6d, 0x61, 0x74, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x6d, 0x61, 0x74, 0x65, 0x72,
    0x69, 0x61, 0x6c, 0x74, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x61, 0x64, 0x69, 0x75,
    0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x72, 0x61, 0x64, 0x69, 0x75, 0x73, 0x12,
    0x1c, 0x0a, 0x09, 0x6d, 0x61, 0x67, 0x6e, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x09, 0x6d, 0x61, 0x67, 0x6e, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x14, 0x0a,
    0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x73, 0x63,
    0x61, 0x6c, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x61, 0x66, 0x66, 0x65, 0x63, 0x74, 0x5f, 0x72, 0x61,
    0x67, 0x64, 0x6f, 0x6c, 0x6c, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x61, 0x66,
    0x66, 0x65, 0x63, 0x74, 0x52, 0x61, 0x67, 0x64, 0x6f, 0x6c, 0x6c, 0x73, 0x22, 0x90, 0x01, 0x0a,
    0x0a, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x44, 0x75, 0x73, 0x74, 0x12, 0x28, 0x0a, 0x06, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x70, 0x65,
    0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x73, 0x70, 0x65, 0x65, 0x64, 0x12,
    0x2e, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22,
    0x59, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x4c, 0x61, 0x72, 0x67, 0x65, 0x46, 0x75,
    0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x1a,
    0x0a, 0x08, 0x72, 0x65, 0x76, 0x65, 0x72, 0x73, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x08, 0x72, 0x65, 0x76, 0x65, 0x72, 0x73, 0x65, 0x64, 0x22, 0x9e, 0x01, 0x0a, 0x0c, 0x43,
    0x4d, 0x73, 0x67, 0x54, 0x45, 0x53, 0x70, 0x61, 0x72, 0x6b, 0x73, 0x12, 0x28, 0x0a, 0x06, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f,
    0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x1c, 0x0a, 0x09, 0x6d, 0x61, 0x67, 0x6e, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x6d, 0x61, 0x67, 0x6e, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x12, 0x2e, 0x0a, 0x09, 0x64,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x52, 0x09, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xd9, 0x02, 0x0a, 0x11,
    0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x50, 0x68, 0x79, 0x73, 0x69, 0x63, 0x73, 0x50, 0x72, 0x6f,
    0x70, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x2c, 0x0a, 0x08, 0x76,
    0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52,
    0x08, 0x76, 0x65, 0x6c, 0x6f, 0x63, 0x69, 0x74, 0x79, 0x12, 0x28, 0x0a, 0x06, 0x61, 0x6e, 0x67,
    0x6c, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x06, 0x61, 0x6e, 0x67,
    0x6c, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x6b, 0x69, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x07, 0x52, 0x04, 0x73, 0x6b, 0x69, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x18, 0x0a,
    0x07, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07,
    0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x07, 0x52, 0x05, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x12, 0x1e, 0x0a,
    0x0a, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x06, 0x52, 0x0a, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x32, 0x0a,
    0x14, 0x62, 0x72, 0x65, 0x61, 0x6b, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x73, 0x6e, 0x6f, 0x74, 0x74,
    0x6f, 0x6d, 0x61, 0x6b, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x62, 0x72, 0x65,
    0x61, 0x6b, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x73, 0x6e, 0x6f, 0x74, 0x74, 0x6f, 0x6d, 0x61, 0x6b,
    0x65, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x22, 0x6d, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x54,
    0x45, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x12, 0x28, 0x0a, 0x06,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06,
    0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x12, 0x16,
    0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06,
    0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x22, 0x9c, 0x01, 0x0a, 0x14, 0x43, 0x4d, 0x73, 0x67, 0x54,
    0x45, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x12,
    0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x28, 0x0a, 0x06, 0x61, 0x6e, 0x67,
    0x6c, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x06, 0x61, 0x6e, 0x67,
    0x6c, 0x65, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x69, 0x73,
    0x74, 0x61, 0x6e, 0x63, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x64, 0x69, 0x73,
    0x74, 0x61, 0x6e, 0x63, 0x65, 0x22, 0x4d, 0x0a, 0x0b, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x53,
    0x6d, 0x6f, 0x6b, 0x65, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x14,
    0x0a, 0x05, 0x73, 0x63, 0x61, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x05, 0x73,
    0x63, 0x61, 0x6c, 0x65, 0x22, 0x7c, 0x0a, 0x10, 0x43, 0x4d, 0x73, 0x67, 0x54, 0x45, 0x57, 0x6f,
    0x72, 0x6c, 0x64, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x12, 0x28, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x12, 0x28, 0x0a, 0x06, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x6e, 0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x12, 0x14, 0x0a, 0x05,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x2a, 0xd3, 0x04, 0x0a, 0x0e, 0x45, 0x54, 0x45, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x49, 0x64, 0x73, 0x12, 0x18, 0x0a, 0x13, 0x54, 0x45, 0x5f, 0x45, 0x66, 0x66, 0x65,
    0x63, 0x74, 0x44, 0x69, 0x73, 0x70, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x10, 0x90, 0x03, 0x12,
    0x17, 0x0a, 0x12, 0x54, 0x45, 0x5f, 0x41, 0x72, 0x6d, 0x6f, 0x72, 0x52, 0x69, 0x63, 0x6f, 0x63,
    0x68, 0x65, 0x74, 0x49, 0x64, 0x10, 0x91, 0x03, 0x12, 0x16, 0x0a, 0x11, 0x54, 0x45, 0x5f, 0x42,
    0x65, 0x61, 0x6d, 0x45, 0x6e, 0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x49, 0x64, 0x10, 0x92, 0x03,
    0x12, 0x12, 0x0a, 0x0d, 0x54, 0x45, 0x5f, 0x42, 0x65, 0x61, 0x6d, 0x45, 0x6e, 0x74, 0x73, 0x49,
    0x64, 0x10, 0x93, 0x03, 0x12, 0x14, 0x0a, 0x0f, 0x54, 0x45, 0x5f, 0x42, 0x65, 0x61, 0x6d, 0x50,
    0x6f, 0x69, 0x6e, 0x74, 0x73, 0x49, 0x64, 0x10, 0x94, 0x03, 0x12, 0x12, 0x0a, 0x0d, 0x54, 0x45,
    0x5f, 0x42, 0x65, 0x61, 0x6d, 0x52, 0x69, 0x6e, 0x67, 0x49, 0x64, 0x10, 0x95, 0x03, 0x12, 0x14,
    0x0a, 0x0f, 0x54, 0x45, 0x5f, 0x42, 0x72, 0x65, 0x61, 0x6b, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x49,
    0x64, 0x10, 0x96, 0x03, 0x12, 0x12, 0x0a, 0x0d, 0x54, 0x45, 0x5f, 0x42, 0x53, 0x50, 0x44, 0x65,
    0x63, 0x61, 0x6c, 0x49, 0x64, 0x10, 0x97, 0x03, 0x12, 0x11, 0x0a, 0x0c, 0x54, 0x45, 0x5f, 0x42,
    0x75, 0x62, 0x62, 0x6c, 0x65, 0x73, 0x49, 0x64, 0x10, 0x98, 0x03, 0x12, 0x15, 0x0a, 0x10, 0x54,
    0x45, 0x5f, 0x42, 0x75, 0x62, 0x62, 0x6c, 0x65, 0x54, 0x72, 0x61, 0x69, 0x6c, 0x49, 0x64, 0x10,
    0x99, 0x03, 0x12, 0x0f, 0x0a, 0x0a, 0x54, 0x45, 0x5f, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x49, 0x64,
    0x10, 0x9a, 0x03, 0x12, 0x14, 0x0a, 0x0f, 0x54, 0x45, 0x5f, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x44,
    0x65, 0x63, 0x61, 0x6c, 0x49, 0x64, 0x10, 0x9b, 0x03, 0x12, 0x16, 0x0a, 0x11, 0x54, 0x45, 0x5f,
    0x45, 0x6e, 0x65, 0x72, 0x67, 0x79, 0x53, 0x70, 0x6c, 0x61, 0x73, 0x68, 0x49, 0x64, 0x10, 0x9c,
    0x03, 0x12, 0x0e, 0x0a, 0x09, 0x54, 0x45, 0x5f, 0x46, 0x69, 0x7a, 0x7a, 0x49, 0x64, 0x10, 0x9d,
    0x03, 0x12, 0x18, 0x0a, 0x13, 0x54, 0x45, 0x5f, 0x53, 0x68, 0x61, 0x74, 0x74, 0x65, 0x72, 0x53,
    0x75, 0x72, 0x66, 0x61, 0x63, 0x65, 0x49, 0x64, 0x10, 0x9e, 0x03, 0x12, 0x14, 0x0a, 0x0f, 0x54,
    0x45, 0x5f, 0x47, 0x6c, 0x6f, 0x77, 0x53, 0x70, 0x72, 0x69, 0x74, 0x65, 0x49, 0x64, 0x10, 0x9f,
    0x03, 0x12, 0x10, 0x0a, 0x0b, 0x54, 0x45, 0x5f, 0x49, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x49, 0x64,
    0x10, 0xa0, 0x03, 0x12, 0x15, 0x0a, 0x10, 0x54, 0x45, 0x5f, 0x4d, 0x75, 0x7a, 0x7a, 0x6c, 0x65,
    0x46, 0x6c, 0x61, 0x73, 0x68, 0x49, 0x64, 0x10, 0xa1, 0x03, 0x12, 0x15, 0x0a, 0x10, 0x54, 0x45,
    0x5f, 0x42, 0x6c, 0x6f, 0x6f, 0x64, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x10, 0xa2,
    0x03, 0x12, 0x13, 0x0a, 0x0e, 0x54, 0x45, 0x5f, 0x45, 0x78, 0x70, 0x6c, 0x6f, 0x73, 0x69, 0x6f,
    0x6e, 0x49, 0x64, 0x10, 0xa3, 0x03, 0x12, 0x0e, 0x0a, 0x09, 0x54, 0x45, 0x5f, 0x44, 0x75, 0x73,
    0x74, 0x49, 0x64, 0x10, 0xa4, 0x03, 0x12, 0x15, 0x0a, 0x10, 0x54, 0x45, 0x5f, 0x4c, 0x61, 0x72,
    0x67, 0x65, 0x46, 0x75, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x10, 0xa5, 0x03, 0x12, 0x10, 0x0a,
    0x0b, 0x54, 0x45, 0x5f, 0x53, 0x70, 0x61, 0x72, 0x6b, 0x73, 0x49, 0x64, 0x10, 0xa6, 0x03, 0x12,
    0x15, 0x0a, 0x10, 0x54, 0x45, 0x5f, 0x50, 0x68, 0x79, 0x73, 0x69, 0x63, 0x73, 0x50, 0x72, 0x6f,
    0x70, 0x49, 0x64, 0x10, 0xa7, 0x03, 0x12, 0x15, 0x0a, 0x10, 0x54, 0x45, 0x5f, 0x50, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x44, 0x65, 0x63, 0x61, 0x6c, 0x49, 0x64, 0x10, 0xa8, 0x03, 0x12, 0x18, 0x0a,
    0x13, 0x54, 0x45, 0x5f, 0x50, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x44, 0x65, 0x63,
    0x61, 0x6c, 0x49, 0x64, 0x10, 0xa9, 0x03, 0x12, 0x0f, 0x0a, 0x0a, 0x54, 0x45, 0x5f, 0x53, 0x6d,
    0x6f, 0x6b, 0x65, 0x49, 0x64, 0x10, 0xaa, 0x03, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a,
    0x9a, 0x64, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x91, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x06, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x06, 0x00,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0e,
    0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03,
    0x07, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x07, 0x07,
    0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07, 0x1a,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x07, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1d, 0x22, 0x0a, 0x0a,
    0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x05, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0a, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x1e, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x1d, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x0c, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x0c, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0c, 0x1c,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0d, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x0e, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03,
    0x0e, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x08, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0f, 0x18, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x10, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02,
    0x12, 0x03, 0x10, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x11,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x11, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x11, 0x18, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x12, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x08, 0x02, 0x12, 0x03, 0x12, 0x17, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12,
    0x03, 0x13, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x13,
    0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x13, 0x1b, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x14, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x14, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x14, 0x15, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x0b, 0x12, 0x03, 0x15, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x15, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x15,
    0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x16, 0x08, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x16, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x16, 0x1c, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x0d, 0x12, 0x03, 0x17, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d,
    0x01, 0x12, 0x03, 0x17, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x02, 0x12,
    0x03, 0x17, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x18, 0x08,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x18, 0x08, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x18, 0x1e, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x19, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0f, 0x01, 0x12, 0x03, 0x19, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f,
    0x02, 0x12, 0x03, 0x19, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x10, 0x12, 0x03,
    0x1a, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x1a, 0x08,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x10, 0x02, 0x12, 0x03, 0x1a, 0x16, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x11, 0x12, 0x03, 0x1b, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x11, 0x02, 0x12, 0x03, 0x1b, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x12,
    0x12, 0x03, 0x1c, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x12, 0x01, 0x12, 0x03,
    0x1c, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x12, 0x02, 0x12, 0x03, 0x1c, 0x1b,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x13, 0x12, 0x03, 0x1d, 0x08, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x13, 0x02, 0x12, 0x03, 0x1d, 0x19, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x14, 0x12, 0x03, 0x1e, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x14, 0x01,
    0x12, 0x03, 0x1e, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x14, 0x02, 0x12, 0x03,
    0x1e, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x15, 0x12, 0x03, 0x1f, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x15, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x15, 0x02, 0x12, 0x03, 0x1f, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x16, 0x12, 0x03, 0x20, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x16, 0x01, 0x12, 0x03, 0x20, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x16, 0x02,
    0x12, 0x03, 0x20, 0x16, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x17, 0x12, 0x03, 0x21,
    0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x17, 0x01, 0x12, 0x03, 0x21, 0x08, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x17, 0x02, 0x12, 0x03, 0x21, 0x1b, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x18, 0x12, 0x03, 0x22, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x18, 0x01, 0x12, 0x03, 0x22, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x18, 0x02, 0x12, 0x03, 0x22, 0x1b, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x19, 0x12,
    0x03, 0x23, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x19, 0x01, 0x12, 0x03, 0x23,
    0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x19, 0x02, 0x12, 0x03, 0x23, 0x1e, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x1a, 0x12, 0x03, 0x24, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x24, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x1a, 0x02, 0x12, 0x03, 0x24, 0x15, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x27, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x27, 0x08,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x28, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x28, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x1c, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x28, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x29, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x29, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x29, 0x11, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x1c, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x2c, 0x00, 0x39, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x2c, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x08, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x19, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x2e, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2e, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x19, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x25, 0x26, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x2f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x2f, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2f,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x30, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x30, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x30, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12,
    0x03, 0x31, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x31,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x31, 0x11, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x31, 0x17, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x31, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x32, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x32, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x32, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x32, 0x1f,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x33, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x33, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x33, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x33, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x33, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03,
    0x34, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x34, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x34, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x34, 0x18, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x07, 0x03, 0x12, 0x03, 0x34, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x08, 0x12, 0x03, 0x35, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08,
    0x04, 0x12, 0x03, 0x35, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12,
    0x03, 0x35, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x35,
    0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x35, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x36, 0x08, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x09, 0x04, 0x12, 0x03, 0x36, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x09, 0x05, 0x12, 0x03, 0x36, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x36, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03,
    0x12, 0x03, 0x36, 0x21, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x37,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x37, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x37, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x37, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x37, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x0b, 0x12, 0x03, 0x38, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x04,
    0x12, 0x03, 0x38, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x05, 0x12, 0x03,
    0x38, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x38, 0x18,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x38, 0x20, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x3b, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x3c, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3c,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3c, 0x11, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x20, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x3d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x3d, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3d, 0x26,
    0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x3e, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x3e, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x3e, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x3f, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3f, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x3f, 0x11, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3f, 0x1c, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3f, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x40, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x40, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x40, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x40,
    0x1c, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x40, 0x22, 0x23,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x43, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x43, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x44, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x44, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x44, 0x11,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x20, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x27, 0x28, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x45, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x45, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x45, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45,
    0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x46, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x46, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x46, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x46, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x46, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x49,
    0x00, 0x4d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x49, 0x08, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x4a, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x4a, 0x20, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x4a, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x4b, 0x08,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4b, 0x11, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x1c, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4b, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x02, 0x12, 0x03, 0x4c, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x4c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4c,
    0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x1c, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4c, 0x22, 0x23, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x4f, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05,
    0x01, 0x12, 0x03, 0x4f, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03,
    0x50, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x50, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x50, 0x11, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x20, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x50, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x01, 0x12, 0x03, 0x51, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x51, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x51, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x51,
    0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x51, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x52, 0x08, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x52, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x52, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x52, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x55, 0x00, 0x5f,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x55, 0x08, 0x18, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x56, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x56, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x56, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x56, 0x1c, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x56,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x57, 0x08, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x57, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x57, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x1c, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x57, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12,
    0x03, 0x58, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x58,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x58, 0x11, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x58, 0x1c, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x58, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x59, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x59, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x59, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x59, 0x1c, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x59, 0x27,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x5a, 0x08, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x5a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x5a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x5a, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x5a, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03,
    0x5b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x5b, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05, 0x12, 0x03, 0x5b, 0x11, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x5b, 0x19, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x5b, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x06, 0x12, 0x03, 0x5c, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x5c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x5c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x5c,
    0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x5c, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x07, 0x12, 0x03, 0x5d, 0x08, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x07, 0x04, 0x12, 0x03, 0x5d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x07, 0x05, 0x12, 0x03, 0x5d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x5d, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x5d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x08, 0x12, 0x03, 0x5e,
    0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x04, 0x12, 0x03, 0x5e, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x05, 0x12, 0x03, 0x5e, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01, 0x12, 0x03, 0x5e, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x08, 0x03, 0x12, 0x03, 0x5e, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x04, 0x61, 0x00, 0x67, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x61,
    0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x62, 0x08, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x62, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x62, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x1c, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x62, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x03, 0x63, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x63,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x63, 0x11, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x63, 0x1c, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x63, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x64, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x64, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x64, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x64, 0x1c, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x64, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x65, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x03, 0x65, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x03, 0x65, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x65, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x65, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03,
    0x66, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x66, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x03, 0x66, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x66, 0x18, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x03, 0x66, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x08, 0x12, 0x04, 0x69, 0x00, 0x6f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03,
    0x69, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x08, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6a, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6a, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x1c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01,
    0x12, 0x03, 0x6b, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x03, 0x6b, 0x11,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6b, 0x1c, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6b, 0x23, 0x24, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x6c, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x6c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x6c, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x6c, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6c,
    0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x6d, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6d, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x6d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6d, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x6d, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12,
    0x03, 0x6e, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x6e,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x03, 0x6e, 0x11, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x6e, 0x17, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x6e, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x09, 0x12, 0x04, 0x71, 0x00, 0x77, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12,
    0x03, 0x71, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x72, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x72, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x72, 0x11, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x72, 0x1c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x72, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x01, 0x12, 0x03, 0x73, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x03, 0x73,
    0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x73, 0x1c, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x73, 0x23, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x74, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x74, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x74, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x74, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x74, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x03, 0x75, 0x08, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03, 0x75, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x03, 0x75, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x75, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x75, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04,
    0x12, 0x03, 0x76, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x76, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x05, 0x12, 0x03, 0x76, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x03, 0x76, 0x17, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x03, 0x76, 0x1f, 0x20, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0a, 0x12, 0x04, 0x79, 0x00, 0x7f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01,
    0x12, 0x03, 0x79, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x7a,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7a, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x7a, 0x11, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7a, 0x1c, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7a, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x01, 0x12, 0x03, 0x7b, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x7b, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7b, 0x1c,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7b, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x7c, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x7c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x7c, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x7c, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x03, 0x12, 0x03, 0x7d, 0x08,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7d, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7d, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7d, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7d, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x04, 0x12, 0x03, 0x7e, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x7e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x03, 0x7e,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7e, 0x18, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x03, 0x7e, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x81, 0x01, 0x00, 0x95, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0b, 0x01, 0x12, 0x04, 0x81, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x00, 0x12, 0x04, 0x82, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x82, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12,
    0x04, 0x82, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x82, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x82,
    0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x04, 0x83, 0x01, 0x08,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x83, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x04, 0x83, 0x01, 0x11, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0x83, 0x01, 0x1c, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04, 0x83, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0x84, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0x84, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x02, 0x06, 0x12, 0x04, 0x84, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x01, 0x12, 0x04, 0x84, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02,
    0x03, 0x12, 0x04, 0x84, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12,
    0x04, 0x85, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x04,
    0x85, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x06, 0x12, 0x04, 0x85,
    0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x04, 0x85, 0x01,
    0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x04, 0x85, 0x01, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x04, 0x86, 0x01, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x04, 0x86, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x04, 0x86, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x04, 0x86, 0x01, 0x19, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x04, 0x86, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x05, 0x12, 0x04, 0x87, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x05, 0x04, 0x12, 0x04, 0x87, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05,
    0x05, 0x12, 0x04, 0x87, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x87, 0x01, 0x19, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12,
    0x04, 0x87, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x06, 0x12, 0x04, 0x88,
    0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x04, 0x12, 0x04, 0x88, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x05, 0x12, 0x04, 0x88, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x01, 0x12, 0x04, 0x88, 0x01, 0x17, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x03, 0x12, 0x04, 0x88, 0x01, 0x1f, 0x20, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x07, 0x12, 0x04, 0x89, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x07, 0x04, 0x12, 0x04, 0x89, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x07, 0x05, 0x12, 0x04, 0x89, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x07, 0x01, 0x12, 0x04, 0x89, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x07, 0x03, 0x12, 0x04, 0x89, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x08, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x04,
    0x12, 0x04, 0x8a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x05, 0x12,
    0x04, 0x8a, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x01, 0x12, 0x04,
    0x8a, 0x01, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x03, 0x12, 0x04, 0x8a,
    0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x09, 0x12, 0x04, 0x8b, 0x01, 0x08,
    0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x05, 0x12, 0x04, 0x8b, 0x01, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x19, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x0a, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x0a, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x0a, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x19, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a,
    0x03, 0x12, 0x04, 0x8c, 0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0b, 0x12,
    0x04, 0x8d, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x04, 0x12, 0x04,
    0x8d, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x05, 0x12, 0x04, 0x8d,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x8d, 0x01,
    0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x25,
    0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0c, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x05, 0x12, 0x04, 0x8e, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x0d, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x0d, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d,
    0x05, 0x12, 0x04, 0x8f, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x01,
    0x12, 0x04, 0x8f, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x03, 0x12,
    0x04, 0x8f, 0x01, 0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0e, 0x12, 0x04, 0x90,
    0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x04, 0x12, 0x04, 0x90, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x05, 0x12, 0x04, 0x90, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x01, 0x12, 0x04, 0x90, 0x01, 0x18, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x03, 0x12, 0x04, 0x90, 0x01, 0x20, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0f, 0x12, 0x04, 0x91, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x91, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x0f, 0x05, 0x12, 0x04, 0x91, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x91, 0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x0f, 0x03, 0x12, 0x04, 0x91, 0x01, 0x20, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x10, 0x12, 0x04, 0x92, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x04,
    0x12, 0x04, 0x92, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x05, 0x12,
    0x04, 0x92, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x01, 0x12, 0x04,
    0x92, 0x01, 0x17, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x03, 0x12, 0x04, 0x92,
    0x01, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x11, 0x12, 0x04, 0x93, 0x01, 0x08,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x11, 0x04, 0x12, 0x04, 0x93, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x11, 0x05, 0x12, 0x04, 0x93, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x11, 0x01, 0x12, 0x04, 0x93, 0x01, 0x18, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x11, 0x03, 0x12, 0x04, 0x93, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x12, 0x12, 0x04, 0x94, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x12, 0x04, 0x12, 0x04, 0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x12, 0x05, 0x12, 0x04, 0x94, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x12, 0x01, 0x12, 0x04, 0x94, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12,
    0x03, 0x12, 0x04, 0x94, 0x01, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x97,
    0x01, 0x00, 0x99, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x97, 0x01,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0x98, 0x01, 0x08, 0x2f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x98, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x04, 0x98, 0x01, 0x11, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x98, 0x01, 0x20, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x98, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x06, 0x9b, 0x01, 0x00, 0x9f, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12,
    0x04, 0x9c, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x9c, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9c,
    0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9c, 0x01,
    0x1c, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9d, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x1c, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x02, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x05, 0x12, 0x04, 0x9e, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x9e, 0x01, 0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x9e, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xa1, 0x01, 0x00,
    0xa5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x12,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xa3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xa3, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xa3, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0xa4, 0x01,
    0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x11, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x17, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xa7, 0x01, 0x00, 0xb2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0f, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x00, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xa8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xa8, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xa8, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa8,
    0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x08,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa9, 0x01, 0x11, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x1c, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x02, 0x06, 0x12, 0x04, 0xaa, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x1c, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xaa, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x03, 0x12,
    0x04, 0xab, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xab, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x06, 0x12, 0x04, 0xab,
    0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12, 0x04, 0xab, 0x01,
    0x1c, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x04, 0xab, 0x01, 0x27,
    0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x04, 0x12, 0x04, 0xac, 0x01, 0x08, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x04, 0x12, 0x04, 0xac, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x05, 0x12, 0x04, 0xac, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x04, 0x01, 0x12, 0x04, 0xac, 0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x04, 0x03, 0x12, 0x04, 0xac, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x05, 0x12, 0x04, 0xad, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x05, 0x04, 0x12, 0x04, 0xad, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05,
    0x05, 0x12, 0x04, 0xad, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x01,
    0x12, 0x04, 0xad, 0x01, 0x17, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x03, 0x12,
    0x04, 0xad, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x06, 0x12, 0x04, 0xae,
    0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x04, 0x12, 0x04, 0xae, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x05, 0x12, 0x04, 0xae, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x01, 0x12, 0x04, 0xae, 0x01, 0x17, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x03, 0x12, 0x04, 0xae, 0x01, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x07, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x07, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x07, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x07, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x07, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x08, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x04,
    0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x05, 0x12,
    0x04, 0xb0, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x01, 0x12, 0x04,
    0xb0, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x03, 0x12, 0x04, 0xb0,
    0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x09, 0x12, 0x04, 0xb1, 0x01, 0x08,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x19, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x10, 0x12, 0x06, 0xb4, 0x01, 0x00, 0xb9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x10, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00,
    0x12, 0x04, 0xb5, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xb5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xb5, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb5,
    0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb5, 0x01,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb6, 0x01, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xb7, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xb7, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04,
    0xb8, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x04, 0xb8,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x05, 0x12, 0x04, 0xb8, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x18,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb8, 0x01, 0x25, 0x26,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xbb, 0x01, 0x00, 0xbf, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xbc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x06, 0x12, 0x04, 0xbc, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xbc, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xbc, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xbd,
    0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbd, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x06, 0x12, 0x04, 0xbd, 0x01, 0x11,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x1c, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbd, 0x01, 0x25, 0x26, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbe, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12,
    0x06, 0xc1, 0x01, 0x00, 0xc6, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04,
    0xc1, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xc2, 0x01,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc2, 0x01, 0x11, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x1c, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc3, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02,
    0x12, 0x04, 0xc4, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xc4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xc4, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc4,
    0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc4, 0x01,
    0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x03, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc5, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc5, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x13, 0x12, 0x06, 0xc8, 0x01, 0x00, 0xcd, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13,
    0x01, 0x12, 0x04, 0xc8, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12,
    0x04, 0xc9, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xc9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc9,
    0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc9, 0x01,
    0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xca, 0x01, 0x08, 0x2a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xca, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x06, 0x12, 0x04, 0xca, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xca, 0x01, 0x1c, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xca, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x02, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xcb, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xcb, 0x01, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xcb, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0xcc,
    0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12, 0x04, 0xcc, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x18, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xcf, 0x01, 0x00, 0xd9, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x00, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xd0, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd0, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd0, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xd1, 0x01,
    0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x18, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03,
    0x12, 0x04, 0xd3, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xd3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x06, 0x12, 0x04,
    0xd3, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd3,
    0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd3, 0x01,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd4, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x05, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x05, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x05, 0x05, 0x12, 0x04, 0xd5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xd5, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x03,
    0x12, 0x04, 0xd5, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x06, 0x12, 0x04,
    0xd6, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x04, 0x12, 0x04, 0xd6,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x05, 0x12, 0x04, 0xd6, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x18,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x24, 0x25,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x07, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x07, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x07, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x07, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x07, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x08, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x08,
    0x04, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x08, 0x05,
    0x12, 0x04, 0xd8, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x08, 0x01, 0x12,
    0x04, 0xd8, 0x01, 0x16, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x08, 0x03, 0x12, 0x04,
    0xd8, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xdb, 0x01, 0x00, 0xe0,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x12, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xdc, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xdc, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x01, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xdd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xdd, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xdd, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdd,
    0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xde, 0x01, 0x08,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04, 0xde, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0xde, 0x01, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0xde, 0x01, 0x17, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0xde, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0xdf, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0xdf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x03, 0x06, 0x12, 0x04, 0xdf, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x1c, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xdf, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xe2,
    0x01, 0x00, 0xe5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xe2, 0x01,
    0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xe3, 0x01, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe3, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe3, 0x01, 0x11, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe3, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xe4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xe4, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xe4, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xe7, 0x01,
    0x00, 0xec, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x08,
    0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe8, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe8, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x01, 0x12, 0x04, 0xe9, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xe9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xe9, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xe9, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xe9, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04, 0xea,
    0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0xea, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x04, 0xea, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0xea, 0x01, 0x18, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0xea, 0x01, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x03, 0x06, 0x12, 0x04, 0xeb, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x1c, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12,
    0x06, 0xee, 0x01, 0x00, 0xf9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04,
    0xee, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xef, 0x01,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xef, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0xef, 0x01, 0x11, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xef, 0x01, 0x1c, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0xef, 0x01, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf0, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x1c, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xf0, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x02,
    0x12, 0x04, 0xf1, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xf1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x06, 0x12, 0x04,
    0xf1, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf1,
    0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf1, 0x01,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x03, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf2, 0x01, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x19, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf2, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x18, 0x02, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xf3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xf3, 0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xf3, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x05, 0x12, 0x04,
    0xf4, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x04, 0x12, 0x04, 0xf4,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x05, 0x12, 0x04, 0xf4, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x01, 0x12, 0x04, 0xf4, 0x01, 0x18,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x22, 0x23,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x06, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x06, 0x04, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf5, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x06, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x06, 0x03, 0x12, 0x04, 0xf5, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x07, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x07,
    0x04, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x07, 0x05,
    0x12, 0x04, 0xf6, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xf6, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x07, 0x03, 0x12, 0x04,
    0xf6, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x08, 0x12, 0x04, 0xf7, 0x01,
    0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x08, 0x04, 0x12, 0x04, 0xf7, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x08, 0x05, 0x12, 0x04, 0xf7, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x08, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x18, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x08, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x2f, 0x30, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x09, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x09, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x09, 0x05, 0x12, 0x04, 0xf8, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x09, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x09, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06,
    0xfb, 0x01, 0x00, 0xff, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xfb,
    0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xfc, 0x01, 0x08,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfc, 0x01, 0x11, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x1c, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xfd, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xfd, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12,
    0x04, 0xfe, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xfe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xfe,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfe, 0x01,
    0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfe, 0x01, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0x81, 0x02, 0x00, 0x86, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0x81, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0x82, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x82, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x82, 0x02, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x82, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04,
    0x83, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x83,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x06, 0x12, 0x04, 0x83, 0x02,
    0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x83, 0x02, 0x1c,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x83, 0x02, 0x25, 0x26,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x02, 0x12, 0x04, 0x84, 0x02, 0x08, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x04, 0x12, 0x04, 0x84, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x02, 0x05, 0x12, 0x04, 0x84, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x02, 0x01, 0x12, 0x04, 0x84, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x02, 0x03, 0x12, 0x04, 0x84, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a,
    0x02, 0x03, 0x12, 0x04, 0x85, 0x02, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x85, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x05,
    0x12, 0x04, 0x85, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x85, 0x02, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x85, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0x88, 0x02, 0x00, 0x8b,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0x88, 0x02, 0x08, 0x13, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0x89, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x89, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0x89, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x89, 0x02, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x89, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02,
    0x01, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x8a, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x8a, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x8a, 0x02, 0x17, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8a,
    0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0x8d, 0x02, 0x00, 0x91, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x08, 0x18, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8e, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x8e, 0x02, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x8e, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01,
    0x12, 0x04, 0x8f, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x8f, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x06, 0x12, 0x04,
    0x8f, 0x02, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8f,
    0x02, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8f, 0x02,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02, 0x12, 0x04, 0x90, 0x02, 0x08, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x90, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x05, 0x12, 0x04, 0x90, 0x02, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x90, 0x02, 0x18, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x02, 0x03, 0x12, 0x04, 0x90, 0x02, 0x20, 0x21,
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
