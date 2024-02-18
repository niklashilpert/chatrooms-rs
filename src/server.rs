use std::{net::TcpListener, sync::mpsc::{self, Receiver, Sender}, thread};

use crate::{connection, db::DbConn, packet::{Packet, PacketWithReturn, RoomListUpdateContext}, threadpool::ThreadPool};




pub fn start(port: u16) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    let accept_pool = ThreadPool::new(4);

    let (in_send, in_recv): (Sender<PacketWithReturn>, Receiver<PacketWithReturn>) = mpsc::channel();
    
    let server_handle = thread::spawn(move || handle_server_interaction(in_recv));

    for connection_attempt in listener.incoming() {
        if let Ok(stream) = connection_attempt {

            let in_send_clone = in_send.clone();

            accept_pool.execute(move || {
                let (out_send, out_recv): (Sender<Packet>, Receiver<Packet>) = mpsc::channel();    
                let stream_clone = stream.try_clone().unwrap();
                thread::spawn(move || connection::start_reading(stream_clone, in_send_clone, out_send));
                thread::spawn(move || connection::start_writing(stream, out_recv));
            });            
        }
    }

    let _ = server_handle.join();

}



fn handle_server_interaction(in_recv: Receiver<PacketWithReturn>) {
    
    let handle_pool = ThreadPool::new(4);

    loop {
        match in_recv.recv() {
            Ok(pwr) => handle_pool.execute(move || handle_packet(pwr)),
            Err(_) => break,
        }
    }
    println!("COMMUNICATION ERROR");
}

fn handle_packet(pwr: PacketWithReturn) {
    
    match pwr.packet {
        Packet::TerminateRequest => {},
        Packet::Login(ctx) => {
            
        },
        Packet::RoomListUpdateRequest => {
            let mut db = DbConn::new();
            let rooms = db.get_rooms();
            let _ = pwr.return_sender.send(Packet::RoomListUpdate(RoomListUpdateContext { rooms }));
        },
        Packet::AddMessage(ctx) => {},
        Packet::FullMessageUpdateRequest(ctx) => {},
        _ => {
            let _ = pwr.return_sender.send(Packet::IllegalPacket);
        }
    }
}
