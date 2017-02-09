extern crate protobuf;
use protobuf::stream::CodedInputStream;

extern crate snap;
use self::snap::Decoder;

use bitstream::BitStream;

use dota::demo::{EDemoCommands, CDemoFileHeader, CDemoFileInfo, CDemoPacket, CDemoFullPacket,
                 CDemoSendTables, CDemoClassInfo, CDemoStringTables, CDemoConsoleCmd,
                 CDemoCustomData, CDemoCustomDataCallbacks, CDemoUserCmd, CDemoSaveGame,
                 CDemoSpawnGroups};
// use dota::networkbasetypes::CNETMsg_Tick;
// use dota::netmessages::{CSVCMsg_ServerInfo, CCLCMsg_ClientInfo};
use dota::usermessages::CUserMessageSayText2;

use std::io::{Result, Error};

pub struct Replay {}

impl Replay {
    pub fn new(bytes: Vec<u8>) -> Replay {
        const HEADER_SOURCE_2: &'static [u8; 8] = b"PBDEMS2\0";
        assert_eq!(HEADER_SOURCE_2, &bytes[0..8]);

        let mut stream = CodedInputStream::from_bytes(&bytes[16..]);

        (0..)
            .map(|_| OuterMessage::new(&mut stream).expect("Error OuterMessage"))
            .take_while(|m| m.kind > 0)
            .map(|m| from_message(&m).expect("Error from_message"))
            .filter(|c| match *c {
                DemoCommand::SignonPacket(_) |
                DemoCommand::Packet(_) => true,
                _ => false,
            })
            .map(|c| match c {
                DemoCommand::SignonPacket(p) |
                DemoCommand::Packet(p) => p,
                _ => unreachable!(), 
            })
            .map(|p| entities_from_packet(&p))
            .flat_map(|entities| entities.into_iter())
            .filter(|e| e.is_ok())
            .map(|e| e.unwrap())
            .map(|e| match e {
                PacketEntity::ChatMessage(ref c) => {
                    println!("{} says {}", c.get_param1(), c.get_param2());
                }
                _ => {}
            })
            .collect::<Vec<()>>();

        Replay {}
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

enum DemoCommand {
    Error,
    Stop,
    FileHeader(CDemoFileHeader),
    FileInfo(CDemoFileInfo),
    SyncTick,
    SendTables(CDemoSendTables),
    ClassInfo(CDemoClassInfo),
    SignonPacket(CDemoPacket),
    Packet(CDemoPacket),
    ConsoleCmd(CDemoConsoleCmd),
    StringTables(CDemoStringTables),
    CustomData(CDemoCustomData),
    CustomDataCallbacks(CDemoCustomDataCallbacks),
    UserCmd(CDemoUserCmd),
    FullPacket(CDemoFullPacket),
    SaveGame(CDemoSaveGame),
    SpawnGroups(CDemoSpawnGroups),
    Max,
    IsCompressed,
    Other(i32),
}

fn from_message(message: &OuterMessage) -> Result<DemoCommand> {
    match message.kind {
            -1 => Ok(DemoCommand::Error), // EDemoCommands::DEM_Error
            0 => Ok(DemoCommand::Stop), // EDemoCommands::DEM_Stop
            1 => {
                protobuf::parse_from_bytes::<CDemoFileHeader>(&message.data)
                    .map(DemoCommand::FileHeader)
            } // EDemoCommands::DEM_FileHeader
            2 => {
                protobuf::parse_from_bytes::<CDemoFileInfo>(&message.data)
                    .map(DemoCommand::FileInfo)
            } // DemoCommands::DEM_FileInfo
            3 => Ok(DemoCommand::SyncTick), // EDemoCommands::DEM_SyncTick
            4 => {
                protobuf::parse_from_bytes::<CDemoSendTables>(&message.data)
                    .map(DemoCommand::SendTables)
            } // EDemoCommands::DEM_SendTables
            5 => {
                protobuf::parse_from_bytes::<CDemoClassInfo>(&message.data)
                    .map(DemoCommand::ClassInfo)
            } // EDemoCommands::DEM_ClassInfo
            6 => {
                protobuf::parse_from_bytes::<CDemoStringTables>(&message.data)
                    .map(DemoCommand::StringTables)
            } // EDemoCommands::DEM_StringTables
            7 => {
                protobuf::parse_from_bytes::<CDemoPacket>(&message.data)
                    .map(DemoCommand::SignonPacket)
            } // EDemoCommands::DEM_SignonPacket
            8 => {
                protobuf::parse_from_bytes::<CDemoPacket>(&message.data)
                    .map(DemoCommand::Packet)
            } // EDemoCommands::DEM_Packet
            9 => {
                protobuf::parse_from_bytes::<CDemoConsoleCmd>(&message.data)
                    .map(DemoCommand::ConsoleCmd)
            } // EDemoCommands::DEM_ConsoleCmd
            10 => {
                protobuf::parse_from_bytes::<CDemoCustomData>(&message.data)
                    .map(DemoCommand::CustomData)
            } // EDemoCommands::DEM_CustomData
            11 => {
                protobuf::parse_from_bytes::<CDemoCustomDataCallbacks>(&message.data)
                    .map(DemoCommand::CustomDataCallbacks)
            } // EDemoCommands::DEM_CustomDataCallbacks
            12 => {
                protobuf::parse_from_bytes::<CDemoUserCmd>(&message.data)
                    .map(DemoCommand::UserCmd)
            } // EDemoCommands::DEM_UserCmd
            13 => {
                protobuf::parse_from_bytes::<CDemoFullPacket>(&message.data)
                    .map(DemoCommand::FullPacket)
            } // EDemoCommands::DEM_FullPacket
            14 => {
                protobuf::parse_from_bytes::<CDemoSaveGame>(&message.data)
                    .map(DemoCommand::SaveGame)
            } // EDemoCommands::DEM_SaveGame
            15 => {
                protobuf::parse_from_bytes::<CDemoSpawnGroups>(&message.data)
                    .map(DemoCommand::SpawnGroups)
            } // EDemoCommands::DEM_SpawnGroups
            16 => Ok(DemoCommand::Max), // EDemoCommands::DEM_Max
            64 => Ok(DemoCommand::IsCompressed), // EDemoCommands::DEM_IsCompressed
            other => Ok(DemoCommand::Other(other)),
        }
        .map_err(Error::from)
}

enum PacketEntity {
    ChatMessage(CUserMessageSayText2),
    Other(u32),
}

fn entities_from_packet(packet: &CDemoPacket) -> Vec<Result<PacketEntity>> {
    let packet_datas = PacketData::from_packet(packet).expect("Error getting packet data");
    packet_datas.iter()
        .map(|d| {
            match d.kind {
                    118 => {
                        protobuf::parse_from_bytes::<CUserMessageSayText2>(&d.data)
                            .map(PacketEntity::ChatMessage)
                    }
                    v => Ok(PacketEntity::Other(v)),
                }
                .map_err(Error::from)
        })
        .collect()
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
        println!("{:?}", data);
        let mut bitstream = BitStream::new(data);

        let mut res = Vec::new();

        while bitstream.remaining_bytes() > 0 {
            let packet_data = PacketData::new(&mut bitstream)?;
            res.push(packet_data);
        }

        Ok(res)
    }
}