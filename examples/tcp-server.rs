use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use modbus::{
    adu::tcp::{request::Request as AduRequest, response::Response as AduResponse},
    error::Error,
    pdu::{request::Request as PduRequest, response::Response as PduResponse, DataWords},
};

fn main() {
    let socket_addr = "localhost:5502";
    let listener = TcpListener::bind(socket_addr).unwrap();
    println!("Modbus server listening on {}", socket_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => {
                eprintln!("Failed creating a connection with error: {}", e)
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Client connected");
    loop {
        let mut buf = [0; 256];
        let bytes_read = match stream.read(&mut buf) {
            Ok(bytes_read) => bytes_read,
            Err(err) => {
                eprintln!("Failed reading stream with error: {}", err);
                return;
            }
        };

        if bytes_read == 0 {
            println!("EOF");
            return;
        }

        let req = match AduRequest::try_from(&buf[..]) {
            Ok(req) => req,
            Err(err) => match err {
                Error::ExceptionFunctionCode(_) => todo!(),
                Error::ExceptionCode(_) => todo!(),
                Error::EmptyBuffer => todo!(),
                Error::IncompleteBuffer => todo!(),
                Error::InvalidBufferSize => todo!(),
            },
        };

        let pdu_res = match req.pdu {
            PduRequest::ReadCoils(_, _) => todo!(),
            PduRequest::ReadDiscreteInput(_, _) => todo!(),
            PduRequest::ReadHoldingRegisters(_, _) => todo!(),
            PduRequest::ReadInputRegisters(_, _) => PduResponse::ReadInputRegisters(DataWords {
                data: &[0x01, 0x02],
                quantity: 1,
            }),
            _ => {
                println!("SERVER: Exception::IllegalFunction - Unimplemented function code in request: {req:?}");
                todo!()
            }
        };

        let res = AduResponse::new(req.header.transaction_id, req.header.unit_id, Ok(pdu_res));
        let mut res_buf = vec![0; res.adu_len()];
        let size = res.encode(&mut res_buf);

        println!("{:?}", req);
        println!("{:?}", res);
        println!("{:?}", res_buf);
        println!("{:?}", size);

        let _ = stream.write_all(&res_buf);
    }
}
