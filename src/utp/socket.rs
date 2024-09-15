use std::collections::HashMap;
use std::{io, thread};
use std::fmt::{Debug, Display};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{mpsc, Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utp::event::{SocketEvent, StreamEvent};
use crate::utp::packet::{UtpPacket, UtpPacketType};
use crate::utp::stream::UtpStream;

const MAX_UDP_PAYLOAD_SIZE: usize = u16::MAX as usize;

pub struct UtpSocket {
    conns: Arc<RwLock<HashMap<u16, Sender<StreamEvent>>>>, //swap this with Connection...
    incoming: Receiver<UtpStream>
}

impl UtpSocket {

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self::with_socket(socket))
    }

    pub fn with_socket(mut socket: UdpSocket) -> Self {
        let (incoming_tx, incoming_rx) = channel();
        let (pool_tx, pool_rx) = channel();

        thread::spawn(move || {
            let mut buf = [0; MAX_UDP_PAYLOAD_SIZE];

            loop {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                let packet = match UtpPacket::decode(&buf[..size]) {
                    Ok(packet) => packet,
                    Err(..) => {
                        //tracing::warn!(?src, "unable to decode uTP packet");
                        continue;
                    }
                };

                pool_tx.send((packet, src_addr)).unwrap();
            }
        });

        let conns = Arc::new(RwLock::new(HashMap::new()));

        let self_ = Self {
            conns: Arc::clone(&conns),
            incoming: incoming_rx
        };

        thread::spawn(move || {
            loop {
                match pool_rx.try_recv() {
                    Ok((packet, src_addr)) => {
                        //Self::on_receive();
                        let conn = conns.read().unwrap().get(&packet.conn_id()).cloned();
                        match conn {
                            Some(conn) => {

                                //EVENT STUFF HERE...



                                //conn.send(StreamEvent::Incoming(packet)).unwrap();

                            }
                            None => {
                                if packet.packet_type() == UtpPacketType::Syn {
                                    let cid = packet.conn_id();

                                    println!("{:?}", packet);

                                    let (tx, rx) = channel();
                                    //conns.write().unwrap().insert(cid, tx);
                                    conns.write().unwrap().insert(cid, tx);

                                    incoming_tx.send(UtpStream::new(cid, rx)).unwrap();
                                }
                            }
                        }


                    }
                    Err(TryRecvError::Empty) => {
                    }
                    Err(TryRecvError::Disconnected) => break
                }
            }
        });

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

