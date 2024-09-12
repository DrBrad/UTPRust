use std::collections::HashMap;
use std::{io, thread};
use std::fmt::{Debug, Display};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{mpsc, Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use crate::utp::cid::ConnectionId;
use crate::utp::event::{SocketEvent, StreamEvent};
use crate::utp::packet::UtpPacket;
use crate::utp::stream::UtpStream;

const MAX_UDP_PAYLOAD_SIZE: usize = u16::MAX as usize;

pub struct UtpSocket {//<P> {
    //conns: Arc<RwLock<HashMap<ConnectionId<P>, ConnChannel>>>,
    //accepts: Sender<Accept<P>>,
    //accepts_with_cid: Sender<(Accept<P>, ConnectionId<P>)>,
    //socket_events: Sender<SocketEvent<P>>,
}
/*
impl UtpSocket {//<SocketAddr> {

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self::with_socket(socket))
    }
}
*/
impl/*<P>*/ UtpSocket//<P>
//where
//    P: ConnectionPeer + 'static,
{

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self::with_socket(socket))
    }

    pub fn with_socket(mut socket: UdpSocket) -> Self
    //where
    //    S: UdpSocket<P> + 'static,
    {
        //let conns = Arc::new(RwLock::new(HashMap::new()));
        //let (socket_event_tx, mut socket_event_rx) = mpsc::channel();
        //let (accepts_tx, mut accepts_rx) = mpsc::channel();
        //let (accepts_with_cid_tx, mut accepts_with_cid_rx) = mpsc::channel();

        let self_ = Self {
            //conns: Arc::clone(&conns),
            //accepts: accepts_tx,
            //accepts_with_cid: accepts_with_cid_tx,
            //socket_events: socket_event_tx.clone(),
        };

        thread::spawn(move || {
            let mut buf = [0; MAX_UDP_PAYLOAD_SIZE];

            loop {
                let (size, src_addr) = {
                    socket.recv_from(&mut buf).expect("Failed to receive message")
                };

                let packet = match UtpPacket::decode(&buf[..size]) {
                    Ok(pkt) => pkt,
                    Err(..) => {
                        //tracing::warn!(?src, "unable to decode uTP packet");
                        continue;
                    }
                };


            }
        });

        self_
    }

    pub fn accept(&self) -> io::Result<UtpStream> {
        todo!()
    }

    pub fn connect(&self) -> io::Result<UtpStream> {
        todo!()
    }

    fn generate_cid(&self) {//-> ConnectionId/*<P>*/ {
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


//type ConnChannel = Sender<StreamEvent>;

//struct Accept<P> {
    //stream: oneshot::Sender<io::Result<UtpStream<P>>>,
    //config: ConnectionConfig,
//}


