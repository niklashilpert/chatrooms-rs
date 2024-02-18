use std::{io::BufReader, net::TcpStream, sync::mpsc::{Receiver, Sender}};

use crate::packet::{self, Packet, PacketWithReturn};


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