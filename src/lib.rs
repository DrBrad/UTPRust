pub mod utp;
pub mod utils;

//INITIATER              ENDPOINT
//      SYN ----------->
//          <----------- STATE
//     DATA ============ DATA

#[cfg(test)]
mod tests {

    use std::net::{Ipv4Addr, SocketAddr};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::utp::utp_socket::UtpSocket;

    #[test]
    fn test() {
        /*
        thread::spawn(|| {
            let mut utp_socket = UtpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).expect("Failed to bind.");

            let (packet, src) = utp_socket.receive();
            println!("{} {} {}", packet.header.connection_id, src.to_string(), String::from_utf8_lossy(packet.payload.as_slice()));

            utp_socket.send_with_retransmission(&src, "Hello World".as_bytes());
        });

        sleep(Duration::from_secs(2));

        let mut utp_socket = UtpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7072))).expect("Failed to bind.");

        utp_socket.send_with_retransmission(&SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070)), "asdasd".as_bytes());

        let (packet, src) = utp_socket.receive();
        println!("{} {} {}", packet.header.connection_id, src.to_string(), String::from_utf8_lossy(packet.payload.as_slice()));
        */
        let mut utp_socket = UtpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).expect("Failed to bind.");
        let (packet, src) = utp_socket.receive();

        println!("[{:?}] [ConnID Sending: {}] [ConnID Recv: {}] [SeqNr. {}] [AckNr: {}]",
                packet.header._type,
                packet.header.connection_id,
                utp_socket.conn_id,
                packet.header.seq_nr,
                packet.header.ack_nr);


        //println!("{}", String::from_utf8_lossy(packet.payload.as_slice()));



        loop {

        }
        //assert_eq!(result, 4);
    }
}
