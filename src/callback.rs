use dota::demo::{CDemoFileHeader, CDemoFileInfo, CDemoPacket, CDemoFullPacket, CDemoSendTables,
                 CDemoClassInfo, CDemoStringTables, CDemoConsoleCmd, CDemoCustomData,
                 CDemoCustomDataCallbacks, CDemoUserCmd, CDemoSaveGame, CDemoSpawnGroups};
use dota::usermessages::{CUserMessageSayText2, CUserMessageSayText, CUserMessageSayTextChannel};
use dota::networkbasetypes::{CNETMsg_Disconnect, CNETMsg_SplitScreenUser, CNETMsg_Tick,
                             CNETMsg_StringCmd, CNETMsg_SetConVar, CNETMsg_SignonState,
                             CNETMsg_SpawnGroup_Load, CNETMsg_SpawnGroup_ManifestUpdate,
                             CNETMsg_SpawnGroup_SetCreationTick, CNETMsg_SpawnGroup_Unload,
                             CNETMsg_SpawnGroup_LoadCompleted};
use dota::netmessages::{CSVCMsg_ServerInfo, CSVCMsg_CreateStringTable, CSVCMsg_UpdateStringTable};

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
    pub on_CNETMsg_Tick: Option<Box<Fn(&CNETMsg_Tick)>>,
    pub on_CNETMsg_StringCmd: Option<Box<Fn(&CNETMsg_StringCmd)>>,
    pub on_CNETMsg_SetConVar: Option<Box<Fn(&CNETMsg_SetConVar)>>,
    pub on_CNETMsg_SignonState: Option<Box<Fn(&CNETMsg_SignonState)>>,
    pub on_CNETMsg_SpawnGroup_Load: Option<Box<Fn(&CNETMsg_SpawnGroup_Load)>>,
    pub on_CNETMsg_SpawnGroup_ManifestUpdate: Option<Box<Fn(&CNETMsg_SpawnGroup_ManifestUpdate)>>,
    pub on_CNETMsg_SpawnGroup_SetCreationTick: Option<Box<Fn(&CNETMsg_SpawnGroup_SetCreationTick)>>,
    pub on_CNETMsg_SpawnGroup_Unload: Option<Box<Fn(&CNETMsg_SpawnGroup_Unload)>>,
    pub on_CNETMsg_SpawnGroup_LoadCompleted: Option<Box<Fn(&CNETMsg_SpawnGroup_LoadCompleted)>>,
    //////////
    pub on_CSVCMsg_ServerInfo: Option<Box<Fn(&CSVCMsg_ServerInfo)>>,
    pub on_CSVCMsg_CreateStringTable: Option<Box<Fn(&CSVCMsg_CreateStringTable)>>,
    pub on_CSVCMsg_UpdateStringTable: Option<Box<Fn(&CSVCMsg_UpdateStringTable)>>,
    /////////
    pub on_CUserMessageSayText: Option<Box<Fn(&CUserMessageSayText)>>,
    pub on_CUserMessageSayText2: Option<Box<Fn(&CUserMessageSayText2)>>,
    pub on_CUserMessageSayTextChannel: Option<Box<Fn(&CUserMessageSayTextChannel)>>,
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
            on_CNETMsg_Tick: None,
            on_CNETMsg_StringCmd: None,
            on_CNETMsg_SetConVar: None,
            on_CNETMsg_SignonState: None,
            on_CNETMsg_SpawnGroup_Load: None,
            on_CNETMsg_SpawnGroup_ManifestUpdate: None,
            on_CNETMsg_SpawnGroup_SetCreationTick: None,
            on_CNETMsg_SpawnGroup_Unload: None,
            on_CNETMsg_SpawnGroup_LoadCompleted: None,
            //////////
            on_CSVCMsg_ServerInfo: None,
            on_CSVCMsg_CreateStringTable: None,
            on_CSVCMsg_UpdateStringTable: None,
            /////////
            on_CUserMessageSayText: None,
            on_CUserMessageSayText2: None,
            on_CUserMessageSayTextChannel: None,
        }
    }
}