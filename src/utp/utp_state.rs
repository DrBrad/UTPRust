#[derive(Debug, PartialEq, Eq)]
pub enum UtpState {
    SynSent,
    SynRecv,
    Connected,
    Waiting,
    Closed
}
