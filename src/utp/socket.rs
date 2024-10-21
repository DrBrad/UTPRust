use std::collections::HashMap;
use std::{io, thread};
use std::fmt::{Debug, Display};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{mpsc, Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::utp::conn::Connection;
use crate::utp::event::{SocketEvent, StreamEvent};
use crate::utp::packet::{UtpPacket, UtpPacketType};
use crate::utp::stream::UtpStream;

const MAX_UDP_PAYLOAD_SIZE: usize = u16::MAX as usize;
const MAX_AWAITING_CONNECTION_TIMEOUT: Duration = Duration::from_secs(20);

pub struct UtpSocket {
    //conns: Arc<RwLock<HashMap<u16, Sender<StreamEvent>>>>, //swap this with Connection...
    incoming: Receiver<UtpStream>
}

impl UtpSocket {

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self::with_socket(UdpSocket::bind(addr)?))
    }

    pub fn with_socket(mut socket: UdpSocket) -> Self {
        //let conns = Arc::new(RwLock::new(HashMap::new()));

        let (incoming_tx, incoming_rx) = channel();

        //DO WE NEED 2 THREADS...?

        let conn = Connection::new();

        thread::spawn(move || {
            let mut buf = [0; MAX_UDP_PAYLOAD_SIZE];

            loop {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                /*let packet = */match UtpPacket::decode(&buf[..size]) {
                    Ok(packet) => /*packet*/{
                        conn.on_packet(packet);

                    },
                    Err(..) => {
                        eprintln!("Unable to decode uTP packet.");
                        //tracing::warn!(?src, "unable to decode uTP packet");
                        continue;
                    }
                };

                /*
                pool_tx.send((packet, src_addr)).unwrap();
                */
            }
        });



        let self_ = Self {
            //conns: Arc::clone(&conns),
            incoming: incoming_rx
        };

        self_
    }
/*
    fn on_receive(packet: UtpPacket, src_addr: SocketAddr, conns: Arc<RwLock<HashMap<u16, Sender<StreamEvent>>>>) {
        match conns.read().unwrap().get(&packet.conn_id()) {
            Some(conn) => {
                conn.send(StreamEvent::Incoming(packet)).unwrap();

            }
            None => {
                if packet.packet_type() == UtpPacketType::Syn {
                    let cid = packet.conn_id();

                    println!("{:?}", packet);

                    let (tx, rx) = mpsc::channel();
                    conns.write().unwrap().insert(cid, tx);

                    let stream = UtpStream::new(cid, rx);

                }
            }
        }
    }
*/
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

    fn generate_cid(&self) {//-> ConnectionId/*<P>*/ {
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
            Err(e) => None,
        }
    }
}

