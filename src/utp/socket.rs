use std::collections::HashMap;
use std::{io, thread};
use std::fmt::{Debug, Display};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, RwLock};
use crate::utp::cid::ConnectionId;
use crate::utp::packet::UtpPacket;
use crate::utp::stream::UtpStream;

const MAX_UDP_PAYLOAD_SIZE: usize = u16::MAX as usize;

pub struct UtpSocket<P> {
    //conns: Arc<RwLock<HashMap<ConnectionId<P>, ConnChannel>>>,
    //accepts: UnboundedSender<Accept<P>>,
    //accepts_with_cid: UnboundedSender<(Accept<P>, ConnectionId<P>)>,
    //socket_events: UnboundedSender<SocketEvent<P>>,
}

impl UtpSocket<SocketAddr> {
    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self::with_socket(socket))
    }
}

impl<P> UtpSocket<P>
//where
//    P: ConnectionPeer + 'static,
{

    pub fn with_socket<S>(mut socket: S) -> Self
    //where
    //    S: UdpSocket<P> + 'static,
    {

        thread::spawn(move || {
            let mut buf = [0; MAX_UDP_PAYLOAD_SIZE];

            loop {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                let packet = UtpPacket::decode(&buf[..size])?;
                println!("{packet}")



            }
        });


        todo!()
    }

    pub fn accept(&self) -> io::Result<UtpStream> {
        todo!()
    }

    pub fn connect(&self) -> io::Result<UtpStream> {
        todo!()
    }

    fn generate_cid(&self) -> ConnectionId<P> {
        todo!()
    }
}
/*
impl<P> Drop for UtpSocket<P> {
    fn drop(&mut self) {
        for conn in self.conns.read().unwrap().values() {
            let _ = conn.send(StreamEvent::Shutdown);
        }
    }
}
*/


