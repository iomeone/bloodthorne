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
pub struct CDOTAModifierBuffTableEntry {
    // message fields
    entry_type: ::std::option::Option<DOTA_MODIFIER_ENTRY_TYPE>,
    parent: ::std::option::Option<i32>,
    index: ::std::option::Option<i32>,
    serial_num: ::std::option::Option<i32>,
    modifier_class: ::std::option::Option<i32>,
    ability_level: ::std::option::Option<i32>,
    stack_count: ::std::option::Option<i32>,
    creation_time: ::std::option::Option<f32>,
    duration: ::std::option::Option<f32>,
    caster: ::std::option::Option<i32>,
    ability: ::std::option::Option<i32>,
    armor: ::std::option::Option<i32>,
    fade_time: ::std::option::Option<f32>,
    subtle: ::std::option::Option<bool>,
    channel_time: ::std::option::Option<f32>,
    v_start: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    v_end: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    portal_loop_appear: ::protobuf::SingularField<::std::string::String>,
    portal_loop_disappear: ::protobuf::SingularField<::std::string::String>,
    hero_loop_appear: ::protobuf::SingularField<::std::string::String>,
    hero_loop_disappear: ::protobuf::SingularField<::std::string::String>,
    movement_speed: ::std::option::Option<i32>,
    aura: ::std::option::Option<bool>,
    activity: ::std::option::Option<i32>,
    damage: ::std::option::Option<i32>,
    range: ::std::option::Option<i32>,
    dd_modifier_index: ::std::option::Option<i32>,
    dd_ability_index: ::std::option::Option<i32>,
    illusion_label: ::protobuf::SingularField<::std::string::String>,
    active: ::std::option::Option<bool>,
    player_ids: ::protobuf::SingularField<::std::string::String>,
    lua_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAModifierBuffTableEntry {}

impl CDOTAModifierBuffTableEntry {
    pub fn new() -> CDOTAModifierBuffTableEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAModifierBuffTableEntry {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAModifierBuffTableEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAModifierBuffTableEntry,
        };
        unsafe {
            instance.get(CDOTAModifierBuffTableEntry::new)
        }
    }

    // required .dota.DOTA_MODIFIER_ENTRY_TYPE entry_type = 1;

    pub fn clear_entry_type(&mut self) {
        self.entry_type = ::std::option::Option::None;
    }

    pub fn has_entry_type(&self) -> bool {
        self.entry_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entry_type(&mut self, v: DOTA_MODIFIER_ENTRY_TYPE) {
        self.entry_type = ::std::option::Option::Some(v);
    }

    pub fn get_entry_type(&self) -> DOTA_MODIFIER_ENTRY_TYPE {
        self.entry_type.unwrap_or(DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_ACTIVE)
    }

    fn get_entry_type_for_reflect(&self) -> &::std::option::Option<DOTA_MODIFIER_ENTRY_TYPE> {
        &self.entry_type
    }

    fn mut_entry_type_for_reflect(&mut self) -> &mut ::std::option::Option<DOTA_MODIFIER_ENTRY_TYPE> {
        &mut self.entry_type
    }

    // required int32 parent = 2;

    pub fn clear_parent(&mut self) {
        self.parent = ::std::option::Option::None;
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent(&mut self, v: i32) {
        self.parent = ::std::option::Option::Some(v);
    }

    pub fn get_parent(&self) -> i32 {
        self.parent.unwrap_or(0)
    }

    fn get_parent_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.parent
    }

    fn mut_parent_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.parent
    }

    // required int32 index = 3;

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

    // required int32 serial_num = 4;

    pub fn clear_serial_num(&mut self) {
        self.serial_num = ::std::option::Option::None;
    }

    pub fn has_serial_num(&self) -> bool {
        self.serial_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serial_num(&mut self, v: i32) {
        self.serial_num = ::std::option::Option::Some(v);
    }

    pub fn get_serial_num(&self) -> i32 {
        self.serial_num.unwrap_or(0)
    }

    fn get_serial_num_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.serial_num
    }

    fn mut_serial_num_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.serial_num
    }

    // optional int32 modifier_class = 5;

    pub fn clear_modifier_class(&mut self) {
        self.modifier_class = ::std::option::Option::None;
    }

    pub fn has_modifier_class(&self) -> bool {
        self.modifier_class.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modifier_class(&mut self, v: i32) {
        self.modifier_class = ::std::option::Option::Some(v);
    }

    pub fn get_modifier_class(&self) -> i32 {
        self.modifier_class.unwrap_or(0)
    }

    fn get_modifier_class_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.modifier_class
    }

    fn mut_modifier_class_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.modifier_class
    }

    // optional int32 ability_level = 6;

    pub fn clear_ability_level(&mut self) {
        self.ability_level = ::std::option::Option::None;
    }

    pub fn has_ability_level(&self) -> bool {
        self.ability_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_level(&mut self, v: i32) {
        self.ability_level = ::std::option::Option::Some(v);
    }

    pub fn get_ability_level(&self) -> i32 {
        self.ability_level.unwrap_or(0)
    }

    fn get_ability_level_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ability_level
    }

    fn mut_ability_level_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ability_level
    }

    // optional int32 stack_count = 7;

    pub fn clear_stack_count(&mut self) {
        self.stack_count = ::std::option::Option::None;
    }

    pub fn has_stack_count(&self) -> bool {
        self.stack_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stack_count(&mut self, v: i32) {
        self.stack_count = ::std::option::Option::Some(v);
    }

    pub fn get_stack_count(&self) -> i32 {
        self.stack_count.unwrap_or(0)
    }

    fn get_stack_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.stack_count
    }

    fn mut_stack_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.stack_count
    }

    // optional float creation_time = 8;

    pub fn clear_creation_time(&mut self) {
        self.creation_time = ::std::option::Option::None;
    }

    pub fn has_creation_time(&self) -> bool {
        self.creation_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creation_time(&mut self, v: f32) {
        self.creation_time = ::std::option::Option::Some(v);
    }

    pub fn get_creation_time(&self) -> f32 {
        self.creation_time.unwrap_or(0.)
    }

    fn get_creation_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.creation_time
    }

    fn mut_creation_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.creation_time
    }

    // optional float duration = 9;

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
        self.duration.unwrap_or(-1f32)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }

    // optional int32 caster = 10;

    pub fn clear_caster(&mut self) {
        self.caster = ::std::option::Option::None;
    }

    pub fn has_caster(&self) -> bool {
        self.caster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster(&mut self, v: i32) {
        self.caster = ::std::option::Option::Some(v);
    }

    pub fn get_caster(&self) -> i32 {
        self.caster.unwrap_or(0)
    }

    fn get_caster_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.caster
    }

    fn mut_caster_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.caster
    }

    // optional int32 ability = 11;

    pub fn clear_ability(&mut self) {
        self.ability = ::std::option::Option::None;
    }

    pub fn has_ability(&self) -> bool {
        self.ability.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability(&mut self, v: i32) {
        self.ability = ::std::option::Option::Some(v);
    }

    pub fn get_ability(&self) -> i32 {
        self.ability.unwrap_or(0)
    }

    fn get_ability_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ability
    }

    fn mut_ability_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ability
    }

    // optional int32 armor = 12;

    pub fn clear_armor(&mut self) {
        self.armor = ::std::option::Option::None;
    }

    pub fn has_armor(&self) -> bool {
        self.armor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_armor(&mut self, v: i32) {
        self.armor = ::std::option::Option::Some(v);
    }

    pub fn get_armor(&self) -> i32 {
        self.armor.unwrap_or(0)
    }

    fn get_armor_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.armor
    }

    fn mut_armor_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.armor
    }

    // optional float fade_time = 13;

    pub fn clear_fade_time(&mut self) {
        self.fade_time = ::std::option::Option::None;
    }

    pub fn has_fade_time(&self) -> bool {
        self.fade_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_time(&mut self, v: f32) {
        self.fade_time = ::std::option::Option::Some(v);
    }

    pub fn get_fade_time(&self) -> f32 {
        self.fade_time.unwrap_or(0.)
    }

    fn get_fade_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fade_time
    }

    fn mut_fade_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fade_time
    }

    // optional bool subtle = 14;

    pub fn clear_subtle(&mut self) {
        self.subtle = ::std::option::Option::None;
    }

    pub fn has_subtle(&self) -> bool {
        self.subtle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subtle(&mut self, v: bool) {
        self.subtle = ::std::option::Option::Some(v);
    }

    pub fn get_subtle(&self) -> bool {
        self.subtle.unwrap_or(false)
    }

    fn get_subtle_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.subtle
    }

    fn mut_subtle_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.subtle
    }

    // optional float channel_time = 15;

    pub fn clear_channel_time(&mut self) {
        self.channel_time = ::std::option::Option::None;
    }

    pub fn has_channel_time(&self) -> bool {
        self.channel_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel_time(&mut self, v: f32) {
        self.channel_time = ::std::option::Option::Some(v);
    }

    pub fn get_channel_time(&self) -> f32 {
        self.channel_time.unwrap_or(0.)
    }

    fn get_channel_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.channel_time
    }

    fn mut_channel_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.channel_time
    }

    // optional .dota.CMsgVector v_start = 16;

    pub fn clear_v_start(&mut self) {
        self.v_start.clear();
    }

    pub fn has_v_start(&self) -> bool {
        self.v_start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v_start(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.v_start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v_start(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.v_start.is_none() {
            self.v_start.set_default();
        };
        self.v_start.as_mut().unwrap()
    }

    // Take field
    pub fn take_v_start(&mut self) -> super::networkbasetypes::CMsgVector {
        self.v_start.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_v_start(&self) -> &super::networkbasetypes::CMsgVector {
        self.v_start.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_v_start_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.v_start
    }

    fn mut_v_start_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.v_start
    }

    // optional .dota.CMsgVector v_end = 17;

    pub fn clear_v_end(&mut self) {
        self.v_end.clear();
    }

    pub fn has_v_end(&self) -> bool {
        self.v_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_v_end(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.v_end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_v_end(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.v_end.is_none() {
            self.v_end.set_default();
        };
        self.v_end.as_mut().unwrap()
    }

    // Take field
    pub fn take_v_end(&mut self) -> super::networkbasetypes::CMsgVector {
        self.v_end.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_v_end(&self) -> &super::networkbasetypes::CMsgVector {
        self.v_end.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_v_end_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.v_end
    }

    fn mut_v_end_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.v_end
    }

    // optional string portal_loop_appear = 18;

    pub fn clear_portal_loop_appear(&mut self) {
        self.portal_loop_appear.clear();
    }

    pub fn has_portal_loop_appear(&self) -> bool {
        self.portal_loop_appear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_portal_loop_appear(&mut self, v: ::std::string::String) {
        self.portal_loop_appear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_portal_loop_appear(&mut self) -> &mut ::std::string::String {
        if self.portal_loop_appear.is_none() {
            self.portal_loop_appear.set_default();
        };
        self.portal_loop_appear.as_mut().unwrap()
    }

    // Take field
    pub fn take_portal_loop_appear(&mut self) -> ::std::string::String {
        self.portal_loop_appear.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_portal_loop_appear(&self) -> &str {
        match self.portal_loop_appear.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_portal_loop_appear_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.portal_loop_appear
    }

    fn mut_portal_loop_appear_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.portal_loop_appear
    }

    // optional string portal_loop_disappear = 19;

    pub fn clear_portal_loop_disappear(&mut self) {
        self.portal_loop_disappear.clear();
    }

    pub fn has_portal_loop_disappear(&self) -> bool {
        self.portal_loop_disappear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_portal_loop_disappear(&mut self, v: ::std::string::String) {
        self.portal_loop_disappear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_portal_loop_disappear(&mut self) -> &mut ::std::string::String {
        if self.portal_loop_disappear.is_none() {
            self.portal_loop_disappear.set_default();
        };
        self.portal_loop_disappear.as_mut().unwrap()
    }

    // Take field
    pub fn take_portal_loop_disappear(&mut self) -> ::std::string::String {
        self.portal_loop_disappear.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_portal_loop_disappear(&self) -> &str {
        match self.portal_loop_disappear.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_portal_loop_disappear_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.portal_loop_disappear
    }

    fn mut_portal_loop_disappear_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.portal_loop_disappear
    }

    // optional string hero_loop_appear = 20;

    pub fn clear_hero_loop_appear(&mut self) {
        self.hero_loop_appear.clear();
    }

    pub fn has_hero_loop_appear(&self) -> bool {
        self.hero_loop_appear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_loop_appear(&mut self, v: ::std::string::String) {
        self.hero_loop_appear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero_loop_appear(&mut self) -> &mut ::std::string::String {
        if self.hero_loop_appear.is_none() {
            self.hero_loop_appear.set_default();
        };
        self.hero_loop_appear.as_mut().unwrap()
    }

    // Take field
    pub fn take_hero_loop_appear(&mut self) -> ::std::string::String {
        self.hero_loop_appear.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hero_loop_appear(&self) -> &str {
        match self.hero_loop_appear.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hero_loop_appear_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hero_loop_appear
    }

    fn mut_hero_loop_appear_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hero_loop_appear
    }

    // optional string hero_loop_disappear = 21;

    pub fn clear_hero_loop_disappear(&mut self) {
        self.hero_loop_disappear.clear();
    }

    pub fn has_hero_loop_disappear(&self) -> bool {
        self.hero_loop_disappear.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero_loop_disappear(&mut self, v: ::std::string::String) {
        self.hero_loop_disappear = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero_loop_disappear(&mut self) -> &mut ::std::string::String {
        if self.hero_loop_disappear.is_none() {
            self.hero_loop_disappear.set_default();
        };
        self.hero_loop_disappear.as_mut().unwrap()
    }

    // Take field
    pub fn take_hero_loop_disappear(&mut self) -> ::std::string::String {
        self.hero_loop_disappear.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hero_loop_disappear(&self) -> &str {
        match self.hero_loop_disappear.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hero_loop_disappear_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hero_loop_disappear
    }

    fn mut_hero_loop_disappear_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hero_loop_disappear
    }

    // optional int32 movement_speed = 22;

    pub fn clear_movement_speed(&mut self) {
        self.movement_speed = ::std::option::Option::None;
    }

    pub fn has_movement_speed(&self) -> bool {
        self.movement_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_movement_speed(&mut self, v: i32) {
        self.movement_speed = ::std::option::Option::Some(v);
    }

    pub fn get_movement_speed(&self) -> i32 {
        self.movement_speed.unwrap_or(0)
    }

    fn get_movement_speed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.movement_speed
    }

    fn mut_movement_speed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.movement_speed
    }

    // optional bool aura = 23;

    pub fn clear_aura(&mut self) {
        self.aura = ::std::option::Option::None;
    }

    pub fn has_aura(&self) -> bool {
        self.aura.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aura(&mut self, v: bool) {
        self.aura = ::std::option::Option::Some(v);
    }

    pub fn get_aura(&self) -> bool {
        self.aura.unwrap_or(false)
    }

    fn get_aura_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.aura
    }

    fn mut_aura_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.aura
    }

    // optional int32 activity = 24;

    pub fn clear_activity(&mut self) {
        self.activity = ::std::option::Option::None;
    }

    pub fn has_activity(&self) -> bool {
        self.activity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_activity(&mut self, v: i32) {
        self.activity = ::std::option::Option::Some(v);
    }

    pub fn get_activity(&self) -> i32 {
        self.activity.unwrap_or(0)
    }

    fn get_activity_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.activity
    }

    fn mut_activity_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.activity
    }

    // optional int32 damage = 25;

    pub fn clear_damage(&mut self) {
        self.damage = ::std::option::Option::None;
    }

    pub fn has_damage(&self) -> bool {
        self.damage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage(&mut self, v: i32) {
        self.damage = ::std::option::Option::Some(v);
    }

    pub fn get_damage(&self) -> i32 {
        self.damage.unwrap_or(0)
    }

    fn get_damage_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.damage
    }

    fn mut_damage_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.damage
    }

    // optional int32 range = 26;

    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None;
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: i32) {
        self.range = ::std::option::Option::Some(v);
    }

    pub fn get_range(&self) -> i32 {
        self.range.unwrap_or(0)
    }

    fn get_range_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.range
    }

    fn mut_range_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.range
    }

    // optional int32 dd_modifier_index = 27;

    pub fn clear_dd_modifier_index(&mut self) {
        self.dd_modifier_index = ::std::option::Option::None;
    }

    pub fn has_dd_modifier_index(&self) -> bool {
        self.dd_modifier_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dd_modifier_index(&mut self, v: i32) {
        self.dd_modifier_index = ::std::option::Option::Some(v);
    }

    pub fn get_dd_modifier_index(&self) -> i32 {
        self.dd_modifier_index.unwrap_or(0)
    }

    fn get_dd_modifier_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dd_modifier_index
    }

    fn mut_dd_modifier_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dd_modifier_index
    }

    // optional int32 dd_ability_index = 28;

    pub fn clear_dd_ability_index(&mut self) {
        self.dd_ability_index = ::std::option::Option::None;
    }

    pub fn has_dd_ability_index(&self) -> bool {
        self.dd_ability_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dd_ability_index(&mut self, v: i32) {
        self.dd_ability_index = ::std::option::Option::Some(v);
    }

    pub fn get_dd_ability_index(&self) -> i32 {
        self.dd_ability_index.unwrap_or(0)
    }

    fn get_dd_ability_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dd_ability_index
    }

    fn mut_dd_ability_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dd_ability_index
    }

    // optional string illusion_label = 29;

    pub fn clear_illusion_label(&mut self) {
        self.illusion_label.clear();
    }

    pub fn has_illusion_label(&self) -> bool {
        self.illusion_label.is_some()
    }

    // Param is passed by value, moved
    pub fn set_illusion_label(&mut self, v: ::std::string::String) {
        self.illusion_label = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_illusion_label(&mut self) -> &mut ::std::string::String {
        if self.illusion_label.is_none() {
            self.illusion_label.set_default();
        };
        self.illusion_label.as_mut().unwrap()
    }

    // Take field
    pub fn take_illusion_label(&mut self) -> ::std::string::String {
        self.illusion_label.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_illusion_label(&self) -> &str {
        match self.illusion_label.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_illusion_label_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.illusion_label
    }

    fn mut_illusion_label_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.illusion_label
    }

    // optional bool active = 30;

    pub fn clear_active(&mut self) {
        self.active = ::std::option::Option::None;
    }

    pub fn has_active(&self) -> bool {
        self.active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active(&mut self, v: bool) {
        self.active = ::std::option::Option::Some(v);
    }

    pub fn get_active(&self) -> bool {
        self.active.unwrap_or(false)
    }

    fn get_active_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.active
    }

    fn mut_active_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.active
    }

    // optional string player_ids = 31;

    pub fn clear_player_ids(&mut self) {
        self.player_ids.clear();
    }

    pub fn has_player_ids(&self) -> bool {
        self.player_ids.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_ids(&mut self, v: ::std::string::String) {
        self.player_ids = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_ids(&mut self) -> &mut ::std::string::String {
        if self.player_ids.is_none() {
            self.player_ids.set_default();
        };
        self.player_ids.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_ids(&mut self) -> ::std::string::String {
        self.player_ids.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_player_ids(&self) -> &str {
        match self.player_ids.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_player_ids_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.player_ids
    }

    fn mut_player_ids_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.player_ids
    }

    // optional string lua_name = 32;

    pub fn clear_lua_name(&mut self) {
        self.lua_name.clear();
    }

    pub fn has_lua_name(&self) -> bool {
        self.lua_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lua_name(&mut self, v: ::std::string::String) {
        self.lua_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lua_name(&mut self) -> &mut ::std::string::String {
        if self.lua_name.is_none() {
            self.lua_name.set_default();
        };
        self.lua_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_lua_name(&mut self) -> ::std::string::String {
        self.lua_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_lua_name(&self) -> &str {
        match self.lua_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_lua_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.lua_name
    }

    fn mut_lua_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.lua_name
    }
}

impl ::protobuf::Message for CDOTAModifierBuffTableEntry {
    fn is_initialized(&self) -> bool {
        if self.entry_type.is_none() {
            return false;
        };
        if self.parent.is_none() {
            return false;
        };
        if self.index.is_none() {
            return false;
        };
        if self.serial_num.is_none() {
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
                    self.entry_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.parent = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.serial_num = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.modifier_class = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ability_level = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.stack_count = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.creation_time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.caster = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.ability = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.armor = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.fade_time = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.subtle = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.channel_time = ::std::option::Option::Some(tmp);
                },
                16 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.v_start)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.v_end)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.portal_loop_appear)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.portal_loop_disappear)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hero_loop_appear)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hero_loop_disappear)?;
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.movement_speed = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.aura = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.activity = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.damage = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.range = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.dd_modifier_index = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.dd_ability_index = ::std::option::Option::Some(tmp);
                },
                29 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.illusion_label)?;
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.active = ::std::option::Option::Some(tmp);
                },
                31 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.player_ids)?;
                },
                32 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.lua_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.entry_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.parent {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.serial_num {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.modifier_class {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ability_level {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.stack_count {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.creation_time {
            my_size += 5;
        };
        if let Some(v) = self.duration {
            my_size += 5;
        };
        if let Some(v) = self.caster {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.ability {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.armor {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.fade_time {
            my_size += 5;
        };
        if let Some(v) = self.subtle {
            my_size += 2;
        };
        if let Some(v) = self.channel_time {
            my_size += 5;
        };
        if let Some(v) = self.v_start.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.v_end.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.portal_loop_appear.as_ref() {
            my_size += ::protobuf::rt::string_size(18, &v);
        };
        if let Some(v) = self.portal_loop_disappear.as_ref() {
            my_size += ::protobuf::rt::string_size(19, &v);
        };
        if let Some(v) = self.hero_loop_appear.as_ref() {
            my_size += ::protobuf::rt::string_size(20, &v);
        };
        if let Some(v) = self.hero_loop_disappear.as_ref() {
            my_size += ::protobuf::rt::string_size(21, &v);
        };
        if let Some(v) = self.movement_speed {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.aura {
            my_size += 3;
        };
        if let Some(v) = self.activity {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.damage {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.range {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dd_modifier_index {
            my_size += ::protobuf::rt::value_size(27, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.dd_ability_index {
            my_size += ::protobuf::rt::value_size(28, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.illusion_label.as_ref() {
            my_size += ::protobuf::rt::string_size(29, &v);
        };
        if let Some(v) = self.active {
            my_size += 3;
        };
        if let Some(v) = self.player_ids.as_ref() {
            my_size += ::protobuf::rt::string_size(31, &v);
        };
        if let Some(v) = self.lua_name.as_ref() {
            my_size += ::protobuf::rt::string_size(32, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entry_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.parent {
            os.write_int32(2, v)?;
        };
        if let Some(v) = self.index {
            os.write_int32(3, v)?;
        };
        if let Some(v) = self.serial_num {
            os.write_int32(4, v)?;
        };
        if let Some(v) = self.modifier_class {
            os.write_int32(5, v)?;
        };
        if let Some(v) = self.ability_level {
            os.write_int32(6, v)?;
        };
        if let Some(v) = self.stack_count {
            os.write_int32(7, v)?;
        };
        if let Some(v) = self.creation_time {
            os.write_float(8, v)?;
        };
        if let Some(v) = self.duration {
            os.write_float(9, v)?;
        };
        if let Some(v) = self.caster {
            os.write_int32(10, v)?;
        };
        if let Some(v) = self.ability {
            os.write_int32(11, v)?;
        };
        if let Some(v) = self.armor {
            os.write_int32(12, v)?;
        };
        if let Some(v) = self.fade_time {
            os.write_float(13, v)?;
        };
        if let Some(v) = self.subtle {
            os.write_bool(14, v)?;
        };
        if let Some(v) = self.channel_time {
            os.write_float(15, v)?;
        };
        if let Some(v) = self.v_start.as_ref() {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.v_end.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.portal_loop_appear.as_ref() {
            os.write_string(18, &v)?;
        };
        if let Some(v) = self.portal_loop_disappear.as_ref() {
            os.write_string(19, &v)?;
        };
        if let Some(v) = self.hero_loop_appear.as_ref() {
            os.write_string(20, &v)?;
        };
        if let Some(v) = self.hero_loop_disappear.as_ref() {
            os.write_string(21, &v)?;
        };
        if let Some(v) = self.movement_speed {
            os.write_int32(22, v)?;
        };
        if let Some(v) = self.aura {
            os.write_bool(23, v)?;
        };
        if let Some(v) = self.activity {
            os.write_int32(24, v)?;
        };
        if let Some(v) = self.damage {
            os.write_int32(25, v)?;
        };
        if let Some(v) = self.range {
            os.write_int32(26, v)?;
        };
        if let Some(v) = self.dd_modifier_index {
            os.write_int32(27, v)?;
        };
        if let Some(v) = self.dd_ability_index {
            os.write_int32(28, v)?;
        };
        if let Some(v) = self.illusion_label.as_ref() {
            os.write_string(29, &v)?;
        };
        if let Some(v) = self.active {
            os.write_bool(30, v)?;
        };
        if let Some(v) = self.player_ids.as_ref() {
            os.write_string(31, &v)?;
        };
        if let Some(v) = self.lua_name.as_ref() {
            os.write_string(32, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAModifierBuffTableEntry {
    fn new() -> CDOTAModifierBuffTableEntry {
        CDOTAModifierBuffTableEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAModifierBuffTableEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DOTA_MODIFIER_ENTRY_TYPE>>(
                    "entry_type",
                    CDOTAModifierBuffTableEntry::get_entry_type_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_entry_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "parent",
                    CDOTAModifierBuffTableEntry::get_parent_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_parent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "index",
                    CDOTAModifierBuffTableEntry::get_index_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "serial_num",
                    CDOTAModifierBuffTableEntry::get_serial_num_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_serial_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "modifier_class",
                    CDOTAModifierBuffTableEntry::get_modifier_class_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_modifier_class_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability_level",
                    CDOTAModifierBuffTableEntry::get_ability_level_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_ability_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "stack_count",
                    CDOTAModifierBuffTableEntry::get_stack_count_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_stack_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "creation_time",
                    CDOTAModifierBuffTableEntry::get_creation_time_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_creation_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CDOTAModifierBuffTableEntry::get_duration_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "caster",
                    CDOTAModifierBuffTableEntry::get_caster_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_caster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability",
                    CDOTAModifierBuffTableEntry::get_ability_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_ability_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "armor",
                    CDOTAModifierBuffTableEntry::get_armor_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_armor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fade_time",
                    CDOTAModifierBuffTableEntry::get_fade_time_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_fade_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "subtle",
                    CDOTAModifierBuffTableEntry::get_subtle_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_subtle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "channel_time",
                    CDOTAModifierBuffTableEntry::get_channel_time_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_channel_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "v_start",
                    CDOTAModifierBuffTableEntry::get_v_start_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_v_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "v_end",
                    CDOTAModifierBuffTableEntry::get_v_end_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_v_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "portal_loop_appear",
                    CDOTAModifierBuffTableEntry::get_portal_loop_appear_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_portal_loop_appear_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "portal_loop_disappear",
                    CDOTAModifierBuffTableEntry::get_portal_loop_disappear_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_portal_loop_disappear_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hero_loop_appear",
                    CDOTAModifierBuffTableEntry::get_hero_loop_appear_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_hero_loop_appear_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hero_loop_disappear",
                    CDOTAModifierBuffTableEntry::get_hero_loop_disappear_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_hero_loop_disappear_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "movement_speed",
                    CDOTAModifierBuffTableEntry::get_movement_speed_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_movement_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "aura",
                    CDOTAModifierBuffTableEntry::get_aura_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_aura_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "activity",
                    CDOTAModifierBuffTableEntry::get_activity_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_activity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "damage",
                    CDOTAModifierBuffTableEntry::get_damage_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_damage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "range",
                    CDOTAModifierBuffTableEntry::get_range_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dd_modifier_index",
                    CDOTAModifierBuffTableEntry::get_dd_modifier_index_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_dd_modifier_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dd_ability_index",
                    CDOTAModifierBuffTableEntry::get_dd_ability_index_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_dd_ability_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "illusion_label",
                    CDOTAModifierBuffTableEntry::get_illusion_label_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_illusion_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "active",
                    CDOTAModifierBuffTableEntry::get_active_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_active_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "player_ids",
                    CDOTAModifierBuffTableEntry::get_player_ids_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_player_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "lua_name",
                    CDOTAModifierBuffTableEntry::get_lua_name_for_reflect,
                    CDOTAModifierBuffTableEntry::mut_lua_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAModifierBuffTableEntry>(
                    "CDOTAModifierBuffTableEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAModifierBuffTableEntry {
    fn clear(&mut self) {
        self.clear_entry_type();
        self.clear_parent();
        self.clear_index();
        self.clear_serial_num();
        self.clear_modifier_class();
        self.clear_ability_level();
        self.clear_stack_count();
        self.clear_creation_time();
        self.clear_duration();
        self.clear_caster();
        self.clear_ability();
        self.clear_armor();
        self.clear_fade_time();
        self.clear_subtle();
        self.clear_channel_time();
        self.clear_v_start();
        self.clear_v_end();
        self.clear_portal_loop_appear();
        self.clear_portal_loop_disappear();
        self.clear_hero_loop_appear();
        self.clear_hero_loop_disappear();
        self.clear_movement_speed();
        self.clear_aura();
        self.clear_activity();
        self.clear_damage();
        self.clear_range();
        self.clear_dd_modifier_index();
        self.clear_dd_ability_index();
        self.clear_illusion_label();
        self.clear_active();
        self.clear_player_ids();
        self.clear_lua_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAModifierBuffTableEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAModifierBuffTableEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTALuaModifierEntry {
    // message fields
    modifier_type: ::std::option::Option<i32>,
    modifier_filename: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTALuaModifierEntry {}

impl CDOTALuaModifierEntry {
    pub fn new() -> CDOTALuaModifierEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTALuaModifierEntry {
        static mut instance: ::protobuf::lazy::Lazy<CDOTALuaModifierEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTALuaModifierEntry,
        };
        unsafe {
            instance.get(CDOTALuaModifierEntry::new)
        }
    }

    // required int32 modifier_type = 1;

    pub fn clear_modifier_type(&mut self) {
        self.modifier_type = ::std::option::Option::None;
    }

    pub fn has_modifier_type(&self) -> bool {
        self.modifier_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modifier_type(&mut self, v: i32) {
        self.modifier_type = ::std::option::Option::Some(v);
    }

    pub fn get_modifier_type(&self) -> i32 {
        self.modifier_type.unwrap_or(0)
    }

    fn get_modifier_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.modifier_type
    }

    fn mut_modifier_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.modifier_type
    }

    // required string modifier_filename = 2;

    pub fn clear_modifier_filename(&mut self) {
        self.modifier_filename.clear();
    }

    pub fn has_modifier_filename(&self) -> bool {
        self.modifier_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modifier_filename(&mut self, v: ::std::string::String) {
        self.modifier_filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_modifier_filename(&mut self) -> &mut ::std::string::String {
        if self.modifier_filename.is_none() {
            self.modifier_filename.set_default();
        };
        self.modifier_filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_modifier_filename(&mut self) -> ::std::string::String {
        self.modifier_filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_modifier_filename(&self) -> &str {
        match self.modifier_filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_modifier_filename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.modifier_filename
    }

    fn mut_modifier_filename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.modifier_filename
    }
}

impl ::protobuf::Message for CDOTALuaModifierEntry {
    fn is_initialized(&self) -> bool {
        if self.modifier_type.is_none() {
            return false;
        };
        if self.modifier_filename.is_none() {
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
                    let tmp = is.read_int32()?;
                    self.modifier_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.modifier_filename)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.modifier_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.modifier_filename.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.modifier_type {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.modifier_filename.as_ref() {
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

impl ::protobuf::MessageStatic for CDOTALuaModifierEntry {
    fn new() -> CDOTALuaModifierEntry {
        CDOTALuaModifierEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTALuaModifierEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "modifier_type",
                    CDOTALuaModifierEntry::get_modifier_type_for_reflect,
                    CDOTALuaModifierEntry::mut_modifier_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "modifier_filename",
                    CDOTALuaModifierEntry::get_modifier_filename_for_reflect,
                    CDOTALuaModifierEntry::mut_modifier_filename_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTALuaModifierEntry>(
                    "CDOTALuaModifierEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTALuaModifierEntry {
    fn clear(&mut self) {
        self.clear_modifier_type();
        self.clear_modifier_filename();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTALuaModifierEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTALuaModifierEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_MODIFIER_ENTRY_TYPE {
    DOTA_MODIFIER_ENTRY_TYPE_ACTIVE = 1,
    DOTA_MODIFIER_ENTRY_TYPE_REMOVED = 2,
}

impl ::protobuf::ProtobufEnum for DOTA_MODIFIER_ENTRY_TYPE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_MODIFIER_ENTRY_TYPE> {
        match value {
            1 => ::std::option::Option::Some(DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_ACTIVE),
            2 => ::std::option::Option::Some(DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_REMOVED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_MODIFIER_ENTRY_TYPE] = &[
            DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_ACTIVE,
            DOTA_MODIFIER_ENTRY_TYPE::DOTA_MODIFIER_ENTRY_TYPE_REMOVED,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DOTA_MODIFIER_ENTRY_TYPE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_MODIFIER_ENTRY_TYPE", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_MODIFIER_ENTRY_TYPE {
}

impl ::protobuf::reflect::ProtobufValue for DOTA_MODIFIER_ENTRY_TYPE {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x16, 0x6e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x62, 0x61, 0x73, 0x65, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0xfe, 0x08, 0x0a, 0x1b, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x6f,
    0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x42, 0x75, 0x66, 0x66, 0x54, 0x61, 0x62, 0x6c, 0x65, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x12, 0x5e, 0x0a, 0x0a, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1e, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49, 0x45, 0x52, 0x5f, 0x45, 0x4e,
    0x54, 0x52, 0x59, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x3a, 0x1f, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4d,
    0x4f, 0x44, 0x49, 0x46, 0x49, 0x45, 0x52, 0x5f, 0x45, 0x4e, 0x54, 0x52, 0x59, 0x5f, 0x54, 0x59,
    0x50, 0x45, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x56, 0x45, 0x52, 0x09, 0x65, 0x6e, 0x74, 0x72, 0x79,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x05, 0x52, 0x06, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x12, 0x14, 0x0a, 0x05,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x03, 0x20, 0x02, 0x28, 0x05, 0x52, 0x05, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x12, 0x1d, 0x0a, 0x0a, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x5f, 0x6e, 0x75, 0x6d,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x52, 0x09, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x4e, 0x75,
    0x6d, 0x12, 0x25, 0x0a, 0x0e, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x5f, 0x63, 0x6c,
    0x61, 0x73, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x6d, 0x6f, 0x64, 0x69, 0x66,
    0x69, 0x65, 0x72, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x12, 0x23, 0x0a, 0x0d, 0x61, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x06, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x0c, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x1f, 0x0a,
    0x0b, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x05, 0x52, 0x0a, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x23,
    0x0a, 0x0d, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x54,
    0x69, 0x6d, 0x65, 0x12, 0x1e, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x02, 0x3a, 0x02, 0x2d, 0x31, 0x52, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x63, 0x61, 0x73, 0x74, 0x65, 0x72, 0x18, 0x0a, 0x20,
    0x01, 0x28, 0x05, 0x52, 0x06, 0x63, 0x61, 0x73, 0x74, 0x65, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x61,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x61, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x61, 0x72, 0x6d, 0x6f, 0x72, 0x18, 0x0c,
    0x20, 0x01, 0x28, 0x05, 0x52, 0x05, 0x61, 0x72, 0x6d, 0x6f, 0x72, 0x12, 0x1b, 0x0a, 0x09, 0x66,
    0x61, 0x64, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x02, 0x52, 0x08,
    0x66, 0x61, 0x64, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x75, 0x62, 0x74,
    0x6c, 0x65, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x08, 0x52, 0x06, 0x73, 0x75, 0x62, 0x74, 0x6c, 0x65,
    0x12, 0x21, 0x0a, 0x0c, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x5f, 0x74, 0x69, 0x6d, 0x65,
    0x18, 0x0f, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x54,
    0x69, 0x6d, 0x65, 0x12, 0x29, 0x0a, 0x07, 0x76, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x18, 0x10,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67,
    0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x76, 0x53, 0x74, 0x61, 0x72, 0x74, 0x12, 0x25,
    0x0a, 0x05, 0x76, 0x5f, 0x65, 0x6e, 0x64, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x4d, 0x73, 0x67, 0x56, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52,
    0x04, 0x76, 0x45, 0x6e, 0x64, 0x12, 0x2c, 0x0a, 0x12, 0x70, 0x6f, 0x72, 0x74, 0x61, 0x6c, 0x5f,
    0x6c, 0x6f, 0x6f, 0x70, 0x5f, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x18, 0x12, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x10, 0x70, 0x6f, 0x72, 0x74, 0x61, 0x6c, 0x4c, 0x6f, 0x6f, 0x70, 0x41, 0x70, 0x70,
    0x65, 0x61, 0x72, 0x12, 0x32, 0x0a, 0x15, 0x70, 0x6f, 0x72, 0x74, 0x61, 0x6c, 0x5f, 0x6c, 0x6f,
    0x6f, 0x70, 0x5f, 0x64, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x18, 0x13, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x13, 0x70, 0x6f, 0x72, 0x74, 0x61, 0x6c, 0x4c, 0x6f, 0x6f, 0x70, 0x44, 0x69,
    0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x12, 0x28, 0x0a, 0x10, 0x68, 0x65, 0x72, 0x6f, 0x5f,
    0x6c, 0x6f, 0x6f, 0x70, 0x5f, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x18, 0x14, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0e, 0x68, 0x65, 0x72, 0x6f, 0x4c, 0x6f, 0x6f, 0x70, 0x41, 0x70, 0x70, 0x65, 0x61,
    0x72, 0x12, 0x2e, 0x0a, 0x13, 0x68, 0x65, 0x72, 0x6f, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x5f, 0x64,
    0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61, 0x72, 0x18, 0x15, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11,
    0x68, 0x65, 0x72, 0x6f, 0x4c, 0x6f, 0x6f, 0x70, 0x44, 0x69, 0x73, 0x61, 0x70, 0x70, 0x65, 0x61,
    0x72, 0x12, 0x25, 0x0a, 0x0e, 0x6d, 0x6f, 0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x70,
    0x65, 0x65, 0x64, 0x18, 0x16, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0d, 0x6d, 0x6f, 0x76, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x53, 0x70, 0x65, 0x65, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x61, 0x75, 0x72, 0x61,
    0x18, 0x17, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x61, 0x75, 0x72, 0x61, 0x12, 0x1a, 0x0a, 0x08,
    0x61, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x18, 0x18, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08,
    0x61, 0x63, 0x74, 0x69, 0x76, 0x69, 0x74, 0x79, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x61, 0x6d, 0x61,
    0x67, 0x65, 0x18, 0x19, 0x20, 0x01, 0x28, 0x05, 0x52, 0x06, 0x64, 0x61, 0x6d, 0x61, 0x67, 0x65,
    0x12, 0x14, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x1a, 0x20, 0x01, 0x28, 0x05, 0x52,
    0x05, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x64, 0x64, 0x5f, 0x6d, 0x6f, 0x64,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x1b, 0x20, 0x01, 0x28,
    0x05, 0x52, 0x0f, 0x64, 0x64, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x49, 0x6e, 0x64,
    0x65, 0x78, 0x12, 0x28, 0x0a, 0x10, 0x64, 0x64, 0x5f, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79,
    0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x1c, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0e, 0x64, 0x64,
    0x41, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x25, 0x0a, 0x0e,
    0x69, 0x6c, 0x6c, 0x75, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x18, 0x1d,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x69, 0x6c, 0x6c, 0x75, 0x73, 0x69, 0x6f, 0x6e, 0x4c, 0x61,
    0x62, 0x65, 0x6c, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x18, 0x1e, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x06, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x1f, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x09, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x64, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x6c, 0x75,
    0x61, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x20, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6c, 0x75,
    0x61, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x69, 0x0a, 0x15, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4c, 0x75,
    0x61, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x23,
    0x0a, 0x0d, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x05, 0x52, 0x0c, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x2b, 0x0a, 0x11, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x5f,
    0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x52, 0x10,
    0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x72, 0x46, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65,
    0x2a, 0x65, 0x0a, 0x18, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49, 0x45,
    0x52, 0x5f, 0x45, 0x4e, 0x54, 0x52, 0x59, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x12, 0x23, 0x0a, 0x1f,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49, 0x45, 0x52, 0x5f, 0x45, 0x4e,
    0x54, 0x52, 0x59, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x56, 0x45, 0x10,
    0x01, 0x12, 0x24, 0x0a, 0x20, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x4d, 0x4f, 0x44, 0x49, 0x46, 0x49,
    0x45, 0x52, 0x5f, 0x45, 0x4e, 0x54, 0x52, 0x59, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x52, 0x45,
    0x4d, 0x4f, 0x56, 0x45, 0x44, 0x10, 0x02, 0x42, 0x05, 0x48, 0x01, 0x80, 0x01, 0x00, 0x4a, 0xc7,
    0x15, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x34, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06,
    0x00, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x06, 0x00, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x06, 0x16, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x07, 0x00,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0e,
    0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12,
    0x03, 0x09, 0x05, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x08,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x0b, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0e, 0x00,
    0x2f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x08, 0x65, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0f, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0f, 0x11, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0f, 0x2a, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0f, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x0f, 0x39,
    0x64, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x07, 0x12, 0x03, 0x0f, 0x44, 0x63, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x10, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x10, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x10, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x11, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x11, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x11, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x11, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x11, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x12, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x12, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x12,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x12, 0x17, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x12, 0x24, 0x25, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x13, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x13, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x13, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x13, 0x17, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x13, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x14, 0x08, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x14, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x14, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x14, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x15, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x03,
    0x15, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x15, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x15, 0x17, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x15, 0x25, 0x26, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x16, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x04, 0x12, 0x03, 0x16, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x16, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x16, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x16,
    0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x17, 0x08, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x17, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x17, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x17, 0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x17, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x08,
    0x12, 0x03, 0x17, 0x24, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x07, 0x12, 0x03,
    0x17, 0x2f, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x18, 0x08, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x18, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x18, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x18, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x18, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0a,
    0x12, 0x03, 0x19, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04, 0x12, 0x03,
    0x19, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x19, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x19, 0x17, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x19, 0x21, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x1a, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0b, 0x04, 0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b,
    0x05, 0x12, 0x03, 0x1a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12,
    0x03, 0x1a, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x1a,
    0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x1b, 0x08, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x1b, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x1b, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x1b, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x1b, 0x23, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0d, 0x12,
    0x03, 0x1c, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x1c,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x1c, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x1c, 0x16, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x1c, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x0e, 0x12, 0x03, 0x1d, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x0e, 0x04, 0x12, 0x03, 0x1d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x05,
    0x12, 0x03, 0x1d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x01, 0x12, 0x03,
    0x1d, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x1d, 0x26,
    0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x1e, 0x08, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x1e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0f, 0x06, 0x12, 0x03, 0x1e, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x0f, 0x01, 0x12, 0x03, 0x1e, 0x1c, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0f,
    0x03, 0x12, 0x03, 0x1e, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x10, 0x12, 0x03,
    0x1f, 0x08, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x04, 0x12, 0x03, 0x1f, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x06, 0x12, 0x03, 0x1f, 0x11, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x10, 0x01, 0x12, 0x03, 0x1f, 0x1c, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x10, 0x03, 0x12, 0x03, 0x1f, 0x24, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x11, 0x12, 0x03, 0x20, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11,
    0x04, 0x12, 0x03, 0x20, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x05, 0x12,
    0x03, 0x20, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x01, 0x12, 0x03, 0x20,
    0x18, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x11, 0x03, 0x12, 0x03, 0x20, 0x2d, 0x2f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x12, 0x12, 0x03, 0x21, 0x08, 0x33, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x12, 0x04, 0x12, 0x03, 0x21, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x12, 0x05, 0x12, 0x03, 0x21, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x12, 0x01, 0x12, 0x03, 0x21, 0x18, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x12, 0x03,
    0x12, 0x03, 0x21, 0x30, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x13, 0x12, 0x03, 0x22,
    0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x04, 0x12, 0x03, 0x22, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x05, 0x12, 0x03, 0x22, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x13, 0x01, 0x12, 0x03, 0x22, 0x18, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x13, 0x03, 0x12, 0x03, 0x22, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x14, 0x12, 0x03, 0x23, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x04,
    0x12, 0x03, 0x23, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x05, 0x12, 0x03,
    0x23, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x01, 0x12, 0x03, 0x23, 0x18,
    0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x14, 0x03, 0x12, 0x03, 0x23, 0x2e, 0x30, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x15, 0x12, 0x03, 0x24, 0x08, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x15, 0x04, 0x12, 0x03, 0x24, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x15, 0x05, 0x12, 0x03, 0x24, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15,
    0x01, 0x12, 0x03, 0x24, 0x17, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x15, 0x03, 0x12,
    0x03, 0x24, 0x28, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x16, 0x12, 0x03, 0x25, 0x08,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x04, 0x12, 0x03, 0x25, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x16, 0x05, 0x12, 0x03, 0x25, 0x11, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x16, 0x01, 0x12, 0x03, 0x25, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x16, 0x03, 0x12, 0x03, 0x25, 0x1d, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x17, 0x12, 0x03, 0x26, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x17, 0x04, 0x12,
    0x03, 0x26, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x17, 0x05, 0x12, 0x03, 0x26,
    0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x17, 0x01, 0x12, 0x03, 0x26, 0x17, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x17, 0x03, 0x12, 0x03, 0x26, 0x22, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x18, 0x12, 0x03, 0x27, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x18, 0x04, 0x12, 0x03, 0x27, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x18, 0x05, 0x12, 0x03, 0x27, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x01,
    0x12, 0x03, 0x27, 0x17, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x18, 0x03, 0x12, 0x03,
    0x27, 0x20, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x19, 0x12, 0x03, 0x28, 0x08, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x19, 0x04, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x19, 0x05, 0x12, 0x03, 0x28, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x19, 0x01, 0x12, 0x03, 0x28, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x19, 0x03, 0x12, 0x03, 0x28, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x1a,
    0x12, 0x03, 0x29, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1a, 0x04, 0x12, 0x03,
    0x29, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1a, 0x05, 0x12, 0x03, 0x29, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1a, 0x01, 0x12, 0x03, 0x29, 0x17, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1a, 0x03, 0x12, 0x03, 0x29, 0x2b, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x1b, 0x12, 0x03, 0x2a, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x1b, 0x04, 0x12, 0x03, 0x2a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1b,
    0x05, 0x12, 0x03, 0x2a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1b, 0x01, 0x12,
    0x03, 0x2a, 0x17, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1b, 0x03, 0x12, 0x03, 0x2a,
    0x2a, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x1c, 0x12, 0x03, 0x2b, 0x08, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1c, 0x04, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x1c, 0x05, 0x12, 0x03, 0x2b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x1c, 0x01, 0x12, 0x03, 0x2b, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x1c, 0x03, 0x12, 0x03, 0x2b, 0x29, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x1d, 0x12,
    0x03, 0x2c, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1d, 0x04, 0x12, 0x03, 0x2c,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1d, 0x05, 0x12, 0x03, 0x2c, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1d, 0x01, 0x12, 0x03, 0x2c, 0x16, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x1d, 0x03, 0x12, 0x03, 0x2c, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x1e, 0x12, 0x03, 0x2d, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x1e, 0x04, 0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1e, 0x05,
    0x12, 0x03, 0x2d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1e, 0x01, 0x12, 0x03,
    0x2d, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1e, 0x03, 0x12, 0x03, 0x2d, 0x25,
    0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x1f, 0x12, 0x03, 0x2e, 0x08, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x1f, 0x04, 0x12, 0x03, 0x2e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x1f, 0x05, 0x12, 0x03, 0x2e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x1f, 0x01, 0x12, 0x03, 0x2e, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x1f,
    0x03, 0x12, 0x03, 0x2e, 0x23, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x31, 0x00,
    0x34, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x31, 0x08, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x32, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x32, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x32, 0x17, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x32, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x33, 0x08, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x33, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x33, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x33, 0x18, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x33, 0x2c, 0x2d,
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
