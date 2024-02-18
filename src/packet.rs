use std::{io::{BufRead, BufReader, Write}, net::TcpStream, sync::mpsc::Sender};

use serde::{Deserialize, Serialize};

use crate::db::{Room, User};


#[derive(Serialize, Deserialize)]
pub struct ClientMessageInfo {
    pub message_id: i32,
    pub message: String,
    pub message_timestamp: String,
    pub user: User,
    pub room: Room,
}



#[derive(Serialize, Deserialize)]
pub struct LoginContext {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddMessageContext {
    pub room: Room,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SingleMessageUpdateContext {
    pub message_info: ClientMessageInfo,
}

#[derive(Serialize, Deserialize)]
pub struct FullMessageUpdateRequestContext {
    pub room_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FullMessageUpdateContext {
    pub message_infos: Vec<ClientMessageInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct RoomListUpdateContext {
    pub rooms: Vec<Room>,
}



pub struct PacketWithReturn {
    pub packet: Packet,
    pub return_sender: Sender<Packet>
}

#[derive(Serialize, Deserialize)]
pub enum Packet {
    TerminateRequest,
    IllegalPacket,
    Login(LoginContext),
    LoginAccept,
    LoginDecline,
    RoomListUpdateRequest,
    RoomListUpdate(RoomListUpdateContext),
    FullMessageUpdateRequest(FullMessageUpdateRequestContext),
    FullMessageUpdate(FullMessageUpdateContext),
    AddMessage(AddMessageContext),
    SingleMessageUpdate(SingleMessageUpdateContext),
}


pub fn read_packet(reader: &mut BufReader<&TcpStream>) -> Packet {
    let mut buf: String = String::new();
    let _ = reader.read_line(&mut buf);
    
    serde_json::from_str(buf.trim()).unwrap_or(Packet::TerminateRequest)
}

pub fn write_packet(stream: &mut TcpStream, packet: &Packet) -> Result<(), std::io::Error> {
    stream.write_all(format!("{}\n", serde_json::to_string(packet).unwrap()).as_bytes())
}