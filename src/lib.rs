pub mod utp;
pub mod utils;

//INITIATER              ENDPOINT
//      SYN ----------->
//          <----------- STATE
//     DATA ============ DATA

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::utp::utp_listener::UtpListener;
    use crate::utp::utp_socket::UtpSocket;
    use crate::utp::utp_stream::UtpStream;

    #[test]
    fn test() {
        /*
        let mut socket = UtpSocket::connect(SocketAddr::from((IpAddr::from([127, 0, 0, 1]), 7070))).unwrap();

        socket.send("TEST hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();

        println!("[State [{:?}]]", socket.state);

        let mut buf = [0; 1500];
        socket.recv(&mut buf);

        println!("{}", String::from_utf8_lossy(&buf));


        return;
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

                    /*
                    let mut buf = [0; 1500];
                    socket.recv(&mut buf);

                    println!("{}", String::from_utf8_lossy(&buf));
                    */

                    /*
                    RECEIVING...

                    RECEIVE [Syn] [ConnID: 60369] [SeqNr. 1] [AckNr: 0]
                    SEND [Ack] [ConnID: 60369] [SeqNr. 1] [AckNr: 1]

                    RECEIVE [Data] [ConnID: 60370] [SeqNr. 2] [AckNr: 1]
                    SEND [Ack] [ConnID: 60369] [SeqNr. 1] [AckNr: 2]

                    RECEIVE [Data] [ConnID: 60370] [SeqNr. 3] [AckNr: 1]
                    SEND [Ack] [ConnID: 60369] [SeqNr. 1] [AckNr: 3]
                    */

                    /*
                    SENDING

                    RECEIVE [Syn] [ConnID: 6077] [SeqNr. 1] [AckNr: 0]
                    SEND [Ack] [ConnID: 6077] [SeqNr. 1] [AckNr: 1]

                    SEND [Data] [ConnID: 6077] [SeqNr. 2] [AckNr: 1]
                    SEND [Data] [ConnID: 6077] [SeqNr. 3] [AckNr: 1]

                    RECEIVE [Ack] [ConnID: 6078] [SeqNr. 0] [AckNr: 3]
                    */


                    /*
                    let mut buf = [0; 1500];
                    socket.recv(&mut buf);

                    println!("{}", String::from_utf8_lossy(&buf));


                    let mut buf = [0; 1500];
                    socket.recv(&mut buf);

                    println!("{}", String::from_utf8_lossy(&buf));*/

                    /*
                    let mut buf = [0; 1500];
                    socket.recv(&mut buf);

                    println!("{}", String::from_utf8_lossy(&buf));*/




                    //socket.send("POOP hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();



                    let mut buf = [0; 1500];
                    socket.recv(&mut buf);

                    println!("{}", String::from_utf8_lossy(&buf));

                    println!("[State [{:?}]]", socket.state);

                    socket.send("TEST hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();


                    //socket.close().unwrap();

                },
                Err(e) => {
                    //println!("{}", e);
                }
            }
        }

        loop {

        }
        //assert_eq!(result, 4);
    }
}
