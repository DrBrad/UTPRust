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
    pub(crate) reply_micro: u32
    /*
    pub(crate) rtt: f64,
    pub(crate) rtt_var: f64,
    pub(crate) timeout: Duration,
    pub(crate) last_packet_sent: Instant,
    pub(crate) last_packet: Option<UtpPacket>,
    pub(crate) packet_rtt: Duration, // RTT for the last packet
    pub(crate) max_window: usize*/
    //pub(crate) buffer: Vec<u8>
}
/*
impl Default for UtpSocket {

    fn default() -> Self {
        let conn_id = random::gen();
        Self {
            socket: (),
            remote_addr: None,
            recv_conn_id: 0,
            send_conn_id: 0,
            seq_nr: 0,
            ack_nr: 0,
            receiver: None,
            state: UtpState::SynSent,
            max_window: 0,
            cur_window: 0,
            wnd_size: 0,
            reply_micro: 0,
        }
    }
}
*/

impl UtpSocket {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let conn_id = random::gen();
        UdpSocket::bind(addr).map(|socket| Self {
            socket,
            remote_addr: None,
            recv_conn_id: conn_id,
            send_conn_id: conn_id+1,
            //last_ack_nr: 0,
            seq_nr: 0,
            ack_nr: 0,
            receiver: None,
            state: Waiting,

            max_window: 1500,
            cur_window: 0,
            wnd_size: 0,
            reply_micro: 0

            //..Default::default()
            /*
            rtt: 0.0,
            rtt_var: 0.0,
            timeout: Duration::from_millis(1000),
            last_packet_sent: Instant::now(),
            last_packet: None,
            packet_rtt: Duration::from_millis(1000),
            max_window: 1500*/
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
            //last_ack_nr: 0,
            seq_nr: 1,
            ack_nr: 0,
            receiver: None,
            state: SynSent,

            max_window: 1500,
            cur_window: 0,
            wnd_size: 0,
            reply_micro: 0
        });

        let packet = UtpPacket::new(UtpType::Syn,
                                  conn_id,
                                  1,
                                  0,
                                  self_.as_ref().unwrap().cur_window,
                                  0,
                                  None);

        self_.as_ref().unwrap().socket.send_to(packet.to_bytes().as_slice(), self_.as_ref().unwrap().remote_addr.unwrap()).unwrap();
        println!("SND: {}", packet.to_string());

        //self_.as_mut().unwrap().recv(&mut [0u8; 65535])?;

        let mut buf = [0; 1500];
        let size = self_.as_ref().unwrap().socket.recv(&mut buf)?;

        let packet = UtpPacket::from_bytes(&mut buf[..size]);
        println!("RCV: {}", packet.to_string());

        match packet.header._type {
            UtpType::Ack => {
                self_.as_mut().unwrap().state = Connected;
                self_.as_mut().unwrap().ack_nr = packet.header.seq_nr;
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::Other, "Unhandled packet type"))
            }
        }

        self_
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn send(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.state != Connected {
            return Err(io::Error::new(io::ErrorKind::Other, "Socket not connected"));
        }

        let seq_nr = self.seq_nr+1;
        let packet = UtpPacket::new(UtpType::Data,
                                    self.send_conn_id,
                                    seq_nr,
                                    self.ack_nr,
                                    self.cur_window,
                                    self.reply_micro,
                                    Some(buf.to_vec()));


        if self.cur_window + seq_nr as u32 <= min(self.max_window, self.wnd_size) {
            self.seq_nr = seq_nr;
            self.cur_window += packet.to_bytes().len() as u32;
            println!("SND: {}", packet.to_string());

            return self.socket.send_to(packet.to_bytes().as_slice(), self.remote_addr.unwrap());
        }

        Err(io::Error::new(io::ErrorKind::Other, "Current window is full"))
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

                if self.state != Connected {
                    return Err(io::Error::new(io::ErrorKind::Other, "Socket not connected"));
                }

                let packet = UtpPacket::from_bytes(&mut buf[..size]);
                println!("RCV: {}", packet.to_string());

                packet
            }
        };

        self.ack_nr = packet.header.seq_nr;
        self.wnd_size = packet.header.wnd_size;
        self.reply_micro = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32-packet.header.timestamp;

        match packet.header._type {
            UtpType::Data => {
                //NEW CONNECTION...
                let pack = UtpPacket::new(UtpType::Ack,
                                          self.send_conn_id,
                                          self.seq_nr,
                                          self.ack_nr,
                                          self.cur_window,
                                          self.reply_micro,
                                          None);
                self.socket.send_to(pack.to_bytes().as_slice(), self.remote_addr.unwrap()).unwrap();
                println!("SND: {}", packet.to_string());
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
        let packet = UtpPacket::new(UtpType::Fin,
                                    self.send_conn_id,
                                    self.seq_nr,
                                    self.ack_nr,
                                    self.cur_window,
                                    self.reply_micro,
                                    None);
        println!("SND: {}", packet.to_string());

        self.socket.send_to(packet.to_bytes().as_slice(), self.remote_addr.unwrap()).unwrap();
        Ok(())
    }
}
