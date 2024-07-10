use std::io;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;
use crate::utils::random;

pub struct UtpSocket {
    socket: UdpSocket
}

impl UtpSocket {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        //take_address(addr).and_then(|a| UdpSocket::bind(a).map(|s| UtpSocket::from_raw_parts(s, a)))
        todo!()
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        todo!()

        /*
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
        */
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

    pub fn recv(&self) {
        todo!()
    }

    pub fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        todo!()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        todo!()
    }

    pub fn close(&mut self) -> io::Result<()> {
        todo!()
    }
}
