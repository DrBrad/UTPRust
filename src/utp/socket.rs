use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, RwLock};

pub struct UtpSocket<P> {
    //conns: Arc<RwLock<HashMap<ConnectionId<P>, ConnChannel>>>,
    //accepts: UnboundedSender<Accept<P>>,
    //accepts_with_cid: UnboundedSender<(Accept<P>, ConnectionId<P>)>,
    //socket_events: UnboundedSender<SocketEvent<P>>,
}

impl UtpSocket<SocketAddr> {

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        let socket = Self::with_socket(socket);
        Ok(socket)
    }
}



