extern crate protobuf;
use protobuf::stream::CodedInputStream;

extern crate snap;
use self::snap::Decoder;

use dota::demo::{CDemoFileHeader, CDemoFileInfo, CDemoPacket, CDemoFullPacket, CDemoSendTables,
                 CDemoClassInfo, CDemoStringTables, CDemoConsoleCmd, CDemoCustomData,
                 CDemoCustomDataCallbacks, CDemoUserCmd, CDemoSaveGame, CDemoSpawnGroups};
use dota::networkbasetypes::{CNETMsg_Disconnect, CNETMsg_SplitScreenUser, CNETMsg_Tick,
                             CNETMsg_StringCmd, CNETMsg_SetConVar, CNETMsg_SignonState,
                             CNETMsg_SpawnGroup_Load, CNETMsg_SpawnGroup_ManifestUpdate,
                             CNETMsg_SpawnGroup_SetCreationTick, CNETMsg_SpawnGroup_Unload,
                             CNETMsg_SpawnGroup_LoadCompleted};
use dota::netmessages::{CSVCMsg_ServerInfo, CSVCMsg_CreateStringTable, CSVCMsg_UpdateStringTable};
use dota::usermessages::{CUserMessageSayText2, CUserMessageSayText, CUserMessageSayTextChannel};

use callback::Callbacks;
use string_table::{StringTableItem, StringTable, StringTables};
use outer_message::OuterMessage;
use packet::PacketData;
use send_tables;

use std::io::{Result, Error, Read, ErrorKind};
use std::path::Path;
use std::fs::File;
use std::string::String;
use std::vec::Vec;
use std::cell::RefCell;
use std::collections::HashMap;
use std::f32;

macro_rules! call_if_exists {
    ($f:expr, $c:expr) => {
        if let Some(ref cb) = $f {
            cb($c);
        }
    };
    ($f:expr) => {
        if let Some(ref cb) = $f {
            cb();
        }
    };
}

/// ```
/// use std::path;
/// extern crate bloodthorne;
/// use bloodthorne::replay::Replay;
///
/// fn main() {
///     let mut replay = Replay::from_file(path::Path::new("example.dem")).expect("Error replay");
///
///     replay.callbacks.on_CUserMessageSayText2 = Some(Box::new(|ref m| {
///         println!("CUserMessageSayText2: `{}` says `{}`",
///                  m.get_param1(),
///                  m.get_param2());
///     }));
///
///     replay.parse().expect("Error parsing");
/// }
/// ```
pub struct Replay {
    bytes: Vec<u8>,
    pub callbacks: Callbacks,
    string_tables: RefCell<StringTables>,
    index_to_class_name: RefCell<HashMap<i32, String>>,
    class_id_size: u32,
    game_build: u32,
}

impl Replay {
    pub fn new(bytes: Vec<u8>) -> Result<Replay> {
        Ok(Replay {
            bytes: bytes,
            callbacks: Callbacks::new(),
            string_tables: RefCell::new(StringTables::new()),
            index_to_class_name: RefCell::new(HashMap::new()),
            class_id_size: 0,
            game_build: 0,
        })
    }

    pub fn from_file(path: &Path) -> Result<Replay> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Replay::new(bytes)
    }

    pub fn parse(&mut self) -> Result<()> {
        const HEADER_SOURCE_2: &'static [u8; 8] = b"PBDEMS2\0";
        if HEADER_SOURCE_2 != &self.bytes[0..8] {
            return Err(Error::new(ErrorKind::InvalidData,
                                  format!("Wrong header: expected {}, found {}",
                                          String::from_utf8_lossy(HEADER_SOURCE_2),
                                          String::from_utf8_lossy(&self.bytes[0..8]))));
        }

        let bytes = self.bytes.clone();
        let mut stream = CodedInputStream::from_bytes(&bytes[16..]);

        loop {
            let outer_message = OuterMessage::new(&mut stream)?;
            self.handle_outer_message_by_type(&outer_message)?;

            if outer_message.kind == 0 {
                // Stop
                return Ok(());
            }

            if outer_message.kind == -1 {
                // Error
                return Err(Error::new(ErrorKind::Other, "EDemoCommands::DEM_Error found"));
            }
        }
    }

    fn on_create_string_table(&self, s: &CSVCMsg_CreateStringTable) -> Result<()> {
        let buf = s.get_string_data();
        let mut data: Vec<u8> = buf.to_vec();

        if s.get_data_compressed() {
            if &buf[0..4] == b"LZSS" {
                return Err(Error::new(ErrorKind::InvalidData, "LZSS not supported"));
            }

            let mut decoder = Decoder::new();
            data = decoder.decompress_vec(&buf).map_err(Error::from)?;
        }

        let table_items = StringTableItem::parse_string_table(data,
                                                              s.get_num_entries(),
                                                              s.get_user_data_fixed_size(),
                                                              s.get_user_data_size())?;

        {
            let mut tables = self.string_tables.borrow_mut();
            let mut table = StringTable::new(s, tables.next_index());
            tables.incr_next_index();
            table.add_items(table_items);
            tables.add_table(table);
        }

        // TODO: "instancebaseline"

        Ok(())
    }

    fn on_update_string_table(&self, s: &CSVCMsg_UpdateStringTable) -> Result<()> {
        let mut tables = self.string_tables.borrow_mut();
        let table = tables.get_table(s.get_table_id());
        if table.is_none() {
            return Err(Error::new(ErrorKind::InvalidData, "Update non-existent string table"));
        }

        let table = table.unwrap();

        let items = StringTableItem::parse_string_table(s.get_string_data().to_vec(),
                                                        s.get_num_changed_entries(),
                                                        table.user_data_fixed_size(),
                                                        table.user_data_size())?;

        for item in items {
            table.update_or_create_item(item);
        }

        // TODO: "instancebaseline"

        Ok(())
    }

    fn handle_outer_message_by_type(&mut self, m: &OuterMessage) -> Result<()> {
        match m.kind {
            -1 => call_if_exists!(self.callbacks.on_CDemoError), // EDemoCommands::DEM_Error
            0 => call_if_exists!(self.callbacks.on_CDemoStop),  // EDemoCommands::DEM_Stop
            1 => {
                let c = protobuf::parse_from_bytes::<CDemoFileHeader>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoFileHeader, &c);
            } // EDemoCommands::DEM_FileHeader
            2 => {
                let c = protobuf::parse_from_bytes::<CDemoFileInfo>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoFileInfo, &c);
            } // DemoCommands::DEM_FileInfo
            3 => call_if_exists!(self.callbacks.on_CDemoSyncTick), // EDemoCommands::DEM_SyncTick
            4 => {
                let c = protobuf::parse_from_bytes::<CDemoSendTables>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoSendTables, &c);
                self.on_send_tables(c);
            } // EDemoCommands::DEM_SendTables
            5 => {
                let c = protobuf::parse_from_bytes::<CDemoClassInfo>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoClassInfo, &c);

                self.on_class_info(c);
            } // EDemoCommands::DEM_ClassInfo
            6 => {
                let c = protobuf::parse_from_bytes::<CDemoStringTables>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoStringTables, &c);
            } // EDemoCommands::DEM_StringTables
            7 => {
                let c = protobuf::parse_from_bytes::<CDemoPacket>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoSignonPacket, &c);
                self.handle_packet_by_type(&c)?;
            } // EDemoCommands::DEM_SignonPacket
            8 => {
                let c = protobuf::parse_from_bytes::<CDemoPacket>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoPacket, &c);
                self.handle_packet_by_type(&c)?;
            } // EDemoCommands::DEM_Packet
            9 => {
                let c = protobuf::parse_from_bytes::<CDemoConsoleCmd>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoConsoleCmd, &c);
            } // EDemoCommands::DEM_ConsoleCmd
            10 => {
                let c = protobuf::parse_from_bytes::<CDemoCustomData>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoCustomData, &c);
            } // EDemoCommands::DEM_CustomData
            11 => {
                let c = protobuf::parse_from_bytes::<CDemoCustomDataCallbacks>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoCustomDataCallbacks, &c);
            } // EDemoCommands::DEM_CustomDataCallbacks
            12 => {
                let c = protobuf::parse_from_bytes::<CDemoUserCmd>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoUserCmd, &c);
            } // EDemoCommands::DEM_UserCmd
            13 => {
                let c = protobuf::parse_from_bytes::<CDemoFullPacket>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoFullPacket, &c);
            } // EDemoCommands::DEM_FullPacket
            14 => {
                let c = protobuf::parse_from_bytes::<CDemoSaveGame>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoSaveGame, &c);
            } // EDemoCommands::DEM_SaveGame
            15 => {
                let c = protobuf::parse_from_bytes::<CDemoSpawnGroups>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoSpawnGroups, &c);
            } // EDemoCommands::DEM_SpawnGroups
            16 => call_if_exists!(self.callbacks.on_CDemoMax), // EDemoCommands::DEM_Max
            64 => call_if_exists!(self.callbacks.on_CDemoIsCompressed), // EDemoCommands::DEM_IsCompressed
            _ => call_if_exists!(self.callbacks.on_CDemoOther),
        };

        Ok(())
    }

    fn handle_packet_by_type(&mut self, packet: &CDemoPacket) -> Result<()> {
        let packet_datas = PacketData::from_packet(packet)?;

        for d in packet_datas {
            match d.kind {
                0 => call_if_exists!(self.callbacks.on_CNETMsg_NOP), // NET_Messages::net_NOP
                1 => {
                    // NET_Messages::net_Disconnect
                    let e = protobuf::parse_from_bytes::<CNETMsg_Disconnect>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_Disconnect, &e);
                }
                3 => {
                    // NET_Messages::net_SplitScreenUser
                    let e = protobuf::parse_from_bytes::<CNETMsg_SplitScreenUser>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_SplitScreenUser, &e);
                }
                4 => {
                    // NET_Messages::net_Tick
                    let e = protobuf::parse_from_bytes::<CNETMsg_Tick>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_Tick, &e);
                }
                5 => {
                    // NET_Messages::net_StringCmd
                    let e = protobuf::parse_from_bytes::<CNETMsg_StringCmd>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_StringCmd, &e);
                }
                6 => {
                    // NET_Messages::net_SetConVar
                    let e = protobuf::parse_from_bytes::<CNETMsg_SetConVar>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_SetConVar, &e);
                }
                7 => {
                    // NET_Messages::net_SignonState
                    let e = protobuf::parse_from_bytes::<CNETMsg_SignonState>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_SignonState, &e);
                }
                8 => {
                    // NET_Messages::net_SpawnGroup_Load
                    let e = protobuf::parse_from_bytes::<CNETMsg_SpawnGroup_Load>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_SpawnGroup_Load, &e);
                }
                9 => {
                    // NET_Messages::net_SpawnGroup_ManifestUpdate
                    let e =
                        protobuf::parse_from_bytes::<CNETMsg_SpawnGroup_ManifestUpdate>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_SpawnGroup_ManifestUpdate, &e);
                }
                11 => {
                    // NET_Messages::net_SpawnGroup_SetCreationTick
                    let e =
                        protobuf::parse_from_bytes::<CNETMsg_SpawnGroup_SetCreationTick>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_SpawnGroup_SetCreationTick, &e);
                }
                12 => {
                    // NET_Messages::net_SpawnGroup_Unload
                    let e = protobuf::parse_from_bytes::<CNETMsg_SpawnGroup_Unload>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_SpawnGroup_Unload, &e);
                }
                13 => {
                    // NET_Messages::net_SpawnGroup_LoadCompleted
                    let e =
                        protobuf::parse_from_bytes::<CNETMsg_SpawnGroup_LoadCompleted>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CNETMsg_SpawnGroup_LoadCompleted, &e);
                }
                40 => {
                    // SVC_Messages::svc_ServerInfo
                    let e = protobuf::parse_from_bytes::<CSVCMsg_ServerInfo>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CSVCMsg_ServerInfo, &e);

                    self.on_server_info(&e);
                }
                44 => {
                    // SVC_Messages::svc_CreateStringTable
                    let e = protobuf::parse_from_bytes::<CSVCMsg_CreateStringTable>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CSVCMsg_CreateStringTable, &e);

                    self.on_create_string_table(&e)?;
                }
                45 => {
                    // SVC_Messages::svc_UpdateStringTable
                    let e = protobuf::parse_from_bytes::<CSVCMsg_UpdateStringTable>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CSVCMsg_UpdateStringTable, &e);

                    self.on_update_string_table(&e)?;
                }
                117 => {
                    // EBaseUserMessages::UM_SayText
                    let e = protobuf::parse_from_bytes::<CUserMessageSayText>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CUserMessageSayText, &e);
                }
                118 => {
                    // EBaseUserMessages::UM_SayText2
                    let e = protobuf::parse_from_bytes::<CUserMessageSayText2>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CUserMessageSayText2, &e);
                }
                119 => {
                    // EBaseUserMessages::UM_SayTextChannel
                    let e = protobuf::parse_from_bytes::<CUserMessageSayTextChannel>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CUserMessageSayTextChannel, &e);
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn on_server_info(&mut self, c: &CSVCMsg_ServerInfo) {
        let max_classes = c.get_max_classes() as f32;
        self.class_id_size = (max_classes / (2.0f32).ln()) as u32 + 1;
        debug!("class_id_size: {}", self.class_id_size);
        debug!("game dir: {}", c.get_game_dir());

        self.game_build = 1; // FIXME
    }

    fn on_class_info(&self, mut c: CDemoClassInfo) {
        let mut index_to_class_name = self.index_to_class_name.borrow_mut();
        let mut classes = c.take_classes();
        for class in classes.iter_mut() {
            index_to_class_name.insert(class.get_class_id(), class.take_network_name());

            // TODO: check if class info already exists from send tables
        }

        // TODO: update baseline info
    }

    fn on_send_tables(&self, s: CDemoSendTables) {
        let _todo = send_tables::parse_send_tables(s);
    }
}
