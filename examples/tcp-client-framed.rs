use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
};

pub enum Error {
    Io(io::Error),
    ConnectionResetByPeer,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

// enum Frame {
//     Simple(String),
//     Error(String),
//     Integer(u64),
//     Null,
//     Array(Vec<Frame>),
// }

// pub struct Connection {
//     stream: TcpStream,
//     buffer: [u8; 512],
//     cursor: usize,
// }

// impl Connection {
//     pub fn new(stream: TcpStream) -> Self {
//         Self {
//             stream,
//             buffer: [0; 512],
//             cursor: 0,
//         }
//     }

//     pub async fn read_frame(&mut self) -> Result<Option<Frame>, Error> {
//         loop {
//             if let Some(frame) = self.parse_frame()? {
//                 return Ok(Some(frame));
//             }

//             // Read into the buffer, tracking the number
//             // of bytes read
//             let n = self.stream.read(&mut self.buffer[self.cursor..])?;

//             if 0 == n {
//                 if self.cursor == 0 {
//                     return Ok(None);
//                 } else {
//                     return Err(Error::ConnectionResetByPeer);
//                 }
//             } else {
//                 // Update our cursor
//                 self.cursor += n;
//             }
//         }
//     }

//     fn parse_frame(&mut self) -> Result<Option<Frame>, Error> {
//         // Create the `T: Buf` type.
//         let mut buf = Cursor::new(&self.buffer[..]);

//         // Check whether a full frame is available
//         match Frame::check(&mut buf) {
//             Ok(_) => {
//                 // Get the byte length of the frame
//                 let len = buf.position() as usize;

//                 // Reset the internal cursor for the
//                 // call to `parse`.
//                 buf.set_position(0);

//                 // Parse the frame
//                 let frame = Frame::parse(&mut buf)?;

//                 // Return the frame to the caller.
//                 Ok(Some(frame))
//             }
//             // Not enough data has been buffered
//             Err(Incomplete) => Ok(None),
//             // An error was encountered
//             Err(e) => Err(e.into()),
//         }
//     }
// }

fn main() {
    let addr: SocketAddr = "127.0.0.1:5503".parse().unwrap();
    let mut stream = TcpStream::connect(addr).unwrap();
    let buf: &[u8] = &[0x0, 0x0, 0x0, 0x0, 0x0, 0x6, 0x1, 0x4, 0x0, 0x0, 0x0, 0x5];
    stream.write_all(buf).unwrap();

    let mut res_buf: [u8; 4096] = [0; 4096];
    let bytes_read = stream.read(&mut res_buf).unwrap();
    let x = &res_buf[0..bytes_read];
    println!("{:?}", buf);
    println!("{:?}", x);
    println!("{:?}", bytes_read);
}
