mod init;
mod shared;
mod server;
mod client;
mod threadpool;

use std::env;
use init::InstanceParams;

use crate::server::Server;

fn main() {
    println!();
    
    let args = env::args().collect::<Vec<String>>();

    match init::parse_arguments(args) {
        Some(params) => {
            print_init_info(&params);
            start_program(params);
        },
        None => println!("No arguments passed")
    }

    println!();
}


fn start_program(params: InstanceParams) {
    match params {
        InstanceParams::Server(rooms, port) => {
            start_server(rooms, port);
        },
        InstanceParams::Client(username, address, port) => {
            start_client(username, address, port);
        },
    }
}

fn start_server(rooms: Vec<String>, port: u16) {
    let server = Server::start(port, Vec::new());
}

fn start_client(username: String, address: String, port: u16) {
    client::start_client(username, address, port);
}



fn print_init_info(params: &InstanceParams) {
    match params {
        InstanceParams::Server(rooms, port) => {
            println!("Server: {}", port);
            println!("Rooms: ");
            for room in rooms {
                println!("  - {}", room);
            }
        },
        InstanceParams::Client(username, address, port) => {
            println!("Connect to {}:{} as {}", address, port, username);
        }
    }
}



