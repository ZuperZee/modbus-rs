use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use modbus::{
    adu::tcp::{request::Request as AduRequest, response::Response as AduResponse},
    pdu::request::Request as PduRequest,
};

fn main() {
    let addr: SocketAddr = "127.0.0.1:5502".parse().unwrap();
    let mut stream = TcpStream::connect(addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    // let pdu_req = PduRequest::Custom(modbus::pdu::function_code::FunctionCode::Custom(0x80), &[0, 0, 0, 0]);
    let pdu_req = PduRequest::ReadHoldingRegisters(500, 1);
    let req = AduRequest::new(1, 1, pdu_req);
    let mut req_buf = vec![0_u8; req.adu_len()];
    req.encode(&mut req_buf).unwrap();
    stream.write_all(&req_buf).unwrap();

    let mut res_buf: [u8; 4096] = [0; 4096];
    let bytes_read = stream.read(&mut res_buf).unwrap();
    let res_buf = &res_buf[0..bytes_read];
    let res = AduResponse::try_from(res_buf).unwrap();
    println!("{:?}", req_buf);
    println!("{:?}", res_buf);
    println!("{:?}", bytes_read);
    println!("{:?}", res);
}
