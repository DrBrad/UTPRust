use std::collections::HashMap;
use std::{io, thread};
use std::fmt::{Debug, Display};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{mpsc, Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::utp::packet::{UtpPacket, UtpPacketError};
use crate::utp::stream::UtpStream;

const MAX_UDP_PAYLOAD_SIZE: usize = u16::MAX as usize;
const MAX_AWAITING_CONNECTION_TIMEOUT: Duration = Duration::from_secs(20);

pub struct UtpSocket {
    incoming: Receiver<UtpStream> //maybe pass connection instead...?
}

impl UtpSocket {

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self::with_socket(UdpSocket::bind(addr)?))
    }

    pub fn with_socket(mut socket: UdpSocket) -> Self {
        let (tx, rx) = channel();

        thread::spawn(move || {
            let mut buf = [0; MAX_UDP_PAYLOAD_SIZE];

            loop {
                let (size, src_addr) = socket.recv_from(&mut buf).unwrap();

                match UtpPacket::decode(&buf[..size]) {
                    Ok(packet) => {


                    }
                    Err(_) => {}
                }

                //DECODE PACKET

                //TICK - timeout / retransmits and acks

                sleep(Duration::from_millis(10));
            }
        });

        Self {
            //conns: Arc::clone(&conns),
            incoming: rx
        }
    }

    pub fn incoming(&mut self) -> Incoming<'_> {
        Incoming {
            listener: self
        }
    }

    pub fn connect(&self) -> io::Result<UtpStream> {
        todo!()
    }

    pub fn total_connections(&self) -> usize {
        todo!()
    }

    fn generate_cid(&self) -> u16 {//-> ConnectionId/*<P>*/ {
        todo!()
    }
}

pub struct Incoming<'a> {
    listener: &'a mut UtpSocket,
}

impl Iterator for Incoming<'_> {

    type Item = UtpStream;

    fn next(&mut self) -> Option<Self::Item> {
        match self.listener.incoming.recv() {
            Ok(stream) => Some(stream),
            Err(e) => None
        }
    }
}
