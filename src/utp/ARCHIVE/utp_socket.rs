use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use crate::utils::random;
use crate::utp::utp_packet::UtpPacket;

//https://www.bittorrent.org/beps/bep_0029.html
//We will be using UTP for speed and multi connectivity (won't require port forwarding)

pub struct UtpSocket {
    socket: UdpSocket,
    pub(crate) conn_id: u16,
    seq_nr: u16,
    ack_nr: u16,
}

impl UtpSocket {

    pub fn bind(addr: SocketAddr) -> Result<Self, Error> {
        let socket = UdpSocket::bind(addr)?;
        Ok(Self {
            socket,
            conn_id: random::gen(),
            seq_nr: 1,
            ack_nr: 0,
        })
    }

    pub fn send_to(&mut self, dest: &SocketAddr, data: &[u8]) {
        let packet = UtpPacket::new(data.to_vec(), self.conn_id, self.seq_nr, self.ack_nr);
        let bytes = packet.to_bytes();
        self.socket.send_to(&bytes, dest).expect("Failed to send packet");
        self.seq_nr += 1;
    }

    pub fn send_with_retransmission(&mut self, dest: &SocketAddr, data: &[u8]) {
        let mut retries = 0;
        let max_retries = 5;
        while retries < max_retries {
            self.send_to(dest, data);
            let (ack_packet, _) = self.receive();
            if ack_packet.header.ack_nr == self.seq_nr - 1 {
                return;
            }
            retries += 1;
        }
        eprintln!("Failed to send packet after {} retries", max_retries);
    }

    pub fn receive(&mut self) -> (UtpPacket, SocketAddr) {
        let mut buf = [0u8; 1500];
        let (amt, src) = self.socket.recv_from(&mut buf).expect("Failed to receive packet");
        let packet = UtpPacket::from_bytes(&buf[..amt]);
        self.ack_nr = packet.header.seq_nr;
        (packet, src)
    }
}


/*
    server: Option<Arc<UdpSocket>>

    pub fn start(&mut self, port: u16) {
        self.server = Some(Arc::new(UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port))).expect("Failed to bind socket")));
        let (tx, rx) = channel();
        let sender = tx.clone();
        let server = Arc::clone(self.server.as_ref().unwrap());

        let receiver_handle = thread::spawn(move || {
            let mut buf = [0u8; 65535];

            while true {
                let (size, src_addr) = {
                    server.recv_from(&mut buf).expect("Failed to receive message")
                };

                sender.send((buf[..size].to_vec(), src_addr)).unwrap();

                /*
                let data = &buf[..size];

                let bytes = data.as_ptr();
                let len = data.len();
                forget(data);

                unsafe {
                    sender.send((from_raw_parts(bytes, len), src_addr)).expect("Failed to send packet to handler");
                }
                */
            }
        });


        let handler_handle = thread::spawn(move || {
            while true {
                match rx.try_recv() {
                    Ok((data, src_addr)) => {
                        //Self::on_receive(kademlia.as_mut(), data.as_slice(), src_addr);
                        println!("RECEIVED");
                    }
                    Err(TryRecvError::Empty) => {
                    }
                    Err(TryRecvError::Disconnected) => break
                }
            }
        });
    }
*/
