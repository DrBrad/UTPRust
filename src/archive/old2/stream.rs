use std::io;
use std::io::{Read, Write};

pub struct UtpStream {
    connection_id: u16,
    //stream_events: Receiver<StreamEvent>
    buffer: Vec<u8>
}

impl UtpStream {

    pub fn new(connection_id: u16/*, stream_events: Receiver<StreamEvent>*/) -> Self {
        Self {
            connection_id,
            //stream_events,
            buffer: Vec::new()
        }
    }

}

impl Read for UtpStream {

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}

impl Write for UtpStream {

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}
