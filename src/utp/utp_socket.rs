use std::cell::RefCell;
use std::cmp::{min, PartialEq};
use std::collections::HashMap;
use std::io;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use crate::utils::random;
use crate::utp::utp_listener::UtpListener;
use crate::utp::utp_packet::{HEADER_SIZE, UtpPacket};
use crate::utp::utp_state::UtpState;
use crate::utp::utp_state::UtpState::{Closed, Connected, Waiting, SynRecv, SynSent};
use crate::utp::utp_type::UtpType;

/*
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
*/
// Maximum age of base delay sample (60 seconds)
//const MAX_BASE_DELAY_AGE: Delay = Delay(60_000_000);

pub struct UtpSocket {
    pub(crate) socket: UdpSocket,
    pub(crate) remote_addr: Option<SocketAddr>,
    pub(crate) recv_conn_id: u16,
    pub(crate) send_conn_id: u16,
    //pub(crate) last_ack_nr: u16,
    pub(crate) seq_nr: u16,
    pub(crate) ack_nr: u16, //DO WE NEED CLIENT ACK AS WELL?
    pub(crate) receiver: Option<Receiver<UtpPacket>>,
    pub(crate) state: UtpState,

    pub(crate) max_window: u32,
    pub(crate) cur_window: u32,
    pub(crate) wnd_size: u32,
    pub(crate) reply_micro: u32,

    pub(crate) receive_buffer: Vec<u8>,
    pub(crate) transmit_buffer: Vec<UtpPacket>
}

impl UtpSocket {

    //pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let conn_id = random::gen();
        let mut self_ = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))).map(|socket| Self {
            socket,
            remote_addr: Some(addr),
            recv_conn_id: conn_id,
            send_conn_id: conn_id+1,
            //last_ack_nr: 0,
            seq_nr: 1,
            ack_nr: 0,
            receiver: None,
            state: SynSent,

            max_window: 1500,
            cur_window: 0,
            wnd_size: 0,
            reply_micro: 0,

            receive_buffer: Vec::new(),
            transmit_buffer: Vec::new()
        });

        self_
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn send(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.state != Connected {
            return Err(io::Error::new(io::ErrorKind::Other, "Socket not connected"));
        }

        if (self.cur_window + buf.len() as u32) < min(self.max_window, self.wnd_size) {
            //RATHER THAN GIVING ERROR WE SHOULD HOLD IN TRANSMIT BUFFER
            return Err(io::Error::new(io::ErrorKind::Other, "Current window is full"));
        }

        let seq_nr = self.seq_nr+1;
        self.cur_window += buf.len() as u32;

        let packet = UtpPacket::new(UtpType::Data,
                                    self.send_conn_id,
                                    seq_nr,
                                    self.ack_nr,
                                    self.cur_window,
                                    self.reply_micro,
                                    Some(buf.to_vec()));
        self.transmit_buffer.push(packet);

        self.socket.send_to(packet.to_bytes().as_slice(), self.remote_addr.unwrap())
    }

    pub fn send_to(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> io::Result<usize> {



        Err(io::Error::new(io::ErrorKind::Other, "Current window is full"))
    }

    pub fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        todo!()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        todo!()
    }

    pub fn close(&mut self) -> io::Result<()> {
        Ok(())
    }
}
