use std::{io, thread};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use crate::utp::stream::UtpStream;

pub struct UtpSocket {
    socket: UdpSocket
}

impl UtpSocket {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        thread::spawn(move || {

        });

        Ok(Self {
            socket
        })
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<UtpStream> {
        //self.socket.send_to("".as_bytes(), addr)?;

        Ok(UtpStream {

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
    listener: &'a mut UtpSocket,
}

impl Iterator for Incoming<'_> {

    type Item = io::Result<UtpStream>;

    fn next(&mut self) -> Option<Self::Item> {

        None
    }
}
