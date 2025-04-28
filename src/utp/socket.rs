use std::io;
use std::net::SocketAddr;
use crate::utp::stream::UtpStream;

pub struct UtpSocket {

}

impl UtpSocket {

    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        Ok(Self {

        })
    }
    /*
    pub fn incoming(&mut self) -> Incoming<'_> {
        Incoming {
            listener: self
        }
    }
    */

    pub fn connect(&self) -> io::Result<UtpStream> {
        todo!()
    }

    pub fn total_connections(&self) -> usize {
        todo!()
    }

    fn generate_cid(&self) -> u16 {
        todo!()
    }
}
