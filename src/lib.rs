pub mod utp;
pub mod utils;

//INITIATER              ENDPOINT
//      SYN ----------->
//          <----------- STATE
//     DATA ============ DATA

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{Ipv4Addr, SocketAddr, TcpListener};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::utp::utp_listener::UtpListener;

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
        let listener = UtpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).expect("Failed to bind");

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    /*
                    println!("[{:?}] [ConnID Sending: {}] [ConnID Recv: {}] [SeqNr. {}] [AckNr: {}]",
                             packet.header._type,
                             packet.header.connection_id,
                             utp_socket.conn_id,
                             packet.header.seq_nr,
                             packet.header.ack_nr);
                    */
                    println!("[Socket: {}] [ConnID: {}] [SeqNr. {}] [AckNr. {}]",
                             stream.remote_addr,
                             stream.conn_id, //SENDING BACK IS -1 I BELIEVE (INIT WILL GEN RANDOM CONN ID SEND = NUM+1, RECV = NUM)
                             stream.seq_nr,
                             stream.ack_nr);


                    let mut buf = [0; 1500];

                    match stream.read(&mut buf) {
                        Ok(0) => {
                            break;
                        }
                        Ok(n) => {
                            println!("{}", String::from_utf8_lossy(&buf[..n]));
                        }
                        Err(e) => {
                            eprintln!("Failed to read from stream: {}", e);
                            break;
                        }
                    }



                    //stream.write("hello world".as_bytes());
                    //stream.flush().unwrap();

                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }


        /*
        let mut utp_socket = UtpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).expect("Failed to bind.");
        let (packet, src) = utp_socket.receive();

        println!("[{:?}] [ConnID Sending: {}] [ConnID Recv: {}] [SeqNr. {}] [AckNr: {}]",
                packet.header._type,
                packet.header.connection_id,
                utp_socket.conn_id,
                packet.header.seq_nr,
                packet.header.ack_nr);*/


        //println!("{}", String::from_utf8_lossy(packet.payload.as_slice()));



        loop {

        }
        //assert_eq!(result, 4);
    }
}
