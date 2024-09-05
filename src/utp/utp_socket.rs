use std::cell::RefCell;
use std::cmp::min;
use std::collections::HashMap;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use crate::utils::random;
use crate::utp::utp_listener::UtpListener;
use crate::utp::utp_packet::{HEADER_SIZE, UtpPacket};
use crate::utp::utp_state::UtpState;
use crate::utp::utp_state::UtpState::{Closed, Connected, Waiting, SynRecv, SynSent};
use crate::utp::utp_type::UtpType;

const BUF_SIZE: usize = 1500;
const GAIN: f64 = 1.0;
const ALLOWED_INCREASE: u32 = 1;
const TARGET: f64 = 100_000.0; // 100 milliseconds
const MSS: u32 = 1400;
const MIN_CWND: u32 = 2;
const INIT_CWND: u32 = 2;
const INITIAL_CONGESTION_TIMEOUT: u64 = 1000; // one second
const MIN_CONGESTION_TIMEOUT: u64 = 500; // 500 ms
const MAX_CONGESTION_TIMEOUT: u64 = 60_000; // one minute
const BASE_HISTORY: usize = 10; // base delays history size
const MAX_SYN_RETRIES: u32 = 5; // maximum connection retries
const MAX_RETRANSMISSION_RETRIES: u32 = 5; // maximum retransmission retries
const WINDOW_SIZE: u32 = 1024 * 1024; // local receive window size

// Maximum time (in microseconds) to wait for incoming packets when the send window is full
const PRE_SEND_TIMEOUT: u32 = 500_000;

// Maximum age of base delay sample (60 seconds)
//const MAX_BASE_DELAY_AGE: Delay = Delay(60_000_000);

pub struct UtpSocket {
    pub(crate) socket: UdpSocket,
    pub(crate) remote_addr: Option<SocketAddr>,
    pub(crate) recv_conn_id: u16,
    pub(crate) send_conn_id: u16,
    pub(crate) seq_nr: u16,
    pub(crate) ack_nr: u16, //DO WE NEED CLIENT ACK AS WELL?
    pub(crate) receiver: Option<Receiver<UtpPacket>>,
    pub(crate) state: UtpState
    //pub(crate) buffer: Vec<u8>
}

impl UtpSocket {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let conn_id = random::gen();
        UdpSocket::bind(addr).map(|socket| Self {
            socket,
            remote_addr: None,
            recv_conn_id: conn_id,
            send_conn_id: conn_id+1,
            seq_nr: 0,
            ack_nr: 0,
            receiver: None,
            state: Waiting
            //buffer: Vec::new()
            //incoming_packets: Rc::new(RefCell::new(Vec::new()))
        })
    }

    //pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let conn_id = random::gen();
        let mut self_ = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))).map(|socket| Self {
            socket,
            remote_addr: Some(addr),
            recv_conn_id: conn_id,
            send_conn_id: conn_id+1,
            seq_nr: 1,
            ack_nr: 0,
            receiver: None,
            state: SynSent
            //buffer: Vec::new()
            //incoming_packets: Rc::new(RefCell::new(Vec::new()))
        });

        let send = UtpPacket::new(UtpType::Syn, conn_id, 1, 0, None);
        println!("SEND [{:?}] [ConnID: {}] [SeqNr. {}] [AckNr: {}]",
                 send.header._type,
                 send.header.conn_id,
                 send.header.seq_nr,
                 send.header.ack_nr);

        self_.as_ref().unwrap().socket.send_to(send.to_bytes().as_slice(), self_.as_ref().unwrap().remote_addr.unwrap()).unwrap();

        //println!("{:?}", self_.as_ref().unwrap().remote_addr.unwrap());

        let mut buf = [0u8; 65535];

        let (size, src_addr) = {
            self_.as_ref().unwrap().socket.recv_from(&mut buf).expect("Failed to receive message")
        };

        let packet = UtpPacket::from_bytes(&buf[..size]);

        println!("RECEIVE [{:?}] [ConnID: {}] [SeqNr. {}] [AckNr: {}]",
                 packet.header._type,
                 packet.header.conn_id,
                 packet.header.seq_nr,
                 packet.header.ack_nr);

        match packet.header._type {
            UtpType::Ack => {
                self_.as_mut().unwrap().state = Connected;
                //self_.as_mut().unwrap().seq_nr = 0;
                self_.as_mut().unwrap().ack_nr = packet.header.seq_nr;

                self_
            }
            _ => {
                Err(io::Error::new(io::ErrorKind::Other, "Unhandled packet type"))
            }
        }
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn send(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.state {
            Connected => {},
            _ => {
                return Err(io::Error::new(io::ErrorKind::Other, "Socket not connected"))
            }
        };

        self.seq_nr += 1;
        let packet = UtpPacket::new(UtpType::Data, self.send_conn_id, self.seq_nr, self.ack_nr, Some(buf.to_vec()));

        println!("SEND [{:?}] [ConnID: {}] [SeqNr. {}] [AckNr: {}]",
                 packet.header._type,
                 packet.header.conn_id,
                 packet.header.seq_nr,
                 packet.header.ack_nr);

        self.socket.send_to(packet.to_bytes().as_slice(), self.remote_addr.unwrap())
    }

    pub fn send_to(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let packet = match &self.receiver {
            Some(receiver) => {
                let packet = receiver.recv().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                match self.state {
                    SynRecv => {
                        self.ack_nr = packet.header.seq_nr;
                        self.state = Connected;
                    }
                    Waiting | Closed => {
                        return Err(io::Error::new(io::ErrorKind::Other, "Socket not connected"));
                    }
                    _ => {}
                };

                packet
            }
            None => {
                let mut buf = [0; 1500];
                let size = self.socket.recv(&mut buf)?;

                match self.state {
                    Connected => {},
                    _ => {
                        return Err(io::Error::new(io::ErrorKind::Other, "Socket not connected"));
                    }
                };

                UtpPacket::from_bytes(&mut buf[..size])
            }
        };

        self.ack_nr = packet.header.seq_nr;

        match packet.header._type {
            UtpType::Data => {
                //NEW CONNECTION...
                let pack = UtpPacket::new(UtpType::Ack, self.send_conn_id, self.seq_nr, self.ack_nr, None);
                self.socket.send_to(pack.to_bytes().as_slice(), self.remote_addr.unwrap()).unwrap();

                println!("SEND [{:?}] [ConnID: {}] [SeqNr. {}] [AckNr: {}]",
                         pack.header._type,
                         pack.header.conn_id,
                         pack.header.seq_nr,
                         pack.header.ack_nr);
            },
            UtpType::Fin => {
                self.state = Closed;
                return Err(io::Error::new(io::ErrorKind::Other, "Socket closed"))
            },
            _ => {
                return self.recv(buf);
            }
        }

        match packet.payload {
            Some(data) => {
                let len = min(buf.len(), data.len());
                buf[..len].copy_from_slice(&data[..len]);
                Ok(len)
            }
            None => Err(io::Error::new(io::ErrorKind::Other, "No data"))
        }
    }

    pub fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        todo!()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        todo!()
    }

    pub fn close(&mut self) -> io::Result<()> {
        let packet = UtpPacket::new(UtpType::Fin, self.send_conn_id, self.seq_nr, self.ack_nr, None);

        println!("[{:?}] [ConnID: {}] [SeqNr. {}] [AckNr: {}]",
                 packet.header._type,
                 packet.header.conn_id,
                 packet.header.seq_nr,
                 packet.header.ack_nr);

        self.socket.send_to(packet.to_bytes().as_slice(), self.remote_addr.unwrap()).unwrap();
        Ok(())
    }
}
