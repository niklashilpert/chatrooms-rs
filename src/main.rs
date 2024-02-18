mod init;
mod server;
mod client;
mod threadpool;
mod db;
mod packet;
mod connection;

use std::env;
use init::InstanceParams;

use crate::db::get_db_credentials;


fn main() {
    println!("{}, {}", get_db_credentials().0, get_db_credentials().1);

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
        InstanceParams::Server(port) => {
            start_server(port);
        },
        InstanceParams::Client((username, password), address, port) => {
            start_client(username, password, address, port);
        },
    }
}

fn start_server(port: u16) {
    server::start(port);
}

fn start_client(username: String, password: String, address: String, port: u16) {
    client::start_client(username, password, address, port);
}



fn print_init_info(params: &InstanceParams) {
    match params {
        InstanceParams::Server(port) => {
            println!("Server: {}", port);
        },
        InstanceParams::Client((username, password), address, port) => {
            println!("Connect to {}:{} as {}:{}", address, port, username, password);
        }
    }
}



