use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, RwLock};

pub struct UtpSocket<P> {
    conns: Arc<RwLock<HashMap<ConnectionId<P>, ConnChannel>>>,
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
where
    P: ConnectionPeer + 'static,
{

    pub fn with_socket<S>(mut socket: S) -> Self
    //where
    //    S: UdpSocket<P> + 'static,
    {
        todo!()
    }

    pub fn accept(&self)/* -> io::Result<UtpStream>*/ {

    }

    pub fn connect(&self) {

    }

    fn generate_cid(&self) {

    }
}



