use std::{io, thread};
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, RecvError, TryRecvError};
use crate::utp::utp_packet::UtpPacket;
use crate::utp::utp_stream::UtpStream;
use crate::utp::utp_type::UtpType;

pub struct Incoming<'a> {
    listener: &'a UtpListener,
}

pub struct UtpListener {
    socket: Arc<UdpSocket>,
    streams: Arc<Mutex<HashMap<u16, Arc<Mutex<Vec<u8>>>>>>,
    receiver: Receiver<(UtpPacket, SocketAddr)>
}

impl UtpListener {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = Arc::new(UdpSocket::bind(addr)?);
        let streams = Arc::new(Mutex::new(HashMap::new()));
        let (tx, rx) = channel();
        let sender = tx.clone();

        let _self = Self {
            socket: socket.clone(),
            streams: streams.clone(),
            receiver: rx
        };

        let receiver_handle = thread::spawn(move || {
            let mut buf = [0u8; 65535];

            while true {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                let packet = UtpPacket::from_bytes(&buf[..size]);

                match(packet.header._type) {
                    UtpType::Data => {
                        //REJECT IF ISN'T KNOWN IE - BLACK HOLE...
                        if !streams.lock().unwrap().contains_key(&packet.header.connection_id) {
                            continue;
                        }

                        println!("DATA MESSAGE");

                        let mut buffer = streams.lock().unwrap().get_mut(&packet.header.connection_id).unwrap();
                        //buffer.lock().unwrap().push(packet.payload);

                    },
                    UtpType::Fin => {
                        println!("FIN");
                    },
                    UtpType::State => {
                        println!("STATE");
                    },
                    UtpType::Reset => {
                        println!("RESET");
                    },
                    UtpType::Syn => {
                        println!("SYN {}", src_addr.to_string());
                        sender.send((packet, src_addr)).unwrap();
                    }
                }

                //sender.send((buf[..size].to_vec(), src_addr)).unwrap();
            }
        });

        Ok(_self)
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }


    pub fn incoming(&self) -> Incoming<'_> {
        Incoming {
            listener: self
        }
    }
}

impl<'a> Iterator for Incoming<'a> {

type Item = io::Result<UtpStream>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.listener.receiver.recv() {
            Ok((packet, src_addr)) => {
                println!("CONNECTION");

                let buffer = Arc::new(Mutex::new(Vec::new()));
                self.listener.streams.lock().unwrap().insert(packet.header.connection_id, buffer.clone());

                let stream = UtpStream {
                    socket: self.listener.socket.try_clone().unwrap(),
                    remote_addr: src_addr,
                    conn_id: packet.header.connection_id+1,
                    seq_nr: packet.header.seq_nr,
                    ack_nr: packet.header.ack_nr,
                    buffer
                };

                Some(Ok(stream))
            }
            Err(e) => Some(Err(io::Error::new(io::ErrorKind::Other, e)))
        }

        /*
        let mut buf = [0; 1500];

        match self.listener.socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let packet = UtpPacket::from_bytes(&buf[..size]);

                //ADD STREAM TO MAP<connection_id, stream>

                Some(Ok(UtpStream {
                    socket: self.listener.socket.try_clone().unwrap(),
                    remote_addr: addr,
                    conn_id: packet.header.connection_id+1,
                    seq_nr: packet.header.seq_nr,
                    ack_nr: packet.header.ack_nr
                }))
            }
            Err(e) => Some(Err(e))
        }
        */
    }
}

/*
pub fn try_clone(&self) -> io::Result<Self> {
    todo!()
    //self.0.duplicate().map(TcpListener)
}

/.*
pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
    todo!()
    //self.0.accept().map(|(a, b)| (TcpStream(a), b))
}
*/
