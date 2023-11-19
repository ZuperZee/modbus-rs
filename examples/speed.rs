use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

fn main() {
    let addr: SocketAddr = "127.0.0.1:5502".parse().unwrap();
    let mut stream = TcpStream::connect(addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    let req_buf = vec![0, 1, 0, 0, 0, 3, 1, 4, 2];
    stream.write_all(&req_buf).unwrap();

    let mut res_buf: [u8; 300] = [0; 300];
    let bytes_read = stream.read(&mut res_buf).unwrap();
    println!("{:?}", &res_buf[..bytes_read]);
    println!("{:?}", bytes_read);
}
