use crate::utp::packet::{UtpPacket, UtpPacketType};

pub struct Connection {
    reply_micro: u32
}

impl Connection {

    pub fn new() -> Self {
        Self {
            reply_micro: 0
        }
    }

    pub fn on_packet(&self, packet: UtpPacket) {
        /*
        let now_micros = crate::time::now_micros();
        self.peer_recv_window = packet.window_size();

        // Cap the diff. If the clock on the remote machine is ahead of the clock on the local
        // machine, then we could end up with large (inaccurate) diffs. Use the max idle timeout as
        // an upper bound on the possible diff. If the diff exceeds the bound, then assume the
        // remote clock is behind the local clock and use a diff of 1s.
        let peer_ts_diff = crate::time::duration_between(packet.ts_micros(), now_micros);
        if peer_ts_diff > self.config.max_idle_timeout {
            self.peer_ts_diff = Duration::from_secs(1);
        } else {
            self.peer_ts_diff = peer_ts_diff;
        }
        */

        match packet.packet_type() {
            UtpPacketType::Syn => self.on_syn(packet.seq_num()),
            UtpPacketType::State => self.on_state(packet.seq_num(), packet.ack_num()),
            UtpPacketType::Data => self.on_data(packet.seq_num(), packet.payload()),
            UtpPacketType::Fin => self.on_fin(packet.seq_num(), packet.payload()),
            UtpPacketType::Reset => self.on_reset(),
        }

        //self.retransmit_lost_packets(now);


    }

    fn on_syn(&self, seq_num: u16) {

    }

    fn on_state(&self, seq_num: u16, ack_num: u16) {
        //ONLY IF CONNECTING...
        //IF INITIATOR
    }

    fn on_data(&self, seq_num: u16, payload: &Vec<u8>) {

    }

    fn on_fin(&self, seq_num: u16, payload: &Vec<u8>) {

    }

    fn on_reset(&self) {

    }
}
