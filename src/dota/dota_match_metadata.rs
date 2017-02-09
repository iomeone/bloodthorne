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
pub struct CDOTAMatchMetadataFile {
    // message fields
    version: ::std::option::Option<i32>,
    match_id: ::std::option::Option<u64>,
    metadata: ::protobuf::SingularPtrField<CDOTAMatchMetadata>,
    private_metadata: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchMetadataFile {}

impl CDOTAMatchMetadataFile {
    pub fn new() -> CDOTAMatchMetadataFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchMetadataFile {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchMetadataFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchMetadataFile,
        };
        unsafe {
            instance.get(CDOTAMatchMetadataFile::new)
        }
    }

    // required int32 version = 1;

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

    // required uint64 match_id = 2;

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

    // optional .dota.CDOTAMatchMetadata metadata = 3;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: CDOTAMatchMetadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut CDOTAMatchMetadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        };
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> CDOTAMatchMetadata {
        self.metadata.take().unwrap_or_else(|| CDOTAMatchMetadata::new())
    }

    pub fn get_metadata(&self) -> &CDOTAMatchMetadata {
        self.metadata.as_ref().unwrap_or_else(|| CDOTAMatchMetadata::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<CDOTAMatchMetadata> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CDOTAMatchMetadata> {
        &mut self.metadata
    }

    // optional bytes private_metadata = 5;

    pub fn clear_private_metadata(&mut self) {
        self.private_metadata.clear();
    }

    pub fn has_private_metadata(&self) -> bool {
        self.private_metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_private_metadata(&mut self, v: ::std::vec::Vec<u8>) {
        self.private_metadata = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_private_metadata(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.private_metadata.is_none() {
            self.private_metadata.set_default();
        };
        self.private_metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_private_metadata(&mut self) -> ::std::vec::Vec<u8> {
        self.private_metadata.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_private_metadata(&self) -> &[u8] {
        match self.private_metadata.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_private_metadata_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.private_metadata
    }

    fn mut_private_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.private_metadata
    }
}

impl ::protobuf::Message for CDOTAMatchMetadataFile {
    fn is_initialized(&self) -> bool {
        if self.version.is_none() {
            return false;
        };
        if self.match_id.is_none() {
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
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.private_metadata)?;
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
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.private_metadata.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_int32(1, v)?;
        };
        if let Some(v) = self.match_id {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.metadata.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.private_metadata.as_ref() {
            os.write_bytes(5, &v)?;
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

impl ::protobuf::MessageStatic for CDOTAMatchMetadataFile {
    fn new() -> CDOTAMatchMetadataFile {
        CDOTAMatchMetadataFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchMetadataFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    CDOTAMatchMetadataFile::get_version_for_reflect,
                    CDOTAMatchMetadataFile::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CDOTAMatchMetadataFile::get_match_id_for_reflect,
                    CDOTAMatchMetadataFile::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchMetadata>>(
                    "metadata",
                    CDOTAMatchMetadataFile::get_metadata_for_reflect,
                    CDOTAMatchMetadataFile::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "private_metadata",
                    CDOTAMatchMetadataFile::get_private_metadata_for_reflect,
                    CDOTAMatchMetadataFile::mut_private_metadata_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchMetadataFile>(
                    "CDOTAMatchMetadataFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchMetadataFile {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_match_id();
        self.clear_metadata();
        self.clear_private_metadata();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchMetadataFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchMetadataFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchMetadata {
    // message fields
    teams: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team>,
    item_rewards: ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTimedRewardDetails>,
    lobby_id: ::std::option::Option<u64>,
    report_until_time: ::std::option::Option<u64>,
    event_game_custom_table: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchMetadata {}

impl CDOTAMatchMetadata {
    pub fn new() -> CDOTAMatchMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchMetadata {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchMetadata,
        };
        unsafe {
            instance.get(CDOTAMatchMetadata::new)
        }
    }

    // repeated .dota.CDOTAMatchMetadata.Team teams = 1;

    pub fn clear_teams(&mut self) {
        self.teams.clear();
    }

    // Param is passed by value, moved
    pub fn set_teams(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team>) {
        self.teams = v;
    }

    // Mutable pointer to the field.
    pub fn mut_teams(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team> {
        &mut self.teams
    }

    // Take field
    pub fn take_teams(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchMetadata_Team> {
        ::std::mem::replace(&mut self.teams, ::protobuf::RepeatedField::new())
    }

    pub fn get_teams(&self) -> &[CDOTAMatchMetadata_Team] {
        &self.teams
    }

    fn get_teams_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchMetadata_Team> {
        &self.teams
    }

    fn mut_teams_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team> {
        &mut self.teams
    }

    // repeated .dota.CLobbyTimedRewardDetails item_rewards = 2;

    pub fn clear_item_rewards(&mut self) {
        self.item_rewards.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_rewards(&mut self, v: ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTimedRewardDetails>) {
        self.item_rewards = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_rewards(&mut self) -> &mut ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTimedRewardDetails> {
        &mut self.item_rewards
    }

    // Take field
    pub fn take_item_rewards(&mut self) -> ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTimedRewardDetails> {
        ::std::mem::replace(&mut self.item_rewards, ::protobuf::RepeatedField::new())
    }

    pub fn get_item_rewards(&self) -> &[super::dota_gcmessages_common_match_management::CLobbyTimedRewardDetails] {
        &self.item_rewards
    }

    fn get_item_rewards_for_reflect(&self) -> &::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTimedRewardDetails> {
        &self.item_rewards
    }

    fn mut_item_rewards_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::dota_gcmessages_common_match_management::CLobbyTimedRewardDetails> {
        &mut self.item_rewards
    }

    // optional fixed64 lobby_id = 3;

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

    // optional fixed64 report_until_time = 4;

    pub fn clear_report_until_time(&mut self) {
        self.report_until_time = ::std::option::Option::None;
    }

    pub fn has_report_until_time(&self) -> bool {
        self.report_until_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report_until_time(&mut self, v: u64) {
        self.report_until_time = ::std::option::Option::Some(v);
    }

    pub fn get_report_until_time(&self) -> u64 {
        self.report_until_time.unwrap_or(0)
    }

    fn get_report_until_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.report_until_time
    }

    fn mut_report_until_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.report_until_time
    }

    // optional bytes event_game_custom_table = 5;

    pub fn clear_event_game_custom_table(&mut self) {
        self.event_game_custom_table.clear();
    }

    pub fn has_event_game_custom_table(&self) -> bool {
        self.event_game_custom_table.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_game_custom_table(&mut self, v: ::std::vec::Vec<u8>) {
        self.event_game_custom_table = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_game_custom_table(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.event_game_custom_table.is_none() {
            self.event_game_custom_table.set_default();
        };
        self.event_game_custom_table.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_game_custom_table(&mut self) -> ::std::vec::Vec<u8> {
        self.event_game_custom_table.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_event_game_custom_table(&self) -> &[u8] {
        match self.event_game_custom_table.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_event_game_custom_table_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.event_game_custom_table
    }

    fn mut_event_game_custom_table_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.event_game_custom_table
    }
}

impl ::protobuf::Message for CDOTAMatchMetadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.teams)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.item_rewards)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.lobby_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_fixed64()?;
                    self.report_until_time = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.event_game_custom_table)?;
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
        for value in &self.teams {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.item_rewards {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.lobby_id {
            my_size += 9;
        };
        if let Some(v) = self.report_until_time {
            my_size += 9;
        };
        if let Some(v) = self.event_game_custom_table.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.teams {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.item_rewards {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.lobby_id {
            os.write_fixed64(3, v)?;
        };
        if let Some(v) = self.report_until_time {
            os.write_fixed64(4, v)?;
        };
        if let Some(v) = self.event_game_custom_table.as_ref() {
            os.write_bytes(5, &v)?;
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

impl ::protobuf::MessageStatic for CDOTAMatchMetadata {
    fn new() -> CDOTAMatchMetadata {
        CDOTAMatchMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchMetadata_Team>>(
                    "teams",
                    CDOTAMatchMetadata::get_teams_for_reflect,
                    CDOTAMatchMetadata::mut_teams_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::dota_gcmessages_common_match_management::CLobbyTimedRewardDetails>>(
                    "item_rewards",
                    CDOTAMatchMetadata::get_item_rewards_for_reflect,
                    CDOTAMatchMetadata::mut_item_rewards_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "lobby_id",
                    CDOTAMatchMetadata::get_lobby_id_for_reflect,
                    CDOTAMatchMetadata::mut_lobby_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "report_until_time",
                    CDOTAMatchMetadata::get_report_until_time_for_reflect,
                    CDOTAMatchMetadata::mut_report_until_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "event_game_custom_table",
                    CDOTAMatchMetadata::get_event_game_custom_table_for_reflect,
                    CDOTAMatchMetadata::mut_event_game_custom_table_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchMetadata>(
                    "CDOTAMatchMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchMetadata {
    fn clear(&mut self) {
        self.clear_teams();
        self.clear_item_rewards();
        self.clear_lobby_id();
        self.clear_report_until_time();
        self.clear_event_game_custom_table();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchMetadata_Team {
    // message fields
    dota_team: ::std::option::Option<u32>,
    players: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_Player>,
    graph_experience: ::std::vec::Vec<f32>,
    graph_gold_earned: ::std::vec::Vec<f32>,
    graph_net_worth: ::std::vec::Vec<f32>,
    cm_first_pick: ::std::option::Option<bool>,
    cm_captain_player_id: ::std::option::Option<u32>,
    cm_bans: ::std::vec::Vec<u32>,
    cm_picks: ::std::vec::Vec<u32>,
    cm_penalty: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchMetadata_Team {}

impl CDOTAMatchMetadata_Team {
    pub fn new() -> CDOTAMatchMetadata_Team {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchMetadata_Team {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchMetadata_Team> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchMetadata_Team,
        };
        unsafe {
            instance.get(CDOTAMatchMetadata_Team::new)
        }
    }

    // optional uint32 dota_team = 1;

    pub fn clear_dota_team(&mut self) {
        self.dota_team = ::std::option::Option::None;
    }

    pub fn has_dota_team(&self) -> bool {
        self.dota_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_team(&mut self, v: u32) {
        self.dota_team = ::std::option::Option::Some(v);
    }

    pub fn get_dota_team(&self) -> u32 {
        self.dota_team.unwrap_or(0)
    }

    fn get_dota_team_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dota_team
    }

    fn mut_dota_team_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dota_team
    }

    // repeated .dota.CDOTAMatchMetadata.Team.Player players = 2;

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_Player>) {
        self.players = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_Player> {
        &mut self.players
    }

    // Take field
    pub fn take_players(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_Player> {
        ::std::mem::replace(&mut self.players, ::protobuf::RepeatedField::new())
    }

    pub fn get_players(&self) -> &[CDOTAMatchMetadata_Team_Player] {
        &self.players
    }

    fn get_players_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchMetadata_Team_Player> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_Player> {
        &mut self.players
    }

    // repeated float graph_experience = 3;

    pub fn clear_graph_experience(&mut self) {
        self.graph_experience.clear();
    }

    // Param is passed by value, moved
    pub fn set_graph_experience(&mut self, v: ::std::vec::Vec<f32>) {
        self.graph_experience = v;
    }

    // Mutable pointer to the field.
    pub fn mut_graph_experience(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.graph_experience
    }

    // Take field
    pub fn take_graph_experience(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.graph_experience, ::std::vec::Vec::new())
    }

    pub fn get_graph_experience(&self) -> &[f32] {
        &self.graph_experience
    }

    fn get_graph_experience_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.graph_experience
    }

    fn mut_graph_experience_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.graph_experience
    }

    // repeated float graph_gold_earned = 4;

    pub fn clear_graph_gold_earned(&mut self) {
        self.graph_gold_earned.clear();
    }

    // Param is passed by value, moved
    pub fn set_graph_gold_earned(&mut self, v: ::std::vec::Vec<f32>) {
        self.graph_gold_earned = v;
    }

    // Mutable pointer to the field.
    pub fn mut_graph_gold_earned(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.graph_gold_earned
    }

    // Take field
    pub fn take_graph_gold_earned(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.graph_gold_earned, ::std::vec::Vec::new())
    }

    pub fn get_graph_gold_earned(&self) -> &[f32] {
        &self.graph_gold_earned
    }

    fn get_graph_gold_earned_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.graph_gold_earned
    }

    fn mut_graph_gold_earned_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.graph_gold_earned
    }

    // repeated float graph_net_worth = 5;

    pub fn clear_graph_net_worth(&mut self) {
        self.graph_net_worth.clear();
    }

    // Param is passed by value, moved
    pub fn set_graph_net_worth(&mut self, v: ::std::vec::Vec<f32>) {
        self.graph_net_worth = v;
    }

    // Mutable pointer to the field.
    pub fn mut_graph_net_worth(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.graph_net_worth
    }

    // Take field
    pub fn take_graph_net_worth(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.graph_net_worth, ::std::vec::Vec::new())
    }

    pub fn get_graph_net_worth(&self) -> &[f32] {
        &self.graph_net_worth
    }

    fn get_graph_net_worth_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.graph_net_worth
    }

    fn mut_graph_net_worth_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.graph_net_worth
    }

    // optional bool cm_first_pick = 6;

    pub fn clear_cm_first_pick(&mut self) {
        self.cm_first_pick = ::std::option::Option::None;
    }

    pub fn has_cm_first_pick(&self) -> bool {
        self.cm_first_pick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cm_first_pick(&mut self, v: bool) {
        self.cm_first_pick = ::std::option::Option::Some(v);
    }

    pub fn get_cm_first_pick(&self) -> bool {
        self.cm_first_pick.unwrap_or(false)
    }

    fn get_cm_first_pick_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.cm_first_pick
    }

    fn mut_cm_first_pick_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.cm_first_pick
    }

    // optional uint32 cm_captain_player_id = 7;

    pub fn clear_cm_captain_player_id(&mut self) {
        self.cm_captain_player_id = ::std::option::Option::None;
    }

    pub fn has_cm_captain_player_id(&self) -> bool {
        self.cm_captain_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cm_captain_player_id(&mut self, v: u32) {
        self.cm_captain_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_cm_captain_player_id(&self) -> u32 {
        self.cm_captain_player_id.unwrap_or(0)
    }

    fn get_cm_captain_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cm_captain_player_id
    }

    fn mut_cm_captain_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cm_captain_player_id
    }

    // repeated uint32 cm_bans = 8;

    pub fn clear_cm_bans(&mut self) {
        self.cm_bans.clear();
    }

    // Param is passed by value, moved
    pub fn set_cm_bans(&mut self, v: ::std::vec::Vec<u32>) {
        self.cm_bans = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cm_bans(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.cm_bans
    }

    // Take field
    pub fn take_cm_bans(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.cm_bans, ::std::vec::Vec::new())
    }

    pub fn get_cm_bans(&self) -> &[u32] {
        &self.cm_bans
    }

    fn get_cm_bans_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.cm_bans
    }

    fn mut_cm_bans_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.cm_bans
    }

    // repeated uint32 cm_picks = 9;

    pub fn clear_cm_picks(&mut self) {
        self.cm_picks.clear();
    }

    // Param is passed by value, moved
    pub fn set_cm_picks(&mut self, v: ::std::vec::Vec<u32>) {
        self.cm_picks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cm_picks(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.cm_picks
    }

    // Take field
    pub fn take_cm_picks(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.cm_picks, ::std::vec::Vec::new())
    }

    pub fn get_cm_picks(&self) -> &[u32] {
        &self.cm_picks
    }

    fn get_cm_picks_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.cm_picks
    }

    fn mut_cm_picks_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.cm_picks
    }

    // optional uint32 cm_penalty = 10;

    pub fn clear_cm_penalty(&mut self) {
        self.cm_penalty = ::std::option::Option::None;
    }

    pub fn has_cm_penalty(&self) -> bool {
        self.cm_penalty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cm_penalty(&mut self, v: u32) {
        self.cm_penalty = ::std::option::Option::Some(v);
    }

    pub fn get_cm_penalty(&self) -> u32 {
        self.cm_penalty.unwrap_or(0)
    }

    fn get_cm_penalty_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cm_penalty
    }

    fn mut_cm_penalty_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cm_penalty
    }
}

impl ::protobuf::Message for CDOTAMatchMetadata_Team {
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
                    self.dota_team = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.players)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.graph_experience)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.graph_gold_earned)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.graph_net_worth)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.cm_first_pick = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.cm_captain_player_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.cm_bans)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.cm_picks)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.cm_penalty = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dota_team {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.players {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += 5 * self.graph_experience.len() as u32;
        my_size += 5 * self.graph_gold_earned.len() as u32;
        my_size += 5 * self.graph_net_worth.len() as u32;
        if let Some(v) = self.cm_first_pick {
            my_size += 2;
        };
        if let Some(v) = self.cm_captain_player_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cm_bans {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.cm_picks {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.cm_penalty {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dota_team {
            os.write_uint32(1, v)?;
        };
        for v in &self.players {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.graph_experience {
            os.write_float(3, *v)?;
        };
        for v in &self.graph_gold_earned {
            os.write_float(4, *v)?;
        };
        for v in &self.graph_net_worth {
            os.write_float(5, *v)?;
        };
        if let Some(v) = self.cm_first_pick {
            os.write_bool(6, v)?;
        };
        if let Some(v) = self.cm_captain_player_id {
            os.write_uint32(7, v)?;
        };
        for v in &self.cm_bans {
            os.write_uint32(8, *v)?;
        };
        for v in &self.cm_picks {
            os.write_uint32(9, *v)?;
        };
        if let Some(v) = self.cm_penalty {
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

impl ::protobuf::MessageStatic for CDOTAMatchMetadata_Team {
    fn new() -> CDOTAMatchMetadata_Team {
        CDOTAMatchMetadata_Team::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchMetadata_Team>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dota_team",
                    CDOTAMatchMetadata_Team::get_dota_team_for_reflect,
                    CDOTAMatchMetadata_Team::mut_dota_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchMetadata_Team_Player>>(
                    "players",
                    CDOTAMatchMetadata_Team::get_players_for_reflect,
                    CDOTAMatchMetadata_Team::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "graph_experience",
                    CDOTAMatchMetadata_Team::get_graph_experience_for_reflect,
                    CDOTAMatchMetadata_Team::mut_graph_experience_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "graph_gold_earned",
                    CDOTAMatchMetadata_Team::get_graph_gold_earned_for_reflect,
                    CDOTAMatchMetadata_Team::mut_graph_gold_earned_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "graph_net_worth",
                    CDOTAMatchMetadata_Team::get_graph_net_worth_for_reflect,
                    CDOTAMatchMetadata_Team::mut_graph_net_worth_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "cm_first_pick",
                    CDOTAMatchMetadata_Team::get_cm_first_pick_for_reflect,
                    CDOTAMatchMetadata_Team::mut_cm_first_pick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cm_captain_player_id",
                    CDOTAMatchMetadata_Team::get_cm_captain_player_id_for_reflect,
                    CDOTAMatchMetadata_Team::mut_cm_captain_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cm_bans",
                    CDOTAMatchMetadata_Team::get_cm_bans_for_reflect,
                    CDOTAMatchMetadata_Team::mut_cm_bans_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cm_picks",
                    CDOTAMatchMetadata_Team::get_cm_picks_for_reflect,
                    CDOTAMatchMetadata_Team::mut_cm_picks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cm_penalty",
                    CDOTAMatchMetadata_Team::get_cm_penalty_for_reflect,
                    CDOTAMatchMetadata_Team::mut_cm_penalty_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchMetadata_Team>(
                    "CDOTAMatchMetadata_Team",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchMetadata_Team {
    fn clear(&mut self) {
        self.clear_dota_team();
        self.clear_players();
        self.clear_graph_experience();
        self.clear_graph_gold_earned();
        self.clear_graph_net_worth();
        self.clear_cm_first_pick();
        self.clear_cm_captain_player_id();
        self.clear_cm_bans();
        self.clear_cm_picks();
        self.clear_cm_penalty();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchMetadata_Team {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchMetadata_Team {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchMetadata_Team_PlayerKill {
    // message fields
    victim_slot: ::std::option::Option<u32>,
    count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchMetadata_Team_PlayerKill {}

impl CDOTAMatchMetadata_Team_PlayerKill {
    pub fn new() -> CDOTAMatchMetadata_Team_PlayerKill {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchMetadata_Team_PlayerKill {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchMetadata_Team_PlayerKill> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchMetadata_Team_PlayerKill,
        };
        unsafe {
            instance.get(CDOTAMatchMetadata_Team_PlayerKill::new)
        }
    }

    // optional uint32 victim_slot = 1;

    pub fn clear_victim_slot(&mut self) {
        self.victim_slot = ::std::option::Option::None;
    }

    pub fn has_victim_slot(&self) -> bool {
        self.victim_slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_victim_slot(&mut self, v: u32) {
        self.victim_slot = ::std::option::Option::Some(v);
    }

    pub fn get_victim_slot(&self) -> u32 {
        self.victim_slot.unwrap_or(0)
    }

    fn get_victim_slot_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.victim_slot
    }

    fn mut_victim_slot_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.victim_slot
    }

    // optional uint32 count = 2;

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
}

impl ::protobuf::Message for CDOTAMatchMetadata_Team_PlayerKill {
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
                    self.victim_slot = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.count = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.victim_slot {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.victim_slot {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.count {
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

impl ::protobuf::MessageStatic for CDOTAMatchMetadata_Team_PlayerKill {
    fn new() -> CDOTAMatchMetadata_Team_PlayerKill {
        CDOTAMatchMetadata_Team_PlayerKill::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchMetadata_Team_PlayerKill>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "victim_slot",
                    CDOTAMatchMetadata_Team_PlayerKill::get_victim_slot_for_reflect,
                    CDOTAMatchMetadata_Team_PlayerKill::mut_victim_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    CDOTAMatchMetadata_Team_PlayerKill::get_count_for_reflect,
                    CDOTAMatchMetadata_Team_PlayerKill::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchMetadata_Team_PlayerKill>(
                    "CDOTAMatchMetadata_Team_PlayerKill",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchMetadata_Team_PlayerKill {
    fn clear(&mut self) {
        self.clear_victim_slot();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchMetadata_Team_PlayerKill {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchMetadata_Team_PlayerKill {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchMetadata_Team_ItemPurchase {
    // message fields
    item_id: ::std::option::Option<u32>,
    purchase_time: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchMetadata_Team_ItemPurchase {}

impl CDOTAMatchMetadata_Team_ItemPurchase {
    pub fn new() -> CDOTAMatchMetadata_Team_ItemPurchase {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchMetadata_Team_ItemPurchase {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchMetadata_Team_ItemPurchase> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchMetadata_Team_ItemPurchase,
        };
        unsafe {
            instance.get(CDOTAMatchMetadata_Team_ItemPurchase::new)
        }
    }

    // optional uint32 item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id = ::std::option::Option::None;
    }

    pub fn has_item_id(&self) -> bool {
        self.item_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: u32) {
        self.item_id = ::std::option::Option::Some(v);
    }

    pub fn get_item_id(&self) -> u32 {
        self.item_id.unwrap_or(0)
    }

    fn get_item_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.item_id
    }

    // optional int32 purchase_time = 2;

    pub fn clear_purchase_time(&mut self) {
        self.purchase_time = ::std::option::Option::None;
    }

    pub fn has_purchase_time(&self) -> bool {
        self.purchase_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_purchase_time(&mut self, v: i32) {
        self.purchase_time = ::std::option::Option::Some(v);
    }

    pub fn get_purchase_time(&self) -> i32 {
        self.purchase_time.unwrap_or(0)
    }

    fn get_purchase_time_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.purchase_time
    }

    fn mut_purchase_time_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.purchase_time
    }
}

impl ::protobuf::Message for CDOTAMatchMetadata_Team_ItemPurchase {
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
                    self.item_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.purchase_time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.item_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.purchase_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.item_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.purchase_time {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for CDOTAMatchMetadata_Team_ItemPurchase {
    fn new() -> CDOTAMatchMetadata_Team_ItemPurchase {
        CDOTAMatchMetadata_Team_ItemPurchase::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchMetadata_Team_ItemPurchase>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_id",
                    CDOTAMatchMetadata_Team_ItemPurchase::get_item_id_for_reflect,
                    CDOTAMatchMetadata_Team_ItemPurchase::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "purchase_time",
                    CDOTAMatchMetadata_Team_ItemPurchase::get_purchase_time_for_reflect,
                    CDOTAMatchMetadata_Team_ItemPurchase::mut_purchase_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchMetadata_Team_ItemPurchase>(
                    "CDOTAMatchMetadata_Team_ItemPurchase",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchMetadata_Team_ItemPurchase {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_purchase_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchMetadata_Team_ItemPurchase {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchMetadata_Team_ItemPurchase {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchMetadata_Team_InventorySnapshot {
    // message fields
    item_id: ::std::vec::Vec<u32>,
    game_time: ::std::option::Option<i32>,
    kills: ::std::option::Option<u32>,
    deaths: ::std::option::Option<u32>,
    assists: ::std::option::Option<u32>,
    level: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchMetadata_Team_InventorySnapshot {}

impl CDOTAMatchMetadata_Team_InventorySnapshot {
    pub fn new() -> CDOTAMatchMetadata_Team_InventorySnapshot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchMetadata_Team_InventorySnapshot {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchMetadata_Team_InventorySnapshot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchMetadata_Team_InventorySnapshot,
        };
        unsafe {
            instance.get(CDOTAMatchMetadata_Team_InventorySnapshot::new)
        }
    }

    // repeated uint32 item_id = 1;

    pub fn clear_item_id(&mut self) {
        self.item_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_item_id(&mut self, v: ::std::vec::Vec<u32>) {
        self.item_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item_id(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.item_id
    }

    // Take field
    pub fn take_item_id(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.item_id, ::std::vec::Vec::new())
    }

    pub fn get_item_id(&self) -> &[u32] {
        &self.item_id
    }

    fn get_item_id_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.item_id
    }

    fn mut_item_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.item_id
    }

    // optional int32 game_time = 2;

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

    // optional uint32 level = 6;

    pub fn clear_level(&mut self) {
        self.level = ::std::option::Option::None;
    }

    pub fn has_level(&self) -> bool {
        self.level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_level(&mut self, v: u32) {
        self.level = ::std::option::Option::Some(v);
    }

    pub fn get_level(&self) -> u32 {
        self.level.unwrap_or(0)
    }

    fn get_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.level
    }

    fn mut_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.level
    }
}

impl ::protobuf::Message for CDOTAMatchMetadata_Team_InventorySnapshot {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.item_id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_int32()?;
                    self.game_time = ::std::option::Option::Some(tmp);
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.level = ::std::option::Option::Some(tmp);
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
        for value in &self.item_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.game_time {
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
        if let Some(v) = self.level {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.item_id {
            os.write_uint32(1, *v)?;
        };
        if let Some(v) = self.game_time {
            os.write_int32(2, v)?;
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
        if let Some(v) = self.level {
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

impl ::protobuf::MessageStatic for CDOTAMatchMetadata_Team_InventorySnapshot {
    fn new() -> CDOTAMatchMetadata_Team_InventorySnapshot {
        CDOTAMatchMetadata_Team_InventorySnapshot::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchMetadata_Team_InventorySnapshot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "item_id",
                    CDOTAMatchMetadata_Team_InventorySnapshot::get_item_id_for_reflect,
                    CDOTAMatchMetadata_Team_InventorySnapshot::mut_item_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "game_time",
                    CDOTAMatchMetadata_Team_InventorySnapshot::get_game_time_for_reflect,
                    CDOTAMatchMetadata_Team_InventorySnapshot::mut_game_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "kills",
                    CDOTAMatchMetadata_Team_InventorySnapshot::get_kills_for_reflect,
                    CDOTAMatchMetadata_Team_InventorySnapshot::mut_kills_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "deaths",
                    CDOTAMatchMetadata_Team_InventorySnapshot::get_deaths_for_reflect,
                    CDOTAMatchMetadata_Team_InventorySnapshot::mut_deaths_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "assists",
                    CDOTAMatchMetadata_Team_InventorySnapshot::get_assists_for_reflect,
                    CDOTAMatchMetadata_Team_InventorySnapshot::mut_assists_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "level",
                    CDOTAMatchMetadata_Team_InventorySnapshot::get_level_for_reflect,
                    CDOTAMatchMetadata_Team_InventorySnapshot::mut_level_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchMetadata_Team_InventorySnapshot>(
                    "CDOTAMatchMetadata_Team_InventorySnapshot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchMetadata_Team_InventorySnapshot {
    fn clear(&mut self) {
        self.clear_item_id();
        self.clear_game_time();
        self.clear_kills();
        self.clear_deaths();
        self.clear_assists();
        self.clear_level();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchMetadata_Team_InventorySnapshot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchMetadata_Team_InventorySnapshot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchMetadata_Team_AutoStyleCriteria {
    // message fields
    name_token: ::std::option::Option<u32>,
    value: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchMetadata_Team_AutoStyleCriteria {}

impl CDOTAMatchMetadata_Team_AutoStyleCriteria {
    pub fn new() -> CDOTAMatchMetadata_Team_AutoStyleCriteria {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchMetadata_Team_AutoStyleCriteria {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchMetadata_Team_AutoStyleCriteria> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchMetadata_Team_AutoStyleCriteria,
        };
        unsafe {
            instance.get(CDOTAMatchMetadata_Team_AutoStyleCriteria::new)
        }
    }

    // optional uint32 name_token = 1;

    pub fn clear_name_token(&mut self) {
        self.name_token = ::std::option::Option::None;
    }

    pub fn has_name_token(&self) -> bool {
        self.name_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name_token(&mut self, v: u32) {
        self.name_token = ::std::option::Option::Some(v);
    }

    pub fn get_name_token(&self) -> u32 {
        self.name_token.unwrap_or(0)
    }

    fn get_name_token_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.name_token
    }

    fn mut_name_token_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.name_token
    }

    // optional float value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> f32 {
        self.value.unwrap_or(0.)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.value
    }
}

impl ::protobuf::Message for CDOTAMatchMetadata_Team_AutoStyleCriteria {
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
                    self.name_token = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.value = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.name_token {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.value {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name_token {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.value {
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

impl ::protobuf::MessageStatic for CDOTAMatchMetadata_Team_AutoStyleCriteria {
    fn new() -> CDOTAMatchMetadata_Team_AutoStyleCriteria {
        CDOTAMatchMetadata_Team_AutoStyleCriteria::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchMetadata_Team_AutoStyleCriteria>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "name_token",
                    CDOTAMatchMetadata_Team_AutoStyleCriteria::get_name_token_for_reflect,
                    CDOTAMatchMetadata_Team_AutoStyleCriteria::mut_name_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "value",
                    CDOTAMatchMetadata_Team_AutoStyleCriteria::get_value_for_reflect,
                    CDOTAMatchMetadata_Team_AutoStyleCriteria::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchMetadata_Team_AutoStyleCriteria>(
                    "CDOTAMatchMetadata_Team_AutoStyleCriteria",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchMetadata_Team_AutoStyleCriteria {
    fn clear(&mut self) {
        self.clear_name_token();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchMetadata_Team_AutoStyleCriteria {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchMetadata_Team_AutoStyleCriteria {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchMetadata_Team_Player {
    // message fields
    account_id: ::std::option::Option<u32>,
    ability_upgrades: ::std::vec::Vec<u32>,
    player_slot: ::std::option::Option<u32>,
    equipped_econ_items: ::protobuf::RepeatedField<super::base_gcmessages::CSOEconItem>,
    kills: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_PlayerKill>,
    items: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_ItemPurchase>,
    avg_kills_x16: ::std::option::Option<u32>,
    avg_deaths_x16: ::std::option::Option<u32>,
    avg_assists_x16: ::std::option::Option<u32>,
    avg_gpm_x16: ::std::option::Option<u32>,
    avg_xpm_x16: ::std::option::Option<u32>,
    best_kills_x16: ::std::option::Option<u32>,
    best_assists_x16: ::std::option::Option<u32>,
    best_gpm_x16: ::std::option::Option<u32>,
    best_xpm_x16: ::std::option::Option<u32>,
    win_streak: ::std::option::Option<u32>,
    best_win_streak: ::std::option::Option<u32>,
    fight_score: ::std::option::Option<f32>,
    farm_score: ::std::option::Option<f32>,
    support_score: ::std::option::Option<f32>,
    push_score: ::std::option::Option<f32>,
    level_up_times: ::std::vec::Vec<u32>,
    graph_net_worth: ::std::vec::Vec<f32>,
    inventory_snapshot: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_InventorySnapshot>,
    avg_stats_calibrated: ::std::option::Option<bool>,
    auto_style_criteria: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_AutoStyleCriteria>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchMetadata_Team_Player {}

impl CDOTAMatchMetadata_Team_Player {
    pub fn new() -> CDOTAMatchMetadata_Team_Player {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchMetadata_Team_Player {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchMetadata_Team_Player> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchMetadata_Team_Player,
        };
        unsafe {
            instance.get(CDOTAMatchMetadata_Team_Player::new)
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

    // repeated uint32 ability_upgrades = 2;

    pub fn clear_ability_upgrades(&mut self) {
        self.ability_upgrades.clear();
    }

    // Param is passed by value, moved
    pub fn set_ability_upgrades(&mut self, v: ::std::vec::Vec<u32>) {
        self.ability_upgrades = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ability_upgrades(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ability_upgrades
    }

    // Take field
    pub fn take_ability_upgrades(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ability_upgrades, ::std::vec::Vec::new())
    }

    pub fn get_ability_upgrades(&self) -> &[u32] {
        &self.ability_upgrades
    }

    fn get_ability_upgrades_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ability_upgrades
    }

    fn mut_ability_upgrades_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ability_upgrades
    }

    // optional uint32 player_slot = 3;

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

    // repeated .dota.CSOEconItem equipped_econ_items = 4;

    pub fn clear_equipped_econ_items(&mut self) {
        self.equipped_econ_items.clear();
    }

    // Param is passed by value, moved
    pub fn set_equipped_econ_items(&mut self, v: ::protobuf::RepeatedField<super::base_gcmessages::CSOEconItem>) {
        self.equipped_econ_items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_equipped_econ_items(&mut self) -> &mut ::protobuf::RepeatedField<super::base_gcmessages::CSOEconItem> {
        &mut self.equipped_econ_items
    }

    // Take field
    pub fn take_equipped_econ_items(&mut self) -> ::protobuf::RepeatedField<super::base_gcmessages::CSOEconItem> {
        ::std::mem::replace(&mut self.equipped_econ_items, ::protobuf::RepeatedField::new())
    }

    pub fn get_equipped_econ_items(&self) -> &[super::base_gcmessages::CSOEconItem] {
        &self.equipped_econ_items
    }

    fn get_equipped_econ_items_for_reflect(&self) -> &::protobuf::RepeatedField<super::base_gcmessages::CSOEconItem> {
        &self.equipped_econ_items
    }

    fn mut_equipped_econ_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::base_gcmessages::CSOEconItem> {
        &mut self.equipped_econ_items
    }

    // repeated .dota.CDOTAMatchMetadata.Team.PlayerKill kills = 5;

    pub fn clear_kills(&mut self) {
        self.kills.clear();
    }

    // Param is passed by value, moved
    pub fn set_kills(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_PlayerKill>) {
        self.kills = v;
    }

    // Mutable pointer to the field.
    pub fn mut_kills(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_PlayerKill> {
        &mut self.kills
    }

    // Take field
    pub fn take_kills(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_PlayerKill> {
        ::std::mem::replace(&mut self.kills, ::protobuf::RepeatedField::new())
    }

    pub fn get_kills(&self) -> &[CDOTAMatchMetadata_Team_PlayerKill] {
        &self.kills
    }

    fn get_kills_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchMetadata_Team_PlayerKill> {
        &self.kills
    }

    fn mut_kills_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_PlayerKill> {
        &mut self.kills
    }

    // repeated .dota.CDOTAMatchMetadata.Team.ItemPurchase items = 6;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_ItemPurchase>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_ItemPurchase> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_ItemPurchase> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CDOTAMatchMetadata_Team_ItemPurchase] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchMetadata_Team_ItemPurchase> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_ItemPurchase> {
        &mut self.items
    }

    // optional uint32 avg_kills_x16 = 7;

    pub fn clear_avg_kills_x16(&mut self) {
        self.avg_kills_x16 = ::std::option::Option::None;
    }

    pub fn has_avg_kills_x16(&self) -> bool {
        self.avg_kills_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_kills_x16(&mut self, v: u32) {
        self.avg_kills_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_avg_kills_x16(&self) -> u32 {
        self.avg_kills_x16.unwrap_or(0)
    }

    fn get_avg_kills_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.avg_kills_x16
    }

    fn mut_avg_kills_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.avg_kills_x16
    }

    // optional uint32 avg_deaths_x16 = 8;

    pub fn clear_avg_deaths_x16(&mut self) {
        self.avg_deaths_x16 = ::std::option::Option::None;
    }

    pub fn has_avg_deaths_x16(&self) -> bool {
        self.avg_deaths_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_deaths_x16(&mut self, v: u32) {
        self.avg_deaths_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_avg_deaths_x16(&self) -> u32 {
        self.avg_deaths_x16.unwrap_or(0)
    }

    fn get_avg_deaths_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.avg_deaths_x16
    }

    fn mut_avg_deaths_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.avg_deaths_x16
    }

    // optional uint32 avg_assists_x16 = 9;

    pub fn clear_avg_assists_x16(&mut self) {
        self.avg_assists_x16 = ::std::option::Option::None;
    }

    pub fn has_avg_assists_x16(&self) -> bool {
        self.avg_assists_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_assists_x16(&mut self, v: u32) {
        self.avg_assists_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_avg_assists_x16(&self) -> u32 {
        self.avg_assists_x16.unwrap_or(0)
    }

    fn get_avg_assists_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.avg_assists_x16
    }

    fn mut_avg_assists_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.avg_assists_x16
    }

    // optional uint32 avg_gpm_x16 = 10;

    pub fn clear_avg_gpm_x16(&mut self) {
        self.avg_gpm_x16 = ::std::option::Option::None;
    }

    pub fn has_avg_gpm_x16(&self) -> bool {
        self.avg_gpm_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_gpm_x16(&mut self, v: u32) {
        self.avg_gpm_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_avg_gpm_x16(&self) -> u32 {
        self.avg_gpm_x16.unwrap_or(0)
    }

    fn get_avg_gpm_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.avg_gpm_x16
    }

    fn mut_avg_gpm_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.avg_gpm_x16
    }

    // optional uint32 avg_xpm_x16 = 11;

    pub fn clear_avg_xpm_x16(&mut self) {
        self.avg_xpm_x16 = ::std::option::Option::None;
    }

    pub fn has_avg_xpm_x16(&self) -> bool {
        self.avg_xpm_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_xpm_x16(&mut self, v: u32) {
        self.avg_xpm_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_avg_xpm_x16(&self) -> u32 {
        self.avg_xpm_x16.unwrap_or(0)
    }

    fn get_avg_xpm_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.avg_xpm_x16
    }

    fn mut_avg_xpm_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.avg_xpm_x16
    }

    // optional uint32 best_kills_x16 = 12;

    pub fn clear_best_kills_x16(&mut self) {
        self.best_kills_x16 = ::std::option::Option::None;
    }

    pub fn has_best_kills_x16(&self) -> bool {
        self.best_kills_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_best_kills_x16(&mut self, v: u32) {
        self.best_kills_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_best_kills_x16(&self) -> u32 {
        self.best_kills_x16.unwrap_or(0)
    }

    fn get_best_kills_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.best_kills_x16
    }

    fn mut_best_kills_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.best_kills_x16
    }

    // optional uint32 best_assists_x16 = 13;

    pub fn clear_best_assists_x16(&mut self) {
        self.best_assists_x16 = ::std::option::Option::None;
    }

    pub fn has_best_assists_x16(&self) -> bool {
        self.best_assists_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_best_assists_x16(&mut self, v: u32) {
        self.best_assists_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_best_assists_x16(&self) -> u32 {
        self.best_assists_x16.unwrap_or(0)
    }

    fn get_best_assists_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.best_assists_x16
    }

    fn mut_best_assists_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.best_assists_x16
    }

    // optional uint32 best_gpm_x16 = 14;

    pub fn clear_best_gpm_x16(&mut self) {
        self.best_gpm_x16 = ::std::option::Option::None;
    }

    pub fn has_best_gpm_x16(&self) -> bool {
        self.best_gpm_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_best_gpm_x16(&mut self, v: u32) {
        self.best_gpm_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_best_gpm_x16(&self) -> u32 {
        self.best_gpm_x16.unwrap_or(0)
    }

    fn get_best_gpm_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.best_gpm_x16
    }

    fn mut_best_gpm_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.best_gpm_x16
    }

    // optional uint32 best_xpm_x16 = 15;

    pub fn clear_best_xpm_x16(&mut self) {
        self.best_xpm_x16 = ::std::option::Option::None;
    }

    pub fn has_best_xpm_x16(&self) -> bool {
        self.best_xpm_x16.is_some()
    }

    // Param is passed by value, moved
    pub fn set_best_xpm_x16(&mut self, v: u32) {
        self.best_xpm_x16 = ::std::option::Option::Some(v);
    }

    pub fn get_best_xpm_x16(&self) -> u32 {
        self.best_xpm_x16.unwrap_or(0)
    }

    fn get_best_xpm_x16_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.best_xpm_x16
    }

    fn mut_best_xpm_x16_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.best_xpm_x16
    }

    // optional uint32 win_streak = 16;

    pub fn clear_win_streak(&mut self) {
        self.win_streak = ::std::option::Option::None;
    }

    pub fn has_win_streak(&self) -> bool {
        self.win_streak.is_some()
    }

    // Param is passed by value, moved
    pub fn set_win_streak(&mut self, v: u32) {
        self.win_streak = ::std::option::Option::Some(v);
    }

    pub fn get_win_streak(&self) -> u32 {
        self.win_streak.unwrap_or(0)
    }

    fn get_win_streak_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.win_streak
    }

    fn mut_win_streak_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.win_streak
    }

    // optional uint32 best_win_streak = 17;

    pub fn clear_best_win_streak(&mut self) {
        self.best_win_streak = ::std::option::Option::None;
    }

    pub fn has_best_win_streak(&self) -> bool {
        self.best_win_streak.is_some()
    }

    // Param is passed by value, moved
    pub fn set_best_win_streak(&mut self, v: u32) {
        self.best_win_streak = ::std::option::Option::Some(v);
    }

    pub fn get_best_win_streak(&self) -> u32 {
        self.best_win_streak.unwrap_or(0)
    }

    fn get_best_win_streak_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.best_win_streak
    }

    fn mut_best_win_streak_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.best_win_streak
    }

    // optional float fight_score = 18;

    pub fn clear_fight_score(&mut self) {
        self.fight_score = ::std::option::Option::None;
    }

    pub fn has_fight_score(&self) -> bool {
        self.fight_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fight_score(&mut self, v: f32) {
        self.fight_score = ::std::option::Option::Some(v);
    }

    pub fn get_fight_score(&self) -> f32 {
        self.fight_score.unwrap_or(0.)
    }

    fn get_fight_score_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fight_score
    }

    fn mut_fight_score_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fight_score
    }

    // optional float farm_score = 19;

    pub fn clear_farm_score(&mut self) {
        self.farm_score = ::std::option::Option::None;
    }

    pub fn has_farm_score(&self) -> bool {
        self.farm_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_farm_score(&mut self, v: f32) {
        self.farm_score = ::std::option::Option::Some(v);
    }

    pub fn get_farm_score(&self) -> f32 {
        self.farm_score.unwrap_or(0.)
    }

    fn get_farm_score_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.farm_score
    }

    fn mut_farm_score_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.farm_score
    }

    // optional float support_score = 20;

    pub fn clear_support_score(&mut self) {
        self.support_score = ::std::option::Option::None;
    }

    pub fn has_support_score(&self) -> bool {
        self.support_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_support_score(&mut self, v: f32) {
        self.support_score = ::std::option::Option::Some(v);
    }

    pub fn get_support_score(&self) -> f32 {
        self.support_score.unwrap_or(0.)
    }

    fn get_support_score_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.support_score
    }

    fn mut_support_score_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.support_score
    }

    // optional float push_score = 21;

    pub fn clear_push_score(&mut self) {
        self.push_score = ::std::option::Option::None;
    }

    pub fn has_push_score(&self) -> bool {
        self.push_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_push_score(&mut self, v: f32) {
        self.push_score = ::std::option::Option::Some(v);
    }

    pub fn get_push_score(&self) -> f32 {
        self.push_score.unwrap_or(0.)
    }

    fn get_push_score_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.push_score
    }

    fn mut_push_score_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.push_score
    }

    // repeated uint32 level_up_times = 22;

    pub fn clear_level_up_times(&mut self) {
        self.level_up_times.clear();
    }

    // Param is passed by value, moved
    pub fn set_level_up_times(&mut self, v: ::std::vec::Vec<u32>) {
        self.level_up_times = v;
    }

    // Mutable pointer to the field.
    pub fn mut_level_up_times(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.level_up_times
    }

    // Take field
    pub fn take_level_up_times(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.level_up_times, ::std::vec::Vec::new())
    }

    pub fn get_level_up_times(&self) -> &[u32] {
        &self.level_up_times
    }

    fn get_level_up_times_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.level_up_times
    }

    fn mut_level_up_times_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.level_up_times
    }

    // repeated float graph_net_worth = 23;

    pub fn clear_graph_net_worth(&mut self) {
        self.graph_net_worth.clear();
    }

    // Param is passed by value, moved
    pub fn set_graph_net_worth(&mut self, v: ::std::vec::Vec<f32>) {
        self.graph_net_worth = v;
    }

    // Mutable pointer to the field.
    pub fn mut_graph_net_worth(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.graph_net_worth
    }

    // Take field
    pub fn take_graph_net_worth(&mut self) -> ::std::vec::Vec<f32> {
        ::std::mem::replace(&mut self.graph_net_worth, ::std::vec::Vec::new())
    }

    pub fn get_graph_net_worth(&self) -> &[f32] {
        &self.graph_net_worth
    }

    fn get_graph_net_worth_for_reflect(&self) -> &::std::vec::Vec<f32> {
        &self.graph_net_worth
    }

    fn mut_graph_net_worth_for_reflect(&mut self) -> &mut ::std::vec::Vec<f32> {
        &mut self.graph_net_worth
    }

    // repeated .dota.CDOTAMatchMetadata.Team.InventorySnapshot inventory_snapshot = 24;

    pub fn clear_inventory_snapshot(&mut self) {
        self.inventory_snapshot.clear();
    }

    // Param is passed by value, moved
    pub fn set_inventory_snapshot(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_InventorySnapshot>) {
        self.inventory_snapshot = v;
    }

    // Mutable pointer to the field.
    pub fn mut_inventory_snapshot(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_InventorySnapshot> {
        &mut self.inventory_snapshot
    }

    // Take field
    pub fn take_inventory_snapshot(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_InventorySnapshot> {
        ::std::mem::replace(&mut self.inventory_snapshot, ::protobuf::RepeatedField::new())
    }

    pub fn get_inventory_snapshot(&self) -> &[CDOTAMatchMetadata_Team_InventorySnapshot] {
        &self.inventory_snapshot
    }

    fn get_inventory_snapshot_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchMetadata_Team_InventorySnapshot> {
        &self.inventory_snapshot
    }

    fn mut_inventory_snapshot_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_InventorySnapshot> {
        &mut self.inventory_snapshot
    }

    // optional bool avg_stats_calibrated = 25;

    pub fn clear_avg_stats_calibrated(&mut self) {
        self.avg_stats_calibrated = ::std::option::Option::None;
    }

    pub fn has_avg_stats_calibrated(&self) -> bool {
        self.avg_stats_calibrated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_avg_stats_calibrated(&mut self, v: bool) {
        self.avg_stats_calibrated = ::std::option::Option::Some(v);
    }

    pub fn get_avg_stats_calibrated(&self) -> bool {
        self.avg_stats_calibrated.unwrap_or(false)
    }

    fn get_avg_stats_calibrated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.avg_stats_calibrated
    }

    fn mut_avg_stats_calibrated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.avg_stats_calibrated
    }

    // repeated .dota.CDOTAMatchMetadata.Team.AutoStyleCriteria auto_style_criteria = 26;

    pub fn clear_auto_style_criteria(&mut self) {
        self.auto_style_criteria.clear();
    }

    // Param is passed by value, moved
    pub fn set_auto_style_criteria(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_AutoStyleCriteria>) {
        self.auto_style_criteria = v;
    }

    // Mutable pointer to the field.
    pub fn mut_auto_style_criteria(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_AutoStyleCriteria> {
        &mut self.auto_style_criteria
    }

    // Take field
    pub fn take_auto_style_criteria(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_AutoStyleCriteria> {
        ::std::mem::replace(&mut self.auto_style_criteria, ::protobuf::RepeatedField::new())
    }

    pub fn get_auto_style_criteria(&self) -> &[CDOTAMatchMetadata_Team_AutoStyleCriteria] {
        &self.auto_style_criteria
    }

    fn get_auto_style_criteria_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchMetadata_Team_AutoStyleCriteria> {
        &self.auto_style_criteria
    }

    fn mut_auto_style_criteria_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchMetadata_Team_AutoStyleCriteria> {
        &mut self.auto_style_criteria
    }
}

impl ::protobuf::Message for CDOTAMatchMetadata_Team_Player {
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
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.ability_upgrades)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.player_slot = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.equipped_econ_items)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.kills)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.avg_kills_x16 = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.avg_deaths_x16 = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.avg_assists_x16 = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.avg_gpm_x16 = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.avg_xpm_x16 = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.best_kills_x16 = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.best_assists_x16 = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.best_gpm_x16 = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.best_xpm_x16 = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.win_streak = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.best_win_streak = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.fight_score = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.farm_score = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.support_score = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.push_score = ::std::option::Option::Some(tmp);
                },
                22 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.level_up_times)?;
                },
                23 => {
                    ::protobuf::rt::read_repeated_float_into(wire_type, is, &mut self.graph_net_worth)?;
                },
                24 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.inventory_snapshot)?;
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.avg_stats_calibrated = ::std::option::Option::Some(tmp);
                },
                26 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.auto_style_criteria)?;
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
        for value in &self.ability_upgrades {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.player_slot {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.equipped_econ_items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.kills {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.avg_kills_x16 {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.avg_deaths_x16 {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.avg_assists_x16 {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.avg_gpm_x16 {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.avg_xpm_x16 {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.best_kills_x16 {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.best_assists_x16 {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.best_gpm_x16 {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.best_xpm_x16 {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.win_streak {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.best_win_streak {
            my_size += ::protobuf::rt::value_size(17, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.fight_score {
            my_size += 6;
        };
        if let Some(v) = self.farm_score {
            my_size += 6;
        };
        if let Some(v) = self.support_score {
            my_size += 6;
        };
        if let Some(v) = self.push_score {
            my_size += 6;
        };
        for value in &self.level_up_times {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += 6 * self.graph_net_worth.len() as u32;
        for value in &self.inventory_snapshot {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.avg_stats_calibrated {
            my_size += 3;
        };
        for value in &self.auto_style_criteria {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        for v in &self.ability_upgrades {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.player_slot {
            os.write_uint32(3, v)?;
        };
        for v in &self.equipped_econ_items {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.kills {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.items {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.avg_kills_x16 {
            os.write_uint32(7, v)?;
        };
        if let Some(v) = self.avg_deaths_x16 {
            os.write_uint32(8, v)?;
        };
        if let Some(v) = self.avg_assists_x16 {
            os.write_uint32(9, v)?;
        };
        if let Some(v) = self.avg_gpm_x16 {
            os.write_uint32(10, v)?;
        };
        if let Some(v) = self.avg_xpm_x16 {
            os.write_uint32(11, v)?;
        };
        if let Some(v) = self.best_kills_x16 {
            os.write_uint32(12, v)?;
        };
        if let Some(v) = self.best_assists_x16 {
            os.write_uint32(13, v)?;
        };
        if let Some(v) = self.best_gpm_x16 {
            os.write_uint32(14, v)?;
        };
        if let Some(v) = self.best_xpm_x16 {
            os.write_uint32(15, v)?;
        };
        if let Some(v) = self.win_streak {
            os.write_uint32(16, v)?;
        };
        if let Some(v) = self.best_win_streak {
            os.write_uint32(17, v)?;
        };
        if let Some(v) = self.fight_score {
            os.write_float(18, v)?;
        };
        if let Some(v) = self.farm_score {
            os.write_float(19, v)?;
        };
        if let Some(v) = self.support_score {
            os.write_float(20, v)?;
        };
        if let Some(v) = self.push_score {
            os.write_float(21, v)?;
        };
        for v in &self.level_up_times {
            os.write_uint32(22, *v)?;
        };
        for v in &self.graph_net_worth {
            os.write_float(23, *v)?;
        };
        for v in &self.inventory_snapshot {
            os.write_tag(24, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.avg_stats_calibrated {
            os.write_bool(25, v)?;
        };
        for v in &self.auto_style_criteria {
            os.write_tag(26, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CDOTAMatchMetadata_Team_Player {
    fn new() -> CDOTAMatchMetadata_Team_Player {
        CDOTAMatchMetadata_Team_Player::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchMetadata_Team_Player>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CDOTAMatchMetadata_Team_Player::get_account_id_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_upgrades",
                    CDOTAMatchMetadata_Team_Player::get_ability_upgrades_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_ability_upgrades_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_slot",
                    CDOTAMatchMetadata_Team_Player::get_player_slot_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_player_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::base_gcmessages::CSOEconItem>>(
                    "equipped_econ_items",
                    CDOTAMatchMetadata_Team_Player::get_equipped_econ_items_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_equipped_econ_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchMetadata_Team_PlayerKill>>(
                    "kills",
                    CDOTAMatchMetadata_Team_Player::get_kills_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_kills_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchMetadata_Team_ItemPurchase>>(
                    "items",
                    CDOTAMatchMetadata_Team_Player::get_items_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "avg_kills_x16",
                    CDOTAMatchMetadata_Team_Player::get_avg_kills_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_avg_kills_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "avg_deaths_x16",
                    CDOTAMatchMetadata_Team_Player::get_avg_deaths_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_avg_deaths_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "avg_assists_x16",
                    CDOTAMatchMetadata_Team_Player::get_avg_assists_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_avg_assists_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "avg_gpm_x16",
                    CDOTAMatchMetadata_Team_Player::get_avg_gpm_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_avg_gpm_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "avg_xpm_x16",
                    CDOTAMatchMetadata_Team_Player::get_avg_xpm_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_avg_xpm_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "best_kills_x16",
                    CDOTAMatchMetadata_Team_Player::get_best_kills_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_best_kills_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "best_assists_x16",
                    CDOTAMatchMetadata_Team_Player::get_best_assists_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_best_assists_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "best_gpm_x16",
                    CDOTAMatchMetadata_Team_Player::get_best_gpm_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_best_gpm_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "best_xpm_x16",
                    CDOTAMatchMetadata_Team_Player::get_best_xpm_x16_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_best_xpm_x16_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "win_streak",
                    CDOTAMatchMetadata_Team_Player::get_win_streak_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_win_streak_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "best_win_streak",
                    CDOTAMatchMetadata_Team_Player::get_best_win_streak_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_best_win_streak_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fight_score",
                    CDOTAMatchMetadata_Team_Player::get_fight_score_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_fight_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "farm_score",
                    CDOTAMatchMetadata_Team_Player::get_farm_score_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_farm_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "support_score",
                    CDOTAMatchMetadata_Team_Player::get_support_score_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_support_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "push_score",
                    CDOTAMatchMetadata_Team_Player::get_push_score_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_push_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "level_up_times",
                    CDOTAMatchMetadata_Team_Player::get_level_up_times_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_level_up_times_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "graph_net_worth",
                    CDOTAMatchMetadata_Team_Player::get_graph_net_worth_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_graph_net_worth_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchMetadata_Team_InventorySnapshot>>(
                    "inventory_snapshot",
                    CDOTAMatchMetadata_Team_Player::get_inventory_snapshot_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_inventory_snapshot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "avg_stats_calibrated",
                    CDOTAMatchMetadata_Team_Player::get_avg_stats_calibrated_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_avg_stats_calibrated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchMetadata_Team_AutoStyleCriteria>>(
                    "auto_style_criteria",
                    CDOTAMatchMetadata_Team_Player::get_auto_style_criteria_for_reflect,
                    CDOTAMatchMetadata_Team_Player::mut_auto_style_criteria_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchMetadata_Team_Player>(
                    "CDOTAMatchMetadata_Team_Player",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchMetadata_Team_Player {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_ability_upgrades();
        self.clear_player_slot();
        self.clear_equipped_econ_items();
        self.clear_kills();
        self.clear_items();
        self.clear_avg_kills_x16();
        self.clear_avg_deaths_x16();
        self.clear_avg_assists_x16();
        self.clear_avg_gpm_x16();
        self.clear_avg_xpm_x16();
        self.clear_best_kills_x16();
        self.clear_best_assists_x16();
        self.clear_best_gpm_x16();
        self.clear_best_xpm_x16();
        self.clear_win_streak();
        self.clear_best_win_streak();
        self.clear_fight_score();
        self.clear_farm_score();
        self.clear_support_score();
        self.clear_push_score();
        self.clear_level_up_times();
        self.clear_graph_net_worth();
        self.clear_inventory_snapshot();
        self.clear_avg_stats_calibrated();
        self.clear_auto_style_criteria();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchMetadata_Team_Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchMetadata_Team_Player {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchPrivateMetadata {
    // message fields
    teams: ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchPrivateMetadata {}

impl CDOTAMatchPrivateMetadata {
    pub fn new() -> CDOTAMatchPrivateMetadata {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchPrivateMetadata {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchPrivateMetadata> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchPrivateMetadata,
        };
        unsafe {
            instance.get(CDOTAMatchPrivateMetadata::new)
        }
    }

    // repeated .dota.CDOTAMatchPrivateMetadata.Team teams = 1;

    pub fn clear_teams(&mut self) {
        self.teams.clear();
    }

    // Param is passed by value, moved
    pub fn set_teams(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team>) {
        self.teams = v;
    }

    // Mutable pointer to the field.
    pub fn mut_teams(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team> {
        &mut self.teams
    }

    // Take field
    pub fn take_teams(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team> {
        ::std::mem::replace(&mut self.teams, ::protobuf::RepeatedField::new())
    }

    pub fn get_teams(&self) -> &[CDOTAMatchPrivateMetadata_Team] {
        &self.teams
    }

    fn get_teams_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team> {
        &self.teams
    }

    fn mut_teams_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team> {
        &mut self.teams
    }
}

impl ::protobuf::Message for CDOTAMatchPrivateMetadata {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.teams)?;
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
        for value in &self.teams {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.teams {
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

impl ::protobuf::MessageStatic for CDOTAMatchPrivateMetadata {
    fn new() -> CDOTAMatchPrivateMetadata {
        CDOTAMatchPrivateMetadata::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchPrivateMetadata>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchPrivateMetadata_Team>>(
                    "teams",
                    CDOTAMatchPrivateMetadata::get_teams_for_reflect,
                    CDOTAMatchPrivateMetadata::mut_teams_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchPrivateMetadata>(
                    "CDOTAMatchPrivateMetadata",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchPrivateMetadata {
    fn clear(&mut self) {
        self.clear_teams();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchPrivateMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchPrivateMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchPrivateMetadata_Team {
    // message fields
    dota_team: ::std::option::Option<u32>,
    players: ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Player>,
    buildings: ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Building>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchPrivateMetadata_Team {}

impl CDOTAMatchPrivateMetadata_Team {
    pub fn new() -> CDOTAMatchPrivateMetadata_Team {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchPrivateMetadata_Team {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchPrivateMetadata_Team> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchPrivateMetadata_Team,
        };
        unsafe {
            instance.get(CDOTAMatchPrivateMetadata_Team::new)
        }
    }

    // optional uint32 dota_team = 1;

    pub fn clear_dota_team(&mut self) {
        self.dota_team = ::std::option::Option::None;
    }

    pub fn has_dota_team(&self) -> bool {
        self.dota_team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dota_team(&mut self, v: u32) {
        self.dota_team = ::std::option::Option::Some(v);
    }

    pub fn get_dota_team(&self) -> u32 {
        self.dota_team.unwrap_or(0)
    }

    fn get_dota_team_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dota_team
    }

    fn mut_dota_team_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dota_team
    }

    // repeated .dota.CDOTAMatchPrivateMetadata.Team.Player players = 2;

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Player>) {
        self.players = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Player> {
        &mut self.players
    }

    // Take field
    pub fn take_players(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Player> {
        ::std::mem::replace(&mut self.players, ::protobuf::RepeatedField::new())
    }

    pub fn get_players(&self) -> &[CDOTAMatchPrivateMetadata_Team_Player] {
        &self.players
    }

    fn get_players_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Player> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Player> {
        &mut self.players
    }

    // repeated .dota.CDOTAMatchPrivateMetadata.Team.Building buildings = 3;

    pub fn clear_buildings(&mut self) {
        self.buildings.clear();
    }

    // Param is passed by value, moved
    pub fn set_buildings(&mut self, v: ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Building>) {
        self.buildings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buildings(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Building> {
        &mut self.buildings
    }

    // Take field
    pub fn take_buildings(&mut self) -> ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Building> {
        ::std::mem::replace(&mut self.buildings, ::protobuf::RepeatedField::new())
    }

    pub fn get_buildings(&self) -> &[CDOTAMatchPrivateMetadata_Team_Building] {
        &self.buildings
    }

    fn get_buildings_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Building> {
        &self.buildings
    }

    fn mut_buildings_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTAMatchPrivateMetadata_Team_Building> {
        &mut self.buildings
    }
}

impl ::protobuf::Message for CDOTAMatchPrivateMetadata_Team {
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
                    self.dota_team = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.players)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.buildings)?;
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
        if let Some(v) = self.dota_team {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.players {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.buildings {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dota_team {
            os.write_uint32(1, v)?;
        };
        for v in &self.players {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.buildings {
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

impl ::protobuf::MessageStatic for CDOTAMatchPrivateMetadata_Team {
    fn new() -> CDOTAMatchPrivateMetadata_Team {
        CDOTAMatchPrivateMetadata_Team::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchPrivateMetadata_Team>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dota_team",
                    CDOTAMatchPrivateMetadata_Team::get_dota_team_for_reflect,
                    CDOTAMatchPrivateMetadata_Team::mut_dota_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchPrivateMetadata_Team_Player>>(
                    "players",
                    CDOTAMatchPrivateMetadata_Team::get_players_for_reflect,
                    CDOTAMatchPrivateMetadata_Team::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTAMatchPrivateMetadata_Team_Building>>(
                    "buildings",
                    CDOTAMatchPrivateMetadata_Team::get_buildings_for_reflect,
                    CDOTAMatchPrivateMetadata_Team::mut_buildings_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchPrivateMetadata_Team>(
                    "CDOTAMatchPrivateMetadata_Team",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchPrivateMetadata_Team {
    fn clear(&mut self) {
        self.clear_dota_team();
        self.clear_players();
        self.clear_buildings();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchPrivateMetadata_Team {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchPrivateMetadata_Team {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchPrivateMetadata_Team_Player {
    // message fields
    account_id: ::std::option::Option<u32>,
    player_slot: ::std::option::Option<u32>,
    position_stream: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchPrivateMetadata_Team_Player {}

impl CDOTAMatchPrivateMetadata_Team_Player {
    pub fn new() -> CDOTAMatchPrivateMetadata_Team_Player {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchPrivateMetadata_Team_Player {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchPrivateMetadata_Team_Player> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchPrivateMetadata_Team_Player,
        };
        unsafe {
            instance.get(CDOTAMatchPrivateMetadata_Team_Player::new)
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

    // optional uint32 player_slot = 2;

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

    // optional bytes position_stream = 3;

    pub fn clear_position_stream(&mut self) {
        self.position_stream.clear();
    }

    pub fn has_position_stream(&self) -> bool {
        self.position_stream.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position_stream(&mut self, v: ::std::vec::Vec<u8>) {
        self.position_stream = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_position_stream(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.position_stream.is_none() {
            self.position_stream.set_default();
        };
        self.position_stream.as_mut().unwrap()
    }

    // Take field
    pub fn take_position_stream(&mut self) -> ::std::vec::Vec<u8> {
        self.position_stream.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_position_stream(&self) -> &[u8] {
        match self.position_stream.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_position_stream_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.position_stream
    }

    fn mut_position_stream_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.position_stream
    }
}

impl ::protobuf::Message for CDOTAMatchPrivateMetadata_Team_Player {
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
                    self.player_slot = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.position_stream)?;
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
        if let Some(v) = self.player_slot {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.position_stream.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_uint32(1, v)?;
        };
        if let Some(v) = self.player_slot {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.position_stream.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for CDOTAMatchPrivateMetadata_Team_Player {
    fn new() -> CDOTAMatchPrivateMetadata_Team_Player {
        CDOTAMatchPrivateMetadata_Team_Player::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchPrivateMetadata_Team_Player>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CDOTAMatchPrivateMetadata_Team_Player::get_account_id_for_reflect,
                    CDOTAMatchPrivateMetadata_Team_Player::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_slot",
                    CDOTAMatchPrivateMetadata_Team_Player::get_player_slot_for_reflect,
                    CDOTAMatchPrivateMetadata_Team_Player::mut_player_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "position_stream",
                    CDOTAMatchPrivateMetadata_Team_Player::get_position_stream_for_reflect,
                    CDOTAMatchPrivateMetadata_Team_Player::mut_position_stream_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchPrivateMetadata_Team_Player>(
                    "CDOTAMatchPrivateMetadata_Team_Player",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchPrivateMetadata_Team_Player {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_player_slot();
        self.clear_position_stream();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchPrivateMetadata_Team_Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchPrivateMetadata_Team_Player {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAMatchPrivateMetadata_Team_Building {
    // message fields
    unit_name: ::protobuf::SingularField<::std::string::String>,
    position_quant_x: ::std::option::Option<u32>,
    position_quant_y: ::std::option::Option<u32>,
    death_time: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAMatchPrivateMetadata_Team_Building {}

impl CDOTAMatchPrivateMetadata_Team_Building {
    pub fn new() -> CDOTAMatchPrivateMetadata_Team_Building {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAMatchPrivateMetadata_Team_Building {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAMatchPrivateMetadata_Team_Building> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAMatchPrivateMetadata_Team_Building,
        };
        unsafe {
            instance.get(CDOTAMatchPrivateMetadata_Team_Building::new)
        }
    }

    // optional string unit_name = 1;

    pub fn clear_unit_name(&mut self) {
        self.unit_name.clear();
    }

    pub fn has_unit_name(&self) -> bool {
        self.unit_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_name(&mut self, v: ::std::string::String) {
        self.unit_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_name(&mut self) -> &mut ::std::string::String {
        if self.unit_name.is_none() {
            self.unit_name.set_default();
        };
        self.unit_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_name(&mut self) -> ::std::string::String {
        self.unit_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_unit_name(&self) -> &str {
        match self.unit_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_unit_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.unit_name
    }

    fn mut_unit_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.unit_name
    }

    // optional uint32 position_quant_x = 2;

    pub fn clear_position_quant_x(&mut self) {
        self.position_quant_x = ::std::option::Option::None;
    }

    pub fn has_position_quant_x(&self) -> bool {
        self.position_quant_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position_quant_x(&mut self, v: u32) {
        self.position_quant_x = ::std::option::Option::Some(v);
    }

    pub fn get_position_quant_x(&self) -> u32 {
        self.position_quant_x.unwrap_or(0)
    }

    fn get_position_quant_x_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.position_quant_x
    }

    fn mut_position_quant_x_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.position_quant_x
    }

    // optional uint32 position_quant_y = 3;

    pub fn clear_position_quant_y(&mut self) {
        self.position_quant_y = ::std::option::Option::None;
    }

    pub fn has_position_quant_y(&self) -> bool {
        self.position_quant_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_position_quant_y(&mut self, v: u32) {
        self.position_quant_y = ::std::option::Option::Some(v);
    }

    pub fn get_position_quant_y(&self) -> u32 {
        self.position_quant_y.unwrap_or(0)
    }

    fn get_position_quant_y_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.position_quant_y
    }

    fn mut_position_quant_y_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.position_quant_y
    }

    // optional float death_time = 4;

    pub fn clear_death_time(&mut self) {
        self.death_time = ::std::option::Option::None;
    }

    pub fn has_death_time(&self) -> bool {
        self.death_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_death_time(&mut self, v: f32) {
        self.death_time = ::std::option::Option::Some(v);
    }

    pub fn get_death_time(&self) -> f32 {
        self.death_time.unwrap_or(0.)
    }

    fn get_death_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.death_time
    }

    fn mut_death_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.death_time
    }
}

impl ::protobuf::Message for CDOTAMatchPrivateMetadata_Team_Building {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.unit_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.position_quant_x = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.position_quant_y = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_float()?;
                    self.death_time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.unit_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.position_quant_x {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.position_quant_y {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.death_time {
            my_size += 5;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_name.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.position_quant_x {
            os.write_uint32(2, v)?;
        };
        if let Some(v) = self.position_quant_y {
            os.write_uint32(3, v)?;
        };
        if let Some(v) = self.death_time {
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

impl ::protobuf::MessageStatic for CDOTAMatchPrivateMetadata_Team_Building {
    fn new() -> CDOTAMatchPrivateMetadata_Team_Building {
        CDOTAMatchPrivateMetadata_Team_Building::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAMatchPrivateMetadata_Team_Building>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "unit_name",
                    CDOTAMatchPrivateMetadata_Team_Building::get_unit_name_for_reflect,
                    CDOTAMatchPrivateMetadata_Team_Building::mut_unit_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position_quant_x",
                    CDOTAMatchPrivateMetadata_Team_Building::get_position_quant_x_for_reflect,
                    CDOTAMatchPrivateMetadata_Team_Building::mut_position_quant_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "position_quant_y",
                    CDOTAMatchPrivateMetadata_Team_Building::get_position_quant_y_for_reflect,
                    CDOTAMatchPrivateMetadata_Team_Building::mut_position_quant_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "death_time",
                    CDOTAMatchPrivateMetadata_Team_Building::get_death_time_for_reflect,
                    CDOTAMatchPrivateMetadata_Team_Building::mut_death_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAMatchPrivateMetadata_Team_Building>(
                    "CDOTAMatchPrivateMetadata_Team_Building",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAMatchPrivateMetadata_Team_Building {
    fn clear(&mut self) {
        self.clear_unit_name();
        self.clear_position_quant_x();
        self.clear_position_quant_y();
        self.clear_death_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAMatchPrivateMetadata_Team_Building {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAMatchPrivateMetadata_Team_Building {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x6d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74,
    0x61, 0x1a, 0x15, 0x62, 0x61, 0x73, 0x65, 0x5f, 0x67, 0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2d, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x67,
    0x63, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
    0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xae, 0x01, 0x0a, 0x16, 0x43, 0x44, 0x4f, 0x54,
    0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x46, 0x69,
    0x6c, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x05, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x19, 0x0a, 0x08,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x52, 0x07,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x49, 0x64, 0x12, 0x34, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x64, 0x6f, 0x74, 0x61,
    0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x29, 0x0a,
    0x10, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x5f, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0f, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65,
    0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0xab, 0x11, 0x0a, 0x12, 0x43, 0x44, 0x4f,
    0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12,
    0x33, 0x0a, 0x05, 0x74, 0x65, 0x61, 0x6d, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1d,
    0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68,
    0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x52, 0x05, 0x74,
    0x65, 0x61, 0x6d, 0x73, 0x12, 0x41, 0x0a, 0x0c, 0x69, 0x74, 0x65, 0x6d, 0x5f, 0x72, 0x65, 0x77,
    0x61, 0x72, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x64, 0x6f, 0x74,
    0x61, 0x2e, 0x43, 0x4c, 0x6f, 0x62, 0x62, 0x79, 0x54, 0x69, 0x6d, 0x65, 0x64, 0x52, 0x65, 0x77,
    0x61, 0x72, 0x64, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x52, 0x0b, 0x69, 0x74, 0x65, 0x6d,
    0x52, 0x65, 0x77, 0x61, 0x72, 0x64, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x6c, 0x6f, 0x62, 0x62, 0x79,
    0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x06, 0x52, 0x07, 0x6c, 0x6f, 0x62, 0x62, 0x79,
    0x49, 0x64, 0x12, 0x2a, 0x0a, 0x11, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x75, 0x6e, 0x74,
    0x69, 0x6c, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x06, 0x52, 0x0f, 0x72,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x55, 0x6e, 0x74, 0x69, 0x6c, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x35,
    0x0a, 0x17, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x63, 0x75, 0x73,
    0x74, 0x6f, 0x6d, 0x5f, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x14, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x47, 0x61, 0x6d, 0x65, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d,
    0x54, 0x61, 0x62, 0x6c, 0x65, 0x1a, 0x9e, 0x0f, 0x0a, 0x04, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x1b,
    0x0a, 0x09, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x08, 0x64, 0x6f, 0x74, 0x61, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x3e, 0x0a, 0x07, 0x70,
    0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x4d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x2e, 0x50, 0x6c, 0x61, 0x79,
    0x65, 0x72, 0x52, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x67,
    0x72, 0x61, 0x70, 0x68, 0x5f, 0x65, 0x78, 0x70, 0x65, 0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x02, 0x52, 0x0f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x45, 0x78, 0x70, 0x65,
    0x72, 0x69, 0x65, 0x6e, 0x63, 0x65, 0x12, 0x2a, 0x0a, 0x11, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f,
    0x67, 0x6f, 0x6c, 0x64, 0x5f, 0x65, 0x61, 0x72, 0x6e, 0x65, 0x64, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x02, 0x52, 0x0f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x47, 0x6f, 0x6c, 0x64, 0x45, 0x61, 0x72, 0x6e,
    0x65, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x67, 0x72, 0x61, 0x70, 0x68, 0x5f, 0x6e, 0x65, 0x74, 0x5f,
    0x77, 0x6f, 0x72, 0x74, 0x68, 0x18, 0x05, 0x20, 0x03, 0x28, 0x02, 0x52, 0x0d, 0x67, 0x72, 0x61,
    0x70, 0x68, 0x4e, 0x65, 0x74, 0x57, 0x6f, 0x72, 0x74, 0x68, 0x12, 0x22, 0x0a, 0x0d, 0x63, 0x6d,
    0x5f, 0x66, 0x69, 0x72, 0x73, 0x74, 0x5f, 0x70, 0x69, 0x63, 0x6b, 0x18, 0x06, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0b, 0x63, 0x6d, 0x46, 0x69, 0x72, 0x73, 0x74, 0x50, 0x69, 0x63, 0x6b, 0x12, 0x2f,
    0x0a, 0x14, 0x63, 0x6d, 0x5f, 0x63, 0x61, 0x70, 0x74, 0x61, 0x69, 0x6e, 0x5f, 0x70, 0x6c, 0x61,
    0x79, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x11, 0x63, 0x6d,
    0x43, 0x61, 0x70, 0x74, 0x61, 0x69, 0x6e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x49, 0x64, 0x12,
    0x17, 0x0a, 0x07, 0x63, 0x6d, 0x5f, 0x62, 0x61, 0x6e, 0x73, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0d,
    0x52, 0x06, 0x63, 0x6d, 0x42, 0x61, 0x6e, 0x73, 0x12, 0x19, 0x0a, 0x08, 0x63, 0x6d, 0x5f, 0x70,
    0x69, 0x63, 0x6b, 0x73, 0x18, 0x09, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x07, 0x63, 0x6d, 0x50, 0x69,
    0x63, 0x6b, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6d, 0x5f, 0x70, 0x65, 0x6e, 0x61, 0x6c, 0x74,
    0x79, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x63, 0x6d, 0x50, 0x65, 0x6e, 0x61, 0x6c,
    0x74, 0x79, 0x1a, 0x43, 0x0a, 0x0a, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4b, 0x69, 0x6c, 0x6c,
    0x12, 0x1f, 0x0a, 0x0b, 0x76, 0x69, 0x63, 0x74, 0x69, 0x6d, 0x5f, 0x73, 0x6c, 0x6f, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x76, 0x69, 0x63, 0x74, 0x69, 0x6d, 0x53, 0x6c, 0x6f,
    0x74, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x1a, 0x4c, 0x0a, 0x0c, 0x49, 0x74, 0x65, 0x6d, 0x50,
    0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x69, 0x74, 0x65, 0x6d, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x69, 0x74, 0x65, 0x6d, 0x49, 0x64,
    0x12, 0x23, 0x0a, 0x0d, 0x70, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x65, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x0c, 0x70, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73,
    0x65, 0x54, 0x69, 0x6d, 0x65, 0x1a, 0xa7, 0x01, 0x0a, 0x11, 0x49, 0x6e, 0x76, 0x65, 0x6e, 0x74,
    0x6f, 0x72, 0x79, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x12, 0x17, 0x0a, 0x07, 0x69,
    0x74, 0x65, 0x6d, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x06, 0x69, 0x74,
    0x65, 0x6d, 0x49, 0x64, 0x12, 0x1b, 0x0a, 0x09, 0x67, 0x61, 0x6d, 0x65, 0x5f, 0x74, 0x69, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05, 0x52, 0x08, 0x67, 0x61, 0x6d, 0x65, 0x54, 0x69, 0x6d,
    0x65, 0x12, 0x14, 0x0a, 0x05, 0x6b, 0x69, 0x6c, 0x6c, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x05, 0x6b, 0x69, 0x6c, 0x6c, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x64, 0x65, 0x61, 0x74, 0x68,
    0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x64, 0x65, 0x61, 0x74, 0x68, 0x73, 0x12,
    0x18, 0x0a, 0x07, 0x61, 0x73, 0x73, 0x69, 0x73, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x07, 0x61, 0x73, 0x73, 0x69, 0x73, 0x74, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x6c, 0x65, 0x76,
    0x65, 0x6c, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x1a,
    0x48, 0x0a, 0x11, 0x41, 0x75, 0x74, 0x6f, 0x53, 0x74, 0x79, 0x6c, 0x65, 0x43, 0x72, 0x69, 0x74,
    0x65, 0x72, 0x69, 0x61, 0x12, 0x1d, 0x0a, 0x0a, 0x6e, 0x61, 0x6d, 0x65, 0x5f, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x6e, 0x61, 0x6d, 0x65, 0x54, 0x6f,
    0x6b, 0x65, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x02, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x1a, 0x8a, 0x09, 0x0a, 0x06, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x49, 0x64, 0x12, 0x29, 0x0a, 0x10, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x5f, 0x75,
    0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0f, 0x61,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x73, 0x12, 0x1f,
    0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x73, 0x6c, 0x6f, 0x74, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0d, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x53, 0x6c, 0x6f, 0x74, 0x12,
    0x41, 0x0a, 0x13, 0x65, 0x71, 0x75, 0x69, 0x70, 0x70, 0x65, 0x64, 0x5f, 0x65, 0x63, 0x6f, 0x6e,
    0x5f, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x64,
    0x6f, 0x74, 0x61, 0x2e, 0x43, 0x53, 0x4f, 0x45, 0x63, 0x6f, 0x6e, 0x49, 0x74, 0x65, 0x6d, 0x52,
    0x11, 0x65, 0x71, 0x75, 0x69, 0x70, 0x70, 0x65, 0x64, 0x45, 0x63, 0x6f, 0x6e, 0x49, 0x74, 0x65,
    0x6d, 0x73, 0x12, 0x3e, 0x0a, 0x05, 0x6b, 0x69, 0x6c, 0x6c, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x28, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x54, 0x65, 0x61, 0x6d,
    0x2e, 0x50, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x4b, 0x69, 0x6c, 0x6c, 0x52, 0x05, 0x6b, 0x69, 0x6c,
    0x6c, 0x73, 0x12, 0x40, 0x0a, 0x05, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x2a, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61,
    0x74, 0x63, 0x68, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x54, 0x65, 0x61, 0x6d,
    0x2e, 0x49, 0x74, 0x65, 0x6d, 0x50, 0x75, 0x72, 0x63, 0x68, 0x61, 0x73, 0x65, 0x52, 0x05, 0x69,
    0x74, 0x65, 0x6d, 0x73, 0x12, 0x22, 0x0a, 0x0d, 0x61, 0x76, 0x67, 0x5f, 0x6b, 0x69, 0x6c, 0x6c,
    0x73, 0x5f, 0x78, 0x31, 0x36, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x61, 0x76, 0x67,
    0x4b, 0x69, 0x6c, 0x6c, 0x73, 0x58, 0x31, 0x36, 0x12, 0x24, 0x0a, 0x0e, 0x61, 0x76, 0x67, 0x5f,
    0x64, 0x65, 0x61, 0x74, 0x68, 0x73, 0x5f, 0x78, 0x31, 0x36, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x0c, 0x61, 0x76, 0x67, 0x44, 0x65, 0x61, 0x74, 0x68, 0x73, 0x58, 0x31, 0x36, 0x12, 0x26,
    0x0a, 0x0f, 0x61, 0x76, 0x67, 0x5f, 0x61, 0x73, 0x73, 0x69, 0x73, 0x74, 0x73, 0x5f, 0x78, 0x31,
    0x36, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d, 0x61, 0x76, 0x67, 0x41, 0x73, 0x73, 0x69,
    0x73, 0x74, 0x73, 0x58, 0x31, 0x36, 0x12, 0x1e, 0x0a, 0x0b, 0x61, 0x76, 0x67, 0x5f, 0x67, 0x70,
    0x6d, 0x5f, 0x78, 0x31, 0x36, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x76, 0x67,
    0x47, 0x70, 0x6d, 0x58, 0x31, 0x36, 0x12, 0x1e, 0x0a, 0x0b, 0x61, 0x76, 0x67, 0x5f, 0x78, 0x70,
    0x6d, 0x5f, 0x78, 0x31, 0x36, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x76, 0x67,
    0x58, 0x70, 0x6d, 0x58, 0x31, 0x36, 0x12, 0x24, 0x0a, 0x0e, 0x62, 0x65, 0x73, 0x74, 0x5f, 0x6b,
    0x69, 0x6c, 0x6c, 0x73, 0x5f, 0x78, 0x31, 0x36, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0c,
    0x62, 0x65, 0x73, 0x74, 0x4b, 0x69, 0x6c, 0x6c, 0x73, 0x58, 0x31, 0x36, 0x12, 0x28, 0x0a, 0x10,
    0x62, 0x65, 0x73, 0x74, 0x5f, 0x61, 0x73, 0x73, 0x69, 0x73, 0x74, 0x73, 0x5f, 0x78, 0x31, 0x36,
    0x18, 0x0d, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x62, 0x65, 0x73, 0x74, 0x41, 0x73, 0x73, 0x69,
    0x73, 0x74, 0x73, 0x58, 0x31, 0x36, 0x12, 0x20, 0x0a, 0x0c, 0x62, 0x65, 0x73, 0x74, 0x5f, 0x67,
    0x70, 0x6d, 0x5f, 0x78, 0x31, 0x36, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x62, 0x65,
    0x73, 0x74, 0x47, 0x70, 0x6d, 0x58, 0x31, 0x36, 0x12, 0x20, 0x0a, 0x0c, 0x62, 0x65, 0x73, 0x74,
    0x5f, 0x78, 0x70, 0x6d, 0x5f, 0x78, 0x31, 0x36, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a,
    0x62, 0x65, 0x73, 0x74, 0x58, 0x70, 0x6d, 0x58, 0x31, 0x36, 0x12, 0x1d, 0x0a, 0x0a, 0x77, 0x69,
    0x6e, 0x5f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6b, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09,
    0x77, 0x69, 0x6e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6b, 0x12, 0x26, 0x0a, 0x0f, 0x62, 0x65, 0x73,
    0x74, 0x5f, 0x77, 0x69, 0x6e, 0x5f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6b, 0x18, 0x11, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x0d, 0x62, 0x65, 0x73, 0x74, 0x57, 0x69, 0x6e, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6b, 0x12, 0x1f, 0x0a, 0x0b, 0x66, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x73, 0x63, 0x6f, 0x72, 0x65,
    0x18, 0x12, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0a, 0x66, 0x69, 0x67, 0x68, 0x74, 0x53, 0x63, 0x6f,
    0x72, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x66, 0x61, 0x72, 0x6d, 0x5f, 0x73, 0x63, 0x6f, 0x72, 0x65,
    0x18, 0x13, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x66, 0x61, 0x72, 0x6d, 0x53, 0x63, 0x6f, 0x72,
    0x65, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x63, 0x6f,
    0x72, 0x65, 0x18, 0x14, 0x20, 0x01, 0x28, 0x02, 0x52, 0x0c, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x53, 0x63, 0x6f, 0x72, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x75, 0x73, 0x68, 0x5f, 0x73,
    0x63, 0x6f, 0x72, 0x65, 0x18, 0x15, 0x20, 0x01, 0x28, 0x02, 0x52, 0x09, 0x70, 0x75, 0x73, 0x68,
    0x53, 0x63, 0x6f, 0x72, 0x65, 0x12, 0x24, 0x0a, 0x0e, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x5f, 0x75,
    0x70, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x18, 0x16, 0x20, 0x03, 0x28, 0x0d, 0x52, 0x0c, 0x6c,
    0x65, 0x76, 0x65, 0x6c, 0x55, 0x70, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x12, 0x26, 0x0a, 0x0f, 0x67,
    0x72, 0x61, 0x70, 0x68, 0x5f, 0x6e, 0x65, 0x74, 0x5f, 0x77, 0x6f, 0x72, 0x74, 0x68, 0x18, 0x17,
    0x20, 0x03, 0x28, 0x02, 0x52, 0x0d, 0x67, 0x72, 0x61, 0x70, 0x68, 0x4e, 0x65, 0x74, 0x57, 0x6f,
    0x72, 0x74, 0x68, 0x12, 0x5e, 0x0a, 0x12, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79,
    0x5f, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x18, 0x18, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x2f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63,
    0x68, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x2e, 0x49,
    0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74,
    0x52, 0x11, 0x69, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x6f, 0x72, 0x79, 0x53, 0x6e, 0x61, 0x70, 0x73,
    0x68, 0x6f, 0x74, 0x12, 0x30, 0x0a, 0x14, 0x61, 0x76, 0x67, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x73,
    0x5f, 0x63, 0x61, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x74, 0x65, 0x64, 0x18, 0x19, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x12, 0x61, 0x76, 0x67, 0x53, 0x74, 0x61, 0x74, 0x73, 0x43, 0x61, 0x6c, 0x69, 0x62,
    0x72, 0x61, 0x74, 0x65, 0x64, 0x12, 0x5f, 0x0a, 0x13, 0x61, 0x75, 0x74, 0x6f, 0x5f, 0x73, 0x74,
    0x79, 0x6c, 0x65, 0x5f, 0x63, 0x72, 0x69, 0x74, 0x65, 0x72, 0x69, 0x61, 0x18, 0x1a, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d,
    0x61, 0x74, 0x63, 0x68, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x54, 0x65, 0x61,
    0x6d, 0x2e, 0x41, 0x75, 0x74, 0x6f, 0x53, 0x74, 0x79, 0x6c, 0x65, 0x43, 0x72, 0x69, 0x74, 0x65,
    0x72, 0x69, 0x61, 0x52, 0x11, 0x61, 0x75, 0x74, 0x6f, 0x53, 0x74, 0x79, 0x6c, 0x65, 0x43, 0x72,
    0x69, 0x74, 0x65, 0x72, 0x69, 0x61, 0x22, 0xa1, 0x04, 0x0a, 0x19, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x12, 0x3a, 0x0a, 0x05, 0x74, 0x65, 0x61, 0x6d, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41,
    0x4d, 0x61, 0x74, 0x63, 0x68, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x52, 0x05, 0x74, 0x65, 0x61, 0x6d, 0x73,
    0x1a, 0xc7, 0x03, 0x0a, 0x04, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x1b, 0x0a, 0x09, 0x64, 0x6f, 0x74,
    0x61, 0x5f, 0x74, 0x65, 0x61, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x64, 0x6f,
    0x74, 0x61, 0x54, 0x65, 0x61, 0x6d, 0x12, 0x45, 0x0a, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43,
    0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65,
    0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x2e, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x52, 0x07, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x73, 0x12, 0x4b, 0x0a,
    0x09, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x2d, 0x2e, 0x64, 0x6f, 0x74, 0x61, 0x2e, 0x43, 0x44, 0x4f, 0x54, 0x41, 0x4d, 0x61, 0x74,
    0x63, 0x68, 0x50, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0x2e, 0x54, 0x65, 0x61, 0x6d, 0x2e, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x52,
    0x09, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x73, 0x1a, 0x71, 0x0a, 0x06, 0x50, 0x6c,
    0x61, 0x79, 0x65, 0x72, 0x12, 0x1d, 0x0a, 0x0a, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x73, 0x6c,
    0x6f, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0a, 0x70, 0x6c, 0x61, 0x79, 0x65, 0x72,
    0x53, 0x6c, 0x6f, 0x74, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x70,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x1a, 0x9a, 0x01,
    0x0a, 0x08, 0x42, 0x75, 0x69, 0x6c, 0x64, 0x69, 0x6e, 0x67, 0x12, 0x1b, 0x0a, 0x09, 0x75, 0x6e,
    0x69, 0x74, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x75,
    0x6e, 0x69, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x70, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x71, 0x75, 0x61, 0x6e, 0x74, 0x5f, 0x78, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0d, 0x52, 0x0e, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75, 0x61, 0x6e, 0x74,
    0x58, 0x12, 0x28, 0x0a, 0x10, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x71, 0x75,
    0x61, 0x6e, 0x74, 0x5f, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x70, 0x6f, 0x73,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x51, 0x75, 0x61, 0x6e, 0x74, 0x59, 0x12, 0x1d, 0x0a, 0x0a, 0x64,
    0x65, 0x61, 0x74, 0x68, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x02, 0x52,
    0x09, 0x64, 0x65, 0x61, 0x74, 0x68, 0x54, 0x69, 0x6d, 0x65, 0x42, 0x03, 0x80, 0x01, 0x00, 0x4a,
    0xbe, 0x30, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x6f, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x07, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x05, 0x07, 0x36, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x07, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x07, 0x1d, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00,
    0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0a, 0x17, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x18, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x0c, 0x08, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0c, 0x11,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x24, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x2f, 0x30, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x0d, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x0d, 0x17, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0d,
    0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10, 0x00, 0x58, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01,
    0x03, 0x00, 0x12, 0x04, 0x11, 0x08, 0x51, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x10, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00,
    0x12, 0x04, 0x12, 0x10, 0x15, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x12, 0x18, 0x22, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x13, 0x18, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x28, 0x33, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x36, 0x37, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x14, 0x18, 0x32,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x14,
    0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x14, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x14, 0x28, 0x2d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x30, 0x31, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x01, 0x12, 0x04, 0x17, 0x10, 0x1a, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x01, 0x01, 0x12, 0x03, 0x17, 0x18, 0x24, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x18, 0x18, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x18, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x21, 0x27, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x28, 0x2f, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x32,
    0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x19,
    0x18, 0x39, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x19, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x19, 0x21, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x27, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x37, 0x38, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x02, 0x12, 0x04, 0x1c, 0x10, 0x23, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x18, 0x29, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x18, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1d, 0x18, 0x20, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x21, 0x27, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x28,
    0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1d, 0x32, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1e, 0x18, 0x35, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x1e, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x1e, 0x21, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x27, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x33, 0x34, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x18, 0x32, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x18, 0x20, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1f, 0x21,
    0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x1f, 0x28, 0x2d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x1f, 0x30, 0x31, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x20, 0x18, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x20, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x20, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x20, 0x28, 0x2e, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x20, 0x31, 0x32, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x21, 0x18, 0x34, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x04, 0x04, 0x12, 0x03, 0x21, 0x18,
    0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x21, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x21, 0x28, 0x2f, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x21, 0x32, 0x33, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x22, 0x18, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x22, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x02, 0x02, 0x05, 0x05, 0x12, 0x03, 0x22, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x22, 0x28, 0x2d, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x22, 0x30, 0x31,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x12, 0x04, 0x25, 0x10, 0x28, 0x11,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x01, 0x12, 0x03, 0x25, 0x18, 0x29,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x02, 0x00, 0x12, 0x03, 0x26, 0x18,
    0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x26, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x26, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x26, 0x28, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x35, 0x36, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x03, 0x02, 0x01, 0x12, 0x03, 0x27, 0x18, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x21, 0x26, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x27, 0x2c,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x27,
    0x2f, 0x30, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x12, 0x04, 0x2a, 0x10,
    0x45, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x01, 0x12, 0x03, 0x2a,
    0x18, 0x1e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x2b, 0x18, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x2b, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x2b, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x28, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x35, 0x36, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x18, 0x3d, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x18, 0x20, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c, 0x21, 0x27,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c,
    0x28, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x2c, 0x3b, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x02,
    0x12, 0x03, 0x2d, 0x18, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x2d, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x28, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x36, 0x37, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x18, 0x45, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2e, 0x18, 0x20,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2e,
    0x21, 0x2c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x2e, 0x2d, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x2e, 0x43, 0x44, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x04, 0x12, 0x03, 0x2f, 0x18, 0x4e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2f, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x04, 0x06, 0x12, 0x03, 0x2f, 0x21, 0x43, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2f, 0x44, 0x49, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2f, 0x4c, 0x4d, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x05, 0x12, 0x03, 0x30, 0x18, 0x50,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x30,
    0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x30, 0x21, 0x45, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x30, 0x46, 0x4b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x30, 0x4e, 0x4f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x04, 0x02, 0x06, 0x12, 0x03, 0x31, 0x18, 0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x06, 0x04, 0x12, 0x03, 0x31, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x31, 0x21, 0x27, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x31, 0x28, 0x35, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x06, 0x03, 0x12, 0x03, 0x31, 0x38,
    0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x07, 0x12, 0x03, 0x32,
    0x18, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x32, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x07,
    0x05, 0x12, 0x03, 0x32, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x32, 0x28, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x32, 0x39, 0x3a, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x04, 0x02, 0x08, 0x12, 0x03, 0x33, 0x18, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x08, 0x04, 0x12, 0x03, 0x33, 0x18, 0x20, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x08, 0x05, 0x12, 0x03, 0x33, 0x21, 0x27, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x33, 0x28,
    0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x33, 0x3a, 0x3b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x09, 0x12,
    0x03, 0x34, 0x18, 0x39, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x09,
    0x04, 0x12, 0x03, 0x34, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x09, 0x05, 0x12, 0x03, 0x34, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x34, 0x28, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x04, 0x02, 0x09, 0x03, 0x12, 0x03, 0x34, 0x36, 0x38, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0a, 0x12, 0x03, 0x35, 0x18, 0x39, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x35, 0x18, 0x20, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x35, 0x21,
    0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0a, 0x01, 0x12, 0x03,
    0x35, 0x28, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0a, 0x03,
    0x12, 0x03, 0x35, 0x36, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02,
    0x0b, 0x12, 0x03, 0x36, 0x18, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x0b, 0x04, 0x12, 0x03, 0x36, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x04, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x36, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x04, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x36, 0x28, 0x36, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x36, 0x39, 0x3b, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0c, 0x12, 0x03, 0x37, 0x18, 0x3e, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x37, 0x18,
    0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0c, 0x05, 0x12, 0x03,
    0x37, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0c, 0x01,
    0x12, 0x03, 0x37, 0x28, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02,
    0x0c, 0x03, 0x12, 0x03, 0x37, 0x3b, 0x3d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x04, 0x02, 0x0d, 0x12, 0x03, 0x38, 0x18, 0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x04, 0x02, 0x0d, 0x04, 0x12, 0x03, 0x38, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x04, 0x02, 0x0d, 0x05, 0x12, 0x03, 0x38, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x38, 0x28, 0x34, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x38, 0x37, 0x39,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0e, 0x12, 0x03, 0x39, 0x18,
    0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0e, 0x04, 0x12, 0x03,
    0x39, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0e, 0x05,
    0x12, 0x03, 0x39, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02,
    0x0e, 0x01, 0x12, 0x03, 0x39, 0x28, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x04, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x39, 0x37, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x0f, 0x12, 0x03, 0x3a, 0x18, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x04, 0x02, 0x0f, 0x04, 0x12, 0x03, 0x3a, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0f, 0x05, 0x12, 0x03, 0x3a, 0x21, 0x27, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x3a, 0x28, 0x32,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x0f, 0x03, 0x12, 0x03, 0x3a,
    0x35, 0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x10, 0x12, 0x03,
    0x3b, 0x18, 0x3d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x10, 0x04,
    0x12, 0x03, 0x3b, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02,
    0x10, 0x05, 0x12, 0x03, 0x3b, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x04, 0x02, 0x10, 0x01, 0x12, 0x03, 0x3b, 0x28, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x10, 0x03, 0x12, 0x03, 0x3b, 0x3a, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x11, 0x12, 0x03, 0x3c, 0x18, 0x38, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x11, 0x04, 0x12, 0x03, 0x3c, 0x18, 0x20, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x11, 0x05, 0x12, 0x03, 0x3c, 0x21, 0x26,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x11, 0x01, 0x12, 0x03, 0x3c,
    0x27, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x11, 0x03, 0x12,
    0x03, 0x3c, 0x35, 0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x12,
    0x12, 0x03, 0x3d, 0x18, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02,
    0x12, 0x04, 0x12, 0x03, 0x3d, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x04, 0x02, 0x12, 0x05, 0x12, 0x03, 0x3d, 0x21, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x12, 0x01, 0x12, 0x03, 0x3d, 0x27, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x12, 0x03, 0x12, 0x03, 0x3d, 0x34, 0x36, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x13, 0x12, 0x03, 0x3e, 0x18, 0x3a, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x13, 0x04, 0x12, 0x03, 0x3e, 0x18, 0x20,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x13, 0x05, 0x12, 0x03, 0x3e,
    0x21, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x13, 0x01, 0x12,
    0x03, 0x3e, 0x27, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x13,
    0x03, 0x12, 0x03, 0x3e, 0x37, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x14, 0x12, 0x03, 0x3f, 0x18, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03,
    0x04, 0x02, 0x14, 0x04, 0x12, 0x03, 0x3f, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x14, 0x05, 0x12, 0x03, 0x3f, 0x21, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x14, 0x01, 0x12, 0x03, 0x3f, 0x27, 0x31, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x14, 0x03, 0x12, 0x03, 0x3f, 0x34, 0x36, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x15, 0x12, 0x03, 0x40, 0x18, 0x3c,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x15, 0x04, 0x12, 0x03, 0x40,
    0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x15, 0x05, 0x12,
    0x03, 0x40, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x15,
    0x01, 0x12, 0x03, 0x40, 0x28, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x15, 0x03, 0x12, 0x03, 0x40, 0x39, 0x3b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x04, 0x02, 0x16, 0x12, 0x03, 0x41, 0x18, 0x3c, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03,
    0x00, 0x03, 0x04, 0x02, 0x16, 0x04, 0x12, 0x03, 0x41, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x16, 0x05, 0x12, 0x03, 0x41, 0x21, 0x26, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x16, 0x01, 0x12, 0x03, 0x41, 0x27, 0x36, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x16, 0x03, 0x12, 0x03, 0x41, 0x39,
    0x3b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x17, 0x12, 0x03, 0x42,
    0x18, 0x63, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x17, 0x04, 0x12,
    0x03, 0x42, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x17,
    0x06, 0x12, 0x03, 0x42, 0x21, 0x4a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x17, 0x01, 0x12, 0x03, 0x42, 0x4b, 0x5d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x04, 0x02, 0x17, 0x03, 0x12, 0x03, 0x42, 0x60, 0x62, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x04, 0x02, 0x18, 0x12, 0x03, 0x43, 0x18, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x18, 0x04, 0x12, 0x03, 0x43, 0x18, 0x20, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x18, 0x05, 0x12, 0x03, 0x43, 0x21, 0x25, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x18, 0x01, 0x12, 0x03, 0x43, 0x26,
    0x3a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x18, 0x03, 0x12, 0x03,
    0x43, 0x3d, 0x3f, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x19, 0x12,
    0x03, 0x44, 0x18, 0x64, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04, 0x02, 0x19,
    0x04, 0x12, 0x03, 0x44, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00, 0x03, 0x04,
    0x02, 0x19, 0x06, 0x12, 0x03, 0x44, 0x21, 0x4a, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x03, 0x00,
    0x03, 0x04, 0x02, 0x19, 0x01, 0x12, 0x03, 0x44, 0x4b, 0x5e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01,
    0x03, 0x00, 0x03, 0x04, 0x02, 0x19, 0x03, 0x12, 0x03, 0x44, 0x61, 0x63, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x47, 0x10, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x47, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x20, 0x29, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x01, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x48, 0x10, 0x44, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x48, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x48, 0x19, 0x37, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x38, 0x3f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x42, 0x43, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01,
    0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x49, 0x10, 0x34, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x49, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x49, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x49, 0x1f, 0x2f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x49, 0x32, 0x33, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x10, 0x35, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x4a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x4a, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x1f, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x4a, 0x33, 0x34, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x4b, 0x10, 0x33, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x4b, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x4b, 0x19, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x4b, 0x1f, 0x2e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x4b, 0x31, 0x32, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x4c, 0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x4c, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x4c, 0x19, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x4c, 0x1e, 0x2b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x4c, 0x2e, 0x2f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x4d, 0x10, 0x39, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x04,
    0x12, 0x03, 0x4d, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x05,
    0x12, 0x03, 0x4d, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x01,
    0x12, 0x03, 0x4d, 0x20, 0x34, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x06, 0x03,
    0x12, 0x03, 0x4d, 0x37, 0x38, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x4e, 0x10, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x04, 0x12,
    0x03, 0x4e, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x05, 0x12,
    0x03, 0x4e, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x4e, 0x20, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x07, 0x03, 0x12,
    0x03, 0x4e, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x12, 0x03,
    0x4f, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03,
    0x4f, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03,
    0x4f, 0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x4f, 0x20, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03,
    0x4f, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x12, 0x03, 0x50,
    0x10, 0x30, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x50,
    0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x50,
    0x19, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x50,
    0x20, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x50,
    0x2d, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x53, 0x08, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x53, 0x11, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x29, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x53, 0x31, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x54, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x54,
    0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x54, 0x11, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x2a, 0x36, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x39, 0x3a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x55, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x55, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x55, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x55, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x55, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x56, 0x08, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x56, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x56, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x56, 0x19, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x56, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03,
    0x57, 0x08, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x57, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x57, 0x11, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x57, 0x17, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x57, 0x31, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x5a, 0x00, 0x6f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x5a, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x03, 0x00, 0x12, 0x04, 0x5b, 0x08, 0x6c,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x10, 0x14, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x12, 0x04, 0x5c, 0x10, 0x60, 0x11, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x18, 0x1e, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5d, 0x18, 0x37,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5d,
    0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x5d, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x5d, 0x28, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x5d, 0x35, 0x36, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x18, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5e, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5e, 0x21, 0x27, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5e, 0x28, 0x33, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5e, 0x36,
    0x37, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5f,
    0x18, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x5f, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x5f, 0x21, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x27, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x39, 0x3a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x01, 0x12, 0x04, 0x62, 0x10, 0x67, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x62, 0x18, 0x20, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x63, 0x18, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x18, 0x20, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x21, 0x27, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x28,
    0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x63, 0x34, 0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x64, 0x18, 0x3d, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x64, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x64, 0x21, 0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x64, 0x28, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x64, 0x3b, 0x3c, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x12, 0x03, 0x65, 0x18, 0x3d, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x65, 0x18, 0x20, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x65, 0x21,
    0x27, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x65, 0x28, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x65, 0x3b, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02,
    0x03, 0x12, 0x03, 0x66, 0x18, 0x36, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00, 0x03, 0x01,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x66, 0x18, 0x20, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x03, 0x00,
    0x03, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x66, 0x21, 0x26, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02,
    0x03, 0x00, 0x03, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x66, 0x27, 0x31, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x02, 0x03, 0x00, 0x03, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x66, 0x34, 0x35, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x69, 0x10, 0x2e, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x69, 0x10, 0x18, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x69, 0x19, 0x1f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x69, 0x20, 0x29, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x69, 0x2c, 0x2d, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x6a, 0x10, 0x4b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6a, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x6a, 0x19, 0x3e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6a, 0x3f, 0x46, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6a, 0x49, 0x4a, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x02, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03, 0x6b, 0x10, 0x4f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6b, 0x10, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x6b, 0x19, 0x40, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6b, 0x41, 0x4a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x02, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x6b, 0x4d, 0x4e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x6e, 0x08, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x6e, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x6e, 0x11, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6e,
    0x30, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6e, 0x38, 0x39,
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
