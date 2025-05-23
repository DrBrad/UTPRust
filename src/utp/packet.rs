use crate::utp::inter::utp_types::UtpTypes;

pub const HEADER_SIZE: usize = 20;

/*
0       4       8               16              24              32
+-------+-------+---------------+---------------+---------------+
| type  | ver   | extension     | connection_id                 |
+-------+-------+---------------+---------------+---------------+
| timestamp_microseconds                                        |
+---------------+---------------+---------------+---------------+
| timestamp_difference_microseconds                             |
+---------------+---------------+---------------+---------------+
| wnd_size                                                      |
+---------------+---------------+---------------+---------------+
| seq_nr                        | ack_nr                        |
+---------------+---------------+---------------+---------------+
*/
#[derive(Debug)]
pub struct UtpPacket {
    pub(crate) _type: UtpTypes,
    pub(crate) version: u8,
    pub(crate) extension: u8,
    pub(crate) conn_id: u16,
    pub(crate) timestamp: u32,
    pub(crate) timestamp_diff: u32,
    pub(crate) wnd_size: u32,
    pub(crate) seq_nr: u16,
    pub(crate) ack_nr: u16,
    //pub(crate) src_addr: SocketAddr,
    pub(crate) payload: Option<Vec<u8>>
}


impl UtpPacket {

    pub fn new(_type: UtpTypes, conn_id: u16, seq_nr: u16, ack_nr: u16, wnd_size: u32, timestamp_diff: u32, payload: Option<Vec<u8>>) -> Self {
        Self {
            _type,
            version: 1,
            extension: 0,
            conn_id,
            timestamp: 0,//SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32,
            timestamp_diff,
            wnd_size,
            seq_nr,
            ack_nr,
            //src_addr,
            payload
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes;// = vec![0u8; HEADER_SIZE + self.payload.as_ref().unwrap().len()];

        match self.payload.as_ref() {
            Some(payload) => {
                bytes = vec![0u8; HEADER_SIZE + payload.len()];
                bytes[HEADER_SIZE..].copy_from_slice(&payload);
            }
            None => {
                bytes = vec![0u8; HEADER_SIZE];
            }
        }

        //let mut bytes = vec![0u8; HEADER_SIZE + self.payload.as_ref().unwrap().len()];
        bytes[0] = (self._type.get_code() << 4) | (self.version & 0x0F);
        bytes[1] = self.extension;
        bytes[2..4].copy_from_slice(&self.conn_id.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.timestamp.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.timestamp_diff.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.wnd_size.to_be_bytes());
        bytes[16..18].copy_from_slice(&self.seq_nr.to_be_bytes());
        bytes[18..20].copy_from_slice(&self.ack_nr.to_be_bytes());
        //bytes[HEADER_SIZE..].copy_from_slice(&self.as_ref().payload.unwrap());
        bytes
    }

    pub fn from_bytes(/*src_addr: SocketAddr, */bytes: &[u8]) -> Self {
        Self {
            _type: UtpTypes::from_code(&(bytes[0] >> 4)).expect("Failed to find packet type"),
            version: bytes[0] & 0x0F,
            extension: bytes[1],
            conn_id: u16::from_be_bytes([bytes[2], bytes[3]]),
            timestamp: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            timestamp_diff: u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            wnd_size: u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            seq_nr: u16::from_be_bytes([bytes[16], bytes[17]]),
            ack_nr: u16::from_be_bytes([bytes[18], bytes[19]]),
            //src_addr,
            payload: Some(bytes[HEADER_SIZE..].to_vec())
        }
    }

    pub fn to_string(&self) -> String {
        format!("[{:?}] [ConnID: {}] [SeqNr. {}] [AckNr: {}] [WndSize: {}] [Diff: {}]",
                self._type,
                self.conn_id,
                self.seq_nr,
                self.ack_nr,
                self.wnd_size,
                self.timestamp_diff)
    }
}
