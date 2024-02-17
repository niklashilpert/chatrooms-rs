use std::{fmt::write, io::{self, Error, Write}, net::TcpStream};

use crate::shared::{MESSAGE_PROTOCOL_LINE, MESSAGE_USERNAME_PREFIX, MESSAGE_ROOM_PREFIX, MESSAGE_LENGTH_PREFIX, MESSAGE_CONTENT_PREFIX};

pub fn start_client(username: String, address: String, port: u16) {
    let input = io::stdin();

    println!("[Setup] Started with username {username}");
    
    print!("[Setup] Enter a chatroom: ");
    io::stdout().flush().unwrap();
    
    let mut room = String::new();
    input.read_line(&mut room).unwrap();
    room = room.trim().to_string();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut message = String::new();
        input.read_line(&mut message).unwrap();
        message = message.trim().to_string();

        match TcpStream::connect(format!("{address}:{port}")) {
            Ok(mut stream) => {
                if let Err(_) = write_message_packet(&mut stream, &username, &room, &message) {
                    println!("[Error] Could not write message to server.");
                    return;
                }
            },
            Err(_) => {
                println!("[Error] Could not connect to the server.");
            }
        }
    }
    
}

fn write_message_packet(stream: &mut TcpStream, username: &String, room: &String, content: &String) -> Result<(), Error> {
    let content_length = content.len();
    println!("{MESSAGE_PROTOCOL_LINE}");
    println!("{MESSAGE_USERNAME_PREFIX}{username}");
    println!("{MESSAGE_ROOM_PREFIX}{room}");
    println!("{MESSAGE_LENGTH_PREFIX}{content_length}");
    println!("{MESSAGE_CONTENT_PREFIX}{content}");

    stream.write_all(format!("{MESSAGE_PROTOCOL_LINE}\n").as_bytes()).unwrap();
    stream.write_all(format!("{MESSAGE_USERNAME_PREFIX}{username}\n").as_bytes()).unwrap();
    stream.write_all(format!("{MESSAGE_ROOM_PREFIX}{room}\n").as_bytes()).unwrap();
    stream.write_all(format!("{MESSAGE_LENGTH_PREFIX}{content_length}\n").as_bytes()).unwrap();
    stream.write_all(format!("{MESSAGE_CONTENT_PREFIX}{content}\n").as_bytes()).unwrap();
    Ok(())
}