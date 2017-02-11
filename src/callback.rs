use dota::demo::{CDemoFileHeader, CDemoFileInfo, CDemoPacket, CDemoFullPacket, CDemoSendTables,
                 CDemoClassInfo, CDemoStringTables, CDemoConsoleCmd, CDemoCustomData,
                 CDemoCustomDataCallbacks, CDemoUserCmd, CDemoSaveGame, CDemoSpawnGroups};
use dota::usermessages::CUserMessageSayText2;
use dota::networkbasetypes::{CNETMsg_Disconnect, CNETMsg_SplitScreenUser};

type NoArg = Option<Box<Fn()>>;

#[allow(non_snake_case)]
pub struct Callbacks {
    pub on_CDemoError: NoArg,
    pub on_CDemoStop: NoArg,
    pub on_CDemoFileHeader: Option<Box<Fn(&CDemoFileHeader)>>,
    pub on_CDemoFileInfo: Option<Box<Fn(&CDemoFileInfo)>>,
    pub on_CDemoSyncTick: NoArg,
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
    pub on_CDemoMax: NoArg,
    pub on_CDemoIsCompressed: NoArg,
    pub on_CDemoOther: NoArg,
    //////////
    pub on_CNETMsg_NOP: NoArg,
    pub on_CNETMsg_Disconnect: Option<Box<Fn(&CNETMsg_Disconnect)>>,
    pub on_CNETMsg_SplitScreenUser: Option<Box<Fn(&CNETMsg_SplitScreenUser)>>,
    //////////
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
            //////////
            on_CNETMsg_NOP: None,
            on_CNETMsg_Disconnect: None,
            on_CNETMsg_SplitScreenUser: None,
            //////////
            on_CUserMessageSayText2: None,
        }
    }
}