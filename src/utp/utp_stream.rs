use std::cmp::min;
use std::io;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use crate::utils::random;

pub struct UtpStream {
    pub(crate) socket: UdpSocket,
    pub(crate) remote_addr: SocketAddr,
    pub(crate) recv_conn_id: u16,
    pub(crate) send_conn_id: u16,
    pub(crate) seq_nr: u16,
    pub(crate) ack_nr: u16,
    pub(crate) buffer: Vec<u8>//Arc<Mutex<Vec<u8>>>
}

impl UtpStream {

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
            buffer: Vec::new()//Arc::new(Mutex::new(Vec::new()))
        })
    }
}

impl Read for UtpStream {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
        /*
        let mut buffer = self.buffer.lock().unwrap();
        let bytes_to_copy = min(buffer.len(), buf.len());

        if bytes_to_copy == 0 {
            return Ok(0);
        }

        buf[..bytes_to_copy].copy_from_slice(&buffer[..bytes_to_copy]);
        buffer.drain(..bytes_to_copy);
        Ok(bytes_to_copy)*/
        //self.buffer.lock().unwrap().get()
        //self.socket.recv(buf)
    }
}

impl Write for UtpStream {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
        //self.socket.send_to(buf, self.remote_addr)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
