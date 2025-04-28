use crate::utp::packet::UtpPacket;

#[derive(Clone, Debug)]
pub enum StreamEvent {
    Incoming(UtpPacket),
    Shutdown,
}

#[derive(Clone, Debug)]
pub enum SocketEvent<P> {
    Outgoing((UtpPacket, P)),
    Shutdown(P),
}
