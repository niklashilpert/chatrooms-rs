/* Imports */
use std::{
    io::{
        BufRead, 
        BufReader, Read
    }, net::{
        TcpListener,
        TcpStream
    }, ptr::read, sync::mpsc::{
        self, 
        Receiver, Sender
    }, time::SystemTime
};

use crate::{
    shared::{MessagePacket, MESSAGE_CONTENT_PREFIX, MESSAGE_LENGTH_PREFIX, MESSAGE_PROTOCOL_LINE, MESSAGE_ROOM_PREFIX, MESSAGE_USERNAME_PREFIX}, 
    threadpool::ThreadPool
};


pub struct Room {
    name: String,
    messages: Vec<Message>,
}

pub struct Message {
    time: SystemTime,
    user: String,
    content: String,
}

/* Code */
pub struct Server {
    port: u16,
    listener: TcpListener,
    threadpool: ThreadPool,
    receiver: Receiver<MessagePacket>,
}

impl Server {
    pub fn start(port: u16, rooms: Vec<String>) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
        let threadpool = ThreadPool::new(4);

        let (sender, receiver) = mpsc::channel();

        let server = Server {
            listener,
            port,
            threadpool,
            receiver,
        };
        
        for connection_attempt in server.listener.incoming() {
            if let Ok(stream) = connection_attempt {
                let sender_clone = sender.clone();
                server.threadpool.execute(move || handle_connection(stream, sender_clone));
            }
        }
    }
}



fn handle_connection(mut stream: TcpStream, sender: Sender<MessagePacket>) {
    if let Some(packet) = read_message_packet_from(BufReader::new(&mut stream)) {
        if let Ok(_) = sender.send(packet) {
            println!("[Incoming Message] Forwarded to server");
        } else {
            println!("[Error] Could not forward message to the server");
        }
    } else {
        println!("[Error] Invalid connection attempt");
    }

}

fn read_message_packet_from(mut reader: BufReader<&mut TcpStream>) -> Option<MessagePacket> {
    let mut protocol_line = String::new();
    reader.read_line(&mut protocol_line).ok()?;
    protocol_line = protocol_line.replace("\n", "").trim().to_string();

    println!("{protocol_line}");
    println!("{MESSAGE_PROTOCOL_LINE}");

    if protocol_line.trim() != MESSAGE_PROTOCOL_LINE.to_string() {
        return None;
    }

    let username = get_next_item(&mut reader, MESSAGE_USERNAME_PREFIX)?;
    println!("Username: {username}");
    
    let room = get_next_item(&mut reader, MESSAGE_ROOM_PREFIX)?;
    println!("Room: {room}");

    let length_string = get_next_item(&mut reader, MESSAGE_LENGTH_PREFIX)?;
    let length: usize = length_string.parse().ok()?;
    
    
    let mut unused_content: Vec<u8> = vec![0; MESSAGE_CONTENT_PREFIX.len()];
    reader.read_exact(&mut unused_content).ok()?;
    
    
    let mut content: Vec<u8> = vec![0; length];
    reader.read_exact(&mut content).ok()?;
    let content: String = String::from_utf8(content).ok()?;

    println!("[Incoming Message] {username} > {room}: \"{content}\"");

    Some(MessagePacket {
        username,
        room,
        content,
    })

}

fn get_next_item(reader: &mut BufReader<&mut TcpStream>, prefix: &str) -> Option<String> {
    let mut line = String::new();
    println!("test1: {line}");
    reader.read_line(&mut line).ok()?;
    println!("test2: {line}");
    line = line.replace("\n", "").trim().to_string();

    println!("\"{line}\"");

    return if line.starts_with(prefix) {
        Some(line.chars().skip(prefix.len()).collect())    
    } else {
        None
    }
}