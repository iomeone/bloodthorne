use dota::demo::{CDemoFileHeader, CDemoFileInfo, CDemoPacket, CDemoFullPacket, CDemoSendTables,
                 CDemoClassInfo, CDemoStringTables, CDemoConsoleCmd, CDemoCustomData,
                 CDemoCustomDataCallbacks, CDemoUserCmd, CDemoSaveGame, CDemoSpawnGroups};
use dota::usermessages::CUserMessageSayText2;

#[allow(non_snake_case)]
pub struct Callbacks {
    pub on_CDemoError: Option<Box<Fn()>>,
    pub on_CDemoStop: Option<Box<Fn()>>,
    pub on_CDemoFileHeader: Option<Box<Fn(&CDemoFileHeader)>>,
    pub on_CDemoFileInfo: Option<Box<Fn(&CDemoFileInfo)>>,
    pub on_CDemoSyncTick: Option<Box<Fn()>>,
    pub on_CDemoSendTables: Option<Box<Fn(&CDemoSendTables)>>,
    pub on_CDemoClassInfo: Option<Box<Fn(&CDemoClassInfo)>>,
    pub on_CDemoStringTables: Option<Box<Fn(&CDemoStringTables)>>,
    pub on_CDemoSignonPacket: Option<Box<Fn(&CDemoPacket)>>,
    pub on_CDemoPacket: Option<Box<Fn(&CDemoPacket)>>,
    pub on_CDemoConsoleCmd: Option<Box<Fn(&CDemoConsoleCmd)>>,
    pub on_CDemoCustomData: Option<Box<Fn(&CDemoCustomData)>>,
    pub on_CDemoCustomDataCallbacks: Option<Box<Fn(&CDemoCustomDataCallbacks)>>,
    pub on_CDemoUserCmd: Option<Box<Fn(&CDemoUserCmd)>>,
    pub on_CDemoFullPacket: Option<Box<Fn(&CDemoFullPacket)>>,
    pub on_CDemoSaveGame: Option<Box<Fn(&CDemoSaveGame)>>,
    pub on_CDemoSpawnGroups: Option<Box<Fn(&CDemoSpawnGroups)>>,
    pub on_CDemoMax: Option<Box<Fn()>>,
    pub on_CDemoIsCompressed: Option<Box<Fn()>>,
    pub on_CDemoOther: Option<Box<Fn()>>,

    pub on_CUserMessageSayText2: Option<Box<Fn(&CUserMessageSayText2)>>,
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks {
            on_CDemoError: None,
            on_CDemoStop: None,
            on_CDemoFileHeader: None,
            on_CDemoFileInfo: None,
            on_CDemoSyncTick: None,
            on_CDemoSendTables: None,
            on_CDemoClassInfo: None,
            on_CDemoStringTables: None,
            on_CDemoSignonPacket: None,
            on_CDemoPacket: None,
            on_CDemoConsoleCmd: None,
            on_CDemoCustomData: None,
            on_CDemoCustomDataCallbacks: None,
            on_CDemoUserCmd: None,
            on_CDemoFullPacket: None,
            on_CDemoSaveGame: None,
            on_CDemoSpawnGroups: None,
            on_CDemoMax: None,
            on_CDemoIsCompressed: None,
            on_CDemoOther: None,
            on_CUserMessageSayText2: None,
        }
    }
}