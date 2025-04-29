
pub struct Connection {
    window_size: u32,
    seq_num: u16,
    ack_num: u16,
    reply_micro: u32
}

impl Connection {

    pub fn new() -> Self {
        Self {
            window_size: 0,
            seq_num: 0,
            ack_num: 0,
            reply_micro: 0
        }
    }
}
