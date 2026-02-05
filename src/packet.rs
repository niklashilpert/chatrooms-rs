use std::{io::{BufRead, BufReader, Write}, net::TcpStream, sync::mpsc::Sender};

use serde::{Deserialize, Serialize};

use crate::db::{Message, Room, User};



#[derive(Serialize, Deserialize, Clone)]
pub struct Metadata {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddMessageContext {
    pub room_id: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SingleMessageUpdateContext {
    pub message: Message,
    pub user: User,
    pub room: Room,
}

#[derive(Serialize, Deserialize)]
pub struct FullMessageUpdateRequestContext {
    pub room_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct FullMessageUpdateContext {
    pub messages: Vec<Message>,
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
    RoomRequest(Metadata, i32),
    UserRequest(Metadata, i32),

    RoomReply(Option<Room>),
    UserReply(Option<User>),

    TerminateRequest,
    IllegalPacket,
    InvalidUser,
    RoomListUpdateRequest(Metadata),
    RoomListUpdate(RoomListUpdateContext),
    FullMessageUpdateRequest(Metadata, FullMessageUpdateRequestContext),
    FullMessageUpdate(FullMessageUpdateContext),
    AddMessage(Metadata, AddMessageContext),
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