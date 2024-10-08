use std::cmp::min;
use std::{io, thread};
use std::io::{ErrorKind, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::os::linux::raw::stat;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicU16, AtomicU32, Ordering};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::Receiver;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::random;
use crate::utp::utp_packet::UtpPacket;
//use crate::utp::utp_socket::UtpSocket;
use crate::utp::utp_state::UtpState;
use crate::utp::utp_state::UtpState::{Connected, SynSent};
use crate::utp::utp_type::UtpType;

pub struct UtpInner {
    ack_nr: u16,
    state: UtpState,
    wnd_size: u32,
    reply_micro: u32,
    receive_buffer: Vec<u8>
}

pub struct UtpStream {
    pub socket: UdpSocket,
    pub(crate) remote_addr: Option<SocketAddr>,
    pub(crate) recv_conn_id: u16,
    pub(crate) send_conn_id: u16,
    //pub(crate) last_ack_nr: u16,
    pub(crate) seq_nr: u16,
    //pub(crate) ack_nr: Arc<AtomicU16>,//u16,
    pub(crate) receiver: Option<Receiver<UtpPacket>>,
    //pub(crate) state: Arc<RwLock<UtpState>>,

    pub(crate) max_window: u32, //MAX WINDOW SIZE
    pub(crate) cur_window: u32, //BYTES IN FLIGHT - NOT ACKED
    //pub(crate) wnd_size: Arc<AtomicU32>, //u32, //WINDOW SIZE CLIENT IS ADVERTISING
    //pub(crate) reply_micro: Arc<AtomicU32>, //u32,

    //pub(crate) receive_buffer: Arc<Mutex<Vec<u8>>>,
    pub(crate) transmit_buffer: Vec<UtpPacket>,
    pub(crate) inner: Arc<Mutex<UtpInner>>
}
/*
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
*/
impl UtpStream {

    //pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
    pub fn connect(addr: SocketAddr) -> io::Result<Self> {
        let conn_id = random::gen();
        /*
        let receive_buffer = Arc::new(Mutex::new(Vec::new()));
        let ack_nr = Arc::new(AtomicU16::new(0));
        let wnd_size = Arc::new(AtomicU32::new(0));
        let reply_micro = Arc::new(AtomicU32::new(0));
        let state = Arc::new(RwLock::new(SynSent));
        */

        let inner = Arc::new(Mutex::new(UtpInner {
            ack_nr: 0,
            state: SynSent,
            wnd_size: 0,
            reply_micro: 0,
            receive_buffer: Vec::new(),
        }));

        let mut self_ = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0))).map(|socket| Self {
            socket,
            remote_addr: Some(addr),
            recv_conn_id: conn_id,
            send_conn_id: conn_id+1,
            //last_ack_nr: 0,
            seq_nr: 1,
            //ack_nr: ack_nr.clone(),
            receiver: None,
            //state: state.clone(),

            max_window: 1500,
            cur_window: 0,
            //wnd_size: wnd_size.clone(),
            //reply_micro: reply_micro.clone(),

            //receive_buffer: receive_buffer.clone(),
            transmit_buffer: Vec::new(),
            inner: inner.clone()
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

        //UNFORTUNATLY WE MUST BLOCK HERE...
        let packet = UtpPacket::from_bytes(&mut buf[..size]);
        println!("RCV: {}", packet.to_string());

        match packet.header._type {
            UtpType::Ack => {
                //*state.write().unwrap() = Connected;
                //self_.as_mut().unwrap().state = Connected;
                //self_.as_mut().unwrap().ack_nr.store(packet.header.seq_nr, Relaxed);
                inner.lock().unwrap().state = Connected;
                inner.lock().unwrap().ack_nr = packet.header.seq_nr;
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::Other, "Unhandled packet type"))
            }
        }


        //NEW THREAD for receiver
        thread::spawn(move || {
            let mut buf = [0u8; 65535];

            loop {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                let packet = UtpPacket::from_bytes(&buf[..size]);
                //println!("PACKET 3");

                inner.lock().unwrap().ack_nr = packet.header.seq_nr; //DONT SET THIS UNLESS WE SEND ACK
                inner.lock().unwrap().wnd_size = packet.header.wnd_size;
                inner.lock().unwrap().reply_micro = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32-packet.header.timestamp;

                //ack_nr.store(packet.header.seq_nr, Relaxed);
                //wnd_size.store(packet.header.wnd_size, Relaxed);
                //reply_micro.store(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32-packet.header.timestamp, Relaxed);

                match packet.header._type {
                    UtpType::Ack => {
                        //*state.write().unwrap() = Connected;
                    }
                    UtpType::Data => {
                        inner.lock().unwrap().receive_buffer.append(&mut packet.payload.unwrap());
                    }
                    _ => {
                        println!("PACKET");
                    }
                }
            }
        });//receiver(socket, receive_buffer, ack_nr, wnd_size, reply_micro));

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
        let n = min(buf.len(), self.inner.lock().unwrap().receive_buffer.len());
        buf[..n].copy_from_slice(&self.inner.lock().unwrap().receive_buffer[..n]);
        self.inner.lock().unwrap().receive_buffer.drain(..n);
        Ok(n)
    }
}

impl Write for UtpStream {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.inner.lock().unwrap().state != Connected {
            return Err(io::Error::new(io::ErrorKind::Other, "Socket not connected"));
        }

        let inner = self.inner.lock().unwrap();

        let seq_nr = self.seq_nr+1;
        let packet = UtpPacket::new(UtpType::Data,
                                    self.send_conn_id,
                                    seq_nr,
                                    inner.ack_nr,
                                    self.cur_window,
                                    inner.reply_micro,
                                    Some(buf.to_vec()));

        self.socket.send_to(packet.to_bytes().as_slice(), self.remote_addr.unwrap())

        //self.socket.send_to(buf)
        //todo!()
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
