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
pub struct CSourceTVGameSmall {
    // message fields
    activate_time: ::std::option::Option<u32>,
    deactivate_time: ::std::option::Option<u32>,
    server_steam_id: ::std::option::Option<u64>,
    lobby_id: ::std::option::Option<u64>,
    league_id: ::std::option::Option<u32>,
    lobby_type: ::std::option::Option<u32>,
    game_time: ::std::option::Option<i32>,
    delay: ::std::option::Option<u32>,
    spectators: ::std::option::Option<u32>,
    game_mode: ::std::option::Option<u32>,
    average_mmr: ::std::option::Option<u32>,
    team_name_radiant: ::protobuf::SingularField<::std::string::String>,
    team_name_dire: ::protobuf::SingularField<::std::string::String>,
    team_logo_radiant: ::std::option::Option<u64>,
    team_logo_dire: ::std::option::Option<u64>,
    sort_score: ::std::option::Option<u32>,
    last_update_time: ::std::option::Option<f32>,
    radiant_lead: ::std::option::Option<i32>,
    radiant_score: ::std::option::Option<u32>,
    dire_score: ::std::option::Option<u32>,
    players: ::protobuf::RepeatedField<CSourceTVGameSmall_Player>,
    building_state: ::std::option::Option<u32>,
    weekend_tourney_tournament_id: ::std::option::Option<u32>,
    weekend_tourney_division: ::std::option::Option<u32>,
    weekend_tourney_skill_level: ::std::option::Option<u32>,
    weekend_tourney_bracket_round: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSourceTVGameSmall {}

impl CSourceTVGameSmall {
    pub fn new() -> CSourceTVGameSmall {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSourceTVGameSmall {
        static mut instance: ::protobuf::lazy::Lazy<CSourceTVGameSmall> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSourceTVGameSmall,
        };
        unsafe {
            instance.get(CSourceTVGameSmall::new)
        }
    }

    // optional uint32 activate_time = 1;

    pub fn clear_activate_time(&mut self) {
        self.activate_time = ::std::option::Option::None;
    }

    pub fn has_activate_time(&self) -> bool {
        self.activate_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_activate_time(&mut self, v: u32) {
        self.activate_time = ::std::option::Option::Some(v);
    }

    pub fn get_activate_time(&self) -> u32 {
        self.activate_time.unwrap_or(0)
    }

    fn get_activate_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.activate_time
    }

    fn mut_activate_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.activate_time
    }

    // optional uint32 deactivate_time = 2;

    pub fn clear_deactivate_time(&mut self) {
        self.deactivate_time = ::std::option::Option::None;
    }

    pub fn has_deactivate_time(&self) -> bool {
        self.deactivate_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deactivate_time(&mut self, v: u32) {
        self.deactivate_time = ::std::option::Option::Some(v);
    }

    pub fn get_deactivate_time(&self) -> u32 {
        self.deactivate_time.unwrap_or(0)
    }

    fn get_deactivate_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.deactivate_time
    }

    fn mut_deactivate_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.deactivate_time
    }

    // optional uint64 server_steam_id = 3;

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

    // optional uint64 lobby_id = 4;

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

    // optional uint32 league_id = 5;

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

    // optional uint32 lobby_type = 6;

    pub fn clear_lobby_type(&mut self) {
        self.lobby_type = ::std::option::Option::None;
    }

    pub fn has_lobby_type(&self) -> bool {
        self.lobby_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lobby_type(&mut self, v: u32) {
        self.lobby_type = ::std::option::Option::Some(v);
    }

    pub fn get_lobby_type(&self) -> u32 {
        self.lobby_type.unwrap_or(0)
    }

    fn get_lobby_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lobby_type
    }

    fn mut_lobby_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lobby_type
    }

    // optional int32 game_time = 7;

    pub fn clear_game_time(&mut self) {
        self.game_time = ::std::option::Option::None;
    }

    pub fn has_game_time(&self) -> bool {
        self.game_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_time(&mut self, v: i32) {
        self.game_time = ::std::option::Option::Some(v);
    }

    pub fn get_game_time(&self) -> i32 {
        self.game_time.unwrap_or(0)
    }

    fn get_game_time_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.game_time
    }

    fn mut_game_time_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.game_time
    }

    // optional uint32 delay = 8;

    pub fn clear_delay(&mut self) {
        self.delay = ::std::option::Option::None;
    }

    pub fn has_delay(&self) -> bool {
        self.delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay(&mut self, v: u32) {
        self.delay = ::std::option::Option::Some(v);
    }

    pub fn get_delay(&self) -> u32 {
        self.delay.unwrap_or(0)
    }

    fn get_delay_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.delay
    }

    fn mut_delay_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.delay
    }

    // optional uint32 spectators = 9;

    pub fn clear_spectators(&mut self) {
        self.spectators = ::std::option::Option::None;
    }

    pub fn has_spectators(&self) -> bool {
        self.spectators.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spectators(&mut self, v: u32) {
        self.spectators = ::std::option::Option::Some(v);
    }

    pub fn get_spectators(&self) -> u32 {
        self.spectators.unwrap_or(0)
    }

    fn get_spectators_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spectators
    }

    fn mut_spectators_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spectators
    }

    // optional uint32 game_mode = 10;

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

    // optional uint32 average_mmr = 11;

    pub fn clear_average_mmr(&mut self) {
        self.average_mmr = ::std::option::Option::None;
    }

    pub fn has_average_mmr(&self) -> bool {
        self.average_mmr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_average_mmr(&mut self, v: u32) {
        self.average_mmr = ::std::option::Option::Some(v);
    }

    pub fn get_average_mmr(&self) -> u32 {
        self.average_mmr.unwrap_or(0)
    }

    fn get_average_mmr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.average_mmr
    }

    fn mut_average_mmr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.average_mmr
    }

    // optional string team_name_radiant = 15;

    pub fn clear_team_name_radiant(&mut self) {
        self.team_name_radiant.clear();
    }

    pub fn has_team_name_radiant(&self) -> bool {
        self.team_name_radiant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name_radiant(&mut self, v: ::std::string::String) {
        self.team_name_radiant = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name_radiant(&mut self) -> &mut ::std::string::String {
        if self.team_name_radiant.is_none() {
            self.team_name_radiant.set_default();
        };
        self.team_name_radiant.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name_radiant(&mut self) -> ::std::string::String {
        self.team_name_radiant.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name_radiant(&self) -> &str {
        match self.team_name_radiant.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_radiant_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name_radiant
    }

    fn mut_team_name_radiant_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name_radiant
    }

    // optional string team_name_dire = 16;

    pub fn clear_team_name_dire(&mut self) {
        self.team_name_dire.clear();
    }

    pub fn has_team_name_dire(&self) -> bool {
        self.team_name_dire.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name_dire(&mut self, v: ::std::string::String) {
        self.team_name_dire = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name_dire(&mut self) -> &mut ::std::string::String {
        if self.team_name_dire.is_none() {
            self.team_name_dire.set_default();
        };
        self.team_name_dire.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name_dire(&mut self) -> ::std::string::String {
        self.team_name_dire.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name_dire(&self) -> &str {
        match self.team_name_dire.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_dire_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name_dire
    }

    fn mut_team_name_dire_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name_dire
    }

    // optional fixed64 team_logo_radiant = 24;

    pub fn clear_team_logo_radiant(&mut self) {
        self.team_logo_radiant = ::std::option::Option::None;
    }

    pub fn has_team_logo_radiant(&self) -> bool {
        self.team_logo_radiant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_logo_radiant(&mut self, v: u64) {
        self.team_logo_radiant = ::std::option::Option::Some(v);
    }

    pub fn get_team_logo_radiant(&self) -> u64 {
        self.team_logo_radiant.unwrap_or(0)
    }

    fn get_team_logo_radiant_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_logo_radiant
    }

    fn mut_team_logo_radiant_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_logo_radiant
    }

    // optional fixed64 team_logo_dire = 25;

    pub fn clear_team_logo_dire(&mut self) {
        self.team_logo_dire = ::std::option::Option::None;
    }

    pub fn has_team_logo_dire(&self) -> bool {
        self.team_logo_dire.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_logo_dire(&mut self, v: u64) {
        self.team_logo_dire = ::std::option::Option::Some(v);
    }

    pub fn get_team_logo_dire(&self) -> u64 {
        self.team_logo_dire.unwrap_or(0)
    }

    fn get_team_logo_dire_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.team_logo_dire
    }

    fn mut_team_logo_dire_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.team_logo_dire
    }

    // optional uint32 sort_score = 17;

    pub fn clear_sort_score(&mut self) {
        self.sort_score = ::std::option::Option::None;
    }

    pub fn has_sort_score(&self) -> bool {
        self.sort_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sort_score(&mut self, v: u32) {
        self.sort_score = ::std::option::Option::Some(v);
    }

    pub fn get_sort_score(&self) -> u32 {
        self.sort_score.unwrap_or(0)
    }

    fn get_sort_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sort_score
    }

    fn mut_sort_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sort_score
    }

    // optional float last_update_time = 18;

    pub fn clear_last_update_time(&mut self) {
        self.last_update_time = ::std::option::Option::None;
    }

    pub fn has_last_update_time(&self) -> bool {
        self.last_update_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_update_time(&mut self, v: f32) {
        self.last_update_time = ::std::option::Option::Some(v);
    }

    pub fn get_last_update_time(&self) -> f32 {
        self.last_update_time.unwrap_or(0.)
    }

    fn get_last_update_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.last_update_time
    }

    fn mut_last_update_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.last_update_time
    }

    // optional int32 radiant_lead = 19;

    pub fn clear_radiant_lead(&mut self) {
        self.radiant_lead = ::std::option::Option::None;
    }

    pub fn has_radiant_lead(&self) -> bool {
        self.radiant_lead.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_lead(&mut self, v: i32) {
        self.radiant_lead = ::std::option::Option::Some(v);
    }

    pub fn get_radiant_lead(&self) -> i32 {
        self.radiant_lead.unwrap_or(0)
    }

    fn get_radiant_lead_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.radiant_lead
    }

    fn mut_radiant_lead_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.radiant_lead
    }

    // optional uint32 radiant_score = 20;

    pub fn clear_radiant_score(&mut self) {
        self.radiant_score = ::std::option::Option::None;
    }

    pub fn has_radiant_score(&self) -> bool {
        self.radiant_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_score(&mut self, v: u32) {
        self.radiant_score = ::std::option::Option::Some(v);
    }

    pub fn get_radiant_score(&self) -> u32 {
        self.radiant_score.unwrap_or(0)
    }

    fn get_radiant_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.radiant_score
    }

    fn mut_radiant_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.radiant_score
    }

    // optional uint32 dire_score = 21;

    pub fn clear_dire_score(&mut self) {
        self.dire_score = ::std::option::Option::None;
    }

    pub fn has_dire_score(&self) -> bool {
        self.dire_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dire_score(&mut self, v: u32) {
        self.dire_score = ::std::option::Option::Some(v);
    }

    pub fn get_dire_score(&self) -> u32 {
        self.dire_score.unwrap_or(0)
    }

    fn get_dire_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dire_score
    }

    fn mut_dire_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dire_score
    }

    // repeated .dota.CSourceTVGameSmall.Player players = 22;

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: ::protobuf::RepeatedField<CSourceTVGameSmall_Player>) {
        self.players = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players(&mut self) -> &mut ::protobuf::RepeatedField<CSourceTVGameSmall_Player> {
        &mut self.players
    }

    // Take field
    pub fn take_players(&mut self) -> ::protobuf::RepeatedField<CSourceTVGameSmall_Player> {
        ::std::mem::replace(&mut self.players, ::protobuf::RepeatedField::new())
    }

    pub fn get_players(&self) -> &[CSourceTVGameSmall_Player] {
        &self.players
    }

    fn get_players_for_reflect(&self) -> &::protobuf::RepeatedField<CSourceTVGameSmall_Player> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSourceTVGameSmall_Player> {
        &mut self.players
    }

    // optional fixed32 building_state = 23;

    pub fn clear_building_state(&mut self) {
        self.building_state = ::std::option::Option::None;
    }

    pub fn has_building_state(&self) -> bool {
        self.building_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_building_state(&mut self, v: u32) {
        self.building_state = ::std::option::Option::Some(v);
    }

    pub fn get_building_state(&self) -> u32 {
        self.building_state.unwrap_or(0)
    }

    fn get_building_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.building_state
    }

    fn mut_building_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.building_state
    }

    // optional uint32 weekend_tourney_tournament_id = 26;

    pub fn clear_weekend_tourney_tournament_id(&mut self) {
        self.weekend_tourney_tournament_id = ::std::option::Option::None;
    }

    pub fn has_weekend_tourney_tournament_id(&self) -> bool {
        self.weekend_tourney_tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weekend_tourney_tournament_id(&mut self, v: u32) {
        self.weekend_tourney_tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_weekend_tourney_tournament_id(&self) -> u32 {
        self.weekend_tourney_tournament_id.unwrap_or(0)
    }

    fn get_weekend_tourney_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.weekend_tourney_tournament_id
    }

    fn mut_weekend_tourney_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.weekend_tourney_tournament_id
    }

    // optional uint32 weekend_tourney_division = 27;

    pub fn clear_weekend_tourney_division(&mut self) {
        self.weekend_tourney_division = ::std::option::Option::None;
    }

    pub fn has_weekend_tourney_division(&self) -> bool {
        self.weekend_tourney_division.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weekend_tourney_division(&mut self, v: u32) {
        self.weekend_tourney_division = ::std::option::Option::Some(v);
    }

    pub fn get_weekend_tourney_division(&self) -> u32 {
        self.weekend_tourney_division.unwrap_or(0)
    }

    fn get_weekend_tourney_division_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.weekend_tourney_division
    }

    fn mut_weekend_tourney_division_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.weekend_tourney_division
    }

    // optional uint32 weekend_tourney_skill_level = 28;

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

    // optional uint32 weekend_tourney_bracket_round = 29;

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
}

impl ::protobuf::Message for CSourceTVGameSmall {
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
                    self.activate_time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.deactivate_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.server_steam_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.lobby_type = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.game_time = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.delay = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.spectators = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.average_mmr = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name_radiant)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name_dire)?;
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.team_logo_radiant = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.team_logo_dire = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.sort_score = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.last_update_time = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.radiant_lead = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.radiant_score = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.dire_score = ::std::option::Option::Some(tmp);
                },
                22 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.players)?;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.building_state = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_tournament_id = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_division = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_skill_level = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_bracket_round = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.activate_time {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.deactivate_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.server_steam_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lobby_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.lobby_type {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_time {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.delay {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.spectators {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.average_mmr {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_name_radiant.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        };
        if let Some(v) = self.team_name_dire.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        };
        if let Some(v) = self.team_logo_radiant {
            my_size += 10;
        };
        if let Some(v) = self.team_logo_dire {
            my_size += 10;
        };
        if let Some(v) = self.sort_score {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.last_update_time {
            my_size += 6;
        };
        if let Some(v) = self.radiant_lead {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radiant_score {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dire_score {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.players {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.building_state {
            my_size += 6;
        };
        if let Some(v) = self.weekend_tourney_tournament_id {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_division {
            my_size += ::protobuf::rt::value_size(27, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_skill_level {
            my_size += ::protobuf::rt::value_size(28, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_bracket_round {
            my_size += ::protobuf::rt::value_size(29, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.activate_time {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.deactivate_time {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.server_steam_id {
            os.write_uint64(3, v)?;
        };
        if let Some(v) = self.lobby_id {
            os.write_uint64(4, v)?;
        };
        if let Some(v) = self.league_id {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.lobby_type {
            os.write_uint32(6, v)?;
        };
        if let Some(v) = self.game_time {
            os.write_int32(7, v)?;
        };
        if let Some(v) = self.delay {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.spectators {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.game_mode {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.average_mmr {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.team_name_radiant.as_ref() {
            os.write_string(15, &v)?;
        };
        if let Some(v) = self.team_name_dire.as_ref() {
            os.write_string(16, &v)?;
        };
        if let Some(v) = self.team_logo_radiant {
            os.write_fixed64(24, v)?;
        };
        if let Some(v) = self.team_logo_dire {
            os.write_fixed64(25, v)?;
        };
        if let Some(v) = self.sort_score {
            os.write_uint32(17, v)?;
        };
        if let Some(v) = self.last_update_time {
            os.write_float(18, v)?;
        };
        if let Some(v) = self.radiant_lead {
            os.write_int32(19, v)?;
        };
        if let Some(v) = self.radiant_score {
            os.write_uint32(20, v)?;
        };
        if let Some(v) = self.dire_score {
            os.write_uint32(21, v)?;
        };
        for v in &self.players {
            os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.building_state {
            os.write_fixed32(23, v)?;
        };
        if let Some(v) = self.weekend_tourney_tournament_id {
            os.write_uint32(26, v)?;
        };
        if let Some(v) = self.weekend_tourney_division {
            os.write_uint32(27, v)?;
        };
        if let Some(v) = self.weekend_tourney_skill_level {
            os.write_uint32(28, v)?;
        };
        if let Some(v) = self.weekend_tourney_bracket_round {
            os.write_uint32(29, v)?;
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

impl ::protobuf::MessageStatic for CSourceTVGameSmall {
    fn new() -> CSourceTVGameSmall {
        CSourceTVGameSmall::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSourceTVGameSmall>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "activate_time",
                    CSourceTVGameSmall::get_activate_time_for_reflect,
                    CSourceTVGameSmall::mut_activate_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "deactivate_time",
                    CSourceTVGameSmall::get_deactivate_time_for_reflect,
                    CSourceTVGameSmall::mut_deactivate_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "server_steam_id",
                    CSourceTVGameSmall::get_server_steam_id_for_reflect,
                    CSourceTVGameSmall::mut_server_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lobby_id",
                    CSourceTVGameSmall::get_lobby_id_for_reflect,
                    CSourceTVGameSmall::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CSourceTVGameSmall::get_league_id_for_reflect,
                    CSourceTVGameSmall::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lobby_type",
                    CSourceTVGameSmall::get_lobby_type_for_reflect,
                    CSourceTVGameSmall::mut_lobby_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "game_time",
                    CSourceTVGameSmall::get_game_time_for_reflect,
                    CSourceTVGameSmall::mut_game_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "delay",
                    CSourceTVGameSmall::get_delay_for_reflect,
                    CSourceTVGameSmall::mut_delay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spectators",
                    CSourceTVGameSmall::get_spectators_for_reflect,
                    CSourceTVGameSmall::mut_spectators_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_mode",
                    CSourceTVGameSmall::get_game_mode_for_reflect,
                    CSourceTVGameSmall::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "average_mmr",
                    CSourceTVGameSmall::get_average_mmr_for_reflect,
                    CSourceTVGameSmall::mut_average_mmr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name_radiant",
                    CSourceTVGameSmall::get_team_name_radiant_for_reflect,
                    CSourceTVGameSmall::mut_team_name_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name_dire",
                    CSourceTVGameSmall::get_team_name_dire_for_reflect,
                    CSourceTVGameSmall::mut_team_name_dire_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "team_logo_radiant",
                    CSourceTVGameSmall::get_team_logo_radiant_for_reflect,
                    CSourceTVGameSmall::mut_team_logo_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "team_logo_dire",
                    CSourceTVGameSmall::get_team_logo_dire_for_reflect,
                    CSourceTVGameSmall::mut_team_logo_dire_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sort_score",
                    CSourceTVGameSmall::get_sort_score_for_reflect,
                    CSourceTVGameSmall::mut_sort_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "last_update_time",
                    CSourceTVGameSmall::get_last_update_time_for_reflect,
                    CSourceTVGameSmall::mut_last_update_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "radiant_lead",
                    CSourceTVGameSmall::get_radiant_lead_for_reflect,
                    CSourceTVGameSmall::mut_radiant_lead_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "radiant_score",
                    CSourceTVGameSmall::get_radiant_score_for_reflect,
                    CSourceTVGameSmall::mut_radiant_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dire_score",
                    CSourceTVGameSmall::get_dire_score_for_reflect,
                    CSourceTVGameSmall::mut_dire_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSourceTVGameSmall_Player>>(
                    "players",
                    CSourceTVGameSmall::get_players_for_reflect,
                    CSourceTVGameSmall::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "building_state",
                    CSourceTVGameSmall::get_building_state_for_reflect,
                    CSourceTVGameSmall::mut_building_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_tournament_id",
                    CSourceTVGameSmall::get_weekend_tourney_tournament_id_for_reflect,
                    CSourceTVGameSmall::mut_weekend_tourney_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_division",
                    CSourceTVGameSmall::get_weekend_tourney_division_for_reflect,
                    CSourceTVGameSmall::mut_weekend_tourney_division_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_skill_level",
                    CSourceTVGameSmall::get_weekend_tourney_skill_level_for_reflect,
                    CSourceTVGameSmall::mut_weekend_tourney_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_bracket_round",
                    CSourceTVGameSmall::get_weekend_tourney_bracket_round_for_reflect,
                    CSourceTVGameSmall::mut_weekend_tourney_bracket_round_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSourceTVGameSmall>(
                    "CSourceTVGameSmall",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSourceTVGameSmall {
    fn clear(&mut self) {
        self.clear_activate_time();
        self.clear_deactivate_time();
        self.clear_server_steam_id();
        self.clear_lobby_id();
        self.clear_league_id();
        self.clear_lobby_type();
        self.clear_game_time();
        self.clear_delay();
        self.clear_spectators();
        self.clear_game_mode();
        self.clear_average_mmr();
        self.clear_team_name_radiant();
        self.clear_team_name_dire();
        self.clear_team_logo_radiant();
        self.clear_team_logo_dire();
        self.clear_sort_score();
        self.clear_last_update_time();
        self.clear_radiant_lead();
        self.clear_radiant_score();
        self.clear_dire_score();
        self.clear_players();
        self.clear_building_state();
        self.clear_weekend_tourney_tournament_id();
        self.clear_weekend_tourney_division();
        self.clear_weekend_tourney_skill_level();
        self.clear_weekend_tourney_bracket_round();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSourceTVGameSmall {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSourceTVGameSmall {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSourceTVGameSmall_Player {
    // message fields
    account_id: ::std::option::Option<u32>,
    hero_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSourceTVGameSmall_Player {}

impl CSourceTVGameSmall_Player {
    pub fn new() -> CSourceTVGameSmall_Player {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSourceTVGameSmall_Player {
        static mut instance: ::protobuf::lazy::Lazy<CSourceTVGameSmall_Player> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSourceTVGameSmall_Player,
        };
        unsafe {
            instance.get(CSourceTVGameSmall_Player::new)
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
}

impl ::protobuf::Message for CSourceTVGameSmall_Player {
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
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.hero_id {
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

impl ::protobuf::MessageStatic for CSourceTVGameSmall_Player {
    fn new() -> CSourceTVGameSmall_Player {
        CSourceTVGameSmall_Player::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSourceTVGameSmall_Player>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CSourceTVGameSmall_Player::get_account_id_for_reflect,
                    CSourceTVGameSmall_Player::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CSourceTVGameSmall_Player::get_hero_id_for_reflect,
                    CSourceTVGameSmall_Player::mut_hero_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSourceTVGameSmall_Player>(
                    "CSourceTVGameSmall_Player",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSourceTVGameSmall_Player {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_hero_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSourceTVGameSmall_Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSourceTVGameSmall_Player {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCFindTopSourceTVGames {
    // message fields
    search_key: ::protobuf::SingularField<::std::string::String>,
    league_id: ::std::option::Option<u32>,
    hero_id: ::std::option::Option<u32>,
    start_game: ::std::option::Option<u32>,
    game_list_index: ::std::option::Option<u32>,
    lobby_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCFindTopSourceTVGames {}

impl CMsgClientToGCFindTopSourceTVGames {
    pub fn new() -> CMsgClientToGCFindTopSourceTVGames {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCFindTopSourceTVGames {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCFindTopSourceTVGames> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCFindTopSourceTVGames,
        };
        unsafe {
            instance.get(CMsgClientToGCFindTopSourceTVGames::new)
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

    // optional uint32 league_id = 2;

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

    // optional uint32 hero_id = 3;

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

    // optional uint32 start_game = 4;

    pub fn clear_start_game(&mut self) {
        self.start_game = ::std::option::Option::None;
    }

    pub fn has_start_game(&self) -> bool {
        self.start_game.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_game(&mut self, v: u32) {
        self.start_game = ::std::option::Option::Some(v);
    }

    pub fn get_start_game(&self) -> u32 {
        self.start_game.unwrap_or(0)
    }

    fn get_start_game_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_game
    }

    fn mut_start_game_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_game
    }

    // optional uint32 game_list_index = 5;

    pub fn clear_game_list_index(&mut self) {
        self.game_list_index = ::std::option::Option::None;
    }

    pub fn has_game_list_index(&self) -> bool {
        self.game_list_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_list_index(&mut self, v: u32) {
        self.game_list_index = ::std::option::Option::Some(v);
    }

    pub fn get_game_list_index(&self) -> u32 {
        self.game_list_index.unwrap_or(0)
    }

    fn get_game_list_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_list_index
    }

    fn mut_game_list_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_list_index
    }

    // repeated uint64 lobby_ids = 6;

    pub fn clear_lobby_ids(&mut self) {
        self.lobby_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_lobby_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.lobby_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lobby_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.lobby_ids
    }

    // Take field
    pub fn take_lobby_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.lobby_ids, ::std::vec::Vec::new())
    }

    pub fn get_lobby_ids(&self) -> &[u64] {
        &self.lobby_ids
    }

    fn get_lobby_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.lobby_ids
    }

    fn mut_lobby_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.lobby_ids
    }
}

impl ::protobuf::Message for CMsgClientToGCFindTopSourceTVGames {
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
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.start_game = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_list_index = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.lobby_ids)?;
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
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.start_game {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_list_index {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.lobby_ids {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.search_key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.league_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.hero_id {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.start_game {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.game_list_index {
            os.write_uint32(5, v)?;
        };
        for v in &self.lobby_ids {
            os.write_uint64(6, *v)?;
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

impl ::protobuf::MessageStatic for CMsgClientToGCFindTopSourceTVGames {
    fn new() -> CMsgClientToGCFindTopSourceTVGames {
        CMsgClientToGCFindTopSourceTVGames::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCFindTopSourceTVGames>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "search_key",
                    CMsgClientToGCFindTopSourceTVGames::get_search_key_for_reflect,
                    CMsgClientToGCFindTopSourceTVGames::mut_search_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgClientToGCFindTopSourceTVGames::get_league_id_for_reflect,
                    CMsgClientToGCFindTopSourceTVGames::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CMsgClientToGCFindTopSourceTVGames::get_hero_id_for_reflect,
                    CMsgClientToGCFindTopSourceTVGames::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "start_game",
                    CMsgClientToGCFindTopSourceTVGames::get_start_game_for_reflect,
                    CMsgClientToGCFindTopSourceTVGames::mut_start_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_list_index",
                    CMsgClientToGCFindTopSourceTVGames::get_game_list_index_for_reflect,
                    CMsgClientToGCFindTopSourceTVGames::mut_game_list_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lobby_ids",
                    CMsgClientToGCFindTopSourceTVGames::get_lobby_ids_for_reflect,
                    CMsgClientToGCFindTopSourceTVGames::mut_lobby_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCFindTopSourceTVGames>(
                    "CMsgClientToGCFindTopSourceTVGames",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCFindTopSourceTVGames {
    fn clear(&mut self) {
        self.clear_search_key();
        self.clear_league_id();
        self.clear_hero_id();
        self.clear_start_game();
        self.clear_game_list_index();
        self.clear_lobby_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCFindTopSourceTVGames {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCFindTopSourceTVGames {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientFindTopSourceTVGamesResponse {
    // message fields
    search_key: ::protobuf::SingularField<::std::string::String>,
    league_id: ::std::option::Option<u32>,
    hero_id: ::std::option::Option<u32>,
    start_game: ::std::option::Option<u32>,
    num_games: ::std::option::Option<u32>,
    game_list_index: ::std::option::Option<u32>,
    game_list: ::protobuf::RepeatedField<CSourceTVGameSmall>,
    specific_games: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientFindTopSourceTVGamesResponse {}

impl CMsgGCToClientFindTopSourceTVGamesResponse {
    pub fn new() -> CMsgGCToClientFindTopSourceTVGamesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientFindTopSourceTVGamesResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientFindTopSourceTVGamesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientFindTopSourceTVGamesResponse,
        };
        unsafe {
            instance.get(CMsgGCToClientFindTopSourceTVGamesResponse::new)
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

    // optional uint32 league_id = 2;

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

    // optional uint32 hero_id = 3;

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

    // optional uint32 start_game = 4;

    pub fn clear_start_game(&mut self) {
        self.start_game = ::std::option::Option::None;
    }

    pub fn has_start_game(&self) -> bool {
        self.start_game.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_game(&mut self, v: u32) {
        self.start_game = ::std::option::Option::Some(v);
    }

    pub fn get_start_game(&self) -> u32 {
        self.start_game.unwrap_or(0)
    }

    fn get_start_game_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_game
    }

    fn mut_start_game_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_game
    }

    // optional uint32 num_games = 5;

    pub fn clear_num_games(&mut self) {
        self.num_games = ::std::option::Option::None;
    }

    pub fn has_num_games(&self) -> bool {
        self.num_games.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_games(&mut self, v: u32) {
        self.num_games = ::std::option::Option::Some(v);
    }

    pub fn get_num_games(&self) -> u32 {
        self.num_games.unwrap_or(0)
    }

    fn get_num_games_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_games
    }

    fn mut_num_games_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_games
    }

    // optional uint32 game_list_index = 6;

    pub fn clear_game_list_index(&mut self) {
        self.game_list_index = ::std::option::Option::None;
    }

    pub fn has_game_list_index(&self) -> bool {
        self.game_list_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_list_index(&mut self, v: u32) {
        self.game_list_index = ::std::option::Option::Some(v);
    }

    pub fn get_game_list_index(&self) -> u32 {
        self.game_list_index.unwrap_or(0)
    }

    fn get_game_list_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_list_index
    }

    fn mut_game_list_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_list_index
    }

    // repeated .dota.CSourceTVGameSmall game_list = 7;

    pub fn clear_game_list(&mut self) {
        self.game_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_game_list(&mut self, v: ::protobuf::RepeatedField<CSourceTVGameSmall>) {
        self.game_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_game_list(&mut self) -> &mut ::protobuf::RepeatedField<CSourceTVGameSmall> {
        &mut self.game_list
    }

    // Take field
    pub fn take_game_list(&mut self) -> ::protobuf::RepeatedField<CSourceTVGameSmall> {
        ::std::mem::replace(&mut self.game_list, ::protobuf::RepeatedField::new())
    }

    pub fn get_game_list(&self) -> &[CSourceTVGameSmall] {
        &self.game_list
    }

    fn get_game_list_for_reflect(&self) -> &::protobuf::RepeatedField<CSourceTVGameSmall> {
        &self.game_list
    }

    fn mut_game_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSourceTVGameSmall> {
        &mut self.game_list
    }

    // optional bool specific_games = 8;

    pub fn clear_specific_games(&mut self) {
        self.specific_games = ::std::option::Option::None;
    }

    pub fn has_specific_games(&self) -> bool {
        self.specific_games.is_some()
    }

    // Param is passed by value, moved
    pub fn set_specific_games(&mut self, v: bool) {
        self.specific_games = ::std::option::Option::Some(v);
    }

    pub fn get_specific_games(&self) -> bool {
        self.specific_games.unwrap_or(false)
    }

    fn get_specific_games_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.specific_games
    }

    fn mut_specific_games_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.specific_games
    }
}

impl ::protobuf::Message for CMsgGCToClientFindTopSourceTVGamesResponse {
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
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.start_game = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.num_games = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.game_list_index = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.game_list)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.specific_games = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.start_game {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.num_games {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_list_index {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.game_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.specific_games {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.search_key.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.league_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.hero_id {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.start_game {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.num_games {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.game_list_index {
            os.write_uint32(6, v)?;
        };
        for v in &self.game_list {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.specific_games {
            os.write_bool(8, v)?;
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

impl ::protobuf::MessageStatic for CMsgGCToClientFindTopSourceTVGamesResponse {
    fn new() -> CMsgGCToClientFindTopSourceTVGamesResponse {
        CMsgGCToClientFindTopSourceTVGamesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientFindTopSourceTVGamesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "search_key",
                    CMsgGCToClientFindTopSourceTVGamesResponse::get_search_key_for_reflect,
                    CMsgGCToClientFindTopSourceTVGamesResponse::mut_search_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgGCToClientFindTopSourceTVGamesResponse::get_league_id_for_reflect,
                    CMsgGCToClientFindTopSourceTVGamesResponse::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CMsgGCToClientFindTopSourceTVGamesResponse::get_hero_id_for_reflect,
                    CMsgGCToClientFindTopSourceTVGamesResponse::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "start_game",
                    CMsgGCToClientFindTopSourceTVGamesResponse::get_start_game_for_reflect,
                    CMsgGCToClientFindTopSourceTVGamesResponse::mut_start_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_games",
                    CMsgGCToClientFindTopSourceTVGamesResponse::get_num_games_for_reflect,
                    CMsgGCToClientFindTopSourceTVGamesResponse::mut_num_games_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_list_index",
                    CMsgGCToClientFindTopSourceTVGamesResponse::get_game_list_index_for_reflect,
                    CMsgGCToClientFindTopSourceTVGamesResponse::mut_game_list_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSourceTVGameSmall>>(
                    "game_list",
                    CMsgGCToClientFindTopSourceTVGamesResponse::get_game_list_for_reflect,
                    CMsgGCToClientFindTopSourceTVGamesResponse::mut_game_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "specific_games",
                    CMsgGCToClientFindTopSourceTVGamesResponse::get_specific_games_for_reflect,
                    CMsgGCToClientFindTopSourceTVGamesResponse::mut_specific_games_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientFindTopSourceTVGamesResponse>(
                    "CMsgGCToClientFindTopSourceTVGamesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientFindTopSourceTVGamesResponse {
    fn clear(&mut self) {
        self.clear_search_key();
        self.clear_league_id();
        self.clear_hero_id();
        self.clear_start_game();
        self.clear_num_games();
        self.clear_game_list_index();
        self.clear_game_list();
        self.clear_specific_games();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientFindTopSourceTVGamesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientFindTopSourceTVGamesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientTopWeekendTourneyGames {
    // message fields
    live_games: ::protobuf::RepeatedField<CSourceTVGameSmall>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientTopWeekendTourneyGames {}

impl CMsgGCToClientTopWeekendTourneyGames {
    pub fn new() -> CMsgGCToClientTopWeekendTourneyGames {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientTopWeekendTourneyGames {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientTopWeekendTourneyGames> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientTopWeekendTourneyGames,
        };
        unsafe {
            instance.get(CMsgGCToClientTopWeekendTourneyGames::new)
        }
    }

    // repeated .dota.CSourceTVGameSmall live_games = 1;

    pub fn clear_live_games(&mut self) {
        self.live_games.clear();
    }

    // Param is passed by value, moved
    pub fn set_live_games(&mut self, v: ::protobuf::RepeatedField<CSourceTVGameSmall>) {
        self.live_games = v;
    }

    // Mutable pointer to the field.
    pub fn mut_live_games(&mut self) -> &mut ::protobuf::RepeatedField<CSourceTVGameSmall> {
        &mut self.live_games
    }

    // Take field
    pub fn take_live_games(&mut self) -> ::protobuf::RepeatedField<CSourceTVGameSmall> {
        ::std::mem::replace(&mut self.live_games, ::protobuf::RepeatedField::new())
    }

    pub fn get_live_games(&self) -> &[CSourceTVGameSmall] {
        &self.live_games
    }

    fn get_live_games_for_reflect(&self) -> &::protobuf::RepeatedField<CSourceTVGameSmall> {
        &self.live_games
    }

    fn mut_live_games_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSourceTVGameSmall> {
        &mut self.live_games
    }
}

impl ::protobuf::Message for CMsgGCToClientTopWeekendTourneyGames {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.live_games)?;
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
        for value in &self.live_games {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.live_games {
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

impl ::protobuf::MessageStatic for CMsgGCToClientTopWeekendTourneyGames {
    fn new() -> CMsgGCToClientTopWeekendTourneyGames {
        CMsgGCToClientTopWeekendTourneyGames::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientTopWeekendTourneyGames>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSourceTVGameSmall>>(
                    "live_games",
                    CMsgGCToClientTopWeekendTourneyGames::get_live_games_for_reflect,
                    CMsgGCToClientTopWeekendTourneyGames::mut_live_games_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientTopWeekendTourneyGames>(
                    "CMsgGCToClientTopWeekendTourneyGames",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientTopWeekendTourneyGames {
    fn clear(&mut self) {
        self.clear_live_games();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientTopWeekendTourneyGames {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientTopWeekendTourneyGames {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCTopMatchesRequest {
    // message fields
    hero_id: ::std::option::Option<u32>,
    player_account_id: ::std::option::Option<u32>,
    team_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCTopMatchesRequest {}

impl CMsgClientToGCTopMatchesRequest {
    pub fn new() -> CMsgClientToGCTopMatchesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCTopMatchesRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCTopMatchesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCTopMatchesRequest,
        };
        unsafe {
            instance.get(CMsgClientToGCTopMatchesRequest::new)
        }
    }

    // optional uint32 hero_id = 1;

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

    // optional uint32 player_account_id = 2;

    pub fn clear_player_account_id(&mut self) {
        self.player_account_id = ::std::option::Option::None;
    }

    pub fn has_player_account_id(&self) -> bool {
        self.player_account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_account_id(&mut self, v: u32) {
        self.player_account_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_account_id(&self) -> u32 {
        self.player_account_id.unwrap_or(0)
    }

    fn get_player_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_account_id
    }

    fn mut_player_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_account_id
    }

    // optional uint32 team_id = 3;

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

impl ::protobuf::Message for CMsgClientToGCTopMatchesRequest {
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
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.player_account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.player_account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.team_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hero_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.player_account_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.team_id {
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

impl ::protobuf::MessageStatic for CMsgClientToGCTopMatchesRequest {
    fn new() -> CMsgClientToGCTopMatchesRequest {
        CMsgClientToGCTopMatchesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCTopMatchesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CMsgClientToGCTopMatchesRequest::get_hero_id_for_reflect,
                    CMsgClientToGCTopMatchesRequest::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_account_id",
                    CMsgClientToGCTopMatchesRequest::get_player_account_id_for_reflect,
                    CMsgClientToGCTopMatchesRequest::mut_player_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team_id",
                    CMsgClientToGCTopMatchesRequest::get_team_id_for_reflect,
                    CMsgClientToGCTopMatchesRequest::mut_team_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCTopMatchesRequest>(
                    "CMsgClientToGCTopMatchesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCTopMatchesRequest {
    fn clear(&mut self) {
        self.clear_hero_id();
        self.clear_player_account_id();
        self.clear_team_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCTopMatchesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCTopMatchesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCTopLeagueMatchesRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCTopLeagueMatchesRequest {}

impl CMsgClientToGCTopLeagueMatchesRequest {
    pub fn new() -> CMsgClientToGCTopLeagueMatchesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCTopLeagueMatchesRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCTopLeagueMatchesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCTopLeagueMatchesRequest,
        };
        unsafe {
            instance.get(CMsgClientToGCTopLeagueMatchesRequest::new)
        }
    }
}

impl ::protobuf::Message for CMsgClientToGCTopLeagueMatchesRequest {
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

impl ::protobuf::MessageStatic for CMsgClientToGCTopLeagueMatchesRequest {
    fn new() -> CMsgClientToGCTopLeagueMatchesRequest {
        CMsgClientToGCTopLeagueMatchesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCTopLeagueMatchesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCTopLeagueMatchesRequest>(
                    "CMsgClientToGCTopLeagueMatchesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCTopLeagueMatchesRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCTopLeagueMatchesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCTopLeagueMatchesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCTopFriendMatchesRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCTopFriendMatchesRequest {}

impl CMsgClientToGCTopFriendMatchesRequest {
    pub fn new() -> CMsgClientToGCTopFriendMatchesRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCTopFriendMatchesRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCTopFriendMatchesRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCTopFriendMatchesRequest,
        };
        unsafe {
            instance.get(CMsgClientToGCTopFriendMatchesRequest::new)
        }
    }
}

impl ::protobuf::Message for CMsgClientToGCTopFriendMatchesRequest {
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

impl ::protobuf::MessageStatic for CMsgClientToGCTopFriendMatchesRequest {
    fn new() -> CMsgClientToGCTopFriendMatchesRequest {
        CMsgClientToGCTopFriendMatchesRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCTopFriendMatchesRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCTopFriendMatchesRequest>(
                    "CMsgClientToGCTopFriendMatchesRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCTopFriendMatchesRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCTopFriendMatchesRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCTopFriendMatchesRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCMatchesMinimalRequest {
    // message fields
    match_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCMatchesMinimalRequest {}

impl CMsgClientToGCMatchesMinimalRequest {
    pub fn new() -> CMsgClientToGCMatchesMinimalRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCMatchesMinimalRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCMatchesMinimalRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCMatchesMinimalRequest,
        };
        unsafe {
            instance.get(CMsgClientToGCMatchesMinimalRequest::new)
        }
    }

    // repeated uint64 match_ids = 1;

    pub fn clear_match_ids(&mut self) {
        self.match_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_match_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.match_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_match_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.match_ids
    }

    // Take field
    pub fn take_match_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.match_ids, ::std::vec::Vec::new())
    }

    pub fn get_match_ids(&self) -> &[u64] {
        &self.match_ids
    }

    fn get_match_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.match_ids
    }

    fn mut_match_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.match_ids
    }
}

impl ::protobuf::Message for CMsgClientToGCMatchesMinimalRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.match_ids)?;
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
        for value in &self.match_ids {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.match_ids {
            os.write_uint64(1, *v)?;
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

impl ::protobuf::MessageStatic for CMsgClientToGCMatchesMinimalRequest {
    fn new() -> CMsgClientToGCMatchesMinimalRequest {
        CMsgClientToGCMatchesMinimalRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCMatchesMinimalRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_ids",
                    CMsgClientToGCMatchesMinimalRequest::get_match_ids_for_reflect,
                    CMsgClientToGCMatchesMinimalRequest::mut_match_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCMatchesMinimalRequest>(
                    "CMsgClientToGCMatchesMinimalRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCMatchesMinimalRequest {
    fn clear(&mut self) {
        self.clear_match_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCMatchesMinimalRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCMatchesMinimalRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCMatchesMinimalResponse {
    // message fields
    matches: ::protobuf::RepeatedField<CMsgDOTAMatchMinimal>,
    last_match: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCMatchesMinimalResponse {}

impl CMsgClientToGCMatchesMinimalResponse {
    pub fn new() -> CMsgClientToGCMatchesMinimalResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCMatchesMinimalResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCMatchesMinimalResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCMatchesMinimalResponse,
        };
        unsafe {
            instance.get(CMsgClientToGCMatchesMinimalResponse::new)
        }
    }

    // repeated .dota.CMsgDOTAMatchMinimal matches = 1;

    pub fn clear_matches(&mut self) {
        self.matches.clear();
    }

    // Param is passed by value, moved
    pub fn set_matches(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAMatchMinimal>) {
        self.matches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_matches(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &mut self.matches
    }

    // Take field
    pub fn take_matches(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        ::std::mem::replace(&mut self.matches, ::protobuf::RepeatedField::new())
    }

    pub fn get_matches(&self) -> &[CMsgDOTAMatchMinimal] {
        &self.matches
    }

    fn get_matches_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &self.matches
    }

    fn mut_matches_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &mut self.matches
    }

    // optional bool last_match = 2;

    pub fn clear_last_match(&mut self) {
        self.last_match = ::std::option::Option::None;
    }

    pub fn has_last_match(&self) -> bool {
        self.last_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_last_match(&mut self, v: bool) {
        self.last_match = ::std::option::Option::Some(v);
    }

    pub fn get_last_match(&self) -> bool {
        self.last_match.unwrap_or(false)
    }

    fn get_last_match_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.last_match
    }

    fn mut_last_match_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.last_match
    }
}

impl ::protobuf::Message for CMsgClientToGCMatchesMinimalResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.matches)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.last_match = ::std::option::Option::Some(tmp);
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
        for value in &self.matches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.last_match {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.matches {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.last_match {
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

impl ::protobuf::MessageStatic for CMsgClientToGCMatchesMinimalResponse {
    fn new() -> CMsgClientToGCMatchesMinimalResponse {
        CMsgClientToGCMatchesMinimalResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCMatchesMinimalResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAMatchMinimal>>(
                    "matches",
                    CMsgClientToGCMatchesMinimalResponse::get_matches_for_reflect,
                    CMsgClientToGCMatchesMinimalResponse::mut_matches_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "last_match",
                    CMsgClientToGCMatchesMinimalResponse::get_last_match_for_reflect,
                    CMsgClientToGCMatchesMinimalResponse::mut_last_match_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCMatchesMinimalResponse>(
                    "CMsgClientToGCMatchesMinimalResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCMatchesMinimalResponse {
    fn clear(&mut self) {
        self.clear_matches();
        self.clear_last_match();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCMatchesMinimalResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCMatchesMinimalResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientTopLeagueMatchesResponse {
    // message fields
    matches: ::protobuf::RepeatedField<CMsgDOTAMatchMinimal>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientTopLeagueMatchesResponse {}

impl CMsgGCToClientTopLeagueMatchesResponse {
    pub fn new() -> CMsgGCToClientTopLeagueMatchesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientTopLeagueMatchesResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientTopLeagueMatchesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientTopLeagueMatchesResponse,
        };
        unsafe {
            instance.get(CMsgGCToClientTopLeagueMatchesResponse::new)
        }
    }

    // repeated .dota.CMsgDOTAMatchMinimal matches = 2;

    pub fn clear_matches(&mut self) {
        self.matches.clear();
    }

    // Param is passed by value, moved
    pub fn set_matches(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAMatchMinimal>) {
        self.matches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_matches(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &mut self.matches
    }

    // Take field
    pub fn take_matches(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        ::std::mem::replace(&mut self.matches, ::protobuf::RepeatedField::new())
    }

    pub fn get_matches(&self) -> &[CMsgDOTAMatchMinimal] {
        &self.matches
    }

    fn get_matches_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &self.matches
    }

    fn mut_matches_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &mut self.matches
    }
}

impl ::protobuf::Message for CMsgGCToClientTopLeagueMatchesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.matches)?;
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
        for value in &self.matches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.matches {
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

impl ::protobuf::MessageStatic for CMsgGCToClientTopLeagueMatchesResponse {
    fn new() -> CMsgGCToClientTopLeagueMatchesResponse {
        CMsgGCToClientTopLeagueMatchesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientTopLeagueMatchesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAMatchMinimal>>(
                    "matches",
                    CMsgGCToClientTopLeagueMatchesResponse::get_matches_for_reflect,
                    CMsgGCToClientTopLeagueMatchesResponse::mut_matches_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientTopLeagueMatchesResponse>(
                    "CMsgGCToClientTopLeagueMatchesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientTopLeagueMatchesResponse {
    fn clear(&mut self) {
        self.clear_matches();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientTopLeagueMatchesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientTopLeagueMatchesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientTopFriendMatchesResponse {
    // message fields
    matches: ::protobuf::RepeatedField<CMsgDOTAMatchMinimal>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientTopFriendMatchesResponse {}

impl CMsgGCToClientTopFriendMatchesResponse {
    pub fn new() -> CMsgGCToClientTopFriendMatchesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientTopFriendMatchesResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientTopFriendMatchesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientTopFriendMatchesResponse,
        };
        unsafe {
            instance.get(CMsgGCToClientTopFriendMatchesResponse::new)
        }
    }

    // repeated .dota.CMsgDOTAMatchMinimal matches = 1;

    pub fn clear_matches(&mut self) {
        self.matches.clear();
    }

    // Param is passed by value, moved
    pub fn set_matches(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAMatchMinimal>) {
        self.matches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_matches(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &mut self.matches
    }

    // Take field
    pub fn take_matches(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        ::std::mem::replace(&mut self.matches, ::protobuf::RepeatedField::new())
    }

    pub fn get_matches(&self) -> &[CMsgDOTAMatchMinimal] {
        &self.matches
    }

    fn get_matches_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &self.matches
    }

    fn mut_matches_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAMatchMinimal> {
        &mut self.matches
    }
}

impl ::protobuf::Message for CMsgGCToClientTopFriendMatchesResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.matches)?;
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
        for value in &self.matches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.matches {
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

impl ::protobuf::MessageStatic for CMsgGCToClientTopFriendMatchesResponse {
    fn new() -> CMsgGCToClientTopFriendMatchesResponse {
        CMsgGCToClientTopFriendMatchesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientTopFriendMatchesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAMatchMinimal>>(
                    "matches",
                    CMsgGCToClientTopFriendMatchesResponse::get_matches_for_reflect,
                    CMsgGCToClientTopFriendMatchesResponse::mut_matches_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientTopFriendMatchesResponse>(
                    "CMsgGCToClientTopFriendMatchesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientTopFriendMatchesResponse {
    fn clear(&mut self) {
        self.clear_matches();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientTopFriendMatchesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientTopFriendMatchesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgClientToGCFindTopMatches {
    // message fields
    start_game: ::std::option::Option<u32>,
    league_id: ::std::option::Option<u32>,
    hero_id: ::std::option::Option<u32>,
    friend_id: ::std::option::Option<u32>,
    friend_list: ::std::option::Option<bool>,
    league_list: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgClientToGCFindTopMatches {}

impl CMsgClientToGCFindTopMatches {
    pub fn new() -> CMsgClientToGCFindTopMatches {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgClientToGCFindTopMatches {
        static mut instance: ::protobuf::lazy::Lazy<CMsgClientToGCFindTopMatches> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgClientToGCFindTopMatches,
        };
        unsafe {
            instance.get(CMsgClientToGCFindTopMatches::new)
        }
    }

    // optional uint32 start_game = 1;

    pub fn clear_start_game(&mut self) {
        self.start_game = ::std::option::Option::None;
    }

    pub fn has_start_game(&self) -> bool {
        self.start_game.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_game(&mut self, v: u32) {
        self.start_game = ::std::option::Option::Some(v);
    }

    pub fn get_start_game(&self) -> u32 {
        self.start_game.unwrap_or(0)
    }

    fn get_start_game_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_game
    }

    fn mut_start_game_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_game
    }

    // optional uint32 league_id = 2;

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

    // optional uint32 hero_id = 3;

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

    // optional uint32 friend_id = 4;

    pub fn clear_friend_id(&mut self) {
        self.friend_id = ::std::option::Option::None;
    }

    pub fn has_friend_id(&self) -> bool {
        self.friend_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friend_id(&mut self, v: u32) {
        self.friend_id = ::std::option::Option::Some(v);
    }

    pub fn get_friend_id(&self) -> u32 {
        self.friend_id.unwrap_or(0)
    }

    fn get_friend_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.friend_id
    }

    fn mut_friend_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.friend_id
    }

    // optional bool friend_list = 5;

    pub fn clear_friend_list(&mut self) {
        self.friend_list = ::std::option::Option::None;
    }

    pub fn has_friend_list(&self) -> bool {
        self.friend_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friend_list(&mut self, v: bool) {
        self.friend_list = ::std::option::Option::Some(v);
    }

    pub fn get_friend_list(&self) -> bool {
        self.friend_list.unwrap_or(false)
    }

    fn get_friend_list_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.friend_list
    }

    fn mut_friend_list_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.friend_list
    }

    // optional bool league_list = 6;

    pub fn clear_league_list(&mut self) {
        self.league_list = ::std::option::Option::None;
    }

    pub fn has_league_list(&self) -> bool {
        self.league_list.is_some()
    }

    // Param is passed by value, moved
    pub fn set_league_list(&mut self, v: bool) {
        self.league_list = ::std::option::Option::Some(v);
    }

    pub fn get_league_list(&self) -> bool {
        self.league_list.unwrap_or(false)
    }

    fn get_league_list_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.league_list
    }

    fn mut_league_list_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.league_list
    }
}

impl ::protobuf::Message for CMsgClientToGCFindTopMatches {
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
                    self.start_game = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.friend_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.friend_list = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.league_list = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.start_game {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.friend_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.friend_list {
            my_size += 2;
        };
        if let Some(v) = self.league_list {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_game {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.league_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.hero_id {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.friend_id {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.friend_list {
            os.write_bool(5, v)?;
        };
        if let Some(v) = self.league_list {
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

impl ::protobuf::MessageStatic for CMsgClientToGCFindTopMatches {
    fn new() -> CMsgClientToGCFindTopMatches {
        CMsgClientToGCFindTopMatches::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgClientToGCFindTopMatches>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "start_game",
                    CMsgClientToGCFindTopMatches::get_start_game_for_reflect,
                    CMsgClientToGCFindTopMatches::mut_start_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgClientToGCFindTopMatches::get_league_id_for_reflect,
                    CMsgClientToGCFindTopMatches::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CMsgClientToGCFindTopMatches::get_hero_id_for_reflect,
                    CMsgClientToGCFindTopMatches::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "friend_id",
                    CMsgClientToGCFindTopMatches::get_friend_id_for_reflect,
                    CMsgClientToGCFindTopMatches::mut_friend_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "friend_list",
                    CMsgClientToGCFindTopMatches::get_friend_list_for_reflect,
                    CMsgClientToGCFindTopMatches::mut_friend_list_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "league_list",
                    CMsgClientToGCFindTopMatches::get_league_list_for_reflect,
                    CMsgClientToGCFindTopMatches::mut_league_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgClientToGCFindTopMatches>(
                    "CMsgClientToGCFindTopMatches",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgClientToGCFindTopMatches {
    fn clear(&mut self) {
        self.clear_start_game();
        self.clear_league_id();
        self.clear_hero_id();
        self.clear_friend_id();
        self.clear_friend_list();
        self.clear_league_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgClientToGCFindTopMatches {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgClientToGCFindTopMatches {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCToClientFindTopLeagueMatchesResponse {
    // message fields
    start_game: ::std::option::Option<u32>,
    league_id: ::std::option::Option<u32>,
    hero_id: ::std::option::Option<u32>,
    match_ids: ::std::vec::Vec<u32>,
    matches: ::protobuf::RepeatedField<super::dota_gcmessages_common::CMsgDOTAMatch>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCToClientFindTopLeagueMatchesResponse {}

impl CMsgGCToClientFindTopLeagueMatchesResponse {
    pub fn new() -> CMsgGCToClientFindTopLeagueMatchesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCToClientFindTopLeagueMatchesResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCToClientFindTopLeagueMatchesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCToClientFindTopLeagueMatchesResponse,
        };
        unsafe {
            instance.get(CMsgGCToClientFindTopLeagueMatchesResponse::new)
        }
    }

    // optional uint32 start_game = 1;

    pub fn clear_start_game(&mut self) {
        self.start_game = ::std::option::Option::None;
    }

    pub fn has_start_game(&self) -> bool {
        self.start_game.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_game(&mut self, v: u32) {
        self.start_game = ::std::option::Option::Some(v);
    }

    pub fn get_start_game(&self) -> u32 {
        self.start_game.unwrap_or(0)
    }

    fn get_start_game_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_game
    }

    fn mut_start_game_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_game
    }

    // optional uint32 league_id = 2;

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

    // optional uint32 hero_id = 3;

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

    // repeated uint32 match_ids = 4;

    pub fn clear_match_ids(&mut self) {
        self.match_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_match_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.match_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_match_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.match_ids
    }

    // Take field
    pub fn take_match_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.match_ids, ::std::vec::Vec::new())
    }

    pub fn get_match_ids(&self) -> &[u32] {
        &self.match_ids
    }

    fn get_match_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.match_ids
    }

    fn mut_match_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.match_ids
    }

    // repeated .dota.CMsgDOTAMatch matches = 5;

    pub fn clear_matches(&mut self) {
        self.matches.clear();
    }

    // Param is passed by value, moved
    pub fn set_matches(&mut self, v: ::protobuf::RepeatedField<super::dota_gcmessages_common::CMsgDOTAMatch>) {
        self.matches = v;
    }

    // Mutable pointer to the field.
    pub fn mut_matches(&mut self) -> &mut ::protobuf::RepeatedField<super::dota_gcmessages_common::CMsgDOTAMatch> {
        &mut self.matches
    }

    // Take field
    pub fn take_matches(&mut self) -> ::protobuf::RepeatedField<super::dota_gcmessages_common::CMsgDOTAMatch> {
        ::std::mem::replace(&mut self.matches, ::protobuf::RepeatedField::new())
    }

    pub fn get_matches(&self) -> &[super::dota_gcmessages_common::CMsgDOTAMatch] {
        &self.matches
    }

    fn get_matches_for_reflect(&self) -> &::protobuf::RepeatedField<super::dota_gcmessages_common::CMsgDOTAMatch> {
        &self.matches
    }

    fn mut_matches_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::dota_gcmessages_common::CMsgDOTAMatch> {
        &mut self.matches
    }
}

impl ::protobuf::Message for CMsgGCToClientFindTopLeagueMatchesResponse {
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
                    self.start_game = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.match_ids)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.matches)?;
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
        if let Some(v) = self.start_game {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.match_ids {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.matches {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_game {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.league_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.hero_id {
            os.write_uint32(3, v)?;
        };
        for v in &self.match_ids {
            os.write_uint32(4, *v)?;
        };
        for v in &self.matches {
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

impl ::protobuf::MessageStatic for CMsgGCToClientFindTopLeagueMatchesResponse {
    fn new() -> CMsgGCToClientFindTopLeagueMatchesResponse {
        CMsgGCToClientFindTopLeagueMatchesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCToClientFindTopLeagueMatchesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "start_game",
                    CMsgGCToClientFindTopLeagueMatchesResponse::get_start_game_for_reflect,
                    CMsgGCToClientFindTopLeagueMatchesResponse::mut_start_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgGCToClientFindTopLeagueMatchesResponse::get_league_id_for_reflect,
                    CMsgGCToClientFindTopLeagueMatchesResponse::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CMsgGCToClientFindTopLeagueMatchesResponse::get_hero_id_for_reflect,
                    CMsgGCToClientFindTopLeagueMatchesResponse::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "match_ids",
                    CMsgGCToClientFindTopLeagueMatchesResponse::get_match_ids_for_reflect,
                    CMsgGCToClientFindTopLeagueMatchesResponse::mut_match_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_gcmessages_common::CMsgDOTAMatch>>(
                    "matches",
                    CMsgGCToClientFindTopLeagueMatchesResponse::get_matches_for_reflect,
                    CMsgGCToClientFindTopLeagueMatchesResponse::mut_matches_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCToClientFindTopLeagueMatchesResponse>(
                    "CMsgGCToClientFindTopLeagueMatchesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCToClientFindTopLeagueMatchesResponse {
    fn clear(&mut self) {
        self.clear_start_game();
        self.clear_league_id();
        self.clear_hero_id();
        self.clear_match_ids();
        self.clear_matches();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCToClientFindTopLeagueMatchesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCToClientFindTopLeagueMatchesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSpectateFriendGame {
    // message fields
    steam_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSpectateFriendGame {}

impl CMsgSpectateFriendGame {
    pub fn new() -> CMsgSpectateFriendGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSpectateFriendGame {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSpectateFriendGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSpectateFriendGame,
        };
        unsafe {
            instance.get(CMsgSpectateFriendGame::new)
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
}

impl ::protobuf::Message for CMsgSpectateFriendGame {
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
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
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

impl ::protobuf::MessageStatic for CMsgSpectateFriendGame {
    fn new() -> CMsgSpectateFriendGame {
        CMsgSpectateFriendGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSpectateFriendGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgSpectateFriendGame::get_steam_id_for_reflect,
                    CMsgSpectateFriendGame::mut_steam_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSpectateFriendGame>(
                    "CMsgSpectateFriendGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSpectateFriendGame {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSpectateFriendGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSpectateFriendGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgSpectateFriendGameResponse {
    // message fields
    server_steamid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgSpectateFriendGameResponse {}

impl CMsgSpectateFriendGameResponse {
    pub fn new() -> CMsgSpectateFriendGameResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgSpectateFriendGameResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgSpectateFriendGameResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgSpectateFriendGameResponse,
        };
        unsafe {
            instance.get(CMsgSpectateFriendGameResponse::new)
        }
    }

    // optional fixed64 server_steamid = 4;

    pub fn clear_server_steamid(&mut self) {
        self.server_steamid = ::std::option::Option::None;
    }

    pub fn has_server_steamid(&self) -> bool {
        self.server_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_steamid(&mut self, v: u64) {
        self.server_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_server_steamid(&self) -> u64 {
        self.server_steamid.unwrap_or(0)
    }

    fn get_server_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.server_steamid
    }

    fn mut_server_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.server_steamid
    }
}

impl ::protobuf::Message for CMsgSpectateFriendGameResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.server_steamid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.server_steamid {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_steamid {
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

impl ::protobuf::MessageStatic for CMsgSpectateFriendGameResponse {
    fn new() -> CMsgSpectateFriendGameResponse {
        CMsgSpectateFriendGameResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgSpectateFriendGameResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "server_steamid",
                    CMsgSpectateFriendGameResponse::get_server_steamid_for_reflect,
                    CMsgSpectateFriendGameResponse::mut_server_steamid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgSpectateFriendGameResponse>(
                    "CMsgSpectateFriendGameResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgSpectateFriendGameResponse {
    fn clear(&mut self) {
        self.clear_server_steamid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgSpectateFriendGameResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgSpectateFriendGameResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAMatchMinimal {
    // message fields
    match_id: ::std::option::Option<u64>,
    start_time: ::std::option::Option<u32>,
    duration: ::std::option::Option<u32>,
    game_mode: ::std::option::Option<super::dota_shared_enums::DOTA_GameMode>,
    players: ::protobuf::RepeatedField<CMsgDOTAMatchMinimal_Player>,
    tourney: ::protobuf::SingularPtrField<CMsgDOTAMatchMinimal_Tourney>,
    match_outcome: ::std::option::Option<super::dota_shared_enums::EMatchOutcome>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAMatchMinimal {}

impl CMsgDOTAMatchMinimal {
    pub fn new() -> CMsgDOTAMatchMinimal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAMatchMinimal {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAMatchMinimal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAMatchMinimal,
        };
        unsafe {
            instance.get(CMsgDOTAMatchMinimal::new)
        }
    }

    // optional uint64 match_id = 1;

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

    // optional fixed32 start_time = 2;

    pub fn clear_start_time(&mut self) {
        self.start_time = ::std::option::Option::None;
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: u32) {
        self.start_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_time(&self) -> u32 {
        self.start_time.unwrap_or(0)
    }

    fn get_start_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.start_time
    }

    fn mut_start_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.start_time
    }

    // optional uint32 duration = 3;

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

    // repeated .dota.CMsgDOTAMatchMinimal.Player players = 6;

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: ::protobuf::RepeatedField<CMsgDOTAMatchMinimal_Player>) {
        self.players = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAMatchMinimal_Player> {
        &mut self.players
    }

    // Take field
    pub fn take_players(&mut self) -> ::protobuf::RepeatedField<CMsgDOTAMatchMinimal_Player> {
        ::std::mem::replace(&mut self.players, ::protobuf::RepeatedField::new())
    }

    pub fn get_players(&self) -> &[CMsgDOTAMatchMinimal_Player] {
        &self.players
    }

    fn get_players_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgDOTAMatchMinimal_Player> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgDOTAMatchMinimal_Player> {
        &mut self.players
    }

    // optional .dota.CMsgDOTAMatchMinimal.Tourney tourney = 7;

    pub fn clear_tourney(&mut self) {
        self.tourney.clear();
    }

    pub fn has_tourney(&self) -> bool {
        self.tourney.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tourney(&mut self, v: CMsgDOTAMatchMinimal_Tourney) {
        self.tourney = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tourney(&mut self) -> &mut CMsgDOTAMatchMinimal_Tourney {
        if self.tourney.is_none() {
            self.tourney.set_default();
        };
        self.tourney.as_mut().unwrap()
    }

    // Take field
    pub fn take_tourney(&mut self) -> CMsgDOTAMatchMinimal_Tourney {
        self.tourney.take().unwrap_or_else(|| CMsgDOTAMatchMinimal_Tourney::new())
    }

    pub fn get_tourney(&self) -> &CMsgDOTAMatchMinimal_Tourney {
        self.tourney.as_ref().unwrap_or_else(|| CMsgDOTAMatchMinimal_Tourney::default_instance())
    }

    fn get_tourney_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgDOTAMatchMinimal_Tourney> {
        &self.tourney
    }

    fn mut_tourney_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgDOTAMatchMinimal_Tourney> {
        &mut self.tourney
    }

    // optional .dota.EMatchOutcome match_outcome = 8;

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
}

impl ::protobuf::Message for CMsgDOTAMatchMinimal {
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
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed32()?;
                    self.start_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.game_mode = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.players)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tourney)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.match_outcome = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.start_time {
            my_size += 5;
        };
        if let Some(v) = self.duration {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_mode {
            my_size += ::protobuf::rt::enum_size(4, v);
        };
        for value in &self.players {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.tourney.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.match_outcome {
            my_size += ::protobuf::rt::enum_size(8, v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.match_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.start_time {
            os.write_fixed32(2, v)?;
        };
        if let Some(v) = self.duration {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.game_mode {
            os.write_enum(4, v.value())?;
        };
        for v in &self.players {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.tourney.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.match_outcome {
            os.write_enum(8, v.value())?;
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

impl ::protobuf::MessageStatic for CMsgDOTAMatchMinimal {
    fn new() -> CMsgDOTAMatchMinimal {
        CMsgDOTAMatchMinimal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAMatchMinimal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CMsgDOTAMatchMinimal::get_match_id_for_reflect,
                    CMsgDOTAMatchMinimal::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "start_time",
                    CMsgDOTAMatchMinimal::get_start_time_for_reflect,
                    CMsgDOTAMatchMinimal::mut_start_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "duration",
                    CMsgDOTAMatchMinimal::get_duration_for_reflect,
                    CMsgDOTAMatchMinimal::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::DOTA_GameMode>>(
                    "game_mode",
                    CMsgDOTAMatchMinimal::get_game_mode_for_reflect,
                    CMsgDOTAMatchMinimal::mut_game_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAMatchMinimal_Player>>(
                    "players",
                    CMsgDOTAMatchMinimal::get_players_for_reflect,
                    CMsgDOTAMatchMinimal::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAMatchMinimal_Tourney>>(
                    "tourney",
                    CMsgDOTAMatchMinimal::get_tourney_for_reflect,
                    CMsgDOTAMatchMinimal::mut_tourney_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::dota_shared_enums::EMatchOutcome>>(
                    "match_outcome",
                    CMsgDOTAMatchMinimal::get_match_outcome_for_reflect,
                    CMsgDOTAMatchMinimal::mut_match_outcome_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAMatchMinimal>(
                    "CMsgDOTAMatchMinimal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAMatchMinimal {
    fn clear(&mut self) {
        self.clear_match_id();
        self.clear_start_time();
        self.clear_duration();
        self.clear_game_mode();
        self.clear_players();
        self.clear_tourney();
        self.clear_match_outcome();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAMatchMinimal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAMatchMinimal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAMatchMinimal_Player {
    // message fields
    account_id: ::std::option::Option<u32>,
    hero_id: ::std::option::Option<u32>,
    kills: ::std::option::Option<u32>,
    deaths: ::std::option::Option<u32>,
    assists: ::std::option::Option<u32>,
    items: ::std::vec::Vec<u32>,
    player_slot: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAMatchMinimal_Player {}

impl CMsgDOTAMatchMinimal_Player {
    pub fn new() -> CMsgDOTAMatchMinimal_Player {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAMatchMinimal_Player {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAMatchMinimal_Player> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAMatchMinimal_Player,
        };
        unsafe {
            instance.get(CMsgDOTAMatchMinimal_Player::new)
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

    // optional uint32 kills = 3;

    pub fn clear_kills(&mut self) {
        self.kills = ::std::option::Option::None;
    }

    pub fn has_kills(&self) -> bool {
        self.kills.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kills(&mut self, v: u32) {
        self.kills = ::std::option::Option::Some(v);
    }

    pub fn get_kills(&self) -> u32 {
        self.kills.unwrap_or(0)
    }

    fn get_kills_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.kills
    }

    fn mut_kills_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.kills
    }

    // optional uint32 deaths = 4;

    pub fn clear_deaths(&mut self) {
        self.deaths = ::std::option::Option::None;
    }

    pub fn has_deaths(&self) -> bool {
        self.deaths.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deaths(&mut self, v: u32) {
        self.deaths = ::std::option::Option::Some(v);
    }

    pub fn get_deaths(&self) -> u32 {
        self.deaths.unwrap_or(0)
    }

    fn get_deaths_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.deaths
    }

    fn mut_deaths_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.deaths
    }

    // optional uint32 assists = 5;

    pub fn clear_assists(&mut self) {
        self.assists = ::std::option::Option::None;
    }

    pub fn has_assists(&self) -> bool {
        self.assists.is_some()
    }

    // Param is passed by value, moved
    pub fn set_assists(&mut self, v: u32) {
        self.assists = ::std::option::Option::Some(v);
    }

    pub fn get_assists(&self) -> u32 {
        self.assists.unwrap_or(0)
    }

    fn get_assists_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.assists
    }

    fn mut_assists_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.assists
    }

    // repeated uint32 items = 6;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::std::vec::Vec<u32>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.items, ::std::vec::Vec::new())
    }

    pub fn get_items(&self) -> &[u32] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.items
    }

    // optional uint32 player_slot = 7;

    pub fn clear_player_slot(&mut self) {
        self.player_slot = ::std::option::Option::None;
    }

    pub fn has_player_slot(&self) -> bool {
        self.player_slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_slot(&mut self, v: u32) {
        self.player_slot = ::std::option::Option::Some(v);
    }

    pub fn get_player_slot(&self) -> u32 {
        self.player_slot.unwrap_or(0)
    }

    fn get_player_slot_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_slot
    }

    fn mut_player_slot_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_slot
    }
}

impl ::protobuf::Message for CMsgDOTAMatchMinimal_Player {
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
                    let tmp = is.read_uint32()?;
                    self.hero_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.kills = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.deaths = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.assists = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.items)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.player_slot = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hero_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.kills {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.deaths {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.assists {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.items {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.player_slot {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.hero_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.kills {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.deaths {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.assists {
            os.write_uint32(5, v)?;
        };
        for v in &self.items {
            os.write_uint32(6, *v)?;
        };
        if let Some(v) = self.player_slot {
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

impl ::protobuf::MessageStatic for CMsgDOTAMatchMinimal_Player {
    fn new() -> CMsgDOTAMatchMinimal_Player {
        CMsgDOTAMatchMinimal_Player::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAMatchMinimal_Player>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgDOTAMatchMinimal_Player::get_account_id_for_reflect,
                    CMsgDOTAMatchMinimal_Player::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hero_id",
                    CMsgDOTAMatchMinimal_Player::get_hero_id_for_reflect,
                    CMsgDOTAMatchMinimal_Player::mut_hero_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "kills",
                    CMsgDOTAMatchMinimal_Player::get_kills_for_reflect,
                    CMsgDOTAMatchMinimal_Player::mut_kills_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "deaths",
                    CMsgDOTAMatchMinimal_Player::get_deaths_for_reflect,
                    CMsgDOTAMatchMinimal_Player::mut_deaths_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "assists",
                    CMsgDOTAMatchMinimal_Player::get_assists_for_reflect,
                    CMsgDOTAMatchMinimal_Player::mut_assists_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "items",
                    CMsgDOTAMatchMinimal_Player::get_items_for_reflect,
                    CMsgDOTAMatchMinimal_Player::mut_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_slot",
                    CMsgDOTAMatchMinimal_Player::get_player_slot_for_reflect,
                    CMsgDOTAMatchMinimal_Player::mut_player_slot_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAMatchMinimal_Player>(
                    "CMsgDOTAMatchMinimal_Player",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAMatchMinimal_Player {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_hero_id();
        self.clear_kills();
        self.clear_deaths();
        self.clear_assists();
        self.clear_items();
        self.clear_player_slot();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAMatchMinimal_Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAMatchMinimal_Player {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgDOTAMatchMinimal_Tourney {
    // message fields
    league_id: ::std::option::Option<u32>,
    series_type: ::std::option::Option<u32>,
    series_game: ::std::option::Option<u32>,
    weekend_tourney_tournament_id: ::std::option::Option<u32>,
    weekend_tourney_season_trophy_id: ::std::option::Option<u32>,
    weekend_tourney_division: ::std::option::Option<u32>,
    weekend_tourney_skill_level: ::std::option::Option<u32>,
    radiant_team_id: ::std::option::Option<u32>,
    radiant_team_name: ::protobuf::SingularField<::std::string::String>,
    radiant_team_logo: ::std::option::Option<u64>,
    dire_team_id: ::std::option::Option<u32>,
    dire_team_name: ::protobuf::SingularField<::std::string::String>,
    dire_team_logo: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgDOTAMatchMinimal_Tourney {}

impl CMsgDOTAMatchMinimal_Tourney {
    pub fn new() -> CMsgDOTAMatchMinimal_Tourney {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgDOTAMatchMinimal_Tourney {
        static mut instance: ::protobuf::lazy::Lazy<CMsgDOTAMatchMinimal_Tourney> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgDOTAMatchMinimal_Tourney,
        };
        unsafe {
            instance.get(CMsgDOTAMatchMinimal_Tourney::new)
        }
    }

    // optional uint32 league_id = 1;

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

    // optional uint32 series_type = 8;

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

    // optional uint32 series_game = 9;

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

    // optional uint32 weekend_tourney_tournament_id = 10;

    pub fn clear_weekend_tourney_tournament_id(&mut self) {
        self.weekend_tourney_tournament_id = ::std::option::Option::None;
    }

    pub fn has_weekend_tourney_tournament_id(&self) -> bool {
        self.weekend_tourney_tournament_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weekend_tourney_tournament_id(&mut self, v: u32) {
        self.weekend_tourney_tournament_id = ::std::option::Option::Some(v);
    }

    pub fn get_weekend_tourney_tournament_id(&self) -> u32 {
        self.weekend_tourney_tournament_id.unwrap_or(0)
    }

    fn get_weekend_tourney_tournament_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.weekend_tourney_tournament_id
    }

    fn mut_weekend_tourney_tournament_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.weekend_tourney_tournament_id
    }

    // optional uint32 weekend_tourney_season_trophy_id = 11;

    pub fn clear_weekend_tourney_season_trophy_id(&mut self) {
        self.weekend_tourney_season_trophy_id = ::std::option::Option::None;
    }

    pub fn has_weekend_tourney_season_trophy_id(&self) -> bool {
        self.weekend_tourney_season_trophy_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weekend_tourney_season_trophy_id(&mut self, v: u32) {
        self.weekend_tourney_season_trophy_id = ::std::option::Option::Some(v);
    }

    pub fn get_weekend_tourney_season_trophy_id(&self) -> u32 {
        self.weekend_tourney_season_trophy_id.unwrap_or(0)
    }

    fn get_weekend_tourney_season_trophy_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.weekend_tourney_season_trophy_id
    }

    fn mut_weekend_tourney_season_trophy_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.weekend_tourney_season_trophy_id
    }

    // optional uint32 weekend_tourney_division = 12;

    pub fn clear_weekend_tourney_division(&mut self) {
        self.weekend_tourney_division = ::std::option::Option::None;
    }

    pub fn has_weekend_tourney_division(&self) -> bool {
        self.weekend_tourney_division.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weekend_tourney_division(&mut self, v: u32) {
        self.weekend_tourney_division = ::std::option::Option::Some(v);
    }

    pub fn get_weekend_tourney_division(&self) -> u32 {
        self.weekend_tourney_division.unwrap_or(0)
    }

    fn get_weekend_tourney_division_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.weekend_tourney_division
    }

    fn mut_weekend_tourney_division_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.weekend_tourney_division
    }

    // optional uint32 weekend_tourney_skill_level = 13;

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

    // optional uint32 radiant_team_id = 2;

    pub fn clear_radiant_team_id(&mut self) {
        self.radiant_team_id = ::std::option::Option::None;
    }

    pub fn has_radiant_team_id(&self) -> bool {
        self.radiant_team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_team_id(&mut self, v: u32) {
        self.radiant_team_id = ::std::option::Option::Some(v);
    }

    pub fn get_radiant_team_id(&self) -> u32 {
        self.radiant_team_id.unwrap_or(0)
    }

    fn get_radiant_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.radiant_team_id
    }

    fn mut_radiant_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.radiant_team_id
    }

    // optional string radiant_team_name = 3;

    pub fn clear_radiant_team_name(&mut self) {
        self.radiant_team_name.clear();
    }

    pub fn has_radiant_team_name(&self) -> bool {
        self.radiant_team_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_team_name(&mut self, v: ::std::string::String) {
        self.radiant_team_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_radiant_team_name(&mut self) -> &mut ::std::string::String {
        if self.radiant_team_name.is_none() {
            self.radiant_team_name.set_default();
        };
        self.radiant_team_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_radiant_team_name(&mut self) -> ::std::string::String {
        self.radiant_team_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_radiant_team_name(&self) -> &str {
        match self.radiant_team_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_radiant_team_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.radiant_team_name
    }

    fn mut_radiant_team_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.radiant_team_name
    }

    // optional fixed64 radiant_team_logo = 4;

    pub fn clear_radiant_team_logo(&mut self) {
        self.radiant_team_logo = ::std::option::Option::None;
    }

    pub fn has_radiant_team_logo(&self) -> bool {
        self.radiant_team_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radiant_team_logo(&mut self, v: u64) {
        self.radiant_team_logo = ::std::option::Option::Some(v);
    }

    pub fn get_radiant_team_logo(&self) -> u64 {
        self.radiant_team_logo.unwrap_or(0)
    }

    fn get_radiant_team_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.radiant_team_logo
    }

    fn mut_radiant_team_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.radiant_team_logo
    }

    // optional uint32 dire_team_id = 5;

    pub fn clear_dire_team_id(&mut self) {
        self.dire_team_id = ::std::option::Option::None;
    }

    pub fn has_dire_team_id(&self) -> bool {
        self.dire_team_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dire_team_id(&mut self, v: u32) {
        self.dire_team_id = ::std::option::Option::Some(v);
    }

    pub fn get_dire_team_id(&self) -> u32 {
        self.dire_team_id.unwrap_or(0)
    }

    fn get_dire_team_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dire_team_id
    }

    fn mut_dire_team_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dire_team_id
    }

    // optional string dire_team_name = 6;

    pub fn clear_dire_team_name(&mut self) {
        self.dire_team_name.clear();
    }

    pub fn has_dire_team_name(&self) -> bool {
        self.dire_team_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dire_team_name(&mut self, v: ::std::string::String) {
        self.dire_team_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dire_team_name(&mut self) -> &mut ::std::string::String {
        if self.dire_team_name.is_none() {
            self.dire_team_name.set_default();
        };
        self.dire_team_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_dire_team_name(&mut self) -> ::std::string::String {
        self.dire_team_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_dire_team_name(&self) -> &str {
        match self.dire_team_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_dire_team_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.dire_team_name
    }

    fn mut_dire_team_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.dire_team_name
    }

    // optional fixed64 dire_team_logo = 7;

    pub fn clear_dire_team_logo(&mut self) {
        self.dire_team_logo = ::std::option::Option::None;
    }

    pub fn has_dire_team_logo(&self) -> bool {
        self.dire_team_logo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dire_team_logo(&mut self, v: u64) {
        self.dire_team_logo = ::std::option::Option::Some(v);
    }

    pub fn get_dire_team_logo(&self) -> u64 {
        self.dire_team_logo.unwrap_or(0)
    }

    fn get_dire_team_logo_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.dire_team_logo
    }

    fn mut_dire_team_logo_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.dire_team_logo
    }
}

impl ::protobuf::Message for CMsgDOTAMatchMinimal_Tourney {
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
                    self.league_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_type = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_game = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_tournament_id = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_season_trophy_id = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_division = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.weekend_tourney_skill_level = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.radiant_team_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.radiant_team_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.radiant_team_logo = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.dire_team_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.dire_team_name)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.dire_team_logo = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.league_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.series_type {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.series_game {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_tournament_id {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_season_trophy_id {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_division {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.weekend_tourney_skill_level {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radiant_team_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.radiant_team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.radiant_team_logo {
            my_size += 9;
        };
        if let Some(v) = self.dire_team_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dire_team_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.dire_team_logo {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.league_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.series_type {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.series_game {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.weekend_tourney_tournament_id {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.weekend_tourney_season_trophy_id {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.weekend_tourney_division {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.weekend_tourney_skill_level {
            os.write_uint32(13, v)?;
        };
        if let Some(v) = self.radiant_team_id {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.radiant_team_name.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.radiant_team_logo {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.dire_team_id {
            os.write_uint32(5, v)?;
        };
        if let Some(v) = self.dire_team_name.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.dire_team_logo {
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

impl ::protobuf::MessageStatic for CMsgDOTAMatchMinimal_Tourney {
    fn new() -> CMsgDOTAMatchMinimal_Tourney {
        CMsgDOTAMatchMinimal_Tourney::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgDOTAMatchMinimal_Tourney>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "league_id",
                    CMsgDOTAMatchMinimal_Tourney::get_league_id_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_league_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_type",
                    CMsgDOTAMatchMinimal_Tourney::get_series_type_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_series_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_game",
                    CMsgDOTAMatchMinimal_Tourney::get_series_game_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_series_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_tournament_id",
                    CMsgDOTAMatchMinimal_Tourney::get_weekend_tourney_tournament_id_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_weekend_tourney_tournament_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_season_trophy_id",
                    CMsgDOTAMatchMinimal_Tourney::get_weekend_tourney_season_trophy_id_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_weekend_tourney_season_trophy_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_division",
                    CMsgDOTAMatchMinimal_Tourney::get_weekend_tourney_division_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_weekend_tourney_division_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "weekend_tourney_skill_level",
                    CMsgDOTAMatchMinimal_Tourney::get_weekend_tourney_skill_level_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_weekend_tourney_skill_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "radiant_team_id",
                    CMsgDOTAMatchMinimal_Tourney::get_radiant_team_id_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_radiant_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "radiant_team_name",
                    CMsgDOTAMatchMinimal_Tourney::get_radiant_team_name_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_radiant_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "radiant_team_logo",
                    CMsgDOTAMatchMinimal_Tourney::get_radiant_team_logo_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_radiant_team_logo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dire_team_id",
                    CMsgDOTAMatchMinimal_Tourney::get_dire_team_id_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_dire_team_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dire_team_name",
                    CMsgDOTAMatchMinimal_Tourney::get_dire_team_name_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_dire_team_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "dire_team_logo",
                    CMsgDOTAMatchMinimal_Tourney::get_dire_team_logo_for_reflect,
                    CMsgDOTAMatchMinimal_Tourney::mut_dire_team_logo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgDOTAMatchMinimal_Tourney>(
                    "CMsgDOTAMatchMinimal_Tourney",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgDOTAMatchMinimal_Tourney {
    fn clear(&mut self) {
        self.clear_league_id();
        self.clear_series_type();
        self.clear_series_game();
        self.clear_weekend_tourney_tournament_id();
        self.clear_weekend_tourney_season_trophy_id();
        self.clear_weekend_tourney_division();
        self.clear_weekend_tourney_skill_level();
        self.clear_radiant_team_id();
        self.clear_radiant_team_name();
        self.clear_radiant_team_logo();
        self.clear_dire_team_id();
        self.clear_dire_team_name();
        self.clear_dire_team_logo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgDOTAMatchMinimal_Tourney {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgDOTAMatchMinimal_Tourney {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAReplayDownloadInfo {
    // message fields
    field_match: ::protobuf::SingularPtrField<CMsgDOTAMatchMinimal>,
    title: ::protobuf::SingularField<::std::string::String>,
    description: ::protobuf::SingularField<::std::string::String>,
    size: ::std::option::Option<u32>,
    tags: ::protobuf::RepeatedField<::std::string::String>,
    exists_on_disk: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAReplayDownloadInfo {}

impl CDOTAReplayDownloadInfo {
    pub fn new() -> CDOTAReplayDownloadInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAReplayDownloadInfo {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAReplayDownloadInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAReplayDownloadInfo,
        };
        unsafe {
            instance.get(CDOTAReplayDownloadInfo::new)
        }
    }

    // optional .dota.CMsgDOTAMatchMinimal match = 1;

    pub fn clear_field_match(&mut self) {
        self.field_match.clear();
    }

    pub fn has_field_match(&self) -> bool {
        self.field_match.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_match(&mut self, v: CMsgDOTAMatchMinimal) {
        self.field_match = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_match(&mut self) -> &mut CMsgDOTAMatchMinimal {
        if self.field_match.is_none() {
            self.field_match.set_default();
        };
        self.field_match.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_match(&mut self) -> CMsgDOTAMatchMinimal {
        self.field_match.take().unwrap_or_else(|| CMsgDOTAMatchMinimal::new())
    }

    pub fn get_field_match(&self) -> &CMsgDOTAMatchMinimal {
        self.field_match.as_ref().unwrap_or_else(|| CMsgDOTAMatchMinimal::default_instance())
    }

    fn get_field_match_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgDOTAMatchMinimal> {
        &self.field_match
    }

    fn mut_field_match_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgDOTAMatchMinimal> {
        &mut self.field_match
    }

    // optional string title = 2;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title(&mut self) -> &mut ::std::string::String {
        if self.title.is_none() {
            self.title.set_default();
        };
        self.title.as_mut().unwrap()
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        self.title.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_title(&self) -> &str {
        match self.title.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_title_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.title
    }

    fn mut_title_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.title
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

    // optional uint32 size = 4;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u32) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u32 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.size
    }

    // repeated string tags = 5;

    pub fn clear_tags(&mut self) {
        self.tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_tags(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tags(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // Take field
    pub fn take_tags(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.tags, ::protobuf::RepeatedField::new())
    }

    pub fn get_tags(&self) -> &[::std::string::String] {
        &self.tags
    }

    fn get_tags_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.tags
    }

    fn mut_tags_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.tags
    }

    // optional bool exists_on_disk = 6;

    pub fn clear_exists_on_disk(&mut self) {
        self.exists_on_disk = ::std::option::Option::None;
    }

    pub fn has_exists_on_disk(&self) -> bool {
        self.exists_on_disk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exists_on_disk(&mut self, v: bool) {
        self.exists_on_disk = ::std::option::Option::Some(v);
    }

    pub fn get_exists_on_disk(&self) -> bool {
        self.exists_on_disk.unwrap_or(false)
    }

    fn get_exists_on_disk_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.exists_on_disk
    }

    fn mut_exists_on_disk_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.exists_on_disk
    }
}

impl ::protobuf::Message for CDOTAReplayDownloadInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field_match)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.title)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.tags)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.exists_on_disk = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_match.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.title.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        };
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.tags {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if let Some(v) = self.exists_on_disk {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_match.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.title.as_ref() {
            os.write_string(2, &v)?;
        };
        if let Some(v) = self.description.as_ref() {
            os.write_string(3, &v)?;
        };
        if let Some(v) = self.size {
            os.write_uint32(4, v)?;
        };
        for v in &self.tags {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.exists_on_disk {
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

impl ::protobuf::MessageStatic for CDOTAReplayDownloadInfo {
    fn new() -> CDOTAReplayDownloadInfo {
        CDOTAReplayDownloadInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAReplayDownloadInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgDOTAMatchMinimal>>(
                    "match",
                    CDOTAReplayDownloadInfo::get_field_match_for_reflect,
                    CDOTAReplayDownloadInfo::mut_field_match_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "title",
                    CDOTAReplayDownloadInfo::get_title_for_reflect,
                    CDOTAReplayDownloadInfo::mut_title_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    CDOTAReplayDownloadInfo::get_description_for_reflect,
                    CDOTAReplayDownloadInfo::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "size",
                    CDOTAReplayDownloadInfo::get_size_for_reflect,
                    CDOTAReplayDownloadInfo::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "tags",
                    CDOTAReplayDownloadInfo::get_tags_for_reflect,
                    CDOTAReplayDownloadInfo::mut_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "exists_on_disk",
                    CDOTAReplayDownloadInfo::get_exists_on_disk_for_reflect,
                    CDOTAReplayDownloadInfo::mut_exists_on_disk_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAReplayDownloadInfo>(
                    "CDOTAReplayDownloadInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAReplayDownloadInfo {
    fn clear(&mut self) {
        self.clear_field_match();
        self.clear_title();
        self.clear_description();
        self.clear_size();
        self.clear_tags();
        self.clear_exists_on_disk();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAReplayDownloadInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAReplayDownloadInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAReplayDownloadInfo_Highlight {
    // message fields
    timestamp: ::std::option::Option<u32>,
    description: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAReplayDownloadInfo_Highlight {}

impl CDOTAReplayDownloadInfo_Highlight {
    pub fn new() -> CDOTAReplayDownloadInfo_Highlight {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAReplayDownloadInfo_Highlight {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAReplayDownloadInfo_Highlight> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAReplayDownloadInfo_Highlight,
        };
        unsafe {
            instance.get(CDOTAReplayDownloadInfo_Highlight::new)
        }
    }

    // optional uint32 timestamp = 1;

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

    // optional string description = 2;

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
}

impl ::protobuf::Message for CDOTAReplayDownloadInfo_Highlight {
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
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
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
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.timestamp {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.description.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTAReplayDownloadInfo_Highlight {
    fn new() -> CDOTAReplayDownloadInfo_Highlight {
        CDOTAReplayDownloadInfo_Highlight::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAReplayDownloadInfo_Highlight>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "timestamp",
                    CDOTAReplayDownloadInfo_Highlight::get_timestamp_for_reflect,
                    CDOTAReplayDownloadInfo_Highlight::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    CDOTAReplayDownloadInfo_Highlight::get_description_for_reflect,
                    CDOTAReplayDownloadInfo_Highlight::mut_description_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAReplayDownloadInfo_Highlight>(
                    "CDOTAReplayDownloadInfo_Highlight",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAReplayDownloadInfo_Highlight {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_description();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAReplayDownloadInfo_Highlight {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAReplayDownloadInfo_Highlight {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgWatchGame {
    // message fields
    server_steamid: ::std::option::Option<u64>,
    client_version: ::std::option::Option<u32>,
    watch_server_steamid: ::std::option::Option<u64>,
    lobby_id: ::std::option::Option<u64>,
    regions: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgWatchGame {}

impl CMsgWatchGame {
    pub fn new() -> CMsgWatchGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgWatchGame {
        static mut instance: ::protobuf::lazy::Lazy<CMsgWatchGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgWatchGame,
        };
        unsafe {
            instance.get(CMsgWatchGame::new)
        }
    }

    // optional fixed64 server_steamid = 1;

    pub fn clear_server_steamid(&mut self) {
        self.server_steamid = ::std::option::Option::None;
    }

    pub fn has_server_steamid(&self) -> bool {
        self.server_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_steamid(&mut self, v: u64) {
        self.server_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_server_steamid(&self) -> u64 {
        self.server_steamid.unwrap_or(0)
    }

    fn get_server_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.server_steamid
    }

    fn mut_server_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.server_steamid
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

    // optional fixed64 watch_server_steamid = 3;

    pub fn clear_watch_server_steamid(&mut self) {
        self.watch_server_steamid = ::std::option::Option::None;
    }

    pub fn has_watch_server_steamid(&self) -> bool {
        self.watch_server_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_watch_server_steamid(&mut self, v: u64) {
        self.watch_server_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_watch_server_steamid(&self) -> u64 {
        self.watch_server_steamid.unwrap_or(0)
    }

    fn get_watch_server_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.watch_server_steamid
    }

    fn mut_watch_server_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.watch_server_steamid
    }

    // optional uint64 lobby_id = 4;

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

    // repeated uint32 regions = 5;

    pub fn clear_regions(&mut self) {
        self.regions.clear();
    }

    // Param is passed by value, moved
    pub fn set_regions(&mut self, v: ::std::vec::Vec<u32>) {
        self.regions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.regions
    }

    // Take field
    pub fn take_regions(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new())
    }

    pub fn get_regions(&self) -> &[u32] {
        &self.regions
    }

    fn get_regions_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.regions
    }

    fn mut_regions_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.regions
    }
}

impl ::protobuf::Message for CMsgWatchGame {
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
                    self.server_steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.client_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.watch_server_steamid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.regions)?;
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
        if let Some(v) = self.server_steamid {
            my_size += 9;
        };
        if let Some(v) = self.client_version {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.watch_server_steamid {
            my_size += 9;
        };
        if let Some(v) = self.lobby_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.regions {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.server_steamid {
            os.write_fixed64(1, v)?;
        };
        if let Some(v) = self.client_version {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.watch_server_steamid {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.lobby_id {
            os.write_uint64(4, v)?;
        };
        for v in &self.regions {
            os.write_uint32(5, *v)?;
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

impl ::protobuf::MessageStatic for CMsgWatchGame {
    fn new() -> CMsgWatchGame {
        CMsgWatchGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgWatchGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "server_steamid",
                    CMsgWatchGame::get_server_steamid_for_reflect,
                    CMsgWatchGame::mut_server_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_version",
                    CMsgWatchGame::get_client_version_for_reflect,
                    CMsgWatchGame::mut_client_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "watch_server_steamid",
                    CMsgWatchGame::get_watch_server_steamid_for_reflect,
                    CMsgWatchGame::mut_watch_server_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lobby_id",
                    CMsgWatchGame::get_lobby_id_for_reflect,
                    CMsgWatchGame::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "regions",
                    CMsgWatchGame::get_regions_for_reflect,
                    CMsgWatchGame::mut_regions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgWatchGame>(
                    "CMsgWatchGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgWatchGame {
    fn clear(&mut self) {
        self.clear_server_steamid();
        self.clear_client_version();
        self.clear_watch_server_steamid();
        self.clear_lobby_id();
        self.clear_regions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgWatchGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgWatchGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgCancelWatchGame {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgCancelWatchGame {}

impl CMsgCancelWatchGame {
    pub fn new() -> CMsgCancelWatchGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgCancelWatchGame {
        static mut instance: ::protobuf::lazy::Lazy<CMsgCancelWatchGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgCancelWatchGame,
        };
        unsafe {
            instance.get(CMsgCancelWatchGame::new)
        }
    }
}

impl ::protobuf::Message for CMsgCancelWatchGame {
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

impl ::protobuf::MessageStatic for CMsgCancelWatchGame {
    fn new() -> CMsgCancelWatchGame {
        CMsgCancelWatchGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgCancelWatchGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CMsgCancelWatchGame>(
                    "CMsgCancelWatchGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgCancelWatchGame {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgCancelWatchGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgCancelWatchGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgWatchGameResponse {
    // message fields
    watch_game_result: ::std::option::Option<CMsgWatchGameResponse_WatchGameResult>,
    source_tv_public_addr: ::std::option::Option<u32>,
    source_tv_private_addr: ::std::option::Option<u32>,
    source_tv_port: ::std::option::Option<u32>,
    game_server_steamid: ::std::option::Option<u64>,
    watch_server_steamid: ::std::option::Option<u64>,
    watch_tv_unique_secret_code: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgWatchGameResponse {}

impl CMsgWatchGameResponse {
    pub fn new() -> CMsgWatchGameResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgWatchGameResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgWatchGameResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgWatchGameResponse,
        };
        unsafe {
            instance.get(CMsgWatchGameResponse::new)
        }
    }

    // optional .dota.CMsgWatchGameResponse.WatchGameResult watch_game_result = 1;

    pub fn clear_watch_game_result(&mut self) {
        self.watch_game_result = ::std::option::Option::None;
    }

    pub fn has_watch_game_result(&self) -> bool {
        self.watch_game_result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_watch_game_result(&mut self, v: CMsgWatchGameResponse_WatchGameResult) {
        self.watch_game_result = ::std::option::Option::Some(v);
    }

    pub fn get_watch_game_result(&self) -> CMsgWatchGameResponse_WatchGameResult {
        self.watch_game_result.unwrap_or(CMsgWatchGameResponse_WatchGameResult::PENDING)
    }

    fn get_watch_game_result_for_reflect(&self) -> &::std::option::Option<CMsgWatchGameResponse_WatchGameResult> {
        &self.watch_game_result
    }

    fn mut_watch_game_result_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgWatchGameResponse_WatchGameResult> {
        &mut self.watch_game_result
    }

    // optional uint32 source_tv_public_addr = 2;

    pub fn clear_source_tv_public_addr(&mut self) {
        self.source_tv_public_addr = ::std::option::Option::None;
    }

    pub fn has_source_tv_public_addr(&self) -> bool {
        self.source_tv_public_addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_tv_public_addr(&mut self, v: u32) {
        self.source_tv_public_addr = ::std::option::Option::Some(v);
    }

    pub fn get_source_tv_public_addr(&self) -> u32 {
        self.source_tv_public_addr.unwrap_or(0)
    }

    fn get_source_tv_public_addr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.source_tv_public_addr
    }

    fn mut_source_tv_public_addr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.source_tv_public_addr
    }

    // optional uint32 source_tv_private_addr = 3;

    pub fn clear_source_tv_private_addr(&mut self) {
        self.source_tv_private_addr = ::std::option::Option::None;
    }

    pub fn has_source_tv_private_addr(&self) -> bool {
        self.source_tv_private_addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_tv_private_addr(&mut self, v: u32) {
        self.source_tv_private_addr = ::std::option::Option::Some(v);
    }

    pub fn get_source_tv_private_addr(&self) -> u32 {
        self.source_tv_private_addr.unwrap_or(0)
    }

    fn get_source_tv_private_addr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.source_tv_private_addr
    }

    fn mut_source_tv_private_addr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.source_tv_private_addr
    }

    // optional uint32 source_tv_port = 4;

    pub fn clear_source_tv_port(&mut self) {
        self.source_tv_port = ::std::option::Option::None;
    }

    pub fn has_source_tv_port(&self) -> bool {
        self.source_tv_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_tv_port(&mut self, v: u32) {
        self.source_tv_port = ::std::option::Option::Some(v);
    }

    pub fn get_source_tv_port(&self) -> u32 {
        self.source_tv_port.unwrap_or(0)
    }

    fn get_source_tv_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.source_tv_port
    }

    fn mut_source_tv_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.source_tv_port
    }

    // optional fixed64 game_server_steamid = 5;

    pub fn clear_game_server_steamid(&mut self) {
        self.game_server_steamid = ::std::option::Option::None;
    }

    pub fn has_game_server_steamid(&self) -> bool {
        self.game_server_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_server_steamid(&mut self, v: u64) {
        self.game_server_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_game_server_steamid(&self) -> u64 {
        self.game_server_steamid.unwrap_or(0)
    }

    fn get_game_server_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.game_server_steamid
    }

    fn mut_game_server_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.game_server_steamid
    }

    // optional fixed64 watch_server_steamid = 6;

    pub fn clear_watch_server_steamid(&mut self) {
        self.watch_server_steamid = ::std::option::Option::None;
    }

    pub fn has_watch_server_steamid(&self) -> bool {
        self.watch_server_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_watch_server_steamid(&mut self, v: u64) {
        self.watch_server_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_watch_server_steamid(&self) -> u64 {
        self.watch_server_steamid.unwrap_or(0)
    }

    fn get_watch_server_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.watch_server_steamid
    }

    fn mut_watch_server_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.watch_server_steamid
    }

    // optional fixed64 watch_tv_unique_secret_code = 7;

    pub fn clear_watch_tv_unique_secret_code(&mut self) {
        self.watch_tv_unique_secret_code = ::std::option::Option::None;
    }

    pub fn has_watch_tv_unique_secret_code(&self) -> bool {
        self.watch_tv_unique_secret_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_watch_tv_unique_secret_code(&mut self, v: u64) {
        self.watch_tv_unique_secret_code = ::std::option::Option::Some(v);
    }

    pub fn get_watch_tv_unique_secret_code(&self) -> u64 {
        self.watch_tv_unique_secret_code.unwrap_or(0)
    }

    fn get_watch_tv_unique_secret_code_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.watch_tv_unique_secret_code
    }

    fn mut_watch_tv_unique_secret_code_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.watch_tv_unique_secret_code
    }
}

impl ::protobuf::Message for CMsgWatchGameResponse {
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
                    self.watch_game_result = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.source_tv_public_addr = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.source_tv_private_addr = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.source_tv_port = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.game_server_steamid = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.watch_server_steamid = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.watch_tv_unique_secret_code = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.watch_game_result {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.source_tv_public_addr {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.source_tv_private_addr {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.source_tv_port {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_server_steamid {
            my_size += 9;
        };
        if let Some(v) = self.watch_server_steamid {
            my_size += 9;
        };
        if let Some(v) = self.watch_tv_unique_secret_code {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.watch_game_result {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.source_tv_public_addr {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.source_tv_private_addr {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.source_tv_port {
            os.write_uint32(4, v)?;
        };
        if let Some(v) = self.game_server_steamid {
            os.write_fixed64(5, v)?;
        };
        if let Some(v) = self.watch_server_steamid {
            os.write_fixed64(6, v)?;
        };
        if let Some(v) = self.watch_tv_unique_secret_code {
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

impl ::protobuf::MessageStatic for CMsgWatchGameResponse {
    fn new() -> CMsgWatchGameResponse {
        CMsgWatchGameResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgWatchGameResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgWatchGameResponse_WatchGameResult>>(
                    "watch_game_result",
                    CMsgWatchGameResponse::get_watch_game_result_for_reflect,
                    CMsgWatchGameResponse::mut_watch_game_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "source_tv_public_addr",
                    CMsgWatchGameResponse::get_source_tv_public_addr_for_reflect,
                    CMsgWatchGameResponse::mut_source_tv_public_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "source_tv_private_addr",
                    CMsgWatchGameResponse::get_source_tv_private_addr_for_reflect,
                    CMsgWatchGameResponse::mut_source_tv_private_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "source_tv_port",
                    CMsgWatchGameResponse::get_source_tv_port_for_reflect,
                    CMsgWatchGameResponse::mut_source_tv_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "game_server_steamid",
                    CMsgWatchGameResponse::get_game_server_steamid_for_reflect,
                    CMsgWatchGameResponse::mut_game_server_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "watch_server_steamid",
                    CMsgWatchGameResponse::get_watch_server_steamid_for_reflect,
                    CMsgWatchGameResponse::mut_watch_server_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "watch_tv_unique_secret_code",
                    CMsgWatchGameResponse::get_watch_tv_unique_secret_code_for_reflect,
                    CMsgWatchGameResponse::mut_watch_tv_unique_secret_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgWatchGameResponse>(
                    "CMsgWatchGameResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgWatchGameResponse {
    fn clear(&mut self) {
        self.clear_watch_game_result();
        self.clear_source_tv_public_addr();
        self.clear_source_tv_private_addr();
        self.clear_source_tv_port();
        self.clear_game_server_steamid();
        self.clear_watch_server_steamid();
        self.clear_watch_tv_unique_secret_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgWatchGameResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgWatchGameResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgWatchGameResponse_WatchGameResult {
    PENDING = 0,
    READY = 1,
    GAMESERVERNOTFOUND = 2,
    UNAVAILABLE = 3,
    CANCELLED = 4,
    INCOMPATIBLEVERSION = 5,
    MISSINGLEAGUESUBSCRIPTION = 6,
    LOBBYNOTFOUND = 7,
}

impl ::protobuf::ProtobufEnum for CMsgWatchGameResponse_WatchGameResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgWatchGameResponse_WatchGameResult> {
        match value {
            0 => ::std::option::Option::Some(CMsgWatchGameResponse_WatchGameResult::PENDING),
            1 => ::std::option::Option::Some(CMsgWatchGameResponse_WatchGameResult::READY),
            2 => ::std::option::Option::Some(CMsgWatchGameResponse_WatchGameResult::GAMESERVERNOTFOUND),
            3 => ::std::option::Option::Some(CMsgWatchGameResponse_WatchGameResult::UNAVAILABLE),
            4 => ::std::option::Option::Some(CMsgWatchGameResponse_WatchGameResult::CANCELLED),
            5 => ::std::option::Option::Some(CMsgWatchGameResponse_WatchGameResult::INCOMPATIBLEVERSION),
            6 => ::std::option::Option::Some(CMsgWatchGameResponse_WatchGameResult::MISSINGLEAGUESUBSCRIPTION),
            7 => ::std::option::Option::Some(CMsgWatchGameResponse_WatchGameResult::LOBBYNOTFOUND),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgWatchGameResponse_WatchGameResult] = &[
            CMsgWatchGameResponse_WatchGameResult::PENDING,
            CMsgWatchGameResponse_WatchGameResult::READY,
            CMsgWatchGameResponse_WatchGameResult::GAMESERVERNOTFOUND,
            CMsgWatchGameResponse_WatchGameResult::UNAVAILABLE,
            CMsgWatchGameResponse_WatchGameResult::CANCELLED,
            CMsgWatchGameResponse_WatchGameResult::INCOMPATIBLEVERSION,
            CMsgWatchGameResponse_WatchGameResult::MISSINGLEAGUESUBSCRIPTION,
            CMsgWatchGameResponse_WatchGameResult::LOBBYNOTFOUND,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CMsgWatchGameResponse_WatchGameResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgWatchGameResponse_WatchGameResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgWatchGameResponse_WatchGameResult {
}

impl ::protobuf::reflect::ProtobufValue for CMsgWatchGameResponse_WatchGameResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPartyLeaderWatchGamePrompt {
    // message fields
    game_server_steamid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPartyLeaderWatchGamePrompt {}

impl CMsgPartyLeaderWatchGamePrompt {
    pub fn new() -> CMsgPartyLeaderWatchGamePrompt {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPartyLeaderWatchGamePrompt {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPartyLeaderWatchGamePrompt> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPartyLeaderWatchGamePrompt,
        };
        unsafe {
            instance.get(CMsgPartyLeaderWatchGamePrompt::new)
        }
    }

    // optional fixed64 game_server_steamid = 5;

    pub fn clear_game_server_steamid(&mut self) {
        self.game_server_steamid = ::std::option::Option::None;
    }

    pub fn has_game_server_steamid(&self) -> bool {
        self.game_server_steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_server_steamid(&mut self, v: u64) {
        self.game_server_steamid = ::std::option::Option::Some(v);
    }

    pub fn get_game_server_steamid(&self) -> u64 {
        self.game_server_steamid.unwrap_or(0)
    }

    fn get_game_server_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.game_server_steamid
    }

    fn mut_game_server_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.game_server_steamid
    }
}

impl ::protobuf::Message for CMsgPartyLeaderWatchGamePrompt {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.game_server_steamid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.game_server_steamid {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.game_server_steamid {
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

impl ::protobuf::MessageStatic for CMsgPartyLeaderWatchGamePrompt {
    fn new() -> CMsgPartyLeaderWatchGamePrompt {
        CMsgPartyLeaderWatchGamePrompt::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPartyLeaderWatchGamePrompt>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "game_server_steamid",
                    CMsgPartyLeaderWatchGamePrompt::get_game_server_steamid_for_reflect,
                    CMsgPartyLeaderWatchGamePrompt::mut_game_server_steamid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPartyLeaderWatchGamePrompt>(
                    "CMsgPartyLeaderWatchGamePrompt",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPartyLeaderWatchGamePrompt {
    fn clear(&mut self) {
        self.clear_game_server_steamid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPartyLeaderWatchGamePrompt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPartyLeaderWatchGamePrompt {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTABroadcasterInfo {
    // message fields
    account_id: ::std::option::Option<u32>,
    server_steam_id: ::std::option::Option<u64>,
    live: ::std::option::Option<bool>,
    team_name_radiant: ::protobuf::SingularField<::std::string::String>,
    team_name_dire: ::protobuf::SingularField<::std::string::String>,
    stage_name: ::protobuf::SingularField<::std::string::String>,
    series_game: ::std::option::Option<u32>,
    series_type: ::std::option::Option<u32>,
    upcoming_broadcast_timestamp: ::std::option::Option<u32>,
    allow_live_video: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTABroadcasterInfo {}

impl CDOTABroadcasterInfo {
    pub fn new() -> CDOTABroadcasterInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTABroadcasterInfo {
        static mut instance: ::protobuf::lazy::Lazy<CDOTABroadcasterInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTABroadcasterInfo,
        };
        unsafe {
            instance.get(CDOTABroadcasterInfo::new)
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

    // optional fixed64 server_steam_id = 2;

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

    // optional bool live = 3;

    pub fn clear_live(&mut self) {
        self.live = ::std::option::Option::None;
    }

    pub fn has_live(&self) -> bool {
        self.live.is_some()
    }

    // Param is passed by value, moved
    pub fn set_live(&mut self, v: bool) {
        self.live = ::std::option::Option::Some(v);
    }

    pub fn get_live(&self) -> bool {
        self.live.unwrap_or(false)
    }

    fn get_live_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.live
    }

    fn mut_live_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.live
    }

    // optional string team_name_radiant = 4;

    pub fn clear_team_name_radiant(&mut self) {
        self.team_name_radiant.clear();
    }

    pub fn has_team_name_radiant(&self) -> bool {
        self.team_name_radiant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name_radiant(&mut self, v: ::std::string::String) {
        self.team_name_radiant = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name_radiant(&mut self) -> &mut ::std::string::String {
        if self.team_name_radiant.is_none() {
            self.team_name_radiant.set_default();
        };
        self.team_name_radiant.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name_radiant(&mut self) -> ::std::string::String {
        self.team_name_radiant.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name_radiant(&self) -> &str {
        match self.team_name_radiant.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_radiant_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name_radiant
    }

    fn mut_team_name_radiant_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name_radiant
    }

    // optional string team_name_dire = 5;

    pub fn clear_team_name_dire(&mut self) {
        self.team_name_dire.clear();
    }

    pub fn has_team_name_dire(&self) -> bool {
        self.team_name_dire.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team_name_dire(&mut self, v: ::std::string::String) {
        self.team_name_dire = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_team_name_dire(&mut self) -> &mut ::std::string::String {
        if self.team_name_dire.is_none() {
            self.team_name_dire.set_default();
        };
        self.team_name_dire.as_mut().unwrap()
    }

    // Take field
    pub fn take_team_name_dire(&mut self) -> ::std::string::String {
        self.team_name_dire.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_team_name_dire(&self) -> &str {
        match self.team_name_dire.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_team_name_dire_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.team_name_dire
    }

    fn mut_team_name_dire_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.team_name_dire
    }

    // optional string stage_name = 6;

    pub fn clear_stage_name(&mut self) {
        self.stage_name.clear();
    }

    pub fn has_stage_name(&self) -> bool {
        self.stage_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stage_name(&mut self, v: ::std::string::String) {
        self.stage_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stage_name(&mut self) -> &mut ::std::string::String {
        if self.stage_name.is_none() {
            self.stage_name.set_default();
        };
        self.stage_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_stage_name(&mut self) -> ::std::string::String {
        self.stage_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_stage_name(&self) -> &str {
        match self.stage_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_stage_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.stage_name
    }

    fn mut_stage_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.stage_name
    }

    // optional uint32 series_game = 7;

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

    // optional uint32 series_type = 8;

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

    // optional uint32 upcoming_broadcast_timestamp = 9;

    pub fn clear_upcoming_broadcast_timestamp(&mut self) {
        self.upcoming_broadcast_timestamp = ::std::option::Option::None;
    }

    pub fn has_upcoming_broadcast_timestamp(&self) -> bool {
        self.upcoming_broadcast_timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upcoming_broadcast_timestamp(&mut self, v: u32) {
        self.upcoming_broadcast_timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_upcoming_broadcast_timestamp(&self) -> u32 {
        self.upcoming_broadcast_timestamp.unwrap_or(0)
    }

    fn get_upcoming_broadcast_timestamp_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.upcoming_broadcast_timestamp
    }

    fn mut_upcoming_broadcast_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.upcoming_broadcast_timestamp
    }

    // optional bool allow_live_video = 10;

    pub fn clear_allow_live_video(&mut self) {
        self.allow_live_video = ::std::option::Option::None;
    }

    pub fn has_allow_live_video(&self) -> bool {
        self.allow_live_video.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_live_video(&mut self, v: bool) {
        self.allow_live_video = ::std::option::Option::Some(v);
    }

    pub fn get_allow_live_video(&self) -> bool {
        self.allow_live_video.unwrap_or(false)
    }

    fn get_allow_live_video_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_live_video
    }

    fn mut_allow_live_video_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_live_video
    }
}

impl ::protobuf::Message for CDOTABroadcasterInfo {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.server_steam_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.live = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name_radiant)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.team_name_dire)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.stage_name)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_game = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.series_type = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.upcoming_broadcast_timestamp = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.allow_live_video = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.server_steam_id {
            my_size += 9;
        };
        if let Some(v) = self.live {
            my_size += 2;
        };
        if let Some(v) = self.team_name_radiant.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        };
        if let Some(v) = self.team_name_dire.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        };
        if let Some(v) = self.stage_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        };
        if let Some(v) = self.series_game {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.series_type {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.upcoming_broadcast_timestamp {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.allow_live_video {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.server_steam_id {
            os.write_fixed64(2, v)?;
        };
        if let Some(v) = self.live {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.team_name_radiant.as_ref() {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.team_name_dire.as_ref() {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.stage_name.as_ref() {
            os.write_string(6, &v)?;
        };
        if let Some(v) = self.series_game {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.series_type {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.upcoming_broadcast_timestamp {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.allow_live_video {
            os.write_bool(10, v)?;
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

impl ::protobuf::MessageStatic for CDOTABroadcasterInfo {
    fn new() -> CDOTABroadcasterInfo {
        CDOTABroadcasterInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTABroadcasterInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CDOTABroadcasterInfo::get_account_id_for_reflect,
                    CDOTABroadcasterInfo::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "server_steam_id",
                    CDOTABroadcasterInfo::get_server_steam_id_for_reflect,
                    CDOTABroadcasterInfo::mut_server_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "live",
                    CDOTABroadcasterInfo::get_live_for_reflect,
                    CDOTABroadcasterInfo::mut_live_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name_radiant",
                    CDOTABroadcasterInfo::get_team_name_radiant_for_reflect,
                    CDOTABroadcasterInfo::mut_team_name_radiant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "team_name_dire",
                    CDOTABroadcasterInfo::get_team_name_dire_for_reflect,
                    CDOTABroadcasterInfo::mut_team_name_dire_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "stage_name",
                    CDOTABroadcasterInfo::get_stage_name_for_reflect,
                    CDOTABroadcasterInfo::mut_stage_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_game",
                    CDOTABroadcasterInfo::get_series_game_for_reflect,
                    CDOTABroadcasterInfo::mut_series_game_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "series_type",
                    CDOTABroadcasterInfo::get_series_type_for_reflect,
                    CDOTABroadcasterInfo::mut_series_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "upcoming_broadcast_timestamp",
                    CDOTABroadcasterInfo::get_upcoming_broadcast_timestamp_for_reflect,
                    CDOTABroadcasterInfo::mut_upcoming_broadcast_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_live_video",
                    CDOTABroadcasterInfo::get_allow_live_video_for_reflect,
                    CDOTABroadcasterInfo::mut_allow_live_video_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTABroadcasterInfo>(
                    "CDOTABroadcasterInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTABroadcasterInfo {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_server_steam_id();
        self.clear_live();
        self.clear_team_name_radiant();
        self.clear_team_name_dire();
        self.clear_stage_name();
        self.clear_series_game();
        self.clear_series_type();
        self.clear_upcoming_broadcast_timestamp();
        self.clear_allow_live_video();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTABroadcasterInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTABroadcasterInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x22, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x17, 0x64, 0x6f, 0x74, 0x61,
    0x5f, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x5f, 0x65, 0x6e, 0x75, 0x6d, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xe9, 0x08, 0x0a, 0x12, 0x43, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54, 0x56, 0x47,
    0x61, 0x6d, 0x65, 0x53, 0x6d, 0x61, 0x6c, 0x6c, 0x12, 0x23, 0x0a, 0x0d, 0x61, 0x63, 0x74, 0x69,
    0x76, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0c, 0x61, 0x63, 0x74, 0x69, 0x76, 0x61, 0x74, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x27, 0x0a,
    0x0f, 0x64, 0x65, 0x61, 0x63, 0x74, 0x69, 0x76, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x64, 0x65, 0x61, 0x63, 0x74, 0x69, 0x76, 0x61,
    0x74, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x19,
    0x0a, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x6c, 0x65,
    0x61, 0x67, 0x75, 0x65, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x6c, 0x6f, 0x62, 0x62,
    0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x54, 0x69,
    0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x18, 0x08, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x05, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x12, 0x1e, 0x0a, 0x0a, 0x73, 0x70, 0x65, 0x63,
    0x74, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x73, 0x70,
    0x65, 0x63, 0x74, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65,
    0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x67, 0x61, 0x6d,
    0x65, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x61, 0x76, 0x65, 0x72, 0x61, 0x67, 0x65,
    0x5f, 0x6d, 0x6d, 0x72, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x61, 0x76, 0x65, 0x72,
    0x61, 0x67, 0x65, 0x4d, 0x6d, 0x72, 0x12, 0x2a, 0x0a, 0x11, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x18, 0x0f, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0f, 0x74, 0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65, 0x52, 0x61, 0x64, 0x69, 0x61,
    0x6e, 0x74, 0x12, 0x24, 0x0a, 0x0e, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x5f,
    0x64, 0x69, 0x72, 0x65, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x74, 0x65, 0x61, 0x6d,
    0x4e, 0x61, 0x6d, 0x65, 0x44, 0x69, 0x72, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x74, 0x65, 0x61, 0x6d,
    0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x18, 0x18, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x0f, 0x74, 0x65, 0x61, 0x6d, 0x4c, 0x6f, 0x67, 0x6f, 0x52, 0x61, 0x64,
    0x69, 0x61, 0x6e, 0x74, 0x12, 0x24, 0x0a, 0x0e, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6c, 0x6f, 0x67,
    0x6f, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x18, 0x19, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0c, 0x74, 0x65,
    0x61, 0x6d, 0x4c, 0x6f, 0x67, 0x6f, 0x44, 0x69, 0x72, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x6f,
    0x72, 0x74, 0x5f, 0x73, 0x63, 0x6f, 0x72, 0x65, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09,
    0x73, 0x6f, 0x72, 0x74, 0x53, 0x63, 0x6f, 0x72, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x6c, 0x61, 0x73,
    0x74, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x12, 0x20,
    0x01, 0x28, 0x02, 0x52, 0x0e, 0x6c, 0x61, 0x73, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x54,
    0x69, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x0c, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x5f, 0x6c,
    0x65, 0x61, 0x64, 0x18, 0x13, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0b, 0x72, 0x61, 0x64, 0x69, 0x61,
    0x6e, 0x74, 0x4c, 0x65, 0x61, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e,
    0x74, 0x5f, 0x73, 0x63, 0x6f, 0x72, 0x65, 0x18, 0x14, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x72,
    0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x53, 0x63, 0x6f, 0x72, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x64,
    0x69, 0x72, 0x65, 0x5f, 0x73, 0x63, 0x6f, 0x72, 0x65, 0x18, 0x15, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x09, 0x64, 0x69, 0x72, 0x65, 0x53, 0x63, 0x6f, 0x72, 0x65, 0x12, 0x39, 0x0a, 0x07, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x73, 0x18, 0x16, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x64, 0x6f,
    0x74, 0x61, 0x2e, 0x43, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54, 0x56, 0x47, 0x61, 0x6d, 0x65,
    0x53, 0x6d, 0x61, 0x6c, 0x6c, 0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x52, 0x07, 0x70, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x69, 0x6e,
    0x67, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x17, 0x20, 0x01, 0x28, 0x07, 0x52, 0x0d, 0x62,
    0x75, 0x69, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x41, 0x0a, 0x1d,
    0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f,
    0x74, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x1a, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x1a, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72,
    0x6e, 0x65, 0x79, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12,
    0x38, 0x0a, 0x18, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e,
    0x65, 0x79, 0x5f, 0x64, 0x69, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x1b, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x16, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65,
    0x79, 0x44, 0x69, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x3d, 0x0a, 0x1b, 0x77, 0x65, 0x65,
    0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f, 0x73, 0x6b, 0x69,
    0x6c, 0x6c, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x1c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x18,
    0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x53, 0x6b,
    0x69, 0x6c, 0x6c, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x41, 0x0a, 0x1d, 0x77, 0x65, 0x65, 0x6b,
    0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f, 0x62, 0x72, 0x61, 0x63,
    0x6b, 0x65, 0x74, 0x5f, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x1d, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x1a, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x42,
    0x72, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x52, 0x6f, 0x75, 0x6e, 0x64, 0x1a, 0x40, 0x0a, 0x06, 0x50,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49, 0x64, 0x22, 0xdd, 0x01,
    0x0a, 0x22, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43,
    0x46, 0x69, 0x6e, 0x64, 0x54, 0x6f, 0x70, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54, 0x56, 0x47,
    0x61, 0x6d, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x6b,
    0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68,
    0x4b, 0x65, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x49, 0x64,
    0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x67, 0x61, 0x6d, 0x65,
    0x5f, 0x6c, 0x69, 0x73, 0x74, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0d, 0x67, 0x61, 0x6d, 0x65, 0x4c, 0x69, 0x73, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78,
    0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x06, 0x20,
    0x03, 0x28, 0x04, 0x52, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x73, 0x22, 0xc3, 0x02,
    0x0a, 0x2a, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x46, 0x69, 0x6e, 0x64, 0x54, 0x6f, 0x70, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54, 0x56, 0x47,
    0x61, 0x6d, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x0a,
    0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x09, 0x73, 0x65, 0x61, 0x72, 0x63, 0x68, 0x4b, 0x65, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x6c,
    0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f,
    0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49,
    0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x47, 0x61, 0x6d, 0x65,
    0x12, 0x1b, 0x0a, 0x09, 0x6e, 0x75, 0x6d, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x08, 0x6e, 0x75, 0x6d, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x12, 0x26, 0x0a,
    0x0f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x67, 0x61, 0x6d, 0x65, 0x4c, 0x69, 0x73, 0x74,
    0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x35, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x6c, 0x69,
    0x73, 0x74, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54, 0x56, 0x47, 0x61, 0x6d, 0x65, 0x53, 0x6d, 0x61,
    0x6c, 0x6c, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x25, 0x0a, 0x0e,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0d, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x47, 0x61,
    0x6d, 0x65, 0x73, 0x22, 0x5f, 0x0a, 0x24, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x70, 0x57, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54,
    0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x47, 0x61, 0x6d, 0x65, 0x73, 0x12, 0x37, 0x0a, 0x0a, 0x6c,
    0x69, 0x76, 0x65, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54, 0x56,
    0x47, 0x61, 0x6d, 0x65, 0x53, 0x6d, 0x61, 0x6c, 0x6c, 0x52, 0x09, 0x6c, 0x69, 0x76, 0x65, 0x47,
    0x61, 0x6d, 0x65, 0x73, 0x22, 0x7f, 0x0a, 0x1f, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x54, 0x6f, 0x70, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49, 0x64,
    0x12, 0x2a, 0x0a, 0x11, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07,
    0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x74,
    0x65, 0x61, 0x6d, 0x49, 0x64, 0x22, 0x27, 0x0a, 0x25, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x54, 0x6f, 0x70, 0x4c, 0x65, 0x61, 0x67, 0x75, 0x65,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x27,
    0x0a, 0x25, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43,
    0x54, 0x6f, 0x70, 0x46, 0x72, 0x69, 0x65, 0x6e, 0x64, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x42, 0x0a, 0x23, 0x43, 0x4d, 0x73, 0x67, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73,
    0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1b,
    0x0a, 0x09, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x04, 0x52, 0x08, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x73, 0x22, 0x7b, 0x0a, 0x24, 0x43,
    0x4d, 0x73, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x73, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x34, 0x0a, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c,
    0x52, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x61, 0x73,
    0x74, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x09, 0x6c,
    0x61, 0x73, 0x74, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x22, 0x5e, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67,
    0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x70, 0x4c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x34, 0x0a, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x52,
    0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x22, 0x5e, 0x0a, 0x26, 0x43, 0x4d, 0x73, 0x67,
    0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x70, 0x46, 0x72, 0x69,
    0x65, 0x6e, 0x64, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x34, 0x0a, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44,
    0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x52,
    0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x22, 0xd2, 0x01, 0x0a, 0x1c, 0x43, 0x4d, 0x73,
    0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x54, 0x6f, 0x47, 0x43, 0x46, 0x69, 0x6e, 0x64, 0x54,
    0x6f, 0x70, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6c, 0x65, 0x61, 0x67,
    0x75, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x6c, 0x65, 0x61,
    0x67, 0x75, 0x65, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x69, 0x64,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49, 0x64, 0x12, 0x1b,
    0x0a, 0x09, 0x66, 0x72, 0x69, 0x65, 0x6e, 0x64, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x08, 0x66, 0x72, 0x69, 0x65, 0x6e, 0x64, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x66,
    0x72, 0x69, 0x65, 0x6e, 0x64, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x0a, 0x66, 0x72, 0x69, 0x65, 0x6e, 0x64, 0x4c, 0x69, 0x73, 0x74, 0x12, 0x1f, 0x0a, 0x0b,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0a, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x4c, 0x69, 0x73, 0x74, 0x22, 0xcd, 0x01,
    0x0a, 0x2a, 0x43, 0x4d, 0x73, 0x67, 0x47, 0x43, 0x54, 0x6f, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x46, 0x69, 0x6e, 0x64, 0x54, 0x6f, 0x70, 0x4c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1d, 0x0a, 0x0a,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6c,
    0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f,
    0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49,
    0x64, 0x12, 0x1b, 0x0a, 0x09, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x04,
    0x20, 0x03, 0x28, 0x0d, 0x52, 0x08, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x73, 0x12, 0x2d,
    0x0a, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x52, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x22, 0x33, 0x0a,
    0x16, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x65, 0x46, 0x72, 0x69,
    0x65, 0x6e, 0x64, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x74, 0x65, 0x61, 0x6d,
    0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6d,
    0x49, 0x64, 0x22, 0x47, 0x0a, 0x1e, 0x43, 0x4d, 0x73, 0x67, 0x53, 0x70, 0x65, 0x63, 0x74, 0x61,
    0x74, 0x65, 0x46, 0x72, 0x69, 0x65, 0x6e, 0x64, 0x47, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x25, 0x0a, 0x0e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73,
    0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x22, 0x9f, 0x09, 0x0a, 0x14,
    0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x69, 0x6e,
    0x69, 0x6d, 0x61, 0x6c, 0x12, 0x19, 0x0a, 0x08, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x12,
    0x1d, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x07, 0x52, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x1a,
    0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x44, 0x0a, 0x09, 0x67, 0x61,
    0x6d, 0x65, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x61, 0x6d, 0x65, 0x4d, 0x6f,
    0x64, 0x65, 0x3a, 0x12, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x47, 0x41, 0x4d, 0x45, 0x4d, 0x4f, 0x44,
    0x45, 0x5f, 0x4e, 0x4f, 0x4e, 0x45, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x4d, 0x6f, 0x64, 0x65,
    0x12, 0x3b, 0x0a, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x21, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54,
    0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x2e, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x52, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x3c, 0x0a,
    0x07, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x2e, 0x54, 0x6f, 0x75, 0x72, 0x6e,
    0x65, 0x79, 0x52, 0x07, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x12, 0x51, 0x0a, 0x0d, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x5f, 0x6f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x13, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x45, 0x4d, 0x61, 0x74, 0x63, 0x68,
    0x4f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x3a, 0x17, 0x6b, 0x5f, 0x45, 0x4d, 0x61, 0x74, 0x63,
    0x68, 0x4f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x5f, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x52, 0x0c, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x4f, 0x75, 0x74, 0x63, 0x6f, 0x6d, 0x65, 0x1a, 0xbf,
    0x01, 0x0a, 0x06, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x68, 0x65, 0x72, 0x6f,
    0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x68, 0x65, 0x72, 0x6f, 0x49,
    0x64, 0x12, 0x14, 0x0a, 0x05, 0x6b, 0x69, 0x6c, 0x6c, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x05, 0x6b, 0x69, 0x6c, 0x6c, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x65, 0x61, 0x74, 0x68,
    0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x64, 0x65, 0x61, 0x74, 0x68, 0x73, 0x12,
    0x18, 0x0a, 0x07, 0x61, 0x73, 0x73, 0x69, 0x73, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x07, 0x61, 0x73, 0x73, 0x69, 0x73, 0x74, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x74, 0x65,
    0x6d, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x12,
    0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x53, 0x6c, 0x6f, 0x74,
    0x1a, 0xda, 0x04, 0x0a, 0x07, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x12, 0x1b, 0x0a, 0x09,
    0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x08, 0x6c, 0x65, 0x61, 0x67, 0x75, 0x65, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x72,
    0x69, 0x65, 0x73, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a,
    0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65,
    0x72, 0x69, 0x65, 0x73, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0a, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x41, 0x0a, 0x1d, 0x77,
    0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f, 0x74,
    0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x1a, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e,
    0x65, 0x79, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x46,
    0x0a, 0x20, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65,
    0x79, 0x5f, 0x73, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x5f, 0x74, 0x72, 0x6f, 0x70, 0x68, 0x79, 0x5f,
    0x69, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x1c, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e,
    0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x53, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x54, 0x72,
    0x6f, 0x70, 0x68, 0x79, 0x49, 0x64, 0x12, 0x38, 0x0a, 0x18, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e,
    0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x5f, 0x64, 0x69, 0x76, 0x69, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x16, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e,
    0x64, 0x54, 0x6f, 0x75, 0x72, 0x6e, 0x65, 0x79, 0x44, 0x69, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e,
    0x12, 0x3d, 0x0a, 0x1b, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x5f, 0x74, 0x6f, 0x75, 0x72,
    0x6e, 0x65, 0x79, 0x5f, 0x73, 0x6b, 0x69, 0x6c, 0x6c, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x18, 0x77, 0x65, 0x65, 0x6b, 0x65, 0x6e, 0x64, 0x54, 0x6f,
    0x75, 0x72, 0x6e, 0x65, 0x79, 0x53, 0x6b, 0x69, 0x6c, 0x6c, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12,
    0x26, 0x0a, 0x0f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e,
    0x74, 0x54, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x2a, 0x0a, 0x11, 0x72, 0x61, 0x64, 0x69, 0x61,
    0x6e, 0x74, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x54, 0x65, 0x61, 0x6d, 0x4e,
    0x61, 0x6d, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x5f, 0x74,
    0x65, 0x61, 0x6d, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0f,
    0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x54, 0x65, 0x61, 0x6d, 0x4c, 0x6f, 0x67, 0x6f, 0x12,
    0x20, 0x0a, 0x0c, 0x64, 0x69, 0x72, 0x65, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x64, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x64, 0x69, 0x72, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x49,
    0x64, 0x12, 0x24, 0x0a, 0x0e, 0x64, 0x69, 0x72, 0x65, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x64, 0x69, 0x72, 0x65, 0x54,
    0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x64, 0x69, 0x72, 0x65, 0x5f,
    0x74, 0x65, 0x61, 0x6d, 0x5f, 0x6c, 0x6f, 0x67, 0x6f, 0x18, 0x07, 0x20, 0x01, 0x28, 0x06, 0x52,
    0x0c, 0x64, 0x69, 0x72, 0x65, 0x54, 0x65, 0x61, 0x6d, 0x4c, 0x6f, 0x67, 0x6f, 0x22, 0x9e, 0x02,
    0x0a, 0x17, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x79, 0x44, 0x6f, 0x77,
    0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x30, 0x0a, 0x05, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x43, 0x4d, 0x73, 0x67, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x69, 0x6e,
    0x69, 0x6d, 0x61, 0x6c, 0x52, 0x05, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x12, 0x14, 0x0a, 0x05, 0x74,
    0x69, 0x74, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x69, 0x74, 0x6c,
    0x65, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x61, 0x67, 0x73, 0x18,
    0x05, 0x20, 0x03, 0x28, 0x09, 0x52, 0x04, 0x74, 0x61, 0x67, 0x73, 0x12, 0x24, 0x0a, 0x0e, 0x65,
    0x78, 0x69, 0x73, 0x74, 0x73, 0x5f, 0x6f, 0x6e, 0x5f, 0x64, 0x69, 0x73, 0x6b, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x0c, 0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x4f, 0x6e, 0x44, 0x69, 0x73,
    0x6b, 0x1a, 0x4b, 0x0a, 0x09, 0x48, 0x69, 0x67, 0x68, 0x6c, 0x69, 0x67, 0x68, 0x74, 0x12, 0x1c,
    0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x20, 0x0a, 0x0b,
    0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xc4,
    0x01, 0x0a, 0x0d, 0x43, 0x4d, 0x73, 0x67, 0x57, 0x61, 0x74, 0x63, 0x68, 0x47, 0x61, 0x6d, 0x65,
    0x12, 0x25, 0x0a, 0x0e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x53, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52,
    0x0d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x30,
    0x0a, 0x14, 0x77, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73,
    0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x12, 0x77, 0x61,
    0x74, 0x63, 0x68, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64,
    0x12, 0x19, 0x0a, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x79, 0x49, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x07, 0x72, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0x15, 0x0a, 0x13, 0x43, 0x4d, 0x73, 0x67, 0x43, 0x61, 0x6e,
    0x63, 0x65, 0x6c, 0x57, 0x61, 0x74, 0x63, 0x68, 0x47, 0x61, 0x6d, 0x65, 0x22, 0xd6, 0x04, 0x0a,
    0x15, 0x43, 0x4d, 0x73, 0x67, 0x57, 0x61, 0x74, 0x63, 0x68, 0x47, 0x61, 0x6d, 0x65, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x60, 0x0a, 0x11, 0x77, 0x61, 0x74, 0x63, 0x68, 0x5f,
    0x67, 0x61, 0x6d, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x2b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x57, 0x61, 0x74,
    0x63, 0x68, 0x47, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x57,
    0x61, 0x74, 0x63, 0x68, 0x47, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x3a, 0x07,
    0x50, 0x45, 0x4e, 0x44, 0x49, 0x4e, 0x47, 0x52, 0x0f, 0x77, 0x61, 0x74, 0x63, 0x68, 0x47, 0x61,
    0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x31, 0x0a, 0x15, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x5f, 0x74, 0x76, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x61, 0x64, 0x64,
    0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54,
    0x76, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x41, 0x64, 0x64, 0x72, 0x12, 0x33, 0x0a, 0x16, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x74, 0x76, 0x5f, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65,
    0x5f, 0x61, 0x64, 0x64, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x13, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x54, 0x76, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x41, 0x64, 0x64, 0x72,
    0x12, 0x24, 0x0a, 0x0e, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x74, 0x76, 0x5f, 0x70, 0x6f,
    0x72, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65,
    0x54, 0x76, 0x50, 0x6f, 0x72, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x06, 0x52, 0x11, 0x67, 0x61, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53,
    0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x77, 0x61, 0x74, 0x63, 0x68, 0x5f,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x12, 0x77, 0x61, 0x74, 0x63, 0x68, 0x53, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x53, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x12, 0x3c, 0x0a, 0x1b, 0x77, 0x61, 0x74, 0x63,
    0x68, 0x5f, 0x74, 0x76, 0x5f, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x72,
    0x65, 0x74, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x06, 0x52, 0x17, 0x77,
    0x61, 0x74, 0x63, 0x68, 0x54, 0x76, 0x55, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x53, 0x65, 0x63, 0x72,
    0x65, 0x74, 0x43, 0x6f, 0x64, 0x65, 0x22, 0xac, 0x01, 0x0a, 0x0f, 0x57, 0x61, 0x74, 0x63, 0x68,
    0x47, 0x61, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0b, 0x0a, 0x07, 0x50, 0x45,
    0x4e, 0x44, 0x49, 0x4e, 0x47, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x52, 0x45, 0x41, 0x44, 0x59,
    0x10, 0x01, 0x12, 0x16, 0x0a, 0x12, 0x47, 0x41, 0x4d, 0x45, 0x53, 0x45, 0x52, 0x56, 0x45, 0x52,
    0x4e, 0x4f, 0x54, 0x46, 0x4f, 0x55, 0x4e, 0x44, 0x10, 0x02, 0x12, 0x0f, 0x0a, 0x0b, 0x55, 0x4e,
    0x41, 0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45, 0x10, 0x03, 0x12, 0x0d, 0x0a, 0x09, 0x43,
    0x41, 0x4e, 0x43, 0x45, 0x4c, 0x4c, 0x45, 0x44, 0x10, 0x04, 0x12, 0x17, 0x0a, 0x13, 0x49, 0x4e,
    0x43, 0x4f, 0x4d, 0x50, 0x41, 0x54, 0x49, 0x42, 0x4c, 0x45, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f,
    0x4e, 0x10, 0x05, 0x12, 0x1d, 0x0a, 0x19, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4e, 0x47, 0x4c, 0x45,
    0x41, 0x47, 0x55, 0x45, 0x53, 0x55, 0x42, 0x53, 0x43, 0x52, 0x49, 0x50, 0x54, 0x49, 0x4f, 0x4e,
    0x10, 0x06, 0x12, 0x11, 0x0a, 0x0d, 0x4c, 0x4f, 0x42, 0x42, 0x59, 0x4e, 0x4f, 0x54, 0x46, 0x4f,
    0x55, 0x4e, 0x44, 0x10, 0x07, 0x22, 0x50, 0x0a, 0x1e, 0x43, 0x4d, 0x73, 0x67, 0x50, 0x61, 0x72,
    0x74, 0x79, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x57, 0x61, 0x74, 0x63, 0x68, 0x47, 0x61, 0x6d,
    0x65, 0x50, 0x72, 0x6f, 0x6d, 0x70, 0x74, 0x12, 0x2e, 0x0a, 0x13, 0x67, 0x61, 0x6d, 0x65, 0x5f,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x06, 0x52, 0x11, 0x67, 0x61, 0x6d, 0x65, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x53, 0x74, 0x65, 0x61, 0x6d, 0x69, 0x64, 0x22, 0x90, 0x03, 0x0a, 0x14, 0x43, 0x44, 0x4f, 0x54,
    0x41, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f,
    0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x64, 0x12,
    0x26, 0x0a, 0x0f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x65, 0x61, 0x6d, 0x5f,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0d, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x53, 0x74, 0x65, 0x61, 0x6d, 0x49, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6c, 0x69, 0x76, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x6c, 0x69, 0x76, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x74,
    0x65, 0x61, 0x6d, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x72, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x74, 0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65,
    0x52, 0x61, 0x64, 0x69, 0x61, 0x6e, 0x74, 0x12, 0x24, 0x0a, 0x0e, 0x74, 0x65, 0x61, 0x6d, 0x5f,
    0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x64, 0x69, 0x72, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0c, 0x74, 0x65, 0x61, 0x6d, 0x4e, 0x61, 0x6d, 0x65, 0x44, 0x69, 0x72, 0x65, 0x12, 0x1d, 0x0a,
    0x0a, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x09, 0x73, 0x74, 0x61, 0x67, 0x65, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b,
    0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0a, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x47, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a,
    0x0b, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0a, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x54, 0x79, 0x70, 0x65, 0x12, 0x40,
    0x0a, 0x1c, 0x75, 0x70, 0x63, 0x6f, 0x6d, 0x69, 0x6e, 0x67, 0x5f, 0x62, 0x72, 0x6f, 0x61, 0x64,
    0x63, 0x61, 0x73, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x09,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x1a, 0x75, 0x70, 0x63, 0x6f, 0x6d, 0x69, 0x6e, 0x67, 0x42, 0x72,
    0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x12, 0x28, 0x0a, 0x10, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x5f, 0x6c, 0x69, 0x76, 0x65, 0x5f, 0x76,
    0x69, 0x64, 0x65, 0x6f, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x61, 0x6c, 0x6c, 0x6f,
    0x77, 0x4c, 0x69, 0x76, 0x65, 0x56, 0x69, 0x64, 0x65, 0x6f, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01,
    0x00, 0x4a, 0xda, 0x50, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xdb, 0x01, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08,
    0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x20, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x05, 0x07, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x07, 0x00, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x07, 0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x08, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x08, 0x00, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x08, 0x07, 0x1a, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x08, 0x07, 0x1a, 0x0a, 0x0e, 0x0a,
    0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x07, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x08, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x0a, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x0a, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x0b, 0x08, 0x0e,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x10, 0x16, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x10, 0x2f, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x10, 0x18, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x19, 0x1f, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x20, 0x2a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x2d, 0x2e, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x10, 0x2c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0d, 0x19, 0x1f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x20, 0x27, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x10, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x10, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10,
    0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x11, 0x08, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x11, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x11, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x12, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x12,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x12, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x18, 0x27, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x12, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x13, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x13, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x13, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x13, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x13, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x14, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x14, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x14, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x14, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x15, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x15, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x15, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x15, 0x18, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x15, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x06, 0x12, 0x03, 0x16, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x16, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12,
    0x03, 0x16, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x16,
    0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x16, 0x23, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x17, 0x08, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x17, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x17, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x17, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x18,
    0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x18, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x18, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x18, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x18, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x09, 0x12, 0x03, 0x19, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04,
    0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03,
    0x19, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x19, 0x18,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x19, 0x24, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x1a, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0a, 0x05, 0x12, 0x03, 0x1a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a,
    0x01, 0x12, 0x03, 0x1a, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12,
    0x03, 0x1a, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x1b, 0x08,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x1b, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x1b, 0x2c, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x0c, 0x12, 0x03, 0x1c, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12,
    0x03, 0x1c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x1c,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x1c, 0x18, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x1c, 0x29, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x1d, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x1d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0d, 0x05, 0x12, 0x03, 0x1d, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01,
    0x12, 0x03, 0x1d, 0x19, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03,
    0x1d, 0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x1e, 0x08, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05, 0x12, 0x03, 0x1e, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03, 0x1e, 0x19, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0e, 0x03, 0x12, 0x03, 0x1e, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f,
    0x12, 0x03, 0x1f, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x03,
    0x1f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x1f, 0x11,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x1f, 0x18, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x1f, 0x25, 0x27, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03, 0x20, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x10, 0x04, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10,
    0x05, 0x12, 0x03, 0x20, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x01, 0x12,
    0x03, 0x20, 0x17, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x03, 0x12, 0x03, 0x20,
    0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x11, 0x12, 0x03, 0x21, 0x08, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x04, 0x12, 0x03, 0x21, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x11, 0x05, 0x12, 0x03, 0x21, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x21, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x11, 0x03, 0x12, 0x03, 0x21, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x12, 0x12,
    0x03, 0x22, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x04, 0x12, 0x03, 0x22,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x05, 0x12, 0x03, 0x22, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x01, 0x12, 0x03, 0x22, 0x18, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x03, 0x12, 0x03, 0x22, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x13, 0x12, 0x03, 0x23, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x13, 0x04, 0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x05,
    0x12, 0x03, 0x23, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03,
    0x23, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x03, 0x12, 0x03, 0x23, 0x25,
    0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x14, 0x12, 0x03, 0x24, 0x08, 0x38, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x14, 0x06, 0x12, 0x03, 0x24, 0x11, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x14, 0x01, 0x12, 0x03, 0x24, 0x2b, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14,
    0x03, 0x12, 0x03, 0x24, 0x35, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x15, 0x12, 0x03,
    0x25, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x04, 0x12, 0x03, 0x25, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x05, 0x12, 0x03, 0x25, 0x11, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x01, 0x12, 0x03, 0x25, 0x19, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x15, 0x03, 0x12, 0x03, 0x25, 0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x16, 0x12, 0x03, 0x26, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16,
    0x04, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x05, 0x12,
    0x03, 0x26, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x01, 0x12, 0x03, 0x26,
    0x18, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x03, 0x12, 0x03, 0x26, 0x38, 0x3a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x17, 0x12, 0x03, 0x27, 0x08, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x17, 0x04, 0x12, 0x03, 0x27, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x17, 0x05, 0x12, 0x03, 0x27, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x17, 0x01, 0x12, 0x03, 0x27, 0x18, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x17, 0x03,
    0x12, 0x03, 0x27, 0x33, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x18, 0x12, 0x03, 0x28,
    0x08, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x04, 0x12, 0x03, 0x28, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x05, 0x12, 0x03, 0x28, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x01, 0x12, 0x03, 0x28, 0x18, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x18, 0x03, 0x12, 0x03, 0x28, 0x36, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x19, 0x12, 0x03, 0x29, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x19, 0x04,
    0x12, 0x03, 0x29, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x19, 0x05, 0x12, 0x03,
    0x29, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x19, 0x01, 0x12, 0x03, 0x29, 0x18,
    0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x19, 0x03, 0x12, 0x03, 0x29, 0x38, 0x3a, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2c, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x2d, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x18, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x2e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x2e, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x08, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2f, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x2f, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03,
    0x30, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x30, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x30, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x18, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x30, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x04, 0x12, 0x03, 0x31, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x31, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x31,
    0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x31, 0x2a, 0x2b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x32, 0x08, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x32, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x32, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x32, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x35, 0x00, 0x3e,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x35, 0x08, 0x32, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x36, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x36, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x36, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x36, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x37, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x37, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x37, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x38, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x38,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x38, 0x11, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x38, 0x18, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x38, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x39, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x39, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x39, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x39, 0x25,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3a, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x3a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x3a, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x3a, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x3a, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x3b, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x3b, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x3b, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x3b, 0x18, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x3b, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x3c, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06,
    0x04, 0x12, 0x03, 0x3c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x06, 0x12,
    0x03, 0x3c, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x3c,
    0x24, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x03, 0x12, 0x03, 0x3c, 0x30, 0x31,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x3d, 0x08, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03, 0x3d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x3d, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x07, 0x01, 0x12, 0x03, 0x3d, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03,
    0x12, 0x03, 0x3d, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x40, 0x00, 0x42,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x40, 0x08, 0x2c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x41, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x41, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x41, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x41, 0x24, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41,
    0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x44, 0x00, 0x48, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x44, 0x08, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x45, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x45, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45, 0x18,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x45, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x46, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x46, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x46, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x46, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x46, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x47, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x47, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x47, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x47, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x47, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x4a, 0x00, 0x4b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x4a, 0x08,
    0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x4d, 0x00, 0x4e, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x50, 0x00, 0x52, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x50, 0x08,
    0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x51, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x51, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x51, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x51, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x51, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x54, 0x00,
    0x57, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x54, 0x08, 0x2c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x55, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x55, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x55, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x55, 0x26, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x55, 0x30, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x56, 0x08, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x56, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x56, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x56, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x56, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04,
    0x59, 0x00, 0x5b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x59, 0x08, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x08, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5a, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x5a, 0x26, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x5a, 0x30, 0x31, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x5d, 0x00, 0x5f,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x2e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x5e, 0x08, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x5e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x5e, 0x11, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x5e, 0x26, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5e,
    0x30, 0x31, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x61, 0x00, 0x68, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x61, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b,
    0x02, 0x00, 0x12, 0x03, 0x62, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x62, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x62, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x18,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x62, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x63, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x63, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x63, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x63, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x63, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x64, 0x08,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x64, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x64, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x64, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x64, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x03, 0x12, 0x03, 0x65, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x65, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x05, 0x12, 0x03, 0x65,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x03, 0x65, 0x18, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x03, 0x65, 0x24, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x03, 0x66, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x04, 0x04, 0x12, 0x03, 0x66, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x66, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x66, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x66, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x05, 0x12, 0x03, 0x67, 0x08, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x04, 0x12, 0x03, 0x67, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x05, 0x12, 0x03, 0x67, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x05, 0x01, 0x12, 0x03, 0x67, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x67, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04,
    0x6a, 0x00, 0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x32,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x08, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x6b, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x6b, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x6c,
    0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6c, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6c, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6c, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x02, 0x12, 0x03, 0x6d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x6d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x6d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6d, 0x18,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6d, 0x22, 0x23, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x03, 0x6e, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x6e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x6e, 0x18, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x6e, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x03, 0x6f, 0x08,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x04, 0x12, 0x03, 0x6f, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x06, 0x12, 0x03, 0x6f, 0x11, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x04, 0x01, 0x12, 0x03, 0x6f, 0x1f, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x04, 0x03, 0x12, 0x03, 0x6f, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12,
    0x04, 0x72, 0x00, 0x74, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x72, 0x08,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x73, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x73, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03, 0x73, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x73, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x73, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x76, 0x00,
    0x78, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x76, 0x08, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x77, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x77, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x77, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x77, 0x19, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x77, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x05, 0x7a, 0x00, 0x9c, 0x01, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x7a, 0x08, 0x1c, 0x0a, 0x0d, 0x0a, 0x04,
    0x04, 0x0f, 0x03, 0x00, 0x12, 0x05, 0x7b, 0x08, 0x83, 0x01, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x03, 0x00, 0x01, 0x12, 0x03, 0x7b, 0x10, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x7c, 0x10, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x7c, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x7c, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x7c, 0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x7c, 0x2d, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x7d, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x7d, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x7d, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x7d, 0x20, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x7d, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x7e, 0x10, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x7e, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x7e, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x7e, 0x20, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x7e, 0x28, 0x29, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x7f, 0x10, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x7f, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x7f, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x7f, 0x20, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x7f, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x04, 0x12,
    0x04, 0x80, 0x01, 0x10, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x80, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x04,
    0x05, 0x12, 0x04, 0x80, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x80, 0x01, 0x20, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x80, 0x01, 0x2a, 0x2b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03,
    0x00, 0x02, 0x05, 0x12, 0x04, 0x81, 0x01, 0x10, 0x2a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03,
    0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x81, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f,
    0x03, 0x00, 0x02, 0x05, 0x05, 0x12, 0x04, 0x81, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0f, 0x03, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0x81, 0x01, 0x20, 0x25, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0f, 0x03, 0x00, 0x02, 0x05, 0x03, 0x12, 0x04, 0x81, 0x01, 0x28, 0x29, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x06, 0x12, 0x04, 0x82, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0x82, 0x01, 0x10, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x06, 0x05, 0x12, 0x04, 0x82, 0x01, 0x19, 0x1f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0x82, 0x01, 0x20, 0x2b,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0x82, 0x01, 0x2e,
    0x2f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0f, 0x03, 0x01, 0x12, 0x06, 0x85, 0x01, 0x08, 0x93, 0x01,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x03, 0x01, 0x01, 0x12, 0x04, 0x85, 0x01, 0x10, 0x17,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x10, 0x2e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0x86, 0x01,
    0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86,
    0x01, 0x20, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x86, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x01, 0x12, 0x04,
    0x87, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x87, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x87, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x87, 0x01, 0x20, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x87, 0x01, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01,
    0x02, 0x02, 0x12, 0x04, 0x88, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x88, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x04, 0x88, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f,
    0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0x88, 0x01, 0x20, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0f, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x04, 0x88, 0x01, 0x2e, 0x2f, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x0f, 0x03, 0x01, 0x02, 0x03, 0x12, 0x04, 0x89, 0x01, 0x10, 0x43, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0f, 0x03, 0x01, 0x02, 0x03, 0x04, 0x12, 0x04, 0x89, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x03, 0x05, 0x12, 0x04, 0x89, 0x01, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x03, 0x01, 0x12, 0x04, 0x89, 0x01, 0x20, 0x3d, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x03, 0x03, 0x12, 0x04, 0x89, 0x01, 0x40, 0x42,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x10, 0x46,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x04, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x04, 0x05, 0x12, 0x04, 0x8a, 0x01,
    0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8a,
    0x01, 0x20, 0x40, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x8a, 0x01, 0x43, 0x45, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x05, 0x12, 0x04,
    0x8b, 0x01, 0x10, 0x3e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x05, 0x04, 0x12,
    0x04, 0x8b, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x05, 0x05,
    0x12, 0x04, 0x8b, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x05,
    0x01, 0x12, 0x04, 0x8b, 0x01, 0x20, 0x38, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02,
    0x05, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x3b, 0x3d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01,
    0x02, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x41, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03,
    0x01, 0x02, 0x06, 0x05, 0x12, 0x04, 0x8c, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f,
    0x03, 0x01, 0x02, 0x06, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x20, 0x3b, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0f, 0x03, 0x01, 0x02, 0x06, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x3e, 0x40, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x0f, 0x03, 0x01, 0x02, 0x07, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x34, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0f, 0x03, 0x01, 0x02, 0x07, 0x04, 0x12, 0x04, 0x8d, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x07, 0x05, 0x12, 0x04, 0x8d, 0x01, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x07, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x20, 0x2f, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x07, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x32, 0x33,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x08, 0x12, 0x04, 0x8e, 0x01, 0x10, 0x36,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x08, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x08, 0x05, 0x12, 0x04, 0x8e, 0x01,
    0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x08, 0x01, 0x12, 0x04, 0x8e,
    0x01, 0x20, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x08, 0x03, 0x12, 0x04,
    0x8e, 0x01, 0x34, 0x35, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x09, 0x12, 0x04,
    0x8f, 0x01, 0x10, 0x37, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x09, 0x04, 0x12,
    0x04, 0x8f, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x09, 0x05,
    0x12, 0x04, 0x8f, 0x01, 0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x09,
    0x01, 0x12, 0x04, 0x8f, 0x01, 0x21, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02,
    0x09, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x35, 0x36, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01,
    0x02, 0x0a, 0x12, 0x04, 0x90, 0x01, 0x10, 0x31, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01,
    0x02, 0x0a, 0x04, 0x12, 0x04, 0x90, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03,
    0x01, 0x02, 0x0a, 0x05, 0x12, 0x04, 0x90, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f,
    0x03, 0x01, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x90, 0x01, 0x20, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0f, 0x03, 0x01, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x90, 0x01, 0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x0f, 0x03, 0x01, 0x02, 0x0b, 0x12, 0x04, 0x91, 0x01, 0x10, 0x33, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0f, 0x03, 0x01, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x91, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x0b, 0x05, 0x12, 0x04, 0x91, 0x01, 0x19, 0x1f, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x91, 0x01, 0x20, 0x2e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x91, 0x01, 0x31, 0x32,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x0c, 0x12, 0x04, 0x92, 0x01, 0x10, 0x34,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x92, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x0c, 0x05, 0x12, 0x04, 0x92, 0x01,
    0x19, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x92,
    0x01, 0x21, 0x2f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0f, 0x03, 0x01, 0x02, 0x0c, 0x03, 0x12, 0x04,
    0x92, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0x95, 0x01,
    0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0x95, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x95, 0x01, 0x11, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x01, 0x18, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x95, 0x01, 0x23, 0x24, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0x96, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0x96, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x96, 0x01, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x96, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02,
    0x12, 0x04, 0x97, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x97, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x05, 0x12, 0x04,
    0x97, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01, 0x12, 0x04, 0x97,
    0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x04, 0x97, 0x01,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x04, 0x98, 0x01, 0x08, 0x4c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x04, 0x12, 0x04, 0x98, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x06, 0x12, 0x04, 0x98, 0x01, 0x11, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12, 0x04, 0x98, 0x01, 0x1f, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x04, 0x98, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x03, 0x08, 0x12, 0x04, 0x98, 0x01, 0x2d, 0x4b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x03, 0x07, 0x12, 0x04, 0x98, 0x01, 0x38, 0x4a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x99, 0x01, 0x11, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x99, 0x01, 0x2d, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x99, 0x01, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x05, 0x12, 0x04, 0x9a, 0x01,
    0x08, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x06, 0x12, 0x04, 0x9a, 0x01, 0x11, 0x2d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x2e, 0x35, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x38, 0x39, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x55, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x06, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x06, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x06, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x1f, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x06, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x2f, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06,
    0x08, 0x12, 0x04, 0x9b, 0x01, 0x31, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x07,
    0x12, 0x04, 0x9b, 0x01, 0x3c, 0x53, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x9e, 0x01,
    0x00, 0xaa, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x08,
    0x1f, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x10, 0x03, 0x00, 0x12, 0x06, 0x9f, 0x01, 0x08, 0xa2, 0x01,
    0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x03, 0x00, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x10, 0x19,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xa0, 0x01, 0x10, 0x2e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa0, 0x01, 0x10,
    0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa0, 0x01,
    0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa0,
    0x01, 0x20, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xa0, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04,
    0xa1, 0x01, 0x10, 0x30, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xa1, 0x01, 0x10, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xa1, 0x01, 0x19, 0x1f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xa1, 0x01, 0x20, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x10, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00,
    0x12, 0x04, 0xa4, 0x01, 0x08, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xa4, 0x01, 0x11, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4,
    0x01, 0x26, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x01,
    0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa5, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xa6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xa6, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xa6, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04,
    0xa7, 0x01, 0x08, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa7,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa7, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x18,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x1f, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x04, 0x05, 0x12, 0x04, 0xa8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x18, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x05, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05,
    0x04, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x05,
    0x12, 0x04, 0xa9, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xa9, 0x01, 0x16, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xa9, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xac, 0x01, 0x00, 0xb2,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xac, 0x01, 0x08, 0x15, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xad, 0x01, 0x08, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xad, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xad, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xad, 0x01, 0x19, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xad, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x01, 0x12, 0x04, 0xae, 0x01, 0x08, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xae, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xae, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xae, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xae,
    0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xaf, 0x01, 0x08,
    0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0xaf, 0x01, 0x11, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x19, 0x2d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x30, 0x31, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x03, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xb0, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xb0, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x04, 0x12,
    0x04, 0xb1, 0x01, 0x08, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xb1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x05, 0x12, 0x04, 0xb1,
    0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb1, 0x01,
    0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x03, 0x12, 0x04, 0xb1, 0x01, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xb4, 0x01, 0x00, 0xb5, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x13, 0x12, 0x06, 0xb7, 0x01, 0x00, 0xca, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13,
    0x01, 0x12, 0x04, 0xb7, 0x01, 0x08, 0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x13, 0x04, 0x00, 0x12,
    0x06, 0xb8, 0x01, 0x08, 0xc1, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x04, 0x00, 0x01,
    0x12, 0x04, 0xb8, 0x01, 0x0d, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x04, 0xb9, 0x01, 0x10, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xb9, 0x01, 0x10, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0xb9, 0x01, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x04, 0xba, 0x01, 0x10, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xba, 0x01, 0x10, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xba, 0x01, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xbb, 0x01, 0x10, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x13, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xbb, 0x01, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06,
    0x04, 0x13, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x10, 0x20, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x13, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x10, 0x1b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xbc, 0x01, 0x1e, 0x1f, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x13, 0x04, 0x00, 0x02, 0x04, 0x12, 0x04, 0xbd, 0x01, 0x10, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x10, 0x19, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x04, 0xbd, 0x01, 0x1c, 0x1d,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13, 0x04, 0x00, 0x02, 0x05, 0x12, 0x04, 0xbe, 0x01, 0x10, 0x28,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x10,
    0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xbe, 0x01,
    0x26, 0x27, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13, 0x04, 0x00, 0x02, 0x06, 0x12, 0x04, 0xbf, 0x01,
    0x10, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xbf,
    0x01, 0x10, 0x29, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04,
    0xbf, 0x01, 0x2c, 0x2d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x13, 0x04, 0x00, 0x02, 0x07, 0x12, 0x04,
    0xc0, 0x01, 0x10, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xc0, 0x01, 0x10, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x13, 0x04, 0x00, 0x02, 0x07, 0x02,
    0x12, 0x04, 0xc0, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04,
    0xc3, 0x01, 0x08, 0x61, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc3,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc3, 0x01,
    0x11, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x37,
    0x48, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc3, 0x01, 0x4b, 0x4c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x08, 0x12, 0x04, 0xc3, 0x01, 0x4d, 0x60, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x07, 0x12, 0x04, 0xc3, 0x01, 0x58, 0x5f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x18, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02,
    0x12, 0x04, 0xc5, 0x01, 0x08, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xc5, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xc5, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc5,
    0x01, 0x18, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc5, 0x01,
    0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x13, 0x02, 0x04, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xc7, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xc7, 0x01, 0x19, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xc7, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x05, 0x12, 0x04,
    0xc8, 0x01, 0x08, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x04, 0x12, 0x04, 0xc8,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x05, 0x12, 0x04, 0xc8, 0x01,
    0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x19,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x30, 0x31,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x06, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x39, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x04, 0x12, 0x04, 0xc9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x06, 0x05, 0x12, 0x04, 0xc9, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x06, 0x01, 0x12, 0x04, 0xc9, 0x01, 0x19, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x06, 0x03, 0x12, 0x04, 0xc9, 0x01, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14,
    0x12, 0x06, 0xcc, 0x01, 0x00, 0xce, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12,
    0x04, 0xcc, 0x01, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xcd,
    0x01, 0x08, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcd, 0x01,
    0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcd, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x19, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcd, 0x01, 0x2f, 0x30, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xd0, 0x01, 0x00, 0xdb, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x00, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd1, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xd1, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd1, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd1, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x01,
    0x08, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd2, 0x01, 0x11, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x19, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x2b, 0x2c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x01, 0x08, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd3, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd3, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03,
    0x12, 0x04, 0xd4, 0x01, 0x08, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xd4, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x05, 0x12, 0x04,
    0xd4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd4,
    0x01, 0x18, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd4, 0x01,
    0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x08, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd5, 0x01, 0x11, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x18, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x15, 0x02, 0x05, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x05, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x05, 0x05, 0x12, 0x04, 0xd6, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05,
    0x01, 0x12, 0x04, 0xd6, 0x01, 0x18, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x03,
    0x12, 0x04, 0xd6, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x06, 0x12, 0x04,
    0xd7, 0x01, 0x08, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x04, 0x12, 0x04, 0xd7,
    0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x05, 0x12, 0x04, 0xd7, 0x01,
    0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x18,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x07, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x07, 0x05, 0x12, 0x04, 0xd8, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x07, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x18, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x07, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x08, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08,
    0x04, 0x12, 0x04, 0xd9, 0x01, 0x08, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x05,
    0x12, 0x04, 0xd9, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x01, 0x12,
    0x04, 0xd9, 0x01, 0x18, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x03, 0x12, 0x04,
    0xd9, 0x01, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x09, 0x12, 0x04, 0xda, 0x01,
    0x08, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x04, 0x12, 0x04, 0xda, 0x01, 0x08,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x05, 0x12, 0x04, 0xda, 0x01, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x01, 0x12, 0x04, 0xda, 0x01, 0x16, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x03, 0x12, 0x04, 0xda, 0x01, 0x29, 0x2b,
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
