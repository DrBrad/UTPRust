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
    use crate::utp::socket::UtpSocket;
    use crate::utp::stream::UtpStream;

    #[test]
    fn test() {

        let mut socket = UtpSocket::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).unwrap();

        for mut stream in socket.incoming() {
            println!("INCOMING");
            //stream.write("asdasdasd".as_bytes()).unwrap();

        }

        //close stream




        loop {

        }





        //let mut stream = UtpStream::connect(SocketAddr::from((IpAddr::from([127, 0, 0, 1]), 7070))).unwrap();

        //stream.write("asdasdasd".as_bytes()).unwrap();
        //stream.flush().unwrap();




        /*
        //let mut socket = UtpSocket::connect(SocketAddr::from((IpAddr::from([127, 0, 0, 1]), 7070))).unwrap();

        socket.send("TEST hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();

        println!("[State [{:?}]]", socket.state);

        let mut buf = [0; 1500];
        socket.recv(&mut buf);

        println!("{}", String::from_utf8_lossy(&buf));


        return;
        */








        /*
        TcpStream::connect()


        let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).expect("Failed to bind");
        for socket in listener.incoming() {
            match socket {
                Ok(mut stream) => {
                    stream.write("asdasd".as_bytes());
                    stream.flush().unwrap();
                }
                Err(_) => {

                }
            }
        }
        */




        /*

        let mut listener = UtpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, 7070))).expect("Failed to bind");
        //let stream = listener.incoming().next().unwrap();
        for socket in listener.incoming() {
            match socket {
                Ok(mut socket) => {
                    println!("NEW SOCKET");

                    let mut buf = [0; 1500];
                    socket.recv(&mut buf);

                    println!("{}", String::from_utf8_lossy(&buf));

                    println!("[State [{:?}]]", socket.state);

                    socket.send("TEST hello world asdjasidjaisjdijasidjaisdjiasjd".as_bytes()).unwrap();



                    socket.close().unwrap();

                },
                Err(e) => {
                    //println!("{}", e);
                }
            }
        }

        loop {

        }
        //assert_eq!(result, 4);
        */
    }
}
