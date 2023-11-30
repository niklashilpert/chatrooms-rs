mod init;
mod chat;

use std::env;
use init::InstanceParams;

fn main() {
    println!();
    
    let args = env::args().collect::<Vec<String>>();

    match init::parse_arguments(args) {
        Some(params) => {
            print_init_info(params);
            match params {
                InstanceParams::Server(rooms, port) => {
                    
                },
                InstanceParams::Client(username, address, port) => {
                    
                }
            }
        },
        None => println!("None")
    }

    println!();

}

fn print_init_info(params: InstanceParams) {
    match params {
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
}



