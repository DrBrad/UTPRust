use std::cmp::min;
use std::{io, thread};
use std::io::{ErrorKind, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use crate::utils::random;
use crate::utp::utp_packet::UtpPacket;
//use crate::utp::utp_socket::UtpSocket;
use crate::utp::utp_state::UtpState;
use crate::utp::utp_state::UtpState::SynSent;
use crate::utp::utp_type::UtpType;

pub struct UtpStream {
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

impl UtpStream {

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

        let socket = self_?.socket;

        //NEW THREAD for receiver
        thread::spawn(|_| {
            let mut buf = [0u8; 65535];

            while true {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                let packet = UtpPacket::from_bytes(&buf[..size]);

                match packet.header._type {
                    UtpType::Ack => {

                    }
                    _ => {
                    }
                }
            }
        });



        self_
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn close(&mut self) -> io::Result<()> {
        Ok(())
    }
}



impl Read for UtpStream {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        //self.socket.recv_from(buf).map(|(read, _src)| read)
        todo!()
    }
}

impl Write for UtpStream {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        //self.socket.send_to(buf)
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        //self.socket.flush()
        todo!()
    }
}
/*
impl From<UtpSocket> for UtpStream {

    fn from(socket: UtpSocket) -> Self {
        Self {
            socket
        }
    }
}
*/
