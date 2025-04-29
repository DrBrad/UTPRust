use std::{io, thread};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use crate::utp::utp_stream::UtpStream;

pub struct UtpListener {
    socket: UdpSocket
}

impl UtpListener {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        thread::spawn(move || {

        });

        Ok(Self {
            socket
        })
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }

    pub fn incoming(&mut self) -> Incoming<'_> {
        Incoming {
            listener: self
        }
    }
}

pub struct Incoming<'a> {
    listener: &'a mut UtpListener,
}

impl Iterator for Incoming<'_> {

    type Item = io::Result<UtpStream>;

    fn next(&mut self) -> Option<Self::Item> {

        None
    }
}
