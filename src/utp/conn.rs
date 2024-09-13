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
            UtpPacketType::Syn => self.on_syn(),
            UtpPacketType::State => self.on_state(),
            UtpPacketType::Data => self.on_data(),
            UtpPacketType::Fin => self.on_fin(),
            UtpPacketType::Reset => self.on_reset(),
        }
    }

    fn on_syn(&self) {

    }

    fn on_state(&self) {

    }

    fn on_data(&self) {

    }

    fn on_fin(&self) {

    }

    fn on_reset(&self) {

    }
}
