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
pub struct CMsgSteamDatagramRouterPingReply {
    // message fields
    client_timestamp: ::std::option::Option<u32>,
    latency_datacenter_ids: ::std::vec::Vec<u32>,
    latency_ping_ms: ::std::vec::Vec<u32>,
    your_public_ip: ::std::option::Option<u32>,
    server_time: ::std::option::Option<u32>,
    challenge: ::std::option::Option<u64>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    client_cookie: ::std::option::Option<u32>,
    scoring_penalty_relay_cluster: ::std::option::Option<u32>,
    scoring_penalty_datacenter_ids: ::std::vec::Vec<u32>,
    scoring_penalty_values: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRouterPingReply {}

impl CMsgSteamDatagramRouterPingReply {
    pub fn new() -> CMsgSteamDatagramRouterPingReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRouterPingReply {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRouterPingReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRouterPingReply,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRouterPingReply::new)
        }
    }

    // optional fixed32 client_timestamp = 1;

    pub fn clear_client_timestamp(&mut self) {
        self.client_timestamp = ::std::option::Option::None;
    }

    pub fn has_client_timestamp(&self) -> bool {
        self.client_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_timestamp(&mut self, v: u32) {
        self.client_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_client_timestamp(&self) -> u32 {
        self.client_timestamp.unwrap_or(0)
    }

    fn get_client_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_timestamp
    }

    fn mut_client_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_timestamp
    }

    // repeated fixed32 latency_datacenter_ids = 2;

    pub fn clear_latency_datacenter_ids(&mut self) {
        self.latency_datacenter_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_latency_datacenter_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.latency_datacenter_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_latency_datacenter_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.latency_datacenter_ids
    }

    // Take field
    pub fn take_latency_datacenter_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.latency_datacenter_ids, ::std::vec::Vec::new())
    }

    pub fn get_latency_datacenter_ids(&self) -> &[u32] {
        &self.latency_datacenter_ids
    }

    fn get_latency_datacenter_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.latency_datacenter_ids
    }

    fn mut_latency_datacenter_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.latency_datacenter_ids
    }

    // repeated uint32 latency_ping_ms = 3;

    pub fn clear_latency_ping_ms(&mut self) {
        self.latency_ping_ms.clear();
    }

    // Param is passed by value, moved
    pub fn set_latency_ping_ms(&mut self, v: ::std::vec::Vec<u32>) {
        self.latency_ping_ms = v;
    }

    // Mutable pointer to the field.
    pub fn mut_latency_ping_ms(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.latency_ping_ms
    }

    // Take field
    pub fn take_latency_ping_ms(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.latency_ping_ms, ::std::vec::Vec::new())
    }

    pub fn get_latency_ping_ms(&self) -> &[u32] {
        &self.latency_ping_ms
    }

    fn get_latency_ping_ms_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.latency_ping_ms
    }

    fn mut_latency_ping_ms_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.latency_ping_ms
    }

    // optional fixed32 your_public_ip = 4;

    pub fn clear_your_public_ip(&mut self) {
        self.your_public_ip = ::std::option::Option::None;
    }

    pub fn has_your_public_ip(&self) -> bool {
        self.your_public_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_your_public_ip(&mut self, v: u32) {
        self.your_public_ip = ::std::option::Option::Some(v);
    }

    pub fn get_your_public_ip(&self) -> u32 {
        self.your_public_ip.unwrap_or(0)
    }

    fn get_your_public_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.your_public_ip
    }

    fn mut_your_public_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.your_public_ip
    }

    // optional fixed32 server_time = 5;

    pub fn clear_server_time(&mut self) {
        self.server_time = ::std::option::Option::None;
    }

    pub fn has_server_time(&self) -> bool {
        self.server_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_time(&mut self, v: u32) {
        self.server_time = ::std::option::Option::Some(v);
    }

    pub fn get_server_time(&self) -> u32 {
        self.server_time.unwrap_or(0)
    }

    fn get_server_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_time
    }

    fn mut_server_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_time
    }

    // optional fixed64 challenge = 6;

    pub fn clear_challenge(&mut self) {
        self.challenge = ::std::option::Option::None;
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: u64) {
        self.challenge = ::std::option::Option::Some(v);
    }

    pub fn get_challenge(&self) -> u64 {
        self.challenge.unwrap_or(0)
    }

    fn get_challenge_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.challenge
    }

    // optional uint32 seconds_until_shutdown = 7;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }

    // optional fixed32 client_cookie = 8;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // optional uint32 scoring_penalty_relay_cluster = 9;

    pub fn clear_scoring_penalty_relay_cluster(&mut self) {
        self.scoring_penalty_relay_cluster = ::std::option::Option::None;
    }

    pub fn has_scoring_penalty_relay_cluster(&self) -> bool {
        self.scoring_penalty_relay_cluster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scoring_penalty_relay_cluster(&mut self, v: u32) {
        self.scoring_penalty_relay_cluster = ::std::option::Option::Some(v);
    }

    pub fn get_scoring_penalty_relay_cluster(&self) -> u32 {
        self.scoring_penalty_relay_cluster.unwrap_or(0)
    }

    fn get_scoring_penalty_relay_cluster_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.scoring_penalty_relay_cluster
    }

    fn mut_scoring_penalty_relay_cluster_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.scoring_penalty_relay_cluster
    }

    // repeated fixed32 scoring_penalty_datacenter_ids = 10;

    pub fn clear_scoring_penalty_datacenter_ids(&mut self) {
        self.scoring_penalty_datacenter_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_scoring_penalty_datacenter_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.scoring_penalty_datacenter_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scoring_penalty_datacenter_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.scoring_penalty_datacenter_ids
    }

    // Take field
    pub fn take_scoring_penalty_datacenter_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.scoring_penalty_datacenter_ids, ::std::vec::Vec::new())
    }

    pub fn get_scoring_penalty_datacenter_ids(&self) -> &[u32] {
        &self.scoring_penalty_datacenter_ids
    }

    fn get_scoring_penalty_datacenter_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.scoring_penalty_datacenter_ids
    }

    fn mut_scoring_penalty_datacenter_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.scoring_penalty_datacenter_ids
    }

    // repeated uint32 scoring_penalty_values = 11;

    pub fn clear_scoring_penalty_values(&mut self) {
        self.scoring_penalty_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_scoring_penalty_values(&mut self, v: ::std::vec::Vec<u32>) {
        self.scoring_penalty_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_scoring_penalty_values(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.scoring_penalty_values
    }

    // Take field
    pub fn take_scoring_penalty_values(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.scoring_penalty_values, ::std::vec::Vec::new())
    }

    pub fn get_scoring_penalty_values(&self) -> &[u32] {
        &self.scoring_penalty_values
    }

    fn get_scoring_penalty_values_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.scoring_penalty_values
    }

    fn mut_scoring_penalty_values_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.scoring_penalty_values
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRouterPingReply {
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
                    self.client_timestamp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.latency_datacenter_ids)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.latency_ping_ms)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.your_public_ip = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.server_time = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.challenge = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.scoring_penalty_relay_cluster = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.scoring_penalty_datacenter_ids)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.scoring_penalty_values)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_timestamp {
            my_size += 5;
        };
        if !self.latency_datacenter_ids.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.latency_datacenter_ids.len() as u32) + (self.latency_datacenter_ids.len() * 4) as u32;
        };
        if !self.latency_ping_ms.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.latency_ping_ms);
        };
        if let Some(v) = self.your_public_ip {
            my_size += 5;
        };
        if let Some(v) = self.server_time {
            my_size += 5;
        };
        if let Some(v) = self.challenge {
            my_size += 9;
        };
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        if let Some(v) = self.scoring_penalty_relay_cluster {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.scoring_penalty_datacenter_ids.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.scoring_penalty_datacenter_ids.len() as u32) + (self.scoring_penalty_datacenter_ids.len() * 4) as u32;
        };
        if !self.scoring_penalty_values.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(11, &self.scoring_penalty_values);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_timestamp {
            os.write_fixed32(1, v)?;
        };
        if !self.latency_datacenter_ids.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.latency_datacenter_ids.len() * 4) as u32)?;
            for v in &self.latency_datacenter_ids {
                os.write_fixed32_no_tag(*v)?;
            };
        };
        if !self.latency_ping_ms.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.latency_ping_ms))?;
            for v in &self.latency_ping_ms {
                os.write_uint32_no_tag(*v)?;
            };
        };
        if let Some(v) = self.your_public_ip {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.server_time {
            os.write_fixed32(5, v)?;
        };
        if let Some(v) = self.challenge {
            os.write_fixed64(6, v)?;
        };
        if let Some(v) = self.seconds_until_shutdown {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.client_cookie {
            os.write_fixed32(8, v)?;
        };
        if let Some(v) = self.scoring_penalty_relay_cluster {
            os.write_uint32(9, v)?;
        };
        if !self.scoring_penalty_datacenter_ids.is_empty() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.scoring_penalty_datacenter_ids.len() * 4) as u32)?;
            for v in &self.scoring_penalty_datacenter_ids {
                os.write_fixed32_no_tag(*v)?;
            };
        };
        if !self.scoring_penalty_values.is_empty() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.scoring_penalty_values))?;
            for v in &self.scoring_penalty_values {
                os.write_uint32_no_tag(*v)?;
            };
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSteamDatagramRouterPingReply {
    fn new() -> CMsgSteamDatagramRouterPingReply {
        CMsgSteamDatagramRouterPingReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRouterPingReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_timestamp",
                    CMsgSteamDatagramRouterPingReply::get_client_timestamp_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_client_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "latency_datacenter_ids",
                    CMsgSteamDatagramRouterPingReply::get_latency_datacenter_ids_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_latency_datacenter_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "latency_ping_ms",
                    CMsgSteamDatagramRouterPingReply::get_latency_ping_ms_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_latency_ping_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "your_public_ip",
                    CMsgSteamDatagramRouterPingReply::get_your_public_ip_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_your_public_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_time",
                    CMsgSteamDatagramRouterPingReply::get_server_time_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_server_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "challenge",
                    CMsgSteamDatagramRouterPingReply::get_challenge_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramRouterPingReply::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_seconds_until_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramRouterPingReply::get_client_cookie_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "scoring_penalty_relay_cluster",
                    CMsgSteamDatagramRouterPingReply::get_scoring_penalty_relay_cluster_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_scoring_penalty_relay_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "scoring_penalty_datacenter_ids",
                    CMsgSteamDatagramRouterPingReply::get_scoring_penalty_datacenter_ids_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_scoring_penalty_datacenter_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "scoring_penalty_values",
                    CMsgSteamDatagramRouterPingReply::get_scoring_penalty_values_for_reflect,
                    CMsgSteamDatagramRouterPingReply::mut_scoring_penalty_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRouterPingReply>(
                    "CMsgSteamDatagramRouterPingReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRouterPingReply {
    fn clear(&mut self) {
        self.clear_client_timestamp();
        self.clear_latency_datacenter_ids();
        self.clear_latency_ping_ms();
        self.clear_your_public_ip();
        self.clear_server_time();
        self.clear_challenge();
        self.clear_seconds_until_shutdown();
        self.clear_client_cookie();
        self.clear_scoring_penalty_relay_cluster();
        self.clear_scoring_penalty_datacenter_ids();
        self.clear_scoring_penalty_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRouterPingReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRouterPingReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramGameserverPing {
    // message fields
    client_session: ::std::option::Option<u32>,
    client_steam_id: ::std::option::Option<u64>,
    client_timestamp: ::std::option::Option<u32>,
    router_timestamp: ::std::option::Option<u32>,
    router_gameserver_latency: ::std::option::Option<u32>,
    seq_number_router: ::std::option::Option<u32>,
    seq_number_e2e: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramGameserverPing {}

impl CMsgSteamDatagramGameserverPing {
    pub fn new() -> CMsgSteamDatagramGameserverPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramGameserverPing {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramGameserverPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramGameserverPing,
        };
        unsafe {
            instance.get(CMsgSteamDatagramGameserverPing::new)
        }
    }

    // optional uint32 client_session = 1;

    pub fn clear_client_session(&mut self) {
        self.client_session = ::std::option::Option::None;
    }

    pub fn has_client_session(&self) -> bool {
        self.client_session.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_session(&mut self, v: u32) {
        self.client_session = ::std::option::Option::Some(v);
    }

    pub fn get_client_session(&self) -> u32 {
        self.client_session.unwrap_or(0)
    }

    fn get_client_session_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_session
    }

    fn mut_client_session_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_session
    }

    // optional fixed64 client_steam_id = 2;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional fixed32 client_timestamp = 3;

    pub fn clear_client_timestamp(&mut self) {
        self.client_timestamp = ::std::option::Option::None;
    }

    pub fn has_client_timestamp(&self) -> bool {
        self.client_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_timestamp(&mut self, v: u32) {
        self.client_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_client_timestamp(&self) -> u32 {
        self.client_timestamp.unwrap_or(0)
    }

    fn get_client_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_timestamp
    }

    fn mut_client_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_timestamp
    }

    // optional fixed32 router_timestamp = 4;

    pub fn clear_router_timestamp(&mut self) {
        self.router_timestamp = ::std::option::Option::None;
    }

    pub fn has_router_timestamp(&self) -> bool {
        self.router_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_router_timestamp(&mut self, v: u32) {
        self.router_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_router_timestamp(&self) -> u32 {
        self.router_timestamp.unwrap_or(0)
    }

    fn get_router_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.router_timestamp
    }

    fn mut_router_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.router_timestamp
    }

    // optional uint32 router_gameserver_latency = 5;

    pub fn clear_router_gameserver_latency(&mut self) {
        self.router_gameserver_latency = ::std::option::Option::None;
    }

    pub fn has_router_gameserver_latency(&self) -> bool {
        self.router_gameserver_latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_router_gameserver_latency(&mut self, v: u32) {
        self.router_gameserver_latency = ::std::option::Option::Some(v);
    }

    pub fn get_router_gameserver_latency(&self) -> u32 {
        self.router_gameserver_latency.unwrap_or(0)
    }

    fn get_router_gameserver_latency_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.router_gameserver_latency
    }

    fn mut_router_gameserver_latency_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.router_gameserver_latency
    }

    // optional uint32 seq_number_router = 6;

    pub fn clear_seq_number_router(&mut self) {
        self.seq_number_router = ::std::option::Option::None;
    }

    pub fn has_seq_number_router(&self) -> bool {
        self.seq_number_router.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_number_router(&mut self, v: u32) {
        self.seq_number_router = ::std::option::Option::Some(v);
    }

    pub fn get_seq_number_router(&self) -> u32 {
        self.seq_number_router.unwrap_or(0)
    }

    fn get_seq_number_router_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_number_router
    }

    fn mut_seq_number_router_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_number_router
    }

    // optional uint32 seq_number_e2e = 7;

    pub fn clear_seq_number_e2e(&mut self) {
        self.seq_number_e2e = ::std::option::Option::None;
    }

    pub fn has_seq_number_e2e(&self) -> bool {
        self.seq_number_e2e.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_number_e2e(&mut self, v: u32) {
        self.seq_number_e2e = ::std::option::Option::Some(v);
    }

    pub fn get_seq_number_e2e(&self) -> u32 {
        self.seq_number_e2e.unwrap_or(0)
    }

    fn get_seq_number_e2e_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_number_e2e
    }

    fn mut_seq_number_e2e_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_number_e2e
    }
}

impl ::protobuf::Message for CMsgSteamDatagramGameserverPing {
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
                    self.client_session = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.router_timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.router_gameserver_latency = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_number_router = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_number_e2e = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_session {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        };
        if let Some(v) = self.client_timestamp {
            my_size += 5;
        };
        if let Some(v) = self.router_timestamp {
            my_size += 5;
        };
        if let Some(v) = self.router_gameserver_latency {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seq_number_router {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seq_number_e2e {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_session {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.client_timestamp {
            os.write_fixed32(3, v)?;
        };
        if let Some(v) = self.router_timestamp {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.router_gameserver_latency {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.seq_number_router {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.seq_number_e2e {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramGameserverPing {
    fn new() -> CMsgSteamDatagramGameserverPing {
        CMsgSteamDatagramGameserverPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramGameserverPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_session",
                    CMsgSteamDatagramGameserverPing::get_client_session_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_client_session_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgSteamDatagramGameserverPing::get_client_steam_id_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_timestamp",
                    CMsgSteamDatagramGameserverPing::get_client_timestamp_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_client_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "router_timestamp",
                    CMsgSteamDatagramGameserverPing::get_router_timestamp_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_router_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "router_gameserver_latency",
                    CMsgSteamDatagramGameserverPing::get_router_gameserver_latency_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_router_gameserver_latency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_number_router",
                    CMsgSteamDatagramGameserverPing::get_seq_number_router_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_seq_number_router_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_number_e2e",
                    CMsgSteamDatagramGameserverPing::get_seq_number_e2e_for_reflect,
                    CMsgSteamDatagramGameserverPing::mut_seq_number_e2e_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramGameserverPing>(
                    "CMsgSteamDatagramGameserverPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramGameserverPing {
    fn clear(&mut self) {
        self.clear_client_session();
        self.clear_client_steam_id();
        self.clear_client_timestamp();
        self.clear_router_timestamp();
        self.clear_router_gameserver_latency();
        self.clear_seq_number_router();
        self.clear_seq_number_e2e();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramGameserverPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramGameserverPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramGameServerAuthTicket {
    // message fields
    time_expiry: ::std::option::Option<u32>,
    authorized_steam_id: ::std::option::Option<u64>,
    authorized_public_ip: ::std::option::Option<u32>,
    gameserver_steam_id: ::std::option::Option<u64>,
    gameserver_net_id: ::std::option::Option<u64>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    app_id: ::std::option::Option<u32>,
    extra_fields: ::protobuf::RepeatedField<CMsgSteamDatagramGameServerAuthTicket_ExtraField>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramGameServerAuthTicket {}

impl CMsgSteamDatagramGameServerAuthTicket {
    pub fn new() -> CMsgSteamDatagramGameServerAuthTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramGameServerAuthTicket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramGameServerAuthTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramGameServerAuthTicket,
        };
        unsafe {
            instance.get(CMsgSteamDatagramGameServerAuthTicket::new)
        }
    }

    // optional fixed32 time_expiry = 1;

    pub fn clear_time_expiry(&mut self) {
        self.time_expiry = ::std::option::Option::None;
    }

    pub fn has_time_expiry(&self) -> bool {
        self.time_expiry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_expiry(&mut self, v: u32) {
        self.time_expiry = ::std::option::Option::Some(v);
    }

    pub fn get_time_expiry(&self) -> u32 {
        self.time_expiry.unwrap_or(0)
    }

    fn get_time_expiry_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_expiry
    }

    fn mut_time_expiry_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_expiry
    }

    // optional fixed64 authorized_steam_id = 2;

    pub fn clear_authorized_steam_id(&mut self) {
        self.authorized_steam_id = ::std::option::Option::None;
    }

    pub fn has_authorized_steam_id(&self) -> bool {
        self.authorized_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_authorized_steam_id(&mut self, v: u64) {
        self.authorized_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_authorized_steam_id(&self) -> u64 {
        self.authorized_steam_id.unwrap_or(0)
    }

    fn get_authorized_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.authorized_steam_id
    }

    fn mut_authorized_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.authorized_steam_id
    }

    // optional fixed32 authorized_public_ip = 3;

    pub fn clear_authorized_public_ip(&mut self) {
        self.authorized_public_ip = ::std::option::Option::None;
    }

    pub fn has_authorized_public_ip(&self) -> bool {
        self.authorized_public_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_authorized_public_ip(&mut self, v: u32) {
        self.authorized_public_ip = ::std::option::Option::Some(v);
    }

    pub fn get_authorized_public_ip(&self) -> u32 {
        self.authorized_public_ip.unwrap_or(0)
    }

    fn get_authorized_public_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.authorized_public_ip
    }

    fn mut_authorized_public_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.authorized_public_ip
    }

    // optional fixed64 gameserver_steam_id = 4;

    pub fn clear_gameserver_steam_id(&mut self) {
        self.gameserver_steam_id = ::std::option::Option::None;
    }

    pub fn has_gameserver_steam_id(&self) -> bool {
        self.gameserver_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameserver_steam_id(&mut self, v: u64) {
        self.gameserver_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_gameserver_steam_id(&self) -> u64 {
        self.gameserver_steam_id.unwrap_or(0)
    }

    fn get_gameserver_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.gameserver_steam_id
    }

    fn mut_gameserver_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.gameserver_steam_id
    }

    // optional fixed64 gameserver_net_id = 5;

    pub fn clear_gameserver_net_id(&mut self) {
        self.gameserver_net_id = ::std::option::Option::None;
    }

    pub fn has_gameserver_net_id(&self) -> bool {
        self.gameserver_net_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameserver_net_id(&mut self, v: u64) {
        self.gameserver_net_id = ::std::option::Option::Some(v);
    }

    pub fn get_gameserver_net_id(&self) -> u64 {
        self.gameserver_net_id.unwrap_or(0)
    }

    fn get_gameserver_net_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.gameserver_net_id
    }

    fn mut_gameserver_net_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.gameserver_net_id
    }

    // optional bytes signature = 6;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        };
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }

    // optional uint32 app_id = 7;

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    fn get_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.app_id
    }

    fn mut_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.app_id
    }

    // repeated .dota.CMsgSteamDatagramGameServerAuthTicket.ExtraField extra_fields = 8;

    pub fn clear_extra_fields(&mut self) {
        self.extra_fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_extra_fields(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramGameServerAuthTicket_ExtraField>) {
        self.extra_fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extra_fields(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramGameServerAuthTicket_ExtraField> {
        &mut self.extra_fields
    }

    // Take field
    pub fn take_extra_fields(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramGameServerAuthTicket_ExtraField> {
        ::std::mem::replace(&mut self.extra_fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_extra_fields(&self) -> &[CMsgSteamDatagramGameServerAuthTicket_ExtraField] {
        &self.extra_fields
    }

    fn get_extra_fields_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramGameServerAuthTicket_ExtraField> {
        &self.extra_fields
    }

    fn mut_extra_fields_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramGameServerAuthTicket_ExtraField> {
        &mut self.extra_fields
    }
}

impl ::protobuf::Message for CMsgSteamDatagramGameServerAuthTicket {
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
                    self.time_expiry = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.authorized_steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.authorized_public_ip = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.gameserver_steam_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.gameserver_net_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extra_fields)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.time_expiry {
            my_size += 5;
        };
        if let Some(v) = self.authorized_steam_id {
            my_size += 9;
        };
        if let Some(v) = self.authorized_public_ip {
            my_size += 5;
        };
        if let Some(v) = self.gameserver_steam_id {
            my_size += 9;
        };
        if let Some(v) = self.gameserver_net_id {
            my_size += 9;
        };
        if let Some(v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        };
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.extra_fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.time_expiry {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.authorized_steam_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.authorized_public_ip {
            os.write_fixed32(3, v)?;
        };
        if let Some(v) = self.gameserver_steam_id {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.gameserver_net_id {
            os.write_fixed64(5, v)?;
        };
        if let Some(v) = self.signature.as_ref() {
            os.write_bytes(6, &v)?;
        };
        if let Some(v) = self.app_id {
            os.write_uint32(7, v)?;
        };
        for v in &self.extra_fields {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramGameServerAuthTicket {
    fn new() -> CMsgSteamDatagramGameServerAuthTicket {
        CMsgSteamDatagramGameServerAuthTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramGameServerAuthTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "time_expiry",
                    CMsgSteamDatagramGameServerAuthTicket::get_time_expiry_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket::mut_time_expiry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "authorized_steam_id",
                    CMsgSteamDatagramGameServerAuthTicket::get_authorized_steam_id_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket::mut_authorized_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "authorized_public_ip",
                    CMsgSteamDatagramGameServerAuthTicket::get_authorized_public_ip_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket::mut_authorized_public_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "gameserver_steam_id",
                    CMsgSteamDatagramGameServerAuthTicket::get_gameserver_steam_id_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket::mut_gameserver_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "gameserver_net_id",
                    CMsgSteamDatagramGameServerAuthTicket::get_gameserver_net_id_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket::mut_gameserver_net_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    CMsgSteamDatagramGameServerAuthTicket::get_signature_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "app_id",
                    CMsgSteamDatagramGameServerAuthTicket::get_app_id_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket::mut_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramGameServerAuthTicket_ExtraField>>(
                    "extra_fields",
                    CMsgSteamDatagramGameServerAuthTicket::get_extra_fields_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket::mut_extra_fields_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramGameServerAuthTicket>(
                    "CMsgSteamDatagramGameServerAuthTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramGameServerAuthTicket {
    fn clear(&mut self) {
        self.clear_time_expiry();
        self.clear_authorized_steam_id();
        self.clear_authorized_public_ip();
        self.clear_gameserver_steam_id();
        self.clear_gameserver_net_id();
        self.clear_signature();
        self.clear_app_id();
        self.clear_extra_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramGameServerAuthTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramGameServerAuthTicket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramGameServerAuthTicket_ExtraField {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    string_value: ::protobuf::SingularField<::std::string::String>,
    int32_value: ::std::option::Option<i32>,
    fixed32_value: ::std::option::Option<u32>,
    fixed64_value: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramGameServerAuthTicket_ExtraField {}

impl CMsgSteamDatagramGameServerAuthTicket_ExtraField {
    pub fn new() -> CMsgSteamDatagramGameServerAuthTicket_ExtraField {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramGameServerAuthTicket_ExtraField {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramGameServerAuthTicket_ExtraField> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramGameServerAuthTicket_ExtraField,
        };
        unsafe {
            instance.get(CMsgSteamDatagramGameServerAuthTicket_ExtraField::new)
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

    // optional string string_value = 2;

    pub fn clear_string_value(&mut self) {
        self.string_value.clear();
    }

    pub fn has_string_value(&self) -> bool {
        self.string_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::string::String) {
        self.string_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_value(&mut self) -> &mut ::std::string::String {
        if self.string_value.is_none() {
            self.string_value.set_default();
        };
        self.string_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::string::String {
        self.string_value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_string_value(&self) -> &str {
        match self.string_value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_string_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.string_value
    }

    fn mut_string_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.string_value
    }

    // optional sint32 int32_value = 3;

    pub fn clear_int32_value(&mut self) {
        self.int32_value = ::std::option::Option::None;
    }

    pub fn has_int32_value(&self) -> bool {
        self.int32_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_int32_value(&mut self, v: i32) {
        self.int32_value = ::std::option::Option::Some(v);
    }

    pub fn get_int32_value(&self) -> i32 {
        self.int32_value.unwrap_or(0)
    }

    fn get_int32_value_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.int32_value
    }

    fn mut_int32_value_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.int32_value
    }

    // optional fixed32 fixed32_value = 4;

    pub fn clear_fixed32_value(&mut self) {
        self.fixed32_value = ::std::option::Option::None;
    }

    pub fn has_fixed32_value(&self) -> bool {
        self.fixed32_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed32_value(&mut self, v: u32) {
        self.fixed32_value = ::std::option::Option::Some(v);
    }

    pub fn get_fixed32_value(&self) -> u32 {
        self.fixed32_value.unwrap_or(0)
    }

    fn get_fixed32_value_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.fixed32_value
    }

    fn mut_fixed32_value_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.fixed32_value
    }

    // optional fixed64 fixed64_value = 5;

    pub fn clear_fixed64_value(&mut self) {
        self.fixed64_value = ::std::option::Option::None;
    }

    pub fn has_fixed64_value(&self) -> bool {
        self.fixed64_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fixed64_value(&mut self, v: u64) {
        self.fixed64_value = ::std::option::Option::Some(v);
    }

    pub fn get_fixed64_value(&self) -> u64 {
        self.fixed64_value.unwrap_or(0)
    }

    fn get_fixed64_value_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fixed64_value
    }

    fn mut_fixed64_value_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fixed64_value
    }
}

impl ::protobuf::Message for CMsgSteamDatagramGameServerAuthTicket_ExtraField {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.string_value)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint32()?;
                    self.int32_value = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.fixed32_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.fixed64_value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        if let Some(v) = self.string_value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.int32_value {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        };
        if let Some(v) = self.fixed32_value {
            my_size += 5;
        };
        if let Some(v) = self.fixed64_value {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.string_value.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.int32_value {
            os.write_sint32(3, v)?;
        };
        if let Some(v) = self.fixed32_value {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.fixed64_value {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramGameServerAuthTicket_ExtraField {
    fn new() -> CMsgSteamDatagramGameServerAuthTicket_ExtraField {
        CMsgSteamDatagramGameServerAuthTicket_ExtraField::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramGameServerAuthTicket_ExtraField>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::get_name_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "string_value",
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::get_string_value_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::mut_string_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "int32_value",
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::get_int32_value_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::mut_int32_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "fixed32_value",
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::get_fixed32_value_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::mut_fixed32_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "fixed64_value",
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::get_fixed64_value_for_reflect,
                    CMsgSteamDatagramGameServerAuthTicket_ExtraField::mut_fixed64_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramGameServerAuthTicket_ExtraField>(
                    "CMsgSteamDatagramGameServerAuthTicket_ExtraField",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramGameServerAuthTicket_ExtraField {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_string_value();
        self.clear_int32_value();
        self.clear_fixed32_value();
        self.clear_fixed64_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramGameServerAuthTicket_ExtraField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramGameServerAuthTicket_ExtraField {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramGameserverSessionRequest {
    // message fields
    ticket: ::protobuf::SingularPtrField<CMsgSteamDatagramGameServerAuthTicket>,
    challenge_time: ::std::option::Option<u32>,
    challenge: ::std::option::Option<u64>,
    client_cookie: ::std::option::Option<u32>,
    network_config_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramGameserverSessionRequest {}

impl CMsgSteamDatagramGameserverSessionRequest {
    pub fn new() -> CMsgSteamDatagramGameserverSessionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramGameserverSessionRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramGameserverSessionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramGameserverSessionRequest,
        };
        unsafe {
            instance.get(CMsgSteamDatagramGameserverSessionRequest::new)
        }
    }

    // optional .dota.CMsgSteamDatagramGameServerAuthTicket ticket = 1;

    pub fn clear_ticket(&mut self) {
        self.ticket.clear();
    }

    pub fn has_ticket(&self) -> bool {
        self.ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ticket(&mut self, v: CMsgSteamDatagramGameServerAuthTicket) {
        self.ticket = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ticket(&mut self) -> &mut CMsgSteamDatagramGameServerAuthTicket {
        if self.ticket.is_none() {
            self.ticket.set_default();
        };
        self.ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_ticket(&mut self) -> CMsgSteamDatagramGameServerAuthTicket {
        self.ticket.take().unwrap_or_else(|| CMsgSteamDatagramGameServerAuthTicket::new())
    }

    pub fn get_ticket(&self) -> &CMsgSteamDatagramGameServerAuthTicket {
        self.ticket.as_ref().unwrap_or_else(|| CMsgSteamDatagramGameServerAuthTicket::default_instance())
    }

    fn get_ticket_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramGameServerAuthTicket> {
        &self.ticket
    }

    fn mut_ticket_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramGameServerAuthTicket> {
        &mut self.ticket
    }

    // optional fixed32 challenge_time = 3;

    pub fn clear_challenge_time(&mut self) {
        self.challenge_time = ::std::option::Option::None;
    }

    pub fn has_challenge_time(&self) -> bool {
        self.challenge_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge_time(&mut self, v: u32) {
        self.challenge_time = ::std::option::Option::Some(v);
    }

    pub fn get_challenge_time(&self) -> u32 {
        self.challenge_time.unwrap_or(0)
    }

    fn get_challenge_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.challenge_time
    }

    fn mut_challenge_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.challenge_time
    }

    // optional fixed64 challenge = 4;

    pub fn clear_challenge(&mut self) {
        self.challenge = ::std::option::Option::None;
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: u64) {
        self.challenge = ::std::option::Option::Some(v);
    }

    pub fn get_challenge(&self) -> u64 {
        self.challenge.unwrap_or(0)
    }

    fn get_challenge_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.challenge
    }

    // optional fixed32 client_cookie = 5;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // optional uint32 network_config_version = 6;

    pub fn clear_network_config_version(&mut self) {
        self.network_config_version = ::std::option::Option::None;
    }

    pub fn has_network_config_version(&self) -> bool {
        self.network_config_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network_config_version(&mut self, v: u32) {
        self.network_config_version = ::std::option::Option::Some(v);
    }

    pub fn get_network_config_version(&self) -> u32 {
        self.network_config_version.unwrap_or(0)
    }

    fn get_network_config_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.network_config_version
    }

    fn mut_network_config_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.network_config_version
    }
}

impl ::protobuf::Message for CMsgSteamDatagramGameserverSessionRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ticket)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.challenge_time = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.challenge = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.network_config_version = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ticket.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.challenge_time {
            my_size += 5;
        };
        if let Some(v) = self.challenge {
            my_size += 9;
        };
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        if let Some(v) = self.network_config_version {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ticket.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.challenge_time {
            os.write_fixed32(3, v)?;
        };
        if let Some(v) = self.challenge {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.client_cookie {
            os.write_fixed32(5, v)?;
        };
        if let Some(v) = self.network_config_version {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramGameserverSessionRequest {
    fn new() -> CMsgSteamDatagramGameserverSessionRequest {
        CMsgSteamDatagramGameserverSessionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramGameserverSessionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramGameServerAuthTicket>>(
                    "ticket",
                    CMsgSteamDatagramGameserverSessionRequest::get_ticket_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_ticket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "challenge_time",
                    CMsgSteamDatagramGameserverSessionRequest::get_challenge_time_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_challenge_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "challenge",
                    CMsgSteamDatagramGameserverSessionRequest::get_challenge_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramGameserverSessionRequest::get_client_cookie_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "network_config_version",
                    CMsgSteamDatagramGameserverSessionRequest::get_network_config_version_for_reflect,
                    CMsgSteamDatagramGameserverSessionRequest::mut_network_config_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramGameserverSessionRequest>(
                    "CMsgSteamDatagramGameserverSessionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramGameserverSessionRequest {
    fn clear(&mut self) {
        self.clear_ticket();
        self.clear_challenge_time();
        self.clear_challenge();
        self.clear_client_cookie();
        self.clear_network_config_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramGameserverSessionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramGameserverSessionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramGameserverSessionEstablished {
    // message fields
    client_cookie: ::std::option::Option<u32>,
    gameserver_steam_id: ::std::option::Option<u64>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    session_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramGameserverSessionEstablished {}

impl CMsgSteamDatagramGameserverSessionEstablished {
    pub fn new() -> CMsgSteamDatagramGameserverSessionEstablished {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramGameserverSessionEstablished {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramGameserverSessionEstablished> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramGameserverSessionEstablished,
        };
        unsafe {
            instance.get(CMsgSteamDatagramGameserverSessionEstablished::new)
        }
    }

    // optional fixed32 client_cookie = 1;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // optional fixed64 gameserver_steam_id = 3;

    pub fn clear_gameserver_steam_id(&mut self) {
        self.gameserver_steam_id = ::std::option::Option::None;
    }

    pub fn has_gameserver_steam_id(&self) -> bool {
        self.gameserver_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gameserver_steam_id(&mut self, v: u64) {
        self.gameserver_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_gameserver_steam_id(&self) -> u64 {
        self.gameserver_steam_id.unwrap_or(0)
    }

    fn get_gameserver_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.gameserver_steam_id
    }

    fn mut_gameserver_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.gameserver_steam_id
    }

    // optional uint32 seconds_until_shutdown = 4;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }

    // optional uint32 session_id = 5;

    pub fn clear_session_id(&mut self) {
        self.session_id = ::std::option::Option::None;
    }

    pub fn has_session_id(&self) -> bool {
        self.session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_session_id(&mut self, v: u32) {
        self.session_id = ::std::option::Option::Some(v);
    }

    pub fn get_session_id(&self) -> u32 {
        self.session_id.unwrap_or(0)
    }

    fn get_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.session_id
    }

    fn mut_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.session_id
    }
}

impl ::protobuf::Message for CMsgSteamDatagramGameserverSessionEstablished {
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
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.gameserver_steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.session_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        if let Some(v) = self.gameserver_steam_id {
            my_size += 9;
        };
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.session_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_cookie {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.gameserver_steam_id {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.seconds_until_shutdown {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.session_id {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramGameserverSessionEstablished {
    fn new() -> CMsgSteamDatagramGameserverSessionEstablished {
        CMsgSteamDatagramGameserverSessionEstablished::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramGameserverSessionEstablished>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramGameserverSessionEstablished::get_client_cookie_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "gameserver_steam_id",
                    CMsgSteamDatagramGameserverSessionEstablished::get_gameserver_steam_id_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_gameserver_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramGameserverSessionEstablished::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_seconds_until_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "session_id",
                    CMsgSteamDatagramGameserverSessionEstablished::get_session_id_for_reflect,
                    CMsgSteamDatagramGameserverSessionEstablished::mut_session_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramGameserverSessionEstablished>(
                    "CMsgSteamDatagramGameserverSessionEstablished",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramGameserverSessionEstablished {
    fn clear(&mut self) {
        self.clear_client_cookie();
        self.clear_gameserver_steam_id();
        self.clear_seconds_until_shutdown();
        self.clear_session_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramGameserverSessionEstablished {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramGameserverSessionEstablished {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramNoSession {
    // message fields
    client_cookie: ::std::option::Option<u32>,
    your_public_ip: ::std::option::Option<u32>,
    server_time: ::std::option::Option<u32>,
    challenge: ::std::option::Option<u64>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramNoSession {}

impl CMsgSteamDatagramNoSession {
    pub fn new() -> CMsgSteamDatagramNoSession {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramNoSession {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramNoSession> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramNoSession,
        };
        unsafe {
            instance.get(CMsgSteamDatagramNoSession::new)
        }
    }

    // optional fixed32 client_cookie = 7;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // optional fixed32 your_public_ip = 2;

    pub fn clear_your_public_ip(&mut self) {
        self.your_public_ip = ::std::option::Option::None;
    }

    pub fn has_your_public_ip(&self) -> bool {
        self.your_public_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_your_public_ip(&mut self, v: u32) {
        self.your_public_ip = ::std::option::Option::Some(v);
    }

    pub fn get_your_public_ip(&self) -> u32 {
        self.your_public_ip.unwrap_or(0)
    }

    fn get_your_public_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.your_public_ip
    }

    fn mut_your_public_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.your_public_ip
    }

    // optional fixed32 server_time = 3;

    pub fn clear_server_time(&mut self) {
        self.server_time = ::std::option::Option::None;
    }

    pub fn has_server_time(&self) -> bool {
        self.server_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_time(&mut self, v: u32) {
        self.server_time = ::std::option::Option::Some(v);
    }

    pub fn get_server_time(&self) -> u32 {
        self.server_time.unwrap_or(0)
    }

    fn get_server_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_time
    }

    fn mut_server_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_time
    }

    // optional fixed64 challenge = 4;

    pub fn clear_challenge(&mut self) {
        self.challenge = ::std::option::Option::None;
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: u64) {
        self.challenge = ::std::option::Option::Some(v);
    }

    pub fn get_challenge(&self) -> u64 {
        self.challenge.unwrap_or(0)
    }

    fn get_challenge_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.challenge
    }

    // optional uint32 seconds_until_shutdown = 5;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }
}

impl ::protobuf::Message for CMsgSteamDatagramNoSession {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.your_public_ip = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.server_time = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.challenge = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        if let Some(v) = self.your_public_ip {
            my_size += 5;
        };
        if let Some(v) = self.server_time {
            my_size += 5;
        };
        if let Some(v) = self.challenge {
            my_size += 9;
        };
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_cookie {
            os.write_fixed32(7, v)?;
        };
        if let Some(v) = self.your_public_ip {
            os.write_fixed32(2, v)?;
        };
        if let Some(v) = self.server_time {
            os.write_fixed32(3, v)?;
        };
        if let Some(v) = self.challenge {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.seconds_until_shutdown {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramNoSession {
    fn new() -> CMsgSteamDatagramNoSession {
        CMsgSteamDatagramNoSession::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramNoSession>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramNoSession::get_client_cookie_for_reflect,
                    CMsgSteamDatagramNoSession::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "your_public_ip",
                    CMsgSteamDatagramNoSession::get_your_public_ip_for_reflect,
                    CMsgSteamDatagramNoSession::mut_your_public_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "server_time",
                    CMsgSteamDatagramNoSession::get_server_time_for_reflect,
                    CMsgSteamDatagramNoSession::mut_server_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "challenge",
                    CMsgSteamDatagramNoSession::get_challenge_for_reflect,
                    CMsgSteamDatagramNoSession::mut_challenge_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramNoSession::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramNoSession::mut_seconds_until_shutdown_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramNoSession>(
                    "CMsgSteamDatagramNoSession",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramNoSession {
    fn clear(&mut self) {
        self.clear_client_cookie();
        self.clear_your_public_ip();
        self.clear_server_time();
        self.clear_challenge();
        self.clear_seconds_until_shutdown();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramNoSession {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramNoSession {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramDiagnostic {
    // message fields
    severity: ::std::option::Option<u32>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramDiagnostic {}

impl CMsgSteamDatagramDiagnostic {
    pub fn new() -> CMsgSteamDatagramDiagnostic {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramDiagnostic {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramDiagnostic> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramDiagnostic,
        };
        unsafe {
            instance.get(CMsgSteamDatagramDiagnostic::new)
        }
    }

    // optional uint32 severity = 1;

    pub fn clear_severity(&mut self) {
        self.severity = ::std::option::Option::None;
    }

    pub fn has_severity(&self) -> bool {
        self.severity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_severity(&mut self, v: u32) {
        self.severity = ::std::option::Option::Some(v);
    }

    pub fn get_severity(&self) -> u32 {
        self.severity.unwrap_or(0)
    }

    fn get_severity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.severity
    }

    fn mut_severity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.severity
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
}

impl ::protobuf::Message for CMsgSteamDatagramDiagnostic {
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
                    self.severity = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        if let Some(v) = self.severity {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.severity {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.text.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramDiagnostic {
    fn new() -> CMsgSteamDatagramDiagnostic {
        CMsgSteamDatagramDiagnostic::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramDiagnostic>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "severity",
                    CMsgSteamDatagramDiagnostic::get_severity_for_reflect,
                    CMsgSteamDatagramDiagnostic::mut_severity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CMsgSteamDatagramDiagnostic::get_text_for_reflect,
                    CMsgSteamDatagramDiagnostic::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramDiagnostic>(
                    "CMsgSteamDatagramDiagnostic",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramDiagnostic {
    fn clear(&mut self) {
        self.clear_severity();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramDiagnostic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramDiagnostic {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramDataCenterState {
    // message fields
    data_centers: ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramDataCenterState {}

impl CMsgSteamDatagramDataCenterState {
    pub fn new() -> CMsgSteamDatagramDataCenterState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramDataCenterState {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramDataCenterState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramDataCenterState,
        };
        unsafe {
            instance.get(CMsgSteamDatagramDataCenterState::new)
        }
    }

    // repeated .dota.CMsgSteamDatagramDataCenterState.DataCenter data_centers = 1;

    pub fn clear_data_centers(&mut self) {
        self.data_centers.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_centers(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter>) {
        self.data_centers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data_centers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter> {
        &mut self.data_centers
    }

    // Take field
    pub fn take_data_centers(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter> {
        ::std::mem::replace(&mut self.data_centers, ::protobuf::RepeatedField::new())
    }

    pub fn get_data_centers(&self) -> &[CMsgSteamDatagramDataCenterState_DataCenter] {
        &self.data_centers
    }

    fn get_data_centers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter> {
        &self.data_centers
    }

    fn mut_data_centers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_DataCenter> {
        &mut self.data_centers
    }
}

impl ::protobuf::Message for CMsgSteamDatagramDataCenterState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data_centers)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.data_centers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.data_centers {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramDataCenterState {
    fn new() -> CMsgSteamDatagramDataCenterState {
        CMsgSteamDatagramDataCenterState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramDataCenterState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramDataCenterState_DataCenter>>(
                    "data_centers",
                    CMsgSteamDatagramDataCenterState::get_data_centers_for_reflect,
                    CMsgSteamDatagramDataCenterState::mut_data_centers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramDataCenterState>(
                    "CMsgSteamDatagramDataCenterState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramDataCenterState {
    fn clear(&mut self) {
        self.clear_data_centers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramDataCenterState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramDataCenterState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramDataCenterState_Server {
    // message fields
    address: ::protobuf::SingularField<::std::string::String>,
    ping_ms: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramDataCenterState_Server {}

impl CMsgSteamDatagramDataCenterState_Server {
    pub fn new() -> CMsgSteamDatagramDataCenterState_Server {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramDataCenterState_Server {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramDataCenterState_Server> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramDataCenterState_Server,
        };
        unsafe {
            instance.get(CMsgSteamDatagramDataCenterState_Server::new)
        }
    }

    // optional string address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        };
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_address_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.address
    }

    // optional uint32 ping_ms = 2;

    pub fn clear_ping_ms(&mut self) {
        self.ping_ms = ::std::option::Option::None;
    }

    pub fn has_ping_ms(&self) -> bool {
        self.ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ms(&mut self, v: u32) {
        self.ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ms(&self) -> u32 {
        self.ping_ms.unwrap_or(0)
    }

    fn get_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ms
    }

    fn mut_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ms
    }
}

impl ::protobuf::Message for CMsgSteamDatagramDataCenterState_Server {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_ms = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.address.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.ping_ms {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.address.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.ping_ms {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramDataCenterState_Server {
    fn new() -> CMsgSteamDatagramDataCenterState_Server {
        CMsgSteamDatagramDataCenterState_Server::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramDataCenterState_Server>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    CMsgSteamDatagramDataCenterState_Server::get_address_for_reflect,
                    CMsgSteamDatagramDataCenterState_Server::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ms",
                    CMsgSteamDatagramDataCenterState_Server::get_ping_ms_for_reflect,
                    CMsgSteamDatagramDataCenterState_Server::mut_ping_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramDataCenterState_Server>(
                    "CMsgSteamDatagramDataCenterState_Server",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramDataCenterState_Server {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_ping_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramDataCenterState_Server {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramDataCenterState_Server {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramDataCenterState_DataCenter {
    // message fields
    code: ::protobuf::SingularField<::std::string::String>,
    server_sample: ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramDataCenterState_DataCenter {}

impl CMsgSteamDatagramDataCenterState_DataCenter {
    pub fn new() -> CMsgSteamDatagramDataCenterState_DataCenter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramDataCenterState_DataCenter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramDataCenterState_DataCenter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramDataCenterState_DataCenter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramDataCenterState_DataCenter::new)
        }
    }

    // optional string code = 1;

    pub fn clear_code(&mut self) {
        self.code.clear();
    }

    pub fn has_code(&self) -> bool {
        self.code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: ::std::string::String) {
        self.code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code(&mut self) -> &mut ::std::string::String {
        if self.code.is_none() {
            self.code.set_default();
        };
        self.code.as_mut().unwrap()
    }

    // Take field
    pub fn take_code(&mut self) -> ::std::string::String {
        self.code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_code(&self) -> &str {
        match self.code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.code
    }

    // repeated .dota.CMsgSteamDatagramDataCenterState.Server server_sample = 2;

    pub fn clear_server_sample(&mut self) {
        self.server_sample.clear();
    }

    // Param is passed by value, moved
    pub fn set_server_sample(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server>) {
        self.server_sample = v;
    }

    // Mutable pointer to the field.
    pub fn mut_server_sample(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server> {
        &mut self.server_sample
    }

    // Take field
    pub fn take_server_sample(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server> {
        ::std::mem::replace(&mut self.server_sample, ::protobuf::RepeatedField::new())
    }

    pub fn get_server_sample(&self) -> &[CMsgSteamDatagramDataCenterState_Server] {
        &self.server_sample
    }

    fn get_server_sample_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server> {
        &self.server_sample
    }

    fn mut_server_sample_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramDataCenterState_Server> {
        &mut self.server_sample
    }
}

impl ::protobuf::Message for CMsgSteamDatagramDataCenterState_DataCenter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.code)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.server_sample)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.code.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        for value in &self.server_sample {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code.as_ref() {
            os.write_string(1, &v)?;
        };
        for v in &self.server_sample {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramDataCenterState_DataCenter {
    fn new() -> CMsgSteamDatagramDataCenterState_DataCenter {
        CMsgSteamDatagramDataCenterState_DataCenter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramDataCenterState_DataCenter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "code",
                    CMsgSteamDatagramDataCenterState_DataCenter::get_code_for_reflect,
                    CMsgSteamDatagramDataCenterState_DataCenter::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramDataCenterState_Server>>(
                    "server_sample",
                    CMsgSteamDatagramDataCenterState_DataCenter::get_server_sample_for_reflect,
                    CMsgSteamDatagramDataCenterState_DataCenter::mut_server_sample_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramDataCenterState_DataCenter>(
                    "CMsgSteamDatagramDataCenterState_DataCenter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramDataCenterState_DataCenter {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_server_sample();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramDataCenterState_DataCenter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramDataCenterState_DataCenter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramLinkInstantaneousStats {
    // message fields
    out_packets_per_sec_x10: ::std::option::Option<u32>,
    out_bytes_per_sec: ::std::option::Option<u32>,
    in_packets_per_sec_x10: ::std::option::Option<u32>,
    in_bytes_per_sec: ::std::option::Option<u32>,
    ping_ms: ::std::option::Option<u32>,
    packets_dropped_pct: ::std::option::Option<u32>,
    packets_weird_sequence_pct: ::std::option::Option<u32>,
    peak_jitter_usec: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramLinkInstantaneousStats {}

impl CMsgSteamDatagramLinkInstantaneousStats {
    pub fn new() -> CMsgSteamDatagramLinkInstantaneousStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramLinkInstantaneousStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramLinkInstantaneousStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramLinkInstantaneousStats,
        };
        unsafe {
            instance.get(CMsgSteamDatagramLinkInstantaneousStats::new)
        }
    }

    // optional uint32 out_packets_per_sec_x10 = 1;

    pub fn clear_out_packets_per_sec_x10(&mut self) {
        self.out_packets_per_sec_x10 = ::std::option::Option::None;
    }

    pub fn has_out_packets_per_sec_x10(&self) -> bool {
        self.out_packets_per_sec_x10.is_some()
    }

    // Param is passed by value, moved
    pub fn set_out_packets_per_sec_x10(&mut self, v: u32) {
        self.out_packets_per_sec_x10 = ::std::option::Option::Some(v);
    }

    pub fn get_out_packets_per_sec_x10(&self) -> u32 {
        self.out_packets_per_sec_x10.unwrap_or(0)
    }

    fn get_out_packets_per_sec_x10_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.out_packets_per_sec_x10
    }

    fn mut_out_packets_per_sec_x10_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.out_packets_per_sec_x10
    }

    // optional uint32 out_bytes_per_sec = 2;

    pub fn clear_out_bytes_per_sec(&mut self) {
        self.out_bytes_per_sec = ::std::option::Option::None;
    }

    pub fn has_out_bytes_per_sec(&self) -> bool {
        self.out_bytes_per_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_out_bytes_per_sec(&mut self, v: u32) {
        self.out_bytes_per_sec = ::std::option::Option::Some(v);
    }

    pub fn get_out_bytes_per_sec(&self) -> u32 {
        self.out_bytes_per_sec.unwrap_or(0)
    }

    fn get_out_bytes_per_sec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.out_bytes_per_sec
    }

    fn mut_out_bytes_per_sec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.out_bytes_per_sec
    }

    // optional uint32 in_packets_per_sec_x10 = 3;

    pub fn clear_in_packets_per_sec_x10(&mut self) {
        self.in_packets_per_sec_x10 = ::std::option::Option::None;
    }

    pub fn has_in_packets_per_sec_x10(&self) -> bool {
        self.in_packets_per_sec_x10.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_packets_per_sec_x10(&mut self, v: u32) {
        self.in_packets_per_sec_x10 = ::std::option::Option::Some(v);
    }

    pub fn get_in_packets_per_sec_x10(&self) -> u32 {
        self.in_packets_per_sec_x10.unwrap_or(0)
    }

    fn get_in_packets_per_sec_x10_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.in_packets_per_sec_x10
    }

    fn mut_in_packets_per_sec_x10_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.in_packets_per_sec_x10
    }

    // optional uint32 in_bytes_per_sec = 4;

    pub fn clear_in_bytes_per_sec(&mut self) {
        self.in_bytes_per_sec = ::std::option::Option::None;
    }

    pub fn has_in_bytes_per_sec(&self) -> bool {
        self.in_bytes_per_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_in_bytes_per_sec(&mut self, v: u32) {
        self.in_bytes_per_sec = ::std::option::Option::Some(v);
    }

    pub fn get_in_bytes_per_sec(&self) -> u32 {
        self.in_bytes_per_sec.unwrap_or(0)
    }

    fn get_in_bytes_per_sec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.in_bytes_per_sec
    }

    fn mut_in_bytes_per_sec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.in_bytes_per_sec
    }

    // optional uint32 ping_ms = 5;

    pub fn clear_ping_ms(&mut self) {
        self.ping_ms = ::std::option::Option::None;
    }

    pub fn has_ping_ms(&self) -> bool {
        self.ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ms(&mut self, v: u32) {
        self.ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ms(&self) -> u32 {
        self.ping_ms.unwrap_or(0)
    }

    fn get_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ms
    }

    fn mut_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ms
    }

    // optional uint32 packets_dropped_pct = 6;

    pub fn clear_packets_dropped_pct(&mut self) {
        self.packets_dropped_pct = ::std::option::Option::None;
    }

    pub fn has_packets_dropped_pct(&self) -> bool {
        self.packets_dropped_pct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_dropped_pct(&mut self, v: u32) {
        self.packets_dropped_pct = ::std::option::Option::Some(v);
    }

    pub fn get_packets_dropped_pct(&self) -> u32 {
        self.packets_dropped_pct.unwrap_or(0)
    }

    fn get_packets_dropped_pct_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.packets_dropped_pct
    }

    fn mut_packets_dropped_pct_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.packets_dropped_pct
    }

    // optional uint32 packets_weird_sequence_pct = 7;

    pub fn clear_packets_weird_sequence_pct(&mut self) {
        self.packets_weird_sequence_pct = ::std::option::Option::None;
    }

    pub fn has_packets_weird_sequence_pct(&self) -> bool {
        self.packets_weird_sequence_pct.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_weird_sequence_pct(&mut self, v: u32) {
        self.packets_weird_sequence_pct = ::std::option::Option::Some(v);
    }

    pub fn get_packets_weird_sequence_pct(&self) -> u32 {
        self.packets_weird_sequence_pct.unwrap_or(0)
    }

    fn get_packets_weird_sequence_pct_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.packets_weird_sequence_pct
    }

    fn mut_packets_weird_sequence_pct_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.packets_weird_sequence_pct
    }

    // optional uint32 peak_jitter_usec = 8;

    pub fn clear_peak_jitter_usec(&mut self) {
        self.peak_jitter_usec = ::std::option::Option::None;
    }

    pub fn has_peak_jitter_usec(&self) -> bool {
        self.peak_jitter_usec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peak_jitter_usec(&mut self, v: u32) {
        self.peak_jitter_usec = ::std::option::Option::Some(v);
    }

    pub fn get_peak_jitter_usec(&self) -> u32 {
        self.peak_jitter_usec.unwrap_or(0)
    }

    fn get_peak_jitter_usec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.peak_jitter_usec
    }

    fn mut_peak_jitter_usec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.peak_jitter_usec
    }
}

impl ::protobuf::Message for CMsgSteamDatagramLinkInstantaneousStats {
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
                    self.out_packets_per_sec_x10 = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.out_bytes_per_sec = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.in_packets_per_sec_x10 = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.in_bytes_per_sec = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_ms = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.packets_dropped_pct = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.packets_weird_sequence_pct = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.peak_jitter_usec = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.out_packets_per_sec_x10 {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.out_bytes_per_sec {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.in_packets_per_sec_x10 {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.in_bytes_per_sec {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_ms {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packets_dropped_pct {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packets_weird_sequence_pct {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.peak_jitter_usec {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.out_packets_per_sec_x10 {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.out_bytes_per_sec {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.in_packets_per_sec_x10 {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.in_bytes_per_sec {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.ping_ms {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.packets_dropped_pct {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.packets_weird_sequence_pct {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.peak_jitter_usec {
            os.write_uint32(8, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSteamDatagramLinkInstantaneousStats {
    fn new() -> CMsgSteamDatagramLinkInstantaneousStats {
        CMsgSteamDatagramLinkInstantaneousStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramLinkInstantaneousStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "out_packets_per_sec_x10",
                    CMsgSteamDatagramLinkInstantaneousStats::get_out_packets_per_sec_x10_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_out_packets_per_sec_x10_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "out_bytes_per_sec",
                    CMsgSteamDatagramLinkInstantaneousStats::get_out_bytes_per_sec_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_out_bytes_per_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "in_packets_per_sec_x10",
                    CMsgSteamDatagramLinkInstantaneousStats::get_in_packets_per_sec_x10_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_in_packets_per_sec_x10_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "in_bytes_per_sec",
                    CMsgSteamDatagramLinkInstantaneousStats::get_in_bytes_per_sec_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_in_bytes_per_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ms",
                    CMsgSteamDatagramLinkInstantaneousStats::get_ping_ms_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_ping_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "packets_dropped_pct",
                    CMsgSteamDatagramLinkInstantaneousStats::get_packets_dropped_pct_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_packets_dropped_pct_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "packets_weird_sequence_pct",
                    CMsgSteamDatagramLinkInstantaneousStats::get_packets_weird_sequence_pct_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_packets_weird_sequence_pct_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "peak_jitter_usec",
                    CMsgSteamDatagramLinkInstantaneousStats::get_peak_jitter_usec_for_reflect,
                    CMsgSteamDatagramLinkInstantaneousStats::mut_peak_jitter_usec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramLinkInstantaneousStats>(
                    "CMsgSteamDatagramLinkInstantaneousStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramLinkInstantaneousStats {
    fn clear(&mut self) {
        self.clear_out_packets_per_sec_x10();
        self.clear_out_bytes_per_sec();
        self.clear_in_packets_per_sec_x10();
        self.clear_in_bytes_per_sec();
        self.clear_ping_ms();
        self.clear_packets_dropped_pct();
        self.clear_packets_weird_sequence_pct();
        self.clear_peak_jitter_usec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramLinkInstantaneousStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramLinkInstantaneousStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramLinkLifetimeStats {
    // message fields
    packets_sent: ::std::option::Option<u64>,
    kb_sent: ::std::option::Option<u64>,
    packets_recv: ::std::option::Option<u64>,
    kb_recv: ::std::option::Option<u64>,
    packets_recv_sequenced: ::std::option::Option<u64>,
    packets_recv_dropped: ::std::option::Option<u64>,
    packets_recv_out_of_order: ::std::option::Option<u64>,
    packets_recv_duplicate: ::std::option::Option<u64>,
    packets_recv_lurch: ::std::option::Option<u64>,
    quality_histogram_100: ::std::option::Option<u32>,
    quality_histogram_99: ::std::option::Option<u32>,
    quality_histogram_97: ::std::option::Option<u32>,
    quality_histogram_95: ::std::option::Option<u32>,
    quality_histogram_90: ::std::option::Option<u32>,
    quality_histogram_75: ::std::option::Option<u32>,
    quality_histogram_50: ::std::option::Option<u32>,
    quality_histogram_1: ::std::option::Option<u32>,
    quality_histogram_dead: ::std::option::Option<u32>,
    quality_ntile_2nd: ::std::option::Option<u32>,
    quality_ntile_5th: ::std::option::Option<u32>,
    quality_ntile_25th: ::std::option::Option<u32>,
    quality_ntile_50th: ::std::option::Option<u32>,
    ping_histogram_25: ::std::option::Option<u32>,
    ping_histogram_50: ::std::option::Option<u32>,
    ping_histogram_75: ::std::option::Option<u32>,
    ping_histogram_100: ::std::option::Option<u32>,
    ping_histogram_125: ::std::option::Option<u32>,
    ping_histogram_150: ::std::option::Option<u32>,
    ping_histogram_200: ::std::option::Option<u32>,
    ping_histogram_300: ::std::option::Option<u32>,
    ping_histogram_max: ::std::option::Option<u32>,
    ping_ntile_5th: ::std::option::Option<u32>,
    ping_ntile_50th: ::std::option::Option<u32>,
    ping_ntile_75th: ::std::option::Option<u32>,
    ping_ntile_95th: ::std::option::Option<u32>,
    ping_ntile_98th: ::std::option::Option<u32>,
    jitter_histogram_negligible: ::std::option::Option<u32>,
    jitter_histogram_1: ::std::option::Option<u32>,
    jitter_histogram_2: ::std::option::Option<u32>,
    jitter_histogram_5: ::std::option::Option<u32>,
    jitter_histogram_10: ::std::option::Option<u32>,
    jitter_histogram_20: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramLinkLifetimeStats {}

impl CMsgSteamDatagramLinkLifetimeStats {
    pub fn new() -> CMsgSteamDatagramLinkLifetimeStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramLinkLifetimeStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramLinkLifetimeStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramLinkLifetimeStats,
        };
        unsafe {
            instance.get(CMsgSteamDatagramLinkLifetimeStats::new)
        }
    }

    // optional uint64 packets_sent = 3;

    pub fn clear_packets_sent(&mut self) {
        self.packets_sent = ::std::option::Option::None;
    }

    pub fn has_packets_sent(&self) -> bool {
        self.packets_sent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_sent(&mut self, v: u64) {
        self.packets_sent = ::std::option::Option::Some(v);
    }

    pub fn get_packets_sent(&self) -> u64 {
        self.packets_sent.unwrap_or(0)
    }

    fn get_packets_sent_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_sent
    }

    fn mut_packets_sent_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_sent
    }

    // optional uint64 kb_sent = 4;

    pub fn clear_kb_sent(&mut self) {
        self.kb_sent = ::std::option::Option::None;
    }

    pub fn has_kb_sent(&self) -> bool {
        self.kb_sent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kb_sent(&mut self, v: u64) {
        self.kb_sent = ::std::option::Option::Some(v);
    }

    pub fn get_kb_sent(&self) -> u64 {
        self.kb_sent.unwrap_or(0)
    }

    fn get_kb_sent_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.kb_sent
    }

    fn mut_kb_sent_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.kb_sent
    }

    // optional uint64 packets_recv = 5;

    pub fn clear_packets_recv(&mut self) {
        self.packets_recv = ::std::option::Option::None;
    }

    pub fn has_packets_recv(&self) -> bool {
        self.packets_recv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv(&mut self, v: u64) {
        self.packets_recv = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv(&self) -> u64 {
        self.packets_recv.unwrap_or(0)
    }

    fn get_packets_recv_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv
    }

    fn mut_packets_recv_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv
    }

    // optional uint64 kb_recv = 6;

    pub fn clear_kb_recv(&mut self) {
        self.kb_recv = ::std::option::Option::None;
    }

    pub fn has_kb_recv(&self) -> bool {
        self.kb_recv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kb_recv(&mut self, v: u64) {
        self.kb_recv = ::std::option::Option::Some(v);
    }

    pub fn get_kb_recv(&self) -> u64 {
        self.kb_recv.unwrap_or(0)
    }

    fn get_kb_recv_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.kb_recv
    }

    fn mut_kb_recv_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.kb_recv
    }

    // optional uint64 packets_recv_sequenced = 7;

    pub fn clear_packets_recv_sequenced(&mut self) {
        self.packets_recv_sequenced = ::std::option::Option::None;
    }

    pub fn has_packets_recv_sequenced(&self) -> bool {
        self.packets_recv_sequenced.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_sequenced(&mut self, v: u64) {
        self.packets_recv_sequenced = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_sequenced(&self) -> u64 {
        self.packets_recv_sequenced.unwrap_or(0)
    }

    fn get_packets_recv_sequenced_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_sequenced
    }

    fn mut_packets_recv_sequenced_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_sequenced
    }

    // optional uint64 packets_recv_dropped = 8;

    pub fn clear_packets_recv_dropped(&mut self) {
        self.packets_recv_dropped = ::std::option::Option::None;
    }

    pub fn has_packets_recv_dropped(&self) -> bool {
        self.packets_recv_dropped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_dropped(&mut self, v: u64) {
        self.packets_recv_dropped = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_dropped(&self) -> u64 {
        self.packets_recv_dropped.unwrap_or(0)
    }

    fn get_packets_recv_dropped_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_dropped
    }

    fn mut_packets_recv_dropped_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_dropped
    }

    // optional uint64 packets_recv_out_of_order = 9;

    pub fn clear_packets_recv_out_of_order(&mut self) {
        self.packets_recv_out_of_order = ::std::option::Option::None;
    }

    pub fn has_packets_recv_out_of_order(&self) -> bool {
        self.packets_recv_out_of_order.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_out_of_order(&mut self, v: u64) {
        self.packets_recv_out_of_order = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_out_of_order(&self) -> u64 {
        self.packets_recv_out_of_order.unwrap_or(0)
    }

    fn get_packets_recv_out_of_order_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_out_of_order
    }

    fn mut_packets_recv_out_of_order_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_out_of_order
    }

    // optional uint64 packets_recv_duplicate = 10;

    pub fn clear_packets_recv_duplicate(&mut self) {
        self.packets_recv_duplicate = ::std::option::Option::None;
    }

    pub fn has_packets_recv_duplicate(&self) -> bool {
        self.packets_recv_duplicate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_duplicate(&mut self, v: u64) {
        self.packets_recv_duplicate = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_duplicate(&self) -> u64 {
        self.packets_recv_duplicate.unwrap_or(0)
    }

    fn get_packets_recv_duplicate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_duplicate
    }

    fn mut_packets_recv_duplicate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_duplicate
    }

    // optional uint64 packets_recv_lurch = 11;

    pub fn clear_packets_recv_lurch(&mut self) {
        self.packets_recv_lurch = ::std::option::Option::None;
    }

    pub fn has_packets_recv_lurch(&self) -> bool {
        self.packets_recv_lurch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packets_recv_lurch(&mut self, v: u64) {
        self.packets_recv_lurch = ::std::option::Option::Some(v);
    }

    pub fn get_packets_recv_lurch(&self) -> u64 {
        self.packets_recv_lurch.unwrap_or(0)
    }

    fn get_packets_recv_lurch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.packets_recv_lurch
    }

    fn mut_packets_recv_lurch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.packets_recv_lurch
    }

    // optional uint32 quality_histogram_100 = 21;

    pub fn clear_quality_histogram_100(&mut self) {
        self.quality_histogram_100 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_100(&self) -> bool {
        self.quality_histogram_100.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_100(&mut self, v: u32) {
        self.quality_histogram_100 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_100(&self) -> u32 {
        self.quality_histogram_100.unwrap_or(0)
    }

    fn get_quality_histogram_100_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_100
    }

    fn mut_quality_histogram_100_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_100
    }

    // optional uint32 quality_histogram_99 = 22;

    pub fn clear_quality_histogram_99(&mut self) {
        self.quality_histogram_99 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_99(&self) -> bool {
        self.quality_histogram_99.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_99(&mut self, v: u32) {
        self.quality_histogram_99 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_99(&self) -> u32 {
        self.quality_histogram_99.unwrap_or(0)
    }

    fn get_quality_histogram_99_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_99
    }

    fn mut_quality_histogram_99_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_99
    }

    // optional uint32 quality_histogram_97 = 23;

    pub fn clear_quality_histogram_97(&mut self) {
        self.quality_histogram_97 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_97(&self) -> bool {
        self.quality_histogram_97.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_97(&mut self, v: u32) {
        self.quality_histogram_97 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_97(&self) -> u32 {
        self.quality_histogram_97.unwrap_or(0)
    }

    fn get_quality_histogram_97_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_97
    }

    fn mut_quality_histogram_97_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_97
    }

    // optional uint32 quality_histogram_95 = 24;

    pub fn clear_quality_histogram_95(&mut self) {
        self.quality_histogram_95 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_95(&self) -> bool {
        self.quality_histogram_95.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_95(&mut self, v: u32) {
        self.quality_histogram_95 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_95(&self) -> u32 {
        self.quality_histogram_95.unwrap_or(0)
    }

    fn get_quality_histogram_95_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_95
    }

    fn mut_quality_histogram_95_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_95
    }

    // optional uint32 quality_histogram_90 = 25;

    pub fn clear_quality_histogram_90(&mut self) {
        self.quality_histogram_90 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_90(&self) -> bool {
        self.quality_histogram_90.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_90(&mut self, v: u32) {
        self.quality_histogram_90 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_90(&self) -> u32 {
        self.quality_histogram_90.unwrap_or(0)
    }

    fn get_quality_histogram_90_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_90
    }

    fn mut_quality_histogram_90_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_90
    }

    // optional uint32 quality_histogram_75 = 26;

    pub fn clear_quality_histogram_75(&mut self) {
        self.quality_histogram_75 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_75(&self) -> bool {
        self.quality_histogram_75.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_75(&mut self, v: u32) {
        self.quality_histogram_75 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_75(&self) -> u32 {
        self.quality_histogram_75.unwrap_or(0)
    }

    fn get_quality_histogram_75_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_75
    }

    fn mut_quality_histogram_75_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_75
    }

    // optional uint32 quality_histogram_50 = 27;

    pub fn clear_quality_histogram_50(&mut self) {
        self.quality_histogram_50 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_50(&self) -> bool {
        self.quality_histogram_50.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_50(&mut self, v: u32) {
        self.quality_histogram_50 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_50(&self) -> u32 {
        self.quality_histogram_50.unwrap_or(0)
    }

    fn get_quality_histogram_50_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_50
    }

    fn mut_quality_histogram_50_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_50
    }

    // optional uint32 quality_histogram_1 = 28;

    pub fn clear_quality_histogram_1(&mut self) {
        self.quality_histogram_1 = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_1(&self) -> bool {
        self.quality_histogram_1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_1(&mut self, v: u32) {
        self.quality_histogram_1 = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_1(&self) -> u32 {
        self.quality_histogram_1.unwrap_or(0)
    }

    fn get_quality_histogram_1_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_1
    }

    fn mut_quality_histogram_1_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_1
    }

    // optional uint32 quality_histogram_dead = 29;

    pub fn clear_quality_histogram_dead(&mut self) {
        self.quality_histogram_dead = ::std::option::Option::None;
    }

    pub fn has_quality_histogram_dead(&self) -> bool {
        self.quality_histogram_dead.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_histogram_dead(&mut self, v: u32) {
        self.quality_histogram_dead = ::std::option::Option::Some(v);
    }

    pub fn get_quality_histogram_dead(&self) -> u32 {
        self.quality_histogram_dead.unwrap_or(0)
    }

    fn get_quality_histogram_dead_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_histogram_dead
    }

    fn mut_quality_histogram_dead_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_histogram_dead
    }

    // optional uint32 quality_ntile_2nd = 30;

    pub fn clear_quality_ntile_2nd(&mut self) {
        self.quality_ntile_2nd = ::std::option::Option::None;
    }

    pub fn has_quality_ntile_2nd(&self) -> bool {
        self.quality_ntile_2nd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_ntile_2nd(&mut self, v: u32) {
        self.quality_ntile_2nd = ::std::option::Option::Some(v);
    }

    pub fn get_quality_ntile_2nd(&self) -> u32 {
        self.quality_ntile_2nd.unwrap_or(0)
    }

    fn get_quality_ntile_2nd_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_ntile_2nd
    }

    fn mut_quality_ntile_2nd_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_ntile_2nd
    }

    // optional uint32 quality_ntile_5th = 31;

    pub fn clear_quality_ntile_5th(&mut self) {
        self.quality_ntile_5th = ::std::option::Option::None;
    }

    pub fn has_quality_ntile_5th(&self) -> bool {
        self.quality_ntile_5th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_ntile_5th(&mut self, v: u32) {
        self.quality_ntile_5th = ::std::option::Option::Some(v);
    }

    pub fn get_quality_ntile_5th(&self) -> u32 {
        self.quality_ntile_5th.unwrap_or(0)
    }

    fn get_quality_ntile_5th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_ntile_5th
    }

    fn mut_quality_ntile_5th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_ntile_5th
    }

    // optional uint32 quality_ntile_25th = 32;

    pub fn clear_quality_ntile_25th(&mut self) {
        self.quality_ntile_25th = ::std::option::Option::None;
    }

    pub fn has_quality_ntile_25th(&self) -> bool {
        self.quality_ntile_25th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_ntile_25th(&mut self, v: u32) {
        self.quality_ntile_25th = ::std::option::Option::Some(v);
    }

    pub fn get_quality_ntile_25th(&self) -> u32 {
        self.quality_ntile_25th.unwrap_or(0)
    }

    fn get_quality_ntile_25th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_ntile_25th
    }

    fn mut_quality_ntile_25th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_ntile_25th
    }

    // optional uint32 quality_ntile_50th = 33;

    pub fn clear_quality_ntile_50th(&mut self) {
        self.quality_ntile_50th = ::std::option::Option::None;
    }

    pub fn has_quality_ntile_50th(&self) -> bool {
        self.quality_ntile_50th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality_ntile_50th(&mut self, v: u32) {
        self.quality_ntile_50th = ::std::option::Option::Some(v);
    }

    pub fn get_quality_ntile_50th(&self) -> u32 {
        self.quality_ntile_50th.unwrap_or(0)
    }

    fn get_quality_ntile_50th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quality_ntile_50th
    }

    fn mut_quality_ntile_50th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quality_ntile_50th
    }

    // optional uint32 ping_histogram_25 = 41;

    pub fn clear_ping_histogram_25(&mut self) {
        self.ping_histogram_25 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_25(&self) -> bool {
        self.ping_histogram_25.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_25(&mut self, v: u32) {
        self.ping_histogram_25 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_25(&self) -> u32 {
        self.ping_histogram_25.unwrap_or(0)
    }

    fn get_ping_histogram_25_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_25
    }

    fn mut_ping_histogram_25_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_25
    }

    // optional uint32 ping_histogram_50 = 42;

    pub fn clear_ping_histogram_50(&mut self) {
        self.ping_histogram_50 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_50(&self) -> bool {
        self.ping_histogram_50.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_50(&mut self, v: u32) {
        self.ping_histogram_50 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_50(&self) -> u32 {
        self.ping_histogram_50.unwrap_or(0)
    }

    fn get_ping_histogram_50_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_50
    }

    fn mut_ping_histogram_50_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_50
    }

    // optional uint32 ping_histogram_75 = 43;

    pub fn clear_ping_histogram_75(&mut self) {
        self.ping_histogram_75 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_75(&self) -> bool {
        self.ping_histogram_75.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_75(&mut self, v: u32) {
        self.ping_histogram_75 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_75(&self) -> u32 {
        self.ping_histogram_75.unwrap_or(0)
    }

    fn get_ping_histogram_75_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_75
    }

    fn mut_ping_histogram_75_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_75
    }

    // optional uint32 ping_histogram_100 = 44;

    pub fn clear_ping_histogram_100(&mut self) {
        self.ping_histogram_100 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_100(&self) -> bool {
        self.ping_histogram_100.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_100(&mut self, v: u32) {
        self.ping_histogram_100 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_100(&self) -> u32 {
        self.ping_histogram_100.unwrap_or(0)
    }

    fn get_ping_histogram_100_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_100
    }

    fn mut_ping_histogram_100_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_100
    }

    // optional uint32 ping_histogram_125 = 45;

    pub fn clear_ping_histogram_125(&mut self) {
        self.ping_histogram_125 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_125(&self) -> bool {
        self.ping_histogram_125.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_125(&mut self, v: u32) {
        self.ping_histogram_125 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_125(&self) -> u32 {
        self.ping_histogram_125.unwrap_or(0)
    }

    fn get_ping_histogram_125_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_125
    }

    fn mut_ping_histogram_125_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_125
    }

    // optional uint32 ping_histogram_150 = 46;

    pub fn clear_ping_histogram_150(&mut self) {
        self.ping_histogram_150 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_150(&self) -> bool {
        self.ping_histogram_150.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_150(&mut self, v: u32) {
        self.ping_histogram_150 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_150(&self) -> u32 {
        self.ping_histogram_150.unwrap_or(0)
    }

    fn get_ping_histogram_150_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_150
    }

    fn mut_ping_histogram_150_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_150
    }

    // optional uint32 ping_histogram_200 = 47;

    pub fn clear_ping_histogram_200(&mut self) {
        self.ping_histogram_200 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_200(&self) -> bool {
        self.ping_histogram_200.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_200(&mut self, v: u32) {
        self.ping_histogram_200 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_200(&self) -> u32 {
        self.ping_histogram_200.unwrap_or(0)
    }

    fn get_ping_histogram_200_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_200
    }

    fn mut_ping_histogram_200_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_200
    }

    // optional uint32 ping_histogram_300 = 48;

    pub fn clear_ping_histogram_300(&mut self) {
        self.ping_histogram_300 = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_300(&self) -> bool {
        self.ping_histogram_300.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_300(&mut self, v: u32) {
        self.ping_histogram_300 = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_300(&self) -> u32 {
        self.ping_histogram_300.unwrap_or(0)
    }

    fn get_ping_histogram_300_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_300
    }

    fn mut_ping_histogram_300_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_300
    }

    // optional uint32 ping_histogram_max = 49;

    pub fn clear_ping_histogram_max(&mut self) {
        self.ping_histogram_max = ::std::option::Option::None;
    }

    pub fn has_ping_histogram_max(&self) -> bool {
        self.ping_histogram_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_histogram_max(&mut self, v: u32) {
        self.ping_histogram_max = ::std::option::Option::Some(v);
    }

    pub fn get_ping_histogram_max(&self) -> u32 {
        self.ping_histogram_max.unwrap_or(0)
    }

    fn get_ping_histogram_max_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_histogram_max
    }

    fn mut_ping_histogram_max_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_histogram_max
    }

    // optional uint32 ping_ntile_5th = 50;

    pub fn clear_ping_ntile_5th(&mut self) {
        self.ping_ntile_5th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_5th(&self) -> bool {
        self.ping_ntile_5th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_5th(&mut self, v: u32) {
        self.ping_ntile_5th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_5th(&self) -> u32 {
        self.ping_ntile_5th.unwrap_or(0)
    }

    fn get_ping_ntile_5th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_5th
    }

    fn mut_ping_ntile_5th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_5th
    }

    // optional uint32 ping_ntile_50th = 51;

    pub fn clear_ping_ntile_50th(&mut self) {
        self.ping_ntile_50th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_50th(&self) -> bool {
        self.ping_ntile_50th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_50th(&mut self, v: u32) {
        self.ping_ntile_50th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_50th(&self) -> u32 {
        self.ping_ntile_50th.unwrap_or(0)
    }

    fn get_ping_ntile_50th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_50th
    }

    fn mut_ping_ntile_50th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_50th
    }

    // optional uint32 ping_ntile_75th = 52;

    pub fn clear_ping_ntile_75th(&mut self) {
        self.ping_ntile_75th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_75th(&self) -> bool {
        self.ping_ntile_75th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_75th(&mut self, v: u32) {
        self.ping_ntile_75th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_75th(&self) -> u32 {
        self.ping_ntile_75th.unwrap_or(0)
    }

    fn get_ping_ntile_75th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_75th
    }

    fn mut_ping_ntile_75th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_75th
    }

    // optional uint32 ping_ntile_95th = 53;

    pub fn clear_ping_ntile_95th(&mut self) {
        self.ping_ntile_95th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_95th(&self) -> bool {
        self.ping_ntile_95th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_95th(&mut self, v: u32) {
        self.ping_ntile_95th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_95th(&self) -> u32 {
        self.ping_ntile_95th.unwrap_or(0)
    }

    fn get_ping_ntile_95th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_95th
    }

    fn mut_ping_ntile_95th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_95th
    }

    // optional uint32 ping_ntile_98th = 54;

    pub fn clear_ping_ntile_98th(&mut self) {
        self.ping_ntile_98th = ::std::option::Option::None;
    }

    pub fn has_ping_ntile_98th(&self) -> bool {
        self.ping_ntile_98th.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_ntile_98th(&mut self, v: u32) {
        self.ping_ntile_98th = ::std::option::Option::Some(v);
    }

    pub fn get_ping_ntile_98th(&self) -> u32 {
        self.ping_ntile_98th.unwrap_or(0)
    }

    fn get_ping_ntile_98th_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping_ntile_98th
    }

    fn mut_ping_ntile_98th_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping_ntile_98th
    }

    // optional uint32 jitter_histogram_negligible = 61;

    pub fn clear_jitter_histogram_negligible(&mut self) {
        self.jitter_histogram_negligible = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_negligible(&self) -> bool {
        self.jitter_histogram_negligible.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_negligible(&mut self, v: u32) {
        self.jitter_histogram_negligible = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_negligible(&self) -> u32 {
        self.jitter_histogram_negligible.unwrap_or(0)
    }

    fn get_jitter_histogram_negligible_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_negligible
    }

    fn mut_jitter_histogram_negligible_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_negligible
    }

    // optional uint32 jitter_histogram_1 = 62;

    pub fn clear_jitter_histogram_1(&mut self) {
        self.jitter_histogram_1 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_1(&self) -> bool {
        self.jitter_histogram_1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_1(&mut self, v: u32) {
        self.jitter_histogram_1 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_1(&self) -> u32 {
        self.jitter_histogram_1.unwrap_or(0)
    }

    fn get_jitter_histogram_1_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_1
    }

    fn mut_jitter_histogram_1_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_1
    }

    // optional uint32 jitter_histogram_2 = 63;

    pub fn clear_jitter_histogram_2(&mut self) {
        self.jitter_histogram_2 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_2(&self) -> bool {
        self.jitter_histogram_2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_2(&mut self, v: u32) {
        self.jitter_histogram_2 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_2(&self) -> u32 {
        self.jitter_histogram_2.unwrap_or(0)
    }

    fn get_jitter_histogram_2_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_2
    }

    fn mut_jitter_histogram_2_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_2
    }

    // optional uint32 jitter_histogram_5 = 64;

    pub fn clear_jitter_histogram_5(&mut self) {
        self.jitter_histogram_5 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_5(&self) -> bool {
        self.jitter_histogram_5.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_5(&mut self, v: u32) {
        self.jitter_histogram_5 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_5(&self) -> u32 {
        self.jitter_histogram_5.unwrap_or(0)
    }

    fn get_jitter_histogram_5_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_5
    }

    fn mut_jitter_histogram_5_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_5
    }

    // optional uint32 jitter_histogram_10 = 65;

    pub fn clear_jitter_histogram_10(&mut self) {
        self.jitter_histogram_10 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_10(&self) -> bool {
        self.jitter_histogram_10.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_10(&mut self, v: u32) {
        self.jitter_histogram_10 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_10(&self) -> u32 {
        self.jitter_histogram_10.unwrap_or(0)
    }

    fn get_jitter_histogram_10_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_10
    }

    fn mut_jitter_histogram_10_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_10
    }

    // optional uint32 jitter_histogram_20 = 66;

    pub fn clear_jitter_histogram_20(&mut self) {
        self.jitter_histogram_20 = ::std::option::Option::None;
    }

    pub fn has_jitter_histogram_20(&self) -> bool {
        self.jitter_histogram_20.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jitter_histogram_20(&mut self, v: u32) {
        self.jitter_histogram_20 = ::std::option::Option::Some(v);
    }

    pub fn get_jitter_histogram_20(&self) -> u32 {
        self.jitter_histogram_20.unwrap_or(0)
    }

    fn get_jitter_histogram_20_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.jitter_histogram_20
    }

    fn mut_jitter_histogram_20_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.jitter_histogram_20
    }
}

impl ::protobuf::Message for CMsgSteamDatagramLinkLifetimeStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.packets_sent = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.kb_sent = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.packets_recv = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.kb_recv = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.packets_recv_sequenced = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.packets_recv_dropped = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.packets_recv_out_of_order = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.packets_recv_duplicate = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.packets_recv_lurch = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_100 = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_99 = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_97 = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_95 = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_90 = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_75 = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_50 = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_1 = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_histogram_dead = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_ntile_2nd = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_ntile_5th = ::std::option::Option::Some(tmp);
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_ntile_25th = ::std::option::Option::Some(tmp);
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.quality_ntile_50th = ::std::option::Option::Some(tmp);
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_25 = ::std::option::Option::Some(tmp);
                },
                42 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_50 = ::std::option::Option::Some(tmp);
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_75 = ::std::option::Option::Some(tmp);
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_100 = ::std::option::Option::Some(tmp);
                },
                45 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_125 = ::std::option::Option::Some(tmp);
                },
                46 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_150 = ::std::option::Option::Some(tmp);
                },
                47 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_200 = ::std::option::Option::Some(tmp);
                },
                48 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_300 = ::std::option::Option::Some(tmp);
                },
                49 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_histogram_max = ::std::option::Option::Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_5th = ::std::option::Option::Some(tmp);
                },
                51 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_50th = ::std::option::Option::Some(tmp);
                },
                52 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_75th = ::std::option::Option::Some(tmp);
                },
                53 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_95th = ::std::option::Option::Some(tmp);
                },
                54 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping_ntile_98th = ::std::option::Option::Some(tmp);
                },
                61 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_negligible = ::std::option::Option::Some(tmp);
                },
                62 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_1 = ::std::option::Option::Some(tmp);
                },
                63 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_2 = ::std::option::Option::Some(tmp);
                },
                64 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_5 = ::std::option::Option::Some(tmp);
                },
                65 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_10 = ::std::option::Option::Some(tmp);
                },
                66 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.jitter_histogram_20 = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.packets_sent {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.kb_sent {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packets_recv {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.kb_recv {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packets_recv_sequenced {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packets_recv_dropped {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packets_recv_out_of_order {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packets_recv_duplicate {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.packets_recv_lurch {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_100 {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_99 {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_97 {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_95 {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_90 {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_75 {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_50 {
            my_size += ::protobuf::rt::value_size(27, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_1 {
            my_size += ::protobuf::rt::value_size(28, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_histogram_dead {
            my_size += ::protobuf::rt::value_size(29, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_ntile_2nd {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_ntile_5th {
            my_size += ::protobuf::rt::value_size(31, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_ntile_25th {
            my_size += ::protobuf::rt::value_size(32, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.quality_ntile_50th {
            my_size += ::protobuf::rt::value_size(33, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_25 {
            my_size += ::protobuf::rt::value_size(41, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_50 {
            my_size += ::protobuf::rt::value_size(42, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_75 {
            my_size += ::protobuf::rt::value_size(43, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_100 {
            my_size += ::protobuf::rt::value_size(44, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_125 {
            my_size += ::protobuf::rt::value_size(45, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_150 {
            my_size += ::protobuf::rt::value_size(46, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_200 {
            my_size += ::protobuf::rt::value_size(47, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_300 {
            my_size += ::protobuf::rt::value_size(48, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_histogram_max {
            my_size += ::protobuf::rt::value_size(49, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_ntile_5th {
            my_size += ::protobuf::rt::value_size(50, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_ntile_50th {
            my_size += ::protobuf::rt::value_size(51, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_ntile_75th {
            my_size += ::protobuf::rt::value_size(52, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_ntile_95th {
            my_size += ::protobuf::rt::value_size(53, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping_ntile_98th {
            my_size += ::protobuf::rt::value_size(54, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.jitter_histogram_negligible {
            my_size += ::protobuf::rt::value_size(61, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.jitter_histogram_1 {
            my_size += ::protobuf::rt::value_size(62, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.jitter_histogram_2 {
            my_size += ::protobuf::rt::value_size(63, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.jitter_histogram_5 {
            my_size += ::protobuf::rt::value_size(64, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.jitter_histogram_10 {
            my_size += ::protobuf::rt::value_size(65, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.jitter_histogram_20 {
            my_size += ::protobuf::rt::value_size(66, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.packets_sent {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.kb_sent {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.packets_recv {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.kb_recv {
            os.write_uint64(6, v)?;
        };
        if let Some(v) = self.packets_recv_sequenced {
            os.write_uint64(7, v)?;
        };
        if let Some(v) = self.packets_recv_dropped {
            os.write_uint64(8, v)?;
        };
        if let Some(v) = self.packets_recv_out_of_order {
            os.write_uint64(9, v)?;
        };
        if let Some(v) = self.packets_recv_duplicate {
            os.write_uint64(10, v)?;
        };
        if let Some(v) = self.packets_recv_lurch {
            os.write_uint64(11, v)?;
        };
        if let Some(v) = self.quality_histogram_100 {
            os.write_uint32(21, v)?;
        };
        if let Some(v) = self.quality_histogram_99 {
            os.write_uint32(22, v)?;
        };
        if let Some(v) = self.quality_histogram_97 {
            os.write_uint32(23, v)?;
        };
        if let Some(v) = self.quality_histogram_95 {
            os.write_uint32(24, v)?;
        };
        if let Some(v) = self.quality_histogram_90 {
            os.write_uint32(25, v)?;
        };
        if let Some(v) = self.quality_histogram_75 {
            os.write_uint32(26, v)?;
        };
        if let Some(v) = self.quality_histogram_50 {
            os.write_uint32(27, v)?;
        };
        if let Some(v) = self.quality_histogram_1 {
            os.write_uint32(28, v)?;
        };
        if let Some(v) = self.quality_histogram_dead {
            os.write_uint32(29, v)?;
        };
        if let Some(v) = self.quality_ntile_2nd {
            os.write_uint32(30, v)?;
        };
        if let Some(v) = self.quality_ntile_5th {
            os.write_uint32(31, v)?;
        };
        if let Some(v) = self.quality_ntile_25th {
            os.write_uint32(32, v)?;
        };
        if let Some(v) = self.quality_ntile_50th {
            os.write_uint32(33, v)?;
        };
        if let Some(v) = self.ping_histogram_25 {
            os.write_uint32(41, v)?;
        };
        if let Some(v) = self.ping_histogram_50 {
            os.write_uint32(42, v)?;
        };
        if let Some(v) = self.ping_histogram_75 {
            os.write_uint32(43, v)?;
        };
        if let Some(v) = self.ping_histogram_100 {
            os.write_uint32(44, v)?;
        };
        if let Some(v) = self.ping_histogram_125 {
            os.write_uint32(45, v)?;
        };
        if let Some(v) = self.ping_histogram_150 {
            os.write_uint32(46, v)?;
        };
        if let Some(v) = self.ping_histogram_200 {
            os.write_uint32(47, v)?;
        };
        if let Some(v) = self.ping_histogram_300 {
            os.write_uint32(48, v)?;
        };
        if let Some(v) = self.ping_histogram_max {
            os.write_uint32(49, v)?;
        };
        if let Some(v) = self.ping_ntile_5th {
            os.write_uint32(50, v)?;
        };
        if let Some(v) = self.ping_ntile_50th {
            os.write_uint32(51, v)?;
        };
        if let Some(v) = self.ping_ntile_75th {
            os.write_uint32(52, v)?;
        };
        if let Some(v) = self.ping_ntile_95th {
            os.write_uint32(53, v)?;
        };
        if let Some(v) = self.ping_ntile_98th {
            os.write_uint32(54, v)?;
        };
        if let Some(v) = self.jitter_histogram_negligible {
            os.write_uint32(61, v)?;
        };
        if let Some(v) = self.jitter_histogram_1 {
            os.write_uint32(62, v)?;
        };
        if let Some(v) = self.jitter_histogram_2 {
            os.write_uint32(63, v)?;
        };
        if let Some(v) = self.jitter_histogram_5 {
            os.write_uint32(64, v)?;
        };
        if let Some(v) = self.jitter_histogram_10 {
            os.write_uint32(65, v)?;
        };
        if let Some(v) = self.jitter_histogram_20 {
            os.write_uint32(66, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSteamDatagramLinkLifetimeStats {
    fn new() -> CMsgSteamDatagramLinkLifetimeStats {
        CMsgSteamDatagramLinkLifetimeStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramLinkLifetimeStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_sent",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_sent_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "kb_sent",
                    CMsgSteamDatagramLinkLifetimeStats::get_kb_sent_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_kb_sent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "kb_recv",
                    CMsgSteamDatagramLinkLifetimeStats::get_kb_recv_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_kb_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_sequenced",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_sequenced_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_sequenced_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_dropped",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_dropped_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_dropped_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_out_of_order",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_out_of_order_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_out_of_order_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_duplicate",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_duplicate_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_duplicate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "packets_recv_lurch",
                    CMsgSteamDatagramLinkLifetimeStats::get_packets_recv_lurch_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_packets_recv_lurch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_100",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_100_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_100_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_99",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_99_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_99_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_97",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_97_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_97_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_95",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_95_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_95_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_90",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_90_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_90_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_75",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_75_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_75_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_50",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_50_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_50_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_1",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_1_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_histogram_dead",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_histogram_dead_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_histogram_dead_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_ntile_2nd",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_ntile_2nd_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_ntile_2nd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_ntile_5th",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_ntile_5th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_ntile_5th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_ntile_25th",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_ntile_25th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_ntile_25th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quality_ntile_50th",
                    CMsgSteamDatagramLinkLifetimeStats::get_quality_ntile_50th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_quality_ntile_50th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_25",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_25_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_25_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_50",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_50_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_50_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_75",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_75_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_75_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_100",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_100_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_100_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_125",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_125_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_125_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_150",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_150_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_150_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_200",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_200_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_200_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_300",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_300_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_300_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_histogram_max",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_histogram_max_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_histogram_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_5th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_5th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_5th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_50th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_50th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_50th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_75th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_75th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_75th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_95th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_95th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_95th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping_ntile_98th",
                    CMsgSteamDatagramLinkLifetimeStats::get_ping_ntile_98th_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_ping_ntile_98th_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_negligible",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_negligible_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_negligible_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_1",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_1_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_2",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_2_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_5",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_5_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_5_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_10",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_10_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_10_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "jitter_histogram_20",
                    CMsgSteamDatagramLinkLifetimeStats::get_jitter_histogram_20_for_reflect,
                    CMsgSteamDatagramLinkLifetimeStats::mut_jitter_histogram_20_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramLinkLifetimeStats>(
                    "CMsgSteamDatagramLinkLifetimeStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramLinkLifetimeStats {
    fn clear(&mut self) {
        self.clear_packets_sent();
        self.clear_kb_sent();
        self.clear_packets_recv();
        self.clear_kb_recv();
        self.clear_packets_recv_sequenced();
        self.clear_packets_recv_dropped();
        self.clear_packets_recv_out_of_order();
        self.clear_packets_recv_duplicate();
        self.clear_packets_recv_lurch();
        self.clear_quality_histogram_100();
        self.clear_quality_histogram_99();
        self.clear_quality_histogram_97();
        self.clear_quality_histogram_95();
        self.clear_quality_histogram_90();
        self.clear_quality_histogram_75();
        self.clear_quality_histogram_50();
        self.clear_quality_histogram_1();
        self.clear_quality_histogram_dead();
        self.clear_quality_ntile_2nd();
        self.clear_quality_ntile_5th();
        self.clear_quality_ntile_25th();
        self.clear_quality_ntile_50th();
        self.clear_ping_histogram_25();
        self.clear_ping_histogram_50();
        self.clear_ping_histogram_75();
        self.clear_ping_histogram_100();
        self.clear_ping_histogram_125();
        self.clear_ping_histogram_150();
        self.clear_ping_histogram_200();
        self.clear_ping_histogram_300();
        self.clear_ping_histogram_max();
        self.clear_ping_ntile_5th();
        self.clear_ping_ntile_50th();
        self.clear_ping_ntile_75th();
        self.clear_ping_ntile_95th();
        self.clear_ping_ntile_98th();
        self.clear_jitter_histogram_negligible();
        self.clear_jitter_histogram_1();
        self.clear_jitter_histogram_2();
        self.clear_jitter_histogram_5();
        self.clear_jitter_histogram_10();
        self.clear_jitter_histogram_20();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramLinkLifetimeStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramLinkLifetimeStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionQuality {
    // message fields
    instantaneous: ::protobuf::SingularPtrField<CMsgSteamDatagramLinkInstantaneousStats>,
    lifetime: ::protobuf::SingularPtrField<CMsgSteamDatagramLinkLifetimeStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionQuality {}

impl CMsgSteamDatagramConnectionQuality {
    pub fn new() -> CMsgSteamDatagramConnectionQuality {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionQuality {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionQuality> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionQuality,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionQuality::new)
        }
    }

    // optional .dota.CMsgSteamDatagramLinkInstantaneousStats instantaneous = 1;

    pub fn clear_instantaneous(&mut self) {
        self.instantaneous.clear();
    }

    pub fn has_instantaneous(&self) -> bool {
        self.instantaneous.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instantaneous(&mut self, v: CMsgSteamDatagramLinkInstantaneousStats) {
        self.instantaneous = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_instantaneous(&mut self) -> &mut CMsgSteamDatagramLinkInstantaneousStats {
        if self.instantaneous.is_none() {
            self.instantaneous.set_default();
        };
        self.instantaneous.as_mut().unwrap()
    }

    // Take field
    pub fn take_instantaneous(&mut self) -> CMsgSteamDatagramLinkInstantaneousStats {
        self.instantaneous.take().unwrap_or_else(|| CMsgSteamDatagramLinkInstantaneousStats::new())
    }

    pub fn get_instantaneous(&self) -> &CMsgSteamDatagramLinkInstantaneousStats {
        self.instantaneous.as_ref().unwrap_or_else(|| CMsgSteamDatagramLinkInstantaneousStats::default_instance())
    }

    fn get_instantaneous_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramLinkInstantaneousStats> {
        &self.instantaneous
    }

    fn mut_instantaneous_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramLinkInstantaneousStats> {
        &mut self.instantaneous
    }

    // optional .dota.CMsgSteamDatagramLinkLifetimeStats lifetime = 2;

    pub fn clear_lifetime(&mut self) {
        self.lifetime.clear();
    }

    pub fn has_lifetime(&self) -> bool {
        self.lifetime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lifetime(&mut self, v: CMsgSteamDatagramLinkLifetimeStats) {
        self.lifetime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lifetime(&mut self) -> &mut CMsgSteamDatagramLinkLifetimeStats {
        if self.lifetime.is_none() {
            self.lifetime.set_default();
        };
        self.lifetime.as_mut().unwrap()
    }

    // Take field
    pub fn take_lifetime(&mut self) -> CMsgSteamDatagramLinkLifetimeStats {
        self.lifetime.take().unwrap_or_else(|| CMsgSteamDatagramLinkLifetimeStats::new())
    }

    pub fn get_lifetime(&self) -> &CMsgSteamDatagramLinkLifetimeStats {
        self.lifetime.as_ref().unwrap_or_else(|| CMsgSteamDatagramLinkLifetimeStats::default_instance())
    }

    fn get_lifetime_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramLinkLifetimeStats> {
        &self.lifetime
    }

    fn mut_lifetime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramLinkLifetimeStats> {
        &mut self.lifetime
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionQuality {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.instantaneous)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lifetime)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.instantaneous.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.lifetime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.instantaneous.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.lifetime.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionQuality {
    fn new() -> CMsgSteamDatagramConnectionQuality {
        CMsgSteamDatagramConnectionQuality::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionQuality>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramLinkInstantaneousStats>>(
                    "instantaneous",
                    CMsgSteamDatagramConnectionQuality::get_instantaneous_for_reflect,
                    CMsgSteamDatagramConnectionQuality::mut_instantaneous_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramLinkLifetimeStats>>(
                    "lifetime",
                    CMsgSteamDatagramConnectionQuality::get_lifetime_for_reflect,
                    CMsgSteamDatagramConnectionQuality::mut_lifetime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionQuality>(
                    "CMsgSteamDatagramConnectionQuality",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionQuality {
    fn clear(&mut self) {
        self.clear_instantaneous();
        self.clear_lifetime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionQuality {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionStatsClientToRouter {
    // message fields
    c2r: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    c2s: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    client_timestamp: ::std::option::Option<u32>,
    client_cookie: ::std::option::Option<u32>,
    seq_num_c2r: ::std::option::Option<u32>,
    seq_num_c2s: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionStatsClientToRouter {}

impl CMsgSteamDatagramConnectionStatsClientToRouter {
    pub fn new() -> CMsgSteamDatagramConnectionStatsClientToRouter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionStatsClientToRouter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionStatsClientToRouter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionStatsClientToRouter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionStatsClientToRouter::new)
        }
    }

    // optional .dota.CMsgSteamDatagramConnectionQuality c2r = 1;

    pub fn clear_c2r(&mut self) {
        self.c2r.clear();
    }

    pub fn has_c2r(&self) -> bool {
        self.c2r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c2r(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.c2r = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c2r(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.c2r.is_none() {
            self.c2r.set_default();
        };
        self.c2r.as_mut().unwrap()
    }

    // Take field
    pub fn take_c2r(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.c2r.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_c2r(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.c2r.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_c2r_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.c2r
    }

    fn mut_c2r_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.c2r
    }

    // optional .dota.CMsgSteamDatagramConnectionQuality c2s = 2;

    pub fn clear_c2s(&mut self) {
        self.c2s.clear();
    }

    pub fn has_c2s(&self) -> bool {
        self.c2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c2s(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.c2s = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c2s(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.c2s.is_none() {
            self.c2s.set_default();
        };
        self.c2s.as_mut().unwrap()
    }

    // Take field
    pub fn take_c2s(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.c2s.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_c2s(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.c2s.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_c2s_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.c2s
    }

    fn mut_c2s_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.c2s
    }

    // optional fixed32 client_timestamp = 3;

    pub fn clear_client_timestamp(&mut self) {
        self.client_timestamp = ::std::option::Option::None;
    }

    pub fn has_client_timestamp(&self) -> bool {
        self.client_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_timestamp(&mut self, v: u32) {
        self.client_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_client_timestamp(&self) -> u32 {
        self.client_timestamp.unwrap_or(0)
    }

    fn get_client_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_timestamp
    }

    fn mut_client_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_timestamp
    }

    // optional fixed32 client_cookie = 8;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // optional uint32 seq_num_c2r = 9;

    pub fn clear_seq_num_c2r(&mut self) {
        self.seq_num_c2r = ::std::option::Option::None;
    }

    pub fn has_seq_num_c2r(&self) -> bool {
        self.seq_num_c2r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_c2r(&mut self, v: u32) {
        self.seq_num_c2r = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_c2r(&self) -> u32 {
        self.seq_num_c2r.unwrap_or(0)
    }

    fn get_seq_num_c2r_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_c2r
    }

    fn mut_seq_num_c2r_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_c2r
    }

    // optional uint32 seq_num_c2s = 10;

    pub fn clear_seq_num_c2s(&mut self) {
        self.seq_num_c2s = ::std::option::Option::None;
    }

    pub fn has_seq_num_c2s(&self) -> bool {
        self.seq_num_c2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_c2s(&mut self, v: u32) {
        self.seq_num_c2s = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_c2s(&self) -> u32 {
        self.seq_num_c2s.unwrap_or(0)
    }

    fn get_seq_num_c2s_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_c2s
    }

    fn mut_seq_num_c2s_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_c2s
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.c2r)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.c2s)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_timestamp = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num_c2r = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num_c2s = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.c2r.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.c2s.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.client_timestamp {
            my_size += 5;
        };
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        if let Some(v) = self.seq_num_c2r {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seq_num_c2s {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.c2r.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.c2s.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.client_timestamp {
            os.write_fixed32(3, v)?;
        };
        if let Some(v) = self.client_cookie {
            os.write_fixed32(8, v)?;
        };
        if let Some(v) = self.seq_num_c2r {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.seq_num_c2s {
            os.write_uint32(10, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn new() -> CMsgSteamDatagramConnectionStatsClientToRouter {
        CMsgSteamDatagramConnectionStatsClientToRouter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsClientToRouter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "c2r",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_c2r_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_c2r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "c2s",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_c2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_c2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_timestamp",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_client_timestamp_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_client_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_client_cookie_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_c2r",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_seq_num_c2r_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_seq_num_c2r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_c2s",
                    CMsgSteamDatagramConnectionStatsClientToRouter::get_seq_num_c2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsClientToRouter::mut_seq_num_c2s_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionStatsClientToRouter>(
                    "CMsgSteamDatagramConnectionStatsClientToRouter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn clear(&mut self) {
        self.clear_c2r();
        self.clear_c2s();
        self.clear_client_timestamp();
        self.clear_client_cookie();
        self.clear_seq_num_c2r();
        self.clear_seq_num_c2s();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsClientToRouter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionStatsRouterToClient {
    // message fields
    r2c: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    s2c: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    client_timestamp_from_router: ::std::option::Option<u32>,
    client_timestamp_from_server: ::std::option::Option<u32>,
    router_gameserver_latency: ::std::option::Option<u32>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    migrate_request_ip: ::std::option::Option<u32>,
    migrate_request_port: ::std::option::Option<u32>,
    scoring_penalty_relay_cluster: ::std::option::Option<u32>,
    client_cookie: ::std::option::Option<u32>,
    seq_num_r2c: ::std::option::Option<u32>,
    seq_num_s2c: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionStatsRouterToClient {}

impl CMsgSteamDatagramConnectionStatsRouterToClient {
    pub fn new() -> CMsgSteamDatagramConnectionStatsRouterToClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionStatsRouterToClient {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionStatsRouterToClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionStatsRouterToClient,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionStatsRouterToClient::new)
        }
    }

    // optional .dota.CMsgSteamDatagramConnectionQuality r2c = 1;

    pub fn clear_r2c(&mut self) {
        self.r2c.clear();
    }

    pub fn has_r2c(&self) -> bool {
        self.r2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r2c(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.r2c = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r2c(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.r2c.is_none() {
            self.r2c.set_default();
        };
        self.r2c.as_mut().unwrap()
    }

    // Take field
    pub fn take_r2c(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.r2c.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_r2c(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.r2c.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_r2c_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.r2c
    }

    fn mut_r2c_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.r2c
    }

    // optional .dota.CMsgSteamDatagramConnectionQuality s2c = 2;

    pub fn clear_s2c(&mut self) {
        self.s2c.clear();
    }

    pub fn has_s2c(&self) -> bool {
        self.s2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2c(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.s2c = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s2c(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.s2c.is_none() {
            self.s2c.set_default();
        };
        self.s2c.as_mut().unwrap()
    }

    // Take field
    pub fn take_s2c(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.s2c.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_s2c(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.s2c.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_s2c_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.s2c
    }

    fn mut_s2c_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.s2c
    }

    // optional fixed32 client_timestamp_from_router = 3;

    pub fn clear_client_timestamp_from_router(&mut self) {
        self.client_timestamp_from_router = ::std::option::Option::None;
    }

    pub fn has_client_timestamp_from_router(&self) -> bool {
        self.client_timestamp_from_router.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_timestamp_from_router(&mut self, v: u32) {
        self.client_timestamp_from_router = ::std::option::Option::Some(v);
    }

    pub fn get_client_timestamp_from_router(&self) -> u32 {
        self.client_timestamp_from_router.unwrap_or(0)
    }

    fn get_client_timestamp_from_router_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_timestamp_from_router
    }

    fn mut_client_timestamp_from_router_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_timestamp_from_router
    }

    // optional fixed32 client_timestamp_from_server = 4;

    pub fn clear_client_timestamp_from_server(&mut self) {
        self.client_timestamp_from_server = ::std::option::Option::None;
    }

    pub fn has_client_timestamp_from_server(&self) -> bool {
        self.client_timestamp_from_server.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_timestamp_from_server(&mut self, v: u32) {
        self.client_timestamp_from_server = ::std::option::Option::Some(v);
    }

    pub fn get_client_timestamp_from_server(&self) -> u32 {
        self.client_timestamp_from_server.unwrap_or(0)
    }

    fn get_client_timestamp_from_server_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_timestamp_from_server
    }

    fn mut_client_timestamp_from_server_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_timestamp_from_server
    }

    // optional uint32 router_gameserver_latency = 5;

    pub fn clear_router_gameserver_latency(&mut self) {
        self.router_gameserver_latency = ::std::option::Option::None;
    }

    pub fn has_router_gameserver_latency(&self) -> bool {
        self.router_gameserver_latency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_router_gameserver_latency(&mut self, v: u32) {
        self.router_gameserver_latency = ::std::option::Option::Some(v);
    }

    pub fn get_router_gameserver_latency(&self) -> u32 {
        self.router_gameserver_latency.unwrap_or(0)
    }

    fn get_router_gameserver_latency_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.router_gameserver_latency
    }

    fn mut_router_gameserver_latency_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.router_gameserver_latency
    }

    // optional uint32 seconds_until_shutdown = 6;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }

    // optional fixed32 migrate_request_ip = 10;

    pub fn clear_migrate_request_ip(&mut self) {
        self.migrate_request_ip = ::std::option::Option::None;
    }

    pub fn has_migrate_request_ip(&self) -> bool {
        self.migrate_request_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_migrate_request_ip(&mut self, v: u32) {
        self.migrate_request_ip = ::std::option::Option::Some(v);
    }

    pub fn get_migrate_request_ip(&self) -> u32 {
        self.migrate_request_ip.unwrap_or(0)
    }

    fn get_migrate_request_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.migrate_request_ip
    }

    fn mut_migrate_request_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.migrate_request_ip
    }

    // optional uint32 migrate_request_port = 11;

    pub fn clear_migrate_request_port(&mut self) {
        self.migrate_request_port = ::std::option::Option::None;
    }

    pub fn has_migrate_request_port(&self) -> bool {
        self.migrate_request_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_migrate_request_port(&mut self, v: u32) {
        self.migrate_request_port = ::std::option::Option::Some(v);
    }

    pub fn get_migrate_request_port(&self) -> u32 {
        self.migrate_request_port.unwrap_or(0)
    }

    fn get_migrate_request_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.migrate_request_port
    }

    fn mut_migrate_request_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.migrate_request_port
    }

    // optional uint32 scoring_penalty_relay_cluster = 12;

    pub fn clear_scoring_penalty_relay_cluster(&mut self) {
        self.scoring_penalty_relay_cluster = ::std::option::Option::None;
    }

    pub fn has_scoring_penalty_relay_cluster(&self) -> bool {
        self.scoring_penalty_relay_cluster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scoring_penalty_relay_cluster(&mut self, v: u32) {
        self.scoring_penalty_relay_cluster = ::std::option::Option::Some(v);
    }

    pub fn get_scoring_penalty_relay_cluster(&self) -> u32 {
        self.scoring_penalty_relay_cluster.unwrap_or(0)
    }

    fn get_scoring_penalty_relay_cluster_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.scoring_penalty_relay_cluster
    }

    fn mut_scoring_penalty_relay_cluster_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.scoring_penalty_relay_cluster
    }

    // optional fixed32 client_cookie = 7;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // optional uint32 seq_num_r2c = 8;

    pub fn clear_seq_num_r2c(&mut self) {
        self.seq_num_r2c = ::std::option::Option::None;
    }

    pub fn has_seq_num_r2c(&self) -> bool {
        self.seq_num_r2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_r2c(&mut self, v: u32) {
        self.seq_num_r2c = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_r2c(&self) -> u32 {
        self.seq_num_r2c.unwrap_or(0)
    }

    fn get_seq_num_r2c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_r2c
    }

    fn mut_seq_num_r2c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_r2c
    }

    // optional uint32 seq_num_s2c = 9;

    pub fn clear_seq_num_s2c(&mut self) {
        self.seq_num_s2c = ::std::option::Option::None;
    }

    pub fn has_seq_num_s2c(&self) -> bool {
        self.seq_num_s2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_s2c(&mut self, v: u32) {
        self.seq_num_s2c = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_s2c(&self) -> u32 {
        self.seq_num_s2c.unwrap_or(0)
    }

    fn get_seq_num_s2c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_s2c
    }

    fn mut_seq_num_s2c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_s2c
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.r2c)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.s2c)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_timestamp_from_router = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_timestamp_from_server = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.router_gameserver_latency = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.migrate_request_ip = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.migrate_request_port = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.scoring_penalty_relay_cluster = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num_r2c = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num_s2c = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.r2c.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.s2c.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.client_timestamp_from_router {
            my_size += 5;
        };
        if let Some(v) = self.client_timestamp_from_server {
            my_size += 5;
        };
        if let Some(v) = self.router_gameserver_latency {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.migrate_request_ip {
            my_size += 5;
        };
        if let Some(v) = self.migrate_request_port {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.scoring_penalty_relay_cluster {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        if let Some(v) = self.seq_num_r2c {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seq_num_s2c {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r2c.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.s2c.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.client_timestamp_from_router {
            os.write_fixed32(3, v)?;
        };
        if let Some(v) = self.client_timestamp_from_server {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.router_gameserver_latency {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.seconds_until_shutdown {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.migrate_request_ip {
            os.write_fixed32(10, v)?;
        };
        if let Some(v) = self.migrate_request_port {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.scoring_penalty_relay_cluster {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.client_cookie {
            os.write_fixed32(7, v)?;
        };
        if let Some(v) = self.seq_num_r2c {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.seq_num_s2c {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn new() -> CMsgSteamDatagramConnectionStatsRouterToClient {
        CMsgSteamDatagramConnectionStatsRouterToClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsRouterToClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "r2c",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_r2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_r2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "s2c",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_s2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_s2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_timestamp_from_router",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_client_timestamp_from_router_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_client_timestamp_from_router_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_timestamp_from_server",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_client_timestamp_from_server_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_client_timestamp_from_server_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "router_gameserver_latency",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_router_gameserver_latency_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_router_gameserver_latency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_seconds_until_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "migrate_request_ip",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_migrate_request_ip_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_migrate_request_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "migrate_request_port",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_migrate_request_port_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_migrate_request_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "scoring_penalty_relay_cluster",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_scoring_penalty_relay_cluster_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_scoring_penalty_relay_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_client_cookie_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_r2c",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_seq_num_r2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_seq_num_r2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_s2c",
                    CMsgSteamDatagramConnectionStatsRouterToClient::get_seq_num_s2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToClient::mut_seq_num_s2c_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionStatsRouterToClient>(
                    "CMsgSteamDatagramConnectionStatsRouterToClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn clear(&mut self) {
        self.clear_r2c();
        self.clear_s2c();
        self.clear_client_timestamp_from_router();
        self.clear_client_timestamp_from_server();
        self.clear_router_gameserver_latency();
        self.clear_seconds_until_shutdown();
        self.clear_migrate_request_ip();
        self.clear_migrate_request_port();
        self.clear_scoring_penalty_relay_cluster();
        self.clear_client_cookie();
        self.clear_seq_num_r2c();
        self.clear_seq_num_s2c();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsRouterToClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionStatsRouterToServer {
    // message fields
    r2s: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    c2s: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    client_timestamp: ::std::option::Option<u32>,
    router_timestamp: ::std::option::Option<u32>,
    seq_num_r2s: ::std::option::Option<u32>,
    seq_num_c2s: ::std::option::Option<u32>,
    client_steam_id: ::std::option::Option<u64>,
    client_session_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionStatsRouterToServer {}

impl CMsgSteamDatagramConnectionStatsRouterToServer {
    pub fn new() -> CMsgSteamDatagramConnectionStatsRouterToServer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionStatsRouterToServer {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionStatsRouterToServer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionStatsRouterToServer,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionStatsRouterToServer::new)
        }
    }

    // optional .dota.CMsgSteamDatagramConnectionQuality r2s = 1;

    pub fn clear_r2s(&mut self) {
        self.r2s.clear();
    }

    pub fn has_r2s(&self) -> bool {
        self.r2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r2s(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.r2s = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_r2s(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.r2s.is_none() {
            self.r2s.set_default();
        };
        self.r2s.as_mut().unwrap()
    }

    // Take field
    pub fn take_r2s(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.r2s.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_r2s(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.r2s.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_r2s_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.r2s
    }

    fn mut_r2s_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.r2s
    }

    // optional .dota.CMsgSteamDatagramConnectionQuality c2s = 2;

    pub fn clear_c2s(&mut self) {
        self.c2s.clear();
    }

    pub fn has_c2s(&self) -> bool {
        self.c2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c2s(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.c2s = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_c2s(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.c2s.is_none() {
            self.c2s.set_default();
        };
        self.c2s.as_mut().unwrap()
    }

    // Take field
    pub fn take_c2s(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.c2s.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_c2s(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.c2s.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_c2s_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.c2s
    }

    fn mut_c2s_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.c2s
    }

    // optional fixed32 client_timestamp = 3;

    pub fn clear_client_timestamp(&mut self) {
        self.client_timestamp = ::std::option::Option::None;
    }

    pub fn has_client_timestamp(&self) -> bool {
        self.client_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_timestamp(&mut self, v: u32) {
        self.client_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_client_timestamp(&self) -> u32 {
        self.client_timestamp.unwrap_or(0)
    }

    fn get_client_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_timestamp
    }

    fn mut_client_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_timestamp
    }

    // optional fixed32 router_timestamp = 4;

    pub fn clear_router_timestamp(&mut self) {
        self.router_timestamp = ::std::option::Option::None;
    }

    pub fn has_router_timestamp(&self) -> bool {
        self.router_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_router_timestamp(&mut self, v: u32) {
        self.router_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_router_timestamp(&self) -> u32 {
        self.router_timestamp.unwrap_or(0)
    }

    fn get_router_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.router_timestamp
    }

    fn mut_router_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.router_timestamp
    }

    // optional uint32 seq_num_r2s = 5;

    pub fn clear_seq_num_r2s(&mut self) {
        self.seq_num_r2s = ::std::option::Option::None;
    }

    pub fn has_seq_num_r2s(&self) -> bool {
        self.seq_num_r2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_r2s(&mut self, v: u32) {
        self.seq_num_r2s = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_r2s(&self) -> u32 {
        self.seq_num_r2s.unwrap_or(0)
    }

    fn get_seq_num_r2s_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_r2s
    }

    fn mut_seq_num_r2s_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_r2s
    }

    // optional uint32 seq_num_c2s = 6;

    pub fn clear_seq_num_c2s(&mut self) {
        self.seq_num_c2s = ::std::option::Option::None;
    }

    pub fn has_seq_num_c2s(&self) -> bool {
        self.seq_num_c2s.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_c2s(&mut self, v: u32) {
        self.seq_num_c2s = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_c2s(&self) -> u32 {
        self.seq_num_c2s.unwrap_or(0)
    }

    fn get_seq_num_c2s_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_c2s
    }

    fn mut_seq_num_c2s_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_c2s
    }

    // optional fixed64 client_steam_id = 7;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional uint32 client_session_id = 8;

    pub fn clear_client_session_id(&mut self) {
        self.client_session_id = ::std::option::Option::None;
    }

    pub fn has_client_session_id(&self) -> bool {
        self.client_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_session_id(&mut self, v: u32) {
        self.client_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_session_id(&self) -> u32 {
        self.client_session_id.unwrap_or(0)
    }

    fn get_client_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_session_id
    }

    fn mut_client_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_session_id
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.r2s)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.c2s)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.client_timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.router_timestamp = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num_r2s = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num_c2s = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_session_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.r2s.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.c2s.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.client_timestamp {
            my_size += 5;
        };
        if let Some(v) = self.router_timestamp {
            my_size += 5;
        };
        if let Some(v) = self.seq_num_r2s {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seq_num_c2s {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        };
        if let Some(v) = self.client_session_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r2s.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.c2s.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.client_timestamp {
            os.write_fixed32(3, v)?;
        };
        if let Some(v) = self.router_timestamp {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.seq_num_r2s {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.seq_num_c2s {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(7, v)?;
        };
        if let Some(v) = self.client_session_id {
            os.write_uint32(8, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn new() -> CMsgSteamDatagramConnectionStatsRouterToServer {
        CMsgSteamDatagramConnectionStatsRouterToServer::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsRouterToServer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "r2s",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_r2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_r2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "c2s",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_c2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_c2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_timestamp",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_client_timestamp_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_client_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "router_timestamp",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_router_timestamp_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_router_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_r2s",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_seq_num_r2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_seq_num_r2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_c2s",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_seq_num_c2s_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_seq_num_c2s_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_client_steam_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_session_id",
                    CMsgSteamDatagramConnectionStatsRouterToServer::get_client_session_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsRouterToServer::mut_client_session_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionStatsRouterToServer>(
                    "CMsgSteamDatagramConnectionStatsRouterToServer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn clear(&mut self) {
        self.clear_r2s();
        self.clear_c2s();
        self.clear_client_timestamp();
        self.clear_router_timestamp();
        self.clear_seq_num_r2s();
        self.clear_seq_num_c2s();
        self.clear_client_steam_id();
        self.clear_client_session_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsRouterToServer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramConnectionStatsServerToRouter {
    // message fields
    s2r: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    s2c: ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality>,
    seq_num_s2r: ::std::option::Option<u32>,
    seq_num_s2c: ::std::option::Option<u32>,
    client_steam_id: ::std::option::Option<u64>,
    client_session_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramConnectionStatsServerToRouter {}

impl CMsgSteamDatagramConnectionStatsServerToRouter {
    pub fn new() -> CMsgSteamDatagramConnectionStatsServerToRouter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramConnectionStatsServerToRouter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramConnectionStatsServerToRouter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramConnectionStatsServerToRouter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramConnectionStatsServerToRouter::new)
        }
    }

    // optional .dota.CMsgSteamDatagramConnectionQuality s2r = 1;

    pub fn clear_s2r(&mut self) {
        self.s2r.clear();
    }

    pub fn has_s2r(&self) -> bool {
        self.s2r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2r(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.s2r = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s2r(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.s2r.is_none() {
            self.s2r.set_default();
        };
        self.s2r.as_mut().unwrap()
    }

    // Take field
    pub fn take_s2r(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.s2r.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_s2r(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.s2r.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_s2r_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.s2r
    }

    fn mut_s2r_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.s2r
    }

    // optional .dota.CMsgSteamDatagramConnectionQuality s2c = 2;

    pub fn clear_s2c(&mut self) {
        self.s2c.clear();
    }

    pub fn has_s2c(&self) -> bool {
        self.s2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_s2c(&mut self, v: CMsgSteamDatagramConnectionQuality) {
        self.s2c = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_s2c(&mut self) -> &mut CMsgSteamDatagramConnectionQuality {
        if self.s2c.is_none() {
            self.s2c.set_default();
        };
        self.s2c.as_mut().unwrap()
    }

    // Take field
    pub fn take_s2c(&mut self) -> CMsgSteamDatagramConnectionQuality {
        self.s2c.take().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::new())
    }

    pub fn get_s2c(&self) -> &CMsgSteamDatagramConnectionQuality {
        self.s2c.as_ref().unwrap_or_else(|| CMsgSteamDatagramConnectionQuality::default_instance())
    }

    fn get_s2c_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &self.s2c
    }

    fn mut_s2c_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramConnectionQuality> {
        &mut self.s2c
    }

    // optional uint32 seq_num_s2r = 3;

    pub fn clear_seq_num_s2r(&mut self) {
        self.seq_num_s2r = ::std::option::Option::None;
    }

    pub fn has_seq_num_s2r(&self) -> bool {
        self.seq_num_s2r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_s2r(&mut self, v: u32) {
        self.seq_num_s2r = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_s2r(&self) -> u32 {
        self.seq_num_s2r.unwrap_or(0)
    }

    fn get_seq_num_s2r_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_s2r
    }

    fn mut_seq_num_s2r_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_s2r
    }

    // optional uint32 seq_num_s2c = 4;

    pub fn clear_seq_num_s2c(&mut self) {
        self.seq_num_s2c = ::std::option::Option::None;
    }

    pub fn has_seq_num_s2c(&self) -> bool {
        self.seq_num_s2c.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seq_num_s2c(&mut self, v: u32) {
        self.seq_num_s2c = ::std::option::Option::Some(v);
    }

    pub fn get_seq_num_s2c(&self) -> u32 {
        self.seq_num_s2c.unwrap_or(0)
    }

    fn get_seq_num_s2c_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seq_num_s2c
    }

    fn mut_seq_num_s2c_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seq_num_s2c
    }

    // optional fixed64 client_steam_id = 5;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional uint32 client_session_id = 6;

    pub fn clear_client_session_id(&mut self) {
        self.client_session_id = ::std::option::Option::None;
    }

    pub fn has_client_session_id(&self) -> bool {
        self.client_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_session_id(&mut self, v: u32) {
        self.client_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_session_id(&self) -> u32 {
        self.client_session_id.unwrap_or(0)
    }

    fn get_client_session_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_session_id
    }

    fn mut_client_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_session_id
    }
}

impl ::protobuf::Message for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.s2r)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.s2c)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num_s2r = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seq_num_s2c = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_session_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.s2r.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.s2c.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.seq_num_s2r {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seq_num_s2c {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        };
        if let Some(v) = self.client_session_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.s2r.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.s2c.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.seq_num_s2r {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.seq_num_s2c {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(5, v)?;
        };
        if let Some(v) = self.client_session_id {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn new() -> CMsgSteamDatagramConnectionStatsServerToRouter {
        CMsgSteamDatagramConnectionStatsServerToRouter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramConnectionStatsServerToRouter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "s2r",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_s2r_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_s2r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramConnectionQuality>>(
                    "s2c",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_s2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_s2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_s2r",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_seq_num_s2r_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_seq_num_s2r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seq_num_s2c",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_seq_num_s2c_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_seq_num_s2c_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_client_steam_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_session_id",
                    CMsgSteamDatagramConnectionStatsServerToRouter::get_client_session_id_for_reflect,
                    CMsgSteamDatagramConnectionStatsServerToRouter::mut_client_session_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramConnectionStatsServerToRouter>(
                    "CMsgSteamDatagramConnectionStatsServerToRouter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn clear(&mut self) {
        self.clear_s2r();
        self.clear_s2c();
        self.clear_seq_num_s2r();
        self.clear_seq_num_s2c();
        self.clear_client_steam_id();
        self.clear_client_session_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramConnectionStatsServerToRouter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientPingSampleRequest {
    // message fields
    client_cookie: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientPingSampleRequest {}

impl CMsgSteamDatagramClientPingSampleRequest {
    pub fn new() -> CMsgSteamDatagramClientPingSampleRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientPingSampleRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientPingSampleRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientPingSampleRequest,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientPingSampleRequest::new)
        }
    }

    // optional fixed32 client_cookie = 1;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientPingSampleRequest {
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
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_cookie {
            os.write_fixed32(1, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientPingSampleRequest {
    fn new() -> CMsgSteamDatagramClientPingSampleRequest {
        CMsgSteamDatagramClientPingSampleRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientPingSampleRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramClientPingSampleRequest::get_client_cookie_for_reflect,
                    CMsgSteamDatagramClientPingSampleRequest::mut_client_cookie_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientPingSampleRequest>(
                    "CMsgSteamDatagramClientPingSampleRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientPingSampleRequest {
    fn clear(&mut self) {
        self.clear_client_cookie();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientPingSampleRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientPingSampleRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientPingSampleReply {
    // message fields
    client_cookie: ::std::option::Option<u32>,
    routing_clusters: ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientPingSampleReply {}

impl CMsgSteamDatagramClientPingSampleReply {
    pub fn new() -> CMsgSteamDatagramClientPingSampleReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientPingSampleReply {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientPingSampleReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientPingSampleReply,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientPingSampleReply::new)
        }
    }

    // optional fixed32 client_cookie = 1;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // repeated .dota.CMsgSteamDatagramClientPingSampleReply.RoutingCluster routing_clusters = 2;

    pub fn clear_routing_clusters(&mut self) {
        self.routing_clusters.clear();
    }

    // Param is passed by value, moved
    pub fn set_routing_clusters(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>) {
        self.routing_clusters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_routing_clusters(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> {
        &mut self.routing_clusters
    }

    // Take field
    pub fn take_routing_clusters(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> {
        ::std::mem::replace(&mut self.routing_clusters, ::protobuf::RepeatedField::new())
    }

    pub fn get_routing_clusters(&self) -> &[CMsgSteamDatagramClientPingSampleReply_RoutingCluster] {
        &self.routing_clusters
    }

    fn get_routing_clusters_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> {
        &self.routing_clusters
    }

    fn mut_routing_clusters_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> {
        &mut self.routing_clusters
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientPingSampleReply {
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
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.routing_clusters)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        for value in &self.routing_clusters {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_cookie {
            os.write_fixed32(1, v)?;
        };
        for v in &self.routing_clusters {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientPingSampleReply {
    fn new() -> CMsgSteamDatagramClientPingSampleReply {
        CMsgSteamDatagramClientPingSampleReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientPingSampleReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramClientPingSampleReply::get_client_cookie_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>>(
                    "routing_clusters",
                    CMsgSteamDatagramClientPingSampleReply::get_routing_clusters_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply::mut_routing_clusters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientPingSampleReply>(
                    "CMsgSteamDatagramClientPingSampleReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientPingSampleReply {
    fn clear(&mut self) {
        self.clear_client_cookie();
        self.clear_routing_clusters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientPingSampleReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientPingSampleReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    // message fields
    id: ::std::option::Option<u32>,
    front_ping_ms: ::std::option::Option<u32>,
    e2e_ping_ms: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {}

impl CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    pub fn new() -> CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientPingSampleReply_RoutingCluster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientPingSampleReply_RoutingCluster,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientPingSampleReply_RoutingCluster::new)
        }
    }

    // optional fixed32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.id
    }

    // optional uint32 front_ping_ms = 2;

    pub fn clear_front_ping_ms(&mut self) {
        self.front_ping_ms = ::std::option::Option::None;
    }

    pub fn has_front_ping_ms(&self) -> bool {
        self.front_ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_front_ping_ms(&mut self, v: u32) {
        self.front_ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_front_ping_ms(&self) -> u32 {
        self.front_ping_ms.unwrap_or(0)
    }

    fn get_front_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.front_ping_ms
    }

    fn mut_front_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.front_ping_ms
    }

    // optional uint32 e2e_ping_ms = 3;

    pub fn clear_e2e_ping_ms(&mut self) {
        self.e2e_ping_ms = ::std::option::Option::None;
    }

    pub fn has_e2e_ping_ms(&self) -> bool {
        self.e2e_ping_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_e2e_ping_ms(&mut self, v: u32) {
        self.e2e_ping_ms = ::std::option::Option::Some(v);
    }

    pub fn get_e2e_ping_ms(&self) -> u32 {
        self.e2e_ping_ms.unwrap_or(0)
    }

    fn get_e2e_ping_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.e2e_ping_ms
    }

    fn mut_e2e_ping_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.e2e_ping_ms
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.front_ping_ms = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.e2e_ping_ms = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.id {
            my_size += 5;
        };
        if let Some(v) = self.front_ping_ms {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.e2e_ping_ms {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.front_ping_ms {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.e2e_ping_ms {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    fn new() -> CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
        CMsgSteamDatagramClientPingSampleReply_RoutingCluster::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "id",
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::get_id_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "front_ping_ms",
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::get_front_ping_ms_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::mut_front_ping_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "e2e_ping_ms",
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::get_e2e_ping_ms_for_reflect,
                    CMsgSteamDatagramClientPingSampleReply_RoutingCluster::mut_e2e_ping_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientPingSampleReply_RoutingCluster>(
                    "CMsgSteamDatagramClientPingSampleReply_RoutingCluster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_front_ping_ms();
        self.clear_e2e_ping_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientPingSampleReply_RoutingCluster {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientSwitchedPrimary {
    // message fields
    client_cookie: ::std::option::Option<u32>,
    from_ip: ::std::option::Option<u32>,
    from_port: ::std::option::Option<u32>,
    from_router_cluster: ::std::option::Option<u32>,
    from_active_time: ::std::option::Option<u32>,
    from_active_packets_recv: ::std::option::Option<u32>,
    from_dropped_reason: ::protobuf::SingularField<::std::string::String>,
    gap_ms: ::std::option::Option<u32>,
    from_quality_now: ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>,
    to_quality_now: ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>,
    from_quality_then: ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>,
    to_quality_then: ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientSwitchedPrimary {}

impl CMsgSteamDatagramClientSwitchedPrimary {
    pub fn new() -> CMsgSteamDatagramClientSwitchedPrimary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientSwitchedPrimary {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientSwitchedPrimary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientSwitchedPrimary,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientSwitchedPrimary::new)
        }
    }

    // optional fixed32 client_cookie = 1;

    pub fn clear_client_cookie(&mut self) {
        self.client_cookie = ::std::option::Option::None;
    }

    pub fn has_client_cookie(&self) -> bool {
        self.client_cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_cookie(&mut self, v: u32) {
        self.client_cookie = ::std::option::Option::Some(v);
    }

    pub fn get_client_cookie(&self) -> u32 {
        self.client_cookie.unwrap_or(0)
    }

    fn get_client_cookie_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_cookie
    }

    fn mut_client_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_cookie
    }

    // optional fixed32 from_ip = 2;

    pub fn clear_from_ip(&mut self) {
        self.from_ip = ::std::option::Option::None;
    }

    pub fn has_from_ip(&self) -> bool {
        self.from_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_ip(&mut self, v: u32) {
        self.from_ip = ::std::option::Option::Some(v);
    }

    pub fn get_from_ip(&self) -> u32 {
        self.from_ip.unwrap_or(0)
    }

    fn get_from_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_ip
    }

    fn mut_from_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_ip
    }

    // optional uint32 from_port = 3;

    pub fn clear_from_port(&mut self) {
        self.from_port = ::std::option::Option::None;
    }

    pub fn has_from_port(&self) -> bool {
        self.from_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_port(&mut self, v: u32) {
        self.from_port = ::std::option::Option::Some(v);
    }

    pub fn get_from_port(&self) -> u32 {
        self.from_port.unwrap_or(0)
    }

    fn get_from_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_port
    }

    fn mut_from_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_port
    }

    // optional fixed32 from_router_cluster = 4;

    pub fn clear_from_router_cluster(&mut self) {
        self.from_router_cluster = ::std::option::Option::None;
    }

    pub fn has_from_router_cluster(&self) -> bool {
        self.from_router_cluster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_router_cluster(&mut self, v: u32) {
        self.from_router_cluster = ::std::option::Option::Some(v);
    }

    pub fn get_from_router_cluster(&self) -> u32 {
        self.from_router_cluster.unwrap_or(0)
    }

    fn get_from_router_cluster_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_router_cluster
    }

    fn mut_from_router_cluster_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_router_cluster
    }

    // optional uint32 from_active_time = 5;

    pub fn clear_from_active_time(&mut self) {
        self.from_active_time = ::std::option::Option::None;
    }

    pub fn has_from_active_time(&self) -> bool {
        self.from_active_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_active_time(&mut self, v: u32) {
        self.from_active_time = ::std::option::Option::Some(v);
    }

    pub fn get_from_active_time(&self) -> u32 {
        self.from_active_time.unwrap_or(0)
    }

    fn get_from_active_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_active_time
    }

    fn mut_from_active_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_active_time
    }

    // optional uint32 from_active_packets_recv = 6;

    pub fn clear_from_active_packets_recv(&mut self) {
        self.from_active_packets_recv = ::std::option::Option::None;
    }

    pub fn has_from_active_packets_recv(&self) -> bool {
        self.from_active_packets_recv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_active_packets_recv(&mut self, v: u32) {
        self.from_active_packets_recv = ::std::option::Option::Some(v);
    }

    pub fn get_from_active_packets_recv(&self) -> u32 {
        self.from_active_packets_recv.unwrap_or(0)
    }

    fn get_from_active_packets_recv_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.from_active_packets_recv
    }

    fn mut_from_active_packets_recv_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.from_active_packets_recv
    }

    // optional string from_dropped_reason = 7;

    pub fn clear_from_dropped_reason(&mut self) {
        self.from_dropped_reason.clear();
    }

    pub fn has_from_dropped_reason(&self) -> bool {
        self.from_dropped_reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_dropped_reason(&mut self, v: ::std::string::String) {
        self.from_dropped_reason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_dropped_reason(&mut self) -> &mut ::std::string::String {
        if self.from_dropped_reason.is_none() {
            self.from_dropped_reason.set_default();
        };
        self.from_dropped_reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_dropped_reason(&mut self) -> ::std::string::String {
        self.from_dropped_reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_from_dropped_reason(&self) -> &str {
        match self.from_dropped_reason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_from_dropped_reason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.from_dropped_reason
    }

    fn mut_from_dropped_reason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.from_dropped_reason
    }

    // optional uint32 gap_ms = 8;

    pub fn clear_gap_ms(&mut self) {
        self.gap_ms = ::std::option::Option::None;
    }

    pub fn has_gap_ms(&self) -> bool {
        self.gap_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gap_ms(&mut self, v: u32) {
        self.gap_ms = ::std::option::Option::Some(v);
    }

    pub fn get_gap_ms(&self) -> u32 {
        self.gap_ms.unwrap_or(0)
    }

    fn get_gap_ms_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gap_ms
    }

    fn mut_gap_ms_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gap_ms
    }

    // optional .dota.CMsgSteamDatagramClientSwitchedPrimary.RouterQuality from_quality_now = 9;

    pub fn clear_from_quality_now(&mut self) {
        self.from_quality_now.clear();
    }

    pub fn has_from_quality_now(&self) -> bool {
        self.from_quality_now.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_quality_now(&mut self, v: CMsgSteamDatagramClientSwitchedPrimary_RouterQuality) {
        self.from_quality_now = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_quality_now(&mut self) -> &mut CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        if self.from_quality_now.is_none() {
            self.from_quality_now.set_default();
        };
        self.from_quality_now.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_quality_now(&mut self) -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.from_quality_now.take().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new())
    }

    pub fn get_from_quality_now(&self) -> &CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.from_quality_now.as_ref().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::default_instance())
    }

    fn get_from_quality_now_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &self.from_quality_now
    }

    fn mut_from_quality_now_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &mut self.from_quality_now
    }

    // optional .dota.CMsgSteamDatagramClientSwitchedPrimary.RouterQuality to_quality_now = 10;

    pub fn clear_to_quality_now(&mut self) {
        self.to_quality_now.clear();
    }

    pub fn has_to_quality_now(&self) -> bool {
        self.to_quality_now.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_quality_now(&mut self, v: CMsgSteamDatagramClientSwitchedPrimary_RouterQuality) {
        self.to_quality_now = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to_quality_now(&mut self) -> &mut CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        if self.to_quality_now.is_none() {
            self.to_quality_now.set_default();
        };
        self.to_quality_now.as_mut().unwrap()
    }

    // Take field
    pub fn take_to_quality_now(&mut self) -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.to_quality_now.take().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new())
    }

    pub fn get_to_quality_now(&self) -> &CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.to_quality_now.as_ref().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::default_instance())
    }

    fn get_to_quality_now_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &self.to_quality_now
    }

    fn mut_to_quality_now_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &mut self.to_quality_now
    }

    // optional .dota.CMsgSteamDatagramClientSwitchedPrimary.RouterQuality from_quality_then = 11;

    pub fn clear_from_quality_then(&mut self) {
        self.from_quality_then.clear();
    }

    pub fn has_from_quality_then(&self) -> bool {
        self.from_quality_then.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_quality_then(&mut self, v: CMsgSteamDatagramClientSwitchedPrimary_RouterQuality) {
        self.from_quality_then = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_quality_then(&mut self) -> &mut CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        if self.from_quality_then.is_none() {
            self.from_quality_then.set_default();
        };
        self.from_quality_then.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_quality_then(&mut self) -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.from_quality_then.take().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new())
    }

    pub fn get_from_quality_then(&self) -> &CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.from_quality_then.as_ref().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::default_instance())
    }

    fn get_from_quality_then_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &self.from_quality_then
    }

    fn mut_from_quality_then_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &mut self.from_quality_then
    }

    // optional .dota.CMsgSteamDatagramClientSwitchedPrimary.RouterQuality to_quality_then = 12;

    pub fn clear_to_quality_then(&mut self) {
        self.to_quality_then.clear();
    }

    pub fn has_to_quality_then(&self) -> bool {
        self.to_quality_then.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_quality_then(&mut self, v: CMsgSteamDatagramClientSwitchedPrimary_RouterQuality) {
        self.to_quality_then = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to_quality_then(&mut self) -> &mut CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        if self.to_quality_then.is_none() {
            self.to_quality_then.set_default();
        };
        self.to_quality_then.as_mut().unwrap()
    }

    // Take field
    pub fn take_to_quality_then(&mut self) -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.to_quality_then.take().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new())
    }

    pub fn get_to_quality_then(&self) -> &CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        self.to_quality_then.as_ref().unwrap_or_else(|| CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::default_instance())
    }

    fn get_to_quality_then_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &self.to_quality_then
    }

    fn mut_to_quality_then_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> {
        &mut self.to_quality_then
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientSwitchedPrimary {
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
                    self.client_cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.from_ip = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.from_port = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.from_router_cluster = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.from_active_time = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.from_active_packets_recv = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.from_dropped_reason)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.gap_ms = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from_quality_now)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.to_quality_now)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from_quality_then)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.to_quality_then)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_cookie {
            my_size += 5;
        };
        if let Some(v) = self.from_ip {
            my_size += 5;
        };
        if let Some(v) = self.from_port {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.from_router_cluster {
            my_size += 5;
        };
        if let Some(v) = self.from_active_time {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.from_active_packets_recv {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.from_dropped_reason.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.gap_ms {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.from_quality_now.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.to_quality_now.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.from_quality_then.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.to_quality_then.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_cookie {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.from_ip {
            os.write_fixed32(2, v)?;
        };
        if let Some(v) = self.from_port {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.from_router_cluster {
            os.write_fixed32(4, v)?;
        };
        if let Some(v) = self.from_active_time {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.from_active_packets_recv {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.from_dropped_reason.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.gap_ms {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.from_quality_now.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.to_quality_now.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.from_quality_then.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.to_quality_then.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientSwitchedPrimary {
    fn new() -> CMsgSteamDatagramClientSwitchedPrimary {
        CMsgSteamDatagramClientSwitchedPrimary::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientSwitchedPrimary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_cookie",
                    CMsgSteamDatagramClientSwitchedPrimary::get_client_cookie_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_client_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_ip",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_ip_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "from_port",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_port_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "from_router_cluster",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_router_cluster_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_router_cluster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "from_active_time",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_active_time_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_active_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "from_active_packets_recv",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_active_packets_recv_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_active_packets_recv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "from_dropped_reason",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_dropped_reason_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_dropped_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gap_ms",
                    CMsgSteamDatagramClientSwitchedPrimary::get_gap_ms_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_gap_ms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>>(
                    "from_quality_now",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_quality_now_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_quality_now_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>>(
                    "to_quality_now",
                    CMsgSteamDatagramClientSwitchedPrimary::get_to_quality_now_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_to_quality_now_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>>(
                    "from_quality_then",
                    CMsgSteamDatagramClientSwitchedPrimary::get_from_quality_then_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_from_quality_then_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>>(
                    "to_quality_then",
                    CMsgSteamDatagramClientSwitchedPrimary::get_to_quality_then_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary::mut_to_quality_then_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientSwitchedPrimary>(
                    "CMsgSteamDatagramClientSwitchedPrimary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientSwitchedPrimary {
    fn clear(&mut self) {
        self.clear_client_cookie();
        self.clear_from_ip();
        self.clear_from_port();
        self.clear_from_router_cluster();
        self.clear_from_active_time();
        self.clear_from_active_packets_recv();
        self.clear_from_dropped_reason();
        self.clear_gap_ms();
        self.clear_from_quality_now();
        self.clear_to_quality_now();
        self.clear_from_quality_then();
        self.clear_to_quality_then();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientSwitchedPrimary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientSwitchedPrimary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    // message fields
    score: ::std::option::Option<u32>,
    front_ping: ::std::option::Option<u32>,
    back_ping: ::std::option::Option<u32>,
    seconds_until_down: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {}

impl CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    pub fn new() -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramClientSwitchedPrimary_RouterQuality,
        };
        unsafe {
            instance.get(CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new)
        }
    }

    // optional uint32 score = 1;

    pub fn clear_score(&mut self) {
        self.score = ::std::option::Option::None;
    }

    pub fn has_score(&self) -> bool {
        self.score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: u32) {
        self.score = ::std::option::Option::Some(v);
    }

    pub fn get_score(&self) -> u32 {
        self.score.unwrap_or(0)
    }

    fn get_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.score
    }

    // optional uint32 front_ping = 2;

    pub fn clear_front_ping(&mut self) {
        self.front_ping = ::std::option::Option::None;
    }

    pub fn has_front_ping(&self) -> bool {
        self.front_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_front_ping(&mut self, v: u32) {
        self.front_ping = ::std::option::Option::Some(v);
    }

    pub fn get_front_ping(&self) -> u32 {
        self.front_ping.unwrap_or(0)
    }

    fn get_front_ping_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.front_ping
    }

    fn mut_front_ping_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.front_ping
    }

    // optional uint32 back_ping = 3;

    pub fn clear_back_ping(&mut self) {
        self.back_ping = ::std::option::Option::None;
    }

    pub fn has_back_ping(&self) -> bool {
        self.back_ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_back_ping(&mut self, v: u32) {
        self.back_ping = ::std::option::Option::Some(v);
    }

    pub fn get_back_ping(&self) -> u32 {
        self.back_ping.unwrap_or(0)
    }

    fn get_back_ping_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.back_ping
    }

    fn mut_back_ping_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.back_ping
    }

    // optional uint32 seconds_until_down = 4;

    pub fn clear_seconds_until_down(&mut self) {
        self.seconds_until_down = ::std::option::Option::None;
    }

    pub fn has_seconds_until_down(&self) -> bool {
        self.seconds_until_down.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_down(&mut self, v: u32) {
        self.seconds_until_down = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_down(&self) -> u32 {
        self.seconds_until_down.unwrap_or(0)
    }

    fn get_seconds_until_down_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_down
    }

    fn mut_seconds_until_down_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_down
    }
}

impl ::protobuf::Message for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
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
                    self.score = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.front_ping = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.back_ping = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seconds_until_down = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.score {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.front_ping {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.back_ping {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seconds_until_down {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.score {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.front_ping {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.back_ping {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.seconds_until_down {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    fn new() -> CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
        CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "score",
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::get_score_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::mut_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "front_ping",
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::get_front_ping_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::mut_front_ping_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "back_ping",
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::get_back_ping_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::mut_back_ping_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_down",
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::get_seconds_until_down_for_reflect,
                    CMsgSteamDatagramClientSwitchedPrimary_RouterQuality::mut_seconds_until_down_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramClientSwitchedPrimary_RouterQuality>(
                    "CMsgSteamDatagramClientSwitchedPrimary_RouterQuality",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    fn clear(&mut self) {
        self.clear_score();
        self.clear_front_ping();
        self.clear_back_ping();
        self.clear_seconds_until_down();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramClientSwitchedPrimary_RouterQuality {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramRouterHealth {
    // message fields
    cpu_load: ::std::option::Option<f32>,
    active_sessions: ::std::option::Option<u32>,
    data_pkts_sec: ::std::option::Option<u32>,
    other_pkts_sec: ::std::option::Option<u32>,
    seconds_until_shutdown: ::std::option::Option<u32>,
    cpu_cost_per_user: ::std::option::Option<f32>,
    cpu_cost_per_packet: ::std::option::Option<f32>,
    data_centers: ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter>,
    magic: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRouterHealth {}

impl CMsgSteamDatagramRouterHealth {
    pub fn new() -> CMsgSteamDatagramRouterHealth {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRouterHealth {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRouterHealth> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRouterHealth,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRouterHealth::new)
        }
    }

    // optional float cpu_load = 1;

    pub fn clear_cpu_load(&mut self) {
        self.cpu_load = ::std::option::Option::None;
    }

    pub fn has_cpu_load(&self) -> bool {
        self.cpu_load.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_load(&mut self, v: f32) {
        self.cpu_load = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_load(&self) -> f32 {
        self.cpu_load.unwrap_or(0.)
    }

    fn get_cpu_load_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cpu_load
    }

    fn mut_cpu_load_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cpu_load
    }

    // optional uint32 active_sessions = 2;

    pub fn clear_active_sessions(&mut self) {
        self.active_sessions = ::std::option::Option::None;
    }

    pub fn has_active_sessions(&self) -> bool {
        self.active_sessions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_sessions(&mut self, v: u32) {
        self.active_sessions = ::std::option::Option::Some(v);
    }

    pub fn get_active_sessions(&self) -> u32 {
        self.active_sessions.unwrap_or(0)
    }

    fn get_active_sessions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.active_sessions
    }

    fn mut_active_sessions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.active_sessions
    }

    // optional uint32 data_pkts_sec = 3;

    pub fn clear_data_pkts_sec(&mut self) {
        self.data_pkts_sec = ::std::option::Option::None;
    }

    pub fn has_data_pkts_sec(&self) -> bool {
        self.data_pkts_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_pkts_sec(&mut self, v: u32) {
        self.data_pkts_sec = ::std::option::Option::Some(v);
    }

    pub fn get_data_pkts_sec(&self) -> u32 {
        self.data_pkts_sec.unwrap_or(0)
    }

    fn get_data_pkts_sec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.data_pkts_sec
    }

    fn mut_data_pkts_sec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.data_pkts_sec
    }

    // optional uint32 other_pkts_sec = 4;

    pub fn clear_other_pkts_sec(&mut self) {
        self.other_pkts_sec = ::std::option::Option::None;
    }

    pub fn has_other_pkts_sec(&self) -> bool {
        self.other_pkts_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_other_pkts_sec(&mut self, v: u32) {
        self.other_pkts_sec = ::std::option::Option::Some(v);
    }

    pub fn get_other_pkts_sec(&self) -> u32 {
        self.other_pkts_sec.unwrap_or(0)
    }

    fn get_other_pkts_sec_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.other_pkts_sec
    }

    fn mut_other_pkts_sec_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.other_pkts_sec
    }

    // optional uint32 seconds_until_shutdown = 5;

    pub fn clear_seconds_until_shutdown(&mut self) {
        self.seconds_until_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_until_shutdown(&self) -> bool {
        self.seconds_until_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_until_shutdown(&mut self, v: u32) {
        self.seconds_until_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_until_shutdown(&self) -> u32 {
        self.seconds_until_shutdown.unwrap_or(0)
    }

    fn get_seconds_until_shutdown_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.seconds_until_shutdown
    }

    fn mut_seconds_until_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.seconds_until_shutdown
    }

    // optional float cpu_cost_per_user = 8;

    pub fn clear_cpu_cost_per_user(&mut self) {
        self.cpu_cost_per_user = ::std::option::Option::None;
    }

    pub fn has_cpu_cost_per_user(&self) -> bool {
        self.cpu_cost_per_user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_cost_per_user(&mut self, v: f32) {
        self.cpu_cost_per_user = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_cost_per_user(&self) -> f32 {
        self.cpu_cost_per_user.unwrap_or(0.)
    }

    fn get_cpu_cost_per_user_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cpu_cost_per_user
    }

    fn mut_cpu_cost_per_user_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cpu_cost_per_user
    }

    // optional float cpu_cost_per_packet = 9;

    pub fn clear_cpu_cost_per_packet(&mut self) {
        self.cpu_cost_per_packet = ::std::option::Option::None;
    }

    pub fn has_cpu_cost_per_packet(&self) -> bool {
        self.cpu_cost_per_packet.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_cost_per_packet(&mut self, v: f32) {
        self.cpu_cost_per_packet = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_cost_per_packet(&self) -> f32 {
        self.cpu_cost_per_packet.unwrap_or(0.)
    }

    fn get_cpu_cost_per_packet_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cpu_cost_per_packet
    }

    fn mut_cpu_cost_per_packet_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cpu_cost_per_packet
    }

    // repeated .dota.CMsgSteamDatagramRouterHealth.DataCenter data_centers = 6;

    pub fn clear_data_centers(&mut self) {
        self.data_centers.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_centers(&mut self, v: ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter>) {
        self.data_centers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data_centers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter> {
        &mut self.data_centers
    }

    // Take field
    pub fn take_data_centers(&mut self) -> ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter> {
        ::std::mem::replace(&mut self.data_centers, ::protobuf::RepeatedField::new())
    }

    pub fn get_data_centers(&self) -> &[CMsgSteamDatagramRouterHealth_DataCenter] {
        &self.data_centers
    }

    fn get_data_centers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter> {
        &self.data_centers
    }

    fn mut_data_centers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSteamDatagramRouterHealth_DataCenter> {
        &mut self.data_centers
    }

    // optional fixed64 magic = 7;

    pub fn clear_magic(&mut self) {
        self.magic = ::std::option::Option::None;
    }

    pub fn has_magic(&self) -> bool {
        self.magic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_magic(&mut self, v: u64) {
        self.magic = ::std::option::Option::Some(v);
    }

    pub fn get_magic(&self) -> u64 {
        self.magic.unwrap_or(0)
    }

    fn get_magic_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.magic
    }

    fn mut_magic_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.magic
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRouterHealth {
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
                    self.cpu_load = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.active_sessions = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.data_pkts_sec = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.other_pkts_sec = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.seconds_until_shutdown = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.cpu_cost_per_user = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.cpu_cost_per_packet = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data_centers)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.magic = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.cpu_load {
            my_size += 5;
        };
        if let Some(v) = self.active_sessions {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.data_pkts_sec {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.other_pkts_sec {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.seconds_until_shutdown {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.cpu_cost_per_user {
            my_size += 5;
        };
        if let Some(v) = self.cpu_cost_per_packet {
            my_size += 5;
        };
        for value in &self.data_centers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.magic {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cpu_load {
            os.write_float(1, v)?;
        };
        if let Some(v) = self.active_sessions {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.data_pkts_sec {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.other_pkts_sec {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.seconds_until_shutdown {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.cpu_cost_per_user {
            os.write_float(8, v)?;
        };
        if let Some(v) = self.cpu_cost_per_packet {
            os.write_float(9, v)?;
        };
        for v in &self.data_centers {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.magic {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramRouterHealth {
    fn new() -> CMsgSteamDatagramRouterHealth {
        CMsgSteamDatagramRouterHealth::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRouterHealth>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cpu_load",
                    CMsgSteamDatagramRouterHealth::get_cpu_load_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_cpu_load_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "active_sessions",
                    CMsgSteamDatagramRouterHealth::get_active_sessions_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_active_sessions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "data_pkts_sec",
                    CMsgSteamDatagramRouterHealth::get_data_pkts_sec_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_data_pkts_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "other_pkts_sec",
                    CMsgSteamDatagramRouterHealth::get_other_pkts_sec_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_other_pkts_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "seconds_until_shutdown",
                    CMsgSteamDatagramRouterHealth::get_seconds_until_shutdown_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_seconds_until_shutdown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cpu_cost_per_user",
                    CMsgSteamDatagramRouterHealth::get_cpu_cost_per_user_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_cpu_cost_per_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cpu_cost_per_packet",
                    CMsgSteamDatagramRouterHealth::get_cpu_cost_per_packet_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_cpu_cost_per_packet_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSteamDatagramRouterHealth_DataCenter>>(
                    "data_centers",
                    CMsgSteamDatagramRouterHealth::get_data_centers_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_data_centers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "magic",
                    CMsgSteamDatagramRouterHealth::get_magic_for_reflect,
                    CMsgSteamDatagramRouterHealth::mut_magic_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRouterHealth>(
                    "CMsgSteamDatagramRouterHealth",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRouterHealth {
    fn clear(&mut self) {
        self.clear_cpu_load();
        self.clear_active_sessions();
        self.clear_data_pkts_sec();
        self.clear_other_pkts_sec();
        self.clear_seconds_until_shutdown();
        self.clear_cpu_cost_per_user();
        self.clear_cpu_cost_per_packet();
        self.clear_data_centers();
        self.clear_magic();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRouterHealth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRouterHealth {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSteamDatagramRouterHealth_DataCenter {
    // message fields
    datacenter_id: ::std::option::Option<u32>,
    state: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSteamDatagramRouterHealth_DataCenter {}

impl CMsgSteamDatagramRouterHealth_DataCenter {
    pub fn new() -> CMsgSteamDatagramRouterHealth_DataCenter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSteamDatagramRouterHealth_DataCenter {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSteamDatagramRouterHealth_DataCenter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSteamDatagramRouterHealth_DataCenter,
        };
        unsafe {
            instance.get(CMsgSteamDatagramRouterHealth_DataCenter::new)
        }
    }

    // optional fixed32 datacenter_id = 1;

    pub fn clear_datacenter_id(&mut self) {
        self.datacenter_id = ::std::option::Option::None;
    }

    pub fn has_datacenter_id(&self) -> bool {
        self.datacenter_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datacenter_id(&mut self, v: u32) {
        self.datacenter_id = ::std::option::Option::Some(v);
    }

    pub fn get_datacenter_id(&self) -> u32 {
        self.datacenter_id.unwrap_or(0)
    }

    fn get_datacenter_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.datacenter_id
    }

    fn mut_datacenter_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.datacenter_id
    }

    // optional uint32 state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: u32) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> u32 {
        self.state.unwrap_or(0)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.state
    }
}

impl ::protobuf::Message for CMsgSteamDatagramRouterHealth_DataCenter {
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
                    self.datacenter_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
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
        if let Some(v) = self.datacenter_id {
            my_size += 5;
        };
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.datacenter_id {
            os.write_fixed32(1, v)?;
        };
        if let Some(v) = self.state {
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

impl ::protobuf::MessageStatic for CMsgSteamDatagramRouterHealth_DataCenter {
    fn new() -> CMsgSteamDatagramRouterHealth_DataCenter {
        CMsgSteamDatagramRouterHealth_DataCenter::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSteamDatagramRouterHealth_DataCenter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "datacenter_id",
                    CMsgSteamDatagramRouterHealth_DataCenter::get_datacenter_id_for_reflect,
                    CMsgSteamDatagramRouterHealth_DataCenter::mut_datacenter_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "state",
                    CMsgSteamDatagramRouterHealth_DataCenter::get_state_for_reflect,
                    CMsgSteamDatagramRouterHealth_DataCenter::mut_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSteamDatagramRouterHealth_DataCenter>(
                    "CMsgSteamDatagramRouterHealth_DataCenter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSteamDatagramRouterHealth_DataCenter {
    fn clear(&mut self) {
        self.clear_datacenter_id();
        self.clear_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSteamDatagramRouterHealth_DataCenter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSteamDatagramRouterHealth_DataCenter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ESteamDatagramMsgID {
    k_ESteamDatagramMsg_Invalid = 0,
    k_ESteamDatagramMsg_RouterPingRequest = 1,
    k_ESteamDatagramMsg_RouterPingReply = 2,
    k_ESteamDatagramMsg_GameserverPingRequest = 3,
    k_ESteamDatagramMsg_GameserverPingReply = 4,
    k_ESteamDatagramMsg_GameserverSessionRequest = 5,
    k_ESteamDatagramMsg_GameserverSessionEstablished = 6,
    k_ESteamDatagramMsg_NoSession = 7,
    k_ESteamDatagramMsg_Diagnostic = 8,
    k_ESteamDatagramMsg_DataClientToRouter = 9,
    k_ESteamDatagramMsg_DataRouterToServer = 10,
    k_ESteamDatagramMsg_DataServerToRouter = 11,
    k_ESteamDatagramMsg_DataRouterToClient = 12,
    k_ESteamDatagramMsg_Stats = 13,
    k_ESteamDatagramMsg_ClientPingSampleRequest = 14,
    k_ESteamDatagramMsg_ClientPingSampleReply = 15,
    k_ESteamDatagramMsg_ClientToRouterSwitchedPrimary = 16,
    k_ESteamDatagramMsg_RelayHealth = 17,
}

impl ::protobuf::ProtobufEnum for ESteamDatagramMsgID {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ESteamDatagramMsgID> {
        match value {
            0 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_Invalid),
            1 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_RouterPingRequest),
            2 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_RouterPingReply),
            3 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverPingRequest),
            4 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverPingReply),
            5 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverSessionRequest),
            6 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverSessionEstablished),
            7 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_NoSession),
            8 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_Diagnostic),
            9 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_DataClientToRouter),
            10 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_DataRouterToServer),
            11 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_DataServerToRouter),
            12 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_DataRouterToClient),
            13 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_Stats),
            14 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientPingSampleRequest),
            15 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientPingSampleReply),
            16 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientToRouterSwitchedPrimary),
            17 => ::std::option::Option::Some(ESteamDatagramMsgID::k_ESteamDatagramMsg_RelayHealth),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ESteamDatagramMsgID] = &[
            ESteamDatagramMsgID::k_ESteamDatagramMsg_Invalid,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_RouterPingRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_RouterPingReply,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverPingRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverPingReply,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverSessionRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_GameserverSessionEstablished,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_NoSession,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_Diagnostic,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_DataClientToRouter,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_DataRouterToServer,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_DataServerToRouter,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_DataRouterToClient,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_Stats,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientPingSampleRequest,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientPingSampleReply,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_ClientToRouterSwitchedPrimary,
            ESteamDatagramMsgID::k_ESteamDatagramMsg_RelayHealth,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ESteamDatagramMsgID>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ESteamDatagramMsgID", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ESteamDatagramMsgID {
}

impl ::protobuf::reflect::ProtobufValue for ESteamDatagramMsgID {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x64, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x5f,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04,
    0x64, 0x6f, 0x74, 0x61, 0x22, 0xb9, 0x04, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65,
    0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72,
    0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x29, 0x0a, 0x10, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x07, 0x52, 0x0f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x12, 0x38, 0x0a, 0x16, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x5f,
    0x64, 0x61, 0x74, 0x61, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x07, 0x52, 0x14, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x44, 0x61, 0x74,
    0x61, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x73, 0x42, 0x02, 0x10, 0x01, 0x12, 0x2a,
    0x0a, 0x0f, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x6d,
    0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0d, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79,
    0x50, 0x69, 0x6e, 0x67, 0x4d, 0x73, 0x42, 0x02, 0x10, 0x01, 0x12, 0x24, 0x0a, 0x0e, 0x79, 0x6f,
    0x75, 0x72, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x69, 0x70, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x07, 0x52, 0x0c, 0x79, 0x6f, 0x75, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x49, 0x70,
    0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x54, 0x69, 0x6d,
    0x65, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12,
    0x34, 0x0a, 0x16, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x5f, 0x75, 0x6e, 0x74, 0x69, 0x6c,
    0x5f, 0x73, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x14, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x55, 0x6e, 0x74, 0x69, 0x6c, 0x53, 0x68, 0x75,
    0x74, 0x64, 0x6f, 0x77, 0x6e, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x12, 0x41, 0x0a, 0x1d, 0x73, 0x63,
    0x6f, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79, 0x5f, 0x72, 0x65,
    0x6c, 0x61, 0x79, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x1a, 0x73, 0x63, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x50, 0x65, 0x6e, 0x61, 0x6c, 0x74,
    0x79, 0x52, 0x65, 0x6c, 0x61, 0x79, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x12, 0x47, 0x0a,
    0x1e, 0x73, 0x63, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79,
    0x5f, 0x64, 0x61, 0x74, 0x61, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x73, 0x18,
    0x0a, 0x20, 0x03, 0x28, 0x07, 0x52, 0x1b, 0x73, 0x63, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x50, 0x65,
    0x6e, 0x61, 0x6c, 0x74, 0x79, 0x44, 0x61, 0x74, 0x61, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x49,
    0x64, 0x73, 0x42, 0x02, 0x10, 0x01, 0x12, 0x38, 0x0a, 0x16, 0x73, 0x63, 0x6f, 0x72, 0x69, 0x6e,
    0x67, 0x5f, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73,
    0x18, 0x0b, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x14, 0x73, 0x63, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x50,
    0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x42, 0x02, 0x10, 0x01,
    0x22, 0xd4, 0x02, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61,
    0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x50, 0x69, 0x6e, 0x67, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73,
    0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x26, 0x0a, 0x0f, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x74, 0x65, 0x61,
    0x6d, 0x49, 0x64, 0x12, 0x29, 0x0a, 0x10, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0f, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x29,
    0x0a, 0x10, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0f, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x3a, 0x0a, 0x19, 0x72, 0x6f, 0x75,
    0x74, 0x65, 0x72, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x6c,
    0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x17, 0x72, 0x6f,
    0x75, 0x74, 0x65, 0x72, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4c, 0x61,
    0x74, 0x65, 0x6e, 0x63, 0x79, 0x12, 0x2a, 0x0a, 0x11, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x5f, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0f, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x6f, 0x75, 0x74, 0x65,
    0x72, 0x12, 0x24, 0x0a, 0x0e, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x5f,
    0x65, 0x32, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x73, 0x65, 0x71, 0x4e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x45, 0x32, 0x65, 0x22, 0xc7, 0x04, 0x0a, 0x25, 0x43, 0x4d, 0x73, 0x67,
    0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x47, 0x61, 0x6d,
    0x65, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65,
    0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x65, 0x78, 0x70, 0x69, 0x72, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0a, 0x74, 0x69, 0x6d, 0x65, 0x45, 0x78, 0x70, 0x69,
    0x72, 0x79, 0x12, 0x2e, 0x0a, 0x13, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x64,
    0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x11, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x64, 0x53, 0x74, 0x65, 0x61, 0x6d,
    0x49, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x64,
    0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x69, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x12, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x64, 0x50, 0x75, 0x62, 0x6c,
    0x69, 0x63, 0x49, 0x70, 0x12, 0x2e, 0x0a, 0x13, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x06, 0x52, 0x11, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x65,
    0x61, 0x6d, 0x49, 0x64, 0x12, 0x2a, 0x0a, 0x11, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x5f, 0x6e, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x0f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4e, 0x65, 0x74, 0x49, 0x64,
    0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0c, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x15,
    0x0a, 0x06, 0x61, 0x70, 0x70, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05,
    0x61, 0x70, 0x70, 0x49, 0x64, 0x12, 0x59, 0x0a, 0x0c, 0x65, 0x78, 0x74, 0x72, 0x61, 0x5f, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x36, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x41, 0x75,
    0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x45, 0x78, 0x74, 0x72, 0x61, 0x46, 0x69,
    0x65, 0x6c, 0x64, 0x52, 0x0b, 0x65, 0x78, 0x74, 0x72, 0x61, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73,
    0x1a, 0xae, 0x01, 0x0a, 0x0a, 0x45, 0x78, 0x74, 0x72, 0x61, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x73, 0x74, 0x72, 0x69, 0x6e,
    0x67, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x69, 0x6e, 0x74, 0x33, 0x32, 0x5f,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x11, 0x52, 0x0a, 0x69, 0x6e, 0x74,
    0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x66, 0x69, 0x78, 0x65, 0x64,
    0x33, 0x32, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0d,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x0c, 0x66, 0x69, 0x78, 0x65, 0x64, 0x36, 0x34, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x22, 0x90, 0x02, 0x0a, 0x29, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44,
    0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x43, 0x0a, 0x06, 0x74, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x2b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d,
    0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x41, 0x75, 0x74, 0x68, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x06, 0x74, 0x69,
    0x63, 0x6b, 0x65, 0x74, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67,
    0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0d, 0x63, 0x68,
    0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x63,
    0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09,
    0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x12, 0x34,
    0x0a, 0x16, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x22, 0xd9, 0x01, 0x0a, 0x2d, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65,
    0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x45, 0x73, 0x74, 0x61, 0x62,
    0x6c, 0x69, 0x73, 0x68, 0x65, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x5f, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x12, 0x2e, 0x0a, 0x13, 0x67,
    0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f,
    0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x11, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x34, 0x0a, 0x16, 0x73,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x5f, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x5f, 0x73, 0x68, 0x75,
    0x74, 0x64, 0x6f, 0x77, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x73, 0x65, 0x63,
    0x6f, 0x6e, 0x64, 0x73, 0x55, 0x6e, 0x74, 0x69, 0x6c, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77,
    0x6e, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x64,
    0x22, 0xdc, 0x01, 0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61,
    0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4e, 0x6f, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x23, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f,
    0x6f, 0x6b, 0x69, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x79, 0x6f, 0x75, 0x72, 0x5f, 0x70, 0x75, 0x62,
    0x6c, 0x69, 0x63, 0x5f, 0x69, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x79, 0x6f,
    0x75, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x49, 0x70, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x07, 0x52,
    0x0a, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x63,
    0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09,
    0x63, 0x68, 0x61, 0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x12, 0x34, 0x0a, 0x16, 0x73, 0x65, 0x63,
    0x6f, 0x6e, 0x64, 0x73, 0x5f, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x5f, 0x73, 0x68, 0x75, 0x74, 0x64,
    0x6f, 0x77, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x73, 0x65, 0x63, 0x6f, 0x6e,
    0x64, 0x73, 0x55, 0x6e, 0x74, 0x69, 0x6c, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x22,
    0x4d, 0x0a, 0x1b, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x44, 0x69, 0x61, 0x67, 0x6e, 0x6f, 0x73, 0x74, 0x69, 0x63, 0x12, 0x1a,
    0x0a, 0x08, 0x73, 0x65, 0x76, 0x65, 0x72, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x08, 0x73, 0x65, 0x76, 0x65, 0x72, 0x69, 0x74, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65,
    0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x74, 0x65, 0x78, 0x74, 0x22, 0xab,
    0x02, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x43, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x12, 0x54, 0x0a, 0x0c, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x63, 0x65, 0x6e, 0x74,
    0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72,
    0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x43, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x43, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x0b, 0x64, 0x61,
    0x74, 0x61, 0x43, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x1a, 0x3b, 0x0a, 0x06, 0x53, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x17, 0x0a,
    0x07, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06,
    0x70, 0x69, 0x6e, 0x67, 0x4d, 0x73, 0x1a, 0x74, 0x0a, 0x0a, 0x44, 0x61, 0x74, 0x61, 0x43, 0x65,
    0x6e, 0x74, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x63, 0x6f, 0x64, 0x65, 0x12, 0x52, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x5f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x2d, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d,
    0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x43, 0x65, 0x6e, 0x74,
    0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x0c,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x22, 0x97, 0x03, 0x0a,
    0x27, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72,
    0x61, 0x6d, 0x4c, 0x69, 0x6e, 0x6b, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x61, 0x6e, 0x65,
    0x6f, 0x75, 0x73, 0x53, 0x74, 0x61, 0x74, 0x73, 0x12, 0x34, 0x0a, 0x17, 0x6f, 0x75, 0x74, 0x5f,
    0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x63, 0x5f,
    0x78, 0x31, 0x30, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x13, 0x6f, 0x75, 0x74, 0x50, 0x61,
    0x63, 0x6b, 0x65, 0x74, 0x73, 0x50, 0x65, 0x72, 0x53, 0x65, 0x63, 0x58, 0x31, 0x30, 0x12, 0x29,
    0x0a, 0x11, 0x6f, 0x75, 0x74, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x70, 0x65, 0x72, 0x5f,
    0x73, 0x65, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x6f, 0x75, 0x74, 0x42, 0x79,
    0x74, 0x65, 0x73, 0x50, 0x65, 0x72, 0x53, 0x65, 0x63, 0x12, 0x32, 0x0a, 0x16, 0x69, 0x6e, 0x5f,
    0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x73, 0x65, 0x63, 0x5f,
    0x78, 0x31, 0x30, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x69, 0x6e, 0x50, 0x61, 0x63,
    0x6b, 0x65, 0x74, 0x73, 0x50, 0x65, 0x72, 0x53, 0x65, 0x63, 0x58, 0x31, 0x30, 0x12, 0x27, 0x0a,
    0x10, 0x69, 0x6e, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x73, 0x65,
    0x63, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x69, 0x6e, 0x42, 0x79, 0x74, 0x65, 0x73,
    0x50, 0x65, 0x72, 0x53, 0x65, 0x63, 0x12, 0x17, 0x0a, 0x07, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x6d,
    0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x70, 0x69, 0x6e, 0x67, 0x4d, 0x73, 0x12,
    0x2e, 0x0a, 0x13, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x64, 0x72, 0x6f, 0x70, 0x70,
    0x65, 0x64, 0x5f, 0x70, 0x63, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x70, 0x61,
    0x63, 0x6b, 0x65, 0x74, 0x73, 0x44, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x50, 0x63, 0x74, 0x12,
    0x3b, 0x0a, 0x1a, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x77, 0x65, 0x69, 0x72, 0x64,
    0x5f, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x70, 0x63, 0x74, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x17, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x57, 0x65, 0x69, 0x72,
    0x64, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x50, 0x63, 0x74, 0x12, 0x28, 0x0a, 0x10,
    0x70, 0x65, 0x61, 0x6b, 0x5f, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x5f, 0x75, 0x73, 0x65, 0x63,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x70, 0x65, 0x61, 0x6b, 0x4a, 0x69, 0x74, 0x74,
    0x65, 0x72, 0x55, 0x73, 0x65, 0x63, 0x22, 0xa4, 0x0f, 0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x53,
    0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4c, 0x69, 0x6e, 0x6b,
    0x4c, 0x69, 0x66, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x73, 0x12, 0x21, 0x0a,
    0x0c, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x0b, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x53, 0x65, 0x6e, 0x74,
    0x12, 0x17, 0x0a, 0x07, 0x6b, 0x62, 0x5f, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x06, 0x6b, 0x62, 0x53, 0x65, 0x6e, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x61, 0x63,
    0x6b, 0x65, 0x74, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x76, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0b, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x52, 0x65, 0x63, 0x76, 0x12, 0x17, 0x0a, 0x07,
    0x6b, 0x62, 0x5f, 0x72, 0x65, 0x63, 0x76, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x6b,
    0x62, 0x52, 0x65, 0x63, 0x76, 0x12, 0x34, 0x0a, 0x16, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73,
    0x5f, 0x72, 0x65, 0x63, 0x76, 0x5f, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x64, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x04, 0x52, 0x14, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x52, 0x65,
    0x63, 0x76, 0x53, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x70,
    0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x76, 0x5f, 0x64, 0x72, 0x6f, 0x70,
    0x70, 0x65, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x52, 0x12, 0x70, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x73, 0x52, 0x65, 0x63, 0x76, 0x44, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x12, 0x38, 0x0a,
    0x19, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x76, 0x5f, 0x6f, 0x75,
    0x74, 0x5f, 0x6f, 0x66, 0x5f, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x15, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x52, 0x65, 0x63, 0x76, 0x4f, 0x75, 0x74,
    0x4f, 0x66, 0x4f, 0x72, 0x64, 0x65, 0x72, 0x12, 0x34, 0x0a, 0x16, 0x70, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x76, 0x5f, 0x64, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74,
    0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x04, 0x52, 0x14, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73,
    0x52, 0x65, 0x63, 0x76, 0x44, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x65, 0x12, 0x2c, 0x0a,
    0x12, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x76, 0x5f, 0x6c, 0x75,
    0x72, 0x63, 0x68, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x52, 0x10, 0x70, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x73, 0x52, 0x65, 0x63, 0x76, 0x4c, 0x75, 0x72, 0x63, 0x68, 0x12, 0x32, 0x0a, 0x15, 0x71,
    0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d,
    0x5f, 0x31, 0x30, 0x30, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x13, 0x71, 0x75, 0x61, 0x6c,
    0x69, 0x74, 0x79, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x31, 0x30, 0x30, 0x12,
    0x30, 0x0a, 0x14, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f,
    0x67, 0x72, 0x61, 0x6d, 0x5f, 0x39, 0x39, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x71,
    0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x39,
    0x39, 0x12, 0x30, 0x0a, 0x14, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x69, 0x73,
    0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x39, 0x37, 0x18, 0x17, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x12, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61,
    0x6d, 0x39, 0x37, 0x12, 0x30, 0x0a, 0x14, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x68,
    0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x39, 0x35, 0x18, 0x18, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x12, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67,
    0x72, 0x61, 0x6d, 0x39, 0x35, 0x12, 0x30, 0x0a, 0x14, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79,
    0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x39, 0x30, 0x18, 0x19, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x12, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x48, 0x69, 0x73, 0x74,
    0x6f, 0x67, 0x72, 0x61, 0x6d, 0x39, 0x30, 0x12, 0x30, 0x0a, 0x14, 0x71, 0x75, 0x61, 0x6c, 0x69,
    0x74, 0x79, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x37, 0x35, 0x18,
    0x1a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x48, 0x69,
    0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x37, 0x35, 0x12, 0x30, 0x0a, 0x14, 0x71, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x35,
    0x30, 0x18, 0x1b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79,
    0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x35, 0x30, 0x12, 0x2e, 0x0a, 0x13, 0x71,
    0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d,
    0x5f, 0x31, 0x18, 0x1c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74,
    0x79, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x31, 0x12, 0x34, 0x0a, 0x16, 0x71,
    0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d,
    0x5f, 0x64, 0x65, 0x61, 0x64, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x71, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x44, 0x65, 0x61,
    0x64, 0x12, 0x2a, 0x0a, 0x11, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x6e, 0x74, 0x69,
    0x6c, 0x65, 0x5f, 0x32, 0x6e, 0x64, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x71, 0x75,
    0x61, 0x6c, 0x69, 0x74, 0x79, 0x4e, 0x74, 0x69, 0x6c, 0x65, 0x32, 0x6e, 0x64, 0x12, 0x2a, 0x0a,
    0x11, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x5f, 0x35,
    0x74, 0x68, 0x18, 0x1f, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74,
    0x79, 0x4e, 0x74, 0x69, 0x6c, 0x65, 0x35, 0x74, 0x68, 0x12, 0x2c, 0x0a, 0x12, 0x71, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x5f, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x5f, 0x32, 0x35, 0x74, 0x68, 0x18,
    0x20, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x4e, 0x74,
    0x69, 0x6c, 0x65, 0x32, 0x35, 0x74, 0x68, 0x12, 0x2c, 0x0a, 0x12, 0x71, 0x75, 0x61, 0x6c, 0x69,
    0x74, 0x79, 0x5f, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x5f, 0x35, 0x30, 0x74, 0x68, 0x18, 0x21, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x10, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x4e, 0x74, 0x69, 0x6c,
    0x65, 0x35, 0x30, 0x74, 0x68, 0x12, 0x2a, 0x0a, 0x11, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x69,
    0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x32, 0x35, 0x18, 0x29, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0f, 0x70, 0x69, 0x6e, 0x67, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x32,
    0x35, 0x12, 0x2a, 0x0a, 0x11, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67,
    0x72, 0x61, 0x6d, 0x5f, 0x35, 0x30, 0x18, 0x2a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x70, 0x69,
    0x6e, 0x67, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x35, 0x30, 0x12, 0x2a, 0x0a,
    0x11, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f,
    0x37, 0x35, 0x18, 0x2b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x70, 0x69, 0x6e, 0x67, 0x48, 0x69,
    0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x37, 0x35, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x69, 0x6e,
    0x67, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x31, 0x30, 0x30, 0x18,
    0x2c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x70, 0x69, 0x6e, 0x67, 0x48, 0x69, 0x73, 0x74, 0x6f,
    0x67, 0x72, 0x61, 0x6d, 0x31, 0x30, 0x30, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x69, 0x6e, 0x67, 0x5f,
    0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x31, 0x32, 0x35, 0x18, 0x2d, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x10, 0x70, 0x69, 0x6e, 0x67, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72,
    0x61, 0x6d, 0x31, 0x32, 0x35, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x69,
    0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x31, 0x35, 0x30, 0x18, 0x2e, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x10, 0x70, 0x69, 0x6e, 0x67, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d,
    0x31, 0x35, 0x30, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x69, 0x73, 0x74,
    0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x32, 0x30, 0x30, 0x18, 0x2f, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x10, 0x70, 0x69, 0x6e, 0x67, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x32, 0x30,
    0x30, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67,
    0x72, 0x61, 0x6d, 0x5f, 0x33, 0x30, 0x30, 0x18, 0x30, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x70,
    0x69, 0x6e, 0x67, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x33, 0x30, 0x30, 0x12,
    0x2c, 0x0a, 0x12, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61,
    0x6d, 0x5f, 0x6d, 0x61, 0x78, 0x18, 0x31, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x70, 0x69, 0x6e,
    0x67, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x61, 0x78, 0x12, 0x24, 0x0a,
    0x0e, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x5f, 0x35, 0x74, 0x68, 0x18,
    0x32, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x70, 0x69, 0x6e, 0x67, 0x4e, 0x74, 0x69, 0x6c, 0x65,
    0x35, 0x74, 0x68, 0x12, 0x26, 0x0a, 0x0f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x6e, 0x74, 0x69, 0x6c,
    0x65, 0x5f, 0x35, 0x30, 0x74, 0x68, 0x18, 0x33, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x70, 0x69,
    0x6e, 0x67, 0x4e, 0x74, 0x69, 0x6c, 0x65, 0x35, 0x30, 0x74, 0x68, 0x12, 0x26, 0x0a, 0x0f, 0x70,
    0x69, 0x6e, 0x67, 0x5f, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x5f, 0x37, 0x35, 0x74, 0x68, 0x18, 0x34,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x70, 0x69, 0x6e, 0x67, 0x4e, 0x74, 0x69, 0x6c, 0x65, 0x37,
    0x35, 0x74, 0x68, 0x12, 0x26, 0x0a, 0x0f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x6e, 0x74, 0x69, 0x6c,
    0x65, 0x5f, 0x39, 0x35, 0x74, 0x68, 0x18, 0x35, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x70, 0x69,
    0x6e, 0x67, 0x4e, 0x74, 0x69, 0x6c, 0x65, 0x39, 0x35, 0x74, 0x68, 0x12, 0x26, 0x0a, 0x0f, 0x70,
    0x69, 0x6e, 0x67, 0x5f, 0x6e, 0x74, 0x69, 0x6c, 0x65, 0x5f, 0x39, 0x38, 0x74, 0x68, 0x18, 0x36,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x70, 0x69, 0x6e, 0x67, 0x4e, 0x74, 0x69, 0x6c, 0x65, 0x39,
    0x38, 0x74, 0x68, 0x12, 0x3e, 0x0a, 0x1b, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x5f, 0x68, 0x69,
    0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x6e, 0x65, 0x67, 0x6c, 0x69, 0x67, 0x69, 0x62,
    0x6c, 0x65, 0x18, 0x3d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x19, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72,
    0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x4e, 0x65, 0x67, 0x6c, 0x69, 0x67, 0x69,
    0x62, 0x6c, 0x65, 0x12, 0x2c, 0x0a, 0x12, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x5f, 0x68, 0x69,
    0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x31, 0x18, 0x3e, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x10, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d,
    0x31, 0x12, 0x2c, 0x0a, 0x12, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x5f, 0x68, 0x69, 0x73, 0x74,
    0x6f, 0x67, 0x72, 0x61, 0x6d, 0x5f, 0x32, 0x18, 0x3f, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x6a,
    0x69, 0x74, 0x74, 0x65, 0x72, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x32, 0x12,
    0x2c, 0x0a, 0x12, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67,
    0x72, 0x61, 0x6d, 0x5f, 0x35, 0x18, 0x40, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x6a, 0x69, 0x74,
    0x74, 0x65, 0x72, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x35, 0x12, 0x2e, 0x0a,
    0x13, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61,
    0x6d, 0x5f, 0x31, 0x30, 0x18, 0x41, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x6a, 0x69, 0x74, 0x74,
    0x65, 0x72, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x31, 0x30, 0x12, 0x2e, 0x0a,
    0x13, 0x6a, 0x69, 0x74, 0x74, 0x65, 0x72, 0x5f, 0x68, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61,
    0x6d, 0x5f, 0x32, 0x30, 0x18, 0x42, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x6a, 0x69, 0x74, 0x74,
    0x65, 0x72, 0x48, 0x69, 0x73, 0x74, 0x6f, 0x67, 0x72, 0x61, 0x6d, 0x32, 0x30, 0x22, 0xbf, 0x01,
    0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67,
    0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x12, 0x53, 0x0a, 0x0d, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x61,
    0x6e, 0x65, 0x6f, 0x75, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x4c, 0x69, 0x6e, 0x6b, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x61,
    0x6e, 0x65, 0x6f, 0x75, 0x73, 0x53, 0x74, 0x61, 0x74, 0x73, 0x52, 0x0d, 0x69, 0x6e, 0x73, 0x74,
    0x61, 0x6e, 0x74, 0x61, 0x6e, 0x65, 0x6f, 0x75, 0x73, 0x12, 0x44, 0x0a, 0x08, 0x6c, 0x69, 0x66,
    0x65, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x4c, 0x69, 0x6e, 0x6b, 0x4c, 0x69, 0x66, 0x65, 0x74, 0x69, 0x6d, 0x65,
    0x53, 0x74, 0x61, 0x74, 0x73, 0x52, 0x08, 0x6c, 0x69, 0x66, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x22,
    0xb8, 0x02, 0x0a, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74,
    0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53,
    0x74, 0x61, 0x74, 0x73, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x52, 0x6f, 0x75, 0x74,
    0x65, 0x72, 0x12, 0x3a, 0x0a, 0x03, 0x63, 0x32, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d,
    0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x03, 0x63, 0x32, 0x72, 0x12, 0x3a,
    0x0a, 0x03, 0x63, 0x32, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75,
    0x61, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x03, 0x63, 0x32, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x07, 0x52, 0x0f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x12, 0x1e, 0x0a, 0x0b, 0x73, 0x65,
    0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x63, 0x32, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x09, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x43, 0x32, 0x72, 0x12, 0x1e, 0x0a, 0x0b, 0x73, 0x65,
    0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x63, 0x32, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x09, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x43, 0x32, 0x73, 0x22, 0xa4, 0x05, 0x0a, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d,
    0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x73, 0x52,
    0x6f, 0x75, 0x74, 0x65, 0x72, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x3a, 0x0a,
    0x03, 0x72, 0x32, 0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67,
    0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x52, 0x03, 0x72, 0x32, 0x63, 0x12, 0x3a, 0x0a, 0x03, 0x73, 0x32, 0x63,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79,
    0x52, 0x03, 0x73, 0x32, 0x63, 0x12, 0x3f, 0x0a, 0x1c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x72,
    0x6f, 0x75, 0x74, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x07, 0x52, 0x19, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x46, 0x72, 0x6f, 0x6d,
    0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x12, 0x3f, 0x0a, 0x1c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52, 0x19, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x46, 0x72, 0x6f,
    0x6d, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x12, 0x3a, 0x0a, 0x19, 0x72, 0x6f, 0x75, 0x74, 0x65,
    0x72, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x6c, 0x61, 0x74,
    0x65, 0x6e, 0x63, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x17, 0x72, 0x6f, 0x75, 0x74,
    0x65, 0x72, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4c, 0x61, 0x74, 0x65,
    0x6e, 0x63, 0x79, 0x12, 0x34, 0x0a, 0x16, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x5f, 0x75,
    0x6e, 0x74, 0x69, 0x6c, 0x5f, 0x73, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x14, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x55, 0x6e, 0x74, 0x69,
    0x6c, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x12, 0x2c, 0x0a, 0x12, 0x6d, 0x69, 0x67,
    0x72, 0x61, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x69, 0x70, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x07, 0x52, 0x10, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x49, 0x70, 0x12, 0x30, 0x0a, 0x14, 0x6d, 0x69, 0x67, 0x72, 0x61,
    0x74, 0x65, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x6f, 0x72, 0x74, 0x12, 0x41, 0x0a, 0x1d, 0x73, 0x63, 0x6f,
    0x72, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79, 0x5f, 0x72, 0x65, 0x6c,
    0x61, 0x79, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x1a, 0x73, 0x63, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x50, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79,
    0x52, 0x65, 0x6c, 0x61, 0x79, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x12, 0x23, 0x0a, 0x0d,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x07, 0x52, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6f, 0x6b, 0x69,
    0x65, 0x12, 0x1e, 0x0a, 0x0b, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x72, 0x32, 0x63,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x52, 0x32,
    0x63, 0x12, 0x1e, 0x0a, 0x0b, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x73, 0x32, 0x63,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x53, 0x32,
    0x63, 0x22, 0x92, 0x03, 0x0a, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44,
    0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x53, 0x74, 0x61, 0x74, 0x73, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x54, 0x6f, 0x53, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x12, 0x3a, 0x0a, 0x03, 0x72, 0x32, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65,
    0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x03, 0x72, 0x32, 0x73,
    0x12, 0x3a, 0x0a, 0x03, 0x63, 0x32, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61,
    0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x03, 0x63, 0x32, 0x73, 0x12, 0x29, 0x0a, 0x10,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x29, 0x0a, 0x10, 0x72, 0x6f, 0x75, 0x74, 0x65,
    0x72, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x07, 0x52, 0x0f, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x12, 0x1e, 0x0a, 0x0b, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x72, 0x32,
    0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x52,
    0x32, 0x73, 0x12, 0x1e, 0x0a, 0x0b, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x63, 0x32,
    0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x43,
    0x32, 0x73, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x74, 0x65,
    0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x2a, 0x0a, 0x11, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0xbc, 0x02, 0x0a, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53,
    0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x73, 0x53, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x54, 0x6f, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x12, 0x3a, 0x0a, 0x03, 0x73, 0x32, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d,
    0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79,
    0x52, 0x03, 0x73, 0x32, 0x72, 0x12, 0x3a, 0x0a, 0x03, 0x73, 0x32, 0x63, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74,
    0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x03, 0x73, 0x32,
    0x63, 0x12, 0x1e, 0x0a, 0x0b, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x73, 0x32, 0x72,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x53, 0x32,
    0x72, 0x12, 0x1e, 0x0a, 0x0b, 0x73, 0x65, 0x71, 0x5f, 0x6e, 0x75, 0x6d, 0x5f, 0x73, 0x32, 0x63,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x65, 0x71, 0x4e, 0x75, 0x6d, 0x53, 0x32,
    0x63, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x74, 0x65, 0x61,
    0x6d, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x2a, 0x0a, 0x11, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x4f, 0x0a, 0x28, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65,
    0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x50, 0x69, 0x6e, 0x67, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6f, 0x6b,
    0x69, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x22, 0x9b, 0x02, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x53,
    0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x52, 0x65, 0x70, 0x6c,
    0x79, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6f, 0x6b,
    0x69, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x12, 0x66, 0x0a, 0x10, 0x72, 0x6f, 0x75, 0x74, 0x69, 0x6e,
    0x67, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x3b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61,
    0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x50,
    0x69, 0x6e, 0x67, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x2e, 0x52,
    0x6f, 0x75, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x52, 0x0f, 0x72,
    0x6f, 0x75, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x1a, 0x64,
    0x0a, 0x0e, 0x52, 0x6f, 0x75, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x02, 0x69, 0x64,
    0x12, 0x22, 0x0a, 0x0d, 0x66, 0x72, 0x6f, 0x6e, 0x74, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x6d,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x66, 0x72, 0x6f, 0x6e, 0x74, 0x50, 0x69,
    0x6e, 0x67, 0x4d, 0x73, 0x12, 0x1e, 0x0a, 0x0b, 0x65, 0x32, 0x65, 0x5f, 0x70, 0x69, 0x6e, 0x67,
    0x5f, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x65, 0x32, 0x65, 0x50, 0x69,
    0x6e, 0x67, 0x4d, 0x73, 0x22, 0x83, 0x07, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65,
    0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x53, 0x77, 0x69, 0x74, 0x63, 0x68, 0x65, 0x64, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x12,
    0x23, 0x0a, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f,
    0x6f, 0x6b, 0x69, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x69, 0x70, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x07, 0x52, 0x06, 0x66, 0x72, 0x6f, 0x6d, 0x49, 0x70, 0x12, 0x1b, 0x0a,
    0x09, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x08, 0x66, 0x72, 0x6f, 0x6d, 0x50, 0x6f, 0x72, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x66, 0x72,
    0x6f, 0x6d, 0x5f, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x07, 0x52, 0x11, 0x66, 0x72, 0x6f, 0x6d, 0x52, 0x6f, 0x75,
    0x74, 0x65, 0x72, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x12, 0x28, 0x0a, 0x10, 0x66, 0x72,
    0x6f, 0x6d, 0x5f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x66, 0x72, 0x6f, 0x6d, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65,
    0x54, 0x69, 0x6d, 0x65, 0x12, 0x37, 0x0a, 0x18, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x61, 0x63, 0x74,
    0x69, 0x76, 0x65, 0x5f, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x5f, 0x72, 0x65, 0x63, 0x76,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x15, 0x66, 0x72, 0x6f, 0x6d, 0x41, 0x63, 0x74, 0x69,
    0x76, 0x65, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x52, 0x65, 0x63, 0x76, 0x12, 0x2e, 0x0a,
    0x13, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x5f, 0x72, 0x65,
    0x61, 0x73, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11, 0x66, 0x72, 0x6f, 0x6d,
    0x44, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x52, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x12, 0x15, 0x0a,
    0x06, 0x67, 0x61, 0x70, 0x5f, 0x6d, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x67,
    0x61, 0x70, 0x4d, 0x73, 0x12, 0x64, 0x0a, 0x10, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x71, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x5f, 0x6e, 0x6f, 0x77, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x3a,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44,
    0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x77, 0x69,
    0x74, 0x63, 0x68, 0x65, 0x64, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x2e, 0x52, 0x6f, 0x75,
    0x74, 0x65, 0x72, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x0e, 0x66, 0x72, 0x6f, 0x6d,
    0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x4e, 0x6f, 0x77, 0x12, 0x60, 0x0a, 0x0e, 0x74, 0x6f,
    0x5f, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x6e, 0x6f, 0x77, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x3a, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74,
    0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x53, 0x77, 0x69, 0x74, 0x63, 0x68, 0x65, 0x64, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79,
    0x2e, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x0c,
    0x74, 0x6f, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x4e, 0x6f, 0x77, 0x12, 0x66, 0x0a, 0x11,
    0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x74, 0x68, 0x65,
    0x6e, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x3a, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x77, 0x69, 0x74, 0x63, 0x68, 0x65, 0x64, 0x50, 0x72,
    0x69, 0x6d, 0x61, 0x72, 0x79, 0x2e, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x51, 0x75, 0x61, 0x6c,
    0x69, 0x74, 0x79, 0x52, 0x0f, 0x66, 0x72, 0x6f, 0x6d, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79,
    0x54, 0x68, 0x65, 0x6e, 0x12, 0x62, 0x0a, 0x0f, 0x74, 0x6f, 0x5f, 0x71, 0x75, 0x61, 0x6c, 0x69,
    0x74, 0x79, 0x5f, 0x74, 0x68, 0x65, 0x6e, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x3a, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61,
    0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x77, 0x69, 0x74,
    0x63, 0x68, 0x65, 0x64, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x2e, 0x52, 0x6f, 0x75, 0x74,
    0x65, 0x72, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x52, 0x0d, 0x74, 0x6f, 0x51, 0x75, 0x61,
    0x6c, 0x69, 0x74, 0x79, 0x54, 0x68, 0x65, 0x6e, 0x1a, 0x8f, 0x01, 0x0a, 0x0d, 0x52, 0x6f, 0x75,
    0x74, 0x65, 0x72, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x63,
    0x6f, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x73, 0x63, 0x6f, 0x72, 0x65,
    0x12, 0x1d, 0x0a, 0x0a, 0x66, 0x72, 0x6f, 0x6e, 0x74, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x66, 0x72, 0x6f, 0x6e, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x12,
    0x1b, 0x0a, 0x09, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x08, 0x62, 0x61, 0x63, 0x6b, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x2c, 0x0a, 0x12,
    0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x5f, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x5f, 0x64, 0x6f,
    0x77, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64,
    0x73, 0x55, 0x6e, 0x74, 0x69, 0x6c, 0x44, 0x6f, 0x77, 0x6e, 0x22, 0xef, 0x03, 0x0a, 0x1d, 0x43,
    0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d,
    0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x12, 0x19, 0x0a, 0x08,
    0x63, 0x70, 0x75, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x02, 0x52, 0x07,
    0x63, 0x70, 0x75, 0x4c, 0x6f, 0x61, 0x64, 0x12, 0x27, 0x0a, 0x0f, 0x61, 0x63, 0x74, 0x69, 0x76,
    0x65, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0e, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x12, 0x22, 0x0a, 0x0d, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x70, 0x6b, 0x74, 0x73, 0x5f, 0x73, 0x65,
    0x63, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x64, 0x61, 0x74, 0x61, 0x50, 0x6b, 0x74,
    0x73, 0x53, 0x65, 0x63, 0x12, 0x24, 0x0a, 0x0e, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x5f, 0x70, 0x6b,
    0x74, 0x73, 0x5f, 0x73, 0x65, 0x63, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x6f, 0x74,
    0x68, 0x65, 0x72, 0x50, 0x6b, 0x74, 0x73, 0x53, 0x65, 0x63, 0x12, 0x34, 0x0a, 0x16, 0x73, 0x65,
    0x63, 0x6f, 0x6e, 0x64, 0x73, 0x5f, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x5f, 0x73, 0x68, 0x75, 0x74,
    0x64, 0x6f, 0x77, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x73, 0x65, 0x63, 0x6f,
    0x6e, 0x64, 0x73, 0x55, 0x6e, 0x74, 0x69, 0x6c, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e,
    0x12, 0x29, 0x0a, 0x11, 0x63, 0x70, 0x75, 0x5f, 0x63, 0x6f, 0x73, 0x74, 0x5f, 0x70, 0x65, 0x72,
    0x5f, 0x75, 0x73, 0x65, 0x72, 0x18, 0x08, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0e, 0x63, 0x70, 0x75,
    0x43, 0x6f, 0x73, 0x74, 0x50, 0x65, 0x72, 0x55, 0x73, 0x65, 0x72, 0x12, 0x2d, 0x0a, 0x13, 0x63,
    0x70, 0x75, 0x5f, 0x63, 0x6f, 0x73, 0x74, 0x5f, 0x70, 0x65, 0x72, 0x5f, 0x70, 0x61, 0x63, 0x6b,
    0x65, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x02, 0x52, 0x10, 0x63, 0x70, 0x75, 0x43, 0x6f, 0x73,
    0x74, 0x50, 0x65, 0x72, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x51, 0x0a, 0x0c, 0x64, 0x61,
    0x74, 0x61, 0x5f, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x2e, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x65, 0x61,
    0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x48,
    0x65, 0x61, 0x6c, 0x74, 0x68, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x43, 0x65, 0x6e, 0x74, 0x65, 0x72,
    0x52, 0x0b, 0x64, 0x61, 0x74, 0x61, 0x43, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x73, 0x12, 0x14, 0x0a,
    0x05, 0x6d, 0x61, 0x67, 0x69, 0x63, 0x18, 0x07, 0x20, 0x01, 0x28, 0x06, 0x52, 0x05, 0x6d, 0x61,
    0x67, 0x69, 0x63, 0x1a, 0x47, 0x0a, 0x0a, 0x44, 0x61, 0x74, 0x61, 0x43, 0x65, 0x6e, 0x74, 0x65,
    0x72, 0x12, 0x23, 0x0a, 0x0d, 0x64, 0x61, 0x74, 0x61, 0x63, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0c, 0x64, 0x61, 0x74, 0x61, 0x63, 0x65,
    0x6e, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2a, 0xa0, 0x06, 0x0a,
    0x13, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d,
    0x73, 0x67, 0x49, 0x44, 0x12, 0x1f, 0x0a, 0x1b, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d,
    0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x49, 0x6e, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x10, 0x00, 0x12, 0x29, 0x0a, 0x25, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61,
    0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x6f, 0x75,
    0x74, 0x65, 0x72, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x01,
    0x12, 0x27, 0x0a, 0x23, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x50, 0x69,
    0x6e, 0x67, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x10, 0x02, 0x12, 0x2d, 0x0a, 0x29, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67,
    0x5f, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x50, 0x69, 0x6e, 0x67, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x03, 0x12, 0x2b, 0x0a, 0x27, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f,
    0x47, 0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x65,
    0x70, 0x6c, 0x79, 0x10, 0x04, 0x12, 0x30, 0x0a, 0x2c, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61,
    0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x47, 0x61, 0x6d,
    0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x05, 0x12, 0x34, 0x0a, 0x30, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x47,
    0x61, 0x6d, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x45, 0x73, 0x74, 0x61, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x65, 0x64, 0x10, 0x06, 0x12, 0x21, 0x0a,
    0x1d, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61,
    0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x4e, 0x6f, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x10, 0x07,
    0x12, 0x22, 0x0a, 0x1e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x44, 0x69, 0x61, 0x67, 0x6e, 0x6f, 0x73, 0x74,
    0x69, 0x63, 0x10, 0x08, 0x12, 0x2a, 0x0a, 0x26, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d,
    0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x44, 0x61, 0x74, 0x61,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x10, 0x09,
    0x12, 0x2a, 0x0a, 0x26, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61,
    0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x44, 0x61, 0x74, 0x61, 0x52, 0x6f, 0x75, 0x74,
    0x65, 0x72, 0x54, 0x6f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x10, 0x0a, 0x12, 0x2a, 0x0a, 0x26,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d,
    0x4d, 0x73, 0x67, 0x5f, 0x44, 0x61, 0x74, 0x61, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x54, 0x6f,
    0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x10, 0x0b, 0x12, 0x2a, 0x0a, 0x26, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f,
    0x44, 0x61, 0x74, 0x61, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x10, 0x0c, 0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d,
    0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x53, 0x74, 0x61, 0x74,
    0x73, 0x10, 0x0d, 0x12, 0x2f, 0x0a, 0x2b, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44,
    0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x50, 0x69, 0x6e, 0x67, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x10, 0x0e, 0x12, 0x2d, 0x0a, 0x29, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d,
    0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x53, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x52, 0x65, 0x70, 0x6c,
    0x79, 0x10, 0x0f, 0x12, 0x35, 0x0a, 0x31, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44,
    0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73, 0x67, 0x5f, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x54, 0x6f, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x53, 0x77, 0x69, 0x74, 0x63, 0x68, 0x65,
    0x64, 0x50, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x10, 0x10, 0x12, 0x23, 0x0a, 0x1f, 0x6b, 0x5f,
    0x45, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x4d, 0x73,
    0x67, 0x5f, 0x52, 0x65, 0x6c, 0x61, 0x79, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x10, 0x11, 0x42,
    0x03, 0x80, 0x01, 0x00, 0x4a, 0xf4, 0x6f, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0x8d, 0x02, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x08, 0x0c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x04, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x06, 0x00,
    0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x06, 0x05, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x07, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x08, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08,
    0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x08, 0x30, 0x31,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x09, 0x08, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x09, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x0a, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x0a, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0a,
    0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x08, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x0b, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x05, 0x12, 0x03, 0x0c, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x0c, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12,
    0x03, 0x0c, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0d, 0x08,
    0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x38, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x0d, 0x3b, 0x3c, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07,
    0x02, 0x12, 0x03, 0x0e, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x0f, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0f, 0x08,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x0f, 0x29, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x10, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x10, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x09, 0x02, 0x12, 0x03, 0x10, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a,
    0x12, 0x03, 0x11, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x11, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x11, 0x31,
    0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x12, 0x08, 0x34, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x12, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x12, 0x31, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x0c, 0x12, 0x03, 0x13, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01,
    0x12, 0x03, 0x13, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03,
    0x13, 0x31, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x14, 0x08, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x14, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x14, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x15, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x0e, 0x01, 0x12, 0x03, 0x15, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02,
    0x12, 0x03, 0x15, 0x36, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x16,
    0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x16, 0x08, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x16, 0x34, 0x36, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x10, 0x12, 0x03, 0x17, 0x08, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x17, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x10, 0x02, 0x12, 0x03, 0x17, 0x3c, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x11, 0x12,
    0x03, 0x18, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x18,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x02, 0x12, 0x03, 0x18, 0x2a, 0x2c,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1b, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x1c, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x1c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x19, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x1d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x1d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1d, 0x19, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d,
    0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x34, 0x43,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1d, 0x35,
    0x42, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x1d, 0x35, 0x3b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x1d, 0x35, 0x3b, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x35, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x3e, 0x42, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x08, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x1e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x1e, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x2a,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x1e, 0x2c, 0x3b, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1e, 0x2d, 0x3a,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1e,
    0x2d, 0x33, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x1e, 0x2d, 0x33, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x2d, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x36, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x1f, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1f,
    0x19, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1f, 0x2a, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x20, 0x08, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x20, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x20, 0x19, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x20, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x21,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x21, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x21, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x21, 0x19, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x21, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x06, 0x12, 0x03, 0x22, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x22, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03,
    0x22, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x22, 0x18,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x22, 0x31, 0x32, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x23, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x05, 0x12, 0x03, 0x23, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x23, 0x19, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12,
    0x03, 0x23, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x24, 0x08,
    0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x24, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x24, 0x18, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x24, 0x38, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x09, 0x12, 0x03, 0x25, 0x08, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12,
    0x03, 0x25, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x25,
    0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x25, 0x19, 0x37,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x25, 0x3a, 0x3c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x08, 0x12, 0x03, 0x25, 0x3d, 0x4c, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x00, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x25, 0x3e, 0x4b, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x00, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x25, 0x3e, 0x44, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x00, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x25,
    0x3e, 0x44, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x09, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x25, 0x3e, 0x44, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x09, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x25, 0x47, 0x4b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a,
    0x12, 0x03, 0x26, 0x08, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03,
    0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x26, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x26, 0x18, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x26, 0x31, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0a, 0x08, 0x12, 0x03, 0x26, 0x34, 0x43, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x00, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x26, 0x35, 0x42, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x00, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x26, 0x35, 0x3b, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x00, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x26, 0x35,
    0x3b, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x0a, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x26, 0x35, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x0a, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x26, 0x3e, 0x42, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x29,
    0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x29, 0x08, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x2a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x2a, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x2a, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x08,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2b, 0x11, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2b, 0x19, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2b, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x2c, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2c,
    0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x19, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x2c, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x2d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x2d, 0x19, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x2d, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x2e, 0x08, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x2e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2e, 0x18, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x2e, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05,
    0x12, 0x03, 0x2f, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x2f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x2f, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2f, 0x18, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2f, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x30, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x30, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x30, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x30,
    0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x33, 0x00, 0x44, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x33, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02,
    0x03, 0x00, 0x12, 0x04, 0x34, 0x08, 0x3a, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x34, 0x10, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x35, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x35, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x35, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x35, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x35, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x36, 0x10, 0x31, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x36, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x36, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x36, 0x20, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x36, 0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x37, 0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x37, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x37, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x37, 0x20, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x37, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x12, 0x03, 0x38,
    0x10, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x38,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x38,
    0x19, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x38,
    0x21, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x38,
    0x31, 0x32, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x04, 0x12, 0x03, 0x39, 0x10,
    0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x39, 0x10,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x39, 0x19,
    0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x39, 0x21,
    0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x39, 0x31,
    0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x08, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3c, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x3c, 0x19, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x3c, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x3d, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3d, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x3d, 0x11, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3d, 0x19, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3d, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x3e, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x3e, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3e,
    0x19, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3e, 0x30, 0x31,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3f, 0x08, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3f, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x3f, 0x19, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x3f, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x40,
    0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x40, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x40, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x40, 0x19, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x40, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x41, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x41, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x41, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x41, 0x17,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x41, 0x23, 0x24, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x42, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x42, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x05, 0x12, 0x03, 0x42, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x42, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x42, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x43, 0x08,
    0x53, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03, 0x43, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x06, 0x12, 0x03, 0x43, 0x11, 0x41, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x43, 0x42, 0x4e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x43, 0x51, 0x52, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x46, 0x00, 0x4c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x46, 0x08,
    0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x47, 0x08, 0x42, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x47, 0x11, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x37, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x47, 0x40, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03,
    0x48, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x48, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x48, 0x11, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x19, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x02, 0x12, 0x03, 0x49, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x49, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x49, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x49,
    0x19, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x49, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x08, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4a, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x4a, 0x19, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x4a, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x4b,
    0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x4b, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4b, 0x18, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4b, 0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x4e, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4e,
    0x08, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x08, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4f, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4f, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x19, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x4f, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x50, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x50,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x50, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x50, 0x19, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x50, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x51, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x51, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x51, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x51, 0x18, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x51, 0x31,
    0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x52, 0x08, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x52, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x52, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x52, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x55, 0x00,
    0x5b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x55, 0x08, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x56, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x56, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x56, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x56, 0x19, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x56, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x57, 0x08, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x57, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x57, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x19, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x57, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02,
    0x12, 0x03, 0x58, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x58, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x58, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x58, 0x19, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x58, 0x27, 0x28, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x59, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x59, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x59, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x59, 0x19, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x59,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x5a, 0x08, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12, 0x03, 0x5a, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12, 0x03, 0x5a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x5a, 0x18, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x5a, 0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x5d,
    0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x5e, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x5e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x5e, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x5e, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5f, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5f, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5f, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x62, 0x00, 0x6e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x62, 0x08,
    0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x03, 0x00, 0x12, 0x04, 0x63, 0x08, 0x66, 0x09, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x03, 0x00, 0x01, 0x12, 0x03, 0x63, 0x10, 0x16, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x64, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x64, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x20, 0x27, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x65, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x65, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x65, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x20, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x65, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x07, 0x03, 0x01, 0x12, 0x04, 0x68, 0x08, 0x6b, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x03,
    0x01, 0x01, 0x12, 0x03, 0x68, 0x10, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x69, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x69, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x69, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x69, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x69, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x6a, 0x10, 0x53, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x6a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x01, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x6a, 0x19, 0x40, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x6a, 0x41, 0x4e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x6a, 0x51, 0x52, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x6d,
    0x08, 0x4e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6d, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6d, 0x11, 0x3c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6d, 0x3d, 0x49, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6d, 0x4c, 0x4d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x04, 0x70, 0x00, 0x79, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x70,
    0x08, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x71, 0x08, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x71, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x71, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x71, 0x18, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x71, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12,
    0x03, 0x72, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x72,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x72, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72, 0x18, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x72, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x73, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x73, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x73, 0x18, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x73, 0x31,
    0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x74, 0x08, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x74, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x74, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x74, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x74, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x03,
    0x75, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x03, 0x75, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x03, 0x75, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x75, 0x18, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x75, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x05, 0x12, 0x03, 0x76, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x76, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x76, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x01, 0x12, 0x03, 0x76,
    0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x05, 0x03, 0x12, 0x03, 0x76, 0x2e, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x06, 0x12, 0x03, 0x77, 0x08, 0x37, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x06, 0x04, 0x12, 0x03, 0x77, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x06, 0x05, 0x12, 0x03, 0x77, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x06, 0x01, 0x12, 0x03, 0x77, 0x18, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x77, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x07, 0x12, 0x03, 0x78,
    0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x04, 0x12, 0x03, 0x78, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x05, 0x12, 0x03, 0x78, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x07, 0x01, 0x12, 0x03, 0x78, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x07, 0x03, 0x12, 0x03, 0x78, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x05, 0x7b, 0x00, 0xa6, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03,
    0x7b, 0x08, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x7c, 0x08, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7c, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7c, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x7c, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01,
    0x12, 0x03, 0x7d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x7d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7d, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7d, 0x18, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7d, 0x22, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x7e, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x7e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x7e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x7e, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x03, 0x7e,
    0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x03, 0x7f, 0x08, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7f, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7f, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x7f, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04, 0x12,
    0x04, 0x80, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x80, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x05, 0x12, 0x04, 0x80,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x04, 0x80, 0x01,
    0x18, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x04, 0x80, 0x01, 0x31,
    0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x05, 0x12, 0x04, 0x81, 0x01, 0x08, 0x31, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x04, 0x12, 0x04, 0x81, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x05, 0x12, 0x04, 0x81, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x05, 0x01, 0x12, 0x04, 0x81, 0x01, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x05, 0x03, 0x12, 0x04, 0x81, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x06, 0x12, 0x04, 0x82, 0x01, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x06, 0x04, 0x12, 0x04, 0x82, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06,
    0x05, 0x12, 0x04, 0x82, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x01,
    0x12, 0x04, 0x82, 0x01, 0x18, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x03, 0x12,
    0x04, 0x82, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x07, 0x12, 0x04, 0x83,
    0x01, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x04, 0x12, 0x04, 0x83, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x05, 0x12, 0x04, 0x83, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x01, 0x12, 0x04, 0x83, 0x01, 0x18, 0x2e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x03, 0x12, 0x04, 0x83, 0x01, 0x31, 0x33, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x08, 0x12, 0x04, 0x84, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x08, 0x04, 0x12, 0x04, 0x84, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x08, 0x05, 0x12, 0x04, 0x84, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x08, 0x01, 0x12, 0x04, 0x84, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x08, 0x03, 0x12, 0x04, 0x84, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x09, 0x12, 0x04, 0x85, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x04,
    0x12, 0x04, 0x85, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x05, 0x12,
    0x04, 0x85, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x01, 0x12, 0x04,
    0x85, 0x01, 0x18, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x03, 0x12, 0x04, 0x85,
    0x01, 0x30, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x0a, 0x12, 0x04, 0x86, 0x01, 0x08,
    0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x86, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0a, 0x05, 0x12, 0x04, 0x86, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x86, 0x01, 0x18, 0x2c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x86, 0x01, 0x2f, 0x31, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x0b, 0x12, 0x04, 0x87, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x87, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x0b, 0x05, 0x12, 0x04, 0x87, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x0b, 0x01, 0x12, 0x04, 0x87, 0x01, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0b,
    0x03, 0x12, 0x04, 0x87, 0x01, 0x2f, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x0c, 0x12,
    0x04, 0x88, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0c, 0x04, 0x12, 0x04,
    0x88, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0c, 0x05, 0x12, 0x04, 0x88,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x88, 0x01,
    0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x88, 0x01, 0x2f,
    0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x0d, 0x12, 0x04, 0x89, 0x01, 0x08, 0x32, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x89, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x0d, 0x05, 0x12, 0x04, 0x89, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x89, 0x01, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x0d, 0x03, 0x12, 0x04, 0x89, 0x01, 0x2f, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x0e, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x0e, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0e,
    0x05, 0x12, 0x04, 0x8a, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0e, 0x01,
    0x12, 0x04, 0x8a, 0x01, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0e, 0x03, 0x12,
    0x04, 0x8a, 0x01, 0x2f, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x0f, 0x12, 0x04, 0x8b,
    0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x8b, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0f, 0x05, 0x12, 0x04, 0x8b, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x18, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0f, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x2f, 0x31, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x10, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x10, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x10, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x10, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x18, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x10, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x11, 0x12, 0x04, 0x8d, 0x01, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x11, 0x04,
    0x12, 0x04, 0x8d, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x11, 0x05, 0x12,
    0x04, 0x8d, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x11, 0x01, 0x12, 0x04,
    0x8d, 0x01, 0x18, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x11, 0x03, 0x12, 0x04, 0x8d,
    0x01, 0x31, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x12, 0x12, 0x04, 0x8e, 0x01, 0x08,
    0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x12, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x12, 0x05, 0x12, 0x04, 0x8e, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x12, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x18, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x12, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x2c, 0x2e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x13, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x13, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x13, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x13, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x13,
    0x03, 0x12, 0x04, 0x8f, 0x01, 0x2c, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x14, 0x12,
    0x04, 0x90, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x14, 0x04, 0x12, 0x04,
    0x90, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x14, 0x05, 0x12, 0x04, 0x90,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x14, 0x01, 0x12, 0x04, 0x90, 0x01,
    0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x14, 0x03, 0x12, 0x04, 0x90, 0x01, 0x2d,
    0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x15, 0x12, 0x04, 0x91, 0x01, 0x08, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x15, 0x04, 0x12, 0x04, 0x91, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x15, 0x05, 0x12, 0x04, 0x91, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x15, 0x01, 0x12, 0x04, 0x91, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x15, 0x03, 0x12, 0x04, 0x91, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x16, 0x12, 0x04, 0x92, 0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x16, 0x04, 0x12, 0x04, 0x92, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x16,
    0x05, 0x12, 0x04, 0x92, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x16, 0x01,
    0x12, 0x04, 0x92, 0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x16, 0x03, 0x12,
    0x04, 0x92, 0x01, 0x2c, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x17, 0x12, 0x04, 0x93,
    0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x17, 0x04, 0x12, 0x04, 0x93, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x17, 0x05, 0x12, 0x04, 0x93, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x17, 0x01, 0x12, 0x04, 0x93, 0x01, 0x18, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x17, 0x03, 0x12, 0x04, 0x93, 0x01, 0x2c, 0x2e, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x18, 0x12, 0x04, 0x94, 0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x18, 0x04, 0x12, 0x04, 0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x18, 0x05, 0x12, 0x04, 0x94, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x18, 0x01, 0x12, 0x04, 0x94, 0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x18, 0x03, 0x12, 0x04, 0x94, 0x01, 0x2c, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x19, 0x12, 0x04, 0x95, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x04,
    0x12, 0x04, 0x95, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x05, 0x12,
    0x04, 0x95, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x01, 0x12, 0x04,
    0x95, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x03, 0x12, 0x04, 0x95,
    0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x1a, 0x12, 0x04, 0x96, 0x01, 0x08,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1a, 0x04, 0x12, 0x04, 0x96, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1a, 0x05, 0x12, 0x04, 0x96, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1a, 0x01, 0x12, 0x04, 0x96, 0x01, 0x18, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x1a, 0x03, 0x12, 0x04, 0x96, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x1b, 0x12, 0x04, 0x97, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x1b, 0x04, 0x12, 0x04, 0x97, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x1b, 0x05, 0x12, 0x04, 0x97, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x1b, 0x01, 0x12, 0x04, 0x97, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1b,
    0x03, 0x12, 0x04, 0x97, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x1c, 0x12,
    0x04, 0x98, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1c, 0x04, 0x12, 0x04,
    0x98, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1c, 0x05, 0x12, 0x04, 0x98,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1c, 0x01, 0x12, 0x04, 0x98, 0x01,
    0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1c, 0x03, 0x12, 0x04, 0x98, 0x01, 0x2d,
    0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x1d, 0x12, 0x04, 0x99, 0x01, 0x08, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1d, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x1d, 0x05, 0x12, 0x04, 0x99, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x1d, 0x01, 0x12, 0x04, 0x99, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x1d, 0x03, 0x12, 0x04, 0x99, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x1e, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x1e, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e,
    0x05, 0x12, 0x04, 0x9a, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e, 0x01,
    0x12, 0x04, 0x9a, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e, 0x03, 0x12,
    0x04, 0x9a, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x1f, 0x12, 0x04, 0x9b,
    0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1f, 0x04, 0x12, 0x04, 0x9b, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1f, 0x05, 0x12, 0x04, 0x9b, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1f, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x18, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1f, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x29, 0x2b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x20, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x20, 0x04, 0x12, 0x04, 0x9c, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x20, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x20, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x20, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x21, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x21, 0x04,
    0x12, 0x04, 0x9d, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x21, 0x05, 0x12,
    0x04, 0x9d, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x21, 0x01, 0x12, 0x04,
    0x9d, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x21, 0x03, 0x12, 0x04, 0x9d,
    0x01, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x22, 0x12, 0x04, 0x9e, 0x01, 0x08,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x22, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x22, 0x05, 0x12, 0x04, 0x9e, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x22, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x18, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x22, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x23, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x23, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x23, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x23, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x23,
    0x03, 0x12, 0x04, 0x9f, 0x01, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x24, 0x12,
    0x04, 0xa0, 0x01, 0x08, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x24, 0x04, 0x12, 0x04,
    0xa0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x24, 0x05, 0x12, 0x04, 0xa0,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x24, 0x01, 0x12, 0x04, 0xa0, 0x01,
    0x18, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x24, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x36,
    0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x25, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x25, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x25, 0x05, 0x12, 0x04, 0xa1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x25, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x25, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x26, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x26, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x26,
    0x05, 0x12, 0x04, 0xa2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x26, 0x01,
    0x12, 0x04, 0xa2, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x26, 0x03, 0x12,
    0x04, 0xa2, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x27, 0x12, 0x04, 0xa3,
    0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27, 0x04, 0x12, 0x04, 0xa3, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27, 0x05, 0x12, 0x04, 0xa3, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x18, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x2d, 0x2f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x28, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x28, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x28, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x28, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x18, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x28, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x29, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x29, 0x04,
    0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x29, 0x05, 0x12,
    0x04, 0xa5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x29, 0x01, 0x12, 0x04,
    0xa5, 0x01, 0x18, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x29, 0x03, 0x12, 0x04, 0xa5,
    0x01, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0xa8, 0x01, 0x00, 0xab, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x2a, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x4b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa9, 0x01, 0x11, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x39, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x49, 0x4a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01,
    0x12, 0x04, 0xaa, 0x01, 0x08, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xaa, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xaa, 0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaa,
    0x01, 0x34, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaa, 0x01,
    0x3f, 0x40, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xad, 0x01, 0x00, 0xb4, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xad, 0x01, 0x08, 0x36, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0xae, 0x01, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xae, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xae, 0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xae, 0x01, 0x34, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xae, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12,
    0x04, 0xaf, 0x01, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xaf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x04, 0xaf,
    0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xaf, 0x01,
    0x34, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x3a,
    0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x2e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x19, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x03, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03,
    0x05, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xb1, 0x01, 0x19, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12,
    0x04, 0xb1, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x04, 0xb2,
    0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x04, 0xb2, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x05, 0x12, 0x04, 0xb2, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x18, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x26, 0x27, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x05, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x05, 0x04, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x05, 0x05, 0x12, 0x04, 0xb3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x05, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x05, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12,
    0x06, 0xb6, 0x01, 0x00, 0xc3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04,
    0xb6, 0x01, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x01,
    0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb7, 0x01, 0x11, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x34, 0x37, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x3a, 0x3b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x06, 0x12, 0x04, 0xb8, 0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x34, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xb8, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02,
    0x12, 0x04, 0xb9, 0x01, 0x08, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xb9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xb9, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb9,
    0x01, 0x19, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x01,
    0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x04, 0xba, 0x01, 0x08, 0x3a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x04, 0xba, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x05, 0x12, 0x04, 0xba, 0x01, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x04, 0xba, 0x01, 0x19, 0x35, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x04, 0xba, 0x01, 0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xbb, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xbb, 0x01, 0x18, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xbb, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x05, 0x12, 0x04,
    0xbc, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x04, 0x12, 0x04, 0xbc,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x05, 0x12, 0x04, 0xbc, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x18,
    0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x31, 0x32,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x06, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x31, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x04, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x06, 0x05, 0x12, 0x04, 0xbd, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x06, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x19, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x06, 0x03, 0x12, 0x04, 0xbd, 0x01, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x07, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07,
    0x04, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x05,
    0x12, 0x04, 0xbe, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xbe, 0x01, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x07, 0x03, 0x12, 0x04,
    0xbe, 0x01, 0x2f, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x08, 0x12, 0x04, 0xbf, 0x01,
    0x08, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x04, 0x12, 0x04, 0xbf, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x18, 0x35, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x08, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x38, 0x3a, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x09, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x09, 0x04, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x09, 0x05, 0x12, 0x04, 0xc0, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x09, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x19, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x09, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0a,
    0x12, 0x04, 0xc1, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x04, 0x12,
    0x04, 0xc1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x05, 0x12, 0x04,
    0xc1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xc1,
    0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xc1, 0x01,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x0b, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0b, 0x05, 0x12, 0x04, 0xc2, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xc2, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x06, 0xc5, 0x01, 0x00, 0xce, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12,
    0x04, 0xc6, 0x01, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xc6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc6,
    0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x01,
    0x34, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x3a,
    0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x3c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc7, 0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x34, 0x37, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc7, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x02, 0x12, 0x04, 0xc8, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xc8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xc8, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xc8, 0x01, 0x19, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xc8, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x04, 0xc9,
    0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc9, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x19, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x2c, 0x2d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x04, 0x12, 0x04, 0xca, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x04, 0x04, 0x12, 0x04, 0xca, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x04, 0x05, 0x12, 0x04, 0xca, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x04, 0x01, 0x12, 0x04, 0xca, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xca, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x05, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04,
    0x12, 0x04, 0xcb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x05, 0x12,
    0x04, 0xcb, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xcb, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0xcb,
    0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x06, 0x12, 0x04, 0xcc, 0x01, 0x08,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x04, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x19, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x07, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x07, 0x04, 0x12, 0x04, 0xcd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x07, 0x05, 0x12, 0x04, 0xcd, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x07, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x07,
    0x03, 0x12, 0x04, 0xcd, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xd0,
    0x01, 0x00, 0xd7, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xd0, 0x01,
    0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x3c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd1, 0x01, 0x11, 0x33, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x34, 0x37, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xd2, 0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xd2, 0x01, 0x34, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xd2, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04,
    0xd3, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd3,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd3, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x18,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x03, 0x05, 0x12, 0x04, 0xd4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x05,
    0x12, 0x04, 0xd5, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xd5, 0x01, 0x19, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xd5, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x05, 0x12, 0x04, 0xd6, 0x01,
    0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x05, 0x12, 0x04, 0xd6, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x18, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x2c, 0x2d, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xd9, 0x01, 0x00, 0xdb, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0f, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x00, 0x12, 0x04, 0xda, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xda, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xda, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xda, 0x01, 0x19, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xda,
    0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xdd, 0x01, 0x00, 0xe6, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x2e, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x10, 0x03, 0x00, 0x12, 0x06, 0xde, 0x01, 0x08, 0xe2, 0x01, 0x09, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x03, 0x00, 0x01, 0x12, 0x04, 0xde, 0x01, 0x10, 0x1e, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xdf, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdf, 0x01, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdf, 0x01, 0x19, 0x20, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x21, 0x23,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdf, 0x01, 0x26,
    0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x10,
    0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe0, 0x01,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe0,
    0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xe0, 0x01, 0x20, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xe0, 0x01, 0x30, 0x31, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x02, 0x12,
    0x04, 0xe1, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x04, 0xe1, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xe1, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x20, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02,
    0x00, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xe4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xe4, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xe4, 0x01, 0x19, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe4,
    0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x08,
    0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe5, 0x01, 0x11, 0x46, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x47, 0x57, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe5, 0x01, 0x5a, 0x5b, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x11, 0x12, 0x06, 0xe8, 0x01, 0x00, 0xfc, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x11, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x2e, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x11, 0x03, 0x00,
    0x12, 0x06, 0xe9, 0x01, 0x08, 0xee, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x03, 0x00,
    0x01, 0x12, 0x04, 0xe9, 0x01, 0x10, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xea, 0x01, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xea, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xea, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xea, 0x01, 0x20, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11,
    0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xea, 0x01, 0x28, 0x29, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x11, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x11, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xeb, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x20, 0x2a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x2d, 0x2e, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xec, 0x01, 0x10, 0x2e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xec, 0x01, 0x10, 0x18,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xec, 0x01, 0x19,
    0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xec, 0x01,
    0x20, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xec,
    0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x12, 0x04, 0xed,
    0x01, 0x10, 0x37, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xed, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12,
    0x04, 0xed, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xed, 0x01, 0x20, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x11, 0x03, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xed, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12,
    0x04, 0xf0, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xf0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf0,
    0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf0, 0x01,
    0x19, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf0, 0x01, 0x29,
    0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0xf1, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x19, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x02, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xf2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xf2, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xf2, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x04, 0xf3,
    0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf3, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf3, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x19, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x2f, 0x30, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x04, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x04, 0x05, 0x12, 0x04, 0xf4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf4, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x05, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x04,
    0x12, 0x04, 0xf5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x05, 0x12,
    0x04, 0xf5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xf5, 0x01, 0x18, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf5,
    0x01, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x06, 0x12, 0x04, 0xf6, 0x01, 0x08,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x06, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x06, 0x05, 0x12, 0x04, 0xf6, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x06, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x18, 0x2b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x06, 0x03, 0x12, 0x04, 0xf6, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x11, 0x02, 0x07, 0x12, 0x04, 0xf7, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x07, 0x04, 0x12, 0x04, 0xf7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x07, 0x05, 0x12, 0x04, 0xf7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x07, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x07,
    0x03, 0x12, 0x04, 0xf7, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x08, 0x12,
    0x04, 0xf8, 0x01, 0x08, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x08, 0x04, 0x12, 0x04,
    0xf8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x08, 0x06, 0x12, 0x04, 0xf8,
    0x01, 0x11, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x08, 0x01, 0x12, 0x04, 0xf8, 0x01,
    0x46, 0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x08, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x59,
    0x5a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x09, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x5a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x09, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x09, 0x06, 0x12, 0x04, 0xf9, 0x01, 0x11, 0x45, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x09, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x46, 0x54, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x09, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x57, 0x59, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x0a, 0x12, 0x04, 0xfa, 0x01, 0x08, 0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x0a, 0x04, 0x12, 0x04, 0xfa, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0a,
    0x06, 0x12, 0x04, 0xfa, 0x01, 0x11, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0a, 0x01,
    0x12, 0x04, 0xfa, 0x01, 0x46, 0x57, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0a, 0x03, 0x12,
    0x04, 0xfa, 0x01, 0x5a, 0x5c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x0b, 0x12, 0x04, 0xfb,
    0x01, 0x08, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xfb, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0b, 0x06, 0x12, 0x04, 0xfb, 0x01, 0x11,
    0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x46, 0x55,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x58, 0x5a, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xfe, 0x01, 0x00, 0x8d, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x08, 0x25, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x12,
    0x03, 0x00, 0x12, 0x06, 0xff, 0x01, 0x08, 0x82, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x03, 0x00, 0x01, 0x12, 0x04, 0xff, 0x01, 0x10, 0x1a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x12, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x04, 0x80, 0x02, 0x10, 0x33, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x80, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12,
    0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x80, 0x02, 0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x12, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x02, 0x21, 0x2e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x12, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x02, 0x31, 0x32, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x12, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x81, 0x02, 0x10, 0x2a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x81, 0x02, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x81, 0x02, 0x19, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x81, 0x02, 0x20, 0x25,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x12, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x81, 0x02, 0x28,
    0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0x84, 0x02, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0x84, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0x84, 0x02, 0x17, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0x84, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x12, 0x02, 0x01, 0x12, 0x04, 0x85, 0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x85, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x85, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x85, 0x02, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x85, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0x86,
    0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x04, 0x12, 0x04, 0x86, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x05, 0x12, 0x04, 0x86, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12, 0x04, 0x86, 0x02, 0x18, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04, 0x86, 0x02, 0x28, 0x29, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x03, 0x12, 0x04, 0x87, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x03, 0x04, 0x12, 0x04, 0x87, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x03, 0x05, 0x12, 0x04, 0x87, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x03, 0x01, 0x12, 0x04, 0x87, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x03, 0x03, 0x12, 0x04, 0x87, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02,
    0x04, 0x12, 0x04, 0x88, 0x02, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x88, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x88, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x88, 0x02, 0x18, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x04, 0x03, 0x12, 0x04, 0x88,
    0x02, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x05, 0x12, 0x04, 0x89, 0x02, 0x08,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x05, 0x04, 0x12, 0x04, 0x89, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x05, 0x05, 0x12, 0x04, 0x89, 0x02, 0x11, 0x16, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x05, 0x01, 0x12, 0x04, 0x89, 0x02, 0x17, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x05, 0x03, 0x12, 0x04, 0x89, 0x02, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x12, 0x02, 0x06, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x06, 0x04, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x06, 0x05, 0x12, 0x04, 0x8a, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x06, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x17, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x06,
    0x03, 0x12, 0x04, 0x8a, 0x02, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x07, 0x12,
    0x04, 0x8b, 0x02, 0x08, 0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x07, 0x04, 0x12, 0x04,
    0x8b, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x07, 0x06, 0x12, 0x04, 0x8b,
    0x02, 0x11, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x07, 0x01, 0x12, 0x04, 0x8b, 0x02,
    0x3a, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x07, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x49,
    0x4a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x08, 0x12, 0x04, 0x8c, 0x02, 0x08, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x08, 0x04, 0x12, 0x04, 0x8c, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x08, 0x05, 0x12, 0x04, 0x8c, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x08, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x19, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x08, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x21, 0x22,
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
