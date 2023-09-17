use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

fn main() {
    let addr: SocketAddr = "127.0.0.1:5502".parse().unwrap();
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
