#[derive(Debug)]
pub enum UtpState {
    SynSent,
    SynRecv,
    Connected,
    Disconnected
}
