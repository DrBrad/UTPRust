use std::cmp::min;
use std::{io, thread};
use std::io::{ErrorKind, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU16, AtomicU32, Ordering};
use std::sync::mpsc::Receiver;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::random;
use crate::utp::utp_packet::UtpPacket;
//use crate::utp::utp_socket::UtpSocket;
use crate::utp::utp_state::UtpState;
use crate::utp::utp_state::UtpState::{Connected, SynSent};
use crate::utp::utp_type::UtpType;

pub struct UtpStream {
    pub socket: UdpSocket,
    pub(crate) remote_addr: Option<SocketAddr>,
    pub(crate) recv_conn_id: u16,
    pub(crate) send_conn_id: u16,
    //pub(crate) last_ack_nr: u16,
    pub(crate) seq_nr: u16,
    pub(crate) ack_nr: Arc<AtomicU16>,//u16,
    pub(crate) receiver: Option<Receiver<UtpPacket>>,
    pub(crate) state: UtpState,

    pub(crate) max_window: u32, //MAX WINDOW SIZE
    pub(crate) cur_window: u32, //BYTES IN FLIGHT - NOT ACKED
    pub(crate) wnd_size: Arc<AtomicU32>, //u32, //WINDOW SIZE CLIENT IS ADVERTISING
    pub(crate) reply_micro: Arc<AtomicU32>, //u32,

    pub(crate) receive_buffer: Arc<Mutex<Vec<u8>>>,
    pub(crate) transmit_buffer: Vec<UtpPacket>
}

fn receiver(socket: UdpSocket, buffer: Arc<Mutex<Vec<u8>>>) {
    let mut buf = [0u8; 65535];

    loop {
        let (size, src_addr) = {
            socket.recv_from(&mut buf).expect("Failed to receive message")
        };

        let packet = UtpPacket::from_bytes(&buf[..size]);


        //self.ack_nr = packet.header.seq_nr;
        //self.wnd_size = packet.header.wnd_size;
        //self.reply_micro = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32-packet.header.timestamp;

        match packet.header._type {
            UtpType::Data => {
                buffer.lock().as_mut().unwrap().append(&mut packet.payload.unwrap());
            }
            _ => {
                println!("PACKET");
            }
        }
    }
}

impl UtpStream {

    //pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let conn_id = random::gen();
        let receive_buffer = Arc::new(Mutex::new(Vec::new()));
        let ack_nr = Arc::new(AtomicU16::new(0));
        let wnd_size = Arc::new(AtomicU32::new(0));
        let reply_micro = Arc::new(AtomicU32::new(0));

        let mut self_ = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))).map(|socket| Self {
            socket,
            remote_addr: Some(addr),
            recv_conn_id: conn_id,
            send_conn_id: conn_id+1,
            //last_ack_nr: 0,
            seq_nr: 1,
            ack_nr: ack_nr.clone(),
            receiver: None,
            state: SynSent,

            max_window: 1500,
            cur_window: 0,
            wnd_size: wnd_size.clone(),
            reply_micro: reply_micro.clone(),

            receive_buffer: receive_buffer.clone(),
            transmit_buffer: Vec::new()
        });

        let socket = self_.as_ref().unwrap().socket.try_clone()?;

        let packet = UtpPacket::new(UtpType::Syn,
                                    conn_id,
                                    1,
                                    0,
                                    self_.as_ref().unwrap().max_window,
                                    0,
                                    None);

        socket.send_to(packet.to_bytes().as_slice(), self_.as_ref().unwrap().remote_addr.unwrap())?;
        println!("SND: {}", packet.to_string());

        let mut buf = [0; 1500];
        let size = self_.as_ref().unwrap().socket.recv(&mut buf)?;

        let packet = UtpPacket::from_bytes(&mut buf[..size]);
        println!("RCV: {}", packet.to_string());

        match packet.header._type {
            UtpType::Ack => {
                self_.as_mut().unwrap().state = Connected;
                self_.as_mut().unwrap().ack_nr.store(packet.header.seq_nr, Ordering::Relaxed);
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::Other, "Unhandled packet type"))
            }
        }

        //NEW THREAD for receiver
        thread::spawn(move || receiver(socket, receive_buffer));

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
