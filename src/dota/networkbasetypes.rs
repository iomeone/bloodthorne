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
pub struct CMsgVector {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgVector {}

impl CMsgVector {
    pub fn new() -> CMsgVector {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgVector {
        static mut instance: ::protobuf::lazy::Lazy<CMsgVector> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgVector,
        };
        unsafe {
            instance.get(CMsgVector::new)
        }
    }

    // optional float x = 1;

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

    // optional float y = 2;

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

    // optional float z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> f32 {
        self.z.unwrap_or(0.)
    }

    fn get_z_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.z
    }
}

impl ::protobuf::Message for CMsgVector {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.z = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.x {
            my_size += 5;
        };
        if let Some(v) = self.y {
            my_size += 5;
        };
        if let Some(v) = self.z {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.y {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.z {
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

impl ::protobuf::MessageStatic for CMsgVector {
    fn new() -> CMsgVector {
        CMsgVector::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgVector>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    CMsgVector::get_x_for_reflect,
                    CMsgVector::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    CMsgVector::get_y_for_reflect,
                    CMsgVector::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "z",
                    CMsgVector::get_z_for_reflect,
                    CMsgVector::mut_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgVector>(
                    "CMsgVector",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgVector {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgVector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgVector {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgVector2D {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgVector2D {}

impl CMsgVector2D {
    pub fn new() -> CMsgVector2D {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgVector2D {
        static mut instance: ::protobuf::lazy::Lazy<CMsgVector2D> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgVector2D,
        };
        unsafe {
            instance.get(CMsgVector2D::new)
        }
    }

    // optional float x = 1;

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

    // optional float y = 2;

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
}

impl ::protobuf::Message for CMsgVector2D {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.x {
            my_size += 5;
        };
        if let Some(v) = self.y {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.y {
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

impl ::protobuf::MessageStatic for CMsgVector2D {
    fn new() -> CMsgVector2D {
        CMsgVector2D::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgVector2D>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    CMsgVector2D::get_x_for_reflect,
                    CMsgVector2D::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    CMsgVector2D::get_y_for_reflect,
                    CMsgVector2D::mut_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgVector2D>(
                    "CMsgVector2D",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgVector2D {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgVector2D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgVector2D {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgQAngle {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgQAngle {}

impl CMsgQAngle {
    pub fn new() -> CMsgQAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgQAngle {
        static mut instance: ::protobuf::lazy::Lazy<CMsgQAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgQAngle,
        };
        unsafe {
            instance.get(CMsgQAngle::new)
        }
    }

    // optional float x = 1;

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

    // optional float y = 2;

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

    // optional float z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> f32 {
        self.z.unwrap_or(0.)
    }

    fn get_z_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.z
    }
}

impl ::protobuf::Message for CMsgQAngle {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.z = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.x {
            my_size += 5;
        };
        if let Some(v) = self.y {
            my_size += 5;
        };
        if let Some(v) = self.z {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.y {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.z {
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

impl ::protobuf::MessageStatic for CMsgQAngle {
    fn new() -> CMsgQAngle {
        CMsgQAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgQAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    CMsgQAngle::get_x_for_reflect,
                    CMsgQAngle::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    CMsgQAngle::get_y_for_reflect,
                    CMsgQAngle::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "z",
                    CMsgQAngle::get_z_for_reflect,
                    CMsgQAngle::mut_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgQAngle>(
                    "CMsgQAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgQAngle {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgQAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgQAngle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPlayerInfo {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    xuid: ::std::option::Option<u64>,
    userid: ::std::option::Option<i32>,
    steamid: ::std::option::Option<u64>,
    fakeplayer: ::std::option::Option<bool>,
    ishltv: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPlayerInfo {}

impl CMsgPlayerInfo {
    pub fn new() -> CMsgPlayerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPlayerInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPlayerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPlayerInfo,
        };
        unsafe {
            instance.get(CMsgPlayerInfo::new)
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

    // optional int32 userid = 3;

    pub fn clear_userid(&mut self) {
        self.userid = ::std::option::Option::None;
    }

    pub fn has_userid(&self) -> bool {
        self.userid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_userid(&mut self, v: i32) {
        self.userid = ::std::option::Option::Some(v);
    }

    pub fn get_userid(&self) -> i32 {
        self.userid.unwrap_or(0)
    }

    fn get_userid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.userid
    }

    fn mut_userid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.userid
    }

    // optional fixed64 steamid = 4;

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

    // optional bool fakeplayer = 5;

    pub fn clear_fakeplayer(&mut self) {
        self.fakeplayer = ::std::option::Option::None;
    }

    pub fn has_fakeplayer(&self) -> bool {
        self.fakeplayer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fakeplayer(&mut self, v: bool) {
        self.fakeplayer = ::std::option::Option::Some(v);
    }

    pub fn get_fakeplayer(&self) -> bool {
        self.fakeplayer.unwrap_or(false)
    }

    fn get_fakeplayer_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.fakeplayer
    }

    fn mut_fakeplayer_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.fakeplayer
    }

    // optional bool ishltv = 6;

    pub fn clear_ishltv(&mut self) {
        self.ishltv = ::std::option::Option::None;
    }

    pub fn has_ishltv(&self) -> bool {
        self.ishltv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ishltv(&mut self, v: bool) {
        self.ishltv = ::std::option::Option::Some(v);
    }

    pub fn get_ishltv(&self) -> bool {
        self.ishltv.unwrap_or(false)
    }

    fn get_ishltv_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ishltv
    }

    fn mut_ishltv_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ishltv
    }
}

impl ::protobuf::Message for CMsgPlayerInfo {
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
                    let tmp = is.read_int32()?;
                    self.userid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.fakeplayer = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.ishltv = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.xuid {
            my_size += 9;
        };
        if let Some(v) = self.userid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.steamid {
            my_size += 9;
        };
        if let Some(v) = self.fakeplayer {
            my_size += 2;
        };
        if let Some(v) = self.ishltv {
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
        if let Some(v) = self.xuid {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.userid {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.steamid {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.fakeplayer {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.ishltv {
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

impl ::protobuf::MessageStatic for CMsgPlayerInfo {
    fn new() -> CMsgPlayerInfo {
        CMsgPlayerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPlayerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgPlayerInfo::get_name_for_reflect,
                    CMsgPlayerInfo::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "xuid",
                    CMsgPlayerInfo::get_xuid_for_reflect,
                    CMsgPlayerInfo::mut_xuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "userid",
                    CMsgPlayerInfo::get_userid_for_reflect,
                    CMsgPlayerInfo::mut_userid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgPlayerInfo::get_steamid_for_reflect,
                    CMsgPlayerInfo::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "fakeplayer",
                    CMsgPlayerInfo::get_fakeplayer_for_reflect,
                    CMsgPlayerInfo::mut_fakeplayer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ishltv",
                    CMsgPlayerInfo::get_ishltv_for_reflect,
                    CMsgPlayerInfo::mut_ishltv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPlayerInfo>(
                    "CMsgPlayerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPlayerInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_xuid();
        self.clear_userid();
        self.clear_steamid();
        self.clear_fakeplayer();
        self.clear_ishltv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPlayerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPlayerInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsg_CVars {
    // message fields
    cvars: ::protobuf::RepeatedField<CMsg_CVars_CVar>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsg_CVars {}

impl CMsg_CVars {
    pub fn new() -> CMsg_CVars {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsg_CVars {
        static mut instance: ::protobuf::lazy::Lazy<CMsg_CVars> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsg_CVars,
        };
        unsafe {
            instance.get(CMsg_CVars::new)
        }
    }

    // repeated .dota.CMsg_CVars.CVar cvars = 1;

    pub fn clear_cvars(&mut self) {
        self.cvars.clear();
    }

    // Param is passed by value, moved
    pub fn set_cvars(&mut self, v: ::protobuf::RepeatedField<CMsg_CVars_CVar>) {
        self.cvars = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cvars(&mut self) -> &mut ::protobuf::RepeatedField<CMsg_CVars_CVar> {
        &mut self.cvars
    }

    // Take field
    pub fn take_cvars(&mut self) -> ::protobuf::RepeatedField<CMsg_CVars_CVar> {
        ::std::mem::replace(&mut self.cvars, ::protobuf::RepeatedField::new())
    }

    pub fn get_cvars(&self) -> &[CMsg_CVars_CVar] {
        &self.cvars
    }

    fn get_cvars_for_reflect(&self) -> &::protobuf::RepeatedField<CMsg_CVars_CVar> {
        &self.cvars
    }

    fn mut_cvars_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsg_CVars_CVar> {
        &mut self.cvars
    }
}

impl ::protobuf::Message for CMsg_CVars {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cvars)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.cvars {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.cvars {
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

impl ::protobuf::MessageStatic for CMsg_CVars {
    fn new() -> CMsg_CVars {
        CMsg_CVars::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsg_CVars>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsg_CVars_CVar>>(
                    "cvars",
                    CMsg_CVars::get_cvars_for_reflect,
                    CMsg_CVars::mut_cvars_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsg_CVars>(
                    "CMsg_CVars",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsg_CVars {
    fn clear(&mut self) {
        self.clear_cvars();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsg_CVars {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsg_CVars {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsg_CVars_CVar {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsg_CVars_CVar {}

impl CMsg_CVars_CVar {
    pub fn new() -> CMsg_CVars_CVar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsg_CVars_CVar {
        static mut instance: ::protobuf::lazy::Lazy<CMsg_CVars_CVar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsg_CVars_CVar,
        };
        unsafe {
            instance.get(CMsg_CVars_CVar::new)
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

impl ::protobuf::Message for CMsg_CVars_CVar {
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

impl ::protobuf::MessageStatic for CMsg_CVars_CVar {
    fn new() -> CMsg_CVars_CVar {
        CMsg_CVars_CVar::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsg_CVars_CVar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsg_CVars_CVar::get_name_for_reflect,
                    CMsg_CVars_CVar::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CMsg_CVars_CVar::get_value_for_reflect,
                    CMsg_CVars_CVar::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsg_CVars_CVar>(
                    "CMsg_CVars_CVar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsg_CVars_CVar {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsg_CVars_CVar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsg_CVars_CVar {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_NOP {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_NOP {}

impl CNETMsg_NOP {
    pub fn new() -> CNETMsg_NOP {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_NOP {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_NOP> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_NOP,
        };
        unsafe {
            instance.get(CNETMsg_NOP::new)
        }
    }
}

impl ::protobuf::Message for CNETMsg_NOP {
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

impl ::protobuf::MessageStatic for CNETMsg_NOP {
    fn new() -> CNETMsg_NOP {
        CNETMsg_NOP::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_NOP>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_NOP>(
                    "CNETMsg_NOP",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_NOP {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_NOP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_NOP {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SplitScreenUser {
    // message fields
    slot: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SplitScreenUser {}

impl CNETMsg_SplitScreenUser {
    pub fn new() -> CNETMsg_SplitScreenUser {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SplitScreenUser {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SplitScreenUser> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SplitScreenUser,
        };
        unsafe {
            instance.get(CNETMsg_SplitScreenUser::new)
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

impl ::protobuf::Message for CNETMsg_SplitScreenUser {
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

impl ::protobuf::MessageStatic for CNETMsg_SplitScreenUser {
    fn new() -> CNETMsg_SplitScreenUser {
        CNETMsg_SplitScreenUser::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SplitScreenUser>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slot",
                    CNETMsg_SplitScreenUser::get_slot_for_reflect,
                    CNETMsg_SplitScreenUser::mut_slot_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SplitScreenUser>(
                    "CNETMsg_SplitScreenUser",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SplitScreenUser {
    fn clear(&mut self) {
        self.clear_slot();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SplitScreenUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SplitScreenUser {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_Disconnect {
    // message fields
    reason: ::std::option::Option<super::network_connection::ENetworkDisconnectionReason>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_Disconnect {}

impl CNETMsg_Disconnect {
    pub fn new() -> CNETMsg_Disconnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_Disconnect {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_Disconnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_Disconnect,
        };
        unsafe {
            instance.get(CNETMsg_Disconnect::new)
        }
    }

    // optional .dota.ENetworkDisconnectionReason reason = 2;

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: super::network_connection::ENetworkDisconnectionReason) {
        self.reason = ::std::option::Option::Some(v);
    }

    pub fn get_reason(&self) -> super::network_connection::ENetworkDisconnectionReason {
        self.reason.unwrap_or(super::network_connection::ENetworkDisconnectionReason::NETWORK_DISCONNECT_INVALID)
    }

    fn get_reason_for_reflect(&self) -> &::std::option::Option<super::network_connection::ENetworkDisconnectionReason> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::option::Option<super::network_connection::ENetworkDisconnectionReason> {
        &mut self.reason
    }
}

impl ::protobuf::Message for CNETMsg_Disconnect {
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
                    let tmp = is.read_enum()?;
                    self.reason = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.reason {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reason {
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

impl ::protobuf::MessageStatic for CNETMsg_Disconnect {
    fn new() -> CNETMsg_Disconnect {
        CNETMsg_Disconnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_Disconnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::network_connection::ENetworkDisconnectionReason>>(
                    "reason",
                    CNETMsg_Disconnect::get_reason_for_reflect,
                    CNETMsg_Disconnect::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_Disconnect>(
                    "CNETMsg_Disconnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_Disconnect {
    fn clear(&mut self) {
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_Disconnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_Disconnect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_Tick {
    // message fields
    tick: ::std::option::Option<u32>,
    host_frametime: ::std::option::Option<u32>,
    host_frametime_std_deviation: ::std::option::Option<u32>,
    host_computationtime: ::std::option::Option<u32>,
    host_computationtime_std_deviation: ::std::option::Option<u32>,
    host_framestarttime_std_deviation: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_Tick {}

impl CNETMsg_Tick {
    pub fn new() -> CNETMsg_Tick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_Tick {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_Tick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_Tick,
        };
        unsafe {
            instance.get(CNETMsg_Tick::new)
        }
    }

    // optional uint32 tick = 1;

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

    // optional uint32 host_frametime = 2;

    pub fn clear_host_frametime(&mut self) {
        self.host_frametime = ::std::option::Option::None;
    }

    pub fn has_host_frametime(&self) -> bool {
        self.host_frametime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_frametime(&mut self, v: u32) {
        self.host_frametime = ::std::option::Option::Some(v);
    }

    pub fn get_host_frametime(&self) -> u32 {
        self.host_frametime.unwrap_or(0)
    }

    fn get_host_frametime_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_frametime
    }

    fn mut_host_frametime_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_frametime
    }

    // optional uint32 host_frametime_std_deviation = 3;

    pub fn clear_host_frametime_std_deviation(&mut self) {
        self.host_frametime_std_deviation = ::std::option::Option::None;
    }

    pub fn has_host_frametime_std_deviation(&self) -> bool {
        self.host_frametime_std_deviation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_frametime_std_deviation(&mut self, v: u32) {
        self.host_frametime_std_deviation = ::std::option::Option::Some(v);
    }

    pub fn get_host_frametime_std_deviation(&self) -> u32 {
        self.host_frametime_std_deviation.unwrap_or(0)
    }

    fn get_host_frametime_std_deviation_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_frametime_std_deviation
    }

    fn mut_host_frametime_std_deviation_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_frametime_std_deviation
    }

    // optional uint32 host_computationtime = 4;

    pub fn clear_host_computationtime(&mut self) {
        self.host_computationtime = ::std::option::Option::None;
    }

    pub fn has_host_computationtime(&self) -> bool {
        self.host_computationtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_computationtime(&mut self, v: u32) {
        self.host_computationtime = ::std::option::Option::Some(v);
    }

    pub fn get_host_computationtime(&self) -> u32 {
        self.host_computationtime.unwrap_or(0)
    }

    fn get_host_computationtime_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_computationtime
    }

    fn mut_host_computationtime_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_computationtime
    }

    // optional uint32 host_computationtime_std_deviation = 5;

    pub fn clear_host_computationtime_std_deviation(&mut self) {
        self.host_computationtime_std_deviation = ::std::option::Option::None;
    }

    pub fn has_host_computationtime_std_deviation(&self) -> bool {
        self.host_computationtime_std_deviation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_computationtime_std_deviation(&mut self, v: u32) {
        self.host_computationtime_std_deviation = ::std::option::Option::Some(v);
    }

    pub fn get_host_computationtime_std_deviation(&self) -> u32 {
        self.host_computationtime_std_deviation.unwrap_or(0)
    }

    fn get_host_computationtime_std_deviation_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_computationtime_std_deviation
    }

    fn mut_host_computationtime_std_deviation_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_computationtime_std_deviation
    }

    // optional uint32 host_framestarttime_std_deviation = 6;

    pub fn clear_host_framestarttime_std_deviation(&mut self) {
        self.host_framestarttime_std_deviation = ::std::option::Option::None;
    }

    pub fn has_host_framestarttime_std_deviation(&self) -> bool {
        self.host_framestarttime_std_deviation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_framestarttime_std_deviation(&mut self, v: u32) {
        self.host_framestarttime_std_deviation = ::std::option::Option::Some(v);
    }

    pub fn get_host_framestarttime_std_deviation(&self) -> u32 {
        self.host_framestarttime_std_deviation.unwrap_or(0)
    }

    fn get_host_framestarttime_std_deviation_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_framestarttime_std_deviation
    }

    fn mut_host_framestarttime_std_deviation_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_framestarttime_std_deviation
    }
}

impl ::protobuf::Message for CNETMsg_Tick {
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
                    self.tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.host_frametime = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.host_frametime_std_deviation = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.host_computationtime = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.host_computationtime_std_deviation = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.host_framestarttime_std_deviation = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.host_frametime {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.host_frametime_std_deviation {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.host_computationtime {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.host_computationtime_std_deviation {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.host_framestarttime_std_deviation {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.host_frametime {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.host_frametime_std_deviation {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.host_computationtime {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.host_computationtime_std_deviation {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.host_framestarttime_std_deviation {
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

impl ::protobuf::MessageStatic for CNETMsg_Tick {
    fn new() -> CNETMsg_Tick {
        CNETMsg_Tick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_Tick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tick",
                    CNETMsg_Tick::get_tick_for_reflect,
                    CNETMsg_Tick::mut_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_frametime",
                    CNETMsg_Tick::get_host_frametime_for_reflect,
                    CNETMsg_Tick::mut_host_frametime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_frametime_std_deviation",
                    CNETMsg_Tick::get_host_frametime_std_deviation_for_reflect,
                    CNETMsg_Tick::mut_host_frametime_std_deviation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_computationtime",
                    CNETMsg_Tick::get_host_computationtime_for_reflect,
                    CNETMsg_Tick::mut_host_computationtime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_computationtime_std_deviation",
                    CNETMsg_Tick::get_host_computationtime_std_deviation_for_reflect,
                    CNETMsg_Tick::mut_host_computationtime_std_deviation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_framestarttime_std_deviation",
                    CNETMsg_Tick::get_host_framestarttime_std_deviation_for_reflect,
                    CNETMsg_Tick::mut_host_framestarttime_std_deviation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_Tick>(
                    "CNETMsg_Tick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_Tick {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_host_frametime();
        self.clear_host_frametime_std_deviation();
        self.clear_host_computationtime();
        self.clear_host_computationtime_std_deviation();
        self.clear_host_framestarttime_std_deviation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_Tick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_Tick {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_StringCmd {
    // message fields
    command: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_StringCmd {}

impl CNETMsg_StringCmd {
    pub fn new() -> CNETMsg_StringCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_StringCmd {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_StringCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_StringCmd,
        };
        unsafe {
            instance.get(CNETMsg_StringCmd::new)
        }
    }

    // optional string command = 1;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: ::std::string::String) {
        self.command = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&mut self) -> &mut ::std::string::String {
        if self.command.is_none() {
            self.command.set_default();
        };
        self.command.as_mut().unwrap()
    }

    // Take field
    pub fn take_command(&mut self) -> ::std::string::String {
        self.command.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_command(&self) -> &str {
        match self.command.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_command_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.command
    }
}

impl ::protobuf::Message for CNETMsg_StringCmd {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.command)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.command.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.command.as_ref() {
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

impl ::protobuf::MessageStatic for CNETMsg_StringCmd {
    fn new() -> CNETMsg_StringCmd {
        CNETMsg_StringCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_StringCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "command",
                    CNETMsg_StringCmd::get_command_for_reflect,
                    CNETMsg_StringCmd::mut_command_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_StringCmd>(
                    "CNETMsg_StringCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_StringCmd {
    fn clear(&mut self) {
        self.clear_command();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_StringCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_StringCmd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SetConVar {
    // message fields
    convars: ::protobuf::SingularPtrField<CMsg_CVars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SetConVar {}

impl CNETMsg_SetConVar {
    pub fn new() -> CNETMsg_SetConVar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SetConVar {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SetConVar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SetConVar,
        };
        unsafe {
            instance.get(CNETMsg_SetConVar::new)
        }
    }

    // optional .dota.CMsg_CVars convars = 1;

    pub fn clear_convars(&mut self) {
        self.convars.clear();
    }

    pub fn has_convars(&self) -> bool {
        self.convars.is_some()
    }

    // Param is passed by value, moved
    pub fn set_convars(&mut self, v: CMsg_CVars) {
        self.convars = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_convars(&mut self) -> &mut CMsg_CVars {
        if self.convars.is_none() {
            self.convars.set_default();
        };
        self.convars.as_mut().unwrap()
    }

    // Take field
    pub fn take_convars(&mut self) -> CMsg_CVars {
        self.convars.take().unwrap_or_else(|| CMsg_CVars::new())
    }

    pub fn get_convars(&self) -> &CMsg_CVars {
        self.convars.as_ref().unwrap_or_else(|| CMsg_CVars::default_instance())
    }

    fn get_convars_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsg_CVars> {
        &self.convars
    }

    fn mut_convars_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsg_CVars> {
        &mut self.convars
    }
}

impl ::protobuf::Message for CNETMsg_SetConVar {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.convars)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.convars.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.convars.as_ref() {
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

impl ::protobuf::MessageStatic for CNETMsg_SetConVar {
    fn new() -> CNETMsg_SetConVar {
        CNETMsg_SetConVar::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SetConVar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsg_CVars>>(
                    "convars",
                    CNETMsg_SetConVar::get_convars_for_reflect,
                    CNETMsg_SetConVar::mut_convars_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SetConVar>(
                    "CNETMsg_SetConVar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SetConVar {
    fn clear(&mut self) {
        self.clear_convars();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SetConVar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SetConVar {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SignonState {
    // message fields
    signon_state: ::std::option::Option<u32>,
    spawn_count: ::std::option::Option<u32>,
    num_server_players: ::std::option::Option<u32>,
    players_networkids: ::protobuf::RepeatedField<::std::string::String>,
    map_name: ::protobuf::SingularField<::std::string::String>,
    addons: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SignonState {}

impl CNETMsg_SignonState {
    pub fn new() -> CNETMsg_SignonState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SignonState {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SignonState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SignonState,
        };
        unsafe {
            instance.get(CNETMsg_SignonState::new)
        }
    }

    // optional uint32 signon_state = 1;

    pub fn clear_signon_state(&mut self) {
        self.signon_state = ::std::option::Option::None;
    }

    pub fn has_signon_state(&self) -> bool {
        self.signon_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signon_state(&mut self, v: u32) {
        self.signon_state = ::std::option::Option::Some(v);
    }

    pub fn get_signon_state(&self) -> u32 {
        self.signon_state.unwrap_or(0)
    }

    fn get_signon_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.signon_state
    }

    fn mut_signon_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.signon_state
    }

    // optional uint32 spawn_count = 2;

    pub fn clear_spawn_count(&mut self) {
        self.spawn_count = ::std::option::Option::None;
    }

    pub fn has_spawn_count(&self) -> bool {
        self.spawn_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawn_count(&mut self, v: u32) {
        self.spawn_count = ::std::option::Option::Some(v);
    }

    pub fn get_spawn_count(&self) -> u32 {
        self.spawn_count.unwrap_or(0)
    }

    fn get_spawn_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawn_count
    }

    fn mut_spawn_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawn_count
    }

    // optional uint32 num_server_players = 3;

    pub fn clear_num_server_players(&mut self) {
        self.num_server_players = ::std::option::Option::None;
    }

    pub fn has_num_server_players(&self) -> bool {
        self.num_server_players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_server_players(&mut self, v: u32) {
        self.num_server_players = ::std::option::Option::Some(v);
    }

    pub fn get_num_server_players(&self) -> u32 {
        self.num_server_players.unwrap_or(0)
    }

    fn get_num_server_players_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_server_players
    }

    fn mut_num_server_players_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_server_players
    }

    // repeated string players_networkids = 4;

    pub fn clear_players_networkids(&mut self) {
        self.players_networkids.clear();
    }

    // Param is passed by value, moved
    pub fn set_players_networkids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.players_networkids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players_networkids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.players_networkids
    }

    // Take field
    pub fn take_players_networkids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.players_networkids, ::protobuf::RepeatedField::new())
    }

    pub fn get_players_networkids(&self) -> &[::std::string::String] {
        &self.players_networkids
    }

    fn get_players_networkids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.players_networkids
    }

    fn mut_players_networkids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.players_networkids
    }

    // optional string map_name = 5;

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

    // optional string addons = 6;

    pub fn clear_addons(&mut self) {
        self.addons.clear();
    }

    pub fn has_addons(&self) -> bool {
        self.addons.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addons(&mut self, v: ::std::string::String) {
        self.addons = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addons(&mut self) -> &mut ::std::string::String {
        if self.addons.is_none() {
            self.addons.set_default();
        };
        self.addons.as_mut().unwrap()
    }

    // Take field
    pub fn take_addons(&mut self) -> ::std::string::String {
        self.addons.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_addons(&self) -> &str {
        match self.addons.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_addons_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.addons
    }

    fn mut_addons_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.addons
    }
}

impl ::protobuf::Message for CNETMsg_SignonState {
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
                    self.signon_state = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.spawn_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.num_server_players = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.players_networkids)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.addons)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.signon_state {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.spawn_count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_server_players {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.players_networkids {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(v) = self.map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.addons.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.signon_state {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.spawn_count {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.num_server_players {
            os.write_uint32(3, v)?;
        };
        for v in &self.players_networkids {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.map_name.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.addons.as_ref() {
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

impl ::protobuf::MessageStatic for CNETMsg_SignonState {
    fn new() -> CNETMsg_SignonState {
        CNETMsg_SignonState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SignonState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "signon_state",
                    CNETMsg_SignonState::get_signon_state_for_reflect,
                    CNETMsg_SignonState::mut_signon_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawn_count",
                    CNETMsg_SignonState::get_spawn_count_for_reflect,
                    CNETMsg_SignonState::mut_spawn_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_server_players",
                    CNETMsg_SignonState::get_num_server_players_for_reflect,
                    CNETMsg_SignonState::mut_num_server_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "players_networkids",
                    CNETMsg_SignonState::get_players_networkids_for_reflect,
                    CNETMsg_SignonState::mut_players_networkids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_name",
                    CNETMsg_SignonState::get_map_name_for_reflect,
                    CNETMsg_SignonState::mut_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addons",
                    CNETMsg_SignonState::get_addons_for_reflect,
                    CNETMsg_SignonState::mut_addons_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SignonState>(
                    "CNETMsg_SignonState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SignonState {
    fn clear(&mut self) {
        self.clear_signon_state();
        self.clear_spawn_count();
        self.clear_num_server_players();
        self.clear_players_networkids();
        self.clear_map_name();
        self.clear_addons();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SignonState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SignonState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEvent {
    // message fields
    event_name: ::protobuf::SingularField<::std::string::String>,
    eventid: ::std::option::Option<i32>,
    keys: ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEvent {}

impl CSVCMsg_GameEvent {
    pub fn new() -> CSVCMsg_GameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEvent,
        };
        unsafe {
            instance.get(CSVCMsg_GameEvent::new)
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

    // repeated .dota.CSVCMsg_GameEvent.key_t keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CSVCMsg_GameEvent_key_t] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CSVCMsg_GameEvent {
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEvent {
    fn new() -> CSVCMsg_GameEvent {
        CSVCMsg_GameEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event_name",
                    CSVCMsg_GameEvent::get_event_name_for_reflect,
                    CSVCMsg_GameEvent::mut_event_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eventid",
                    CSVCMsg_GameEvent::get_eventid_for_reflect,
                    CSVCMsg_GameEvent::mut_eventid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_GameEvent_key_t>>(
                    "keys",
                    CSVCMsg_GameEvent::get_keys_for_reflect,
                    CSVCMsg_GameEvent::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEvent>(
                    "CSVCMsg_GameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEvent {
    fn clear(&mut self) {
        self.clear_event_name();
        self.clear_eventid();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEvent_key_t {
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
unsafe impl ::std::marker::Sync for CSVCMsg_GameEvent_key_t {}

impl CSVCMsg_GameEvent_key_t {
    pub fn new() -> CSVCMsg_GameEvent_key_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEvent_key_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEvent_key_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEvent_key_t,
        };
        unsafe {
            instance.get(CSVCMsg_GameEvent_key_t::new)
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

impl ::protobuf::Message for CSVCMsg_GameEvent_key_t {
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEvent_key_t {
    fn new() -> CSVCMsg_GameEvent_key_t {
        CSVCMsg_GameEvent_key_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEvent_key_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CSVCMsg_GameEvent_key_t::get_field_type_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "val_string",
                    CSVCMsg_GameEvent_key_t::get_val_string_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "val_float",
                    CSVCMsg_GameEvent_key_t::get_val_float_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_float_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_long",
                    CSVCMsg_GameEvent_key_t::get_val_long_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_long_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_short",
                    CSVCMsg_GameEvent_key_t::get_val_short_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_short_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_byte",
                    CSVCMsg_GameEvent_key_t::get_val_byte_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_byte_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "val_bool",
                    CSVCMsg_GameEvent_key_t::get_val_bool_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_bool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "val_uint64",
                    CSVCMsg_GameEvent_key_t::get_val_uint64_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_uint64_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEvent_key_t>(
                    "CSVCMsg_GameEvent_key_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEvent_key_t {
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

impl ::std::fmt::Debug for CSVCMsg_GameEvent_key_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEvent_key_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsgList_GameEvents {
    // message fields
    events: ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsgList_GameEvents {}

impl CSVCMsgList_GameEvents {
    pub fn new() -> CSVCMsgList_GameEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsgList_GameEvents {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsgList_GameEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsgList_GameEvents,
        };
        unsafe {
            instance.get(CSVCMsgList_GameEvents::new)
        }
    }

    // repeated .dota.CSVCMsgList_GameEvents.event_t events = 1;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[CSVCMsgList_GameEvents_event_t] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsgList_GameEvents_event_t> {
        &mut self.events
    }
}

impl ::protobuf::Message for CSVCMsgList_GameEvents {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.events {
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

impl ::protobuf::MessageStatic for CSVCMsgList_GameEvents {
    fn new() -> CSVCMsgList_GameEvents {
        CSVCMsgList_GameEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsgList_GameEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsgList_GameEvents_event_t>>(
                    "events",
                    CSVCMsgList_GameEvents::get_events_for_reflect,
                    CSVCMsgList_GameEvents::mut_events_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsgList_GameEvents>(
                    "CSVCMsgList_GameEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsgList_GameEvents {
    fn clear(&mut self) {
        self.clear_events();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsgList_GameEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsgList_GameEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsgList_GameEvents_event_t {
    // message fields
    tick: ::std::option::Option<i32>,
    event: ::protobuf::SingularPtrField<CSVCMsg_GameEvent>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsgList_GameEvents_event_t {}

impl CSVCMsgList_GameEvents_event_t {
    pub fn new() -> CSVCMsgList_GameEvents_event_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsgList_GameEvents_event_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsgList_GameEvents_event_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsgList_GameEvents_event_t,
        };
        unsafe {
            instance.get(CSVCMsgList_GameEvents_event_t::new)
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

    // optional .dota.CSVCMsg_GameEvent event = 2;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: CSVCMsg_GameEvent) {
        self.event = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut CSVCMsg_GameEvent {
        if self.event.is_none() {
            self.event.set_default();
        };
        self.event.as_mut().unwrap()
    }

    // Take field
    pub fn take_event(&mut self) -> CSVCMsg_GameEvent {
        self.event.take().unwrap_or_else(|| CSVCMsg_GameEvent::new())
    }

    pub fn get_event(&self) -> &CSVCMsg_GameEvent {
        self.event.as_ref().unwrap_or_else(|| CSVCMsg_GameEvent::default_instance())
    }

    fn get_event_for_reflect(&self) -> &::protobuf::SingularPtrField<CSVCMsg_GameEvent> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CSVCMsg_GameEvent> {
        &mut self.event
    }
}

impl ::protobuf::Message for CSVCMsgList_GameEvents_event_t {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.event)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.event.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.event.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsgList_GameEvents_event_t {
    fn new() -> CSVCMsgList_GameEvents_event_t {
        CSVCMsgList_GameEvents_event_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsgList_GameEvents_event_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tick",
                    CSVCMsgList_GameEvents_event_t::get_tick_for_reflect,
                    CSVCMsgList_GameEvents_event_t::mut_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_GameEvent>>(
                    "event",
                    CSVCMsgList_GameEvents_event_t::get_event_for_reflect,
                    CSVCMsgList_GameEvents_event_t::mut_event_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsgList_GameEvents_event_t>(
                    "CSVCMsgList_GameEvents_event_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsgList_GameEvents_event_t {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_event();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsgList_GameEvents_event_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsgList_GameEvents_event_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_UserMessage {
    // message fields
    msg_type: ::std::option::Option<i32>,
    msg_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_UserMessage {}

impl CSVCMsg_UserMessage {
    pub fn new() -> CSVCMsg_UserMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_UserMessage {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_UserMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_UserMessage,
        };
        unsafe {
            instance.get(CSVCMsg_UserMessage::new)
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

    // optional bytes msg_data = 2;

    pub fn clear_msg_data(&mut self) {
        self.msg_data.clear();
    }

    pub fn has_msg_data(&self) -> bool {
        self.msg_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.msg_data.is_none() {
            self.msg_data.set_default();
        };
        self.msg_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg_data(&mut self) -> ::std::vec::Vec<u8> {
        self.msg_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_msg_data(&self) -> &[u8] {
        match self.msg_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_msg_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.msg_data
    }

    fn mut_msg_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.msg_data
    }
}

impl ::protobuf::Message for CSVCMsg_UserMessage {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.msg_data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.msg_data.as_ref() {
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
        if let Some(v) = self.msg_data.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_UserMessage {
    fn new() -> CSVCMsg_UserMessage {
        CSVCMsg_UserMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_UserMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "msg_type",
                    CSVCMsg_UserMessage::get_msg_type_for_reflect,
                    CSVCMsg_UserMessage::mut_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "msg_data",
                    CSVCMsg_UserMessage::get_msg_data_for_reflect,
                    CSVCMsg_UserMessage::mut_msg_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_UserMessage>(
                    "CSVCMsg_UserMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_UserMessage {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_msg_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_UserMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_UserMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsgList_UserMessages {
    // message fields
    usermsgs: ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsgList_UserMessages {}

impl CSVCMsgList_UserMessages {
    pub fn new() -> CSVCMsgList_UserMessages {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsgList_UserMessages {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsgList_UserMessages> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsgList_UserMessages,
        };
        unsafe {
            instance.get(CSVCMsgList_UserMessages::new)
        }
    }

    // repeated .dota.CSVCMsgList_UserMessages.usermsg_t usermsgs = 1;

    pub fn clear_usermsgs(&mut self) {
        self.usermsgs.clear();
    }

    // Param is passed by value, moved
    pub fn set_usermsgs(&mut self, v: ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t>) {
        self.usermsgs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_usermsgs(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t> {
        &mut self.usermsgs
    }

    // Take field
    pub fn take_usermsgs(&mut self) -> ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t> {
        ::std::mem::replace(&mut self.usermsgs, ::protobuf::RepeatedField::new())
    }

    pub fn get_usermsgs(&self) -> &[CSVCMsgList_UserMessages_usermsg_t] {
        &self.usermsgs
    }

    fn get_usermsgs_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t> {
        &self.usermsgs
    }

    fn mut_usermsgs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsgList_UserMessages_usermsg_t> {
        &mut self.usermsgs
    }
}

impl ::protobuf::Message for CSVCMsgList_UserMessages {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.usermsgs)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.usermsgs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.usermsgs {
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

impl ::protobuf::MessageStatic for CSVCMsgList_UserMessages {
    fn new() -> CSVCMsgList_UserMessages {
        CSVCMsgList_UserMessages::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsgList_UserMessages>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsgList_UserMessages_usermsg_t>>(
                    "usermsgs",
                    CSVCMsgList_UserMessages::get_usermsgs_for_reflect,
                    CSVCMsgList_UserMessages::mut_usermsgs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsgList_UserMessages>(
                    "CSVCMsgList_UserMessages",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsgList_UserMessages {
    fn clear(&mut self) {
        self.clear_usermsgs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsgList_UserMessages {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsgList_UserMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsgList_UserMessages_usermsg_t {
    // message fields
    tick: ::std::option::Option<i32>,
    msg: ::protobuf::SingularPtrField<CSVCMsg_UserMessage>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsgList_UserMessages_usermsg_t {}

impl CSVCMsgList_UserMessages_usermsg_t {
    pub fn new() -> CSVCMsgList_UserMessages_usermsg_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsgList_UserMessages_usermsg_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsgList_UserMessages_usermsg_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsgList_UserMessages_usermsg_t,
        };
        unsafe {
            instance.get(CSVCMsgList_UserMessages_usermsg_t::new)
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

    // optional .dota.CSVCMsg_UserMessage msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: CSVCMsg_UserMessage) {
        self.msg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut CSVCMsg_UserMessage {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> CSVCMsg_UserMessage {
        self.msg.take().unwrap_or_else(|| CSVCMsg_UserMessage::new())
    }

    pub fn get_msg(&self) -> &CSVCMsg_UserMessage {
        self.msg.as_ref().unwrap_or_else(|| CSVCMsg_UserMessage::default_instance())
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularPtrField<CSVCMsg_UserMessage> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CSVCMsg_UserMessage> {
        &mut self.msg
    }
}

impl ::protobuf::Message for CSVCMsgList_UserMessages_usermsg_t {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.msg)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.msg.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.msg.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsgList_UserMessages_usermsg_t {
    fn new() -> CSVCMsgList_UserMessages_usermsg_t {
        CSVCMsgList_UserMessages_usermsg_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsgList_UserMessages_usermsg_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tick",
                    CSVCMsgList_UserMessages_usermsg_t::get_tick_for_reflect,
                    CSVCMsgList_UserMessages_usermsg_t::mut_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_UserMessage>>(
                    "msg",
                    CSVCMsgList_UserMessages_usermsg_t::get_msg_for_reflect,
                    CSVCMsgList_UserMessages_usermsg_t::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsgList_UserMessages_usermsg_t>(
                    "CSVCMsgList_UserMessages_usermsg_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsgList_UserMessages_usermsg_t {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsgList_UserMessages_usermsg_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsgList_UserMessages_usermsg_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SpawnGroup_Load {
    // message fields
    worldname: ::protobuf::SingularField<::std::string::String>,
    entitylumpname: ::protobuf::SingularField<::std::string::String>,
    entityfiltername: ::protobuf::SingularField<::std::string::String>,
    spawngrouphandle: ::std::option::Option<u32>,
    spawngroupownerhandle: ::std::option::Option<u32>,
    world_offset_pos: ::protobuf::SingularPtrField<CMsgVector>,
    world_offset_angle: ::protobuf::SingularPtrField<CMsgQAngle>,
    spawngroupmanifest: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    flags: ::std::option::Option<u32>,
    tickcount: ::std::option::Option<i32>,
    manifestincomplete: ::std::option::Option<bool>,
    localnamefixup: ::protobuf::SingularField<::std::string::String>,
    parentnamefixup: ::protobuf::SingularField<::std::string::String>,
    manifestloadpriority: ::std::option::Option<i32>,
    worldgroupid: ::std::option::Option<u32>,
    creationsequence: ::std::option::Option<u32>,
    savegamefilename: ::protobuf::SingularField<::std::string::String>,
    spawngroupparenthandle: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SpawnGroup_Load {}

impl CNETMsg_SpawnGroup_Load {
    pub fn new() -> CNETMsg_SpawnGroup_Load {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SpawnGroup_Load {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SpawnGroup_Load> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SpawnGroup_Load,
        };
        unsafe {
            instance.get(CNETMsg_SpawnGroup_Load::new)
        }
    }

    // optional string worldname = 1;

    pub fn clear_worldname(&mut self) {
        self.worldname.clear();
    }

    pub fn has_worldname(&self) -> bool {
        self.worldname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_worldname(&mut self, v: ::std::string::String) {
        self.worldname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_worldname(&mut self) -> &mut ::std::string::String {
        if self.worldname.is_none() {
            self.worldname.set_default();
        };
        self.worldname.as_mut().unwrap()
    }

    // Take field
    pub fn take_worldname(&mut self) -> ::std::string::String {
        self.worldname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_worldname(&self) -> &str {
        match self.worldname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_worldname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.worldname
    }

    fn mut_worldname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.worldname
    }

    // optional string entitylumpname = 2;

    pub fn clear_entitylumpname(&mut self) {
        self.entitylumpname.clear();
    }

    pub fn has_entitylumpname(&self) -> bool {
        self.entitylumpname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entitylumpname(&mut self, v: ::std::string::String) {
        self.entitylumpname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entitylumpname(&mut self) -> &mut ::std::string::String {
        if self.entitylumpname.is_none() {
            self.entitylumpname.set_default();
        };
        self.entitylumpname.as_mut().unwrap()
    }

    // Take field
    pub fn take_entitylumpname(&mut self) -> ::std::string::String {
        self.entitylumpname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_entitylumpname(&self) -> &str {
        match self.entitylumpname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_entitylumpname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.entitylumpname
    }

    fn mut_entitylumpname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.entitylumpname
    }

    // optional string entityfiltername = 3;

    pub fn clear_entityfiltername(&mut self) {
        self.entityfiltername.clear();
    }

    pub fn has_entityfiltername(&self) -> bool {
        self.entityfiltername.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entityfiltername(&mut self, v: ::std::string::String) {
        self.entityfiltername = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entityfiltername(&mut self) -> &mut ::std::string::String {
        if self.entityfiltername.is_none() {
            self.entityfiltername.set_default();
        };
        self.entityfiltername.as_mut().unwrap()
    }

    // Take field
    pub fn take_entityfiltername(&mut self) -> ::std::string::String {
        self.entityfiltername.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_entityfiltername(&self) -> &str {
        match self.entityfiltername.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_entityfiltername_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.entityfiltername
    }

    fn mut_entityfiltername_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.entityfiltername
    }

    // optional uint32 spawngrouphandle = 4;

    pub fn clear_spawngrouphandle(&mut self) {
        self.spawngrouphandle = ::std::option::Option::None;
    }

    pub fn has_spawngrouphandle(&self) -> bool {
        self.spawngrouphandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngrouphandle(&mut self, v: u32) {
        self.spawngrouphandle = ::std::option::Option::Some(v);
    }

    pub fn get_spawngrouphandle(&self) -> u32 {
        self.spawngrouphandle.unwrap_or(0)
    }

    fn get_spawngrouphandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawngrouphandle
    }

    fn mut_spawngrouphandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawngrouphandle
    }

    // optional uint32 spawngroupownerhandle = 5;

    pub fn clear_spawngroupownerhandle(&mut self) {
        self.spawngroupownerhandle = ::std::option::Option::None;
    }

    pub fn has_spawngroupownerhandle(&self) -> bool {
        self.spawngroupownerhandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngroupownerhandle(&mut self, v: u32) {
        self.spawngroupownerhandle = ::std::option::Option::Some(v);
    }

    pub fn get_spawngroupownerhandle(&self) -> u32 {
        self.spawngroupownerhandle.unwrap_or(0)
    }

    fn get_spawngroupownerhandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawngroupownerhandle
    }

    fn mut_spawngroupownerhandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawngroupownerhandle
    }

    // optional .dota.CMsgVector world_offset_pos = 6;

    pub fn clear_world_offset_pos(&mut self) {
        self.world_offset_pos.clear();
    }

    pub fn has_world_offset_pos(&self) -> bool {
        self.world_offset_pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_offset_pos(&mut self, v: CMsgVector) {
        self.world_offset_pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_offset_pos(&mut self) -> &mut CMsgVector {
        if self.world_offset_pos.is_none() {
            self.world_offset_pos.set_default();
        };
        self.world_offset_pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_offset_pos(&mut self) -> CMsgVector {
        self.world_offset_pos.take().unwrap_or_else(|| CMsgVector::new())
    }

    pub fn get_world_offset_pos(&self) -> &CMsgVector {
        self.world_offset_pos.as_ref().unwrap_or_else(|| CMsgVector::default_instance())
    }

    fn get_world_offset_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgVector> {
        &self.world_offset_pos
    }

    fn mut_world_offset_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgVector> {
        &mut self.world_offset_pos
    }

    // optional .dota.CMsgQAngle world_offset_angle = 7;

    pub fn clear_world_offset_angle(&mut self) {
        self.world_offset_angle.clear();
    }

    pub fn has_world_offset_angle(&self) -> bool {
        self.world_offset_angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_offset_angle(&mut self, v: CMsgQAngle) {
        self.world_offset_angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_offset_angle(&mut self) -> &mut CMsgQAngle {
        if self.world_offset_angle.is_none() {
            self.world_offset_angle.set_default();
        };
        self.world_offset_angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_offset_angle(&mut self) -> CMsgQAngle {
        self.world_offset_angle.take().unwrap_or_else(|| CMsgQAngle::new())
    }

    pub fn get_world_offset_angle(&self) -> &CMsgQAngle {
        self.world_offset_angle.as_ref().unwrap_or_else(|| CMsgQAngle::default_instance())
    }

    fn get_world_offset_angle_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgQAngle> {
        &self.world_offset_angle
    }

    fn mut_world_offset_angle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgQAngle> {
        &mut self.world_offset_angle
    }

    // optional bytes spawngroupmanifest = 8;

    pub fn clear_spawngroupmanifest(&mut self) {
        self.spawngroupmanifest.clear();
    }

    pub fn has_spawngroupmanifest(&self) -> bool {
        self.spawngroupmanifest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngroupmanifest(&mut self, v: ::std::vec::Vec<u8>) {
        self.spawngroupmanifest = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spawngroupmanifest(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.spawngroupmanifest.is_none() {
            self.spawngroupmanifest.set_default();
        };
        self.spawngroupmanifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_spawngroupmanifest(&mut self) -> ::std::vec::Vec<u8> {
        self.spawngroupmanifest.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_spawngroupmanifest(&self) -> &[u8] {
        match self.spawngroupmanifest.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_spawngroupmanifest_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.spawngroupmanifest
    }

    fn mut_spawngroupmanifest_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.spawngroupmanifest
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

    // optional int32 tickcount = 10;

    pub fn clear_tickcount(&mut self) {
        self.tickcount = ::std::option::Option::None;
    }

    pub fn has_tickcount(&self) -> bool {
        self.tickcount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tickcount(&mut self, v: i32) {
        self.tickcount = ::std::option::Option::Some(v);
    }

    pub fn get_tickcount(&self) -> i32 {
        self.tickcount.unwrap_or(0)
    }

    fn get_tickcount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.tickcount
    }

    fn mut_tickcount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.tickcount
    }

    // optional bool manifestincomplete = 11;

    pub fn clear_manifestincomplete(&mut self) {
        self.manifestincomplete = ::std::option::Option::None;
    }

    pub fn has_manifestincomplete(&self) -> bool {
        self.manifestincomplete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manifestincomplete(&mut self, v: bool) {
        self.manifestincomplete = ::std::option::Option::Some(v);
    }

    pub fn get_manifestincomplete(&self) -> bool {
        self.manifestincomplete.unwrap_or(false)
    }

    fn get_manifestincomplete_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.manifestincomplete
    }

    fn mut_manifestincomplete_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.manifestincomplete
    }

    // optional string localnamefixup = 12;

    pub fn clear_localnamefixup(&mut self) {
        self.localnamefixup.clear();
    }

    pub fn has_localnamefixup(&self) -> bool {
        self.localnamefixup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localnamefixup(&mut self, v: ::std::string::String) {
        self.localnamefixup = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localnamefixup(&mut self) -> &mut ::std::string::String {
        if self.localnamefixup.is_none() {
            self.localnamefixup.set_default();
        };
        self.localnamefixup.as_mut().unwrap()
    }

    // Take field
    pub fn take_localnamefixup(&mut self) -> ::std::string::String {
        self.localnamefixup.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_localnamefixup(&self) -> &str {
        match self.localnamefixup.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_localnamefixup_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.localnamefixup
    }

    fn mut_localnamefixup_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.localnamefixup
    }

    // optional string parentnamefixup = 13;

    pub fn clear_parentnamefixup(&mut self) {
        self.parentnamefixup.clear();
    }

    pub fn has_parentnamefixup(&self) -> bool {
        self.parentnamefixup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parentnamefixup(&mut self, v: ::std::string::String) {
        self.parentnamefixup = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parentnamefixup(&mut self) -> &mut ::std::string::String {
        if self.parentnamefixup.is_none() {
            self.parentnamefixup.set_default();
        };
        self.parentnamefixup.as_mut().unwrap()
    }

    // Take field
    pub fn take_parentnamefixup(&mut self) -> ::std::string::String {
        self.parentnamefixup.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_parentnamefixup(&self) -> &str {
        match self.parentnamefixup.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_parentnamefixup_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.parentnamefixup
    }

    fn mut_parentnamefixup_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.parentnamefixup
    }

    // optional int32 manifestloadpriority = 14;

    pub fn clear_manifestloadpriority(&mut self) {
        self.manifestloadpriority = ::std::option::Option::None;
    }

    pub fn has_manifestloadpriority(&self) -> bool {
        self.manifestloadpriority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manifestloadpriority(&mut self, v: i32) {
        self.manifestloadpriority = ::std::option::Option::Some(v);
    }

    pub fn get_manifestloadpriority(&self) -> i32 {
        self.manifestloadpriority.unwrap_or(0)
    }

    fn get_manifestloadpriority_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.manifestloadpriority
    }

    fn mut_manifestloadpriority_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.manifestloadpriority
    }

    // optional uint32 worldgroupid = 15;

    pub fn clear_worldgroupid(&mut self) {
        self.worldgroupid = ::std::option::Option::None;
    }

    pub fn has_worldgroupid(&self) -> bool {
        self.worldgroupid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_worldgroupid(&mut self, v: u32) {
        self.worldgroupid = ::std::option::Option::Some(v);
    }

    pub fn get_worldgroupid(&self) -> u32 {
        self.worldgroupid.unwrap_or(0)
    }

    fn get_worldgroupid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.worldgroupid
    }

    fn mut_worldgroupid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.worldgroupid
    }

    // optional uint32 creationsequence = 16;

    pub fn clear_creationsequence(&mut self) {
        self.creationsequence = ::std::option::Option::None;
    }

    pub fn has_creationsequence(&self) -> bool {
        self.creationsequence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creationsequence(&mut self, v: u32) {
        self.creationsequence = ::std::option::Option::Some(v);
    }

    pub fn get_creationsequence(&self) -> u32 {
        self.creationsequence.unwrap_or(0)
    }

    fn get_creationsequence_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.creationsequence
    }

    fn mut_creationsequence_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.creationsequence
    }

    // optional string savegamefilename = 17;

    pub fn clear_savegamefilename(&mut self) {
        self.savegamefilename.clear();
    }

    pub fn has_savegamefilename(&self) -> bool {
        self.savegamefilename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_savegamefilename(&mut self, v: ::std::string::String) {
        self.savegamefilename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_savegamefilename(&mut self) -> &mut ::std::string::String {
        if self.savegamefilename.is_none() {
            self.savegamefilename.set_default();
        };
        self.savegamefilename.as_mut().unwrap()
    }

    // Take field
    pub fn take_savegamefilename(&mut self) -> ::std::string::String {
        self.savegamefilename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_savegamefilename(&self) -> &str {
        match self.savegamefilename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_savegamefilename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.savegamefilename
    }

    fn mut_savegamefilename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.savegamefilename
    }

    // optional uint32 spawngroupparenthandle = 18;

    pub fn clear_spawngroupparenthandle(&mut self) {
        self.spawngroupparenthandle = ::std::option::Option::None;
    }

    pub fn has_spawngroupparenthandle(&self) -> bool {
        self.spawngroupparenthandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngroupparenthandle(&mut self, v: u32) {
        self.spawngroupparenthandle = ::std::option::Option::Some(v);
    }

    pub fn get_spawngroupparenthandle(&self) -> u32 {
        self.spawngroupparenthandle.unwrap_or(0)
    }

    fn get_spawngroupparenthandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawngroupparenthandle
    }

    fn mut_spawngroupparenthandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawngroupparenthandle
    }
}

impl ::protobuf::Message for CNETMsg_SpawnGroup_Load {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.worldname)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.entitylumpname)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.entityfiltername)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.spawngrouphandle = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.spawngroupownerhandle = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.world_offset_pos)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.world_offset_angle)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.spawngroupmanifest)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.tickcount = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.manifestincomplete = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.localnamefixup)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.parentnamefixup)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.manifestloadpriority = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.worldgroupid = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.creationsequence = ::std::option::Option::Some(tmp);
                },
                17 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.savegamefilename)?;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.spawngroupparenthandle = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.worldname.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.entitylumpname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.entityfiltername.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.spawngrouphandle {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.spawngroupownerhandle {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.world_offset_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.world_offset_angle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.spawngroupmanifest.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tickcount {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.manifestincomplete {
            my_size += 2;
        };
        if let Some(v) = self.localnamefixup.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        };
        if let Some(v) = self.parentnamefixup.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        };
        if let Some(v) = self.manifestloadpriority {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.worldgroupid {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.creationsequence {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.savegamefilename.as_ref() {
            my_size += ::protobuf::rt::string_size(17, &v);
        };
        if let Some(v) = self.spawngroupparenthandle {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.worldname.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.entitylumpname.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.entityfiltername.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.spawngrouphandle {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.spawngroupownerhandle {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.world_offset_pos.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.world_offset_angle.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.spawngroupmanifest.as_ref() {
            os.write_bytes(8, &v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.tickcount {
            os.write_int32(10, v)?;
        };
        if let Some(v) = self.manifestincomplete {
            os.write_bool(11, v)?;
        };
        if let Some(v) = self.localnamefixup.as_ref() {
            os.write_string(12, &v)?;
        };
        if let Some(v) = self.parentnamefixup.as_ref() {
            os.write_string(13, &v)?;
        };
        if let Some(v) = self.manifestloadpriority {
            os.write_int32(14, v)?;
        };
        if let Some(v) = self.worldgroupid {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.creationsequence {
            os.write_uint32(16, v)?;
        };
        if let Some(v) = self.savegamefilename.as_ref() {
            os.write_string(17, &v)?;
        };
        if let Some(v) = self.spawngroupparenthandle {
            os.write_uint32(18, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CNETMsg_SpawnGroup_Load {
    fn new() -> CNETMsg_SpawnGroup_Load {
        CNETMsg_SpawnGroup_Load::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SpawnGroup_Load>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "worldname",
                    CNETMsg_SpawnGroup_Load::get_worldname_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_worldname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "entitylumpname",
                    CNETMsg_SpawnGroup_Load::get_entitylumpname_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_entitylumpname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "entityfiltername",
                    CNETMsg_SpawnGroup_Load::get_entityfiltername_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_entityfiltername_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawngrouphandle",
                    CNETMsg_SpawnGroup_Load::get_spawngrouphandle_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_spawngrouphandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawngroupownerhandle",
                    CNETMsg_SpawnGroup_Load::get_spawngroupownerhandle_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_spawngroupownerhandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgVector>>(
                    "world_offset_pos",
                    CNETMsg_SpawnGroup_Load::get_world_offset_pos_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_world_offset_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgQAngle>>(
                    "world_offset_angle",
                    CNETMsg_SpawnGroup_Load::get_world_offset_angle_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_world_offset_angle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "spawngroupmanifest",
                    CNETMsg_SpawnGroup_Load::get_spawngroupmanifest_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_spawngroupmanifest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CNETMsg_SpawnGroup_Load::get_flags_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tickcount",
                    CNETMsg_SpawnGroup_Load::get_tickcount_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_tickcount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "manifestincomplete",
                    CNETMsg_SpawnGroup_Load::get_manifestincomplete_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_manifestincomplete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "localnamefixup",
                    CNETMsg_SpawnGroup_Load::get_localnamefixup_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_localnamefixup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "parentnamefixup",
                    CNETMsg_SpawnGroup_Load::get_parentnamefixup_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_parentnamefixup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "manifestloadpriority",
                    CNETMsg_SpawnGroup_Load::get_manifestloadpriority_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_manifestloadpriority_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "worldgroupid",
                    CNETMsg_SpawnGroup_Load::get_worldgroupid_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_worldgroupid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "creationsequence",
                    CNETMsg_SpawnGroup_Load::get_creationsequence_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_creationsequence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "savegamefilename",
                    CNETMsg_SpawnGroup_Load::get_savegamefilename_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_savegamefilename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawngroupparenthandle",
                    CNETMsg_SpawnGroup_Load::get_spawngroupparenthandle_for_reflect,
                    CNETMsg_SpawnGroup_Load::mut_spawngroupparenthandle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SpawnGroup_Load>(
                    "CNETMsg_SpawnGroup_Load",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SpawnGroup_Load {
    fn clear(&mut self) {
        self.clear_worldname();
        self.clear_entitylumpname();
        self.clear_entityfiltername();
        self.clear_spawngrouphandle();
        self.clear_spawngroupownerhandle();
        self.clear_world_offset_pos();
        self.clear_world_offset_angle();
        self.clear_spawngroupmanifest();
        self.clear_flags();
        self.clear_tickcount();
        self.clear_manifestincomplete();
        self.clear_localnamefixup();
        self.clear_parentnamefixup();
        self.clear_manifestloadpriority();
        self.clear_worldgroupid();
        self.clear_creationsequence();
        self.clear_savegamefilename();
        self.clear_spawngroupparenthandle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SpawnGroup_Load {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SpawnGroup_Load {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SpawnGroup_ManifestUpdate {
    // message fields
    spawngrouphandle: ::std::option::Option<u32>,
    spawngroupmanifest: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    manifestincomplete: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SpawnGroup_ManifestUpdate {}

impl CNETMsg_SpawnGroup_ManifestUpdate {
    pub fn new() -> CNETMsg_SpawnGroup_ManifestUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SpawnGroup_ManifestUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SpawnGroup_ManifestUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SpawnGroup_ManifestUpdate,
        };
        unsafe {
            instance.get(CNETMsg_SpawnGroup_ManifestUpdate::new)
        }
    }

    // optional uint32 spawngrouphandle = 1;

    pub fn clear_spawngrouphandle(&mut self) {
        self.spawngrouphandle = ::std::option::Option::None;
    }

    pub fn has_spawngrouphandle(&self) -> bool {
        self.spawngrouphandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngrouphandle(&mut self, v: u32) {
        self.spawngrouphandle = ::std::option::Option::Some(v);
    }

    pub fn get_spawngrouphandle(&self) -> u32 {
        self.spawngrouphandle.unwrap_or(0)
    }

    fn get_spawngrouphandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawngrouphandle
    }

    fn mut_spawngrouphandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawngrouphandle
    }

    // optional bytes spawngroupmanifest = 2;

    pub fn clear_spawngroupmanifest(&mut self) {
        self.spawngroupmanifest.clear();
    }

    pub fn has_spawngroupmanifest(&self) -> bool {
        self.spawngroupmanifest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngroupmanifest(&mut self, v: ::std::vec::Vec<u8>) {
        self.spawngroupmanifest = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spawngroupmanifest(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.spawngroupmanifest.is_none() {
            self.spawngroupmanifest.set_default();
        };
        self.spawngroupmanifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_spawngroupmanifest(&mut self) -> ::std::vec::Vec<u8> {
        self.spawngroupmanifest.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_spawngroupmanifest(&self) -> &[u8] {
        match self.spawngroupmanifest.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_spawngroupmanifest_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.spawngroupmanifest
    }

    fn mut_spawngroupmanifest_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.spawngroupmanifest
    }

    // optional bool manifestincomplete = 3;

    pub fn clear_manifestincomplete(&mut self) {
        self.manifestincomplete = ::std::option::Option::None;
    }

    pub fn has_manifestincomplete(&self) -> bool {
        self.manifestincomplete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manifestincomplete(&mut self, v: bool) {
        self.manifestincomplete = ::std::option::Option::Some(v);
    }

    pub fn get_manifestincomplete(&self) -> bool {
        self.manifestincomplete.unwrap_or(false)
    }

    fn get_manifestincomplete_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.manifestincomplete
    }

    fn mut_manifestincomplete_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.manifestincomplete
    }
}

impl ::protobuf::Message for CNETMsg_SpawnGroup_ManifestUpdate {
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
                    self.spawngrouphandle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.spawngroupmanifest)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.manifestincomplete = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.spawngrouphandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.spawngroupmanifest.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.manifestincomplete {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.spawngrouphandle {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.spawngroupmanifest.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.manifestincomplete {
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

impl ::protobuf::MessageStatic for CNETMsg_SpawnGroup_ManifestUpdate {
    fn new() -> CNETMsg_SpawnGroup_ManifestUpdate {
        CNETMsg_SpawnGroup_ManifestUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SpawnGroup_ManifestUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawngrouphandle",
                    CNETMsg_SpawnGroup_ManifestUpdate::get_spawngrouphandle_for_reflect,
                    CNETMsg_SpawnGroup_ManifestUpdate::mut_spawngrouphandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "spawngroupmanifest",
                    CNETMsg_SpawnGroup_ManifestUpdate::get_spawngroupmanifest_for_reflect,
                    CNETMsg_SpawnGroup_ManifestUpdate::mut_spawngroupmanifest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "manifestincomplete",
                    CNETMsg_SpawnGroup_ManifestUpdate::get_manifestincomplete_for_reflect,
                    CNETMsg_SpawnGroup_ManifestUpdate::mut_manifestincomplete_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SpawnGroup_ManifestUpdate>(
                    "CNETMsg_SpawnGroup_ManifestUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SpawnGroup_ManifestUpdate {
    fn clear(&mut self) {
        self.clear_spawngrouphandle();
        self.clear_spawngroupmanifest();
        self.clear_manifestincomplete();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SpawnGroup_ManifestUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SpawnGroup_ManifestUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SpawnGroup_SetCreationTick {
    // message fields
    spawngrouphandle: ::std::option::Option<u32>,
    tickcount: ::std::option::Option<i32>,
    creationsequence: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SpawnGroup_SetCreationTick {}

impl CNETMsg_SpawnGroup_SetCreationTick {
    pub fn new() -> CNETMsg_SpawnGroup_SetCreationTick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SpawnGroup_SetCreationTick {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SpawnGroup_SetCreationTick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SpawnGroup_SetCreationTick,
        };
        unsafe {
            instance.get(CNETMsg_SpawnGroup_SetCreationTick::new)
        }
    }

    // optional uint32 spawngrouphandle = 1;

    pub fn clear_spawngrouphandle(&mut self) {
        self.spawngrouphandle = ::std::option::Option::None;
    }

    pub fn has_spawngrouphandle(&self) -> bool {
        self.spawngrouphandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngrouphandle(&mut self, v: u32) {
        self.spawngrouphandle = ::std::option::Option::Some(v);
    }

    pub fn get_spawngrouphandle(&self) -> u32 {
        self.spawngrouphandle.unwrap_or(0)
    }

    fn get_spawngrouphandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawngrouphandle
    }

    fn mut_spawngrouphandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawngrouphandle
    }

    // optional int32 tickcount = 2;

    pub fn clear_tickcount(&mut self) {
        self.tickcount = ::std::option::Option::None;
    }

    pub fn has_tickcount(&self) -> bool {
        self.tickcount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tickcount(&mut self, v: i32) {
        self.tickcount = ::std::option::Option::Some(v);
    }

    pub fn get_tickcount(&self) -> i32 {
        self.tickcount.unwrap_or(0)
    }

    fn get_tickcount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.tickcount
    }

    fn mut_tickcount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.tickcount
    }

    // optional uint32 creationsequence = 3;

    pub fn clear_creationsequence(&mut self) {
        self.creationsequence = ::std::option::Option::None;
    }

    pub fn has_creationsequence(&self) -> bool {
        self.creationsequence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creationsequence(&mut self, v: u32) {
        self.creationsequence = ::std::option::Option::Some(v);
    }

    pub fn get_creationsequence(&self) -> u32 {
        self.creationsequence.unwrap_or(0)
    }

    fn get_creationsequence_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.creationsequence
    }

    fn mut_creationsequence_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.creationsequence
    }
}

impl ::protobuf::Message for CNETMsg_SpawnGroup_SetCreationTick {
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
                    self.spawngrouphandle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.tickcount = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.creationsequence = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.spawngrouphandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tickcount {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.creationsequence {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.spawngrouphandle {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.tickcount {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.creationsequence {
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

impl ::protobuf::MessageStatic for CNETMsg_SpawnGroup_SetCreationTick {
    fn new() -> CNETMsg_SpawnGroup_SetCreationTick {
        CNETMsg_SpawnGroup_SetCreationTick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SpawnGroup_SetCreationTick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawngrouphandle",
                    CNETMsg_SpawnGroup_SetCreationTick::get_spawngrouphandle_for_reflect,
                    CNETMsg_SpawnGroup_SetCreationTick::mut_spawngrouphandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tickcount",
                    CNETMsg_SpawnGroup_SetCreationTick::get_tickcount_for_reflect,
                    CNETMsg_SpawnGroup_SetCreationTick::mut_tickcount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "creationsequence",
                    CNETMsg_SpawnGroup_SetCreationTick::get_creationsequence_for_reflect,
                    CNETMsg_SpawnGroup_SetCreationTick::mut_creationsequence_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SpawnGroup_SetCreationTick>(
                    "CNETMsg_SpawnGroup_SetCreationTick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SpawnGroup_SetCreationTick {
    fn clear(&mut self) {
        self.clear_spawngrouphandle();
        self.clear_tickcount();
        self.clear_creationsequence();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SpawnGroup_SetCreationTick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SpawnGroup_SetCreationTick {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SpawnGroup_Unload {
    // message fields
    spawngrouphandle: ::std::option::Option<u32>,
    flags: ::std::option::Option<u32>,
    tickcount: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SpawnGroup_Unload {}

impl CNETMsg_SpawnGroup_Unload {
    pub fn new() -> CNETMsg_SpawnGroup_Unload {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SpawnGroup_Unload {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SpawnGroup_Unload> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SpawnGroup_Unload,
        };
        unsafe {
            instance.get(CNETMsg_SpawnGroup_Unload::new)
        }
    }

    // optional uint32 spawngrouphandle = 1;

    pub fn clear_spawngrouphandle(&mut self) {
        self.spawngrouphandle = ::std::option::Option::None;
    }

    pub fn has_spawngrouphandle(&self) -> bool {
        self.spawngrouphandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngrouphandle(&mut self, v: u32) {
        self.spawngrouphandle = ::std::option::Option::Some(v);
    }

    pub fn get_spawngrouphandle(&self) -> u32 {
        self.spawngrouphandle.unwrap_or(0)
    }

    fn get_spawngrouphandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawngrouphandle
    }

    fn mut_spawngrouphandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawngrouphandle
    }

    // optional uint32 flags = 2;

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

    // optional int32 tickcount = 3;

    pub fn clear_tickcount(&mut self) {
        self.tickcount = ::std::option::Option::None;
    }

    pub fn has_tickcount(&self) -> bool {
        self.tickcount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tickcount(&mut self, v: i32) {
        self.tickcount = ::std::option::Option::Some(v);
    }

    pub fn get_tickcount(&self) -> i32 {
        self.tickcount.unwrap_or(0)
    }

    fn get_tickcount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.tickcount
    }

    fn mut_tickcount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.tickcount
    }
}

impl ::protobuf::Message for CNETMsg_SpawnGroup_Unload {
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
                    self.spawngrouphandle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.tickcount = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.spawngrouphandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tickcount {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.spawngrouphandle {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.flags {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.tickcount {
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

impl ::protobuf::MessageStatic for CNETMsg_SpawnGroup_Unload {
    fn new() -> CNETMsg_SpawnGroup_Unload {
        CNETMsg_SpawnGroup_Unload::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SpawnGroup_Unload>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawngrouphandle",
                    CNETMsg_SpawnGroup_Unload::get_spawngrouphandle_for_reflect,
                    CNETMsg_SpawnGroup_Unload::mut_spawngrouphandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flags",
                    CNETMsg_SpawnGroup_Unload::get_flags_for_reflect,
                    CNETMsg_SpawnGroup_Unload::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "tickcount",
                    CNETMsg_SpawnGroup_Unload::get_tickcount_for_reflect,
                    CNETMsg_SpawnGroup_Unload::mut_tickcount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SpawnGroup_Unload>(
                    "CNETMsg_SpawnGroup_Unload",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SpawnGroup_Unload {
    fn clear(&mut self) {
        self.clear_spawngrouphandle();
        self.clear_flags();
        self.clear_tickcount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SpawnGroup_Unload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SpawnGroup_Unload {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SpawnGroup_LoadCompleted {
    // message fields
    spawngrouphandle: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SpawnGroup_LoadCompleted {}

impl CNETMsg_SpawnGroup_LoadCompleted {
    pub fn new() -> CNETMsg_SpawnGroup_LoadCompleted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SpawnGroup_LoadCompleted {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SpawnGroup_LoadCompleted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SpawnGroup_LoadCompleted,
        };
        unsafe {
            instance.get(CNETMsg_SpawnGroup_LoadCompleted::new)
        }
    }

    // optional uint32 spawngrouphandle = 1;

    pub fn clear_spawngrouphandle(&mut self) {
        self.spawngrouphandle = ::std::option::Option::None;
    }

    pub fn has_spawngrouphandle(&self) -> bool {
        self.spawngrouphandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawngrouphandle(&mut self, v: u32) {
        self.spawngrouphandle = ::std::option::Option::Some(v);
    }

    pub fn get_spawngrouphandle(&self) -> u32 {
        self.spawngrouphandle.unwrap_or(0)
    }

    fn get_spawngrouphandle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawngrouphandle
    }

    fn mut_spawngrouphandle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawngrouphandle
    }
}

impl ::protobuf::Message for CNETMsg_SpawnGroup_LoadCompleted {
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
                    self.spawngrouphandle = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.spawngrouphandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.spawngrouphandle {
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

impl ::protobuf::MessageStatic for CNETMsg_SpawnGroup_LoadCompleted {
    fn new() -> CNETMsg_SpawnGroup_LoadCompleted {
        CNETMsg_SpawnGroup_LoadCompleted::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SpawnGroup_LoadCompleted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawngrouphandle",
                    CNETMsg_SpawnGroup_LoadCompleted::get_spawngrouphandle_for_reflect,
                    CNETMsg_SpawnGroup_LoadCompleted::mut_spawngrouphandle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SpawnGroup_LoadCompleted>(
                    "CNETMsg_SpawnGroup_LoadCompleted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SpawnGroup_LoadCompleted {
    fn clear(&mut self) {
        self.clear_spawngrouphandle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SpawnGroup_LoadCompleted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SpawnGroup_LoadCompleted {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameSessionConfiguration {
    // message fields
    is_multiplayer: ::std::option::Option<bool>,
    is_loadsavegame: ::std::option::Option<bool>,
    is_background_map: ::std::option::Option<bool>,
    is_headless: ::std::option::Option<bool>,
    min_client_limit: ::std::option::Option<u32>,
    max_client_limit: ::std::option::Option<u32>,
    max_clients: ::std::option::Option<u32>,
    tick_interval: ::std::option::Option<u32>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    savegamename: ::protobuf::SingularField<::std::string::String>,
    s1_mapname: ::protobuf::SingularField<::std::string::String>,
    gamemode: ::protobuf::SingularField<::std::string::String>,
    server_ip_address: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    is_localonly: ::std::option::Option<bool>,
    is_transition: ::std::option::Option<bool>,
    previouslevel: ::protobuf::SingularField<::std::string::String>,
    landmarkname: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameSessionConfiguration {}

impl CSVCMsg_GameSessionConfiguration {
    pub fn new() -> CSVCMsg_GameSessionConfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameSessionConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameSessionConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameSessionConfiguration,
        };
        unsafe {
            instance.get(CSVCMsg_GameSessionConfiguration::new)
        }
    }

    // optional bool is_multiplayer = 1;

    pub fn clear_is_multiplayer(&mut self) {
        self.is_multiplayer = ::std::option::Option::None;
    }

    pub fn has_is_multiplayer(&self) -> bool {
        self.is_multiplayer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_multiplayer(&mut self, v: bool) {
        self.is_multiplayer = ::std::option::Option::Some(v);
    }

    pub fn get_is_multiplayer(&self) -> bool {
        self.is_multiplayer.unwrap_or(false)
    }

    fn get_is_multiplayer_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_multiplayer
    }

    fn mut_is_multiplayer_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_multiplayer
    }

    // optional bool is_loadsavegame = 2;

    pub fn clear_is_loadsavegame(&mut self) {
        self.is_loadsavegame = ::std::option::Option::None;
    }

    pub fn has_is_loadsavegame(&self) -> bool {
        self.is_loadsavegame.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_loadsavegame(&mut self, v: bool) {
        self.is_loadsavegame = ::std::option::Option::Some(v);
    }

    pub fn get_is_loadsavegame(&self) -> bool {
        self.is_loadsavegame.unwrap_or(false)
    }

    fn get_is_loadsavegame_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_loadsavegame
    }

    fn mut_is_loadsavegame_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_loadsavegame
    }

    // optional bool is_background_map = 3;

    pub fn clear_is_background_map(&mut self) {
        self.is_background_map = ::std::option::Option::None;
    }

    pub fn has_is_background_map(&self) -> bool {
        self.is_background_map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_background_map(&mut self, v: bool) {
        self.is_background_map = ::std::option::Option::Some(v);
    }

    pub fn get_is_background_map(&self) -> bool {
        self.is_background_map.unwrap_or(false)
    }

    fn get_is_background_map_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_background_map
    }

    fn mut_is_background_map_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_background_map
    }

    // optional bool is_headless = 4;

    pub fn clear_is_headless(&mut self) {
        self.is_headless = ::std::option::Option::None;
    }

    pub fn has_is_headless(&self) -> bool {
        self.is_headless.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_headless(&mut self, v: bool) {
        self.is_headless = ::std::option::Option::Some(v);
    }

    pub fn get_is_headless(&self) -> bool {
        self.is_headless.unwrap_or(false)
    }

    fn get_is_headless_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_headless
    }

    fn mut_is_headless_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_headless
    }

    // optional uint32 min_client_limit = 5;

    pub fn clear_min_client_limit(&mut self) {
        self.min_client_limit = ::std::option::Option::None;
    }

    pub fn has_min_client_limit(&self) -> bool {
        self.min_client_limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_client_limit(&mut self, v: u32) {
        self.min_client_limit = ::std::option::Option::Some(v);
    }

    pub fn get_min_client_limit(&self) -> u32 {
        self.min_client_limit.unwrap_or(0)
    }

    fn get_min_client_limit_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.min_client_limit
    }

    fn mut_min_client_limit_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.min_client_limit
    }

    // optional uint32 max_client_limit = 6;

    pub fn clear_max_client_limit(&mut self) {
        self.max_client_limit = ::std::option::Option::None;
    }

    pub fn has_max_client_limit(&self) -> bool {
        self.max_client_limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_client_limit(&mut self, v: u32) {
        self.max_client_limit = ::std::option::Option::Some(v);
    }

    pub fn get_max_client_limit(&self) -> u32 {
        self.max_client_limit.unwrap_or(0)
    }

    fn get_max_client_limit_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_client_limit
    }

    fn mut_max_client_limit_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_client_limit
    }

    // optional uint32 max_clients = 7;

    pub fn clear_max_clients(&mut self) {
        self.max_clients = ::std::option::Option::None;
    }

    pub fn has_max_clients(&self) -> bool {
        self.max_clients.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_clients(&mut self, v: u32) {
        self.max_clients = ::std::option::Option::Some(v);
    }

    pub fn get_max_clients(&self) -> u32 {
        self.max_clients.unwrap_or(0)
    }

    fn get_max_clients_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_clients
    }

    fn mut_max_clients_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_clients
    }

    // optional fixed32 tick_interval = 8;

    pub fn clear_tick_interval(&mut self) {
        self.tick_interval = ::std::option::Option::None;
    }

    pub fn has_tick_interval(&self) -> bool {
        self.tick_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick_interval(&mut self, v: u32) {
        self.tick_interval = ::std::option::Option::Some(v);
    }

    pub fn get_tick_interval(&self) -> u32 {
        self.tick_interval.unwrap_or(0)
    }

    fn get_tick_interval_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tick_interval
    }

    fn mut_tick_interval_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tick_interval
    }

    // optional string hostname = 9;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostname
    }

    fn mut_hostname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostname
    }

    // optional string savegamename = 10;

    pub fn clear_savegamename(&mut self) {
        self.savegamename.clear();
    }

    pub fn has_savegamename(&self) -> bool {
        self.savegamename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_savegamename(&mut self, v: ::std::string::String) {
        self.savegamename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_savegamename(&mut self) -> &mut ::std::string::String {
        if self.savegamename.is_none() {
            self.savegamename.set_default();
        };
        self.savegamename.as_mut().unwrap()
    }

    // Take field
    pub fn take_savegamename(&mut self) -> ::std::string::String {
        self.savegamename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_savegamename(&self) -> &str {
        match self.savegamename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_savegamename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.savegamename
    }

    fn mut_savegamename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.savegamename
    }

    // optional string s1_mapname = 11;

    pub fn clear_s1_mapname(&mut self) {
        self.s1_mapname.clear();
    }

    pub fn has_s1_mapname(&self) -> bool {
        self.s1_mapname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s1_mapname(&mut self, v: ::std::string::String) {
        self.s1_mapname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s1_mapname(&mut self) -> &mut ::std::string::String {
        if self.s1_mapname.is_none() {
            self.s1_mapname.set_default();
        };
        self.s1_mapname.as_mut().unwrap()
    }

    // Take field
    pub fn take_s1_mapname(&mut self) -> ::std::string::String {
        self.s1_mapname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_s1_mapname(&self) -> &str {
        match self.s1_mapname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_s1_mapname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.s1_mapname
    }

    fn mut_s1_mapname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.s1_mapname
    }

    // optional string gamemode = 12;

    pub fn clear_gamemode(&mut self) {
        self.gamemode.clear();
    }

    pub fn has_gamemode(&self) -> bool {
        self.gamemode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gamemode(&mut self, v: ::std::string::String) {
        self.gamemode = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gamemode(&mut self) -> &mut ::std::string::String {
        if self.gamemode.is_none() {
            self.gamemode.set_default();
        };
        self.gamemode.as_mut().unwrap()
    }

    // Take field
    pub fn take_gamemode(&mut self) -> ::std::string::String {
        self.gamemode.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gamemode(&self) -> &str {
        match self.gamemode.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_gamemode_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.gamemode
    }

    fn mut_gamemode_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.gamemode
    }

    // optional string server_ip_address = 13;

    pub fn clear_server_ip_address(&mut self) {
        self.server_ip_address.clear();
    }

    pub fn has_server_ip_address(&self) -> bool {
        self.server_ip_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_ip_address(&mut self, v: ::std::string::String) {
        self.server_ip_address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_ip_address(&mut self) -> &mut ::std::string::String {
        if self.server_ip_address.is_none() {
            self.server_ip_address.set_default();
        };
        self.server_ip_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_ip_address(&mut self) -> ::std::string::String {
        self.server_ip_address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_server_ip_address(&self) -> &str {
        match self.server_ip_address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_server_ip_address_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.server_ip_address
    }

    fn mut_server_ip_address_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.server_ip_address
    }

    // optional bytes data = 14;

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

    // optional bool is_localonly = 15;

    pub fn clear_is_localonly(&mut self) {
        self.is_localonly = ::std::option::Option::None;
    }

    pub fn has_is_localonly(&self) -> bool {
        self.is_localonly.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_localonly(&mut self, v: bool) {
        self.is_localonly = ::std::option::Option::Some(v);
    }

    pub fn get_is_localonly(&self) -> bool {
        self.is_localonly.unwrap_or(false)
    }

    fn get_is_localonly_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_localonly
    }

    fn mut_is_localonly_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_localonly
    }

    // optional bool is_transition = 16;

    pub fn clear_is_transition(&mut self) {
        self.is_transition = ::std::option::Option::None;
    }

    pub fn has_is_transition(&self) -> bool {
        self.is_transition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_transition(&mut self, v: bool) {
        self.is_transition = ::std::option::Option::Some(v);
    }

    pub fn get_is_transition(&self) -> bool {
        self.is_transition.unwrap_or(false)
    }

    fn get_is_transition_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_transition
    }

    fn mut_is_transition_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_transition
    }

    // optional string previouslevel = 17;

    pub fn clear_previouslevel(&mut self) {
        self.previouslevel.clear();
    }

    pub fn has_previouslevel(&self) -> bool {
        self.previouslevel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previouslevel(&mut self, v: ::std::string::String) {
        self.previouslevel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previouslevel(&mut self) -> &mut ::std::string::String {
        if self.previouslevel.is_none() {
            self.previouslevel.set_default();
        };
        self.previouslevel.as_mut().unwrap()
    }

    // Take field
    pub fn take_previouslevel(&mut self) -> ::std::string::String {
        self.previouslevel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_previouslevel(&self) -> &str {
        match self.previouslevel.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_previouslevel_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.previouslevel
    }

    fn mut_previouslevel_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.previouslevel
    }

    // optional string landmarkname = 18;

    pub fn clear_landmarkname(&mut self) {
        self.landmarkname.clear();
    }

    pub fn has_landmarkname(&self) -> bool {
        self.landmarkname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_landmarkname(&mut self, v: ::std::string::String) {
        self.landmarkname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_landmarkname(&mut self) -> &mut ::std::string::String {
        if self.landmarkname.is_none() {
            self.landmarkname.set_default();
        };
        self.landmarkname.as_mut().unwrap()
    }

    // Take field
    pub fn take_landmarkname(&mut self) -> ::std::string::String {
        self.landmarkname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_landmarkname(&self) -> &str {
        match self.landmarkname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_landmarkname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.landmarkname
    }

    fn mut_landmarkname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.landmarkname
    }
}

impl ::protobuf::Message for CSVCMsg_GameSessionConfiguration {
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
                    self.is_multiplayer = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_loadsavegame = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_background_map = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_headless = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.min_client_limit = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_client_limit = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_clients = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.tick_interval = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.savegamename)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.s1_mapname)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gamemode)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.server_ip_address)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_localonly = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_transition = ::std::option::Option::Some(tmp);
                },
                17 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.previouslevel)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.landmarkname)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.is_multiplayer {
            my_size += 2;
        };
        if let Some(v) = self.is_loadsavegame {
            my_size += 2;
        };
        if let Some(v) = self.is_background_map {
            my_size += 2;
        };
        if let Some(v) = self.is_headless {
            my_size += 2;
        };
        if let Some(v) = self.min_client_limit {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.max_client_limit {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.max_clients {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tick_interval {
            my_size += 5;
        };
        if let Some(v) = self.hostname.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        };
        if let Some(v) = self.savegamename.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        };
        if let Some(v) = self.s1_mapname.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        };
        if let Some(v) = self.gamemode.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        };
        if let Some(v) = self.server_ip_address.as_ref() {
            my_size += ::protobuf::rt::string_size(13, &v);
        };
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(14, &v);
        };
        if let Some(v) = self.is_localonly {
            my_size += 2;
        };
        if let Some(v) = self.is_transition {
            my_size += 3;
        };
        if let Some(v) = self.previouslevel.as_ref() {
            my_size += ::protobuf::rt::string_size(17, &v);
        };
        if let Some(v) = self.landmarkname.as_ref() {
            my_size += ::protobuf::rt::string_size(18, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_multiplayer {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.is_loadsavegame {
            os.write_bool(2, v)?;
        };
        if let Some(v) = self.is_background_map {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.is_headless {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.min_client_limit {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.max_client_limit {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.max_clients {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.tick_interval {
            os.write_fixed32(8, v)?;
        };
        if let Some(v) = self.hostname.as_ref() {
            os.write_string(9, &v)?;
        };
        if let Some(v) = self.savegamename.as_ref() {
            os.write_string(10, &v)?;
        };
        if let Some(v) = self.s1_mapname.as_ref() {
            os.write_string(11, &v)?;
        };
        if let Some(v) = self.gamemode.as_ref() {
            os.write_string(12, &v)?;
        };
        if let Some(v) = self.server_ip_address.as_ref() {
            os.write_string(13, &v)?;
        };
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(14, &v)?;
        };
        if let Some(v) = self.is_localonly {
            os.write_bool(15, v)?;
        };
        if let Some(v) = self.is_transition {
            os.write_bool(16, v)?;
        };
        if let Some(v) = self.previouslevel.as_ref() {
            os.write_string(17, &v)?;
        };
        if let Some(v) = self.landmarkname.as_ref() {
            os.write_string(18, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CSVCMsg_GameSessionConfiguration {
    fn new() -> CSVCMsg_GameSessionConfiguration {
        CSVCMsg_GameSessionConfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameSessionConfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_multiplayer",
                    CSVCMsg_GameSessionConfiguration::get_is_multiplayer_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_is_multiplayer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_loadsavegame",
                    CSVCMsg_GameSessionConfiguration::get_is_loadsavegame_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_is_loadsavegame_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_background_map",
                    CSVCMsg_GameSessionConfiguration::get_is_background_map_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_is_background_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_headless",
                    CSVCMsg_GameSessionConfiguration::get_is_headless_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_is_headless_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "min_client_limit",
                    CSVCMsg_GameSessionConfiguration::get_min_client_limit_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_min_client_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_client_limit",
                    CSVCMsg_GameSessionConfiguration::get_max_client_limit_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_max_client_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_clients",
                    CSVCMsg_GameSessionConfiguration::get_max_clients_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_max_clients_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "tick_interval",
                    CSVCMsg_GameSessionConfiguration::get_tick_interval_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_tick_interval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostname",
                    CSVCMsg_GameSessionConfiguration::get_hostname_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_hostname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "savegamename",
                    CSVCMsg_GameSessionConfiguration::get_savegamename_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_savegamename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "s1_mapname",
                    CSVCMsg_GameSessionConfiguration::get_s1_mapname_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_s1_mapname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gamemode",
                    CSVCMsg_GameSessionConfiguration::get_gamemode_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_gamemode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "server_ip_address",
                    CSVCMsg_GameSessionConfiguration::get_server_ip_address_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_server_ip_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CSVCMsg_GameSessionConfiguration::get_data_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_localonly",
                    CSVCMsg_GameSessionConfiguration::get_is_localonly_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_is_localonly_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_transition",
                    CSVCMsg_GameSessionConfiguration::get_is_transition_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_is_transition_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "previouslevel",
                    CSVCMsg_GameSessionConfiguration::get_previouslevel_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_previouslevel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "landmarkname",
                    CSVCMsg_GameSessionConfiguration::get_landmarkname_for_reflect,
                    CSVCMsg_GameSessionConfiguration::mut_landmarkname_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameSessionConfiguration>(
                    "CSVCMsg_GameSessionConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameSessionConfiguration {
    fn clear(&mut self) {
        self.clear_is_multiplayer();
        self.clear_is_loadsavegame();
        self.clear_is_background_map();
        self.clear_is_headless();
        self.clear_min_client_limit();
        self.clear_max_client_limit();
        self.clear_max_clients();
        self.clear_tick_interval();
        self.clear_hostname();
        self.clear_savegamename();
        self.clear_s1_mapname();
        self.clear_gamemode();
        self.clear_server_ip_address();
        self.clear_data();
        self.clear_is_localonly();
        self.clear_is_transition();
        self.clear_previouslevel();
        self.clear_landmarkname();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameSessionConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameSessionConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NET_Messages {
    net_NOP = 0,
    net_Disconnect = 1,
    net_SplitScreenUser = 3,
    net_Tick = 4,
    net_StringCmd = 5,
    net_SetConVar = 6,
    net_SignonState = 7,
    net_SpawnGroup_Load = 8,
    net_SpawnGroup_ManifestUpdate = 9,
    net_SpawnGroup_SetCreationTick = 11,
    net_SpawnGroup_Unload = 12,
    net_SpawnGroup_LoadCompleted = 13,
}

impl ::protobuf::ProtobufEnum for NET_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NET_Messages> {
        match value {
            0 => ::std::option::Option::Some(NET_Messages::net_NOP),
            1 => ::std::option::Option::Some(NET_Messages::net_Disconnect),
            3 => ::std::option::Option::Some(NET_Messages::net_SplitScreenUser),
            4 => ::std::option::Option::Some(NET_Messages::net_Tick),
            5 => ::std::option::Option::Some(NET_Messages::net_StringCmd),
            6 => ::std::option::Option::Some(NET_Messages::net_SetConVar),
            7 => ::std::option::Option::Some(NET_Messages::net_SignonState),
            8 => ::std::option::Option::Some(NET_Messages::net_SpawnGroup_Load),
            9 => ::std::option::Option::Some(NET_Messages::net_SpawnGroup_ManifestUpdate),
            11 => ::std::option::Option::Some(NET_Messages::net_SpawnGroup_SetCreationTick),
            12 => ::std::option::Option::Some(NET_Messages::net_SpawnGroup_Unload),
            13 => ::std::option::Option::Some(NET_Messages::net_SpawnGroup_LoadCompleted),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NET_Messages] = &[
            NET_Messages::net_NOP,
            NET_Messages::net_Disconnect,
            NET_Messages::net_SplitScreenUser,
            NET_Messages::net_Tick,
            NET_Messages::net_StringCmd,
            NET_Messages::net_SetConVar,
            NET_Messages::net_SignonState,
            NET_Messages::net_SpawnGroup_Load,
            NET_Messages::net_SpawnGroup_ManifestUpdate,
            NET_Messages::net_SpawnGroup_SetCreationTick,
            NET_Messages::net_SpawnGroup_Unload,
            NET_Messages::net_SpawnGroup_LoadCompleted,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<NET_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NET_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NET_Messages {
}

impl ::protobuf::reflect::ProtobufValue for NET_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SpawnGroupFlags_t {
    SPAWN_GROUP_LOAD_ENTITIES_FROM_SAVE = 1,
    SPAWN_GROUP_DONT_SPAWN_ENTITIES = 2,
    SPAWN_GROUP_SYNCHRONOUS_SPAWN = 4,
    SPAWN_GROUP_IS_INITIAL_SPAWN_GROUP = 8,
    SPAWN_GROUP_CREATE_CLIENT_ONLY_ENTITIES = 16,
    SPAWN_GROUP_SAVE_ENTITIES = 32,
    SPAWN_GROUP_BLOCK_UNTIL_LOADED = 64,
    SPAWN_GROUP_LOAD_STREAMING_DATA = 128,
    SPAWN_GROUP_CREATE_NEW_SCENE_WORLD = 256,
}

impl ::protobuf::ProtobufEnum for SpawnGroupFlags_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SpawnGroupFlags_t> {
        match value {
            1 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_LOAD_ENTITIES_FROM_SAVE),
            2 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_DONT_SPAWN_ENTITIES),
            4 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_SYNCHRONOUS_SPAWN),
            8 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_IS_INITIAL_SPAWN_GROUP),
            16 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_CREATE_CLIENT_ONLY_ENTITIES),
            32 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_SAVE_ENTITIES),
            64 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_BLOCK_UNTIL_LOADED),
            128 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_LOAD_STREAMING_DATA),
            256 => ::std::option::Option::Some(SpawnGroupFlags_t::SPAWN_GROUP_CREATE_NEW_SCENE_WORLD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SpawnGroupFlags_t] = &[
            SpawnGroupFlags_t::SPAWN_GROUP_LOAD_ENTITIES_FROM_SAVE,
            SpawnGroupFlags_t::SPAWN_GROUP_DONT_SPAWN_ENTITIES,
            SpawnGroupFlags_t::SPAWN_GROUP_SYNCHRONOUS_SPAWN,
            SpawnGroupFlags_t::SPAWN_GROUP_IS_INITIAL_SPAWN_GROUP,
            SpawnGroupFlags_t::SPAWN_GROUP_CREATE_CLIENT_ONLY_ENTITIES,
            SpawnGroupFlags_t::SPAWN_GROUP_SAVE_ENTITIES,
            SpawnGroupFlags_t::SPAWN_GROUP_BLOCK_UNTIL_LOADED,
            SpawnGroupFlags_t::SPAWN_GROUP_LOAD_STREAMING_DATA,
            SpawnGroupFlags_t::SPAWN_GROUP_CREATE_NEW_SCENE_WORLD,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SpawnGroupFlags_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SpawnGroupFlags_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SpawnGroupFlags_t {
}

impl ::protobuf::reflect::ProtobufValue for SpawnGroupFlags_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x16, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x62, 0x61, 0x73, 0x65, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x18,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x36, 0x0a, 0x0a, 0x43, 0x4d, 0x73, 0x67,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x12, 0x0c, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x01, 0x78, 0x12, 0x0c, 0x0a, 0x01, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x01, 0x79, 0x12, 0x0c, 0x0a, 0x01, 0x7a, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01, 0x7a,
    0x22, 0x2a, 0x0a, 0x0c, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x32, 0x44,
    0x12, 0x0c, 0x0a, 0x01, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01, 0x78, 0x12, 0x0c,
    0x0a, 0x01, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01, 0x79, 0x22, 0x36, 0x0a, 0x0a,
    0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x12, 0x0c, 0x0a, 0x01, 0x78, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x01, 0x78, 0x12, 0x0c, 0x0a, 0x01, 0x79, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x01, 0x79, 0x12, 0x0c, 0x0a, 0x01, 0x7a, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x02, 0x52, 0x01, 0x7a, 0x22, 0xa2, 0x01, 0x0a, 0x0e, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x78,
    0x75, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x04, 0x78, 0x75, 0x69, 0x64, 0x12,
    0x16, 0x0a, 0x06, 0x75, 0x73, 0x65, 0x72, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x06, 0x75, 0x73, 0x65, 0x72, 0x69, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d,
    0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69,
    0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x66, 0x61, 0x6b, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x66, 0x61, 0x6b, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x12, 0x16, 0x0a, 0x06, 0x69, 0x73, 0x68, 0x6c, 0x74, 0x76, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x06, 0x69, 0x73, 0x68, 0x6c, 0x74, 0x76, 0x22, 0x6b, 0x0a, 0x0a, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x43, 0x56, 0x61, 0x72, 0x73, 0x12, 0x2b, 0x0a, 0x05, 0x63, 0x76, 0x61, 0x72, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x5f, 0x43, 0x56, 0x61, 0x72, 0x73, 0x2e, 0x43, 0x56, 0x61, 0x72, 0x52, 0x05, 0x63,
    0x76, 0x61, 0x72, 0x73, 0x1a, 0x30, 0x0a, 0x04, 0x43, 0x56, 0x61, 0x72, 0x12, 0x12, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x0d, 0x0a, 0x0b, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73,
    0x67, 0x5f, 0x4e, 0x4f, 0x50, 0x22, 0x2d, 0x0a, 0x17, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67,
    0x5f, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x55, 0x73, 0x65, 0x72,
    0x12, 0x12, 0x0a, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04,
    0x73, 0x6c, 0x6f, 0x74, 0x22, 0x6b, 0x0a, 0x12, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f,
    0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x12, 0x55, 0x0a, 0x06, 0x72, 0x65,
    0x61, 0x73, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x45, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x3a, 0x1a, 0x4e,
    0x45, 0x54, 0x57, 0x4f, 0x52, 0x4b, 0x5f, 0x44, 0x49, 0x53, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43,
    0x54, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x52, 0x06, 0x72, 0x65, 0x61, 0x73, 0x6f,
    0x6e, 0x22, 0xd5, 0x02, 0x0a, 0x0c, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x54, 0x69,
    0x63, 0x6b, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x12, 0x25, 0x0a, 0x0e, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d,
    0x68, 0x6f, 0x73, 0x74, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x3f, 0x0a,
    0x1c, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x5f,
    0x73, 0x74, 0x64, 0x5f, 0x64, 0x65, 0x76, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x19, 0x68, 0x6f, 0x73, 0x74, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x69,
    0x6d, 0x65, 0x53, 0x74, 0x64, 0x44, 0x65, 0x76, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x31,
    0x0a, 0x14, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x75, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x13, 0x68, 0x6f,
    0x73, 0x74, 0x43, 0x6f, 0x6d, 0x70, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x74, 0x69, 0x6d,
    0x65, 0x12, 0x4b, 0x0a, 0x22, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x75, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73, 0x74, 0x64, 0x5f, 0x64, 0x65,
    0x76, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1f, 0x68,
    0x6f, 0x73, 0x74, 0x43, 0x6f, 0x6d, 0x70, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x74, 0x69,
    0x6d, 0x65, 0x53, 0x74, 0x64, 0x44, 0x65, 0x76, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x49,
    0x0a, 0x21, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73, 0x74, 0x64, 0x5f, 0x64, 0x65, 0x76, 0x69, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1e, 0x68, 0x6f, 0x73, 0x74, 0x46,
    0x72, 0x61, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x74, 0x69, 0x6d, 0x65, 0x53, 0x74, 0x64,
    0x44, 0x65, 0x76, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x2d, 0x0a, 0x11, 0x43, 0x4e, 0x45,
    0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x43, 0x6d, 0x64, 0x12, 0x18,
    0x0a, 0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x22, 0x3f, 0x0a, 0x11, 0x43, 0x4e, 0x45, 0x54,
    0x4d, 0x73, 0x67, 0x5f, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x56, 0x61, 0x72, 0x12, 0x2a, 0x0a,
    0x07, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x72, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x56, 0x61, 0x72, 0x73,
    0x52, 0x07, 0x63, 0x6f, 0x6e, 0x76, 0x61, 0x72, 0x73, 0x22, 0xe9, 0x01, 0x0a, 0x13, 0x43, 0x4e,
    0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x69, 0x67, 0x6e, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x69, 0x67, 0x6e, 0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x73, 0x69, 0x67, 0x6e, 0x6f, 0x6e, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x5f, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x73, 0x70, 0x61, 0x77, 0x6e,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x2c, 0x0a, 0x12, 0x6e, 0x75, 0x6d, 0x5f, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x10, 0x6e, 0x75, 0x6d, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x73, 0x12, 0x2d, 0x0a, 0x12, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x5f, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x64, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52,
    0x11, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x69,
    0x64, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x61, 0x70, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x61, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a,
    0x06, 0x61, 0x64, 0x64, 0x6f, 0x6e, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61,
    0x64, 0x64, 0x6f, 0x6e, 0x73, 0x22, 0xe6, 0x02, 0x0a, 0x11, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x65,
    0x76, 0x65, 0x6e, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x09, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x69, 0x64, 0x12, 0x31, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73,
    0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x6b, 0x65, 0x79, 0x5f,
    0x74, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x1a, 0xe4, 0x01, 0x0a, 0x05, 0x6b, 0x65, 0x79, 0x5f,
    0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x76, 0x61, 0x6c, 0x5f, 0x73, 0x74, 0x72,
    0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x76, 0x61, 0x6c, 0x53, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x09, 0x76, 0x61, 0x6c, 0x5f, 0x66, 0x6c, 0x6f, 0x61,
    0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08, 0x76, 0x61, 0x6c, 0x46, 0x6c, 0x6f, 0x61,
    0x74, 0x12, 0x19, 0x0a, 0x08, 0x76, 0x61, 0x6c, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x07, 0x76, 0x61, 0x6c, 0x4c, 0x6f, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x09,
    0x76, 0x61, 0x6c, 0x5f, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x08, 0x76, 0x61, 0x6c, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x76, 0x61, 0x6c,
    0x5f, 0x62, 0x79, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x76, 0x61, 0x6c,
    0x42, 0x79, 0x74, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x76, 0x61, 0x6c, 0x5f, 0x62, 0x6f, 0x6f, 0x6c,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x76, 0x61, 0x6c, 0x42, 0x6f, 0x6f, 0x6c, 0x12,
    0x1d, 0x0a, 0x0a, 0x76, 0x61, 0x6c, 0x5f, 0x75, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x09, 0x76, 0x61, 0x6c, 0x55, 0x69, 0x6e, 0x74, 0x36, 0x34, 0x22, 0xa4,
    0x01, 0x0a, 0x16, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x69, 0x73, 0x74, 0x5f, 0x47,
    0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x3c, 0x0a, 0x06, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x69, 0x73, 0x74, 0x5f, 0x47, 0x61, 0x6d,
    0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x52,
    0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x1a, 0x4c, 0x0a, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x5f, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56,
    0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x05,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x22, 0x4b, 0x0a, 0x13, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67,
    0x5f, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x19, 0x0a, 0x08,
    0x6d, 0x73, 0x67, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07,
    0x6d, 0x73, 0x67, 0x54, 0x79, 0x70, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x73, 0x67, 0x5f, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x6d, 0x73, 0x67, 0x44, 0x61,
    0x74, 0x61, 0x22, 0xae, 0x01, 0x0a, 0x18, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x69,
    0x73, 0x74, 0x5f, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12,
    0x44, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x73, 0x67, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d, 0x73, 0x67,
    0x4c, 0x69, 0x73, 0x74, 0x5f, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x2e, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x73, 0x67, 0x5f, 0x74, 0x52, 0x08, 0x75, 0x73, 0x65,
    0x72, 0x6d, 0x73, 0x67, 0x73, 0x1a, 0x4c, 0x0a, 0x09, 0x75, 0x73, 0x65, 0x72, 0x6d, 0x73, 0x67,
    0x5f, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05,
    0x52, 0x04, 0x74, 0x69, 0x63, 0x6b, 0x12, 0x2b, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x56, 0x43, 0x4d,
    0x73, 0x67, 0x5f, 0x55, 0x73, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x03,
    0x6d, 0x73, 0x67, 0x22, 0xb7, 0x06, 0x0a, 0x17, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f,
    0x53, 0x70, 0x61, 0x77, 0x6e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x4c, 0x6f, 0x61, 0x64, 0x12,
    0x1c, 0x0a, 0x09, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x09, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a,
    0x0e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x6c, 0x75, 0x6d, 0x70, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x6c, 0x75, 0x6d,
    0x70, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2a, 0x0a, 0x10, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x10, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6d,
    0x65, 0x12, 0x2a, 0x0a, 0x10, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x68,
    0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x73, 0x70, 0x61,
    0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x12, 0x34, 0x0a,
    0x15, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x6f, 0x77, 0x6e, 0x65, 0x72,
    0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x15, 0x73, 0x70,
    0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x68, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x12, 0x3a, 0x0a, 0x10, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x6f, 0x66, 0x66,
    0x73, 0x65, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52,
    0x0e, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x12,
    0x3e, 0x0a, 0x12, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x5f,
    0x61, 0x6e, 0x67, 0x6c, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x52, 0x10, 0x77,
    0x6f, 0x72, 0x6c, 0x64, 0x4f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x41, 0x6e, 0x67, 0x6c, 0x65, 0x12,
    0x2e, 0x0a, 0x12, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x6d, 0x61, 0x6e,
    0x69, 0x66, 0x65, 0x73, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x12, 0x73, 0x70, 0x61,
    0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x6d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x12,
    0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05,
    0x66, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x63, 0x6b, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x74, 0x69, 0x63, 0x6b, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x12, 0x2e, 0x0a, 0x12, 0x6d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x69,
    0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x12, 0x6d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x6c,
    0x65, 0x74, 0x65, 0x12, 0x26, 0x0a, 0x0e, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x6e, 0x61, 0x6d, 0x65,
    0x66, 0x69, 0x78, 0x75, 0x70, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x6c, 0x6f, 0x63,
    0x61, 0x6c, 0x6e, 0x61, 0x6d, 0x65, 0x66, 0x69, 0x78, 0x75, 0x70, 0x12, 0x28, 0x0a, 0x0f, 0x70,
    0x61, 0x72, 0x65, 0x6e, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x66, 0x69, 0x78, 0x75, 0x70, 0x18, 0x0d,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x6e, 0x61, 0x6d, 0x65,
    0x66, 0x69, 0x78, 0x75, 0x70, 0x12, 0x32, 0x0a, 0x14, 0x6d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73,
    0x74, 0x6c, 0x6f, 0x61, 0x64, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x14, 0x6d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x6c, 0x6f, 0x61,
    0x64, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x12, 0x22, 0x0a, 0x0c, 0x77, 0x6f, 0x72,
    0x6c, 0x64, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x69, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0c, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x69, 0x64, 0x12, 0x2a, 0x0a,
    0x10, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63,
    0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x2a, 0x0a, 0x10, 0x73, 0x61, 0x76,
    0x65, 0x67, 0x61, 0x6d, 0x65, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x11, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x10, 0x73, 0x61, 0x76, 0x65, 0x67, 0x61, 0x6d, 0x65, 0x66, 0x69, 0x6c,
    0x65, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x36, 0x0a, 0x16, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72,
    0x6f, 0x75, 0x70, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18,
    0x12, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x16, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75,
    0x70, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x22, 0xaf, 0x01,
    0x0a, 0x21, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x47,
    0x72, 0x6f, 0x75, 0x70, 0x5f, 0x4d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x12, 0x2a, 0x0a, 0x10, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75,
    0x70, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x73,
    0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x12,
    0x2e, 0x0a, 0x12, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x6d, 0x61, 0x6e,
    0x69, 0x66, 0x65, 0x73, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x12, 0x73, 0x70, 0x61,
    0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x6d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x12,
    0x2e, 0x0a, 0x12, 0x6d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x63, 0x6f, 0x6d,
    0x70, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x6d, 0x61, 0x6e,
    0x69, 0x66, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x22,
    0x9a, 0x01, 0x0a, 0x22, 0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x61, 0x77,
    0x6e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x53, 0x65, 0x74, 0x43, 0x72, 0x65, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x54, 0x69, 0x63, 0x6b, 0x12, 0x2a, 0x0a, 0x10, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67,
    0x72, 0x6f, 0x75, 0x70, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x10, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x68, 0x61, 0x6e, 0x64,
    0x6c, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x63, 0x6b, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09, 0x74, 0x69, 0x63, 0x6b, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x12, 0x2a, 0x0a, 0x10, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75,
    0x65, 0x6e, 0x63, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x63, 0x72, 0x65, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x22, 0x7b, 0x0a, 0x19,
    0x43, 0x4e, 0x45, 0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x47, 0x72, 0x6f,
    0x75, 0x70, 0x5f, 0x55, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x12, 0x2a, 0x0a, 0x10, 0x73, 0x70, 0x61,
    0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x10, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x68,
    0x61, 0x6e, 0x64, 0x6c, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x1c, 0x0a, 0x09, 0x74,
    0x69, 0x63, 0x6b, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x09,
    0x74, 0x69, 0x63, 0x6b, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x4e, 0x0a, 0x20, 0x43, 0x4e, 0x45,
    0x54, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x5f,
    0x4c, 0x6f, 0x61, 0x64, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x12, 0x2a, 0x0a,
    0x10, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x68, 0x61, 0x6e, 0x64, 0x6c,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x73, 0x70, 0x61, 0x77, 0x6e, 0x67, 0x72,
    0x6f, 0x75, 0x70, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x22, 0xa6, 0x05, 0x0a, 0x20, 0x43, 0x53,
    0x56, 0x43, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x25,
    0x0a, 0x0e, 0x69, 0x73, 0x5f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x69, 0x73, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x12, 0x27, 0x0a, 0x0f, 0x69, 0x73, 0x5f, 0x6c, 0x6f, 0x61, 0x64,
    0x73, 0x61, 0x76, 0x65, 0x67, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e,
    0x69, 0x73, 0x4c, 0x6f, 0x61, 0x64, 0x73, 0x61, 0x76, 0x65, 0x67, 0x61, 0x6d, 0x65, 0x12, 0x2a,
    0x0a, 0x11, 0x69, 0x73, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x67, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x5f,
    0x6d, 0x61, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x69, 0x73, 0x42, 0x61, 0x63,
    0x6b, 0x67, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x4d, 0x61, 0x70, 0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x73,
    0x5f, 0x68, 0x65, 0x61, 0x64, 0x6c, 0x65, 0x73, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x0a, 0x69, 0x73, 0x48, 0x65, 0x61, 0x64, 0x6c, 0x65, 0x73, 0x73, 0x12, 0x28, 0x0a, 0x10, 0x6d,
    0x69, 0x6e, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x6d, 0x69, 0x6e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x28, 0x0a, 0x10, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0e, 0x6d, 0x61, 0x78, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12,
    0x1f, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x6d, 0x61, 0x78, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73,
    0x12, 0x23, 0x0a, 0x0d, 0x74, 0x69, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61,
    0x6c, 0x18, 0x08, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x74, 0x69, 0x63, 0x6b, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x76, 0x61, 0x6c, 0x12, 0x1a, 0x0a, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d,
    0x65, 0x12, 0x22, 0x0a, 0x0c, 0x73, 0x61, 0x76, 0x65, 0x67, 0x61, 0x6d, 0x65, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x73, 0x61, 0x76, 0x65, 0x67, 0x61, 0x6d,
    0x65, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x31, 0x5f, 0x6d, 0x61, 0x70, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x31, 0x4d, 0x61, 0x70,
    0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x6d, 0x6f, 0x64, 0x65,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x6d, 0x6f, 0x64, 0x65,
    0x12, 0x2a, 0x0a, 0x11, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x69, 0x70, 0x5f, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x49, 0x70, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x12, 0x0a, 0x04,
    0x64, 0x61, 0x74, 0x61, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61,
    0x12, 0x21, 0x0a, 0x0c, 0x69, 0x73, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x6f, 0x6e, 0x6c, 0x79,
    0x18, 0x0f, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x69, 0x73, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x6f,
    0x6e, 0x6c, 0x79, 0x12, 0x23, 0x0a, 0x0d, 0x69, 0x73, 0x5f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x18, 0x10, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x69, 0x73, 0x54, 0x72,
    0x61, 0x6e, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x0d, 0x70, 0x72, 0x65, 0x76,
    0x69, 0x6f, 0x75, 0x73, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x11, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0d, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x22,
    0x0a, 0x0c, 0x6c, 0x61, 0x6e, 0x64, 0x6d, 0x61, 0x72, 0x6b, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x12,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x6c, 0x61, 0x6e, 0x64, 0x6d, 0x61, 0x72, 0x6b, 0x6e, 0x61,
    0x6d, 0x65, 0x2a, 0xae, 0x02, 0x0a, 0x0c, 0x4e, 0x45, 0x54, 0x5f, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x73, 0x12, 0x0b, 0x0a, 0x07, 0x6e, 0x65, 0x74, 0x5f, 0x4e, 0x4f, 0x50, 0x10, 0x00,
    0x12, 0x12, 0x0a, 0x0e, 0x6e, 0x65, 0x74, 0x5f, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x70, 0x6c, 0x69,
    0x74, 0x53, 0x63, 0x72, 0x65, 0x65, 0x6e, 0x55, 0x73, 0x65, 0x72, 0x10, 0x03, 0x12, 0x0c, 0x0a,
    0x08, 0x6e, 0x65, 0x74, 0x5f, 0x54, 0x69, 0x63, 0x6b, 0x10, 0x04, 0x12, 0x11, 0x0a, 0x0d, 0x6e,
    0x65, 0x74, 0x5f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x43, 0x6d, 0x64, 0x10, 0x05, 0x12, 0x11,
    0x0a, 0x0d, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x56, 0x61, 0x72, 0x10,
    0x06, 0x12, 0x13, 0x0a, 0x0f, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x69, 0x67, 0x6e, 0x6f, 0x6e, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x10, 0x07, 0x12, 0x17, 0x0a, 0x13, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x70,
    0x61, 0x77, 0x6e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x4c, 0x6f, 0x61, 0x64, 0x10, 0x08, 0x12,
    0x21, 0x0a, 0x1d, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x47, 0x72, 0x6f, 0x75,
    0x70, 0x5f, 0x4d, 0x61, 0x6e, 0x69, 0x66, 0x65, 0x73, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x10, 0x09, 0x12, 0x22, 0x0a, 0x1e, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x47,
    0x72, 0x6f, 0x75, 0x70, 0x5f, 0x53, 0x65, 0x74, 0x43, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x54, 0x69, 0x63, 0x6b, 0x10, 0x0b, 0x12, 0x19, 0x0a, 0x15, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x70,
    0x61, 0x77, 0x6e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x55, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x10,
    0x0c, 0x12, 0x20, 0x0a, 0x1c, 0x6e, 0x65, 0x74, 0x5f, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x47, 0x72,
    0x6f, 0x75, 0x70, 0x5f, 0x4c, 0x6f, 0x61, 0x64, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65,
    0x64, 0x10, 0x0d, 0x2a, 0xeb, 0x02, 0x0a, 0x11, 0x53, 0x70, 0x61, 0x77, 0x6e, 0x47, 0x72, 0x6f,
    0x75, 0x70, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x5f, 0x74, 0x12, 0x27, 0x0a, 0x23, 0x53, 0x50, 0x41,
    0x57, 0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55, 0x50, 0x5f, 0x4c, 0x4f, 0x41, 0x44, 0x5f, 0x45, 0x4e,
    0x54, 0x49, 0x54, 0x49, 0x45, 0x53, 0x5f, 0x46, 0x52, 0x4f, 0x4d, 0x5f, 0x53, 0x41, 0x56, 0x45,
    0x10, 0x01, 0x12, 0x23, 0x0a, 0x1f, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55,
    0x50, 0x5f, 0x44, 0x4f, 0x4e, 0x54, 0x5f, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x5f, 0x45, 0x4e, 0x54,
    0x49, 0x54, 0x49, 0x45, 0x53, 0x10, 0x02, 0x12, 0x21, 0x0a, 0x1d, 0x53, 0x50, 0x41, 0x57, 0x4e,
    0x5f, 0x47, 0x52, 0x4f, 0x55, 0x50, 0x5f, 0x53, 0x59, 0x4e, 0x43, 0x48, 0x52, 0x4f, 0x4e, 0x4f,
    0x55, 0x53, 0x5f, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x10, 0x04, 0x12, 0x26, 0x0a, 0x22, 0x53, 0x50,
    0x41, 0x57, 0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55, 0x50, 0x5f, 0x49, 0x53, 0x5f, 0x49, 0x4e, 0x49,
    0x54, 0x49, 0x41, 0x4c, 0x5f, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55, 0x50,
    0x10, 0x08, 0x12, 0x2b, 0x0a, 0x27, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55,
    0x50, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x5f, 0x43, 0x4c, 0x49, 0x45, 0x4e, 0x54, 0x5f,
    0x4f, 0x4e, 0x4c, 0x59, 0x5f, 0x45, 0x4e, 0x54, 0x49, 0x54, 0x49, 0x45, 0x53, 0x10, 0x10, 0x12,
    0x1d, 0x0a, 0x19, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55, 0x50, 0x5f, 0x53,
    0x41, 0x56, 0x45, 0x5f, 0x45, 0x4e, 0x54, 0x49, 0x54, 0x49, 0x45, 0x53, 0x10, 0x20, 0x12, 0x22,
    0x0a, 0x1e, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55, 0x50, 0x5f, 0x42, 0x4c,
    0x4f, 0x43, 0x4b, 0x5f, 0x55, 0x4e, 0x54, 0x49, 0x4c, 0x5f, 0x4c, 0x4f, 0x41, 0x44, 0x45, 0x44,
    0x10, 0x40, 0x12, 0x24, 0x0a, 0x1f, 0x53, 0x50, 0x41, 0x57, 0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55,
    0x50, 0x5f, 0x4c, 0x4f, 0x41, 0x44, 0x5f, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4d, 0x49, 0x4e, 0x47,
    0x5f, 0x44, 0x41, 0x54, 0x41, 0x10, 0x80, 0x01, 0x12, 0x27, 0x0a, 0x22, 0x53, 0x50, 0x41, 0x57,
    0x4e, 0x5f, 0x47, 0x52, 0x4f, 0x55, 0x50, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x5f, 0x4e,
    0x45, 0x57, 0x5f, 0x53, 0x43, 0x45, 0x4e, 0x45, 0x5f, 0x57, 0x4f, 0x52, 0x4c, 0x44, 0x10, 0x80,
    0x02, 0x42, 0x03, 0x80, 0x01, 0x00, 0x4a, 0xba, 0x45, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xd1,
    0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07,
    0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x06, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x06, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x06, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x06, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x06, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x08, 0x00, 0x15, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x08, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x09, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x09, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x08,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0b, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x0b, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03,
    0x0c, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0c, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0c, 0x13, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x0d, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x0e, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x0e, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0e, 0x18,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x08, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x0f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x07, 0x12, 0x03, 0x10, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x10, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03,
    0x10, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x11, 0x08, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x11, 0x08, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x11, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x12, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x09, 0x01, 0x12, 0x03, 0x12, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02,
    0x12, 0x03, 0x12, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x13,
    0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x13, 0x08, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x13, 0x20, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x14, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x14, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x0b, 0x02, 0x12, 0x03, 0x14, 0x27, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x17,
    0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x17, 0x05, 0x16, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x18, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x18, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x19, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x19, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x19, 0x2a,
    0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x08, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x03, 0x12, 0x03, 0x1b, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x1b, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x1b, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12, 0x03, 0x1c, 0x08, 0x35,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x02, 0x12, 0x03, 0x1c, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x02,
    0x12, 0x03, 0x1d, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x06, 0x12, 0x03, 0x1e,
    0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06, 0x02, 0x12, 0x03, 0x1e, 0x29, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x07, 0x12, 0x03, 0x1f, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x07, 0x02, 0x12, 0x03, 0x1f, 0x2a, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x08, 0x12,
    0x03, 0x20, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x20,
    0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x08, 0x02, 0x12, 0x03, 0x20, 0x2d, 0x30,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x23, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x24, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x17, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x25, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x25, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x25, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x25, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25,
    0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x26, 0x08, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x26, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x26, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x29,
    0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x29, 0x08, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x2a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x2a, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x2a, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2b, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2b, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2b, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x2e, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x08,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x08, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2f, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x30, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x30, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x30, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x30, 0x17, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x31, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x31, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x31,
    0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x31, 0x1b, 0x1c,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x34, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x34, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x35, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x35, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x35, 0x18, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x35, 0x1f, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x36, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x36, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x36, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x36, 0x19, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36,
    0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x37, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x37, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x37, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12,
    0x03, 0x38, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x38,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x38, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x38, 0x19, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x38, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x39, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x39, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x39, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x39, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x3a, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x04, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x05, 0x05, 0x12, 0x03, 0x3a, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x3a, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x3a, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x3d, 0x00,
    0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x12, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x3e, 0x08, 0x41, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3f, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3f, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x40, 0x10, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x40, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x40, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x40, 0x20, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x43, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x43, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x43, 0x11,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x21, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x29, 0x2a, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x46, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x46, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x49, 0x00, 0x4b,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x49, 0x08, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x4a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4a, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4a,
    0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x4d, 0x00, 0x4f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x00, 0x12, 0x03, 0x4e, 0x08, 0x5f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x4e, 0x11, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x2d,
    0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x36, 0x37, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x08, 0x12, 0x03, 0x4e, 0x38, 0x5e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x07, 0x12, 0x03, 0x4e, 0x43, 0x5d, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x08, 0x12, 0x04, 0x51, 0x00, 0x58, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03,
    0x51, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x52, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x52, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01,
    0x12, 0x03, 0x53, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x53, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x53, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x53, 0x18, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x53, 0x29, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x54, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x54, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x54, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x54, 0x18, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x54,
    0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x55, 0x08, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x55, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x55, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x55, 0x18, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x55, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12,
    0x03, 0x56, 0x08, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x56,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x03, 0x56, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x56, 0x18, 0x3a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x56, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x05, 0x12, 0x03, 0x57, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x57, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x57, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x57, 0x18, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x57, 0x3c,
    0x3d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x5a, 0x00, 0x5c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x03, 0x5b, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x5b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5b,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x18, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5b, 0x22, 0x23, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x5e, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x03, 0x5e, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03,
    0x5f, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5f, 0x11, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x1c, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5f, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0b, 0x12, 0x04, 0x62, 0x00, 0x69, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03,
    0x62, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x63, 0x08, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x63, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01,
    0x12, 0x03, 0x64, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x64, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x64, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x64, 0x18, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x64, 0x26, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x65, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x65, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x65, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x65, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x65,
    0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x66, 0x08, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x03, 0x66, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x03, 0x66, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x03, 0x01, 0x12, 0x03, 0x66, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x66, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12,
    0x03, 0x67, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x03, 0x67,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x03, 0x67, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x03, 0x67, 0x18, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x03, 0x67, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x05, 0x12, 0x03, 0x68, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x68, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x68, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x68, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12, 0x03, 0x68, 0x21,
    0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x6b, 0x00, 0x7a, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x6b, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x03,
    0x00, 0x12, 0x04, 0x6c, 0x08, 0x75, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x6c, 0x10, 0x15, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x6d, 0x10, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x6d, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x6d, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x6d, 0x1f, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x6d, 0x26, 0x27, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x6e, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x6e, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x6e, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x6e, 0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x6e, 0x2d, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x6f,
    0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6f,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6f,
    0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6f,
    0x1f, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6f,
    0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x70, 0x10,
    0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x70, 0x10,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x70, 0x19,
    0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x70, 0x1f,
    0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x70, 0x2a,
    0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x71, 0x10, 0x2d,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x71, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x71, 0x19, 0x1e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x71, 0x1f, 0x28,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x71, 0x2b, 0x2c,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x05, 0x12, 0x03, 0x72, 0x10, 0x2c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x72, 0x10, 0x18, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x72, 0x19, 0x1e, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x72, 0x1f, 0x27, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x72, 0x2a, 0x2b, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x06, 0x12, 0x03, 0x73, 0x10, 0x2b, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x73, 0x10, 0x18, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x73, 0x19, 0x1d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x73, 0x1e, 0x26, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x73, 0x29, 0x2a, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x07, 0x12, 0x03, 0x74, 0x10, 0x2f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x74, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x74, 0x19, 0x1f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x74, 0x20, 0x2a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x74, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x77, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x77, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x77, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x77, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x78, 0x08, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x78, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x78, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x78, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x78, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12,
    0x03, 0x79, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x79,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x03, 0x79, 0x11, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x79, 0x29, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x79, 0x30, 0x31, 0x0a, 0x0b, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x05, 0x7c, 0x00, 0x83, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01,
    0x12, 0x03, 0x7c, 0x08, 0x1e, 0x0a, 0x0d, 0x0a, 0x04, 0x04, 0x0d, 0x03, 0x00, 0x12, 0x05, 0x7d,
    0x08, 0x80, 0x01, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x03, 0x00, 0x01, 0x12, 0x03, 0x7d,
    0x10, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x7e, 0x10,
    0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7e, 0x10,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7e, 0x19,
    0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7e, 0x1f,
    0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7e, 0x26,
    0x27, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x7f, 0x10, 0x35,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7f, 0x10, 0x18,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x7f, 0x19, 0x2a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7f, 0x2b, 0x30,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0d, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7f, 0x33, 0x34,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0x82, 0x01, 0x08, 0x3b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0x82, 0x01, 0x11, 0x2f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x01, 0x30, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x01, 0x39, 0x3a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e,
    0x12, 0x06, 0x85, 0x01, 0x00, 0x88, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12,
    0x04, 0x85, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0x86,
    0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x17, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0x87, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x04, 0x87, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0x87, 0x01, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x87, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12,
    0x06, 0x8a, 0x01, 0x00, 0x91, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04,
    0x8a, 0x01, 0x08, 0x20, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0f, 0x03, 0x00, 0x12, 0x06, 0x8b, 0x01,
    0x08, 0x8e, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x03, 0x00, 0x01, 0x12, 0x04, 0x8b,
    0x01, 0x10, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8c,
    0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x8c, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x04, 0x8c, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x8c, 0x01, 0x1f, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x8c, 0x01, 0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x01, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x35, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x8d, 0x01, 0x19, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x2d, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x00, 0x12, 0x04, 0x90, 0x01, 0x08, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x90, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x90, 0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x90, 0x01, 0x34, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x90, 0x01, 0x3f, 0x40, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x93, 0x01, 0x00,
    0xa6, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x93, 0x01, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0x94, 0x01, 0x08, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04, 0x94, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x01, 0x12, 0x04, 0x95, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x95, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x95, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x95, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x95, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0x96, 0x01,
    0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0x96, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x05, 0x12, 0x04, 0x96, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x18, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0x96, 0x01, 0x2b, 0x2c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0x97, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x04, 0x97, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x03, 0x05, 0x12, 0x04, 0x97, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x97, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x97, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04,
    0x12, 0x04, 0x98, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x98, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x98, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x01, 0x12, 0x04, 0x98,
    0x01, 0x18, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x03, 0x12, 0x04, 0x98, 0x01,
    0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x05, 0x12, 0x04, 0x99, 0x01, 0x08, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x06, 0x12, 0x04, 0x99, 0x01, 0x11, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x01, 0x12, 0x04, 0x99, 0x01, 0x1c, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x05, 0x03, 0x12, 0x04, 0x99, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x06, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x06, 0x06, 0x12, 0x04, 0x9a, 0x01, 0x11, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06,
    0x01, 0x12, 0x04, 0x9a, 0x01, 0x1c, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x06, 0x03,
    0x12, 0x04, 0x9a, 0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x07, 0x12, 0x04,
    0x9b, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07, 0x04, 0x12, 0x04, 0x9b,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07, 0x05, 0x12, 0x04, 0x9b, 0x01,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x17,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x07, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x2c, 0x2d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x08, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x08, 0x04, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x08, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x08, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x08, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x09, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x09,
    0x04, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x09, 0x05,
    0x12, 0x04, 0x9d, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x09, 0x01, 0x12,
    0x04, 0x9d, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x09, 0x03, 0x12, 0x04,
    0x9d, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x0a, 0x12, 0x04, 0x9e, 0x01,
    0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0a, 0x05, 0x12, 0x04, 0x9e, 0x01, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x16, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x2b, 0x2d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x0b, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x0b, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x0b, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x0b, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x0c,
    0x12, 0x04, 0xa0, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0c, 0x04, 0x12,
    0x04, 0xa0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0c, 0x05, 0x12, 0x04,
    0xa0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xa0,
    0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xa0, 0x01,
    0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x0d, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0d, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x17, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x0e, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x0e, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x0e, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0e,
    0x01, 0x12, 0x04, 0xa2, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0e, 0x03,
    0x12, 0x04, 0xa2, 0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x0f, 0x12, 0x04,
    0xa3, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0f, 0x04, 0x12, 0x04, 0xa3,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0f, 0x05, 0x12, 0x04, 0xa3, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x18,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x2b, 0x2d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x10, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x2e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x10, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x10, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x10, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x10, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x11, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x11,
    0x04, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x11, 0x05,
    0x12, 0x04, 0xa5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x11, 0x01, 0x12,
    0x04, 0xa5, 0x01, 0x18, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x11, 0x03, 0x12, 0x04,
    0xa5, 0x01, 0x31, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xa8, 0x01, 0x00, 0xac,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x01, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xaa, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xaa, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xaa, 0x01, 0x17, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa,
    0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xab, 0x01, 0x08,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xab, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0xab, 0x01, 0x11, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xab, 0x01, 0x16, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0xab, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x12, 0x12, 0x06, 0xae, 0x01, 0x00, 0xb2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x12, 0x01, 0x12, 0x04, 0xae, 0x01, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00,
    0x12, 0x04, 0xaf, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xaf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xaf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaf,
    0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaf, 0x01,
    0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x11, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x17, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xb1, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xb1, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xb4, 0x01,
    0x00, 0xb8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x2d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb5, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb5, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb5, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xb6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb6, 0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xb6, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0xb7,
    0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb7, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x11,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x17, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xba, 0x01, 0x00, 0xbc, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x00, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xbb, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xbb, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xbb, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xbe, 0x01, 0x00, 0xd1,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x16, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x01, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xc0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xc0, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xc0, 0x01, 0x16, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc0,
    0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xc1, 0x01, 0x08,
    0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0xc1, 0x01, 0x11, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x16, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xc2, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xc2, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12,
    0x04, 0xc3, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xc3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x05, 0x12, 0x04, 0xc3,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01, 0x12, 0x04, 0xc3, 0x01,
    0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x2b,
    0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x05, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x2d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x05, 0x12, 0x04, 0xc4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x05, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x05, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x06, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x06, 0x04, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06,
    0x05, 0x12, 0x04, 0xc5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x01,
    0x12, 0x04, 0xc5, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x03, 0x12,
    0x04, 0xc5, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x07, 0x12, 0x04, 0xc6,
    0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x04, 0x12, 0x04, 0xc6, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x19, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x29, 0x2a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x08, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x08, 0x04, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x08, 0x05, 0x12, 0x04, 0xc7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x08, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x08, 0x03, 0x12, 0x04, 0xc7, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x09, 0x12, 0x04, 0xc8, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x04,
    0x12, 0x04, 0xc8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x05, 0x12,
    0x04, 0xc8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x01, 0x12, 0x04,
    0xc8, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x03, 0x12, 0x04, 0xc8,
    0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x0a, 0x12, 0x04, 0xc9, 0x01, 0x08,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x18, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x0b, 0x12, 0x04, 0xca, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xca, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x0b, 0x05, 0x12, 0x04, 0xca, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x0b, 0x01, 0x12, 0x04, 0xca, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0b,
    0x03, 0x12, 0x04, 0xca, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x0c, 0x12,
    0x04, 0xcb, 0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0c, 0x04, 0x12, 0x04,
    0xcb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0c, 0x05, 0x12, 0x04, 0xcb,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xcb, 0x01,
    0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xcb, 0x01, 0x2c,
    0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x0d, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x0d, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x17, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x1e, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x0e, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x0e, 0x04, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0e,
    0x05, 0x12, 0x04, 0xcd, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0e, 0x01,
    0x12, 0x04, 0xcd, 0x01, 0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0e, 0x03, 0x12,
    0x04, 0xcd, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x0f, 0x12, 0x04, 0xce,
    0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0f, 0x04, 0x12, 0x04, 0xce, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0f, 0x05, 0x12, 0x04, 0xce, 0x01, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xce, 0x01, 0x16, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xce, 0x01, 0x26, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x10, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x10, 0x04, 0x12, 0x04, 0xcf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x10, 0x05, 0x12, 0x04, 0xcf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x10, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x10, 0x03, 0x12, 0x04, 0xcf, 0x01, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x11, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x11, 0x04,
    0x12, 0x04, 0xd0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x11, 0x05, 0x12,
    0x04, 0xd0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x11, 0x01, 0x12, 0x04,
    0xd0, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x11, 0x03, 0x12, 0x04, 0xd0,
    0x01, 0x27, 0x29,
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
