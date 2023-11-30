mod init;
mod chat;

use std::env;
use init::InstanceParams;

fn main() {
    println!();
    
    let args = env::args().collect::<Vec<String>>();


    
    let params = init::parse_arguments(args);

    match params {
        Some(p) => {
            match p {
                InstanceParams::Server(rooms, port) => {
                    println!("Server: {}", port);
                    println!("Rooms: ");
                    for room in rooms {
                        println!("  - {}", room);
                    }
                },
                InstanceParams::Client(username, address, port) => {
                    println!("Connect to {}:{} as {}", address, port, username.unwrap_or(String::from("null")));
                }
            }
        },
        None => println!("None")
    }

    println!();

}



