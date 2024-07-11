use std::{io, thread};
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, RecvError, TryRecvError};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utp::utp_packet::{HEADER_SIZE, UtpHeader, UtpPacket};
use crate::utp::utp_socket::UtpSocket;
use crate::utp::utp_stream::UtpStream;
use crate::utp::utp_type::UtpType;

pub struct Incoming<'a> {
    listener: &'a UtpListener,
}

pub struct UtpListener {
    pub socket: UdpSocket,
    streams: HashMap<u16, UtpSocket>,
    receiver: Receiver<(UtpPacket, SocketAddr)>
    //incoming_buffer: HashMap<u16, Arc<Mutex<Vec<UtpPacket>>>>
}

impl UtpListener {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        //let socket = ;
        //socket.set_nonblocking(true)?;
        let (tx, rx) = channel();

        let _self = Self {
            socket: UdpSocket::bind(addr)?,
            streams: HashMap::new(),
            receiver: rx
            //new_connections: HashMap::new()
            //incoming_buffer: HashMap::new()
        };

        let socket = _self.socket.try_clone()?;

        //let sender = tx.clone();

        thread::spawn(move || {
            let mut buf = [0u8; 65535];

            while true {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                let packet = UtpPacket::from_bytes(&buf[..size]);

                match packet.header._type {
                    UtpType::Data => {
                        println!("DATA");
                    },
                    UtpType::Fin => {
                        println!("FIN");
                    },
                    UtpType::State => {
                        println!("STATE"); //SHOULDNT OCCUR
                    },
                    UtpType::Reset => {
                        println!("RESET");
                    },
                    UtpType::Syn => {
                        println!("SYN {}", src_addr.to_string());
                        tx.send((packet, src_addr)).unwrap();
                    }
                }
            }
        });

        Ok(_self)
        /*
        UdpSocket::bind(addr).map(|socket| Self {
            socket
        })
        */
        /*
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

                        streams.lock().unwrap().get_mut(&packet.header.connection_id).unwrap().lock().unwrap().extend(packet.payload.unwrap());

                        println!("{}", streams.lock().unwrap().get(&packet.header.connection_id).unwrap().lock().unwrap().len());


                        let packet = UtpPacket::new(UtpType::Data, packet.header.connection_id, 1, packet.header.seq_nr, None);

                        //Self::send(&socket, src_addr, UtpType::Data, packet.header.connection_id, 1, packet.header.seq_nr, Vec::new());
                        socket.send_to(packet.to_bytes().as_slice(), src_addr).unwrap();

                    },
                    UtpType::Fin => {
                        println!("FIN");
                    },
                    UtpType::State => {
                        println!("STATE"); //SHOULDNT OCCUR
                    },
                    UtpType::Reset => {
                        println!("RESET");
                    },
                    UtpType::Syn => {
                        println!("SYN {}", src_addr.to_string());
                        sender.send((packet, src_addr)).unwrap();
                    }
                }
            }
        });

        Ok(_self)
        */
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn incoming(&self) -> Incoming<'_> {
        Incoming {
            listener: self
        }
    }

    fn recv(&mut self) {
        let mut buf = [0; 1500]; //CHANGE / CORRECT
        let (size, src_addr) = self.socket.recv_from(&mut buf).unwrap();
        let packet = UtpPacket::from_bytes(&buf[..size]);

        let conn_id = packet.header.conn_id.clone();
        if !self.streams.contains_key(&conn_id) {
            println!("NEW STREAM");
            //self.streams.insert(conn_id, packet);
        }

        /*
        if self.incoming_buffer.contains_key(&packet.header.connection_id) {
            self.incoming_buffer.get_mut(&packet.header.connection_id).unwrap().lock().unwrap().push(packet);
            return;
        }

        //self.incoming_buffer.lock().unwrap().insert(src_addr, Vec::new());
        let conn_id = packet.header.connection_id.clone();
        let mut packets = Vec::new();
        packets.push(packet);
        self.incoming_buffer.insert(conn_id, Arc::new(Mutex::new(packets)));
        */
    }

    /*
    pub fn send(socket: &UdpSocket, src_addr: SocketAddr, _type: UtpType, connection_id: u16, seq_nr: u16, ack_nr: u16, payload: Vec<u8>) {
        let packet = UtpPacket {
            header: UtpHeader {
                _type,
                version: 1,
                extension: 0,
                connection_id,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32,
                timestamp_diff: 0,
                wnd_size: 0,
                seq_nr, //Server Ack Number
                ack_nr,
            },
            payload,
        }.to_bytes();

        socket.send_to(packet.as_slice(), src_addr).unwrap();
    }
    */
}

impl<'a> Iterator for Incoming<'a> {

    type Item = io::Result<UtpStream>;

    fn next(&mut self) -> Option<Self::Item> {
        //self.listener.recv();

        /*
        for (conn_id, packets) in &self.listener.incoming_buffer {
            if self.listener.
        }
        */

        match self.listener.receiver.recv() {
            Ok((packet, src_addr)) => {
                println!("PACKET RECEIVED");

                let stream = UtpStream::connect(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))).unwrap();
                Some(
                    Ok(
                        stream
                    )
                )
            }
            Err(e) => Some(Err(io::Error::new(io::ErrorKind::Other, e)))
        }



    }
}
        /*
        for packet in &self.listener.incoming_buffer {
            match packet.header._type {
                UtpType::Syn => {
                    //let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))).unwrap();
                    //let send_packet = UtpPacket::new(UtpType::State, packet.header.connection_id, 1, packet.header.seq_nr+1, None);

                    //socket.send_to(send_packet.to_bytes().as_slice(), addr).unwrap();

                    /.*
                    return Some(Ok(UtpStream {
                        socket,//: self.listener.socket.clone(),
                        remote_addr: SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)),
                        recv_conn_id: packet.header.connection_id+1,
                        send_conn_id: packet.header.connection_id,
                        seq_nr: 1,
                        ack_nr: 0,
                        buffer: Vec::new(),
                    }))*./
                    //tosdo!()
                }
                _ => {
                    continue;
                }
            }
        }
        */

        //Some(Err(Self::Error("")))

        /*
        let mut buf = [0; 1500];

        match self.listener.socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let packet = UtpPacket::from_bytes(&buf[..size]);

                match packet.header._type {
                    UtpType::Syn => {
                        let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))).unwrap();
                        let send_packet = UtpPacket::new(UtpType::State, packet.header.connection_id, 1, packet.header.seq_nr+1, None);

                        socket.send_to(send_packet.to_bytes().as_slice(), addr).unwrap();

                        Some(Ok(UtpStream {
                            socket,
                            remote_addr: addr,
                            recv_conn_id: packet.header.connection_id+1,
                            send_conn_id: packet.header.connection_id,
                            seq_nr: 1,
                            ack_nr: 0,
                            buffer: Vec::new(),
                        }))
                    }
                    _ => {
                        Some(Err(io::Error::new(ErrorKind::Other, "Invalid type")))
                    }
                }
            }
            Err(e) => Some(Err(e))
        }
        */


        /*
        match self.listener.receiver.recv() {
            Ok((packet, src_addr)) => {
                println!("CONNECTION");

                let buffer = Arc::new(Mutex::new(Vec::new()));
                self.listener.streams.lock().unwrap().insert(packet.header.connection_id+1, buffer.clone());

                let stream = UtpStream {
                    socket: self.listener.socket.try_clone().unwrap(),
                    remote_addr: src_addr,
                    conn_id: packet.header.connection_id+1,
                    seq_nr: 1,
                    ack_nr: 0,
                    buffer
                };

                let packet = UtpPacket::new(UtpType::State, packet.header.connection_id, stream.ack_nr, packet.header.seq_nr+1, None);

                self.listener.socket.send_to(packet.to_bytes().as_slice(), src_addr).unwrap();

                Some(Ok(stream))
            }
            Err(e) => Some(Err(io::Error::new(io::ErrorKind::Other, e)))
        }
        */

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
