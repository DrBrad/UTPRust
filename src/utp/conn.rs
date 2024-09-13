use crate::utp::packet::{UtpPacket, UtpPacketType};

pub struct Connection {

}

impl Connection {

    pub fn new() -> Self {
        Self {

        }
    }

    pub fn on_packet(&self, packet: UtpPacket) {
        match packet.packet_type() {
            UtpPacketType::Syn => self.on_syn(packet.seq_num()),
            UtpPacketType::State => self.on_state(packet.seq_num(), packet.ack_num()),
            UtpPacketType::Data => self.on_data(packet.seq_num(), packet.payload()),
            UtpPacketType::Fin => self.on_fin(packet.seq_num(), packet.payload()),
            UtpPacketType::Reset => self.on_reset(),
        }
    }

    fn on_syn(&self, seq_num: u16) {

    }

    fn on_state(&self, seq_num: u16, ack_num: u16) {

    }

    fn on_data(&self, seq_num: u16, payload: &Vec<u8>) {

    }

    fn on_fin(&self, seq_num: u16, payload: &Vec<u8>) {

    }

    fn on_reset(&self) {

    }
}
