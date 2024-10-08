use std::{io, thread};
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender, TryRecvError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use crate::utp::utp_packet::{HEADER_SIZE, UtpHeader, UtpPacket};
//use crate::utp::utp_socket::UtpSocket;
use crate::utp::utp_state::UtpState::SynRecv;
use crate::utp::utp_stream::UtpStream;
use crate::utp::utp_type::UtpType;

pub struct UtpListener {
    pub socket: UdpSocket,
    channels: Arc<Mutex<HashMap<u16, Sender<UtpPacket>>>>,
    //syn_queue: Vec<UtpPacket>
    receiver: Receiver<(UtpPacket, SocketAddr)>
    //incoming_buffer: HashMap<u16, Arc<Mutex<Vec<UtpPacket>>>>
}

impl UtpListener {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        //socket.set_nonblocking(true)?;
        let (tx, rx) = channel();

        let _self = Self {
            socket,//: UdpSocket::bind(addr)?,
            channels: Arc::new(Mutex::new(HashMap::new())),
            //syn_queue: Vec::new()
            receiver: rx
            //new_connections: HashMap::new()
            //incoming_buffer: HashMap::new()
        };

        //Ok(_self)

        let socket = _self.socket.try_clone()?;
        let channels = _self.channels.clone();

        //let sender = tx.clone();

        thread::spawn(move || {
            let mut buf = [0u8; 65535];

            while true {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                let packet = UtpPacket::from_bytes(&buf[..size]);

                match packet.header._type {
                    UtpType::Syn => {
                        tx.send((packet, src_addr)).unwrap();
                    }
                    _ => {
                        let conn_id = packet.header.conn_id;

                        if !channels.lock().unwrap().contains_key(&conn_id) {
                            continue;
                        }

                        channels.lock().unwrap().get_mut(&conn_id).unwrap().send(packet);
                    }
                }
            }
        });

        Ok(_self)
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    /*
    pub fn incoming(&mut self) -> Incoming<'_> {
        Incoming {
            listener: self
        }
    }*/
}
/*
pub struct Incoming<'a> {
    listener: &'a mut UtpListener,
}

impl Iterator for Incoming<'_> {

    type Item = io::Result<UtpSocket>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.listener.receiver.recv() {
            Ok((packet, src_addr)) => {
                println!("RCV: {}", packet.to_string());
                let reply_micro = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32-packet.header.timestamp;

                let send = UtpPacket::new(UtpType::Ack,
                                          packet.header.conn_id,
                                          0,
                                          packet.header.seq_nr,
                                          1500,
                                          reply_micro,
                                          None);
                println!("SND: {}", send.to_string());

                self.listener.socket.send_to(send.to_bytes().as_slice(), src_addr).unwrap();
                let (tx, rx) = channel();

                let socket = UtpSocket {
                    socket: self.listener.socket.try_clone().unwrap(),
                    remote_addr: Some(src_addr),
                    recv_conn_id: packet.header.conn_id+1,
                    send_conn_id: packet.header.conn_id,
                    //last_ack_nr: 0,
                    seq_nr: 0, //RANDOM
                    ack_nr: packet.header.seq_nr,
                    //seq_nr: 1,
                    //ack_nr: packet.header.seq_nr,
                    receiver: Some(rx),
                    state: SynRecv,

                    max_window: 1500,
                    cur_window: 0,
                    wnd_size: packet.header.wnd_size,
                    reply_micro
                    /*
                    rtt: 0.0,
                    rtt_var: 0.0,
                    timeout: Duration::from_millis(1000),
                    last_packet_sent: Instant::now(),
                    last_packet: None,
                    packet_rtt: Duration::from_millis(1000),
                    max_window: 1500*/
                    //incoming_packets: Rc::new(RefCell::new(Vec::new()))//Arc::new(Mutex::new(Vec::new()))
                };

                self.listener.channels.lock().unwrap().insert(packet.header.conn_id+1, tx);

                Some(Ok(socket))
            }
            Err(e) => Some(Err(io::Error::new(io::ErrorKind::Other, e)))
        }
    }
}
*/
