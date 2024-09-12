
const PACKET_HEADER_LEN: usize = 20;
const SELECTIVE_ACK_BITS: usize = 32;
const EXTENSION_TYPE_LEN: usize = 1;
const EXTENSION_LEN_LEN: usize = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
struct UtpPacketHeader {
    packet_type: UtpType,
    version: Version,
    extension: Extension,
    conn_id: u16,
    ts_micros: u32,
    ts_diff_micros: u32,
    window_size: u32,
    seq_num: u16,
    ack_num: u16,
}

impl UtpPacketHeader {

}

pub struct UtpPacket {

}

impl UtpPacket {

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UtpType {
    Data,
    Fin,
    Ack, //Also known as State
    Reset,
    Syn
}

impl UtpType {

    pub fn from_value(value: &u8) -> Result<Self, ()> {
        for _type in [Self::Data, Self::Fin, Self::Ack, Self::Reset, Self::Syn] {
            if _type.value().eq(value) {
                return Ok(_type);
            }
        }

        Err(())
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::Data => 0,
            Self::Fin => 1,
            Self::Ack => 2,
            Self::Reset => 3,
            Self::Syn => 4,
        }
    }
}
