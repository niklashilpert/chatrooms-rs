use std::{io::BufReader, net::{TcpListener, TcpStream}, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread};

use crate::{db::{DbConn, Message, User}, packet::{self, write_packet, FullMessageUpdateContext, Metadata, Packet, PacketWithReturn, RoomListUpdateContext, SingleMessageUpdateContext}, threadpool::ThreadPool};

struct Client {
    id: i32,
    out_sender: Sender<Packet>
}


pub fn start(port: u16) {
    let mut counter = 0;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    let accept_pool = ThreadPool::new(4);

    let (in_send, in_recv): (Sender<PacketWithReturn>, Receiver<PacketWithReturn>) = mpsc::channel();
    
    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));

    let out_senders_clone = clients.clone();
    let server_handle = thread::spawn(move || handle_server_interaction(in_recv, out_senders_clone));

    for connection_attempt in listener.incoming() {
        if let Ok(stream) = connection_attempt {

            let in_send_clone = in_send.clone();

            let clients = clients.clone();

            accept_pool.execute(move || {
                let (out_send, out_recv): (Sender<Packet>, Receiver<Packet>) = mpsc::channel();    
                let stream_clone = stream.try_clone().unwrap();

                let out_send_clone = out_send.clone();
                clients.lock().unwrap().push(Client {
                    id: counter,
                    out_sender: out_send_clone,
                });
                

                thread::spawn(move || start_reading(stream_clone, in_send_clone, out_send));
                thread::spawn(move || start_writing(stream, out_recv));
            });   

            counter += 1;         
        }
    }

    let _ = server_handle.join();

}



/* SERVER THREAD */

fn handle_server_interaction(in_recv: Receiver<PacketWithReturn>, clients: Arc<Mutex<Vec<Client>>>) {
    
    let handle_pool = ThreadPool::new(4);

    loop {
        let clients = clients.clone();

        match in_recv.recv() {
            Ok(pwr) => handle_pool.execute(move || handle_packet(pwr, clients)),
            Err(_) => break,
        }
    }
    println!("COMMUNICATION ERROR");
}

fn handle_packet(pwr: PacketWithReturn, clients: Arc<Mutex<Vec<Client>>>) {
    
    let sender = pwr.return_sender;

    match pwr.packet {
        
        Packet::TerminateRequest => {},
        
        Packet::RoomRequest(metadata, room_id) => {
            require_auth(sender, metadata, move |sender, mut db, _| {
                _ = sender.send(Packet::RoomReply(db.get_room(room_id)));
            });

        },
        Packet::UserRequest(metadata, user_id) => {
            require_auth(sender, metadata, move |sender, mut db, _| {
                _ = sender.send(Packet::UserReply(db.get_user(user_id)));
            });
        },

        Packet::RoomListUpdateRequest(metadata) => {
            require_auth(sender, metadata, move |sender, mut db, _| {
                let _ = sender.send(Packet::RoomListUpdate(RoomListUpdateContext { rooms: db.get_rooms() }));
            });            
        },


        Packet::AddMessage(metadata, ctx) => {
            require_auth(sender, metadata, move |sender, mut db, user| {
                
                let room_opt = db.get_room(ctx.room_id);

                match room_opt {
                    Some(room) => {
                        db.insert_message(&ctx.message, &room, &user);

                        let message = db.get_newest_message(ctx.room_id).unwrap();
                        
                        let mut locked_clients = clients.lock().unwrap();

                        let mut to_remove = vec![];

                        for client in locked_clients.iter() {
                            let message = message.clone();
                            match client.out_sender.send(Packet::SingleMessageUpdate(SingleMessageUpdateContext {message, room: room.clone(), user: user.clone()})) {
                                Ok(_) => {},
                                Err(_) => {
                                    to_remove.push(client.id);
                                },
                            }
                        }

                        for id in to_remove {
                            locked_clients.retain(|c| c.id != id);
                        }
                    },
                    None => _ = sender.send(Packet::IllegalPacket),
                }

            });

           
        },
        Packet::FullMessageUpdateRequest(metadata, ctx) => {
            
            require_auth(sender, metadata, move |sender, mut db, _| {
                let room_opt = db.get_room(ctx.room_id);

                match room_opt {
                    Some(room) => {
                        let message_infos = db.get_messages_by_room(room.room_id);
                        _ = sender.send(Packet::FullMessageUpdate(FullMessageUpdateContext { messages: message_infos }));
                    },
                    None => {
                        _ = sender.send(Packet::IllegalPacket);
                    },
                }
            });
        },
        _ => {
            let _ = sender.send(Packet::IllegalPacket);
        }
    }
}


/* CONNECTION THREADS */

pub fn start_reading(stream: TcpStream, in_send: Sender<PacketWithReturn>, out_send: Sender<Packet>) {
    println!("[I]: Reading thread started");

    let mut reader = BufReader::new(&stream);

    loop {
        let packet = packet::read_packet(&mut reader);
        println!("[I] Packet received");
        
        let is_termination_request = if let Packet::TerminateRequest = &packet { true } else { false };
        
        let _ = in_send.send(PacketWithReturn{ packet, return_sender: out_send.clone() });
        
        if is_termination_request {
            break;
        }
    }

    let _ = stream.shutdown(std::net::Shutdown::Both);
    println!("[I] Reading thread stopped");
}

pub fn start_writing(mut stream: TcpStream, out_recv: Receiver<Packet>) {
    println!("[O]: Writing thread started");

    while let Ok(packet) = out_recv.recv() {
        if let Err(_) = packet::write_packet(&mut stream, &packet) {
            break;
        }
        println!("[O]: Packet sent");
    }
    let _ = stream.shutdown(std::net::Shutdown::Both);
    println!("[O]: Writing thread stopped");
}

pub fn verify_user(db: &mut DbConn ,metadata: &Metadata) -> bool {
    db.get_user_by_credentials(&metadata.username, &metadata.password).is_some()
}



fn require_auth<F>(out_send: Sender<Packet>, metadata: Metadata, f: F) where F: Fn(Sender<Packet>, DbConn, User) + 'static {
    
    let mut db = DbConn::new();
    let user_opt = db.get_user_by_credentials(&metadata.username, &metadata.password);
    match user_opt {
        Some(user) => f(out_send, db, user),
        None => _ = out_send.send(Packet::InvalidUser),
    }
}