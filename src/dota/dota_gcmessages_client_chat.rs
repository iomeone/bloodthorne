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
pub struct CMsgClientToGCPrivateChatInvite {
    // message fields
    private_chat_channel_name: ::protobuf::SingularField<::std::string::String>,
    invited_account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCPrivateChatInvite {}

impl CMsgClientToGCPrivateChatInvite {
    pub fn new() -> CMsgClientToGCPrivateChatInvite {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCPrivateChatInvite {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCPrivateChatInvite> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCPrivateChatInvite,
        };
        unsafe {
            instance.get(CMsgClientToGCPrivateChatInvite::new)
        }
    }

    // optional string private_chat_channel_name = 1;

    pub fn clear_private_chat_channel_name(&mut self) {
        self.private_chat_channel_name.clear();
    }

    pub fn has_private_chat_channel_name(&self) -> bool {
        self.private_chat_channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_chat_channel_name(&mut self, v: ::std::string::String) {
        self.private_chat_channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_chat_channel_name(&mut self) -> &mut ::std::string::String {
        if self.private_chat_channel_name.is_none() {
            self.private_chat_channel_name.set_default();
        };
        self.private_chat_channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_chat_channel_name(&mut self) -> ::std::string::String {
        self.private_chat_channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_chat_channel_name(&self) -> &str {
        match self.private_chat_channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_chat_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_chat_channel_name
    }

    fn mut_private_chat_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_chat_channel_name
    }

    // optional uint32 invited_account_id = 2;

    pub fn clear_invited_account_id(&mut self) {
        self.invited_account_id = ::std::option::Option::None;
    }

    pub fn has_invited_account_id(&self) -> bool {
        self.invited_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invited_account_id(&mut self, v: u32) {
        self.invited_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_invited_account_id(&self) -> u32 {
        self.invited_account_id.unwrap_or(0)
    }

    fn get_invited_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.invited_account_id
    }

    fn mut_invited_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.invited_account_id
    }
}

impl ::protobuf::Message for CMsgClientToGCPrivateChatInvite {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_chat_channel_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.invited_account_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.invited_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.invited_account_id {
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

impl ::protobuf::MessageStatic for CMsgClientToGCPrivateChatInvite {
    fn new() -> CMsgClientToGCPrivateChatInvite {
        CMsgClientToGCPrivateChatInvite::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCPrivateChatInvite>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_chat_channel_name",
                    CMsgClientToGCPrivateChatInvite::get_private_chat_channel_name_for_reflect,
                    CMsgClientToGCPrivateChatInvite::mut_private_chat_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "invited_account_id",
                    CMsgClientToGCPrivateChatInvite::get_invited_account_id_for_reflect,
                    CMsgClientToGCPrivateChatInvite::mut_invited_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCPrivateChatInvite>(
                    "CMsgClientToGCPrivateChatInvite",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCPrivateChatInvite {
    fn clear(&mut self) {
        self.clear_private_chat_channel_name();
        self.clear_invited_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCPrivateChatInvite {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCPrivateChatInvite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCPrivateChatKick {
    // message fields
    private_chat_channel_name: ::protobuf::SingularField<::std::string::String>,
    kick_account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCPrivateChatKick {}

impl CMsgClientToGCPrivateChatKick {
    pub fn new() -> CMsgClientToGCPrivateChatKick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCPrivateChatKick {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCPrivateChatKick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCPrivateChatKick,
        };
        unsafe {
            instance.get(CMsgClientToGCPrivateChatKick::new)
        }
    }

    // optional string private_chat_channel_name = 1;

    pub fn clear_private_chat_channel_name(&mut self) {
        self.private_chat_channel_name.clear();
    }

    pub fn has_private_chat_channel_name(&self) -> bool {
        self.private_chat_channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_chat_channel_name(&mut self, v: ::std::string::String) {
        self.private_chat_channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_chat_channel_name(&mut self) -> &mut ::std::string::String {
        if self.private_chat_channel_name.is_none() {
            self.private_chat_channel_name.set_default();
        };
        self.private_chat_channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_chat_channel_name(&mut self) -> ::std::string::String {
        self.private_chat_channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_chat_channel_name(&self) -> &str {
        match self.private_chat_channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_chat_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_chat_channel_name
    }

    fn mut_private_chat_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_chat_channel_name
    }

    // optional uint32 kick_account_id = 2;

    pub fn clear_kick_account_id(&mut self) {
        self.kick_account_id = ::std::option::Option::None;
    }

    pub fn has_kick_account_id(&self) -> bool {
        self.kick_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kick_account_id(&mut self, v: u32) {
        self.kick_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_kick_account_id(&self) -> u32 {
        self.kick_account_id.unwrap_or(0)
    }

    fn get_kick_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.kick_account_id
    }

    fn mut_kick_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.kick_account_id
    }
}

impl ::protobuf::Message for CMsgClientToGCPrivateChatKick {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_chat_channel_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.kick_account_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.kick_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.kick_account_id {
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

impl ::protobuf::MessageStatic for CMsgClientToGCPrivateChatKick {
    fn new() -> CMsgClientToGCPrivateChatKick {
        CMsgClientToGCPrivateChatKick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCPrivateChatKick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_chat_channel_name",
                    CMsgClientToGCPrivateChatKick::get_private_chat_channel_name_for_reflect,
                    CMsgClientToGCPrivateChatKick::mut_private_chat_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "kick_account_id",
                    CMsgClientToGCPrivateChatKick::get_kick_account_id_for_reflect,
                    CMsgClientToGCPrivateChatKick::mut_kick_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCPrivateChatKick>(
                    "CMsgClientToGCPrivateChatKick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCPrivateChatKick {
    fn clear(&mut self) {
        self.clear_private_chat_channel_name();
        self.clear_kick_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCPrivateChatKick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCPrivateChatKick {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCPrivateChatPromote {
    // message fields
    private_chat_channel_name: ::protobuf::SingularField<::std::string::String>,
    promote_account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCPrivateChatPromote {}

impl CMsgClientToGCPrivateChatPromote {
    pub fn new() -> CMsgClientToGCPrivateChatPromote {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCPrivateChatPromote {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCPrivateChatPromote> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCPrivateChatPromote,
        };
        unsafe {
            instance.get(CMsgClientToGCPrivateChatPromote::new)
        }
    }

    // optional string private_chat_channel_name = 1;

    pub fn clear_private_chat_channel_name(&mut self) {
        self.private_chat_channel_name.clear();
    }

    pub fn has_private_chat_channel_name(&self) -> bool {
        self.private_chat_channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_chat_channel_name(&mut self, v: ::std::string::String) {
        self.private_chat_channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_chat_channel_name(&mut self) -> &mut ::std::string::String {
        if self.private_chat_channel_name.is_none() {
            self.private_chat_channel_name.set_default();
        };
        self.private_chat_channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_chat_channel_name(&mut self) -> ::std::string::String {
        self.private_chat_channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_chat_channel_name(&self) -> &str {
        match self.private_chat_channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_chat_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_chat_channel_name
    }

    fn mut_private_chat_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_chat_channel_name
    }

    // optional uint32 promote_account_id = 2;

    pub fn clear_promote_account_id(&mut self) {
        self.promote_account_id = ::std::option::Option::None;
    }

    pub fn has_promote_account_id(&self) -> bool {
        self.promote_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_promote_account_id(&mut self, v: u32) {
        self.promote_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_promote_account_id(&self) -> u32 {
        self.promote_account_id.unwrap_or(0)
    }

    fn get_promote_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.promote_account_id
    }

    fn mut_promote_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.promote_account_id
    }
}

impl ::protobuf::Message for CMsgClientToGCPrivateChatPromote {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_chat_channel_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.promote_account_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.promote_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.promote_account_id {
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

impl ::protobuf::MessageStatic for CMsgClientToGCPrivateChatPromote {
    fn new() -> CMsgClientToGCPrivateChatPromote {
        CMsgClientToGCPrivateChatPromote::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCPrivateChatPromote>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_chat_channel_name",
                    CMsgClientToGCPrivateChatPromote::get_private_chat_channel_name_for_reflect,
                    CMsgClientToGCPrivateChatPromote::mut_private_chat_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "promote_account_id",
                    CMsgClientToGCPrivateChatPromote::get_promote_account_id_for_reflect,
                    CMsgClientToGCPrivateChatPromote::mut_promote_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCPrivateChatPromote>(
                    "CMsgClientToGCPrivateChatPromote",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCPrivateChatPromote {
    fn clear(&mut self) {
        self.clear_private_chat_channel_name();
        self.clear_promote_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCPrivateChatPromote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCPrivateChatPromote {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCPrivateChatDemote {
    // message fields
    private_chat_channel_name: ::protobuf::SingularField<::std::string::String>,
    demote_account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCPrivateChatDemote {}

impl CMsgClientToGCPrivateChatDemote {
    pub fn new() -> CMsgClientToGCPrivateChatDemote {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCPrivateChatDemote {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCPrivateChatDemote> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCPrivateChatDemote,
        };
        unsafe {
            instance.get(CMsgClientToGCPrivateChatDemote::new)
        }
    }

    // optional string private_chat_channel_name = 1;

    pub fn clear_private_chat_channel_name(&mut self) {
        self.private_chat_channel_name.clear();
    }

    pub fn has_private_chat_channel_name(&self) -> bool {
        self.private_chat_channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_chat_channel_name(&mut self, v: ::std::string::String) {
        self.private_chat_channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_chat_channel_name(&mut self) -> &mut ::std::string::String {
        if self.private_chat_channel_name.is_none() {
            self.private_chat_channel_name.set_default();
        };
        self.private_chat_channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_chat_channel_name(&mut self) -> ::std::string::String {
        self.private_chat_channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_chat_channel_name(&self) -> &str {
        match self.private_chat_channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_chat_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_chat_channel_name
    }

    fn mut_private_chat_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_chat_channel_name
    }

    // optional uint32 demote_account_id = 2;

    pub fn clear_demote_account_id(&mut self) {
        self.demote_account_id = ::std::option::Option::None;
    }

    pub fn has_demote_account_id(&self) -> bool {
        self.demote_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_demote_account_id(&mut self, v: u32) {
        self.demote_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_demote_account_id(&self) -> u32 {
        self.demote_account_id.unwrap_or(0)
    }

    fn get_demote_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.demote_account_id
    }

    fn mut_demote_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.demote_account_id
    }
}

impl ::protobuf::Message for CMsgClientToGCPrivateChatDemote {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_chat_channel_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.demote_account_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.demote_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.demote_account_id {
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

impl ::protobuf::MessageStatic for CMsgClientToGCPrivateChatDemote {
    fn new() -> CMsgClientToGCPrivateChatDemote {
        CMsgClientToGCPrivateChatDemote::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCPrivateChatDemote>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_chat_channel_name",
                    CMsgClientToGCPrivateChatDemote::get_private_chat_channel_name_for_reflect,
                    CMsgClientToGCPrivateChatDemote::mut_private_chat_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "demote_account_id",
                    CMsgClientToGCPrivateChatDemote::get_demote_account_id_for_reflect,
                    CMsgClientToGCPrivateChatDemote::mut_demote_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCPrivateChatDemote>(
                    "CMsgClientToGCPrivateChatDemote",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCPrivateChatDemote {
    fn clear(&mut self) {
        self.clear_private_chat_channel_name();
        self.clear_demote_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCPrivateChatDemote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCPrivateChatDemote {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientPrivateChatResponse {
    // message fields
    private_chat_channel_name: ::protobuf::SingularField<::std::string::String>,
    result: ::std::option::Option<CMsgGCToClientPrivateChatResponse_Result>,
    username: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientPrivateChatResponse {}

impl CMsgGCToClientPrivateChatResponse {
    pub fn new() -> CMsgGCToClientPrivateChatResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientPrivateChatResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientPrivateChatResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientPrivateChatResponse,
        };
        unsafe {
            instance.get(CMsgGCToClientPrivateChatResponse::new)
        }
    }

    // optional string private_chat_channel_name = 1;

    pub fn clear_private_chat_channel_name(&mut self) {
        self.private_chat_channel_name.clear();
    }

    pub fn has_private_chat_channel_name(&self) -> bool {
        self.private_chat_channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_chat_channel_name(&mut self, v: ::std::string::String) {
        self.private_chat_channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_chat_channel_name(&mut self) -> &mut ::std::string::String {
        if self.private_chat_channel_name.is_none() {
            self.private_chat_channel_name.set_default();
        };
        self.private_chat_channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_chat_channel_name(&mut self) -> ::std::string::String {
        self.private_chat_channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_chat_channel_name(&self) -> &str {
        match self.private_chat_channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_chat_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_chat_channel_name
    }

    fn mut_private_chat_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_chat_channel_name
    }

    // optional .dota.CMsgGCToClientPrivateChatResponse.Result result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgGCToClientPrivateChatResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgGCToClientPrivateChatResponse_Result {
        self.result.unwrap_or(CMsgGCToClientPrivateChatResponse_Result::SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgGCToClientPrivateChatResponse_Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgGCToClientPrivateChatResponse_Result> {
        &mut self.result
    }

    // optional string username = 3;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    pub fn has_username(&self) -> bool {
        self.username.is_some()
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        if self.username.is_none() {
            self.username.set_default();
        };
        self.username.as_mut().unwrap()
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        self.username.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        match self.username.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_username_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.username
    }
}

impl ::protobuf::Message for CMsgGCToClientPrivateChatResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_chat_channel_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.username)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.username.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.result {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.username.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgGCToClientPrivateChatResponse {
    fn new() -> CMsgGCToClientPrivateChatResponse {
        CMsgGCToClientPrivateChatResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientPrivateChatResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_chat_channel_name",
                    CMsgGCToClientPrivateChatResponse::get_private_chat_channel_name_for_reflect,
                    CMsgGCToClientPrivateChatResponse::mut_private_chat_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgGCToClientPrivateChatResponse_Result>>(
                    "result",
                    CMsgGCToClientPrivateChatResponse::get_result_for_reflect,
                    CMsgGCToClientPrivateChatResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    CMsgGCToClientPrivateChatResponse::get_username_for_reflect,
                    CMsgGCToClientPrivateChatResponse::mut_username_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientPrivateChatResponse>(
                    "CMsgGCToClientPrivateChatResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientPrivateChatResponse {
    fn clear(&mut self) {
        self.clear_private_chat_channel_name();
        self.clear_result();
        self.clear_username();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientPrivateChatResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientPrivateChatResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgGCToClientPrivateChatResponse_Result {
    SUCCESS = 0,
    FAILURE_CREATION_LOCK = 1,
    FAILURE_SQL_TRANSACTION = 2,
    FAILURE_SDO_LOAD = 3,
    FAILURE_NO_PERMISSION = 4,
    FAILURE_ALREADY_MEMBER = 5,
    FAILURE_NOT_A_MEMBER = 7,
    FAILURE_NO_REMAINING_ADMINS = 8,
    FAILURE_NO_ROOM = 9,
    FAILURE_CREATION_RATE_LIMITED = 10,
    FAILURE_UNKNOWN_CHANNEL_NAME = 11,
    FAILURE_UNKNOWN_USER = 12,
    FAILURE_UNKNOWN_ERROR = 13,
    FAILURE_CANNOT_KICK_ADMIN = 14,
    FAILURE_ALREADY_ADMIN = 15,
}

impl ::protobuf::ProtobufEnum for CMsgGCToClientPrivateChatResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgGCToClientPrivateChatResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::SUCCESS),
            1 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_CREATION_LOCK),
            2 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_SQL_TRANSACTION),
            3 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_SDO_LOAD),
            4 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_NO_PERMISSION),
            5 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_ALREADY_MEMBER),
            7 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_NOT_A_MEMBER),
            8 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_NO_REMAINING_ADMINS),
            9 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_NO_ROOM),
            10 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_CREATION_RATE_LIMITED),
            11 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_UNKNOWN_CHANNEL_NAME),
            12 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_UNKNOWN_USER),
            13 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_UNKNOWN_ERROR),
            14 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_CANNOT_KICK_ADMIN),
            15 => ::std::option::Option::Some(CMsgGCToClientPrivateChatResponse_Result::FAILURE_ALREADY_ADMIN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgGCToClientPrivateChatResponse_Result] = &[
            CMsgGCToClientPrivateChatResponse_Result::SUCCESS,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_CREATION_LOCK,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_SQL_TRANSACTION,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_SDO_LOAD,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_NO_PERMISSION,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_ALREADY_MEMBER,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_NOT_A_MEMBER,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_NO_REMAINING_ADMINS,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_NO_ROOM,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_CREATION_RATE_LIMITED,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_UNKNOWN_CHANNEL_NAME,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_UNKNOWN_USER,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_UNKNOWN_ERROR,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_CANNOT_KICK_ADMIN,
            CMsgGCToClientPrivateChatResponse_Result::FAILURE_ALREADY_ADMIN,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgGCToClientPrivateChatResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgGCToClientPrivateChatResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgGCToClientPrivateChatResponse_Result {
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientPrivateChatResponse_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCPrivateChatInfoRequest {
    // message fields
    private_chat_channel_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCPrivateChatInfoRequest {}

impl CMsgClientToGCPrivateChatInfoRequest {
    pub fn new() -> CMsgClientToGCPrivateChatInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCPrivateChatInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCPrivateChatInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCPrivateChatInfoRequest,
        };
        unsafe {
            instance.get(CMsgClientToGCPrivateChatInfoRequest::new)
        }
    }

    // optional string private_chat_channel_name = 1;

    pub fn clear_private_chat_channel_name(&mut self) {
        self.private_chat_channel_name.clear();
    }

    pub fn has_private_chat_channel_name(&self) -> bool {
        self.private_chat_channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_chat_channel_name(&mut self, v: ::std::string::String) {
        self.private_chat_channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_chat_channel_name(&mut self) -> &mut ::std::string::String {
        if self.private_chat_channel_name.is_none() {
            self.private_chat_channel_name.set_default();
        };
        self.private_chat_channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_chat_channel_name(&mut self) -> ::std::string::String {
        self.private_chat_channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_chat_channel_name(&self) -> &str {
        match self.private_chat_channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_chat_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_chat_channel_name
    }

    fn mut_private_chat_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_chat_channel_name
    }
}

impl ::protobuf::Message for CMsgClientToGCPrivateChatInfoRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_chat_channel_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.private_chat_channel_name.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgClientToGCPrivateChatInfoRequest {
    fn new() -> CMsgClientToGCPrivateChatInfoRequest {
        CMsgClientToGCPrivateChatInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCPrivateChatInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_chat_channel_name",
                    CMsgClientToGCPrivateChatInfoRequest::get_private_chat_channel_name_for_reflect,
                    CMsgClientToGCPrivateChatInfoRequest::mut_private_chat_channel_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCPrivateChatInfoRequest>(
                    "CMsgClientToGCPrivateChatInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCPrivateChatInfoRequest {
    fn clear(&mut self) {
        self.clear_private_chat_channel_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCPrivateChatInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCPrivateChatInfoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientPrivateChatInfoResponse {
    // message fields
    private_chat_channel_name: ::protobuf::SingularField<::std::string::String>,
    members: ::protobuf::RepeatedField<CMsgGCToClientPrivateChatInfoResponse_Member>,
    creator: ::std::option::Option<u32>,
    creation_date: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientPrivateChatInfoResponse {}

impl CMsgGCToClientPrivateChatInfoResponse {
    pub fn new() -> CMsgGCToClientPrivateChatInfoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientPrivateChatInfoResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientPrivateChatInfoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientPrivateChatInfoResponse,
        };
        unsafe {
            instance.get(CMsgGCToClientPrivateChatInfoResponse::new)
        }
    }

    // optional string private_chat_channel_name = 1;

    pub fn clear_private_chat_channel_name(&mut self) {
        self.private_chat_channel_name.clear();
    }

    pub fn has_private_chat_channel_name(&self) -> bool {
        self.private_chat_channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_chat_channel_name(&mut self, v: ::std::string::String) {
        self.private_chat_channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_chat_channel_name(&mut self) -> &mut ::std::string::String {
        if self.private_chat_channel_name.is_none() {
            self.private_chat_channel_name.set_default();
        };
        self.private_chat_channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_chat_channel_name(&mut self) -> ::std::string::String {
        self.private_chat_channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_private_chat_channel_name(&self) -> &str {
        match self.private_chat_channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_private_chat_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.private_chat_channel_name
    }

    fn mut_private_chat_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.private_chat_channel_name
    }

    // repeated .dota.CMsgGCToClientPrivateChatInfoResponse.Member members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CMsgGCToClientPrivateChatInfoResponse_Member>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCToClientPrivateChatInfoResponse_Member> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CMsgGCToClientPrivateChatInfoResponse_Member> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CMsgGCToClientPrivateChatInfoResponse_Member] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCToClientPrivateChatInfoResponse_Member> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCToClientPrivateChatInfoResponse_Member> {
        &mut self.members
    }

    // optional uint32 creator = 3;

    pub fn clear_creator(&mut self) {
        self.creator = ::std::option::Option::None;
    }

    pub fn has_creator(&self) -> bool {
        self.creator.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creator(&mut self, v: u32) {
        self.creator = ::std::option::Option::Some(v);
    }

    pub fn get_creator(&self) -> u32 {
        self.creator.unwrap_or(0)
    }

    fn get_creator_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.creator
    }

    fn mut_creator_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.creator
    }

    // optional uint32 creation_date = 4;

    pub fn clear_creation_date(&mut self) {
        self.creation_date = ::std::option::Option::None;
    }

    pub fn has_creation_date(&self) -> bool {
        self.creation_date.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creation_date(&mut self, v: u32) {
        self.creation_date = ::std::option::Option::Some(v);
    }

    pub fn get_creation_date(&self) -> u32 {
        self.creation_date.unwrap_or(0)
    }

    fn get_creation_date_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.creation_date
    }

    fn mut_creation_date_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.creation_date
    }
}

impl ::protobuf::Message for CMsgGCToClientPrivateChatInfoResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.private_chat_channel_name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.creator = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.creation_date = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.creator {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.creation_date {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.private_chat_channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        for v in &self.members {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.creator {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.creation_date {
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

impl ::protobuf::MessageStatic for CMsgGCToClientPrivateChatInfoResponse {
    fn new() -> CMsgGCToClientPrivateChatInfoResponse {
        CMsgGCToClientPrivateChatInfoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientPrivateChatInfoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "private_chat_channel_name",
                    CMsgGCToClientPrivateChatInfoResponse::get_private_chat_channel_name_for_reflect,
                    CMsgGCToClientPrivateChatInfoResponse::mut_private_chat_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCToClientPrivateChatInfoResponse_Member>>(
                    "members",
                    CMsgGCToClientPrivateChatInfoResponse::get_members_for_reflect,
                    CMsgGCToClientPrivateChatInfoResponse::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "creator",
                    CMsgGCToClientPrivateChatInfoResponse::get_creator_for_reflect,
                    CMsgGCToClientPrivateChatInfoResponse::mut_creator_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "creation_date",
                    CMsgGCToClientPrivateChatInfoResponse::get_creation_date_for_reflect,
                    CMsgGCToClientPrivateChatInfoResponse::mut_creation_date_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientPrivateChatInfoResponse>(
                    "CMsgGCToClientPrivateChatInfoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientPrivateChatInfoResponse {
    fn clear(&mut self) {
        self.clear_private_chat_channel_name();
        self.clear_members();
        self.clear_creator();
        self.clear_creation_date();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientPrivateChatInfoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientPrivateChatInfoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientPrivateChatInfoResponse_Member {
    // message fields
    account_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    status: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientPrivateChatInfoResponse_Member {}

impl CMsgGCToClientPrivateChatInfoResponse_Member {
    pub fn new() -> CMsgGCToClientPrivateChatInfoResponse_Member {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientPrivateChatInfoResponse_Member {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientPrivateChatInfoResponse_Member> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientPrivateChatInfoResponse_Member,
        };
        unsafe {
            instance.get(CMsgGCToClientPrivateChatInfoResponse_Member::new)
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

    // optional uint32 status = 3;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> u32 {
        self.status.unwrap_or(0)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status
    }
}

impl ::protobuf::Message for CMsgGCToClientPrivateChatInfoResponse_Member {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.status {
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
        if let Some(v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.status {
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

impl ::protobuf::MessageStatic for CMsgGCToClientPrivateChatInfoResponse_Member {
    fn new() -> CMsgGCToClientPrivateChatInfoResponse_Member {
        CMsgGCToClientPrivateChatInfoResponse_Member::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientPrivateChatInfoResponse_Member>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgGCToClientPrivateChatInfoResponse_Member::get_account_id_for_reflect,
                    CMsgGCToClientPrivateChatInfoResponse_Member::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgGCToClientPrivateChatInfoResponse_Member::get_name_for_reflect,
                    CMsgGCToClientPrivateChatInfoResponse_Member::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    CMsgGCToClientPrivateChatInfoResponse_Member::get_status_for_reflect,
                    CMsgGCToClientPrivateChatInfoResponse_Member::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientPrivateChatInfoResponse_Member>(
                    "CMsgGCToClientPrivateChatInfoResponse_Member",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientPrivateChatInfoResponse_Member {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_name();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientPrivateChatInfoResponse_Member {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientPrivateChatInfoResponse_Member {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAJoinChatChannel {
    // message fields
    channel_name: ::protobuf::SingularField<::std::string::String>,
    channel_type: ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAJoinChatChannel {}

impl CMsgDOTAJoinChatChannel {
    pub fn new() -> CMsgDOTAJoinChatChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAJoinChatChannel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAJoinChatChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAJoinChatChannel,
        };
        unsafe {
            instance.get(CMsgDOTAJoinChatChannel::new)
        }
    }

    // optional string channel_name = 2;

    pub fn clear_channel_name(&mut self) {
        self.channel_name.clear();
    }

    pub fn has_channel_name(&self) -> bool {
        self.channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_name(&mut self, v: ::std::string::String) {
        self.channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_name(&mut self) -> &mut ::std::string::String {
        if self.channel_name.is_none() {
            self.channel_name.set_default();
        };
        self.channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel_name(&mut self) -> ::std::string::String {
        self.channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel_name(&self) -> &str {
        match self.channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel_name
    }

    fn mut_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel_name
    }

    // optional .dota.DOTAChatChannelType_t channel_type = 4;

    pub fn clear_channel_type(&mut self) {
        self.channel_type = ::std::option::Option::None;
    }

    pub fn has_channel_type(&self) -> bool {
        self.channel_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_type(&mut self, v: super::dota_shared_enums::DOTAChatChannelType_t) {
        self.channel_type = ::std::option::Option::Some(v);
    }

    pub fn get_channel_type(&self) -> super::dota_shared_enums::DOTAChatChannelType_t {
        self.channel_type.unwrap_or(super::dota_shared_enums::DOTAChatChannelType_t::DOTAChannelType_Regional)
    }

    fn get_channel_type_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &self.channel_type
    }

    fn mut_channel_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &mut self.channel_type
    }
}

impl ::protobuf::Message for CMsgDOTAJoinChatChannel {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
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
        if let Some(v) = self.channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.channel_type {
            my_size += ::protobuf::rt::enum_size(4, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.channel_type {
            os.write_enum(4, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgDOTAJoinChatChannel {
    fn new() -> CMsgDOTAJoinChatChannel {
        CMsgDOTAJoinChatChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAJoinChatChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel_name",
                    CMsgDOTAJoinChatChannel::get_channel_name_for_reflect,
                    CMsgDOTAJoinChatChannel::mut_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAChatChannelType_t>>(
                    "channel_type",
                    CMsgDOTAJoinChatChannel::get_channel_type_for_reflect,
                    CMsgDOTAJoinChatChannel::mut_channel_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAJoinChatChannel>(
                    "CMsgDOTAJoinChatChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAJoinChatChannel {
    fn clear(&mut self) {
        self.clear_channel_name();
        self.clear_channel_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAJoinChatChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAJoinChatChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTALeaveChatChannel {
    // message fields
    channel_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTALeaveChatChannel {}

impl CMsgDOTALeaveChatChannel {
    pub fn new() -> CMsgDOTALeaveChatChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTALeaveChatChannel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTALeaveChatChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTALeaveChatChannel,
        };
        unsafe {
            instance.get(CMsgDOTALeaveChatChannel::new)
        }
    }

    // optional uint64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }
}

impl ::protobuf::Message for CMsgDOTALeaveChatChannel {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
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

impl ::protobuf::MessageStatic for CMsgDOTALeaveChatChannel {
    fn new() -> CMsgDOTALeaveChatChannel {
        CMsgDOTALeaveChatChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTALeaveChatChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "channel_id",
                    CMsgDOTALeaveChatChannel::get_channel_id_for_reflect,
                    CMsgDOTALeaveChatChannel::mut_channel_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTALeaveChatChannel>(
                    "CMsgDOTALeaveChatChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTALeaveChatChannel {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTALeaveChatChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTALeaveChatChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCChatReportPublicSpam {
    // message fields
    channel_id: ::std::option::Option<u64>,
    channel_user_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCChatReportPublicSpam {}

impl CMsgGCChatReportPublicSpam {
    pub fn new() -> CMsgGCChatReportPublicSpam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCChatReportPublicSpam {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCChatReportPublicSpam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCChatReportPublicSpam,
        };
        unsafe {
            instance.get(CMsgGCChatReportPublicSpam::new)
        }
    }

    // optional uint64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // optional uint32 channel_user_id = 2;

    pub fn clear_channel_user_id(&mut self) {
        self.channel_user_id = ::std::option::Option::None;
    }

    pub fn has_channel_user_id(&self) -> bool {
        self.channel_user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_user_id(&mut self, v: u32) {
        self.channel_user_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_user_id(&self) -> u32 {
        self.channel_user_id.unwrap_or(0)
    }

    fn get_channel_user_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_user_id
    }

    fn mut_channel_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_user_id
    }
}

impl ::protobuf::Message for CMsgGCChatReportPublicSpam {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel_user_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.channel_user_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.channel_user_id {
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

impl ::protobuf::MessageStatic for CMsgGCChatReportPublicSpam {
    fn new() -> CMsgGCChatReportPublicSpam {
        CMsgGCChatReportPublicSpam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCChatReportPublicSpam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "channel_id",
                    CMsgGCChatReportPublicSpam::get_channel_id_for_reflect,
                    CMsgGCChatReportPublicSpam::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_user_id",
                    CMsgGCChatReportPublicSpam::get_channel_user_id_for_reflect,
                    CMsgGCChatReportPublicSpam::mut_channel_user_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCChatReportPublicSpam>(
                    "CMsgGCChatReportPublicSpam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCChatReportPublicSpam {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_channel_user_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCChatReportPublicSpam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCChatReportPublicSpam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAClientIgnoredUser {
    // message fields
    ignored_account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAClientIgnoredUser {}

impl CMsgDOTAClientIgnoredUser {
    pub fn new() -> CMsgDOTAClientIgnoredUser {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAClientIgnoredUser {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAClientIgnoredUser> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAClientIgnoredUser,
        };
        unsafe {
            instance.get(CMsgDOTAClientIgnoredUser::new)
        }
    }

    // optional uint32 ignored_account_id = 1;

    pub fn clear_ignored_account_id(&mut self) {
        self.ignored_account_id = ::std::option::Option::None;
    }

    pub fn has_ignored_account_id(&self) -> bool {
        self.ignored_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignored_account_id(&mut self, v: u32) {
        self.ignored_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_ignored_account_id(&self) -> u32 {
        self.ignored_account_id.unwrap_or(0)
    }

    fn get_ignored_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ignored_account_id
    }

    fn mut_ignored_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ignored_account_id
    }
}

impl ::protobuf::Message for CMsgDOTAClientIgnoredUser {
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
                    self.ignored_account_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ignored_account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ignored_account_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAClientIgnoredUser {
    fn new() -> CMsgDOTAClientIgnoredUser {
        CMsgDOTAClientIgnoredUser::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAClientIgnoredUser>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ignored_account_id",
                    CMsgDOTAClientIgnoredUser::get_ignored_account_id_for_reflect,
                    CMsgDOTAClientIgnoredUser::mut_ignored_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAClientIgnoredUser>(
                    "CMsgDOTAClientIgnoredUser",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAClientIgnoredUser {
    fn clear(&mut self) {
        self.clear_ignored_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAClientIgnoredUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAClientIgnoredUser {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatMessage {
    // message fields
    account_id: ::std::option::Option<u32>,
    channel_id: ::std::option::Option<u64>,
    persona_name: ::protobuf::SingularField<::std::string::String>,
    text: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<u32>,
    suggest_invite_account_id: ::std::option::Option<u32>,
    suggest_invite_name: ::protobuf::SingularField<::std::string::String>,
    fantasy_draft_owner_account_id: ::std::option::Option<u32>,
    fantasy_draft_player_account_id: ::std::option::Option<u32>,
    event_id: ::std::option::Option<u32>,
    suggest_invite_to_lobby: ::std::option::Option<bool>,
    event_points: ::std::option::Option<u32>,
    coin_flip: ::std::option::Option<bool>,
    player_id: ::std::option::Option<i32>,
    share_profile_account_id: ::std::option::Option<u32>,
    channel_user_id: ::std::option::Option<u32>,
    dice_roll: ::protobuf::SingularPtrField<CMsgDOTAChatMessage_DiceRoll>,
    share_party_id: ::std::option::Option<u64>,
    share_lobby_id: ::std::option::Option<u64>,
    share_lobby_custom_game_id: ::std::option::Option<u64>,
    share_lobby_passkey: ::protobuf::SingularField<::std::string::String>,
    private_chat_channel_id: ::std::option::Option<u32>,
    status: ::std::option::Option<u32>,
    legacy_battle_cup_victory: ::std::option::Option<bool>,
    battle_cup_streak: ::std::option::Option<u32>,
    badge_level: ::std::option::Option<u32>,
    suggest_pick_hero_id: ::std::option::Option<u32>,
    suggest_pick_hero_role: ::protobuf::SingularField<::std::string::String>,
    terse: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatMessage {}

impl CMsgDOTAChatMessage {
    pub fn new() -> CMsgDOTAChatMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatMessage {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatMessage,
        };
        unsafe {
            instance.get(CMsgDOTAChatMessage::new)
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

    // optional uint64 channel_id = 2;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // optional string persona_name = 3;

    pub fn clear_persona_name(&mut self) {
        self.persona_name.clear();
    }

    pub fn has_persona_name(&self) -> bool {
        self.persona_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persona_name(&mut self, v: ::std::string::String) {
        self.persona_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_persona_name(&mut self) -> &mut ::std::string::String {
        if self.persona_name.is_none() {
            self.persona_name.set_default();
        };
        self.persona_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_persona_name(&mut self) -> ::std::string::String {
        self.persona_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_persona_name(&self) -> &str {
        match self.persona_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_persona_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.persona_name
    }

    fn mut_persona_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.persona_name
    }

    // optional string text = 4;

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

    // optional uint32 timestamp = 5;

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

    // optional uint32 suggest_invite_account_id = 6;

    pub fn clear_suggest_invite_account_id(&mut self) {
        self.suggest_invite_account_id = ::std::option::Option::None;
    }

    pub fn has_suggest_invite_account_id(&self) -> bool {
        self.suggest_invite_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suggest_invite_account_id(&mut self, v: u32) {
        self.suggest_invite_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_suggest_invite_account_id(&self) -> u32 {
        self.suggest_invite_account_id.unwrap_or(0)
    }

    fn get_suggest_invite_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.suggest_invite_account_id
    }

    fn mut_suggest_invite_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.suggest_invite_account_id
    }

    // optional string suggest_invite_name = 7;

    pub fn clear_suggest_invite_name(&mut self) {
        self.suggest_invite_name.clear();
    }

    pub fn has_suggest_invite_name(&self) -> bool {
        self.suggest_invite_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suggest_invite_name(&mut self, v: ::std::string::String) {
        self.suggest_invite_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_suggest_invite_name(&mut self) -> &mut ::std::string::String {
        if self.suggest_invite_name.is_none() {
            self.suggest_invite_name.set_default();
        };
        self.suggest_invite_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_suggest_invite_name(&mut self) -> ::std::string::String {
        self.suggest_invite_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_suggest_invite_name(&self) -> &str {
        match self.suggest_invite_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_suggest_invite_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.suggest_invite_name
    }

    fn mut_suggest_invite_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.suggest_invite_name
    }

    // optional uint32 fantasy_draft_owner_account_id = 8;

    pub fn clear_fantasy_draft_owner_account_id(&mut self) {
        self.fantasy_draft_owner_account_id = ::std::option::Option::None;
    }

    pub fn has_fantasy_draft_owner_account_id(&self) -> bool {
        self.fantasy_draft_owner_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fantasy_draft_owner_account_id(&mut self, v: u32) {
        self.fantasy_draft_owner_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_fantasy_draft_owner_account_id(&self) -> u32 {
        self.fantasy_draft_owner_account_id.unwrap_or(0)
    }

    fn get_fantasy_draft_owner_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.fantasy_draft_owner_account_id
    }

    fn mut_fantasy_draft_owner_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.fantasy_draft_owner_account_id
    }

    // optional uint32 fantasy_draft_player_account_id = 9;

    pub fn clear_fantasy_draft_player_account_id(&mut self) {
        self.fantasy_draft_player_account_id = ::std::option::Option::None;
    }

    pub fn has_fantasy_draft_player_account_id(&self) -> bool {
        self.fantasy_draft_player_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fantasy_draft_player_account_id(&mut self, v: u32) {
        self.fantasy_draft_player_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_fantasy_draft_player_account_id(&self) -> u32 {
        self.fantasy_draft_player_account_id.unwrap_or(0)
    }

    fn get_fantasy_draft_player_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.fantasy_draft_player_account_id
    }

    fn mut_fantasy_draft_player_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.fantasy_draft_player_account_id
    }

    // optional uint32 event_id = 10;

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

    // optional bool suggest_invite_to_lobby = 11;

    pub fn clear_suggest_invite_to_lobby(&mut self) {
        self.suggest_invite_to_lobby = ::std::option::Option::None;
    }

    pub fn has_suggest_invite_to_lobby(&self) -> bool {
        self.suggest_invite_to_lobby.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suggest_invite_to_lobby(&mut self, v: bool) {
        self.suggest_invite_to_lobby = ::std::option::Option::Some(v);
    }

    pub fn get_suggest_invite_to_lobby(&self) -> bool {
        self.suggest_invite_to_lobby.unwrap_or(false)
    }

    fn get_suggest_invite_to_lobby_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.suggest_invite_to_lobby
    }

    fn mut_suggest_invite_to_lobby_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.suggest_invite_to_lobby
    }

    // optional uint32 event_points = 12;

    pub fn clear_event_points(&mut self) {
        self.event_points = ::std::option::Option::None;
    }

    pub fn has_event_points(&self) -> bool {
        self.event_points.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_points(&mut self, v: u32) {
        self.event_points = ::std::option::Option::Some(v);
    }

    pub fn get_event_points(&self) -> u32 {
        self.event_points.unwrap_or(0)
    }

    fn get_event_points_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.event_points
    }

    fn mut_event_points_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.event_points
    }

    // optional bool coin_flip = 13;

    pub fn clear_coin_flip(&mut self) {
        self.coin_flip = ::std::option::Option::None;
    }

    pub fn has_coin_flip(&self) -> bool {
        self.coin_flip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_coin_flip(&mut self, v: bool) {
        self.coin_flip = ::std::option::Option::Some(v);
    }

    pub fn get_coin_flip(&self) -> bool {
        self.coin_flip.unwrap_or(false)
    }

    fn get_coin_flip_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.coin_flip
    }

    fn mut_coin_flip_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.coin_flip
    }

    // optional int32 player_id = 14;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: i32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> i32 {
        self.player_id.unwrap_or(-1i32)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_id
    }

    // optional uint32 share_profile_account_id = 15;

    pub fn clear_share_profile_account_id(&mut self) {
        self.share_profile_account_id = ::std::option::Option::None;
    }

    pub fn has_share_profile_account_id(&self) -> bool {
        self.share_profile_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_share_profile_account_id(&mut self, v: u32) {
        self.share_profile_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_share_profile_account_id(&self) -> u32 {
        self.share_profile_account_id.unwrap_or(0)
    }

    fn get_share_profile_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.share_profile_account_id
    }

    fn mut_share_profile_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.share_profile_account_id
    }

    // optional uint32 channel_user_id = 16;

    pub fn clear_channel_user_id(&mut self) {
        self.channel_user_id = ::std::option::Option::None;
    }

    pub fn has_channel_user_id(&self) -> bool {
        self.channel_user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_user_id(&mut self, v: u32) {
        self.channel_user_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_user_id(&self) -> u32 {
        self.channel_user_id.unwrap_or(0)
    }

    fn get_channel_user_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_user_id
    }

    fn mut_channel_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_user_id
    }

    // optional .dota.CMsgDOTAChatMessage.DiceRoll dice_roll = 17;

    pub fn clear_dice_roll(&mut self) {
        self.dice_roll.clear();
    }

    pub fn has_dice_roll(&self) -> bool {
        self.dice_roll.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dice_roll(&mut self, v: CMsgDOTAChatMessage_DiceRoll) {
        self.dice_roll = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dice_roll(&mut self) -> &mut CMsgDOTAChatMessage_DiceRoll {
        if self.dice_roll.is_none() {
            self.dice_roll.set_default();
        };
        self.dice_roll.as_mut().unwrap()
    }

    // Take field
    pub fn take_dice_roll(&mut self) -> CMsgDOTAChatMessage_DiceRoll {
        self.dice_roll.take().unwrap_or_else(|| CMsgDOTAChatMessage_DiceRoll::new())
    }

    pub fn get_dice_roll(&self) -> &CMsgDOTAChatMessage_DiceRoll {
        self.dice_roll.as_ref().unwrap_or_else(|| CMsgDOTAChatMessage_DiceRoll::default_instance())
    }

    fn get_dice_roll_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgDOTAChatMessage_DiceRoll> {
        &self.dice_roll
    }

    fn mut_dice_roll_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgDOTAChatMessage_DiceRoll> {
        &mut self.dice_roll
    }

    // optional uint64 share_party_id = 18;

    pub fn clear_share_party_id(&mut self) {
        self.share_party_id = ::std::option::Option::None;
    }

    pub fn has_share_party_id(&self) -> bool {
        self.share_party_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_share_party_id(&mut self, v: u64) {
        self.share_party_id = ::std::option::Option::Some(v);
    }

    pub fn get_share_party_id(&self) -> u64 {
        self.share_party_id.unwrap_or(0)
    }

    fn get_share_party_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.share_party_id
    }

    fn mut_share_party_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.share_party_id
    }

    // optional uint64 share_lobby_id = 19;

    pub fn clear_share_lobby_id(&mut self) {
        self.share_lobby_id = ::std::option::Option::None;
    }

    pub fn has_share_lobby_id(&self) -> bool {
        self.share_lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_share_lobby_id(&mut self, v: u64) {
        self.share_lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_share_lobby_id(&self) -> u64 {
        self.share_lobby_id.unwrap_or(0)
    }

    fn get_share_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.share_lobby_id
    }

    fn mut_share_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.share_lobby_id
    }

    // optional uint64 share_lobby_custom_game_id = 20;

    pub fn clear_share_lobby_custom_game_id(&mut self) {
        self.share_lobby_custom_game_id = ::std::option::Option::None;
    }

    pub fn has_share_lobby_custom_game_id(&self) -> bool {
        self.share_lobby_custom_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_share_lobby_custom_game_id(&mut self, v: u64) {
        self.share_lobby_custom_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_share_lobby_custom_game_id(&self) -> u64 {
        self.share_lobby_custom_game_id.unwrap_or(0)
    }

    fn get_share_lobby_custom_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.share_lobby_custom_game_id
    }

    fn mut_share_lobby_custom_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.share_lobby_custom_game_id
    }

    // optional string share_lobby_passkey = 21;

    pub fn clear_share_lobby_passkey(&mut self) {
        self.share_lobby_passkey.clear();
    }

    pub fn has_share_lobby_passkey(&self) -> bool {
        self.share_lobby_passkey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_share_lobby_passkey(&mut self, v: ::std::string::String) {
        self.share_lobby_passkey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_share_lobby_passkey(&mut self) -> &mut ::std::string::String {
        if self.share_lobby_passkey.is_none() {
            self.share_lobby_passkey.set_default();
        };
        self.share_lobby_passkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_share_lobby_passkey(&mut self) -> ::std::string::String {
        self.share_lobby_passkey.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_share_lobby_passkey(&self) -> &str {
        match self.share_lobby_passkey.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_share_lobby_passkey_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.share_lobby_passkey
    }

    fn mut_share_lobby_passkey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.share_lobby_passkey
    }

    // optional uint32 private_chat_channel_id = 22;

    pub fn clear_private_chat_channel_id(&mut self) {
        self.private_chat_channel_id = ::std::option::Option::None;
    }

    pub fn has_private_chat_channel_id(&self) -> bool {
        self.private_chat_channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_chat_channel_id(&mut self, v: u32) {
        self.private_chat_channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_private_chat_channel_id(&self) -> u32 {
        self.private_chat_channel_id.unwrap_or(0)
    }

    fn get_private_chat_channel_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.private_chat_channel_id
    }

    fn mut_private_chat_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.private_chat_channel_id
    }

    // optional uint32 status = 23;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> u32 {
        self.status.unwrap_or(0)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status
    }

    // optional bool legacy_battle_cup_victory = 24;

    pub fn clear_legacy_battle_cup_victory(&mut self) {
        self.legacy_battle_cup_victory = ::std::option::Option::None;
    }

    pub fn has_legacy_battle_cup_victory(&self) -> bool {
        self.legacy_battle_cup_victory.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_battle_cup_victory(&mut self, v: bool) {
        self.legacy_battle_cup_victory = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_battle_cup_victory(&self) -> bool {
        self.legacy_battle_cup_victory.unwrap_or(false)
    }

    fn get_legacy_battle_cup_victory_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.legacy_battle_cup_victory
    }

    fn mut_legacy_battle_cup_victory_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.legacy_battle_cup_victory
    }

    // optional uint32 battle_cup_streak = 29;

    pub fn clear_battle_cup_streak(&mut self) {
        self.battle_cup_streak = ::std::option::Option::None;
    }

    pub fn has_battle_cup_streak(&self) -> bool {
        self.battle_cup_streak.is_some()
    }

    // Param is passed by value, moved
    pub fn set_battle_cup_streak(&mut self, v: u32) {
        self.battle_cup_streak = ::std::option::Option::Some(v);
    }

    pub fn get_battle_cup_streak(&self) -> u32 {
        self.battle_cup_streak.unwrap_or(0)
    }

    fn get_battle_cup_streak_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.battle_cup_streak
    }

    fn mut_battle_cup_streak_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.battle_cup_streak
    }

    // optional uint32 badge_level = 25;

    pub fn clear_badge_level(&mut self) {
        self.badge_level = ::std::option::Option::None;
    }

    pub fn has_badge_level(&self) -> bool {
        self.badge_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_badge_level(&mut self, v: u32) {
        self.badge_level = ::std::option::Option::Some(v);
    }

    pub fn get_badge_level(&self) -> u32 {
        self.badge_level.unwrap_or(0)
    }

    fn get_badge_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.badge_level
    }

    fn mut_badge_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.badge_level
    }

    // optional uint32 suggest_pick_hero_id = 26;

    pub fn clear_suggest_pick_hero_id(&mut self) {
        self.suggest_pick_hero_id = ::std::option::Option::None;
    }

    pub fn has_suggest_pick_hero_id(&self) -> bool {
        self.suggest_pick_hero_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suggest_pick_hero_id(&mut self, v: u32) {
        self.suggest_pick_hero_id = ::std::option::Option::Some(v);
    }

    pub fn get_suggest_pick_hero_id(&self) -> u32 {
        self.suggest_pick_hero_id.unwrap_or(0)
    }

    fn get_suggest_pick_hero_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.suggest_pick_hero_id
    }

    fn mut_suggest_pick_hero_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.suggest_pick_hero_id
    }

    // optional string suggest_pick_hero_role = 27;

    pub fn clear_suggest_pick_hero_role(&mut self) {
        self.suggest_pick_hero_role.clear();
    }

    pub fn has_suggest_pick_hero_role(&self) -> bool {
        self.suggest_pick_hero_role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suggest_pick_hero_role(&mut self, v: ::std::string::String) {
        self.suggest_pick_hero_role = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_suggest_pick_hero_role(&mut self) -> &mut ::std::string::String {
        if self.suggest_pick_hero_role.is_none() {
            self.suggest_pick_hero_role.set_default();
        };
        self.suggest_pick_hero_role.as_mut().unwrap()
    }

    // Take field
    pub fn take_suggest_pick_hero_role(&mut self) -> ::std::string::String {
        self.suggest_pick_hero_role.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_suggest_pick_hero_role(&self) -> &str {
        match self.suggest_pick_hero_role.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_suggest_pick_hero_role_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.suggest_pick_hero_role
    }

    fn mut_suggest_pick_hero_role_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.suggest_pick_hero_role
    }

    // optional bool terse = 28;

    pub fn clear_terse(&mut self) {
        self.terse = ::std::option::Option::None;
    }

    pub fn has_terse(&self) -> bool {
        self.terse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_terse(&mut self, v: bool) {
        self.terse = ::std::option::Option::Some(v);
    }

    pub fn get_terse(&self) -> bool {
        self.terse.unwrap_or(false)
    }

    fn get_terse_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.terse
    }

    fn mut_terse_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.terse
    }
}

impl ::protobuf::Message for CMsgDOTAChatMessage {
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
                    let tmp = is.read_uint64()?;
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.persona_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.suggest_invite_account_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.suggest_invite_name)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.fantasy_draft_owner_account_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.fantasy_draft_player_account_id = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.event_id = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.suggest_invite_to_lobby = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.event_points = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.coin_flip = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.share_profile_account_id = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel_user_id = ::std::option::Option::Some(tmp);
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dice_roll)?;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.share_party_id = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.share_lobby_id = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.share_lobby_custom_game_id = ::std::option::Option::Some(tmp);
                },
                21 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.share_lobby_passkey)?;
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.private_chat_channel_id = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.legacy_battle_cup_victory = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.battle_cup_streak = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.badge_level = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.suggest_pick_hero_id = ::std::option::Option::Some(tmp);
                },
                27 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.suggest_pick_hero_role)?;
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.terse = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.channel_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.persona_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.suggest_invite_account_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.suggest_invite_name.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.fantasy_draft_owner_account_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.fantasy_draft_player_account_id {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.event_id {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.suggest_invite_to_lobby {
            my_size += 2;
        };
        if let Some(v) = self.event_points {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.coin_flip {
            my_size += 2;
        };
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.share_profile_account_id {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.channel_user_id {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dice_roll.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.share_party_id {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.share_lobby_id {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.share_lobby_custom_game_id {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.share_lobby_passkey.as_ref() {
            my_size += ::protobuf::rt::string_size(21, &v);
        };
        if let Some(v) = self.private_chat_channel_id {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.legacy_battle_cup_victory {
            my_size += 3;
        };
        if let Some(v) = self.battle_cup_streak {
            my_size += ::protobuf::rt::value_size(29, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.badge_level {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.suggest_pick_hero_id {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.suggest_pick_hero_role.as_ref() {
            my_size += ::protobuf::rt::string_size(27, &v);
        };
        if let Some(v) = self.terse {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.channel_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.persona_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.text.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.timestamp {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.suggest_invite_account_id {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.suggest_invite_name.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.fantasy_draft_owner_account_id {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.fantasy_draft_player_account_id {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.event_id {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.suggest_invite_to_lobby {
            os.write_bool(11, v)?;
        };
        if let Some(v) = self.event_points {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.coin_flip {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.player_id {
            os.write_int32(14, v)?;
        };
        if let Some(v) = self.share_profile_account_id {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.channel_user_id {
            os.write_uint32(16, v)?;
        };
        if let Some(v) = self.dice_roll.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.share_party_id {
            os.write_uint64(18, v)?;
        };
        if let Some(v) = self.share_lobby_id {
            os.write_uint64(19, v)?;
        };
        if let Some(v) = self.share_lobby_custom_game_id {
            os.write_uint64(20, v)?;
        };
        if let Some(v) = self.share_lobby_passkey.as_ref() {
            os.write_string(21, &v)?;
        };
        if let Some(v) = self.private_chat_channel_id {
            os.write_uint32(22, v)?;
        };
        if let Some(v) = self.status {
            os.write_uint32(23, v)?;
        };
        if let Some(v) = self.legacy_battle_cup_victory {
            os.write_bool(24, v)?;
        };
        if let Some(v) = self.battle_cup_streak {
            os.write_uint32(29, v)?;
        };
        if let Some(v) = self.badge_level {
            os.write_uint32(25, v)?;
        };
        if let Some(v) = self.suggest_pick_hero_id {
            os.write_uint32(26, v)?;
        };
        if let Some(v) = self.suggest_pick_hero_role.as_ref() {
            os.write_string(27, &v)?;
        };
        if let Some(v) = self.terse {
            os.write_bool(28, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgDOTAChatMessage {
    fn new() -> CMsgDOTAChatMessage {
        CMsgDOTAChatMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAChatMessage::get_account_id_for_reflect,
                    CMsgDOTAChatMessage::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "channel_id",
                    CMsgDOTAChatMessage::get_channel_id_for_reflect,
                    CMsgDOTAChatMessage::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "persona_name",
                    CMsgDOTAChatMessage::get_persona_name_for_reflect,
                    CMsgDOTAChatMessage::mut_persona_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CMsgDOTAChatMessage::get_text_for_reflect,
                    CMsgDOTAChatMessage::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timestamp",
                    CMsgDOTAChatMessage::get_timestamp_for_reflect,
                    CMsgDOTAChatMessage::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "suggest_invite_account_id",
                    CMsgDOTAChatMessage::get_suggest_invite_account_id_for_reflect,
                    CMsgDOTAChatMessage::mut_suggest_invite_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "suggest_invite_name",
                    CMsgDOTAChatMessage::get_suggest_invite_name_for_reflect,
                    CMsgDOTAChatMessage::mut_suggest_invite_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "fantasy_draft_owner_account_id",
                    CMsgDOTAChatMessage::get_fantasy_draft_owner_account_id_for_reflect,
                    CMsgDOTAChatMessage::mut_fantasy_draft_owner_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "fantasy_draft_player_account_id",
                    CMsgDOTAChatMessage::get_fantasy_draft_player_account_id_for_reflect,
                    CMsgDOTAChatMessage::mut_fantasy_draft_player_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "event_id",
                    CMsgDOTAChatMessage::get_event_id_for_reflect,
                    CMsgDOTAChatMessage::mut_event_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "suggest_invite_to_lobby",
                    CMsgDOTAChatMessage::get_suggest_invite_to_lobby_for_reflect,
                    CMsgDOTAChatMessage::mut_suggest_invite_to_lobby_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "event_points",
                    CMsgDOTAChatMessage::get_event_points_for_reflect,
                    CMsgDOTAChatMessage::mut_event_points_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "coin_flip",
                    CMsgDOTAChatMessage::get_coin_flip_for_reflect,
                    CMsgDOTAChatMessage::mut_coin_flip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_id",
                    CMsgDOTAChatMessage::get_player_id_for_reflect,
                    CMsgDOTAChatMessage::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "share_profile_account_id",
                    CMsgDOTAChatMessage::get_share_profile_account_id_for_reflect,
                    CMsgDOTAChatMessage::mut_share_profile_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_user_id",
                    CMsgDOTAChatMessage::get_channel_user_id_for_reflect,
                    CMsgDOTAChatMessage::mut_channel_user_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAChatMessage_DiceRoll>>(
                    "dice_roll",
                    CMsgDOTAChatMessage::get_dice_roll_for_reflect,
                    CMsgDOTAChatMessage::mut_dice_roll_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "share_party_id",
                    CMsgDOTAChatMessage::get_share_party_id_for_reflect,
                    CMsgDOTAChatMessage::mut_share_party_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "share_lobby_id",
                    CMsgDOTAChatMessage::get_share_lobby_id_for_reflect,
                    CMsgDOTAChatMessage::mut_share_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "share_lobby_custom_game_id",
                    CMsgDOTAChatMessage::get_share_lobby_custom_game_id_for_reflect,
                    CMsgDOTAChatMessage::mut_share_lobby_custom_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "share_lobby_passkey",
                    CMsgDOTAChatMessage::get_share_lobby_passkey_for_reflect,
                    CMsgDOTAChatMessage::mut_share_lobby_passkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "private_chat_channel_id",
                    CMsgDOTAChatMessage::get_private_chat_channel_id_for_reflect,
                    CMsgDOTAChatMessage::mut_private_chat_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    CMsgDOTAChatMessage::get_status_for_reflect,
                    CMsgDOTAChatMessage::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "legacy_battle_cup_victory",
                    CMsgDOTAChatMessage::get_legacy_battle_cup_victory_for_reflect,
                    CMsgDOTAChatMessage::mut_legacy_battle_cup_victory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "battle_cup_streak",
                    CMsgDOTAChatMessage::get_battle_cup_streak_for_reflect,
                    CMsgDOTAChatMessage::mut_battle_cup_streak_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "badge_level",
                    CMsgDOTAChatMessage::get_badge_level_for_reflect,
                    CMsgDOTAChatMessage::mut_badge_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "suggest_pick_hero_id",
                    CMsgDOTAChatMessage::get_suggest_pick_hero_id_for_reflect,
                    CMsgDOTAChatMessage::mut_suggest_pick_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "suggest_pick_hero_role",
                    CMsgDOTAChatMessage::get_suggest_pick_hero_role_for_reflect,
                    CMsgDOTAChatMessage::mut_suggest_pick_hero_role_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "terse",
                    CMsgDOTAChatMessage::get_terse_for_reflect,
                    CMsgDOTAChatMessage::mut_terse_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatMessage>(
                    "CMsgDOTAChatMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatMessage {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_channel_id();
        self.clear_persona_name();
        self.clear_text();
        self.clear_timestamp();
        self.clear_suggest_invite_account_id();
        self.clear_suggest_invite_name();
        self.clear_fantasy_draft_owner_account_id();
        self.clear_fantasy_draft_player_account_id();
        self.clear_event_id();
        self.clear_suggest_invite_to_lobby();
        self.clear_event_points();
        self.clear_coin_flip();
        self.clear_player_id();
        self.clear_share_profile_account_id();
        self.clear_channel_user_id();
        self.clear_dice_roll();
        self.clear_share_party_id();
        self.clear_share_lobby_id();
        self.clear_share_lobby_custom_game_id();
        self.clear_share_lobby_passkey();
        self.clear_private_chat_channel_id();
        self.clear_status();
        self.clear_legacy_battle_cup_victory();
        self.clear_battle_cup_streak();
        self.clear_badge_level();
        self.clear_suggest_pick_hero_id();
        self.clear_suggest_pick_hero_role();
        self.clear_terse();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatMessage_DiceRoll {
    // message fields
    roll_min: ::std::option::Option<i32>,
    roll_max: ::std::option::Option<i32>,
    result: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatMessage_DiceRoll {}

impl CMsgDOTAChatMessage_DiceRoll {
    pub fn new() -> CMsgDOTAChatMessage_DiceRoll {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatMessage_DiceRoll {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatMessage_DiceRoll> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatMessage_DiceRoll,
        };
        unsafe {
            instance.get(CMsgDOTAChatMessage_DiceRoll::new)
        }
    }

    // optional int32 roll_min = 1;

    pub fn clear_roll_min(&mut self) {
        self.roll_min = ::std::option::Option::None;
    }

    pub fn has_roll_min(&self) -> bool {
        self.roll_min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_roll_min(&mut self, v: i32) {
        self.roll_min = ::std::option::Option::Some(v);
    }

    pub fn get_roll_min(&self) -> i32 {
        self.roll_min.unwrap_or(0)
    }

    fn get_roll_min_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.roll_min
    }

    fn mut_roll_min_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.roll_min
    }

    // optional int32 roll_max = 2;

    pub fn clear_roll_max(&mut self) {
        self.roll_max = ::std::option::Option::None;
    }

    pub fn has_roll_max(&self) -> bool {
        self.roll_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_roll_max(&mut self, v: i32) {
        self.roll_max = ::std::option::Option::Some(v);
    }

    pub fn get_roll_max(&self) -> i32 {
        self.roll_max.unwrap_or(0)
    }

    fn get_roll_max_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.roll_max
    }

    fn mut_roll_max_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.roll_max
    }

    // optional int32 result = 3;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: i32) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> i32 {
        self.result.unwrap_or(0)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAChatMessage_DiceRoll {
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
                    self.roll_min = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.roll_max = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
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
        if let Some(v) = self.roll_min {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.roll_max {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.roll_min {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.roll_max {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.result {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatMessage_DiceRoll {
    fn new() -> CMsgDOTAChatMessage_DiceRoll {
        CMsgDOTAChatMessage_DiceRoll::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatMessage_DiceRoll>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "roll_min",
                    CMsgDOTAChatMessage_DiceRoll::get_roll_min_for_reflect,
                    CMsgDOTAChatMessage_DiceRoll::mut_roll_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "roll_max",
                    CMsgDOTAChatMessage_DiceRoll::get_roll_max_for_reflect,
                    CMsgDOTAChatMessage_DiceRoll::mut_roll_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "result",
                    CMsgDOTAChatMessage_DiceRoll::get_result_for_reflect,
                    CMsgDOTAChatMessage_DiceRoll::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatMessage_DiceRoll>(
                    "CMsgDOTAChatMessage_DiceRoll",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatMessage_DiceRoll {
    fn clear(&mut self) {
        self.clear_roll_min();
        self.clear_roll_max();
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatMessage_DiceRoll {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatMessage_DiceRoll {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatMember {
    // message fields
    steam_id: ::std::option::Option<u64>,
    persona_name: ::protobuf::SingularField<::std::string::String>,
    channel_user_id: ::std::option::Option<u32>,
    status: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatMember {}

impl CMsgDOTAChatMember {
    pub fn new() -> CMsgDOTAChatMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatMember {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatMember,
        };
        unsafe {
            instance.get(CMsgDOTAChatMember::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional string persona_name = 2;

    pub fn clear_persona_name(&mut self) {
        self.persona_name.clear();
    }

    pub fn has_persona_name(&self) -> bool {
        self.persona_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persona_name(&mut self, v: ::std::string::String) {
        self.persona_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_persona_name(&mut self) -> &mut ::std::string::String {
        if self.persona_name.is_none() {
            self.persona_name.set_default();
        };
        self.persona_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_persona_name(&mut self) -> ::std::string::String {
        self.persona_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_persona_name(&self) -> &str {
        match self.persona_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_persona_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.persona_name
    }

    fn mut_persona_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.persona_name
    }

    // optional uint32 channel_user_id = 3;

    pub fn clear_channel_user_id(&mut self) {
        self.channel_user_id = ::std::option::Option::None;
    }

    pub fn has_channel_user_id(&self) -> bool {
        self.channel_user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_user_id(&mut self, v: u32) {
        self.channel_user_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_user_id(&self) -> u32 {
        self.channel_user_id.unwrap_or(0)
    }

    fn get_channel_user_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_user_id
    }

    fn mut_channel_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_user_id
    }

    // optional uint32 status = 4;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> u32 {
        self.status.unwrap_or(0)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status
    }
}

impl ::protobuf::Message for CMsgDOTAChatMember {
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
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.persona_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel_user_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        };
        if let Some(v) = self.persona_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.channel_user_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.persona_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.channel_user_id {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.status {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatMember {
    fn new() -> CMsgDOTAChatMember {
        CMsgDOTAChatMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgDOTAChatMember::get_steam_id_for_reflect,
                    CMsgDOTAChatMember::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "persona_name",
                    CMsgDOTAChatMember::get_persona_name_for_reflect,
                    CMsgDOTAChatMember::mut_persona_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_user_id",
                    CMsgDOTAChatMember::get_channel_user_id_for_reflect,
                    CMsgDOTAChatMember::mut_channel_user_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    CMsgDOTAChatMember::get_status_for_reflect,
                    CMsgDOTAChatMember::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatMember>(
                    "CMsgDOTAChatMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatMember {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_persona_name();
        self.clear_channel_user_id();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAJoinChatChannelResponse {
    // message fields
    response: ::std::option::Option<u32>,
    channel_name: ::protobuf::SingularField<::std::string::String>,
    channel_id: ::std::option::Option<u64>,
    max_members: ::std::option::Option<u32>,
    members: ::protobuf::RepeatedField<CMsgDOTAChatMember>,
    channel_type: ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t>,
    result: ::std::option::Option<CMsgDOTAJoinChatChannelResponse_Result>,
    gc_initiated_join: ::std::option::Option<bool>,
    channel_user_id: ::std::option::Option<u32>,
    welcome_message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAJoinChatChannelResponse {}

impl CMsgDOTAJoinChatChannelResponse {
    pub fn new() -> CMsgDOTAJoinChatChannelResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAJoinChatChannelResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAJoinChatChannelResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAJoinChatChannelResponse,
        };
        unsafe {
            instance.get(CMsgDOTAJoinChatChannelResponse::new)
        }
    }

    // optional uint32 response = 1;

    pub fn clear_response(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_response(&self) -> bool {
        self.response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_response(&mut self, v: u32) {
        self.response = ::std::option::Option::Some(v);
    }

    pub fn get_response(&self) -> u32 {
        self.response.unwrap_or(0)
    }

    fn get_response_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.response
    }

    fn mut_response_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.response
    }

    // optional string channel_name = 2;

    pub fn clear_channel_name(&mut self) {
        self.channel_name.clear();
    }

    pub fn has_channel_name(&self) -> bool {
        self.channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_name(&mut self, v: ::std::string::String) {
        self.channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_name(&mut self) -> &mut ::std::string::String {
        if self.channel_name.is_none() {
            self.channel_name.set_default();
        };
        self.channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel_name(&mut self) -> ::std::string::String {
        self.channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel_name(&self) -> &str {
        match self.channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel_name
    }

    fn mut_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel_name
    }

    // optional fixed64 channel_id = 3;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // optional uint32 max_members = 4;

    pub fn clear_max_members(&mut self) {
        self.max_members = ::std::option::Option::None;
    }

    pub fn has_max_members(&self) -> bool {
        self.max_members.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_members(&mut self, v: u32) {
        self.max_members = ::std::option::Option::Some(v);
    }

    pub fn get_max_members(&self) -> u32 {
        self.max_members.unwrap_or(0)
    }

    fn get_max_members_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_members
    }

    fn mut_max_members_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_members
    }

    // repeated .dota.CMsgDOTAChatMember members = 5;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAChatMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAChatMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CMsgDOTAChatMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAChatMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatMember> {
        &mut self.members
    }

    // optional .dota.DOTAChatChannelType_t channel_type = 6;

    pub fn clear_channel_type(&mut self) {
        self.channel_type = ::std::option::Option::None;
    }

    pub fn has_channel_type(&self) -> bool {
        self.channel_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_type(&mut self, v: super::dota_shared_enums::DOTAChatChannelType_t) {
        self.channel_type = ::std::option::Option::Some(v);
    }

    pub fn get_channel_type(&self) -> super::dota_shared_enums::DOTAChatChannelType_t {
        self.channel_type.unwrap_or(super::dota_shared_enums::DOTAChatChannelType_t::DOTAChannelType_Regional)
    }

    fn get_channel_type_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &self.channel_type
    }

    fn mut_channel_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &mut self.channel_type
    }

    // optional .dota.CMsgDOTAJoinChatChannelResponse.Result result = 7;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: CMsgDOTAJoinChatChannelResponse_Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> CMsgDOTAJoinChatChannelResponse_Result {
        self.result.unwrap_or(CMsgDOTAJoinChatChannelResponse_Result::JOIN_SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<CMsgDOTAJoinChatChannelResponse_Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgDOTAJoinChatChannelResponse_Result> {
        &mut self.result
    }

    // optional bool gc_initiated_join = 8;

    pub fn clear_gc_initiated_join(&mut self) {
        self.gc_initiated_join = ::std::option::Option::None;
    }

    pub fn has_gc_initiated_join(&self) -> bool {
        self.gc_initiated_join.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_initiated_join(&mut self, v: bool) {
        self.gc_initiated_join = ::std::option::Option::Some(v);
    }

    pub fn get_gc_initiated_join(&self) -> bool {
        self.gc_initiated_join.unwrap_or(false)
    }

    fn get_gc_initiated_join_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.gc_initiated_join
    }

    fn mut_gc_initiated_join_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.gc_initiated_join
    }

    // optional uint32 channel_user_id = 9;

    pub fn clear_channel_user_id(&mut self) {
        self.channel_user_id = ::std::option::Option::None;
    }

    pub fn has_channel_user_id(&self) -> bool {
        self.channel_user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_user_id(&mut self, v: u32) {
        self.channel_user_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_user_id(&self) -> u32 {
        self.channel_user_id.unwrap_or(0)
    }

    fn get_channel_user_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_user_id
    }

    fn mut_channel_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_user_id
    }

    // optional string welcome_message = 10;

    pub fn clear_welcome_message(&mut self) {
        self.welcome_message.clear();
    }

    pub fn has_welcome_message(&self) -> bool {
        self.welcome_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_welcome_message(&mut self, v: ::std::string::String) {
        self.welcome_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_welcome_message(&mut self) -> &mut ::std::string::String {
        if self.welcome_message.is_none() {
            self.welcome_message.set_default();
        };
        self.welcome_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_welcome_message(&mut self) -> ::std::string::String {
        self.welcome_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_welcome_message(&self) -> &str {
        match self.welcome_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_welcome_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.welcome_message
    }

    fn mut_welcome_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.welcome_message
    }
}

impl ::protobuf::Message for CMsgDOTAJoinChatChannelResponse {
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
                    self.response = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_members = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.channel_type = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.gc_initiated_join = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel_user_id = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.welcome_message)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.response {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.channel_id {
            my_size += 9;
        };
        if let Some(v) = self.max_members {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.channel_type {
            my_size += ::protobuf::rt::enum_size(6, v);
        };
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(7, v);
        };
        if let Some(v) = self.gc_initiated_join {
            my_size += 2;
        };
        if let Some(v) = self.channel_user_id {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.welcome_message.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.response {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.channel_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.channel_id {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.max_members {
            os.write_uint32(4, v)?;
        };
        for v in &self.members {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.channel_type {
            os.write_enum(6, v.value())?;
        };
        if let Some(v) = self.result {
            os.write_enum(7, v.value())?;
        };
        if let Some(v) = self.gc_initiated_join {
            os.write_bool(8, v)?;
        };
        if let Some(v) = self.channel_user_id {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.welcome_message.as_ref() {
            os.write_string(10, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgDOTAJoinChatChannelResponse {
    fn new() -> CMsgDOTAJoinChatChannelResponse {
        CMsgDOTAJoinChatChannelResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAJoinChatChannelResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "response",
                    CMsgDOTAJoinChatChannelResponse::get_response_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel_name",
                    CMsgDOTAJoinChatChannelResponse::get_channel_name_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "channel_id",
                    CMsgDOTAJoinChatChannelResponse::get_channel_id_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_members",
                    CMsgDOTAJoinChatChannelResponse::get_max_members_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_max_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAChatMember>>(
                    "members",
                    CMsgDOTAJoinChatChannelResponse::get_members_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAChatChannelType_t>>(
                    "channel_type",
                    CMsgDOTAJoinChatChannelResponse::get_channel_type_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_channel_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgDOTAJoinChatChannelResponse_Result>>(
                    "result",
                    CMsgDOTAJoinChatChannelResponse::get_result_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "gc_initiated_join",
                    CMsgDOTAJoinChatChannelResponse::get_gc_initiated_join_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_gc_initiated_join_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_user_id",
                    CMsgDOTAJoinChatChannelResponse::get_channel_user_id_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_channel_user_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "welcome_message",
                    CMsgDOTAJoinChatChannelResponse::get_welcome_message_for_reflect,
                    CMsgDOTAJoinChatChannelResponse::mut_welcome_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAJoinChatChannelResponse>(
                    "CMsgDOTAJoinChatChannelResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAJoinChatChannelResponse {
    fn clear(&mut self) {
        self.clear_response();
        self.clear_channel_name();
        self.clear_channel_id();
        self.clear_max_members();
        self.clear_members();
        self.clear_channel_type();
        self.clear_result();
        self.clear_gc_initiated_join();
        self.clear_channel_user_id();
        self.clear_welcome_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAJoinChatChannelResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAJoinChatChannelResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgDOTAJoinChatChannelResponse_Result {
    JOIN_SUCCESS = 0,
    INVALID_CHANNEL_TYPE = 1,
    ACCOUNT_NOT_FOUND = 2,
    ACH_FAILED = 3,
    USER_IN_TOO_MANY_CHANNELS = 4,
    RATE_LIMIT_EXCEEDED = 5,
    CHANNEL_FULL = 6,
    CHANNEL_FULL_OVERFLOWED = 7,
    FAILED_TO_ADD_USER = 8,
    CHANNEL_TYPE_DISABLED = 9,
    PRIVATE_CHAT_CREATE_FAILED = 10,
    PRIVATE_CHAT_NO_PERMISSION = 11,
    PRIVATE_CHAT_CREATE_LOCK_FAILED = 12,
    PRIVATE_CHAT_KICKED = 13,
}

impl ::protobuf::ProtobufEnum for CMsgDOTAJoinChatChannelResponse_Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgDOTAJoinChatChannelResponse_Result> {
        match value {
            0 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::JOIN_SUCCESS),
            1 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::INVALID_CHANNEL_TYPE),
            2 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::ACCOUNT_NOT_FOUND),
            3 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::ACH_FAILED),
            4 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::USER_IN_TOO_MANY_CHANNELS),
            5 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::RATE_LIMIT_EXCEEDED),
            6 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::CHANNEL_FULL),
            7 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::CHANNEL_FULL_OVERFLOWED),
            8 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::FAILED_TO_ADD_USER),
            9 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::CHANNEL_TYPE_DISABLED),
            10 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::PRIVATE_CHAT_CREATE_FAILED),
            11 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::PRIVATE_CHAT_NO_PERMISSION),
            12 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::PRIVATE_CHAT_CREATE_LOCK_FAILED),
            13 => ::std::option::Option::Some(CMsgDOTAJoinChatChannelResponse_Result::PRIVATE_CHAT_KICKED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgDOTAJoinChatChannelResponse_Result] = &[
            CMsgDOTAJoinChatChannelResponse_Result::JOIN_SUCCESS,
            CMsgDOTAJoinChatChannelResponse_Result::INVALID_CHANNEL_TYPE,
            CMsgDOTAJoinChatChannelResponse_Result::ACCOUNT_NOT_FOUND,
            CMsgDOTAJoinChatChannelResponse_Result::ACH_FAILED,
            CMsgDOTAJoinChatChannelResponse_Result::USER_IN_TOO_MANY_CHANNELS,
            CMsgDOTAJoinChatChannelResponse_Result::RATE_LIMIT_EXCEEDED,
            CMsgDOTAJoinChatChannelResponse_Result::CHANNEL_FULL,
            CMsgDOTAJoinChatChannelResponse_Result::CHANNEL_FULL_OVERFLOWED,
            CMsgDOTAJoinChatChannelResponse_Result::FAILED_TO_ADD_USER,
            CMsgDOTAJoinChatChannelResponse_Result::CHANNEL_TYPE_DISABLED,
            CMsgDOTAJoinChatChannelResponse_Result::PRIVATE_CHAT_CREATE_FAILED,
            CMsgDOTAJoinChatChannelResponse_Result::PRIVATE_CHAT_NO_PERMISSION,
            CMsgDOTAJoinChatChannelResponse_Result::PRIVATE_CHAT_CREATE_LOCK_FAILED,
            CMsgDOTAJoinChatChannelResponse_Result::PRIVATE_CHAT_KICKED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgDOTAJoinChatChannelResponse_Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgDOTAJoinChatChannelResponse_Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgDOTAJoinChatChannelResponse_Result {
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAJoinChatChannelResponse_Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatChannelFullUpdate {
    // message fields
    channel_id: ::std::option::Option<u64>,
    members: ::protobuf::RepeatedField<CMsgDOTAChatMember>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatChannelFullUpdate {}

impl CMsgDOTAChatChannelFullUpdate {
    pub fn new() -> CMsgDOTAChatChannelFullUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatChannelFullUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatChannelFullUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatChannelFullUpdate,
        };
        unsafe {
            instance.get(CMsgDOTAChatChannelFullUpdate::new)
        }
    }

    // optional fixed64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // repeated .dota.CMsgDOTAChatMember members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAChatMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAChatMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CMsgDOTAChatMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAChatMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatMember> {
        &mut self.members
    }
}

impl ::protobuf::Message for CMsgDOTAChatChannelFullUpdate {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel_id {
            my_size += 9;
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_fixed64(1, v)?;
        };
        for v in &self.members {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatChannelFullUpdate {
    fn new() -> CMsgDOTAChatChannelFullUpdate {
        CMsgDOTAChatChannelFullUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatChannelFullUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "channel_id",
                    CMsgDOTAChatChannelFullUpdate::get_channel_id_for_reflect,
                    CMsgDOTAChatChannelFullUpdate::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAChatMember>>(
                    "members",
                    CMsgDOTAChatChannelFullUpdate::get_members_for_reflect,
                    CMsgDOTAChatChannelFullUpdate::mut_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatChannelFullUpdate>(
                    "CMsgDOTAChatChannelFullUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatChannelFullUpdate {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatChannelFullUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatChannelFullUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAOtherJoinedChatChannel {
    // message fields
    channel_id: ::std::option::Option<u64>,
    persona_name: ::protobuf::SingularField<::std::string::String>,
    steam_id: ::std::option::Option<u64>,
    channel_user_id: ::std::option::Option<u32>,
    status: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAOtherJoinedChatChannel {}

impl CMsgDOTAOtherJoinedChatChannel {
    pub fn new() -> CMsgDOTAOtherJoinedChatChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAOtherJoinedChatChannel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAOtherJoinedChatChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAOtherJoinedChatChannel,
        };
        unsafe {
            instance.get(CMsgDOTAOtherJoinedChatChannel::new)
        }
    }

    // optional fixed64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // optional string persona_name = 2;

    pub fn clear_persona_name(&mut self) {
        self.persona_name.clear();
    }

    pub fn has_persona_name(&self) -> bool {
        self.persona_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persona_name(&mut self, v: ::std::string::String) {
        self.persona_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_persona_name(&mut self) -> &mut ::std::string::String {
        if self.persona_name.is_none() {
            self.persona_name.set_default();
        };
        self.persona_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_persona_name(&mut self) -> ::std::string::String {
        self.persona_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_persona_name(&self) -> &str {
        match self.persona_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_persona_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.persona_name
    }

    fn mut_persona_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.persona_name
    }

    // optional fixed64 steam_id = 3;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional uint32 channel_user_id = 4;

    pub fn clear_channel_user_id(&mut self) {
        self.channel_user_id = ::std::option::Option::None;
    }

    pub fn has_channel_user_id(&self) -> bool {
        self.channel_user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_user_id(&mut self, v: u32) {
        self.channel_user_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_user_id(&self) -> u32 {
        self.channel_user_id.unwrap_or(0)
    }

    fn get_channel_user_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_user_id
    }

    fn mut_channel_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_user_id
    }

    // optional uint32 status = 5;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> u32 {
        self.status.unwrap_or(0)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status
    }
}

impl ::protobuf::Message for CMsgDOTAOtherJoinedChatChannel {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.persona_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel_user_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel_id {
            my_size += 9;
        };
        if let Some(v) = self.persona_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.steam_id {
            my_size += 9;
        };
        if let Some(v) = self.channel_user_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.persona_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.steam_id {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.channel_user_id {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.status {
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

impl ::protobuf::MessageStatic for CMsgDOTAOtherJoinedChatChannel {
    fn new() -> CMsgDOTAOtherJoinedChatChannel {
        CMsgDOTAOtherJoinedChatChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAOtherJoinedChatChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "channel_id",
                    CMsgDOTAOtherJoinedChatChannel::get_channel_id_for_reflect,
                    CMsgDOTAOtherJoinedChatChannel::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "persona_name",
                    CMsgDOTAOtherJoinedChatChannel::get_persona_name_for_reflect,
                    CMsgDOTAOtherJoinedChatChannel::mut_persona_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgDOTAOtherJoinedChatChannel::get_steam_id_for_reflect,
                    CMsgDOTAOtherJoinedChatChannel::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_user_id",
                    CMsgDOTAOtherJoinedChatChannel::get_channel_user_id_for_reflect,
                    CMsgDOTAOtherJoinedChatChannel::mut_channel_user_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    CMsgDOTAOtherJoinedChatChannel::get_status_for_reflect,
                    CMsgDOTAOtherJoinedChatChannel::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAOtherJoinedChatChannel>(
                    "CMsgDOTAOtherJoinedChatChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAOtherJoinedChatChannel {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_persona_name();
        self.clear_steam_id();
        self.clear_channel_user_id();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAOtherJoinedChatChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAOtherJoinedChatChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAOtherLeftChatChannel {
    // message fields
    channel_id: ::std::option::Option<u64>,
    steam_id: ::std::option::Option<u64>,
    channel_user_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAOtherLeftChatChannel {}

impl CMsgDOTAOtherLeftChatChannel {
    pub fn new() -> CMsgDOTAOtherLeftChatChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAOtherLeftChatChannel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAOtherLeftChatChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAOtherLeftChatChannel,
        };
        unsafe {
            instance.get(CMsgDOTAOtherLeftChatChannel::new)
        }
    }

    // optional fixed64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // optional fixed64 steam_id = 2;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional uint32 channel_user_id = 3;

    pub fn clear_channel_user_id(&mut self) {
        self.channel_user_id = ::std::option::Option::None;
    }

    pub fn has_channel_user_id(&self) -> bool {
        self.channel_user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_user_id(&mut self, v: u32) {
        self.channel_user_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_user_id(&self) -> u32 {
        self.channel_user_id.unwrap_or(0)
    }

    fn get_channel_user_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_user_id
    }

    fn mut_channel_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_user_id
    }
}

impl ::protobuf::Message for CMsgDOTAOtherLeftChatChannel {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel_user_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel_id {
            my_size += 9;
        };
        if let Some(v) = self.steam_id {
            my_size += 9;
        };
        if let Some(v) = self.channel_user_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.steam_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.channel_user_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAOtherLeftChatChannel {
    fn new() -> CMsgDOTAOtherLeftChatChannel {
        CMsgDOTAOtherLeftChatChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAOtherLeftChatChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "channel_id",
                    CMsgDOTAOtherLeftChatChannel::get_channel_id_for_reflect,
                    CMsgDOTAOtherLeftChatChannel::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgDOTAOtherLeftChatChannel::get_steam_id_for_reflect,
                    CMsgDOTAOtherLeftChatChannel::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_user_id",
                    CMsgDOTAOtherLeftChatChannel::get_channel_user_id_for_reflect,
                    CMsgDOTAOtherLeftChatChannel::mut_channel_user_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAOtherLeftChatChannel>(
                    "CMsgDOTAOtherLeftChatChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAOtherLeftChatChannel {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_steam_id();
        self.clear_channel_user_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAOtherLeftChatChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAOtherLeftChatChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatChannelMemberUpdate {
    // message fields
    channel_id: ::std::option::Option<u64>,
    left_steam_ids: ::std::vec::Vec<u64>,
    joined_members: ::protobuf::RepeatedField<CMsgDOTAChatChannelMemberUpdate_JoinedMember>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatChannelMemberUpdate {}

impl CMsgDOTAChatChannelMemberUpdate {
    pub fn new() -> CMsgDOTAChatChannelMemberUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatChannelMemberUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatChannelMemberUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatChannelMemberUpdate,
        };
        unsafe {
            instance.get(CMsgDOTAChatChannelMemberUpdate::new)
        }
    }

    // optional fixed64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // repeated fixed64 left_steam_ids = 2;

    pub fn clear_left_steam_ids(&mut self) {
        self.left_steam_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_left_steam_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.left_steam_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_left_steam_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.left_steam_ids
    }

    // Take field
    pub fn take_left_steam_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.left_steam_ids, ::std::vec::Vec::new())
    }

    pub fn get_left_steam_ids(&self) -> &[u64] {
        &self.left_steam_ids
    }

    fn get_left_steam_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.left_steam_ids
    }

    fn mut_left_steam_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.left_steam_ids
    }

    // repeated .dota.CMsgDOTAChatChannelMemberUpdate.JoinedMember joined_members = 3;

    pub fn clear_joined_members(&mut self) {
        self.joined_members.clear();
    }

    // Param is passed by value, moved
    pub fn set_joined_members(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAChatChannelMemberUpdate_JoinedMember>) {
        self.joined_members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_joined_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatChannelMemberUpdate_JoinedMember> {
        &mut self.joined_members
    }

    // Take field
    pub fn take_joined_members(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAChatChannelMemberUpdate_JoinedMember> {
        ::std::mem::replace(&mut self.joined_members, ::protobuf::RepeatedField::new())
    }

    pub fn get_joined_members(&self) -> &[CMsgDOTAChatChannelMemberUpdate_JoinedMember] {
        &self.joined_members
    }

    fn get_joined_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAChatChannelMemberUpdate_JoinedMember> {
        &self.joined_members
    }

    fn mut_joined_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatChannelMemberUpdate_JoinedMember> {
        &mut self.joined_members
    }
}

impl ::protobuf::Message for CMsgDOTAChatChannelMemberUpdate {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.left_steam_ids)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.joined_members)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel_id {
            my_size += 9;
        };
        my_size += 9 * self.left_steam_ids.len() as u32;
        for value in &self.joined_members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_fixed64(1, v)?;
        };
        for v in &self.left_steam_ids {
            os.write_fixed64(2, *v)?;
        };
        for v in &self.joined_members {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatChannelMemberUpdate {
    fn new() -> CMsgDOTAChatChannelMemberUpdate {
        CMsgDOTAChatChannelMemberUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatChannelMemberUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "channel_id",
                    CMsgDOTAChatChannelMemberUpdate::get_channel_id_for_reflect,
                    CMsgDOTAChatChannelMemberUpdate::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "left_steam_ids",
                    CMsgDOTAChatChannelMemberUpdate::get_left_steam_ids_for_reflect,
                    CMsgDOTAChatChannelMemberUpdate::mut_left_steam_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAChatChannelMemberUpdate_JoinedMember>>(
                    "joined_members",
                    CMsgDOTAChatChannelMemberUpdate::get_joined_members_for_reflect,
                    CMsgDOTAChatChannelMemberUpdate::mut_joined_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatChannelMemberUpdate>(
                    "CMsgDOTAChatChannelMemberUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatChannelMemberUpdate {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_left_steam_ids();
        self.clear_joined_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatChannelMemberUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatChannelMemberUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatChannelMemberUpdate_JoinedMember {
    // message fields
    steam_id: ::std::option::Option<u64>,
    persona_name: ::protobuf::SingularField<::std::string::String>,
    channel_user_id: ::std::option::Option<u32>,
    status: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatChannelMemberUpdate_JoinedMember {}

impl CMsgDOTAChatChannelMemberUpdate_JoinedMember {
    pub fn new() -> CMsgDOTAChatChannelMemberUpdate_JoinedMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatChannelMemberUpdate_JoinedMember {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatChannelMemberUpdate_JoinedMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatChannelMemberUpdate_JoinedMember,
        };
        unsafe {
            instance.get(CMsgDOTAChatChannelMemberUpdate_JoinedMember::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional string persona_name = 2;

    pub fn clear_persona_name(&mut self) {
        self.persona_name.clear();
    }

    pub fn has_persona_name(&self) -> bool {
        self.persona_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persona_name(&mut self, v: ::std::string::String) {
        self.persona_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_persona_name(&mut self) -> &mut ::std::string::String {
        if self.persona_name.is_none() {
            self.persona_name.set_default();
        };
        self.persona_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_persona_name(&mut self) -> ::std::string::String {
        self.persona_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_persona_name(&self) -> &str {
        match self.persona_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_persona_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.persona_name
    }

    fn mut_persona_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.persona_name
    }

    // optional uint32 channel_user_id = 3;

    pub fn clear_channel_user_id(&mut self) {
        self.channel_user_id = ::std::option::Option::None;
    }

    pub fn has_channel_user_id(&self) -> bool {
        self.channel_user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_user_id(&mut self, v: u32) {
        self.channel_user_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_user_id(&self) -> u32 {
        self.channel_user_id.unwrap_or(0)
    }

    fn get_channel_user_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_user_id
    }

    fn mut_channel_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_user_id
    }

    // optional uint32 status = 4;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> u32 {
        self.status.unwrap_or(0)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status
    }
}

impl ::protobuf::Message for CMsgDOTAChatChannelMemberUpdate_JoinedMember {
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
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.persona_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel_user_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        };
        if let Some(v) = self.persona_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.channel_user_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.persona_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.channel_user_id {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.status {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatChannelMemberUpdate_JoinedMember {
    fn new() -> CMsgDOTAChatChannelMemberUpdate_JoinedMember {
        CMsgDOTAChatChannelMemberUpdate_JoinedMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatChannelMemberUpdate_JoinedMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgDOTAChatChannelMemberUpdate_JoinedMember::get_steam_id_for_reflect,
                    CMsgDOTAChatChannelMemberUpdate_JoinedMember::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "persona_name",
                    CMsgDOTAChatChannelMemberUpdate_JoinedMember::get_persona_name_for_reflect,
                    CMsgDOTAChatChannelMemberUpdate_JoinedMember::mut_persona_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_user_id",
                    CMsgDOTAChatChannelMemberUpdate_JoinedMember::get_channel_user_id_for_reflect,
                    CMsgDOTAChatChannelMemberUpdate_JoinedMember::mut_channel_user_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    CMsgDOTAChatChannelMemberUpdate_JoinedMember::get_status_for_reflect,
                    CMsgDOTAChatChannelMemberUpdate_JoinedMember::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatChannelMemberUpdate_JoinedMember>(
                    "CMsgDOTAChatChannelMemberUpdate_JoinedMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatChannelMemberUpdate_JoinedMember {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_persona_name();
        self.clear_channel_user_id();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatChannelMemberUpdate_JoinedMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatChannelMemberUpdate_JoinedMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTARequestChatChannelList {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTARequestChatChannelList {}

impl CMsgDOTARequestChatChannelList {
    pub fn new() -> CMsgDOTARequestChatChannelList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTARequestChatChannelList {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTARequestChatChannelList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTARequestChatChannelList,
        };
        unsafe {
            instance.get(CMsgDOTARequestChatChannelList::new)
        }
    }
}

impl ::protobuf::Message for CMsgDOTARequestChatChannelList {
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

impl ::protobuf::MessageStatic for CMsgDOTARequestChatChannelList {
    fn new() -> CMsgDOTARequestChatChannelList {
        CMsgDOTARequestChatChannelList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTARequestChatChannelList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTARequestChatChannelList>(
                    "CMsgDOTARequestChatChannelList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTARequestChatChannelList {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTARequestChatChannelList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTARequestChatChannelList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTARequestChatChannelListResponse {
    // message fields
    channels: ::protobuf::RepeatedField<CMsgDOTARequestChatChannelListResponse_ChatChannel>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTARequestChatChannelListResponse {}

impl CMsgDOTARequestChatChannelListResponse {
    pub fn new() -> CMsgDOTARequestChatChannelListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTARequestChatChannelListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTARequestChatChannelListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTARequestChatChannelListResponse,
        };
        unsafe {
            instance.get(CMsgDOTARequestChatChannelListResponse::new)
        }
    }

    // repeated .dota.CMsgDOTARequestChatChannelListResponse.ChatChannel channels = 1;

    pub fn clear_channels(&mut self) {
        self.channels.clear();
    }

    // Param is passed by value, moved
    pub fn set_channels(&mut self, v: ::protobuf::RepeatedField<CMsgDOTARequestChatChannelListResponse_ChatChannel>) {
        self.channels = v;
    }

    // Mutable pointer to the field.
    pub fn mut_channels(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTARequestChatChannelListResponse_ChatChannel> {
        &mut self.channels
    }

    // Take field
    pub fn take_channels(&mut self) -> ::protobuf::RepeatedField<CMsgDOTARequestChatChannelListResponse_ChatChannel> {
        ::std::mem::replace(&mut self.channels, ::protobuf::RepeatedField::new())
    }

    pub fn get_channels(&self) -> &[CMsgDOTARequestChatChannelListResponse_ChatChannel] {
        &self.channels
    }

    fn get_channels_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTARequestChatChannelListResponse_ChatChannel> {
        &self.channels
    }

    fn mut_channels_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTARequestChatChannelListResponse_ChatChannel> {
        &mut self.channels
    }
}

impl ::protobuf::Message for CMsgDOTARequestChatChannelListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.channels)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.channels {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.channels {
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

impl ::protobuf::MessageStatic for CMsgDOTARequestChatChannelListResponse {
    fn new() -> CMsgDOTARequestChatChannelListResponse {
        CMsgDOTARequestChatChannelListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTARequestChatChannelListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTARequestChatChannelListResponse_ChatChannel>>(
                    "channels",
                    CMsgDOTARequestChatChannelListResponse::get_channels_for_reflect,
                    CMsgDOTARequestChatChannelListResponse::mut_channels_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTARequestChatChannelListResponse>(
                    "CMsgDOTARequestChatChannelListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTARequestChatChannelListResponse {
    fn clear(&mut self) {
        self.clear_channels();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTARequestChatChannelListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTARequestChatChannelListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTARequestChatChannelListResponse_ChatChannel {
    // message fields
    channel_name: ::protobuf::SingularField<::std::string::String>,
    num_members: ::std::option::Option<u32>,
    channel_type: ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTARequestChatChannelListResponse_ChatChannel {}

impl CMsgDOTARequestChatChannelListResponse_ChatChannel {
    pub fn new() -> CMsgDOTARequestChatChannelListResponse_ChatChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTARequestChatChannelListResponse_ChatChannel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTARequestChatChannelListResponse_ChatChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTARequestChatChannelListResponse_ChatChannel,
        };
        unsafe {
            instance.get(CMsgDOTARequestChatChannelListResponse_ChatChannel::new)
        }
    }

    // optional string channel_name = 1;

    pub fn clear_channel_name(&mut self) {
        self.channel_name.clear();
    }

    pub fn has_channel_name(&self) -> bool {
        self.channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_name(&mut self, v: ::std::string::String) {
        self.channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_name(&mut self) -> &mut ::std::string::String {
        if self.channel_name.is_none() {
            self.channel_name.set_default();
        };
        self.channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel_name(&mut self) -> ::std::string::String {
        self.channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel_name(&self) -> &str {
        match self.channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel_name
    }

    fn mut_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel_name
    }

    // optional uint32 num_members = 2;

    pub fn clear_num_members(&mut self) {
        self.num_members = ::std::option::Option::None;
    }

    pub fn has_num_members(&self) -> bool {
        self.num_members.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_members(&mut self, v: u32) {
        self.num_members = ::std::option::Option::Some(v);
    }

    pub fn get_num_members(&self) -> u32 {
        self.num_members.unwrap_or(0)
    }

    fn get_num_members_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_members
    }

    fn mut_num_members_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_members
    }

    // optional .dota.DOTAChatChannelType_t channel_type = 3;

    pub fn clear_channel_type(&mut self) {
        self.channel_type = ::std::option::Option::None;
    }

    pub fn has_channel_type(&self) -> bool {
        self.channel_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_type(&mut self, v: super::dota_shared_enums::DOTAChatChannelType_t) {
        self.channel_type = ::std::option::Option::Some(v);
    }

    pub fn get_channel_type(&self) -> super::dota_shared_enums::DOTAChatChannelType_t {
        self.channel_type.unwrap_or(super::dota_shared_enums::DOTAChatChannelType_t::DOTAChannelType_Regional)
    }

    fn get_channel_type_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &self.channel_type
    }

    fn mut_channel_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &mut self.channel_type
    }
}

impl ::protobuf::Message for CMsgDOTARequestChatChannelListResponse_ChatChannel {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.num_members = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
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
        if let Some(v) = self.channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.num_members {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.channel_type {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.num_members {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.channel_type {
            os.write_enum(3, v.value())?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgDOTARequestChatChannelListResponse_ChatChannel {
    fn new() -> CMsgDOTARequestChatChannelListResponse_ChatChannel {
        CMsgDOTARequestChatChannelListResponse_ChatChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTARequestChatChannelListResponse_ChatChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel_name",
                    CMsgDOTARequestChatChannelListResponse_ChatChannel::get_channel_name_for_reflect,
                    CMsgDOTARequestChatChannelListResponse_ChatChannel::mut_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_members",
                    CMsgDOTARequestChatChannelListResponse_ChatChannel::get_num_members_for_reflect,
                    CMsgDOTARequestChatChannelListResponse_ChatChannel::mut_num_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAChatChannelType_t>>(
                    "channel_type",
                    CMsgDOTARequestChatChannelListResponse_ChatChannel::get_channel_type_for_reflect,
                    CMsgDOTARequestChatChannelListResponse_ChatChannel::mut_channel_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTARequestChatChannelListResponse_ChatChannel>(
                    "CMsgDOTARequestChatChannelListResponse_ChatChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTARequestChatChannelListResponse_ChatChannel {
    fn clear(&mut self) {
        self.clear_channel_name();
        self.clear_num_members();
        self.clear_channel_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTARequestChatChannelListResponse_ChatChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTARequestChatChannelListResponse_ChatChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatGetUserList {
    // message fields
    channel_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatGetUserList {}

impl CMsgDOTAChatGetUserList {
    pub fn new() -> CMsgDOTAChatGetUserList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatGetUserList {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatGetUserList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatGetUserList,
        };
        unsafe {
            instance.get(CMsgDOTAChatGetUserList::new)
        }
    }

    // optional fixed64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }
}

impl ::protobuf::Message for CMsgDOTAChatGetUserList {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel_id {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatGetUserList {
    fn new() -> CMsgDOTAChatGetUserList {
        CMsgDOTAChatGetUserList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatGetUserList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "channel_id",
                    CMsgDOTAChatGetUserList::get_channel_id_for_reflect,
                    CMsgDOTAChatGetUserList::mut_channel_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatGetUserList>(
                    "CMsgDOTAChatGetUserList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatGetUserList {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatGetUserList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatGetUserList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatGetUserListResponse {
    // message fields
    channel_id: ::std::option::Option<u64>,
    members: ::protobuf::RepeatedField<CMsgDOTAChatGetUserListResponse_Member>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatGetUserListResponse {}

impl CMsgDOTAChatGetUserListResponse {
    pub fn new() -> CMsgDOTAChatGetUserListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatGetUserListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatGetUserListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatGetUserListResponse,
        };
        unsafe {
            instance.get(CMsgDOTAChatGetUserListResponse::new)
        }
    }

    // optional fixed64 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u64) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u64 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.channel_id
    }

    // repeated .dota.CMsgDOTAChatGetUserListResponse.Member members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAChatGetUserListResponse_Member>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatGetUserListResponse_Member> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAChatGetUserListResponse_Member> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CMsgDOTAChatGetUserListResponse_Member] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAChatGetUserListResponse_Member> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatGetUserListResponse_Member> {
        &mut self.members
    }
}

impl ::protobuf::Message for CMsgDOTAChatGetUserListResponse {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.channel_id {
            my_size += 9;
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_fixed64(1, v)?;
        };
        for v in &self.members {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatGetUserListResponse {
    fn new() -> CMsgDOTAChatGetUserListResponse {
        CMsgDOTAChatGetUserListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatGetUserListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "channel_id",
                    CMsgDOTAChatGetUserListResponse::get_channel_id_for_reflect,
                    CMsgDOTAChatGetUserListResponse::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAChatGetUserListResponse_Member>>(
                    "members",
                    CMsgDOTAChatGetUserListResponse::get_members_for_reflect,
                    CMsgDOTAChatGetUserListResponse::mut_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatGetUserListResponse>(
                    "CMsgDOTAChatGetUserListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatGetUserListResponse {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatGetUserListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatGetUserListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatGetUserListResponse_Member {
    // message fields
    steam_id: ::std::option::Option<u64>,
    persona_name: ::protobuf::SingularField<::std::string::String>,
    channel_user_id: ::std::option::Option<u32>,
    status: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatGetUserListResponse_Member {}

impl CMsgDOTAChatGetUserListResponse_Member {
    pub fn new() -> CMsgDOTAChatGetUserListResponse_Member {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatGetUserListResponse_Member {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatGetUserListResponse_Member> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatGetUserListResponse_Member,
        };
        unsafe {
            instance.get(CMsgDOTAChatGetUserListResponse_Member::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional string persona_name = 2;

    pub fn clear_persona_name(&mut self) {
        self.persona_name.clear();
    }

    pub fn has_persona_name(&self) -> bool {
        self.persona_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persona_name(&mut self, v: ::std::string::String) {
        self.persona_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_persona_name(&mut self) -> &mut ::std::string::String {
        if self.persona_name.is_none() {
            self.persona_name.set_default();
        };
        self.persona_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_persona_name(&mut self) -> ::std::string::String {
        self.persona_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_persona_name(&self) -> &str {
        match self.persona_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_persona_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.persona_name
    }

    fn mut_persona_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.persona_name
    }

    // optional uint32 channel_user_id = 3;

    pub fn clear_channel_user_id(&mut self) {
        self.channel_user_id = ::std::option::Option::None;
    }

    pub fn has_channel_user_id(&self) -> bool {
        self.channel_user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_user_id(&mut self, v: u32) {
        self.channel_user_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_user_id(&self) -> u32 {
        self.channel_user_id.unwrap_or(0)
    }

    fn get_channel_user_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_user_id
    }

    fn mut_channel_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_user_id
    }

    // optional uint32 status = 4;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> u32 {
        self.status.unwrap_or(0)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status
    }
}

impl ::protobuf::Message for CMsgDOTAChatGetUserListResponse_Member {
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
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.persona_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel_user_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        };
        if let Some(v) = self.persona_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.channel_user_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.persona_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.channel_user_id {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.status {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatGetUserListResponse_Member {
    fn new() -> CMsgDOTAChatGetUserListResponse_Member {
        CMsgDOTAChatGetUserListResponse_Member::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatGetUserListResponse_Member>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgDOTAChatGetUserListResponse_Member::get_steam_id_for_reflect,
                    CMsgDOTAChatGetUserListResponse_Member::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "persona_name",
                    CMsgDOTAChatGetUserListResponse_Member::get_persona_name_for_reflect,
                    CMsgDOTAChatGetUserListResponse_Member::mut_persona_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_user_id",
                    CMsgDOTAChatGetUserListResponse_Member::get_channel_user_id_for_reflect,
                    CMsgDOTAChatGetUserListResponse_Member::mut_channel_user_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    CMsgDOTAChatGetUserListResponse_Member::get_status_for_reflect,
                    CMsgDOTAChatGetUserListResponse_Member::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatGetUserListResponse_Member>(
                    "CMsgDOTAChatGetUserListResponse_Member",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatGetUserListResponse_Member {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_persona_name();
        self.clear_channel_user_id();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatGetUserListResponse_Member {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatGetUserListResponse_Member {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatGetMemberCount {
    // message fields
    channel_name: ::protobuf::SingularField<::std::string::String>,
    channel_type: ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatGetMemberCount {}

impl CMsgDOTAChatGetMemberCount {
    pub fn new() -> CMsgDOTAChatGetMemberCount {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatGetMemberCount {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatGetMemberCount> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatGetMemberCount,
        };
        unsafe {
            instance.get(CMsgDOTAChatGetMemberCount::new)
        }
    }

    // optional string channel_name = 1;

    pub fn clear_channel_name(&mut self) {
        self.channel_name.clear();
    }

    pub fn has_channel_name(&self) -> bool {
        self.channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_name(&mut self, v: ::std::string::String) {
        self.channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_name(&mut self) -> &mut ::std::string::String {
        if self.channel_name.is_none() {
            self.channel_name.set_default();
        };
        self.channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel_name(&mut self) -> ::std::string::String {
        self.channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel_name(&self) -> &str {
        match self.channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel_name
    }

    fn mut_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel_name
    }

    // optional .dota.DOTAChatChannelType_t channel_type = 2;

    pub fn clear_channel_type(&mut self) {
        self.channel_type = ::std::option::Option::None;
    }

    pub fn has_channel_type(&self) -> bool {
        self.channel_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_type(&mut self, v: super::dota_shared_enums::DOTAChatChannelType_t) {
        self.channel_type = ::std::option::Option::Some(v);
    }

    pub fn get_channel_type(&self) -> super::dota_shared_enums::DOTAChatChannelType_t {
        self.channel_type.unwrap_or(super::dota_shared_enums::DOTAChatChannelType_t::DOTAChannelType_Regional)
    }

    fn get_channel_type_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &self.channel_type
    }

    fn mut_channel_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &mut self.channel_type
    }
}

impl ::protobuf::Message for CMsgDOTAChatGetMemberCount {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
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
        if let Some(v) = self.channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.channel_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.channel_type {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatGetMemberCount {
    fn new() -> CMsgDOTAChatGetMemberCount {
        CMsgDOTAChatGetMemberCount::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatGetMemberCount>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel_name",
                    CMsgDOTAChatGetMemberCount::get_channel_name_for_reflect,
                    CMsgDOTAChatGetMemberCount::mut_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAChatChannelType_t>>(
                    "channel_type",
                    CMsgDOTAChatGetMemberCount::get_channel_type_for_reflect,
                    CMsgDOTAChatGetMemberCount::mut_channel_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatGetMemberCount>(
                    "CMsgDOTAChatGetMemberCount",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatGetMemberCount {
    fn clear(&mut self) {
        self.clear_channel_name();
        self.clear_channel_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatGetMemberCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatGetMemberCount {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatGetMemberCountResponse {
    // message fields
    channel_name: ::protobuf::SingularField<::std::string::String>,
    channel_type: ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t>,
    member_count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatGetMemberCountResponse {}

impl CMsgDOTAChatGetMemberCountResponse {
    pub fn new() -> CMsgDOTAChatGetMemberCountResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatGetMemberCountResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatGetMemberCountResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatGetMemberCountResponse,
        };
        unsafe {
            instance.get(CMsgDOTAChatGetMemberCountResponse::new)
        }
    }

    // optional string channel_name = 1;

    pub fn clear_channel_name(&mut self) {
        self.channel_name.clear();
    }

    pub fn has_channel_name(&self) -> bool {
        self.channel_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_name(&mut self, v: ::std::string::String) {
        self.channel_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_channel_name(&mut self) -> &mut ::std::string::String {
        if self.channel_name.is_none() {
            self.channel_name.set_default();
        };
        self.channel_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_channel_name(&mut self) -> ::std::string::String {
        self.channel_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_channel_name(&self) -> &str {
        match self.channel_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_channel_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.channel_name
    }

    fn mut_channel_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.channel_name
    }

    // optional .dota.DOTAChatChannelType_t channel_type = 2;

    pub fn clear_channel_type(&mut self) {
        self.channel_type = ::std::option::Option::None;
    }

    pub fn has_channel_type(&self) -> bool {
        self.channel_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_type(&mut self, v: super::dota_shared_enums::DOTAChatChannelType_t) {
        self.channel_type = ::std::option::Option::Some(v);
    }

    pub fn get_channel_type(&self) -> super::dota_shared_enums::DOTAChatChannelType_t {
        self.channel_type.unwrap_or(super::dota_shared_enums::DOTAChatChannelType_t::DOTAChannelType_Regional)
    }

    fn get_channel_type_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &self.channel_type
    }

    fn mut_channel_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTAChatChannelType_t> {
        &mut self.channel_type
    }

    // optional uint32 member_count = 3;

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

impl ::protobuf::Message for CMsgDOTAChatGetMemberCountResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.channel_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.channel_type = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(v) = self.channel_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.channel_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.member_count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.channel_type {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.member_count {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatGetMemberCountResponse {
    fn new() -> CMsgDOTAChatGetMemberCountResponse {
        CMsgDOTAChatGetMemberCountResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatGetMemberCountResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "channel_name",
                    CMsgDOTAChatGetMemberCountResponse::get_channel_name_for_reflect,
                    CMsgDOTAChatGetMemberCountResponse::mut_channel_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAChatChannelType_t>>(
                    "channel_type",
                    CMsgDOTAChatGetMemberCountResponse::get_channel_type_for_reflect,
                    CMsgDOTAChatGetMemberCountResponse::mut_channel_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "member_count",
                    CMsgDOTAChatGetMemberCountResponse::get_member_count_for_reflect,
                    CMsgDOTAChatGetMemberCountResponse::mut_member_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatGetMemberCountResponse>(
                    "CMsgDOTAChatGetMemberCountResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatGetMemberCountResponse {
    fn clear(&mut self) {
        self.clear_channel_name();
        self.clear_channel_type();
        self.clear_member_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatGetMemberCountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatGetMemberCountResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatRegionsEnabled {
    // message fields
    enable_all_regions: ::std::option::Option<bool>,
    enabled_regions: ::protobuf::RepeatedField<CMsgDOTAChatRegionsEnabled_Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatRegionsEnabled {}

impl CMsgDOTAChatRegionsEnabled {
    pub fn new() -> CMsgDOTAChatRegionsEnabled {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatRegionsEnabled {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatRegionsEnabled> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatRegionsEnabled,
        };
        unsafe {
            instance.get(CMsgDOTAChatRegionsEnabled::new)
        }
    }

    // optional bool enable_all_regions = 1;

    pub fn clear_enable_all_regions(&mut self) {
        self.enable_all_regions = ::std::option::Option::None;
    }

    pub fn has_enable_all_regions(&self) -> bool {
        self.enable_all_regions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_enable_all_regions(&mut self, v: bool) {
        self.enable_all_regions = ::std::option::Option::Some(v);
    }

    pub fn get_enable_all_regions(&self) -> bool {
        self.enable_all_regions.unwrap_or(false)
    }

    fn get_enable_all_regions_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.enable_all_regions
    }

    fn mut_enable_all_regions_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.enable_all_regions
    }

    // repeated .dota.CMsgDOTAChatRegionsEnabled.Region enabled_regions = 2;

    pub fn clear_enabled_regions(&mut self) {
        self.enabled_regions.clear();
    }

    // Param is passed by value, moved
    pub fn set_enabled_regions(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAChatRegionsEnabled_Region>) {
        self.enabled_regions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enabled_regions(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatRegionsEnabled_Region> {
        &mut self.enabled_regions
    }

    // Take field
    pub fn take_enabled_regions(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAChatRegionsEnabled_Region> {
        ::std::mem::replace(&mut self.enabled_regions, ::protobuf::RepeatedField::new())
    }

    pub fn get_enabled_regions(&self) -> &[CMsgDOTAChatRegionsEnabled_Region] {
        &self.enabled_regions
    }

    fn get_enabled_regions_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAChatRegionsEnabled_Region> {
        &self.enabled_regions
    }

    fn mut_enabled_regions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAChatRegionsEnabled_Region> {
        &mut self.enabled_regions
    }
}

impl ::protobuf::Message for CMsgDOTAChatRegionsEnabled {
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
                    self.enable_all_regions = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.enabled_regions)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.enable_all_regions {
            my_size += 2;
        };
        for value in &self.enabled_regions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.enable_all_regions {
            os.write_bool(1, v)?;
        };
        for v in &self.enabled_regions {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatRegionsEnabled {
    fn new() -> CMsgDOTAChatRegionsEnabled {
        CMsgDOTAChatRegionsEnabled::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatRegionsEnabled>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "enable_all_regions",
                    CMsgDOTAChatRegionsEnabled::get_enable_all_regions_for_reflect,
                    CMsgDOTAChatRegionsEnabled::mut_enable_all_regions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAChatRegionsEnabled_Region>>(
                    "enabled_regions",
                    CMsgDOTAChatRegionsEnabled::get_enabled_regions_for_reflect,
                    CMsgDOTAChatRegionsEnabled::mut_enabled_regions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatRegionsEnabled>(
                    "CMsgDOTAChatRegionsEnabled",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatRegionsEnabled {
    fn clear(&mut self) {
        self.clear_enable_all_regions();
        self.clear_enabled_regions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatRegionsEnabled {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatRegionsEnabled {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAChatRegionsEnabled_Region {
    // message fields
    min_latitude: ::std::option::Option<f32>,
    max_latitude: ::std::option::Option<f32>,
    min_longitude: ::std::option::Option<f32>,
    max_longitude: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAChatRegionsEnabled_Region {}

impl CMsgDOTAChatRegionsEnabled_Region {
    pub fn new() -> CMsgDOTAChatRegionsEnabled_Region {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAChatRegionsEnabled_Region {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAChatRegionsEnabled_Region> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAChatRegionsEnabled_Region,
        };
        unsafe {
            instance.get(CMsgDOTAChatRegionsEnabled_Region::new)
        }
    }

    // optional float min_latitude = 1;

    pub fn clear_min_latitude(&mut self) {
        self.min_latitude = ::std::option::Option::None;
    }

    pub fn has_min_latitude(&self) -> bool {
        self.min_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_latitude(&mut self, v: f32) {
        self.min_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_min_latitude(&self) -> f32 {
        self.min_latitude.unwrap_or(0.)
    }

    fn get_min_latitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.min_latitude
    }

    fn mut_min_latitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.min_latitude
    }

    // optional float max_latitude = 2;

    pub fn clear_max_latitude(&mut self) {
        self.max_latitude = ::std::option::Option::None;
    }

    pub fn has_max_latitude(&self) -> bool {
        self.max_latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_latitude(&mut self, v: f32) {
        self.max_latitude = ::std::option::Option::Some(v);
    }

    pub fn get_max_latitude(&self) -> f32 {
        self.max_latitude.unwrap_or(0.)
    }

    fn get_max_latitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.max_latitude
    }

    fn mut_max_latitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.max_latitude
    }

    // optional float min_longitude = 3;

    pub fn clear_min_longitude(&mut self) {
        self.min_longitude = ::std::option::Option::None;
    }

    pub fn has_min_longitude(&self) -> bool {
        self.min_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min_longitude(&mut self, v: f32) {
        self.min_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_min_longitude(&self) -> f32 {
        self.min_longitude.unwrap_or(0.)
    }

    fn get_min_longitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.min_longitude
    }

    fn mut_min_longitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.min_longitude
    }

    // optional float max_longitude = 4;

    pub fn clear_max_longitude(&mut self) {
        self.max_longitude = ::std::option::Option::None;
    }

    pub fn has_max_longitude(&self) -> bool {
        self.max_longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_longitude(&mut self, v: f32) {
        self.max_longitude = ::std::option::Option::Some(v);
    }

    pub fn get_max_longitude(&self) -> f32 {
        self.max_longitude.unwrap_or(0.)
    }

    fn get_max_longitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.max_longitude
    }

    fn mut_max_longitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.max_longitude
    }
}

impl ::protobuf::Message for CMsgDOTAChatRegionsEnabled_Region {
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
                    self.min_latitude = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.max_latitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.min_longitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.max_longitude = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.min_latitude {
            my_size += 5;
        };
        if let Some(v) = self.max_latitude {
            my_size += 5;
        };
        if let Some(v) = self.min_longitude {
            my_size += 5;
        };
        if let Some(v) = self.max_longitude {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.min_latitude {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.max_latitude {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.min_longitude {
            os.write_float(3, v)?;
        };
        if let Some(v) = self.max_longitude {
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

impl ::protobuf::MessageStatic for CMsgDOTAChatRegionsEnabled_Region {
    fn new() -> CMsgDOTAChatRegionsEnabled_Region {
        CMsgDOTAChatRegionsEnabled_Region::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAChatRegionsEnabled_Region>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "min_latitude",
                    CMsgDOTAChatRegionsEnabled_Region::get_min_latitude_for_reflect,
                    CMsgDOTAChatRegionsEnabled_Region::mut_min_latitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "max_latitude",
                    CMsgDOTAChatRegionsEnabled_Region::get_max_latitude_for_reflect,
                    CMsgDOTAChatRegionsEnabled_Region::mut_max_latitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "min_longitude",
                    CMsgDOTAChatRegionsEnabled_Region::get_min_longitude_for_reflect,
                    CMsgDOTAChatRegionsEnabled_Region::mut_min_longitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "max_longitude",
                    CMsgDOTAChatRegionsEnabled_Region::get_max_longitude_for_reflect,
                    CMsgDOTAChatRegionsEnabled_Region::mut_max_longitude_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAChatRegionsEnabled_Region>(
                    "CMsgDOTAChatRegionsEnabled_Region",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAChatRegionsEnabled_Region {
    fn clear(&mut self) {
        self.clear_min_latitude();
        self.clear_max_latitude();
        self.clear_min_longitude();
        self.clear_max_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAChatRegionsEnabled_Region {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAChatRegionsEnabled_Region {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x21, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x17, 0x64, 0x6f, 0x74, 0x61, 0x5f,
    0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x8a, 0x01, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x54, 0x6f, 0x47, 0x43, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74,
    0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x12, 0x39, 0x0a, 0x19, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74,
    0x65, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x16, 0x70, 0x72, 0x69, 0x76, 0x61,
    0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d,
    0x65, 0x12, 0x2c, 0x0a, 0x12, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x69,
    0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x22,
    0x82, 0x01, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f,
    0x47, 0x43, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x4b, 0x69, 0x63,
    0x6b, 0x12, 0x39, 0x0a, 0x19, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x5f, 0x63, 0x68, 0x61,
    0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x16, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61,
    0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x0f,
    0x6b, 0x69, 0x63, 0x6b, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x6b, 0x69, 0x63, 0x6b, 0x41, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x49, 0x64, 0x22, 0x8b, 0x01, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68,
    0x61, 0x74, 0x50, 0x72, 0x6f, 0x6d, 0x6f, 0x74, 0x65, 0x12, 0x39, 0x0a, 0x19, 0x70, 0x72, 0x69,
    0x76, 0x61, 0x74, 0x65, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
    0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x16, 0x70, 0x72,
    0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x4e, 0x61, 0x6d, 0x65, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x72, 0x6f, 0x6d, 0x6f, 0x74, 0x65, 0x5f,
    0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x10, 0x70, 0x72, 0x6f, 0x6d, 0x6f, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x49, 0x64, 0x22, 0x88, 0x01, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x54, 0x6f, 0x47, 0x43, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74,
    0x44, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x12, 0x39, 0x0a, 0x19, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74,
    0x65, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x16, 0x70, 0x72, 0x69, 0x76, 0x61,
    0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d,
    0x65, 0x12, 0x2a, 0x0a, 0x11, 0x64, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x64, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x22, 0xec, 0x04,
    0x0a, 0x21, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x39, 0x0a, 0x19, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x5f, 0x63,
    0x68, 0x61, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x16, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43,
    0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x4f,
    0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2e,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07,
    0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12,
    0x1a, 0x0a, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x9e, 0x03, 0x0a, 0x06,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53,
    0x53, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x15, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x43,
    0x52, 0x45, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4c, 0x4f, 0x43, 0x4b, 0x10, 0x01, 0x12, 0x1b,
    0x0a, 0x17, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x53, 0x51, 0x4c, 0x5f, 0x54, 0x52,
    0x41, 0x4e, 0x53, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x46,
    0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x53, 0x44, 0x4f, 0x5f, 0x4c, 0x4f, 0x41, 0x44, 0x10,
    0x03, 0x12, 0x19, 0x0a, 0x15, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f, 0x5f,
    0x50, 0x45, 0x52, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x04, 0x12, 0x1a, 0x0a, 0x16,
    0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x41, 0x4c, 0x52, 0x45, 0x41, 0x44, 0x59, 0x5f,
    0x4d, 0x45, 0x4d, 0x42, 0x45, 0x52, 0x10, 0x05, 0x12, 0x18, 0x0a, 0x14, 0x46, 0x41, 0x49, 0x4c,
    0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x5f, 0x4d, 0x45, 0x4d, 0x42, 0x45, 0x52,
    0x10, 0x07, 0x12, 0x1f, 0x0a, 0x1b, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e, 0x4f,
    0x5f, 0x52, 0x45, 0x4d, 0x41, 0x49, 0x4e, 0x49, 0x4e, 0x47, 0x5f, 0x41, 0x44, 0x4d, 0x49, 0x4e,
    0x53, 0x10, 0x08, 0x12, 0x13, 0x0a, 0x0f, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x4e,
    0x4f, 0x5f, 0x52, 0x4f, 0x4f, 0x4d, 0x10, 0x09, 0x12, 0x21, 0x0a, 0x1d, 0x46, 0x41, 0x49, 0x4c,
    0x55, 0x52, 0x45, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x52, 0x41, 0x54,
    0x45, 0x5f, 0x4c, 0x49, 0x4d, 0x49, 0x54, 0x45, 0x44, 0x10, 0x0a, 0x12, 0x20, 0x0a, 0x1c, 0x46,
    0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x43,
    0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x4e, 0x41, 0x4d, 0x45, 0x10, 0x0b, 0x12, 0x18, 0x0a,
    0x14, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e,
    0x5f, 0x55, 0x53, 0x45, 0x52, 0x10, 0x0c, 0x12, 0x19, 0x0a, 0x15, 0x46, 0x41, 0x49, 0x4c, 0x55,
    0x52, 0x45, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52,
    0x10, 0x0d, 0x12, 0x1d, 0x0a, 0x19, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x43, 0x41,
    0x4e, 0x4e, 0x4f, 0x54, 0x5f, 0x4b, 0x49, 0x43, 0x4b, 0x5f, 0x41, 0x44, 0x4d, 0x49, 0x4e, 0x10,
    0x0e, 0x12, 0x19, 0x0a, 0x15, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x5f, 0x41, 0x4c, 0x52,
    0x45, 0x41, 0x44, 0x59, 0x5f, 0x41, 0x44, 0x4d, 0x49, 0x4e, 0x10, 0x0f, 0x22, 0x61, 0x0a, 0x24,
    0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x50, 0x72,
    0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x39, 0x0a, 0x19, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x5f,
    0x63, 0x68, 0x61, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x16, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65,
    0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x22,
    0xc4, 0x02, 0x0a, 0x25, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x49, 0x6e, 0x66,
    0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x39, 0x0a, 0x19, 0x70, 0x72, 0x69,
    0x76, 0x61, 0x74, 0x65, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
    0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x16, 0x70, 0x72,
    0x69, 0x76, 0x61, 0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x4e, 0x61, 0x6d, 0x65, 0x12, 0x4c, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x69, 0x76, 0x61,
    0x74, 0x65, 0x43, 0x68, 0x61, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x72, 0x65, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x72, 0x65, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x23, 0x0a, 0x0d,
    0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0c, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x61, 0x74,
    0x65, 0x1a, 0x53, 0x0a, 0x06, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16,
    0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x96, 0x01, 0x0a, 0x17, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x4a, 0x6f, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
    0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x58, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x5f, 0x74, 0x3a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x5f, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x52, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x22,
    0x39, 0x0a, 0x18, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x65, 0x61, 0x76, 0x65,
    0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x1d, 0x0a, 0x0a, 0x63,
    0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x22, 0x63, 0x0a, 0x1a, 0x43, 0x4d,
    0x73, 0x67, 0x47, 0x43, 0x43, 0x68, 0x61, 0x74, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x50, 0x75,
    0x62, 0x6c, 0x69, 0x63, 0x53, 0x70, 0x61, 0x6d, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x63, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0d, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x22,
    0x49, 0x0a, 0x19, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x49, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x55, 0x73, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x12,
    0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65,
    0x64, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x22, 0xd7, 0x0a, 0x0a, 0x13, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49,
    0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64,
    0x12, 0x21, 0x0a, 0x0c, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x61, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x61, 0x4e,
    0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x39, 0x0a, 0x19, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73, 0x74,
    0x5f, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x16, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73,
    0x74, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64,
    0x12, 0x2e, 0x0a, 0x13, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x6e, 0x76, 0x69,
    0x74, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11, 0x73,
    0x75, 0x67, 0x67, 0x65, 0x73, 0x74, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x4e, 0x61, 0x6d, 0x65,
    0x12, 0x42, 0x0a, 0x1e, 0x66, 0x61, 0x6e, 0x74, 0x61, 0x73, 0x79, 0x5f, 0x64, 0x72, 0x61, 0x66,
    0x74, 0x5f, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1a, 0x66, 0x61, 0x6e, 0x74, 0x61, 0x73,
    0x79, 0x44, 0x72, 0x61, 0x66, 0x74, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x44, 0x0a, 0x1f, 0x66, 0x61, 0x6e, 0x74, 0x61, 0x73, 0x79, 0x5f,
    0x64, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1b, 0x66,
    0x61, 0x6e, 0x74, 0x61, 0x73, 0x79, 0x44, 0x72, 0x61, 0x66, 0x74, 0x50, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x35, 0x0a, 0x17, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73, 0x74,
    0x5f, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x74, 0x6f, 0x5f, 0x6c, 0x6f, 0x62, 0x62, 0x79,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x52, 0x14, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73, 0x74, 0x49,
    0x6e, 0x76, 0x69, 0x74, 0x65, 0x54, 0x6f, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x12, 0x21, 0x0a, 0x0c,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x18, 0x0c, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0b, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x73, 0x12,
    0x1b, 0x0a, 0x09, 0x63, 0x6f, 0x69, 0x6e, 0x5f, 0x66, 0x6c, 0x69, 0x70, 0x18, 0x0d, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x08, 0x63, 0x6f, 0x69, 0x6e, 0x46, 0x6c, 0x69, 0x70, 0x12, 0x1f, 0x0a, 0x09,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x05, 0x3a,
    0x02, 0x2d, 0x31, 0x52, 0x08, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x64, 0x12, 0x37, 0x0a,
    0x18, 0x73, 0x68, 0x61, 0x72, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x15, 0x73, 0x68, 0x61, 0x72, 0x65, 0x50, 0x72, 0x6f, 0x66, 0x69, 0x6c, 0x65, 0x41, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
    0x6c, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0d, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x3f,
    0x0a, 0x09, 0x64, 0x69, 0x63, 0x65, 0x5f, 0x72, 0x6f, 0x6c, 0x6c, 0x18, 0x11, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x22, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54,
    0x41, 0x43, 0x68, 0x61, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x44, 0x69, 0x63,
    0x65, 0x52, 0x6f, 0x6c, 0x6c, 0x52, 0x08, 0x64, 0x69, 0x63, 0x65, 0x52, 0x6f, 0x6c, 0x6c, 0x12,
    0x24, 0x0a, 0x0e, 0x73, 0x68, 0x61, 0x72, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x69,
    0x64, 0x18, 0x12, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x68, 0x61, 0x72, 0x65, 0x50, 0x61,
    0x72, 0x74, 0x79, 0x49, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x68, 0x61, 0x72, 0x65, 0x5f, 0x6c,
    0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73,
    0x68, 0x61, 0x72, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x12, 0x3a, 0x0a, 0x1a, 0x73,
    0x68, 0x61, 0x72, 0x65, 0x5f, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x14, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x16, 0x73, 0x68, 0x61, 0x72, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x43, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x2e, 0x0a, 0x13, 0x73, 0x68, 0x61, 0x72, 0x65,
    0x5f, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x70, 0x61, 0x73, 0x73, 0x6b, 0x65, 0x79, 0x18, 0x15,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x11, 0x73, 0x68, 0x61, 0x72, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79,
    0x50, 0x61, 0x73, 0x73, 0x6b, 0x65, 0x79, 0x12, 0x35, 0x0a, 0x17, 0x70, 0x72, 0x69, 0x76, 0x61,
    0x74, 0x65, 0x5f, 0x63, 0x68, 0x61, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f,
    0x69, 0x64, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74,
    0x65, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x16,
    0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x17, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x39, 0x0a, 0x19, 0x6c, 0x65, 0x67, 0x61, 0x63, 0x79,
    0x5f, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f, 0x63, 0x75, 0x70, 0x5f, 0x76, 0x69, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x18, 0x18, 0x20, 0x01, 0x28, 0x08, 0x52, 0x16, 0x6c, 0x65, 0x67, 0x61, 0x63,
    0x79, 0x42, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x43, 0x75, 0x70, 0x56, 0x69, 0x63, 0x74, 0x6f, 0x72,
    0x79, 0x12, 0x2a, 0x0a, 0x11, 0x62, 0x61, 0x74, 0x74, 0x6c, 0x65, 0x5f, 0x63, 0x75, 0x70, 0x5f,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6b, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x62, 0x61,
    0x74, 0x74, 0x6c, 0x65, 0x43, 0x75, 0x70, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6b, 0x12, 0x1f, 0x0a,
    0x0b, 0x62, 0x61, 0x64, 0x67, 0x65, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x19, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0a, 0x62, 0x61, 0x64, 0x67, 0x65, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x2f,
    0x0a, 0x14, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73, 0x74, 0x5f, 0x70, 0x69, 0x63, 0x6b, 0x5f, 0x68,
    0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64, 0x18, 0x1a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x73, 0x75,
    0x67, 0x67, 0x65, 0x73, 0x74, 0x50, 0x69, 0x63, 0x6b, 0x48, 0x65, 0x72, 0x6f, 0x49, 0x64, 0x12,
    0x33, 0x0a, 0x16, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73, 0x74, 0x5f, 0x70, 0x69, 0x63, 0x6b, 0x5f,
    0x68, 0x65, 0x72, 0x6f, 0x5f, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x1b, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x13, 0x73, 0x75, 0x67, 0x67, 0x65, 0x73, 0x74, 0x50, 0x69, 0x63, 0x6b, 0x48, 0x65, 0x72, 0x6f,
    0x52, 0x6f, 0x6c, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x65, 0x72, 0x73, 0x65, 0x18, 0x1c, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x05, 0x74, 0x65, 0x72, 0x73, 0x65, 0x1a, 0x58, 0x0a, 0x08, 0x44, 0x69,
    0x63, 0x65, 0x52, 0x6f, 0x6c, 0x6c, 0x12, 0x19, 0x0a, 0x08, 0x72, 0x6f, 0x6c, 0x6c, 0x5f, 0x6d,
    0x69, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x72, 0x6f, 0x6c, 0x6c, 0x4d, 0x69,
    0x6e, 0x12, 0x19, 0x0a, 0x08, 0x72, 0x6f, 0x6c, 0x6c, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x07, 0x72, 0x6f, 0x6c, 0x6c, 0x4d, 0x61, 0x78, 0x12, 0x16, 0x0a, 0x06,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x22, 0x92, 0x01, 0x0a, 0x12, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54,
    0x41, 0x43, 0x68, 0x61, 0x74, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x19, 0x0a, 0x08, 0x73,
    0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73,
    0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e,
    0x61, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x70, 0x65,
    0x72, 0x73, 0x6f, 0x6e, 0x61, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0d, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x55, 0x73, 0x65, 0x72, 0x49,
    0x64, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0xf5, 0x06, 0x0a, 0x1f, 0x43, 0x4d,
    0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4a, 0x6f, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a, 0x0a,
    0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x08, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a,
    0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06,
    0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6d,
    0x61, 0x78, 0x5f, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0a, 0x6d, 0x61, 0x78, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x12, 0x32, 0x0a, 0x07,
    0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61,
    0x74, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x12, 0x58, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70,
    0x65, 0x5f, 0x74, 0x3a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x54, 0x79, 0x70, 0x65, 0x5f, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x52, 0x0b, 0x63,
    0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x52, 0x0a, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2c, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4a, 0x6f, 0x69, 0x6e, 0x43, 0x68,
    0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x0c, 0x4a, 0x4f, 0x49, 0x4e, 0x5f, 0x53,
    0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x2a,
    0x0a, 0x11, 0x67, 0x63, 0x5f, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x6a,
    0x6f, 0x69, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x67, 0x63, 0x49, 0x6e, 0x69,
    0x74, 0x69, 0x61, 0x74, 0x65, 0x64, 0x4a, 0x6f, 0x69, 0x6e, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x55, 0x73, 0x65, 0x72,
    0x49, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x77, 0x65, 0x6c, 0x63, 0x6f, 0x6d, 0x65, 0x5f, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x77, 0x65, 0x6c,
    0x63, 0x6f, 0x6d, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0xf3, 0x02, 0x0a, 0x06,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x10, 0x0a, 0x0c, 0x4a, 0x4f, 0x49, 0x4e, 0x5f, 0x53,
    0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12, 0x18, 0x0a, 0x14, 0x49, 0x4e, 0x56, 0x41,
    0x4c, 0x49, 0x44, 0x5f, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x54, 0x59, 0x50, 0x45,
    0x10, 0x01, 0x12, 0x15, 0x0a, 0x11, 0x41, 0x43, 0x43, 0x4f, 0x55, 0x4e, 0x54, 0x5f, 0x4e, 0x4f,
    0x54, 0x5f, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x02, 0x12, 0x0e, 0x0a, 0x0a, 0x41, 0x43, 0x48,
    0x5f, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x03, 0x12, 0x1d, 0x0a, 0x19, 0x55, 0x53, 0x45,
    0x52, 0x5f, 0x49, 0x4e, 0x5f, 0x54, 0x4f, 0x4f, 0x5f, 0x4d, 0x41, 0x4e, 0x59, 0x5f, 0x43, 0x48,
    0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x53, 0x10, 0x04, 0x12, 0x17, 0x0a, 0x13, 0x52, 0x41, 0x54, 0x45,
    0x5f, 0x4c, 0x49, 0x4d, 0x49, 0x54, 0x5f, 0x45, 0x58, 0x43, 0x45, 0x45, 0x44, 0x45, 0x44, 0x10,
    0x05, 0x12, 0x10, 0x0a, 0x0c, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x46, 0x55, 0x4c,
    0x4c, 0x10, 0x06, 0x12, 0x1b, 0x0a, 0x17, 0x43, 0x48, 0x41, 0x4e, 0x4e, 0x45, 0x4c, 0x5f, 0x46,
    0x55, 0x4c, 0x4c, 0x5f, 0x4f, 0x56, 0x45, 0x52, 0x46, 0x4c, 0x4f, 0x57, 0x45, 0x44, 0x10, 0x07,
    0x12, 0x16, 0x0a, 0x12, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x5f, 0x54, 0x4f, 0x5f, 0x41, 0x44,
    0x44, 0x5f, 0x55, 0x53, 0x45, 0x52, 0x10, 0x08, 0x12, 0x19, 0x0a, 0x15, 0x43, 0x48, 0x41, 0x4e,
    0x4e, 0x45, 0x4c, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x44, 0x49, 0x53, 0x41, 0x42, 0x4c, 0x45,
    0x44, 0x10, 0x09, 0x12, 0x1e, 0x0a, 0x1a, 0x50, 0x52, 0x49, 0x56, 0x41, 0x54, 0x45, 0x5f, 0x43,
    0x48, 0x41, 0x54, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x45,
    0x44, 0x10, 0x0a, 0x12, 0x1e, 0x0a, 0x1a, 0x50, 0x52, 0x49, 0x56, 0x41, 0x54, 0x45, 0x5f, 0x43,
    0x48, 0x41, 0x54, 0x5f, 0x4e, 0x4f, 0x5f, 0x50, 0x45, 0x52, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f,
    0x4e, 0x10, 0x0b, 0x12, 0x23, 0x0a, 0x1f, 0x50, 0x52, 0x49, 0x56, 0x41, 0x54, 0x45, 0x5f, 0x43,
    0x48, 0x41, 0x54, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x5f, 0x4c, 0x4f, 0x43, 0x4b, 0x5f,
    0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x0c, 0x12, 0x17, 0x0a, 0x13, 0x50, 0x52, 0x49, 0x56,
    0x41, 0x54, 0x45, 0x5f, 0x43, 0x48, 0x41, 0x54, 0x5f, 0x4b, 0x49, 0x43, 0x4b, 0x45, 0x44, 0x10,
    0x0d, 0x22, 0x72, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61,
    0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x46, 0x75, 0x6c, 0x6c, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49,
    0x64, 0x12, 0x32, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x73, 0x22, 0xbd, 0x01, 0x0a, 0x1e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f,
    0x54, 0x41, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x4a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x43, 0x68, 0x61,
    0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x63, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x65, 0x72, 0x73, 0x6f,
    0x6e, 0x61, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x70,
    0x65, 0x72, 0x73, 0x6f, 0x6e, 0x61, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x74,
    0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74,
    0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d,
    0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x16, 0x0a,
    0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x80, 0x01, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f,
    0x54, 0x41, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x4c, 0x65, 0x66, 0x74, 0x43, 0x68, 0x61, 0x74, 0x43,
    0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65,
    0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64,
    0x12, 0x26, 0x0a, 0x0f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x75, 0x73, 0x65, 0x72,
    0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x22, 0xd0, 0x02, 0x0a, 0x1f, 0x43, 0x4d, 0x73,
    0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a,
    0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06,
    0x52, 0x09, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x6c,
    0x65, 0x66, 0x74, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x06, 0x52, 0x0c, 0x6c, 0x65, 0x66, 0x74, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64,
    0x73, 0x12, 0x59, 0x0a, 0x0e, 0x6a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x5f, 0x6d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x2e, 0x4a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x0d, 0x6a,
    0x6f, 0x69, 0x6e, 0x65, 0x64, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x1a, 0x8c, 0x01, 0x0a,
    0x0c, 0x4a, 0x6f, 0x69, 0x6e, 0x65, 0x64, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x19, 0x0a,
    0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x65, 0x72, 0x73,
    0x6f, 0x6e, 0x61, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x61, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x63,
    0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x55, 0x73, 0x65,
    0x72, 0x49, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x20, 0x0a, 0x1e, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43, 0x68,
    0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4c, 0x69, 0x73, 0x74, 0x22, 0xac, 0x02,
    0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4c, 0x69, 0x73, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x54, 0x0a, 0x08, 0x63, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x38, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4c, 0x69, 0x73, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x52, 0x08, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73, 0x1a, 0xab,
    0x01, 0x0a, 0x0b, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x21,
    0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x6e, 0x75, 0x6d, 0x5f, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x6e, 0x75, 0x6d, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x73, 0x12, 0x58, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54,
    0x79, 0x70, 0x65, 0x5f, 0x74, 0x3a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x5f, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x52,
    0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x22, 0x38, 0x0a, 0x17,
    0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x47, 0x65, 0x74, 0x55,
    0x73, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x63, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x22, 0x91, 0x02, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x47, 0x65, 0x74, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x69,
    0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09,
    0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x46, 0x0a, 0x07, 0x6d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x47, 0x65,
    0x74, 0x55, 0x73, 0x65, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x1a, 0x86, 0x01, 0x0a, 0x06, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x19, 0x0a, 0x08,
    0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07,
    0x73, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x65, 0x72, 0x73, 0x6f,
    0x6e, 0x61, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x70,
    0x65, 0x72, 0x73, 0x6f, 0x6e, 0x61, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x68,
    0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x55, 0x73, 0x65, 0x72,
    0x49, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x99, 0x01, 0x0a, 0x1a, 0x43,
    0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x47, 0x65, 0x74, 0x4d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x58, 0x0a, 0x0c,
    0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68,
    0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x5f, 0x74, 0x3a,
    0x18, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65,
    0x5f, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x52, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x22, 0xc4, 0x01, 0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x47, 0x65, 0x74, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x21, 0x0a,
    0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65,
    0x12, 0x58, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70,
    0x65, 0x5f, 0x74, 0x3a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c,
    0x54, 0x79, 0x70, 0x65, 0x5f, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x52, 0x0b, 0x63,
    0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0xb7, 0x02,
    0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x52, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x73, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x2c, 0x0a, 0x12,
    0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x5f, 0x61, 0x6c, 0x6c, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x10, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65,
    0x41, 0x6c, 0x6c, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x50, 0x0a, 0x0f, 0x65, 0x6e,
    0x61, 0x62, 0x6c, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x43, 0x68, 0x61, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x73, 0x45, 0x6e,
    0x61, 0x62, 0x6c, 0x65, 0x64, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x0e, 0x65, 0x6e,
    0x61, 0x62, 0x6c, 0x65, 0x64, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x73, 0x1a, 0x98, 0x01, 0x0a,
    0x06, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x69, 0x6e, 0x5f, 0x6c,
    0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0b, 0x6d,
    0x69, 0x6e, 0x4c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x6d, 0x61,
    0x78, 0x5f, 0x6c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x0b, 0x6d, 0x61, 0x78, 0x4c, 0x61, 0x74, 0x69, 0x74, 0x75, 0x64, 0x65, 0x12, 0x23, 0x0a,
    0x0d, 0x6d, 0x69, 0x6e, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x6d, 0x69, 0x6e, 0x4c, 0x6f, 0x6e, 0x67, 0x69, 0x74, 0x75,
    0x64, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x6d, 0x61, 0x78, 0x5f, 0x6c, 0x6f, 0x6e, 0x67, 0x69, 0x74,
    0x75, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x6d, 0x61, 0x78, 0x4c, 0x6f,
    0x6e, 0x67, 0x69, 0x74, 0x75, 0x64, 0x65, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0x9f,
    0x54, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xf5, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x20, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x06, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x06, 0x00, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x07,
    0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x07, 0x07, 0x1a,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1d, 0x22, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x09, 0x08, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a,
    0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x18, 0x31, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x0b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x0b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x18,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x2d, 0x2e, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0e, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x0f, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0f, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x18, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x10, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x10, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x10, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x2a,
    0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x13, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x14, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x14, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x18, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x34, 0x35, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x15, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x15, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x15, 0x2d, 0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x18, 0x00, 0x1b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x18, 0x08, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x19, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x19, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19,
    0x18, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x34, 0x35,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1a, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1a, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1d, 0x00, 0x33,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x04, 0x04, 0x00, 0x12, 0x04, 0x1e, 0x08, 0x2e, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x10, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x1f, 0x1a, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x20, 0x10, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x20, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x20, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x21, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x21, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x21, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x22, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x22, 0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x22, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x23, 0x10, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x23, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x23, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x24, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x24, 0x10, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03,
    0x24, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x25,
    0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x25,
    0x10, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x25,
    0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x26, 0x10,
    0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x26, 0x10,
    0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x26, 0x2e,
    0x2f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x27, 0x10, 0x24,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x27, 0x10, 0x1f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x27, 0x22, 0x23,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x28, 0x10, 0x33, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x28, 0x10, 0x2d, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x28, 0x30, 0x32, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x29, 0x10, 0x32, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x29, 0x10, 0x2c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x29, 0x2f, 0x31, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x2a, 0x10, 0x2a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x2a, 0x10, 0x24, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x2a, 0x27, 0x29, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x2b, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x2b, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x2b, 0x28, 0x2a, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x2c, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x2c, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x2c, 0x2c, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x04, 0x04, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x2d, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x2d, 0x10, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x04, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03, 0x2d, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x30, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x30, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x18,
    0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x30, 0x34, 0x35, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x31, 0x08, 0x59, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x31, 0x11, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x31, 0x3a, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x31, 0x43, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x08, 0x12, 0x03, 0x31,
    0x45, 0x58, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x07, 0x12, 0x03, 0x31, 0x50, 0x57,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x32, 0x08, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x32, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x32, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x32, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x35, 0x00, 0x37,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x35, 0x08, 0x2c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x36, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x36, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x36, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x36, 0x18, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36,
    0x34, 0x35, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x39, 0x00, 0x44, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x39, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06,
    0x03, 0x00, 0x12, 0x04, 0x3a, 0x08, 0x3e, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x3a, 0x10, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x3b, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x3b, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x3b, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x3b, 0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x3b, 0x2d, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x3c, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x3c, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x3c, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x3c, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x3c, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x3d, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x3d, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x3d, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x3d, 0x20, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x3d, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x40, 0x08, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x40, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x40, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x18, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x40, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01,
    0x12, 0x03, 0x41, 0x08, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x41, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x41, 0x11,
    0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x41, 0x3e, 0x45, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x41, 0x48, 0x49, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x42, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x42, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x42, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x42, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x42,
    0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x43, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x43, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x43, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x43, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x43, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x46,
    0x00, 0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x46, 0x08, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x47, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x47, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x47, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x48, 0x08,
    0x5d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x48, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x48, 0x11, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x27, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x08, 0x12, 0x03, 0x48, 0x38, 0x5c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x07,
    0x12, 0x03, 0x48, 0x43, 0x5b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x4b, 0x00, 0x4d,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x4c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x4c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x4c, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4c,
    0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x4f, 0x00, 0x52, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x00, 0x12, 0x03, 0x50, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x50, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x50, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x18,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x50, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x51, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x51, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x51, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x51, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x51, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x54, 0x00, 0x56, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x54, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x55, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x55, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x55, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x55, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x55, 0x2d,
    0x2e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x58, 0x00, 0x7c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x58, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x03,
    0x00, 0x12, 0x04, 0x59, 0x08, 0x5d, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x03, 0x00, 0x01,
    0x12, 0x03, 0x59, 0x10, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x5a, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x5a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x5a, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x5a, 0x1f, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x5a, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x5b, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x5b, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x5b, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x5b, 0x1f, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x5b, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5c,
    0x10, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5c,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5c,
    0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5c,
    0x1f, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0b, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5c,
    0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x08, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x5f, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12,
    0x03, 0x60, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x60,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x60, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x60, 0x18, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x60, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x61, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x61, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x61, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x61, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x61, 0x27,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x03, 0x62, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x03, 0x62, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x03, 0x62, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x62, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x62, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x03,
    0x63, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x03, 0x63, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x03, 0x63, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x03, 0x63, 0x18, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x03, 0x63, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x05, 0x12, 0x03, 0x64, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x64, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x64, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01, 0x12, 0x03, 0x64,
    0x18, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12, 0x03, 0x64, 0x34, 0x35,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x06, 0x12, 0x03, 0x65, 0x08, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x06, 0x04, 0x12, 0x03, 0x65, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x06, 0x05, 0x12, 0x03, 0x65, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x65, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x65, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x07, 0x12, 0x03, 0x66,
    0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x04, 0x12, 0x03, 0x66, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x05, 0x12, 0x03, 0x66, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x07, 0x01, 0x12, 0x03, 0x66, 0x18, 0x36, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x07, 0x03, 0x12, 0x03, 0x66, 0x39, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x08, 0x12, 0x03, 0x67, 0x08, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x04,
    0x12, 0x03, 0x67, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x67, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x01, 0x12, 0x03, 0x67, 0x18,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x08, 0x03, 0x12, 0x03, 0x67, 0x3a, 0x3b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x09, 0x12, 0x03, 0x68, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x09, 0x04, 0x12, 0x03, 0x68, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x09, 0x05, 0x12, 0x03, 0x68, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09,
    0x01, 0x12, 0x03, 0x68, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x09, 0x03, 0x12,
    0x03, 0x68, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0a, 0x12, 0x03, 0x69, 0x08,
    0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x69, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x69, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x69, 0x16, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x69, 0x30, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x0b, 0x12, 0x03, 0x6a, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x04, 0x12,
    0x03, 0x6a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x6a,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x6a, 0x18, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x6a, 0x27, 0x29, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0c, 0x12, 0x03, 0x6b, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x0c, 0x05, 0x12, 0x03, 0x6b, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x01,
    0x12, 0x03, 0x6b, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0c, 0x03, 0x12, 0x03,
    0x6b, 0x22, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0d, 0x12, 0x03, 0x6c, 0x08, 0x35,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x6c, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x6c, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x6c, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x0d, 0x03, 0x12, 0x03, 0x6c, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d,
    0x08, 0x12, 0x03, 0x6c, 0x26, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0d, 0x07, 0x12,
    0x03, 0x6c, 0x31, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x0e, 0x12, 0x03, 0x6d, 0x08,
    0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x6d, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x6d, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x6d, 0x18, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x6d, 0x33, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x0f, 0x12, 0x03, 0x6e, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x04, 0x12,
    0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x6e,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x6e, 0x18, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x6e, 0x2a, 0x2c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x10, 0x12, 0x03, 0x6f, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x10, 0x04, 0x12, 0x03, 0x6f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x10, 0x06, 0x12, 0x03, 0x6f, 0x11, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x01,
    0x12, 0x03, 0x6f, 0x2e, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x10, 0x03, 0x12, 0x03,
    0x6f, 0x3a, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x11, 0x12, 0x03, 0x70, 0x08, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x11, 0x04, 0x12, 0x03, 0x70, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x11, 0x05, 0x12, 0x03, 0x70, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x11, 0x01, 0x12, 0x03, 0x70, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x11, 0x03, 0x12, 0x03, 0x70, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x12,
    0x12, 0x03, 0x71, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12, 0x04, 0x12, 0x03,
    0x71, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12, 0x05, 0x12, 0x03, 0x71, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12, 0x01, 0x12, 0x03, 0x71, 0x18, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x12, 0x03, 0x12, 0x03, 0x71, 0x29, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x13, 0x12, 0x03, 0x72, 0x08, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x13, 0x04, 0x12, 0x03, 0x72, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x13,
    0x05, 0x12, 0x03, 0x72, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x13, 0x01, 0x12,
    0x03, 0x72, 0x18, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x13, 0x03, 0x12, 0x03, 0x72,
    0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x14, 0x12, 0x03, 0x73, 0x08, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x14, 0x04, 0x12, 0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x14, 0x05, 0x12, 0x03, 0x73, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x14, 0x01, 0x12, 0x03, 0x73, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x14, 0x03, 0x12, 0x03, 0x73, 0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x15, 0x12,
    0x03, 0x74, 0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x15, 0x04, 0x12, 0x03, 0x74,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x15, 0x05, 0x12, 0x03, 0x74, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x15, 0x01, 0x12, 0x03, 0x74, 0x18, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x15, 0x03, 0x12, 0x03, 0x74, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x16, 0x12, 0x03, 0x75, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x16, 0x04, 0x12, 0x03, 0x75, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x16, 0x05,
    0x12, 0x03, 0x75, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x16, 0x01, 0x12, 0x03,
    0x75, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x16, 0x03, 0x12, 0x03, 0x75, 0x21,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x17, 0x12, 0x03, 0x76, 0x08, 0x35, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x17, 0x04, 0x12, 0x03, 0x76, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x17, 0x05, 0x12, 0x03, 0x76, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x17, 0x01, 0x12, 0x03, 0x76, 0x16, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x17,
    0x03, 0x12, 0x03, 0x76, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x18, 0x12, 0x03,
    0x77, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x18, 0x04, 0x12, 0x03, 0x77, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x18, 0x05, 0x12, 0x03, 0x77, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x18, 0x01, 0x12, 0x03, 0x77, 0x18, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x18, 0x03, 0x12, 0x03, 0x77, 0x2c, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x19, 0x12, 0x03, 0x78, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x19,
    0x04, 0x12, 0x03, 0x78, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x19, 0x05, 0x12,
    0x03, 0x78, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x19, 0x01, 0x12, 0x03, 0x78,
    0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x19, 0x03, 0x12, 0x03, 0x78, 0x26, 0x28,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x1a, 0x12, 0x03, 0x79, 0x08, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x1a, 0x04, 0x12, 0x03, 0x79, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x1a, 0x05, 0x12, 0x03, 0x79, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x1a, 0x01, 0x12, 0x03, 0x79, 0x18, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1a, 0x03,
    0x12, 0x03, 0x79, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x1b, 0x12, 0x03, 0x7a,
    0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1b, 0x04, 0x12, 0x03, 0x7a, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1b, 0x05, 0x12, 0x03, 0x7a, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1b, 0x01, 0x12, 0x03, 0x7a, 0x18, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x1b, 0x03, 0x12, 0x03, 0x7a, 0x31, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x1c, 0x12, 0x03, 0x7b, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1c, 0x04,
    0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1c, 0x05, 0x12, 0x03,
    0x7b, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1c, 0x01, 0x12, 0x03, 0x7b, 0x16,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x1c, 0x03, 0x12, 0x03, 0x7b, 0x1e, 0x20, 0x0a,
    0x0b, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x05, 0x7e, 0x00, 0x83, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0c, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00,
    0x12, 0x03, 0x7f, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x7f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7f, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7f, 0x19, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7f, 0x24, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0x80, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x80, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x80, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x80, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x80, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12,
    0x04, 0x81, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x81, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x04, 0x81,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x81, 0x01,
    0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0x81, 0x01, 0x2a,
    0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x04, 0x82, 0x01, 0x08, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x04, 0x82, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05, 0x12, 0x04, 0x82, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x04, 0x82, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x04, 0x82, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x0d, 0x12, 0x06, 0x85, 0x01, 0x00, 0xa1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01,
    0x12, 0x04, 0x85, 0x01, 0x08, 0x27, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0d, 0x04, 0x00, 0x12, 0x06,
    0x86, 0x01, 0x08, 0x95, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x04, 0x00, 0x01, 0x12,
    0x04, 0x86, 0x01, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x04, 0x87, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x87, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x04, 0x87, 0x01, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x04, 0x88, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x88, 0x01, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x04, 0x88, 0x01, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0x89, 0x01, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x89, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0x89, 0x01, 0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x0d, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x10, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0d, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0d, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0x8a, 0x01, 0x1d, 0x1e, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x10, 0x29, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0x8b, 0x01, 0x2c, 0x2d, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x28, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x23,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x26,
    0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0x8d, 0x01, 0x10,
    0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0x8d, 0x01,
    0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0x8d,
    0x01, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x07, 0x12, 0x04, 0x8e,
    0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04,
    0x8e, 0x01, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12,
    0x04, 0x8e, 0x01, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x08, 0x12,
    0x04, 0x8f, 0x01, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x08, 0x01,
    0x12, 0x04, 0x8f, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x08,
    0x02, 0x12, 0x04, 0x8f, 0x01, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02,
    0x09, 0x12, 0x04, 0x90, 0x01, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02,
    0x09, 0x01, 0x12, 0x04, 0x90, 0x01, 0x10, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00,
    0x02, 0x09, 0x02, 0x12, 0x04, 0x90, 0x01, 0x28, 0x29, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04,
    0x00, 0x02, 0x0a, 0x12, 0x04, 0x91, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04,
    0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x91, 0x01, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d,
    0x04, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x04, 0x91, 0x01, 0x2d, 0x2f, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x0d, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x04, 0x92, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0d, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x92, 0x01, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0d, 0x04, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x04, 0x92, 0x01, 0x2d, 0x2f, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x04, 0x93, 0x01, 0x10, 0x35, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x93, 0x01, 0x10, 0x2f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x04, 0x93, 0x01, 0x32, 0x34, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x04, 0x94, 0x01, 0x10, 0x29, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x94, 0x01, 0x10, 0x23,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0d, 0x04, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x04, 0x94, 0x01, 0x26,
    0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0x97, 0x01, 0x08, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x97, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x04, 0x97, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x97, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x97, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x01, 0x12, 0x04, 0x98, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x98, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x98, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x98, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x98, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x04, 0x99,
    0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x04, 0x12, 0x04, 0x99, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x05, 0x12, 0x04, 0x99, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01, 0x12, 0x04, 0x99, 0x01, 0x19, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x04, 0x99, 0x01, 0x26, 0x27, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x03, 0x05, 0x12, 0x04, 0x9a, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x03, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x04, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x9b, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x06, 0x12,
    0x04, 0x9b, 0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x9b, 0x01, 0x24, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12, 0x04, 0x9b,
    0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x08,
    0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x06, 0x12, 0x04, 0x9c, 0x01, 0x11, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x27, 0x33, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x36, 0x37, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x05, 0x08, 0x12, 0x04, 0x9c, 0x01, 0x38, 0x5c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x05, 0x07, 0x12, 0x04, 0x9c, 0x01, 0x43, 0x5b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x06, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x06, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06,
    0x06, 0x12, 0x04, 0x9d, 0x01, 0x11, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x01,
    0x12, 0x04, 0x9d, 0x01, 0x38, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x03, 0x12,
    0x04, 0x9d, 0x01, 0x41, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x08, 0x12, 0x04,
    0x9d, 0x01, 0x43, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x07, 0x12, 0x04, 0x9d,
    0x01, 0x4e, 0x5a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x07, 0x12, 0x04, 0x9e, 0x01, 0x08,
    0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x05, 0x12, 0x04, 0x9e, 0x01, 0x11, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x16, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x08, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x08, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x08, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x08, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x08,
    0x03, 0x12, 0x04, 0x9f, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x09, 0x12,
    0x04, 0xa0, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x04, 0x12, 0x04,
    0xa0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x05, 0x12, 0x04, 0xa0,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x01, 0x12, 0x04, 0xa0, 0x01,
    0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x09, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x2a,
    0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xa3, 0x01, 0x00, 0xa6, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa4, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa4, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x04,
    0xa5, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa5,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa5, 0x01,
    0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x24,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x2e, 0x2f,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xa8, 0x01, 0x00, 0xae, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x00, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xa9, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xa9, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xa9, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xaa,
    0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xaa, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x18, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x27, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x04, 0xab, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xab, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x02, 0x05, 0x12, 0x04, 0xab, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xab, 0x01, 0x19, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xab, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x03, 0x12, 0x04, 0xac, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x04,
    0x12, 0x04, 0xac, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x05, 0x12,
    0x04, 0xac, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xac, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x04, 0xac,
    0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x04, 0x12, 0x04, 0xad, 0x01, 0x08,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x04, 0x12, 0x04, 0xad, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x05, 0x12, 0x04, 0xad, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x01, 0x12, 0x04, 0xad, 0x01, 0x18, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x03, 0x12, 0x04, 0xad, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x10, 0x12, 0x06, 0xb0, 0x01, 0x00, 0xb4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x10, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00,
    0x12, 0x04, 0xb1, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xb1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xb1, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb1,
    0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb1, 0x01,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb2, 0x01, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x19, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xb3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xb3, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xb3, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xb6, 0x01,
    0x00, 0xc1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08,
    0x27, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x11, 0x03, 0x00, 0x12, 0x06, 0xb7, 0x01, 0x08, 0xbc, 0x01,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x03, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x10, 0x1c,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb8, 0x01, 0x10, 0x2e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb8, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb8, 0x01,
    0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb8,
    0x01, 0x21, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xb8, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04,
    0xb9, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xb9, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xb9, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xb9, 0x01, 0x20, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03, 0x00,
    0x02, 0x02, 0x12, 0x04, 0xba, 0x01, 0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xba, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xba, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11,
    0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xba, 0x01, 0x20, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x11, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xba, 0x01, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x04, 0xbb, 0x01, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x20, 0x26, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xbb, 0x01, 0x29, 0x2a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbe, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xbf, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xbf, 0x01, 0x19, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xbf, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xc0, 0x01,
    0x08, 0x51, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc0, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x06, 0x12, 0x04, 0xc0, 0x01, 0x11, 0x3d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x3e, 0x4c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x4f, 0x50, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xc3, 0x01, 0x00, 0xc4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x12, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12,
    0x06, 0xc6, 0x01, 0x00, 0xce, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04,
    0xc6, 0x01, 0x08, 0x2e, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x13, 0x03, 0x00, 0x12, 0x06, 0xc7, 0x01,
    0x08, 0xcb, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x03, 0x00, 0x01, 0x12, 0x04, 0xc7,
    0x01, 0x10, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc8,
    0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xc8, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xc8, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xc8, 0x01, 0x20, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xc8, 0x01, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13, 0x03, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xc9, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x20, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x13, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xca, 0x01, 0x10, 0x65, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x13, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xca, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x13, 0x03, 0x00, 0x02, 0x02, 0x06, 0x12, 0x04, 0xca, 0x01, 0x19, 0x2e, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x13, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xca, 0x01, 0x2f, 0x3b, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x13, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xca, 0x01, 0x3e, 0x3f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03, 0x00, 0x02, 0x02, 0x08, 0x12, 0x04, 0xca, 0x01, 0x40, 0x64,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x03, 0x00, 0x02, 0x02, 0x07, 0x12, 0x04, 0xca, 0x01, 0x4b,
    0x63, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x51, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xcd, 0x01, 0x11, 0x43, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x44, 0x4c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcd, 0x01, 0x4f, 0x50, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x14, 0x12, 0x06, 0xd0, 0x01, 0x00, 0xd2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01,
    0x12, 0x04, 0xd0, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04,
    0xd1, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd1,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd1, 0x01,
    0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x19,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xd4, 0x01, 0x00, 0xde, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x27, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x15, 0x03, 0x00, 0x12, 0x06, 0xd5, 0x01, 0x08, 0xda, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x03, 0x00, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd6, 0x01, 0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x21, 0x29, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x2c, 0x2d, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x10, 0x31, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x10, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x19, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x20,
    0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd7, 0x01,
    0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xd8, 0x01,
    0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd8,
    0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xd8, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xd8, 0x01, 0x20, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xd8, 0x01, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03,
    0x12, 0x04, 0xd9, 0x01, 0x10, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xd9, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xd9, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x20, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x00, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xdc, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xdc, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xdc, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xdd, 0x01,
    0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xdd, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x06, 0x12, 0x04, 0xdd, 0x01, 0x11, 0x37,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x38, 0x3f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x42, 0x43, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xe0, 0x01, 0x00, 0xe3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x16, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02,
    0x00, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xe1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xe1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xe1, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe1,
    0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x08,
    0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe2, 0x01, 0x11, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x27, 0x33, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x36, 0x37, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x01, 0x08, 0x12, 0x04, 0xe2, 0x01, 0x38, 0x5c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x01, 0x07, 0x12, 0x04, 0xe2, 0x01, 0x43, 0x5b, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x17, 0x12, 0x06, 0xe5, 0x01, 0x00, 0xe9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01,
    0x12, 0x04, 0xe5, 0x01, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04,
    0xe6, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe6,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe6, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x18,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x27, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x5d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe7, 0x01, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x27, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x36, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x01, 0x08, 0x12, 0x04, 0xe7, 0x01, 0x38, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x07, 0x12, 0x04, 0xe7, 0x01, 0x43, 0x5b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02,
    0x12, 0x04, 0xe8, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xe8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xe8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe8,
    0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe8, 0x01,
    0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xeb, 0x01, 0x00, 0xf5, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x22, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x18, 0x03, 0x00, 0x12, 0x06, 0xec, 0x01, 0x08, 0xf1, 0x01, 0x09, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x03, 0x00, 0x01, 0x12, 0x04, 0xec, 0x01, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xed, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xed, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xed, 0x01, 0x19, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xed, 0x01, 0x1f, 0x2b, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xed, 0x01, 0x2e, 0x2f,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xee, 0x01, 0x10, 0x30,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xee, 0x01,
    0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee,
    0x01, 0x1f, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xee, 0x01, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04,
    0xef, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xef, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xef, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xef, 0x01, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xef, 0x01, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x18, 0x03, 0x00,
    0x02, 0x03, 0x12, 0x04, 0xf0, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xf0, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18, 0x03,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf0, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x18,
    0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x1f, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x18, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf0, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xf3, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xf3, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xf3, 0x01, 0x16, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xf3, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04,
    0xf4, 0x01, 0x08, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf4,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf4, 0x01,
    0x11, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf4, 0x01, 0x33,
    0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x45, 0x46,
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
