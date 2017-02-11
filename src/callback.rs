use dota::demo::{EDemoCommands, CDemoFileHeader, CDemoFileInfo, CDemoPacket, CDemoFullPacket,
                 CDemoSendTables, CDemoClassInfo, CDemoStringTables, CDemoConsoleCmd,
                 CDemoCustomData, CDemoCustomDataCallbacks, CDemoUserCmd, CDemoSaveGame,
                 CDemoSpawnGroups, CDemoSyncTick};

pub struct Callbacks {
    pub on_CDemoFileHeader: Option<Box<Fn(CDemoFileHeader)>>,
    pub on_CDemoFileInfo: Option<Box<Fn(CDemoFileInfo)>>,
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks {
            on_CDemoFileHeader: None,
            on_CDemoFileInfo: None,
        }
    }
}