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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EHeroSelectionText {
    k_EHeroSelectionText_Invalid = -1,
    k_EHeroSelectionText_None = 0,
    k_EHeroSelectionText_ChooseHero = 1,
    k_EHeroSelectionText_AllDraft_Planning_YouFirst = 2,
    k_EHeroSelectionText_AllDraft_Planning_TheyFirst = 3,
    k_EHeroSelectionText_AllDraft_BanSelected_YouFirst = 4,
    k_EHeroSelectionText_AllDraft_BanSelected_TheyFirst = 5,
    k_EHeroSelectionText_AllDraft_YouPicking = 6,
    k_EHeroSelectionText_AllDraft_TheyPicking = 7,
    k_EHeroSelectionText_AllDraft_TeammateRandomed = 8,
    k_EHeroSelectionText_AllDraft_YouPicking_LosingGold = 9,
    k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold = 10,
}

impl ::protobuf::ProtobufEnum for EHeroSelectionText {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EHeroSelectionText> {
        match value {
            -1 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_Invalid),
            0 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_None),
            1 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_ChooseHero),
            2 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_Planning_YouFirst),
            3 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_Planning_TheyFirst),
            4 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_BanSelected_YouFirst),
            5 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_BanSelected_TheyFirst),
            6 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_YouPicking),
            7 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_TheyPicking),
            8 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_TeammateRandomed),
            9 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_YouPicking_LosingGold),
            10 => ::std::option::Option::Some(EHeroSelectionText::k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EHeroSelectionText] = &[
            EHeroSelectionText::k_EHeroSelectionText_Invalid,
            EHeroSelectionText::k_EHeroSelectionText_None,
            EHeroSelectionText::k_EHeroSelectionText_ChooseHero,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_Planning_YouFirst,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_Planning_TheyFirst,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_BanSelected_YouFirst,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_BanSelected_TheyFirst,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_YouPicking,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_TheyPicking,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_TeammateRandomed,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_YouPicking_LosingGold,
            EHeroSelectionText::k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EHeroSelectionText>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EHeroSelectionText", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EHeroSelectionText {
}

impl ::protobuf::reflect::ProtobufValue for EHeroSelectionText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x14, 0x64, 0x6f, 0x74, 0x61, 0x5f, 0x68, 0x75, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x64, 0x6f, 0x74, 0x61, 0x1a, 0x20, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2a, 0xf1,
    0x08, 0x0a, 0x12, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x54, 0x65, 0x78, 0x74, 0x12, 0x29, 0x0a, 0x1c, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f,
    0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x49, 0x6e,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
    0x12, 0x1d, 0x0a, 0x19, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x4e, 0x6f, 0x6e, 0x65, 0x10, 0x00, 0x12,
    0x48, 0x0a, 0x1f, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x43, 0x68, 0x6f, 0x6f, 0x73, 0x65, 0x48, 0x65,
    0x72, 0x6f, 0x10, 0x01, 0x1a, 0x23, 0xaa, 0xd4, 0x18, 0x1f, 0x23, 0x44, 0x4f, 0x54, 0x41, 0x5f,
    0x48, 0x65, 0x72, 0x6f, 0x5f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x43,
    0x68, 0x6f, 0x6f, 0x73, 0x65, 0x48, 0x65, 0x72, 0x6f, 0x12, 0x68, 0x0a, 0x2f, 0x6b, 0x5f, 0x45,
    0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x65, 0x78,
    0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x50, 0x6c, 0x61, 0x6e, 0x6e,
    0x69, 0x6e, 0x67, 0x5f, 0x59, 0x6f, 0x75, 0x46, 0x69, 0x72, 0x73, 0x74, 0x10, 0x02, 0x1a, 0x33,
    0xaa, 0xd4, 0x18, 0x2f, 0x23, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f, 0x53,
    0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66,
    0x74, 0x5f, 0x50, 0x6c, 0x61, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x5f, 0x59, 0x6f, 0x75, 0x46, 0x69,
    0x72, 0x73, 0x74, 0x12, 0x6a, 0x0a, 0x30, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44,
    0x72, 0x61, 0x66, 0x74, 0x5f, 0x50, 0x6c, 0x61, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x5f, 0x54, 0x68,
    0x65, 0x79, 0x46, 0x69, 0x72, 0x73, 0x74, 0x10, 0x03, 0x1a, 0x34, 0xaa, 0xd4, 0x18, 0x30, 0x23,
    0x44, 0x4f, 0x54, 0x41, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x50, 0x6c, 0x61,
    0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x5f, 0x54, 0x68, 0x65, 0x79, 0x46, 0x69, 0x72, 0x73, 0x74, 0x12,
    0x6e, 0x0a, 0x32, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74,
    0x5f, 0x42, 0x61, 0x6e, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x59, 0x6f, 0x75,
    0x46, 0x69, 0x72, 0x73, 0x74, 0x10, 0x04, 0x1a, 0x36, 0xaa, 0xd4, 0x18, 0x32, 0x23, 0x44, 0x4f,
    0x54, 0x41, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x42, 0x61, 0x6e, 0x53, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x59, 0x6f, 0x75, 0x46, 0x69, 0x72, 0x73, 0x74, 0x12,
    0x70, 0x0a, 0x33, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74,
    0x5f, 0x42, 0x61, 0x6e, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x54, 0x68, 0x65,
    0x79, 0x46, 0x69, 0x72, 0x73, 0x74, 0x10, 0x05, 0x1a, 0x37, 0xaa, 0xd4, 0x18, 0x33, 0x23, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x42, 0x61, 0x6e, 0x53,
    0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x54, 0x68, 0x65, 0x79, 0x46, 0x69, 0x72, 0x73,
    0x74, 0x12, 0x5a, 0x0a, 0x28, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61,
    0x66, 0x74, 0x5f, 0x59, 0x6f, 0x75, 0x50, 0x69, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x10, 0x06, 0x1a,
    0x2c, 0xaa, 0xd4, 0x18, 0x28, 0x23, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f,
    0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61,
    0x66, 0x74, 0x5f, 0x59, 0x6f, 0x75, 0x50, 0x69, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x12, 0x5c, 0x0a,
    0x29, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x54,
    0x68, 0x65, 0x79, 0x50, 0x69, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x10, 0x07, 0x1a, 0x2d, 0xaa, 0xd4,
    0x18, 0x29, 0x23, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f, 0x53, 0x65, 0x6c,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f,
    0x54, 0x68, 0x65, 0x79, 0x50, 0x69, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x12, 0x6f, 0x0a, 0x2e, 0x6b,
    0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54,
    0x65, 0x78, 0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x54, 0x65, 0x61,
    0x6d, 0x6d, 0x61, 0x74, 0x65, 0x52, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x65, 0x64, 0x10, 0x08, 0x1a,
    0x3b, 0xaa, 0xd4, 0x18, 0x37, 0x23, 0x44, 0x4f, 0x54, 0x41, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f,
    0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61,
    0x66, 0x74, 0x5f, 0x54, 0x65, 0x61, 0x6d, 0x6d, 0x61, 0x74, 0x65, 0x52, 0x61, 0x6e, 0x64, 0x6f,
    0x6d, 0x65, 0x64, 0x5f, 0x50, 0x61, 0x6e, 0x6f, 0x72, 0x61, 0x6d, 0x61, 0x12, 0x70, 0x0a, 0x33,
    0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x54, 0x65, 0x78, 0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x59, 0x6f,
    0x75, 0x50, 0x69, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x5f, 0x4c, 0x6f, 0x73, 0x69, 0x6e, 0x67, 0x47,
    0x6f, 0x6c, 0x64, 0x10, 0x09, 0x1a, 0x37, 0xaa, 0xd4, 0x18, 0x33, 0x23, 0x44, 0x4f, 0x54, 0x41,
    0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f,
    0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x59, 0x6f, 0x75, 0x50, 0x69, 0x63, 0x6b,
    0x69, 0x6e, 0x67, 0x5f, 0x4c, 0x6f, 0x73, 0x69, 0x6e, 0x67, 0x47, 0x6f, 0x6c, 0x64, 0x12, 0x72,
    0x0a, 0x34, 0x6b, 0x5f, 0x45, 0x48, 0x65, 0x72, 0x6f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x54, 0x65, 0x78, 0x74, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f,
    0x54, 0x68, 0x65, 0x79, 0x50, 0x69, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x5f, 0x4c, 0x6f, 0x73, 0x69,
    0x6e, 0x67, 0x47, 0x6f, 0x6c, 0x64, 0x10, 0x0a, 0x1a, 0x38, 0xaa, 0xd4, 0x18, 0x34, 0x23, 0x44,
    0x4f, 0x54, 0x41, 0x5f, 0x48, 0x65, 0x72, 0x6f, 0x5f, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x5f, 0x41, 0x6c, 0x6c, 0x44, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x54, 0x68, 0x65, 0x79,
    0x50, 0x69, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x5f, 0x4c, 0x6f, 0x73, 0x69, 0x6e, 0x67, 0x47, 0x6f,
    0x6c, 0x64, 0x3a, 0x51, 0x0a, 0x12, 0x68, 0x75, 0x64, 0x5f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x69,
    0x7a, 0x65, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0xc5, 0x8a, 0x03, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x21, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x45, 0x6e, 0x75, 0x6d, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x4f, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x52, 0x10, 0x68, 0x75, 0x64, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x69, 0x7a, 0x65,
    0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x42, 0x03, 0x80, 0x01, 0x00, 0x4a, 0x8f, 0x0e, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x19, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x04, 0x07, 0x29, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x06, 0x00, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x06, 0x1d, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x07, 0x12, 0x04, 0x08, 0x00, 0x0a,
    0x01, 0x0a, 0x09, 0x0a, 0x02, 0x07, 0x00, 0x12, 0x03, 0x09, 0x08, 0x33, 0x0a, 0x0a, 0x0a, 0x03,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x08, 0x07, 0x27, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x04, 0x12,
    0x03, 0x09, 0x08, 0x10, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x05, 0x12, 0x03, 0x09, 0x11, 0x17,
    0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x01, 0x12, 0x03, 0x09, 0x18, 0x2a, 0x0a, 0x0a, 0x0a, 0x03,
    0x07, 0x00, 0x03, 0x12, 0x03, 0x09, 0x2d, 0x32, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04,
    0x0c, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x05, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x08, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x27, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x0e, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x0e, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0e,
    0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x08, 0x67, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0f, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x2c, 0x66, 0x0a, 0x0f, 0x0a, 0x08, 0x05, 0x00, 0x02,
    0x02, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0f, 0x2d, 0x65, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00,
    0x02, 0x02, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x2d, 0x41, 0x0a, 0x11, 0x0a, 0x0a,
    0x05, 0x00, 0x02, 0x02, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0f, 0x2d, 0x41, 0x0a,
    0x12, 0x0a, 0x0b, 0x05, 0x00, 0x02, 0x02, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0f, 0x2e, 0x40, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x02, 0x03, 0xe7, 0x07, 0x00, 0x07,
    0x12, 0x03, 0x0f, 0x44, 0x65, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x04, 0x10,
    0x08, 0x87, 0x01, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x08,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x10, 0x3a, 0x3b, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0x10, 0x3c, 0x86, 0x01, 0x0a, 0x10,
    0x0a, 0x08, 0x05, 0x00, 0x02, 0x03, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x10, 0x3d, 0x85, 0x01,
    0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x03, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x10,
    0x3d, 0x51, 0x0a, 0x11, 0x0a, 0x0a, 0x05, 0x00, 0x02, 0x03, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x10, 0x3d, 0x51, 0x0a, 0x12, 0x0a, 0x0b, 0x05, 0x00, 0x02, 0x03, 0x03, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x3e, 0x50, 0x0a, 0x11, 0x0a, 0x09, 0x05, 0x00, 0x02,
    0x03, 0x03, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x04, 0x10, 0x54, 0x85, 0x01, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x04, 0x12, 0x04, 0x11, 0x08, 0x89, 0x01, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x08, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x11, 0x3b, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x11, 0x3d, 0x88, 0x01, 0x0a, 0x10, 0x0a, 0x08, 0x05, 0x00, 0x02, 0x04, 0x03, 0xe7, 0x07,
    0x00, 0x12, 0x04, 0x11, 0x3e, 0x87, 0x01, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x04, 0x03,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x11, 0x3e, 0x52, 0x0a, 0x11, 0x0a, 0x0a, 0x05, 0x00, 0x02,
    0x04, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x11, 0x3e, 0x52, 0x0a, 0x12, 0x0a, 0x0b,
    0x05, 0x00, 0x02, 0x04, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x3f, 0x51,
    0x0a, 0x11, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x04, 0x03, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x04, 0x11,
    0x55, 0x87, 0x01, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x04, 0x12, 0x08, 0x8d,
    0x01, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x12, 0x08, 0x3a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x12, 0x3d, 0x3e, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x05, 0x03, 0x12, 0x04, 0x12, 0x3f, 0x8c, 0x01, 0x0a, 0x10, 0x0a, 0x08,
    0x05, 0x00, 0x02, 0x05, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x12, 0x40, 0x8b, 0x01, 0x0a, 0x10,
    0x0a, 0x09, 0x05, 0x00, 0x02, 0x05, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x12, 0x40, 0x54,
    0x0a, 0x11, 0x0a, 0x0a, 0x05, 0x00, 0x02, 0x05, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x12, 0x40, 0x54, 0x0a, 0x12, 0x0a, 0x0b, 0x05, 0x00, 0x02, 0x05, 0x03, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x12, 0x41, 0x53, 0x0a, 0x11, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x05, 0x03,
    0xe7, 0x07, 0x00, 0x07, 0x12, 0x04, 0x12, 0x57, 0x8b, 0x01, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x06, 0x12, 0x04, 0x13, 0x08, 0x8f, 0x01, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x13, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12,
    0x03, 0x13, 0x3e, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0x13,
    0x40, 0x8e, 0x01, 0x0a, 0x10, 0x0a, 0x08, 0x05, 0x00, 0x02, 0x06, 0x03, 0xe7, 0x07, 0x00, 0x12,
    0x04, 0x13, 0x41, 0x8d, 0x01, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x06, 0x03, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x13, 0x41, 0x55, 0x0a, 0x11, 0x0a, 0x0a, 0x05, 0x00, 0x02, 0x06, 0x03,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x13, 0x41, 0x55, 0x0a, 0x12, 0x0a, 0x0b, 0x05, 0x00,
    0x02, 0x06, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x42, 0x54, 0x0a, 0x11,
    0x0a, 0x09, 0x05, 0x00, 0x02, 0x06, 0x03, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x04, 0x13, 0x58, 0x8d,
    0x01, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x14, 0x08, 0x79, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x14, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x14, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x07, 0x03, 0x12, 0x03, 0x14, 0x35, 0x78, 0x0a, 0x0f, 0x0a, 0x08, 0x05, 0x00, 0x02, 0x07,
    0x03, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x14, 0x36, 0x77, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02,
    0x07, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x14, 0x36, 0x4a, 0x0a, 0x11, 0x0a, 0x0a, 0x05,
    0x00, 0x02, 0x07, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x36, 0x4a, 0x0a, 0x12,
    0x0a, 0x0b, 0x05, 0x00, 0x02, 0x07, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14,
    0x37, 0x49, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x07, 0x03, 0xe7, 0x07, 0x00, 0x07, 0x12,
    0x03, 0x14, 0x4d, 0x77, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x15, 0x08,
    0x7b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x15, 0x08, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x15, 0x34, 0x35, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x15, 0x36, 0x7a, 0x0a, 0x0f, 0x0a, 0x08, 0x05,
    0x00, 0x02, 0x08, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x15, 0x37, 0x79, 0x0a, 0x10, 0x0a, 0x09,
    0x05, 0x00, 0x02, 0x08, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x15, 0x37, 0x4b, 0x0a, 0x11,
    0x0a, 0x0a, 0x05, 0x00, 0x02, 0x08, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x37,
    0x4b, 0x0a, 0x12, 0x0a, 0x0b, 0x05, 0x00, 0x02, 0x08, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x15, 0x38, 0x4a, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x08, 0x03, 0xe7, 0x07,
    0x00, 0x07, 0x12, 0x03, 0x15, 0x4e, 0x79, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12,
    0x04, 0x16, 0x08, 0x8e, 0x01, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03,
    0x16, 0x08, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x16, 0x39,
    0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x03, 0x12, 0x04, 0x16, 0x3b, 0x8d, 0x01,
    0x0a, 0x10, 0x0a, 0x08, 0x05, 0x00, 0x02, 0x09, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x16, 0x3c,
    0x8c, 0x01, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x09, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x16, 0x3c, 0x50, 0x0a, 0x11, 0x0a, 0x0a, 0x05, 0x00, 0x02, 0x09, 0x03, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x16, 0x3c, 0x50, 0x0a, 0x12, 0x0a, 0x0b, 0x05, 0x00, 0x02, 0x09, 0x03,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x3d, 0x4f, 0x0a, 0x11, 0x0a, 0x09, 0x05,
    0x00, 0x02, 0x09, 0x03, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x04, 0x16, 0x53, 0x8c, 0x01, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x04, 0x17, 0x08, 0x8f, 0x01, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x17, 0x08, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0a, 0x02, 0x12, 0x03, 0x17, 0x3e, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a,
    0x03, 0x12, 0x04, 0x17, 0x40, 0x8e, 0x01, 0x0a, 0x10, 0x0a, 0x08, 0x05, 0x00, 0x02, 0x0a, 0x03,
    0xe7, 0x07, 0x00, 0x12, 0x04, 0x17, 0x41, 0x8d, 0x01, 0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02,
    0x0a, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x17, 0x41, 0x55, 0x0a, 0x11, 0x0a, 0x0a, 0x05,
    0x00, 0x02, 0x0a, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x17, 0x41, 0x55, 0x0a, 0x12,
    0x0a, 0x0b, 0x05, 0x00, 0x02, 0x0a, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17,
    0x42, 0x54, 0x0a, 0x11, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x0a, 0x03, 0xe7, 0x07, 0x00, 0x07, 0x12,
    0x04, 0x17, 0x58, 0x8d, 0x01, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x04, 0x18,
    0x08, 0x92, 0x01, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x18, 0x08,
    0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x18, 0x3f, 0x41, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x18, 0x42, 0x91, 0x01, 0x0a, 0x10,
    0x0a, 0x08, 0x05, 0x00, 0x02, 0x0b, 0x03, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x18, 0x43, 0x90, 0x01,
    0x0a, 0x10, 0x0a, 0x09, 0x05, 0x00, 0x02, 0x0b, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x18,
    0x43, 0x57, 0x0a, 0x11, 0x0a, 0x0a, 0x05, 0x00, 0x02, 0x0b, 0x03, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x18, 0x43, 0x57, 0x0a, 0x12, 0x0a, 0x0b, 0x05, 0x00, 0x02, 0x0b, 0x03, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x44, 0x56, 0x0a, 0x11, 0x0a, 0x09, 0x05, 0x00, 0x02,
    0x0b, 0x03, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x04, 0x18, 0x5a, 0x90, 0x01,
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
