mod threadpool;
mod db;
mod packet;
mod server;
mod client;

use clap::{arg, command, Args, Parser};

#[derive(Parser)]
#[command(name = "chatrooms-rs")]
pub struct App {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
enum Command {
    #[command(name = "server")]
    Server(ServerArgs),
    #[command(name = "client")]
    Client(ClientArgs),
}

#[derive(Args)]
struct ServerArgs {
    #[arg(short, long)]
    port: u16,
}

#[derive(Args)]
struct ClientArgs {
    /// Address of the server. Expected in the following format: <ip>:<port>
    #[arg(short, long)]
    address: String,

    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    password: String,
}

fn main() {
    let app = App::parse();

    match app.command {
        Some(command) => match command {
            Command::Client(args) => {
                match split_address(args.address) {
                    Ok((address, port)) => {
                        client::start(args.user, args.password, address, port);
                    },
                    Err(code) => match code {
                        0 => println!("The address is in the wrong format. Use --help for more info."),
                        _ => println!("The given port is invalid."),
                    },
                    
                }
            },
            Command::Server(args) => {
                server::start(args.port)
            },
        },
        None => {
            println!("none");
        },
    }


}

fn split_address(addr: String) -> Result<(String, u16), u8> {
    let addr_parts: Vec<&str> = addr.split(":").collect();
    if addr_parts.len() == 2 {

        let ip = addr_parts[0].to_string();
        let port = match addr_parts[1].parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!("The given port is not valid.");
                return Err(1);
            }
        };

        return Ok((ip, port));
        
    } else {
        println!("The address is in the wrong format. Use --help for more info.");
        return Err(0);
    }
}





