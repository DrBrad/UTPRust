pub mod utp;
pub mod utils;

//INITIATER              ENDPOINT
//      SYN ----------->
//          <----------- STATE
//     DATA ============ DATA

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::utp::utp_listener::UtpListener;
    use crate::utp::utp_stream::UtpStream;

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


        /*
        let server = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).unwrap();
        let client = UdpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7071))).unwrap();

        let server_clone = server.try_clone().unwrap();
        client.send_to("asdasd".as_bytes(), server_clone.local_addr().unwrap()).unwrap();

        thread::sleep(Duration::from_secs(2));

        thread::spawn(move || {
            let mut buf = [0; 1500];
            let (n, src_addr) = server.recv_from(&mut buf).unwrap();
            println!("{}", String::from_utf8_lossy(&buf[..n]));
        });




        loop {}
        */


        let mut listener = UtpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).expect("Failed to bind");
        //let stream = listener.incoming().next().unwrap();
        for socket in listener.incoming() {
            match socket {
                Ok(mut socket) => {
                    println!("NEW SOCKET");
                    //let mut buf = [0; 1500];
                    //let n = socket.recv(&mut buf).unwrap();
                    //println!("Packet: {}", String::from_utf8_lossy(&buf[..n]));

                    //socket.send("TEST hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();


                    //socket.send("POOP hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();

                    let mut buf = [0; 1500];
                    socket.recv(&mut buf);

                    println!("{}", String::from_utf8_lossy(&buf));



                    socket.send("TEST hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();

                    //socket.send("POOP hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();



                    let mut buf = [0; 1500];
                    socket.recv(&mut buf);

                    println!("{}", String::from_utf8_lossy(&buf));


                    //socket.send("NUMBER 2".as_bytes()).unwrap();


                    //socket.close().unwrap();

                },
                Err(e) => {
                    //println!("{}", e);
                }
            }
        }

        //let mut buf = [0; 1500];
        //let n = listener.socket.recv(&mut buf).expect("FAILED_TO_GET_RECEIVE");
        //println!("END");


        /*
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    /.*
                    println!("[{:?}] [ConnID Sending: {}] [ConnID Recv: {}] [SeqNr. {}] [AckNr: {}]",
                             packet.header._type,
                             packet.header.connection_id,
                             utp_socket.conn_id,
                             packet.header.seq_nr,
                             packet.header.ack_nr);
                    *./
                    println!("[Socket: {}] [RecvConnID: {}] [SendConnID: {}] [SeqNr. {}] [AckNr. {}]",
                             stream.remote_addr,
                             stream.recv_conn_id,
                             stream.send_conn_id,
                             stream.seq_nr,
                             stream.ack_nr);


                    /.*
                    let mut buf = [0; 1500];

                    loop {
                    match stream.read(&mut buf) {
                        Ok(0) => {
                            //println!("Empty");
                            //break;
                        }
                        Ok(n) => {
                            println!("{}", String::from_utf8_lossy(&buf[..n]));
                        }
                        Err(e) => {
                            eprintln!("Failed to read from stream: {}", e);
                            break;
                        }
                    }
                    }
                    *./



                    //stream.write("hello world".as_bytes());
                    //stream.flush().unwrap();

                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        */


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
