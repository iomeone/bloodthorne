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
pub struct CDOTABroadcastMsg {
    // message fields
    field_type: ::std::option::Option<EDotaBroadcastMessages>,
    msg: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcastMsg {}

impl CDOTABroadcastMsg {
    pub fn new() -> CDOTABroadcastMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcastMsg {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcastMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcastMsg,
        };
        unsafe {
            instance.get(CDOTABroadcastMsg::new)
        }
    }

    // required .dota.EDotaBroadcastMessages type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: EDotaBroadcastMessages) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> EDotaBroadcastMessages {
        self.field_type.unwrap_or(EDotaBroadcastMessages::DOTA_BM_LANLobbyRequest)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<EDotaBroadcastMessages> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<EDotaBroadcastMessages> {
        &mut self.field_type
    }

    // optional bytes msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::vec::Vec<u8> {
        self.msg.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_msg(&self) -> &[u8] {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.msg
    }
}

impl ::protobuf::Message for CDOTABroadcastMsg {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.msg)?;
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
        if let Some(v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.msg.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTABroadcastMsg {
    fn new() -> CDOTABroadcastMsg {
        CDOTABroadcastMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcastMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EDotaBroadcastMessages>>(
                    "type",
                    CDOTABroadcastMsg::get_field_type_for_reflect,
                    CDOTABroadcastMsg::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "msg",
                    CDOTABroadcastMsg::get_msg_for_reflect,
                    CDOTABroadcastMsg::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcastMsg>(
                    "CDOTABroadcastMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcastMsg {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcastMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcastMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTABroadcastMsg_LANLobbyRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcastMsg_LANLobbyRequest {}

impl CDOTABroadcastMsg_LANLobbyRequest {
    pub fn new() -> CDOTABroadcastMsg_LANLobbyRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcastMsg_LANLobbyRequest {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcastMsg_LANLobbyRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcastMsg_LANLobbyRequest,
        };
        unsafe {
            instance.get(CDOTABroadcastMsg_LANLobbyRequest::new)
        }
    }
}

impl ::protobuf::Message for CDOTABroadcastMsg_LANLobbyRequest {
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

impl ::protobuf::MessageStatic for CDOTABroadcastMsg_LANLobbyRequest {
    fn new() -> CDOTABroadcastMsg_LANLobbyRequest {
        CDOTABroadcastMsg_LANLobbyRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcastMsg_LANLobbyRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcastMsg_LANLobbyRequest>(
                    "CDOTABroadcastMsg_LANLobbyRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcastMsg_LANLobbyRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcastMsg_LANLobbyRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcastMsg_LANLobbyRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTABroadcastMsg_LANLobbyReply {
    // message fields
    id: ::std::option::Option<u64>,
    tournament_id: ::std::option::Option<u32>,
    tournament_game_id: ::std::option::Option<u32>,
    members: ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>,
    requires_pass_key: ::std::option::Option<bool>,
    leader_account_id: ::std::option::Option<u32>,
    game_mode: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    players: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcastMsg_LANLobbyReply {}

impl CDOTABroadcastMsg_LANLobbyReply {
    pub fn new() -> CDOTABroadcastMsg_LANLobbyReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcastMsg_LANLobbyReply {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcastMsg_LANLobbyReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcastMsg_LANLobbyReply,
        };
        unsafe {
            instance.get(CDOTABroadcastMsg_LANLobbyReply::new)
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional uint32 tournament_id = 2;

    pub fn clear_tournament_id(&mut self) {
        self.tournament_id = ::std::option::Option::None;
    }

    pub fn has_tournament_id(&self) -> bool {
        self.tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_id(&mut self, v: u32) {
        self.tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_id(&self) -> u32 {
        self.tournament_id.unwrap_or(0)
    }

    fn get_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_id
    }

    fn mut_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_id
    }

    // optional uint32 tournament_game_id = 3;

    pub fn clear_tournament_game_id(&mut self) {
        self.tournament_game_id = ::std::option::Option::None;
    }

    pub fn has_tournament_game_id(&self) -> bool {
        self.tournament_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_game_id(&mut self, v: u32) {
        self.tournament_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_game_id(&self) -> u32 {
        self.tournament_game_id.unwrap_or(0)
    }

    fn get_tournament_game_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tournament_game_id
    }

    fn mut_tournament_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tournament_game_id
    }

    // repeated .dota.CDOTABroadcastMsg_LANLobbyReply.CLobbyMember members = 4;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CDOTABroadcastMsg_LANLobbyReply_CLobbyMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> {
        &mut self.members
    }

    // optional bool requires_pass_key = 5;

    pub fn clear_requires_pass_key(&mut self) {
        self.requires_pass_key = ::std::option::Option::None;
    }

    pub fn has_requires_pass_key(&self) -> bool {
        self.requires_pass_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requires_pass_key(&mut self, v: bool) {
        self.requires_pass_key = ::std::option::Option::Some(v);
    }

    pub fn get_requires_pass_key(&self) -> bool {
        self.requires_pass_key.unwrap_or(false)
    }

    fn get_requires_pass_key_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.requires_pass_key
    }

    fn mut_requires_pass_key_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.requires_pass_key
    }

    // optional uint32 leader_account_id = 6;

    pub fn clear_leader_account_id(&mut self) {
        self.leader_account_id = ::std::option::Option::None;
    }

    pub fn has_leader_account_id(&self) -> bool {
        self.leader_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_account_id(&mut self, v: u32) {
        self.leader_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_leader_account_id(&self) -> u32 {
        self.leader_account_id.unwrap_or(0)
    }

    fn get_leader_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.leader_account_id
    }

    fn mut_leader_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.leader_account_id
    }

    // optional uint32 game_mode = 7;

    pub fn clear_game_mode(&mut self) {
        self.game_mode = ::std::option::Option::None;
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: u32) {
        self.game_mode = ::std::option::Option::Some(v);
    }

    pub fn get_game_mode(&self) -> u32 {
        self.game_mode.unwrap_or(0)
    }

    fn get_game_mode_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_mode
    }

    fn mut_game_mode_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_mode
    }

    // optional string name = 8;

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

    // optional uint32 players = 9;

    pub fn clear_players(&mut self) {
        self.players = ::std::option::Option::None;
    }

    pub fn has_players(&self) -> bool {
        self.players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: u32) {
        self.players = ::std::option::Option::Some(v);
    }

    pub fn get_players(&self) -> u32 {
        self.players.unwrap_or(0)
    }

    fn get_players_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.players
    }
}

impl ::protobuf::Message for CDOTABroadcastMsg_LANLobbyReply {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tournament_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tournament_game_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.requires_pass_key = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.leader_account_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.players = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tournament_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tournament_game_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.requires_pass_key {
            my_size += 2;
        };
        if let Some(v) = self.leader_account_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        };
        if let Some(v) = self.players {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.tournament_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.tournament_game_id {
            os.write_uint32(3, v)?;
        };
        for v in &self.members {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.requires_pass_key {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.leader_account_id {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.game_mode {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(8, &v)?;
        };
        if let Some(v) = self.players {
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

impl ::protobuf::MessageStatic for CDOTABroadcastMsg_LANLobbyReply {
    fn new() -> CDOTABroadcastMsg_LANLobbyReply {
        CDOTABroadcastMsg_LANLobbyReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcastMsg_LANLobbyReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CDOTABroadcastMsg_LANLobbyReply::get_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_id",
                    CDOTABroadcastMsg_LANLobbyReply::get_tournament_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_game_id",
                    CDOTABroadcastMsg_LANLobbyReply::get_tournament_game_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_tournament_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>>(
                    "members",
                    CDOTABroadcastMsg_LANLobbyReply::get_members_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "requires_pass_key",
                    CDOTABroadcastMsg_LANLobbyReply::get_requires_pass_key_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_requires_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leader_account_id",
                    CDOTABroadcastMsg_LANLobbyReply::get_leader_account_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_leader_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_mode",
                    CDOTABroadcastMsg_LANLobbyReply::get_game_mode_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CDOTABroadcastMsg_LANLobbyReply::get_name_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players",
                    CDOTABroadcastMsg_LANLobbyReply::get_players_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply::mut_players_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcastMsg_LANLobbyReply>(
                    "CDOTABroadcastMsg_LANLobbyReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcastMsg_LANLobbyReply {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_tournament_id();
        self.clear_tournament_game_id();
        self.clear_members();
        self.clear_requires_pass_key();
        self.clear_leader_account_id();
        self.clear_game_mode();
        self.clear_name();
        self.clear_players();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcastMsg_LANLobbyReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcastMsg_LANLobbyReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    // message fields
    account_id: ::std::option::Option<u32>,
    player_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {}

impl CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    pub fn new() -> CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcastMsg_LANLobbyReply_CLobbyMember,
        };
        unsafe {
            instance.get(CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::new)
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

    // optional string player_name = 2;

    pub fn clear_player_name(&mut self) {
        self.player_name.clear();
    }

    pub fn has_player_name(&self) -> bool {
        self.player_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_name(&mut self, v: ::std::string::String) {
        self.player_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_name(&mut self) -> &mut ::std::string::String {
        if self.player_name.is_none() {
            self.player_name.set_default();
        };
        self.player_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_name(&mut self) -> ::std::string::String {
        self.player_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_player_name(&self) -> &str {
        match self.player_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_player_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.player_name
    }

    fn mut_player_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.player_name
    }
}

impl ::protobuf::Message for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.player_name)?;
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
        if let Some(v) = self.player_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.player_name.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    fn new() -> CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
        CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::get_account_id_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "player_name",
                    CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::get_player_name_for_reflect,
                    CDOTABroadcastMsg_LANLobbyReply_CLobbyMember::mut_player_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcastMsg_LANLobbyReply_CLobbyMember>(
                    "CDOTABroadcastMsg_LANLobbyReply_CLobbyMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_player_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcastMsg_LANLobbyReply_CLobbyMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDotaBroadcastMessages {
    DOTA_BM_LANLobbyRequest = 1,
    DOTA_BM_LANLobbyReply = 2,
}

impl ::protobuf::ProtobufEnum for EDotaBroadcastMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDotaBroadcastMessages> {
        match value {
            1 => ::std::option::Option::Some(EDotaBroadcastMessages::DOTA_BM_LANLobbyRequest),
            2 => ::std::option::Option::Some(EDotaBroadcastMessages::DOTA_BM_LANLobbyReply),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDotaBroadcastMessages] = &[
            EDotaBroadcastMessages::DOTA_BM_LANLobbyRequest,
            EDotaBroadcastMessages::DOTA_BM_LANLobbyReply,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EDotaBroadcastMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDotaBroadcastMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDotaBroadcastMessages {
}

impl ::protobuf::reflect::ProtobufValue for EDotaBroadcastMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04,
    0x64, 0x6f, 0x74, 0x61, 0x22, 0x70, 0x0a, 0x11, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x42, 0x72, 0x6f,
    0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x12, 0x49, 0x0a, 0x04, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45,
    0x44, 0x6f, 0x74, 0x61, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x3a, 0x17, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x42, 0x4d, 0x5f, 0x4c,
    0x41, 0x4e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x04,
    0x74, 0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x03, 0x6d, 0x73, 0x67, 0x22, 0x23, 0x0a, 0x21, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x42,
    0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x41, 0x4e, 0x4c,
    0x6f, 0x62, 0x62, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0xc5, 0x03, 0x0a, 0x1f,
    0x43, 0x44, 0x4f, 0x54, 0x41, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73,
    0x67, 0x5f, 0x4c, 0x41, 0x4e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12,
    0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12,
    0x23, 0x0a, 0x0d, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x12, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65,
    0x6e, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x10, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x47, 0x61, 0x6d, 0x65,
    0x49, 0x64, 0x12, 0x4c, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x73, 0x67, 0x5f, 0x4c, 0x41, 0x4e,
    0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x2e, 0x43, 0x4c, 0x6f, 0x62, 0x62,
    0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x12, 0x2a, 0x0a, 0x11, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x5f, 0x70, 0x61, 0x73,
    0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x72, 0x65, 0x71,
    0x75, 0x69, 0x72, 0x65, 0x73, 0x50, 0x61, 0x73, 0x73, 0x4b, 0x65, 0x79, 0x12, 0x2a, 0x0a, 0x11,
    0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x41,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65,
    0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x67, 0x61, 0x6d,
    0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x70, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x73, 0x1a, 0x4e, 0x0a, 0x0c, 0x43, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4e,
    0x61, 0x6d, 0x65, 0x2a, 0x50, 0x0a, 0x16, 0x45, 0x44, 0x6f, 0x74, 0x61, 0x42, 0x72, 0x6f, 0x61,
    0x64, 0x63, 0x61, 0x73, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x1b, 0x0a,
    0x17, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x42, 0x4d, 0x5f, 0x4c, 0x41, 0x4e, 0x4c, 0x6f, 0x62, 0x62,
    0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x10, 0x01, 0x12, 0x19, 0x0a, 0x15, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x42, 0x4d, 0x5f, 0x4c, 0x41, 0x4e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52, 0x65,
    0x70, 0x6c, 0x79, 0x10, 0x02, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xbf, 0x0a, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x23, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x04, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x04,
    0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x04, 0x07, 0x13,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x07, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x04, 0x16, 0x1b, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12,
    0x03, 0x05, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x05,
    0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x05, 0x07,
    0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x07,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x05, 0x1d, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x00, 0x01, 0x12, 0x03, 0x07, 0x05, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x08, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x08, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x09, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x0c, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08,
    0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x08, 0x55, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x11, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x28, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x0d, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12,
    0x03, 0x0d, 0x31, 0x54, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x07, 0x12, 0x03, 0x0d,
    0x3c, 0x53, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x17, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0e, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x11,
    0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x11, 0x08, 0x29, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x14, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x00, 0x12,
    0x04, 0x15, 0x08, 0x18, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x00, 0x01, 0x12, 0x03,
    0x15, 0x10, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16,
    0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x16,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16,
    0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16,
    0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16,
    0x2d, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x17, 0x10,
    0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x17, 0x10,
    0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x19,
    0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x20,
    0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x2e,
    0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x08, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x18, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1b, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x18, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1b, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x1c, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x1c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x1c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c,
    0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x2d, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x08, 0x4a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1d, 0x11, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1d, 0x3e, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x1d, 0x48, 0x49, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1e,
    0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1e, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1e, 0x16, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1e, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x1f, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x1f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1f, 0x18,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1f, 0x2c, 0x2d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x20, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x05, 0x12, 0x03, 0x20, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x20, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x20, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x21, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03, 0x21, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x21, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x21, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x21, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x08, 0x12, 0x03, 0x22, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x04, 0x12,
    0x03, 0x22, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x05, 0x12, 0x03, 0x22,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03, 0x22, 0x18, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03, 0x22, 0x22, 0x23,
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
