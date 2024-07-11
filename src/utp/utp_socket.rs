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
    pub(crate) ack_nr: u16,
    pub(crate) receiver: Option<Receiver<UtpPacket>>,
    //pub(crate) buffer: Vec<u8>
    //pub(crate) incoming_packets: Rc<RefCell<Vec<UtpPacket>>>
}

impl UtpSocket {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let conn_id = random::gen();
        UdpSocket::bind(addr).map(|socket| Self {
            socket,
            remote_addr: None,
            recv_conn_id: conn_id+1,
            send_conn_id: conn_id,
            seq_nr: 1,
            ack_nr: 0,
            receiver: None,
            //buffer: Vec::new()
            //incoming_packets: Rc::new(RefCell::new(Vec::new()))
        })
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let conn_id = random::gen();
        UdpSocket::bind(addr).map(|socket| Self {
            socket,
            remote_addr: None,
            recv_conn_id: conn_id+1,
            send_conn_id: conn_id,
            seq_nr: 1,
            ack_nr: 0,
            receiver: None,
            //buffer: Vec::new()
            //incoming_packets: Rc::new(RefCell::new(Vec::new()))
        })
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn send(&self) {
        todo!()
    }

    pub fn send_to(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let packet = match &self.receiver {
            Some(receiver) => {
                receiver.recv().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            }
            None => {
                let mut buf = [0; 1500];
                let size = self.socket.recv(&mut buf)?;
                UtpPacket::from_bytes(&mut buf[..size])
            }
        };

        self.seq_nr += 1;
        self.socket.send_to(UtpPacket::new(UtpType::State, packet.header.conn_id, self.seq_nr, packet.header.seq_nr+1, None).to_bytes().as_slice(), self.remote_addr.unwrap()).unwrap();

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
        self.socket.send_to(UtpPacket::new(UtpType::Fin, self.send_conn_id, self.seq_nr, self.ack_nr, None).to_bytes().as_slice(), self.remote_addr.unwrap()).unwrap();
        Ok(())
    }
}
