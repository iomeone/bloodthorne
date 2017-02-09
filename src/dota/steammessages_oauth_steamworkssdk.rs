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
pub struct COAuthToken_ImplicitGrantNoPrompt_Request {
    // message fields
    clientid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for COAuthToken_ImplicitGrantNoPrompt_Request {}

impl COAuthToken_ImplicitGrantNoPrompt_Request {
    pub fn new() -> COAuthToken_ImplicitGrantNoPrompt_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static COAuthToken_ImplicitGrantNoPrompt_Request {
        static mut instance: ::protobuf::lazy::Lazy<COAuthToken_ImplicitGrantNoPrompt_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const COAuthToken_ImplicitGrantNoPrompt_Request,
        };
        unsafe {
            instance.get(COAuthToken_ImplicitGrantNoPrompt_Request::new)
        }
    }

    // optional string clientid = 1;

    pub fn clear_clientid(&mut self) {
        self.clientid.clear();
    }

    pub fn has_clientid(&self) -> bool {
        self.clientid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientid(&mut self, v: ::std::string::String) {
        self.clientid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientid(&mut self) -> &mut ::std::string::String {
        if self.clientid.is_none() {
            self.clientid.set_default();
        };
        self.clientid.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientid(&mut self) -> ::std::string::String {
        self.clientid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientid(&self) -> &str {
        match self.clientid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_clientid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.clientid
    }

    fn mut_clientid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.clientid
    }
}

impl ::protobuf::Message for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clientid)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.clientid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.clientid.as_ref() {
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

impl ::protobuf::MessageStatic for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn new() -> COAuthToken_ImplicitGrantNoPrompt_Request {
        COAuthToken_ImplicitGrantNoPrompt_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<COAuthToken_ImplicitGrantNoPrompt_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientid",
                    COAuthToken_ImplicitGrantNoPrompt_Request::get_clientid_for_reflect,
                    COAuthToken_ImplicitGrantNoPrompt_Request::mut_clientid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<COAuthToken_ImplicitGrantNoPrompt_Request>(
                    "COAuthToken_ImplicitGrantNoPrompt_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn clear(&mut self) {
        self.clear_clientid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for COAuthToken_ImplicitGrantNoPrompt_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct COAuthToken_ImplicitGrantNoPrompt_Response {
    // message fields
    access_token: ::protobuf::SingularField<::std::string::String>,
    redirect_uri: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for COAuthToken_ImplicitGrantNoPrompt_Response {}

impl COAuthToken_ImplicitGrantNoPrompt_Response {
    pub fn new() -> COAuthToken_ImplicitGrantNoPrompt_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static COAuthToken_ImplicitGrantNoPrompt_Response {
        static mut instance: ::protobuf::lazy::Lazy<COAuthToken_ImplicitGrantNoPrompt_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const COAuthToken_ImplicitGrantNoPrompt_Response,
        };
        unsafe {
            instance.get(COAuthToken_ImplicitGrantNoPrompt_Response::new)
        }
    }

    // optional string access_token = 1;

    pub fn clear_access_token(&mut self) {
        self.access_token.clear();
    }

    pub fn has_access_token(&self) -> bool {
        self.access_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_token(&mut self, v: ::std::string::String) {
        self.access_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_access_token(&mut self) -> &mut ::std::string::String {
        if self.access_token.is_none() {
            self.access_token.set_default();
        };
        self.access_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_access_token(&mut self) -> ::std::string::String {
        self.access_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_access_token(&self) -> &str {
        match self.access_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_access_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.access_token
    }

    fn mut_access_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.access_token
    }

    // optional string redirect_uri = 2;

    pub fn clear_redirect_uri(&mut self) {
        self.redirect_uri.clear();
    }

    pub fn has_redirect_uri(&self) -> bool {
        self.redirect_uri.is_some()
    }

    // Param is passed by value, moved
    pub fn set_redirect_uri(&mut self, v: ::std::string::String) {
        self.redirect_uri = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_redirect_uri(&mut self) -> &mut ::std::string::String {
        if self.redirect_uri.is_none() {
            self.redirect_uri.set_default();
        };
        self.redirect_uri.as_mut().unwrap()
    }

    // Take field
    pub fn take_redirect_uri(&mut self) -> ::std::string::String {
        self.redirect_uri.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_redirect_uri(&self) -> &str {
        match self.redirect_uri.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_redirect_uri_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.redirect_uri
    }

    fn mut_redirect_uri_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.redirect_uri
    }
}

impl ::protobuf::Message for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.access_token)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.redirect_uri)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.access_token.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.redirect_uri.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.access_token.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.redirect_uri.as_ref() {
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

impl ::protobuf::MessageStatic for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn new() -> COAuthToken_ImplicitGrantNoPrompt_Response {
        COAuthToken_ImplicitGrantNoPrompt_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<COAuthToken_ImplicitGrantNoPrompt_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "access_token",
                    COAuthToken_ImplicitGrantNoPrompt_Response::get_access_token_for_reflect,
                    COAuthToken_ImplicitGrantNoPrompt_Response::mut_access_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "redirect_uri",
                    COAuthToken_ImplicitGrantNoPrompt_Response::get_redirect_uri_for_reflect,
                    COAuthToken_ImplicitGrantNoPrompt_Response::mut_redirect_uri_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<COAuthToken_ImplicitGrantNoPrompt_Response>(
                    "COAuthToken_ImplicitGrantNoPrompt_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn clear(&mut self) {
        self.clear_access_token();
        self.clear_redirect_uri();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for COAuthToken_ImplicitGrantNoPrompt_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x27, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f,
    0x6f, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x77, 0x6f, 0x72, 0x6b, 0x73,
    0x73, 0x64, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a,
    0x2e, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x75,
    0x6e, 0x69, 0x66, 0x69, 0x65, 0x64, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x73, 0x74, 0x65, 0x61,
    0x6d, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x73, 0x64, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x85, 0x01, 0x0a, 0x29, 0x43, 0x4f, 0x41, 0x75, 0x74, 0x68, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x5f,
    0x49, 0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x4e, 0x6f, 0x50,
    0x72, 0x6f, 0x6d, 0x70, 0x74, 0x5f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x58, 0x0a,
    0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x69, 0x64, 0x42, 0x3c, 0x82, 0xb5, 0x18, 0x38, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x49, 0x44, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x77, 0x68, 0x69,
    0x63, 0x68, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x73, 0x73, 0x75, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x22, 0xd1, 0x01, 0x0a, 0x2a, 0x43, 0x4f, 0x41, 0x75,
    0x74, 0x68, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x49, 0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74,
    0x47, 0x72, 0x61, 0x6e, 0x74, 0x4e, 0x6f, 0x50, 0x72, 0x6f, 0x6d, 0x70, 0x74, 0x5f, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x46, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73,
    0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x61, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x42, 0x23, 0x82, 0xb5, 0x18, 0x1f, 0x4f,
    0x41, 0x75, 0x74, 0x68, 0x20, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x2c, 0x20, 0x67, 0x72, 0x61, 0x6e,
    0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x12, 0x5b,
    0x0a, 0x0c, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x5f, 0x75, 0x72, 0x69, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x72, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x55, 0x72,
    0x69, 0x42, 0x38, 0x82, 0xb5, 0x18, 0x34, 0x52, 0x65, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x55, 0x52, 0x49, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20,
    0x64, 0x75, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x32, 0xbb, 0x02, 0x0a, 0x0a,
    0x4f, 0x41, 0x75, 0x74, 0x68, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0xf5, 0x01, 0x0a, 0x15, 0x49,
    0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x4e, 0x6f, 0x50, 0x72,
    0x6f, 0x6d, 0x70, 0x74, 0x12, 0x2f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4f, 0x41, 0x75,
    0x74, 0x68, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x49, 0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74,
    0x47, 0x72, 0x61, 0x6e, 0x74, 0x4e, 0x6f, 0x50, 0x72, 0x6f, 0x6d, 0x70, 0x74, 0x5f, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x30, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4f, 0x41,
    0x75, 0x74, 0x68, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x49, 0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69,
    0x74, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x4e, 0x6f, 0x50, 0x72, 0x6f, 0x6d, 0x70, 0x74, 0x5f, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x79, 0x82, 0xb5, 0x18, 0x75, 0x47, 0x72, 0x61,
    0x6e, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x20,
    0x4f, 0x41, 0x75, 0x74, 0x68, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x28, 0x67, 0x72, 0x61,
    0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x27, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x27, 0x29,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69,
    0x65, 0x64, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x49, 0x44, 0x20, 0x6f, 0x6e, 0x20,
    0x62, 0x65, 0x68, 0x61, 0x6c, 0x66, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x75, 0x73, 0x65, 0x72,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x6d, 0x70, 0x74, 0x69,
    0x6e, 0x67, 0x1a, 0x35, 0x82, 0xb5, 0x18, 0x31, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f,
    0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x20, 0x4f, 0x41, 0x75,
    0x74, 0x68, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x4a, 0xff, 0x06, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x14, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x04, 0x07, 0x37, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x72, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x07, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x07, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07,
    0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x07, 0x25, 0x71, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x07, 0x26, 0x70, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x07, 0x26,
    0x33, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x07, 0x26, 0x33, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x27, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x07, 0x36, 0x70, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x0a, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x08, 0x5d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0b, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x0b, 0x29, 0x5c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x0b, 0x2a, 0x5b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x0b, 0x2a, 0x37, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x2a, 0x37, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x2b, 0x36, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x0b, 0x3a,
    0x5b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x72, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x0c, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x0c, 0x29, 0x71, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x0c, 0x2a, 0x70, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x0c, 0x2a, 0x37, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x2a, 0x37, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x2b, 0x36, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x0c, 0x3a, 0x70,
    0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0f, 0x00, 0x14, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x06, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x03, 0x12,
    0x03, 0x10, 0x08, 0x5b, 0x0a, 0x0d, 0x0a, 0x06, 0x06, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x10, 0x08, 0x5b, 0x0a, 0x0e, 0x0a, 0x07, 0x06, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x10, 0x0f, 0x24, 0x0a, 0x0f, 0x0a, 0x08, 0x06, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x10, 0x0f, 0x24, 0x0a, 0x10, 0x0a, 0x09, 0x06, 0x00, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x10, 0x10, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x06, 0x00, 0x03, 0xe7, 0x07, 0x00,
    0x07, 0x12, 0x03, 0x10, 0x27, 0x5a, 0x0a, 0x0c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x11, 0x00, 0x13, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11,
    0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x11, 0x1b, 0x44,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x4f, 0x79, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x12, 0x10, 0xa6, 0x01, 0x0a, 0x10, 0x0a,
    0x08, 0x06, 0x00, 0x02, 0x00, 0x04, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x12, 0x10, 0xa6, 0x01, 0x0a,
    0x10, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x00, 0x04, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x12, 0x17,
    0x2b, 0x0a, 0x11, 0x0a, 0x0a, 0x06, 0x00, 0x02, 0x00, 0x04, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x12, 0x17, 0x2b, 0x0a, 0x12, 0x0a, 0x0b, 0x06, 0x00, 0x02, 0x00, 0x04, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x18, 0x2a, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x00,
    0x04, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x04, 0x12, 0x2e, 0xa5, 0x01,
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
