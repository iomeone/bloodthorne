extern crate protobuf;
use protobuf::stream::CodedInputStream;

extern crate snap;
use self::snap::Decoder;

use bitstream::BitStream;

use dota::demo::{EDemoCommands, CDemoFileHeader, CDemoFileInfo, CDemoPacket, CDemoFullPacket,
                 CDemoSendTables, CDemoClassInfo, CDemoStringTables, CDemoConsoleCmd,
                 CDemoCustomData, CDemoCustomDataCallbacks, CDemoUserCmd, CDemoSaveGame,
                 CDemoSpawnGroups, CDemoSyncTick};
// use dota::networkbasetypes::CNETMsg_Tick;
// use dota::netmessages::{CSVCMsg_ServerInfo, CCLCMsg_ClientInfo};
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
    }
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
                return Ok(());
            }

            if outer_message.kind == -1 {
                return Err(Error::new(ErrorKind::Other, "EDemoCommands::DEM_Error found"));
            }
        }

        Ok(())
    }

    fn handle_outer_message_by_type(&self, m: &OuterMessage) -> Result<()> {
        match m.kind {
            -1 => {} // EDemoCommands::DEM_Error
            0 => {} // EDemoCommands::DEM_Stop
            1 => {
                let c = protobuf::parse_from_bytes::<CDemoFileHeader>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoFileHeader, c);
            } // EDemoCommands::DEM_FileHeader
            2 => {
                let c = protobuf::parse_from_bytes::<CDemoFileInfo>(&m.data)?;
                call_if_exists!(self.callbacks.on_CDemoFileInfo, c);
            } // DemoCommands::DEM_FileInfo
            3 => {
                let c = protobuf::parse_from_bytes::<CDemoSyncTick>(&m.data)?;

            } // EDemoCommands::DEM_SyncTick
            4 => {
                let c = protobuf::parse_from_bytes::<CDemoSendTables>(&m.data)?;

            } // EDemoCommands::DEM_SendTables
            5 => {
                let c = protobuf::parse_from_bytes::<CDemoClassInfo>(&m.data)?;

            } // EDemoCommands::DEM_ClassInfo
            6 => {
                let c = protobuf::parse_from_bytes::<CDemoStringTables>(&m.data)?;

            } // EDemoCommands::DEM_StringTables
            7 | 8 => {
                let c = protobuf::parse_from_bytes::<CDemoPacket>(&m.data)?;

            } // EDemoCommands::DEM_SignonPacket // EDemoCommands::DEM_Packet
            9 => {
                let c = protobuf::parse_from_bytes::<CDemoConsoleCmd>(&m.data)?;

            } // EDemoCommands::DEM_ConsoleCmd
            10 => {
                let c = protobuf::parse_from_bytes::<CDemoCustomData>(&m.data)?;

            } // EDemoCommands::DEM_CustomData
            11 => {
                let c = protobuf::parse_from_bytes::<CDemoCustomDataCallbacks>(&m.data)?;

            } // EDemoCommands::DEM_CustomDataCallbacks
            12 => {
                let c = protobuf::parse_from_bytes::<CDemoUserCmd>(&m.data)?;

            } // EDemoCommands::DEM_UserCmd
            13 => {
                let c = protobuf::parse_from_bytes::<CDemoFullPacket>(&m.data)?;

            } // EDemoCommands::DEM_FullPacket
            14 => {
                let c = protobuf::parse_from_bytes::<CDemoSaveGame>(&m.data)?;

            } // EDemoCommands::DEM_SaveGame
            15 => {
                let c = protobuf::parse_from_bytes::<CDemoSpawnGroups>(&m.data)?;

            } // EDemoCommands::DEM_SpawnGroups
            16 => {} // EDemoCommands::DEM_Max
            64 => {} // EDemoCommands::DEM_IsCompressed
            _ => {}
        };

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


// fn entities_from_packet(packet: &CDemoPacket) -> Vec<Result<PacketEntity>> {
//     let packet_datas = PacketData::from_packet(packet).expect("Error getting packet data");
//     packet_datas.iter()
//         .map(|d| {
//             match d.kind {
//                     118 => {
//                         protobuf::parse_from_bytes::<CUserMessageSayText2>(&d.data)
//                             .map(PacketEntity::ChatMessage)
//                     }
//                     v => Ok(PacketEntity::Other(v)),
//                 }
//                 .map_err(Error::from)
//         })
//         .collect()
// }

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