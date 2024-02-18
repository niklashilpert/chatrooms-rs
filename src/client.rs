use std::{io::{self, BufRead, BufReader, Error, Write}, net::TcpStream};

pub fn start_client(username: String, password: String, address: String, port: u16) {
    let input = io::stdin();

    println!("[Setup] Started with username {username}");
    
    print!("[Setup] Enter a chatroom: ");
    io::stdout().flush().unwrap();
    
    let mut room = String::new();
    input.read_line(&mut room).unwrap();
    room = room.trim().to_string();

    loop {
        /*print!("> ");
        io::stdout().flush().unwrap();
        let mut message = String::new();
        input.read_line(&mut message).unwrap();
        message = message.trim().to_string();

        match TcpStream::connect(format!("{address}:{port}")) {
            Ok(mut stream) => {
                if let Err(_) = write_message_packet(&mut stream, &username, &password, &room, &message) {
                    println!("[Error] Could not write message to server.");
                    return;
                } else {
                    let reader = BufReader::new(&stream);
                    println!("Awaiting response");
                    for line in reader.lines() {
                        println!("Result: {}", line.unwrap());
                        break;
                    }
                    _ = stream.write_all(String::from("Terminate connection\n").as_bytes());
                }
            },
            Err(_) => {
                println!("[Error] Could not connect to the server.");
            }
        }*/
    }
    
}
