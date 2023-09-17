use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream}, time::Duration,
};

use modbus::adu::tcp::response::Response;

fn main() {
    let addr: SocketAddr = "127.0.0.1:5502".parse().unwrap();
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    let transaction_id: u16 = 1;
    let protocol_id: u16 = 0;
    let unit_id: u8 = 1;
    let pdu = [0x4, 0x0, 0x0, 0x0, 0x7d];
    let length: u16 = (pdu.len() + 1).try_into().unwrap();
    let buf: &[u8] = &[
        transaction_id.to_be_bytes().as_ref(),
        protocol_id.to_be_bytes().as_ref(),
        length.to_be_bytes().as_ref(),
        &[unit_id],
        &pdu,
    ]
    .concat();
    stream.write_all(buf).unwrap();

    let mut res_buf: [u8; 4096] = [0; 4096];
    let bytes_read = stream.read(&mut res_buf).unwrap();
    let res_buf = &res_buf[0..bytes_read];
    let res = Response::try_from(res_buf).unwrap();
    println!("{:?}", buf);
    println!("{:?}", res_buf);
    println!("{:?}", bytes_read);
    println!("{:?}", res);
}
