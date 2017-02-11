extern crate protobuf;
use protobuf::stream::CodedInputStream;

extern crate snap;
use self::snap::Decoder;

use bitstream::BitStream;

use dota::demo::{EDemoCommands, CDemoFileHeader, CDemoFileInfo, CDemoPacket, CDemoFullPacket,
                 CDemoSendTables, CDemoClassInfo, CDemoStringTables, CDemoConsoleCmd,
                 CDemoCustomData, CDemoCustomDataCallbacks, CDemoUserCmd, CDemoSaveGame,
                 CDemoSpawnGroups};
use dota::networkbasetypes::{CNETMsg_Disconnect, CNETMsg_SplitScreenUser, CNETMsg_Tick,
                             CNETMsg_StringCmd, CNETMsg_SetConVar, CNETMsg_SignonState,
                             CNETMsg_SpawnGroup_Load, CNETMsg_SpawnGroup_ManifestUpdate,
                             CNETMsg_SpawnGroup_SetCreationTick, CNETMsg_SpawnGroup_Unload,
                             CNETMsg_SpawnGroup_LoadCompleted};
use dota::netmessages::CSVCMsg_ServerInfo;
use dota::usermessages::CUserMessageSayText2;

use callback::Callbacks;

use std::io::{Result, Error, Read, ErrorKind};
use std::path::Path;
use std::fs::File;

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

pub struct Replay {
    bytes: Vec<u8>,
    pub callbacks: Callbacks,
}

impl Replay {
    pub fn new(bytes: Vec<u8>) -> Result<Replay> {
        Ok(Replay {
            bytes: bytes,
            callbacks: Callbacks::new(),
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
                                  format!("Wrong header: expect {:?}, found {:?}",
                                          HEADER_SOURCE_2,
                                          &self.bytes[0..8])));
        }

        let mut stream = CodedInputStream::from_bytes(&self.bytes[16..]);

        loop {
            let outer_message = OuterMessage::new(&mut stream)?;
            self.handle_outer_message_by_type(&outer_message).unwrap();

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

    fn handle_outer_message_by_type(&self, m: &OuterMessage) -> Result<()> {
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
            } // EDemoCommands::DEM_SendTables
            5 => {
                let c = protobuf::parse_from_bytes::<CDemoClassInfo>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoClassInfo, &c);
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

    fn handle_packet_by_type(&self, packet: &CDemoPacket) -> Result<()> {
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
                }
                118 => {
                    // EBaseUserMessages::UM_SayText2
                    let e = protobuf::parse_from_bytes::<CUserMessageSayText2>(&d.data)?;
                    call_if_exists!(self.callbacks.on_CUserMessageSayText2, &e);
                }
                _ => {}
            }
        }

        Ok(())
    }
}

struct OuterMessage {
    kind: i32,
    tick: u32,
    data: Vec<u8>,
}

impl OuterMessage {
    fn new(stream: &mut CodedInputStream) -> Result<OuterMessage> {
        let command = stream.read_uint32()?;
        let is_compressed_bitmask = EDemoCommands::DEM_IsCompressed as u32;
        let is_compressed = (command & is_compressed_bitmask) == is_compressed_bitmask;

        let mut kind: i32 = command as i32 & !is_compressed_bitmask as i32;
        if is_compressed {
            kind = kind & !is_compressed_bitmask as i32;
        }

        let mut tick = stream.read_uint32().map_err(Error::from)?;
        if tick == <u32>::max_value() {
            tick = 0;
        }

        let size = stream.read_uint32().map_err(Error::from)?;

        let mut data = stream.read_raw_bytes(size).map_err(Error::from)?;
        if is_compressed {
            let mut decoder = Decoder::new();
            data = decoder.decompress_vec(&data).map_err(Error::from)?;
        }

        Ok(OuterMessage {
            kind: kind,
            tick: tick,
            data: data,
        })
    }
}

struct PacketData {
    kind: u32,
    data: Vec<u8>,
}

impl PacketData {
    fn new(bitstream: &mut BitStream) -> Result<PacketData> {
        let kind = bitstream.read_ubitvarint().map_err(Error::from)?;
        let size = bitstream.read_u32var().map_err(Error::from)?;
        let data = bitstream.read_bytes(size as usize).map_err(Error::from)?;

        Ok(PacketData {
            kind: kind,
            data: data,
        })
    }

    fn from_packet(packet: &CDemoPacket) -> Result<Vec<PacketData>> {
        let data = packet.get_data().to_vec();
        let mut bitstream = BitStream::new(data);

        let mut res = Vec::new();

        while bitstream.remaining_bytes() > 0 {
            let packet_data = PacketData::new(&mut bitstream)?;
            res.push(packet_data);
        }

        Ok(res)
    }
}