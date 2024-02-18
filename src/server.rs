use std::{io::BufReader, net::{TcpListener, TcpStream}, sync::mpsc::{self, Receiver, Sender}, thread};

use crate::{db::DbConn, packet::{self, Packet, PacketWithReturn, RoomListUpdateContext}, threadpool::ThreadPool};


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
                thread::spawn(move || start_reading(stream_clone, in_send_clone, out_send));
                thread::spawn(move || start_writing(stream, out_recv));
            });            
        }
    }

    let _ = server_handle.join();

}



/* SERVER THREAD */

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

    let _ = stream.shutdown(std::net::Shutdown::Read);
    println!("[I] Reading thread stopped");
}

pub fn start_writing(mut stream: TcpStream, out_recv: Receiver<Packet>) {
    println!("[O]: Writing thread started");

    while let Ok(packet) = out_recv.recv() {
        let _ = packet::write_packet(&mut stream, &packet);
        println!("[O]: Packet sent");
    }

    let _ = stream.shutdown(std::net::Shutdown::Write);
    println!("[O]: Writing thread stopped");
}