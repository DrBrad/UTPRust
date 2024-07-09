use std::time::{SystemTime, UNIX_EPOCH};

const HEADER_SIZE: usize = 20;

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
pub struct UtpHeader {
    pub(crate) type_version: u8,
    pub(crate) extension: u8,
    pub(crate) connection_id: u16,
    pub(crate) timestamp: u32,
    pub(crate) timestamp_diff: u32,
    pub(crate) wnd_size: u32,
    pub(crate) seq_nr: u16,
    pub(crate) ack_nr: u16,
}

#[derive(Debug)]
pub struct UtpPacket {
    pub(crate) header: UtpHeader,
    pub(crate) payload: Vec<u8>,
}


impl UtpPacket {

    pub fn new(payload: Vec<u8>, conn_id: u16, seq_nr: u16, ack_nr: u16) -> Self {
        Self {
            header: UtpHeader {
                type_version: 1, // Set appropriate type and version
                extension: 0,
                connection_id: conn_id,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u32,
                timestamp_diff: 0,
                wnd_size: 0,
                seq_nr,
                ack_nr,
            },
            payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; HEADER_SIZE + self.payload.len()];
        bytes[0] = self.header.type_version;
        bytes[1] = self.header.extension;
        bytes[2..4].copy_from_slice(&self.header.connection_id.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.header.timestamp.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.header.timestamp_diff.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.header.wnd_size.to_be_bytes());
        bytes[16..18].copy_from_slice(&self.header.seq_nr.to_be_bytes());
        bytes[18..20].copy_from_slice(&self.header.ack_nr.to_be_bytes());
        bytes[HEADER_SIZE..].copy_from_slice(&self.payload);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let header = UtpHeader {
            type_version: bytes[0],
            extension: bytes[1],
            connection_id: u16::from_be_bytes([bytes[2], bytes[3]]),
            timestamp: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            timestamp_diff: u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            wnd_size: u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            seq_nr: u16::from_be_bytes([bytes[16], bytes[17]]),
            ack_nr: u16::from_be_bytes([bytes[18], bytes[19]]),
        };
        let payload = bytes[HEADER_SIZE..].to_vec();
        Self {
            header,
            payload
        }
    }
}
