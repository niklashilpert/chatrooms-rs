use std::{io::{self, BufRead, BufReader, Error, Write}, net::TcpStream, thread};

use crate::packet::{self, AddMessageContext, FullMessageUpdateRequestContext, Metadata, Packet, RoomListUpdateContext, SingleMessageUpdateContext};

pub fn start(username: String, password: String, address: String, port: u16) {
    let input = io::stdin();

    println!("[Setup] Started with username {username}");

    let metadata = Metadata {username, password };
    
    match TcpStream::connect(format!("{address}:{port}")) {
        Ok(mut stream) => {
            println!("Connection established.");


            let mut current_room_id = 1;

            let stream_clone = stream.try_clone().unwrap();
            thread::spawn(move || {
                let mut reader = BufReader::new(&stream_clone);
                loop {
                    let packet = packet::read_packet(&mut reader);
                    
                    match packet {
                        Packet::SingleMessageUpdate(ctx) => {
                            println!("[{}] {} {} > {}: {}", ctx.message.timestamp, ctx.user.first_name, ctx.user.last_name, ctx.room.room_name, ctx.message.content);
                        },
                        _ => {
                            println!("{}", serde_json::to_string_pretty(&packet).unwrap());
                        }
                    }
                    
                    



                }
            });

            loop {
                let metadata = metadata.clone();

                print!("> ");
                io::stdout().flush().unwrap();
                let mut buf = String::new();
                input.read_line(&mut buf).unwrap();
                let buf = buf.trim();

                let command: Vec<&str> = buf.split(" ").collect();

                if (command.len() > 0) {
                    match command[0] {
                        "room_list" => {
                            _ = packet::write_packet(&mut stream, &Packet::RoomListUpdateRequest(metadata));
                            
                        },
                        "select_room" => {
                            current_room_id = command[1].parse().unwrap();
                        },
                        "list_messages" => {
                            _ = packet::write_packet(&mut stream, &Packet::FullMessageUpdateRequest(metadata, FullMessageUpdateRequestContext { room_id: current_room_id }));
                           
                        },
                        
                        "exit" => {
                            return;
                        }
                        _ => {
                            _ = packet::write_packet(&mut stream, &Packet::AddMessage(metadata, AddMessageContext { message: command[..].join(" "), room_id: current_room_id }));
                        }
                    }
                }

            }
        },
        Err(_) => {
            println!("[Error] Could not connect to the server.");
        }
    }      
}
  