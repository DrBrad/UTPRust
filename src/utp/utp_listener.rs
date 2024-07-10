use std::io;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use crate::utp::utp_packet::UtpPacket;
use crate::utp::utp_stream::UtpStream;

pub struct Incoming<'a> {
    listener: &'a UtpListener,
}

pub struct UtpListener {
    socket: UdpSocket,
    streams: Vec<u16>
}

impl UtpListener {

    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;

        Ok(Self {
            socket,
            streams: Vec::new()
        })
    }

    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        self.socket.local_addr()
    }


    pub fn incoming(&self) -> Incoming<'_> {
        Incoming {
            listener: self
        }
    }
}

impl<'a> Iterator for Incoming<'a> {

    type Item = io::Result<UtpStream>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 1500];

        match self.listener.socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                let packet = UtpPacket::from_bytes(&buf[..size]);

                //ADD STREAM TO MAP<connection_id, stream>

                Some(Ok(UtpStream {
                    socket: self.listener.socket.try_clone().unwrap(),
                    remote_addr: addr,
                    conn_id: packet.header.connection_id+1,
                    seq_nr: packet.header.seq_nr,
                    ack_nr: packet.header.ack_nr
                }))
            }
            Err(e) => Some(Err(e))
        }
    }
}

/*
pub fn try_clone(&self) -> io::Result<Self> {
    todo!()
    //self.0.duplicate().map(TcpListener)
}

/.*
pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
    todo!()
    //self.0.accept().map(|(a, b)| (TcpStream(a), b))
}
*/
