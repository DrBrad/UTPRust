use std::cmp::min;
use std::io;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use crate::utils::random;
use crate::utp::utp_socket::UtpSocket;

pub struct UtpStream {
    socket: UtpSocket
    /*
    socket: UdpSocket,
    remote_addr: SocketAddr,
    recv_conn_id: u16,
    send_conn_id: u16,
    seq_nr: u16,
    ack_nr: u16,
    */
    //buffer: Vec<u8>//Arc<Mutex<Vec<u8>>>
}

impl UtpStream {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        UtpSocket::bind(addr).map(|s| Self {
            socket: s
        })
    }

    pub fn connect<A: ToSocketAddrs>(dst: A) -> io::Result<Self> {
        UtpSocket::connect(dst).map(|s| Self {
            socket: s
        })
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.socket.close()
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn set_max_retransmission_retries(&mut self, n: u32) {
        //self.socket.max_retransmission_retries = n;
        todo!()
    }
    /*
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0)))?;
        let remote_addr = addr.to_socket_addrs()?.next().unwrap();

        let conn_id = random::gen();

        Ok(Self {
            socket,
            remote_addr,
            recv_conn_id: conn_id,
            send_conn_id: conn_id+1,
            seq_nr: 1,
            ack_nr: 0,
            //buffer: Vec::new()//Arc::new(Mutex::new(Vec::new()))
        })
    }

    pub fn send()

    pub fn recv(&self) {

    }
    */
}

impl Read for UtpStream {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.recv_from(buf).map(|(read, _src)| read)
    }
}

impl Write for UtpStream {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.socket.send_to(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.socket.flush()
    }
}

impl From<UtpSocket> for UtpStream {
    fn from(socket: UtpSocket) -> Self {
        Self {
            socket
        }
    }
}
