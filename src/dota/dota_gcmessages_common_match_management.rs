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
pub struct CSODOTAPartyMember {
    // message fields
    partner_type: ::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType>,
    is_coach: ::std::option::Option<bool>,
    region_ping_codes: ::std::vec::Vec<u32>,
    region_ping_times: ::std::vec::Vec<u32>,
    region_ping_failed_bitmask: ::std::option::Option<u32>,
    tourney_skill_level: ::std::option::Option<u32>,
    tourney_buyin: ::std::option::Option<u32>,
    tourney_prevent_until: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSODOTAPartyMember {}

impl CSODOTAPartyMember {
    pub fn new() -> CSODOTAPartyMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSODOTAPartyMember {
        static mut instance: ::protobuf::lazy::Lazy<CSODOTAPartyMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSODOTAPartyMember,
        };
        unsafe {
            instance.get(CSODOTAPartyMember::new)
        }
    }

    // optional .dota.PartnerAccountType partner_type = 1;

    pub fn clear_partner_type(&mut self) {
        self.partner_type = ::std::option::Option::None;
    }

    pub fn has_partner_type(&self) -> bool {
        self.partner_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partner_type(&mut self, v: super::gcsdk_gcmessages::PartnerAccountType) {
        self.partner_type = ::std::option::Option::Some(v);
    }

    pub fn get_partner_type(&self) -> super::gcsdk_gcmessages::PartnerAccountType {
        self.partner_type.unwrap_or(super::gcsdk_gcmessages::PartnerAccountType::PARTNER_NONE)
    }

    fn get_partner_type_for_reflect(&self) -> &::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType> {
        &self.partner_type
    }

    fn mut_partner_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType> {
        &mut self.partner_type
    }

    // optional bool is_coach = 2;

    pub fn clear_is_coach(&mut self) {
        self.is_coach = ::std::option::Option::None;
    }

    pub fn has_is_coach(&self) -> bool {
        self.is_coach.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_coach(&mut self, v: bool) {
        self.is_coach = ::std::option::Option::Some(v);
    }

    pub fn get_is_coach(&self) -> bool {
        self.is_coach.unwrap_or(false)
    }

    fn get_is_coach_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_coach
    }

    fn mut_is_coach_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_coach
    }

    // repeated uint32 region_ping_codes = 4;

    pub fn clear_region_ping_codes(&mut self) {
        self.region_ping_codes.clear();
    }

    // Param is passed by value, moved
    pub fn set_region_ping_codes(&mut self, v: ::std::vec::Vec<u32>) {
        self.region_ping_codes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_region_ping_codes(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.region_ping_codes
    }

    // Take field
    pub fn take_region_ping_codes(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.region_ping_codes, ::std::vec::Vec::new())
    }

    pub fn get_region_ping_codes(&self) -> &[u32] {
        &self.region_ping_codes
    }

    fn get_region_ping_codes_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.region_ping_codes
    }

    fn mut_region_ping_codes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.region_ping_codes
    }

    // repeated uint32 region_ping_times = 5;

    pub fn clear_region_ping_times(&mut self) {
        self.region_ping_times.clear();
    }

    // Param is passed by value, moved
    pub fn set_region_ping_times(&mut self, v: ::std::vec::Vec<u32>) {
        self.region_ping_times = v;
    }

    // Mutable pointer to the field.
    pub fn mut_region_ping_times(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.region_ping_times
    }

    // Take field
    pub fn take_region_ping_times(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.region_ping_times, ::std::vec::Vec::new())
    }

    pub fn get_region_ping_times(&self) -> &[u32] {
        &self.region_ping_times
    }

    fn get_region_ping_times_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.region_ping_times
    }

    fn mut_region_ping_times_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.region_ping_times
    }

    // optional uint32 region_ping_failed_bitmask = 6;

    pub fn clear_region_ping_failed_bitmask(&mut self) {
        self.region_ping_failed_bitmask = ::std::option::Option::None;
    }

    pub fn has_region_ping_failed_bitmask(&self) -> bool {
        self.region_ping_failed_bitmask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_ping_failed_bitmask(&mut self, v: u32) {
        self.region_ping_failed_bitmask = ::std::option::Option::Some(v);
    }

    pub fn get_region_ping_failed_bitmask(&self) -> u32 {
        self.region_ping_failed_bitmask.unwrap_or(0)
    }

    fn get_region_ping_failed_bitmask_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.region_ping_failed_bitmask
    }

    fn mut_region_ping_failed_bitmask_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.region_ping_failed_bitmask
    }

    // optional uint32 tourney_skill_level = 7;

    pub fn clear_tourney_skill_level(&mut self) {
        self.tourney_skill_level = ::std::option::Option::None;
    }

    pub fn has_tourney_skill_level(&self) -> bool {
        self.tourney_skill_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_skill_level(&mut self, v: u32) {
        self.tourney_skill_level = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_skill_level(&self) -> u32 {
        self.tourney_skill_level.unwrap_or(0)
    }

    fn get_tourney_skill_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tourney_skill_level
    }

    fn mut_tourney_skill_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tourney_skill_level
    }

    // optional uint32 tourney_buyin = 8;

    pub fn clear_tourney_buyin(&mut self) {
        self.tourney_buyin = ::std::option::Option::None;
    }

    pub fn has_tourney_buyin(&self) -> bool {
        self.tourney_buyin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_buyin(&mut self, v: u32) {
        self.tourney_buyin = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_buyin(&self) -> u32 {
        self.tourney_buyin.unwrap_or(0)
    }

    fn get_tourney_buyin_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tourney_buyin
    }

    fn mut_tourney_buyin_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tourney_buyin
    }

    // optional uint32 tourney_prevent_until = 9;

    pub fn clear_tourney_prevent_until(&mut self) {
        self.tourney_prevent_until = ::std::option::Option::None;
    }

    pub fn has_tourney_prevent_until(&self) -> bool {
        self.tourney_prevent_until.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_prevent_until(&mut self, v: u32) {
        self.tourney_prevent_until = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_prevent_until(&self) -> u32 {
        self.tourney_prevent_until.unwrap_or(0)
    }

    fn get_tourney_prevent_until_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tourney_prevent_until
    }

    fn mut_tourney_prevent_until_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tourney_prevent_until
    }
}

impl ::protobuf::Message for CSODOTAPartyMember {
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
                    self.partner_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_coach = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.region_ping_codes)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.region_ping_times)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.region_ping_failed_bitmask = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tourney_skill_level = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tourney_buyin = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tourney_prevent_until = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.partner_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.is_coach {
            my_size += 2;
        };
        if !self.region_ping_codes.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, &self.region_ping_codes);
        };
        if !self.region_ping_times.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(5, &self.region_ping_times);
        };
        if let Some(v) = self.region_ping_failed_bitmask {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_skill_level {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_buyin {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_prevent_until {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.partner_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.is_coach {
            os.write_bool(2, v)?;
        };
        if !self.region_ping_codes.is_empty() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.region_ping_codes))?;
            for v in &self.region_ping_codes {
                os.write_uint32_no_tag(*v)?;
            };
        };
        if !self.region_ping_times.is_empty() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.region_ping_times))?;
            for v in &self.region_ping_times {
                os.write_uint32_no_tag(*v)?;
            };
        };
        if let Some(v) = self.region_ping_failed_bitmask {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.tourney_skill_level {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.tourney_buyin {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.tourney_prevent_until {
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

impl ::protobuf::MessageStatic for CSODOTAPartyMember {
    fn new() -> CSODOTAPartyMember {
        CSODOTAPartyMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSODOTAPartyMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::gcsdk_gcmessages::PartnerAccountType>>(
                    "partner_type",
                    CSODOTAPartyMember::get_partner_type_for_reflect,
                    CSODOTAPartyMember::mut_partner_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_coach",
                    CSODOTAPartyMember::get_is_coach_for_reflect,
                    CSODOTAPartyMember::mut_is_coach_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region_ping_codes",
                    CSODOTAPartyMember::get_region_ping_codes_for_reflect,
                    CSODOTAPartyMember::mut_region_ping_codes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region_ping_times",
                    CSODOTAPartyMember::get_region_ping_times_for_reflect,
                    CSODOTAPartyMember::mut_region_ping_times_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region_ping_failed_bitmask",
                    CSODOTAPartyMember::get_region_ping_failed_bitmask_for_reflect,
                    CSODOTAPartyMember::mut_region_ping_failed_bitmask_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tourney_skill_level",
                    CSODOTAPartyMember::get_tourney_skill_level_for_reflect,
                    CSODOTAPartyMember::mut_tourney_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tourney_buyin",
                    CSODOTAPartyMember::get_tourney_buyin_for_reflect,
                    CSODOTAPartyMember::mut_tourney_buyin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tourney_prevent_until",
                    CSODOTAPartyMember::get_tourney_prevent_until_for_reflect,
                    CSODOTAPartyMember::mut_tourney_prevent_until_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSODOTAPartyMember>(
                    "CSODOTAPartyMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSODOTAPartyMember {
    fn clear(&mut self) {
        self.clear_partner_type();
        self.clear_is_coach();
        self.clear_region_ping_codes();
        self.clear_region_ping_times();
        self.clear_region_ping_failed_bitmask();
        self.clear_tourney_skill_level();
        self.clear_tourney_buyin();
        self.clear_tourney_prevent_until();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSODOTAPartyMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSODOTAPartyMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSODOTAParty {
    // message fields
    party_id: ::std::option::Option<u64>,
    leader_id: ::std::option::Option<u64>,
    member_ids: ::std::vec::Vec<u64>,
    game_modes: ::std::option::Option<u32>,
    state: ::std::option::Option<CSODOTAParty_State>,
    effective_started_matchmaking_time: ::std::option::Option<u32>,
    raw_started_matchmaking_time: ::std::option::Option<u32>,
    attempt_start_time: ::std::option::Option<u32>,
    attempt_num: ::std::option::Option<u32>,
    matchgroups: ::std::option::Option<u32>,
    low_priority_account_id: ::std::option::Option<u32>,
    match_type: ::std::option::Option<super::dota_shared_enums::MatchType>,
    bot_difficulty: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    team_id: ::std::option::Option<u32>,
    team_name: ::protobuf::SingularField<::std::string::String>,
    team_ui_logo: ::std::option::Option<u64>,
    team_base_logo: ::std::option::Option<u64>,
    match_disabled_until_date: ::std::option::Option<u32>,
    match_disabled_account_id: ::std::option::Option<u32>,
    matchmaking_max_range_minutes: ::std::option::Option<u32>,
    matchlanguages: ::std::option::Option<u32>,
    map_preference: ::std::option::Option<u32>,
    members: ::protobuf::RepeatedField<CSODOTAPartyMember>,
    open_guild_id: ::std::option::Option<u32>,
    common_guilds: ::std::vec::Vec<u32>,
    low_priority_games_remaining: ::std::option::Option<u32>,
    active_ingame_events: ::std::vec::Vec<super::dota_shared_enums::EEvent>,
    open_for_join_requests: ::std::option::Option<bool>,
    sent_invites: ::protobuf::RepeatedField<CSODOTAPartyInvite>,
    recv_invites: ::protobuf::RepeatedField<CSODOTAPartyInvite>,
    account_flags: ::std::option::Option<u32>,
    region_select_flags: ::std::option::Option<u32>,
    exclusive_tournament_id: ::std::option::Option<u32>,
    tourney_division_id: ::std::option::Option<u32>,
    tourney_schedule_time: ::std::option::Option<u32>,
    tourney_skill_level: ::std::option::Option<u32>,
    tourney_bracket_round: ::std::option::Option<u32>,
    tourney_queue_deadline_time: ::std::option::Option<u32>,
    tourney_queue_deadline_state: ::std::option::Option<super::dota_shared_enums::ETourneyQueueDeadlineState>,
    party_builder_slots_to_fill: ::std::option::Option<u32>,
    party_builder_match_groups: ::std::option::Option<u32>,
    party_builder_start_time: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSODOTAParty {}

impl CSODOTAParty {
    pub fn new() -> CSODOTAParty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSODOTAParty {
        static mut instance: ::protobuf::lazy::Lazy<CSODOTAParty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSODOTAParty,
        };
        unsafe {
            instance.get(CSODOTAParty::new)
        }
    }

    // optional uint64 party_id = 1;

    pub fn clear_party_id(&mut self) {
        self.party_id = ::std::option::Option::None;
    }

    pub fn has_party_id(&self) -> bool {
        self.party_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_id(&mut self, v: u64) {
        self.party_id = ::std::option::Option::Some(v);
    }

    pub fn get_party_id(&self) -> u64 {
        self.party_id.unwrap_or(0)
    }

    fn get_party_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.party_id
    }

    fn mut_party_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.party_id
    }

    // optional fixed64 leader_id = 2;

    pub fn clear_leader_id(&mut self) {
        self.leader_id = ::std::option::Option::None;
    }

    pub fn has_leader_id(&self) -> bool {
        self.leader_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_id(&mut self, v: u64) {
        self.leader_id = ::std::option::Option::Some(v);
    }

    pub fn get_leader_id(&self) -> u64 {
        self.leader_id.unwrap_or(0)
    }

    fn get_leader_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.leader_id
    }

    fn mut_leader_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.leader_id
    }

    // repeated fixed64 member_ids = 3;

    pub fn clear_member_ids(&mut self) {
        self.member_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_member_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.member_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_member_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.member_ids
    }

    // Take field
    pub fn take_member_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.member_ids, ::std::vec::Vec::new())
    }

    pub fn get_member_ids(&self) -> &[u64] {
        &self.member_ids
    }

    fn get_member_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.member_ids
    }

    fn mut_member_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.member_ids
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
        self.game_modes.unwrap_or(0)
    }

    fn get_game_modes_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_modes
    }

    fn mut_game_modes_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_modes
    }

    // optional .dota.CSODOTAParty.State state = 6;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: CSODOTAParty_State) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> CSODOTAParty_State {
        self.state.unwrap_or(CSODOTAParty_State::UI)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<CSODOTAParty_State> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<CSODOTAParty_State> {
        &mut self.state
    }

    // optional uint32 effective_started_matchmaking_time = 7;

    pub fn clear_effective_started_matchmaking_time(&mut self) {
        self.effective_started_matchmaking_time = ::std::option::Option::None;
    }

    pub fn has_effective_started_matchmaking_time(&self) -> bool {
        self.effective_started_matchmaking_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effective_started_matchmaking_time(&mut self, v: u32) {
        self.effective_started_matchmaking_time = ::std::option::Option::Some(v);
    }

    pub fn get_effective_started_matchmaking_time(&self) -> u32 {
        self.effective_started_matchmaking_time.unwrap_or(0)
    }

    fn get_effective_started_matchmaking_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.effective_started_matchmaking_time
    }

    fn mut_effective_started_matchmaking_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.effective_started_matchmaking_time
    }

    // optional uint32 raw_started_matchmaking_time = 32;

    pub fn clear_raw_started_matchmaking_time(&mut self) {
        self.raw_started_matchmaking_time = ::std::option::Option::None;
    }

    pub fn has_raw_started_matchmaking_time(&self) -> bool {
        self.raw_started_matchmaking_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_started_matchmaking_time(&mut self, v: u32) {
        self.raw_started_matchmaking_time = ::std::option::Option::Some(v);
    }

    pub fn get_raw_started_matchmaking_time(&self) -> u32 {
        self.raw_started_matchmaking_time.unwrap_or(0)
    }

    fn get_raw_started_matchmaking_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.raw_started_matchmaking_time
    }

    fn mut_raw_started_matchmaking_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.raw_started_matchmaking_time
    }

    // optional uint32 attempt_start_time = 33;

    pub fn clear_attempt_start_time(&mut self) {
        self.attempt_start_time = ::std::option::Option::None;
    }

    pub fn has_attempt_start_time(&self) -> bool {
        self.attempt_start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attempt_start_time(&mut self, v: u32) {
        self.attempt_start_time = ::std::option::Option::Some(v);
    }

    pub fn get_attempt_start_time(&self) -> u32 {
        self.attempt_start_time.unwrap_or(0)
    }

    fn get_attempt_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attempt_start_time
    }

    fn mut_attempt_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attempt_start_time
    }

    // optional uint32 attempt_num = 34;

    pub fn clear_attempt_num(&mut self) {
        self.attempt_num = ::std::option::Option::None;
    }

    pub fn has_attempt_num(&self) -> bool {
        self.attempt_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attempt_num(&mut self, v: u32) {
        self.attempt_num = ::std::option::Option::Some(v);
    }

    pub fn get_attempt_num(&self) -> u32 {
        self.attempt_num.unwrap_or(0)
    }

    fn get_attempt_num_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attempt_num
    }

    fn mut_attempt_num_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attempt_num
    }

    // optional uint32 matchgroups = 11;

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

    // optional uint32 low_priority_account_id = 19;

    pub fn clear_low_priority_account_id(&mut self) {
        self.low_priority_account_id = ::std::option::Option::None;
    }

    pub fn has_low_priority_account_id(&self) -> bool {
        self.low_priority_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_priority_account_id(&mut self, v: u32) {
        self.low_priority_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_low_priority_account_id(&self) -> u32 {
        self.low_priority_account_id.unwrap_or(0)
    }

    fn get_low_priority_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.low_priority_account_id
    }

    fn mut_low_priority_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.low_priority_account_id
    }

    // optional .dota.MatchType match_type = 21;

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

    // optional .dota.DOTABotDifficulty bot_difficulty = 22;

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

    // optional uint32 team_id = 23;

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

    // optional string team_name = 51;

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

    // optional uint64 team_ui_logo = 52;

    pub fn clear_team_ui_logo(&mut self) {
        self.team_ui_logo = ::std::option::Option::None;
    }

    pub fn has_team_ui_logo(&self) -> bool {
        self.team_ui_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_ui_logo(&mut self, v: u64) {
        self.team_ui_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team_ui_logo(&self) -> u64 {
        self.team_ui_logo.unwrap_or(0)
    }

    fn get_team_ui_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_ui_logo
    }

    fn mut_team_ui_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_ui_logo
    }

    // optional uint64 team_base_logo = 53;

    pub fn clear_team_base_logo(&mut self) {
        self.team_base_logo = ::std::option::Option::None;
    }

    pub fn has_team_base_logo(&self) -> bool {
        self.team_base_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_base_logo(&mut self, v: u64) {
        self.team_base_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team_base_logo(&self) -> u64 {
        self.team_base_logo.unwrap_or(0)
    }

    fn get_team_base_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_base_logo
    }

    fn mut_team_base_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_base_logo
    }

    // optional uint32 match_disabled_until_date = 24;

    pub fn clear_match_disabled_until_date(&mut self) {
        self.match_disabled_until_date = ::std::option::Option::None;
    }

    pub fn has_match_disabled_until_date(&self) -> bool {
        self.match_disabled_until_date.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_disabled_until_date(&mut self, v: u32) {
        self.match_disabled_until_date = ::std::option::Option::Some(v);
    }

    pub fn get_match_disabled_until_date(&self) -> u32 {
        self.match_disabled_until_date.unwrap_or(0)
    }

    fn get_match_disabled_until_date_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.match_disabled_until_date
    }

    fn mut_match_disabled_until_date_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.match_disabled_until_date
    }

    // optional uint32 match_disabled_account_id = 25;

    pub fn clear_match_disabled_account_id(&mut self) {
        self.match_disabled_account_id = ::std::option::Option::None;
    }

    pub fn has_match_disabled_account_id(&self) -> bool {
        self.match_disabled_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_disabled_account_id(&mut self, v: u32) {
        self.match_disabled_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_match_disabled_account_id(&self) -> u32 {
        self.match_disabled_account_id.unwrap_or(0)
    }

    fn get_match_disabled_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.match_disabled_account_id
    }

    fn mut_match_disabled_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.match_disabled_account_id
    }

    // optional uint32 matchmaking_max_range_minutes = 26;

    pub fn clear_matchmaking_max_range_minutes(&mut self) {
        self.matchmaking_max_range_minutes = ::std::option::Option::None;
    }

    pub fn has_matchmaking_max_range_minutes(&self) -> bool {
        self.matchmaking_max_range_minutes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_matchmaking_max_range_minutes(&mut self, v: u32) {
        self.matchmaking_max_range_minutes = ::std::option::Option::Some(v);
    }

    pub fn get_matchmaking_max_range_minutes(&self) -> u32 {
        self.matchmaking_max_range_minutes.unwrap_or(0)
    }

    fn get_matchmaking_max_range_minutes_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.matchmaking_max_range_minutes
    }

    fn mut_matchmaking_max_range_minutes_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.matchmaking_max_range_minutes
    }

    // optional uint32 matchlanguages = 27;

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
        self.matchlanguages.unwrap_or(0)
    }

    fn get_matchlanguages_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.matchlanguages
    }

    fn mut_matchlanguages_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.matchlanguages
    }

    // optional uint32 map_preference = 38;

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

    // repeated .dota.CSODOTAPartyMember members = 29;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CSODOTAPartyMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTAPartyMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CSODOTAPartyMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CSODOTAPartyMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CSODOTAPartyMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTAPartyMember> {
        &mut self.members
    }

    // optional uint32 open_guild_id = 30;

    pub fn clear_open_guild_id(&mut self) {
        self.open_guild_id = ::std::option::Option::None;
    }

    pub fn has_open_guild_id(&self) -> bool {
        self.open_guild_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_open_guild_id(&mut self, v: u32) {
        self.open_guild_id = ::std::option::Option::Some(v);
    }

    pub fn get_open_guild_id(&self) -> u32 {
        self.open_guild_id.unwrap_or(0)
    }

    fn get_open_guild_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.open_guild_id
    }

    fn mut_open_guild_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.open_guild_id
    }

    // repeated uint32 common_guilds = 31;

    pub fn clear_common_guilds(&mut self) {
        self.common_guilds.clear();
    }

    // Param is passed by value, moved
    pub fn set_common_guilds(&mut self, v: ::std::vec::Vec<u32>) {
        self.common_guilds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_common_guilds(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.common_guilds
    }

    // Take field
    pub fn take_common_guilds(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.common_guilds, ::std::vec::Vec::new())
    }

    pub fn get_common_guilds(&self) -> &[u32] {
        &self.common_guilds
    }

    fn get_common_guilds_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.common_guilds
    }

    fn mut_common_guilds_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.common_guilds
    }

    // optional uint32 low_priority_games_remaining = 35;

    pub fn clear_low_priority_games_remaining(&mut self) {
        self.low_priority_games_remaining = ::std::option::Option::None;
    }

    pub fn has_low_priority_games_remaining(&self) -> bool {
        self.low_priority_games_remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_priority_games_remaining(&mut self, v: u32) {
        self.low_priority_games_remaining = ::std::option::Option::Some(v);
    }

    pub fn get_low_priority_games_remaining(&self) -> u32 {
        self.low_priority_games_remaining.unwrap_or(0)
    }

    fn get_low_priority_games_remaining_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.low_priority_games_remaining
    }

    fn mut_low_priority_games_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.low_priority_games_remaining
    }

    // repeated .dota.EEvent active_ingame_events = 39;

    pub fn clear_active_ingame_events(&mut self) {
        self.active_ingame_events.clear();
    }

    // Param is passed by value, moved
    pub fn set_active_ingame_events(&mut self, v: ::std::vec::Vec<super::dota_shared_enums::EEvent>) {
        self.active_ingame_events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_active_ingame_events(&mut self) -> &mut ::std::vec::Vec<super::dota_shared_enums::EEvent> {
        &mut self.active_ingame_events
    }

    // Take field
    pub fn take_active_ingame_events(&mut self) -> ::std::vec::Vec<super::dota_shared_enums::EEvent> {
        ::std::mem::replace(&mut self.active_ingame_events, ::std::vec::Vec::new())
    }

    pub fn get_active_ingame_events(&self) -> &[super::dota_shared_enums::EEvent] {
        &self.active_ingame_events
    }

    fn get_active_ingame_events_for_reflect(&self) -> &::std::vec::Vec<super::dota_shared_enums::EEvent> {
        &self.active_ingame_events
    }

    fn mut_active_ingame_events_for_reflect(&mut self) -> &mut ::std::vec::Vec<super::dota_shared_enums::EEvent> {
        &mut self.active_ingame_events
    }

    // optional bool open_for_join_requests = 40;

    pub fn clear_open_for_join_requests(&mut self) {
        self.open_for_join_requests = ::std::option::Option::None;
    }

    pub fn has_open_for_join_requests(&self) -> bool {
        self.open_for_join_requests.is_some()
    }

    // Param is passed by value, moved
    pub fn set_open_for_join_requests(&mut self, v: bool) {
        self.open_for_join_requests = ::std::option::Option::Some(v);
    }

    pub fn get_open_for_join_requests(&self) -> bool {
        self.open_for_join_requests.unwrap_or(false)
    }

    fn get_open_for_join_requests_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.open_for_join_requests
    }

    fn mut_open_for_join_requests_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.open_for_join_requests
    }

    // repeated .dota.CSODOTAPartyInvite sent_invites = 41;

    pub fn clear_sent_invites(&mut self) {
        self.sent_invites.clear();
    }

    // Param is passed by value, moved
    pub fn set_sent_invites(&mut self, v: ::protobuf::RepeatedField<CSODOTAPartyInvite>) {
        self.sent_invites = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sent_invites(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTAPartyInvite> {
        &mut self.sent_invites
    }

    // Take field
    pub fn take_sent_invites(&mut self) -> ::protobuf::RepeatedField<CSODOTAPartyInvite> {
        ::std::mem::replace(&mut self.sent_invites, ::protobuf::RepeatedField::new())
    }

    pub fn get_sent_invites(&self) -> &[CSODOTAPartyInvite] {
        &self.sent_invites
    }

    fn get_sent_invites_for_reflect(&self) -> &::protobuf::RepeatedField<CSODOTAPartyInvite> {
        &self.sent_invites
    }

    fn mut_sent_invites_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTAPartyInvite> {
        &mut self.sent_invites
    }

    // repeated .dota.CSODOTAPartyInvite recv_invites = 42;

    pub fn clear_recv_invites(&mut self) {
        self.recv_invites.clear();
    }

    // Param is passed by value, moved
    pub fn set_recv_invites(&mut self, v: ::protobuf::RepeatedField<CSODOTAPartyInvite>) {
        self.recv_invites = v;
    }

    // Mutable pointer to the field.
    pub fn mut_recv_invites(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTAPartyInvite> {
        &mut self.recv_invites
    }

    // Take field
    pub fn take_recv_invites(&mut self) -> ::protobuf::RepeatedField<CSODOTAPartyInvite> {
        ::std::mem::replace(&mut self.recv_invites, ::protobuf::RepeatedField::new())
    }

    pub fn get_recv_invites(&self) -> &[CSODOTAPartyInvite] {
        &self.recv_invites
    }

    fn get_recv_invites_for_reflect(&self) -> &::protobuf::RepeatedField<CSODOTAPartyInvite> {
        &self.recv_invites
    }

    fn mut_recv_invites_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTAPartyInvite> {
        &mut self.recv_invites
    }

    // optional uint32 account_flags = 43;

    pub fn clear_account_flags(&mut self) {
        self.account_flags = ::std::option::Option::None;
    }

    pub fn has_account_flags(&self) -> bool {
        self.account_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_flags(&mut self, v: u32) {
        self.account_flags = ::std::option::Option::Some(v);
    }

    pub fn get_account_flags(&self) -> u32 {
        self.account_flags.unwrap_or(0)
    }

    fn get_account_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_flags
    }

    fn mut_account_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_flags
    }

    // optional uint32 region_select_flags = 44;

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

    // optional uint32 exclusive_tournament_id = 45;

    pub fn clear_exclusive_tournament_id(&mut self) {
        self.exclusive_tournament_id = ::std::option::Option::None;
    }

    pub fn has_exclusive_tournament_id(&self) -> bool {
        self.exclusive_tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exclusive_tournament_id(&mut self, v: u32) {
        self.exclusive_tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_exclusive_tournament_id(&self) -> u32 {
        self.exclusive_tournament_id.unwrap_or(0)
    }

    fn get_exclusive_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.exclusive_tournament_id
    }

    fn mut_exclusive_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.exclusive_tournament_id
    }

    // optional uint32 tourney_division_id = 47;

    pub fn clear_tourney_division_id(&mut self) {
        self.tourney_division_id = ::std::option::Option::None;
    }

    pub fn has_tourney_division_id(&self) -> bool {
        self.tourney_division_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_division_id(&mut self, v: u32) {
        self.tourney_division_id = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_division_id(&self) -> u32 {
        self.tourney_division_id.unwrap_or(0)
    }

    fn get_tourney_division_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tourney_division_id
    }

    fn mut_tourney_division_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tourney_division_id
    }

    // optional uint32 tourney_schedule_time = 48;

    pub fn clear_tourney_schedule_time(&mut self) {
        self.tourney_schedule_time = ::std::option::Option::None;
    }

    pub fn has_tourney_schedule_time(&self) -> bool {
        self.tourney_schedule_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_schedule_time(&mut self, v: u32) {
        self.tourney_schedule_time = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_schedule_time(&self) -> u32 {
        self.tourney_schedule_time.unwrap_or(0)
    }

    fn get_tourney_schedule_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tourney_schedule_time
    }

    fn mut_tourney_schedule_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tourney_schedule_time
    }

    // optional uint32 tourney_skill_level = 49;

    pub fn clear_tourney_skill_level(&mut self) {
        self.tourney_skill_level = ::std::option::Option::None;
    }

    pub fn has_tourney_skill_level(&self) -> bool {
        self.tourney_skill_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_skill_level(&mut self, v: u32) {
        self.tourney_skill_level = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_skill_level(&self) -> u32 {
        self.tourney_skill_level.unwrap_or(0)
    }

    fn get_tourney_skill_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tourney_skill_level
    }

    fn mut_tourney_skill_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tourney_skill_level
    }

    // optional uint32 tourney_bracket_round = 50;

    pub fn clear_tourney_bracket_round(&mut self) {
        self.tourney_bracket_round = ::std::option::Option::None;
    }

    pub fn has_tourney_bracket_round(&self) -> bool {
        self.tourney_bracket_round.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_bracket_round(&mut self, v: u32) {
        self.tourney_bracket_round = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_bracket_round(&self) -> u32 {
        self.tourney_bracket_round.unwrap_or(0)
    }

    fn get_tourney_bracket_round_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tourney_bracket_round
    }

    fn mut_tourney_bracket_round_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tourney_bracket_round
    }

    // optional uint32 tourney_queue_deadline_time = 54;

    pub fn clear_tourney_queue_deadline_time(&mut self) {
        self.tourney_queue_deadline_time = ::std::option::Option::None;
    }

    pub fn has_tourney_queue_deadline_time(&self) -> bool {
        self.tourney_queue_deadline_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_queue_deadline_time(&mut self, v: u32) {
        self.tourney_queue_deadline_time = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_queue_deadline_time(&self) -> u32 {
        self.tourney_queue_deadline_time.unwrap_or(0)
    }

    fn get_tourney_queue_deadline_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tourney_queue_deadline_time
    }

    fn mut_tourney_queue_deadline_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tourney_queue_deadline_time
    }

    // optional .dota.ETourneyQueueDeadlineState tourney_queue_deadline_state = 55;

    pub fn clear_tourney_queue_deadline_state(&mut self) {
        self.tourney_queue_deadline_state = ::std::option::Option::None;
    }

    pub fn has_tourney_queue_deadline_state(&self) -> bool {
        self.tourney_queue_deadline_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney_queue_deadline_state(&mut self, v: super::dota_shared_enums::ETourneyQueueDeadlineState) {
        self.tourney_queue_deadline_state = ::std::option::Option::Some(v);
    }

    pub fn get_tourney_queue_deadline_state(&self) -> super::dota_shared_enums::ETourneyQueueDeadlineState {
        self.tourney_queue_deadline_state.unwrap_or(super::dota_shared_enums::ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_Normal)
    }

    fn get_tourney_queue_deadline_state_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::ETourneyQueueDeadlineState> {
        &self.tourney_queue_deadline_state
    }

    fn mut_tourney_queue_deadline_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::ETourneyQueueDeadlineState> {
        &mut self.tourney_queue_deadline_state
    }

    // optional uint32 party_builder_slots_to_fill = 56;

    pub fn clear_party_builder_slots_to_fill(&mut self) {
        self.party_builder_slots_to_fill = ::std::option::Option::None;
    }

    pub fn has_party_builder_slots_to_fill(&self) -> bool {
        self.party_builder_slots_to_fill.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_builder_slots_to_fill(&mut self, v: u32) {
        self.party_builder_slots_to_fill = ::std::option::Option::Some(v);
    }

    pub fn get_party_builder_slots_to_fill(&self) -> u32 {
        self.party_builder_slots_to_fill.unwrap_or(0)
    }

    fn get_party_builder_slots_to_fill_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.party_builder_slots_to_fill
    }

    fn mut_party_builder_slots_to_fill_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.party_builder_slots_to_fill
    }

    // optional uint32 party_builder_match_groups = 57;

    pub fn clear_party_builder_match_groups(&mut self) {
        self.party_builder_match_groups = ::std::option::Option::None;
    }

    pub fn has_party_builder_match_groups(&self) -> bool {
        self.party_builder_match_groups.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_builder_match_groups(&mut self, v: u32) {
        self.party_builder_match_groups = ::std::option::Option::Some(v);
    }

    pub fn get_party_builder_match_groups(&self) -> u32 {
        self.party_builder_match_groups.unwrap_or(0)
    }

    fn get_party_builder_match_groups_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.party_builder_match_groups
    }

    fn mut_party_builder_match_groups_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.party_builder_match_groups
    }

    // optional uint32 party_builder_start_time = 58;

    pub fn clear_party_builder_start_time(&mut self) {
        self.party_builder_start_time = ::std::option::Option::None;
    }

    pub fn has_party_builder_start_time(&self) -> bool {
        self.party_builder_start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_builder_start_time(&mut self, v: u32) {
        self.party_builder_start_time = ::std::option::Option::Some(v);
    }

    pub fn get_party_builder_start_time(&self) -> u32 {
        self.party_builder_start_time.unwrap_or(0)
    }

    fn get_party_builder_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.party_builder_start_time
    }

    fn mut_party_builder_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.party_builder_start_time
    }
}

impl ::protobuf::Message for CSODOTAParty {
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
                    self.party_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.leader_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.member_ids)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_modes = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.effective_started_matchmaking_time = ::std::option::Option::Some(tmp);
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.raw_started_matchmaking_time = ::std::option::Option::Some(tmp);
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.attempt_start_time = ::std::option::Option::Some(tmp);
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.attempt_num = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.matchgroups = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.low_priority_account_id = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.match_type = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.bot_difficulty = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                51 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name)?;
                },
                52 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.team_ui_logo = ::std::option::Option::Some(tmp);
                },
                53 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.team_base_logo = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.match_disabled_until_date = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.match_disabled_account_id = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.matchmaking_max_range_minutes = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.matchlanguages = ::std::option::Option::Some(tmp);
                },
                38 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.map_preference = ::std::option::Option::Some(tmp);
                },
                29 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.open_guild_id = ::std::option::Option::Some(tmp);
                },
                31 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.common_guilds)?;
                },
                35 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.low_priority_games_remaining = ::std::option::Option::Some(tmp);
                },
                39 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.active_ingame_events)?;
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.open_for_join_requests = ::std::option::Option::Some(tmp);
                },
                41 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sent_invites)?;
                },
                42 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.recv_invites)?;
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.account_flags = ::std::option::Option::Some(tmp);
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.region_select_flags = ::std::option::Option::Some(tmp);
                },
                45 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.exclusive_tournament_id = ::std::option::Option::Some(tmp);
                },
                47 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tourney_division_id = ::std::option::Option::Some(tmp);
                },
                48 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tourney_schedule_time = ::std::option::Option::Some(tmp);
                },
                49 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tourney_skill_level = ::std::option::Option::Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tourney_bracket_round = ::std::option::Option::Some(tmp);
                },
                54 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tourney_queue_deadline_time = ::std::option::Option::Some(tmp);
                },
                55 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.tourney_queue_deadline_state = ::std::option::Option::Some(tmp);
                },
                56 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.party_builder_slots_to_fill = ::std::option::Option::Some(tmp);
                },
                57 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.party_builder_match_groups = ::std::option::Option::Some(tmp);
                },
                58 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.party_builder_start_time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.party_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.leader_id {
            my_size += 9;
        };
        my_size += 9 * self.member_ids.len() as u32;
        if let Some(v) = self.game_modes {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(6, v);
        };
        if let Some(v) = self.effective_started_matchmaking_time {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.raw_started_matchmaking_time {
            my_size += ::protobuf::rt::value_size(32, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attempt_start_time {
            my_size += ::protobuf::rt::value_size(33, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.attempt_num {
            my_size += ::protobuf::rt::value_size(34, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.matchgroups {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.low_priority_account_id {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.match_type {
            my_size += ::protobuf::rt::enum_size(21, v);
        };
        if let Some(v) = self.bot_difficulty {
            my_size += ::protobuf::rt::enum_size(22, v);
        };
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(51, &v);
        };
        if let Some(v) = self.team_ui_logo {
            my_size += ::protobuf::rt::value_size(52, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_base_logo {
            my_size += ::protobuf::rt::value_size(53, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.match_disabled_until_date {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.match_disabled_account_id {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.matchmaking_max_range_minutes {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.matchlanguages {
            my_size += ::protobuf::rt::value_size(27, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.map_preference {
            my_size += ::protobuf::rt::value_size(38, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.open_guild_id {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.common_guilds {
            my_size += ::protobuf::rt::value_size(31, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.low_priority_games_remaining {
            my_size += ::protobuf::rt::value_size(35, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.active_ingame_events {
            my_size += ::protobuf::rt::enum_size(39, *value);
        };
        if let Some(v) = self.open_for_join_requests {
            my_size += 3;
        };
        for value in &self.sent_invites {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.recv_invites {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.account_flags {
            my_size += ::protobuf::rt::value_size(43, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.region_select_flags {
            my_size += ::protobuf::rt::value_size(44, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.exclusive_tournament_id {
            my_size += ::protobuf::rt::value_size(45, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_division_id {
            my_size += ::protobuf::rt::value_size(47, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_schedule_time {
            my_size += ::protobuf::rt::value_size(48, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_skill_level {
            my_size += ::protobuf::rt::value_size(49, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_bracket_round {
            my_size += ::protobuf::rt::value_size(50, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_queue_deadline_time {
            my_size += ::protobuf::rt::value_size(54, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tourney_queue_deadline_state {
            my_size += ::protobuf::rt::enum_size(55, v);
        };
        if let Some(v) = self.party_builder_slots_to_fill {
            my_size += ::protobuf::rt::value_size(56, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.party_builder_match_groups {
            my_size += ::protobuf::rt::value_size(57, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.party_builder_start_time {
            my_size += ::protobuf::rt::value_size(58, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.party_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.leader_id {
            os.write_fixed64(2, v)?;
        };
        for v in &self.member_ids {
            os.write_fixed64(3, *v)?;
        };
        if let Some(v) = self.game_modes {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.state {
            os.write_enum(6, v.value())?;
        };
        if let Some(v) = self.effective_started_matchmaking_time {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.raw_started_matchmaking_time {
            os.write_uint32(32, v)?;
        };
        if let Some(v) = self.attempt_start_time {
            os.write_uint32(33, v)?;
        };
        if let Some(v) = self.attempt_num {
            os.write_uint32(34, v)?;
        };
        if let Some(v) = self.matchgroups {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.low_priority_account_id {
            os.write_uint32(19, v)?;
        };
        if let Some(v) = self.match_type {
            os.write_enum(21, v.value())?;
        };
        if let Some(v) = self.bot_difficulty {
            os.write_enum(22, v.value())?;
        };
        if let Some(v) = self.team_id {
            os.write_uint32(23, v)?;
        };
        if let Some(v) = self.team_name.as_ref() {
            os.write_string(51, &v)?;
        };
        if let Some(v) = self.team_ui_logo {
            os.write_uint64(52, v)?;
        };
        if let Some(v) = self.team_base_logo {
            os.write_uint64(53, v)?;
        };
        if let Some(v) = self.match_disabled_until_date {
            os.write_uint32(24, v)?;
        };
        if let Some(v) = self.match_disabled_account_id {
            os.write_uint32(25, v)?;
        };
        if let Some(v) = self.matchmaking_max_range_minutes {
            os.write_uint32(26, v)?;
        };
        if let Some(v) = self.matchlanguages {
            os.write_uint32(27, v)?;
        };
        if let Some(v) = self.map_preference {
            os.write_uint32(38, v)?;
        };
        for v in &self.members {
            os.write_tag(29, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.open_guild_id {
            os.write_uint32(30, v)?;
        };
        for v in &self.common_guilds {
            os.write_uint32(31, *v)?;
        };
        if let Some(v) = self.low_priority_games_remaining {
            os.write_uint32(35, v)?;
        };
        for v in &self.active_ingame_events {
            os.write_enum(39, v.value())?;
        };
        if let Some(v) = self.open_for_join_requests {
            os.write_bool(40, v)?;
        };
        for v in &self.sent_invites {
            os.write_tag(41, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.recv_invites {
            os.write_tag(42, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.account_flags {
            os.write_uint32(43, v)?;
        };
        if let Some(v) = self.region_select_flags {
            os.write_uint32(44, v)?;
        };
        if let Some(v) = self.exclusive_tournament_id {
            os.write_uint32(45, v)?;
        };
        if let Some(v) = self.tourney_division_id {
            os.write_uint32(47, v)?;
        };
        if let Some(v) = self.tourney_schedule_time {
            os.write_uint32(48, v)?;
        };
        if let Some(v) = self.tourney_skill_level {
            os.write_uint32(49, v)?;
        };
        if let Some(v) = self.tourney_bracket_round {
            os.write_uint32(50, v)?;
        };
        if let Some(v) = self.tourney_queue_deadline_time {
            os.write_uint32(54, v)?;
        };
        if let Some(v) = self.tourney_queue_deadline_state {
            os.write_enum(55, v.value())?;
        };
        if let Some(v) = self.party_builder_slots_to_fill {
            os.write_uint32(56, v)?;
        };
        if let Some(v) = self.party_builder_match_groups {
            os.write_uint32(57, v)?;
        };
        if let Some(v) = self.party_builder_start_time {
            os.write_uint32(58, v)?;
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

impl ::protobuf::MessageStatic for CSODOTAParty {
    fn new() -> CSODOTAParty {
        CSODOTAParty::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSODOTAParty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "party_id",
                    CSODOTAParty::get_party_id_for_reflect,
                    CSODOTAParty::mut_party_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "leader_id",
                    CSODOTAParty::get_leader_id_for_reflect,
                    CSODOTAParty::mut_leader_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "member_ids",
                    CSODOTAParty::get_member_ids_for_reflect,
                    CSODOTAParty::mut_member_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_modes",
                    CSODOTAParty::get_game_modes_for_reflect,
                    CSODOTAParty::mut_game_modes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CSODOTAParty_State>>(
                    "state",
                    CSODOTAParty::get_state_for_reflect,
                    CSODOTAParty::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "effective_started_matchmaking_time",
                    CSODOTAParty::get_effective_started_matchmaking_time_for_reflect,
                    CSODOTAParty::mut_effective_started_matchmaking_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "raw_started_matchmaking_time",
                    CSODOTAParty::get_raw_started_matchmaking_time_for_reflect,
                    CSODOTAParty::mut_raw_started_matchmaking_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attempt_start_time",
                    CSODOTAParty::get_attempt_start_time_for_reflect,
                    CSODOTAParty::mut_attempt_start_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attempt_num",
                    CSODOTAParty::get_attempt_num_for_reflect,
                    CSODOTAParty::mut_attempt_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "matchgroups",
                    CSODOTAParty::get_matchgroups_for_reflect,
                    CSODOTAParty::mut_matchgroups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "low_priority_account_id",
                    CSODOTAParty::get_low_priority_account_id_for_reflect,
                    CSODOTAParty::mut_low_priority_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::MatchType>>(
                    "match_type",
                    CSODOTAParty::get_match_type_for_reflect,
                    CSODOTAParty::mut_match_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "bot_difficulty",
                    CSODOTAParty::get_bot_difficulty_for_reflect,
                    CSODOTAParty::mut_bot_difficulty_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CSODOTAParty::get_team_id_for_reflect,
                    CSODOTAParty::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name",
                    CSODOTAParty::get_team_name_for_reflect,
                    CSODOTAParty::mut_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_ui_logo",
                    CSODOTAParty::get_team_ui_logo_for_reflect,
                    CSODOTAParty::mut_team_ui_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_base_logo",
                    CSODOTAParty::get_team_base_logo_for_reflect,
                    CSODOTAParty::mut_team_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "match_disabled_until_date",
                    CSODOTAParty::get_match_disabled_until_date_for_reflect,
                    CSODOTAParty::mut_match_disabled_until_date_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "match_disabled_account_id",
                    CSODOTAParty::get_match_disabled_account_id_for_reflect,
                    CSODOTAParty::mut_match_disabled_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "matchmaking_max_range_minutes",
                    CSODOTAParty::get_matchmaking_max_range_minutes_for_reflect,
                    CSODOTAParty::mut_matchmaking_max_range_minutes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "matchlanguages",
                    CSODOTAParty::get_matchlanguages_for_reflect,
                    CSODOTAParty::mut_matchlanguages_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "map_preference",
                    CSODOTAParty::get_map_preference_for_reflect,
                    CSODOTAParty::mut_map_preference_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSODOTAPartyMember>>(
                    "members",
                    CSODOTAParty::get_members_for_reflect,
                    CSODOTAParty::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "open_guild_id",
                    CSODOTAParty::get_open_guild_id_for_reflect,
                    CSODOTAParty::mut_open_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "common_guilds",
                    CSODOTAParty::get_common_guilds_for_reflect,
                    CSODOTAParty::mut_common_guilds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "low_priority_games_remaining",
                    CSODOTAParty::get_low_priority_games_remaining_for_reflect,
                    CSODOTAParty::mut_low_priority_games_remaining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::EEvent>>(
                    "active_ingame_events",
                    CSODOTAParty::get_active_ingame_events_for_reflect,
                    CSODOTAParty::mut_active_ingame_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "open_for_join_requests",
                    CSODOTAParty::get_open_for_join_requests_for_reflect,
                    CSODOTAParty::mut_open_for_join_requests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSODOTAPartyInvite>>(
                    "sent_invites",
                    CSODOTAParty::get_sent_invites_for_reflect,
                    CSODOTAParty::mut_sent_invites_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSODOTAPartyInvite>>(
                    "recv_invites",
                    CSODOTAParty::get_recv_invites_for_reflect,
                    CSODOTAParty::mut_recv_invites_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_flags",
                    CSODOTAParty::get_account_flags_for_reflect,
                    CSODOTAParty::mut_account_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "region_select_flags",
                    CSODOTAParty::get_region_select_flags_for_reflect,
                    CSODOTAParty::mut_region_select_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "exclusive_tournament_id",
                    CSODOTAParty::get_exclusive_tournament_id_for_reflect,
                    CSODOTAParty::mut_exclusive_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tourney_division_id",
                    CSODOTAParty::get_tourney_division_id_for_reflect,
                    CSODOTAParty::mut_tourney_division_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tourney_schedule_time",
                    CSODOTAParty::get_tourney_schedule_time_for_reflect,
                    CSODOTAParty::mut_tourney_schedule_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tourney_skill_level",
                    CSODOTAParty::get_tourney_skill_level_for_reflect,
                    CSODOTAParty::mut_tourney_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tourney_bracket_round",
                    CSODOTAParty::get_tourney_bracket_round_for_reflect,
                    CSODOTAParty::mut_tourney_bracket_round_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tourney_queue_deadline_time",
                    CSODOTAParty::get_tourney_queue_deadline_time_for_reflect,
                    CSODOTAParty::mut_tourney_queue_deadline_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::ETourneyQueueDeadlineState>>(
                    "tourney_queue_deadline_state",
                    CSODOTAParty::get_tourney_queue_deadline_state_for_reflect,
                    CSODOTAParty::mut_tourney_queue_deadline_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "party_builder_slots_to_fill",
                    CSODOTAParty::get_party_builder_slots_to_fill_for_reflect,
                    CSODOTAParty::mut_party_builder_slots_to_fill_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "party_builder_match_groups",
                    CSODOTAParty::get_party_builder_match_groups_for_reflect,
                    CSODOTAParty::mut_party_builder_match_groups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "party_builder_start_time",
                    CSODOTAParty::get_party_builder_start_time_for_reflect,
                    CSODOTAParty::mut_party_builder_start_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSODOTAParty>(
                    "CSODOTAParty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSODOTAParty {
    fn clear(&mut self) {
        self.clear_party_id();
        self.clear_leader_id();
        self.clear_member_ids();
        self.clear_game_modes();
        self.clear_state();
        self.clear_effective_started_matchmaking_time();
        self.clear_raw_started_matchmaking_time();
        self.clear_attempt_start_time();
        self.clear_attempt_num();
        self.clear_matchgroups();
        self.clear_low_priority_account_id();
        self.clear_match_type();
        self.clear_bot_difficulty();
        self.clear_team_id();
        self.clear_team_name();
        self.clear_team_ui_logo();
        self.clear_team_base_logo();
        self.clear_match_disabled_until_date();
        self.clear_match_disabled_account_id();
        self.clear_matchmaking_max_range_minutes();
        self.clear_matchlanguages();
        self.clear_map_preference();
        self.clear_members();
        self.clear_open_guild_id();
        self.clear_common_guilds();
        self.clear_low_priority_games_remaining();
        self.clear_active_ingame_events();
        self.clear_open_for_join_requests();
        self.clear_sent_invites();
        self.clear_recv_invites();
        self.clear_account_flags();
        self.clear_region_select_flags();
        self.clear_exclusive_tournament_id();
        self.clear_tourney_division_id();
        self.clear_tourney_schedule_time();
        self.clear_tourney_skill_level();
        self.clear_tourney_bracket_round();
        self.clear_tourney_queue_deadline_time();
        self.clear_tourney_queue_deadline_state();
        self.clear_party_builder_slots_to_fill();
        self.clear_party_builder_match_groups();
        self.clear_party_builder_start_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSODOTAParty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSODOTAParty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CSODOTAParty_State {
    UI = 0,
    FINDING_MATCH = 1,
    IN_MATCH = 2,
}

impl ::protobuf::ProtobufEnum for CSODOTAParty_State {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CSODOTAParty_State> {
        match value {
            0 => ::std::option::Option::Some(CSODOTAParty_State::UI),
            1 => ::std::option::Option::Some(CSODOTAParty_State::FINDING_MATCH),
            2 => ::std::option::Option::Some(CSODOTAParty_State::IN_MATCH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CSODOTAParty_State] = &[
            CSODOTAParty_State::UI,
            CSODOTAParty_State::FINDING_MATCH,
            CSODOTAParty_State::IN_MATCH,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CSODOTAParty_State>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CSODOTAParty_State", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CSODOTAParty_State {
}

impl ::protobuf::reflect::ProtobufValue for CSODOTAParty_State {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSODOTAPartyInvite {
    // message fields
    group_id: ::std::option::Option<u64>,
    sender_id: ::std::option::Option<u64>,
    sender_name: ::protobuf::SingularField<::std::string::String>,
    members: ::protobuf::RepeatedField<CSODOTAPartyInvite_PartyMember>,
    team_id: ::std::option::Option<u32>,
    low_priority_status: ::std::option::Option<bool>,
    as_coach: ::std::option::Option<bool>,
    invite_gid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSODOTAPartyInvite {}

impl CSODOTAPartyInvite {
    pub fn new() -> CSODOTAPartyInvite {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSODOTAPartyInvite {
        static mut instance: ::protobuf::lazy::Lazy<CSODOTAPartyInvite> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSODOTAPartyInvite,
        };
        unsafe {
            instance.get(CSODOTAPartyInvite::new)
        }
    }

    // optional uint64 group_id = 1;

    pub fn clear_group_id(&mut self) {
        self.group_id = ::std::option::Option::None;
    }

    pub fn has_group_id(&self) -> bool {
        self.group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group_id(&mut self, v: u64) {
        self.group_id = ::std::option::Option::Some(v);
    }

    pub fn get_group_id(&self) -> u64 {
        self.group_id.unwrap_or(0)
    }

    fn get_group_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.group_id
    }

    fn mut_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.group_id
    }

    // optional fixed64 sender_id = 2;

    pub fn clear_sender_id(&mut self) {
        self.sender_id = ::std::option::Option::None;
    }

    pub fn has_sender_id(&self) -> bool {
        self.sender_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender_id(&mut self, v: u64) {
        self.sender_id = ::std::option::Option::Some(v);
    }

    pub fn get_sender_id(&self) -> u64 {
        self.sender_id.unwrap_or(0)
    }

    fn get_sender_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sender_id
    }

    fn mut_sender_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sender_id
    }

    // optional string sender_name = 3;

    pub fn clear_sender_name(&mut self) {
        self.sender_name.clear();
    }

    pub fn has_sender_name(&self) -> bool {
        self.sender_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender_name(&mut self, v: ::std::string::String) {
        self.sender_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender_name(&mut self) -> &mut ::std::string::String {
        if self.sender_name.is_none() {
            self.sender_name.set_default();
        };
        self.sender_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender_name(&mut self) -> ::std::string::String {
        self.sender_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sender_name(&self) -> &str {
        match self.sender_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sender_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sender_name
    }

    fn mut_sender_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sender_name
    }

    // repeated .dota.CSODOTAPartyInvite.PartyMember members = 4;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CSODOTAPartyInvite_PartyMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTAPartyInvite_PartyMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CSODOTAPartyInvite_PartyMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CSODOTAPartyInvite_PartyMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CSODOTAPartyInvite_PartyMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTAPartyInvite_PartyMember> {
        &mut self.members
    }

    // optional uint32 team_id = 5;

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

    // optional bool low_priority_status = 6;

    pub fn clear_low_priority_status(&mut self) {
        self.low_priority_status = ::std::option::Option::None;
    }

    pub fn has_low_priority_status(&self) -> bool {
        self.low_priority_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_priority_status(&mut self, v: bool) {
        self.low_priority_status = ::std::option::Option::Some(v);
    }

    pub fn get_low_priority_status(&self) -> bool {
        self.low_priority_status.unwrap_or(false)
    }

    fn get_low_priority_status_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.low_priority_status
    }

    fn mut_low_priority_status_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.low_priority_status
    }

    // optional bool as_coach = 7;

    pub fn clear_as_coach(&mut self) {
        self.as_coach = ::std::option::Option::None;
    }

    pub fn has_as_coach(&self) -> bool {
        self.as_coach.is_some()
    }

    // Param is passed by value, moved
    pub fn set_as_coach(&mut self, v: bool) {
        self.as_coach = ::std::option::Option::Some(v);
    }

    pub fn get_as_coach(&self) -> bool {
        self.as_coach.unwrap_or(false)
    }

    fn get_as_coach_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.as_coach
    }

    fn mut_as_coach_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.as_coach
    }

    // optional fixed64 invite_gid = 8;

    pub fn clear_invite_gid(&mut self) {
        self.invite_gid = ::std::option::Option::None;
    }

    pub fn has_invite_gid(&self) -> bool {
        self.invite_gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invite_gid(&mut self, v: u64) {
        self.invite_gid = ::std::option::Option::Some(v);
    }

    pub fn get_invite_gid(&self) -> u64 {
        self.invite_gid.unwrap_or(0)
    }

    fn get_invite_gid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invite_gid
    }

    fn mut_invite_gid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invite_gid
    }
}

impl ::protobuf::Message for CSODOTAPartyInvite {
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
                    self.group_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.sender_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sender_name)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.low_priority_status = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.as_coach = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.invite_gid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.group_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sender_id {
            my_size += 9;
        };
        if let Some(v) = self.sender_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.low_priority_status {
            my_size += 2;
        };
        if let Some(v) = self.as_coach {
            my_size += 2;
        };
        if let Some(v) = self.invite_gid {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.group_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.sender_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.sender_name.as_ref() {
            os.write_string(3, &v)?;
        };
        for v in &self.members {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.team_id {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.low_priority_status {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.as_coach {
            os.write_bool(7, v)?;
        };
        if let Some(v) = self.invite_gid {
            os.write_fixed64(8, v)?;
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

impl ::protobuf::MessageStatic for CSODOTAPartyInvite {
    fn new() -> CSODOTAPartyInvite {
        CSODOTAPartyInvite::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSODOTAPartyInvite>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "group_id",
                    CSODOTAPartyInvite::get_group_id_for_reflect,
                    CSODOTAPartyInvite::mut_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "sender_id",
                    CSODOTAPartyInvite::get_sender_id_for_reflect,
                    CSODOTAPartyInvite::mut_sender_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sender_name",
                    CSODOTAPartyInvite::get_sender_name_for_reflect,
                    CSODOTAPartyInvite::mut_sender_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSODOTAPartyInvite_PartyMember>>(
                    "members",
                    CSODOTAPartyInvite::get_members_for_reflect,
                    CSODOTAPartyInvite::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CSODOTAPartyInvite::get_team_id_for_reflect,
                    CSODOTAPartyInvite::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "low_priority_status",
                    CSODOTAPartyInvite::get_low_priority_status_for_reflect,
                    CSODOTAPartyInvite::mut_low_priority_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "as_coach",
                    CSODOTAPartyInvite::get_as_coach_for_reflect,
                    CSODOTAPartyInvite::mut_as_coach_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "invite_gid",
                    CSODOTAPartyInvite::get_invite_gid_for_reflect,
                    CSODOTAPartyInvite::mut_invite_gid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSODOTAPartyInvite>(
                    "CSODOTAPartyInvite",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSODOTAPartyInvite {
    fn clear(&mut self) {
        self.clear_group_id();
        self.clear_sender_id();
        self.clear_sender_name();
        self.clear_members();
        self.clear_team_id();
        self.clear_low_priority_status();
        self.clear_as_coach();
        self.clear_invite_gid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSODOTAPartyInvite {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSODOTAPartyInvite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSODOTAPartyInvite_PartyMember {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    steam_id: ::std::option::Option<u64>,
    is_coach: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSODOTAPartyInvite_PartyMember {}

impl CSODOTAPartyInvite_PartyMember {
    pub fn new() -> CSODOTAPartyInvite_PartyMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSODOTAPartyInvite_PartyMember {
        static mut instance: ::protobuf::lazy::Lazy<CSODOTAPartyInvite_PartyMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSODOTAPartyInvite_PartyMember,
        };
        unsafe {
            instance.get(CSODOTAPartyInvite_PartyMember::new)
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

    // optional bool is_coach = 4;

    pub fn clear_is_coach(&mut self) {
        self.is_coach = ::std::option::Option::None;
    }

    pub fn has_is_coach(&self) -> bool {
        self.is_coach.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_coach(&mut self, v: bool) {
        self.is_coach = ::std::option::Option::Some(v);
    }

    pub fn get_is_coach(&self) -> bool {
        self.is_coach.unwrap_or(false)
    }

    fn get_is_coach_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_coach
    }

    fn mut_is_coach_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_coach
    }
}

impl ::protobuf::Message for CSODOTAPartyInvite_PartyMember {
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
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_coach = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.steam_id {
            my_size += 9;
        };
        if let Some(v) = self.is_coach {
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
        if let Some(v) = self.steam_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.is_coach {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for CSODOTAPartyInvite_PartyMember {
    fn new() -> CSODOTAPartyInvite_PartyMember {
        CSODOTAPartyInvite_PartyMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSODOTAPartyInvite_PartyMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSODOTAPartyInvite_PartyMember::get_name_for_reflect,
                    CSODOTAPartyInvite_PartyMember::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CSODOTAPartyInvite_PartyMember::get_steam_id_for_reflect,
                    CSODOTAPartyInvite_PartyMember::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_coach",
                    CSODOTAPartyInvite_PartyMember::get_is_coach_for_reflect,
                    CSODOTAPartyInvite_PartyMember::mut_is_coach_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSODOTAPartyInvite_PartyMember>(
                    "CSODOTAPartyInvite_PartyMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSODOTAPartyInvite_PartyMember {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_steam_id();
        self.clear_is_coach();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSODOTAPartyInvite_PartyMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSODOTAPartyInvite_PartyMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSODOTALobbyInvite {
    // message fields
    group_id: ::std::option::Option<u64>,
    sender_id: ::std::option::Option<u64>,
    sender_name: ::protobuf::SingularField<::std::string::String>,
    members: ::protobuf::RepeatedField<CSODOTALobbyInvite_LobbyMember>,
    custom_game_id: ::std::option::Option<u64>,
    invite_gid: ::std::option::Option<u64>,
    custom_game_crc: ::std::option::Option<u64>,
    custom_game_timestamp: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSODOTALobbyInvite {}

impl CSODOTALobbyInvite {
    pub fn new() -> CSODOTALobbyInvite {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSODOTALobbyInvite {
        static mut instance: ::protobuf::lazy::Lazy<CSODOTALobbyInvite> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSODOTALobbyInvite,
        };
        unsafe {
            instance.get(CSODOTALobbyInvite::new)
        }
    }

    // optional uint64 group_id = 1;

    pub fn clear_group_id(&mut self) {
        self.group_id = ::std::option::Option::None;
    }

    pub fn has_group_id(&self) -> bool {
        self.group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group_id(&mut self, v: u64) {
        self.group_id = ::std::option::Option::Some(v);
    }

    pub fn get_group_id(&self) -> u64 {
        self.group_id.unwrap_or(0)
    }

    fn get_group_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.group_id
    }

    fn mut_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.group_id
    }

    // optional fixed64 sender_id = 2;

    pub fn clear_sender_id(&mut self) {
        self.sender_id = ::std::option::Option::None;
    }

    pub fn has_sender_id(&self) -> bool {
        self.sender_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender_id(&mut self, v: u64) {
        self.sender_id = ::std::option::Option::Some(v);
    }

    pub fn get_sender_id(&self) -> u64 {
        self.sender_id.unwrap_or(0)
    }

    fn get_sender_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sender_id
    }

    fn mut_sender_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sender_id
    }

    // optional string sender_name = 3;

    pub fn clear_sender_name(&mut self) {
        self.sender_name.clear();
    }

    pub fn has_sender_name(&self) -> bool {
        self.sender_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sender_name(&mut self, v: ::std::string::String) {
        self.sender_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender_name(&mut self) -> &mut ::std::string::String {
        if self.sender_name.is_none() {
            self.sender_name.set_default();
        };
        self.sender_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_sender_name(&mut self) -> ::std::string::String {
        self.sender_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sender_name(&self) -> &str {
        match self.sender_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sender_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sender_name
    }

    fn mut_sender_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sender_name
    }

    // repeated .dota.CSODOTALobbyInvite.LobbyMember members = 4;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CSODOTALobbyInvite_LobbyMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTALobbyInvite_LobbyMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CSODOTALobbyInvite_LobbyMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CSODOTALobbyInvite_LobbyMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CSODOTALobbyInvite_LobbyMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTALobbyInvite_LobbyMember> {
        &mut self.members
    }

    // optional uint64 custom_game_id = 5;

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

    // optional fixed64 invite_gid = 6;

    pub fn clear_invite_gid(&mut self) {
        self.invite_gid = ::std::option::Option::None;
    }

    pub fn has_invite_gid(&self) -> bool {
        self.invite_gid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_invite_gid(&mut self, v: u64) {
        self.invite_gid = ::std::option::Option::Some(v);
    }

    pub fn get_invite_gid(&self) -> u64 {
        self.invite_gid.unwrap_or(0)
    }

    fn get_invite_gid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.invite_gid
    }

    fn mut_invite_gid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.invite_gid
    }

    // optional fixed64 custom_game_crc = 7;

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

    // optional fixed32 custom_game_timestamp = 8;

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

impl ::protobuf::Message for CSODOTALobbyInvite {
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
                    self.group_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.sender_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sender_name)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.custom_game_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.invite_gid = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.custom_game_crc = ::std::option::Option::Some(tmp);
                },
                8 => {
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
        if let Some(v) = self.group_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.sender_id {
            my_size += 9;
        };
        if let Some(v) = self.sender_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.invite_gid {
            my_size += 9;
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
        if let Some(v) = self.group_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.sender_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.sender_name.as_ref() {
            os.write_string(3, &v)?;
        };
        for v in &self.members {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.custom_game_id {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.invite_gid {
            os.write_fixed64(6, v)?;
        };
        if let Some(v) = self.custom_game_crc {
            os.write_fixed64(7, v)?;
        };
        if let Some(v) = self.custom_game_timestamp {
            os.write_fixed32(8, v)?;
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

impl ::protobuf::MessageStatic for CSODOTALobbyInvite {
    fn new() -> CSODOTALobbyInvite {
        CSODOTALobbyInvite::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSODOTALobbyInvite>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "group_id",
                    CSODOTALobbyInvite::get_group_id_for_reflect,
                    CSODOTALobbyInvite::mut_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "sender_id",
                    CSODOTALobbyInvite::get_sender_id_for_reflect,
                    CSODOTALobbyInvite::mut_sender_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sender_name",
                    CSODOTALobbyInvite::get_sender_name_for_reflect,
                    CSODOTALobbyInvite::mut_sender_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSODOTALobbyInvite_LobbyMember>>(
                    "members",
                    CSODOTALobbyInvite::get_members_for_reflect,
                    CSODOTALobbyInvite::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CSODOTALobbyInvite::get_custom_game_id_for_reflect,
                    CSODOTALobbyInvite::mut_custom_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "invite_gid",
                    CSODOTALobbyInvite::get_invite_gid_for_reflect,
                    CSODOTALobbyInvite::mut_invite_gid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "custom_game_crc",
                    CSODOTALobbyInvite::get_custom_game_crc_for_reflect,
                    CSODOTALobbyInvite::mut_custom_game_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "custom_game_timestamp",
                    CSODOTALobbyInvite::get_custom_game_timestamp_for_reflect,
                    CSODOTALobbyInvite::mut_custom_game_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSODOTALobbyInvite>(
                    "CSODOTALobbyInvite",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSODOTALobbyInvite {
    fn clear(&mut self) {
        self.clear_group_id();
        self.clear_sender_id();
        self.clear_sender_name();
        self.clear_members();
        self.clear_custom_game_id();
        self.clear_invite_gid();
        self.clear_custom_game_crc();
        self.clear_custom_game_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSODOTALobbyInvite {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSODOTALobbyInvite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSODOTALobbyInvite_LobbyMember {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    steam_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSODOTALobbyInvite_LobbyMember {}

impl CSODOTALobbyInvite_LobbyMember {
    pub fn new() -> CSODOTALobbyInvite_LobbyMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSODOTALobbyInvite_LobbyMember {
        static mut instance: ::protobuf::lazy::Lazy<CSODOTALobbyInvite_LobbyMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSODOTALobbyInvite_LobbyMember,
        };
        unsafe {
            instance.get(CSODOTALobbyInvite_LobbyMember::new)
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
}

impl ::protobuf::Message for CSODOTALobbyInvite_LobbyMember {
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
                    self.steam_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.steam_id {
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
        if let Some(v) = self.steam_id {
            os.write_fixed64(2, v)?;
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

impl ::protobuf::MessageStatic for CSODOTALobbyInvite_LobbyMember {
    fn new() -> CSODOTALobbyInvite_LobbyMember {
        CSODOTALobbyInvite_LobbyMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSODOTALobbyInvite_LobbyMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSODOTALobbyInvite_LobbyMember::get_name_for_reflect,
                    CSODOTALobbyInvite_LobbyMember::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CSODOTALobbyInvite_LobbyMember::get_steam_id_for_reflect,
                    CSODOTALobbyInvite_LobbyMember::mut_steam_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSODOTALobbyInvite_LobbyMember>(
                    "CSODOTALobbyInvite_LobbyMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSODOTALobbyInvite_LobbyMember {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_steam_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSODOTALobbyInvite_LobbyMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSODOTALobbyInvite_LobbyMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgLeaverState {
    // message fields
    lobby_state: ::std::option::Option<u32>,
    game_state: ::std::option::Option<super::dota_shared_enums::DOTA_GameState>,
    leaver_detected: ::std::option::Option<bool>,
    first_blood_happened: ::std::option::Option<bool>,
    discard_match_results: ::std::option::Option<bool>,
    mass_disconnect: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgLeaverState {}

impl CMsgLeaverState {
    pub fn new() -> CMsgLeaverState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgLeaverState {
        static mut instance: ::protobuf::lazy::Lazy<CMsgLeaverState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgLeaverState,
        };
        unsafe {
            instance.get(CMsgLeaverState::new)
        }
    }

    // optional uint32 lobby_state = 1;

    pub fn clear_lobby_state(&mut self) {
        self.lobby_state = ::std::option::Option::None;
    }

    pub fn has_lobby_state(&self) -> bool {
        self.lobby_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_state(&mut self, v: u32) {
        self.lobby_state = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_state(&self) -> u32 {
        self.lobby_state.unwrap_or(0)
    }

    fn get_lobby_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lobby_state
    }

    fn mut_lobby_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lobby_state
    }

    // optional .dota.DOTA_GameState game_state = 2;

    pub fn clear_game_state(&mut self) {
        self.game_state = ::std::option::Option::None;
    }

    pub fn has_game_state(&self) -> bool {
        self.game_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_state(&mut self, v: super::dota_shared_enums::DOTA_GameState) {
        self.game_state = ::std::option::Option::Some(v);
    }

    pub fn get_game_state(&self) -> super::dota_shared_enums::DOTA_GameState {
        self.game_state.unwrap_or(super::dota_shared_enums::DOTA_GameState::DOTA_GAMERULES_STATE_INIT)
    }

    fn get_game_state_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GameState> {
        &self.game_state
    }

    fn mut_game_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GameState> {
        &mut self.game_state
    }

    // optional bool leaver_detected = 3;

    pub fn clear_leaver_detected(&mut self) {
        self.leaver_detected = ::std::option::Option::None;
    }

    pub fn has_leaver_detected(&self) -> bool {
        self.leaver_detected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leaver_detected(&mut self, v: bool) {
        self.leaver_detected = ::std::option::Option::Some(v);
    }

    pub fn get_leaver_detected(&self) -> bool {
        self.leaver_detected.unwrap_or(false)
    }

    fn get_leaver_detected_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.leaver_detected
    }

    fn mut_leaver_detected_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.leaver_detected
    }

    // optional bool first_blood_happened = 4;

    pub fn clear_first_blood_happened(&mut self) {
        self.first_blood_happened = ::std::option::Option::None;
    }

    pub fn has_first_blood_happened(&self) -> bool {
        self.first_blood_happened.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_blood_happened(&mut self, v: bool) {
        self.first_blood_happened = ::std::option::Option::Some(v);
    }

    pub fn get_first_blood_happened(&self) -> bool {
        self.first_blood_happened.unwrap_or(false)
    }

    fn get_first_blood_happened_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.first_blood_happened
    }

    fn mut_first_blood_happened_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.first_blood_happened
    }

    // optional bool discard_match_results = 5;

    pub fn clear_discard_match_results(&mut self) {
        self.discard_match_results = ::std::option::Option::None;
    }

    pub fn has_discard_match_results(&self) -> bool {
        self.discard_match_results.is_some()
    }

    // Param is passed by value, moved
    pub fn set_discard_match_results(&mut self, v: bool) {
        self.discard_match_results = ::std::option::Option::Some(v);
    }

    pub fn get_discard_match_results(&self) -> bool {
        self.discard_match_results.unwrap_or(false)
    }

    fn get_discard_match_results_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.discard_match_results
    }

    fn mut_discard_match_results_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.discard_match_results
    }

    // optional bool mass_disconnect = 6;

    pub fn clear_mass_disconnect(&mut self) {
        self.mass_disconnect = ::std::option::Option::None;
    }

    pub fn has_mass_disconnect(&self) -> bool {
        self.mass_disconnect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mass_disconnect(&mut self, v: bool) {
        self.mass_disconnect = ::std::option::Option::Some(v);
    }

    pub fn get_mass_disconnect(&self) -> bool {
        self.mass_disconnect.unwrap_or(false)
    }

    fn get_mass_disconnect_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.mass_disconnect
    }

    fn mut_mass_disconnect_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.mass_disconnect
    }
}

impl ::protobuf::Message for CMsgLeaverState {
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
                    self.lobby_state = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.leaver_detected = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.first_blood_happened = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.discard_match_results = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.mass_disconnect = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.lobby_state {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_state {
            my_size += ::protobuf::rt::enum_size(2, v);
        };
        if let Some(v) = self.leaver_detected {
            my_size += 2;
        };
        if let Some(v) = self.first_blood_happened {
            my_size += 2;
        };
        if let Some(v) = self.discard_match_results {
            my_size += 2;
        };
        if let Some(v) = self.mass_disconnect {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_state {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.game_state {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.leaver_detected {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.first_blood_happened {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.discard_match_results {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.mass_disconnect {
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

impl ::protobuf::MessageStatic for CMsgLeaverState {
    fn new() -> CMsgLeaverState {
        CMsgLeaverState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgLeaverState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lobby_state",
                    CMsgLeaverState::get_lobby_state_for_reflect,
                    CMsgLeaverState::mut_lobby_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GameState>>(
                    "game_state",
                    CMsgLeaverState::get_game_state_for_reflect,
                    CMsgLeaverState::mut_game_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "leaver_detected",
                    CMsgLeaverState::get_leaver_detected_for_reflect,
                    CMsgLeaverState::mut_leaver_detected_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "first_blood_happened",
                    CMsgLeaverState::get_first_blood_happened_for_reflect,
                    CMsgLeaverState::mut_first_blood_happened_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "discard_match_results",
                    CMsgLeaverState::get_discard_match_results_for_reflect,
                    CMsgLeaverState::mut_discard_match_results_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "mass_disconnect",
                    CMsgLeaverState::get_mass_disconnect_for_reflect,
                    CMsgLeaverState::mut_mass_disconnect_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgLeaverState>(
                    "CMsgLeaverState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgLeaverState {
    fn clear(&mut self) {
        self.clear_lobby_state();
        self.clear_game_state();
        self.clear_leaver_detected();
        self.clear_first_blood_happened();
        self.clear_discard_match_results();
        self.clear_mass_disconnect();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgLeaverState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgLeaverState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTALobbyMember {
    // message fields
    id: ::std::option::Option<u64>,
    hero_id: ::std::option::Option<u32>,
    team: ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM>,
    name: ::protobuf::SingularField<::std::string::String>,
    slot: ::std::option::Option<u32>,
    party_id: ::std::option::Option<u64>,
    meta_level: ::std::option::Option<u32>,
    meta_xp: ::std::option::Option<u32>,
    meta_xp_awarded: ::std::option::Option<u32>,
    leaver_status: ::std::option::Option<super::dota_shared_enums::DOTALeaverStatus_t>,
    leaver_actions: ::std::option::Option<u32>,
    channel: ::std::option::Option<u32>,
    prize_def_index: ::std::option::Option<u32>,
    disabled_hero_id: ::std::vec::Vec<u32>,
    partner_account_type: ::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType>,
    enabled_hero_id: ::std::vec::Vec<u32>,
    coach_team: ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM>,
    nexon_pc_bang_no: ::std::option::Option<u32>,
    nexon_pc_bang_name: ::protobuf::SingularField<::std::string::String>,
    xp_bonuses: ::protobuf::RepeatedField<CDOTALobbyMember_CDOTALobbyMemberXPBonus>,
    rank_change: ::std::option::Option<i32>,
    cameraman: ::std::option::Option<bool>,
    custom_game_product_ids: ::std::vec::Vec<u32>,
    lobby_mvp_vote_account_id: ::std::option::Option<u32>,
    search_match_type: ::std::option::Option<super::dota_shared_enums::MatchType>,
    favorite_team_and_quality: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTALobbyMember {}

impl CDOTALobbyMember {
    pub fn new() -> CDOTALobbyMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTALobbyMember {
        static mut instance: ::protobuf::lazy::Lazy<CDOTALobbyMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTALobbyMember,
        };
        unsafe {
            instance.get(CDOTALobbyMember::new)
        }
    }

    // optional fixed64 id = 1;

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

    // optional uint32 hero_id = 2;

    pub fn clear_hero_id(&mut self) {
        self.hero_id = ::std::option::Option::None;
    }

    pub fn has_hero_id(&self) -> bool {
        self.hero_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_id(&mut self, v: u32) {
        self.hero_id = ::std::option::Option::Some(v);
    }

    pub fn get_hero_id(&self) -> u32 {
        self.hero_id.unwrap_or(0)
    }

    fn get_hero_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hero_id
    }

    fn mut_hero_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hero_id
    }

    // optional .dota.DOTA_GC_TEAM team = 3;

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

    // optional string name = 6;

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

    // optional uint32 slot = 7;

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

    // optional uint64 party_id = 12;

    pub fn clear_party_id(&mut self) {
        self.party_id = ::std::option::Option::None;
    }

    pub fn has_party_id(&self) -> bool {
        self.party_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_party_id(&mut self, v: u64) {
        self.party_id = ::std::option::Option::Some(v);
    }

    pub fn get_party_id(&self) -> u64 {
        self.party_id.unwrap_or(0)
    }

    fn get_party_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.party_id
    }

    fn mut_party_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.party_id
    }

    // optional uint32 meta_level = 13;

    pub fn clear_meta_level(&mut self) {
        self.meta_level = ::std::option::Option::None;
    }

    pub fn has_meta_level(&self) -> bool {
        self.meta_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta_level(&mut self, v: u32) {
        self.meta_level = ::std::option::Option::Some(v);
    }

    pub fn get_meta_level(&self) -> u32 {
        self.meta_level.unwrap_or(0)
    }

    fn get_meta_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.meta_level
    }

    fn mut_meta_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.meta_level
    }

    // optional uint32 meta_xp = 14;

    pub fn clear_meta_xp(&mut self) {
        self.meta_xp = ::std::option::Option::None;
    }

    pub fn has_meta_xp(&self) -> bool {
        self.meta_xp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta_xp(&mut self, v: u32) {
        self.meta_xp = ::std::option::Option::Some(v);
    }

    pub fn get_meta_xp(&self) -> u32 {
        self.meta_xp.unwrap_or(0)
    }

    fn get_meta_xp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.meta_xp
    }

    fn mut_meta_xp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.meta_xp
    }

    // optional uint32 meta_xp_awarded = 15;

    pub fn clear_meta_xp_awarded(&mut self) {
        self.meta_xp_awarded = ::std::option::Option::None;
    }

    pub fn has_meta_xp_awarded(&self) -> bool {
        self.meta_xp_awarded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta_xp_awarded(&mut self, v: u32) {
        self.meta_xp_awarded = ::std::option::Option::Some(v);
    }

    pub fn get_meta_xp_awarded(&self) -> u32 {
        self.meta_xp_awarded.unwrap_or(0)
    }

    fn get_meta_xp_awarded_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.meta_xp_awarded
    }

    fn mut_meta_xp_awarded_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.meta_xp_awarded
    }

    // optional .dota.DOTALeaverStatus_t leaver_status = 16;

    pub fn clear_leaver_status(&mut self) {
        self.leaver_status = ::std::option::Option::None;
    }

    pub fn has_leaver_status(&self) -> bool {
        self.leaver_status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leaver_status(&mut self, v: super::dota_shared_enums::DOTALeaverStatus_t) {
        self.leaver_status = ::std::option::Option::Some(v);
    }

    pub fn get_leaver_status(&self) -> super::dota_shared_enums::DOTALeaverStatus_t {
        self.leaver_status.unwrap_or(super::dota_shared_enums::DOTALeaverStatus_t::DOTA_LEAVER_NONE)
    }

    fn get_leaver_status_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTALeaverStatus_t> {
        &self.leaver_status
    }

    fn mut_leaver_status_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTALeaverStatus_t> {
        &mut self.leaver_status
    }

    // optional uint32 leaver_actions = 28;

    pub fn clear_leaver_actions(&mut self) {
        self.leaver_actions = ::std::option::Option::None;
    }

    pub fn has_leaver_actions(&self) -> bool {
        self.leaver_actions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leaver_actions(&mut self, v: u32) {
        self.leaver_actions = ::std::option::Option::Some(v);
    }

    pub fn get_leaver_actions(&self) -> u32 {
        self.leaver_actions.unwrap_or(0)
    }

    fn get_leaver_actions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.leaver_actions
    }

    fn mut_leaver_actions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.leaver_actions
    }

    // optional uint32 channel = 17;

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

    // optional uint32 prize_def_index = 18;

    pub fn clear_prize_def_index(&mut self) {
        self.prize_def_index = ::std::option::Option::None;
    }

    pub fn has_prize_def_index(&self) -> bool {
        self.prize_def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prize_def_index(&mut self, v: u32) {
        self.prize_def_index = ::std::option::Option::Some(v);
    }

    pub fn get_prize_def_index(&self) -> u32 {
        self.prize_def_index.unwrap_or(0)
    }

    fn get_prize_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.prize_def_index
    }

    fn mut_prize_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.prize_def_index
    }

    // repeated uint32 disabled_hero_id = 20;

    pub fn clear_disabled_hero_id(&mut self) {
        self.disabled_hero_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_disabled_hero_id(&mut self, v: ::std::vec::Vec<u32>) {
        self.disabled_hero_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_disabled_hero_id(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.disabled_hero_id
    }

    // Take field
    pub fn take_disabled_hero_id(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.disabled_hero_id, ::std::vec::Vec::new())
    }

    pub fn get_disabled_hero_id(&self) -> &[u32] {
        &self.disabled_hero_id
    }

    fn get_disabled_hero_id_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.disabled_hero_id
    }

    fn mut_disabled_hero_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.disabled_hero_id
    }

    // optional .dota.PartnerAccountType partner_account_type = 21;

    pub fn clear_partner_account_type(&mut self) {
        self.partner_account_type = ::std::option::Option::None;
    }

    pub fn has_partner_account_type(&self) -> bool {
        self.partner_account_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partner_account_type(&mut self, v: super::gcsdk_gcmessages::PartnerAccountType) {
        self.partner_account_type = ::std::option::Option::Some(v);
    }

    pub fn get_partner_account_type(&self) -> super::gcsdk_gcmessages::PartnerAccountType {
        self.partner_account_type.unwrap_or(super::gcsdk_gcmessages::PartnerAccountType::PARTNER_NONE)
    }

    fn get_partner_account_type_for_reflect(&self) -> &::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType> {
        &self.partner_account_type
    }

    fn mut_partner_account_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType> {
        &mut self.partner_account_type
    }

    // repeated uint32 enabled_hero_id = 22;

    pub fn clear_enabled_hero_id(&mut self) {
        self.enabled_hero_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_enabled_hero_id(&mut self, v: ::std::vec::Vec<u32>) {
        self.enabled_hero_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enabled_hero_id(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.enabled_hero_id
    }

    // Take field
    pub fn take_enabled_hero_id(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.enabled_hero_id, ::std::vec::Vec::new())
    }

    pub fn get_enabled_hero_id(&self) -> &[u32] {
        &self.enabled_hero_id
    }

    fn get_enabled_hero_id_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.enabled_hero_id
    }

    fn mut_enabled_hero_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.enabled_hero_id
    }

    // optional .dota.DOTA_GC_TEAM coach_team = 23;

    pub fn clear_coach_team(&mut self) {
        self.coach_team = ::std::option::Option::None;
    }

    pub fn has_coach_team(&self) -> bool {
        self.coach_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_coach_team(&mut self, v: super::dota_shared_enums::DOTA_GC_TEAM) {
        self.coach_team = ::std::option::Option::Some(v);
    }

    pub fn get_coach_team(&self) -> super::dota_shared_enums::DOTA_GC_TEAM {
        self.coach_team.unwrap_or(super::dota_shared_enums::DOTA_GC_TEAM::DOTA_GC_TEAM_NOTEAM)
    }

    fn get_coach_team_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM> {
        &self.coach_team
    }

    fn mut_coach_team_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GC_TEAM> {
        &mut self.coach_team
    }

    // optional uint32 nexon_pc_bang_no = 24;

    pub fn clear_nexon_pc_bang_no(&mut self) {
        self.nexon_pc_bang_no = ::std::option::Option::None;
    }

    pub fn has_nexon_pc_bang_no(&self) -> bool {
        self.nexon_pc_bang_no.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nexon_pc_bang_no(&mut self, v: u32) {
        self.nexon_pc_bang_no = ::std::option::Option::Some(v);
    }

    pub fn get_nexon_pc_bang_no(&self) -> u32 {
        self.nexon_pc_bang_no.unwrap_or(0)
    }

    fn get_nexon_pc_bang_no_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.nexon_pc_bang_no
    }

    fn mut_nexon_pc_bang_no_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.nexon_pc_bang_no
    }

    // optional string nexon_pc_bang_name = 25;

    pub fn clear_nexon_pc_bang_name(&mut self) {
        self.nexon_pc_bang_name.clear();
    }

    pub fn has_nexon_pc_bang_name(&self) -> bool {
        self.nexon_pc_bang_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nexon_pc_bang_name(&mut self, v: ::std::string::String) {
        self.nexon_pc_bang_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nexon_pc_bang_name(&mut self) -> &mut ::std::string::String {
        if self.nexon_pc_bang_name.is_none() {
            self.nexon_pc_bang_name.set_default();
        };
        self.nexon_pc_bang_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_nexon_pc_bang_name(&mut self) -> ::std::string::String {
        self.nexon_pc_bang_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_nexon_pc_bang_name(&self) -> &str {
        match self.nexon_pc_bang_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_nexon_pc_bang_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.nexon_pc_bang_name
    }

    fn mut_nexon_pc_bang_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.nexon_pc_bang_name
    }

    // repeated .dota.CDOTALobbyMember.CDOTALobbyMemberXPBonus xp_bonuses = 27;

    pub fn clear_xp_bonuses(&mut self) {
        self.xp_bonuses.clear();
    }

    // Param is passed by value, moved
    pub fn set_xp_bonuses(&mut self, v: ::protobuf::RepeatedField<CDOTALobbyMember_CDOTALobbyMemberXPBonus>) {
        self.xp_bonuses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xp_bonuses(&mut self) -> &mut ::protobuf::RepeatedField<CDOTALobbyMember_CDOTALobbyMemberXPBonus> {
        &mut self.xp_bonuses
    }

    // Take field
    pub fn take_xp_bonuses(&mut self) -> ::protobuf::RepeatedField<CDOTALobbyMember_CDOTALobbyMemberXPBonus> {
        ::std::mem::replace(&mut self.xp_bonuses, ::protobuf::RepeatedField::new())
    }

    pub fn get_xp_bonuses(&self) -> &[CDOTALobbyMember_CDOTALobbyMemberXPBonus] {
        &self.xp_bonuses
    }

    fn get_xp_bonuses_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTALobbyMember_CDOTALobbyMemberXPBonus> {
        &self.xp_bonuses
    }

    fn mut_xp_bonuses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTALobbyMember_CDOTALobbyMemberXPBonus> {
        &mut self.xp_bonuses
    }

    // optional sint32 rank_change = 29;

    pub fn clear_rank_change(&mut self) {
        self.rank_change = ::std::option::Option::None;
    }

    pub fn has_rank_change(&self) -> bool {
        self.rank_change.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank_change(&mut self, v: i32) {
        self.rank_change = ::std::option::Option::Some(v);
    }

    pub fn get_rank_change(&self) -> i32 {
        self.rank_change.unwrap_or(0)
    }

    fn get_rank_change_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.rank_change
    }

    fn mut_rank_change_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.rank_change
    }

    // optional bool cameraman = 30;

    pub fn clear_cameraman(&mut self) {
        self.cameraman = ::std::option::Option::None;
    }

    pub fn has_cameraman(&self) -> bool {
        self.cameraman.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cameraman(&mut self, v: bool) {
        self.cameraman = ::std::option::Option::Some(v);
    }

    pub fn get_cameraman(&self) -> bool {
        self.cameraman.unwrap_or(false)
    }

    fn get_cameraman_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.cameraman
    }

    fn mut_cameraman_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.cameraman
    }

    // repeated uint32 custom_game_product_ids = 31;

    pub fn clear_custom_game_product_ids(&mut self) {
        self.custom_game_product_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_custom_game_product_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.custom_game_product_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_custom_game_product_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.custom_game_product_ids
    }

    // Take field
    pub fn take_custom_game_product_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.custom_game_product_ids, ::std::vec::Vec::new())
    }

    pub fn get_custom_game_product_ids(&self) -> &[u32] {
        &self.custom_game_product_ids
    }

    fn get_custom_game_product_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.custom_game_product_ids
    }

    fn mut_custom_game_product_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.custom_game_product_ids
    }

    // optional uint32 lobby_mvp_vote_account_id = 32;

    pub fn clear_lobby_mvp_vote_account_id(&mut self) {
        self.lobby_mvp_vote_account_id = ::std::option::Option::None;
    }

    pub fn has_lobby_mvp_vote_account_id(&self) -> bool {
        self.lobby_mvp_vote_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_mvp_vote_account_id(&mut self, v: u32) {
        self.lobby_mvp_vote_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_mvp_vote_account_id(&self) -> u32 {
        self.lobby_mvp_vote_account_id.unwrap_or(0)
    }

    fn get_lobby_mvp_vote_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lobby_mvp_vote_account_id
    }

    fn mut_lobby_mvp_vote_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lobby_mvp_vote_account_id
    }

    // optional .dota.MatchType search_match_type = 33;

    pub fn clear_search_match_type(&mut self) {
        self.search_match_type = ::std::option::Option::None;
    }

    pub fn has_search_match_type(&self) -> bool {
        self.search_match_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_match_type(&mut self, v: super::dota_shared_enums::MatchType) {
        self.search_match_type = ::std::option::Option::Some(v);
    }

    pub fn get_search_match_type(&self) -> super::dota_shared_enums::MatchType {
        self.search_match_type.unwrap_or(super::dota_shared_enums::MatchType::MATCH_TYPE_CASUAL)
    }

    fn get_search_match_type_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::MatchType> {
        &self.search_match_type
    }

    fn mut_search_match_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::MatchType> {
        &mut self.search_match_type
    }

    // optional uint32 favorite_team_and_quality = 34;

    pub fn clear_favorite_team_and_quality(&mut self) {
        self.favorite_team_and_quality = ::std::option::Option::None;
    }

    pub fn has_favorite_team_and_quality(&self) -> bool {
        self.favorite_team_and_quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_favorite_team_and_quality(&mut self, v: u32) {
        self.favorite_team_and_quality = ::std::option::Option::Some(v);
    }

    pub fn get_favorite_team_and_quality(&self) -> u32 {
        self.favorite_team_and_quality.unwrap_or(0)
    }

    fn get_favorite_team_and_quality_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.favorite_team_and_quality
    }

    fn mut_favorite_team_and_quality_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.favorite_team_and_quality
    }
}

impl ::protobuf::Message for CDOTALobbyMember {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.team = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.slot = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.party_id = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.meta_level = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.meta_xp = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.meta_xp_awarded = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.leaver_status = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.leaver_actions = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.channel = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.prize_def_index = ::std::option::Option::Some(tmp);
                },
                20 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.disabled_hero_id)?;
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.partner_account_type = ::std::option::Option::Some(tmp);
                },
                22 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.enabled_hero_id)?;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.coach_team = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.nexon_pc_bang_no = ::std::option::Option::Some(tmp);
                },
                25 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nexon_pc_bang_name)?;
                },
                27 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.xp_bonuses)?;
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint32()?;
                    self.rank_change = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.cameraman = ::std::option::Option::Some(tmp);
                },
                31 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.custom_game_product_ids)?;
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lobby_mvp_vote_account_id = ::std::option::Option::Some(tmp);
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.search_match_type = ::std::option::Option::Some(tmp);
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.favorite_team_and_quality = ::std::option::Option::Some(tmp);
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
            my_size += 9;
        };
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::enum_size(3, v);
        };
        if let Some(v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.party_id {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.meta_level {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.meta_xp {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.meta_xp_awarded {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.leaver_status {
            my_size += ::protobuf::rt::enum_size(16, v);
        };
        if let Some(v) = self.leaver_actions {
            my_size += ::protobuf::rt::value_size(28, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.channel {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.prize_def_index {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.disabled_hero_id {
            my_size += ::protobuf::rt::value_size(20, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.partner_account_type {
            my_size += ::protobuf::rt::enum_size(21, v);
        };
        for value in &self.enabled_hero_id {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.coach_team {
            my_size += ::protobuf::rt::enum_size(23, v);
        };
        if let Some(v) = self.nexon_pc_bang_no {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.nexon_pc_bang_name.as_ref() {
            my_size += ::protobuf::rt::string_size(25, &v);
        };
        for value in &self.xp_bonuses {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.rank_change {
            my_size += ::protobuf::rt::value_varint_zigzag_size(29, v);
        };
        if let Some(v) = self.cameraman {
            my_size += 3;
        };
        for value in &self.custom_game_product_ids {
            my_size += ::protobuf::rt::value_size(31, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lobby_mvp_vote_account_id {
            my_size += ::protobuf::rt::value_size(32, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.search_match_type {
            my_size += ::protobuf::rt::enum_size(33, v);
        };
        if let Some(v) = self.favorite_team_and_quality {
            my_size += ::protobuf::rt::value_size(34, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.hero_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.team {
            os.write_enum(3, v.value())?;
        };
        if let Some(v) = self.name.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.slot {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.party_id {
            os.write_uint64(12, v)?;
        };
        if let Some(v) = self.meta_level {
            os.write_uint32(13, v)?;
        };
        if let Some(v) = self.meta_xp {
            os.write_uint32(14, v)?;
        };
        if let Some(v) = self.meta_xp_awarded {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.leaver_status {
            os.write_enum(16, v.value())?;
        };
        if let Some(v) = self.leaver_actions {
            os.write_uint32(28, v)?;
        };
        if let Some(v) = self.channel {
            os.write_uint32(17, v)?;
        };
        if let Some(v) = self.prize_def_index {
            os.write_uint32(18, v)?;
        };
        for v in &self.disabled_hero_id {
            os.write_uint32(20, *v)?;
        };
        if let Some(v) = self.partner_account_type {
            os.write_enum(21, v.value())?;
        };
        for v in &self.enabled_hero_id {
            os.write_uint32(22, *v)?;
        };
        if let Some(v) = self.coach_team {
            os.write_enum(23, v.value())?;
        };
        if let Some(v) = self.nexon_pc_bang_no {
            os.write_uint32(24, v)?;
        };
        if let Some(v) = self.nexon_pc_bang_name.as_ref() {
            os.write_string(25, &v)?;
        };
        for v in &self.xp_bonuses {
            os.write_tag(27, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.rank_change {
            os.write_sint32(29, v)?;
        };
        if let Some(v) = self.cameraman {
            os.write_bool(30, v)?;
        };
        for v in &self.custom_game_product_ids {
            os.write_uint32(31, *v)?;
        };
        if let Some(v) = self.lobby_mvp_vote_account_id {
            os.write_uint32(32, v)?;
        };
        if let Some(v) = self.search_match_type {
            os.write_enum(33, v.value())?;
        };
        if let Some(v) = self.favorite_team_and_quality {
            os.write_uint32(34, v)?;
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

impl ::protobuf::MessageStatic for CDOTALobbyMember {
    fn new() -> CDOTALobbyMember {
        CDOTALobbyMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTALobbyMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "id",
                    CDOTALobbyMember::get_id_for_reflect,
                    CDOTALobbyMember::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CDOTALobbyMember::get_hero_id_for_reflect,
                    CDOTALobbyMember::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GC_TEAM>>(
                    "team",
                    CDOTALobbyMember::get_team_for_reflect,
                    CDOTALobbyMember::mut_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CDOTALobbyMember::get_name_for_reflect,
                    CDOTALobbyMember::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "slot",
                    CDOTALobbyMember::get_slot_for_reflect,
                    CDOTALobbyMember::mut_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "party_id",
                    CDOTALobbyMember::get_party_id_for_reflect,
                    CDOTALobbyMember::mut_party_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "meta_level",
                    CDOTALobbyMember::get_meta_level_for_reflect,
                    CDOTALobbyMember::mut_meta_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "meta_xp",
                    CDOTALobbyMember::get_meta_xp_for_reflect,
                    CDOTALobbyMember::mut_meta_xp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "meta_xp_awarded",
                    CDOTALobbyMember::get_meta_xp_awarded_for_reflect,
                    CDOTALobbyMember::mut_meta_xp_awarded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTALeaverStatus_t>>(
                    "leaver_status",
                    CDOTALobbyMember::get_leaver_status_for_reflect,
                    CDOTALobbyMember::mut_leaver_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leaver_actions",
                    CDOTALobbyMember::get_leaver_actions_for_reflect,
                    CDOTALobbyMember::mut_leaver_actions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel",
                    CDOTALobbyMember::get_channel_for_reflect,
                    CDOTALobbyMember::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "prize_def_index",
                    CDOTALobbyMember::get_prize_def_index_for_reflect,
                    CDOTALobbyMember::mut_prize_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "disabled_hero_id",
                    CDOTALobbyMember::get_disabled_hero_id_for_reflect,
                    CDOTALobbyMember::mut_disabled_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::gcsdk_gcmessages::PartnerAccountType>>(
                    "partner_account_type",
                    CDOTALobbyMember::get_partner_account_type_for_reflect,
                    CDOTALobbyMember::mut_partner_account_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "enabled_hero_id",
                    CDOTALobbyMember::get_enabled_hero_id_for_reflect,
                    CDOTALobbyMember::mut_enabled_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GC_TEAM>>(
                    "coach_team",
                    CDOTALobbyMember::get_coach_team_for_reflect,
                    CDOTALobbyMember::mut_coach_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "nexon_pc_bang_no",
                    CDOTALobbyMember::get_nexon_pc_bang_no_for_reflect,
                    CDOTALobbyMember::mut_nexon_pc_bang_no_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nexon_pc_bang_name",
                    CDOTALobbyMember::get_nexon_pc_bang_name_for_reflect,
                    CDOTALobbyMember::mut_nexon_pc_bang_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTALobbyMember_CDOTALobbyMemberXPBonus>>(
                    "xp_bonuses",
                    CDOTALobbyMember::get_xp_bonuses_for_reflect,
                    CDOTALobbyMember::mut_xp_bonuses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "rank_change",
                    CDOTALobbyMember::get_rank_change_for_reflect,
                    CDOTALobbyMember::mut_rank_change_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "cameraman",
                    CDOTALobbyMember::get_cameraman_for_reflect,
                    CDOTALobbyMember::mut_cameraman_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "custom_game_product_ids",
                    CDOTALobbyMember::get_custom_game_product_ids_for_reflect,
                    CDOTALobbyMember::mut_custom_game_product_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lobby_mvp_vote_account_id",
                    CDOTALobbyMember::get_lobby_mvp_vote_account_id_for_reflect,
                    CDOTALobbyMember::mut_lobby_mvp_vote_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::MatchType>>(
                    "search_match_type",
                    CDOTALobbyMember::get_search_match_type_for_reflect,
                    CDOTALobbyMember::mut_search_match_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "favorite_team_and_quality",
                    CDOTALobbyMember::get_favorite_team_and_quality_for_reflect,
                    CDOTALobbyMember::mut_favorite_team_and_quality_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTALobbyMember>(
                    "CDOTALobbyMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTALobbyMember {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_hero_id();
        self.clear_team();
        self.clear_name();
        self.clear_slot();
        self.clear_party_id();
        self.clear_meta_level();
        self.clear_meta_xp();
        self.clear_meta_xp_awarded();
        self.clear_leaver_status();
        self.clear_leaver_actions();
        self.clear_channel();
        self.clear_prize_def_index();
        self.clear_disabled_hero_id();
        self.clear_partner_account_type();
        self.clear_enabled_hero_id();
        self.clear_coach_team();
        self.clear_nexon_pc_bang_no();
        self.clear_nexon_pc_bang_name();
        self.clear_xp_bonuses();
        self.clear_rank_change();
        self.clear_cameraman();
        self.clear_custom_game_product_ids();
        self.clear_lobby_mvp_vote_account_id();
        self.clear_search_match_type();
        self.clear_favorite_team_and_quality();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTALobbyMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTALobbyMember {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTALobbyMember_CDOTALobbyMemberXPBonus {
    // message fields
    field_type: ::std::option::Option<u32>,
    xp_bonus: ::std::option::Option<f32>,
    source_key: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTALobbyMember_CDOTALobbyMemberXPBonus {}

impl CDOTALobbyMember_CDOTALobbyMemberXPBonus {
    pub fn new() -> CDOTALobbyMember_CDOTALobbyMemberXPBonus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTALobbyMember_CDOTALobbyMemberXPBonus {
        static mut instance: ::protobuf::lazy::Lazy<CDOTALobbyMember_CDOTALobbyMemberXPBonus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTALobbyMember_CDOTALobbyMemberXPBonus,
        };
        unsafe {
            instance.get(CDOTALobbyMember_CDOTALobbyMemberXPBonus::new)
        }
    }

    // optional uint32 type = 1;

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

    // optional float xp_bonus = 2;

    pub fn clear_xp_bonus(&mut self) {
        self.xp_bonus = ::std::option::Option::None;
    }

    pub fn has_xp_bonus(&self) -> bool {
        self.xp_bonus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xp_bonus(&mut self, v: f32) {
        self.xp_bonus = ::std::option::Option::Some(v);
    }

    pub fn get_xp_bonus(&self) -> f32 {
        self.xp_bonus.unwrap_or(0.)
    }

    fn get_xp_bonus_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.xp_bonus
    }

    fn mut_xp_bonus_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.xp_bonus
    }

    // optional uint64 source_key = 3;

    pub fn clear_source_key(&mut self) {
        self.source_key = ::std::option::Option::None;
    }

    pub fn has_source_key(&self) -> bool {
        self.source_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_key(&mut self, v: u64) {
        self.source_key = ::std::option::Option::Some(v);
    }

    pub fn get_source_key(&self) -> u64 {
        self.source_key.unwrap_or(0)
    }

    fn get_source_key_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.source_key
    }

    fn mut_source_key_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.source_key
    }
}

impl ::protobuf::Message for CDOTALobbyMember_CDOTALobbyMemberXPBonus {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.xp_bonus = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.source_key = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.xp_bonus {
            my_size += 5;
        };
        if let Some(v) = self.source_key {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.xp_bonus {
            os.write_float(2, v)?;
        };
        if let Some(v) = self.source_key {
            os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for CDOTALobbyMember_CDOTALobbyMemberXPBonus {
    fn new() -> CDOTALobbyMember_CDOTALobbyMemberXPBonus {
        CDOTALobbyMember_CDOTALobbyMemberXPBonus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTALobbyMember_CDOTALobbyMemberXPBonus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "type",
                    CDOTALobbyMember_CDOTALobbyMemberXPBonus::get_field_type_for_reflect,
                    CDOTALobbyMember_CDOTALobbyMemberXPBonus::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "xp_bonus",
                    CDOTALobbyMember_CDOTALobbyMemberXPBonus::get_xp_bonus_for_reflect,
                    CDOTALobbyMember_CDOTALobbyMemberXPBonus::mut_xp_bonus_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "source_key",
                    CDOTALobbyMember_CDOTALobbyMemberXPBonus::get_source_key_for_reflect,
                    CDOTALobbyMember_CDOTALobbyMemberXPBonus::mut_source_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTALobbyMember_CDOTALobbyMemberXPBonus>(
                    "CDOTALobbyMember_CDOTALobbyMemberXPBonus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTALobbyMember_CDOTALobbyMemberXPBonus {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_xp_bonus();
        self.clear_source_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTALobbyMember_CDOTALobbyMemberXPBonus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTALobbyMember_CDOTALobbyMemberXPBonus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CLobbyTeamDetails {
    // message fields
    team_name: ::protobuf::SingularField<::std::string::String>,
    team_tag: ::protobuf::SingularField<::std::string::String>,
    team_id: ::std::option::Option<u32>,
    team_logo: ::std::option::Option<u64>,
    team_base_logo: ::std::option::Option<u64>,
    team_banner_logo: ::std::option::Option<u64>,
    team_complete: ::std::option::Option<bool>,
    guild_name: ::protobuf::SingularField<::std::string::String>,
    guild_tag: ::protobuf::SingularField<::std::string::String>,
    guild_id: ::std::option::Option<u32>,
    guild_logo: ::std::option::Option<u64>,
    guild_base_logo: ::std::option::Option<u64>,
    guild_banner_logo: ::std::option::Option<u64>,
    rank: ::std::option::Option<u32>,
    rank_change: ::std::option::Option<i32>,
    is_home_team: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CLobbyTeamDetails {}

impl CLobbyTeamDetails {
    pub fn new() -> CLobbyTeamDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CLobbyTeamDetails {
        static mut instance: ::protobuf::lazy::Lazy<CLobbyTeamDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CLobbyTeamDetails,
        };
        unsafe {
            instance.get(CLobbyTeamDetails::new)
        }
    }

    // optional string team_name = 1;

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

    // optional string team_tag = 3;

    pub fn clear_team_tag(&mut self) {
        self.team_tag.clear();
    }

    pub fn has_team_tag(&self) -> bool {
        self.team_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_tag(&mut self, v: ::std::string::String) {
        self.team_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_tag(&mut self) -> &mut ::std::string::String {
        if self.team_tag.is_none() {
            self.team_tag.set_default();
        };
        self.team_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_tag(&mut self) -> ::std::string::String {
        self.team_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_tag(&self) -> &str {
        match self.team_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_tag
    }

    fn mut_team_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_tag
    }

    // optional uint32 team_id = 4;

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

    // optional uint64 team_logo = 5;

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

    // optional uint64 team_base_logo = 6;

    pub fn clear_team_base_logo(&mut self) {
        self.team_base_logo = ::std::option::Option::None;
    }

    pub fn has_team_base_logo(&self) -> bool {
        self.team_base_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_base_logo(&mut self, v: u64) {
        self.team_base_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team_base_logo(&self) -> u64 {
        self.team_base_logo.unwrap_or(0)
    }

    fn get_team_base_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_base_logo
    }

    fn mut_team_base_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_base_logo
    }

    // optional uint64 team_banner_logo = 7;

    pub fn clear_team_banner_logo(&mut self) {
        self.team_banner_logo = ::std::option::Option::None;
    }

    pub fn has_team_banner_logo(&self) -> bool {
        self.team_banner_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_banner_logo(&mut self, v: u64) {
        self.team_banner_logo = ::std::option::Option::Some(v);
    }

    pub fn get_team_banner_logo(&self) -> u64 {
        self.team_banner_logo.unwrap_or(0)
    }

    fn get_team_banner_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_banner_logo
    }

    fn mut_team_banner_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_banner_logo
    }

    // optional bool team_complete = 8;

    pub fn clear_team_complete(&mut self) {
        self.team_complete = ::std::option::Option::None;
    }

    pub fn has_team_complete(&self) -> bool {
        self.team_complete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_complete(&mut self, v: bool) {
        self.team_complete = ::std::option::Option::Some(v);
    }

    pub fn get_team_complete(&self) -> bool {
        self.team_complete.unwrap_or(false)
    }

    fn get_team_complete_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.team_complete
    }

    fn mut_team_complete_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.team_complete
    }

    // optional string guild_name = 9;

    pub fn clear_guild_name(&mut self) {
        self.guild_name.clear();
    }

    pub fn has_guild_name(&self) -> bool {
        self.guild_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_name(&mut self, v: ::std::string::String) {
        self.guild_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guild_name(&mut self) -> &mut ::std::string::String {
        if self.guild_name.is_none() {
            self.guild_name.set_default();
        };
        self.guild_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_guild_name(&mut self) -> ::std::string::String {
        self.guild_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_guild_name(&self) -> &str {
        match self.guild_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_guild_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.guild_name
    }

    fn mut_guild_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.guild_name
    }

    // optional string guild_tag = 10;

    pub fn clear_guild_tag(&mut self) {
        self.guild_tag.clear();
    }

    pub fn has_guild_tag(&self) -> bool {
        self.guild_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_tag(&mut self, v: ::std::string::String) {
        self.guild_tag = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_guild_tag(&mut self) -> &mut ::std::string::String {
        if self.guild_tag.is_none() {
            self.guild_tag.set_default();
        };
        self.guild_tag.as_mut().unwrap()
    }

    // Take field
    pub fn take_guild_tag(&mut self) -> ::std::string::String {
        self.guild_tag.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_guild_tag(&self) -> &str {
        match self.guild_tag.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_guild_tag_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.guild_tag
    }

    fn mut_guild_tag_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.guild_tag
    }

    // optional uint32 guild_id = 11;

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

    // optional uint64 guild_logo = 12;

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

    // optional uint64 guild_base_logo = 13;

    pub fn clear_guild_base_logo(&mut self) {
        self.guild_base_logo = ::std::option::Option::None;
    }

    pub fn has_guild_base_logo(&self) -> bool {
        self.guild_base_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_base_logo(&mut self, v: u64) {
        self.guild_base_logo = ::std::option::Option::Some(v);
    }

    pub fn get_guild_base_logo(&self) -> u64 {
        self.guild_base_logo.unwrap_or(0)
    }

    fn get_guild_base_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.guild_base_logo
    }

    fn mut_guild_base_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.guild_base_logo
    }

    // optional uint64 guild_banner_logo = 14;

    pub fn clear_guild_banner_logo(&mut self) {
        self.guild_banner_logo = ::std::option::Option::None;
    }

    pub fn has_guild_banner_logo(&self) -> bool {
        self.guild_banner_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_guild_banner_logo(&mut self, v: u64) {
        self.guild_banner_logo = ::std::option::Option::Some(v);
    }

    pub fn get_guild_banner_logo(&self) -> u64 {
        self.guild_banner_logo.unwrap_or(0)
    }

    fn get_guild_banner_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.guild_banner_logo
    }

    fn mut_guild_banner_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.guild_banner_logo
    }

    // optional uint32 rank = 15;

    pub fn clear_rank(&mut self) {
        self.rank = ::std::option::Option::None;
    }

    pub fn has_rank(&self) -> bool {
        self.rank.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank(&mut self, v: u32) {
        self.rank = ::std::option::Option::Some(v);
    }

    pub fn get_rank(&self) -> u32 {
        self.rank.unwrap_or(0)
    }

    fn get_rank_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.rank
    }

    fn mut_rank_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.rank
    }

    // optional sint32 rank_change = 16;

    pub fn clear_rank_change(&mut self) {
        self.rank_change = ::std::option::Option::None;
    }

    pub fn has_rank_change(&self) -> bool {
        self.rank_change.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank_change(&mut self, v: i32) {
        self.rank_change = ::std::option::Option::Some(v);
    }

    pub fn get_rank_change(&self) -> i32 {
        self.rank_change.unwrap_or(0)
    }

    fn get_rank_change_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.rank_change
    }

    fn mut_rank_change_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.rank_change
    }

    // optional bool is_home_team = 17;

    pub fn clear_is_home_team(&mut self) {
        self.is_home_team = ::std::option::Option::None;
    }

    pub fn has_is_home_team(&self) -> bool {
        self.is_home_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_home_team(&mut self, v: bool) {
        self.is_home_team = ::std::option::Option::Some(v);
    }

    pub fn get_is_home_team(&self) -> bool {
        self.is_home_team.unwrap_or(false)
    }

    fn get_is_home_team_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_home_team
    }

    fn mut_is_home_team_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_home_team
    }
}

impl ::protobuf::Message for CLobbyTeamDetails {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_tag)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.team_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.team_logo = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.team_base_logo = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.team_banner_logo = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.team_complete = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.guild_name)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.guild_tag)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.guild_id = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.guild_logo = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.guild_base_logo = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.guild_banner_logo = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.rank = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_sint32()?;
                    self.rank_change = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_home_team = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.team_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_logo {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_base_logo {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_banner_logo {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_complete {
            my_size += 2;
        };
        if let Some(v) = self.guild_name.as_ref() {
            my_size += ::protobuf::rt::string_size(9, &v);
        };
        if let Some(v) = self.guild_tag.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        };
        if let Some(v) = self.guild_id {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.guild_logo {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.guild_base_logo {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.guild_banner_logo {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.rank {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.rank_change {
            my_size += ::protobuf::rt::value_varint_zigzag_size(16, v);
        };
        if let Some(v) = self.is_home_team {
            my_size += 3;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.team_tag.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.team_id {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.team_logo {
            os.write_uint64(5, v)?;
        };
        if let Some(v) = self.team_base_logo {
            os.write_uint64(6, v)?;
        };
        if let Some(v) = self.team_banner_logo {
            os.write_uint64(7, v)?;
        };
        if let Some(v) = self.team_complete {
            os.write_bool(8, v)?;
        };
        if let Some(v) = self.guild_name.as_ref() {
            os.write_string(9, &v)?;
        };
        if let Some(v) = self.guild_tag.as_ref() {
            os.write_string(10, &v)?;
        };
        if let Some(v) = self.guild_id {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.guild_logo {
            os.write_uint64(12, v)?;
        };
        if let Some(v) = self.guild_base_logo {
            os.write_uint64(13, v)?;
        };
        if let Some(v) = self.guild_banner_logo {
            os.write_uint64(14, v)?;
        };
        if let Some(v) = self.rank {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.rank_change {
            os.write_sint32(16, v)?;
        };
        if let Some(v) = self.is_home_team {
            os.write_bool(17, v)?;
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

impl ::protobuf::MessageStatic for CLobbyTeamDetails {
    fn new() -> CLobbyTeamDetails {
        CLobbyTeamDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CLobbyTeamDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name",
                    CLobbyTeamDetails::get_team_name_for_reflect,
                    CLobbyTeamDetails::mut_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_tag",
                    CLobbyTeamDetails::get_team_tag_for_reflect,
                    CLobbyTeamDetails::mut_team_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CLobbyTeamDetails::get_team_id_for_reflect,
                    CLobbyTeamDetails::mut_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_logo",
                    CLobbyTeamDetails::get_team_logo_for_reflect,
                    CLobbyTeamDetails::mut_team_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_base_logo",
                    CLobbyTeamDetails::get_team_base_logo_for_reflect,
                    CLobbyTeamDetails::mut_team_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "team_banner_logo",
                    CLobbyTeamDetails::get_team_banner_logo_for_reflect,
                    CLobbyTeamDetails::mut_team_banner_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "team_complete",
                    CLobbyTeamDetails::get_team_complete_for_reflect,
                    CLobbyTeamDetails::mut_team_complete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "guild_name",
                    CLobbyTeamDetails::get_guild_name_for_reflect,
                    CLobbyTeamDetails::mut_guild_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "guild_tag",
                    CLobbyTeamDetails::get_guild_tag_for_reflect,
                    CLobbyTeamDetails::mut_guild_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "guild_id",
                    CLobbyTeamDetails::get_guild_id_for_reflect,
                    CLobbyTeamDetails::mut_guild_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "guild_logo",
                    CLobbyTeamDetails::get_guild_logo_for_reflect,
                    CLobbyTeamDetails::mut_guild_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "guild_base_logo",
                    CLobbyTeamDetails::get_guild_base_logo_for_reflect,
                    CLobbyTeamDetails::mut_guild_base_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "guild_banner_logo",
                    CLobbyTeamDetails::get_guild_banner_logo_for_reflect,
                    CLobbyTeamDetails::mut_guild_banner_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "rank",
                    CLobbyTeamDetails::get_rank_for_reflect,
                    CLobbyTeamDetails::mut_rank_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "rank_change",
                    CLobbyTeamDetails::get_rank_change_for_reflect,
                    CLobbyTeamDetails::mut_rank_change_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_home_team",
                    CLobbyTeamDetails::get_is_home_team_for_reflect,
                    CLobbyTeamDetails::mut_is_home_team_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CLobbyTeamDetails>(
                    "CLobbyTeamDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CLobbyTeamDetails {
    fn clear(&mut self) {
        self.clear_team_name();
        self.clear_team_tag();
        self.clear_team_id();
        self.clear_team_logo();
        self.clear_team_base_logo();
        self.clear_team_banner_logo();
        self.clear_team_complete();
        self.clear_guild_name();
        self.clear_guild_tag();
        self.clear_guild_id();
        self.clear_guild_logo();
        self.clear_guild_base_logo();
        self.clear_guild_banner_logo();
        self.clear_rank();
        self.clear_rank_change();
        self.clear_is_home_team();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CLobbyTeamDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CLobbyTeamDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CLobbyTimedRewardDetails {
    // message fields
    item_def_index: ::std::option::Option<u32>,
    is_supply_crate: ::std::option::Option<bool>,
    is_timed_drop: ::std::option::Option<bool>,
    account_id: ::std::option::Option<u32>,
    origin: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CLobbyTimedRewardDetails {}

impl CLobbyTimedRewardDetails {
    pub fn new() -> CLobbyTimedRewardDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CLobbyTimedRewardDetails {
        static mut instance: ::protobuf::lazy::Lazy<CLobbyTimedRewardDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CLobbyTimedRewardDetails,
        };
        unsafe {
            instance.get(CLobbyTimedRewardDetails::new)
        }
    }

    // optional uint32 item_def_index = 2;

    pub fn clear_item_def_index(&mut self) {
        self.item_def_index = ::std::option::Option::None;
    }

    pub fn has_item_def_index(&self) -> bool {
        self.item_def_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_def_index(&mut self, v: u32) {
        self.item_def_index = ::std::option::Option::Some(v);
    }

    pub fn get_item_def_index(&self) -> u32 {
        self.item_def_index.unwrap_or(0)
    }

    fn get_item_def_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_def_index
    }

    fn mut_item_def_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_def_index
    }

    // optional bool is_supply_crate = 3;

    pub fn clear_is_supply_crate(&mut self) {
        self.is_supply_crate = ::std::option::Option::None;
    }

    pub fn has_is_supply_crate(&self) -> bool {
        self.is_supply_crate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_supply_crate(&mut self, v: bool) {
        self.is_supply_crate = ::std::option::Option::Some(v);
    }

    pub fn get_is_supply_crate(&self) -> bool {
        self.is_supply_crate.unwrap_or(false)
    }

    fn get_is_supply_crate_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_supply_crate
    }

    fn mut_is_supply_crate_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_supply_crate
    }

    // optional bool is_timed_drop = 4;

    pub fn clear_is_timed_drop(&mut self) {
        self.is_timed_drop = ::std::option::Option::None;
    }

    pub fn has_is_timed_drop(&self) -> bool {
        self.is_timed_drop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_timed_drop(&mut self, v: bool) {
        self.is_timed_drop = ::std::option::Option::Some(v);
    }

    pub fn get_is_timed_drop(&self) -> bool {
        self.is_timed_drop.unwrap_or(false)
    }

    fn get_is_timed_drop_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_timed_drop
    }

    fn mut_is_timed_drop_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_timed_drop
    }

    // optional uint32 account_id = 5;

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

    // optional uint32 origin = 6;

    pub fn clear_origin(&mut self) {
        self.origin = ::std::option::Option::None;
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: u32) {
        self.origin = ::std::option::Option::Some(v);
    }

    pub fn get_origin(&self) -> u32 {
        self.origin.unwrap_or(0)
    }

    fn get_origin_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.origin
    }
}

impl ::protobuf::Message for CLobbyTimedRewardDetails {
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
                    let tmp = is.read_uint32()?;
                    self.item_def_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_supply_crate = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_timed_drop = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.origin = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.item_def_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.is_supply_crate {
            my_size += 2;
        };
        if let Some(v) = self.is_timed_drop {
            my_size += 2;
        };
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.origin {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_def_index {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.is_supply_crate {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.is_timed_drop {
            os.write_bool(4, v)?;
        };
        if let Some(v) = self.account_id {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.origin {
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

impl ::protobuf::MessageStatic for CLobbyTimedRewardDetails {
    fn new() -> CLobbyTimedRewardDetails {
        CLobbyTimedRewardDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CLobbyTimedRewardDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_def_index",
                    CLobbyTimedRewardDetails::get_item_def_index_for_reflect,
                    CLobbyTimedRewardDetails::mut_item_def_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_supply_crate",
                    CLobbyTimedRewardDetails::get_is_supply_crate_for_reflect,
                    CLobbyTimedRewardDetails::mut_is_supply_crate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_timed_drop",
                    CLobbyTimedRewardDetails::get_is_timed_drop_for_reflect,
                    CLobbyTimedRewardDetails::mut_is_timed_drop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CLobbyTimedRewardDetails::get_account_id_for_reflect,
                    CLobbyTimedRewardDetails::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "origin",
                    CLobbyTimedRewardDetails::get_origin_for_reflect,
                    CLobbyTimedRewardDetails::mut_origin_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CLobbyTimedRewardDetails>(
                    "CLobbyTimedRewardDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CLobbyTimedRewardDetails {
    fn clear(&mut self) {
        self.clear_item_def_index();
        self.clear_is_supply_crate();
        self.clear_is_timed_drop();
        self.clear_account_id();
        self.clear_origin();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CLobbyTimedRewardDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CLobbyTimedRewardDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CLobbyBroadcastChannelInfo {
    // message fields
    channel_id: ::std::option::Option<u32>,
    country_code: ::protobuf::SingularField<::std::string::String>,
    description: ::protobuf::SingularField<::std::string::String>,
    language_code: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CLobbyBroadcastChannelInfo {}

impl CLobbyBroadcastChannelInfo {
    pub fn new() -> CLobbyBroadcastChannelInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CLobbyBroadcastChannelInfo {
        static mut instance: ::protobuf::lazy::Lazy<CLobbyBroadcastChannelInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CLobbyBroadcastChannelInfo,
        };
        unsafe {
            instance.get(CLobbyBroadcastChannelInfo::new)
        }
    }

    // optional uint32 channel_id = 1;

    pub fn clear_channel_id(&mut self) {
        self.channel_id = ::std::option::Option::None;
    }

    pub fn has_channel_id(&self) -> bool {
        self.channel_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_id(&mut self, v: u32) {
        self.channel_id = ::std::option::Option::Some(v);
    }

    pub fn get_channel_id(&self) -> u32 {
        self.channel_id.unwrap_or(0)
    }

    fn get_channel_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.channel_id
    }

    fn mut_channel_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.channel_id
    }

    // optional string country_code = 2;

    pub fn clear_country_code(&mut self) {
        self.country_code.clear();
    }

    pub fn has_country_code(&self) -> bool {
        self.country_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country_code(&mut self, v: ::std::string::String) {
        self.country_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country_code(&mut self) -> &mut ::std::string::String {
        if self.country_code.is_none() {
            self.country_code.set_default();
        };
        self.country_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_country_code(&mut self) -> ::std::string::String {
        self.country_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country_code(&self) -> &str {
        match self.country_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country_code
    }

    fn mut_country_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country_code
    }

    // optional string description = 3;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        };
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.description
    }

    // optional string language_code = 4;

    pub fn clear_language_code(&mut self) {
        self.language_code.clear();
    }

    pub fn has_language_code(&self) -> bool {
        self.language_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_language_code(&mut self, v: ::std::string::String) {
        self.language_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_language_code(&mut self) -> &mut ::std::string::String {
        if self.language_code.is_none() {
            self.language_code.set_default();
        };
        self.language_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_language_code(&mut self) -> ::std::string::String {
        self.language_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_language_code(&self) -> &str {
        match self.language_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_language_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.language_code
    }

    fn mut_language_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.language_code
    }
}

impl ::protobuf::Message for CLobbyBroadcastChannelInfo {
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
                    self.channel_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country_code)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.language_code)?;
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
        if let Some(v) = self.country_code.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.language_code.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.country_code.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.description.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.language_code.as_ref() {
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

impl ::protobuf::MessageStatic for CLobbyBroadcastChannelInfo {
    fn new() -> CLobbyBroadcastChannelInfo {
        CLobbyBroadcastChannelInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CLobbyBroadcastChannelInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "channel_id",
                    CLobbyBroadcastChannelInfo::get_channel_id_for_reflect,
                    CLobbyBroadcastChannelInfo::mut_channel_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country_code",
                    CLobbyBroadcastChannelInfo::get_country_code_for_reflect,
                    CLobbyBroadcastChannelInfo::mut_country_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    CLobbyBroadcastChannelInfo::get_description_for_reflect,
                    CLobbyBroadcastChannelInfo::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "language_code",
                    CLobbyBroadcastChannelInfo::get_language_code_for_reflect,
                    CLobbyBroadcastChannelInfo::mut_language_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CLobbyBroadcastChannelInfo>(
                    "CLobbyBroadcastChannelInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CLobbyBroadcastChannelInfo {
    fn clear(&mut self) {
        self.clear_channel_id();
        self.clear_country_code();
        self.clear_description();
        self.clear_language_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CLobbyBroadcastChannelInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CLobbyBroadcastChannelInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSODOTALobby {
    // message fields
    lobby_id: ::std::option::Option<u64>,
    members: ::protobuf::RepeatedField<CDOTALobbyMember>,
    left_members: ::protobuf::RepeatedField<CDOTALobbyMember>,
    leader_id: ::std::option::Option<u64>,
    server_id: ::std::option::Option<u64>,
    game_mode: ::std::option::Option<u32>,
    pending_invites: ::std::vec::Vec<u64>,
    state: ::std::option::Option<CSODOTALobby_State>,
    connect: ::protobuf::SingularField<::std::string::String>,
    lobby_type: ::std::option::Option<CSODOTALobby_LobbyType>,
    allow_cheats: ::std::option::Option<bool>,
    fill_with_bots: ::std::option::Option<bool>,
    intro_mode: ::std::option::Option<bool>,
    game_name: ::protobuf::SingularField<::std::string::String>,
    team_details: ::protobuf::RepeatedField<CLobbyTeamDetails>,
    tutorial_lesson: ::std::option::Option<u32>,
    tournament_id: ::std::option::Option<u32>,
    tournament_game_id: ::std::option::Option<u32>,
    server_region: ::std::option::Option<u32>,
    game_state: ::std::option::Option<super::dota_shared_enums::DOTA_GameState>,
    num_spectators: ::std::option::Option<u32>,
    matchgroup: ::std::option::Option<u32>,
    cm_pick: ::std::option::Option<super::dota_shared_enums::DOTA_CM_PICK>,
    match_id: ::std::option::Option<u64>,
    allow_spectating: ::std::option::Option<bool>,
    bot_difficulty_radiant: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    game_version: ::std::option::Option<super::dota_shared_enums::DOTAGameVersion>,
    timed_reward_details: ::protobuf::RepeatedField<CLobbyTimedRewardDetails>,
    pass_key: ::protobuf::SingularField<::std::string::String>,
    leagueid: ::std::option::Option<u32>,
    penalty_level_radiant: ::std::option::Option<u32>,
    penalty_level_dire: ::std::option::Option<u32>,
    load_game_id: ::std::option::Option<u32>,
    series_type: ::std::option::Option<u32>,
    radiant_series_wins: ::std::option::Option<u32>,
    dire_series_wins: ::std::option::Option<u32>,
    loot_generated: ::std::option::Option<u32>,
    loot_awarded: ::std::option::Option<u32>,
    allchat: ::std::option::Option<bool>,
    dota_tv_delay: ::std::option::Option<LobbyDotaTVDelay>,
    custom_game_mode: ::protobuf::SingularField<::std::string::String>,
    custom_map_name: ::protobuf::SingularField<::std::string::String>,
    custom_difficulty: ::std::option::Option<u32>,
    lan: ::std::option::Option<bool>,
    broadcast_channel_info: ::protobuf::RepeatedField<CLobbyBroadcastChannelInfo>,
    first_leaver_accountid: ::std::option::Option<u32>,
    series_id: ::std::option::Option<u32>,
    low_priority: ::std::option::Option<bool>,
    extra_messages: ::protobuf::RepeatedField<CSODOTALobby_CExtraMsg>,
    save_game: ::protobuf::SingularPtrField<super::dota_shared_enums::CDOTASaveGame>,
    first_blood_happened: ::std::option::Option<bool>,
    match_outcome: ::std::option::Option<super::dota_shared_enums::EMatchOutcome>,
    mass_disconnect: ::std::option::Option<bool>,
    custom_game_id: ::std::option::Option<u64>,
    active_ingame_events: ::std::vec::Vec<super::dota_shared_enums::EEvent>,
    custom_min_players: ::std::option::Option<u32>,
    custom_max_players: ::std::option::Option<u32>,
    partner_type: ::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType>,
    lan_host_ping_to_server_region: ::std::option::Option<u32>,
    visibility: ::std::option::Option<super::dota_shared_enums::DOTALobbyVisibility>,
    custom_game_crc: ::std::option::Option<u64>,
    custom_game_auto_created_lobby: ::std::option::Option<bool>,
    league_series_id: ::std::option::Option<u32>,
    league_game_id: ::std::option::Option<u32>,
    custom_game_timestamp: ::std::option::Option<u32>,
    previous_series_matches: ::std::vec::Vec<u64>,
    previous_match_override: ::std::option::Option<u64>,
    custom_game_uses_account_records: ::std::option::Option<bool>,
    league_selection_priority_team: ::std::option::Option<u32>,
    league_selection_priority_choice: ::std::option::Option<super::dota_shared_enums::SelectionPriorityType>,
    league_non_selection_priority_choice: ::std::option::Option<super::dota_shared_enums::SelectionPriorityType>,
    game_start_time: ::std::option::Option<u32>,
    pause_setting: ::std::option::Option<LobbyDotaPauseSetting>,
    lobby_mvp_account_id: ::std::option::Option<u32>,
    weekend_tourney_division_id: ::std::option::Option<u32>,
    weekend_tourney_skill_level: ::std::option::Option<u32>,
    weekend_tourney_bracket_round: ::std::option::Option<u32>,
    bot_difficulty_dire: ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty>,
    bot_radiant: ::std::option::Option<u64>,
    bot_dire: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSODOTALobby {}

impl CSODOTALobby {
    pub fn new() -> CSODOTALobby {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSODOTALobby {
        static mut instance: ::protobuf::lazy::Lazy<CSODOTALobby> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSODOTALobby,
        };
        unsafe {
            instance.get(CSODOTALobby::new)
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

    // repeated .dota.CDOTALobbyMember members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<CDOTALobbyMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<CDOTALobbyMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<CDOTALobbyMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[CDOTALobbyMember] {
        &self.members
    }

    fn get_members_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTALobbyMember> {
        &self.members
    }

    fn mut_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTALobbyMember> {
        &mut self.members
    }

    // repeated .dota.CDOTALobbyMember left_members = 7;

    pub fn clear_left_members(&mut self) {
        self.left_members.clear();
    }

    // Param is passed by value, moved
    pub fn set_left_members(&mut self, v: ::protobuf::RepeatedField<CDOTALobbyMember>) {
        self.left_members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_left_members(&mut self) -> &mut ::protobuf::RepeatedField<CDOTALobbyMember> {
        &mut self.left_members
    }

    // Take field
    pub fn take_left_members(&mut self) -> ::protobuf::RepeatedField<CDOTALobbyMember> {
        ::std::mem::replace(&mut self.left_members, ::protobuf::RepeatedField::new())
    }

    pub fn get_left_members(&self) -> &[CDOTALobbyMember] {
        &self.left_members
    }

    fn get_left_members_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTALobbyMember> {
        &self.left_members
    }

    fn mut_left_members_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTALobbyMember> {
        &mut self.left_members
    }

    // optional fixed64 leader_id = 11;

    pub fn clear_leader_id(&mut self) {
        self.leader_id = ::std::option::Option::None;
    }

    pub fn has_leader_id(&self) -> bool {
        self.leader_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_id(&mut self, v: u64) {
        self.leader_id = ::std::option::Option::Some(v);
    }

    pub fn get_leader_id(&self) -> u64 {
        self.leader_id.unwrap_or(0)
    }

    fn get_leader_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.leader_id
    }

    fn mut_leader_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.leader_id
    }

    // optional fixed64 server_id = 6;

    pub fn clear_server_id(&mut self) {
        self.server_id = ::std::option::Option::None;
    }

    pub fn has_server_id(&self) -> bool {
        self.server_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_id(&mut self, v: u64) {
        self.server_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_id(&self) -> u64 {
        self.server_id.unwrap_or(0u64)
    }

    fn get_server_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.server_id
    }

    fn mut_server_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.server_id
    }

    // optional uint32 game_mode = 3;

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

    // repeated fixed64 pending_invites = 10;

    pub fn clear_pending_invites(&mut self) {
        self.pending_invites.clear();
    }

    // Param is passed by value, moved
    pub fn set_pending_invites(&mut self, v: ::std::vec::Vec<u64>) {
        self.pending_invites = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pending_invites(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.pending_invites
    }

    // Take field
    pub fn take_pending_invites(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.pending_invites, ::std::vec::Vec::new())
    }

    pub fn get_pending_invites(&self) -> &[u64] {
        &self.pending_invites
    }

    fn get_pending_invites_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.pending_invites
    }

    fn mut_pending_invites_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.pending_invites
    }

    // optional .dota.CSODOTALobby.State state = 4;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: CSODOTALobby_State) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> CSODOTALobby_State {
        self.state.unwrap_or(CSODOTALobby_State::UI)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<CSODOTALobby_State> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<CSODOTALobby_State> {
        &mut self.state
    }

    // optional string connect = 5;

    pub fn clear_connect(&mut self) {
        self.connect.clear();
    }

    pub fn has_connect(&self) -> bool {
        self.connect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connect(&mut self, v: ::std::string::String) {
        self.connect = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connect(&mut self) -> &mut ::std::string::String {
        if self.connect.is_none() {
            self.connect.set_default();
        };
        self.connect.as_mut().unwrap()
    }

    // Take field
    pub fn take_connect(&mut self) -> ::std::string::String {
        self.connect.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_connect(&self) -> &str {
        match self.connect.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_connect_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.connect
    }

    fn mut_connect_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.connect
    }

    // optional .dota.CSODOTALobby.LobbyType lobby_type = 12;

    pub fn clear_lobby_type(&mut self) {
        self.lobby_type = ::std::option::Option::None;
    }

    pub fn has_lobby_type(&self) -> bool {
        self.lobby_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_type(&mut self, v: CSODOTALobby_LobbyType) {
        self.lobby_type = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_type(&self) -> CSODOTALobby_LobbyType {
        self.lobby_type.unwrap_or(CSODOTALobby_LobbyType::INVALID)
    }

    fn get_lobby_type_for_reflect(&self) -> &::std::option::Option<CSODOTALobby_LobbyType> {
        &self.lobby_type
    }

    fn mut_lobby_type_for_reflect(&mut self) -> &mut ::std::option::Option<CSODOTALobby_LobbyType> {
        &mut self.lobby_type
    }

    // optional bool allow_cheats = 13;

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

    // optional bool fill_with_bots = 14;

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

    // optional bool intro_mode = 15;

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

    // optional string game_name = 16;

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

    // repeated .dota.CLobbyTeamDetails team_details = 17;

    pub fn clear_team_details(&mut self) {
        self.team_details.clear();
    }

    // Param is passed by value, moved
    pub fn set_team_details(&mut self, v: ::protobuf::RepeatedField<CLobbyTeamDetails>) {
        self.team_details = v;
    }

    // Mutable pointer to the field.
    pub fn mut_team_details(&mut self) -> &mut ::protobuf::RepeatedField<CLobbyTeamDetails> {
        &mut self.team_details
    }

    // Take field
    pub fn take_team_details(&mut self) -> ::protobuf::RepeatedField<CLobbyTeamDetails> {
        ::std::mem::replace(&mut self.team_details, ::protobuf::RepeatedField::new())
    }

    pub fn get_team_details(&self) -> &[CLobbyTeamDetails] {
        &self.team_details
    }

    fn get_team_details_for_reflect(&self) -> &::protobuf::RepeatedField<CLobbyTeamDetails> {
        &self.team_details
    }

    fn mut_team_details_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CLobbyTeamDetails> {
        &mut self.team_details
    }

    // optional uint32 tutorial_lesson = 18;

    pub fn clear_tutorial_lesson(&mut self) {
        self.tutorial_lesson = ::std::option::Option::None;
    }

    pub fn has_tutorial_lesson(&self) -> bool {
        self.tutorial_lesson.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tutorial_lesson(&mut self, v: u32) {
        self.tutorial_lesson = ::std::option::Option::Some(v);
    }

    pub fn get_tutorial_lesson(&self) -> u32 {
        self.tutorial_lesson.unwrap_or(0)
    }

    fn get_tutorial_lesson_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tutorial_lesson
    }

    fn mut_tutorial_lesson_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tutorial_lesson
    }

    // optional uint32 tournament_id = 19;

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

    // optional uint32 tournament_game_id = 20;

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

    // optional uint32 server_region = 21;

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

    // optional .dota.DOTA_GameState game_state = 22;

    pub fn clear_game_state(&mut self) {
        self.game_state = ::std::option::Option::None;
    }

    pub fn has_game_state(&self) -> bool {
        self.game_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_state(&mut self, v: super::dota_shared_enums::DOTA_GameState) {
        self.game_state = ::std::option::Option::Some(v);
    }

    pub fn get_game_state(&self) -> super::dota_shared_enums::DOTA_GameState {
        self.game_state.unwrap_or(super::dota_shared_enums::DOTA_GameState::DOTA_GAMERULES_STATE_INIT)
    }

    fn get_game_state_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTA_GameState> {
        &self.game_state
    }

    fn mut_game_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTA_GameState> {
        &mut self.game_state
    }

    // optional uint32 num_spectators = 23;

    pub fn clear_num_spectators(&mut self) {
        self.num_spectators = ::std::option::Option::None;
    }

    pub fn has_num_spectators(&self) -> bool {
        self.num_spectators.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_spectators(&mut self, v: u32) {
        self.num_spectators = ::std::option::Option::Some(v);
    }

    pub fn get_num_spectators(&self) -> u32 {
        self.num_spectators.unwrap_or(0)
    }

    fn get_num_spectators_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_spectators
    }

    fn mut_num_spectators_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_spectators
    }

    // optional uint32 matchgroup = 25;

    pub fn clear_matchgroup(&mut self) {
        self.matchgroup = ::std::option::Option::None;
    }

    pub fn has_matchgroup(&self) -> bool {
        self.matchgroup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_matchgroup(&mut self, v: u32) {
        self.matchgroup = ::std::option::Option::Some(v);
    }

    pub fn get_matchgroup(&self) -> u32 {
        self.matchgroup.unwrap_or(0)
    }

    fn get_matchgroup_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.matchgroup
    }

    fn mut_matchgroup_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.matchgroup
    }

    // optional .dota.DOTA_CM_PICK cm_pick = 28;

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

    // optional uint64 match_id = 30;

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

    // optional bool allow_spectating = 31;

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
        self.allow_spectating.unwrap_or(true)
    }

    fn get_allow_spectating_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_spectating
    }

    fn mut_allow_spectating_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_spectating
    }

    // optional .dota.DOTABotDifficulty bot_difficulty_radiant = 36;

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
        self.bot_difficulty_radiant.unwrap_or(super::dota_shared_enums::DOTABotDifficulty::BOT_DIFFICULTY_HARD)
    }

    fn get_bot_difficulty_radiant_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &self.bot_difficulty_radiant
    }

    fn mut_bot_difficulty_radiant_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &mut self.bot_difficulty_radiant
    }

    // optional .dota.DOTAGameVersion game_version = 37;

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

    // repeated .dota.CLobbyTimedRewardDetails timed_reward_details = 38;

    pub fn clear_timed_reward_details(&mut self) {
        self.timed_reward_details.clear();
    }

    // Param is passed by value, moved
    pub fn set_timed_reward_details(&mut self, v: ::protobuf::RepeatedField<CLobbyTimedRewardDetails>) {
        self.timed_reward_details = v;
    }

    // Mutable pointer to the field.
    pub fn mut_timed_reward_details(&mut self) -> &mut ::protobuf::RepeatedField<CLobbyTimedRewardDetails> {
        &mut self.timed_reward_details
    }

    // Take field
    pub fn take_timed_reward_details(&mut self) -> ::protobuf::RepeatedField<CLobbyTimedRewardDetails> {
        ::std::mem::replace(&mut self.timed_reward_details, ::protobuf::RepeatedField::new())
    }

    pub fn get_timed_reward_details(&self) -> &[CLobbyTimedRewardDetails] {
        &self.timed_reward_details
    }

    fn get_timed_reward_details_for_reflect(&self) -> &::protobuf::RepeatedField<CLobbyTimedRewardDetails> {
        &self.timed_reward_details
    }

    fn mut_timed_reward_details_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CLobbyTimedRewardDetails> {
        &mut self.timed_reward_details
    }

    // optional string pass_key = 39;

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

    // optional uint32 leagueid = 42;

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

    // optional uint32 penalty_level_radiant = 43;

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
        self.penalty_level_radiant.unwrap_or(0u32)
    }

    fn get_penalty_level_radiant_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.penalty_level_radiant
    }

    fn mut_penalty_level_radiant_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.penalty_level_radiant
    }

    // optional uint32 penalty_level_dire = 44;

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
        self.penalty_level_dire.unwrap_or(0u32)
    }

    fn get_penalty_level_dire_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.penalty_level_dire
    }

    fn mut_penalty_level_dire_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.penalty_level_dire
    }

    // optional uint32 load_game_id = 45;

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

    // optional uint32 series_type = 46;

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

    // optional uint32 radiant_series_wins = 47;

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

    // optional uint32 dire_series_wins = 48;

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

    // optional uint32 loot_generated = 49;

    pub fn clear_loot_generated(&mut self) {
        self.loot_generated = ::std::option::Option::None;
    }

    pub fn has_loot_generated(&self) -> bool {
        self.loot_generated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_loot_generated(&mut self, v: u32) {
        self.loot_generated = ::std::option::Option::Some(v);
    }

    pub fn get_loot_generated(&self) -> u32 {
        self.loot_generated.unwrap_or(0)
    }

    fn get_loot_generated_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.loot_generated
    }

    fn mut_loot_generated_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.loot_generated
    }

    // optional uint32 loot_awarded = 50;

    pub fn clear_loot_awarded(&mut self) {
        self.loot_awarded = ::std::option::Option::None;
    }

    pub fn has_loot_awarded(&self) -> bool {
        self.loot_awarded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_loot_awarded(&mut self, v: u32) {
        self.loot_awarded = ::std::option::Option::Some(v);
    }

    pub fn get_loot_awarded(&self) -> u32 {
        self.loot_awarded.unwrap_or(0)
    }

    fn get_loot_awarded_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.loot_awarded
    }

    fn mut_loot_awarded_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.loot_awarded
    }

    // optional bool allchat = 51;

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

    // optional .dota.LobbyDotaTVDelay dota_tv_delay = 53;

    pub fn clear_dota_tv_delay(&mut self) {
        self.dota_tv_delay = ::std::option::Option::None;
    }

    pub fn has_dota_tv_delay(&self) -> bool {
        self.dota_tv_delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_tv_delay(&mut self, v: LobbyDotaTVDelay) {
        self.dota_tv_delay = ::std::option::Option::Some(v);
    }

    pub fn get_dota_tv_delay(&self) -> LobbyDotaTVDelay {
        self.dota_tv_delay.unwrap_or(LobbyDotaTVDelay::LobbyDotaTV_10)
    }

    fn get_dota_tv_delay_for_reflect(&self) -> &::std::option::Option<LobbyDotaTVDelay> {
        &self.dota_tv_delay
    }

    fn mut_dota_tv_delay_for_reflect(&mut self) -> &mut ::std::option::Option<LobbyDotaTVDelay> {
        &mut self.dota_tv_delay
    }

    // optional string custom_game_mode = 54;

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

    // optional string custom_map_name = 55;

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

    // optional uint32 custom_difficulty = 56;

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

    // optional bool lan = 57;

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

    // repeated .dota.CLobbyBroadcastChannelInfo broadcast_channel_info = 58;

    pub fn clear_broadcast_channel_info(&mut self) {
        self.broadcast_channel_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_broadcast_channel_info(&mut self, v: ::protobuf::RepeatedField<CLobbyBroadcastChannelInfo>) {
        self.broadcast_channel_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_broadcast_channel_info(&mut self) -> &mut ::protobuf::RepeatedField<CLobbyBroadcastChannelInfo> {
        &mut self.broadcast_channel_info
    }

    // Take field
    pub fn take_broadcast_channel_info(&mut self) -> ::protobuf::RepeatedField<CLobbyBroadcastChannelInfo> {
        ::std::mem::replace(&mut self.broadcast_channel_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_broadcast_channel_info(&self) -> &[CLobbyBroadcastChannelInfo] {
        &self.broadcast_channel_info
    }

    fn get_broadcast_channel_info_for_reflect(&self) -> &::protobuf::RepeatedField<CLobbyBroadcastChannelInfo> {
        &self.broadcast_channel_info
    }

    fn mut_broadcast_channel_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CLobbyBroadcastChannelInfo> {
        &mut self.broadcast_channel_info
    }

    // optional uint32 first_leaver_accountid = 59;

    pub fn clear_first_leaver_accountid(&mut self) {
        self.first_leaver_accountid = ::std::option::Option::None;
    }

    pub fn has_first_leaver_accountid(&self) -> bool {
        self.first_leaver_accountid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_leaver_accountid(&mut self, v: u32) {
        self.first_leaver_accountid = ::std::option::Option::Some(v);
    }

    pub fn get_first_leaver_accountid(&self) -> u32 {
        self.first_leaver_accountid.unwrap_or(0)
    }

    fn get_first_leaver_accountid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.first_leaver_accountid
    }

    fn mut_first_leaver_accountid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.first_leaver_accountid
    }

    // optional uint32 series_id = 60;

    pub fn clear_series_id(&mut self) {
        self.series_id = ::std::option::Option::None;
    }

    pub fn has_series_id(&self) -> bool {
        self.series_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_series_id(&mut self, v: u32) {
        self.series_id = ::std::option::Option::Some(v);
    }

    pub fn get_series_id(&self) -> u32 {
        self.series_id.unwrap_or(0)
    }

    fn get_series_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.series_id
    }

    fn mut_series_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.series_id
    }

    // optional bool low_priority = 61;

    pub fn clear_low_priority(&mut self) {
        self.low_priority = ::std::option::Option::None;
    }

    pub fn has_low_priority(&self) -> bool {
        self.low_priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_priority(&mut self, v: bool) {
        self.low_priority = ::std::option::Option::Some(v);
    }

    pub fn get_low_priority(&self) -> bool {
        self.low_priority.unwrap_or(false)
    }

    fn get_low_priority_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.low_priority
    }

    fn mut_low_priority_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.low_priority
    }

    // repeated .dota.CSODOTALobby.CExtraMsg extra_messages = 62;

    pub fn clear_extra_messages(&mut self) {
        self.extra_messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_extra_messages(&mut self, v: ::protobuf::RepeatedField<CSODOTALobby_CExtraMsg>) {
        self.extra_messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extra_messages(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTALobby_CExtraMsg> {
        &mut self.extra_messages
    }

    // Take field
    pub fn take_extra_messages(&mut self) -> ::protobuf::RepeatedField<CSODOTALobby_CExtraMsg> {
        ::std::mem::replace(&mut self.extra_messages, ::protobuf::RepeatedField::new())
    }

    pub fn get_extra_messages(&self) -> &[CSODOTALobby_CExtraMsg] {
        &self.extra_messages
    }

    fn get_extra_messages_for_reflect(&self) -> &::protobuf::RepeatedField<CSODOTALobby_CExtraMsg> {
        &self.extra_messages
    }

    fn mut_extra_messages_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSODOTALobby_CExtraMsg> {
        &mut self.extra_messages
    }

    // optional .dota.CDOTASaveGame save_game = 63;

    pub fn clear_save_game(&mut self) {
        self.save_game.clear();
    }

    pub fn has_save_game(&self) -> bool {
        self.save_game.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_game(&mut self, v: super::dota_shared_enums::CDOTASaveGame) {
        self.save_game = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_save_game(&mut self) -> &mut super::dota_shared_enums::CDOTASaveGame {
        if self.save_game.is_none() {
            self.save_game.set_default();
        };
        self.save_game.as_mut().unwrap()
    }

    // Take field
    pub fn take_save_game(&mut self) -> super::dota_shared_enums::CDOTASaveGame {
        self.save_game.take().unwrap_or_else(|| super::dota_shared_enums::CDOTASaveGame::new())
    }

    pub fn get_save_game(&self) -> &super::dota_shared_enums::CDOTASaveGame {
        self.save_game.as_ref().unwrap_or_else(|| super::dota_shared_enums::CDOTASaveGame::default_instance())
    }

    fn get_save_game_for_reflect(&self) -> &::protobuf::SingularPtrField<super::dota_shared_enums::CDOTASaveGame> {
        &self.save_game
    }

    fn mut_save_game_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::dota_shared_enums::CDOTASaveGame> {
        &mut self.save_game
    }

    // optional bool first_blood_happened = 65;

    pub fn clear_first_blood_happened(&mut self) {
        self.first_blood_happened = ::std::option::Option::None;
    }

    pub fn has_first_blood_happened(&self) -> bool {
        self.first_blood_happened.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_blood_happened(&mut self, v: bool) {
        self.first_blood_happened = ::std::option::Option::Some(v);
    }

    pub fn get_first_blood_happened(&self) -> bool {
        self.first_blood_happened.unwrap_or(false)
    }

    fn get_first_blood_happened_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.first_blood_happened
    }

    fn mut_first_blood_happened_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.first_blood_happened
    }

    // optional .dota.EMatchOutcome match_outcome = 70;

    pub fn clear_match_outcome(&mut self) {
        self.match_outcome = ::std::option::Option::None;
    }

    pub fn has_match_outcome(&self) -> bool {
        self.match_outcome.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_outcome(&mut self, v: super::dota_shared_enums::EMatchOutcome) {
        self.match_outcome = ::std::option::Option::Some(v);
    }

    pub fn get_match_outcome(&self) -> super::dota_shared_enums::EMatchOutcome {
        self.match_outcome.unwrap_or(super::dota_shared_enums::EMatchOutcome::k_EMatchOutcome_Unknown)
    }

    fn get_match_outcome_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::EMatchOutcome> {
        &self.match_outcome
    }

    fn mut_match_outcome_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::EMatchOutcome> {
        &mut self.match_outcome
    }

    // optional bool mass_disconnect = 67;

    pub fn clear_mass_disconnect(&mut self) {
        self.mass_disconnect = ::std::option::Option::None;
    }

    pub fn has_mass_disconnect(&self) -> bool {
        self.mass_disconnect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mass_disconnect(&mut self, v: bool) {
        self.mass_disconnect = ::std::option::Option::Some(v);
    }

    pub fn get_mass_disconnect(&self) -> bool {
        self.mass_disconnect.unwrap_or(false)
    }

    fn get_mass_disconnect_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.mass_disconnect
    }

    fn mut_mass_disconnect_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.mass_disconnect
    }

    // optional uint64 custom_game_id = 68;

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

    // repeated .dota.EEvent active_ingame_events = 69;

    pub fn clear_active_ingame_events(&mut self) {
        self.active_ingame_events.clear();
    }

    // Param is passed by value, moved
    pub fn set_active_ingame_events(&mut self, v: ::std::vec::Vec<super::dota_shared_enums::EEvent>) {
        self.active_ingame_events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_active_ingame_events(&mut self) -> &mut ::std::vec::Vec<super::dota_shared_enums::EEvent> {
        &mut self.active_ingame_events
    }

    // Take field
    pub fn take_active_ingame_events(&mut self) -> ::std::vec::Vec<super::dota_shared_enums::EEvent> {
        ::std::mem::replace(&mut self.active_ingame_events, ::std::vec::Vec::new())
    }

    pub fn get_active_ingame_events(&self) -> &[super::dota_shared_enums::EEvent] {
        &self.active_ingame_events
    }

    fn get_active_ingame_events_for_reflect(&self) -> &::std::vec::Vec<super::dota_shared_enums::EEvent> {
        &self.active_ingame_events
    }

    fn mut_active_ingame_events_for_reflect(&mut self) -> &mut ::std::vec::Vec<super::dota_shared_enums::EEvent> {
        &mut self.active_ingame_events
    }

    // optional uint32 custom_min_players = 71;

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

    // optional uint32 custom_max_players = 72;

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

    // optional .dota.PartnerAccountType partner_type = 73;

    pub fn clear_partner_type(&mut self) {
        self.partner_type = ::std::option::Option::None;
    }

    pub fn has_partner_type(&self) -> bool {
        self.partner_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partner_type(&mut self, v: super::gcsdk_gcmessages::PartnerAccountType) {
        self.partner_type = ::std::option::Option::Some(v);
    }

    pub fn get_partner_type(&self) -> super::gcsdk_gcmessages::PartnerAccountType {
        self.partner_type.unwrap_or(super::gcsdk_gcmessages::PartnerAccountType::PARTNER_NONE)
    }

    fn get_partner_type_for_reflect(&self) -> &::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType> {
        &self.partner_type
    }

    fn mut_partner_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::gcsdk_gcmessages::PartnerAccountType> {
        &mut self.partner_type
    }

    // optional uint32 lan_host_ping_to_server_region = 74;

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

    // optional .dota.DOTALobbyVisibility visibility = 75;

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

    // optional fixed64 custom_game_crc = 76;

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

    // optional bool custom_game_auto_created_lobby = 77;

    pub fn clear_custom_game_auto_created_lobby(&mut self) {
        self.custom_game_auto_created_lobby = ::std::option::Option::None;
    }

    pub fn has_custom_game_auto_created_lobby(&self) -> bool {
        self.custom_game_auto_created_lobby.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_auto_created_lobby(&mut self, v: bool) {
        self.custom_game_auto_created_lobby = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_auto_created_lobby(&self) -> bool {
        self.custom_game_auto_created_lobby.unwrap_or(false)
    }

    fn get_custom_game_auto_created_lobby_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.custom_game_auto_created_lobby
    }

    fn mut_custom_game_auto_created_lobby_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.custom_game_auto_created_lobby
    }

    // optional uint32 league_series_id = 78;

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

    // optional uint32 league_game_id = 79;

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

    // optional fixed32 custom_game_timestamp = 80;

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

    // repeated uint64 previous_series_matches = 81;

    pub fn clear_previous_series_matches(&mut self) {
        self.previous_series_matches.clear();
    }

    // Param is passed by value, moved
    pub fn set_previous_series_matches(&mut self, v: ::std::vec::Vec<u64>) {
        self.previous_series_matches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_previous_series_matches(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.previous_series_matches
    }

    // Take field
    pub fn take_previous_series_matches(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.previous_series_matches, ::std::vec::Vec::new())
    }

    pub fn get_previous_series_matches(&self) -> &[u64] {
        &self.previous_series_matches
    }

    fn get_previous_series_matches_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.previous_series_matches
    }

    fn mut_previous_series_matches_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.previous_series_matches
    }

    // optional uint64 previous_match_override = 82;

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

    // optional bool custom_game_uses_account_records = 83;

    pub fn clear_custom_game_uses_account_records(&mut self) {
        self.custom_game_uses_account_records = ::std::option::Option::None;
    }

    pub fn has_custom_game_uses_account_records(&self) -> bool {
        self.custom_game_uses_account_records.is_some()
    }

    // Param is passed by value, moved
    pub fn set_custom_game_uses_account_records(&mut self, v: bool) {
        self.custom_game_uses_account_records = ::std::option::Option::Some(v);
    }

    pub fn get_custom_game_uses_account_records(&self) -> bool {
        self.custom_game_uses_account_records.unwrap_or(false)
    }

    fn get_custom_game_uses_account_records_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.custom_game_uses_account_records
    }

    fn mut_custom_game_uses_account_records_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.custom_game_uses_account_records
    }

    // optional uint32 league_selection_priority_team = 84;

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

    // optional .dota.SelectionPriorityType league_selection_priority_choice = 85;

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

    // optional .dota.SelectionPriorityType league_non_selection_priority_choice = 86;

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

    // optional uint32 game_start_time = 87;

    pub fn clear_game_start_time(&mut self) {
        self.game_start_time = ::std::option::Option::None;
    }

    pub fn has_game_start_time(&self) -> bool {
        self.game_start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_start_time(&mut self, v: u32) {
        self.game_start_time = ::std::option::Option::Some(v);
    }

    pub fn get_game_start_time(&self) -> u32 {
        self.game_start_time.unwrap_or(0)
    }

    fn get_game_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_start_time
    }

    fn mut_game_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_start_time
    }

    // optional .dota.LobbyDotaPauseSetting pause_setting = 88;

    pub fn clear_pause_setting(&mut self) {
        self.pause_setting = ::std::option::Option::None;
    }

    pub fn has_pause_setting(&self) -> bool {
        self.pause_setting.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pause_setting(&mut self, v: LobbyDotaPauseSetting) {
        self.pause_setting = ::std::option::Option::Some(v);
    }

    pub fn get_pause_setting(&self) -> LobbyDotaPauseSetting {
        self.pause_setting.unwrap_or(LobbyDotaPauseSetting::LobbyDotaPauseSetting_Unlimited)
    }

    fn get_pause_setting_for_reflect(&self) -> &::std::option::Option<LobbyDotaPauseSetting> {
        &self.pause_setting
    }

    fn mut_pause_setting_for_reflect(&mut self) -> &mut ::std::option::Option<LobbyDotaPauseSetting> {
        &mut self.pause_setting
    }

    // optional uint32 lobby_mvp_account_id = 89;

    pub fn clear_lobby_mvp_account_id(&mut self) {
        self.lobby_mvp_account_id = ::std::option::Option::None;
    }

    pub fn has_lobby_mvp_account_id(&self) -> bool {
        self.lobby_mvp_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_mvp_account_id(&mut self, v: u32) {
        self.lobby_mvp_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_mvp_account_id(&self) -> u32 {
        self.lobby_mvp_account_id.unwrap_or(0)
    }

    fn get_lobby_mvp_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lobby_mvp_account_id
    }

    fn mut_lobby_mvp_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lobby_mvp_account_id
    }

    // optional uint32 weekend_tourney_division_id = 90;

    pub fn clear_weekend_tourney_division_id(&mut self) {
        self.weekend_tourney_division_id = ::std::option::Option::None;
    }

    pub fn has_weekend_tourney_division_id(&self) -> bool {
        self.weekend_tourney_division_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weekend_tourney_division_id(&mut self, v: u32) {
        self.weekend_tourney_division_id = ::std::option::Option::Some(v);
    }

    pub fn get_weekend_tourney_division_id(&self) -> u32 {
        self.weekend_tourney_division_id.unwrap_or(0)
    }

    fn get_weekend_tourney_division_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.weekend_tourney_division_id
    }

    fn mut_weekend_tourney_division_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.weekend_tourney_division_id
    }

    // optional uint32 weekend_tourney_skill_level = 91;

    pub fn clear_weekend_tourney_skill_level(&mut self) {
        self.weekend_tourney_skill_level = ::std::option::Option::None;
    }

    pub fn has_weekend_tourney_skill_level(&self) -> bool {
        self.weekend_tourney_skill_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weekend_tourney_skill_level(&mut self, v: u32) {
        self.weekend_tourney_skill_level = ::std::option::Option::Some(v);
    }

    pub fn get_weekend_tourney_skill_level(&self) -> u32 {
        self.weekend_tourney_skill_level.unwrap_or(0)
    }

    fn get_weekend_tourney_skill_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.weekend_tourney_skill_level
    }

    fn mut_weekend_tourney_skill_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.weekend_tourney_skill_level
    }

    // optional uint32 weekend_tourney_bracket_round = 92;

    pub fn clear_weekend_tourney_bracket_round(&mut self) {
        self.weekend_tourney_bracket_round = ::std::option::Option::None;
    }

    pub fn has_weekend_tourney_bracket_round(&self) -> bool {
        self.weekend_tourney_bracket_round.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weekend_tourney_bracket_round(&mut self, v: u32) {
        self.weekend_tourney_bracket_round = ::std::option::Option::Some(v);
    }

    pub fn get_weekend_tourney_bracket_round(&self) -> u32 {
        self.weekend_tourney_bracket_round.unwrap_or(0)
    }

    fn get_weekend_tourney_bracket_round_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.weekend_tourney_bracket_round
    }

    fn mut_weekend_tourney_bracket_round_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.weekend_tourney_bracket_round
    }

    // optional .dota.DOTABotDifficulty bot_difficulty_dire = 93;

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
        self.bot_difficulty_dire.unwrap_or(super::dota_shared_enums::DOTABotDifficulty::BOT_DIFFICULTY_HARD)
    }

    fn get_bot_difficulty_dire_for_reflect(&self) -> &::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &self.bot_difficulty_dire
    }

    fn mut_bot_difficulty_dire_for_reflect(&mut self) -> &mut ::std::option::Option<super::dota_shared_enums::DOTABotDifficulty> {
        &mut self.bot_difficulty_dire
    }

    // optional uint64 bot_radiant = 94;

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

    // optional uint64 bot_dire = 95;

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

impl ::protobuf::Message for CSODOTALobby {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.left_members)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.leader_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.server_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.pending_invites)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.connect)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.lobby_type = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allow_cheats = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.fill_with_bots = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.intro_mode = ::std::option::Option::Some(tmp);
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_name)?;
                },
                17 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.team_details)?;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tutorial_lesson = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tournament_id = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.tournament_game_id = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.server_region = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_state = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.num_spectators = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.matchgroup = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.cm_pick = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allow_spectating = ::std::option::Option::Some(tmp);
                },
                36 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.bot_difficulty_radiant = ::std::option::Option::Some(tmp);
                },
                37 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_version = ::std::option::Option::Some(tmp);
                },
                38 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.timed_reward_details)?;
                },
                39 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.pass_key)?;
                },
                42 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.leagueid = ::std::option::Option::Some(tmp);
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.penalty_level_radiant = ::std::option::Option::Some(tmp);
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.penalty_level_dire = ::std::option::Option::Some(tmp);
                },
                45 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.load_game_id = ::std::option::Option::Some(tmp);
                },
                46 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_type = ::std::option::Option::Some(tmp);
                },
                47 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.radiant_series_wins = ::std::option::Option::Some(tmp);
                },
                48 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.dire_series_wins = ::std::option::Option::Some(tmp);
                },
                49 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.loot_generated = ::std::option::Option::Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.loot_awarded = ::std::option::Option::Some(tmp);
                },
                51 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allchat = ::std::option::Option::Some(tmp);
                },
                53 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.dota_tv_delay = ::std::option::Option::Some(tmp);
                },
                54 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.custom_game_mode)?;
                },
                55 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.custom_map_name)?;
                },
                56 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.custom_difficulty = ::std::option::Option::Some(tmp);
                },
                57 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.lan = ::std::option::Option::Some(tmp);
                },
                58 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.broadcast_channel_info)?;
                },
                59 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.first_leaver_accountid = ::std::option::Option::Some(tmp);
                },
                60 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_id = ::std::option::Option::Some(tmp);
                },
                61 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.low_priority = ::std::option::Option::Some(tmp);
                },
                62 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extra_messages)?;
                },
                63 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.save_game)?;
                },
                65 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.first_blood_happened = ::std::option::Option::Some(tmp);
                },
                70 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.match_outcome = ::std::option::Option::Some(tmp);
                },
                67 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.mass_disconnect = ::std::option::Option::Some(tmp);
                },
                68 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.custom_game_id = ::std::option::Option::Some(tmp);
                },
                69 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.active_ingame_events)?;
                },
                71 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.custom_min_players = ::std::option::Option::Some(tmp);
                },
                72 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.custom_max_players = ::std::option::Option::Some(tmp);
                },
                73 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.partner_type = ::std::option::Option::Some(tmp);
                },
                74 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lan_host_ping_to_server_region = ::std::option::Option::Some(tmp);
                },
                75 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.visibility = ::std::option::Option::Some(tmp);
                },
                76 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.custom_game_crc = ::std::option::Option::Some(tmp);
                },
                77 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.custom_game_auto_created_lobby = ::std::option::Option::Some(tmp);
                },
                78 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_series_id = ::std::option::Option::Some(tmp);
                },
                79 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_game_id = ::std::option::Option::Some(tmp);
                },
                80 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.custom_game_timestamp = ::std::option::Option::Some(tmp);
                },
                81 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.previous_series_matches)?;
                },
                82 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.previous_match_override = ::std::option::Option::Some(tmp);
                },
                83 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.custom_game_uses_account_records = ::std::option::Option::Some(tmp);
                },
                84 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_selection_priority_team = ::std::option::Option::Some(tmp);
                },
                85 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.league_selection_priority_choice = ::std::option::Option::Some(tmp);
                },
                86 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.league_non_selection_priority_choice = ::std::option::Option::Some(tmp);
                },
                87 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_start_time = ::std::option::Option::Some(tmp);
                },
                88 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.pause_setting = ::std::option::Option::Some(tmp);
                },
                89 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lobby_mvp_account_id = ::std::option::Option::Some(tmp);
                },
                90 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_division_id = ::std::option::Option::Some(tmp);
                },
                91 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_skill_level = ::std::option::Option::Some(tmp);
                },
                92 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_bracket_round = ::std::option::Option::Some(tmp);
                },
                93 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.bot_difficulty_dire = ::std::option::Option::Some(tmp);
                },
                94 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.bot_radiant = ::std::option::Option::Some(tmp);
                },
                95 => {
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
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.left_members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.leader_id {
            my_size += 9;
        };
        if let Some(v) = self.server_id {
            my_size += 9;
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 9 * self.pending_invites.len() as u32;
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(4, v);
        };
        if let Some(v) = self.connect.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.lobby_type {
            my_size += ::protobuf::rt::enum_size(12, v);
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
        if let Some(v) = self.game_name.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        };
        for value in &self.team_details {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.tutorial_lesson {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tournament_id {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.tournament_game_id {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.server_region {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_state {
            my_size += ::protobuf::rt::enum_size(22, v);
        };
        if let Some(v) = self.num_spectators {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.matchgroup {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.cm_pick {
            my_size += ::protobuf::rt::enum_size(28, v);
        };
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.allow_spectating {
            my_size += 3;
        };
        if let Some(v) = self.bot_difficulty_radiant {
            my_size += ::protobuf::rt::enum_size(36, v);
        };
        if let Some(v) = self.game_version {
            my_size += ::protobuf::rt::enum_size(37, v);
        };
        for value in &self.timed_reward_details {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.pass_key.as_ref() {
            my_size += ::protobuf::rt::string_size(39, &v);
        };
        if let Some(v) = self.leagueid {
            my_size += ::protobuf::rt::value_size(42, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.penalty_level_radiant {
            my_size += ::protobuf::rt::value_size(43, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.penalty_level_dire {
            my_size += ::protobuf::rt::value_size(44, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.load_game_id {
            my_size += ::protobuf::rt::value_size(45, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.series_type {
            my_size += ::protobuf::rt::value_size(46, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radiant_series_wins {
            my_size += ::protobuf::rt::value_size(47, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dire_series_wins {
            my_size += ::protobuf::rt::value_size(48, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.loot_generated {
            my_size += ::protobuf::rt::value_size(49, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.loot_awarded {
            my_size += ::protobuf::rt::value_size(50, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.allchat {
            my_size += 3;
        };
        if let Some(v) = self.dota_tv_delay {
            my_size += ::protobuf::rt::enum_size(53, v);
        };
        if let Some(v) = self.custom_game_mode.as_ref() {
            my_size += ::protobuf::rt::string_size(54, &v);
        };
        if let Some(v) = self.custom_map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(55, &v);
        };
        if let Some(v) = self.custom_difficulty {
            my_size += ::protobuf::rt::value_size(56, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lan {
            my_size += 3;
        };
        for value in &self.broadcast_channel_info {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.first_leaver_accountid {
            my_size += ::protobuf::rt::value_size(59, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.series_id {
            my_size += ::protobuf::rt::value_size(60, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.low_priority {
            my_size += 3;
        };
        for value in &self.extra_messages {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.save_game.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.first_blood_happened {
            my_size += 3;
        };
        if let Some(v) = self.match_outcome {
            my_size += ::protobuf::rt::enum_size(70, v);
        };
        if let Some(v) = self.mass_disconnect {
            my_size += 3;
        };
        if let Some(v) = self.custom_game_id {
            my_size += ::protobuf::rt::value_size(68, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.active_ingame_events {
            my_size += ::protobuf::rt::enum_size(69, *value);
        };
        if let Some(v) = self.custom_min_players {
            my_size += ::protobuf::rt::value_size(71, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_max_players {
            my_size += ::protobuf::rt::value_size(72, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.partner_type {
            my_size += ::protobuf::rt::enum_size(73, v);
        };
        if let Some(v) = self.lan_host_ping_to_server_region {
            my_size += ::protobuf::rt::value_size(74, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.visibility {
            my_size += ::protobuf::rt::enum_size(75, v);
        };
        if let Some(v) = self.custom_game_crc {
            my_size += 10;
        };
        if let Some(v) = self.custom_game_auto_created_lobby {
            my_size += 3;
        };
        if let Some(v) = self.league_series_id {
            my_size += ::protobuf::rt::value_size(78, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_game_id {
            my_size += ::protobuf::rt::value_size(79, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_game_timestamp {
            my_size += 6;
        };
        for value in &self.previous_series_matches {
            my_size += ::protobuf::rt::value_size(81, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.previous_match_override {
            my_size += ::protobuf::rt::value_size(82, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.custom_game_uses_account_records {
            my_size += 3;
        };
        if let Some(v) = self.league_selection_priority_team {
            my_size += ::protobuf::rt::value_size(84, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_selection_priority_choice {
            my_size += ::protobuf::rt::enum_size(85, v);
        };
        if let Some(v) = self.league_non_selection_priority_choice {
            my_size += ::protobuf::rt::enum_size(86, v);
        };
        if let Some(v) = self.game_start_time {
            my_size += ::protobuf::rt::value_size(87, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.pause_setting {
            my_size += ::protobuf::rt::enum_size(88, v);
        };
        if let Some(v) = self.lobby_mvp_account_id {
            my_size += ::protobuf::rt::value_size(89, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_division_id {
            my_size += ::protobuf::rt::value_size(90, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_skill_level {
            my_size += ::protobuf::rt::value_size(91, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_bracket_round {
            my_size += ::protobuf::rt::value_size(92, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.bot_difficulty_dire {
            my_size += ::protobuf::rt::enum_size(93, v);
        };
        if let Some(v) = self.bot_radiant {
            my_size += ::protobuf::rt::value_size(94, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.bot_dire {
            my_size += ::protobuf::rt::value_size(95, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lobby_id {
            os.write_uint64(1, v)?;
        };
        for v in &self.members {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.left_members {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.leader_id {
            os.write_fixed64(11, v)?;
        };
        if let Some(v) = self.server_id {
            os.write_fixed64(6, v)?;
        };
        if let Some(v) = self.game_mode {
            os.write_uint32(3, v)?;
        };
        for v in &self.pending_invites {
            os.write_fixed64(10, *v)?;
        };
        if let Some(v) = self.state {
            os.write_enum(4, v.value())?;
        };
        if let Some(v) = self.connect.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.lobby_type {
            os.write_enum(12, v.value())?;
        };
        if let Some(v) = self.allow_cheats {
            os.write_bool(13, v)?;
        };
        if let Some(v) = self.fill_with_bots {
            os.write_bool(14, v)?;
        };
        if let Some(v) = self.intro_mode {
            os.write_bool(15, v)?;
        };
        if let Some(v) = self.game_name.as_ref() {
            os.write_string(16, &v)?;
        };
        for v in &self.team_details {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.tutorial_lesson {
            os.write_uint32(18, v)?;
        };
        if let Some(v) = self.tournament_id {
            os.write_uint32(19, v)?;
        };
        if let Some(v) = self.tournament_game_id {
            os.write_uint32(20, v)?;
        };
        if let Some(v) = self.server_region {
            os.write_uint32(21, v)?;
        };
        if let Some(v) = self.game_state {
            os.write_enum(22, v.value())?;
        };
        if let Some(v) = self.num_spectators {
            os.write_uint32(23, v)?;
        };
        if let Some(v) = self.matchgroup {
            os.write_uint32(25, v)?;
        };
        if let Some(v) = self.cm_pick {
            os.write_enum(28, v.value())?;
        };
        if let Some(v) = self.match_id {
            os.write_uint64(30, v)?;
        };
        if let Some(v) = self.allow_spectating {
            os.write_bool(31, v)?;
        };
        if let Some(v) = self.bot_difficulty_radiant {
            os.write_enum(36, v.value())?;
        };
        if let Some(v) = self.game_version {
            os.write_enum(37, v.value())?;
        };
        for v in &self.timed_reward_details {
            os.write_tag(38, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.pass_key.as_ref() {
            os.write_string(39, &v)?;
        };
        if let Some(v) = self.leagueid {
            os.write_uint32(42, v)?;
        };
        if let Some(v) = self.penalty_level_radiant {
            os.write_uint32(43, v)?;
        };
        if let Some(v) = self.penalty_level_dire {
            os.write_uint32(44, v)?;
        };
        if let Some(v) = self.load_game_id {
            os.write_uint32(45, v)?;
        };
        if let Some(v) = self.series_type {
            os.write_uint32(46, v)?;
        };
        if let Some(v) = self.radiant_series_wins {
            os.write_uint32(47, v)?;
        };
        if let Some(v) = self.dire_series_wins {
            os.write_uint32(48, v)?;
        };
        if let Some(v) = self.loot_generated {
            os.write_uint32(49, v)?;
        };
        if let Some(v) = self.loot_awarded {
            os.write_uint32(50, v)?;
        };
        if let Some(v) = self.allchat {
            os.write_bool(51, v)?;
        };
        if let Some(v) = self.dota_tv_delay {
            os.write_enum(53, v.value())?;
        };
        if let Some(v) = self.custom_game_mode.as_ref() {
            os.write_string(54, &v)?;
        };
        if let Some(v) = self.custom_map_name.as_ref() {
            os.write_string(55, &v)?;
        };
        if let Some(v) = self.custom_difficulty {
            os.write_uint32(56, v)?;
        };
        if let Some(v) = self.lan {
            os.write_bool(57, v)?;
        };
        for v in &self.broadcast_channel_info {
            os.write_tag(58, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.first_leaver_accountid {
            os.write_uint32(59, v)?;
        };
        if let Some(v) = self.series_id {
            os.write_uint32(60, v)?;
        };
        if let Some(v) = self.low_priority {
            os.write_bool(61, v)?;
        };
        for v in &self.extra_messages {
            os.write_tag(62, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.save_game.as_ref() {
            os.write_tag(63, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.first_blood_happened {
            os.write_bool(65, v)?;
        };
        if let Some(v) = self.match_outcome {
            os.write_enum(70, v.value())?;
        };
        if let Some(v) = self.mass_disconnect {
            os.write_bool(67, v)?;
        };
        if let Some(v) = self.custom_game_id {
            os.write_uint64(68, v)?;
        };
        for v in &self.active_ingame_events {
            os.write_enum(69, v.value())?;
        };
        if let Some(v) = self.custom_min_players {
            os.write_uint32(71, v)?;
        };
        if let Some(v) = self.custom_max_players {
            os.write_uint32(72, v)?;
        };
        if let Some(v) = self.partner_type {
            os.write_enum(73, v.value())?;
        };
        if let Some(v) = self.lan_host_ping_to_server_region {
            os.write_uint32(74, v)?;
        };
        if let Some(v) = self.visibility {
            os.write_enum(75, v.value())?;
        };
        if let Some(v) = self.custom_game_crc {
            os.write_fixed64(76, v)?;
        };
        if let Some(v) = self.custom_game_auto_created_lobby {
            os.write_bool(77, v)?;
        };
        if let Some(v) = self.league_series_id {
            os.write_uint32(78, v)?;
        };
        if let Some(v) = self.league_game_id {
            os.write_uint32(79, v)?;
        };
        if let Some(v) = self.custom_game_timestamp {
            os.write_fixed32(80, v)?;
        };
        for v in &self.previous_series_matches {
            os.write_uint64(81, *v)?;
        };
        if let Some(v) = self.previous_match_override {
            os.write_uint64(82, v)?;
        };
        if let Some(v) = self.custom_game_uses_account_records {
            os.write_bool(83, v)?;
        };
        if let Some(v) = self.league_selection_priority_team {
            os.write_uint32(84, v)?;
        };
        if let Some(v) = self.league_selection_priority_choice {
            os.write_enum(85, v.value())?;
        };
        if let Some(v) = self.league_non_selection_priority_choice {
            os.write_enum(86, v.value())?;
        };
        if let Some(v) = self.game_start_time {
            os.write_uint32(87, v)?;
        };
        if let Some(v) = self.pause_setting {
            os.write_enum(88, v.value())?;
        };
        if let Some(v) = self.lobby_mvp_account_id {
            os.write_uint32(89, v)?;
        };
        if let Some(v) = self.weekend_tourney_division_id {
            os.write_uint32(90, v)?;
        };
        if let Some(v) = self.weekend_tourney_skill_level {
            os.write_uint32(91, v)?;
        };
        if let Some(v) = self.weekend_tourney_bracket_round {
            os.write_uint32(92, v)?;
        };
        if let Some(v) = self.bot_difficulty_dire {
            os.write_enum(93, v.value())?;
        };
        if let Some(v) = self.bot_radiant {
            os.write_uint64(94, v)?;
        };
        if let Some(v) = self.bot_dire {
            os.write_uint64(95, v)?;
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

impl ::protobuf::MessageStatic for CSODOTALobby {
    fn new() -> CSODOTALobby {
        CSODOTALobby::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSODOTALobby>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lobby_id",
                    CSODOTALobby::get_lobby_id_for_reflect,
                    CSODOTALobby::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTALobbyMember>>(
                    "members",
                    CSODOTALobby::get_members_for_reflect,
                    CSODOTALobby::mut_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTALobbyMember>>(
                    "left_members",
                    CSODOTALobby::get_left_members_for_reflect,
                    CSODOTALobby::mut_left_members_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "leader_id",
                    CSODOTALobby::get_leader_id_for_reflect,
                    CSODOTALobby::mut_leader_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "server_id",
                    CSODOTALobby::get_server_id_for_reflect,
                    CSODOTALobby::mut_server_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_mode",
                    CSODOTALobby::get_game_mode_for_reflect,
                    CSODOTALobby::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "pending_invites",
                    CSODOTALobby::get_pending_invites_for_reflect,
                    CSODOTALobby::mut_pending_invites_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CSODOTALobby_State>>(
                    "state",
                    CSODOTALobby::get_state_for_reflect,
                    CSODOTALobby::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "connect",
                    CSODOTALobby::get_connect_for_reflect,
                    CSODOTALobby::mut_connect_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CSODOTALobby_LobbyType>>(
                    "lobby_type",
                    CSODOTALobby::get_lobby_type_for_reflect,
                    CSODOTALobby::mut_lobby_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_cheats",
                    CSODOTALobby::get_allow_cheats_for_reflect,
                    CSODOTALobby::mut_allow_cheats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "fill_with_bots",
                    CSODOTALobby::get_fill_with_bots_for_reflect,
                    CSODOTALobby::mut_fill_with_bots_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "intro_mode",
                    CSODOTALobby::get_intro_mode_for_reflect,
                    CSODOTALobby::mut_intro_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_name",
                    CSODOTALobby::get_game_name_for_reflect,
                    CSODOTALobby::mut_game_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CLobbyTeamDetails>>(
                    "team_details",
                    CSODOTALobby::get_team_details_for_reflect,
                    CSODOTALobby::mut_team_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tutorial_lesson",
                    CSODOTALobby::get_tutorial_lesson_for_reflect,
                    CSODOTALobby::mut_tutorial_lesson_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_id",
                    CSODOTALobby::get_tournament_id_for_reflect,
                    CSODOTALobby::mut_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tournament_game_id",
                    CSODOTALobby::get_tournament_game_id_for_reflect,
                    CSODOTALobby::mut_tournament_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_region",
                    CSODOTALobby::get_server_region_for_reflect,
                    CSODOTALobby::mut_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GameState>>(
                    "game_state",
                    CSODOTALobby::get_game_state_for_reflect,
                    CSODOTALobby::mut_game_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_spectators",
                    CSODOTALobby::get_num_spectators_for_reflect,
                    CSODOTALobby::mut_num_spectators_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "matchgroup",
                    CSODOTALobby::get_matchgroup_for_reflect,
                    CSODOTALobby::mut_matchgroup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_CM_PICK>>(
                    "cm_pick",
                    CSODOTALobby::get_cm_pick_for_reflect,
                    CSODOTALobby::mut_cm_pick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CSODOTALobby::get_match_id_for_reflect,
                    CSODOTALobby::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_spectating",
                    CSODOTALobby::get_allow_spectating_for_reflect,
                    CSODOTALobby::mut_allow_spectating_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "bot_difficulty_radiant",
                    CSODOTALobby::get_bot_difficulty_radiant_for_reflect,
                    CSODOTALobby::mut_bot_difficulty_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTAGameVersion>>(
                    "game_version",
                    CSODOTALobby::get_game_version_for_reflect,
                    CSODOTALobby::mut_game_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CLobbyTimedRewardDetails>>(
                    "timed_reward_details",
                    CSODOTALobby::get_timed_reward_details_for_reflect,
                    CSODOTALobby::mut_timed_reward_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pass_key",
                    CSODOTALobby::get_pass_key_for_reflect,
                    CSODOTALobby::mut_pass_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leagueid",
                    CSODOTALobby::get_leagueid_for_reflect,
                    CSODOTALobby::mut_leagueid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "penalty_level_radiant",
                    CSODOTALobby::get_penalty_level_radiant_for_reflect,
                    CSODOTALobby::mut_penalty_level_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "penalty_level_dire",
                    CSODOTALobby::get_penalty_level_dire_for_reflect,
                    CSODOTALobby::mut_penalty_level_dire_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "load_game_id",
                    CSODOTALobby::get_load_game_id_for_reflect,
                    CSODOTALobby::mut_load_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_type",
                    CSODOTALobby::get_series_type_for_reflect,
                    CSODOTALobby::mut_series_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "radiant_series_wins",
                    CSODOTALobby::get_radiant_series_wins_for_reflect,
                    CSODOTALobby::mut_radiant_series_wins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dire_series_wins",
                    CSODOTALobby::get_dire_series_wins_for_reflect,
                    CSODOTALobby::mut_dire_series_wins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "loot_generated",
                    CSODOTALobby::get_loot_generated_for_reflect,
                    CSODOTALobby::mut_loot_generated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "loot_awarded",
                    CSODOTALobby::get_loot_awarded_for_reflect,
                    CSODOTALobby::mut_loot_awarded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allchat",
                    CSODOTALobby::get_allchat_for_reflect,
                    CSODOTALobby::mut_allchat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<LobbyDotaTVDelay>>(
                    "dota_tv_delay",
                    CSODOTALobby::get_dota_tv_delay_for_reflect,
                    CSODOTALobby::mut_dota_tv_delay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "custom_game_mode",
                    CSODOTALobby::get_custom_game_mode_for_reflect,
                    CSODOTALobby::mut_custom_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "custom_map_name",
                    CSODOTALobby::get_custom_map_name_for_reflect,
                    CSODOTALobby::mut_custom_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "custom_difficulty",
                    CSODOTALobby::get_custom_difficulty_for_reflect,
                    CSODOTALobby::mut_custom_difficulty_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "lan",
                    CSODOTALobby::get_lan_for_reflect,
                    CSODOTALobby::mut_lan_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CLobbyBroadcastChannelInfo>>(
                    "broadcast_channel_info",
                    CSODOTALobby::get_broadcast_channel_info_for_reflect,
                    CSODOTALobby::mut_broadcast_channel_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "first_leaver_accountid",
                    CSODOTALobby::get_first_leaver_accountid_for_reflect,
                    CSODOTALobby::mut_first_leaver_accountid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_id",
                    CSODOTALobby::get_series_id_for_reflect,
                    CSODOTALobby::mut_series_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "low_priority",
                    CSODOTALobby::get_low_priority_for_reflect,
                    CSODOTALobby::mut_low_priority_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSODOTALobby_CExtraMsg>>(
                    "extra_messages",
                    CSODOTALobby::get_extra_messages_for_reflect,
                    CSODOTALobby::mut_extra_messages_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_shared_enums::CDOTASaveGame>>(
                    "save_game",
                    CSODOTALobby::get_save_game_for_reflect,
                    CSODOTALobby::mut_save_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "first_blood_happened",
                    CSODOTALobby::get_first_blood_happened_for_reflect,
                    CSODOTALobby::mut_first_blood_happened_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::EMatchOutcome>>(
                    "match_outcome",
                    CSODOTALobby::get_match_outcome_for_reflect,
                    CSODOTALobby::mut_match_outcome_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "mass_disconnect",
                    CSODOTALobby::get_mass_disconnect_for_reflect,
                    CSODOTALobby::mut_mass_disconnect_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "custom_game_id",
                    CSODOTALobby::get_custom_game_id_for_reflect,
                    CSODOTALobby::mut_custom_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::EEvent>>(
                    "active_ingame_events",
                    CSODOTALobby::get_active_ingame_events_for_reflect,
                    CSODOTALobby::mut_active_ingame_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "custom_min_players",
                    CSODOTALobby::get_custom_min_players_for_reflect,
                    CSODOTALobby::mut_custom_min_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "custom_max_players",
                    CSODOTALobby::get_custom_max_players_for_reflect,
                    CSODOTALobby::mut_custom_max_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::gcsdk_gcmessages::PartnerAccountType>>(
                    "partner_type",
                    CSODOTALobby::get_partner_type_for_reflect,
                    CSODOTALobby::mut_partner_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lan_host_ping_to_server_region",
                    CSODOTALobby::get_lan_host_ping_to_server_region_for_reflect,
                    CSODOTALobby::mut_lan_host_ping_to_server_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTALobbyVisibility>>(
                    "visibility",
                    CSODOTALobby::get_visibility_for_reflect,
                    CSODOTALobby::mut_visibility_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "custom_game_crc",
                    CSODOTALobby::get_custom_game_crc_for_reflect,
                    CSODOTALobby::mut_custom_game_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "custom_game_auto_created_lobby",
                    CSODOTALobby::get_custom_game_auto_created_lobby_for_reflect,
                    CSODOTALobby::mut_custom_game_auto_created_lobby_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_series_id",
                    CSODOTALobby::get_league_series_id_for_reflect,
                    CSODOTALobby::mut_league_series_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_game_id",
                    CSODOTALobby::get_league_game_id_for_reflect,
                    CSODOTALobby::mut_league_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "custom_game_timestamp",
                    CSODOTALobby::get_custom_game_timestamp_for_reflect,
                    CSODOTALobby::mut_custom_game_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "previous_series_matches",
                    CSODOTALobby::get_previous_series_matches_for_reflect,
                    CSODOTALobby::mut_previous_series_matches_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "previous_match_override",
                    CSODOTALobby::get_previous_match_override_for_reflect,
                    CSODOTALobby::mut_previous_match_override_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "custom_game_uses_account_records",
                    CSODOTALobby::get_custom_game_uses_account_records_for_reflect,
                    CSODOTALobby::mut_custom_game_uses_account_records_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_selection_priority_team",
                    CSODOTALobby::get_league_selection_priority_team_for_reflect,
                    CSODOTALobby::mut_league_selection_priority_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::SelectionPriorityType>>(
                    "league_selection_priority_choice",
                    CSODOTALobby::get_league_selection_priority_choice_for_reflect,
                    CSODOTALobby::mut_league_selection_priority_choice_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::SelectionPriorityType>>(
                    "league_non_selection_priority_choice",
                    CSODOTALobby::get_league_non_selection_priority_choice_for_reflect,
                    CSODOTALobby::mut_league_non_selection_priority_choice_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_start_time",
                    CSODOTALobby::get_game_start_time_for_reflect,
                    CSODOTALobby::mut_game_start_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<LobbyDotaPauseSetting>>(
                    "pause_setting",
                    CSODOTALobby::get_pause_setting_for_reflect,
                    CSODOTALobby::mut_pause_setting_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lobby_mvp_account_id",
                    CSODOTALobby::get_lobby_mvp_account_id_for_reflect,
                    CSODOTALobby::mut_lobby_mvp_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_division_id",
                    CSODOTALobby::get_weekend_tourney_division_id_for_reflect,
                    CSODOTALobby::mut_weekend_tourney_division_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_skill_level",
                    CSODOTALobby::get_weekend_tourney_skill_level_for_reflect,
                    CSODOTALobby::mut_weekend_tourney_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_bracket_round",
                    CSODOTALobby::get_weekend_tourney_bracket_round_for_reflect,
                    CSODOTALobby::mut_weekend_tourney_bracket_round_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTABotDifficulty>>(
                    "bot_difficulty_dire",
                    CSODOTALobby::get_bot_difficulty_dire_for_reflect,
                    CSODOTALobby::mut_bot_difficulty_dire_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bot_radiant",
                    CSODOTALobby::get_bot_radiant_for_reflect,
                    CSODOTALobby::mut_bot_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bot_dire",
                    CSODOTALobby::get_bot_dire_for_reflect,
                    CSODOTALobby::mut_bot_dire_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSODOTALobby>(
                    "CSODOTALobby",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSODOTALobby {
    fn clear(&mut self) {
        self.clear_lobby_id();
        self.clear_members();
        self.clear_left_members();
        self.clear_leader_id();
        self.clear_server_id();
        self.clear_game_mode();
        self.clear_pending_invites();
        self.clear_state();
        self.clear_connect();
        self.clear_lobby_type();
        self.clear_allow_cheats();
        self.clear_fill_with_bots();
        self.clear_intro_mode();
        self.clear_game_name();
        self.clear_team_details();
        self.clear_tutorial_lesson();
        self.clear_tournament_id();
        self.clear_tournament_game_id();
        self.clear_server_region();
        self.clear_game_state();
        self.clear_num_spectators();
        self.clear_matchgroup();
        self.clear_cm_pick();
        self.clear_match_id();
        self.clear_allow_spectating();
        self.clear_bot_difficulty_radiant();
        self.clear_game_version();
        self.clear_timed_reward_details();
        self.clear_pass_key();
        self.clear_leagueid();
        self.clear_penalty_level_radiant();
        self.clear_penalty_level_dire();
        self.clear_load_game_id();
        self.clear_series_type();
        self.clear_radiant_series_wins();
        self.clear_dire_series_wins();
        self.clear_loot_generated();
        self.clear_loot_awarded();
        self.clear_allchat();
        self.clear_dota_tv_delay();
        self.clear_custom_game_mode();
        self.clear_custom_map_name();
        self.clear_custom_difficulty();
        self.clear_lan();
        self.clear_broadcast_channel_info();
        self.clear_first_leaver_accountid();
        self.clear_series_id();
        self.clear_low_priority();
        self.clear_extra_messages();
        self.clear_save_game();
        self.clear_first_blood_happened();
        self.clear_match_outcome();
        self.clear_mass_disconnect();
        self.clear_custom_game_id();
        self.clear_active_ingame_events();
        self.clear_custom_min_players();
        self.clear_custom_max_players();
        self.clear_partner_type();
        self.clear_lan_host_ping_to_server_region();
        self.clear_visibility();
        self.clear_custom_game_crc();
        self.clear_custom_game_auto_created_lobby();
        self.clear_league_series_id();
        self.clear_league_game_id();
        self.clear_custom_game_timestamp();
        self.clear_previous_series_matches();
        self.clear_previous_match_override();
        self.clear_custom_game_uses_account_records();
        self.clear_league_selection_priority_team();
        self.clear_league_selection_priority_choice();
        self.clear_league_non_selection_priority_choice();
        self.clear_game_start_time();
        self.clear_pause_setting();
        self.clear_lobby_mvp_account_id();
        self.clear_weekend_tourney_division_id();
        self.clear_weekend_tourney_skill_level();
        self.clear_weekend_tourney_bracket_round();
        self.clear_bot_difficulty_dire();
        self.clear_bot_radiant();
        self.clear_bot_dire();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSODOTALobby {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSODOTALobby {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSODOTALobby_CExtraMsg {
    // message fields
    id: ::std::option::Option<u32>,
    contents: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSODOTALobby_CExtraMsg {}

impl CSODOTALobby_CExtraMsg {
    pub fn new() -> CSODOTALobby_CExtraMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSODOTALobby_CExtraMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSODOTALobby_CExtraMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSODOTALobby_CExtraMsg,
        };
        unsafe {
            instance.get(CSODOTALobby_CExtraMsg::new)
        }
    }

    // optional uint32 id = 1;

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

    // optional bytes contents = 2;

    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    pub fn has_contents(&self) -> bool {
        self.contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contents(&mut self, v: ::std::vec::Vec<u8>) {
        self.contents = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contents(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.contents.is_none() {
            self.contents.set_default();
        };
        self.contents.as_mut().unwrap()
    }

    // Take field
    pub fn take_contents(&mut self) -> ::std::vec::Vec<u8> {
        self.contents.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_contents(&self) -> &[u8] {
        match self.contents.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_contents_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.contents
    }

    fn mut_contents_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.contents
    }
}

impl ::protobuf::Message for CSODOTALobby_CExtraMsg {
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
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.contents)?;
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
        if let Some(v) = self.contents.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.contents.as_ref() {
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

impl ::protobuf::MessageStatic for CSODOTALobby_CExtraMsg {
    fn new() -> CSODOTALobby_CExtraMsg {
        CSODOTALobby_CExtraMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSODOTALobby_CExtraMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    CSODOTALobby_CExtraMsg::get_id_for_reflect,
                    CSODOTALobby_CExtraMsg::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "contents",
                    CSODOTALobby_CExtraMsg::get_contents_for_reflect,
                    CSODOTALobby_CExtraMsg::mut_contents_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSODOTALobby_CExtraMsg>(
                    "CSODOTALobby_CExtraMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSODOTALobby_CExtraMsg {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_contents();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSODOTALobby_CExtraMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSODOTALobby_CExtraMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CSODOTALobby_State {
    UI = 0,
    READYUP = 4,
    SERVERSETUP = 1,
    RUN = 2,
    POSTGAME = 3,
    NOTREADY = 5,
    SERVERASSIGN = 6,
}

impl ::protobuf::ProtobufEnum for CSODOTALobby_State {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CSODOTALobby_State> {
        match value {
            0 => ::std::option::Option::Some(CSODOTALobby_State::UI),
            4 => ::std::option::Option::Some(CSODOTALobby_State::READYUP),
            1 => ::std::option::Option::Some(CSODOTALobby_State::SERVERSETUP),
            2 => ::std::option::Option::Some(CSODOTALobby_State::RUN),
            3 => ::std::option::Option::Some(CSODOTALobby_State::POSTGAME),
            5 => ::std::option::Option::Some(CSODOTALobby_State::NOTREADY),
            6 => ::std::option::Option::Some(CSODOTALobby_State::SERVERASSIGN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CSODOTALobby_State] = &[
            CSODOTALobby_State::UI,
            CSODOTALobby_State::READYUP,
            CSODOTALobby_State::SERVERSETUP,
            CSODOTALobby_State::RUN,
            CSODOTALobby_State::POSTGAME,
            CSODOTALobby_State::NOTREADY,
            CSODOTALobby_State::SERVERASSIGN,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CSODOTALobby_State>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CSODOTALobby_State", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CSODOTALobby_State {
}

impl ::protobuf::reflect::ProtobufValue for CSODOTALobby_State {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CSODOTALobby_LobbyType {
    INVALID = -1,
    CASUAL_MATCH = 0,
    PRACTICE = 1,
    TOURNAMENT = 2,
    COOP_BOT_MATCH = 4,
    LEGACY_TEAM_MATCH = 5,
    LEGACY_SOLO_QUEUE_MATCH = 6,
    COMPETITIVE_MATCH = 7,
    CASUAL_1V1_MATCH = 8,
    WEEKEND_TOURNEY = 9,
    LOCAL_BOT_MATCH = 10,
    SPECTATOR = 11,
}

impl ::protobuf::ProtobufEnum for CSODOTALobby_LobbyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CSODOTALobby_LobbyType> {
        match value {
            -1 => ::std::option::Option::Some(CSODOTALobby_LobbyType::INVALID),
            0 => ::std::option::Option::Some(CSODOTALobby_LobbyType::CASUAL_MATCH),
            1 => ::std::option::Option::Some(CSODOTALobby_LobbyType::PRACTICE),
            2 => ::std::option::Option::Some(CSODOTALobby_LobbyType::TOURNAMENT),
            4 => ::std::option::Option::Some(CSODOTALobby_LobbyType::COOP_BOT_MATCH),
            5 => ::std::option::Option::Some(CSODOTALobby_LobbyType::LEGACY_TEAM_MATCH),
            6 => ::std::option::Option::Some(CSODOTALobby_LobbyType::LEGACY_SOLO_QUEUE_MATCH),
            7 => ::std::option::Option::Some(CSODOTALobby_LobbyType::COMPETITIVE_MATCH),
            8 => ::std::option::Option::Some(CSODOTALobby_LobbyType::CASUAL_1V1_MATCH),
            9 => ::std::option::Option::Some(CSODOTALobby_LobbyType::WEEKEND_TOURNEY),
            10 => ::std::option::Option::Some(CSODOTALobby_LobbyType::LOCAL_BOT_MATCH),
            11 => ::std::option::Option::Some(CSODOTALobby_LobbyType::SPECTATOR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CSODOTALobby_LobbyType] = &[
            CSODOTALobby_LobbyType::INVALID,
            CSODOTALobby_LobbyType::CASUAL_MATCH,
            CSODOTALobby_LobbyType::PRACTICE,
            CSODOTALobby_LobbyType::TOURNAMENT,
            CSODOTALobby_LobbyType::COOP_BOT_MATCH,
            CSODOTALobby_LobbyType::LEGACY_TEAM_MATCH,
            CSODOTALobby_LobbyType::LEGACY_SOLO_QUEUE_MATCH,
            CSODOTALobby_LobbyType::COMPETITIVE_MATCH,
            CSODOTALobby_LobbyType::CASUAL_1V1_MATCH,
            CSODOTALobby_LobbyType::WEEKEND_TOURNEY,
            CSODOTALobby_LobbyType::LOCAL_BOT_MATCH,
            CSODOTALobby_LobbyType::SPECTATOR,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CSODOTALobby_LobbyType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CSODOTALobby_LobbyType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CSODOTALobby_LobbyType {
}

impl ::protobuf::reflect::ProtobufValue for CSODOTALobby_LobbyType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgLobbyPlaytestDetails {
    // message fields
    json: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgLobbyPlaytestDetails {}

impl CMsgLobbyPlaytestDetails {
    pub fn new() -> CMsgLobbyPlaytestDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgLobbyPlaytestDetails {
        static mut instance: ::protobuf::lazy::Lazy<CMsgLobbyPlaytestDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgLobbyPlaytestDetails,
        };
        unsafe {
            instance.get(CMsgLobbyPlaytestDetails::new)
        }
    }

    // optional string json = 1;

    pub fn clear_json(&mut self) {
        self.json.clear();
    }

    pub fn has_json(&self) -> bool {
        self.json.is_some()
    }

    // Param is passed by value, moved
    pub fn set_json(&mut self, v: ::std::string::String) {
        self.json = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_json(&mut self) -> &mut ::std::string::String {
        if self.json.is_none() {
            self.json.set_default();
        };
        self.json.as_mut().unwrap()
    }

    // Take field
    pub fn take_json(&mut self) -> ::std::string::String {
        self.json.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_json(&self) -> &str {
        match self.json.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_json_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.json
    }

    fn mut_json_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.json
    }
}

impl ::protobuf::Message for CMsgLobbyPlaytestDetails {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.json)?;
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
        if let Some(v) = self.json.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.json.as_ref() {
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

impl ::protobuf::MessageStatic for CMsgLobbyPlaytestDetails {
    fn new() -> CMsgLobbyPlaytestDetails {
        CMsgLobbyPlaytestDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgLobbyPlaytestDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "json",
                    CMsgLobbyPlaytestDetails::get_json_for_reflect,
                    CMsgLobbyPlaytestDetails::mut_json_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgLobbyPlaytestDetails>(
                    "CMsgLobbyPlaytestDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgLobbyPlaytestDetails {
    fn clear(&mut self) {
        self.clear_json();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgLobbyPlaytestDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgLobbyPlaytestDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum LobbyDotaTVDelay {
    LobbyDotaTV_10 = 0,
    LobbyDotaTV_120 = 1,
    LobbyDotaTV_300 = 2,
}

impl ::protobuf::ProtobufEnum for LobbyDotaTVDelay {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LobbyDotaTVDelay> {
        match value {
            0 => ::std::option::Option::Some(LobbyDotaTVDelay::LobbyDotaTV_10),
            1 => ::std::option::Option::Some(LobbyDotaTVDelay::LobbyDotaTV_120),
            2 => ::std::option::Option::Some(LobbyDotaTVDelay::LobbyDotaTV_300),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [LobbyDotaTVDelay] = &[
            LobbyDotaTVDelay::LobbyDotaTV_10,
            LobbyDotaTVDelay::LobbyDotaTV_120,
            LobbyDotaTVDelay::LobbyDotaTV_300,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<LobbyDotaTVDelay>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("LobbyDotaTVDelay", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for LobbyDotaTVDelay {
}

impl ::protobuf::reflect::ProtobufValue for LobbyDotaTVDelay {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum LobbyDotaPauseSetting {
    LobbyDotaPauseSetting_Unlimited = 0,
    LobbyDotaPauseSetting_Limited = 1,
    LobbyDotaPauseSetting_Disabled = 2,
}

impl ::protobuf::ProtobufEnum for LobbyDotaPauseSetting {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LobbyDotaPauseSetting> {
        match value {
            0 => ::std::option::Option::Some(LobbyDotaPauseSetting::LobbyDotaPauseSetting_Unlimited),
            1 => ::std::option::Option::Some(LobbyDotaPauseSetting::LobbyDotaPauseSetting_Limited),
            2 => ::std::option::Option::Some(LobbyDotaPauseSetting::LobbyDotaPauseSetting_Disabled),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [LobbyDotaPauseSetting] = &[
            LobbyDotaPauseSetting::LobbyDotaPauseSetting_Unlimited,
            LobbyDotaPauseSetting::LobbyDotaPauseSetting_Limited,
            LobbyDotaPauseSetting::LobbyDotaPauseSetting_Disabled,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<LobbyDotaPauseSetting>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("LobbyDotaPauseSetting", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for LobbyDotaPauseSetting {
}

impl ::protobuf::reflect::ProtobufValue for LobbyDotaPauseSetting {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x2d, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x6d,
    0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x13, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x67, 0x63, 0x73, 0x64,
    0x6b, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x17, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x5f,
    0x65, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa0, 0x03, 0x0a, 0x12,
    0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79, 0x4d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x12, 0x49, 0x0a, 0x0c, 0x70, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x50, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x79,
    0x70, 0x65, 0x3a, 0x0c, 0x50, 0x41, 0x52, 0x54, 0x4e, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x4e, 0x45,
    0x52, 0x0b, 0x70, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x19, 0x0a,
    0x08, 0x69, 0x73, 0x5f, 0x63, 0x6f, 0x61, 0x63, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x07, 0x69, 0x73, 0x43, 0x6f, 0x61, 0x63, 0x68, 0x12, 0x2e, 0x0a, 0x11, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x73, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x0d, 0x52, 0x0f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x43,
    0x6f, 0x64, 0x65, 0x73, 0x42, 0x02, 0x10, 0x01, 0x12, 0x2e, 0x0a, 0x11, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x18, 0x05, 0x20,
    0x03, 0x28, 0x0d, 0x52, 0x0f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x54,
    0x69, 0x6d, 0x65, 0x73, 0x42, 0x02, 0x10, 0x01, 0x12, 0x3b, 0x0a, 0x1a, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x5f, 0x62,
    0x69, 0x74, 0x6d, 0x61, 0x73, 0x6b, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x17, 0x72, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x50, 0x69, 0x6e, 0x67, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x42, 0x69,
    0x74, 0x6d, 0x61, 0x73, 0x6b, 0x12, 0x2e, 0x0a, 0x13, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79,
    0x5f, 0x73, 0x6b, 0x69, 0x6c, 0x6c, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x11, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x53, 0x6b, 0x69, 0x6c, 0x6c,
    0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x23, 0x0a, 0x0d, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79,
    0x5f, 0x62, 0x75, 0x79, 0x69, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x74, 0x6f,
    0x75, 0x72, 0x6e, 0x65, 0x79, 0x42, 0x75, 0x79, 0x69, 0x6e, 0x12, 0x32, 0x0a, 0x15, 0x74, 0x6f,
    0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f, 0x70, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x75, 0x6e,
    0x74, 0x69, 0x6c, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x13, 0x74, 0x6f, 0x75, 0x72, 0x6e,
    0x65, 0x79, 0x50, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x55, 0x6e, 0x74, 0x69, 0x6c, 0x22, 0xc5,
    0x11, 0x0a, 0x0c, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79, 0x12,
    0x1f, 0x0a, 0x08, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x07, 0x70, 0x61, 0x72, 0x74, 0x79, 0x49, 0x64, 0x42, 0x04, 0x80, 0xa6, 0x1d, 0x01,
    0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x08, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1d, 0x0a,
    0x0a, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x06, 0x52, 0x09, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x49, 0x64, 0x73, 0x12, 0x1d, 0x0a, 0x0a,
    0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x73, 0x12, 0x32, 0x0a, 0x05, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79, 0x2e, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x3a, 0x02, 0x55, 0x49, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12,
    0x4b, 0x0a, 0x22, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x65, 0x64, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x6d, 0x61, 0x6b, 0x69, 0x6e, 0x67,
    0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1f, 0x65, 0x66, 0x66,
    0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x53, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x6d, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x3f, 0x0a, 0x1c,
    0x72, 0x61, 0x77, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x5f, 0x6d, 0x61, 0x74, 0x63,
    0x68, 0x6d, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x20, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x19, 0x72, 0x61, 0x77, 0x53, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x6d, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x2c, 0x0a,
    0x12, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x18, 0x21, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x61, 0x74, 0x74, 0x65, 0x6d,
    0x70, 0x74, 0x53, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x61,
    0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x22, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0a, 0x61, 0x74, 0x74, 0x65, 0x6d, 0x70, 0x74, 0x4e, 0x75, 0x6d, 0x12, 0x20, 0x0a, 0x0b,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0b, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x12, 0x35,
    0x0a, 0x17, 0x6c, 0x6f, 0x77, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x14, 0x6c, 0x6f, 0x77, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x41, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x41, 0x0a, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0f, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x11, 0x4d, 0x41, 0x54, 0x43,
    0x48, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x43, 0x41, 0x53, 0x55, 0x41, 0x4c, 0x52, 0x09, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x54, 0x79, 0x70, 0x65, 0x12, 0x56, 0x0a, 0x0e, 0x62, 0x6f, 0x74, 0x5f,
    0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x42, 0x6f, 0x74, 0x44,
    0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x3a, 0x16, 0x42, 0x4f, 0x54, 0x5f, 0x44,
    0x49, 0x46, 0x46, 0x49, 0x43, 0x55, 0x4c, 0x54, 0x59, 0x5f, 0x50, 0x41, 0x53, 0x53, 0x49, 0x56,
    0x45, 0x52, 0x0d, 0x62, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79,
    0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x17, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x74, 0x65, 0x61,
    0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x33, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x74, 0x65,
    0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x20, 0x0a, 0x0c, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x75,
    0x69, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x34, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x74, 0x65,
    0x61, 0x6d, 0x55, 0x69, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x24, 0x0a, 0x0e, 0x74, 0x65, 0x61, 0x6d,
    0x5f, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x35, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x0c, 0x74, 0x65, 0x61, 0x6d, 0x42, 0x61, 0x73, 0x65, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x39,
    0x0a, 0x19, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64,
    0x5f, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x5f, 0x64, 0x61, 0x74, 0x65, 0x18, 0x18, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x16, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64,
    0x55, 0x6e, 0x74, 0x69, 0x6c, 0x44, 0x61, 0x74, 0x65, 0x12, 0x39, 0x0a, 0x19, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x5f, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x19, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x16, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x41, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x41, 0x0a, 0x1d, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x6d, 0x61, 0x6b,
    0x69, 0x6e, 0x67, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x6d, 0x69,
    0x6e, 0x75, 0x74, 0x65, 0x73, 0x18, 0x1a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1a, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x6d, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x4d, 0x61, 0x78, 0x52, 0x61, 0x6e, 0x67, 0x65,
    0x4d, 0x69, 0x6e, 0x75, 0x74, 0x65, 0x73, 0x12, 0x26, 0x0a, 0x0e, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x18, 0x1b, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x73, 0x12,
    0x25, 0x0a, 0x0e, 0x6d, 0x61, 0x70, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63,
    0x65, 0x18, 0x26, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x6d, 0x61, 0x70, 0x50, 0x72, 0x65, 0x66,
    0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x32, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x18, 0x1d, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x12, 0x22, 0x0a, 0x0d, 0x6f, 0x70,
    0x65, 0x6e, 0x5f, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x1e, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0b, 0x6f, 0x70, 0x65, 0x6e, 0x47, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x23,
    0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x73, 0x18,
    0x1f, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x47, 0x75, 0x69,
    0x6c, 0x64, 0x73, 0x12, 0x3f, 0x0a, 0x1c, 0x6c, 0x6f, 0x77, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72,
    0x69, 0x74, 0x79, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x5f, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e,
    0x69, 0x6e, 0x67, 0x18, 0x23, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x19, 0x6c, 0x6f, 0x77, 0x50, 0x72,
    0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x6d, 0x61, 0x69,
    0x6e, 0x69, 0x6e, 0x67, 0x12, 0x3e, 0x0a, 0x14, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x69,
    0x6e, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x27, 0x20, 0x03,
    0x28, 0x0e, 0x32, 0x0c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x52, 0x12, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x49, 0x6e, 0x67, 0x61, 0x6d, 0x65, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x73, 0x12, 0x33, 0x0a, 0x16, 0x6f, 0x70, 0x65, 0x6e, 0x5f, 0x66, 0x6f, 0x72,
    0x5f, 0x6a, 0x6f, 0x69, 0x6e, 0x5f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x18, 0x28,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x13, 0x6f, 0x70, 0x65, 0x6e, 0x46, 0x6f, 0x72, 0x4a, 0x6f, 0x69,
    0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x12, 0x3b, 0x0a, 0x0c, 0x73, 0x65, 0x6e,
    0x74, 0x5f, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x73, 0x18, 0x29, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61,
    0x72, 0x74, 0x79, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x52, 0x0b, 0x73, 0x65, 0x6e, 0x74, 0x49,
    0x6e, 0x76, 0x69, 0x74, 0x65, 0x73, 0x12, 0x3b, 0x0a, 0x0c, 0x72, 0x65, 0x63, 0x76, 0x5f, 0x69,
    0x6e, 0x76, 0x69, 0x74, 0x65, 0x73, 0x18, 0x2a, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79,
    0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x52, 0x0b, 0x72, 0x65, 0x63, 0x76, 0x49, 0x6e, 0x76, 0x69,
    0x74, 0x65, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x66,
    0x6c, 0x61, 0x67, 0x73, 0x18, 0x2b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x2e, 0x0a, 0x13, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x5f, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18,
    0x2c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x53, 0x65, 0x6c,
    0x65, 0x63, 0x74, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x36, 0x0a, 0x17, 0x65, 0x78, 0x63, 0x6c,
    0x75, 0x73, 0x69, 0x76, 0x65, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x2d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x15, 0x65, 0x78, 0x63, 0x6c, 0x75,
    0x73, 0x69, 0x76, 0x65, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64,
    0x12, 0x2e, 0x0a, 0x13, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f, 0x64, 0x69, 0x76, 0x69,
    0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x2f, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x74,
    0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x44, 0x69, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x64,
    0x12, 0x32, 0x0a, 0x15, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f, 0x73, 0x63, 0x68, 0x65,
    0x64, 0x75, 0x6c, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x30, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x13, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65,
    0x54, 0x69, 0x6d, 0x65, 0x12, 0x2e, 0x0a, 0x13, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f,
    0x73, 0x6b, 0x69, 0x6c, 0x6c, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x31, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x11, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x53, 0x6b, 0x69, 0x6c, 0x6c, 0x4c,
    0x65, 0x76, 0x65, 0x6c, 0x12, 0x32, 0x0a, 0x15, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f,
    0x62, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x32, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x13, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x42, 0x72, 0x61, 0x63,
    0x6b, 0x65, 0x74, 0x52, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x3d, 0x0a, 0x1b, 0x74, 0x6f, 0x75, 0x72,
    0x6e, 0x65, 0x79, 0x5f, 0x71, 0x75, 0x65, 0x75, 0x65, 0x5f, 0x64, 0x65, 0x61, 0x64, 0x6c, 0x69,
    0x6e, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x36, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x18, 0x74,
    0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x51, 0x75, 0x65, 0x75, 0x65, 0x44, 0x65, 0x61, 0x64, 0x6c,
    0x69, 0x6e, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x86, 0x01, 0x0a, 0x1c, 0x74, 0x6f, 0x75, 0x72,
    0x6e, 0x65, 0x79, 0x5f, 0x71, 0x75, 0x65, 0x75, 0x65, 0x5f, 0x64, 0x65, 0x61, 0x64, 0x6c, 0x69,
    0x6e, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x37, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x20,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x51, 0x75,
    0x65, 0x75, 0x65, 0x44, 0x65, 0x61, 0x64, 0x6c, 0x69, 0x6e, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x3a, 0x23, 0x6b, 0x5f, 0x45, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x51, 0x75, 0x65, 0x75,
    0x65, 0x44, 0x65, 0x61, 0x64, 0x6c, 0x69, 0x6e, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x5f, 0x4e,
    0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x52, 0x19, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x51, 0x75,
    0x65, 0x75, 0x65, 0x44, 0x65, 0x61, 0x64, 0x6c, 0x69, 0x6e, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x12, 0x3c, 0x0a, 0x1b, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x65,
    0x72, 0x5f, 0x73, 0x6c, 0x6f, 0x74, 0x73, 0x5f, 0x74, 0x6f, 0x5f, 0x66, 0x69, 0x6c, 0x6c, 0x18,
    0x38, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x17, 0x70, 0x61, 0x72, 0x74, 0x79, 0x42, 0x75, 0x69, 0x6c,
    0x64, 0x65, 0x72, 0x53, 0x6c, 0x6f, 0x74, 0x73, 0x54, 0x6f, 0x46, 0x69, 0x6c, 0x6c, 0x12, 0x3b,
    0x0a, 0x1a, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x65, 0x72, 0x5f,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x18, 0x39, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x17, 0x70, 0x61, 0x72, 0x74, 0x79, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x65, 0x72,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x73, 0x12, 0x37, 0x0a, 0x18, 0x70,
    0x61, 0x72, 0x74, 0x79, 0x5f, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x3a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x15, 0x70,
    0x61, 0x72, 0x74, 0x79, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x65, 0x72, 0x53, 0x74, 0x61, 0x72, 0x74,
    0x54, 0x69, 0x6d, 0x65, 0x22, 0x30, 0x0a, 0x05, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x06, 0x0a,
    0x02, 0x55, 0x49, 0x10, 0x00, 0x12, 0x11, 0x0a, 0x0d, 0x46, 0x49, 0x4e, 0x44, 0x49, 0x4e, 0x47,
    0x5f, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x10, 0x01, 0x12, 0x0c, 0x0a, 0x08, 0x49, 0x4e, 0x5f, 0x4d,
    0x41, 0x54, 0x43, 0x48, 0x10, 0x02, 0x22, 0x8f, 0x03, 0x0a, 0x12, 0x43, 0x53, 0x4f, 0x44, 0x4f,
    0x54, 0x41, 0x50, 0x61, 0x72, 0x74, 0x79, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x12, 0x1f, 0x0a,
    0x08, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x07, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x49, 0x64, 0x42, 0x04, 0x80, 0xa6, 0x1d, 0x01, 0x12, 0x1b,
    0x0a, 0x09, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x06, 0x52, 0x08, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x73,
    0x65, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x0a, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x3e, 0x0a, 0x07,
    0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x50, 0x61, 0x72, 0x74,
    0x79, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x79, 0x4d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x12, 0x17, 0x0a, 0x07,
    0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74,
    0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x2e, 0x0a, 0x13, 0x6c, 0x6f, 0x77, 0x5f, 0x70, 0x72, 0x69,
    0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x11, 0x6c, 0x6f, 0x77, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x61, 0x73, 0x5f, 0x63, 0x6f, 0x61, 0x63,
    0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x61, 0x73, 0x43, 0x6f, 0x61, 0x63, 0x68,
    0x12, 0x1d, 0x0a, 0x0a, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x67, 0x69, 0x64, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x47, 0x69, 0x64, 0x1a,
    0x57, 0x0a, 0x0b, 0x50, 0x61, 0x72, 0x74, 0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x12,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x19, 0x0a,
    0x08, 0x69, 0x73, 0x5f, 0x63, 0x6f, 0x61, 0x63, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x07, 0x69, 0x73, 0x43, 0x6f, 0x61, 0x63, 0x68, 0x22, 0x92, 0x03, 0x0a, 0x12, 0x43, 0x53, 0x4f,
    0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x12,
    0x1f, 0x0a, 0x08, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x07, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x49, 0x64, 0x42, 0x04, 0x80, 0xa6, 0x1d, 0x01,
    0x12, 0x1b, 0x0a, 0x09, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x08, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1f, 0x0a,
    0x0b, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0a, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x3e,
    0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x24, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f,
    0x62, 0x62, 0x79, 0x49, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x2e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x12, 0x24,
    0x0a, 0x0e, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61,
    0x6d, 0x65, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x5f, 0x67,
    0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x06, 0x52, 0x09, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65,
    0x47, 0x69, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61,
    0x6d, 0x65, 0x5f, 0x63, 0x72, 0x63, 0x18, 0x07, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x63, 0x75,
    0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x43, 0x72, 0x63, 0x12, 0x32, 0x0a, 0x15, 0x63,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x18, 0x08, 0x20, 0x01, 0x28, 0x07, 0x52, 0x13, 0x63, 0x75, 0x73, 0x74,
    0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x1a,
    0x3c, 0x0a, 0x0b, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x12,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x22, 0xba, 0x02,
    0x0a, 0x0f, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x65, 0x61, 0x76, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x12, 0x4e, 0x0a, 0x0a, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x19, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x47, 0x41, 0x4d, 0x45, 0x52, 0x55, 0x4c, 0x45, 0x53, 0x5f, 0x53, 0x54, 0x41,
    0x54, 0x45, 0x5f, 0x49, 0x4e, 0x49, 0x54, 0x52, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x6c, 0x65, 0x61, 0x76, 0x65, 0x72, 0x5f, 0x64, 0x65, 0x74,
    0x65, 0x63, 0x74, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x6c, 0x65, 0x61,
    0x76, 0x65, 0x72, 0x44, 0x65, 0x74, 0x65, 0x63, 0x74, 0x65, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x66,
    0x69, 0x72, 0x73, 0x74, 0x5f, 0x62, 0x6c, 0x6f, 0x6f, 0x64, 0x5f, 0x68, 0x61, 0x70, 0x70, 0x65,
    0x6e, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x66, 0x69, 0x72, 0x73, 0x74,
    0x42, 0x6c, 0x6f, 0x6f, 0x64, 0x48, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x65, 0x64, 0x12, 0x32, 0x0a,
    0x15, 0x64, 0x69, 0x73, 0x63, 0x61, 0x72, 0x64, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x13, 0x64, 0x69,
    0x73, 0x63, 0x61, 0x72, 0x64, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x73, 0x12, 0x27, 0x0a, 0x0f, 0x6d, 0x61, 0x73, 0x73, 0x5f, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x6d, 0x61, 0x73, 0x73,
    0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x22, 0x9b, 0x0a, 0x0a, 0x10, 0x43,
    0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12,
    0x14, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x02, 0x69, 0x64, 0x42,
    0x04, 0x80, 0xa6, 0x1d, 0x01, 0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49, 0x64, 0x12, 0x3e,
    0x0a, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41, 0x4d,
    0x3a, 0x16, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x47,
    0x4f, 0x4f, 0x44, 0x5f, 0x47, 0x55, 0x59, 0x53, 0x52, 0x04, 0x74, 0x65, 0x61, 0x6d, 0x12, 0x12,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x04, 0x73, 0x6c, 0x6f, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x61, 0x72, 0x74, 0x79, 0x5f,
    0x69, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x70, 0x61, 0x72, 0x74, 0x79, 0x49,
    0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x65, 0x74, 0x61, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x6d, 0x65, 0x74, 0x61, 0x4c, 0x65, 0x76, 0x65, 0x6c,
    0x12, 0x17, 0x0a, 0x07, 0x6d, 0x65, 0x74, 0x61, 0x5f, 0x78, 0x70, 0x18, 0x0e, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x06, 0x6d, 0x65, 0x74, 0x61, 0x58, 0x70, 0x12, 0x26, 0x0a, 0x0f, 0x6d, 0x65, 0x74,
    0x61, 0x5f, 0x78, 0x70, 0x5f, 0x61, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x18, 0x0f, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0d, 0x6d, 0x65, 0x74, 0x61, 0x58, 0x70, 0x41, 0x77, 0x61, 0x72, 0x64, 0x65,
    0x64, 0x12, 0x4f, 0x0a, 0x0d, 0x6c, 0x65, 0x61, 0x76, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x44, 0x4f, 0x54, 0x41, 0x4c, 0x65, 0x61, 0x76, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x5f, 0x74, 0x3a, 0x10, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4c, 0x45, 0x41, 0x56, 0x45, 0x52, 0x5f,
    0x4e, 0x4f, 0x4e, 0x45, 0x52, 0x0c, 0x6c, 0x65, 0x61, 0x76, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x6c, 0x65, 0x61, 0x76, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x18, 0x1c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x6c, 0x65, 0x61, 0x76,
    0x65, 0x72, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x12, 0x26, 0x0a, 0x0f, 0x70, 0x72, 0x69, 0x7a, 0x65, 0x5f, 0x64, 0x65, 0x66,
    0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x70, 0x72,
    0x69, 0x7a, 0x65, 0x44, 0x65, 0x66, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x28, 0x0a, 0x10, 0x64,
    0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64, 0x18,
    0x14, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0e, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x48,
    0x65, 0x72, 0x6f, 0x49, 0x64, 0x12, 0x58, 0x0a, 0x14, 0x70, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72,
    0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x15, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x6e,
    0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x0c, 0x50,
    0x41, 0x52, 0x54, 0x4e, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x52, 0x12, 0x70, 0x61, 0x72,
    0x74, 0x6e, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x26, 0x0a, 0x0f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x72, 0x6f, 0x5f,
    0x69, 0x64, 0x18, 0x16, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0d, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65,
    0x64, 0x48, 0x65, 0x72, 0x6f, 0x49, 0x64, 0x12, 0x46, 0x0a, 0x0a, 0x63, 0x6f, 0x61, 0x63, 0x68,
    0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x17, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x3a,
    0x13, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x43, 0x5f, 0x54, 0x45, 0x41, 0x4d, 0x5f, 0x4e, 0x4f,
    0x54, 0x45, 0x41, 0x4d, 0x52, 0x09, 0x63, 0x6f, 0x61, 0x63, 0x68, 0x54, 0x65, 0x61, 0x6d, 0x12,
    0x27, 0x0a, 0x10, 0x6e, 0x65, 0x78, 0x6f, 0x6e, 0x5f, 0x70, 0x63, 0x5f, 0x62, 0x61, 0x6e, 0x67,
    0x5f, 0x6e, 0x6f, 0x18, 0x18, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x6e, 0x65, 0x78, 0x6f, 0x6e,
    0x50, 0x63, 0x42, 0x61, 0x6e, 0x67, 0x4e, 0x6f, 0x12, 0x2b, 0x0a, 0x12, 0x6e, 0x65, 0x78, 0x6f,
    0x6e, 0x5f, 0x70, 0x63, 0x5f, 0x62, 0x61, 0x6e, 0x67, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x19,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x6e, 0x65, 0x78, 0x6f, 0x6e, 0x50, 0x63, 0x42, 0x61, 0x6e,
    0x67, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x4d, 0x0a, 0x0a, 0x78, 0x70, 0x5f, 0x62, 0x6f, 0x6e, 0x75,
    0x73, 0x65, 0x73, 0x18, 0x1b, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x58, 0x50, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x52, 0x09, 0x78, 0x70, 0x42, 0x6f, 0x6e,
    0x75, 0x73, 0x65, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x61, 0x6e, 0x6b, 0x5f, 0x63, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x11, 0x52, 0x0a, 0x72, 0x61, 0x6e, 0x6b, 0x43,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61, 0x6d,
    0x61, 0x6e, 0x18, 0x1e, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x63, 0x61, 0x6d, 0x65, 0x72, 0x61,
    0x6d, 0x61, 0x6e, 0x12, 0x35, 0x0a, 0x17, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61,
    0x6d, 0x65, 0x5f, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x1f,
    0x20, 0x03, 0x28, 0x0d, 0x52, 0x14, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65,
    0x50, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x49, 0x64, 0x73, 0x12, 0x38, 0x0a, 0x19, 0x6c, 0x6f,
    0x62, 0x62, 0x79, 0x5f, 0x6d, 0x76, 0x70, 0x5f, 0x76, 0x6f, 0x74, 0x65, 0x5f, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x20, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x15, 0x6c,
    0x6f, 0x62, 0x62, 0x79, 0x4d, 0x76, 0x70, 0x56, 0x6f, 0x74, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x4e, 0x0a, 0x11, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x21, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x0f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x54, 0x79, 0x70, 0x65,
    0x3a, 0x11, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x43, 0x41, 0x53,
    0x55, 0x41, 0x4c, 0x52, 0x0f, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4d, 0x61, 0x74, 0x63, 0x68,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x39, 0x0a, 0x19, 0x66, 0x61, 0x76, 0x6f, 0x72, 0x69, 0x74, 0x65,
    0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x61, 0x6e, 0x64, 0x5f, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x74,
    0x79, 0x18, 0x22, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x16, 0x66, 0x61, 0x76, 0x6f, 0x72, 0x69, 0x74,
    0x65, 0x54, 0x65, 0x61, 0x6d, 0x41, 0x6e, 0x64, 0x51, 0x75, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x1a,
    0x67, 0x0a, 0x17, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x58, 0x50, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x19,
    0x0a, 0x08, 0x78, 0x70, 0x5f, 0x62, 0x6f, 0x6e, 0x75, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x02,
    0x52, 0x07, 0x78, 0x70, 0x42, 0x6f, 0x6e, 0x75, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x4b, 0x65, 0x79, 0x22, 0x97, 0x04, 0x0a, 0x11, 0x43, 0x4c, 0x6f,
    0x62, 0x62, 0x79, 0x54, 0x65, 0x61, 0x6d, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x1b,
    0x0a, 0x09, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x74, 0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x74,
    0x65, 0x61, 0x6d, 0x5f, 0x74, 0x61, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x74,
    0x65, 0x61, 0x6d, 0x54, 0x61, 0x67, 0x12, 0x17, 0x0a, 0x07, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69,
    0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12,
    0x1b, 0x0a, 0x09, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x08, 0x74, 0x65, 0x61, 0x6d, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x24, 0x0a, 0x0e,
    0x74, 0x65, 0x61, 0x6d, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x74, 0x65, 0x61, 0x6d, 0x42, 0x61, 0x73, 0x65, 0x4c, 0x6f,
    0x67, 0x6f, 0x12, 0x28, 0x0a, 0x10, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x62, 0x61, 0x6e, 0x6e, 0x65,
    0x72, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0e, 0x74, 0x65,
    0x61, 0x6d, 0x42, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x23, 0x0a, 0x0d,
    0x74, 0x65, 0x61, 0x6d, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x0c, 0x74, 0x65, 0x61, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74,
    0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x4e, 0x61, 0x6d, 0x65,
    0x12, 0x1b, 0x0a, 0x09, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x74, 0x61, 0x67, 0x18, 0x0a, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x54, 0x61, 0x67, 0x12, 0x19, 0x0a,
    0x08, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x07, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x67, 0x75, 0x69, 0x6c,
    0x64, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x67, 0x75,
    0x69, 0x6c, 0x64, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x26, 0x0a, 0x0f, 0x67, 0x75, 0x69, 0x6c, 0x64,
    0x5f, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x0d, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x42, 0x61, 0x73, 0x65, 0x4c, 0x6f, 0x67, 0x6f, 0x12,
    0x2a, 0x0a, 0x11, 0x67, 0x75, 0x69, 0x6c, 0x64, 0x5f, 0x62, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x5f,
    0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0f, 0x67, 0x75, 0x69, 0x6c,
    0x64, 0x42, 0x61, 0x6e, 0x6e, 0x65, 0x72, 0x4c, 0x6f, 0x67, 0x6f, 0x12, 0x12, 0x0a, 0x04, 0x72,
    0x61, 0x6e, 0x6b, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x04, 0x72, 0x61, 0x6e, 0x6b, 0x12,
    0x1f, 0x0a, 0x0b, 0x72, 0x61, 0x6e, 0x6b, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x10,
    0x20, 0x01, 0x28, 0x11, 0x52, 0x0a, 0x72, 0x61, 0x6e, 0x6b, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x12, 0x20, 0x0a, 0x0c, 0x69, 0x73, 0x5f, 0x68, 0x6f, 0x6d, 0x65, 0x5f, 0x74, 0x65, 0x61, 0x6d,
    0x18, 0x11, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x69, 0x73, 0x48, 0x6f, 0x6d, 0x65, 0x54, 0x65,
    0x61, 0x6d, 0x22, 0xc3, 0x01, 0x0a, 0x18, 0x43, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x54, 0x69, 0x6d,
    0x65, 0x64, 0x52, 0x65, 0x77, 0x61, 0x72, 0x64, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12,
    0x24, 0x0a, 0x0e, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x64, 0x65, 0x66, 0x5f, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x69, 0x74, 0x65, 0x6d, 0x44, 0x65, 0x66,
    0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x26, 0x0a, 0x0f, 0x69, 0x73, 0x5f, 0x73, 0x75, 0x70, 0x70,
    0x6c, 0x79, 0x5f, 0x63, 0x72, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0d,
    0x69, 0x73, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x43, 0x72, 0x61, 0x74, 0x65, 0x12, 0x22, 0x0a,
    0x0d, 0x69, 0x73, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x64, 0x5f, 0x64, 0x72, 0x6f, 0x70, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x69, 0x73, 0x54, 0x69, 0x6d, 0x65, 0x64, 0x44, 0x72, 0x6f,
    0x70, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64,
    0x12, 0x16, 0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x22, 0xa5, 0x01, 0x0a, 0x1a, 0x43, 0x4c, 0x6f,
    0x62, 0x62, 0x79, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x6e,
    0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x63, 0x68, 0x61,
    0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72,
    0x79, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x72, 0x79, 0x43, 0x6f, 0x64, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x23, 0x0a, 0x0d, 0x6c,
    0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0c, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x64, 0x65,
    0x22, 0xea, 0x23, 0x0a, 0x0c, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62,
    0x79, 0x12, 0x1f, 0x0a, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x42, 0x04, 0x80, 0xa6,
    0x1d, 0x01, 0x12, 0x30, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x12, 0x39, 0x0a, 0x0c, 0x6c, 0x65, 0x66, 0x74, 0x5f, 0x6d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x52, 0x0b, 0x6c, 0x65, 0x66, 0x74, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x12,
    0x1b, 0x0a, 0x09, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x0b, 0x20, 0x01,
    0x28, 0x06, 0x52, 0x08, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1e, 0x0a, 0x09,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x06, 0x3a,
    0x01, 0x30, 0x52, 0x08, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09,
    0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x08, 0x67, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x65, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x5f, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x73, 0x18, 0x0a, 0x20, 0x03,
    0x28, 0x06, 0x52, 0x0e, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x49, 0x6e, 0x76, 0x69, 0x74,
    0x65, 0x73, 0x12, 0x32, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41,
    0x4c, 0x6f, 0x62, 0x62, 0x79, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x02, 0x55, 0x49, 0x52,
    0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x12, 0x44, 0x0a, 0x0a, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x0c,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x4f, 0x44,
    0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x2e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x54, 0x79,
    0x70, 0x65, 0x3a, 0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44, 0x52, 0x09, 0x6c, 0x6f, 0x62,
    0x62, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x5f,
    0x63, 0x68, 0x65, 0x61, 0x74, 0x73, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x43, 0x68, 0x65, 0x61, 0x74, 0x73, 0x12, 0x24, 0x0a, 0x0e, 0x66, 0x69, 0x6c,
    0x6c, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x5f, 0x62, 0x6f, 0x74, 0x73, 0x18, 0x0e, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0c, 0x66, 0x69, 0x6c, 0x6c, 0x57, 0x69, 0x74, 0x68, 0x42, 0x6f, 0x74, 0x73, 0x12,
    0x1d, 0x0a, 0x0a, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x0f, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x09, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x1b,
    0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x3a, 0x0a, 0x0c, 0x74,
    0x65, 0x61, 0x6d, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x11, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x54,
    0x65, 0x61, 0x6d, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x0b, 0x74, 0x65, 0x61, 0x6d,
    0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x74, 0x75, 0x74, 0x6f, 0x72,
    0x69, 0x61, 0x6c, 0x5f, 0x6c, 0x65, 0x73, 0x73, 0x6f, 0x6e, 0x18, 0x12, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0e, 0x74, 0x75, 0x74, 0x6f, 0x72, 0x69, 0x61, 0x6c, 0x4c, 0x65, 0x73, 0x73, 0x6f, 0x6e,
    0x12, 0x23, 0x0a, 0x0d, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69,
    0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d,
    0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x2c, 0x0a, 0x12, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d,
    0x65, 0x6e, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x14, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x10, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x47, 0x61, 0x6d,
    0x65, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x72, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x30, 0x52, 0x0c, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x4e, 0x0a, 0x0a, 0x67,
    0x61, 0x6d, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x16, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x14, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x61, 0x6d, 0x65,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x19, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x41, 0x4d, 0x45,
    0x52, 0x55, 0x4c, 0x45, 0x53, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x45, 0x5f, 0x49, 0x4e, 0x49, 0x54,
    0x52, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x6e,
    0x75, 0x6d, 0x5f, 0x73, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x17, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0d, 0x6e, 0x75, 0x6d, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x6f,
    0x72, 0x73, 0x12, 0x1e, 0x0a, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x67, 0x72, 0x6f, 0x75, 0x70,
    0x18, 0x19, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x12, 0x3b, 0x0a, 0x07, 0x63, 0x6d, 0x5f, 0x70, 0x69, 0x63, 0x6b, 0x18, 0x1c, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x43, 0x4d, 0x5f, 0x50, 0x49, 0x43, 0x4b, 0x3a, 0x0e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x43, 0x4d,
    0x5f, 0x52, 0x41, 0x4e, 0x44, 0x4f, 0x4d, 0x52, 0x06, 0x63, 0x6d, 0x50, 0x69, 0x63, 0x6b, 0x12,
    0x19, 0x0a, 0x08, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x1e, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x12, 0x2f, 0x0a, 0x10, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x5f, 0x73, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x1f,
    0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x52, 0x0f, 0x61, 0x6c, 0x6c, 0x6f,
    0x77, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x12, 0x62, 0x0a, 0x16, 0x62,
    0x6f, 0x74, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x5f, 0x72, 0x61,
    0x64, 0x69, 0x61, 0x6e, 0x74, 0x18, 0x24, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x42, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63,
    0x75, 0x6c, 0x74, 0x79, 0x3a, 0x13, 0x42, 0x4f, 0x54, 0x5f, 0x44, 0x49, 0x46, 0x46, 0x49, 0x43,
    0x55, 0x4c, 0x54, 0x59, 0x5f, 0x48, 0x41, 0x52, 0x44, 0x52, 0x14, 0x62, 0x6f, 0x74, 0x44, 0x69,
    0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x52, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x12,
    0x4e, 0x0a, 0x0c, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x25, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x15, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54,
    0x41, 0x47, 0x61, 0x6d, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x14, 0x47, 0x41,
    0x4d, 0x45, 0x5f, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x55, 0x52, 0x52, 0x45,
    0x4e, 0x54, 0x52, 0x0b, 0x67, 0x61, 0x6d, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x50, 0x0a, 0x14, 0x74, 0x69, 0x6d, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x77, 0x61, 0x72, 0x64, 0x5f,
    0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x18, 0x26, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x54, 0x69, 0x6d, 0x65, 0x64,
    0x52, 0x65, 0x77, 0x61, 0x72, 0x64, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x12, 0x74,
    0x69, 0x6d, 0x65, 0x64, 0x52, 0x65, 0x77, 0x61, 0x72, 0x64, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c,
    0x73, 0x12, 0x19, 0x0a, 0x08, 0x70, 0x61, 0x73, 0x73, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x27, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x07, 0x70, 0x61, 0x73, 0x73, 0x4b, 0x65, 0x79, 0x12, 0x1a, 0x0a, 0x08,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x69, 0x64, 0x18, 0x2a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x69, 0x64, 0x12, 0x35, 0x0a, 0x15, 0x70, 0x65, 0x6e, 0x61,
    0x6c, 0x74, 0x79, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e,
    0x74, 0x18, 0x2b, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x30, 0x52, 0x13, 0x70, 0x65, 0x6e, 0x61,
    0x6c, 0x74, 0x79, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x52, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x12,
    0x2f, 0x0a, 0x12, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c,
    0x5f, 0x64, 0x69, 0x72, 0x65, 0x18, 0x2c, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x30, 0x52, 0x10,
    0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74, 0x79, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x44, 0x69, 0x72, 0x65,
    0x12, 0x20, 0x0a, 0x0c, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x2d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x6c, 0x6f, 0x61, 0x64, 0x47, 0x61, 0x6d, 0x65,
    0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x2e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x2e, 0x0a, 0x13, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x5f, 0x73,
    0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x77, 0x69, 0x6e, 0x73, 0x18, 0x2f, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x11, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x57,
    0x69, 0x6e, 0x73, 0x12, 0x28, 0x0a, 0x10, 0x64, 0x69, 0x72, 0x65, 0x5f, 0x73, 0x65, 0x72, 0x69,
    0x65, 0x73, 0x5f, 0x77, 0x69, 0x6e, 0x73, 0x18, 0x30, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x64,
    0x69, 0x72, 0x65, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73, 0x57, 0x69, 0x6e, 0x73, 0x12, 0x25, 0x0a,
    0x0e, 0x6c, 0x6f, 0x6f, 0x74, 0x5f, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x18,
    0x31, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x6c, 0x6f, 0x6f, 0x74, 0x47, 0x65, 0x6e, 0x65, 0x72,
    0x61, 0x74, 0x65, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x6c, 0x6f, 0x6f, 0x74, 0x5f, 0x61, 0x77, 0x61,
    0x72, 0x64, 0x65, 0x64, 0x18, 0x32, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x6c, 0x6f, 0x6f, 0x74,
    0x41, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x12, 0x1f, 0x0a, 0x07, 0x61, 0x6c, 0x6c, 0x63, 0x68,
    0x61, 0x74, 0x18, 0x33, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x52,
    0x07, 0x61, 0x6c, 0x6c, 0x63, 0x68, 0x61, 0x74, 0x12, 0x4a, 0x0a, 0x0d, 0x64, 0x6f, 0x74, 0x61,
    0x5f, 0x74, 0x76, 0x5f, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x18, 0x35, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x16, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61,
    0x54, 0x56, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x3a, 0x0e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f,
    0x74, 0x61, 0x54, 0x56, 0x5f, 0x31, 0x30, 0x52, 0x0b, 0x64, 0x6f, 0x74, 0x61, 0x54, 0x76, 0x44,
    0x65, 0x6c, 0x61, 0x79, 0x12, 0x28, 0x0a, 0x10, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67,
    0x61, 0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x36, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e,
    0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x26,
    0x0a, 0x0f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x6d, 0x61, 0x70, 0x5f, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x37, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x4d,
    0x61, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x2b, 0x0a, 0x11, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x5f, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x18, 0x38, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x10, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75,
    0x6c, 0x74, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6c, 0x61, 0x6e, 0x18, 0x39, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x03, 0x6c, 0x61, 0x6e, 0x12, 0x56, 0x0a, 0x16, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61,
    0x73, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18,
    0x3a, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4c, 0x6f,
    0x62, 0x62, 0x79, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x43, 0x68, 0x61, 0x6e,
    0x6e, 0x65, 0x6c, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x14, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61,
    0x73, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x34, 0x0a,
    0x16, 0x66, 0x69, 0x72, 0x73, 0x74, 0x5f, 0x6c, 0x65, 0x61, 0x76, 0x65, 0x72, 0x5f, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x69, 0x64, 0x18, 0x3b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x14, 0x66,
    0x69, 0x72, 0x73, 0x74, 0x4c, 0x65, 0x61, 0x76, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x69, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x69, 0x64,
    0x18, 0x3c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x49, 0x64,
    0x12, 0x21, 0x0a, 0x0c, 0x6c, 0x6f, 0x77, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x18, 0x3d, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x6c, 0x6f, 0x77, 0x50, 0x72, 0x69, 0x6f, 0x72,
    0x69, 0x74, 0x79, 0x12, 0x43, 0x0a, 0x0e, 0x65, 0x78, 0x74, 0x72, 0x61, 0x5f, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x73, 0x18, 0x3e, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x53, 0x4f, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x2e,
    0x43, 0x45, 0x78, 0x74, 0x72, 0x61, 0x4d, 0x73, 0x67, 0x52, 0x0d, 0x65, 0x78, 0x74, 0x72, 0x61,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x12, 0x30, 0x0a, 0x09, 0x73, 0x61, 0x76, 0x65,
    0x5f, 0x67, 0x61, 0x6d, 0x65, 0x18, 0x3f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x53, 0x61, 0x76, 0x65, 0x47, 0x61, 0x6d, 0x65,
    0x52, 0x08, 0x73, 0x61, 0x76, 0x65, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x30, 0x0a, 0x14, 0x66, 0x69,
    0x72, 0x73, 0x74, 0x5f, 0x62, 0x6c, 0x6f, 0x6f, 0x64, 0x5f, 0x68, 0x61, 0x70, 0x70, 0x65, 0x6e,
    0x65, 0x64, 0x18, 0x41, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x66, 0x69, 0x72, 0x73, 0x74, 0x42,
    0x6c, 0x6f, 0x6f, 0x64, 0x48, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x65, 0x64, 0x12, 0x51, 0x0a, 0x0d,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x6f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x18, 0x46, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x4d, 0x61, 0x74, 0x63,
    0x68, 0x4f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x3a, 0x17, 0x6b, 0x5f, 0x45, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x4f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x5f, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77,
    0x6e, 0x52, 0x0c, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x4f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x12,
    0x27, 0x0a, 0x0f, 0x6d, 0x61, 0x73, 0x73, 0x5f, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x18, 0x43, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x6d, 0x61, 0x73, 0x73, 0x44, 0x69,
    0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x12, 0x24, 0x0a, 0x0e, 0x63, 0x75, 0x73, 0x74,
    0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x44, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x3e,
    0x0a, 0x14, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x69, 0x6e, 0x67, 0x61, 0x6d, 0x65, 0x5f,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x45, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x0c, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x45, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x12, 0x61, 0x63, 0x74, 0x69,
    0x76, 0x65, 0x49, 0x6e, 0x67, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x2c,
    0x0a, 0x12, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x6d, 0x69, 0x6e, 0x5f, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x73, 0x18, 0x47, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x63, 0x75, 0x73, 0x74,
    0x6f, 0x6d, 0x4d, 0x69, 0x6e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x2c, 0x0a, 0x12,
    0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x70, 0x6c, 0x61, 0x79, 0x65,
    0x72, 0x73, 0x18, 0x48, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x10, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x4d, 0x61, 0x78, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x49, 0x0a, 0x0c, 0x70, 0x61,
    0x72, 0x74, 0x6e, 0x65, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x49, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x50, 0x61, 0x72, 0x74, 0x6e, 0x65, 0x72, 0x41,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x0c, 0x50, 0x41, 0x52, 0x54,
    0x4e, 0x45, 0x52, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x52, 0x0b, 0x70, 0x61, 0x72, 0x74, 0x6e, 0x65,
    0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x41, 0x0a, 0x1e, 0x6c, 0x61, 0x6e, 0x5f, 0x68, 0x6f, 0x73,
    0x74, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x5f, 0x74, 0x6f, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x4a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x19, 0x6c,
    0x61, 0x6e, 0x48, 0x6f, 0x73, 0x74, 0x50, 0x69, 0x6e, 0x67, 0x54, 0x6f, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x55, 0x0a, 0x0a, 0x76, 0x69, 0x73, 0x69,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x4b, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x56, 0x69, 0x73,
    0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x3a, 0x1a, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x6f, 0x62,
    0x62, 0x79, 0x56, 0x69, 0x73, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x50, 0x75, 0x62,
    0x6c, 0x69, 0x63, 0x52, 0x0a, 0x76, 0x69, 0x73, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12,
    0x26, 0x0a, 0x0f, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x63,
    0x72, 0x63, 0x18, 0x4c, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x47, 0x61, 0x6d, 0x65, 0x43, 0x72, 0x63, 0x12, 0x42, 0x0a, 0x1e, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x61, 0x75, 0x74, 0x6f, 0x5f, 0x63, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x64, 0x5f, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x18, 0x4d, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x1a, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x41, 0x75, 0x74, 0x6f, 0x43,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x12, 0x28, 0x0a, 0x10, 0x6c,
    0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x69, 0x64, 0x18,
    0x4e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x53, 0x65, 0x72,
    0x69, 0x65, 0x73, 0x49, 0x64, 0x12, 0x24, 0x0a, 0x0e, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f,
    0x67, 0x61, 0x6d, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x4f, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x6c,
    0x65, 0x61, 0x67, 0x75, 0x65, 0x47, 0x61, 0x6d, 0x65, 0x49, 0x64, 0x12, 0x32, 0x0a, 0x15, 0x63,
    0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x18, 0x50, 0x20, 0x01, 0x28, 0x07, 0x52, 0x13, 0x63, 0x75, 0x73, 0x74,
    0x6f, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12,
    0x36, 0x0a, 0x17, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x5f, 0x73, 0x65, 0x72, 0x69,
    0x65, 0x73, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x51, 0x20, 0x03, 0x28, 0x04,
    0x52, 0x15, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x53, 0x65, 0x72, 0x69, 0x65, 0x73,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x12, 0x36, 0x0a, 0x17, 0x70, 0x72, 0x65, 0x76, 0x69,
    0x6f, 0x75, 0x73, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x6f, 0x76, 0x65, 0x72, 0x72, 0x69,
    0x64, 0x65, 0x18, 0x52, 0x20, 0x01, 0x28, 0x04, 0x52, 0x15, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f,
    0x75, 0x73, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x12,
    0x46, 0x0a, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x75,
    0x73, 0x65, 0x73, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x72, 0x65, 0x63, 0x6f,
    0x72, 0x64, 0x73, 0x18, 0x53, 0x20, 0x01, 0x28, 0x08, 0x52, 0x1c, 0x63, 0x75, 0x73, 0x74, 0x6f,
    0x6d, 0x47, 0x61, 0x6d, 0x65, 0x55, 0x73, 0x65, 0x73, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x73, 0x12, 0x43, 0x0a, 0x1e, 0x6c, 0x65, 0x61, 0x67, 0x75,
    0x65, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x79, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x54, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x1b, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x6f, 0x0a, 0x20,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x63, 0x68, 0x6f, 0x69, 0x63, 0x65,
    0x18, 0x55, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x53, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x54,
    0x79, 0x70, 0x65, 0x3a, 0x09, 0x55, 0x4e, 0x44, 0x45, 0x46, 0x49, 0x4e, 0x45, 0x44, 0x52, 0x1d,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50,
    0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x43, 0x68, 0x6f, 0x69, 0x63, 0x65, 0x12, 0x76, 0x0a,
    0x24, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x6e, 0x6f, 0x6e, 0x5f, 0x73, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x63,
    0x68, 0x6f, 0x69, 0x63, 0x65, 0x18, 0x56, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x69, 0x6f,
    0x72, 0x69, 0x74, 0x79, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x09, 0x55, 0x4e, 0x44, 0x45, 0x46, 0x49,
    0x4e, 0x45, 0x44, 0x52, 0x20, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x4e, 0x6f, 0x6e, 0x53, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x69, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x43,
    0x68, 0x6f, 0x69, 0x63, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x57, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d,
    0x67, 0x61, 0x6d, 0x65, 0x53, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x61, 0x0a,
    0x0d, 0x70, 0x61, 0x75, 0x73, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x58,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x4c, 0x6f, 0x62, 0x62,
    0x79, 0x44, 0x6f, 0x74, 0x61, 0x50, 0x61, 0x75, 0x73, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e,
    0x67, 0x3a, 0x1f, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x50, 0x61, 0x75, 0x73,
    0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x55, 0x6e, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x65, 0x64, 0x52, 0x0c, 0x70, 0x61, 0x75, 0x73, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x12, 0x2f, 0x0a, 0x14, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x6d, 0x76, 0x70, 0x5f, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x59, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11,
    0x6c, 0x6f, 0x62, 0x62, 0x79, 0x4d, 0x76, 0x70, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49,
    0x64, 0x12, 0x3d, 0x0a, 0x1b, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75,
    0x72, 0x6e, 0x65, 0x79, 0x5f, 0x64, 0x69, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64,
    0x18, 0x5a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x18, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54,
    0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x44, 0x69, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x49, 0x64,
    0x12, 0x3d, 0x0a, 0x1b, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72,
    0x6e, 0x65, 0x79, 0x5f, 0x73, 0x6b, 0x69, 0x6c, 0x6c, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18,
    0x5b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x18, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f,
    0x75, 0x72, 0x6e, 0x65, 0x79, 0x53, 0x6b, 0x69, 0x6c, 0x6c, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12,
    0x41, 0x0a, 0x1d, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e,
    0x65, 0x79, 0x5f, 0x62, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64,
    0x18, 0x5c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1a, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54,
    0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x42, 0x72, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x6f, 0x75,
    0x6e, 0x64, 0x12, 0x5c, 0x0a, 0x13, 0x62, 0x6f, 0x74, 0x5f, 0x64, 0x69, 0x66, 0x66, 0x69, 0x63,
    0x75, 0x6c, 0x74, 0x79, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x18, 0x5d, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x17, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x42, 0x6f, 0x74, 0x44, 0x69,
    0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x3a, 0x13, 0x42, 0x4f, 0x54, 0x5f, 0x44, 0x49,
    0x46, 0x46, 0x49, 0x43, 0x55, 0x4c, 0x54, 0x59, 0x5f, 0x48, 0x41, 0x52, 0x44, 0x52, 0x11, 0x62,
    0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x69, 0x63, 0x75, 0x6c, 0x74, 0x79, 0x44, 0x69, 0x72, 0x65,
    0x12, 0x1f, 0x0a, 0x0b, 0x62, 0x6f, 0x74, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x18,
    0x5e, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x62, 0x6f, 0x74, 0x52, 0x61, 0x64, 0x69, 0x61, 0x6e,
    0x74, 0x12, 0x19, 0x0a, 0x08, 0x62, 0x6f, 0x74, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x18, 0x5f, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x07, 0x62, 0x6f, 0x74, 0x44, 0x69, 0x72, 0x65, 0x1a, 0x37, 0x0a, 0x09,
    0x43, 0x45, 0x78, 0x74, 0x72, 0x61, 0x4d, 0x73, 0x67, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x02, 0x69, 0x64, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x73, 0x22, 0x64, 0x0a, 0x05, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x06,
    0x0a, 0x02, 0x55, 0x49, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x52, 0x45, 0x41, 0x44, 0x59, 0x55,
    0x50, 0x10, 0x04, 0x12, 0x0f, 0x0a, 0x0b, 0x53, 0x45, 0x52, 0x56, 0x45, 0x52, 0x53, 0x45, 0x54,
    0x55, 0x50, 0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x52, 0x55, 0x4e, 0x10, 0x02, 0x12, 0x0c, 0x0a,
    0x08, 0x50, 0x4f, 0x53, 0x54, 0x47, 0x41, 0x4d, 0x45, 0x10, 0x03, 0x12, 0x0c, 0x0a, 0x08, 0x4e,
    0x4f, 0x54, 0x52, 0x45, 0x41, 0x44, 0x59, 0x10, 0x05, 0x12, 0x10, 0x0a, 0x0c, 0x53, 0x45, 0x52,
    0x56, 0x45, 0x52, 0x41, 0x53, 0x53, 0x49, 0x47, 0x4e, 0x10, 0x06, 0x22, 0xff, 0x01, 0x0a, 0x09,
    0x4c, 0x6f, 0x62, 0x62, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x14, 0x0a, 0x07, 0x49, 0x4e, 0x56,
    0x41, 0x4c, 0x49, 0x44, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x12,
    0x10, 0x0a, 0x0c, 0x43, 0x41, 0x53, 0x55, 0x41, 0x4c, 0x5f, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x10,
    0x00, 0x12, 0x0c, 0x0a, 0x08, 0x50, 0x52, 0x41, 0x43, 0x54, 0x49, 0x43, 0x45, 0x10, 0x01, 0x12,
    0x0e, 0x0a, 0x0a, 0x54, 0x4f, 0x55, 0x52, 0x4e, 0x41, 0x4d, 0x45, 0x4e, 0x54, 0x10, 0x02, 0x12,
    0x12, 0x0a, 0x0e, 0x43, 0x4f, 0x4f, 0x50, 0x5f, 0x42, 0x4f, 0x54, 0x5f, 0x4d, 0x41, 0x54, 0x43,
    0x48, 0x10, 0x04, 0x12, 0x15, 0x0a, 0x11, 0x4c, 0x45, 0x47, 0x41, 0x43, 0x59, 0x5f, 0x54, 0x45,
    0x41, 0x4d, 0x5f, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x10, 0x05, 0x12, 0x1b, 0x0a, 0x17, 0x4c, 0x45,
    0x47, 0x41, 0x43, 0x59, 0x5f, 0x53, 0x4f, 0x4c, 0x4f, 0x5f, 0x51, 0x55, 0x45, 0x55, 0x45, 0x5f,
    0x4d, 0x41, 0x54, 0x43, 0x48, 0x10, 0x06, 0x12, 0x15, 0x0a, 0x11, 0x43, 0x4f, 0x4d, 0x50, 0x45,
    0x54, 0x49, 0x54, 0x49, 0x56, 0x45, 0x5f, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x10, 0x07, 0x12, 0x14,
    0x0a, 0x10, 0x43, 0x41, 0x53, 0x55, 0x41, 0x4c, 0x5f, 0x31, 0x56, 0x31, 0x5f, 0x4d, 0x41, 0x54,
    0x43, 0x48, 0x10, 0x08, 0x12, 0x13, 0x0a, 0x0f, 0x57, 0x45, 0x45, 0x4b, 0x45, 0x4e, 0x44, 0x5f,
    0x54, 0x4f, 0x55, 0x52, 0x4e, 0x45, 0x59, 0x10, 0x09, 0x12, 0x13, 0x0a, 0x0f, 0x4c, 0x4f, 0x43,
    0x41, 0x4c, 0x5f, 0x42, 0x4f, 0x54, 0x5f, 0x4d, 0x41, 0x54, 0x43, 0x48, 0x10, 0x0a, 0x12, 0x0d,
    0x0a, 0x09, 0x53, 0x50, 0x45, 0x43, 0x54, 0x41, 0x54, 0x4f, 0x52, 0x10, 0x0b, 0x22, 0x2e, 0x0a,
    0x18, 0x43, 0x4d, 0x73, 0x67, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x50, 0x6c, 0x61, 0x79, 0x74, 0x65,
    0x73, 0x74, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x6a, 0x73, 0x6f,
    0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6a, 0x73, 0x6f, 0x6e, 0x2a, 0x50, 0x0a,
    0x10, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x54, 0x56, 0x44, 0x65, 0x6c, 0x61,
    0x79, 0x12, 0x12, 0x0a, 0x0e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x54, 0x56,
    0x5f, 0x31, 0x30, 0x10, 0x00, 0x12, 0x13, 0x0a, 0x0f, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f,
    0x74, 0x61, 0x54, 0x56, 0x5f, 0x31, 0x32, 0x30, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f, 0x4c, 0x6f,
    0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x54, 0x56, 0x5f, 0x33, 0x30, 0x30, 0x10, 0x02, 0x2a,
    0x83, 0x01, 0x0a, 0x15, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x50, 0x61, 0x75,
    0x73, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x12, 0x23, 0x0a, 0x1f, 0x4c, 0x6f, 0x62,
    0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x50, 0x61, 0x75, 0x73, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69,
    0x6e, 0x67, 0x5f, 0x55, 0x6e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x10, 0x00, 0x12, 0x21,
    0x0a, 0x1d, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x50, 0x61, 0x75, 0x73, 0x65,
    0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x10,
    0x01, 0x12, 0x22, 0x0a, 0x1e, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x44, 0x6f, 0x74, 0x61, 0x50, 0x61,
    0x75, 0x73, 0x65, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x44, 0x69, 0x73, 0x61, 0x62,
    0x6c, 0x65, 0x64, 0x10, 0x02, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xfa, 0x96, 0x01,
    0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xb7, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x05, 0x07, 0x1f, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x07, 0x20, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x08, 0x00, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03,
    0x08, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08,
    0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08,
    0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x08, 0x16, 0x1b,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x01, 0x12, 0x03, 0x09, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02,
    0x12, 0x03, 0x09, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x09, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x09, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x09,
    0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x05, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x0c, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0c, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x0c, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0d, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x0e, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x11, 0x00, 0x15,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x11, 0x05, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x12, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x12, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03,
    0x13, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x08,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x13, 0x28, 0x29, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x14, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x14, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x17, 0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x17, 0x08, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x08, 0x4e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x18, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x18, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x18, 0x24, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x18, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03,
    0x18, 0x35, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x07, 0x12, 0x03, 0x18, 0x40,
    0x4c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x19, 0x08, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x19, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x19, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x1a, 0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x18, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x1a, 0x2e, 0x3d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1a, 0x2f, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00,
    0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1a, 0x2f, 0x35, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x2f, 0x35, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1a, 0x2f, 0x35, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x1a, 0x38, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1b,
    0x08, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1b, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x08, 0x12, 0x03, 0x1b, 0x2e, 0x3d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x03,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1b, 0x2f, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x2f, 0x35, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x2f, 0x35, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b,
    0x2f, 0x35, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x1b, 0x38, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x1c, 0x08,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x1c, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x1c, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1c, 0x18, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1c, 0x35, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x1d, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x1d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1d,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1d, 0x18, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x1d, 0x2e, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x1e, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x05, 0x12, 0x03, 0x1e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x1e, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x1e, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x1f, 0x08, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x1f, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x1f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x1f, 0x18, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x1f, 0x30, 0x31, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x22, 0x00, 0x53, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x22, 0x08, 0x14,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12, 0x04, 0x23, 0x08, 0x27, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03, 0x23, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x10, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x10, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x24, 0x15, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x25, 0x10, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x10, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x25, 0x20, 0x21, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x26, 0x10, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x26, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x29, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x29, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x18, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x23, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08, 0x12, 0x03, 0x29, 0x25, 0x39, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x29, 0x26, 0x38, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x29, 0x26, 0x31, 0x0a,
    0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29,
    0x26, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x29, 0x27, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x29, 0x34, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x2a, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x11,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x19, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x25, 0x26, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x2b, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x2b, 0x19, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b,
    0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x08, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x2c, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12,
    0x03, 0x2d, 0x08, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2d,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x2d, 0x11, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2d, 0x24, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2d, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x08, 0x12, 0x03, 0x2d, 0x2e, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x07, 0x12, 0x03, 0x2d, 0x39, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05,
    0x12, 0x03, 0x2e, 0x08, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x2e, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2e, 0x18, 0x3a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2e, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06,
    0x05, 0x12, 0x03, 0x2f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x2f, 0x18, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2f,
    0x37, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x30, 0x08, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x30, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x30, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x30, 0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x30, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x08, 0x12,
    0x03, 0x31, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x04, 0x12, 0x03, 0x31,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x05, 0x12, 0x03, 0x31, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x01, 0x12, 0x03, 0x31, 0x18, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x08, 0x03, 0x12, 0x03, 0x31, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x09, 0x12, 0x03, 0x32, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x09, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x05,
    0x12, 0x03, 0x32, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x32, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x09, 0x03, 0x12, 0x03, 0x32, 0x26,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0a, 0x12, 0x03, 0x33, 0x08, 0x35, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x33, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x33, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0a, 0x01, 0x12, 0x03, 0x33, 0x18, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0a,
    0x03, 0x12, 0x03, 0x33, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x03,
    0x34, 0x08, 0x49, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x34, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x34, 0x11, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x34, 0x1b, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x34, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0b, 0x08, 0x12, 0x03, 0x34, 0x2b, 0x48, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0b, 0x07, 0x12, 0x03, 0x34, 0x36, 0x47, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0c, 0x12,
    0x03, 0x35, 0x08, 0x5a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x35,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x06, 0x12, 0x03, 0x35, 0x11, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x35, 0x23, 0x31, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x35, 0x34, 0x36, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x0c, 0x08, 0x12, 0x03, 0x35, 0x37, 0x59, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0c, 0x07, 0x12, 0x03, 0x35, 0x42, 0x58, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0d,
    0x12, 0x03, 0x36, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x04, 0x12, 0x03,
    0x36, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x36, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x36, 0x18, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x36, 0x22, 0x24, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x0e, 0x12, 0x03, 0x37, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x0e, 0x04, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e,
    0x05, 0x12, 0x03, 0x37, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x01, 0x12,
    0x03, 0x37, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x37,
    0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x0f, 0x12, 0x03, 0x38, 0x08, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x38, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x38, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x38, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x0f, 0x03, 0x12, 0x03, 0x38, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x10, 0x12,
    0x03, 0x39, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x04, 0x12, 0x03, 0x39,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x05, 0x12, 0x03, 0x39, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x01, 0x12, 0x03, 0x39, 0x18, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x10, 0x03, 0x12, 0x03, 0x39, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x11, 0x12, 0x03, 0x3a, 0x08, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x11, 0x04, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x05,
    0x12, 0x03, 0x3a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x01, 0x12, 0x03,
    0x3a, 0x18, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x11, 0x03, 0x12, 0x03, 0x3a, 0x34,
    0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x12, 0x12, 0x03, 0x3b, 0x08, 0x37, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x12, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x12, 0x05, 0x12, 0x03, 0x3b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x12, 0x01, 0x12, 0x03, 0x3b, 0x18, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x12,
    0x03, 0x12, 0x03, 0x3b, 0x34, 0x36, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x13, 0x12, 0x03,
    0x3c, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x13, 0x04, 0x12, 0x03, 0x3c, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x13, 0x05, 0x12, 0x03, 0x3c, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x13, 0x01, 0x12, 0x03, 0x3c, 0x18, 0x35, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x13, 0x03, 0x12, 0x03, 0x3c, 0x38, 0x3a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x14, 0x12, 0x03, 0x3d, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14,
    0x04, 0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x05, 0x12,
    0x03, 0x3d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x01, 0x12, 0x03, 0x3d,
    0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x14, 0x03, 0x12, 0x03, 0x3d, 0x29, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x15, 0x12, 0x03, 0x3e, 0x08, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x15, 0x04, 0x12, 0x03, 0x3e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x15, 0x05, 0x12, 0x03, 0x3e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x15, 0x01, 0x12, 0x03, 0x3e, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x15, 0x03,
    0x12, 0x03, 0x3e, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x16, 0x12, 0x03, 0x3f,
    0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x16, 0x04, 0x12, 0x03, 0x3f, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x16, 0x06, 0x12, 0x03, 0x3f, 0x11, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x16, 0x01, 0x12, 0x03, 0x3f, 0x24, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x16, 0x03, 0x12, 0x03, 0x3f, 0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x17, 0x12, 0x03, 0x40, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x04,
    0x12, 0x03, 0x40, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x05, 0x12, 0x03,
    0x40, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x01, 0x12, 0x03, 0x40, 0x18,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x17, 0x03, 0x12, 0x03, 0x40, 0x28, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x18, 0x12, 0x03, 0x41, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x18, 0x04, 0x12, 0x03, 0x41, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x18, 0x05, 0x12, 0x03, 0x41, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18,
    0x01, 0x12, 0x03, 0x41, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x18, 0x03, 0x12,
    0x03, 0x41, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x19, 0x12, 0x03, 0x42, 0x08,
    0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x19, 0x04, 0x12, 0x03, 0x42, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x19, 0x05, 0x12, 0x03, 0x42, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x19, 0x01, 0x12, 0x03, 0x42, 0x18, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x19, 0x03, 0x12, 0x03, 0x42, 0x37, 0x39, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x1a, 0x12, 0x03, 0x43, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1a, 0x04, 0x12,
    0x03, 0x43, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1a, 0x06, 0x12, 0x03, 0x43,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x43, 0x18, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1a, 0x03, 0x12, 0x03, 0x43, 0x2f, 0x31, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x1b, 0x12, 0x03, 0x44, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x1b, 0x04, 0x12, 0x03, 0x44, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x1b, 0x05, 0x12, 0x03, 0x44, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x01,
    0x12, 0x03, 0x44, 0x16, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1b, 0x03, 0x12, 0x03,
    0x44, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1c, 0x12, 0x03, 0x45, 0x08, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1c, 0x04, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x1c, 0x06, 0x12, 0x03, 0x45, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x1c, 0x01, 0x12, 0x03, 0x45, 0x24, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x1c, 0x03, 0x12, 0x03, 0x45, 0x33, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1d,
    0x12, 0x03, 0x46, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1d, 0x04, 0x12, 0x03,
    0x46, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1d, 0x06, 0x12, 0x03, 0x46, 0x11,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1d, 0x01, 0x12, 0x03, 0x46, 0x24, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1d, 0x03, 0x12, 0x03, 0x46, 0x33, 0x35, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x1e, 0x12, 0x03, 0x47, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x1e, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e,
    0x05, 0x12, 0x03, 0x47, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x01, 0x12,
    0x03, 0x47, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1e, 0x03, 0x12, 0x03, 0x47,
    0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x1f, 0x12, 0x03, 0x48, 0x08, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x1f, 0x04, 0x12, 0x03, 0x48, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x1f, 0x05, 0x12, 0x03, 0x48, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x1f, 0x01, 0x12, 0x03, 0x48, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x1f, 0x03, 0x12, 0x03, 0x48, 0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x20, 0x12,
    0x03, 0x49, 0x08, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x20, 0x04, 0x12, 0x03, 0x49,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x20, 0x05, 0x12, 0x03, 0x49, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x20, 0x01, 0x12, 0x03, 0x49, 0x18, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x20, 0x03, 0x12, 0x03, 0x49, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x21, 0x12, 0x03, 0x4a, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x21, 0x04, 0x12, 0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x21, 0x05,
    0x12, 0x03, 0x4a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x21, 0x01, 0x12, 0x03,
    0x4a, 0x18, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x21, 0x03, 0x12, 0x03, 0x4a, 0x2e,
    0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x22, 0x12, 0x03, 0x4b, 0x08, 0x33, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x22, 0x04, 0x12, 0x03, 0x4b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x22, 0x05, 0x12, 0x03, 0x4b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x22, 0x01, 0x12, 0x03, 0x4b, 0x18, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x22,
    0x03, 0x12, 0x03, 0x4b, 0x30, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x23, 0x12, 0x03,
    0x4c, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x23, 0x04, 0x12, 0x03, 0x4c, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x23, 0x05, 0x12, 0x03, 0x4c, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x23, 0x01, 0x12, 0x03, 0x4c, 0x18, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x23, 0x03, 0x12, 0x03, 0x4c, 0x2e, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x24, 0x12, 0x03, 0x4d, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x24,
    0x04, 0x12, 0x03, 0x4d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x24, 0x05, 0x12,
    0x03, 0x4d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x24, 0x01, 0x12, 0x03, 0x4d,
    0x18, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x24, 0x03, 0x12, 0x03, 0x4d, 0x30, 0x32,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x25, 0x12, 0x03, 0x4e, 0x08, 0x39, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x25, 0x04, 0x12, 0x03, 0x4e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x25, 0x05, 0x12, 0x03, 0x4e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x25, 0x01, 0x12, 0x03, 0x4e, 0x18, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x25, 0x03,
    0x12, 0x03, 0x4e, 0x36, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x26, 0x12, 0x03, 0x4f,
    0x08, 0x7e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x26, 0x04, 0x12, 0x03, 0x4f, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x26, 0x06, 0x12, 0x03, 0x4f, 0x11, 0x2b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x26, 0x01, 0x12, 0x03, 0x4f, 0x2c, 0x48, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x26, 0x03, 0x12, 0x03, 0x4f, 0x4b, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x26, 0x08, 0x12, 0x03, 0x4f, 0x4e, 0x7d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x26,
    0x07, 0x12, 0x03, 0x4f, 0x59, 0x7c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x27, 0x12, 0x03,
    0x50, 0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x27, 0x04, 0x12, 0x03, 0x50, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x27, 0x05, 0x12, 0x03, 0x50, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x27, 0x01, 0x12, 0x03, 0x50, 0x18, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x27, 0x03, 0x12, 0x03, 0x50, 0x36, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x28, 0x12, 0x03, 0x51, 0x08, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x28,
    0x04, 0x12, 0x03, 0x51, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x28, 0x05, 0x12,
    0x03, 0x51, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x28, 0x01, 0x12, 0x03, 0x51,
    0x18, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x28, 0x03, 0x12, 0x03, 0x51, 0x35, 0x37,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x29, 0x12, 0x03, 0x52, 0x08, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x29, 0x04, 0x12, 0x03, 0x52, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x29, 0x05, 0x12, 0x03, 0x52, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x29, 0x01, 0x12, 0x03, 0x52, 0x18, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x29, 0x03,
    0x12, 0x03, 0x52, 0x33, 0x35, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x55, 0x00, 0x64,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x55, 0x08, 0x1a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x02, 0x03, 0x00, 0x12, 0x04, 0x56, 0x08, 0x5a, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x03, 0x00, 0x01, 0x12, 0x03, 0x56, 0x10, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x57, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x57, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x57, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x57, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x58, 0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x58, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x58, 0x19, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x58, 0x21, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x58, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x59, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x59, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x59, 0x19, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x59, 0x1e, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x59, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x5c, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5c, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5c, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x18, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x08, 0x12, 0x03, 0x5c, 0x25, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x5c, 0x26, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x5c, 0x26, 0x31, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x26, 0x31, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x5c, 0x27, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x5c, 0x34, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5d,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5d, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5d, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x19, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x5e, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x5e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x18,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x26, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x08, 0x3c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x5f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x03, 0x06, 0x12, 0x03, 0x5f, 0x11, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x5f, 0x30, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x5f, 0x3a, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x60, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x60, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x60, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x60, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x60, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x61, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x61, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x61,
    0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x61, 0x16, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x61, 0x2c, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x62, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x62, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x06, 0x05, 0x12, 0x03, 0x62, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x62, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03,
    0x62, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x63, 0x08, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03, 0x63, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x63, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x63, 0x19, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x63, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x66, 0x00, 0x74, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x66, 0x08, 0x1a,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x03, 0x00, 0x12, 0x04, 0x67, 0x08, 0x6a, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x03, 0x00, 0x01, 0x12, 0x03, 0x67, 0x10, 0x1b, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x03, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x68, 0x10, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x68, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x68, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x68, 0x20, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x03, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x68, 0x27, 0x28, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x03, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x69, 0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x69, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x69, 0x19, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x69, 0x21, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x03,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x69, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x6c, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x6c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x6c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6c, 0x18,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6c, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x03, 0x6c, 0x25, 0x39, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x6c, 0x26, 0x38, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x6c, 0x26, 0x31,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x6c, 0x26, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x6c, 0x27, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x6c, 0x34, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x01, 0x12, 0x03, 0x6d, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x6d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6d,
    0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6d, 0x19, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6d, 0x25, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x6e, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x6e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x6e, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x6e, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x6f, 0x08, 0x3c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6f, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x06, 0x12, 0x03, 0x6f, 0x11, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x6f, 0x30, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x6f, 0x3a, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04,
    0x12, 0x03, 0x70, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x70, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x70, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x70, 0x18, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x70, 0x29, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x05, 0x12, 0x03, 0x71, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x71, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x71, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x71, 0x19, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x05, 0x03, 0x12, 0x03, 0x71,
    0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x06, 0x12, 0x03, 0x72, 0x08, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x06, 0x04, 0x12, 0x03, 0x72, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x06, 0x05, 0x12, 0x03, 0x72, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x06, 0x01, 0x12, 0x03, 0x72, 0x19, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x72, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x07, 0x12,
    0x03, 0x73, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x04, 0x12, 0x03, 0x73,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x05, 0x12, 0x03, 0x73, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x01, 0x12, 0x03, 0x73, 0x19, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x07, 0x03, 0x12, 0x03, 0x73, 0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x76, 0x00, 0x7d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x76, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x77, 0x08,
    0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x77, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x77, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x77, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x78, 0x08, 0x55, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x78, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x78,
    0x11, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x78, 0x20, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x78, 0x2d, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x08, 0x12, 0x03, 0x78, 0x2f, 0x54, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x07, 0x12, 0x03, 0x78, 0x3a, 0x53, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x02, 0x12, 0x03, 0x79, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x79, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x79, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x79, 0x16,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x79, 0x28, 0x29, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x7a, 0x08, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x7a, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x7a, 0x16, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x7a, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x7b, 0x08,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x7b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x7b, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7b, 0x16, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x7b, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x05, 0x12, 0x03, 0x7c, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12,
    0x03, 0x7c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x7c,
    0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x7c, 0x16, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x7c, 0x28, 0x29, 0x0a, 0x0b,
    0x0a, 0x02, 0x04, 0x05, 0x12, 0x05, 0x7f, 0x00, 0xa0, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x7f, 0x08, 0x18, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x00, 0x12,
    0x06, 0x80, 0x01, 0x08, 0x84, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x03, 0x00, 0x01,
    0x12, 0x04, 0x80, 0x01, 0x10, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x04, 0x81, 0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x81, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x81, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x20, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x05, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x01, 0x27, 0x28, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x05,
    0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x05,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x82, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x05, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x82, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x05, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x01, 0x1f, 0x27, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x82, 0x01, 0x2a, 0x2b, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0x83, 0x01, 0x10, 0x2f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x83, 0x01, 0x10, 0x18, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0x83, 0x01, 0x19, 0x1f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x83, 0x01, 0x20,
    0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x05, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0x83, 0x01,
    0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x08, 0x35,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01, 0x11, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x19, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x08, 0x12, 0x04, 0x86, 0x01, 0x20, 0x34, 0x0a, 0x10, 0x0a, 0x08, 0x04,
    0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x86, 0x01, 0x21, 0x33, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0x86, 0x01, 0x21, 0x2c,
    0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04,
    0x86, 0x01, 0x21, 0x2c, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x22, 0x2b, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x05, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x2f, 0x33, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x87, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x87, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x87, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x87, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x04,
    0x88, 0x01, 0x08, 0x4a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x04, 0x88,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x04, 0x88, 0x01,
    0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x1e,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x04, 0x88, 0x01, 0x25, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x08, 0x12, 0x04, 0x88, 0x01, 0x27, 0x49, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x07, 0x12, 0x04, 0x88, 0x01, 0x32, 0x48, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x04, 0x89, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x04, 0x89, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x05, 0x12, 0x04, 0x89, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x89, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x89, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04,
    0x12, 0x04, 0x8a, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x8a, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x8a, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8a,
    0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8a, 0x01,
    0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x05, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x05, 0x12, 0x04, 0x8b, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x06, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06,
    0x01, 0x12, 0x04, 0x8c, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x06, 0x03,
    0x12, 0x04, 0x8c, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x07, 0x12, 0x04,
    0x8d, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x04, 0x12, 0x04, 0x8d,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x05, 0x12, 0x04, 0x8d, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x18,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x07, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x22, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x08, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x2d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x08, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x08, 0x05, 0x12, 0x04, 0x8e, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x08, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x08, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x09, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09,
    0x04, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x06,
    0x12, 0x04, 0x8f, 0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x01, 0x12,
    0x04, 0x8f, 0x01, 0x24, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x03, 0x12, 0x04,
    0x8f, 0x01, 0x34, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x08, 0x12, 0x04, 0x8f,
    0x01, 0x37, 0x53, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x09, 0x07, 0x12, 0x04, 0x8f, 0x01,
    0x42, 0x52, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0a, 0x12, 0x04, 0x90, 0x01, 0x08, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x90, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x05, 0x12, 0x04, 0x90, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x90, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x90, 0x01, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x0b, 0x12, 0x04, 0x91, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x0b, 0x04, 0x12, 0x04, 0x91, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x0b, 0x05, 0x12, 0x04, 0x91, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b,
    0x01, 0x12, 0x04, 0x91, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0b, 0x03,
    0x12, 0x04, 0x91, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0c, 0x12, 0x04,
    0x92, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x92,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x05, 0x12, 0x04, 0x92, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x92, 0x01, 0x18,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x92, 0x01, 0x2a, 0x2c,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0d, 0x12, 0x04, 0x93, 0x01, 0x08, 0x2e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x93, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0d, 0x05, 0x12, 0x04, 0x93, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x93, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x0d, 0x03, 0x12, 0x04, 0x93, 0x01, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x0e, 0x12, 0x04, 0x94, 0x01, 0x08, 0x57, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e,
    0x04, 0x12, 0x04, 0x94, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x06,
    0x12, 0x04, 0x94, 0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x01, 0x12,
    0x04, 0x94, 0x01, 0x24, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x03, 0x12, 0x04,
    0x94, 0x01, 0x3b, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x08, 0x12, 0x04, 0x94,
    0x01, 0x3e, 0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0e, 0x07, 0x12, 0x04, 0x94, 0x01,
    0x49, 0x55, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x0f, 0x12, 0x04, 0x95, 0x01, 0x08, 0x2d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x95, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x0f, 0x05, 0x12, 0x04, 0x95, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x95, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x0f, 0x03, 0x12, 0x04, 0x95, 0x01, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x10, 0x12, 0x04, 0x96, 0x01, 0x08, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x10, 0x04, 0x12, 0x04, 0x96, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x10, 0x06, 0x12, 0x04, 0x96, 0x01, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x10,
    0x01, 0x12, 0x04, 0x96, 0x01, 0x1e, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x10, 0x03,
    0x12, 0x04, 0x96, 0x01, 0x2b, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x10, 0x08, 0x12,
    0x04, 0x96, 0x01, 0x2e, 0x4d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x10, 0x07, 0x12, 0x04,
    0x96, 0x01, 0x39, 0x4c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x11, 0x12, 0x04, 0x97, 0x01,
    0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x11, 0x04, 0x12, 0x04, 0x97, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x11, 0x05, 0x12, 0x04, 0x97, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x11, 0x01, 0x12, 0x04, 0x97, 0x01, 0x18, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x11, 0x03, 0x12, 0x04, 0x97, 0x01, 0x2b, 0x2d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x12, 0x12, 0x04, 0x98, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x12, 0x04, 0x12, 0x04, 0x98, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x12, 0x05, 0x12, 0x04, 0x98, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x12, 0x01, 0x12, 0x04, 0x98, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x12, 0x03, 0x12, 0x04, 0x98, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x13,
    0x12, 0x04, 0x99, 0x01, 0x08, 0x4a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x13, 0x04, 0x12,
    0x04, 0x99, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x13, 0x06, 0x12, 0x04,
    0x99, 0x01, 0x11, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x13, 0x01, 0x12, 0x04, 0x99,
    0x01, 0x3a, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x13, 0x03, 0x12, 0x04, 0x99, 0x01,
    0x47, 0x49, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x14, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x29,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x14, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x14, 0x05, 0x12, 0x04, 0x9a, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x14, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x14, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x15, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x15, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x15, 0x05, 0x12, 0x04, 0x9b, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x15,
    0x01, 0x12, 0x04, 0x9b, 0x01, 0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x15, 0x03,
    0x12, 0x04, 0x9b, 0x01, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x16, 0x12, 0x04,
    0x9c, 0x01, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x16, 0x04, 0x12, 0x04, 0x9c,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x16, 0x05, 0x12, 0x04, 0x9c, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x16, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x18,
    0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x16, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x32, 0x34,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x17, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x37, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x17, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x17, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x17, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x18, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x17, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x34, 0x36, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x18, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x18,
    0x04, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x18, 0x06,
    0x12, 0x04, 0x9e, 0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x18, 0x01, 0x12,
    0x04, 0x9e, 0x01, 0x1b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x18, 0x03, 0x12, 0x04,
    0x9e, 0x01, 0x2f, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x18, 0x08, 0x12, 0x04, 0x9e,
    0x01, 0x32, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x18, 0x07, 0x12, 0x04, 0x9e, 0x01,
    0x3d, 0x4e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x19, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x37,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x19, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x19, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x19, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x18, 0x31, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x19, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x34, 0x36, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x06, 0xa2, 0x01, 0x00, 0xb3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12,
    0x04, 0xa3, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xa3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa3,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x01,
    0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x02, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xa5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xa5, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xa5, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x04, 0xa6,
    0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa6, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x18, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x24, 0x25, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x04, 0xa7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x05, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04,
    0x12, 0x04, 0xa8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05, 0x12,
    0x04, 0xa8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xa8, 0x01, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x04, 0xa8,
    0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x04, 0xa9, 0x01, 0x08,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x11, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x16, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x07, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x07, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x07, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x07, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07,
    0x03, 0x12, 0x04, 0xaa, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x08, 0x12,
    0x04, 0xab, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x04, 0x12, 0x04,
    0xab, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x05, 0x12, 0x04, 0xab,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01, 0x12, 0x04, 0xab, 0x01,
    0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12, 0x04, 0xab, 0x01, 0x24,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x09, 0x12, 0x04, 0xac, 0x01, 0x08, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x04, 0x12, 0x04, 0xac, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x05, 0x12, 0x04, 0xac, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x09, 0x01, 0x12, 0x04, 0xac, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x09, 0x03, 0x12, 0x04, 0xac, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x0a, 0x12, 0x04, 0xad, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x0a, 0x04, 0x12, 0x04, 0xad, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a,
    0x05, 0x12, 0x04, 0xad, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x01,
    0x12, 0x04, 0xad, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x03, 0x12,
    0x04, 0xad, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0b, 0x12, 0x04, 0xae,
    0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xae, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x05, 0x12, 0x04, 0xae, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xae, 0x01, 0x18, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xae, 0x01, 0x2a, 0x2c, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0c, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x2f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x0c, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x0c, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x2c, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x0d, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0d, 0x04,
    0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0d, 0x05, 0x12,
    0x04, 0xb0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0d, 0x01, 0x12, 0x04,
    0xb0, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xb0,
    0x01, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x0e, 0x12, 0x04, 0xb1, 0x01, 0x08,
    0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0e, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x18, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x26, 0x28, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x0f, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x0f, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x0f, 0x05, 0x12, 0x04, 0xb2, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x0f, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0f,
    0x03, 0x12, 0x04, 0xb2, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0xb5,
    0x01, 0x00, 0xbb, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x04, 0xb5, 0x01,
    0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb6, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xb7, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xb7, 0x01, 0x16, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xb7, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x04,
    0xb8, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb8,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb8, 0x01,
    0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x01, 0x16,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb8, 0x01, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x08, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x04, 0xb9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x04, 0xb9, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x04, 0x12, 0x04, 0xba, 0x01, 0x08, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xba, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x05,
    0x12, 0x04, 0xba, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xba, 0x01, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xba, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0xbd, 0x01, 0x00, 0xc2,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbe, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x01, 0x12, 0x04, 0xbf, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xbf, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xbf, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xbf, 0x01, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbf,
    0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x04, 0xc0, 0x01, 0x08,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc0, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x04, 0xc0, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x18, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xc1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xc1, 0x01, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xc1, 0x01, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x06, 0xc4,
    0x01, 0x00, 0xb3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x04, 0xc4, 0x01,
    0x08, 0x14, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x09, 0x03, 0x00, 0x12, 0x06, 0xc5, 0x01, 0x08, 0xc8,
    0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x03, 0x00, 0x01, 0x12, 0x04, 0xc5, 0x01, 0x10,
    0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc6, 0x01, 0x10,
    0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc6, 0x01,
    0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc6,
    0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xc6, 0x01, 0x20, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xc6, 0x01, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x04, 0xc7, 0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xc7, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x03, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xc7, 0x01, 0x19, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x1f, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xc7, 0x01, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x09, 0x04,
    0x00, 0x12, 0x06, 0xca, 0x01, 0x08, 0xd2, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x04,
    0x00, 0x01, 0x12, 0x04, 0xca, 0x01, 0x0d, 0x12, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xcb, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x10, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xcb, 0x01, 0x15, 0x16, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcc, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x09, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xcc, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x01, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x10, 0x1b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x01, 0x1e, 0x1f, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xce, 0x01, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xce, 0x01, 0x10, 0x13, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xce, 0x01, 0x16, 0x17,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xcf, 0x01, 0x10, 0x1d,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xcf, 0x01,
    0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xd0, 0x01,
    0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd0,
    0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04,
    0xd0, 0x01, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04,
    0xd1, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xd1, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x00, 0x02, 0x06, 0x02,
    0x12, 0x04, 0xd1, 0x01, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x09, 0x04, 0x01, 0x12, 0x06,
    0xd4, 0x01, 0x08, 0xe1, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x04, 0x01, 0x01, 0x12,
    0x04, 0xd4, 0x01, 0x0d, 0x16, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x04, 0xd5, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xd5, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xd5, 0x01, 0x1a, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02,
    0x01, 0x12, 0x04, 0xd6, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xd6, 0x01, 0x1f, 0x20, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x04, 0xd7, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09,
    0x04, 0x01, 0x02, 0x02, 0x02, 0x12, 0x04, 0xd7, 0x01, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x09, 0x04, 0x01, 0x02, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x10, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x09, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x09, 0x04, 0x01, 0x02, 0x03, 0x02, 0x12, 0x04, 0xd8, 0x01, 0x1d, 0x1e, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x09, 0x04, 0x01, 0x02, 0x04, 0x12, 0x04, 0xd9, 0x01, 0x10, 0x23, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x10, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x04, 0x02, 0x12, 0x04, 0xd9, 0x01, 0x21, 0x22, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02, 0x05, 0x12, 0x04, 0xda, 0x01, 0x10, 0x26, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x04, 0xda, 0x01, 0x10, 0x21,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x05, 0x02, 0x12, 0x04, 0xda, 0x01, 0x24,
    0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02, 0x06, 0x12, 0x04, 0xdb, 0x01, 0x10,
    0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x04, 0xdb, 0x01,
    0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x06, 0x02, 0x12, 0x04, 0xdb,
    0x01, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02, 0x07, 0x12, 0x04, 0xdc,
    0x01, 0x10, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x04,
    0xdc, 0x01, 0x10, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x07, 0x02, 0x12,
    0x04, 0xdc, 0x01, 0x24, 0x25, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02, 0x08, 0x12,
    0x04, 0xdd, 0x01, 0x10, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x08, 0x01,
    0x12, 0x04, 0xdd, 0x01, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02, 0x08,
    0x02, 0x12, 0x04, 0xdd, 0x01, 0x23, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04, 0x01, 0x02,
    0x09, 0x12, 0x04, 0xde, 0x01, 0x10, 0x24, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01, 0x02,
    0x09, 0x01, 0x12, 0x04, 0xde, 0x01, 0x10, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04, 0x01,
    0x02, 0x09, 0x02, 0x12, 0x04, 0xde, 0x01, 0x22, 0x23, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x09, 0x04,
    0x01, 0x02, 0x0a, 0x12, 0x04, 0xdf, 0x01, 0x10, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09, 0x04,
    0x01, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x10, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x09,
    0x04, 0x01, 0x02, 0x0a, 0x02, 0x12, 0x04, 0xdf, 0x01, 0x22, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x09, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x04, 0xe0, 0x01, 0x10, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x09, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x10, 0x19, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x09, 0x04, 0x01, 0x02, 0x0b, 0x02, 0x12, 0x04, 0xe0, 0x01, 0x1c, 0x1e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04, 0xe3, 0x01, 0x08, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xe3, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xe3, 0x01, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x08,
    0x12, 0x04, 0xe3, 0x01, 0x25, 0x39, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x09, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x04, 0xe3, 0x01, 0x26, 0x38, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xe3, 0x01, 0x26, 0x31, 0x0a, 0x12, 0x0a, 0x0a, 0x04,
    0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xe3, 0x01, 0x26, 0x31, 0x0a,
    0x13, 0x0a, 0x0b, 0x04, 0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xe3, 0x01, 0x27, 0x30, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x04, 0xe3, 0x01, 0x34, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12,
    0x04, 0xe4, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xe4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe4,
    0x01, 0x11, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe4, 0x01,
    0x22, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe4, 0x01, 0x2c,
    0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x33, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe5, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x06, 0x12, 0x04, 0xe5, 0x01, 0x11, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x22, 0x2e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe5, 0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x03, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03,
    0x05, 0x12, 0x04, 0xe6, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xe6, 0x01, 0x19, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12,
    0x04, 0xe6, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04, 0x12, 0x04, 0xe7,
    0x01, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x04, 0xe7, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x05, 0x12, 0x04, 0xe7, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x19, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x25, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x08, 0x12, 0x04, 0xe7, 0x01, 0x27, 0x34, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x07, 0x12, 0x04, 0xe7, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x05, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x05, 0x04, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x05, 0x05, 0x12, 0x04, 0xe8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x05, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05,
    0x03, 0x12, 0x04, 0xe8, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x06, 0x12,
    0x04, 0xe9, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x04, 0x12, 0x04,
    0xe9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x05, 0x12, 0x04, 0xe9,
    0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x01, 0x12, 0x04, 0xe9, 0x01,
    0x19, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x03, 0x12, 0x04, 0xe9, 0x01, 0x2b,
    0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x07, 0x12, 0x04, 0xea, 0x01, 0x08, 0x3d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x04, 0x12, 0x04, 0xea, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x06, 0x12, 0x04, 0xea, 0x01, 0x11, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x07, 0x01, 0x12, 0x04, 0xea, 0x01, 0x24, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x07, 0x03, 0x12, 0x04, 0xea, 0x01, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x07, 0x08, 0x12, 0x04, 0xea, 0x01, 0x2e, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x07, 0x07, 0x12, 0x04, 0xea, 0x01, 0x39, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x08, 0x12, 0x04, 0xeb, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x08, 0x04,
    0x12, 0x04, 0xeb, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x08, 0x05, 0x12,
    0x04, 0xeb, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x08, 0x01, 0x12, 0x04,
    0xeb, 0x01, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x08, 0x03, 0x12, 0x04, 0xeb,
    0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x09, 0x12, 0x04, 0xec, 0x01, 0x08,
    0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x04, 0x12, 0x04, 0xec, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x06, 0x12, 0x04, 0xec, 0x01, 0x11, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x01, 0x12, 0x04, 0xec, 0x01, 0x28, 0x32, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x09, 0x03, 0x12, 0x04, 0xec, 0x01, 0x35, 0x37, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x09, 0x08, 0x12, 0x04, 0xec, 0x01, 0x38, 0x4b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x09, 0x07, 0x12, 0x04, 0xec, 0x01, 0x43, 0x4a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x0a, 0x12, 0x04, 0xed, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x0a, 0x04, 0x12, 0x04, 0xed, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0a,
    0x05, 0x12, 0x04, 0xed, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0a, 0x01,
    0x12, 0x04, 0xed, 0x01, 0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0a, 0x03, 0x12,
    0x04, 0xed, 0x01, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x0b, 0x12, 0x04, 0xee,
    0x01, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xee, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0b, 0x05, 0x12, 0x04, 0xee, 0x01, 0x11,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xee, 0x01, 0x16, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xee, 0x01, 0x27, 0x29, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x0c, 0x12, 0x04, 0xef, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xef, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x0c, 0x05, 0x12, 0x04, 0xef, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xef, 0x01, 0x16, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x0c, 0x03, 0x12, 0x04, 0xef, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x0d, 0x12, 0x04, 0xf0, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0d, 0x04,
    0x12, 0x04, 0xf0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0d, 0x05, 0x12,
    0x04, 0xf0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0d, 0x01, 0x12, 0x04,
    0xf0, 0x01, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xf0,
    0x01, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x0e, 0x12, 0x04, 0xf1, 0x01, 0x08,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xf1, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0e, 0x06, 0x12, 0x04, 0xf1, 0x01, 0x11, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x23, 0x2f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xf1, 0x01, 0x32, 0x34, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x0f, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x0f, 0x04, 0x12, 0x04, 0xf2, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x0f, 0x05, 0x12, 0x04, 0xf2, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x0f, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x0f,
    0x03, 0x12, 0x04, 0xf2, 0x01, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x10, 0x12,
    0x04, 0xf3, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x10, 0x04, 0x12, 0x04,
    0xf3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x10, 0x05, 0x12, 0x04, 0xf3,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x10, 0x01, 0x12, 0x04, 0xf3, 0x01,
    0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x10, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x28,
    0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x11, 0x12, 0x04, 0xf4, 0x01, 0x08, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x11, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x11, 0x05, 0x12, 0x04, 0xf4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x11, 0x01, 0x12, 0x04, 0xf4, 0x01, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x11, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x12, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x12, 0x04, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x12,
    0x05, 0x12, 0x04, 0xf5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x12, 0x01,
    0x12, 0x04, 0xf5, 0x01, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x12, 0x03, 0x12,
    0x04, 0xf5, 0x01, 0x28, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x12, 0x08, 0x12, 0x04,
    0xf5, 0x01, 0x2b, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x12, 0x07, 0x12, 0x04, 0xf5,
    0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x13, 0x12, 0x04, 0xf6, 0x01, 0x08,
    0x56, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x13, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x13, 0x06, 0x12, 0x04, 0xf6, 0x01, 0x11, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x13, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x20, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x13, 0x03, 0x12, 0x04, 0xf6, 0x01, 0x2d, 0x2f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x13, 0x08, 0x12, 0x04, 0xf6, 0x01, 0x30, 0x55, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x13, 0x07, 0x12, 0x04, 0xf6, 0x01, 0x3b, 0x54, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x14, 0x12, 0x04, 0xf7, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x14, 0x04, 0x12, 0x04, 0xf7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x14,
    0x05, 0x12, 0x04, 0xf7, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x14, 0x01,
    0x12, 0x04, 0xf7, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x14, 0x03, 0x12,
    0x04, 0xf7, 0x01, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x15, 0x12, 0x04, 0xf8,
    0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x15, 0x04, 0x12, 0x04, 0xf8, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x15, 0x05, 0x12, 0x04, 0xf8, 0x01, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x15, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x18, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x15, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x25, 0x27, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x16, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x46, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x16, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x16, 0x06, 0x12, 0x04, 0xf9, 0x01, 0x11, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x16, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x1e, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x16, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x28, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x16, 0x08, 0x12, 0x04, 0xf9, 0x01, 0x2b, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x16,
    0x07, 0x12, 0x04, 0xf9, 0x01, 0x36, 0x44, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x17, 0x12,
    0x04, 0xfa, 0x01, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x17, 0x04, 0x12, 0x04,
    0xfa, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x17, 0x05, 0x12, 0x04, 0xfa,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x17, 0x01, 0x12, 0x04, 0xfa, 0x01,
    0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x17, 0x03, 0x12, 0x04, 0xfa, 0x01, 0x23,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x18, 0x12, 0x04, 0xfb, 0x01, 0x08, 0x3d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x18, 0x04, 0x12, 0x04, 0xfb, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x18, 0x05, 0x12, 0x04, 0xfb, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x18, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x16, 0x26, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x18, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x29, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x18, 0x08, 0x12, 0x04, 0xfb, 0x01, 0x2c, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x18, 0x07, 0x12, 0x04, 0xfb, 0x01, 0x37, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x19, 0x12, 0x04, 0xfc, 0x01, 0x08, 0x5f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x04,
    0x12, 0x04, 0xfc, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x06, 0x12,
    0x04, 0xfc, 0x01, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x01, 0x12, 0x04,
    0xfc, 0x01, 0x23, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x03, 0x12, 0x04, 0xfc,
    0x01, 0x3c, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x08, 0x12, 0x04, 0xfc, 0x01,
    0x3f, 0x5e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x19, 0x07, 0x12, 0x04, 0xfc, 0x01, 0x4a,
    0x5d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x1a, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x54, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1a, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x1a, 0x06, 0x12, 0x04, 0xfd, 0x01, 0x11, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x1a, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x1a, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x30, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x1a, 0x08, 0x12, 0x04, 0xfd, 0x01, 0x33, 0x53, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x1a, 0x07, 0x12, 0x04, 0xfd, 0x01, 0x3e, 0x52, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x1b, 0x12, 0x04, 0xfe, 0x01, 0x08, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1b, 0x04,
    0x12, 0x04, 0xfe, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1b, 0x06, 0x12,
    0x04, 0xfe, 0x01, 0x11, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1b, 0x01, 0x12, 0x04,
    0xfe, 0x01, 0x2a, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1b, 0x03, 0x12, 0x04, 0xfe,
    0x01, 0x41, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x1c, 0x12, 0x04, 0xff, 0x01, 0x08,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1c, 0x04, 0x12, 0x04, 0xff, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1c, 0x05, 0x12, 0x04, 0xff, 0x01, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1c, 0x01, 0x12, 0x04, 0xff, 0x01, 0x18, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x1c, 0x03, 0x12, 0x04, 0xff, 0x01, 0x23, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x1d, 0x12, 0x04, 0x80, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x1d, 0x04, 0x12, 0x04, 0x80, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x1d, 0x05, 0x12, 0x04, 0x80, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x1d, 0x01, 0x12, 0x04, 0x80, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1d,
    0x03, 0x12, 0x04, 0x80, 0x02, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x1e, 0x12,
    0x04, 0x81, 0x02, 0x08, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e, 0x04, 0x12, 0x04,
    0x81, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e, 0x05, 0x12, 0x04, 0x81,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e, 0x01, 0x12, 0x04, 0x81, 0x02,
    0x18, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e, 0x03, 0x12, 0x04, 0x81, 0x02, 0x30,
    0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e, 0x08, 0x12, 0x04, 0x81, 0x02, 0x33, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1e, 0x07, 0x12, 0x04, 0x81, 0x02, 0x3e, 0x3f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x1f, 0x12, 0x04, 0x82, 0x02, 0x08, 0x3e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x1f, 0x04, 0x12, 0x04, 0x82, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x1f, 0x05, 0x12, 0x04, 0x82, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x1f, 0x01, 0x12, 0x04, 0x82, 0x02, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x1f, 0x03, 0x12, 0x04, 0x82, 0x02, 0x2d, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x1f, 0x08, 0x12, 0x04, 0x82, 0x02, 0x30, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x1f,
    0x07, 0x12, 0x04, 0x82, 0x02, 0x3b, 0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x20, 0x12,
    0x04, 0x83, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x20, 0x04, 0x12, 0x04,
    0x83, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x20, 0x05, 0x12, 0x04, 0x83,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x20, 0x01, 0x12, 0x04, 0x83, 0x02,
    0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x20, 0x03, 0x12, 0x04, 0x83, 0x02, 0x27,
    0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x21, 0x12, 0x04, 0x84, 0x02, 0x08, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x21, 0x04, 0x12, 0x04, 0x84, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x21, 0x05, 0x12, 0x04, 0x84, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x21, 0x01, 0x12, 0x04, 0x84, 0x02, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x21, 0x03, 0x12, 0x04, 0x84, 0x02, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x22, 0x12, 0x04, 0x85, 0x02, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x22, 0x04, 0x12, 0x04, 0x85, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x22,
    0x05, 0x12, 0x04, 0x85, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x22, 0x01,
    0x12, 0x04, 0x85, 0x02, 0x18, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x22, 0x03, 0x12,
    0x04, 0x85, 0x02, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x23, 0x12, 0x04, 0x86,
    0x02, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x23, 0x04, 0x12, 0x04, 0x86, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x23, 0x05, 0x12, 0x04, 0x86, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x23, 0x01, 0x12, 0x04, 0x86, 0x02, 0x18, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x23, 0x03, 0x12, 0x04, 0x86, 0x02, 0x2b, 0x2d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x24, 0x12, 0x04, 0x87, 0x02, 0x08, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x24, 0x04, 0x12, 0x04, 0x87, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x24, 0x05, 0x12, 0x04, 0x87, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x24, 0x01, 0x12, 0x04, 0x87, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x24, 0x03, 0x12, 0x04, 0x87, 0x02, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x25, 0x12, 0x04, 0x88, 0x02, 0x08, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x25, 0x04,
    0x12, 0x04, 0x88, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x25, 0x05, 0x12,
    0x04, 0x88, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x25, 0x01, 0x12, 0x04,
    0x88, 0x02, 0x18, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x25, 0x03, 0x12, 0x04, 0x88,
    0x02, 0x27, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x26, 0x12, 0x04, 0x89, 0x02, 0x08,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x26, 0x04, 0x12, 0x04, 0x89, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x26, 0x05, 0x12, 0x04, 0x89, 0x02, 0x11, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x26, 0x01, 0x12, 0x04, 0x89, 0x02, 0x16, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x26, 0x03, 0x12, 0x04, 0x89, 0x02, 0x20, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x26, 0x08, 0x12, 0x04, 0x89, 0x02, 0x23, 0x34, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x26, 0x07, 0x12, 0x04, 0x89, 0x02, 0x2e, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x27, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x27, 0x04, 0x12, 0x04, 0x8a, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27,
    0x06, 0x12, 0x04, 0x8a, 0x02, 0x11, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27, 0x01,
    0x12, 0x04, 0x8a, 0x02, 0x22, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27, 0x03, 0x12,
    0x04, 0x8a, 0x02, 0x32, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27, 0x08, 0x12, 0x04,
    0x8a, 0x02, 0x35, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x27, 0x07, 0x12, 0x04, 0x8a,
    0x02, 0x40, 0x4e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x28, 0x12, 0x04, 0x8b, 0x02, 0x08,
    0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x28, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x28, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x28, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x18, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x28, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x29, 0x12, 0x04, 0x8c, 0x02, 0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x29, 0x04, 0x12, 0x04, 0x8c, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x29, 0x05, 0x12, 0x04, 0x8c, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x29, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x29,
    0x03, 0x12, 0x04, 0x8c, 0x02, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x2a, 0x12,
    0x04, 0x8d, 0x02, 0x08, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2a, 0x04, 0x12, 0x04,
    0x8d, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2a, 0x05, 0x12, 0x04, 0x8d,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2a, 0x01, 0x12, 0x04, 0x8d, 0x02,
    0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2a, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x2c,
    0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x2b, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2b, 0x04, 0x12, 0x04, 0x8e, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x2b, 0x05, 0x12, 0x04, 0x8e, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x2b, 0x01, 0x12, 0x04, 0x8e, 0x02, 0x16, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x2b, 0x03, 0x12, 0x04, 0x8e, 0x02, 0x1c, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x2c, 0x12, 0x04, 0x8f, 0x02, 0x08, 0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x2c, 0x04, 0x12, 0x04, 0x8f, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2c,
    0x06, 0x12, 0x04, 0x8f, 0x02, 0x11, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2c, 0x01,
    0x12, 0x04, 0x8f, 0x02, 0x2c, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2c, 0x03, 0x12,
    0x04, 0x8f, 0x02, 0x45, 0x47, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x2d, 0x12, 0x04, 0x90,
    0x02, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2d, 0x04, 0x12, 0x04, 0x90, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2d, 0x05, 0x12, 0x04, 0x90, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2d, 0x01, 0x12, 0x04, 0x90, 0x02, 0x18, 0x2e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2d, 0x03, 0x12, 0x04, 0x90, 0x02, 0x31, 0x33, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x2e, 0x12, 0x04, 0x91, 0x02, 0x08, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x2e, 0x04, 0x12, 0x04, 0x91, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x2e, 0x05, 0x12, 0x04, 0x91, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x2e, 0x01, 0x12, 0x04, 0x91, 0x02, 0x18, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x2e, 0x03, 0x12, 0x04, 0x91, 0x02, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x2f, 0x12, 0x04, 0x92, 0x02, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2f, 0x04,
    0x12, 0x04, 0x92, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2f, 0x05, 0x12,
    0x04, 0x92, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2f, 0x01, 0x12, 0x04,
    0x92, 0x02, 0x16, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x2f, 0x03, 0x12, 0x04, 0x92,
    0x02, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x30, 0x12, 0x04, 0x93, 0x02, 0x08,
    0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x30, 0x04, 0x12, 0x04, 0x93, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x30, 0x06, 0x12, 0x04, 0x93, 0x02, 0x11, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x30, 0x01, 0x12, 0x04, 0x93, 0x02, 0x28, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x30, 0x03, 0x12, 0x04, 0x93, 0x02, 0x39, 0x3b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x31, 0x12, 0x04, 0x94, 0x02, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x31, 0x04, 0x12, 0x04, 0x94, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x31, 0x06, 0x12, 0x04, 0x94, 0x02, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x31, 0x01, 0x12, 0x04, 0x94, 0x02, 0x1f, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x31,
    0x03, 0x12, 0x04, 0x94, 0x02, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x32, 0x12,
    0x04, 0x95, 0x02, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x32, 0x04, 0x12, 0x04,
    0x95, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x32, 0x05, 0x12, 0x04, 0x95,
    0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x32, 0x01, 0x12, 0x04, 0x95, 0x02,
    0x16, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x32, 0x03, 0x12, 0x04, 0x95, 0x02, 0x2d,
    0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x33, 0x12, 0x04, 0x96, 0x02, 0x08, 0x56, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x33, 0x04, 0x12, 0x04, 0x96, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x33, 0x06, 0x12, 0x04, 0x96, 0x02, 0x11, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x33, 0x01, 0x12, 0x04, 0x96, 0x02, 0x1f, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x33, 0x03, 0x12, 0x04, 0x96, 0x02, 0x2f, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x33, 0x08, 0x12, 0x04, 0x96, 0x02, 0x32, 0x55, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x33, 0x07, 0x12, 0x04, 0x96, 0x02, 0x3d, 0x54, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x34, 0x12, 0x04, 0x97, 0x02, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x34, 0x04,
    0x12, 0x04, 0x97, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x34, 0x05, 0x12,
    0x04, 0x97, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x34, 0x01, 0x12, 0x04,
    0x97, 0x02, 0x16, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x34, 0x03, 0x12, 0x04, 0x97,
    0x02, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x35, 0x12, 0x04, 0x98, 0x02, 0x08,
    0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x35, 0x04, 0x12, 0x04, 0x98, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x35, 0x05, 0x12, 0x04, 0x98, 0x02, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x35, 0x01, 0x12, 0x04, 0x98, 0x02, 0x18, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x35, 0x03, 0x12, 0x04, 0x98, 0x02, 0x29, 0x2b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x36, 0x12, 0x04, 0x99, 0x02, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x36, 0x04, 0x12, 0x04, 0x99, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x36, 0x06, 0x12, 0x04, 0x99, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x36, 0x01, 0x12, 0x04, 0x99, 0x02, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x36,
    0x03, 0x12, 0x04, 0x99, 0x02, 0x2f, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x37, 0x12,
    0x04, 0x9a, 0x02, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x37, 0x04, 0x12, 0x04,
    0x9a, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x37, 0x05, 0x12, 0x04, 0x9a,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x37, 0x01, 0x12, 0x04, 0x9a, 0x02,
    0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x37, 0x03, 0x12, 0x04, 0x9a, 0x02, 0x2d,
    0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x38, 0x12, 0x04, 0x9b, 0x02, 0x08, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x38, 0x04, 0x12, 0x04, 0x9b, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x38, 0x05, 0x12, 0x04, 0x9b, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x38, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x18, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x38, 0x03, 0x12, 0x04, 0x9b, 0x02, 0x2d, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x39, 0x12, 0x04, 0x9c, 0x02, 0x08, 0x4f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x39, 0x04, 0x12, 0x04, 0x9c, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x39,
    0x06, 0x12, 0x04, 0x9c, 0x02, 0x11, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x39, 0x01,
    0x12, 0x04, 0x9c, 0x02, 0x24, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x39, 0x03, 0x12,
    0x04, 0x9c, 0x02, 0x33, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x39, 0x08, 0x12, 0x04,
    0x9c, 0x02, 0x36, 0x4e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x39, 0x07, 0x12, 0x04, 0x9c,
    0x02, 0x41, 0x4d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x3a, 0x12, 0x04, 0x9d, 0x02, 0x08,
    0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3a, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3a, 0x05, 0x12, 0x04, 0x9d, 0x02, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3a, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x18, 0x36, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x3a, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x39, 0x3b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x3b, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x3b, 0x04, 0x12, 0x04, 0x9e, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x3b, 0x06, 0x12, 0x04, 0x9e, 0x02, 0x11, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x3b, 0x01, 0x12, 0x04, 0x9e, 0x02, 0x25, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3b,
    0x03, 0x12, 0x04, 0x9e, 0x02, 0x32, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3b, 0x08,
    0x12, 0x04, 0x9e, 0x02, 0x35, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3b, 0x07, 0x12,
    0x04, 0x9e, 0x02, 0x40, 0x5a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x3c, 0x12, 0x04, 0x9f,
    0x02, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3c, 0x04, 0x12, 0x04, 0x9f, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3c, 0x05, 0x12, 0x04, 0x9f, 0x02, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3c, 0x01, 0x12, 0x04, 0x9f, 0x02, 0x19, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3c, 0x03, 0x12, 0x04, 0x9f, 0x02, 0x2b, 0x2d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x3d, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x3a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x3d, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x3d, 0x05, 0x12, 0x04, 0xa0, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x3d, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x16, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x3d, 0x03, 0x12, 0x04, 0xa0, 0x02, 0x37, 0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x3e, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3e, 0x04,
    0x12, 0x04, 0xa1, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3e, 0x05, 0x12,
    0x04, 0xa1, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3e, 0x01, 0x12, 0x04,
    0xa1, 0x02, 0x18, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3e, 0x03, 0x12, 0x04, 0xa1,
    0x02, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x3f, 0x12, 0x04, 0xa2, 0x02, 0x08,
    0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3f, 0x04, 0x12, 0x04, 0xa2, 0x02, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3f, 0x05, 0x12, 0x04, 0xa2, 0x02, 0x11, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x3f, 0x01, 0x12, 0x04, 0xa2, 0x02, 0x18, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x3f, 0x03, 0x12, 0x04, 0xa2, 0x02, 0x29, 0x2b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x09, 0x02, 0x40, 0x12, 0x04, 0xa3, 0x02, 0x08, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x40, 0x04, 0x12, 0x04, 0xa3, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x40, 0x05, 0x12, 0x04, 0xa3, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x40, 0x01, 0x12, 0x04, 0xa3, 0x02, 0x19, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x40,
    0x03, 0x12, 0x04, 0xa3, 0x02, 0x31, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x41, 0x12,
    0x04, 0xa4, 0x02, 0x08, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x41, 0x04, 0x12, 0x04,
    0xa4, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x41, 0x05, 0x12, 0x04, 0xa4,
    0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x41, 0x01, 0x12, 0x04, 0xa4, 0x02,
    0x18, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x41, 0x03, 0x12, 0x04, 0xa4, 0x02, 0x32,
    0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x42, 0x12, 0x04, 0xa5, 0x02, 0x08, 0x35, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x42, 0x04, 0x12, 0x04, 0xa5, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x42, 0x05, 0x12, 0x04, 0xa5, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x42, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x18, 0x2f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x42, 0x03, 0x12, 0x04, 0xa5, 0x02, 0x32, 0x34, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x43, 0x12, 0x04, 0xa6, 0x02, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x43, 0x04, 0x12, 0x04, 0xa6, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x43,
    0x05, 0x12, 0x04, 0xa6, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x43, 0x01,
    0x12, 0x04, 0xa6, 0x02, 0x16, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x43, 0x03, 0x12,
    0x04, 0xa6, 0x02, 0x39, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x44, 0x12, 0x04, 0xa7,
    0x02, 0x08, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x44, 0x04, 0x12, 0x04, 0xa7, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x44, 0x05, 0x12, 0x04, 0xa7, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x44, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x18, 0x36,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x44, 0x03, 0x12, 0x04, 0xa7, 0x02, 0x39, 0x3b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x45, 0x12, 0x04, 0xa8, 0x02, 0x08, 0x63, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x45, 0x04, 0x12, 0x04, 0xa8, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x45, 0x06, 0x12, 0x04, 0xa8, 0x02, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x45, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x27, 0x47, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x45, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x4a, 0x4c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x45, 0x08, 0x12, 0x04, 0xa8, 0x02, 0x4d, 0x62, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x45,
    0x07, 0x12, 0x04, 0xa8, 0x02, 0x58, 0x61, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x46, 0x12,
    0x04, 0xa9, 0x02, 0x08, 0x67, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x46, 0x04, 0x12, 0x04,
    0xa9, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x46, 0x06, 0x12, 0x04, 0xa9,
    0x02, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x46, 0x01, 0x12, 0x04, 0xa9, 0x02,
    0x27, 0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x46, 0x03, 0x12, 0x04, 0xa9, 0x02, 0x4e,
    0x50, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x46, 0x08, 0x12, 0x04, 0xa9, 0x02, 0x51, 0x66,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x46, 0x07, 0x12, 0x04, 0xa9, 0x02, 0x5c, 0x65, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x47, 0x12, 0x04, 0xaa, 0x02, 0x08, 0x2d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x47, 0x04, 0x12, 0x04, 0xaa, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x47, 0x05, 0x12, 0x04, 0xaa, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x47, 0x01, 0x12, 0x04, 0xaa, 0x02, 0x18, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x47, 0x03, 0x12, 0x04, 0xaa, 0x02, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x48, 0x12, 0x04, 0xab, 0x02, 0x08, 0x66, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x48, 0x04,
    0x12, 0x04, 0xab, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x48, 0x06, 0x12,
    0x04, 0xab, 0x02, 0x11, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x48, 0x01, 0x12, 0x04,
    0xab, 0x02, 0x27, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x48, 0x03, 0x12, 0x04, 0xab,
    0x02, 0x37, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x48, 0x08, 0x12, 0x04, 0xab, 0x02,
    0x3a, 0x65, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x48, 0x07, 0x12, 0x04, 0xab, 0x02, 0x45,
    0x64, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x49, 0x12, 0x04, 0xac, 0x02, 0x08, 0x32, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x49, 0x04, 0x12, 0x04, 0xac, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x49, 0x05, 0x12, 0x04, 0xac, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x49, 0x01, 0x12, 0x04, 0xac, 0x02, 0x18, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x49, 0x03, 0x12, 0x04, 0xac, 0x02, 0x2f, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x4a, 0x12, 0x04, 0xad, 0x02, 0x08, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x4a, 0x04, 0x12, 0x04, 0xad, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4a,
    0x05, 0x12, 0x04, 0xad, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4a, 0x01,
    0x12, 0x04, 0xad, 0x02, 0x18, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4a, 0x03, 0x12,
    0x04, 0xad, 0x02, 0x36, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x4b, 0x12, 0x04, 0xae,
    0x02, 0x08, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4b, 0x04, 0x12, 0x04, 0xae, 0x02,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4b, 0x05, 0x12, 0x04, 0xae, 0x02, 0x11,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4b, 0x01, 0x12, 0x04, 0xae, 0x02, 0x18, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4b, 0x03, 0x12, 0x04, 0xae, 0x02, 0x36, 0x38, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x4c, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x3b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x4c, 0x04, 0x12, 0x04, 0xaf, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x4c, 0x05, 0x12, 0x04, 0xaf, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x4c, 0x01, 0x12, 0x04, 0xaf, 0x02, 0x18, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x4c, 0x03, 0x12, 0x04, 0xaf, 0x02, 0x38, 0x3a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x4d, 0x12, 0x04, 0xb0, 0x02, 0x08, 0x5c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4d, 0x04,
    0x12, 0x04, 0xb0, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4d, 0x06, 0x12,
    0x04, 0xb0, 0x02, 0x11, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4d, 0x01, 0x12, 0x04,
    0xb0, 0x02, 0x23, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4d, 0x03, 0x12, 0x04, 0xb0,
    0x02, 0x39, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4d, 0x08, 0x12, 0x04, 0xb0, 0x02,
    0x3c, 0x5b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4d, 0x07, 0x12, 0x04, 0xb0, 0x02, 0x47,
    0x5a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x4e, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4e, 0x04, 0x12, 0x04, 0xb1, 0x02, 0x08, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x4e, 0x05, 0x12, 0x04, 0xb1, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x4e, 0x01, 0x12, 0x04, 0xb1, 0x02, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x4e, 0x03, 0x12, 0x04, 0xb1, 0x02, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x4f, 0x12, 0x04, 0xb2, 0x02, 0x08, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x4f, 0x04, 0x12, 0x04, 0xb2, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4f,
    0x05, 0x12, 0x04, 0xb2, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4f, 0x01,
    0x12, 0x04, 0xb2, 0x02, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x4f, 0x03, 0x12,
    0x04, 0xb2, 0x02, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0xb5, 0x02, 0x00,
    0xb7, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0xb5, 0x02, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04, 0xb6, 0x02, 0x08, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb6, 0x02, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xb6, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb6, 0x02, 0x1f, 0x20,
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
