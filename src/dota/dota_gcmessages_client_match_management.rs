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
pub struct CMsgStartFindingMatch {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    matchgroups: ::std::option::Option<u32>,
    client_version: ::std::option::Option<u32>,
    game_modes: ::std::option::Option<u32>,
    bot_difficulty: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    match_type: ::std::option::Option<super::dota_shared_enums::MatchType>,
    matchlanguages: ::std::option::Option<u32>,
    map_preference: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    game_language_enum: ::std::option::Option<super::dota_shared_enums::MatchLanguages>,
    game_language_name: ::protobuf::SingularField<::std::string::String>,
    ping_data: ::protobuf::SingularPtrField<super::base_gcmessages::CMsgClientPingData>,
    region_select_flags: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgStartFindingMatch {}

impl CMsgStartFindingMatch {
    pub fn new() -> CMsgStartFindingMatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgStartFindingMatch {
        static mut instance: ::protobuf::lazy::Lazy<CMsgStartFindingMatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgStartFindingMatch,
        };
        unsafe {
            instance.get(CMsgStartFindingMatch::new)
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.key
    }

    // optional uint32 matchgroups = 2;

    pub fn clear_matchgroups(&mut self) {
        self.matchgroups = ::std::option::Option::None;
    }

    pub fn has_matchgroups(&self) -> bool {
        self.matchgroups.is_some()
    }

    // Param is passed by value, moved
    pub fn set_matchgroups(&mut self, v: u32) {
        self.matchgroups = ::std::option::Option::Some(v);
    }

    pub fn get_matchgroups(&self) -> u32 {
        self.matchgroups.unwrap_or(4294967295u32)
    }

    fn get_matchgroups_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.matchgroups
    }

    fn mut_matchgroups_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.matchgroups
    }

    // optional uint32 client_version = 3;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional uint32 game_modes = 4;

    pub fn clear_game_modes(&mut self) {
        self.game_modes = ::std::option::Option::None;
    }

    pub fn has_game_modes(&self) -> bool {
        self.game_modes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_modes(&mut self, v: u32) {
        self.game_modes = ::std::option::Option::Some(v);
    }

    pub fn get_game_modes(&self) -> u32 {
        self.game_modes.unwrap_or(4294967295u32)
    }

    fn get_game_modes_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_modes
    }

    fn mut_game_modes_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_modes
    }

    // optional .dota.DOTABotDifficulty bot_difficulty = 5;

    pub fn clear_bot_difficulty(&mut self) {
        self.bot_difficulty = ::std::option::Option::None;
    }

    pub fn has_bot_difficulty(&self) -> bool {
        self.bot_difficulty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bot_difficulty(&mut self, v: super::dota_shared_enums::DOTABotDifficulty) {
        self.bot_difficulty = ::std::option::Option::Some(v);
    }

    pub fn get_bot_difficulty(&self) -> super::dota_shared_enums::DOTABotDifficulty {
        self.bot_difficulty.unwrap_or(super::dota_shared_enums::DOTABotDifficulty::BOT_DIFFICULTY_HARD)
    }

    fn get_bot_difficulty_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &self.bot_difficulty
    }

    fn mut_bot_difficulty_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &mut self.bot_difficulty
    }

    // optional .dota.MatchType match_type = 6;

    pub fn clear_match_type(&mut self) {
        self.match_type = ::std::option::Option::None;
    }

    pub fn has_match_type(&self) -> bool {
        self.match_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_type(&mut self, v: super::dota_shared_enums::MatchType) {
        self.match_type = ::std::option::Option::Some(v);
    }

    pub fn get_match_type(&self) -> super::dota_shared_enums::MatchType {
        self.match_type.unwrap_or(super::dota_shared_enums::MatchType::MATCH_TYPE_CASUAL)
    }

    fn get_match_type_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::MatchType> {
        &self.match_type
    }

    fn mut_match_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::MatchType> {
        &mut self.match_type
    }

    // optional uint32 matchlanguages = 7;

    pub fn clear_matchlanguages(&mut self) {
        self.matchlanguages = ::std::option::Option::None;
    }

    pub fn has_matchlanguages(&self) -> bool {
        self.matchlanguages.is_some()
    }

    // Param is passed by value, moved
    pub fn set_matchlanguages(&mut self, v: u32) {
        self.matchlanguages = ::std::option::Option::Some(v);
    }

    pub fn get_matchlanguages(&self) -> u32 {
        self.matchlanguages.unwrap_or(4294967295u32)
    }

    fn get_matchlanguages_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.matchlanguages
    }

    fn mut_matchlanguages_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.matchlanguages
    }

    // optional uint32 map_preference = 9;

    pub fn clear_map_preference(&mut self) {
        self.map_preference = ::std::option::Option::None;
    }

    pub fn has_map_preference(&self) -> bool {
        self.map_preference.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_preference(&mut self, v: u32) {
        self.map_preference = ::std::option::Option::Some(v);
    }

    pub fn get_map_preference(&self) -> u32 {
        self.map_preference.unwrap_or(0)
    }

    fn get_map_preference_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.map_preference
    }

    fn mut_map_preference_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.map_preference
    }

    // optional uint32 team_id = 8;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }

    // optional .dota.MatchLanguages game_language_enum = 10;

    pub fn clear_game_language_enum(&mut self) {
        self.game_language_enum = ::std::option::Option::None;
    }

    pub fn has_game_language_enum(&self) -> bool {
        self.game_language_enum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_language_enum(&mut self, v: super::dota_shared_enums::MatchLanguages) {
        self.game_language_enum = ::std::option::Option::Some(v);
    }

    pub fn get_game_language_enum(&self) -> super::dota_shared_enums::MatchLanguages {
        self.game_language_enum.unwrap_or(super::dota_shared_enums::MatchLanguages::MATCH_LANGUAGE_INVALID)
    }

    fn get_game_language_enum_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::MatchLanguages> {
        &self.game_language_enum
    }

    fn mut_game_language_enum_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::MatchLanguages> {
        &mut self.game_language_enum
    }

    // optional string game_language_name = 11;

    pub fn clear_game_language_name(&mut self) {
        self.game_language_name.clear();
    }

    pub fn has_game_language_name(&self) -> bool {
        self.game_language_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_language_name(&mut self, v: ::std::string::String) {
        self.game_language_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_language_name(&mut self) -> &mut ::std::string::String {
        if self.game_language_name.is_none() {
            self.game_language_name.set_default();
        };
        self.game_language_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_language_name(&mut self) -> ::std::string::String {
        self.game_language_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_language_name(&self) -> &str {
        match self.game_language_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_language_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_language_name
    }

    fn mut_game_language_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_language_name
    }

    // optional .dota.CMsgClientPingData ping_data = 12;

    pub fn clear_ping_data(&mut self) {
        self.ping_data.clear();
    }

    pub fn has_ping_data(&self) -> bool {
        self.ping_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_data(&mut self, v: super::base_gcmessages::CMsgClientPingData) {
        self.ping_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_data(&mut self) -> &mut super::base_gcmessages::CMsgClientPingData {
        if self.ping_data.is_none() {
            self.ping_data.set_default();
        };
        self.ping_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_data(&mut self) -> super::base_gcmessages::CMsgClientPingData {
        self.ping_data.take().unwrap_or_else(|| super::base_gcmessages::CMsgClientPingData::new())
    }

    pub fn get_ping_data(&self) -> &super::base_gcmessages::CMsgClientPingData {
        self.ping_data.as_ref().unwrap_or_else(|| super::base_gcmessages::CMsgClientPingData::default_instance())
    }

    fn get_ping_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base_gcmessages::CMsgClientPingData> {
        &self.ping_data
    }

    fn mut_ping_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base_gcmessages::CMsgClientPingData> {
        &mut self.ping_data
    }

    // optional uint32 region_select_flags = 13;

    pub fn clear_region_select_flags(&mut self) {
        self.region_select_flags = ::std::option::Option::None;
    }

    pub fn has_region_select_flags(&self) -> bool {
        self.region_select_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_select_flags(&mut self, v: u32) {
        self.region_select_flags = ::std::option::Option::Some(v);
    }

    pub fn get_region_select_flags(&self) -> u32 {
        self.region_select_flags.unwrap_or(0)
    }

    fn get_region_select_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.region_select_flags
    }

    fn mut_region_select_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.region_select_flags
    }
}

impl ::protobuf::Message for CMsgStartFindingMatch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.matchgroups = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_modes = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.bot_difficulty = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.match_type = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.matchlanguages = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.map_preference = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_language_enum = ::std::option::Option::Some(tmp);
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_language_name)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_data)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.region_select_flags = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.matchgroups {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_modes {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.bot_difficulty {
            my_size += ::protobuf::rt::enum_size(5, v);
        };
        if let Some(v) = self.match_type {
            my_size += ::protobuf::rt::enum_size(6, v);
        };
        if let Some(v) = self.matchlanguages {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.map_preference {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_language_enum {
            my_size += ::protobuf::rt::enum_size(10, v);
        };
        if let Some(v) = self.game_language_name.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        };
        if let Some(v) = self.ping_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.region_select_flags {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.matchgroups {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.game_modes {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.bot_difficulty {
            os.write_enum(5, v.value())?;
        };
        if let Some(v) = self.match_type {
            os.write_enum(6, v.value())?;
        };
        if let Some(v) = self.matchlanguages {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.map_preference {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.team_id {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.game_language_enum {
            os.write_enum(10, v.value())?;
        };
        if let Some(v) = self.game_language_name.as_ref() {
            os.write_string(11, &v)?;
        };
        if let Some(v) = self.ping_data.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.region_select_flags {
            os.write_uint32(13, v)?;
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

impl ::protobuf::MessageStatic for CMsgStartFindingMatch {
    fn new() -> CMsgStartFindingMatch {
        CMsgStartFindingMatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgStartFindingMatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    CMsgStartFindingMatch::get_key_for_reflect,
                    CMsgStartFindingMatch::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "matchgroups",
                    CMsgStartFindingMatch::get_matchgroups_for_reflect,
                    CMsgStartFindingMatch::mut_matchgroups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgStartFindingMatch::get_client_version_for_reflect,
                    CMsgStartFindingMatch::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_modes",
                    CMsgStartFindingMatch::get_game_modes_for_reflect,
                    CMsgStartFindingMatch::mut_game_modes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "bot_difficulty",
                    CMsgStartFindingMatch::get_bot_difficulty_for_reflect,
                    CMsgStartFindingMatch::mut_bot_difficulty_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::MatchType>>(
                    "match_type",
                    CMsgStartFindingMatch::get_match_type_for_reflect,
                    CMsgStartFindingMatch::mut_match_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "matchlanguages",
                    CMsgStartFindingMatch::get_matchlanguages_for_reflect,
                    CMsgStartFindingMatch::mut_matchlanguages_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "map_preference",
                    CMsgStartFindingMatch::get_map_preference_for_reflect,
                    CMsgStartFindingMatch::mut_map_preference_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgStartFindingMatch::get_team_id_for_reflect,
                    CMsgStartFindingMatch::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::MatchLanguages>>(
                    "game_language_enum",
                    CMsgStartFindingMatch::get_game_language_enum_for_reflect,
                    CMsgStartFindingMatch::mut_game_language_enum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_language_name",
                    CMsgStartFindingMatch::get_game_language_name_for_reflect,
                    CMsgStartFindingMatch::mut_game_language_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base_gcmessages::CMsgClientPingData>>(
                    "ping_data",
                    CMsgStartFindingMatch::get_ping_data_for_reflect,
                    CMsgStartFindingMatch::mut_ping_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region_select_flags",
                    CMsgStartFindingMatch::get_region_select_flags_for_reflect,
                    CMsgStartFindingMatch::mut_region_select_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgStartFindingMatch>(
                    "CMsgStartFindingMatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgStartFindingMatch {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_matchgroups();
        self.clear_client_version();
        self.clear_game_modes();
        self.clear_bot_difficulty();
        self.clear_match_type();
        self.clear_matchlanguages();
        self.clear_map_preference();
        self.clear_team_id();
        self.clear_game_language_enum();
        self.clear_game_language_name();
        self.clear_ping_data();
        self.clear_region_select_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgStartFindingMatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgStartFindingMatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgStartFindingMatchResult {
    // message fields
    legacy_generic_eresult: ::std::option::Option<u32>,
    result: ::std::option::Option<EStartFindingMatchResult>,
    error_token: ::protobuf::SingularField<::std::string::String>,
    debug_message: ::protobuf::SingularField<::std::string::String>,
    responsible_party_members: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgStartFindingMatchResult {}

impl CMsgStartFindingMatchResult {
    pub fn new() -> CMsgStartFindingMatchResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgStartFindingMatchResult {
        static mut instance: ::protobuf::lazy::Lazy<CMsgStartFindingMatchResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgStartFindingMatchResult,
        };
        unsafe {
            instance.get(CMsgStartFindingMatchResult::new)
        }
    }

    // optional uint32 legacy_generic_eresult = 1;

    pub fn clear_legacy_generic_eresult(&mut self) {
        self.legacy_generic_eresult = ::std::option::Option::None;
    }

    pub fn has_legacy_generic_eresult(&self) -> bool {
        self.legacy_generic_eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_generic_eresult(&mut self, v: u32) {
        self.legacy_generic_eresult = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_generic_eresult(&self) -> u32 {
        self.legacy_generic_eresult.unwrap_or(2u32)
    }

    fn get_legacy_generic_eresult_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_generic_eresult
    }

    fn mut_legacy_generic_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_generic_eresult
    }

    // optional .dota.EStartFindingMatchResult result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: EStartFindingMatchResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> EStartFindingMatchResult {
        self.result.unwrap_or(EStartFindingMatchResult::k_EStartFindingMatchResult_Invalid)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<EStartFindingMatchResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<EStartFindingMatchResult> {
        &mut self.result
    }

    // optional string error_token = 3;

    pub fn clear_error_token(&mut self) {
        self.error_token.clear();
    }

    pub fn has_error_token(&self) -> bool {
        self.error_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_token(&mut self, v: ::std::string::String) {
        self.error_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_token(&mut self) -> &mut ::std::string::String {
        if self.error_token.is_none() {
            self.error_token.set_default();
        };
        self.error_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_token(&mut self) -> ::std::string::String {
        self.error_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_token(&self) -> &str {
        match self.error_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_token
    }

    fn mut_error_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_token
    }

    // optional string debug_message = 4;

    pub fn clear_debug_message(&mut self) {
        self.debug_message.clear();
    }

    pub fn has_debug_message(&self) -> bool {
        self.debug_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_debug_message(&mut self, v: ::std::string::String) {
        self.debug_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_debug_message(&mut self) -> &mut ::std::string::String {
        if self.debug_message.is_none() {
            self.debug_message.set_default();
        };
        self.debug_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_debug_message(&mut self) -> ::std::string::String {
        self.debug_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_debug_message(&self) -> &str {
        match self.debug_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_debug_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.debug_message
    }

    fn mut_debug_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.debug_message
    }

    // repeated fixed64 responsible_party_members = 5;

    pub fn clear_responsible_party_members(&mut self) {
        self.responsible_party_members.clear();
    }

    // Param is passed by value, moved
    pub fn set_responsible_party_members(&mut self, v: ::std::vec::Vec<u64>) {
        self.responsible_party_members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responsible_party_members(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.responsible_party_members
    }

    // Take field
    pub fn take_responsible_party_members(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.responsible_party_members, ::std::vec::Vec::new())
    }

    pub fn get_responsible_party_members(&self) -> &[u64] {
        &self.responsible_party_members
    }

    fn get_responsible_party_members_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.responsible_party_members
    }

    fn mut_responsible_party_members_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.responsible_party_members
    }
}

impl ::protobuf::Message for CMsgStartFindingMatchResult {
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
                    self.legacy_generic_eresult = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_token)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.debug_message)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.responsible_party_members)?;
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
        if let Some(v) = self.legacy_generic_eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.error_token.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.debug_message.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += 9 * self.responsible_party_members.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.legacy_generic_eresult {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.result {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.error_token.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.debug_message.as_ref() {
            os.write_string(4, &v)?;
        };
        for v in &self.responsible_party_members {
            os.write_fixed64(5, *v)?;
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

impl ::protobuf::MessageStatic for CMsgStartFindingMatchResult {
    fn new() -> CMsgStartFindingMatchResult {
        CMsgStartFindingMatchResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgStartFindingMatchResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "legacy_generic_eresult",
                    CMsgStartFindingMatchResult::get_legacy_generic_eresult_for_reflect,
                    CMsgStartFindingMatchResult::mut_legacy_generic_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EStartFindingMatchResult>>(
                    "result",
                    CMsgStartFindingMatchResult::get_result_for_reflect,
                    CMsgStartFindingMatchResult::mut_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_token",
                    CMsgStartFindingMatchResult::get_error_token_for_reflect,
                    CMsgStartFindingMatchResult::mut_error_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "debug_message",
                    CMsgStartFindingMatchResult::get_debug_message_for_reflect,
                    CMsgStartFindingMatchResult::mut_debug_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "responsible_party_members",
                    CMsgStartFindingMatchResult::get_responsible_party_members_for_reflect,
                    CMsgStartFindingMatchResult::mut_responsible_party_members_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgStartFindingMatchResult>(
                    "CMsgStartFindingMatchResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgStartFindingMatchResult {
    fn clear(&mut self) {
        self.clear_legacy_generic_eresult();
        self.clear_result();
        self.clear_error_token();
        self.clear_debug_message();
        self.clear_responsible_party_members();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgStartFindingMatchResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgStartFindingMatchResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgStopFindingMatch {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgStopFindingMatch {}

impl CMsgStopFindingMatch {
    pub fn new() -> CMsgStopFindingMatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgStopFindingMatch {
        static mut instance: ::protobuf::lazy::Lazy<CMsgStopFindingMatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgStopFindingMatch,
        };
        unsafe {
            instance.get(CMsgStopFindingMatch::new)
        }
    }
}

impl ::protobuf::Message for CMsgStopFindingMatch {
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

impl ::protobuf::MessageStatic for CMsgStopFindingMatch {
    fn new() -> CMsgStopFindingMatch {
        CMsgStopFindingMatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgStopFindingMatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgStopFindingMatch>(
                    "CMsgStopFindingMatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgStopFindingMatch {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgStopFindingMatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgStopFindingMatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPartyBuilderOptions {
    // message fields
    additional_slots: ::std::option::Option<u32>,
    match_type: ::std::option::Option<super::dota_shared_enums::MatchType>,
    matchgroups: ::std::option::Option<u32>,
    client_version: ::std::option::Option<u32>,
    language: ::std::option::Option<super::dota_shared_enums::MatchLanguages>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPartyBuilderOptions {}

impl CMsgPartyBuilderOptions {
    pub fn new() -> CMsgPartyBuilderOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPartyBuilderOptions {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPartyBuilderOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPartyBuilderOptions,
        };
        unsafe {
            instance.get(CMsgPartyBuilderOptions::new)
        }
    }

    // optional uint32 additional_slots = 1;

    pub fn clear_additional_slots(&mut self) {
        self.additional_slots = ::std::option::Option::None;
    }

    pub fn has_additional_slots(&self) -> bool {
        self.additional_slots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_additional_slots(&mut self, v: u32) {
        self.additional_slots = ::std::option::Option::Some(v);
    }

    pub fn get_additional_slots(&self) -> u32 {
        self.additional_slots.unwrap_or(0)
    }

    fn get_additional_slots_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.additional_slots
    }

    fn mut_additional_slots_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.additional_slots
    }

    // optional .dota.MatchType match_type = 2;

    pub fn clear_match_type(&mut self) {
        self.match_type = ::std::option::Option::None;
    }

    pub fn has_match_type(&self) -> bool {
        self.match_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_type(&mut self, v: super::dota_shared_enums::MatchType) {
        self.match_type = ::std::option::Option::Some(v);
    }

    pub fn get_match_type(&self) -> super::dota_shared_enums::MatchType {
        self.match_type.unwrap_or(super::dota_shared_enums::MatchType::MATCH_TYPE_CASUAL)
    }

    fn get_match_type_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::MatchType> {
        &self.match_type
    }

    fn mut_match_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::MatchType> {
        &mut self.match_type
    }

    // optional uint32 matchgroups = 3;

    pub fn clear_matchgroups(&mut self) {
        self.matchgroups = ::std::option::Option::None;
    }

    pub fn has_matchgroups(&self) -> bool {
        self.matchgroups.is_some()
    }

    // Param is passed by value, moved
    pub fn set_matchgroups(&mut self, v: u32) {
        self.matchgroups = ::std::option::Option::Some(v);
    }

    pub fn get_matchgroups(&self) -> u32 {
        self.matchgroups.unwrap_or(0)
    }

    fn get_matchgroups_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.matchgroups
    }

    fn mut_matchgroups_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.matchgroups
    }

    // optional uint32 client_version = 4;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional .dota.MatchLanguages language = 5;

    pub fn clear_language(&mut self) {
        self.language = ::std::option::Option::None;
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: super::dota_shared_enums::MatchLanguages) {
        self.language = ::std::option::Option::Some(v);
    }

    pub fn get_language(&self) -> super::dota_shared_enums::MatchLanguages {
        self.language.unwrap_or(super::dota_shared_enums::MatchLanguages::MATCH_LANGUAGE_INVALID)
    }

    fn get_language_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::MatchLanguages> {
        &self.language
    }

    fn mut_language_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::MatchLanguages> {
        &mut self.language
    }
}

impl ::protobuf::Message for CMsgPartyBuilderOptions {
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
                    self.additional_slots = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.match_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.matchgroups = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.language = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.additional_slots {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.match_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.matchgroups {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.language {
            my_size += ::protobuf::rt::enum_size(5, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.additional_slots {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.match_type {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.matchgroups {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.language {
            os.write_enum(5, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgPartyBuilderOptions {
    fn new() -> CMsgPartyBuilderOptions {
        CMsgPartyBuilderOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPartyBuilderOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "additional_slots",
                    CMsgPartyBuilderOptions::get_additional_slots_for_reflect,
                    CMsgPartyBuilderOptions::mut_additional_slots_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::MatchType>>(
                    "match_type",
                    CMsgPartyBuilderOptions::get_match_type_for_reflect,
                    CMsgPartyBuilderOptions::mut_match_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "matchgroups",
                    CMsgPartyBuilderOptions::get_matchgroups_for_reflect,
                    CMsgPartyBuilderOptions::mut_matchgroups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgPartyBuilderOptions::get_client_version_for_reflect,
                    CMsgPartyBuilderOptions::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::MatchLanguages>>(
                    "language",
                    CMsgPartyBuilderOptions::get_language_for_reflect,
                    CMsgPartyBuilderOptions::mut_language_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPartyBuilderOptions>(
                    "CMsgPartyBuilderOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPartyBuilderOptions {
    fn clear(&mut self) {
        self.clear_additional_slots();
        self.clear_match_type();
        self.clear_matchgroups();
        self.clear_client_version();
        self.clear_language();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPartyBuilderOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPartyBuilderOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgReadyUp {
    // message fields
    state: ::std::option::Option<super::dota_shared_enums::DOTALobbyReadyState>,
    ready_up_key: ::std::option::Option<u64>,
    hardware_specs: ::protobuf::SingularPtrField<super::dota_shared_enums::CDOTAClientHardwareSpecs>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgReadyUp {}

impl CMsgReadyUp {
    pub fn new() -> CMsgReadyUp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgReadyUp {
        static mut instance: ::protobuf::lazy::Lazy<CMsgReadyUp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgReadyUp,
        };
        unsafe {
            instance.get(CMsgReadyUp::new)
        }
    }

    // optional .dota.DOTALobbyReadyState state = 1;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: super::dota_shared_enums::DOTALobbyReadyState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> super::dota_shared_enums::DOTALobbyReadyState {
        self.state.unwrap_or(super::dota_shared_enums::DOTALobbyReadyState::DOTALobbyReadyState_UNDECLARED)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTALobbyReadyState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTALobbyReadyState> {
        &mut self.state
    }

    // optional fixed64 ready_up_key = 2;

    pub fn clear_ready_up_key(&mut self) {
        self.ready_up_key = ::std::option::Option::None;
    }

    pub fn has_ready_up_key(&self) -> bool {
        self.ready_up_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ready_up_key(&mut self, v: u64) {
        self.ready_up_key = ::std::option::Option::Some(v);
    }

    pub fn get_ready_up_key(&self) -> u64 {
        self.ready_up_key.unwrap_or(0)
    }

    fn get_ready_up_key_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ready_up_key
    }

    fn mut_ready_up_key_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ready_up_key
    }

    // optional .dota.CDOTAClientHardwareSpecs hardware_specs = 3;

    pub fn clear_hardware_specs(&mut self) {
        self.hardware_specs.clear();
    }

    pub fn has_hardware_specs(&self) -> bool {
        self.hardware_specs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hardware_specs(&mut self, v: super::dota_shared_enums::CDOTAClientHardwareSpecs) {
        self.hardware_specs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hardware_specs(&mut self) -> &mut super::dota_shared_enums::CDOTAClientHardwareSpecs {
        if self.hardware_specs.is_none() {
            self.hardware_specs.set_default();
        };
        self.hardware_specs.as_mut().unwrap()
    }

    // Take field
    pub fn take_hardware_specs(&mut self) -> super::dota_shared_enums::CDOTAClientHardwareSpecs {
        self.hardware_specs.take().unwrap_or_else(|| super::dota_shared_enums::CDOTAClientHardwareSpecs::new())
    }

    pub fn get_hardware_specs(&self) -> &super::dota_shared_enums::CDOTAClientHardwareSpecs {
        self.hardware_specs.as_ref().unwrap_or_else(|| super::dota_shared_enums::CDOTAClientHardwareSpecs::default_instance())
    }

    fn get_hardware_specs_for_reflect(&self) -> &::protobuf::SingularPtrField<super::dota_shared_enums::CDOTAClientHardwareSpecs> {
        &self.hardware_specs
    }

    fn mut_hardware_specs_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::dota_shared_enums::CDOTAClientHardwareSpecs> {
        &mut self.hardware_specs
    }
}

impl ::protobuf::Message for CMsgReadyUp {
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
                    self.state = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.ready_up_key = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hardware_specs)?;
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
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.ready_up_key {
            my_size += 9;
        };
        if let Some(v) = self.hardware_specs.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.state {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.ready_up_key {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.hardware_specs.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgReadyUp {
    fn new() -> CMsgReadyUp {
        CMsgReadyUp::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgReadyUp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTALobbyReadyState>>(
                    "state",
                    CMsgReadyUp::get_state_for_reflect,
                    CMsgReadyUp::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "ready_up_key",
                    CMsgReadyUp::get_ready_up_key_for_reflect,
                    CMsgReadyUp::mut_ready_up_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_shared_enums::CDOTAClientHardwareSpecs>>(
                    "hardware_specs",
                    CMsgReadyUp::get_hardware_specs_for_reflect,
                    CMsgReadyUp::mut_hardware_specs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgReadyUp>(
                    "CMsgReadyUp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgReadyUp {
    fn clear(&mut self) {
        self.clear_state();
        self.clear_ready_up_key();
        self.clear_hardware_specs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgReadyUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgReadyUp {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgReadyUpStatus {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    accepted_ids: ::std::vec::Vec<u32>,
    declined_ids: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgReadyUpStatus {}

impl CMsgReadyUpStatus {
    pub fn new() -> CMsgReadyUpStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgReadyUpStatus {
        static mut instance: ::protobuf::lazy::Lazy<CMsgReadyUpStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgReadyUpStatus,
        };
        unsafe {
            instance.get(CMsgReadyUpStatus::new)
        }
    }

    // optional fixed64 lobby_id = 1;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }

    // repeated uint32 accepted_ids = 2;

    pub fn clear_accepted_ids(&mut self) {
        self.accepted_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_accepted_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.accepted_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_accepted_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.accepted_ids
    }

    // Take field
    pub fn take_accepted_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.accepted_ids, ::std::vec::Vec::new())
    }

    pub fn get_accepted_ids(&self) -> &[u32] {
        &self.accepted_ids
    }

    fn get_accepted_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.accepted_ids
    }

    fn mut_accepted_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.accepted_ids
    }

    // repeated uint32 declined_ids = 3;

    pub fn clear_declined_ids(&mut self) {
        self.declined_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_declined_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.declined_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_declined_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.declined_ids
    }

    // Take field
    pub fn take_declined_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.declined_ids, ::std::vec::Vec::new())
    }

    pub fn get_declined_ids(&self) -> &[u32] {
        &self.declined_ids
    }

    fn get_declined_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.declined_ids
    }

    fn mut_declined_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.declined_ids
    }
}

impl ::protobuf::Message for CMsgReadyUpStatus {
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
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.accepted_ids)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.declined_ids)?;
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
        if let Some(v) = self.lobby_id {
            my_size += 9;
        };
        for value in &self.accepted_ids {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.declined_ids {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_fixed64(1, v)?;
        };
        for v in &self.accepted_ids {
            os.write_uint32(2, *v)?;
        };
        for v in &self.declined_ids {
            os.write_uint32(3, *v)?;
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

impl ::protobuf::MessageStatic for CMsgReadyUpStatus {
    fn new() -> CMsgReadyUpStatus {
        CMsgReadyUpStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgReadyUpStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "lobby_id",
                    CMsgReadyUpStatus::get_lobby_id_for_reflect,
                    CMsgReadyUpStatus::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "accepted_ids",
                    CMsgReadyUpStatus::get_accepted_ids_for_reflect,
                    CMsgReadyUpStatus::mut_accepted_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "declined_ids",
                    CMsgReadyUpStatus::get_declined_ids_for_reflect,
                    CMsgReadyUpStatus::mut_declined_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgReadyUpStatus>(
                    "CMsgReadyUpStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgReadyUpStatus {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.clear_accepted_ids();
        self.clear_declined_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgReadyUpStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgReadyUpStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAbandonCurrentGame {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAbandonCurrentGame {}

impl CMsgAbandonCurrentGame {
    pub fn new() -> CMsgAbandonCurrentGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAbandonCurrentGame {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAbandonCurrentGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAbandonCurrentGame,
        };
        unsafe {
            instance.get(CMsgAbandonCurrentGame::new)
        }
    }
}

impl ::protobuf::Message for CMsgAbandonCurrentGame {
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

impl ::protobuf::MessageStatic for CMsgAbandonCurrentGame {
    fn new() -> CMsgAbandonCurrentGame {
        CMsgAbandonCurrentGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAbandonCurrentGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAbandonCurrentGame>(
                    "CMsgAbandonCurrentGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAbandonCurrentGame {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAbandonCurrentGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAbandonCurrentGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbySetDetails {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    game_name: ::protobuf::SingularField<::std::string::String>,
    team_details: ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTeamDetails>,
    server_region: ::std::option::Option<u32>,
    game_mode: ::std::option::Option<u32>,
    cm_pick: ::std::option::Option<super::dota_shared_enums::DOTA_CM_PICK>,
    bot_difficulty_radiant: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    allow_cheats: ::std::option::Option<bool>,
    fill_with_bots: ::std::option::Option<bool>,
    intro_mode: ::std::option::Option<bool>,
    allow_spectating: ::std::option::Option<bool>,
    game_version: ::std::option::Option<super::dota_shared_enums::DOTAGameVersion>,
    pass_key: ::protobuf::SingularField<::std::string::String>,
    leagueid: ::std::option::Option<u32>,
    penalty_level_radiant: ::std::option::Option<u32>,
    penalty_level_dire: ::std::option::Option<u32>,
    load_game_id: ::std::option::Option<u32>,
    series_type: ::std::option::Option<u32>,
    radiant_series_wins: ::std::option::Option<u32>,
    dire_series_wins: ::std::option::Option<u32>,
    allchat: ::std::option::Option<bool>,
    dota_tv_delay: ::std::option::Option<super::dota_gcmessages_common_match_management::LobbyDotaTVDelay>,
    lan: ::std::option::Option<bool>,
    custom_game_mode: ::protobuf::SingularField<::std::string::String>,
    custom_map_name: ::protobuf::SingularField<::std::string::String>,
    custom_difficulty: ::std::option::Option<u32>,
    custom_game_id: ::std::option::Option<u64>,
    custom_min_players: ::std::option::Option<u32>,
    custom_max_players: ::std::option::Option<u32>,
    lan_host_ping_to_server_region: ::std::option::Option<u32>,
    visibility: ::std::option::Option<super::dota_shared_enums::DOTALobbyVisibility>,
    custom_game_crc: ::std::option::Option<u64>,
    league_series_id: ::std::option::Option<u32>,
    league_game_id: ::std::option::Option<u32>,
    custom_game_timestamp: ::std::option::Option<u32>,
    previous_match_override: ::std::option::Option<u64>,
    league_selection_priority_team: ::std::option::Option<u32>,
    league_selection_priority_choice: ::std::option::Option<super::dota_shared_enums::SelectionPriorityType>,
    league_non_selection_priority_choice: ::std::option::Option<super::dota_shared_enums::SelectionPriorityType>,
    pause_setting: ::std::option::Option<super::dota_gcmessages_common_match_management::LobbyDotaPauseSetting>,
    bot_difficulty_dire: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    bot_radiant: ::std::option::Option<u64>,
    bot_dire: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbySetDetails {}

impl CMsgPracticeLobbySetDetails {
    pub fn new() -> CMsgPracticeLobbySetDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbySetDetails {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbySetDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbySetDetails,
        };
        unsafe {
            instance.get(CMsgPracticeLobbySetDetails::new)
        }
    }

    // optional uint64 lobby_id = 1;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }

    // optional string game_name = 2;

    pub fn clear_game_name(&mut self) {
        self.game_name.clear();
    }

    pub fn has_game_name(&self) -> bool {
        self.game_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_name(&mut self, v: ::std::string::String) {
        self.game_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_name(&mut self) -> &mut ::std::string::String {
        if self.game_name.is_none() {
            self.game_name.set_default();
        };
        self.game_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_name(&mut self) -> ::std::string::String {
        self.game_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_name(&self) -> &str {
        match self.game_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_name
    }

    fn mut_game_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_name
    }

    // repeated .dota.CLobbyTeamDetails team_details = 3;

    pub fn clear_team_details(&mut self) {
        self.team_details.clear();
    }

    // Param is passed by value, moved
    pub fn set_team_details(&mut self, v: ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTeamDetails>) {
        self.team_details = v;
    }

    // Mutable pointer to the field.
    pub fn mut_team_details(&mut self) -> &mut ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTeamDetails> {
        &mut self.team_details
    }

    // Take field
    pub fn take_team_details(&mut self) -> ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTeamDetails> {
        ::std::mem::replace(&mut self.team_details, ::protobuf::RepeatedField::new())
    }

    pub fn get_team_details(&self) -> &[super::dota_gcmessages_common_match_management::CLobbyTeamDetails] {
        &self.team_details
    }

    fn get_team_details_for_reflect(&self) -> &::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTeamDetails> {
        &self.team_details
    }

    fn mut_team_details_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTeamDetails> {
        &mut self.team_details
    }

    // optional uint32 server_region = 4;

    pub fn clear_server_region(&mut self) {
        self.server_region = ::std::option::Option::None;
    }

    pub fn has_server_region(&self) -> bool {
        self.server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_region(&mut self, v: u32) {
        self.server_region = ::std::option::Option::Some(v);
    }

    pub fn get_server_region(&self) -> u32 {
        self.server_region.unwrap_or(0)
    }

    fn get_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_region
    }

    fn mut_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_region
    }

    // optional uint32 game_mode = 5;

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

    // optional .dota.DOTA_CM_PICK cm_pick = 6;

    pub fn clear_cm_pick(&mut self) {
        self.cm_pick = ::std::option::Option::None;
    }

    pub fn has_cm_pick(&self) -> bool {
        self.cm_pick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cm_pick(&mut self, v: super::dota_shared_enums::DOTA_CM_PICK) {
        self.cm_pick = ::std::option::Option::Some(v);
    }

    pub fn get_cm_pick(&self) -> super::dota_shared_enums::DOTA_CM_PICK {
        self.cm_pick.unwrap_or(super::dota_shared_enums::DOTA_CM_PICK::DOTA_CM_RANDOM)
    }

    fn get_cm_pick_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_CM_PICK> {
        &self.cm_pick
    }

    fn mut_cm_pick_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_CM_PICK> {
        &mut self.cm_pick
    }

    // optional .dota.DOTABotDifficulty bot_difficulty_radiant = 9;

    pub fn clear_bot_difficulty_radiant(&mut self) {
        self.bot_difficulty_radiant = ::std::option::Option::None;
    }

    pub fn has_bot_difficulty_radiant(&self) -> bool {
        self.bot_difficulty_radiant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bot_difficulty_radiant(&mut self, v: super::dota_shared_enums::DOTABotDifficulty) {
        self.bot_difficulty_radiant = ::std::option::Option::Some(v);
    }

    pub fn get_bot_difficulty_radiant(&self) -> super::dota_shared_enums::DOTABotDifficulty {
        self.bot_difficulty_radiant.unwrap_or(super::dota_shared_enums::DOTABotDifficulty::BOT_DIFFICULTY_PASSIVE)
    }

    fn get_bot_difficulty_radiant_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &self.bot_difficulty_radiant
    }

    fn mut_bot_difficulty_radiant_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &mut self.bot_difficulty_radiant
    }

    // optional bool allow_cheats = 10;

    pub fn clear_allow_cheats(&mut self) {
        self.allow_cheats = ::std::option::Option::None;
    }

    pub fn has_allow_cheats(&self) -> bool {
        self.allow_cheats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_cheats(&mut self, v: bool) {
        self.allow_cheats = ::std::option::Option::Some(v);
    }

    pub fn get_allow_cheats(&self) -> bool {
        self.allow_cheats.unwrap_or(false)
    }

    fn get_allow_cheats_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_cheats
    }

    fn mut_allow_cheats_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_cheats
    }

    // optional bool fill_with_bots = 11;

    pub fn clear_fill_with_bots(&mut self) {
        self.fill_with_bots = ::std::option::Option::None;
    }

    pub fn has_fill_with_bots(&self) -> bool {
        self.fill_with_bots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fill_with_bots(&mut self, v: bool) {
        self.fill_with_bots = ::std::option::Option::Some(v);
    }

    pub fn get_fill_with_bots(&self) -> bool {
        self.fill_with_bots.unwrap_or(false)
    }

    fn get_fill_with_bots_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.fill_with_bots
    }

    fn mut_fill_with_bots_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.fill_with_bots
    }

    // optional bool intro_mode = 12;

    pub fn clear_intro_mode(&mut self) {
        self.intro_mode = ::std::option::Option::None;
    }

    pub fn has_intro_mode(&self) -> bool {
        self.intro_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_intro_mode(&mut self, v: bool) {
        self.intro_mode = ::std::option::Option::Some(v);
    }

    pub fn get_intro_mode(&self) -> bool {
        self.intro_mode.unwrap_or(false)
    }

    fn get_intro_mode_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.intro_mode
    }

    fn mut_intro_mode_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.intro_mode
    }

    // optional bool allow_spectating = 13;

    pub fn clear_allow_spectating(&mut self) {
        self.allow_spectating = ::std::option::Option::None;
    }

    pub fn has_allow_spectating(&self) -> bool {
        self.allow_spectating.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_spectating(&mut self, v: bool) {
        self.allow_spectating = ::std::option::Option::Some(v);
    }

    pub fn get_allow_spectating(&self) -> bool {
        self.allow_spectating.unwrap_or(false)
    }

    fn get_allow_spectating_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_spectating
    }

    fn mut_allow_spectating_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_spectating
    }

    // optional .dota.DOTAGameVersion game_version = 14;

    pub fn clear_game_version(&mut self) {
        self.game_version = ::std::option::Option::None;
    }

    pub fn has_game_version(&self) -> bool {
        self.game_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_version(&mut self, v: super::dota_shared_enums::DOTAGameVersion) {
        self.game_version = ::std::option::Option::Some(v);
    }

    pub fn get_game_version(&self) -> super::dota_shared_enums::DOTAGameVersion {
        self.game_version.unwrap_or(super::dota_shared_enums::DOTAGameVersion::GAME_VERSION_CURRENT)
    }

    fn get_game_version_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTAGameVersion> {
        &self.game_version
    }

    fn mut_game_version_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTAGameVersion> {
        &mut self.game_version
    }

    // optional string pass_key = 15;

    pub fn clear_pass_key(&mut self) {
        self.pass_key.clear();
    }

    pub fn has_pass_key(&self) -> bool {
        self.pass_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pass_key(&mut self, v: ::std::string::String) {
        self.pass_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pass_key(&mut self) -> &mut ::std::string::String {
        if self.pass_key.is_none() {
            self.pass_key.set_default();
        };
        self.pass_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_pass_key(&mut self) -> ::std::string::String {
        self.pass_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pass_key(&self) -> &str {
        match self.pass_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_pass_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.pass_key
    }

    fn mut_pass_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.pass_key
    }

    // optional uint32 leagueid = 16;

    pub fn clear_leagueid(&mut self) {
        self.leagueid = ::std::option::Option::None;
    }

    pub fn has_leagueid(&self) -> bool {
        self.leagueid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leagueid(&mut self, v: u32) {
        self.leagueid = ::std::option::Option::Some(v);
    }

    pub fn get_leagueid(&self) -> u32 {
        self.leagueid.unwrap_or(0)
    }

    fn get_leagueid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.leagueid
    }

    fn mut_leagueid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.leagueid
    }

    // optional uint32 penalty_level_radiant = 17;

    pub fn clear_penalty_level_radiant(&mut self) {
        self.penalty_level_radiant = ::std::option::Option::None;
    }

    pub fn has_penalty_level_radiant(&self) -> bool {
        self.penalty_level_radiant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_penalty_level_radiant(&mut self, v: u32) {
        self.penalty_level_radiant = ::std::option::Option::Some(v);
    }

    pub fn get_penalty_level_radiant(&self) -> u32 {
        self.penalty_level_radiant.unwrap_or(0)
    }

    fn get_penalty_level_radiant_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.penalty_level_radiant
    }

    fn mut_penalty_level_radiant_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.penalty_level_radiant
    }

    // optional uint32 penalty_level_dire = 18;

    pub fn clear_penalty_level_dire(&mut self) {
        self.penalty_level_dire = ::std::option::Option::None;
    }

    pub fn has_penalty_level_dire(&self) -> bool {
        self.penalty_level_dire.is_some()
    }

    // Param is passed by value, moved
    pub fn set_penalty_level_dire(&mut self, v: u32) {
        self.penalty_level_dire = ::std::option::Option::Some(v);
    }

    pub fn get_penalty_level_dire(&self) -> u32 {
        self.penalty_level_dire.unwrap_or(0)
    }

    fn get_penalty_level_dire_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.penalty_level_dire
    }

    fn mut_penalty_level_dire_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.penalty_level_dire
    }

    // optional uint32 load_game_id = 19;

    pub fn clear_load_game_id(&mut self) {
        self.load_game_id = ::std::option::Option::None;
    }

    pub fn has_load_game_id(&self) -> bool {
        self.load_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_load_game_id(&mut self, v: u32) {
        self.load_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_load_game_id(&self) -> u32 {
        self.load_game_id.unwrap_or(0)
    }

    fn get_load_game_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.load_game_id
    }

    fn mut_load_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.load_game_id
    }

    // optional uint32 series_type = 20;

    pub fn clear_series_type(&mut self) {
        self.series_type = ::std::option::Option::None;
    }

    pub fn has_series_type(&self) -> bool {
        self.series_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_series_type(&mut self, v: u32) {
        self.series_type = ::std::option::Option::Some(v);
    }

    pub fn get_series_type(&self) -> u32 {
        self.series_type.unwrap_or(0)
    }

    fn get_series_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.series_type
    }

    fn mut_series_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.series_type
    }

    // optional uint32 radiant_series_wins = 21;

    pub fn clear_radiant_series_wins(&mut self) {
        self.radiant_series_wins = ::std::option::Option::None;
    }

    pub fn has_radiant_series_wins(&self) -> bool {
        self.radiant_series_wins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_series_wins(&mut self, v: u32) {
        self.radiant_series_wins = ::std::option::Option::Some(v);
    }

    pub fn get_radiant_series_wins(&self) -> u32 {
        self.radiant_series_wins.unwrap_or(0)
    }

    fn get_radiant_series_wins_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.radiant_series_wins
    }

    fn mut_radiant_series_wins_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.radiant_series_wins
    }

    // optional uint32 dire_series_wins = 22;

    pub fn clear_dire_series_wins(&mut self) {
        self.dire_series_wins = ::std::option::Option::None;
    }

    pub fn has_dire_series_wins(&self) -> bool {
        self.dire_series_wins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dire_series_wins(&mut self, v: u32) {
        self.dire_series_wins = ::std::option::Option::Some(v);
    }

    pub fn get_dire_series_wins(&self) -> u32 {
        self.dire_series_wins.unwrap_or(0)
    }

    fn get_dire_series_wins_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dire_series_wins
    }

    fn mut_dire_series_wins_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dire_series_wins
    }

    // optional bool allchat = 23;

    pub fn clear_allchat(&mut self) {
        self.allchat = ::std::option::Option::None;
    }

    pub fn has_allchat(&self) -> bool {
        self.allchat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allchat(&mut self, v: bool) {
        self.allchat = ::std::option::Option::Some(v);
    }

    pub fn get_allchat(&self) -> bool {
        self.allchat.unwrap_or(false)
    }

    fn get_allchat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allchat
    }

    fn mut_allchat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allchat
    }

    // optional .dota.LobbyDotaTVDelay dota_tv_delay = 24;

    pub fn clear_dota_tv_delay(&mut self) {
        self.dota_tv_delay = ::std::option::Option::None;
    }

    pub fn has_dota_tv_delay(&self) -> bool {
        self.dota_tv_delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_tv_delay(&mut self, v: super::dota_gcmessages_common_match_management::LobbyDotaTVDelay) {
        self.dota_tv_delay = ::std::option::Option::Some(v);
    }

    pub fn get_dota_tv_delay(&self) -> super::dota_gcmessages_common_match_management::LobbyDotaTVDelay {
        self.dota_tv_delay.unwrap_or(super::dota_gcmessages_common_match_management::LobbyDotaTVDelay::LobbyDotaTV_120)
    }

    fn get_dota_tv_delay_for_reflect(&self) -> &::std::option::Option<super::dota_gcmessages_common_match_management::LobbyDotaTVDelay> {
        &self.dota_tv_delay
    }

    fn mut_dota_tv_delay_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_gcmessages_common_match_management::LobbyDotaTVDelay> {
        &mut self.dota_tv_delay
    }

    // optional bool lan = 25;

    pub fn clear_lan(&mut self) {
        self.lan = ::std::option::Option::None;
    }

    pub fn has_lan(&self) -> bool {
        self.lan.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lan(&mut self, v: bool) {
        self.lan = ::std::option::Option::Some(v);
    }

    pub fn get_lan(&self) -> bool {
        self.lan.unwrap_or(false)
    }

    fn get_lan_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.lan
    }

    fn mut_lan_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.lan
    }

    // optional string custom_game_mode = 26;

    pub fn clear_custom_game_mode(&mut self) {
        self.custom_game_mode.clear();
    }

    pub fn has_custom_game_mode(&self) -> bool {
        self.custom_game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_mode(&mut self, v: ::std::string::String) {
        self.custom_game_mode = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_custom_game_mode(&mut self) -> &mut ::std::string::String {
        if self.custom_game_mode.is_none() {
            self.custom_game_mode.set_default();
        };
        self.custom_game_mode.as_mut().unwrap()
    }

    // Take field
    pub fn take_custom_game_mode(&mut self) -> ::std::string::String {
        self.custom_game_mode.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_custom_game_mode(&self) -> &str {
        match self.custom_game_mode.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_custom_game_mode_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.custom_game_mode
    }

    fn mut_custom_game_mode_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.custom_game_mode
    }

    // optional string custom_map_name = 27;

    pub fn clear_custom_map_name(&mut self) {
        self.custom_map_name.clear();
    }

    pub fn has_custom_map_name(&self) -> bool {
        self.custom_map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_map_name(&mut self, v: ::std::string::String) {
        self.custom_map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_custom_map_name(&mut self) -> &mut ::std::string::String {
        if self.custom_map_name.is_none() {
            self.custom_map_name.set_default();
        };
        self.custom_map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_custom_map_name(&mut self) -> ::std::string::String {
        self.custom_map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_custom_map_name(&self) -> &str {
        match self.custom_map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_custom_map_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.custom_map_name
    }

    fn mut_custom_map_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.custom_map_name
    }

    // optional uint32 custom_difficulty = 28;

    pub fn clear_custom_difficulty(&mut self) {
        self.custom_difficulty = ::std::option::Option::None;
    }

    pub fn has_custom_difficulty(&self) -> bool {
        self.custom_difficulty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_difficulty(&mut self, v: u32) {
        self.custom_difficulty = ::std::option::Option::Some(v);
    }

    pub fn get_custom_difficulty(&self) -> u32 {
        self.custom_difficulty.unwrap_or(0)
    }

    fn get_custom_difficulty_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.custom_difficulty
    }

    fn mut_custom_difficulty_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.custom_difficulty
    }

    // optional uint64 custom_game_id = 29;

    pub fn clear_custom_game_id(&mut self) {
        self.custom_game_id = ::std::option::Option::None;
    }

    pub fn has_custom_game_id(&self) -> bool {
        self.custom_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_id(&mut self, v: u64) {
        self.custom_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_id(&self) -> u64 {
        self.custom_game_id.unwrap_or(0)
    }

    fn get_custom_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_id
    }

    fn mut_custom_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_id
    }

    // optional uint32 custom_min_players = 30;

    pub fn clear_custom_min_players(&mut self) {
        self.custom_min_players = ::std::option::Option::None;
    }

    pub fn has_custom_min_players(&self) -> bool {
        self.custom_min_players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_min_players(&mut self, v: u32) {
        self.custom_min_players = ::std::option::Option::Some(v);
    }

    pub fn get_custom_min_players(&self) -> u32 {
        self.custom_min_players.unwrap_or(0)
    }

    fn get_custom_min_players_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.custom_min_players
    }

    fn mut_custom_min_players_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.custom_min_players
    }

    // optional uint32 custom_max_players = 31;

    pub fn clear_custom_max_players(&mut self) {
        self.custom_max_players = ::std::option::Option::None;
    }

    pub fn has_custom_max_players(&self) -> bool {
        self.custom_max_players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_max_players(&mut self, v: u32) {
        self.custom_max_players = ::std::option::Option::Some(v);
    }

    pub fn get_custom_max_players(&self) -> u32 {
        self.custom_max_players.unwrap_or(0)
    }

    fn get_custom_max_players_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.custom_max_players
    }

    fn mut_custom_max_players_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.custom_max_players
    }

    // optional uint32 lan_host_ping_to_server_region = 32;

    pub fn clear_lan_host_ping_to_server_region(&mut self) {
        self.lan_host_ping_to_server_region = ::std::option::Option::None;
    }

    pub fn has_lan_host_ping_to_server_region(&self) -> bool {
        self.lan_host_ping_to_server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lan_host_ping_to_server_region(&mut self, v: u32) {
        self.lan_host_ping_to_server_region = ::std::option::Option::Some(v);
    }

    pub fn get_lan_host_ping_to_server_region(&self) -> u32 {
        self.lan_host_ping_to_server_region.unwrap_or(0)
    }

    fn get_lan_host_ping_to_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lan_host_ping_to_server_region
    }

    fn mut_lan_host_ping_to_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lan_host_ping_to_server_region
    }

    // optional .dota.DOTALobbyVisibility visibility = 33;

    pub fn clear_visibility(&mut self) {
        self.visibility = ::std::option::Option::None;
    }

    pub fn has_visibility(&self) -> bool {
        self.visibility.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visibility(&mut self, v: super::dota_shared_enums::DOTALobbyVisibility) {
        self.visibility = ::std::option::Option::Some(v);
    }

    pub fn get_visibility(&self) -> super::dota_shared_enums::DOTALobbyVisibility {
        self.visibility.unwrap_or(super::dota_shared_enums::DOTALobbyVisibility::DOTALobbyVisibility_Public)
    }

    fn get_visibility_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTALobbyVisibility> {
        &self.visibility
    }

    fn mut_visibility_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTALobbyVisibility> {
        &mut self.visibility
    }

    // optional fixed64 custom_game_crc = 34;

    pub fn clear_custom_game_crc(&mut self) {
        self.custom_game_crc = ::std::option::Option::None;
    }

    pub fn has_custom_game_crc(&self) -> bool {
        self.custom_game_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_crc(&mut self, v: u64) {
        self.custom_game_crc = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_crc(&self) -> u64 {
        self.custom_game_crc.unwrap_or(0)
    }

    fn get_custom_game_crc_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_crc
    }

    fn mut_custom_game_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_crc
    }

    // optional uint32 league_series_id = 35;

    pub fn clear_league_series_id(&mut self) {
        self.league_series_id = ::std::option::Option::None;
    }

    pub fn has_league_series_id(&self) -> bool {
        self.league_series_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_series_id(&mut self, v: u32) {
        self.league_series_id = ::std::option::Option::Some(v);
    }

    pub fn get_league_series_id(&self) -> u32 {
        self.league_series_id.unwrap_or(0)
    }

    fn get_league_series_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_series_id
    }

    fn mut_league_series_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_series_id
    }

    // optional uint32 league_game_id = 36;

    pub fn clear_league_game_id(&mut self) {
        self.league_game_id = ::std::option::Option::None;
    }

    pub fn has_league_game_id(&self) -> bool {
        self.league_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_game_id(&mut self, v: u32) {
        self.league_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_league_game_id(&self) -> u32 {
        self.league_game_id.unwrap_or(0)
    }

    fn get_league_game_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_game_id
    }

    fn mut_league_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_game_id
    }

    // optional fixed32 custom_game_timestamp = 37;

    pub fn clear_custom_game_timestamp(&mut self) {
        self.custom_game_timestamp = ::std::option::Option::None;
    }

    pub fn has_custom_game_timestamp(&self) -> bool {
        self.custom_game_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_timestamp(&mut self, v: u32) {
        self.custom_game_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_timestamp(&self) -> u32 {
        self.custom_game_timestamp.unwrap_or(0)
    }

    fn get_custom_game_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.custom_game_timestamp
    }

    fn mut_custom_game_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.custom_game_timestamp
    }

    // optional uint64 previous_match_override = 38;

    pub fn clear_previous_match_override(&mut self) {
        self.previous_match_override = ::std::option::Option::None;
    }

    pub fn has_previous_match_override(&self) -> bool {
        self.previous_match_override.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previous_match_override(&mut self, v: u64) {
        self.previous_match_override = ::std::option::Option::Some(v);
    }

    pub fn get_previous_match_override(&self) -> u64 {
        self.previous_match_override.unwrap_or(0)
    }

    fn get_previous_match_override_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.previous_match_override
    }

    fn mut_previous_match_override_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.previous_match_override
    }

    // optional uint32 league_selection_priority_team = 39;

    pub fn clear_league_selection_priority_team(&mut self) {
        self.league_selection_priority_team = ::std::option::Option::None;
    }

    pub fn has_league_selection_priority_team(&self) -> bool {
        self.league_selection_priority_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_selection_priority_team(&mut self, v: u32) {
        self.league_selection_priority_team = ::std::option::Option::Some(v);
    }

    pub fn get_league_selection_priority_team(&self) -> u32 {
        self.league_selection_priority_team.unwrap_or(0)
    }

    fn get_league_selection_priority_team_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_selection_priority_team
    }

    fn mut_league_selection_priority_team_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_selection_priority_team
    }

    // optional .dota.SelectionPriorityType league_selection_priority_choice = 40;

    pub fn clear_league_selection_priority_choice(&mut self) {
        self.league_selection_priority_choice = ::std::option::Option::None;
    }

    pub fn has_league_selection_priority_choice(&self) -> bool {
        self.league_selection_priority_choice.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_selection_priority_choice(&mut self, v: super::dota_shared_enums::SelectionPriorityType) {
        self.league_selection_priority_choice = ::std::option::Option::Some(v);
    }

    pub fn get_league_selection_priority_choice(&self) -> super::dota_shared_enums::SelectionPriorityType {
        self.league_selection_priority_choice.unwrap_or(super::dota_shared_enums::SelectionPriorityType::UNDEFINED)
    }

    fn get_league_selection_priority_choice_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::SelectionPriorityType> {
        &self.league_selection_priority_choice
    }

    fn mut_league_selection_priority_choice_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::SelectionPriorityType> {
        &mut self.league_selection_priority_choice
    }

    // optional .dota.SelectionPriorityType league_non_selection_priority_choice = 41;

    pub fn clear_league_non_selection_priority_choice(&mut self) {
        self.league_non_selection_priority_choice = ::std::option::Option::None;
    }

    pub fn has_league_non_selection_priority_choice(&self) -> bool {
        self.league_non_selection_priority_choice.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_non_selection_priority_choice(&mut self, v: super::dota_shared_enums::SelectionPriorityType) {
        self.league_non_selection_priority_choice = ::std::option::Option::Some(v);
    }

    pub fn get_league_non_selection_priority_choice(&self) -> super::dota_shared_enums::SelectionPriorityType {
        self.league_non_selection_priority_choice.unwrap_or(super::dota_shared_enums::SelectionPriorityType::UNDEFINED)
    }

    fn get_league_non_selection_priority_choice_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::SelectionPriorityType> {
        &self.league_non_selection_priority_choice
    }

    fn mut_league_non_selection_priority_choice_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::SelectionPriorityType> {
        &mut self.league_non_selection_priority_choice
    }

    // optional .dota.LobbyDotaPauseSetting pause_setting = 42;

    pub fn clear_pause_setting(&mut self) {
        self.pause_setting = ::std::option::Option::None;
    }

    pub fn has_pause_setting(&self) -> bool {
        self.pause_setting.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pause_setting(&mut self, v: super::dota_gcmessages_common_match_management::LobbyDotaPauseSetting) {
        self.pause_setting = ::std::option::Option::Some(v);
    }

    pub fn get_pause_setting(&self) -> super::dota_gcmessages_common_match_management::LobbyDotaPauseSetting {
        self.pause_setting.unwrap_or(super::dota_gcmessages_common_match_management::LobbyDotaPauseSetting::LobbyDotaPauseSetting_Unlimited)
    }

    fn get_pause_setting_for_reflect(&self) -> &::std::option::Option<super::dota_gcmessages_common_match_management::LobbyDotaPauseSetting> {
        &self.pause_setting
    }

    fn mut_pause_setting_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_gcmessages_common_match_management::LobbyDotaPauseSetting> {
        &mut self.pause_setting
    }

    // optional .dota.DOTABotDifficulty bot_difficulty_dire = 43;

    pub fn clear_bot_difficulty_dire(&mut self) {
        self.bot_difficulty_dire = ::std::option::Option::None;
    }

    pub fn has_bot_difficulty_dire(&self) -> bool {
        self.bot_difficulty_dire.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bot_difficulty_dire(&mut self, v: super::dota_shared_enums::DOTABotDifficulty) {
        self.bot_difficulty_dire = ::std::option::Option::Some(v);
    }

    pub fn get_bot_difficulty_dire(&self) -> super::dota_shared_enums::DOTABotDifficulty {
        self.bot_difficulty_dire.unwrap_or(super::dota_shared_enums::DOTABotDifficulty::BOT_DIFFICULTY_PASSIVE)
    }

    fn get_bot_difficulty_dire_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &self.bot_difficulty_dire
    }

    fn mut_bot_difficulty_dire_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &mut self.bot_difficulty_dire
    }

    // optional uint64 bot_radiant = 44;

    pub fn clear_bot_radiant(&mut self) {
        self.bot_radiant = ::std::option::Option::None;
    }

    pub fn has_bot_radiant(&self) -> bool {
        self.bot_radiant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bot_radiant(&mut self, v: u64) {
        self.bot_radiant = ::std::option::Option::Some(v);
    }

    pub fn get_bot_radiant(&self) -> u64 {
        self.bot_radiant.unwrap_or(0)
    }

    fn get_bot_radiant_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bot_radiant
    }

    fn mut_bot_radiant_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bot_radiant
    }

    // optional uint64 bot_dire = 45;

    pub fn clear_bot_dire(&mut self) {
        self.bot_dire = ::std::option::Option::None;
    }

    pub fn has_bot_dire(&self) -> bool {
        self.bot_dire.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bot_dire(&mut self, v: u64) {
        self.bot_dire = ::std::option::Option::Some(v);
    }

    pub fn get_bot_dire(&self) -> u64 {
        self.bot_dire.unwrap_or(0)
    }

    fn get_bot_dire_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bot_dire
    }

    fn mut_bot_dire_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bot_dire
    }
}

impl ::protobuf::Message for CMsgPracticeLobbySetDetails {
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
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_name)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.team_details)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.server_region = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.cm_pick = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.bot_difficulty_radiant = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allow_cheats = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.fill_with_bots = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.intro_mode = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allow_spectating = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_version = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pass_key)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.leagueid = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.penalty_level_radiant = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.penalty_level_dire = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.load_game_id = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_type = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.radiant_series_wins = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.dire_series_wins = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allchat = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.dota_tv_delay = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.lan = ::std::option::Option::Some(tmp);
                },
                26 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.custom_game_mode)?;
                },
                27 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.custom_map_name)?;
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.custom_difficulty = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.custom_game_id = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.custom_min_players = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.custom_max_players = ::std::option::Option::Some(tmp);
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lan_host_ping_to_server_region = ::std::option::Option::Some(tmp);
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.visibility = ::std::option::Option::Some(tmp);
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.custom_game_crc = ::std::option::Option::Some(tmp);
                },
                35 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_series_id = ::std::option::Option::Some(tmp);
                },
                36 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_game_id = ::std::option::Option::Some(tmp);
                },
                37 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.custom_game_timestamp = ::std::option::Option::Some(tmp);
                },
                38 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.previous_match_override = ::std::option::Option::Some(tmp);
                },
                39 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_selection_priority_team = ::std::option::Option::Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.league_selection_priority_choice = ::std::option::Option::Some(tmp);
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.league_non_selection_priority_choice = ::std::option::Option::Some(tmp);
                },
                42 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.pause_setting = ::std::option::Option::Some(tmp);
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.bot_difficulty_dire = ::std::option::Option::Some(tmp);
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.bot_radiant = ::std::option::Option::Some(tmp);
                },
                45 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.bot_dire = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.lobby_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        for value in &self.team_details {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.server_region {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.cm_pick {
            my_size += ::protobuf::rt::enum_size(6, v);
        };
        if let Some(v) = self.bot_difficulty_radiant {
            my_size += ::protobuf::rt::enum_size(9, v);
        };
        if let Some(v) = self.allow_cheats {
            my_size += 2;
        };
        if let Some(v) = self.fill_with_bots {
            my_size += 2;
        };
        if let Some(v) = self.intro_mode {
            my_size += 2;
        };
        if let Some(v) = self.allow_spectating {
            my_size += 2;
        };
        if let Some(v) = self.game_version {
            my_size += ::protobuf::rt::enum_size(14, v);
        };
        if let Some(v) = self.pass_key.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        };
        if let Some(v) = self.leagueid {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.penalty_level_radiant {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.penalty_level_dire {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.load_game_id {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.series_type {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radiant_series_wins {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dire_series_wins {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.allchat {
            my_size += 3;
        };
        if let Some(v) = self.dota_tv_delay {
            my_size += ::protobuf::rt::enum_size(24, v);
        };
        if let Some(v) = self.lan {
            my_size += 3;
        };
        if let Some(v) = self.custom_game_mode.as_ref() {
            my_size += ::protobuf::rt::string_size(26, &v);
        };
        if let Some(v) = self.custom_map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(27, &v);
        };
        if let Some(v) = self.custom_difficulty {
            my_size += ::protobuf::rt::value_size(28, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(29, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_min_players {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_max_players {
            my_size += ::protobuf::rt::value_size(31, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lan_host_ping_to_server_region {
            my_size += ::protobuf::rt::value_size(32, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.visibility {
            my_size += ::protobuf::rt::enum_size(33, v);
        };
        if let Some(v) = self.custom_game_crc {
            my_size += 10;
        };
        if let Some(v) = self.league_series_id {
            my_size += ::protobuf::rt::value_size(35, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_game_id {
            my_size += ::protobuf::rt::value_size(36, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_game_timestamp {
            my_size += 6;
        };
        if let Some(v) = self.previous_match_override {
            my_size += ::protobuf::rt::value_size(38, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_selection_priority_team {
            my_size += ::protobuf::rt::value_size(39, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_selection_priority_choice {
            my_size += ::protobuf::rt::enum_size(40, v);
        };
        if let Some(v) = self.league_non_selection_priority_choice {
            my_size += ::protobuf::rt::enum_size(41, v);
        };
        if let Some(v) = self.pause_setting {
            my_size += ::protobuf::rt::enum_size(42, v);
        };
        if let Some(v) = self.bot_difficulty_dire {
            my_size += ::protobuf::rt::enum_size(43, v);
        };
        if let Some(v) = self.bot_radiant {
            my_size += ::protobuf::rt::value_size(44, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.bot_dire {
            my_size += ::protobuf::rt::value_size(45, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.game_name.as_ref() {
            os.write_string(2, &v)?;
        };
        for v in &self.team_details {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.server_region {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.game_mode {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.cm_pick {
            os.write_enum(6, v.value())?;
        };
        if let Some(v) = self.bot_difficulty_radiant {
            os.write_enum(9, v.value())?;
        };
        if let Some(v) = self.allow_cheats {
            os.write_bool(10, v)?;
        };
        if let Some(v) = self.fill_with_bots {
            os.write_bool(11, v)?;
        };
        if let Some(v) = self.intro_mode {
            os.write_bool(12, v)?;
        };
        if let Some(v) = self.allow_spectating {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.game_version {
            os.write_enum(14, v.value())?;
        };
        if let Some(v) = self.pass_key.as_ref() {
            os.write_string(15, &v)?;
        };
        if let Some(v) = self.leagueid {
            os.write_uint32(16, v)?;
        };
        if let Some(v) = self.penalty_level_radiant {
            os.write_uint32(17, v)?;
        };
        if let Some(v) = self.penalty_level_dire {
            os.write_uint32(18, v)?;
        };
        if let Some(v) = self.load_game_id {
            os.write_uint32(19, v)?;
        };
        if let Some(v) = self.series_type {
            os.write_uint32(20, v)?;
        };
        if let Some(v) = self.radiant_series_wins {
            os.write_uint32(21, v)?;
        };
        if let Some(v) = self.dire_series_wins {
            os.write_uint32(22, v)?;
        };
        if let Some(v) = self.allchat {
            os.write_bool(23, v)?;
        };
        if let Some(v) = self.dota_tv_delay {
            os.write_enum(24, v.value())?;
        };
        if let Some(v) = self.lan {
            os.write_bool(25, v)?;
        };
        if let Some(v) = self.custom_game_mode.as_ref() {
            os.write_string(26, &v)?;
        };
        if let Some(v) = self.custom_map_name.as_ref() {
            os.write_string(27, &v)?;
        };
        if let Some(v) = self.custom_difficulty {
            os.write_uint32(28, v)?;
        };
        if let Some(v) = self.custom_game_id {
            os.write_uint64(29, v)?;
        };
        if let Some(v) = self.custom_min_players {
            os.write_uint32(30, v)?;
        };
        if let Some(v) = self.custom_max_players {
            os.write_uint32(31, v)?;
        };
        if let Some(v) = self.lan_host_ping_to_server_region {
            os.write_uint32(32, v)?;
        };
        if let Some(v) = self.visibility {
            os.write_enum(33, v.value())?;
        };
        if let Some(v) = self.custom_game_crc {
            os.write_fixed64(34, v)?;
        };
        if let Some(v) = self.league_series_id {
            os.write_uint32(35, v)?;
        };
        if let Some(v) = self.league_game_id {
            os.write_uint32(36, v)?;
        };
        if let Some(v) = self.custom_game_timestamp {
            os.write_fixed32(37, v)?;
        };
        if let Some(v) = self.previous_match_override {
            os.write_uint64(38, v)?;
        };
        if let Some(v) = self.league_selection_priority_team {
            os.write_uint32(39, v)?;
        };
        if let Some(v) = self.league_selection_priority_choice {
            os.write_enum(40, v.value())?;
        };
        if let Some(v) = self.league_non_selection_priority_choice {
            os.write_enum(41, v.value())?;
        };
        if let Some(v) = self.pause_setting {
            os.write_enum(42, v.value())?;
        };
        if let Some(v) = self.bot_difficulty_dire {
            os.write_enum(43, v.value())?;
        };
        if let Some(v) = self.bot_radiant {
            os.write_uint64(44, v)?;
        };
        if let Some(v) = self.bot_dire {
            os.write_uint64(45, v)?;
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbySetDetails {
    fn new() -> CMsgPracticeLobbySetDetails {
        CMsgPracticeLobbySetDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbySetDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lobby_id",
                    CMsgPracticeLobbySetDetails::get_lobby_id_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_name",
                    CMsgPracticeLobbySetDetails::get_game_name_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_game_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_gcmessages_common_match_management::CLobbyTeamDetails>>(
                    "team_details",
                    CMsgPracticeLobbySetDetails::get_team_details_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_team_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_region",
                    CMsgPracticeLobbySetDetails::get_server_region_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_mode",
                    CMsgPracticeLobbySetDetails::get_game_mode_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_CM_PICK>>(
                    "cm_pick",
                    CMsgPracticeLobbySetDetails::get_cm_pick_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_cm_pick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "bot_difficulty_radiant",
                    CMsgPracticeLobbySetDetails::get_bot_difficulty_radiant_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_bot_difficulty_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_cheats",
                    CMsgPracticeLobbySetDetails::get_allow_cheats_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_allow_cheats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "fill_with_bots",
                    CMsgPracticeLobbySetDetails::get_fill_with_bots_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_fill_with_bots_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "intro_mode",
                    CMsgPracticeLobbySetDetails::get_intro_mode_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_intro_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_spectating",
                    CMsgPracticeLobbySetDetails::get_allow_spectating_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_allow_spectating_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAGameVersion>>(
                    "game_version",
                    CMsgPracticeLobbySetDetails::get_game_version_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_game_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pass_key",
                    CMsgPracticeLobbySetDetails::get_pass_key_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leagueid",
                    CMsgPracticeLobbySetDetails::get_leagueid_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_leagueid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "penalty_level_radiant",
                    CMsgPracticeLobbySetDetails::get_penalty_level_radiant_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_penalty_level_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "penalty_level_dire",
                    CMsgPracticeLobbySetDetails::get_penalty_level_dire_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_penalty_level_dire_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "load_game_id",
                    CMsgPracticeLobbySetDetails::get_load_game_id_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_load_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_type",
                    CMsgPracticeLobbySetDetails::get_series_type_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_series_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "radiant_series_wins",
                    CMsgPracticeLobbySetDetails::get_radiant_series_wins_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_radiant_series_wins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dire_series_wins",
                    CMsgPracticeLobbySetDetails::get_dire_series_wins_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_dire_series_wins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allchat",
                    CMsgPracticeLobbySetDetails::get_allchat_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_allchat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_gcmessages_common_match_management::LobbyDotaTVDelay>>(
                    "dota_tv_delay",
                    CMsgPracticeLobbySetDetails::get_dota_tv_delay_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_dota_tv_delay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "lan",
                    CMsgPracticeLobbySetDetails::get_lan_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_lan_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "custom_game_mode",
                    CMsgPracticeLobbySetDetails::get_custom_game_mode_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_custom_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "custom_map_name",
                    CMsgPracticeLobbySetDetails::get_custom_map_name_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_custom_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "custom_difficulty",
                    CMsgPracticeLobbySetDetails::get_custom_difficulty_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_custom_difficulty_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CMsgPracticeLobbySetDetails::get_custom_game_id_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_custom_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "custom_min_players",
                    CMsgPracticeLobbySetDetails::get_custom_min_players_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_custom_min_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "custom_max_players",
                    CMsgPracticeLobbySetDetails::get_custom_max_players_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_custom_max_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lan_host_ping_to_server_region",
                    CMsgPracticeLobbySetDetails::get_lan_host_ping_to_server_region_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_lan_host_ping_to_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTALobbyVisibility>>(
                    "visibility",
                    CMsgPracticeLobbySetDetails::get_visibility_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_visibility_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "custom_game_crc",
                    CMsgPracticeLobbySetDetails::get_custom_game_crc_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_custom_game_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_series_id",
                    CMsgPracticeLobbySetDetails::get_league_series_id_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_league_series_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_game_id",
                    CMsgPracticeLobbySetDetails::get_league_game_id_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_league_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "custom_game_timestamp",
                    CMsgPracticeLobbySetDetails::get_custom_game_timestamp_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_custom_game_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "previous_match_override",
                    CMsgPracticeLobbySetDetails::get_previous_match_override_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_previous_match_override_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_selection_priority_team",
                    CMsgPracticeLobbySetDetails::get_league_selection_priority_team_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_league_selection_priority_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::SelectionPriorityType>>(
                    "league_selection_priority_choice",
                    CMsgPracticeLobbySetDetails::get_league_selection_priority_choice_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_league_selection_priority_choice_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::SelectionPriorityType>>(
                    "league_non_selection_priority_choice",
                    CMsgPracticeLobbySetDetails::get_league_non_selection_priority_choice_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_league_non_selection_priority_choice_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_gcmessages_common_match_management::LobbyDotaPauseSetting>>(
                    "pause_setting",
                    CMsgPracticeLobbySetDetails::get_pause_setting_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_pause_setting_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "bot_difficulty_dire",
                    CMsgPracticeLobbySetDetails::get_bot_difficulty_dire_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_bot_difficulty_dire_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bot_radiant",
                    CMsgPracticeLobbySetDetails::get_bot_radiant_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_bot_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bot_dire",
                    CMsgPracticeLobbySetDetails::get_bot_dire_for_reflect,
                    CMsgPracticeLobbySetDetails::mut_bot_dire_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbySetDetails>(
                    "CMsgPracticeLobbySetDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbySetDetails {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.clear_game_name();
        self.clear_team_details();
        self.clear_server_region();
        self.clear_game_mode();
        self.clear_cm_pick();
        self.clear_bot_difficulty_radiant();
        self.clear_allow_cheats();
        self.clear_fill_with_bots();
        self.clear_intro_mode();
        self.clear_allow_spectating();
        self.clear_game_version();
        self.clear_pass_key();
        self.clear_leagueid();
        self.clear_penalty_level_radiant();
        self.clear_penalty_level_dire();
        self.clear_load_game_id();
        self.clear_series_type();
        self.clear_radiant_series_wins();
        self.clear_dire_series_wins();
        self.clear_allchat();
        self.clear_dota_tv_delay();
        self.clear_lan();
        self.clear_custom_game_mode();
        self.clear_custom_map_name();
        self.clear_custom_difficulty();
        self.clear_custom_game_id();
        self.clear_custom_min_players();
        self.clear_custom_max_players();
        self.clear_lan_host_ping_to_server_region();
        self.clear_visibility();
        self.clear_custom_game_crc();
        self.clear_league_series_id();
        self.clear_league_game_id();
        self.clear_custom_game_timestamp();
        self.clear_previous_match_override();
        self.clear_league_selection_priority_team();
        self.clear_league_selection_priority_choice();
        self.clear_league_non_selection_priority_choice();
        self.clear_pause_setting();
        self.clear_bot_difficulty_dire();
        self.clear_bot_radiant();
        self.clear_bot_dire();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbySetDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbySetDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyCreate {
    // message fields
    search_key: ::protobuf::SingularField<::std::string::String>,
    pass_key: ::protobuf::SingularField<::std::string::String>,
    client_version: ::std::option::Option<u32>,
    lobby_details: ::protobuf::SingularPtrField<CMsgPracticeLobbySetDetails>,
    save_game: ::protobuf::SingularPtrField<CMsgPracticeLobbyCreate_SaveGame>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyCreate {}

impl CMsgPracticeLobbyCreate {
    pub fn new() -> CMsgPracticeLobbyCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyCreate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyCreate,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyCreate::new)
        }
    }

    // optional string search_key = 1;

    pub fn clear_search_key(&mut self) {
        self.search_key.clear();
    }

    pub fn has_search_key(&self) -> bool {
        self.search_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_key(&mut self, v: ::std::string::String) {
        self.search_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search_key(&mut self) -> &mut ::std::string::String {
        if self.search_key.is_none() {
            self.search_key.set_default();
        };
        self.search_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_search_key(&mut self) -> ::std::string::String {
        self.search_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_search_key(&self) -> &str {
        match self.search_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_search_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.search_key
    }

    fn mut_search_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.search_key
    }

    // optional string pass_key = 5;

    pub fn clear_pass_key(&mut self) {
        self.pass_key.clear();
    }

    pub fn has_pass_key(&self) -> bool {
        self.pass_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pass_key(&mut self, v: ::std::string::String) {
        self.pass_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pass_key(&mut self) -> &mut ::std::string::String {
        if self.pass_key.is_none() {
            self.pass_key.set_default();
        };
        self.pass_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_pass_key(&mut self) -> ::std::string::String {
        self.pass_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pass_key(&self) -> &str {
        match self.pass_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_pass_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.pass_key
    }

    fn mut_pass_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.pass_key
    }

    // optional uint32 client_version = 6;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional .dota.CMsgPracticeLobbySetDetails lobby_details = 7;

    pub fn clear_lobby_details(&mut self) {
        self.lobby_details.clear();
    }

    pub fn has_lobby_details(&self) -> bool {
        self.lobby_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_details(&mut self, v: CMsgPracticeLobbySetDetails) {
        self.lobby_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lobby_details(&mut self) -> &mut CMsgPracticeLobbySetDetails {
        if self.lobby_details.is_none() {
            self.lobby_details.set_default();
        };
        self.lobby_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_lobby_details(&mut self) -> CMsgPracticeLobbySetDetails {
        self.lobby_details.take().unwrap_or_else(|| CMsgPracticeLobbySetDetails::new())
    }

    pub fn get_lobby_details(&self) -> &CMsgPracticeLobbySetDetails {
        self.lobby_details.as_ref().unwrap_or_else(|| CMsgPracticeLobbySetDetails::default_instance())
    }

    fn get_lobby_details_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgPracticeLobbySetDetails> {
        &self.lobby_details
    }

    fn mut_lobby_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgPracticeLobbySetDetails> {
        &mut self.lobby_details
    }

    // optional .dota.CMsgPracticeLobbyCreate.SaveGame save_game = 8;

    pub fn clear_save_game(&mut self) {
        self.save_game.clear();
    }

    pub fn has_save_game(&self) -> bool {
        self.save_game.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_game(&mut self, v: CMsgPracticeLobbyCreate_SaveGame) {
        self.save_game = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_save_game(&mut self) -> &mut CMsgPracticeLobbyCreate_SaveGame {
        if self.save_game.is_none() {
            self.save_game.set_default();
        };
        self.save_game.as_mut().unwrap()
    }

    // Take field
    pub fn take_save_game(&mut self) -> CMsgPracticeLobbyCreate_SaveGame {
        self.save_game.take().unwrap_or_else(|| CMsgPracticeLobbyCreate_SaveGame::new())
    }

    pub fn get_save_game(&self) -> &CMsgPracticeLobbyCreate_SaveGame {
        self.save_game.as_ref().unwrap_or_else(|| CMsgPracticeLobbyCreate_SaveGame::default_instance())
    }

    fn get_save_game_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgPracticeLobbyCreate_SaveGame> {
        &self.save_game
    }

    fn mut_save_game_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgPracticeLobbyCreate_SaveGame> {
        &mut self.save_game
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.search_key)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pass_key)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lobby_details)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.save_game)?;
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
        if let Some(v) = self.search_key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.pass_key.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lobby_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.save_game.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.search_key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.pass_key.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.lobby_details.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.save_game.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyCreate {
    fn new() -> CMsgPracticeLobbyCreate {
        CMsgPracticeLobbyCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "search_key",
                    CMsgPracticeLobbyCreate::get_search_key_for_reflect,
                    CMsgPracticeLobbyCreate::mut_search_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pass_key",
                    CMsgPracticeLobbyCreate::get_pass_key_for_reflect,
                    CMsgPracticeLobbyCreate::mut_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgPracticeLobbyCreate::get_client_version_for_reflect,
                    CMsgPracticeLobbyCreate::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPracticeLobbySetDetails>>(
                    "lobby_details",
                    CMsgPracticeLobbyCreate::get_lobby_details_for_reflect,
                    CMsgPracticeLobbyCreate::mut_lobby_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPracticeLobbyCreate_SaveGame>>(
                    "save_game",
                    CMsgPracticeLobbyCreate::get_save_game_for_reflect,
                    CMsgPracticeLobbyCreate::mut_save_game_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyCreate>(
                    "CMsgPracticeLobbyCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyCreate {
    fn clear(&mut self) {
        self.clear_search_key();
        self.clear_pass_key();
        self.clear_client_version();
        self.clear_lobby_details();
        self.clear_save_game();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyCreate_SaveGame {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    version: ::std::option::Option<i32>,
    steam_id: ::std::option::Option<u64>,
    signature: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyCreate_SaveGame {}

impl CMsgPracticeLobbyCreate_SaveGame {
    pub fn new() -> CMsgPracticeLobbyCreate_SaveGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyCreate_SaveGame {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyCreate_SaveGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyCreate_SaveGame,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyCreate_SaveGame::new)
        }
    }

    // optional bytes data = 1;

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

    // optional int32 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> i32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.version
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

    // optional fixed64 signature = 4;

    pub fn clear_signature(&mut self) {
        self.signature = ::std::option::Option::None;
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: u64) {
        self.signature = ::std::option::Option::Some(v);
    }

    pub fn get_signature(&self) -> u64 {
        self.signature.unwrap_or(0)
    }

    fn get_signature_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.signature
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyCreate_SaveGame {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.signature = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.steam_id {
            my_size += 9;
        };
        if let Some(v) = self.signature {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.data.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.version {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.steam_id {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.signature {
            os.write_fixed64(4, v)?;
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyCreate_SaveGame {
    fn new() -> CMsgPracticeLobbyCreate_SaveGame {
        CMsgPracticeLobbyCreate_SaveGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyCreate_SaveGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CMsgPracticeLobbyCreate_SaveGame::get_data_for_reflect,
                    CMsgPracticeLobbyCreate_SaveGame::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    CMsgPracticeLobbyCreate_SaveGame::get_version_for_reflect,
                    CMsgPracticeLobbyCreate_SaveGame::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgPracticeLobbyCreate_SaveGame::get_steam_id_for_reflect,
                    CMsgPracticeLobbyCreate_SaveGame::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "signature",
                    CMsgPracticeLobbyCreate_SaveGame::get_signature_for_reflect,
                    CMsgPracticeLobbyCreate_SaveGame::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyCreate_SaveGame>(
                    "CMsgPracticeLobbyCreate_SaveGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyCreate_SaveGame {
    fn clear(&mut self) {
        self.clear_data();
        self.clear_version();
        self.clear_steam_id();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyCreate_SaveGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyCreate_SaveGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbySetTeamSlot {
    // message fields
    team: ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM>,
    slot: ::std::option::Option<u32>,
    bot_difficulty: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbySetTeamSlot {}

impl CMsgPracticeLobbySetTeamSlot {
    pub fn new() -> CMsgPracticeLobbySetTeamSlot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbySetTeamSlot {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbySetTeamSlot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbySetTeamSlot,
        };
        unsafe {
            instance.get(CMsgPracticeLobbySetTeamSlot::new)
        }
    }

    // optional .dota.DOTA_GC_TEAM team = 1;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: super::dota_shared_enums::DOTA_GC_TEAM) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> super::dota_shared_enums::DOTA_GC_TEAM {
        self.team.unwrap_or(super::dota_shared_enums::DOTA_GC_TEAM::DOTA_GC_TEAM_GOOD_GUYS)
    }

    fn get_team_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM> {
        &mut self.team
    }

    // optional uint32 slot = 2;

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: u32) {
        self.slot = ::std::option::Option::Some(v);
    }

    pub fn get_slot(&self) -> u32 {
        self.slot.unwrap_or(0)
    }

    fn get_slot_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.slot
    }

    fn mut_slot_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.slot
    }

    // optional .dota.DOTABotDifficulty bot_difficulty = 3;

    pub fn clear_bot_difficulty(&mut self) {
        self.bot_difficulty = ::std::option::Option::None;
    }

    pub fn has_bot_difficulty(&self) -> bool {
        self.bot_difficulty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bot_difficulty(&mut self, v: super::dota_shared_enums::DOTABotDifficulty) {
        self.bot_difficulty = ::std::option::Option::Some(v);
    }

    pub fn get_bot_difficulty(&self) -> super::dota_shared_enums::DOTABotDifficulty {
        self.bot_difficulty.unwrap_or(super::dota_shared_enums::DOTABotDifficulty::BOT_DIFFICULTY_PASSIVE)
    }

    fn get_bot_difficulty_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &self.bot_difficulty
    }

    fn mut_bot_difficulty_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &mut self.bot_difficulty
    }
}

impl ::protobuf::Message for CMsgPracticeLobbySetTeamSlot {
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
                    self.team = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.slot = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.bot_difficulty = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.bot_difficulty {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.slot {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.bot_difficulty {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbySetTeamSlot {
    fn new() -> CMsgPracticeLobbySetTeamSlot {
        CMsgPracticeLobbySetTeamSlot::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbySetTeamSlot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GC_TEAM>>(
                    "team",
                    CMsgPracticeLobbySetTeamSlot::get_team_for_reflect,
                    CMsgPracticeLobbySetTeamSlot::mut_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "slot",
                    CMsgPracticeLobbySetTeamSlot::get_slot_for_reflect,
                    CMsgPracticeLobbySetTeamSlot::mut_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "bot_difficulty",
                    CMsgPracticeLobbySetTeamSlot::get_bot_difficulty_for_reflect,
                    CMsgPracticeLobbySetTeamSlot::mut_bot_difficulty_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbySetTeamSlot>(
                    "CMsgPracticeLobbySetTeamSlot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbySetTeamSlot {
    fn clear(&mut self) {
        self.clear_team();
        self.clear_slot();
        self.clear_bot_difficulty();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbySetTeamSlot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbySetTeamSlot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbySetCoach {
    // message fields
    team: ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbySetCoach {}

impl CMsgPracticeLobbySetCoach {
    pub fn new() -> CMsgPracticeLobbySetCoach {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbySetCoach {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbySetCoach> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbySetCoach,
        };
        unsafe {
            instance.get(CMsgPracticeLobbySetCoach::new)
        }
    }

    // optional .dota.DOTA_GC_TEAM team = 1;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: super::dota_shared_enums::DOTA_GC_TEAM) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> super::dota_shared_enums::DOTA_GC_TEAM {
        self.team.unwrap_or(super::dota_shared_enums::DOTA_GC_TEAM::DOTA_GC_TEAM_GOOD_GUYS)
    }

    fn get_team_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM> {
        &mut self.team
    }
}

impl ::protobuf::Message for CMsgPracticeLobbySetCoach {
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
                    self.team = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbySetCoach {
    fn new() -> CMsgPracticeLobbySetCoach {
        CMsgPracticeLobbySetCoach::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbySetCoach>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GC_TEAM>>(
                    "team",
                    CMsgPracticeLobbySetCoach::get_team_for_reflect,
                    CMsgPracticeLobbySetCoach::mut_team_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbySetCoach>(
                    "CMsgPracticeLobbySetCoach",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbySetCoach {
    fn clear(&mut self) {
        self.clear_team();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbySetCoach {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbySetCoach {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyJoinBroadcastChannel {
    // message fields
    channel: ::std::option::Option<u32>,
    preferred_description: ::protobuf::SingularField<::std::string::String>,
    preferred_country_code: ::protobuf::SingularField<::std::string::String>,
    preferred_language_code: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyJoinBroadcastChannel {}

impl CMsgPracticeLobbyJoinBroadcastChannel {
    pub fn new() -> CMsgPracticeLobbyJoinBroadcastChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyJoinBroadcastChannel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyJoinBroadcastChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyJoinBroadcastChannel,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyJoinBroadcastChannel::new)
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

    // optional string preferred_description = 2;

    pub fn clear_preferred_description(&mut self) {
        self.preferred_description.clear();
    }

    pub fn has_preferred_description(&self) -> bool {
        self.preferred_description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preferred_description(&mut self, v: ::std::string::String) {
        self.preferred_description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preferred_description(&mut self) -> &mut ::std::string::String {
        if self.preferred_description.is_none() {
            self.preferred_description.set_default();
        };
        self.preferred_description.as_mut().unwrap()
    }

    // Take field
    pub fn take_preferred_description(&mut self) -> ::std::string::String {
        self.preferred_description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_preferred_description(&self) -> &str {
        match self.preferred_description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_preferred_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.preferred_description
    }

    fn mut_preferred_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.preferred_description
    }

    // optional string preferred_country_code = 3;

    pub fn clear_preferred_country_code(&mut self) {
        self.preferred_country_code.clear();
    }

    pub fn has_preferred_country_code(&self) -> bool {
        self.preferred_country_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preferred_country_code(&mut self, v: ::std::string::String) {
        self.preferred_country_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preferred_country_code(&mut self) -> &mut ::std::string::String {
        if self.preferred_country_code.is_none() {
            self.preferred_country_code.set_default();
        };
        self.preferred_country_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_preferred_country_code(&mut self) -> ::std::string::String {
        self.preferred_country_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_preferred_country_code(&self) -> &str {
        match self.preferred_country_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_preferred_country_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.preferred_country_code
    }

    fn mut_preferred_country_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.preferred_country_code
    }

    // optional string preferred_language_code = 4;

    pub fn clear_preferred_language_code(&mut self) {
        self.preferred_language_code.clear();
    }

    pub fn has_preferred_language_code(&self) -> bool {
        self.preferred_language_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preferred_language_code(&mut self, v: ::std::string::String) {
        self.preferred_language_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_preferred_language_code(&mut self) -> &mut ::std::string::String {
        if self.preferred_language_code.is_none() {
            self.preferred_language_code.set_default();
        };
        self.preferred_language_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_preferred_language_code(&mut self) -> ::std::string::String {
        self.preferred_language_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_preferred_language_code(&self) -> &str {
        match self.preferred_language_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_preferred_language_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.preferred_language_code
    }

    fn mut_preferred_language_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.preferred_language_code
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyJoinBroadcastChannel {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.preferred_description)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.preferred_country_code)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.preferred_language_code)?;
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
        if let Some(v) = self.preferred_description.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.preferred_country_code.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.preferred_language_code.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.preferred_description.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.preferred_country_code.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.preferred_language_code.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyJoinBroadcastChannel {
    fn new() -> CMsgPracticeLobbyJoinBroadcastChannel {
        CMsgPracticeLobbyJoinBroadcastChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyJoinBroadcastChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel",
                    CMsgPracticeLobbyJoinBroadcastChannel::get_channel_for_reflect,
                    CMsgPracticeLobbyJoinBroadcastChannel::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "preferred_description",
                    CMsgPracticeLobbyJoinBroadcastChannel::get_preferred_description_for_reflect,
                    CMsgPracticeLobbyJoinBroadcastChannel::mut_preferred_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "preferred_country_code",
                    CMsgPracticeLobbyJoinBroadcastChannel::get_preferred_country_code_for_reflect,
                    CMsgPracticeLobbyJoinBroadcastChannel::mut_preferred_country_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "preferred_language_code",
                    CMsgPracticeLobbyJoinBroadcastChannel::get_preferred_language_code_for_reflect,
                    CMsgPracticeLobbyJoinBroadcastChannel::mut_preferred_language_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyJoinBroadcastChannel>(
                    "CMsgPracticeLobbyJoinBroadcastChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyJoinBroadcastChannel {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_preferred_description();
        self.clear_preferred_country_code();
        self.clear_preferred_language_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyJoinBroadcastChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyJoinBroadcastChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyCloseBroadcastChannel {
    // message fields
    channel: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyCloseBroadcastChannel {}

impl CMsgPracticeLobbyCloseBroadcastChannel {
    pub fn new() -> CMsgPracticeLobbyCloseBroadcastChannel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyCloseBroadcastChannel {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyCloseBroadcastChannel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyCloseBroadcastChannel,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyCloseBroadcastChannel::new)
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
}

impl ::protobuf::Message for CMsgPracticeLobbyCloseBroadcastChannel {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyCloseBroadcastChannel {
    fn new() -> CMsgPracticeLobbyCloseBroadcastChannel {
        CMsgPracticeLobbyCloseBroadcastChannel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyCloseBroadcastChannel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel",
                    CMsgPracticeLobbyCloseBroadcastChannel::get_channel_for_reflect,
                    CMsgPracticeLobbyCloseBroadcastChannel::mut_channel_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyCloseBroadcastChannel>(
                    "CMsgPracticeLobbyCloseBroadcastChannel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyCloseBroadcastChannel {
    fn clear(&mut self) {
        self.clear_channel();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyCloseBroadcastChannel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyCloseBroadcastChannel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {}

impl CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
    pub fn new() -> CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus::new)
        }
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
    fn new() -> CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
        CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus>(
                    "CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyToggleBroadcastChannelCameramanStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyKick {
    // message fields
    account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyKick {}

impl CMsgPracticeLobbyKick {
    pub fn new() -> CMsgPracticeLobbyKick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyKick {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyKick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyKick,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyKick::new)
        }
    }

    // optional uint32 account_id = 3;

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
}

impl ::protobuf::Message for CMsgPracticeLobbyKick {
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
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyKick {
    fn new() -> CMsgPracticeLobbyKick {
        CMsgPracticeLobbyKick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyKick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgPracticeLobbyKick::get_account_id_for_reflect,
                    CMsgPracticeLobbyKick::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyKick>(
                    "CMsgPracticeLobbyKick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyKick {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyKick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyKick {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyKickFromTeam {
    // message fields
    account_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyKickFromTeam {}

impl CMsgPracticeLobbyKickFromTeam {
    pub fn new() -> CMsgPracticeLobbyKickFromTeam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyKickFromTeam {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyKickFromTeam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyKickFromTeam,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyKickFromTeam::new)
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
}

impl ::protobuf::Message for CMsgPracticeLobbyKickFromTeam {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyKickFromTeam {
    fn new() -> CMsgPracticeLobbyKickFromTeam {
        CMsgPracticeLobbyKickFromTeam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyKickFromTeam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgPracticeLobbyKickFromTeam::get_account_id_for_reflect,
                    CMsgPracticeLobbyKickFromTeam::mut_account_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyKickFromTeam>(
                    "CMsgPracticeLobbyKickFromTeam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyKickFromTeam {
    fn clear(&mut self) {
        self.clear_account_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyKickFromTeam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyKickFromTeam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyLeave {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyLeave {}

impl CMsgPracticeLobbyLeave {
    pub fn new() -> CMsgPracticeLobbyLeave {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyLeave {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyLeave> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyLeave,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyLeave::new)
        }
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyLeave {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyLeave {
    fn new() -> CMsgPracticeLobbyLeave {
        CMsgPracticeLobbyLeave::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyLeave>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyLeave>(
                    "CMsgPracticeLobbyLeave",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyLeave {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyLeave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyLeave {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyLaunch {
    // message fields
    client_version: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyLaunch {}

impl CMsgPracticeLobbyLaunch {
    pub fn new() -> CMsgPracticeLobbyLaunch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyLaunch {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyLaunch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyLaunch,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyLaunch::new)
        }
    }

    // optional uint32 client_version = 5;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyLaunch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_version {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyLaunch {
    fn new() -> CMsgPracticeLobbyLaunch {
        CMsgPracticeLobbyLaunch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyLaunch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgPracticeLobbyLaunch::get_client_version_for_reflect,
                    CMsgPracticeLobbyLaunch::mut_client_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyLaunch>(
                    "CMsgPracticeLobbyLaunch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyLaunch {
    fn clear(&mut self) {
        self.clear_client_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyLaunch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyLaunch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgApplyTeamToPracticeLobby {
    // message fields
    team_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgApplyTeamToPracticeLobby {}

impl CMsgApplyTeamToPracticeLobby {
    pub fn new() -> CMsgApplyTeamToPracticeLobby {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgApplyTeamToPracticeLobby {
        static mut instance: ::protobuf::lazy::Lazy<CMsgApplyTeamToPracticeLobby> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgApplyTeamToPracticeLobby,
        };
        unsafe {
            instance.get(CMsgApplyTeamToPracticeLobby::new)
        }
    }

    // optional uint32 team_id = 1;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }
}

impl ::protobuf::Message for CMsgApplyTeamToPracticeLobby {
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
                    self.team_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
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

impl ::protobuf::MessageStatic for CMsgApplyTeamToPracticeLobby {
    fn new() -> CMsgApplyTeamToPracticeLobby {
        CMsgApplyTeamToPracticeLobby::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgApplyTeamToPracticeLobby>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgApplyTeamToPracticeLobby::get_team_id_for_reflect,
                    CMsgApplyTeamToPracticeLobby::mut_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgApplyTeamToPracticeLobby>(
                    "CMsgApplyTeamToPracticeLobby",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgApplyTeamToPracticeLobby {
    fn clear(&mut self) {
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgApplyTeamToPracticeLobby {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgApplyTeamToPracticeLobby {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClearPracticeLobbyTeam {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClearPracticeLobbyTeam {}

impl CMsgClearPracticeLobbyTeam {
    pub fn new() -> CMsgClearPracticeLobbyTeam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClearPracticeLobbyTeam {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClearPracticeLobbyTeam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClearPracticeLobbyTeam,
        };
        unsafe {
            instance.get(CMsgClearPracticeLobbyTeam::new)
        }
    }
}

impl ::protobuf::Message for CMsgClearPracticeLobbyTeam {
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

impl ::protobuf::MessageStatic for CMsgClearPracticeLobbyTeam {
    fn new() -> CMsgClearPracticeLobbyTeam {
        CMsgClearPracticeLobbyTeam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClearPracticeLobbyTeam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClearPracticeLobbyTeam>(
                    "CMsgClearPracticeLobbyTeam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClearPracticeLobbyTeam {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClearPracticeLobbyTeam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClearPracticeLobbyTeam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyList {
    // message fields
    tournament_games: ::std::option::Option<bool>,
    pass_key: ::protobuf::SingularField<::std::string::String>,
    region: ::std::option::Option<u32>,
    game_mode: ::std::option::Option<super::dota_shared_enums::DOTA_GameMode>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyList {}

impl CMsgPracticeLobbyList {
    pub fn new() -> CMsgPracticeLobbyList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyList {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyList,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyList::new)
        }
    }

    // optional bool tournament_games = 1;

    pub fn clear_tournament_games(&mut self) {
        self.tournament_games = ::std::option::Option::None;
    }

    pub fn has_tournament_games(&self) -> bool {
        self.tournament_games.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_games(&mut self, v: bool) {
        self.tournament_games = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_games(&self) -> bool {
        self.tournament_games.unwrap_or(false)
    }

    fn get_tournament_games_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.tournament_games
    }

    fn mut_tournament_games_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.tournament_games
    }

    // optional string pass_key = 2;

    pub fn clear_pass_key(&mut self) {
        self.pass_key.clear();
    }

    pub fn has_pass_key(&self) -> bool {
        self.pass_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pass_key(&mut self, v: ::std::string::String) {
        self.pass_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pass_key(&mut self) -> &mut ::std::string::String {
        if self.pass_key.is_none() {
            self.pass_key.set_default();
        };
        self.pass_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_pass_key(&mut self) -> ::std::string::String {
        self.pass_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pass_key(&self) -> &str {
        match self.pass_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_pass_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.pass_key
    }

    fn mut_pass_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.pass_key
    }

    // optional uint32 region = 3;

    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None;
    }

    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region(&mut self, v: u32) {
        self.region = ::std::option::Option::Some(v);
    }

    pub fn get_region(&self) -> u32 {
        self.region.unwrap_or(0)
    }

    fn get_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.region
    }

    fn mut_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.region
    }

    // optional .dota.DOTA_GameMode game_mode = 4;

    pub fn clear_game_mode(&mut self) {
        self.game_mode = ::std::option::Option::None;
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: super::dota_shared_enums::DOTA_GameMode) {
        self.game_mode = ::std::option::Option::Some(v);
    }

    pub fn get_game_mode(&self) -> super::dota_shared_enums::DOTA_GameMode {
        self.game_mode.unwrap_or(super::dota_shared_enums::DOTA_GameMode::DOTA_GAMEMODE_NONE)
    }

    fn get_game_mode_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GameMode> {
        &self.game_mode
    }

    fn mut_game_mode_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GameMode> {
        &mut self.game_mode
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyList {
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
                    self.tournament_games = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pass_key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.region = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tournament_games {
            my_size += 2;
        };
        if let Some(v) = self.pass_key.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.region {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::enum_size(4, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tournament_games {
            os.write_bool(1, v)?;
        };
        if let Some(v) = self.pass_key.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.region {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.game_mode {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyList {
    fn new() -> CMsgPracticeLobbyList {
        CMsgPracticeLobbyList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "tournament_games",
                    CMsgPracticeLobbyList::get_tournament_games_for_reflect,
                    CMsgPracticeLobbyList::mut_tournament_games_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pass_key",
                    CMsgPracticeLobbyList::get_pass_key_for_reflect,
                    CMsgPracticeLobbyList::mut_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region",
                    CMsgPracticeLobbyList::get_region_for_reflect,
                    CMsgPracticeLobbyList::mut_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GameMode>>(
                    "game_mode",
                    CMsgPracticeLobbyList::get_game_mode_for_reflect,
                    CMsgPracticeLobbyList::mut_game_mode_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyList>(
                    "CMsgPracticeLobbyList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyList {
    fn clear(&mut self) {
        self.clear_tournament_games();
        self.clear_pass_key();
        self.clear_region();
        self.clear_game_mode();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyListResponseEntry {
    // message fields
    id: ::std::option::Option<u64>,
    tournament_id: ::std::option::Option<u32>,
    tournament_game_id: ::std::option::Option<u32>,
    members: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry_CLobbyMember>,
    requires_pass_key: ::std::option::Option<bool>,
    leader_account_id: ::std::option::Option<u32>,
    guild_id: ::std::option::Option<u32>,
    guild_logo: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::string::String>,
    custom_game_mode: ::protobuf::SingularField<::std::string::String>,
    game_mode: ::std::option::Option<super::dota_shared_enums::DOTA_GameMode>,
    friend_present: ::std::option::Option<bool>,
    players: ::std::option::Option<u32>,
    custom_map_name: ::protobuf::SingularField<::std::string::String>,
    max_player_count: ::std::option::Option<u32>,
    server_region: ::std::option::Option<u32>,
    lan_host_ping_to_server_region: ::std::option::Option<u32>,
    league_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyListResponseEntry {}

impl CMsgPracticeLobbyListResponseEntry {
    pub fn new() -> CMsgPracticeLobbyListResponseEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyListResponseEntry {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyListResponseEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyListResponseEntry,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyListResponseEntry::new)
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

    // optional uint32 tournament_id = 3;

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

    // optional uint32 tournament_game_id = 4;

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

    // repeated .dota.CMsgPracticeLobbyListResponseEntry.CLobbyMember members = 5;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry_CLobbyMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry_CLobbyMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry_CLobbyMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CMsgPracticeLobbyListResponseEntry_CLobbyMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry_CLobbyMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry_CLobbyMember> {
        &mut self.members
    }

    // optional bool requires_pass_key = 6;

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

    // optional uint32 leader_account_id = 7;

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

    // optional uint32 guild_id = 8;

    pub fn clear_guild_id(&mut self) {
        self.guild_id = ::std::option::Option::None;
    }

    pub fn has_guild_id(&self) -> bool {
        self.guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_id(&mut self, v: u32) {
        self.guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_guild_id(&self) -> u32 {
        self.guild_id.unwrap_or(0)
    }

    fn get_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.guild_id
    }

    fn mut_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.guild_id
    }

    // optional uint64 guild_logo = 9;

    pub fn clear_guild_logo(&mut self) {
        self.guild_logo = ::std::option::Option::None;
    }

    pub fn has_guild_logo(&self) -> bool {
        self.guild_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_logo(&mut self, v: u64) {
        self.guild_logo = ::std::option::Option::Some(v);
    }

    pub fn get_guild_logo(&self) -> u64 {
        self.guild_logo.unwrap_or(0)
    }

    fn get_guild_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.guild_logo
    }

    fn mut_guild_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.guild_logo
    }

    // optional string name = 10;

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

    // optional string custom_game_mode = 11;

    pub fn clear_custom_game_mode(&mut self) {
        self.custom_game_mode.clear();
    }

    pub fn has_custom_game_mode(&self) -> bool {
        self.custom_game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_mode(&mut self, v: ::std::string::String) {
        self.custom_game_mode = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_custom_game_mode(&mut self) -> &mut ::std::string::String {
        if self.custom_game_mode.is_none() {
            self.custom_game_mode.set_default();
        };
        self.custom_game_mode.as_mut().unwrap()
    }

    // Take field
    pub fn take_custom_game_mode(&mut self) -> ::std::string::String {
        self.custom_game_mode.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_custom_game_mode(&self) -> &str {
        match self.custom_game_mode.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_custom_game_mode_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.custom_game_mode
    }

    fn mut_custom_game_mode_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.custom_game_mode
    }

    // optional .dota.DOTA_GameMode game_mode = 12;

    pub fn clear_game_mode(&mut self) {
        self.game_mode = ::std::option::Option::None;
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: super::dota_shared_enums::DOTA_GameMode) {
        self.game_mode = ::std::option::Option::Some(v);
    }

    pub fn get_game_mode(&self) -> super::dota_shared_enums::DOTA_GameMode {
        self.game_mode.unwrap_or(super::dota_shared_enums::DOTA_GameMode::DOTA_GAMEMODE_NONE)
    }

    fn get_game_mode_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GameMode> {
        &self.game_mode
    }

    fn mut_game_mode_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GameMode> {
        &mut self.game_mode
    }

    // optional bool friend_present = 13;

    pub fn clear_friend_present(&mut self) {
        self.friend_present = ::std::option::Option::None;
    }

    pub fn has_friend_present(&self) -> bool {
        self.friend_present.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friend_present(&mut self, v: bool) {
        self.friend_present = ::std::option::Option::Some(v);
    }

    pub fn get_friend_present(&self) -> bool {
        self.friend_present.unwrap_or(false)
    }

    fn get_friend_present_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.friend_present
    }

    fn mut_friend_present_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.friend_present
    }

    // optional uint32 players = 14;

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

    // optional string custom_map_name = 15;

    pub fn clear_custom_map_name(&mut self) {
        self.custom_map_name.clear();
    }

    pub fn has_custom_map_name(&self) -> bool {
        self.custom_map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_map_name(&mut self, v: ::std::string::String) {
        self.custom_map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_custom_map_name(&mut self) -> &mut ::std::string::String {
        if self.custom_map_name.is_none() {
            self.custom_map_name.set_default();
        };
        self.custom_map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_custom_map_name(&mut self) -> ::std::string::String {
        self.custom_map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_custom_map_name(&self) -> &str {
        match self.custom_map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_custom_map_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.custom_map_name
    }

    fn mut_custom_map_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.custom_map_name
    }

    // optional uint32 max_player_count = 16;

    pub fn clear_max_player_count(&mut self) {
        self.max_player_count = ::std::option::Option::None;
    }

    pub fn has_max_player_count(&self) -> bool {
        self.max_player_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_player_count(&mut self, v: u32) {
        self.max_player_count = ::std::option::Option::Some(v);
    }

    pub fn get_max_player_count(&self) -> u32 {
        self.max_player_count.unwrap_or(0)
    }

    fn get_max_player_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_player_count
    }

    fn mut_max_player_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_player_count
    }

    // optional uint32 server_region = 17;

    pub fn clear_server_region(&mut self) {
        self.server_region = ::std::option::Option::None;
    }

    pub fn has_server_region(&self) -> bool {
        self.server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_region(&mut self, v: u32) {
        self.server_region = ::std::option::Option::Some(v);
    }

    pub fn get_server_region(&self) -> u32 {
        self.server_region.unwrap_or(0)
    }

    fn get_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_region
    }

    fn mut_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_region
    }

    // optional uint32 lan_host_ping_to_server_region = 18;

    pub fn clear_lan_host_ping_to_server_region(&mut self) {
        self.lan_host_ping_to_server_region = ::std::option::Option::None;
    }

    pub fn has_lan_host_ping_to_server_region(&self) -> bool {
        self.lan_host_ping_to_server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lan_host_ping_to_server_region(&mut self, v: u32) {
        self.lan_host_ping_to_server_region = ::std::option::Option::Some(v);
    }

    pub fn get_lan_host_ping_to_server_region(&self) -> u32 {
        self.lan_host_ping_to_server_region.unwrap_or(0)
    }

    fn get_lan_host_ping_to_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lan_host_ping_to_server_region
    }

    fn mut_lan_host_ping_to_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lan_host_ping_to_server_region
    }

    // optional uint32 league_id = 19;

    pub fn clear_league_id(&mut self) {
        self.league_id = ::std::option::Option::None;
    }

    pub fn has_league_id(&self) -> bool {
        self.league_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_id(&mut self, v: u32) {
        self.league_id = ::std::option::Option::Some(v);
    }

    pub fn get_league_id(&self) -> u32 {
        self.league_id.unwrap_or(0)
    }

    fn get_league_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_id
    }

    fn mut_league_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_id
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyListResponseEntry {
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
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tournament_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tournament_game_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.requires_pass_key = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.leader_account_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.guild_logo = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.custom_game_mode)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.friend_present = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.players = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.custom_map_name)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_player_count = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.server_region = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lan_host_ping_to_server_region = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_id = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tournament_game_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.requires_pass_key {
            my_size += 2;
        };
        if let Some(v) = self.leader_account_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.guild_logo {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        };
        if let Some(v) = self.custom_game_mode.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::enum_size(12, v);
        };
        if let Some(v) = self.friend_present {
            my_size += 2;
        };
        if let Some(v) = self.players {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        };
        if let Some(v) = self.max_player_count {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.server_region {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lan_host_ping_to_server_region {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
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
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.tournament_game_id {
            os.write_uint32(4, v)?;
        };
        for v in &self.members {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.requires_pass_key {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.leader_account_id {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.guild_id {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.guild_logo {
            os.write_uint64(9, v)?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(10, &v)?;
        };
        if let Some(v) = self.custom_game_mode.as_ref() {
            os.write_string(11, &v)?;
        };
        if let Some(v) = self.game_mode {
            os.write_enum(12, v.value())?;
        };
        if let Some(v) = self.friend_present {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.players {
            os.write_uint32(14, v)?;
        };
        if let Some(v) = self.custom_map_name.as_ref() {
            os.write_string(15, &v)?;
        };
        if let Some(v) = self.max_player_count {
            os.write_uint32(16, v)?;
        };
        if let Some(v) = self.server_region {
            os.write_uint32(17, v)?;
        };
        if let Some(v) = self.lan_host_ping_to_server_region {
            os.write_uint32(18, v)?;
        };
        if let Some(v) = self.league_id {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyListResponseEntry {
    fn new() -> CMsgPracticeLobbyListResponseEntry {
        CMsgPracticeLobbyListResponseEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyListResponseEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    CMsgPracticeLobbyListResponseEntry::get_id_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_id",
                    CMsgPracticeLobbyListResponseEntry::get_tournament_id_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_game_id",
                    CMsgPracticeLobbyListResponseEntry::get_tournament_game_id_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_tournament_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPracticeLobbyListResponseEntry_CLobbyMember>>(
                    "members",
                    CMsgPracticeLobbyListResponseEntry::get_members_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "requires_pass_key",
                    CMsgPracticeLobbyListResponseEntry::get_requires_pass_key_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_requires_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leader_account_id",
                    CMsgPracticeLobbyListResponseEntry::get_leader_account_id_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_leader_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CMsgPracticeLobbyListResponseEntry::get_guild_id_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "guild_logo",
                    CMsgPracticeLobbyListResponseEntry::get_guild_logo_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_guild_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgPracticeLobbyListResponseEntry::get_name_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "custom_game_mode",
                    CMsgPracticeLobbyListResponseEntry::get_custom_game_mode_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_custom_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GameMode>>(
                    "game_mode",
                    CMsgPracticeLobbyListResponseEntry::get_game_mode_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "friend_present",
                    CMsgPracticeLobbyListResponseEntry::get_friend_present_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_friend_present_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "players",
                    CMsgPracticeLobbyListResponseEntry::get_players_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "custom_map_name",
                    CMsgPracticeLobbyListResponseEntry::get_custom_map_name_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_custom_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_player_count",
                    CMsgPracticeLobbyListResponseEntry::get_max_player_count_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_max_player_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_region",
                    CMsgPracticeLobbyListResponseEntry::get_server_region_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lan_host_ping_to_server_region",
                    CMsgPracticeLobbyListResponseEntry::get_lan_host_ping_to_server_region_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_lan_host_ping_to_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgPracticeLobbyListResponseEntry::get_league_id_for_reflect,
                    CMsgPracticeLobbyListResponseEntry::mut_league_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyListResponseEntry>(
                    "CMsgPracticeLobbyListResponseEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyListResponseEntry {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_tournament_id();
        self.clear_tournament_game_id();
        self.clear_members();
        self.clear_requires_pass_key();
        self.clear_leader_account_id();
        self.clear_guild_id();
        self.clear_guild_logo();
        self.clear_name();
        self.clear_custom_game_mode();
        self.clear_game_mode();
        self.clear_friend_present();
        self.clear_players();
        self.clear_custom_map_name();
        self.clear_max_player_count();
        self.clear_server_region();
        self.clear_lan_host_ping_to_server_region();
        self.clear_league_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyListResponseEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyListResponseEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyListResponseEntry_CLobbyMember {
    // message fields
    account_id: ::std::option::Option<u32>,
    player_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyListResponseEntry_CLobbyMember {}

impl CMsgPracticeLobbyListResponseEntry_CLobbyMember {
    pub fn new() -> CMsgPracticeLobbyListResponseEntry_CLobbyMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyListResponseEntry_CLobbyMember {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyListResponseEntry_CLobbyMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyListResponseEntry_CLobbyMember,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyListResponseEntry_CLobbyMember::new)
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

impl ::protobuf::Message for CMsgPracticeLobbyListResponseEntry_CLobbyMember {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyListResponseEntry_CLobbyMember {
    fn new() -> CMsgPracticeLobbyListResponseEntry_CLobbyMember {
        CMsgPracticeLobbyListResponseEntry_CLobbyMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyListResponseEntry_CLobbyMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgPracticeLobbyListResponseEntry_CLobbyMember::get_account_id_for_reflect,
                    CMsgPracticeLobbyListResponseEntry_CLobbyMember::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "player_name",
                    CMsgPracticeLobbyListResponseEntry_CLobbyMember::get_player_name_for_reflect,
                    CMsgPracticeLobbyListResponseEntry_CLobbyMember::mut_player_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyListResponseEntry_CLobbyMember>(
                    "CMsgPracticeLobbyListResponseEntry_CLobbyMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyListResponseEntry_CLobbyMember {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_player_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyListResponseEntry_CLobbyMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyListResponseEntry_CLobbyMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyListResponse {
    // message fields
    tournament_games: ::std::option::Option<bool>,
    lobbies: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyListResponse {}

impl CMsgPracticeLobbyListResponse {
    pub fn new() -> CMsgPracticeLobbyListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyListResponse,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyListResponse::new)
        }
    }

    // optional bool tournament_games = 1;

    pub fn clear_tournament_games(&mut self) {
        self.tournament_games = ::std::option::Option::None;
    }

    pub fn has_tournament_games(&self) -> bool {
        self.tournament_games.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tournament_games(&mut self, v: bool) {
        self.tournament_games = ::std::option::Option::Some(v);
    }

    pub fn get_tournament_games(&self) -> bool {
        self.tournament_games.unwrap_or(false)
    }

    fn get_tournament_games_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.tournament_games
    }

    fn mut_tournament_games_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.tournament_games
    }

    // repeated .dota.CMsgPracticeLobbyListResponseEntry lobbies = 2;

    pub fn clear_lobbies(&mut self) {
        self.lobbies.clear();
    }

    // Param is passed by value, moved
    pub fn set_lobbies(&mut self, v: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry>) {
        self.lobbies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lobbies(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &mut self.lobbies
    }

    // Take field
    pub fn take_lobbies(&mut self) -> ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        ::std::mem::replace(&mut self.lobbies, ::protobuf::RepeatedField::new())
    }

    pub fn get_lobbies(&self) -> &[CMsgPracticeLobbyListResponseEntry] {
        &self.lobbies
    }

    fn get_lobbies_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &self.lobbies
    }

    fn mut_lobbies_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &mut self.lobbies
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyListResponse {
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
                    self.tournament_games = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lobbies)?;
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
        if let Some(v) = self.tournament_games {
            my_size += 2;
        };
        for value in &self.lobbies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tournament_games {
            os.write_bool(1, v)?;
        };
        for v in &self.lobbies {
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyListResponse {
    fn new() -> CMsgPracticeLobbyListResponse {
        CMsgPracticeLobbyListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "tournament_games",
                    CMsgPracticeLobbyListResponse::get_tournament_games_for_reflect,
                    CMsgPracticeLobbyListResponse::mut_tournament_games_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPracticeLobbyListResponseEntry>>(
                    "lobbies",
                    CMsgPracticeLobbyListResponse::get_lobbies_for_reflect,
                    CMsgPracticeLobbyListResponse::mut_lobbies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyListResponse>(
                    "CMsgPracticeLobbyListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyListResponse {
    fn clear(&mut self) {
        self.clear_tournament_games();
        self.clear_lobbies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgLobbyList {
    // message fields
    server_region: ::std::option::Option<u32>,
    game_mode: ::std::option::Option<super::dota_shared_enums::DOTA_GameMode>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgLobbyList {}

impl CMsgLobbyList {
    pub fn new() -> CMsgLobbyList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgLobbyList {
        static mut instance: ::protobuf::lazy::Lazy<CMsgLobbyList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgLobbyList,
        };
        unsafe {
            instance.get(CMsgLobbyList::new)
        }
    }

    // optional uint32 server_region = 1;

    pub fn clear_server_region(&mut self) {
        self.server_region = ::std::option::Option::None;
    }

    pub fn has_server_region(&self) -> bool {
        self.server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_region(&mut self, v: u32) {
        self.server_region = ::std::option::Option::Some(v);
    }

    pub fn get_server_region(&self) -> u32 {
        self.server_region.unwrap_or(0u32)
    }

    fn get_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_region
    }

    fn mut_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_region
    }

    // optional .dota.DOTA_GameMode game_mode = 2;

    pub fn clear_game_mode(&mut self) {
        self.game_mode = ::std::option::Option::None;
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: super::dota_shared_enums::DOTA_GameMode) {
        self.game_mode = ::std::option::Option::Some(v);
    }

    pub fn get_game_mode(&self) -> super::dota_shared_enums::DOTA_GameMode {
        self.game_mode.unwrap_or(super::dota_shared_enums::DOTA_GameMode::DOTA_GAMEMODE_NONE)
    }

    fn get_game_mode_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GameMode> {
        &self.game_mode
    }

    fn mut_game_mode_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GameMode> {
        &mut self.game_mode
    }
}

impl ::protobuf::Message for CMsgLobbyList {
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
                    self.server_region = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.server_region {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_region {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.game_mode {
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

impl ::protobuf::MessageStatic for CMsgLobbyList {
    fn new() -> CMsgLobbyList {
        CMsgLobbyList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgLobbyList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_region",
                    CMsgLobbyList::get_server_region_for_reflect,
                    CMsgLobbyList::mut_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GameMode>>(
                    "game_mode",
                    CMsgLobbyList::get_game_mode_for_reflect,
                    CMsgLobbyList::mut_game_mode_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgLobbyList>(
                    "CMsgLobbyList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgLobbyList {
    fn clear(&mut self) {
        self.clear_server_region();
        self.clear_game_mode();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgLobbyList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgLobbyList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgLobbyListResponse {
    // message fields
    lobbies: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgLobbyListResponse {}

impl CMsgLobbyListResponse {
    pub fn new() -> CMsgLobbyListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgLobbyListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgLobbyListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgLobbyListResponse,
        };
        unsafe {
            instance.get(CMsgLobbyListResponse::new)
        }
    }

    // repeated .dota.CMsgPracticeLobbyListResponseEntry lobbies = 1;

    pub fn clear_lobbies(&mut self) {
        self.lobbies.clear();
    }

    // Param is passed by value, moved
    pub fn set_lobbies(&mut self, v: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry>) {
        self.lobbies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lobbies(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &mut self.lobbies
    }

    // Take field
    pub fn take_lobbies(&mut self) -> ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        ::std::mem::replace(&mut self.lobbies, ::protobuf::RepeatedField::new())
    }

    pub fn get_lobbies(&self) -> &[CMsgPracticeLobbyListResponseEntry] {
        &self.lobbies
    }

    fn get_lobbies_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &self.lobbies
    }

    fn mut_lobbies_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &mut self.lobbies
    }
}

impl ::protobuf::Message for CMsgLobbyListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lobbies)?;
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
        for value in &self.lobbies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.lobbies {
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

impl ::protobuf::MessageStatic for CMsgLobbyListResponse {
    fn new() -> CMsgLobbyListResponse {
        CMsgLobbyListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgLobbyListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPracticeLobbyListResponseEntry>>(
                    "lobbies",
                    CMsgLobbyListResponse::get_lobbies_for_reflect,
                    CMsgLobbyListResponse::mut_lobbies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgLobbyListResponse>(
                    "CMsgLobbyListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgLobbyListResponse {
    fn clear(&mut self) {
        self.clear_lobbies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgLobbyListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgLobbyListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyJoin {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    client_version: ::std::option::Option<u32>,
    pass_key: ::protobuf::SingularField<::std::string::String>,
    custom_game_crc: ::std::option::Option<u64>,
    custom_game_timestamp: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyJoin {}

impl CMsgPracticeLobbyJoin {
    pub fn new() -> CMsgPracticeLobbyJoin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyJoin {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyJoin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyJoin,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyJoin::new)
        }
    }

    // optional uint64 lobby_id = 1;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }

    // optional uint32 client_version = 2;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional string pass_key = 3;

    pub fn clear_pass_key(&mut self) {
        self.pass_key.clear();
    }

    pub fn has_pass_key(&self) -> bool {
        self.pass_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pass_key(&mut self, v: ::std::string::String) {
        self.pass_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pass_key(&mut self) -> &mut ::std::string::String {
        if self.pass_key.is_none() {
            self.pass_key.set_default();
        };
        self.pass_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_pass_key(&mut self) -> ::std::string::String {
        self.pass_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pass_key(&self) -> &str {
        match self.pass_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_pass_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.pass_key
    }

    fn mut_pass_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.pass_key
    }

    // optional fixed64 custom_game_crc = 4;

    pub fn clear_custom_game_crc(&mut self) {
        self.custom_game_crc = ::std::option::Option::None;
    }

    pub fn has_custom_game_crc(&self) -> bool {
        self.custom_game_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_crc(&mut self, v: u64) {
        self.custom_game_crc = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_crc(&self) -> u64 {
        self.custom_game_crc.unwrap_or(0)
    }

    fn get_custom_game_crc_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_crc
    }

    fn mut_custom_game_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_crc
    }

    // optional fixed32 custom_game_timestamp = 5;

    pub fn clear_custom_game_timestamp(&mut self) {
        self.custom_game_timestamp = ::std::option::Option::None;
    }

    pub fn has_custom_game_timestamp(&self) -> bool {
        self.custom_game_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_timestamp(&mut self, v: u32) {
        self.custom_game_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_timestamp(&self) -> u32 {
        self.custom_game_timestamp.unwrap_or(0)
    }

    fn get_custom_game_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.custom_game_timestamp
    }

    fn mut_custom_game_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.custom_game_timestamp
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyJoin {
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
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pass_key)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.custom_game_crc = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.custom_game_timestamp = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.lobby_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pass_key.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.custom_game_crc {
            my_size += 9;
        };
        if let Some(v) = self.custom_game_timestamp {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.pass_key.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.custom_game_crc {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.custom_game_timestamp {
            os.write_fixed32(5, v)?;
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyJoin {
    fn new() -> CMsgPracticeLobbyJoin {
        CMsgPracticeLobbyJoin::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyJoin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lobby_id",
                    CMsgPracticeLobbyJoin::get_lobby_id_for_reflect,
                    CMsgPracticeLobbyJoin::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgPracticeLobbyJoin::get_client_version_for_reflect,
                    CMsgPracticeLobbyJoin::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pass_key",
                    CMsgPracticeLobbyJoin::get_pass_key_for_reflect,
                    CMsgPracticeLobbyJoin::mut_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "custom_game_crc",
                    CMsgPracticeLobbyJoin::get_custom_game_crc_for_reflect,
                    CMsgPracticeLobbyJoin::mut_custom_game_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "custom_game_timestamp",
                    CMsgPracticeLobbyJoin::get_custom_game_timestamp_for_reflect,
                    CMsgPracticeLobbyJoin::mut_custom_game_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyJoin>(
                    "CMsgPracticeLobbyJoin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyJoin {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.clear_client_version();
        self.clear_pass_key();
        self.clear_custom_game_crc();
        self.clear_custom_game_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyJoin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyJoin {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPracticeLobbyJoinResponse {
    // message fields
    result: ::std::option::Option<super::dota_shared_enums::DOTAJoinLobbyResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPracticeLobbyJoinResponse {}

impl CMsgPracticeLobbyJoinResponse {
    pub fn new() -> CMsgPracticeLobbyJoinResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPracticeLobbyJoinResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPracticeLobbyJoinResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPracticeLobbyJoinResponse,
        };
        unsafe {
            instance.get(CMsgPracticeLobbyJoinResponse::new)
        }
    }

    // optional .dota.DOTAJoinLobbyResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: super::dota_shared_enums::DOTAJoinLobbyResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> super::dota_shared_enums::DOTAJoinLobbyResult {
        self.result.unwrap_or(super::dota_shared_enums::DOTAJoinLobbyResult::DOTA_JOIN_RESULT_SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTAJoinLobbyResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTAJoinLobbyResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgPracticeLobbyJoinResponse {
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgPracticeLobbyJoinResponse {
    fn new() -> CMsgPracticeLobbyJoinResponse {
        CMsgPracticeLobbyJoinResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPracticeLobbyJoinResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAJoinLobbyResult>>(
                    "result",
                    CMsgPracticeLobbyJoinResponse::get_result_for_reflect,
                    CMsgPracticeLobbyJoinResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPracticeLobbyJoinResponse>(
                    "CMsgPracticeLobbyJoinResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPracticeLobbyJoinResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPracticeLobbyJoinResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPracticeLobbyJoinResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgFriendPracticeLobbyListRequest {
    // message fields
    friends: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgFriendPracticeLobbyListRequest {}

impl CMsgFriendPracticeLobbyListRequest {
    pub fn new() -> CMsgFriendPracticeLobbyListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgFriendPracticeLobbyListRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgFriendPracticeLobbyListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgFriendPracticeLobbyListRequest,
        };
        unsafe {
            instance.get(CMsgFriendPracticeLobbyListRequest::new)
        }
    }

    // repeated uint32 friends = 1;

    pub fn clear_friends(&mut self) {
        self.friends.clear();
    }

    // Param is passed by value, moved
    pub fn set_friends(&mut self, v: ::std::vec::Vec<u32>) {
        self.friends = v;
    }

    // Mutable pointer to the field.
    pub fn mut_friends(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.friends
    }

    // Take field
    pub fn take_friends(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.friends, ::std::vec::Vec::new())
    }

    pub fn get_friends(&self) -> &[u32] {
        &self.friends
    }

    fn get_friends_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.friends
    }

    fn mut_friends_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.friends
    }
}

impl ::protobuf::Message for CMsgFriendPracticeLobbyListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.friends)?;
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
        for value in &self.friends {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.friends {
            os.write_uint32(1, *v)?;
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

impl ::protobuf::MessageStatic for CMsgFriendPracticeLobbyListRequest {
    fn new() -> CMsgFriendPracticeLobbyListRequest {
        CMsgFriendPracticeLobbyListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgFriendPracticeLobbyListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "friends",
                    CMsgFriendPracticeLobbyListRequest::get_friends_for_reflect,
                    CMsgFriendPracticeLobbyListRequest::mut_friends_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgFriendPracticeLobbyListRequest>(
                    "CMsgFriendPracticeLobbyListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgFriendPracticeLobbyListRequest {
    fn clear(&mut self) {
        self.clear_friends();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgFriendPracticeLobbyListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgFriendPracticeLobbyListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgFriendPracticeLobbyListResponse {
    // message fields
    lobbies: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgFriendPracticeLobbyListResponse {}

impl CMsgFriendPracticeLobbyListResponse {
    pub fn new() -> CMsgFriendPracticeLobbyListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgFriendPracticeLobbyListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgFriendPracticeLobbyListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgFriendPracticeLobbyListResponse,
        };
        unsafe {
            instance.get(CMsgFriendPracticeLobbyListResponse::new)
        }
    }

    // repeated .dota.CMsgPracticeLobbyListResponseEntry lobbies = 1;

    pub fn clear_lobbies(&mut self) {
        self.lobbies.clear();
    }

    // Param is passed by value, moved
    pub fn set_lobbies(&mut self, v: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry>) {
        self.lobbies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lobbies(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &mut self.lobbies
    }

    // Take field
    pub fn take_lobbies(&mut self) -> ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        ::std::mem::replace(&mut self.lobbies, ::protobuf::RepeatedField::new())
    }

    pub fn get_lobbies(&self) -> &[CMsgPracticeLobbyListResponseEntry] {
        &self.lobbies
    }

    fn get_lobbies_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &self.lobbies
    }

    fn mut_lobbies_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &mut self.lobbies
    }
}

impl ::protobuf::Message for CMsgFriendPracticeLobbyListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lobbies)?;
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
        for value in &self.lobbies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.lobbies {
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

impl ::protobuf::MessageStatic for CMsgFriendPracticeLobbyListResponse {
    fn new() -> CMsgFriendPracticeLobbyListResponse {
        CMsgFriendPracticeLobbyListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgFriendPracticeLobbyListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPracticeLobbyListResponseEntry>>(
                    "lobbies",
                    CMsgFriendPracticeLobbyListResponse::get_lobbies_for_reflect,
                    CMsgFriendPracticeLobbyListResponse::mut_lobbies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgFriendPracticeLobbyListResponse>(
                    "CMsgFriendPracticeLobbyListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgFriendPracticeLobbyListResponse {
    fn clear(&mut self) {
        self.clear_lobbies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgFriendPracticeLobbyListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgFriendPracticeLobbyListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGuildmatePracticeLobbyListRequest {
    // message fields
    guilds: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGuildmatePracticeLobbyListRequest {}

impl CMsgGuildmatePracticeLobbyListRequest {
    pub fn new() -> CMsgGuildmatePracticeLobbyListRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGuildmatePracticeLobbyListRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGuildmatePracticeLobbyListRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGuildmatePracticeLobbyListRequest,
        };
        unsafe {
            instance.get(CMsgGuildmatePracticeLobbyListRequest::new)
        }
    }

    // repeated uint32 guilds = 1;

    pub fn clear_guilds(&mut self) {
        self.guilds.clear();
    }

    // Param is passed by value, moved
    pub fn set_guilds(&mut self, v: ::std::vec::Vec<u32>) {
        self.guilds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_guilds(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.guilds
    }

    // Take field
    pub fn take_guilds(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.guilds, ::std::vec::Vec::new())
    }

    pub fn get_guilds(&self) -> &[u32] {
        &self.guilds
    }

    fn get_guilds_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.guilds
    }

    fn mut_guilds_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.guilds
    }
}

impl ::protobuf::Message for CMsgGuildmatePracticeLobbyListRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.guilds)?;
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
        for value in &self.guilds {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.guilds {
            os.write_uint32(1, *v)?;
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

impl ::protobuf::MessageStatic for CMsgGuildmatePracticeLobbyListRequest {
    fn new() -> CMsgGuildmatePracticeLobbyListRequest {
        CMsgGuildmatePracticeLobbyListRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGuildmatePracticeLobbyListRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guilds",
                    CMsgGuildmatePracticeLobbyListRequest::get_guilds_for_reflect,
                    CMsgGuildmatePracticeLobbyListRequest::mut_guilds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGuildmatePracticeLobbyListRequest>(
                    "CMsgGuildmatePracticeLobbyListRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGuildmatePracticeLobbyListRequest {
    fn clear(&mut self) {
        self.clear_guilds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGuildmatePracticeLobbyListRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGuildmatePracticeLobbyListRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGuildmatePracticeLobbyListResponse {
    // message fields
    lobbies: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGuildmatePracticeLobbyListResponse {}

impl CMsgGuildmatePracticeLobbyListResponse {
    pub fn new() -> CMsgGuildmatePracticeLobbyListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGuildmatePracticeLobbyListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGuildmatePracticeLobbyListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGuildmatePracticeLobbyListResponse,
        };
        unsafe {
            instance.get(CMsgGuildmatePracticeLobbyListResponse::new)
        }
    }

    // repeated .dota.CMsgPracticeLobbyListResponseEntry lobbies = 1;

    pub fn clear_lobbies(&mut self) {
        self.lobbies.clear();
    }

    // Param is passed by value, moved
    pub fn set_lobbies(&mut self, v: ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry>) {
        self.lobbies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lobbies(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &mut self.lobbies
    }

    // Take field
    pub fn take_lobbies(&mut self) -> ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        ::std::mem::replace(&mut self.lobbies, ::protobuf::RepeatedField::new())
    }

    pub fn get_lobbies(&self) -> &[CMsgPracticeLobbyListResponseEntry] {
        &self.lobbies
    }

    fn get_lobbies_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &self.lobbies
    }

    fn mut_lobbies_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPracticeLobbyListResponseEntry> {
        &mut self.lobbies
    }
}

impl ::protobuf::Message for CMsgGuildmatePracticeLobbyListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lobbies)?;
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
        for value in &self.lobbies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.lobbies {
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

impl ::protobuf::MessageStatic for CMsgGuildmatePracticeLobbyListResponse {
    fn new() -> CMsgGuildmatePracticeLobbyListResponse {
        CMsgGuildmatePracticeLobbyListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGuildmatePracticeLobbyListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPracticeLobbyListResponseEntry>>(
                    "lobbies",
                    CMsgGuildmatePracticeLobbyListResponse::get_lobbies_for_reflect,
                    CMsgGuildmatePracticeLobbyListResponse::mut_lobbies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGuildmatePracticeLobbyListResponse>(
                    "CMsgGuildmatePracticeLobbyListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGuildmatePracticeLobbyListResponse {
    fn clear(&mut self) {
        self.clear_lobbies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGuildmatePracticeLobbyListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGuildmatePracticeLobbyListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgJoinableCustomGameModesRequest {
    // message fields
    server_region: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgJoinableCustomGameModesRequest {}

impl CMsgJoinableCustomGameModesRequest {
    pub fn new() -> CMsgJoinableCustomGameModesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgJoinableCustomGameModesRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgJoinableCustomGameModesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgJoinableCustomGameModesRequest,
        };
        unsafe {
            instance.get(CMsgJoinableCustomGameModesRequest::new)
        }
    }

    // optional uint32 server_region = 1;

    pub fn clear_server_region(&mut self) {
        self.server_region = ::std::option::Option::None;
    }

    pub fn has_server_region(&self) -> bool {
        self.server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_region(&mut self, v: u32) {
        self.server_region = ::std::option::Option::Some(v);
    }

    pub fn get_server_region(&self) -> u32 {
        self.server_region.unwrap_or(0)
    }

    fn get_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_region
    }

    fn mut_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_region
    }
}

impl ::protobuf::Message for CMsgJoinableCustomGameModesRequest {
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
                    self.server_region = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.server_region {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_region {
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

impl ::protobuf::MessageStatic for CMsgJoinableCustomGameModesRequest {
    fn new() -> CMsgJoinableCustomGameModesRequest {
        CMsgJoinableCustomGameModesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgJoinableCustomGameModesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_region",
                    CMsgJoinableCustomGameModesRequest::get_server_region_for_reflect,
                    CMsgJoinableCustomGameModesRequest::mut_server_region_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgJoinableCustomGameModesRequest>(
                    "CMsgJoinableCustomGameModesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgJoinableCustomGameModesRequest {
    fn clear(&mut self) {
        self.clear_server_region();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgJoinableCustomGameModesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgJoinableCustomGameModesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgJoinableCustomGameModesResponseEntry {
    // message fields
    custom_game_id: ::std::option::Option<u64>,
    lobby_count: ::std::option::Option<u32>,
    player_count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgJoinableCustomGameModesResponseEntry {}

impl CMsgJoinableCustomGameModesResponseEntry {
    pub fn new() -> CMsgJoinableCustomGameModesResponseEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgJoinableCustomGameModesResponseEntry {
        static mut instance: ::protobuf::lazy::Lazy<CMsgJoinableCustomGameModesResponseEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgJoinableCustomGameModesResponseEntry,
        };
        unsafe {
            instance.get(CMsgJoinableCustomGameModesResponseEntry::new)
        }
    }

    // optional uint64 custom_game_id = 1;

    pub fn clear_custom_game_id(&mut self) {
        self.custom_game_id = ::std::option::Option::None;
    }

    pub fn has_custom_game_id(&self) -> bool {
        self.custom_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_id(&mut self, v: u64) {
        self.custom_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_id(&self) -> u64 {
        self.custom_game_id.unwrap_or(0)
    }

    fn get_custom_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_id
    }

    fn mut_custom_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_id
    }

    // optional uint32 lobby_count = 2;

    pub fn clear_lobby_count(&mut self) {
        self.lobby_count = ::std::option::Option::None;
    }

    pub fn has_lobby_count(&self) -> bool {
        self.lobby_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_count(&mut self, v: u32) {
        self.lobby_count = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_count(&self) -> u32 {
        self.lobby_count.unwrap_or(0)
    }

    fn get_lobby_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lobby_count
    }

    fn mut_lobby_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lobby_count
    }

    // optional uint32 player_count = 3;

    pub fn clear_player_count(&mut self) {
        self.player_count = ::std::option::Option::None;
    }

    pub fn has_player_count(&self) -> bool {
        self.player_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_count(&mut self, v: u32) {
        self.player_count = ::std::option::Option::Some(v);
    }

    pub fn get_player_count(&self) -> u32 {
        self.player_count.unwrap_or(0)
    }

    fn get_player_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_count
    }

    fn mut_player_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_count
    }
}

impl ::protobuf::Message for CMsgJoinableCustomGameModesResponseEntry {
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
                    self.custom_game_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lobby_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.player_count = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lobby_count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.player_count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.custom_game_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.lobby_count {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.player_count {
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

impl ::protobuf::MessageStatic for CMsgJoinableCustomGameModesResponseEntry {
    fn new() -> CMsgJoinableCustomGameModesResponseEntry {
        CMsgJoinableCustomGameModesResponseEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgJoinableCustomGameModesResponseEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CMsgJoinableCustomGameModesResponseEntry::get_custom_game_id_for_reflect,
                    CMsgJoinableCustomGameModesResponseEntry::mut_custom_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lobby_count",
                    CMsgJoinableCustomGameModesResponseEntry::get_lobby_count_for_reflect,
                    CMsgJoinableCustomGameModesResponseEntry::mut_lobby_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_count",
                    CMsgJoinableCustomGameModesResponseEntry::get_player_count_for_reflect,
                    CMsgJoinableCustomGameModesResponseEntry::mut_player_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgJoinableCustomGameModesResponseEntry>(
                    "CMsgJoinableCustomGameModesResponseEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgJoinableCustomGameModesResponseEntry {
    fn clear(&mut self) {
        self.clear_custom_game_id();
        self.clear_lobby_count();
        self.clear_player_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgJoinableCustomGameModesResponseEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgJoinableCustomGameModesResponseEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgJoinableCustomGameModesResponse {
    // message fields
    game_modes: ::protobuf::RepeatedField<CMsgJoinableCustomGameModesResponseEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgJoinableCustomGameModesResponse {}

impl CMsgJoinableCustomGameModesResponse {
    pub fn new() -> CMsgJoinableCustomGameModesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgJoinableCustomGameModesResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgJoinableCustomGameModesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgJoinableCustomGameModesResponse,
        };
        unsafe {
            instance.get(CMsgJoinableCustomGameModesResponse::new)
        }
    }

    // repeated .dota.CMsgJoinableCustomGameModesResponseEntry game_modes = 1;

    pub fn clear_game_modes(&mut self) {
        self.game_modes.clear();
    }

    // Param is passed by value, moved
    pub fn set_game_modes(&mut self, v: ::protobuf::RepeatedField<CMsgJoinableCustomGameModesResponseEntry>) {
        self.game_modes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_game_modes(&mut self) -> &mut ::protobuf::RepeatedField<CMsgJoinableCustomGameModesResponseEntry> {
        &mut self.game_modes
    }

    // Take field
    pub fn take_game_modes(&mut self) -> ::protobuf::RepeatedField<CMsgJoinableCustomGameModesResponseEntry> {
        ::std::mem::replace(&mut self.game_modes, ::protobuf::RepeatedField::new())
    }

    pub fn get_game_modes(&self) -> &[CMsgJoinableCustomGameModesResponseEntry] {
        &self.game_modes
    }

    fn get_game_modes_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgJoinableCustomGameModesResponseEntry> {
        &self.game_modes
    }

    fn mut_game_modes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgJoinableCustomGameModesResponseEntry> {
        &mut self.game_modes
    }
}

impl ::protobuf::Message for CMsgJoinableCustomGameModesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.game_modes)?;
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
        for value in &self.game_modes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.game_modes {
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

impl ::protobuf::MessageStatic for CMsgJoinableCustomGameModesResponse {
    fn new() -> CMsgJoinableCustomGameModesResponse {
        CMsgJoinableCustomGameModesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgJoinableCustomGameModesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgJoinableCustomGameModesResponseEntry>>(
                    "game_modes",
                    CMsgJoinableCustomGameModesResponse::get_game_modes_for_reflect,
                    CMsgJoinableCustomGameModesResponse::mut_game_modes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgJoinableCustomGameModesResponse>(
                    "CMsgJoinableCustomGameModesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgJoinableCustomGameModesResponse {
    fn clear(&mut self) {
        self.clear_game_modes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgJoinableCustomGameModesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgJoinableCustomGameModesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgJoinableCustomLobbiesRequest {
    // message fields
    server_region: ::std::option::Option<u32>,
    custom_game_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgJoinableCustomLobbiesRequest {}

impl CMsgJoinableCustomLobbiesRequest {
    pub fn new() -> CMsgJoinableCustomLobbiesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgJoinableCustomLobbiesRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgJoinableCustomLobbiesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgJoinableCustomLobbiesRequest,
        };
        unsafe {
            instance.get(CMsgJoinableCustomLobbiesRequest::new)
        }
    }

    // optional uint32 server_region = 1;

    pub fn clear_server_region(&mut self) {
        self.server_region = ::std::option::Option::None;
    }

    pub fn has_server_region(&self) -> bool {
        self.server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_region(&mut self, v: u32) {
        self.server_region = ::std::option::Option::Some(v);
    }

    pub fn get_server_region(&self) -> u32 {
        self.server_region.unwrap_or(0)
    }

    fn get_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_region
    }

    fn mut_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_region
    }

    // optional uint64 custom_game_id = 2;

    pub fn clear_custom_game_id(&mut self) {
        self.custom_game_id = ::std::option::Option::None;
    }

    pub fn has_custom_game_id(&self) -> bool {
        self.custom_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_id(&mut self, v: u64) {
        self.custom_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_id(&self) -> u64 {
        self.custom_game_id.unwrap_or(0)
    }

    fn get_custom_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_id
    }

    fn mut_custom_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_id
    }
}

impl ::protobuf::Message for CMsgJoinableCustomLobbiesRequest {
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
                    self.server_region = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.custom_game_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.server_region {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_region {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.custom_game_id {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for CMsgJoinableCustomLobbiesRequest {
    fn new() -> CMsgJoinableCustomLobbiesRequest {
        CMsgJoinableCustomLobbiesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgJoinableCustomLobbiesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_region",
                    CMsgJoinableCustomLobbiesRequest::get_server_region_for_reflect,
                    CMsgJoinableCustomLobbiesRequest::mut_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CMsgJoinableCustomLobbiesRequest::get_custom_game_id_for_reflect,
                    CMsgJoinableCustomLobbiesRequest::mut_custom_game_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgJoinableCustomLobbiesRequest>(
                    "CMsgJoinableCustomLobbiesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgJoinableCustomLobbiesRequest {
    fn clear(&mut self) {
        self.clear_server_region();
        self.clear_custom_game_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgJoinableCustomLobbiesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgJoinableCustomLobbiesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgJoinableCustomLobbiesResponseEntry {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    custom_game_id: ::std::option::Option<u64>,
    lobby_name: ::protobuf::SingularField<::std::string::String>,
    member_count: ::std::option::Option<u32>,
    leader_account_id: ::std::option::Option<u32>,
    leader_name: ::protobuf::SingularField<::std::string::String>,
    custom_map_name: ::protobuf::SingularField<::std::string::String>,
    max_player_count: ::std::option::Option<u32>,
    server_region: ::std::option::Option<u32>,
    lan_host_ping_to_server_region: ::std::option::Option<u32>,
    has_pass_key: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgJoinableCustomLobbiesResponseEntry {}

impl CMsgJoinableCustomLobbiesResponseEntry {
    pub fn new() -> CMsgJoinableCustomLobbiesResponseEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgJoinableCustomLobbiesResponseEntry {
        static mut instance: ::protobuf::lazy::Lazy<CMsgJoinableCustomLobbiesResponseEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgJoinableCustomLobbiesResponseEntry,
        };
        unsafe {
            instance.get(CMsgJoinableCustomLobbiesResponseEntry::new)
        }
    }

    // optional fixed64 lobby_id = 1;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }

    // optional uint64 custom_game_id = 2;

    pub fn clear_custom_game_id(&mut self) {
        self.custom_game_id = ::std::option::Option::None;
    }

    pub fn has_custom_game_id(&self) -> bool {
        self.custom_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_id(&mut self, v: u64) {
        self.custom_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_id(&self) -> u64 {
        self.custom_game_id.unwrap_or(0)
    }

    fn get_custom_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_id
    }

    fn mut_custom_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_id
    }

    // optional string lobby_name = 3;

    pub fn clear_lobby_name(&mut self) {
        self.lobby_name.clear();
    }

    pub fn has_lobby_name(&self) -> bool {
        self.lobby_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_name(&mut self, v: ::std::string::String) {
        self.lobby_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lobby_name(&mut self) -> &mut ::std::string::String {
        if self.lobby_name.is_none() {
            self.lobby_name.set_default();
        };
        self.lobby_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_lobby_name(&mut self) -> ::std::string::String {
        self.lobby_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_lobby_name(&self) -> &str {
        match self.lobby_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_lobby_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.lobby_name
    }

    fn mut_lobby_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.lobby_name
    }

    // optional uint32 member_count = 4;

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

    // optional uint32 leader_account_id = 5;

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

    // optional string leader_name = 6;

    pub fn clear_leader_name(&mut self) {
        self.leader_name.clear();
    }

    pub fn has_leader_name(&self) -> bool {
        self.leader_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_name(&mut self, v: ::std::string::String) {
        self.leader_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader_name(&mut self) -> &mut ::std::string::String {
        if self.leader_name.is_none() {
            self.leader_name.set_default();
        };
        self.leader_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_leader_name(&mut self) -> ::std::string::String {
        self.leader_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_leader_name(&self) -> &str {
        match self.leader_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_leader_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.leader_name
    }

    fn mut_leader_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.leader_name
    }

    // optional string custom_map_name = 7;

    pub fn clear_custom_map_name(&mut self) {
        self.custom_map_name.clear();
    }

    pub fn has_custom_map_name(&self) -> bool {
        self.custom_map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_map_name(&mut self, v: ::std::string::String) {
        self.custom_map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_custom_map_name(&mut self) -> &mut ::std::string::String {
        if self.custom_map_name.is_none() {
            self.custom_map_name.set_default();
        };
        self.custom_map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_custom_map_name(&mut self) -> ::std::string::String {
        self.custom_map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_custom_map_name(&self) -> &str {
        match self.custom_map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_custom_map_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.custom_map_name
    }

    fn mut_custom_map_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.custom_map_name
    }

    // optional uint32 max_player_count = 8;

    pub fn clear_max_player_count(&mut self) {
        self.max_player_count = ::std::option::Option::None;
    }

    pub fn has_max_player_count(&self) -> bool {
        self.max_player_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_player_count(&mut self, v: u32) {
        self.max_player_count = ::std::option::Option::Some(v);
    }

    pub fn get_max_player_count(&self) -> u32 {
        self.max_player_count.unwrap_or(0)
    }

    fn get_max_player_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.max_player_count
    }

    fn mut_max_player_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.max_player_count
    }

    // optional uint32 server_region = 9;

    pub fn clear_server_region(&mut self) {
        self.server_region = ::std::option::Option::None;
    }

    pub fn has_server_region(&self) -> bool {
        self.server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_region(&mut self, v: u32) {
        self.server_region = ::std::option::Option::Some(v);
    }

    pub fn get_server_region(&self) -> u32 {
        self.server_region.unwrap_or(0)
    }

    fn get_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_region
    }

    fn mut_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_region
    }

    // optional uint32 lan_host_ping_to_server_region = 10;

    pub fn clear_lan_host_ping_to_server_region(&mut self) {
        self.lan_host_ping_to_server_region = ::std::option::Option::None;
    }

    pub fn has_lan_host_ping_to_server_region(&self) -> bool {
        self.lan_host_ping_to_server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lan_host_ping_to_server_region(&mut self, v: u32) {
        self.lan_host_ping_to_server_region = ::std::option::Option::Some(v);
    }

    pub fn get_lan_host_ping_to_server_region(&self) -> u32 {
        self.lan_host_ping_to_server_region.unwrap_or(0)
    }

    fn get_lan_host_ping_to_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lan_host_ping_to_server_region
    }

    fn mut_lan_host_ping_to_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lan_host_ping_to_server_region
    }

    // optional bool has_pass_key = 11;

    pub fn clear_has_pass_key(&mut self) {
        self.has_pass_key = ::std::option::Option::None;
    }

    pub fn has_has_pass_key(&self) -> bool {
        self.has_pass_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_pass_key(&mut self, v: bool) {
        self.has_pass_key = ::std::option::Option::Some(v);
    }

    pub fn get_has_pass_key(&self) -> bool {
        self.has_pass_key.unwrap_or(false)
    }

    fn get_has_pass_key_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_pass_key
    }

    fn mut_has_pass_key_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_pass_key
    }
}

impl ::protobuf::Message for CMsgJoinableCustomLobbiesResponseEntry {
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
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.custom_game_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.lobby_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.member_count = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.leader_account_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.leader_name)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.custom_map_name)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.max_player_count = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.server_region = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lan_host_ping_to_server_region = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.has_pass_key = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.lobby_id {
            my_size += 9;
        };
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lobby_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.member_count {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.leader_account_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.leader_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.custom_map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        };
        if let Some(v) = self.max_player_count {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.server_region {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lan_host_ping_to_server_region {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.has_pass_key {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.custom_game_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.lobby_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.member_count {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.leader_account_id {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.leader_name.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.custom_map_name.as_ref() {
            os.write_string(7, &v)?;
        };
        if let Some(v) = self.max_player_count {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.server_region {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.lan_host_ping_to_server_region {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.has_pass_key {
            os.write_bool(11, v)?;
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

impl ::protobuf::MessageStatic for CMsgJoinableCustomLobbiesResponseEntry {
    fn new() -> CMsgJoinableCustomLobbiesResponseEntry {
        CMsgJoinableCustomLobbiesResponseEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgJoinableCustomLobbiesResponseEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "lobby_id",
                    CMsgJoinableCustomLobbiesResponseEntry::get_lobby_id_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CMsgJoinableCustomLobbiesResponseEntry::get_custom_game_id_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_custom_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lobby_name",
                    CMsgJoinableCustomLobbiesResponseEntry::get_lobby_name_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_lobby_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "member_count",
                    CMsgJoinableCustomLobbiesResponseEntry::get_member_count_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_member_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leader_account_id",
                    CMsgJoinableCustomLobbiesResponseEntry::get_leader_account_id_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_leader_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "leader_name",
                    CMsgJoinableCustomLobbiesResponseEntry::get_leader_name_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_leader_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "custom_map_name",
                    CMsgJoinableCustomLobbiesResponseEntry::get_custom_map_name_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_custom_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_player_count",
                    CMsgJoinableCustomLobbiesResponseEntry::get_max_player_count_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_max_player_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_region",
                    CMsgJoinableCustomLobbiesResponseEntry::get_server_region_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lan_host_ping_to_server_region",
                    CMsgJoinableCustomLobbiesResponseEntry::get_lan_host_ping_to_server_region_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_lan_host_ping_to_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_pass_key",
                    CMsgJoinableCustomLobbiesResponseEntry::get_has_pass_key_for_reflect,
                    CMsgJoinableCustomLobbiesResponseEntry::mut_has_pass_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgJoinableCustomLobbiesResponseEntry>(
                    "CMsgJoinableCustomLobbiesResponseEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgJoinableCustomLobbiesResponseEntry {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.clear_custom_game_id();
        self.clear_lobby_name();
        self.clear_member_count();
        self.clear_leader_account_id();
        self.clear_leader_name();
        self.clear_custom_map_name();
        self.clear_max_player_count();
        self.clear_server_region();
        self.clear_lan_host_ping_to_server_region();
        self.clear_has_pass_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgJoinableCustomLobbiesResponseEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgJoinableCustomLobbiesResponseEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgJoinableCustomLobbiesResponse {
    // message fields
    lobbies: ::protobuf::RepeatedField<CMsgJoinableCustomLobbiesResponseEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgJoinableCustomLobbiesResponse {}

impl CMsgJoinableCustomLobbiesResponse {
    pub fn new() -> CMsgJoinableCustomLobbiesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgJoinableCustomLobbiesResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgJoinableCustomLobbiesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgJoinableCustomLobbiesResponse,
        };
        unsafe {
            instance.get(CMsgJoinableCustomLobbiesResponse::new)
        }
    }

    // repeated .dota.CMsgJoinableCustomLobbiesResponseEntry lobbies = 1;

    pub fn clear_lobbies(&mut self) {
        self.lobbies.clear();
    }

    // Param is passed by value, moved
    pub fn set_lobbies(&mut self, v: ::protobuf::RepeatedField<CMsgJoinableCustomLobbiesResponseEntry>) {
        self.lobbies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lobbies(&mut self) -> &mut ::protobuf::RepeatedField<CMsgJoinableCustomLobbiesResponseEntry> {
        &mut self.lobbies
    }

    // Take field
    pub fn take_lobbies(&mut self) -> ::protobuf::RepeatedField<CMsgJoinableCustomLobbiesResponseEntry> {
        ::std::mem::replace(&mut self.lobbies, ::protobuf::RepeatedField::new())
    }

    pub fn get_lobbies(&self) -> &[CMsgJoinableCustomLobbiesResponseEntry] {
        &self.lobbies
    }

    fn get_lobbies_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgJoinableCustomLobbiesResponseEntry> {
        &self.lobbies
    }

    fn mut_lobbies_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgJoinableCustomLobbiesResponseEntry> {
        &mut self.lobbies
    }
}

impl ::protobuf::Message for CMsgJoinableCustomLobbiesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lobbies)?;
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
        for value in &self.lobbies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.lobbies {
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

impl ::protobuf::MessageStatic for CMsgJoinableCustomLobbiesResponse {
    fn new() -> CMsgJoinableCustomLobbiesResponse {
        CMsgJoinableCustomLobbiesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgJoinableCustomLobbiesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgJoinableCustomLobbiesResponseEntry>>(
                    "lobbies",
                    CMsgJoinableCustomLobbiesResponse::get_lobbies_for_reflect,
                    CMsgJoinableCustomLobbiesResponse::mut_lobbies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgJoinableCustomLobbiesResponse>(
                    "CMsgJoinableCustomLobbiesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgJoinableCustomLobbiesResponse {
    fn clear(&mut self) {
        self.clear_lobbies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgJoinableCustomLobbiesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgJoinableCustomLobbiesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgQuickJoinCustomLobby {
    // message fields
    legacy_server_region: ::std::option::Option<u32>,
    custom_game_id: ::std::option::Option<u64>,
    client_version: ::std::option::Option<u32>,
    create_lobby_details: ::protobuf::SingularPtrField<CMsgPracticeLobbySetDetails>,
    allow_any_map: ::std::option::Option<bool>,
    legacy_region_pings: ::protobuf::RepeatedField<CMsgQuickJoinCustomLobby_LegacyRegionPing>,
    ping_data: ::protobuf::SingularPtrField<super::base_gcmessages::CMsgClientPingData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgQuickJoinCustomLobby {}

impl CMsgQuickJoinCustomLobby {
    pub fn new() -> CMsgQuickJoinCustomLobby {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgQuickJoinCustomLobby {
        static mut instance: ::protobuf::lazy::Lazy<CMsgQuickJoinCustomLobby> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgQuickJoinCustomLobby,
        };
        unsafe {
            instance.get(CMsgQuickJoinCustomLobby::new)
        }
    }

    // optional uint32 legacy_server_region = 1;

    pub fn clear_legacy_server_region(&mut self) {
        self.legacy_server_region = ::std::option::Option::None;
    }

    pub fn has_legacy_server_region(&self) -> bool {
        self.legacy_server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_legacy_server_region(&mut self, v: u32) {
        self.legacy_server_region = ::std::option::Option::Some(v);
    }

    pub fn get_legacy_server_region(&self) -> u32 {
        self.legacy_server_region.unwrap_or(0)
    }

    fn get_legacy_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.legacy_server_region
    }

    fn mut_legacy_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.legacy_server_region
    }

    // optional uint64 custom_game_id = 2;

    pub fn clear_custom_game_id(&mut self) {
        self.custom_game_id = ::std::option::Option::None;
    }

    pub fn has_custom_game_id(&self) -> bool {
        self.custom_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_id(&mut self, v: u64) {
        self.custom_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_id(&self) -> u64 {
        self.custom_game_id.unwrap_or(0)
    }

    fn get_custom_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_id
    }

    fn mut_custom_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_id
    }

    // optional uint32 client_version = 3;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional .dota.CMsgPracticeLobbySetDetails create_lobby_details = 4;

    pub fn clear_create_lobby_details(&mut self) {
        self.create_lobby_details.clear();
    }

    pub fn has_create_lobby_details(&self) -> bool {
        self.create_lobby_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_lobby_details(&mut self, v: CMsgPracticeLobbySetDetails) {
        self.create_lobby_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_create_lobby_details(&mut self) -> &mut CMsgPracticeLobbySetDetails {
        if self.create_lobby_details.is_none() {
            self.create_lobby_details.set_default();
        };
        self.create_lobby_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_create_lobby_details(&mut self) -> CMsgPracticeLobbySetDetails {
        self.create_lobby_details.take().unwrap_or_else(|| CMsgPracticeLobbySetDetails::new())
    }

    pub fn get_create_lobby_details(&self) -> &CMsgPracticeLobbySetDetails {
        self.create_lobby_details.as_ref().unwrap_or_else(|| CMsgPracticeLobbySetDetails::default_instance())
    }

    fn get_create_lobby_details_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgPracticeLobbySetDetails> {
        &self.create_lobby_details
    }

    fn mut_create_lobby_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgPracticeLobbySetDetails> {
        &mut self.create_lobby_details
    }

    // optional bool allow_any_map = 5;

    pub fn clear_allow_any_map(&mut self) {
        self.allow_any_map = ::std::option::Option::None;
    }

    pub fn has_allow_any_map(&self) -> bool {
        self.allow_any_map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_any_map(&mut self, v: bool) {
        self.allow_any_map = ::std::option::Option::Some(v);
    }

    pub fn get_allow_any_map(&self) -> bool {
        self.allow_any_map.unwrap_or(false)
    }

    fn get_allow_any_map_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_any_map
    }

    fn mut_allow_any_map_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_any_map
    }

    // repeated .dota.CMsgQuickJoinCustomLobby.LegacyRegionPing legacy_region_pings = 6;

    pub fn clear_legacy_region_pings(&mut self) {
        self.legacy_region_pings.clear();
    }

    // Param is passed by value, moved
    pub fn set_legacy_region_pings(&mut self, v: ::protobuf::RepeatedField<CMsgQuickJoinCustomLobby_LegacyRegionPing>) {
        self.legacy_region_pings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_legacy_region_pings(&mut self) -> &mut ::protobuf::RepeatedField<CMsgQuickJoinCustomLobby_LegacyRegionPing> {
        &mut self.legacy_region_pings
    }

    // Take field
    pub fn take_legacy_region_pings(&mut self) -> ::protobuf::RepeatedField<CMsgQuickJoinCustomLobby_LegacyRegionPing> {
        ::std::mem::replace(&mut self.legacy_region_pings, ::protobuf::RepeatedField::new())
    }

    pub fn get_legacy_region_pings(&self) -> &[CMsgQuickJoinCustomLobby_LegacyRegionPing] {
        &self.legacy_region_pings
    }

    fn get_legacy_region_pings_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgQuickJoinCustomLobby_LegacyRegionPing> {
        &self.legacy_region_pings
    }

    fn mut_legacy_region_pings_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgQuickJoinCustomLobby_LegacyRegionPing> {
        &mut self.legacy_region_pings
    }

    // optional .dota.CMsgClientPingData ping_data = 7;

    pub fn clear_ping_data(&mut self) {
        self.ping_data.clear();
    }

    pub fn has_ping_data(&self) -> bool {
        self.ping_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping_data(&mut self, v: super::base_gcmessages::CMsgClientPingData) {
        self.ping_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ping_data(&mut self) -> &mut super::base_gcmessages::CMsgClientPingData {
        if self.ping_data.is_none() {
            self.ping_data.set_default();
        };
        self.ping_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_ping_data(&mut self) -> super::base_gcmessages::CMsgClientPingData {
        self.ping_data.take().unwrap_or_else(|| super::base_gcmessages::CMsgClientPingData::new())
    }

    pub fn get_ping_data(&self) -> &super::base_gcmessages::CMsgClientPingData {
        self.ping_data.as_ref().unwrap_or_else(|| super::base_gcmessages::CMsgClientPingData::default_instance())
    }

    fn get_ping_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::base_gcmessages::CMsgClientPingData> {
        &self.ping_data
    }

    fn mut_ping_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::base_gcmessages::CMsgClientPingData> {
        &mut self.ping_data
    }
}

impl ::protobuf::Message for CMsgQuickJoinCustomLobby {
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
                    self.legacy_server_region = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.custom_game_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.create_lobby_details)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allow_any_map = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.legacy_region_pings)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ping_data)?;
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
        if let Some(v) = self.legacy_server_region {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.create_lobby_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.allow_any_map {
            my_size += 2;
        };
        for value in &self.legacy_region_pings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.ping_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.legacy_server_region {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.custom_game_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.create_lobby_details.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.allow_any_map {
            os.write_bool(5, v)?;
        };
        for v in &self.legacy_region_pings {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.ping_data.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgQuickJoinCustomLobby {
    fn new() -> CMsgQuickJoinCustomLobby {
        CMsgQuickJoinCustomLobby::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgQuickJoinCustomLobby>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "legacy_server_region",
                    CMsgQuickJoinCustomLobby::get_legacy_server_region_for_reflect,
                    CMsgQuickJoinCustomLobby::mut_legacy_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CMsgQuickJoinCustomLobby::get_custom_game_id_for_reflect,
                    CMsgQuickJoinCustomLobby::mut_custom_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgQuickJoinCustomLobby::get_client_version_for_reflect,
                    CMsgQuickJoinCustomLobby::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPracticeLobbySetDetails>>(
                    "create_lobby_details",
                    CMsgQuickJoinCustomLobby::get_create_lobby_details_for_reflect,
                    CMsgQuickJoinCustomLobby::mut_create_lobby_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_any_map",
                    CMsgQuickJoinCustomLobby::get_allow_any_map_for_reflect,
                    CMsgQuickJoinCustomLobby::mut_allow_any_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgQuickJoinCustomLobby_LegacyRegionPing>>(
                    "legacy_region_pings",
                    CMsgQuickJoinCustomLobby::get_legacy_region_pings_for_reflect,
                    CMsgQuickJoinCustomLobby::mut_legacy_region_pings_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base_gcmessages::CMsgClientPingData>>(
                    "ping_data",
                    CMsgQuickJoinCustomLobby::get_ping_data_for_reflect,
                    CMsgQuickJoinCustomLobby::mut_ping_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgQuickJoinCustomLobby>(
                    "CMsgQuickJoinCustomLobby",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgQuickJoinCustomLobby {
    fn clear(&mut self) {
        self.clear_legacy_server_region();
        self.clear_custom_game_id();
        self.clear_client_version();
        self.clear_create_lobby_details();
        self.clear_allow_any_map();
        self.clear_legacy_region_pings();
        self.clear_ping_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgQuickJoinCustomLobby {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgQuickJoinCustomLobby {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgQuickJoinCustomLobby_LegacyRegionPing {
    // message fields
    server_region: ::std::option::Option<u32>,
    ping: ::std::option::Option<u32>,
    region_code: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgQuickJoinCustomLobby_LegacyRegionPing {}

impl CMsgQuickJoinCustomLobby_LegacyRegionPing {
    pub fn new() -> CMsgQuickJoinCustomLobby_LegacyRegionPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgQuickJoinCustomLobby_LegacyRegionPing {
        static mut instance: ::protobuf::lazy::Lazy<CMsgQuickJoinCustomLobby_LegacyRegionPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgQuickJoinCustomLobby_LegacyRegionPing,
        };
        unsafe {
            instance.get(CMsgQuickJoinCustomLobby_LegacyRegionPing::new)
        }
    }

    // optional uint32 server_region = 1;

    pub fn clear_server_region(&mut self) {
        self.server_region = ::std::option::Option::None;
    }

    pub fn has_server_region(&self) -> bool {
        self.server_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_region(&mut self, v: u32) {
        self.server_region = ::std::option::Option::Some(v);
    }

    pub fn get_server_region(&self) -> u32 {
        self.server_region.unwrap_or(0)
    }

    fn get_server_region_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_region
    }

    fn mut_server_region_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_region
    }

    // optional uint32 ping = 2;

    pub fn clear_ping(&mut self) {
        self.ping = ::std::option::Option::None;
    }

    pub fn has_ping(&self) -> bool {
        self.ping.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: u32) {
        self.ping = ::std::option::Option::Some(v);
    }

    pub fn get_ping(&self) -> u32 {
        self.ping.unwrap_or(0)
    }

    fn get_ping_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ping
    }

    fn mut_ping_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ping
    }

    // optional fixed32 region_code = 3;

    pub fn clear_region_code(&mut self) {
        self.region_code = ::std::option::Option::None;
    }

    pub fn has_region_code(&self) -> bool {
        self.region_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_code(&mut self, v: u32) {
        self.region_code = ::std::option::Option::Some(v);
    }

    pub fn get_region_code(&self) -> u32 {
        self.region_code.unwrap_or(0)
    }

    fn get_region_code_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.region_code
    }

    fn mut_region_code_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.region_code
    }
}

impl ::protobuf::Message for CMsgQuickJoinCustomLobby_LegacyRegionPing {
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
                    self.server_region = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.ping = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.region_code = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.server_region {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ping {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.region_code {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_region {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.ping {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.region_code {
            os.write_fixed32(3, v)?;
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

impl ::protobuf::MessageStatic for CMsgQuickJoinCustomLobby_LegacyRegionPing {
    fn new() -> CMsgQuickJoinCustomLobby_LegacyRegionPing {
        CMsgQuickJoinCustomLobby_LegacyRegionPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgQuickJoinCustomLobby_LegacyRegionPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_region",
                    CMsgQuickJoinCustomLobby_LegacyRegionPing::get_server_region_for_reflect,
                    CMsgQuickJoinCustomLobby_LegacyRegionPing::mut_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ping",
                    CMsgQuickJoinCustomLobby_LegacyRegionPing::get_ping_for_reflect,
                    CMsgQuickJoinCustomLobby_LegacyRegionPing::mut_ping_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "region_code",
                    CMsgQuickJoinCustomLobby_LegacyRegionPing::get_region_code_for_reflect,
                    CMsgQuickJoinCustomLobby_LegacyRegionPing::mut_region_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgQuickJoinCustomLobby_LegacyRegionPing>(
                    "CMsgQuickJoinCustomLobby_LegacyRegionPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgQuickJoinCustomLobby_LegacyRegionPing {
    fn clear(&mut self) {
        self.clear_server_region();
        self.clear_ping();
        self.clear_region_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgQuickJoinCustomLobby_LegacyRegionPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgQuickJoinCustomLobby_LegacyRegionPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgQuickJoinCustomLobbyResponse {
    // message fields
    result: ::std::option::Option<super::dota_shared_enums::DOTAJoinLobbyResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgQuickJoinCustomLobbyResponse {}

impl CMsgQuickJoinCustomLobbyResponse {
    pub fn new() -> CMsgQuickJoinCustomLobbyResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgQuickJoinCustomLobbyResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgQuickJoinCustomLobbyResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgQuickJoinCustomLobbyResponse,
        };
        unsafe {
            instance.get(CMsgQuickJoinCustomLobbyResponse::new)
        }
    }

    // optional .dota.DOTAJoinLobbyResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: super::dota_shared_enums::DOTAJoinLobbyResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> super::dota_shared_enums::DOTAJoinLobbyResult {
        self.result.unwrap_or(super::dota_shared_enums::DOTAJoinLobbyResult::DOTA_JOIN_RESULT_SUCCESS)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTAJoinLobbyResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTAJoinLobbyResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgQuickJoinCustomLobbyResponse {
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgQuickJoinCustomLobbyResponse {
    fn new() -> CMsgQuickJoinCustomLobbyResponse {
        CMsgQuickJoinCustomLobbyResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgQuickJoinCustomLobbyResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAJoinLobbyResult>>(
                    "result",
                    CMsgQuickJoinCustomLobbyResponse::get_result_for_reflect,
                    CMsgQuickJoinCustomLobbyResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgQuickJoinCustomLobbyResponse>(
                    "CMsgQuickJoinCustomLobbyResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgQuickJoinCustomLobbyResponse {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgQuickJoinCustomLobbyResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgQuickJoinCustomLobbyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgBotGameCreate {
    // message fields
    search_key: ::protobuf::SingularField<::std::string::String>,
    client_version: ::std::option::Option<u32>,
    difficulty_radiant: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    team: ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM>,
    game_mode: ::std::option::Option<u32>,
    difficulty_dire: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgBotGameCreate {}

impl CMsgBotGameCreate {
    pub fn new() -> CMsgBotGameCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgBotGameCreate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgBotGameCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgBotGameCreate,
        };
        unsafe {
            instance.get(CMsgBotGameCreate::new)
        }
    }

    // optional string search_key = 1;

    pub fn clear_search_key(&mut self) {
        self.search_key.clear();
    }

    pub fn has_search_key(&self) -> bool {
        self.search_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_key(&mut self, v: ::std::string::String) {
        self.search_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search_key(&mut self) -> &mut ::std::string::String {
        if self.search_key.is_none() {
            self.search_key.set_default();
        };
        self.search_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_search_key(&mut self) -> ::std::string::String {
        self.search_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_search_key(&self) -> &str {
        match self.search_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_search_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.search_key
    }

    fn mut_search_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.search_key
    }

    // optional uint32 client_version = 2;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional .dota.DOTABotDifficulty difficulty_radiant = 3;

    pub fn clear_difficulty_radiant(&mut self) {
        self.difficulty_radiant = ::std::option::Option::None;
    }

    pub fn has_difficulty_radiant(&self) -> bool {
        self.difficulty_radiant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_difficulty_radiant(&mut self, v: super::dota_shared_enums::DOTABotDifficulty) {
        self.difficulty_radiant = ::std::option::Option::Some(v);
    }

    pub fn get_difficulty_radiant(&self) -> super::dota_shared_enums::DOTABotDifficulty {
        self.difficulty_radiant.unwrap_or(super::dota_shared_enums::DOTABotDifficulty::BOT_DIFFICULTY_PASSIVE)
    }

    fn get_difficulty_radiant_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &self.difficulty_radiant
    }

    fn mut_difficulty_radiant_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &mut self.difficulty_radiant
    }

    // optional .dota.DOTA_GC_TEAM team = 4;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: super::dota_shared_enums::DOTA_GC_TEAM) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> super::dota_shared_enums::DOTA_GC_TEAM {
        self.team.unwrap_or(super::dota_shared_enums::DOTA_GC_TEAM::DOTA_GC_TEAM_GOOD_GUYS)
    }

    fn get_team_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM> {
        &mut self.team
    }

    // optional uint32 game_mode = 5;

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

    // optional .dota.DOTABotDifficulty difficulty_dire = 6;

    pub fn clear_difficulty_dire(&mut self) {
        self.difficulty_dire = ::std::option::Option::None;
    }

    pub fn has_difficulty_dire(&self) -> bool {
        self.difficulty_dire.is_some()
    }

    // Param is passed by value, moved
    pub fn set_difficulty_dire(&mut self, v: super::dota_shared_enums::DOTABotDifficulty) {
        self.difficulty_dire = ::std::option::Option::Some(v);
    }

    pub fn get_difficulty_dire(&self) -> super::dota_shared_enums::DOTABotDifficulty {
        self.difficulty_dire.unwrap_or(super::dota_shared_enums::DOTABotDifficulty::BOT_DIFFICULTY_PASSIVE)
    }

    fn get_difficulty_dire_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &self.difficulty_dire
    }

    fn mut_difficulty_dire_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &mut self.difficulty_dire
    }
}

impl ::protobuf::Message for CMsgBotGameCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.search_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.difficulty_radiant = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.team = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.difficulty_dire = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.search_key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.difficulty_radiant {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::enum_size(4, v);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.difficulty_dire {
            my_size += ::protobuf::rt::enum_size(6, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.search_key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.difficulty_radiant {
            os.write_enum(3, v.value())?;
        };
        if let Some(v) = self.team {
            os.write_enum(4, v.value())?;
        };
        if let Some(v) = self.game_mode {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.difficulty_dire {
            os.write_enum(6, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgBotGameCreate {
    fn new() -> CMsgBotGameCreate {
        CMsgBotGameCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgBotGameCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "search_key",
                    CMsgBotGameCreate::get_search_key_for_reflect,
                    CMsgBotGameCreate::mut_search_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgBotGameCreate::get_client_version_for_reflect,
                    CMsgBotGameCreate::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "difficulty_radiant",
                    CMsgBotGameCreate::get_difficulty_radiant_for_reflect,
                    CMsgBotGameCreate::mut_difficulty_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GC_TEAM>>(
                    "team",
                    CMsgBotGameCreate::get_team_for_reflect,
                    CMsgBotGameCreate::mut_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_mode",
                    CMsgBotGameCreate::get_game_mode_for_reflect,
                    CMsgBotGameCreate::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "difficulty_dire",
                    CMsgBotGameCreate::get_difficulty_dire_for_reflect,
                    CMsgBotGameCreate::mut_difficulty_dire_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgBotGameCreate>(
                    "CMsgBotGameCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgBotGameCreate {
    fn clear(&mut self) {
        self.clear_search_key();
        self.clear_client_version();
        self.clear_difficulty_radiant();
        self.clear_team();
        self.clear_game_mode();
        self.clear_difficulty_dire();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgBotGameCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgBotGameCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgCustomGameCreate {
    // message fields
    search_key: ::protobuf::SingularField<::std::string::String>,
    client_version: ::std::option::Option<u32>,
    difficulty: ::std::option::Option<u32>,
    game_mode: ::protobuf::SingularField<::std::string::String>,
    map: ::protobuf::SingularField<::std::string::String>,
    custom_game_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgCustomGameCreate {}

impl CMsgCustomGameCreate {
    pub fn new() -> CMsgCustomGameCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgCustomGameCreate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgCustomGameCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgCustomGameCreate,
        };
        unsafe {
            instance.get(CMsgCustomGameCreate::new)
        }
    }

    // optional string search_key = 1;

    pub fn clear_search_key(&mut self) {
        self.search_key.clear();
    }

    pub fn has_search_key(&self) -> bool {
        self.search_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_key(&mut self, v: ::std::string::String) {
        self.search_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search_key(&mut self) -> &mut ::std::string::String {
        if self.search_key.is_none() {
            self.search_key.set_default();
        };
        self.search_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_search_key(&mut self) -> ::std::string::String {
        self.search_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_search_key(&self) -> &str {
        match self.search_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_search_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.search_key
    }

    fn mut_search_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.search_key
    }

    // optional uint32 client_version = 2;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional uint32 difficulty = 3;

    pub fn clear_difficulty(&mut self) {
        self.difficulty = ::std::option::Option::None;
    }

    pub fn has_difficulty(&self) -> bool {
        self.difficulty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_difficulty(&mut self, v: u32) {
        self.difficulty = ::std::option::Option::Some(v);
    }

    pub fn get_difficulty(&self) -> u32 {
        self.difficulty.unwrap_or(0)
    }

    fn get_difficulty_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.difficulty
    }

    fn mut_difficulty_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.difficulty
    }

    // optional string game_mode = 4;

    pub fn clear_game_mode(&mut self) {
        self.game_mode.clear();
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: ::std::string::String) {
        self.game_mode = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_mode(&mut self) -> &mut ::std::string::String {
        if self.game_mode.is_none() {
            self.game_mode.set_default();
        };
        self.game_mode.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_mode(&mut self) -> ::std::string::String {
        self.game_mode.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_mode(&self) -> &str {
        match self.game_mode.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_mode_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_mode
    }

    fn mut_game_mode_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_mode
    }

    // optional string map = 5;

    pub fn clear_map(&mut self) {
        self.map.clear();
    }

    pub fn has_map(&self) -> bool {
        self.map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map(&mut self, v: ::std::string::String) {
        self.map = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map(&mut self) -> &mut ::std::string::String {
        if self.map.is_none() {
            self.map.set_default();
        };
        self.map.as_mut().unwrap()
    }

    // Take field
    pub fn take_map(&mut self) -> ::std::string::String {
        self.map.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map(&self) -> &str {
        match self.map.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map
    }

    fn mut_map_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map
    }

    // optional uint64 custom_game_id = 7;

    pub fn clear_custom_game_id(&mut self) {
        self.custom_game_id = ::std::option::Option::None;
    }

    pub fn has_custom_game_id(&self) -> bool {
        self.custom_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_id(&mut self, v: u64) {
        self.custom_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_id(&self) -> u64 {
        self.custom_game_id.unwrap_or(0)
    }

    fn get_custom_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_id
    }

    fn mut_custom_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_id
    }
}

impl ::protobuf::Message for CMsgCustomGameCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.search_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.difficulty = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_mode)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.custom_game_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.search_key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.difficulty {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.map.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.search_key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.difficulty {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.game_mode.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.map.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.custom_game_id {
            os.write_uint64(7, v)?;
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

impl ::protobuf::MessageStatic for CMsgCustomGameCreate {
    fn new() -> CMsgCustomGameCreate {
        CMsgCustomGameCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgCustomGameCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "search_key",
                    CMsgCustomGameCreate::get_search_key_for_reflect,
                    CMsgCustomGameCreate::mut_search_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgCustomGameCreate::get_client_version_for_reflect,
                    CMsgCustomGameCreate::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "difficulty",
                    CMsgCustomGameCreate::get_difficulty_for_reflect,
                    CMsgCustomGameCreate::mut_difficulty_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_mode",
                    CMsgCustomGameCreate::get_game_mode_for_reflect,
                    CMsgCustomGameCreate::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map",
                    CMsgCustomGameCreate::get_map_for_reflect,
                    CMsgCustomGameCreate::mut_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CMsgCustomGameCreate::get_custom_game_id_for_reflect,
                    CMsgCustomGameCreate::mut_custom_game_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgCustomGameCreate>(
                    "CMsgCustomGameCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgCustomGameCreate {
    fn clear(&mut self) {
        self.clear_search_key();
        self.clear_client_version();
        self.clear_difficulty();
        self.clear_game_mode();
        self.clear_map();
        self.clear_custom_game_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgCustomGameCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgCustomGameCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgEventGameCreate {
    // message fields
    search_key: ::protobuf::SingularField<::std::string::String>,
    client_version: ::std::option::Option<u32>,
    difficulty: ::std::option::Option<u32>,
    game_mode: ::protobuf::SingularField<::std::string::String>,
    map: ::protobuf::SingularField<::std::string::String>,
    custom_game_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgEventGameCreate {}

impl CMsgEventGameCreate {
    pub fn new() -> CMsgEventGameCreate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgEventGameCreate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgEventGameCreate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgEventGameCreate,
        };
        unsafe {
            instance.get(CMsgEventGameCreate::new)
        }
    }

    // optional string search_key = 1;

    pub fn clear_search_key(&mut self) {
        self.search_key.clear();
    }

    pub fn has_search_key(&self) -> bool {
        self.search_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_key(&mut self, v: ::std::string::String) {
        self.search_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search_key(&mut self) -> &mut ::std::string::String {
        if self.search_key.is_none() {
            self.search_key.set_default();
        };
        self.search_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_search_key(&mut self) -> ::std::string::String {
        self.search_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_search_key(&self) -> &str {
        match self.search_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_search_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.search_key
    }

    fn mut_search_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.search_key
    }

    // optional uint32 client_version = 2;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional uint32 difficulty = 3;

    pub fn clear_difficulty(&mut self) {
        self.difficulty = ::std::option::Option::None;
    }

    pub fn has_difficulty(&self) -> bool {
        self.difficulty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_difficulty(&mut self, v: u32) {
        self.difficulty = ::std::option::Option::Some(v);
    }

    pub fn get_difficulty(&self) -> u32 {
        self.difficulty.unwrap_or(0)
    }

    fn get_difficulty_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.difficulty
    }

    fn mut_difficulty_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.difficulty
    }

    // optional string game_mode = 4;

    pub fn clear_game_mode(&mut self) {
        self.game_mode.clear();
    }

    pub fn has_game_mode(&self) -> bool {
        self.game_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_mode(&mut self, v: ::std::string::String) {
        self.game_mode = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_mode(&mut self) -> &mut ::std::string::String {
        if self.game_mode.is_none() {
            self.game_mode.set_default();
        };
        self.game_mode.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_mode(&mut self) -> ::std::string::String {
        self.game_mode.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_mode(&self) -> &str {
        match self.game_mode.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_mode_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_mode
    }

    fn mut_game_mode_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_mode
    }

    // optional string map = 5;

    pub fn clear_map(&mut self) {
        self.map.clear();
    }

    pub fn has_map(&self) -> bool {
        self.map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map(&mut self, v: ::std::string::String) {
        self.map = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map(&mut self) -> &mut ::std::string::String {
        if self.map.is_none() {
            self.map.set_default();
        };
        self.map.as_mut().unwrap()
    }

    // Take field
    pub fn take_map(&mut self) -> ::std::string::String {
        self.map.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map(&self) -> &str {
        match self.map.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map
    }

    fn mut_map_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map
    }

    // optional uint64 custom_game_id = 7;

    pub fn clear_custom_game_id(&mut self) {
        self.custom_game_id = ::std::option::Option::None;
    }

    pub fn has_custom_game_id(&self) -> bool {
        self.custom_game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_id(&mut self, v: u64) {
        self.custom_game_id = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_id(&self) -> u64 {
        self.custom_game_id.unwrap_or(0)
    }

    fn get_custom_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.custom_game_id
    }

    fn mut_custom_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.custom_game_id
    }
}

impl ::protobuf::Message for CMsgEventGameCreate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.search_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.difficulty = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_mode)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.custom_game_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.search_key.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.difficulty {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.map.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.search_key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.difficulty {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.game_mode.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.map.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.custom_game_id {
            os.write_uint64(7, v)?;
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

impl ::protobuf::MessageStatic for CMsgEventGameCreate {
    fn new() -> CMsgEventGameCreate {
        CMsgEventGameCreate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgEventGameCreate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "search_key",
                    CMsgEventGameCreate::get_search_key_for_reflect,
                    CMsgEventGameCreate::mut_search_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgEventGameCreate::get_client_version_for_reflect,
                    CMsgEventGameCreate::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "difficulty",
                    CMsgEventGameCreate::get_difficulty_for_reflect,
                    CMsgEventGameCreate::mut_difficulty_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_mode",
                    CMsgEventGameCreate::get_game_mode_for_reflect,
                    CMsgEventGameCreate::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map",
                    CMsgEventGameCreate::get_map_for_reflect,
                    CMsgEventGameCreate::mut_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CMsgEventGameCreate::get_custom_game_id_for_reflect,
                    CMsgEventGameCreate::mut_custom_game_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgEventGameCreate>(
                    "CMsgEventGameCreate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgEventGameCreate {
    fn clear(&mut self) {
        self.clear_search_key();
        self.clear_client_version();
        self.clear_difficulty();
        self.clear_game_mode();
        self.clear_map();
        self.clear_custom_game_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgEventGameCreate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgEventGameCreate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAPartyMemberSetCoach {
    // message fields
    wants_coach: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAPartyMemberSetCoach {}

impl CMsgDOTAPartyMemberSetCoach {
    pub fn new() -> CMsgDOTAPartyMemberSetCoach {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAPartyMemberSetCoach {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAPartyMemberSetCoach> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAPartyMemberSetCoach,
        };
        unsafe {
            instance.get(CMsgDOTAPartyMemberSetCoach::new)
        }
    }

    // optional bool wants_coach = 1;

    pub fn clear_wants_coach(&mut self) {
        self.wants_coach = ::std::option::Option::None;
    }

    pub fn has_wants_coach(&self) -> bool {
        self.wants_coach.is_some()
    }

    // Param is passed by value, moved
    pub fn set_wants_coach(&mut self, v: bool) {
        self.wants_coach = ::std::option::Option::Some(v);
    }

    pub fn get_wants_coach(&self) -> bool {
        self.wants_coach.unwrap_or(false)
    }

    fn get_wants_coach_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.wants_coach
    }

    fn mut_wants_coach_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.wants_coach
    }
}

impl ::protobuf::Message for CMsgDOTAPartyMemberSetCoach {
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
                    self.wants_coach = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.wants_coach {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.wants_coach {
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

impl ::protobuf::MessageStatic for CMsgDOTAPartyMemberSetCoach {
    fn new() -> CMsgDOTAPartyMemberSetCoach {
        CMsgDOTAPartyMemberSetCoach::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAPartyMemberSetCoach>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "wants_coach",
                    CMsgDOTAPartyMemberSetCoach::get_wants_coach_for_reflect,
                    CMsgDOTAPartyMemberSetCoach::mut_wants_coach_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAPartyMemberSetCoach>(
                    "CMsgDOTAPartyMemberSetCoach",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAPartyMemberSetCoach {
    fn clear(&mut self) {
        self.clear_wants_coach();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAPartyMemberSetCoach {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAPartyMemberSetCoach {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTASetGroupLeader {
    // message fields
    new_leader_steamid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTASetGroupLeader {}

impl CMsgDOTASetGroupLeader {
    pub fn new() -> CMsgDOTASetGroupLeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTASetGroupLeader {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTASetGroupLeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTASetGroupLeader,
        };
        unsafe {
            instance.get(CMsgDOTASetGroupLeader::new)
        }
    }

    // optional fixed64 new_leader_steamid = 1;

    pub fn clear_new_leader_steamid(&mut self) {
        self.new_leader_steamid = ::std::option::Option::None;
    }

    pub fn has_new_leader_steamid(&self) -> bool {
        self.new_leader_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_leader_steamid(&mut self, v: u64) {
        self.new_leader_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_new_leader_steamid(&self) -> u64 {
        self.new_leader_steamid.unwrap_or(0)
    }

    fn get_new_leader_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.new_leader_steamid
    }

    fn mut_new_leader_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.new_leader_steamid
    }
}

impl ::protobuf::Message for CMsgDOTASetGroupLeader {
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
                    self.new_leader_steamid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.new_leader_steamid {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_leader_steamid {
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

impl ::protobuf::MessageStatic for CMsgDOTASetGroupLeader {
    fn new() -> CMsgDOTASetGroupLeader {
        CMsgDOTASetGroupLeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTASetGroupLeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "new_leader_steamid",
                    CMsgDOTASetGroupLeader::get_new_leader_steamid_for_reflect,
                    CMsgDOTASetGroupLeader::mut_new_leader_steamid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTASetGroupLeader>(
                    "CMsgDOTASetGroupLeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTASetGroupLeader {
    fn clear(&mut self) {
        self.clear_new_leader_steamid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTASetGroupLeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTASetGroupLeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTACancelGroupInvites {
    // message fields
    invited_steamids: ::std::vec::Vec<u64>,
    invited_groupids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTACancelGroupInvites {}

impl CMsgDOTACancelGroupInvites {
    pub fn new() -> CMsgDOTACancelGroupInvites {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTACancelGroupInvites {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTACancelGroupInvites> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTACancelGroupInvites,
        };
        unsafe {
            instance.get(CMsgDOTACancelGroupInvites::new)
        }
    }

    // repeated fixed64 invited_steamids = 1;

    pub fn clear_invited_steamids(&mut self) {
        self.invited_steamids.clear();
    }

    // Param is passed by value, moved
    pub fn set_invited_steamids(&mut self, v: ::std::vec::Vec<u64>) {
        self.invited_steamids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invited_steamids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.invited_steamids
    }

    // Take field
    pub fn take_invited_steamids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.invited_steamids, ::std::vec::Vec::new())
    }

    pub fn get_invited_steamids(&self) -> &[u64] {
        &self.invited_steamids
    }

    fn get_invited_steamids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.invited_steamids
    }

    fn mut_invited_steamids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.invited_steamids
    }

    // repeated fixed64 invited_groupids = 2;

    pub fn clear_invited_groupids(&mut self) {
        self.invited_groupids.clear();
    }

    // Param is passed by value, moved
    pub fn set_invited_groupids(&mut self, v: ::std::vec::Vec<u64>) {
        self.invited_groupids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_invited_groupids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.invited_groupids
    }

    // Take field
    pub fn take_invited_groupids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.invited_groupids, ::std::vec::Vec::new())
    }

    pub fn get_invited_groupids(&self) -> &[u64] {
        &self.invited_groupids
    }

    fn get_invited_groupids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.invited_groupids
    }

    fn mut_invited_groupids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.invited_groupids
    }
}

impl ::protobuf::Message for CMsgDOTACancelGroupInvites {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.invited_steamids)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.invited_groupids)?;
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
        my_size += 9 * self.invited_steamids.len() as u32;
        my_size += 9 * self.invited_groupids.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.invited_steamids {
            os.write_fixed64(1, *v)?;
        };
        for v in &self.invited_groupids {
            os.write_fixed64(2, *v)?;
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

impl ::protobuf::MessageStatic for CMsgDOTACancelGroupInvites {
    fn new() -> CMsgDOTACancelGroupInvites {
        CMsgDOTACancelGroupInvites::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTACancelGroupInvites>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "invited_steamids",
                    CMsgDOTACancelGroupInvites::get_invited_steamids_for_reflect,
                    CMsgDOTACancelGroupInvites::mut_invited_steamids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "invited_groupids",
                    CMsgDOTACancelGroupInvites::get_invited_groupids_for_reflect,
                    CMsgDOTACancelGroupInvites::mut_invited_groupids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTACancelGroupInvites>(
                    "CMsgDOTACancelGroupInvites",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTACancelGroupInvites {
    fn clear(&mut self) {
        self.clear_invited_steamids();
        self.clear_invited_groupids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTACancelGroupInvites {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTACancelGroupInvites {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTASetGroupOpenStatus {
    // message fields
    open: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTASetGroupOpenStatus {}

impl CMsgDOTASetGroupOpenStatus {
    pub fn new() -> CMsgDOTASetGroupOpenStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTASetGroupOpenStatus {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTASetGroupOpenStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTASetGroupOpenStatus,
        };
        unsafe {
            instance.get(CMsgDOTASetGroupOpenStatus::new)
        }
    }

    // optional bool open = 1;

    pub fn clear_open(&mut self) {
        self.open = ::std::option::Option::None;
    }

    pub fn has_open(&self) -> bool {
        self.open.is_some()
    }

    // Param is passed by value, moved
    pub fn set_open(&mut self, v: bool) {
        self.open = ::std::option::Option::Some(v);
    }

    pub fn get_open(&self) -> bool {
        self.open.unwrap_or(false)
    }

    fn get_open_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.open
    }

    fn mut_open_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.open
    }
}

impl ::protobuf::Message for CMsgDOTASetGroupOpenStatus {
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
                    self.open = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.open {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.open {
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

impl ::protobuf::MessageStatic for CMsgDOTASetGroupOpenStatus {
    fn new() -> CMsgDOTASetGroupOpenStatus {
        CMsgDOTASetGroupOpenStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTASetGroupOpenStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "open",
                    CMsgDOTASetGroupOpenStatus::get_open_for_reflect,
                    CMsgDOTASetGroupOpenStatus::mut_open_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTASetGroupOpenStatus>(
                    "CMsgDOTASetGroupOpenStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTASetGroupOpenStatus {
    fn clear(&mut self) {
        self.clear_open();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTASetGroupOpenStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTASetGroupOpenStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGroupMergeInvite {
    // message fields
    other_group_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGroupMergeInvite {}

impl CMsgDOTAGroupMergeInvite {
    pub fn new() -> CMsgDOTAGroupMergeInvite {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGroupMergeInvite {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGroupMergeInvite> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGroupMergeInvite,
        };
        unsafe {
            instance.get(CMsgDOTAGroupMergeInvite::new)
        }
    }

    // optional fixed64 other_group_id = 1;

    pub fn clear_other_group_id(&mut self) {
        self.other_group_id = ::std::option::Option::None;
    }

    pub fn has_other_group_id(&self) -> bool {
        self.other_group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_other_group_id(&mut self, v: u64) {
        self.other_group_id = ::std::option::Option::Some(v);
    }

    pub fn get_other_group_id(&self) -> u64 {
        self.other_group_id.unwrap_or(0)
    }

    fn get_other_group_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.other_group_id
    }

    fn mut_other_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.other_group_id
    }
}

impl ::protobuf::Message for CMsgDOTAGroupMergeInvite {
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
                    self.other_group_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.other_group_id {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.other_group_id {
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

impl ::protobuf::MessageStatic for CMsgDOTAGroupMergeInvite {
    fn new() -> CMsgDOTAGroupMergeInvite {
        CMsgDOTAGroupMergeInvite::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGroupMergeInvite>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "other_group_id",
                    CMsgDOTAGroupMergeInvite::get_other_group_id_for_reflect,
                    CMsgDOTAGroupMergeInvite::mut_other_group_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGroupMergeInvite>(
                    "CMsgDOTAGroupMergeInvite",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGroupMergeInvite {
    fn clear(&mut self) {
        self.clear_other_group_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGroupMergeInvite {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGroupMergeInvite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGroupMergeResponse {
    // message fields
    initiator_group_id: ::std::option::Option<u64>,
    accept: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGroupMergeResponse {}

impl CMsgDOTAGroupMergeResponse {
    pub fn new() -> CMsgDOTAGroupMergeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGroupMergeResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGroupMergeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGroupMergeResponse,
        };
        unsafe {
            instance.get(CMsgDOTAGroupMergeResponse::new)
        }
    }

    // optional fixed64 initiator_group_id = 1;

    pub fn clear_initiator_group_id(&mut self) {
        self.initiator_group_id = ::std::option::Option::None;
    }

    pub fn has_initiator_group_id(&self) -> bool {
        self.initiator_group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initiator_group_id(&mut self, v: u64) {
        self.initiator_group_id = ::std::option::Option::Some(v);
    }

    pub fn get_initiator_group_id(&self) -> u64 {
        self.initiator_group_id.unwrap_or(0)
    }

    fn get_initiator_group_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.initiator_group_id
    }

    fn mut_initiator_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.initiator_group_id
    }

    // optional bool accept = 2;

    pub fn clear_accept(&mut self) {
        self.accept = ::std::option::Option::None;
    }

    pub fn has_accept(&self) -> bool {
        self.accept.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accept(&mut self, v: bool) {
        self.accept = ::std::option::Option::Some(v);
    }

    pub fn get_accept(&self) -> bool {
        self.accept.unwrap_or(false)
    }

    fn get_accept_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.accept
    }

    fn mut_accept_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.accept
    }
}

impl ::protobuf::Message for CMsgDOTAGroupMergeResponse {
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
                    self.initiator_group_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.accept = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.initiator_group_id {
            my_size += 9;
        };
        if let Some(v) = self.accept {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.initiator_group_id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.accept {
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

impl ::protobuf::MessageStatic for CMsgDOTAGroupMergeResponse {
    fn new() -> CMsgDOTAGroupMergeResponse {
        CMsgDOTAGroupMergeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGroupMergeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "initiator_group_id",
                    CMsgDOTAGroupMergeResponse::get_initiator_group_id_for_reflect,
                    CMsgDOTAGroupMergeResponse::mut_initiator_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "accept",
                    CMsgDOTAGroupMergeResponse::get_accept_for_reflect,
                    CMsgDOTAGroupMergeResponse::mut_accept_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGroupMergeResponse>(
                    "CMsgDOTAGroupMergeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGroupMergeResponse {
    fn clear(&mut self) {
        self.clear_initiator_group_id();
        self.clear_accept();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGroupMergeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGroupMergeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAGroupMergeReply {
    // message fields
    result: ::std::option::Option<super::dota_client_enums::EDOTAGroupMergeResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAGroupMergeReply {}

impl CMsgDOTAGroupMergeReply {
    pub fn new() -> CMsgDOTAGroupMergeReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAGroupMergeReply {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAGroupMergeReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAGroupMergeReply,
        };
        unsafe {
            instance.get(CMsgDOTAGroupMergeReply::new)
        }
    }

    // optional .dota.EDOTAGroupMergeResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: super::dota_client_enums::EDOTAGroupMergeResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> super::dota_client_enums::EDOTAGroupMergeResult {
        self.result.unwrap_or(super::dota_client_enums::EDOTAGroupMergeResult::k_EDOTAGroupMergeResult_OK)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<super::dota_client_enums::EDOTAGroupMergeResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_client_enums::EDOTAGroupMergeResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgDOTAGroupMergeReply {
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTAGroupMergeReply {
    fn new() -> CMsgDOTAGroupMergeReply {
        CMsgDOTAGroupMergeReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAGroupMergeReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_client_enums::EDOTAGroupMergeResult>>(
                    "result",
                    CMsgDOTAGroupMergeReply::get_result_for_reflect,
                    CMsgDOTAGroupMergeReply::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAGroupMergeReply>(
                    "CMsgDOTAGroupMergeReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAGroupMergeReply {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAGroupMergeReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAGroupMergeReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSpectatorLobbyGameDetails {
    // message fields
    language: ::std::option::Option<u32>,
    match_id: ::std::option::Option<u64>,
    server_steam_id: ::std::option::Option<u64>,
    stream_url: ::protobuf::SingularField<::std::string::String>,
    stream_name: ::protobuf::SingularField<::std::string::String>,
    league_id: ::std::option::Option<u32>,
    series_type: ::std::option::Option<u32>,
    series_game: ::std::option::Option<u32>,
    radiant_team: ::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails_Team>,
    dire_team: ::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails_Team>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSpectatorLobbyGameDetails {}

impl CMsgSpectatorLobbyGameDetails {
    pub fn new() -> CMsgSpectatorLobbyGameDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSpectatorLobbyGameDetails {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSpectatorLobbyGameDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSpectatorLobbyGameDetails,
        };
        unsafe {
            instance.get(CMsgSpectatorLobbyGameDetails::new)
        }
    }

    // optional uint32 language = 1;

    pub fn clear_language(&mut self) {
        self.language = ::std::option::Option::None;
    }

    pub fn has_language(&self) -> bool {
        self.language.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language(&mut self, v: u32) {
        self.language = ::std::option::Option::Some(v);
    }

    pub fn get_language(&self) -> u32 {
        self.language.unwrap_or(0)
    }

    fn get_language_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.language
    }

    fn mut_language_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.language
    }

    // optional uint64 match_id = 2;

    pub fn clear_match_id(&mut self) {
        self.match_id = ::std::option::Option::None;
    }

    pub fn has_match_id(&self) -> bool {
        self.match_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_id(&mut self, v: u64) {
        self.match_id = ::std::option::Option::Some(v);
    }

    pub fn get_match_id(&self) -> u64 {
        self.match_id.unwrap_or(0)
    }

    fn get_match_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.match_id
    }

    fn mut_match_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.match_id
    }

    // optional fixed64 server_steam_id = 3;

    pub fn clear_server_steam_id(&mut self) {
        self.server_steam_id = ::std::option::Option::None;
    }

    pub fn has_server_steam_id(&self) -> bool {
        self.server_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_steam_id(&mut self, v: u64) {
        self.server_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_steam_id(&self) -> u64 {
        self.server_steam_id.unwrap_or(0)
    }

    fn get_server_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.server_steam_id
    }

    fn mut_server_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.server_steam_id
    }

    // optional string stream_url = 4;

    pub fn clear_stream_url(&mut self) {
        self.stream_url.clear();
    }

    pub fn has_stream_url(&self) -> bool {
        self.stream_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stream_url(&mut self, v: ::std::string::String) {
        self.stream_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stream_url(&mut self) -> &mut ::std::string::String {
        if self.stream_url.is_none() {
            self.stream_url.set_default();
        };
        self.stream_url.as_mut().unwrap()
    }

    // Take field
    pub fn take_stream_url(&mut self) -> ::std::string::String {
        self.stream_url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_stream_url(&self) -> &str {
        match self.stream_url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_stream_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.stream_url
    }

    fn mut_stream_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.stream_url
    }

    // optional string stream_name = 5;

    pub fn clear_stream_name(&mut self) {
        self.stream_name.clear();
    }

    pub fn has_stream_name(&self) -> bool {
        self.stream_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stream_name(&mut self, v: ::std::string::String) {
        self.stream_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stream_name(&mut self) -> &mut ::std::string::String {
        if self.stream_name.is_none() {
            self.stream_name.set_default();
        };
        self.stream_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_stream_name(&mut self) -> ::std::string::String {
        self.stream_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_stream_name(&self) -> &str {
        match self.stream_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_stream_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.stream_name
    }

    fn mut_stream_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.stream_name
    }

    // optional uint32 league_id = 6;

    pub fn clear_league_id(&mut self) {
        self.league_id = ::std::option::Option::None;
    }

    pub fn has_league_id(&self) -> bool {
        self.league_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_id(&mut self, v: u32) {
        self.league_id = ::std::option::Option::Some(v);
    }

    pub fn get_league_id(&self) -> u32 {
        self.league_id.unwrap_or(0)
    }

    fn get_league_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.league_id
    }

    fn mut_league_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.league_id
    }

    // optional uint32 series_type = 7;

    pub fn clear_series_type(&mut self) {
        self.series_type = ::std::option::Option::None;
    }

    pub fn has_series_type(&self) -> bool {
        self.series_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_series_type(&mut self, v: u32) {
        self.series_type = ::std::option::Option::Some(v);
    }

    pub fn get_series_type(&self) -> u32 {
        self.series_type.unwrap_or(0)
    }

    fn get_series_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.series_type
    }

    fn mut_series_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.series_type
    }

    // optional uint32 series_game = 8;

    pub fn clear_series_game(&mut self) {
        self.series_game = ::std::option::Option::None;
    }

    pub fn has_series_game(&self) -> bool {
        self.series_game.is_some()
    }

    // Param is passed by value, moved
    pub fn set_series_game(&mut self, v: u32) {
        self.series_game = ::std::option::Option::Some(v);
    }

    pub fn get_series_game(&self) -> u32 {
        self.series_game.unwrap_or(0)
    }

    fn get_series_game_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.series_game
    }

    fn mut_series_game_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.series_game
    }

    // optional .dota.CMsgSpectatorLobbyGameDetails.Team radiant_team = 9;

    pub fn clear_radiant_team(&mut self) {
        self.radiant_team.clear();
    }

    pub fn has_radiant_team(&self) -> bool {
        self.radiant_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_team(&mut self, v: CMsgSpectatorLobbyGameDetails_Team) {
        self.radiant_team = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_radiant_team(&mut self) -> &mut CMsgSpectatorLobbyGameDetails_Team {
        if self.radiant_team.is_none() {
            self.radiant_team.set_default();
        };
        self.radiant_team.as_mut().unwrap()
    }

    // Take field
    pub fn take_radiant_team(&mut self) -> CMsgSpectatorLobbyGameDetails_Team {
        self.radiant_team.take().unwrap_or_else(|| CMsgSpectatorLobbyGameDetails_Team::new())
    }

    pub fn get_radiant_team(&self) -> &CMsgSpectatorLobbyGameDetails_Team {
        self.radiant_team.as_ref().unwrap_or_else(|| CMsgSpectatorLobbyGameDetails_Team::default_instance())
    }

    fn get_radiant_team_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails_Team> {
        &self.radiant_team
    }

    fn mut_radiant_team_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails_Team> {
        &mut self.radiant_team
    }

    // optional .dota.CMsgSpectatorLobbyGameDetails.Team dire_team = 10;

    pub fn clear_dire_team(&mut self) {
        self.dire_team.clear();
    }

    pub fn has_dire_team(&self) -> bool {
        self.dire_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dire_team(&mut self, v: CMsgSpectatorLobbyGameDetails_Team) {
        self.dire_team = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dire_team(&mut self) -> &mut CMsgSpectatorLobbyGameDetails_Team {
        if self.dire_team.is_none() {
            self.dire_team.set_default();
        };
        self.dire_team.as_mut().unwrap()
    }

    // Take field
    pub fn take_dire_team(&mut self) -> CMsgSpectatorLobbyGameDetails_Team {
        self.dire_team.take().unwrap_or_else(|| CMsgSpectatorLobbyGameDetails_Team::new())
    }

    pub fn get_dire_team(&self) -> &CMsgSpectatorLobbyGameDetails_Team {
        self.dire_team.as_ref().unwrap_or_else(|| CMsgSpectatorLobbyGameDetails_Team::default_instance())
    }

    fn get_dire_team_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails_Team> {
        &self.dire_team
    }

    fn mut_dire_team_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails_Team> {
        &mut self.dire_team
    }
}

impl ::protobuf::Message for CMsgSpectatorLobbyGameDetails {
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
                    self.language = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.server_steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.stream_url)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.stream_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_type = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_game = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.radiant_team)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dire_team)?;
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
        if let Some(v) = self.language {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.server_steam_id {
            my_size += 9;
        };
        if let Some(v) = self.stream_url.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.stream_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.series_type {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.series_game {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radiant_team.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.dire_team.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.language {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.match_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.server_steam_id {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.stream_url.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.stream_name.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.league_id {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.series_type {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.series_game {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.radiant_team.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.dire_team.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgSpectatorLobbyGameDetails {
    fn new() -> CMsgSpectatorLobbyGameDetails {
        CMsgSpectatorLobbyGameDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSpectatorLobbyGameDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "language",
                    CMsgSpectatorLobbyGameDetails::get_language_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_language_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CMsgSpectatorLobbyGameDetails::get_match_id_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "server_steam_id",
                    CMsgSpectatorLobbyGameDetails::get_server_steam_id_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_server_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stream_url",
                    CMsgSpectatorLobbyGameDetails::get_stream_url_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_stream_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stream_name",
                    CMsgSpectatorLobbyGameDetails::get_stream_name_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_stream_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgSpectatorLobbyGameDetails::get_league_id_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_type",
                    CMsgSpectatorLobbyGameDetails::get_series_type_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_series_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_game",
                    CMsgSpectatorLobbyGameDetails::get_series_game_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_series_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSpectatorLobbyGameDetails_Team>>(
                    "radiant_team",
                    CMsgSpectatorLobbyGameDetails::get_radiant_team_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_radiant_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSpectatorLobbyGameDetails_Team>>(
                    "dire_team",
                    CMsgSpectatorLobbyGameDetails::get_dire_team_for_reflect,
                    CMsgSpectatorLobbyGameDetails::mut_dire_team_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSpectatorLobbyGameDetails>(
                    "CMsgSpectatorLobbyGameDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSpectatorLobbyGameDetails {
    fn clear(&mut self) {
        self.clear_language();
        self.clear_match_id();
        self.clear_server_steam_id();
        self.clear_stream_url();
        self.clear_stream_name();
        self.clear_league_id();
        self.clear_series_type();
        self.clear_series_game();
        self.clear_radiant_team();
        self.clear_dire_team();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSpectatorLobbyGameDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSpectatorLobbyGameDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSpectatorLobbyGameDetails_Team {
    // message fields
    team_id: ::std::option::Option<u32>,
    team_name: ::protobuf::SingularField<::std::string::String>,
    team_logo: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSpectatorLobbyGameDetails_Team {}

impl CMsgSpectatorLobbyGameDetails_Team {
    pub fn new() -> CMsgSpectatorLobbyGameDetails_Team {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSpectatorLobbyGameDetails_Team {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSpectatorLobbyGameDetails_Team> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSpectatorLobbyGameDetails_Team,
        };
        unsafe {
            instance.get(CMsgSpectatorLobbyGameDetails_Team::new)
        }
    }

    // optional uint32 team_id = 1;

    pub fn clear_team_id(&mut self) {
        self.team_id = ::std::option::Option::None;
    }

    pub fn has_team_id(&self) -> bool {
        self.team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_id(&mut self, v: u32) {
        self.team_id = ::std::option::Option::Some(v);
    }

    pub fn get_team_id(&self) -> u32 {
        self.team_id.unwrap_or(0)
    }

    fn get_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team_id
    }

    fn mut_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team_id
    }

    // optional string team_name = 2;

    pub fn clear_team_name(&mut self) {
        self.team_name.clear();
    }

    pub fn has_team_name(&self) -> bool {
        self.team_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name(&mut self, v: ::std::string::String) {
        self.team_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name(&mut self) -> &mut ::std::string::String {
        if self.team_name.is_none() {
            self.team_name.set_default();
        };
        self.team_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name(&mut self) -> ::std::string::String {
        self.team_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name(&self) -> &str {
        match self.team_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name
    }

    fn mut_team_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name
    }

    // optional fixed64 team_logo = 3;

    pub fn clear_team_logo(&mut self) {
        self.team_logo = ::std::option::Option::None;
    }

    pub fn has_team_logo(&self) -> bool {
        self.team_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_logo(&mut self, v: u64) {
        self.team_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team_logo(&self) -> u64 {
        self.team_logo.unwrap_or(0)
    }

    fn get_team_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_logo
    }

    fn mut_team_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_logo
    }
}

impl ::protobuf::Message for CMsgSpectatorLobbyGameDetails_Team {
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
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.team_logo = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.team_logo {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.team_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.team_logo {
            os.write_fixed64(3, v)?;
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

impl ::protobuf::MessageStatic for CMsgSpectatorLobbyGameDetails_Team {
    fn new() -> CMsgSpectatorLobbyGameDetails_Team {
        CMsgSpectatorLobbyGameDetails_Team::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSpectatorLobbyGameDetails_Team>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgSpectatorLobbyGameDetails_Team::get_team_id_for_reflect,
                    CMsgSpectatorLobbyGameDetails_Team::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name",
                    CMsgSpectatorLobbyGameDetails_Team::get_team_name_for_reflect,
                    CMsgSpectatorLobbyGameDetails_Team::mut_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "team_logo",
                    CMsgSpectatorLobbyGameDetails_Team::get_team_logo_for_reflect,
                    CMsgSpectatorLobbyGameDetails_Team::mut_team_logo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSpectatorLobbyGameDetails_Team>(
                    "CMsgSpectatorLobbyGameDetails_Team",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSpectatorLobbyGameDetails_Team {
    fn clear(&mut self) {
        self.clear_team_id();
        self.clear_team_name();
        self.clear_team_logo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSpectatorLobbyGameDetails_Team {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSpectatorLobbyGameDetails_Team {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSetSpectatorLobbyDetails {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    lobby_name: ::protobuf::SingularField<::std::string::String>,
    pass_key: ::protobuf::SingularField<::std::string::String>,
    game_details: ::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSetSpectatorLobbyDetails {}

impl CMsgSetSpectatorLobbyDetails {
    pub fn new() -> CMsgSetSpectatorLobbyDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSetSpectatorLobbyDetails {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSetSpectatorLobbyDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSetSpectatorLobbyDetails,
        };
        unsafe {
            instance.get(CMsgSetSpectatorLobbyDetails::new)
        }
    }

    // optional uint64 lobby_id = 1;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }

    // optional string lobby_name = 2;

    pub fn clear_lobby_name(&mut self) {
        self.lobby_name.clear();
    }

    pub fn has_lobby_name(&self) -> bool {
        self.lobby_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_name(&mut self, v: ::std::string::String) {
        self.lobby_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lobby_name(&mut self) -> &mut ::std::string::String {
        if self.lobby_name.is_none() {
            self.lobby_name.set_default();
        };
        self.lobby_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_lobby_name(&mut self) -> ::std::string::String {
        self.lobby_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_lobby_name(&self) -> &str {
        match self.lobby_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_lobby_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.lobby_name
    }

    fn mut_lobby_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.lobby_name
    }

    // optional string pass_key = 3;

    pub fn clear_pass_key(&mut self) {
        self.pass_key.clear();
    }

    pub fn has_pass_key(&self) -> bool {
        self.pass_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pass_key(&mut self, v: ::std::string::String) {
        self.pass_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pass_key(&mut self) -> &mut ::std::string::String {
        if self.pass_key.is_none() {
            self.pass_key.set_default();
        };
        self.pass_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_pass_key(&mut self) -> ::std::string::String {
        self.pass_key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pass_key(&self) -> &str {
        match self.pass_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_pass_key_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.pass_key
    }

    fn mut_pass_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.pass_key
    }

    // optional .dota.CMsgSpectatorLobbyGameDetails game_details = 4;

    pub fn clear_game_details(&mut self) {
        self.game_details.clear();
    }

    pub fn has_game_details(&self) -> bool {
        self.game_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_details(&mut self, v: CMsgSpectatorLobbyGameDetails) {
        self.game_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_details(&mut self) -> &mut CMsgSpectatorLobbyGameDetails {
        if self.game_details.is_none() {
            self.game_details.set_default();
        };
        self.game_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_details(&mut self) -> CMsgSpectatorLobbyGameDetails {
        self.game_details.take().unwrap_or_else(|| CMsgSpectatorLobbyGameDetails::new())
    }

    pub fn get_game_details(&self) -> &CMsgSpectatorLobbyGameDetails {
        self.game_details.as_ref().unwrap_or_else(|| CMsgSpectatorLobbyGameDetails::default_instance())
    }

    fn get_game_details_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails> {
        &self.game_details
    }

    fn mut_game_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails> {
        &mut self.game_details
    }
}

impl ::protobuf::Message for CMsgSetSpectatorLobbyDetails {
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
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.lobby_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pass_key)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.game_details)?;
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
        if let Some(v) = self.lobby_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lobby_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.pass_key.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.game_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.lobby_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.pass_key.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.game_details.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgSetSpectatorLobbyDetails {
    fn new() -> CMsgSetSpectatorLobbyDetails {
        CMsgSetSpectatorLobbyDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSetSpectatorLobbyDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lobby_id",
                    CMsgSetSpectatorLobbyDetails::get_lobby_id_for_reflect,
                    CMsgSetSpectatorLobbyDetails::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lobby_name",
                    CMsgSetSpectatorLobbyDetails::get_lobby_name_for_reflect,
                    CMsgSetSpectatorLobbyDetails::mut_lobby_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pass_key",
                    CMsgSetSpectatorLobbyDetails::get_pass_key_for_reflect,
                    CMsgSetSpectatorLobbyDetails::mut_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSpectatorLobbyGameDetails>>(
                    "game_details",
                    CMsgSetSpectatorLobbyDetails::get_game_details_for_reflect,
                    CMsgSetSpectatorLobbyDetails::mut_game_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSetSpectatorLobbyDetails>(
                    "CMsgSetSpectatorLobbyDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSetSpectatorLobbyDetails {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.clear_lobby_name();
        self.clear_pass_key();
        self.clear_game_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSetSpectatorLobbyDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSetSpectatorLobbyDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgCreateSpectatorLobby {
    // message fields
    client_version: ::std::option::Option<u32>,
    details: ::protobuf::SingularPtrField<CMsgSetSpectatorLobbyDetails>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgCreateSpectatorLobby {}

impl CMsgCreateSpectatorLobby {
    pub fn new() -> CMsgCreateSpectatorLobby {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgCreateSpectatorLobby {
        static mut instance: ::protobuf::lazy::Lazy<CMsgCreateSpectatorLobby> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgCreateSpectatorLobby,
        };
        unsafe {
            instance.get(CMsgCreateSpectatorLobby::new)
        }
    }

    // optional uint32 client_version = 1;

    pub fn clear_client_version(&mut self) {
        self.client_version = ::std::option::Option::None;
    }

    pub fn has_client_version(&self) -> bool {
        self.client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_version(&mut self, v: u32) {
        self.client_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_version(&self) -> u32 {
        self.client_version.unwrap_or(0)
    }

    fn get_client_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_version
    }

    fn mut_client_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_version
    }

    // optional .dota.CMsgSetSpectatorLobbyDetails details = 2;

    pub fn clear_details(&mut self) {
        self.details.clear();
    }

    pub fn has_details(&self) -> bool {
        self.details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_details(&mut self, v: CMsgSetSpectatorLobbyDetails) {
        self.details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_details(&mut self) -> &mut CMsgSetSpectatorLobbyDetails {
        if self.details.is_none() {
            self.details.set_default();
        };
        self.details.as_mut().unwrap()
    }

    // Take field
    pub fn take_details(&mut self) -> CMsgSetSpectatorLobbyDetails {
        self.details.take().unwrap_or_else(|| CMsgSetSpectatorLobbyDetails::new())
    }

    pub fn get_details(&self) -> &CMsgSetSpectatorLobbyDetails {
        self.details.as_ref().unwrap_or_else(|| CMsgSetSpectatorLobbyDetails::default_instance())
    }

    fn get_details_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSetSpectatorLobbyDetails> {
        &self.details
    }

    fn mut_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSetSpectatorLobbyDetails> {
        &mut self.details
    }
}

impl ::protobuf::Message for CMsgCreateSpectatorLobby {
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
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.details)?;
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
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_version {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.details.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgCreateSpectatorLobby {
    fn new() -> CMsgCreateSpectatorLobby {
        CMsgCreateSpectatorLobby::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgCreateSpectatorLobby>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgCreateSpectatorLobby::get_client_version_for_reflect,
                    CMsgCreateSpectatorLobby::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSetSpectatorLobbyDetails>>(
                    "details",
                    CMsgCreateSpectatorLobby::get_details_for_reflect,
                    CMsgCreateSpectatorLobby::mut_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgCreateSpectatorLobby>(
                    "CMsgCreateSpectatorLobby",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgCreateSpectatorLobby {
    fn clear(&mut self) {
        self.clear_client_version();
        self.clear_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgCreateSpectatorLobby {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgCreateSpectatorLobby {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSpectatorLobbyList {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSpectatorLobbyList {}

impl CMsgSpectatorLobbyList {
    pub fn new() -> CMsgSpectatorLobbyList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSpectatorLobbyList {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSpectatorLobbyList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSpectatorLobbyList,
        };
        unsafe {
            instance.get(CMsgSpectatorLobbyList::new)
        }
    }
}

impl ::protobuf::Message for CMsgSpectatorLobbyList {
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

impl ::protobuf::MessageStatic for CMsgSpectatorLobbyList {
    fn new() -> CMsgSpectatorLobbyList {
        CMsgSpectatorLobbyList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSpectatorLobbyList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSpectatorLobbyList>(
                    "CMsgSpectatorLobbyList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSpectatorLobbyList {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSpectatorLobbyList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSpectatorLobbyList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSpectatorLobbyListResponse {
    // message fields
    lobbies: ::protobuf::RepeatedField<CMsgSpectatorLobbyListResponse_SpectatorLobby>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSpectatorLobbyListResponse {}

impl CMsgSpectatorLobbyListResponse {
    pub fn new() -> CMsgSpectatorLobbyListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSpectatorLobbyListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSpectatorLobbyListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSpectatorLobbyListResponse,
        };
        unsafe {
            instance.get(CMsgSpectatorLobbyListResponse::new)
        }
    }

    // repeated .dota.CMsgSpectatorLobbyListResponse.SpectatorLobby lobbies = 1;

    pub fn clear_lobbies(&mut self) {
        self.lobbies.clear();
    }

    // Param is passed by value, moved
    pub fn set_lobbies(&mut self, v: ::protobuf::RepeatedField<CMsgSpectatorLobbyListResponse_SpectatorLobby>) {
        self.lobbies = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lobbies(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSpectatorLobbyListResponse_SpectatorLobby> {
        &mut self.lobbies
    }

    // Take field
    pub fn take_lobbies(&mut self) -> ::protobuf::RepeatedField<CMsgSpectatorLobbyListResponse_SpectatorLobby> {
        ::std::mem::replace(&mut self.lobbies, ::protobuf::RepeatedField::new())
    }

    pub fn get_lobbies(&self) -> &[CMsgSpectatorLobbyListResponse_SpectatorLobby] {
        &self.lobbies
    }

    fn get_lobbies_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgSpectatorLobbyListResponse_SpectatorLobby> {
        &self.lobbies
    }

    fn mut_lobbies_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgSpectatorLobbyListResponse_SpectatorLobby> {
        &mut self.lobbies
    }
}

impl ::protobuf::Message for CMsgSpectatorLobbyListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lobbies)?;
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
        for value in &self.lobbies {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.lobbies {
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

impl ::protobuf::MessageStatic for CMsgSpectatorLobbyListResponse {
    fn new() -> CMsgSpectatorLobbyListResponse {
        CMsgSpectatorLobbyListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSpectatorLobbyListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSpectatorLobbyListResponse_SpectatorLobby>>(
                    "lobbies",
                    CMsgSpectatorLobbyListResponse::get_lobbies_for_reflect,
                    CMsgSpectatorLobbyListResponse::mut_lobbies_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSpectatorLobbyListResponse>(
                    "CMsgSpectatorLobbyListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSpectatorLobbyListResponse {
    fn clear(&mut self) {
        self.clear_lobbies();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSpectatorLobbyListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSpectatorLobbyListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSpectatorLobbyListResponse_SpectatorLobby {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    game_name: ::protobuf::SingularField<::std::string::String>,
    requires_pass_key: ::std::option::Option<bool>,
    leader_account_id: ::std::option::Option<u32>,
    member_count: ::std::option::Option<u32>,
    game_details: ::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSpectatorLobbyListResponse_SpectatorLobby {}

impl CMsgSpectatorLobbyListResponse_SpectatorLobby {
    pub fn new() -> CMsgSpectatorLobbyListResponse_SpectatorLobby {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSpectatorLobbyListResponse_SpectatorLobby {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSpectatorLobbyListResponse_SpectatorLobby> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSpectatorLobbyListResponse_SpectatorLobby,
        };
        unsafe {
            instance.get(CMsgSpectatorLobbyListResponse_SpectatorLobby::new)
        }
    }

    // optional uint64 lobby_id = 1;

    pub fn clear_lobby_id(&mut self) {
        self.lobby_id = ::std::option::Option::None;
    }

    pub fn has_lobby_id(&self) -> bool {
        self.lobby_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_id(&mut self, v: u64) {
        self.lobby_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_id(&self) -> u64 {
        self.lobby_id.unwrap_or(0)
    }

    fn get_lobby_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lobby_id
    }

    fn mut_lobby_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lobby_id
    }

    // optional string game_name = 2;

    pub fn clear_game_name(&mut self) {
        self.game_name.clear();
    }

    pub fn has_game_name(&self) -> bool {
        self.game_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_name(&mut self, v: ::std::string::String) {
        self.game_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_name(&mut self) -> &mut ::std::string::String {
        if self.game_name.is_none() {
            self.game_name.set_default();
        };
        self.game_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_name(&mut self) -> ::std::string::String {
        self.game_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_name(&self) -> &str {
        match self.game_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_name
    }

    fn mut_game_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_name
    }

    // optional bool requires_pass_key = 3;

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

    // optional uint32 leader_account_id = 4;

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

    // optional uint32 member_count = 5;

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

    // optional .dota.CMsgSpectatorLobbyGameDetails game_details = 7;

    pub fn clear_game_details(&mut self) {
        self.game_details.clear();
    }

    pub fn has_game_details(&self) -> bool {
        self.game_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_details(&mut self, v: CMsgSpectatorLobbyGameDetails) {
        self.game_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_details(&mut self) -> &mut CMsgSpectatorLobbyGameDetails {
        if self.game_details.is_none() {
            self.game_details.set_default();
        };
        self.game_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_details(&mut self) -> CMsgSpectatorLobbyGameDetails {
        self.game_details.take().unwrap_or_else(|| CMsgSpectatorLobbyGameDetails::new())
    }

    pub fn get_game_details(&self) -> &CMsgSpectatorLobbyGameDetails {
        self.game_details.as_ref().unwrap_or_else(|| CMsgSpectatorLobbyGameDetails::default_instance())
    }

    fn get_game_details_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails> {
        &self.game_details
    }

    fn mut_game_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgSpectatorLobbyGameDetails> {
        &mut self.game_details
    }
}

impl ::protobuf::Message for CMsgSpectatorLobbyListResponse_SpectatorLobby {
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
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.requires_pass_key = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.leader_account_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.member_count = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.game_details)?;
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
        if let Some(v) = self.lobby_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.requires_pass_key {
            my_size += 2;
        };
        if let Some(v) = self.leader_account_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.member_count {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.game_name.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.requires_pass_key {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.leader_account_id {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.member_count {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.game_details.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CMsgSpectatorLobbyListResponse_SpectatorLobby {
    fn new() -> CMsgSpectatorLobbyListResponse_SpectatorLobby {
        CMsgSpectatorLobbyListResponse_SpectatorLobby::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSpectatorLobbyListResponse_SpectatorLobby>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lobby_id",
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::get_lobby_id_for_reflect,
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_name",
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::get_game_name_for_reflect,
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::mut_game_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "requires_pass_key",
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::get_requires_pass_key_for_reflect,
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::mut_requires_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leader_account_id",
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::get_leader_account_id_for_reflect,
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::mut_leader_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "member_count",
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::get_member_count_for_reflect,
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::mut_member_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgSpectatorLobbyGameDetails>>(
                    "game_details",
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::get_game_details_for_reflect,
                    CMsgSpectatorLobbyListResponse_SpectatorLobby::mut_game_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSpectatorLobbyListResponse_SpectatorLobby>(
                    "CMsgSpectatorLobbyListResponse_SpectatorLobby",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSpectatorLobbyListResponse_SpectatorLobby {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.clear_game_name();
        self.clear_requires_pass_key();
        self.clear_leader_account_id();
        self.clear_member_count();
        self.clear_game_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSpectatorLobbyListResponse_SpectatorLobby {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSpectatorLobbyListResponse_SpectatorLobby {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCRequestSteamDatagramTicket {
    // message fields
    server_steam_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCRequestSteamDatagramTicket {}

impl CMsgClientToGCRequestSteamDatagramTicket {
    pub fn new() -> CMsgClientToGCRequestSteamDatagramTicket {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCRequestSteamDatagramTicket {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCRequestSteamDatagramTicket> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCRequestSteamDatagramTicket,
        };
        unsafe {
            instance.get(CMsgClientToGCRequestSteamDatagramTicket::new)
        }
    }

    // optional fixed64 server_steam_id = 1;

    pub fn clear_server_steam_id(&mut self) {
        self.server_steam_id = ::std::option::Option::None;
    }

    pub fn has_server_steam_id(&self) -> bool {
        self.server_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_steam_id(&mut self, v: u64) {
        self.server_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_steam_id(&self) -> u64 {
        self.server_steam_id.unwrap_or(0)
    }

    fn get_server_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.server_steam_id
    }

    fn mut_server_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.server_steam_id
    }
}

impl ::protobuf::Message for CMsgClientToGCRequestSteamDatagramTicket {
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
                    self.server_steam_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.server_steam_id {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_steam_id {
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

impl ::protobuf::MessageStatic for CMsgClientToGCRequestSteamDatagramTicket {
    fn new() -> CMsgClientToGCRequestSteamDatagramTicket {
        CMsgClientToGCRequestSteamDatagramTicket::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCRequestSteamDatagramTicket>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "server_steam_id",
                    CMsgClientToGCRequestSteamDatagramTicket::get_server_steam_id_for_reflect,
                    CMsgClientToGCRequestSteamDatagramTicket::mut_server_steam_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCRequestSteamDatagramTicket>(
                    "CMsgClientToGCRequestSteamDatagramTicket",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCRequestSteamDatagramTicket {
    fn clear(&mut self) {
        self.clear_server_steam_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCRequestSteamDatagramTicket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCRequestSteamDatagramTicket {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCRequestSteamDatagramTicketResponse {
    // message fields
    serialized_ticket: ::protobuf::SingularField<::std::string::String>,
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCRequestSteamDatagramTicketResponse {}

impl CMsgClientToGCRequestSteamDatagramTicketResponse {
    pub fn new() -> CMsgClientToGCRequestSteamDatagramTicketResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCRequestSteamDatagramTicketResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCRequestSteamDatagramTicketResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCRequestSteamDatagramTicketResponse,
        };
        unsafe {
            instance.get(CMsgClientToGCRequestSteamDatagramTicketResponse::new)
        }
    }

    // optional string serialized_ticket = 1;

    pub fn clear_serialized_ticket(&mut self) {
        self.serialized_ticket.clear();
    }

    pub fn has_serialized_ticket(&self) -> bool {
        self.serialized_ticket.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serialized_ticket(&mut self, v: ::std::string::String) {
        self.serialized_ticket = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serialized_ticket(&mut self) -> &mut ::std::string::String {
        if self.serialized_ticket.is_none() {
            self.serialized_ticket.set_default();
        };
        self.serialized_ticket.as_mut().unwrap()
    }

    // Take field
    pub fn take_serialized_ticket(&mut self) -> ::std::string::String {
        self.serialized_ticket.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_serialized_ticket(&self) -> &str {
        match self.serialized_ticket.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_serialized_ticket_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.serialized_ticket
    }

    fn mut_serialized_ticket_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.serialized_ticket
    }

    // optional string message = 2;

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

impl ::protobuf::Message for CMsgClientToGCRequestSteamDatagramTicketResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.serialized_ticket)?;
                },
                2 => {
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
        if let Some(v) = self.serialized_ticket.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.serialized_ticket.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.message.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgClientToGCRequestSteamDatagramTicketResponse {
    fn new() -> CMsgClientToGCRequestSteamDatagramTicketResponse {
        CMsgClientToGCRequestSteamDatagramTicketResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCRequestSteamDatagramTicketResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "serialized_ticket",
                    CMsgClientToGCRequestSteamDatagramTicketResponse::get_serialized_ticket_for_reflect,
                    CMsgClientToGCRequestSteamDatagramTicketResponse::mut_serialized_ticket_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CMsgClientToGCRequestSteamDatagramTicketResponse::get_message_for_reflect,
                    CMsgClientToGCRequestSteamDatagramTicketResponse::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCRequestSteamDatagramTicketResponse>(
                    "CMsgClientToGCRequestSteamDatagramTicketResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCRequestSteamDatagramTicketResponse {
    fn clear(&mut self) {
        self.clear_serialized_ticket();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCRequestSteamDatagramTicketResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCRequestSteamDatagramTicketResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EStartFindingMatchResult {
    k_EStartFindingMatchResult_Invalid = 0,
    k_EStartFindingMatchResult_OK = 1,
    k_EStartFindingMatchResult_AlreadySearching = 2,
    k_EStartFindingMatchResult_FailGeneric = 100,
    k_EStartFindingMatchResult_FailedIgnore = 101,
    k_EStartFindingMatchResult_MatchmakingDisabled = 102,
    k_EStartFindingMatchResult_RegionOffline = 103,
    k_EStartFindingMatchResult_MatchmakingCooldown = 104,
    k_EStartFindingMatchResult_ClientOutOfDate = 105,
    k_EStartFindingMatchResult_CompetitiveNoLowPriority = 106,
    k_EStartFindingMatchResult_CompetitiveNotUnlocked = 107,
    k_EStartFindingMatchResult_GameModeNotUnlocked = 108,
    k_EStartFindingMatchResult_CompetitiveNotEnoughSkillData = 109,
    k_EStartFindingMatchResult_MissingInitialSkill = 110,
    k_EStartFindingMatchResult_CompetitiveRankSpreadTooLarge = 111,
    k_EStartFindingMatchResult_MemberAlreadyInLobby = 112,
    k_EStartFindingMatchResult_MemberNotVACVerified = 113,
    k_EStartFindingMatchResult_WeekendTourneyBadPartySize = 114,
    k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooSmall = 115,
    k_EStartFindingMatchResult_WeekendTourneyIndividualBuyInTooLarge = 116,
    k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooLarge = 117,
    k_EStartFindingMatchResult_MemberMissingEventOwnership = 118,
    k_EStartFindingMatchResult_WeekendTourneyNotUnlocked = 119,
    k_EStartFindingMatchResult_WeekendTourneyRecentParticipation = 120,
}

impl ::protobuf::ProtobufEnum for EStartFindingMatchResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EStartFindingMatchResult> {
        match value {
            0 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_Invalid),
            1 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_OK),
            2 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_AlreadySearching),
            100 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_FailGeneric),
            101 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_FailedIgnore),
            102 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_MatchmakingDisabled),
            103 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_RegionOffline),
            104 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_MatchmakingCooldown),
            105 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_ClientOutOfDate),
            106 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_CompetitiveNoLowPriority),
            107 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_CompetitiveNotUnlocked),
            108 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_GameModeNotUnlocked),
            109 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_CompetitiveNotEnoughSkillData),
            110 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_MissingInitialSkill),
            111 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_CompetitiveRankSpreadTooLarge),
            112 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_MemberAlreadyInLobby),
            113 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_MemberNotVACVerified),
            114 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyBadPartySize),
            115 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooSmall),
            116 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyIndividualBuyInTooLarge),
            117 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooLarge),
            118 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_MemberMissingEventOwnership),
            119 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyNotUnlocked),
            120 => ::std::option::Option::Some(EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyRecentParticipation),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EStartFindingMatchResult] = &[
            EStartFindingMatchResult::k_EStartFindingMatchResult_Invalid,
            EStartFindingMatchResult::k_EStartFindingMatchResult_OK,
            EStartFindingMatchResult::k_EStartFindingMatchResult_AlreadySearching,
            EStartFindingMatchResult::k_EStartFindingMatchResult_FailGeneric,
            EStartFindingMatchResult::k_EStartFindingMatchResult_FailedIgnore,
            EStartFindingMatchResult::k_EStartFindingMatchResult_MatchmakingDisabled,
            EStartFindingMatchResult::k_EStartFindingMatchResult_RegionOffline,
            EStartFindingMatchResult::k_EStartFindingMatchResult_MatchmakingCooldown,
            EStartFindingMatchResult::k_EStartFindingMatchResult_ClientOutOfDate,
            EStartFindingMatchResult::k_EStartFindingMatchResult_CompetitiveNoLowPriority,
            EStartFindingMatchResult::k_EStartFindingMatchResult_CompetitiveNotUnlocked,
            EStartFindingMatchResult::k_EStartFindingMatchResult_GameModeNotUnlocked,
            EStartFindingMatchResult::k_EStartFindingMatchResult_CompetitiveNotEnoughSkillData,
            EStartFindingMatchResult::k_EStartFindingMatchResult_MissingInitialSkill,
            EStartFindingMatchResult::k_EStartFindingMatchResult_CompetitiveRankSpreadTooLarge,
            EStartFindingMatchResult::k_EStartFindingMatchResult_MemberAlreadyInLobby,
            EStartFindingMatchResult::k_EStartFindingMatchResult_MemberNotVACVerified,
            EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyBadPartySize,
            EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooSmall,
            EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyIndividualBuyInTooLarge,
            EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooLarge,
            EStartFindingMatchResult::k_EStartFindingMatchResult_MemberMissingEventOwnership,
            EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyNotUnlocked,
            EStartFindingMatchResult::k_EStartFindingMatchResult_WeekendTourneyRecentParticipation,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EStartFindingMatchResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EStartFindingMatchResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EStartFindingMatchResult {
}

impl ::protobuf::reflect::ProtobufValue for EStartFindingMatchResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x2d, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x6d,
    0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x13, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x64, 0x6f, 0x74, 0x61,
    0x5f, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x62, 0x61,
    0x73, 0x65, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x2d, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x6d, 0x61, 0x74, 0x63,
    0x68, 0x5f, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0xa6, 0x05, 0x0a, 0x15, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x12, 0x10, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x2c,
    0x0a, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0d, 0x3a, 0x0a, 0x34, 0x32, 0x39, 0x34, 0x39, 0x36, 0x37, 0x32, 0x39, 0x35, 0x52,
    0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x12, 0x25, 0x0a, 0x0e,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x12, 0x29, 0x0a, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65,
    0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x0a, 0x34, 0x32, 0x39, 0x34, 0x39, 0x36, 0x37,
    0x32, 0x39, 0x35, 0x52, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x73, 0x12, 0x53,
    0x0a, 0x0e, 0x62, 0x6f, 0x74, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f,
    0x54, 0x41, 0x42, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x3a,
    0x13, 0x42, 0x4f, 0x54, 0x5f, 0x44, 0x49, 0x46, 0x46, 0x49, 0x43, 0x55, 0x4c, 0x54, 0x59, 0x5f,
    0x48, 0x41, 0x52, 0x44, 0x52, 0x0d, 0x62, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75,
    0x6c, 0x74, 0x79, 0x12, 0x41, 0x0a, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x11, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x5f,
    0x54, 0x59, 0x50, 0x45, 0x5f, 0x43, 0x41, 0x53, 0x55, 0x41, 0x4c, 0x52, 0x09, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x54, 0x79, 0x70, 0x65, 0x12, 0x32, 0x0a, 0x0e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x6c,
    0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x0a,
    0x34, 0x32, 0x39, 0x34, 0x39, 0x36, 0x37, 0x32, 0x39, 0x35, 0x52, 0x0e, 0x6d, 0x61, 0x74, 0x63,
    0x68, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x6d, 0x61,
    0x70, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0d, 0x6d, 0x61, 0x70, 0x50, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63,
    0x65, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x5a, 0x0a, 0x12, 0x67, 0x61,
    0x6d, 0x65, 0x5f, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x5f, 0x65, 0x6e, 0x75, 0x6d,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x3a, 0x16, 0x4d, 0x41,
    0x54, 0x43, 0x48, 0x5f, 0x4c, 0x41, 0x4e, 0x47, 0x55, 0x41, 0x47, 0x45, 0x5f, 0x49, 0x4e, 0x56,
    0x41, 0x4c, 0x49, 0x44, 0x52, 0x10, 0x67, 0x61, 0x6d, 0x65, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61,
    0x67, 0x65, 0x45, 0x6e, 0x75, 0x6d, 0x12, 0x2c, 0x0a, 0x12, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6c,
    0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x10, 0x67, 0x61, 0x6d, 0x65, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65,
    0x4e, 0x61, 0x6d, 0x65, 0x12, 0x35, 0x0a, 0x09, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x44, 0x61, 0x74,
    0x61, 0x52, 0x08, 0x70, 0x69, 0x6e, 0x67, 0x44, 0x61, 0x74, 0x61, 0x12, 0x2e, 0x0a, 0x13, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x5f, 0x66, 0x6c, 0x61,
    0x67, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x22, 0xb4, 0x02, 0x0a, 0x1b,
    0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x37, 0x0a, 0x16, 0x6c,
    0x65, 0x67, 0x61, 0x63, 0x79, 0x5f, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x65, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x32, 0x52, 0x14,
    0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x45, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x12, 0x5a, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x1e, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x53, 0x74, 0x61,
    0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x3a, 0x22, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x5f, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x12, 0x1f, 0x0a, 0x0b, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x54, 0x6f, 0x6b, 0x65,
    0x6e, 0x12, 0x23, 0x0a, 0x0d, 0x64, 0x65, 0x62, 0x75, 0x67, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x64, 0x65, 0x62, 0x75, 0x67, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3a, 0x0a, 0x19, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x69, 0x62, 0x6c, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x6d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x06, 0x52, 0x17, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x50, 0x61, 0x72, 0x74, 0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x73, 0x22, 0x16, 0x0a, 0x14, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x74, 0x6f, 0x70, 0x46, 0x69,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x22, 0x9a, 0x02, 0x0a, 0x17, 0x43,
    0x4d, 0x73, 0x67, 0x50, 0x61, 0x72, 0x74, 0x79, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x65, 0x72, 0x4f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x73, 0x6c, 0x6f, 0x74, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0f, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x53, 0x6c, 0x6f, 0x74,
    0x73, 0x12, 0x41, 0x0a, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x11, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x5f, 0x54, 0x59,
    0x50, 0x45, 0x5f, 0x43, 0x41, 0x53, 0x55, 0x41, 0x4c, 0x52, 0x09, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x48, 0x0a,
    0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4c, 0x61, 0x6e, 0x67,
    0x75, 0x61, 0x67, 0x65, 0x73, 0x3a, 0x16, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x5f, 0x4c, 0x41, 0x4e,
    0x47, 0x55, 0x41, 0x47, 0x45, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x52, 0x08, 0x6c,
    0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x22, 0xc7, 0x01, 0x0a, 0x0b, 0x43, 0x4d, 0x73, 0x67,
    0x52, 0x65, 0x61, 0x64, 0x79, 0x55, 0x70, 0x12, 0x4f, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f,
    0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52, 0x65, 0x61, 0x64, 0x79, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x3a, 0x1e, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52, 0x65, 0x61, 0x64,
    0x79, 0x53, 0x74, 0x61, 0x74, 0x65, 0x5f, 0x55, 0x4e, 0x44, 0x45, 0x43, 0x4c, 0x41, 0x52, 0x45,
    0x44, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12, 0x20, 0x0a, 0x0c, 0x72, 0x65, 0x61, 0x64,
    0x79, 0x5f, 0x75, 0x70, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0a,
    0x72, 0x65, 0x61, 0x64, 0x79, 0x55, 0x70, 0x4b, 0x65, 0x79, 0x12, 0x45, 0x0a, 0x0e, 0x68, 0x61,
    0x72, 0x64, 0x77, 0x61, 0x72, 0x65, 0x5f, 0x73, 0x70, 0x65, 0x63, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x48, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72, 0x65, 0x53, 0x70, 0x65,
    0x63, 0x73, 0x52, 0x0d, 0x68, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72, 0x65, 0x53, 0x70, 0x65, 0x63,
    0x73, 0x22, 0x74, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x61, 0x64, 0x79, 0x55, 0x70,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49,
    0x64, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x65, 0x64, 0x5f, 0x69, 0x64,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x65,
    0x64, 0x49, 0x64, 0x73, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x65, 0x63, 0x6c, 0x69, 0x6e, 0x65, 0x64,
    0x5f, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0b, 0x64, 0x65, 0x63, 0x6c,
    0x69, 0x6e, 0x65, 0x64, 0x49, 0x64, 0x73, 0x22, 0x18, 0x0a, 0x16, 0x43, 0x4d, 0x73, 0x67, 0x41,
    0x62, 0x61, 0x6e, 0x64, 0x6f, 0x6e, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x47, 0x61, 0x6d,
    0x65, 0x22, 0xbd, 0x11, 0x0a, 0x1b, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69,
    0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x53, 0x65, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c,
    0x73, 0x12, 0x19, 0x0a, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09,
    0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x08, 0x67, 0x61, 0x6d, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x3a, 0x0a, 0x0c, 0x74, 0x65, 0x61,
    0x6d, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x54, 0x65, 0x61,
    0x6d, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x0b, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x65,
    0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f,
    0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61,
    0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x67,
    0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x3b, 0x0a, 0x07, 0x63, 0x6d, 0x5f, 0x70, 0x69,
    0x63, 0x6b, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x50, 0x49, 0x43, 0x4b, 0x3a, 0x0e, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x43, 0x4d, 0x5f, 0x52, 0x41, 0x4e, 0x44, 0x4f, 0x4d, 0x52, 0x06, 0x63, 0x6d,
    0x50, 0x69, 0x63, 0x6b, 0x12, 0x65, 0x0a, 0x16, 0x62, 0x6f, 0x74, 0x5f, 0x64, 0x69, 0x66, 0x66,
    0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41,
    0x42, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x3a, 0x16, 0x42,
    0x4f, 0x54, 0x5f, 0x44, 0x49, 0x46, 0x46, 0x49, 0x43, 0x55, 0x4c, 0x54, 0x59, 0x5f, 0x50, 0x41,
    0x53, 0x53, 0x49, 0x56, 0x45, 0x52, 0x14, 0x62, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63,
    0x75, 0x6c, 0x74, 0x79, 0x52, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x61,
    0x6c, 0x6c, 0x6f, 0x77, 0x5f, 0x63, 0x68, 0x65, 0x61, 0x74, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0b, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x43, 0x68, 0x65, 0x61, 0x74, 0x73, 0x12, 0x24,
    0x0a, 0x0e, 0x66, 0x69, 0x6c, 0x6c, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x5f, 0x62, 0x6f, 0x74, 0x73,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x66, 0x69, 0x6c, 0x6c, 0x57, 0x69, 0x74, 0x68,
    0x42, 0x6f, 0x74, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x5f, 0x6d, 0x6f,
    0x64, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x4d,
    0x6f, 0x64, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x5f, 0x73, 0x70, 0x65,
    0x63, 0x74, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x61,
    0x6c, 0x6c, 0x6f, 0x77, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x12, 0x4e,
    0x0a, 0x0c, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x0e,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x15, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41,
    0x47, 0x61, 0x6d, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x14, 0x47, 0x41, 0x4d,
    0x45, 0x5f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x55, 0x52, 0x52, 0x45, 0x4e,
    0x54, 0x52, 0x0b, 0x67, 0x61, 0x6d, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x19,
    0x0a, 0x08, 0x70, 0x61, 0x73, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x70, 0x61, 0x73, 0x73, 0x4b, 0x65, 0x79, 0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x69, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x6c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x69, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79,
    0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x18, 0x11,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x13, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79, 0x4c, 0x65, 0x76,
    0x65, 0x6c, 0x52, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x65, 0x6e,
    0x61, 0x6c, 0x74, 0x79, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x18,
    0x12, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79, 0x4c, 0x65,
    0x76, 0x65, 0x6c, 0x44, 0x69, 0x72, 0x65, 0x12, 0x20, 0x0a, 0x0c, 0x6c, 0x6f, 0x61, 0x64, 0x5f,
    0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x6c,
    0x6f, 0x61, 0x64, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x72,
    0x69, 0x65, 0x73, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a,
    0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x2e, 0x0a, 0x13, 0x72, 0x61,
    0x64, 0x69, 0x61, 0x6e, 0x74, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x77, 0x69, 0x6e,
    0x73, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74,
    0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x57, 0x69, 0x6e, 0x73, 0x12, 0x28, 0x0a, 0x10, 0x64, 0x69,
    0x72, 0x65, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x77, 0x69, 0x6e, 0x73, 0x18, 0x16,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x64, 0x69, 0x72, 0x65, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73,
    0x57, 0x69, 0x6e, 0x73, 0x12, 0x1f, 0x0a, 0x07, 0x61, 0x6c, 0x6c, 0x63, 0x68, 0x61, 0x74, 0x18,
    0x17, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x52, 0x07, 0x61, 0x6c,
    0x6c, 0x63, 0x68, 0x61, 0x74, 0x12, 0x4b, 0x0a, 0x0d, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x74, 0x76,
    0x5f, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x18, 0x18, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x54, 0x56, 0x44,
    0x65, 0x6c, 0x61, 0x79, 0x3a, 0x0f, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x54,
    0x56, 0x5f, 0x31, 0x32, 0x30, 0x52, 0x0b, 0x64, 0x6f, 0x74, 0x61, 0x54, 0x76, 0x44, 0x65, 0x6c,
    0x61, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x61, 0x6e, 0x18, 0x19, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x03, 0x6c, 0x61, 0x6e, 0x12, 0x28, 0x0a, 0x10, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67,
    0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x1a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e,
    0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x26,
    0x0a, 0x0f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x6d, 0x61, 0x70, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x1b, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4d,
    0x61, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x2b, 0x0a, 0x11, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x5f, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x18, 0x1c, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x10, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75,
    0x6c, 0x74, 0x79, 0x12, 0x24, 0x0a, 0x0e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61,
    0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x63, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x12, 0x63, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x5f, 0x6d, 0x69, 0x6e, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x18,
    0x1e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4d, 0x69, 0x6e,
    0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x2c, 0x0a, 0x12, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x18, 0x1f, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x10, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4d, 0x61, 0x78, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x41, 0x0a, 0x1e, 0x6c, 0x61, 0x6e, 0x5f, 0x68, 0x6f, 0x73,
    0x74, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x6f, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x20, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x19, 0x6c,
    0x61, 0x6e, 0x48, 0x6f, 0x73, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x54, 0x6f, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x55, 0x0a, 0x0a, 0x76, 0x69, 0x73, 0x69,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x21, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x56, 0x69, 0x73,
    0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x3a, 0x1a, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62,
    0x62, 0x79, 0x56, 0x69, 0x73, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x50, 0x75, 0x62,
    0x6c, 0x69, 0x63, 0x52, 0x0a, 0x76, 0x69, 0x73, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12,
    0x26, 0x0a, 0x0f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x63,
    0x72, 0x63, 0x18, 0x22, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x47, 0x61, 0x6d, 0x65, 0x43, 0x72, 0x63, 0x12, 0x28, 0x0a, 0x10, 0x6c, 0x65, 0x61, 0x67, 0x75,
    0x65, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x69, 0x64, 0x18, 0x23, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0e, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x49,
    0x64, 0x12, 0x24, 0x0a, 0x0e, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x67, 0x61, 0x6d, 0x65,
    0x5f, 0x69, 0x64, 0x18, 0x24, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x6c, 0x65, 0x61, 0x67, 0x75,
    0x65, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x18, 0x25, 0x20, 0x01, 0x28, 0x07, 0x52, 0x13, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61,
    0x6d, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x36, 0x0a, 0x17, 0x70,
    0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x6f, 0x76,
    0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x18, 0x26, 0x20, 0x01, 0x28, 0x04, 0x52, 0x15, 0x70, 0x72,
    0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4f, 0x76, 0x65, 0x72, 0x72,
    0x69, 0x64, 0x65, 0x12, 0x43, 0x0a, 0x1e, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x73, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x27, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1b, 0x6c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x79, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x6f, 0x0a, 0x20, 0x6c, 0x65, 0x61, 0x67,
    0x75, 0x65, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x63, 0x68, 0x6f, 0x69, 0x63, 0x65, 0x18, 0x28, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x3a,
    0x09, 0x55, 0x4e, 0x44, 0x45, 0x46, 0x49, 0x4e, 0x45, 0x44, 0x52, 0x1d, 0x6c, 0x65, 0x61, 0x67,
    0x75, 0x65, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x69, 0x6f, 0x72,
    0x69, 0x74, 0x79, 0x43, 0x68, 0x6f, 0x69, 0x63, 0x65, 0x12, 0x76, 0x0a, 0x24, 0x6c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x5f, 0x6e, 0x6f, 0x6e, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x63, 0x68, 0x6f, 0x69, 0x63,
    0x65, 0x18, 0x29, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x53,
    0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x54, 0x79, 0x70, 0x65, 0x3a, 0x09, 0x55, 0x4e, 0x44, 0x45, 0x46, 0x49, 0x4e, 0x45, 0x44, 0x52,
    0x20, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x4e, 0x6f, 0x6e, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x43, 0x68, 0x6f, 0x69, 0x63,
    0x65, 0x12, 0x61, 0x0a, 0x0d, 0x70, 0x61, 0x75, 0x73, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x18, 0x2a, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x50, 0x61, 0x75, 0x73, 0x65, 0x53, 0x65,
    0x74, 0x74, 0x69, 0x6e, 0x67, 0x3a, 0x1f, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61,
    0x50, 0x61, 0x75, 0x73, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x55, 0x6e, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x52, 0x0c, 0x70, 0x61, 0x75, 0x73, 0x65, 0x53, 0x65, 0x74,
    0x74, 0x69, 0x6e, 0x67, 0x12, 0x5f, 0x0a, 0x13, 0x62, 0x6f, 0x74, 0x5f, 0x64, 0x69, 0x66, 0x66,
    0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x18, 0x2b, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x42, 0x6f, 0x74,
    0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x3a, 0x16, 0x42, 0x4f, 0x54, 0x5f,
    0x44, 0x49, 0x46, 0x46, 0x49, 0x43, 0x55, 0x4c, 0x54, 0x59, 0x5f, 0x50, 0x41, 0x53, 0x53, 0x49,
    0x56, 0x45, 0x52, 0x11, 0x62, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74,
    0x79, 0x44, 0x69, 0x72, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x6f, 0x74, 0x5f, 0x72, 0x61, 0x64,
    0x69, 0x61, 0x6e, 0x74, 0x18, 0x2c, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x62, 0x6f, 0x74, 0x52,
    0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x62, 0x6f, 0x74, 0x5f, 0x64, 0x69,
    0x72, 0x65, 0x18, 0x2d, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x62, 0x6f, 0x74, 0x44, 0x69, 0x72,
    0x65, 0x22, 0xfa, 0x02, 0x0a, 0x17, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69,
    0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x1d, 0x0a,
    0x0a, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x09, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4b, 0x65, 0x79, 0x12, 0x19, 0x0a, 0x08,
    0x70, 0x61, 0x73, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07,
    0x70, 0x61, 0x73, 0x73, 0x4b, 0x65, 0x79, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x46,
    0x0a, 0x0d, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18,
    0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73,
    0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x53, 0x65,
    0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x0c, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x44,
    0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x43, 0x0a, 0x09, 0x73, 0x61, 0x76, 0x65, 0x5f, 0x67,
    0x61, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62,
    0x62, 0x79, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x2e, 0x53, 0x61, 0x76, 0x65, 0x47, 0x61, 0x6d,
    0x65, 0x52, 0x08, 0x73, 0x61, 0x76, 0x65, 0x47, 0x61, 0x6d, 0x65, 0x1a, 0x71, 0x0a, 0x08, 0x53,
    0x61, 0x76, 0x65, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x18, 0x0a, 0x07, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64,
    0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x22, 0xca,
    0x01, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c,
    0x6f, 0x62, 0x62, 0x79, 0x53, 0x65, 0x74, 0x54, 0x65, 0x61, 0x6d, 0x53, 0x6c, 0x6f, 0x74, 0x12,
    0x3e, 0x0a, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41,
    0x4d, 0x3a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f,
    0x47, 0x4f, 0x4f, 0x44, 0x5f, 0x47, 0x55, 0x59, 0x53, 0x52, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x12,
    0x12, 0x0a, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x73,
    0x6c, 0x6f, 0x74, 0x12, 0x56, 0x0a, 0x0e, 0x62, 0x6f, 0x74, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x69,
    0x63, 0x75, 0x6c, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x42, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63,
    0x75, 0x6c, 0x74, 0x79, 0x3a, 0x16, 0x42, 0x4f, 0x54, 0x5f, 0x44, 0x49, 0x46, 0x46, 0x49, 0x43,
    0x55, 0x4c, 0x54, 0x59, 0x5f, 0x50, 0x41, 0x53, 0x53, 0x49, 0x56, 0x45, 0x52, 0x0d, 0x62, 0x6f,
    0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x22, 0x5b, 0x0a, 0x19, 0x43,
    0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79,
    0x53, 0x65, 0x74, 0x43, 0x6f, 0x61, 0x63, 0x68, 0x12, 0x3e, 0x0a, 0x04, 0x74, 0x65, 0x61, 0x6d,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x3a, 0x16, 0x44, 0x4f, 0x54, 0x41,
    0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x47, 0x4f, 0x4f, 0x44, 0x5f, 0x47, 0x55,
    0x59, 0x53, 0x52, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x22, 0xe4, 0x01, 0x0a, 0x25, 0x43, 0x4d, 0x73,
    0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4a, 0x6f,
    0x69, 0x6e, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x33, 0x0a, 0x15,
    0x70, 0x72, 0x65, 0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x14, 0x70, 0x72, 0x65,
    0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x34, 0x0a, 0x16, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x5f, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x14, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x43, 0x6f, 0x75, 0x6e,
    0x74, 0x72, 0x79, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x36, 0x0a, 0x17, 0x70, 0x72, 0x65, 0x66, 0x65,
    0x72, 0x72, 0x65, 0x64, 0x5f, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f,
    0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x15, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72,
    0x72, 0x65, 0x64, 0x4c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x64, 0x65, 0x22,
    0x42, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c,
    0x6f, 0x62, 0x62, 0x79, 0x43, 0x6c, 0x6f, 0x73, 0x65, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61,
    0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x22, 0x38, 0x0a, 0x36, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74,
    0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x54, 0x6f, 0x67, 0x67, 0x6c, 0x65, 0x42, 0x72,
    0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x43, 0x61,
    0x6d, 0x65, 0x72, 0x61, 0x6d, 0x61, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x36, 0x0a,
    0x15, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62,
    0x62, 0x79, 0x4b, 0x69, 0x63, 0x6b, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x49, 0x64, 0x22, 0x3e, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61,
    0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4b, 0x69, 0x63, 0x6b, 0x46, 0x72,
    0x6f, 0x6d, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x49, 0x64, 0x22, 0x18, 0x0a, 0x16, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61,
    0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x65, 0x61, 0x76, 0x65, 0x22,
    0x40, 0x0a, 0x17, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c,
    0x6f, 0x62, 0x62, 0x79, 0x4c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x22, 0x37, 0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x41, 0x70, 0x70, 0x6c, 0x79, 0x54, 0x65,
    0x61, 0x6d, 0x54, 0x6f, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62,
    0x79, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x22, 0x1c, 0x0a, 0x1a, 0x43, 0x4d,
    0x73, 0x67, 0x43, 0x6c, 0x65, 0x61, 0x72, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c,
    0x6f, 0x62, 0x62, 0x79, 0x54, 0x65, 0x61, 0x6d, 0x22, 0xbb, 0x01, 0x0a, 0x15, 0x43, 0x4d, 0x73,
    0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69,
    0x73, 0x74, 0x12, 0x29, 0x0a, 0x10, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74,
    0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f, 0x74, 0x6f,
    0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x12, 0x19, 0x0a,
    0x08, 0x70, 0x61, 0x73, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x70, 0x61, 0x73, 0x73, 0x4b, 0x65, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x12, 0x44, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x47, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x3a, 0x12, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47,
    0x41, 0x4d, 0x45, 0x4d, 0x4f, 0x44, 0x45, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x52, 0x08, 0x67, 0x61,
    0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x22, 0xdc, 0x06, 0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x50,
    0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x14, 0x0a,
    0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x42, 0x04, 0x80,
    0xa6, 0x1d, 0x01, 0x12, 0x23, 0x0a, 0x0d, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x74, 0x6f, 0x75, 0x72,
    0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x12, 0x74, 0x6f, 0x75, 0x72,
    0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74,
    0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x4f, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x35, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79,
    0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x2e, 0x43, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07,
    0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x12, 0x2a, 0x0a, 0x11, 0x72, 0x65, 0x71, 0x75, 0x69,
    0x72, 0x65, 0x73, 0x5f, 0x70, 0x61, 0x73, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0f, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x50, 0x61, 0x73, 0x73,
    0x4b, 0x65, 0x79, 0x12, 0x2a, 0x0a, 0x11, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f,
    0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12,
    0x19, 0x0a, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x67, 0x75,
    0x69, 0x6c, 0x64, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x09, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09,
    0x67, 0x75, 0x69, 0x6c, 0x64, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x28, 0x0a,
    0x10, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64,
    0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47,
    0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x44, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f,
    0x6d, 0x6f, 0x64, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x3a,
    0x12, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x41, 0x4d, 0x45, 0x4d, 0x4f, 0x44, 0x45, 0x5f, 0x4e,
    0x4f, 0x4e, 0x45, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x25, 0x0a,
    0x0e, 0x66, 0x72, 0x69, 0x65, 0x6e, 0x64, 0x5f, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x66, 0x72, 0x69, 0x65, 0x6e, 0x64, 0x50, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x18,
    0x0e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x26,
    0x0a, 0x0f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x6d, 0x61, 0x70, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4d,
    0x61, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x6d, 0x61, 0x78, 0x5f, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0e, 0x6d, 0x61, 0x78, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e, 0x74,
    0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x41, 0x0a, 0x1e, 0x6c, 0x61, 0x6e, 0x5f, 0x68, 0x6f, 0x73,
    0x74, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x6f, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x19, 0x6c,
    0x61, 0x6e, 0x48, 0x6f, 0x73, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x54, 0x6f, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x65, 0x61, 0x67,
    0x75, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x6c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x49, 0x64, 0x1a, 0x4e, 0x0a, 0x0c, 0x43, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x8e, 0x01, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72,
    0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x29, 0x0a, 0x10, 0x74, 0x6f, 0x75, 0x72, 0x6e,
    0x61, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0f, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x47, 0x61, 0x6d,
    0x65, 0x73, 0x12, 0x42, 0x0a, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x50,
    0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x07, 0x6c,
    0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x22, 0x7d, 0x0a, 0x0d, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x6f,
    0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x26, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01,
    0x30, 0x52, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12,
    0x44, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47,
    0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x3a, 0x12, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x41,
    0x4d, 0x45, 0x4d, 0x4f, 0x44, 0x45, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x52, 0x08, 0x67, 0x61, 0x6d,
    0x65, 0x4d, 0x6f, 0x64, 0x65, 0x22, 0x5b, 0x0a, 0x15, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x6f, 0x62,
    0x62, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x42,
    0x0a, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74,
    0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x69,
    0x65, 0x73, 0x22, 0xd0, 0x01, 0x0a, 0x15, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74,
    0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4a, 0x6f, 0x69, 0x6e, 0x12, 0x19, 0x0a, 0x08,
    0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07,
    0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x19,
    0x0a, 0x08, 0x70, 0x61, 0x73, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x70, 0x61, 0x73, 0x73, 0x4b, 0x65, 0x79, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x06, 0x52, 0x0d, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x43, 0x72,
    0x63, 0x12, 0x32, 0x0a, 0x15, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65,
    0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x07,
    0x52, 0x13, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x54, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x6c, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61,
    0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4a, 0x6f, 0x69, 0x6e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4b, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f,
    0x54, 0x41, 0x4a, 0x6f, 0x69, 0x6e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x3a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4a, 0x4f, 0x49, 0x4e, 0x5f, 0x52, 0x45, 0x53,
    0x55, 0x4c, 0x54, 0x5f, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x22, 0x3e, 0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x46, 0x72, 0x69, 0x65, 0x6e,
    0x64, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69,
    0x73, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x66, 0x72, 0x69,
    0x65, 0x6e, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x07, 0x66, 0x72, 0x69, 0x65,
    0x6e, 0x64, 0x73, 0x22, 0x69, 0x0a, 0x23, 0x43, 0x4d, 0x73, 0x67, 0x46, 0x72, 0x69, 0x65, 0x6e,
    0x64, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69,
    0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x42, 0x0a, 0x07, 0x6c, 0x6f,
    0x62, 0x62, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c,
    0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x22, 0x3f,
    0x0a, 0x25, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x6d, 0x61, 0x74, 0x65, 0x50,
    0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x67, 0x75, 0x69, 0x6c, 0x64,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x06, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x73, 0x22,
    0x6c, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x6d, 0x61, 0x74, 0x65,
    0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x42, 0x0a, 0x07, 0x6c, 0x6f, 0x62,
    0x62, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72, 0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f,
    0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x22, 0x49, 0x0a,
    0x22, 0x43, 0x4d, 0x73, 0x67, 0x4a, 0x6f, 0x69, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x72, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x22, 0x94, 0x01, 0x0a, 0x28, 0x43, 0x4d, 0x73,
    0x67, 0x4a, 0x6f, 0x69, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47,
    0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x24, 0x0a, 0x0e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f,
    0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x63,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x6c,
    0x6f, 0x62, 0x62, 0x79, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0a, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x21, 0x0a, 0x0c,
    0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x22,
    0x74, 0x0a, 0x23, 0x43, 0x4d, 0x73, 0x67, 0x4a, 0x6f, 0x69, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x43,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4d, 0x0a, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d,
    0x6f, 0x64, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x4a, 0x6f, 0x69, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x75,
    0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x09, 0x67, 0x61, 0x6d, 0x65,
    0x4d, 0x6f, 0x64, 0x65, 0x73, 0x22, 0x6d, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x4a, 0x6f, 0x69,
    0x6e, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4c, 0x6f, 0x62, 0x62, 0x69,
    0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x24,
    0x0a, 0x0e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61,
    0x6d, 0x65, 0x49, 0x64, 0x22, 0xd4, 0x03, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67, 0x4a, 0x6f, 0x69,
    0x6e, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4c, 0x6f, 0x62, 0x62, 0x69,
    0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12,
    0x19, 0x0a, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x06, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x63, 0x75,
    0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64,
    0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x4e, 0x61, 0x6d, 0x65, 0x12,
    0x21, 0x0a, 0x0c, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x43, 0x6f, 0x75,
    0x6e, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x6c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1f,
    0x0a, 0x0b, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0a, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12,
    0x26, 0x0a, 0x0f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x6d, 0x61, 0x70, 0x5f, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x4d, 0x61, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x6d, 0x61, 0x78, 0x5f, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0e, 0x6d, 0x61, 0x78, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e,
    0x74, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x41, 0x0a, 0x1e, 0x6c, 0x61, 0x6e, 0x5f, 0x68, 0x6f,
    0x73, 0x74, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x6f, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x19,
    0x6c, 0x61, 0x6e, 0x48, 0x6f, 0x73, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x54, 0x6f, 0x53, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x20, 0x0a, 0x0c, 0x68, 0x61, 0x73,
    0x5f, 0x70, 0x61, 0x73, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x0a, 0x68, 0x61, 0x73, 0x50, 0x61, 0x73, 0x73, 0x4b, 0x65, 0x79, 0x22, 0x6b, 0x0a, 0x21, 0x43,
    0x4d, 0x73, 0x67, 0x4a, 0x6f, 0x69, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x4c, 0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x46, 0x0a, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x2c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x4a, 0x6f, 0x69,
    0x6e, 0x61, 0x62, 0x6c, 0x65, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4c, 0x6f, 0x62, 0x62, 0x69,
    0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52,
    0x07, 0x6c, 0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x22, 0x98, 0x04, 0x0a, 0x18, 0x43, 0x4d, 0x73,
    0x67, 0x51, 0x75, 0x69, 0x63, 0x6b, 0x4a, 0x6f, 0x69, 0x6e, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x4c, 0x6f, 0x62, 0x62, 0x79, 0x12, 0x30, 0x0a, 0x14, 0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x5f,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x12, 0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x53, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x0e, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x25, 0x0a,
    0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x12, 0x53, 0x0a, 0x14, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x5f, 0x6c,
    0x6f, 0x62, 0x62, 0x79, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x72,
    0x61, 0x63, 0x74, 0x69, 0x63, 0x65, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x53, 0x65, 0x74, 0x44, 0x65,
    0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x12, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x4c, 0x6f, 0x62,
    0x62, 0x79, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x22, 0x0a, 0x0d, 0x61, 0x6c, 0x6c,
    0x6f, 0x77, 0x5f, 0x61, 0x6e, 0x79, 0x5f, 0x6d, 0x61, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x0b, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x41, 0x6e, 0x79, 0x4d, 0x61, 0x70, 0x12, 0x5f, 0x0a,
    0x13, 0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x70,
    0x69, 0x6e, 0x67, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x75, 0x69, 0x63, 0x6b, 0x4a, 0x6f, 0x69, 0x6e, 0x43,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x2e, 0x4c, 0x65, 0x67, 0x61, 0x63,
    0x79, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x52, 0x11, 0x6c, 0x65, 0x67,
    0x61, 0x63, 0x79, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x73, 0x12, 0x35,
    0x0a, 0x09, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x44, 0x61, 0x74, 0x61, 0x52, 0x08, 0x70, 0x69, 0x6e,
    0x67, 0x44, 0x61, 0x74, 0x61, 0x1a, 0x6c, 0x0a, 0x10, 0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x52,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0c, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x12,
    0x0a, 0x04, 0x70, 0x69, 0x6e, 0x67, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x70, 0x69,
    0x6e, 0x67, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x64,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0a, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x43,
    0x6f, 0x64, 0x65, 0x22, 0x6f, 0x0a, 0x20, 0x43, 0x4d, 0x73, 0x67, 0x51, 0x75, 0x69, 0x63, 0x6b,
    0x4a, 0x6f, 0x69, 0x6e, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4b, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44,
    0x4f, 0x54, 0x41, 0x4a, 0x6f, 0x69, 0x6e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x3a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4a, 0x4f, 0x49, 0x4e, 0x5f, 0x52, 0x45,
    0x53, 0x55, 0x4c, 0x54, 0x5f, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53, 0x53, 0x52, 0x06, 0x72, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x22, 0xf0, 0x02, 0x0a, 0x11, 0x43, 0x4d, 0x73, 0x67, 0x42, 0x6f, 0x74,
    0x47, 0x61, 0x6d, 0x65, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65,
    0x61, 0x72, 0x63, 0x68, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
    0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4b, 0x65, 0x79, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x12, 0x5e, 0x0a, 0x12, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x5f, 0x72,
    0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x42, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69,
    0x63, 0x75, 0x6c, 0x74, 0x79, 0x3a, 0x16, 0x42, 0x4f, 0x54, 0x5f, 0x44, 0x49, 0x46, 0x46, 0x49,
    0x43, 0x55, 0x4c, 0x54, 0x59, 0x5f, 0x50, 0x41, 0x53, 0x53, 0x49, 0x56, 0x45, 0x52, 0x11, 0x64,
    0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x52, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74,
    0x12, 0x3e, 0x0a, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45,
    0x41, 0x4d, 0x3a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41, 0x4d,
    0x5f, 0x47, 0x4f, 0x4f, 0x44, 0x5f, 0x47, 0x55, 0x59, 0x53, 0x52, 0x04, 0x74, 0x65, 0x61, 0x6d,
    0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x58, 0x0a,
    0x0f, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x5f, 0x64, 0x69, 0x72, 0x65,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f,
    0x54, 0x41, 0x42, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x3a,
    0x16, 0x42, 0x4f, 0x54, 0x5f, 0x44, 0x49, 0x46, 0x46, 0x49, 0x43, 0x55, 0x4c, 0x54, 0x59, 0x5f,
    0x50, 0x41, 0x53, 0x53, 0x49, 0x56, 0x45, 0x52, 0x0e, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75,
    0x6c, 0x74, 0x79, 0x44, 0x69, 0x72, 0x65, 0x22, 0xd1, 0x01, 0x0a, 0x14, 0x43, 0x4d, 0x73, 0x67,
    0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4b, 0x65, 0x79, 0x12,
    0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x0a, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63,
    0x75, 0x6c, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x64, 0x69, 0x66, 0x66,
    0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d,
    0x6f, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x4d,
    0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6d, 0x61, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x6d, 0x61, 0x70, 0x12, 0x24, 0x0a, 0x0e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f,
    0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x63,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x22, 0xd0, 0x01, 0x0a, 0x13,
    0x43, 0x4d, 0x73, 0x67, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x6b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4b,
    0x65, 0x79, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1e, 0x0a, 0x0a, 0x64, 0x69, 0x66,
    0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x64,
    0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d,
    0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x67, 0x61,
    0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6d, 0x61, 0x70, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x03, 0x6d, 0x61, 0x70, 0x12, 0x24, 0x0a, 0x0e, 0x63, 0x75, 0x73, 0x74,
    0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x22, 0x3e,
    0x0a, 0x1b, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79, 0x4d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x53, 0x65, 0x74, 0x43, 0x6f, 0x61, 0x63, 0x68, 0x12, 0x1f, 0x0a,
    0x0b, 0x77, 0x61, 0x6e, 0x74, 0x73, 0x5f, 0x63, 0x6f, 0x61, 0x63, 0x68, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0a, 0x77, 0x61, 0x6e, 0x74, 0x73, 0x43, 0x6f, 0x61, 0x63, 0x68, 0x22, 0x46,
    0x0a, 0x16, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x53, 0x65, 0x74, 0x47, 0x72, 0x6f,
    0x75, 0x70, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x12, 0x6e, 0x65, 0x77, 0x5f,
    0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x10, 0x6e, 0x65, 0x77, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x53,
    0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x22, 0x72, 0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f,
    0x54, 0x41, 0x43, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x49, 0x6e, 0x76,
    0x69, 0x74, 0x65, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x5f,
    0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x06, 0x52, 0x0f,
    0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x73, 0x12,
    0x29, 0x0a, 0x10, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x64, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70,
    0x69, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x06, 0x52, 0x0f, 0x69, 0x6e, 0x76, 0x69, 0x74,
    0x65, 0x64, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x69, 0x64, 0x73, 0x22, 0x30, 0x0a, 0x1a, 0x43, 0x4d,
    0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x53, 0x65, 0x74, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4f, 0x70,
    0x65, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x6f, 0x70, 0x65, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x6f, 0x70, 0x65, 0x6e, 0x22, 0x40, 0x0a, 0x18,
    0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x72,
    0x67, 0x65, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x6f, 0x74, 0x68, 0x65,
    0x72, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06,
    0x52, 0x0c, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x49, 0x64, 0x22, 0x62,
    0x0a, 0x1a, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d,
    0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2c, 0x0a, 0x12,
    0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x74, 0x6f, 0x72, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x10, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61,
    0x74, 0x6f, 0x72, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x49, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x63,
    0x63, 0x65, 0x70, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x61, 0x63, 0x63, 0x65,
    0x70, 0x74, 0x22, 0x6a, 0x0a, 0x17, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x72,
    0x6f, 0x75, 0x70, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x4f, 0x0a,
    0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x44, 0x4f, 0x54, 0x41, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d,
    0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x1a, 0x6b, 0x5f, 0x45, 0x44,
    0x4f, 0x54, 0x41, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x72, 0x67, 0x65, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x5f, 0x4f, 0x4b, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x22, 0x8c,
    0x04, 0x0a, 0x1d, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72,
    0x4c, 0x6f, 0x62, 0x62, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73,
    0x12, 0x1a, 0x0a, 0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x08, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x12, 0x19, 0x0a, 0x08,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06,
    0x52, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12,
    0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x55, 0x72, 0x6c, 0x12, 0x1f,
    0x0a, 0x0b, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0a, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65, 0x12,
    0x1b, 0x0a, 0x09, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x08, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b,
    0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0a, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a,
    0x0b, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0a, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x4b,
    0x0a, 0x0c, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x47, 0x61,
    0x6d, 0x65, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x52, 0x0b,
    0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x45, 0x0a, 0x09, 0x64,
    0x69, 0x72, 0x65, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61,
    0x74, 0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x44, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x52, 0x08, 0x64, 0x69, 0x72, 0x65, 0x54, 0x65,
    0x61, 0x6d, 0x1a, 0x59, 0x0a, 0x04, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65,
    0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61,
    0x6d, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x74, 0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65,
    0x12, 0x1b, 0x0a, 0x09, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x08, 0x74, 0x65, 0x61, 0x6d, 0x4c, 0x6f, 0x67, 0x6f, 0x22, 0xbb, 0x01,
    0x0a, 0x1c, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x74, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74,
    0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x19,
    0x0a, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x6f, 0x62,
    0x62, 0x79, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6c,
    0x6f, 0x62, 0x62, 0x79, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x61, 0x73, 0x73,
    0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x61, 0x73, 0x73,
    0x4b, 0x65, 0x79, 0x12, 0x46, 0x0a, 0x0c, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x64, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c, 0x6f,
    0x62, 0x62, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x0b,
    0x67, 0x61, 0x6d, 0x65, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x22, 0x7f, 0x0a, 0x18, 0x43,
    0x4d, 0x73, 0x67, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74,
    0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x3c,
    0x0a, 0x07, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x22, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x74, 0x53, 0x70,
    0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x73, 0x52, 0x07, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x22, 0x18, 0x0a, 0x16,
    0x43, 0x4d, 0x73, 0x67, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c, 0x6f, 0x62,
    0x62, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x22, 0xfd, 0x02, 0x0a, 0x1e, 0x43, 0x4d, 0x73, 0x67, 0x53,
    0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4d, 0x0a, 0x07, 0x6c, 0x6f, 0x62,
    0x62, 0x69, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c,
    0x6f, 0x62, 0x62, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x2e, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x52,
    0x07, 0x6c, 0x6f, 0x62, 0x62, 0x69, 0x65, 0x73, 0x1a, 0x8b, 0x02, 0x0a, 0x0e, 0x53, 0x70, 0x65,
    0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x12, 0x19, 0x0a, 0x08, 0x6c,
    0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6c,
    0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x4e,
    0x61, 0x6d, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x5f,
    0x70, 0x61, 0x73, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f,
    0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x50, 0x61, 0x73, 0x73, 0x4b, 0x65, 0x79, 0x12,
    0x2a, 0x0a, 0x11, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x6c, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x6d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0b, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x46,
    0x0a, 0x0c, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x47, 0x61,
    0x6d, 0x65, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x0b, 0x67, 0x61, 0x6d, 0x65, 0x44,
    0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x22, 0x52, 0x0a, 0x28, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x53,
    0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d, 0x54, 0x69, 0x63, 0x6b,
    0x65, 0x74, 0x12, 0x26, 0x0a, 0x0f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x65,
    0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x22, 0x79, 0x0a, 0x30, 0x43, 0x4d,
    0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x44, 0x61, 0x74, 0x61, 0x67, 0x72, 0x61, 0x6d,
    0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2b,
    0x0a, 0x11, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x5f, 0x74, 0x69, 0x63,
    0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x73, 0x65, 0x72, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x54, 0x69, 0x63, 0x6b, 0x65, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x2a, 0xac, 0x0a, 0x0a, 0x18, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x12, 0x26, 0x0a, 0x22, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x5f, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x10, 0x00, 0x12, 0x21, 0x0a, 0x1d, 0x6b, 0x5f,
    0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x4f, 0x4b, 0x10, 0x01, 0x12, 0x2f, 0x0a,
    0x2b, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x41, 0x6c, 0x72, 0x65,
    0x61, 0x64, 0x79, 0x53, 0x65, 0x61, 0x72, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x10, 0x02, 0x12, 0x2a,
    0x0a, 0x26, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e,
    0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x46, 0x61, 0x69,
    0x6c, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x10, 0x64, 0x12, 0x2b, 0x0a, 0x27, 0x6b, 0x5f,
    0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x49,
    0x67, 0x6e, 0x6f, 0x72, 0x65, 0x10, 0x65, 0x12, 0x32, 0x0a, 0x2e, 0x6b, 0x5f, 0x45, 0x53, 0x74,
    0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x6d, 0x61, 0x6b, 0x69, 0x6e,
    0x67, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x10, 0x66, 0x12, 0x2c, 0x0a, 0x28, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x4f, 0x66, 0x66, 0x6c, 0x69, 0x6e, 0x65, 0x10, 0x67, 0x12, 0x32, 0x0a, 0x2e, 0x6b, 0x5f, 0x45,
    0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63,
    0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x6d, 0x61, 0x6b,
    0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6f, 0x6c, 0x64, 0x6f, 0x77, 0x6e, 0x10, 0x68, 0x12, 0x2e, 0x0a,
    0x2a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x4f, 0x75, 0x74, 0x4f, 0x66, 0x44, 0x61, 0x74, 0x65, 0x10, 0x69, 0x12, 0x37, 0x0a,
    0x33, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x43, 0x6f, 0x6d, 0x70,
    0x65, 0x74, 0x69, 0x74, 0x69, 0x76, 0x65, 0x4e, 0x6f, 0x4c, 0x6f, 0x77, 0x50, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x79, 0x10, 0x6a, 0x12, 0x35, 0x0a, 0x31, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61,
    0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x5f, 0x43, 0x6f, 0x6d, 0x70, 0x65, 0x74, 0x69, 0x74, 0x69, 0x76, 0x65,
    0x4e, 0x6f, 0x74, 0x55, 0x6e, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x10, 0x6b, 0x12, 0x32, 0x0a,
    0x2e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x47, 0x61, 0x6d, 0x65,
    0x4d, 0x6f, 0x64, 0x65, 0x4e, 0x6f, 0x74, 0x55, 0x6e, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x10,
    0x6c, 0x12, 0x3c, 0x0a, 0x38, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f,
    0x43, 0x6f, 0x6d, 0x70, 0x65, 0x74, 0x69, 0x74, 0x69, 0x76, 0x65, 0x4e, 0x6f, 0x74, 0x45, 0x6e,
    0x6f, 0x75, 0x67, 0x68, 0x53, 0x6b, 0x69, 0x6c, 0x6c, 0x44, 0x61, 0x74, 0x61, 0x10, 0x6d, 0x12,
    0x32, 0x0a, 0x2e, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69,
    0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x4d, 0x69,
    0x73, 0x73, 0x69, 0x6e, 0x67, 0x49, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x53, 0x6b, 0x69, 0x6c,
    0x6c, 0x10, 0x6e, 0x12, 0x3c, 0x0a, 0x38, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46,
    0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x5f, 0x43, 0x6f, 0x6d, 0x70, 0x65, 0x74, 0x69, 0x74, 0x69, 0x76, 0x65, 0x52, 0x61, 0x6e,
    0x6b, 0x53, 0x70, 0x72, 0x65, 0x61, 0x64, 0x54, 0x6f, 0x6f, 0x4c, 0x61, 0x72, 0x67, 0x65, 0x10,
    0x6f, 0x12, 0x33, 0x0a, 0x2f, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f,
    0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x41, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x49, 0x6e, 0x4c,
    0x6f, 0x62, 0x62, 0x79, 0x10, 0x70, 0x12, 0x33, 0x0a, 0x2f, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61,
    0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x5f, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x4e, 0x6f, 0x74, 0x56, 0x41,
    0x43, 0x56, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x64, 0x10, 0x71, 0x12, 0x39, 0x0a, 0x35, 0x6b,
    0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x57, 0x65, 0x65, 0x6b, 0x65, 0x6e,
    0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x42, 0x61, 0x64, 0x50, 0x61, 0x72, 0x74, 0x79,
    0x53, 0x69, 0x7a, 0x65, 0x10, 0x72, 0x12, 0x3e, 0x0a, 0x3a, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61,
    0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x5f, 0x57, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72,
    0x6e, 0x65, 0x79, 0x54, 0x65, 0x61, 0x6d, 0x42, 0x75, 0x79, 0x49, 0x6e, 0x54, 0x6f, 0x6f, 0x53,
    0x6d, 0x61, 0x6c, 0x6c, 0x10, 0x73, 0x12, 0x44, 0x0a, 0x40, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61,
    0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65,
    0x73, 0x75, 0x6c, 0x74, 0x5f, 0x57, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72,
    0x6e, 0x65, 0x79, 0x49, 0x6e, 0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x42, 0x75, 0x79,
    0x49, 0x6e, 0x54, 0x6f, 0x6f, 0x4c, 0x61, 0x72, 0x67, 0x65, 0x10, 0x74, 0x12, 0x3e, 0x0a, 0x3a,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x57, 0x65, 0x65, 0x6b, 0x65,
    0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x54, 0x65, 0x61, 0x6d, 0x42, 0x75, 0x79,
    0x49, 0x6e, 0x54, 0x6f, 0x6f, 0x4c, 0x61, 0x72, 0x67, 0x65, 0x10, 0x75, 0x12, 0x3a, 0x0a, 0x36,
    0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x4d, 0x69, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x4f, 0x77, 0x6e,
    0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x10, 0x76, 0x12, 0x38, 0x0a, 0x34, 0x6b, 0x5f, 0x45, 0x53,
    0x74, 0x61, 0x72, 0x74, 0x46, 0x69, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68,
    0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x5f, 0x57, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f,
    0x75, 0x72, 0x6e, 0x65, 0x79, 0x4e, 0x6f, 0x74, 0x55, 0x6e, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64,
    0x10, 0x77, 0x12, 0x40, 0x0a, 0x3c, 0x6b, 0x5f, 0x45, 0x53, 0x74, 0x61, 0x72, 0x74, 0x46, 0x69,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x5f, 0x57, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x52,
    0x65, 0x63, 0x65, 0x6e, 0x74, 0x50, 0x61, 0x72, 0x74, 0x69, 0x63, 0x69, 0x70, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x10, 0x78, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xd6, 0x9c, 0x01, 0x0a,
    0x07, 0x12, 0x05, 0x00, 0x00, 0xb8, 0x03, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05,
    0x07, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x07, 0x20, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x07, 0x07, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x08, 0x07, 0x36, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0a, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x0a, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0a, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x0a, 0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x0b, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x01, 0x03, 0x12, 0x03, 0x0b, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x0d,
    0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x05, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x0f, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x0f, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0f, 0x28,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x10, 0x08, 0x38, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x10, 0x36, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x11, 0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x11, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x11, 0x31, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x12, 0x08, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x12, 0x08, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x12, 0x32, 0x35, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x13, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x13, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02,
    0x12, 0x03, 0x13, 0x39, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x14,
    0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x14, 0x08, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x14, 0x33, 0x36, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x15, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x15, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x07, 0x02, 0x12, 0x03, 0x15, 0x39, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12,
    0x03, 0x16, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x16,
    0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x16, 0x35, 0x38,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x17, 0x08, 0x42, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x17, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x17, 0x3e, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x0a, 0x12, 0x03, 0x18, 0x08, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12,
    0x03, 0x18, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x18,
    0x3c, 0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x19, 0x08, 0x3d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x19, 0x08, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x19, 0x39, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x0c, 0x12, 0x03, 0x1a, 0x08, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c,
    0x01, 0x12, 0x03, 0x1a, 0x08, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12,
    0x03, 0x1a, 0x43, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x1b, 0x08,
    0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x36, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x1b, 0x39, 0x3c, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x1c, 0x08, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0e, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e,
    0x02, 0x12, 0x03, 0x1c, 0x43, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03,
    0x1d, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x1d, 0x08,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x1d, 0x3a, 0x3d, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x10, 0x12, 0x03, 0x1e, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x10, 0x02, 0x12, 0x03, 0x1e, 0x3a, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x11,
    0x12, 0x03, 0x1f, 0x08, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03,
    0x1f, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x11, 0x02, 0x12, 0x03, 0x1f, 0x40,
    0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x12, 0x12, 0x03, 0x20, 0x08, 0x49, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x20, 0x08, 0x42, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x12, 0x02, 0x12, 0x03, 0x20, 0x45, 0x48, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x13, 0x12, 0x03, 0x21, 0x08, 0x4f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x01,
    0x12, 0x03, 0x21, 0x08, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x13, 0x02, 0x12, 0x03,
    0x21, 0x4b, 0x4e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x14, 0x12, 0x03, 0x22, 0x08, 0x49,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x14, 0x01, 0x12, 0x03, 0x22, 0x08, 0x42, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x14, 0x02, 0x12, 0x03, 0x22, 0x45, 0x48, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x15, 0x12, 0x03, 0x23, 0x08, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x15, 0x01, 0x12, 0x03, 0x23, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x15, 0x02,
    0x12, 0x03, 0x23, 0x41, 0x44, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x16, 0x12, 0x03, 0x24,
    0x08, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x16, 0x01, 0x12, 0x03, 0x24, 0x08, 0x3c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x16, 0x02, 0x12, 0x03, 0x24, 0x3f, 0x42, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x17, 0x12, 0x03, 0x25, 0x08, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x17, 0x01, 0x12, 0x03, 0x25, 0x08, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x17, 0x02, 0x12, 0x03, 0x25, 0x47, 0x4a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x28,
    0x00, 0x36, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x28, 0x08, 0x1d, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x29, 0x18, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x29, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x08,
    0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x08, 0x12, 0x03, 0x2a, 0x28, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x07,
    0x12, 0x03, 0x2a, 0x33, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2b,
    0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2b, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x2c, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x2c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x18,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2c, 0x25, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x2c, 0x27, 0x3d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x07, 0x12, 0x03, 0x2c, 0x32, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x56, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x2d, 0x11, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2d,
    0x23, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2d, 0x34, 0x35,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x08, 0x12, 0x03, 0x2d, 0x36, 0x55, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x07, 0x12, 0x03, 0x2d, 0x41, 0x54, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x08, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x04, 0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06,
    0x12, 0x03, 0x2e, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x2e, 0x1b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2e, 0x28,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x08, 0x12, 0x03, 0x2e, 0x2a, 0x47, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x07, 0x12, 0x03, 0x2e, 0x35, 0x46, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x08, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x2f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x2f, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2f,
    0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x08, 0x12, 0x03, 0x2f, 0x2b, 0x41,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x07, 0x12, 0x03, 0x2f, 0x36, 0x40, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x30, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x30, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x30, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x30, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x31, 0x08, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x31, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x31, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x31, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09,
    0x12, 0x03, 0x32, 0x08, 0x5b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03,
    0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x06, 0x12, 0x03, 0x32, 0x11,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x32, 0x20, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x32, 0x35, 0x37, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x09, 0x08, 0x12, 0x03, 0x32, 0x38, 0x5a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x09, 0x07, 0x12, 0x03, 0x32, 0x43, 0x59, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0a, 0x12, 0x03, 0x33, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12,
    0x03, 0x33, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x33,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x33, 0x18, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x33, 0x2d, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x34, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x34, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0b, 0x06, 0x12, 0x03, 0x34, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01,
    0x12, 0x03, 0x34, 0x24, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03,
    0x34, 0x30, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x35, 0x08, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x35, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x35, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x35, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0c, 0x03, 0x12, 0x03, 0x35, 0x2e, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x38, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x38, 0x08, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x39, 0x08, 0x41, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x39, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x39, 0x18, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x39, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08, 0x12, 0x03,
    0x39, 0x33, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x07, 0x12, 0x03, 0x39, 0x3e,
    0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x64, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3a, 0x11, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x3a, 0x2a, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x3a, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x3a, 0x35, 0x63, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x07, 0x12, 0x03, 0x3a,
    0x40, 0x62, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x3b, 0x08, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x3b, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12,
    0x03, 0x3c, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3c,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3c, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3c, 0x18, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3c, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x3d, 0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x3d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x3d, 0x19, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x3d, 0x35,
    0x36, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x40, 0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x40, 0x08, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x43, 0x00, 0x49, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x43, 0x08,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x44, 0x08, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x44, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x44, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x44, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03,
    0x45, 0x08, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x45, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x45, 0x11, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x45, 0x1b, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x08, 0x12, 0x03, 0x45, 0x2a, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x07, 0x12, 0x03, 0x45, 0x35, 0x46, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12,
    0x03, 0x46, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x46,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x46, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x46, 0x18, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x46, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x47, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x47, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x47, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x47, 0x29,
    0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x48, 0x08, 0x50, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x48, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x04, 0x06, 0x12, 0x03, 0x48, 0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x48, 0x20, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x48, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x08, 0x12,
    0x03, 0x48, 0x2d, 0x4f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x07, 0x12, 0x03, 0x48,
    0x38, 0x4e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x4b, 0x00, 0x4f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x4c, 0x08, 0x5a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x4c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x4c, 0x11, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x25,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4c, 0x2d, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x08, 0x12, 0x03, 0x4c, 0x2f, 0x59, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x07, 0x12, 0x03, 0x4c, 0x3a, 0x58, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x4d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x4d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4d,
    0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4d, 0x28, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x4e, 0x08, 0x3d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4e, 0x11, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x4e, 0x2a, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x4e, 0x3b, 0x3c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x51, 0x00, 0x55,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x51, 0x08, 0x19, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x52, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x52, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x52, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52,
    0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x53, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x53, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x53, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x53, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x53, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12,
    0x03, 0x54, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x54,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x54, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x54, 0x18, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x54, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x57, 0x00, 0x58, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x57, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x05, 0x5a, 0x00, 0x86, 0x01,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x5b, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x5b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x5b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x5b, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5b,
    0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5c, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5c, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x5c, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12,
    0x03, 0x5d, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5d,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x5d, 0x11, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x23, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5d, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x5e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x5e, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x5e, 0x28,
    0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x03, 0x5f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x5f, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x5f, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12, 0x03,
    0x60, 0x08, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x03, 0x60, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x06, 0x12, 0x03, 0x60, 0x11, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x03, 0x60, 0x1e, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x05, 0x03, 0x12, 0x03, 0x60, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x05, 0x08, 0x12, 0x03, 0x60, 0x2a, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x05, 0x07, 0x12, 0x03, 0x60, 0x35, 0x43, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x06, 0x12,
    0x03, 0x61, 0x08, 0x61, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x04, 0x12, 0x03, 0x61,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x06, 0x12, 0x03, 0x61, 0x11, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x01, 0x12, 0x03, 0x61, 0x23, 0x39, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x03, 0x12, 0x03, 0x61, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x06, 0x08, 0x12, 0x03, 0x61, 0x3e, 0x60, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x06, 0x07, 0x12, 0x03, 0x61, 0x49, 0x5f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x07,
    0x12, 0x03, 0x62, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x04, 0x12, 0x03,
    0x62, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x05, 0x12, 0x03, 0x62, 0x11,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x01, 0x12, 0x03, 0x62, 0x16, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x03, 0x12, 0x03, 0x62, 0x25, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x08, 0x12, 0x03, 0x63, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x08, 0x04, 0x12, 0x03, 0x63, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08,
    0x05, 0x12, 0x03, 0x63, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x63, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x03, 0x12, 0x03, 0x63,
    0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x09, 0x12, 0x03, 0x64, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x09, 0x04, 0x12, 0x03, 0x64, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x09, 0x05, 0x12, 0x03, 0x64, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x09, 0x01, 0x12, 0x03, 0x64, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x09, 0x03, 0x12, 0x03, 0x64, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0a, 0x12,
    0x03, 0x65, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x65,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x65, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x65, 0x16, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x65, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x0b, 0x12, 0x03, 0x66, 0x08, 0x54, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x0b, 0x04, 0x12, 0x03, 0x66, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x06,
    0x12, 0x03, 0x66, 0x11, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x66, 0x21, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x66, 0x30,
    0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x08, 0x12, 0x03, 0x66, 0x33, 0x53, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x07, 0x12, 0x03, 0x66, 0x3e, 0x52, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x0c, 0x12, 0x03, 0x67, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x0c, 0x04, 0x12, 0x03, 0x67, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0c,
    0x05, 0x12, 0x03, 0x67, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0c, 0x01, 0x12,
    0x03, 0x67, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x67,
    0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0d, 0x12, 0x03, 0x68, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x68, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x68, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x68, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x0d, 0x03, 0x12, 0x03, 0x68, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0e, 0x12,
    0x03, 0x69, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x69,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x69, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x69, 0x18, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x69, 0x30, 0x32, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x0f, 0x12, 0x03, 0x6a, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x0f, 0x04, 0x12, 0x03, 0x6a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0f, 0x05,
    0x12, 0x03, 0x6a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0f, 0x01, 0x12, 0x03,
    0x6a, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x6a, 0x2d,
    0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x10, 0x12, 0x03, 0x6b, 0x08, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x10, 0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x10, 0x05, 0x12, 0x03, 0x6b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x10, 0x01, 0x12, 0x03, 0x6b, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x10,
    0x03, 0x12, 0x03, 0x6b, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x11, 0x12, 0x03,
    0x6c, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x11, 0x04, 0x12, 0x03, 0x6c, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x11, 0x05, 0x12, 0x03, 0x6c, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x11, 0x01, 0x12, 0x03, 0x6c, 0x18, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x11, 0x03, 0x12, 0x03, 0x6c, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x12, 0x12, 0x03, 0x6d, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x12,
    0x04, 0x12, 0x03, 0x6d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x12, 0x05, 0x12,
    0x03, 0x6d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x12, 0x01, 0x12, 0x03, 0x6d,
    0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x12, 0x03, 0x12, 0x03, 0x6d, 0x2e, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x13, 0x12, 0x03, 0x6e, 0x08, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x13, 0x04, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x13, 0x05, 0x12, 0x03, 0x6e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x13, 0x01, 0x12, 0x03, 0x6e, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x13, 0x03,
    0x12, 0x03, 0x6e, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x14, 0x12, 0x03, 0x6f,
    0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x14, 0x04, 0x12, 0x03, 0x6f, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x14, 0x05, 0x12, 0x03, 0x6f, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x14, 0x01, 0x12, 0x03, 0x6f, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x14, 0x03, 0x12, 0x03, 0x6f, 0x20, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x14, 0x08, 0x12, 0x03, 0x6f, 0x23, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x14,
    0x07, 0x12, 0x03, 0x6f, 0x2e, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x15, 0x12, 0x03,
    0x70, 0x08, 0x51, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x15, 0x04, 0x12, 0x03, 0x70, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x15, 0x06, 0x12, 0x03, 0x70, 0x11, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x15, 0x01, 0x12, 0x03, 0x70, 0x22, 0x2f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x15, 0x03, 0x12, 0x03, 0x70, 0x32, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x15, 0x08, 0x12, 0x03, 0x70, 0x35, 0x50, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x15, 0x07, 0x12, 0x03, 0x70, 0x40, 0x4f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x16, 0x12,
    0x03, 0x71, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x16, 0x04, 0x12, 0x03, 0x71,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x16, 0x05, 0x12, 0x03, 0x71, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x16, 0x01, 0x12, 0x03, 0x71, 0x16, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x16, 0x03, 0x12, 0x03, 0x71, 0x1c, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x17, 0x12, 0x03, 0x72, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x17, 0x04, 0x12, 0x03, 0x72, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x17, 0x05,
    0x12, 0x03, 0x72, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x17, 0x01, 0x12, 0x03,
    0x72, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x17, 0x03, 0x12, 0x03, 0x72, 0x2b,
    0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x18, 0x12, 0x03, 0x73, 0x08, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x18, 0x04, 0x12, 0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x18, 0x05, 0x12, 0x03, 0x73, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x18, 0x01, 0x12, 0x03, 0x73, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x18,
    0x03, 0x12, 0x03, 0x73, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x19, 0x12, 0x03,
    0x74, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x19, 0x04, 0x12, 0x03, 0x74, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x19, 0x05, 0x12, 0x03, 0x74, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x19, 0x01, 0x12, 0x03, 0x74, 0x18, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x19, 0x03, 0x12, 0x03, 0x74, 0x2c, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x1a, 0x12, 0x03, 0x75, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1a,
    0x04, 0x12, 0x03, 0x75, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1a, 0x05, 0x12,
    0x03, 0x75, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x75,
    0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1a, 0x03, 0x12, 0x03, 0x75, 0x29, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x1b, 0x12, 0x03, 0x76, 0x08, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x1b, 0x04, 0x12, 0x03, 0x76, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x1b, 0x05, 0x12, 0x03, 0x76, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x1b, 0x01, 0x12, 0x03, 0x76, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1b, 0x03,
    0x12, 0x03, 0x76, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x1c, 0x12, 0x03, 0x77,
    0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1c, 0x04, 0x12, 0x03, 0x77, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1c, 0x05, 0x12, 0x03, 0x77, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x1c, 0x01, 0x12, 0x03, 0x77, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x1c, 0x03, 0x12, 0x03, 0x77, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x1d, 0x12, 0x03, 0x78, 0x08, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1d, 0x04,
    0x12, 0x03, 0x78, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1d, 0x05, 0x12, 0x03,
    0x78, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1d, 0x01, 0x12, 0x03, 0x78, 0x18,
    0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1d, 0x03, 0x12, 0x03, 0x78, 0x39, 0x3b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x1e, 0x12, 0x03, 0x79, 0x08, 0x5c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x1e, 0x04, 0x12, 0x03, 0x79, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x1e, 0x06, 0x12, 0x03, 0x79, 0x11, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1e,
    0x01, 0x12, 0x03, 0x79, 0x25, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1e, 0x03, 0x12,
    0x03, 0x79, 0x32, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1e, 0x08, 0x12, 0x03, 0x79,
    0x35, 0x5b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1e, 0x07, 0x12, 0x03, 0x79, 0x40, 0x5a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x1f, 0x12, 0x03, 0x7a, 0x08, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x1f, 0x04, 0x12, 0x03, 0x7a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x1f, 0x05, 0x12, 0x03, 0x7a, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x1f, 0x01, 0x12, 0x03, 0x7a, 0x19, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x1f, 0x03,
    0x12, 0x03, 0x7a, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x20, 0x12, 0x03, 0x7b,
    0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x20, 0x04, 0x12, 0x03, 0x7b, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x20, 0x05, 0x12, 0x03, 0x7b, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x20, 0x01, 0x12, 0x03, 0x7b, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x20, 0x03, 0x12, 0x03, 0x7b, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x21, 0x12, 0x03, 0x7c, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x21, 0x04,
    0x12, 0x03, 0x7c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x21, 0x05, 0x12, 0x03,
    0x7c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x21, 0x01, 0x12, 0x03, 0x7c, 0x18,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x21, 0x03, 0x12, 0x03, 0x7c, 0x29, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x22, 0x12, 0x03, 0x7d, 0x08, 0x34, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x22, 0x04, 0x12, 0x03, 0x7d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x22, 0x05, 0x12, 0x03, 0x7d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x22,
    0x01, 0x12, 0x03, 0x7d, 0x19, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x22, 0x03, 0x12,
    0x03, 0x7d, 0x31, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x23, 0x12, 0x03, 0x7e, 0x08,
    0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x23, 0x04, 0x12, 0x03, 0x7e, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x23, 0x05, 0x12, 0x03, 0x7e, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x23, 0x01, 0x12, 0x03, 0x7e, 0x18, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x23, 0x03, 0x12, 0x03, 0x7e, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x24, 0x12, 0x03, 0x7f, 0x08, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x24, 0x04, 0x12,
    0x03, 0x7f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x24, 0x05, 0x12, 0x03, 0x7f,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x24, 0x01, 0x12, 0x03, 0x7f, 0x18, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x24, 0x03, 0x12, 0x03, 0x7f, 0x39, 0x3b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x25, 0x12, 0x04, 0x80, 0x01, 0x08, 0x63, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x25, 0x04, 0x12, 0x04, 0x80, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x25, 0x06, 0x12, 0x04, 0x80, 0x01, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x25, 0x01, 0x12, 0x04, 0x80, 0x01, 0x27, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x25, 0x03, 0x12, 0x04, 0x80, 0x01, 0x4a, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x25,
    0x08, 0x12, 0x04, 0x80, 0x01, 0x4d, 0x62, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x25, 0x07,
    0x12, 0x04, 0x80, 0x01, 0x58, 0x61, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x26, 0x12, 0x04,
    0x81, 0x01, 0x08, 0x67, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x26, 0x04, 0x12, 0x04, 0x81,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x26, 0x06, 0x12, 0x04, 0x81, 0x01,
    0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x26, 0x01, 0x12, 0x04, 0x81, 0x01, 0x27,
    0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x26, 0x03, 0x12, 0x04, 0x81, 0x01, 0x4e, 0x50,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x26, 0x08, 0x12, 0x04, 0x81, 0x01, 0x51, 0x66, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x26, 0x07, 0x12, 0x04, 0x81, 0x01, 0x5c, 0x65, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x27, 0x12, 0x04, 0x82, 0x01, 0x08, 0x66, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x27, 0x04, 0x12, 0x04, 0x82, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x27, 0x06, 0x12, 0x04, 0x82, 0x01, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x27, 0x01, 0x12, 0x04, 0x82, 0x01, 0x27, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x27, 0x03, 0x12, 0x04, 0x82, 0x01, 0x37, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x27,
    0x08, 0x12, 0x04, 0x82, 0x01, 0x3a, 0x65, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x27, 0x07,
    0x12, 0x04, 0x82, 0x01, 0x45, 0x64, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x28, 0x12, 0x04,
    0x83, 0x01, 0x08, 0x5f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x28, 0x04, 0x12, 0x04, 0x83,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x28, 0x06, 0x12, 0x04, 0x83, 0x01,
    0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x28, 0x01, 0x12, 0x04, 0x83, 0x01, 0x23,
    0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x28, 0x03, 0x12, 0x04, 0x83, 0x01, 0x39, 0x3b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x28, 0x08, 0x12, 0x04, 0x83, 0x01, 0x3c, 0x5e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x28, 0x07, 0x12, 0x04, 0x83, 0x01, 0x47, 0x5d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x29, 0x12, 0x04, 0x84, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x29, 0x04, 0x12, 0x04, 0x84, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x29, 0x05, 0x12, 0x04, 0x84, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x29, 0x01, 0x12, 0x04, 0x84, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x29, 0x03, 0x12, 0x04, 0x84, 0x01, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x2a,
    0x12, 0x04, 0x85, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x2a, 0x04, 0x12,
    0x04, 0x85, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x2a, 0x05, 0x12, 0x04,
    0x85, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x2a, 0x01, 0x12, 0x04, 0x85,
    0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x2a, 0x03, 0x12, 0x04, 0x85, 0x01,
    0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x88, 0x01, 0x00, 0x95, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0x88, 0x01, 0x08, 0x1f, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x08, 0x03, 0x00, 0x12, 0x06, 0x89, 0x01, 0x08, 0x8e, 0x01, 0x09, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x03, 0x00, 0x01, 0x12, 0x04, 0x89, 0x01, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x01, 0x10, 0x28, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8a, 0x01, 0x19, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x1f, 0x23, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x26, 0x27,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x10, 0x2b,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8b, 0x01,
    0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b,
    0x01, 0x1f, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x8b, 0x01, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04,
    0x8c, 0x01, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x8c, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x04, 0x8c, 0x01, 0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x8c, 0x01, 0x21, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x03, 0x00,
    0x02, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8d, 0x01, 0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08,
    0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x21, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x08, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0x90, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x90, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x90, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x90, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x90, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x04,
    0x91, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x91,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x04, 0x91, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0x91, 0x01, 0x18,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0x91, 0x01, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x04, 0x92, 0x01, 0x08, 0x2b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0x92, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x04, 0x92, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x04, 0x92, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x02, 0x03, 0x12, 0x04, 0x92, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x03, 0x12, 0x04, 0x93, 0x01, 0x08, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x93, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x06,
    0x12, 0x04, 0x93, 0x01, 0x11, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x93, 0x01, 0x2d, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x93, 0x01, 0x3d, 0x3e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x04, 0x94, 0x01,
    0x08, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x04, 0x94, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06, 0x12, 0x04, 0x94, 0x01, 0x11, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x04, 0x94, 0x01, 0x32, 0x3b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x04, 0x94, 0x01, 0x3e, 0x3f, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x09, 0x12, 0x06, 0x97, 0x01, 0x00, 0x9b, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x04, 0x97, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x04, 0x98, 0x01, 0x08, 0x4a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x98, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12,
    0x04, 0x98, 0x01, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x98, 0x01, 0x1e, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x98,
    0x01, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x08, 0x12, 0x04, 0x98, 0x01,
    0x27, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x07, 0x12, 0x04, 0x98, 0x01, 0x32,
    0x48, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x99, 0x01, 0x08, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x04, 0x99, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0x99, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x99, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x02, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x59, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x06, 0x12, 0x04, 0x9a, 0x01, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x9a, 0x01, 0x23, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x9a, 0x01, 0x34, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x08, 0x12, 0x04,
    0x9a, 0x01, 0x36, 0x58, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x07, 0x12, 0x04, 0x9a,
    0x01, 0x41, 0x57, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0x9d, 0x01, 0x00, 0x9f, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x4a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9e, 0x01, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x1e, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x08, 0x12, 0x04, 0x9e, 0x01, 0x27, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x07,
    0x12, 0x04, 0x9e, 0x01, 0x32, 0x48, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xa1, 0x01,
    0x00, 0xa6, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x08,
    0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xa3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xa3, 0x01, 0x18, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xa3, 0x01, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0xa4,
    0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa4, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x18, 0x2e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x31, 0x32, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x34, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x18, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12,
    0x06, 0xa8, 0x01, 0x00, 0xaa, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04,
    0xa8, 0x01, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xa9, 0x01,
    0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x18, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x22, 0x23, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0xac, 0x01, 0x00, 0xad, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0d, 0x01, 0x12, 0x04, 0xac, 0x01, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12,
    0x06, 0xaf, 0x01, 0x00, 0xb1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04,
    0xaf, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xb0, 0x01,
    0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x18, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xb3, 0x01, 0x00, 0xb5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x0f, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x00, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xb4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xb4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xb4, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb4,
    0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xb7, 0x01, 0x00, 0xb8, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x1e, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xba, 0x01, 0x00, 0xbc, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x11, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x00, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xbb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xbb, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xbb, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbb,
    0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xbe, 0x01, 0x00, 0xc0, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06,
    0xc2, 0x01, 0x00, 0xc3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xc2,
    0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xc5, 0x01, 0x00, 0xca, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x08, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x16, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01,
    0x12, 0x04, 0xc7, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xc7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xc7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc7,
    0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc7, 0x01,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xc8, 0x01, 0x08, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc8, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xc8, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x03, 0x06, 0x12, 0x04, 0xc9, 0x01, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xc9, 0x01, 0x1f, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xc9, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x08, 0x12,
    0x04, 0xc9, 0x01, 0x2d, 0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x07, 0x12, 0x04,
    0xc9, 0x01, 0x38, 0x4a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xcc, 0x01, 0x00, 0xe4,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x08, 0x2a, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x15, 0x03, 0x00, 0x12, 0x06, 0xcd, 0x01, 0x08, 0xd0, 0x01, 0x09, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x03, 0x00, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x10, 0x1c, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xce, 0x01, 0x10, 0x2f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xce, 0x01, 0x10, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xce, 0x01, 0x19, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xce, 0x01, 0x20,
    0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xce, 0x01,
    0x2d, 0x2e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xcf, 0x01,
    0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcf,
    0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xcf, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xcf, 0x01, 0x20, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x03, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xcf, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04,
    0xd2, 0x01, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd2,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd2, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x18,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x1d, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x08, 0x12, 0x04, 0xd2, 0x01, 0x1f, 0x33, 0x0a,
    0x10, 0x0a, 0x08, 0x04, 0x15, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xd2, 0x01, 0x20,
    0x32, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x15, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04,
    0xd2, 0x01, 0x20, 0x2b, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x15, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xd2, 0x01, 0x20, 0x2b, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x15, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x21, 0x2a, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x15, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x2e, 0x32,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x08, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x02, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xd4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xd4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xd4, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xd4, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0xd5, 0x01,
    0x08, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x06, 0x12, 0x04, 0xd5, 0x01, 0x11, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x41, 0x48, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x4b, 0x4c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd6, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x16, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x04, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x05,
    0x12, 0x04, 0xd7, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xd7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x05, 0x12, 0x04,
    0xd7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd7,
    0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd7, 0x01,
    0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x06, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x05, 0x12, 0x04, 0xd8, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x06, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x15, 0x02, 0x07, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x07, 0x04, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x07, 0x05, 0x12, 0x04, 0xd9, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07,
    0x01, 0x12, 0x04, 0xd9, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x03,
    0x12, 0x04, 0xd9, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x08, 0x12, 0x04,
    0xda, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x04, 0x12, 0x04, 0xda,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x05, 0x12, 0x04, 0xda, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x01, 0x12, 0x04, 0xda, 0x01, 0x18,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x03, 0x12, 0x04, 0xda, 0x01, 0x1f, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x09, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x2e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x09, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x09, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x09, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x0a, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a,
    0x04, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x06,
    0x12, 0x04, 0xdc, 0x01, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x01, 0x12,
    0x04, 0xdc, 0x01, 0x1f, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x03, 0x12, 0x04,
    0xdc, 0x01, 0x2b, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x08, 0x12, 0x04, 0xdc,
    0x01, 0x2e, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0a, 0x07, 0x12, 0x04, 0xdc, 0x01,
    0x39, 0x4b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x0b, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xdd, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0b, 0x05, 0x12, 0x04, 0xdd, 0x01, 0x11, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x16, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x15, 0x02, 0x0c, 0x12, 0x04, 0xde, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x0c, 0x04, 0x12, 0x04, 0xde, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x0c, 0x05, 0x12, 0x04, 0xde, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0c,
    0x01, 0x12, 0x04, 0xde, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0c, 0x03,
    0x12, 0x04, 0xde, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x0d, 0x12, 0x04,
    0xdf, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xdf,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0d, 0x05, 0x12, 0x04, 0xdf, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x18,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xdf, 0x01, 0x2a, 0x2c,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x0e, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x2e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x0e, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x0f, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0f,
    0x04, 0x12, 0x04, 0xe1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0f, 0x05,
    0x12, 0x04, 0xe1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0f, 0x01, 0x12,
    0x04, 0xe1, 0x01, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x0f, 0x03, 0x12, 0x04,
    0xe1, 0x01, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x10, 0x12, 0x04, 0xe2, 0x01,
    0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x10, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x10, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x10, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x18, 0x36, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x10, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x39, 0x3b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x11, 0x12, 0x04, 0xe3, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x11, 0x04, 0x12, 0x04, 0xe3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x11, 0x05, 0x12, 0x04, 0xe3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x11, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x11, 0x03, 0x12, 0x04, 0xe3, 0x01, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06,
    0xe6, 0x01, 0x00, 0xe9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xe6,
    0x01, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xe7, 0x01, 0x08,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe7, 0x01, 0x11, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x16, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xe8, 0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x34, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xe8, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xeb,
    0x01, 0x00, 0xee, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xeb, 0x01,
    0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xec, 0x01, 0x08, 0x38,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xec, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x05, 0x12, 0x04, 0xec, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xec, 0x01, 0x18, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xec, 0x01, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x08, 0x12, 0x04, 0xec, 0x01, 0x2a, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x00, 0x07, 0x12, 0x04, 0xec, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x01, 0x12, 0x04, 0xed, 0x01, 0x08, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xed, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x06,
    0x12, 0x04, 0xed, 0x01, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xed, 0x01, 0x1f, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xed, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x08, 0x12, 0x04, 0xed,
    0x01, 0x2d, 0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x07, 0x12, 0x04, 0xed, 0x01,
    0x38, 0x4a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xf0, 0x01, 0x00, 0xf2, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xf1, 0x01, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x34, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xf1, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xf4,
    0x01, 0x00, 0xfa, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xf4, 0x01,
    0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf5, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf5, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xf6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xf6, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xf6, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04,
    0xf7, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf7,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf7, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x18,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x23, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x2d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf8, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x19, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19,
    0x02, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x05,
    0x12, 0x04, 0xf9, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xf9, 0x01, 0x19, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xf9, 0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xfc, 0x01, 0x00, 0xfe,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x08, 0x25, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x55, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfd, 0x01, 0x11, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x25, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x00, 0x08, 0x12, 0x04, 0xfd, 0x01, 0x30, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00,
    0x07, 0x12, 0x04, 0xfd, 0x01, 0x3b, 0x53, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0x80,
    0x02, 0x00, 0x82, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0x80, 0x02,
    0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0x81, 0x02, 0x08, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x81, 0x02, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1c, 0x12, 0x06, 0x84, 0x02, 0x00, 0x86, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c,
    0x01, 0x12, 0x04, 0x84, 0x02, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12,
    0x04, 0x85, 0x02, 0x08, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x85, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x06, 0x12, 0x04, 0x85,
    0x02, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x85, 0x02,
    0x34, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x85, 0x02, 0x3e,
    0x3f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0x88, 0x02, 0x00, 0x8a, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0x88, 0x02, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0x89, 0x02, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x89, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x89, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x89, 0x02, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x89, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0x8c, 0x02,
    0x00, 0x8e, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x08,
    0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0x8d, 0x02, 0x08, 0x40, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8d, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x11, 0x33, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x34, 0x3b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x1f, 0x12, 0x06, 0x90, 0x02, 0x00, 0x92, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01,
    0x12, 0x04, 0x90, 0x02, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04,
    0x91, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0x91,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x91, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x91, 0x02, 0x18,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x91, 0x02, 0x28, 0x29,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0x94, 0x02, 0x00, 0x98, 0x02, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0x94, 0x02, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x20, 0x02, 0x00, 0x12, 0x04, 0x95, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x95, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x95, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x95, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x95, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04, 0x96,
    0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04, 0x96, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x05, 0x12, 0x04, 0x96, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0x96, 0x02, 0x18, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0x96, 0x02, 0x26, 0x27, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x02, 0x12, 0x04, 0x97, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x02, 0x04, 0x12, 0x04, 0x97, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x02, 0x05, 0x12, 0x04, 0x97, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x02, 0x01, 0x12, 0x04, 0x97, 0x02, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x97, 0x02, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12,
    0x06, 0x9a, 0x02, 0x00, 0x9c, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04,
    0x9a, 0x02, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0x9b, 0x02,
    0x08, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9b, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9b, 0x02, 0x11, 0x39,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x3a, 0x44, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9b, 0x02, 0x47, 0x48, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0x9e, 0x02, 0x00, 0xa1, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x22, 0x01, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02,
    0x00, 0x12, 0x04, 0x9f, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x9f, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x05, 0x12,
    0x04, 0x9f, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x9f, 0x02, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9f,
    0x02, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x08,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa0, 0x02, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x18, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa0, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x23, 0x12, 0x06, 0xa3, 0x02, 0x00, 0xaf, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x23, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x00,
    0x12, 0x04, 0xa4, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa4, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xa4, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4,
    0x02, 0x19, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x02,
    0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x08, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa5, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa5, 0x02, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x23, 0x02, 0x02, 0x12, 0x04, 0xa6, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xa6, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xa6, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xa6, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xa6, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x03, 0x12, 0x04,
    0xa7, 0x02, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa7,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa7, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x18,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa7, 0x02, 0x27, 0x28,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x02, 0x08, 0x2e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x23, 0x02, 0x04, 0x04, 0x12, 0x04, 0xa8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x04, 0x05, 0x12, 0x04, 0xa8, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23,
    0x02, 0x05, 0x12, 0x04, 0xa9, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x05,
    0x04, 0x12, 0x04, 0xa9, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x05, 0x05,
    0x12, 0x04, 0xa9, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xa9, 0x02, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xa9, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x06, 0x12, 0x04, 0xaa, 0x02,
    0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x06, 0x04, 0x12, 0x04, 0xaa, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x06, 0x05, 0x12, 0x04, 0xaa, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x06, 0x01, 0x12, 0x04, 0xaa, 0x02, 0x18, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x06, 0x03, 0x12, 0x04, 0xaa, 0x02, 0x2a, 0x2b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x23, 0x02, 0x07, 0x12, 0x04, 0xab, 0x02, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x07, 0x04, 0x12, 0x04, 0xab, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x07, 0x05, 0x12, 0x04, 0xab, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x07, 0x01, 0x12, 0x04, 0xab, 0x02, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x07, 0x03, 0x12, 0x04, 0xab, 0x02, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x08,
    0x12, 0x04, 0xac, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x08, 0x04, 0x12,
    0x04, 0xac, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x08, 0x05, 0x12, 0x04,
    0xac, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x08, 0x01, 0x12, 0x04, 0xac,
    0x02, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x08, 0x03, 0x12, 0x04, 0xac, 0x02,
    0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x09, 0x12, 0x04, 0xad, 0x02, 0x08, 0x3c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x09, 0x04, 0x12, 0x04, 0xad, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x09, 0x05, 0x12, 0x04, 0xad, 0x02, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x23, 0x02, 0x09, 0x01, 0x12, 0x04, 0xad, 0x02, 0x18, 0x36, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x09, 0x03, 0x12, 0x04, 0xad, 0x02, 0x39, 0x3b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x23, 0x02, 0x0a, 0x12, 0x04, 0xae, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x0a, 0x04, 0x12, 0x04, 0xae, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x0a, 0x05, 0x12, 0x04, 0xae, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x0a,
    0x01, 0x12, 0x04, 0xae, 0x02, 0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x0a, 0x03,
    0x12, 0x04, 0xae, 0x02, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xb1, 0x02,
    0x00, 0xb3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xb1, 0x02, 0x08,
    0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x00, 0x12, 0x04, 0xb2, 0x02, 0x08, 0x44, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb2, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb2, 0x02, 0x11, 0x37, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x24, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb2, 0x02, 0x38, 0x3f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x24, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb2, 0x02, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x25, 0x12, 0x06, 0xb5, 0x02, 0x00, 0xc3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x25, 0x01,
    0x12, 0x04, 0xb5, 0x02, 0x08, 0x20, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x25, 0x03, 0x00, 0x12, 0x06,
    0xb6, 0x02, 0x08, 0xba, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x03, 0x00, 0x01, 0x12,
    0x04, 0xb6, 0x02, 0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x25, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xb7, 0x02, 0x10, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x25, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xb7, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x25, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xb7, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x25, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x20, 0x2d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x25, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x02, 0x30, 0x31, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x25, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x25, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb8, 0x02, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x25,
    0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb8, 0x02, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x25, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x20, 0x24, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x25, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb8, 0x02, 0x27, 0x28, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x25, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xb9, 0x02, 0x10, 0x31, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x25, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb9, 0x02, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x25, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb9, 0x02, 0x19, 0x20, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x25, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x21, 0x2c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x25, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x2f,
    0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x02, 0x08, 0x31, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbc, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbc, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x02, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x02, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x25, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xbd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xbd, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xbd, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xbd, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x02, 0x12, 0x04, 0xbe,
    0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbe, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbe, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x02, 0x18, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbe, 0x02, 0x29, 0x2a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x03, 0x12, 0x04, 0xbf, 0x02, 0x08, 0x46, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbf, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x03, 0x06, 0x12, 0x04, 0xbf, 0x02, 0x11, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbf, 0x02, 0x2d, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xbf, 0x02, 0x44, 0x45, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02,
    0x04, 0x12, 0x04, 0xc0, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x04,
    0x12, 0x04, 0xc0, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x05, 0x12,
    0x04, 0xc0, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xc0, 0x02, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x04, 0x03, 0x12, 0x04, 0xc0,
    0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x05, 0x12, 0x04, 0xc1, 0x02, 0x08,
    0x53, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x05, 0x04, 0x12, 0x04, 0xc1, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x05, 0x06, 0x12, 0x04, 0xc1, 0x02, 0x11, 0x3a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x05, 0x01, 0x12, 0x04, 0xc1, 0x02, 0x3b, 0x4e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x05, 0x03, 0x12, 0x04, 0xc1, 0x02, 0x51, 0x52, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x25, 0x02, 0x06, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x06, 0x04, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x06, 0x06, 0x12, 0x04, 0xc2, 0x02, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x06, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x24, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x06,
    0x03, 0x12, 0x04, 0xc2, 0x02, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0xc5,
    0x02, 0x00, 0xc7, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0xc5, 0x02,
    0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x02, 0x08, 0x55,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc6, 0x02, 0x11, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x02, 0x25, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc6, 0x02, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x00, 0x08, 0x12, 0x04, 0xc6, 0x02, 0x30, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x00, 0x07, 0x12, 0x04, 0xc6, 0x02, 0x3b, 0x53, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27,
    0x12, 0x06, 0xc9, 0x02, 0x00, 0xd0, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12,
    0x04, 0xc9, 0x02, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0xca,
    0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0xca, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x05, 0x12, 0x04, 0xca, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0xca, 0x02, 0x18, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0xca, 0x02, 0x25, 0x26, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01, 0x12, 0x04, 0xcb, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcb, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x01, 0x05, 0x12, 0x04, 0xcb, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcb, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xcb, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02,
    0x02, 0x12, 0x04, 0xcc, 0x02, 0x08, 0x5d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x04,
    0x12, 0x04, 0xcc, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x06, 0x12,
    0x04, 0xcc, 0x02, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x01, 0x12, 0x04,
    0xcc, 0x02, 0x23, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x03, 0x12, 0x04, 0xcc,
    0x02, 0x38, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x08, 0x12, 0x04, 0xcc, 0x02,
    0x3a, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x07, 0x12, 0x04, 0xcc, 0x02, 0x45,
    0x5b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x03, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x4a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03, 0x04, 0x12, 0x04, 0xcd, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x03, 0x06, 0x12, 0x04, 0xcd, 0x02, 0x11, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x03, 0x01, 0x12, 0x04, 0xcd, 0x02, 0x1e, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x03, 0x03, 0x12, 0x04, 0xcd, 0x02, 0x25, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x03, 0x08, 0x12, 0x04, 0xcd, 0x02, 0x27, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x03, 0x07, 0x12, 0x04, 0xcd, 0x02, 0x32, 0x48, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02,
    0x04, 0x12, 0x04, 0xce, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x04, 0x04,
    0x12, 0x04, 0xce, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x04, 0x05, 0x12,
    0x04, 0xce, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xce, 0x02, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x04, 0x03, 0x12, 0x04, 0xce,
    0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x05, 0x12, 0x04, 0xcf, 0x02, 0x08,
    0x5a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x04, 0x12, 0x04, 0xcf, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x06, 0x12, 0x04, 0xcf, 0x02, 0x11, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x01, 0x12, 0x04, 0xcf, 0x02, 0x23, 0x32, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x03, 0x12, 0x04, 0xcf, 0x02, 0x35, 0x36, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x05, 0x08, 0x12, 0x04, 0xcf, 0x02, 0x37, 0x59, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x05, 0x07, 0x12, 0x04, 0xcf, 0x02, 0x42, 0x58, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x28, 0x12, 0x06, 0xd2, 0x02, 0x00, 0xd9, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01,
    0x12, 0x04, 0xd2, 0x02, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04,
    0xd3, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd3,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd3, 0x02,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd3, 0x02, 0x18,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd3, 0x02, 0x25, 0x26,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x01, 0x12, 0x04, 0xd4, 0x02, 0x08, 0x2b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd4, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd4, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd4, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd4, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28,
    0x02, 0x02, 0x12, 0x04, 0xd5, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xd5, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xd5, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xd5, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xd5, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x03, 0x12, 0x04, 0xd6, 0x02,
    0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd6, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x05, 0x12, 0x04, 0xd6, 0x02, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd6, 0x02, 0x18, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd6, 0x02, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x28, 0x02, 0x04, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd7, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x18, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x04, 0x03, 0x12, 0x04, 0xd7, 0x02, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x05,
    0x12, 0x04, 0xd8, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x05, 0x04, 0x12,
    0x04, 0xd8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x05, 0x05, 0x12, 0x04,
    0xd8, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd8,
    0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd8, 0x02,
    0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x29, 0x12, 0x06, 0xdb, 0x02, 0x00, 0xe2, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29, 0x01, 0x12, 0x04, 0xdb, 0x02, 0x08, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x29, 0x02, 0x00, 0x12, 0x04, 0xdc, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdc, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xdc, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xdc, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x01, 0x12,
    0x04, 0xdd, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xdd, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x05, 0x12, 0x04, 0xdd,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdd, 0x02,
    0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdd, 0x02, 0x29,
    0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x02, 0x12, 0x04, 0xde, 0x02, 0x08, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x04, 0x12, 0x04, 0xde, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x05, 0x12, 0x04, 0xde, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x02, 0x01, 0x12, 0x04, 0xde, 0x02, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x02, 0x03, 0x12, 0x04, 0xde, 0x02, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x29, 0x02, 0x03, 0x12, 0x04, 0xdf, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x03, 0x04, 0x12, 0x04, 0xdf, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03,
    0x05, 0x12, 0x04, 0xdf, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xdf, 0x02, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x03, 0x03, 0x12,
    0x04, 0xdf, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x04, 0x12, 0x04, 0xe0,
    0x02, 0x08, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x04, 0x12, 0x04, 0xe0, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x05, 0x12, 0x04, 0xe0, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x01, 0x12, 0x04, 0xe0, 0x02, 0x18, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x03, 0x12, 0x04, 0xe0, 0x02, 0x1e, 0x1f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x05, 0x12, 0x04, 0xe1, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x05, 0x04, 0x12, 0x04, 0xe1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x05, 0x05, 0x12, 0x04, 0xe1, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x05, 0x01, 0x12, 0x04, 0xe1, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x05, 0x03, 0x12, 0x04, 0xe1, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2a, 0x12,
    0x06, 0xe4, 0x02, 0x00, 0xe6, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01, 0x12, 0x04,
    0xe4, 0x02, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x00, 0x12, 0x04, 0xe5, 0x02,
    0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe5, 0x02, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe5, 0x02, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe5, 0x02, 0x16, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe5, 0x02, 0x24, 0x25, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06, 0xe8, 0x02, 0x00, 0xea, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x2b, 0x01, 0x12, 0x04, 0xe8, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02,
    0x00, 0x12, 0x04, 0xe9, 0x02, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xe9, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xe9, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xe9, 0x02, 0x19, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe9,
    0x02, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xec, 0x02, 0x00, 0xef, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0xec, 0x02, 0x08, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x2c, 0x02, 0x00, 0x12, 0x04, 0xed, 0x02, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xed, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2c, 0x02, 0x00, 0x05, 0x12, 0x04, 0xed, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xed, 0x02, 0x19, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xed, 0x02, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x01,
    0x12, 0x04, 0xee, 0x02, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xee, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xee, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee,
    0x02, 0x19, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x01, 0x03, 0x12, 0x04, 0xee, 0x02,
    0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2d, 0x12, 0x06, 0xf1, 0x02, 0x00, 0xf3, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2d, 0x01, 0x12, 0x04, 0xf1, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x2d, 0x02, 0x00, 0x12, 0x04, 0xf2, 0x02, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xf2, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xf2, 0x02, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xf2, 0x02, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2e, 0x12, 0x06, 0xf5,
    0x02, 0x00, 0xf7, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12, 0x04, 0xf5, 0x02,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xf6, 0x02, 0x08, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf6, 0x02, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf6, 0x02, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf6, 0x02, 0x19, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf6, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x2f, 0x12, 0x06, 0xf9, 0x02, 0x00, 0xfc, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2f,
    0x01, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02, 0x00, 0x12,
    0x04, 0xfa, 0x02, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xfa, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xfa,
    0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfa, 0x02,
    0x19, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfa, 0x02, 0x2e,
    0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2f, 0x02, 0x01, 0x12, 0x04, 0xfb, 0x02, 0x08, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfb, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xfb, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfb, 0x02, 0x16, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfb, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x30, 0x12, 0x06, 0xfe, 0x02, 0x00, 0x80, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x30, 0x01,
    0x12, 0x04, 0xfe, 0x02, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x30, 0x02, 0x00, 0x12, 0x04,
    0xff, 0x02, 0x08, 0x59, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x04, 0x12, 0x04, 0xff,
    0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x06, 0x12, 0x04, 0xff, 0x02,
    0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x01, 0x12, 0x04, 0xff, 0x02, 0x27,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x03, 0x12, 0x04, 0xff, 0x02, 0x30, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x08, 0x12, 0x04, 0xff, 0x02, 0x32, 0x58, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x30, 0x02, 0x00, 0x07, 0x12, 0x04, 0xff, 0x02, 0x3d, 0x57, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x31, 0x12, 0x06, 0x82, 0x03, 0x00, 0x93, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x31, 0x01, 0x12, 0x04, 0x82, 0x03, 0x08, 0x25, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x31, 0x03,
    0x00, 0x12, 0x06, 0x83, 0x03, 0x08, 0x87, 0x03, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x03,
    0x00, 0x01, 0x12, 0x04, 0x83, 0x03, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x31, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x04, 0x84, 0x03, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0x84, 0x03, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x84, 0x03, 0x20, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x31, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x84, 0x03, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x31, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x85, 0x03, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x31, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x85, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x31, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x85, 0x03, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x31, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x85, 0x03, 0x20, 0x29, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x31, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x85, 0x03, 0x2c, 0x2d,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x31, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0x86, 0x03, 0x10, 0x2f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x86, 0x03, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0x86, 0x03,
    0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x86,
    0x03, 0x21, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x31, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x86, 0x03, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x00, 0x12, 0x04, 0x89, 0x03,
    0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x04, 0x12, 0x04, 0x89, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x05, 0x12, 0x04, 0x89, 0x03, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x01, 0x12, 0x04, 0x89, 0x03, 0x18, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x00, 0x03, 0x12, 0x04, 0x89, 0x03, 0x23, 0x24, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x31, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x03, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8a, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8a, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x8a, 0x03, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x8a, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x02,
    0x12, 0x04, 0x8b, 0x03, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x8b, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x05, 0x12, 0x04,
    0x8b, 0x03, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8b,
    0x03, 0x19, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8b, 0x03,
    0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x03, 0x12, 0x04, 0x8c, 0x03, 0x08, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8c, 0x03, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x03, 0x05, 0x12, 0x04, 0x8c, 0x03, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x31, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8c, 0x03, 0x18, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x31, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8c, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x31, 0x02, 0x04, 0x12, 0x04, 0x8d, 0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x8d, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x8d, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x8d, 0x03, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x8d, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x05, 0x12, 0x04,
    0x8e, 0x03, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x05, 0x04, 0x12, 0x04, 0x8e,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x05, 0x05, 0x12, 0x04, 0x8e, 0x03,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x05, 0x01, 0x12, 0x04, 0x8e, 0x03, 0x18,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x05, 0x03, 0x12, 0x04, 0x8e, 0x03, 0x24, 0x25,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x06, 0x12, 0x04, 0x8f, 0x03, 0x08, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x31, 0x02, 0x06, 0x04, 0x12, 0x04, 0x8f, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x31, 0x02, 0x06, 0x05, 0x12, 0x04, 0x8f, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x06, 0x01, 0x12, 0x04, 0x8f, 0x03, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x06, 0x03, 0x12, 0x04, 0x8f, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31,
    0x02, 0x07, 0x12, 0x04, 0x90, 0x03, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x07,
    0x04, 0x12, 0x04, 0x90, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x07, 0x05,
    0x12, 0x04, 0x90, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x07, 0x01, 0x12,
    0x04, 0x90, 0x03, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x07, 0x03, 0x12, 0x04,
    0x90, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x31, 0x02, 0x08, 0x12, 0x04, 0x91, 0x03,
    0x08, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x08, 0x04, 0x12, 0x04, 0x91, 0x03, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x08, 0x06, 0x12, 0x04, 0x91, 0x03, 0x11, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x08, 0x01, 0x12, 0x04, 0x91, 0x03, 0x34, 0x40, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02, 0x08, 0x03, 0x12, 0x04, 0x91, 0x03, 0x43, 0x44, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x31, 0x02, 0x09, 0x12, 0x04, 0x92, 0x03, 0x08, 0x43, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x31, 0x02, 0x09, 0x04, 0x12, 0x04, 0x92, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x31, 0x02, 0x09, 0x06, 0x12, 0x04, 0x92, 0x03, 0x11, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31,
    0x02, 0x09, 0x01, 0x12, 0x04, 0x92, 0x03, 0x34, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x31, 0x02,
    0x09, 0x03, 0x12, 0x04, 0x92, 0x03, 0x40, 0x42, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x32, 0x12, 0x06,
    0x95, 0x03, 0x00, 0x9a, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x32, 0x01, 0x12, 0x04, 0x95,
    0x03, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x00, 0x12, 0x04, 0x96, 0x03, 0x08,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x04, 0x12, 0x04, 0x96, 0x03, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x05, 0x12, 0x04, 0x96, 0x03, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x01, 0x12, 0x04, 0x96, 0x03, 0x18, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x32, 0x02, 0x00, 0x03, 0x12, 0x04, 0x96, 0x03, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x32, 0x02, 0x01, 0x12, 0x04, 0x97, 0x03, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x32, 0x02, 0x01, 0x04, 0x12, 0x04, 0x97, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32,
    0x02, 0x01, 0x05, 0x12, 0x04, 0x97, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x97, 0x03, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x97, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x02, 0x12,
    0x04, 0x98, 0x03, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x98, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x05, 0x12, 0x04, 0x98,
    0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x01, 0x12, 0x04, 0x98, 0x03,
    0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x02, 0x03, 0x12, 0x04, 0x98, 0x03, 0x23,
    0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x32, 0x02, 0x03, 0x12, 0x04, 0x99, 0x03, 0x08, 0x40, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x04, 0x12, 0x04, 0x99, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x32, 0x02, 0x03, 0x06, 0x12, 0x04, 0x99, 0x03, 0x11, 0x2e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x32, 0x02, 0x03, 0x01, 0x12, 0x04, 0x99, 0x03, 0x2f, 0x3b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x32, 0x02, 0x03, 0x03, 0x12, 0x04, 0x99, 0x03, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x33, 0x12, 0x06, 0x9c, 0x03, 0x00, 0x9f, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x33, 0x01,
    0x12, 0x04, 0x9c, 0x03, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x00, 0x12, 0x04,
    0x9d, 0x03, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d,
    0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9d, 0x03,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d, 0x03, 0x18,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x33, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x03, 0x29, 0x2a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x33, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x03, 0x08, 0x3a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x33, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9e, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x33, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9e, 0x03, 0x11, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x33, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x03, 0x2e, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x33, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x03, 0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x34,
    0x12, 0x06, 0xa1, 0x03, 0x00, 0xa2, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x34, 0x01, 0x12,
    0x04, 0xa1, 0x03, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x35, 0x12, 0x06, 0xa4, 0x03, 0x00,
    0xaf, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x35, 0x01, 0x12, 0x04, 0xa4, 0x03, 0x08, 0x26,
    0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x35, 0x03, 0x00, 0x12, 0x06, 0xa5, 0x03, 0x08, 0xac, 0x03, 0x09,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x03, 0x00, 0x01, 0x12, 0x04, 0xa5, 0x03, 0x10, 0x1e, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x35, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x03, 0x10, 0x2d, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa6, 0x03, 0x10, 0x18,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6, 0x03, 0x19,
    0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x03,
    0x20, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6,
    0x03, 0x2b, 0x2c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xa7,
    0x03, 0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xa7, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xa7, 0x03, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xa7, 0x03, 0x20, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xa7, 0x03, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x03, 0x00, 0x02,
    0x02, 0x12, 0x04, 0xa8, 0x03, 0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xa8, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x04, 0xa8, 0x03, 0x19, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x03, 0x1e, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35,
    0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa8, 0x03, 0x32, 0x33, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x35, 0x03, 0x00, 0x02, 0x03, 0x12, 0x04, 0xa9, 0x03, 0x10, 0x36, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x35, 0x03, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa9, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x35, 0x03, 0x00, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa9, 0x03, 0x19, 0x1f, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa9, 0x03, 0x20, 0x31, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa9, 0x03, 0x34, 0x35, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x35, 0x03, 0x00, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x03, 0x10, 0x31, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0xaa, 0x03, 0x10, 0x18,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x04, 0x05, 0x12, 0x04, 0xaa, 0x03, 0x19,
    0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xaa, 0x03,
    0x20, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x04, 0x03, 0x12, 0x04, 0xaa,
    0x03, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x35, 0x03, 0x00, 0x02, 0x05, 0x12, 0x04, 0xab,
    0x03, 0x10, 0x48, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04,
    0xab, 0x03, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x05, 0x06, 0x12,
    0x04, 0xab, 0x03, 0x19, 0x36, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x05, 0x01,
    0x12, 0x04, 0xab, 0x03, 0x37, 0x43, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x35, 0x03, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x04, 0xab, 0x03, 0x46, 0x47, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x35, 0x02, 0x00, 0x12,
    0x04, 0xae, 0x03, 0x08, 0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xae, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x06, 0x12, 0x04, 0xae,
    0x03, 0x11, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x01, 0x12, 0x04, 0xae, 0x03,
    0x3f, 0x46, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x35, 0x02, 0x00, 0x03, 0x12, 0x04, 0xae, 0x03, 0x49,
    0x4a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x36, 0x12, 0x06, 0xb1, 0x03, 0x00, 0xb3, 0x03, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x36, 0x01, 0x12, 0x04, 0xb1, 0x03, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x36, 0x02, 0x00, 0x12, 0x04, 0xb2, 0x03, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xb2, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xb2, 0x03, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xb2, 0x03, 0x19, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x36, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xb2, 0x03, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x37, 0x12, 0x06, 0xb5, 0x03,
    0x00, 0xb8, 0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x37, 0x01, 0x12, 0x04, 0xb5, 0x03, 0x08,
    0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x37, 0x02, 0x00, 0x12, 0x04, 0xb6, 0x03, 0x08, 0x2e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb6, 0x03, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x37, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb6, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x37, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb6, 0x03, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x37, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb6, 0x03, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x37, 0x02, 0x01, 0x12, 0x04, 0xb7, 0x03, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xb7, 0x03, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xb7, 0x03, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb7, 0x03, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x37, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xb7, 0x03, 0x22, 0x23,
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
