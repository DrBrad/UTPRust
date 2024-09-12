use std::io;
use crate::utp::cid::ConnectionPeer;

pub trait UdpSocket<P: ConnectionPeer>: Send + Sync {
    /// Attempts to send data on the socket to a given address.
    /// Note that this should return nearly immediately, rather than awaiting something internally.
    fn send_to(&mut self, buf: &[u8], target: &P) -> io::Result<usize>;
    /// Attempts to receive a single datagram on the socket.
    fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, P)>;
}
