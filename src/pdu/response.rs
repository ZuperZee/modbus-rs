use crate::{
    error::{Error, ExceptionError},
    exception_code::ExceptionCode,
};

use super::{
    coil_to_u16_coil, function_code::FunctionCode, Address, DataCoils, DataWords, Quantity,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Response<'a> {
    ReadCoils(DataCoils<'a>),
    ReadDiscreteInput(DataCoils<'a>),
    ReadHoldingRegisters(DataWords<'a>),
    ReadInputRegisters(DataWords<'a>),
    WriteSingleCoil(Address, bool),
    WriteSingleRegister(Address, u16),
    WriteMultipleCoils(Address, Quantity),
    WriteMultipleRegisters(Address, Quantity),
    MaskWriteRegister(Address, u16, u16),
    ReadWriteMultipleRegisters(DataWords<'a>),
    Custom(FunctionCode, &'a [u8]),
}

impl<'a> TryFrom<&'a [u8]> for Response<'a> {
    type Error = Error;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        if buf.is_empty() {
            return Err(Error::EmptyBuffer);
        }

        let fn_code: FunctionCode = buf[0].try_into().map_err(|c| {
            if buf.len() > 1 {
                Error::ModbusExceptionCode(
                    FunctionCode::try_from(c & 0x7f).unwrap(),
                    ExceptionCode::try_from(buf[1]),
                )
            } else {
                Error::IncompleteBuffer {
                    current_size: 1,
                    min_needed_size: 2,
                }
            }
        })?;

        let response = match fn_code {
            FunctionCode::ReadCoils | FunctionCode::ReadDiscreteInput => {
                let Some(byte_count) = buf.get(1).map(|&v| v as usize) else {
                    return Err(Error::IncompleteBuffer {
                        current_size: 1,
                        min_needed_size: 2,
                    });
                };
                if byte_count + 2 > buf.len() {
                    return Err(Error::IncompleteBuffer {
                        current_size: buf.len(),
                        min_needed_size: byte_count + 2,
                    });
                }
                let data = &buf[2..byte_count + 2];
                let quantity = byte_count * 8;
                match fn_code {
                    FunctionCode::ReadCoils => Response::ReadCoils(DataCoils { data, quantity }),
                    FunctionCode::ReadDiscreteInput => {
                        Response::ReadDiscreteInput(DataCoils { data, quantity })
                    }
                    _ => unreachable!(),
                }
            }
            FunctionCode::ReadHoldingRegisters
            | FunctionCode::ReadInputRegisters
            | FunctionCode::ReadWriteMultipleRegisters => {
                let Some(byte_count) = buf.get(1).map(|&v| v as usize) else {
                    return Err(Error::IncompleteBuffer {
                        current_size: 1,
                        min_needed_size: 2,
                    });
                };
                if byte_count + 2 > buf.len() {
                    return Err(Error::IncompleteBuffer {
                        current_size: buf.len(),
                        min_needed_size: byte_count + 2,
                    });
                }
                let data = &buf[2..byte_count + 2];
                let quantity = byte_count / 2;
                match fn_code {
                    FunctionCode::ReadHoldingRegisters => {
                        Response::ReadHoldingRegisters(DataWords { data, quantity })
                    }
                    FunctionCode::ReadInputRegisters => {
                        Response::ReadInputRegisters(DataWords { data, quantity })
                    }
                    FunctionCode::ReadWriteMultipleRegisters => {
                        Response::ReadWriteMultipleRegisters(DataWords { data, quantity })
                    }
                    _ => unreachable!(),
                }
            }
            FunctionCode::WriteSingleCoil
            | FunctionCode::WriteSingleRegister
            | FunctionCode::WriteMultipleCoils
            | FunctionCode::WriteMultipleRegisters => {
                if 5 > buf.len() {
                    return Err(Error::IncompleteBuffer {
                        current_size: buf.len(),
                        min_needed_size: 5,
                    });
                }
                let address = u16::from_be_bytes(buf[1..3].try_into().unwrap());
                let data = u16::from_be_bytes(buf[3..5].try_into().unwrap());

                match fn_code {
                    FunctionCode::WriteSingleCoil => {
                        let value = match data {
                            0x0000 => false,
                            0xff00 => true,
                            _ => {
                                return Err(Error::ModbusExceptionError(
                                    fn_code,
                                    ExceptionError::IllegalDataValue,
                                ))
                            }
                        };
                        Response::WriteSingleCoil(address, value)
                    }
                    FunctionCode::WriteSingleRegister => {
                        Response::WriteSingleRegister(address, data)
                    }
                    FunctionCode::WriteMultipleCoils => {
                        if data >= 0x0001 || data <= 0x07b0 {
                            Response::WriteMultipleCoils(address, data)
                        } else {
                            return Err(Error::ModbusExceptionError(
                                fn_code,
                                ExceptionError::IllegalDataValue,
                            ));
                        }
                    }
                    FunctionCode::WriteMultipleRegisters => {
                        if data >= 0x0001 || data <= 0x007b {
                            Response::WriteMultipleRegisters(address, data)
                        } else {
                            return Err(Error::ModbusExceptionError(
                                fn_code,
                                ExceptionError::IllegalDataValue,
                            ));
                        }
                    }
                    _ => unreachable!(),
                }
            }
            FunctionCode::MaskWriteRegister => {
                if 7 > buf.len() {
                    return Err(Error::IncompleteBuffer {
                        current_size: buf.len(),
                        min_needed_size: 7,
                    });
                }
                let reference_address = u16::from_be_bytes(buf[1..3].try_into().unwrap());
                let and_mask = u16::from_be_bytes(buf[3..5].try_into().unwrap());
                let or_mask = u16::from_be_bytes(buf[5..7].try_into().unwrap());
                Response::MaskWriteRegister(reference_address, and_mask, or_mask)
            }
            FunctionCode::Custom(_) => Response::Custom(fn_code, &buf[1..]),
        };

        Ok(response)
    }
}

impl<'a> Response<'a> {
    pub fn pdu_len(&self) -> usize {
        match &self {
            Response::ReadCoils(coils) | Response::ReadDiscreteInput(coils) => 2 + coils.data.len(),
            Response::ReadHoldingRegisters(words)
            | Response::ReadInputRegisters(words)
            | Response::ReadWriteMultipleRegisters(words) => 2 + words.data.len(),
            Response::WriteSingleCoil(_, _)
            | Response::WriteSingleRegister(_, _)
            | Response::WriteMultipleCoils(_, _)
            | Response::WriteMultipleRegisters(_, _) => 5,
            Response::MaskWriteRegister(_, _, _) => 7,
            Response::Custom(_, d) => 1 + d.len(),
        }
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.pdu_len() > buf.len() {
            return Err(Error::InvalidBufferSize);
        }

        buf[0] = FunctionCode::from(self).into();

        match &self {
            Response::ReadCoils(coils) | Response::ReadDiscreteInput(coils) => {
                buf[1] = coils.data.len() as u8;
                buf[2..2 + coils.data.len()].copy_from_slice(coils.data);
            }
            Response::ReadHoldingRegisters(words)
            | Response::ReadInputRegisters(words)
            | Response::ReadWriteMultipleRegisters(words) => {
                buf[1] = words.data.len() as u8;
                buf[2..2 + words.data.len()].copy_from_slice(words.data);
            }
            Response::WriteSingleCoil(address, coil) => {
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                let data = coil_to_u16_coil(*coil);
                buf[3..5].copy_from_slice(&data.to_be_bytes());
            }
            Response::WriteSingleRegister(address, data)
            | Response::WriteMultipleCoils(address, data)
            | Response::WriteMultipleRegisters(address, data) => {
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                buf[3..5].copy_from_slice(&data.to_be_bytes());
            }
            Response::MaskWriteRegister(address, and_mask, or_mask) => {
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                buf[3..5].copy_from_slice(&and_mask.to_be_bytes());
                buf[5..7].copy_from_slice(&or_mask.to_be_bytes());
            }
            Response::Custom(_, data) => {
                buf[1..1 + data.len()].copy_from_slice(data);
            }
        };
        Ok(self.pdu_len())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        exception_code::ExceptionCode,
        pdu::{function_code::FunctionCode, DataWords},
    };

    use super::{DataCoils, Error, Response};

    #[test]
    fn response_from_buffer() {
        let buf: &[u8] = &[];
        assert_eq!(Response::try_from(buf), Err(Error::EmptyBuffer));

        let buf: &[u8] = &[0x81, 0x01];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::ModbusExceptionCode(
                FunctionCode::ReadCoils,
                Ok(ExceptionCode::IllegalFunction)
            ))
        );
        let buf: &[u8] = &[0x90, 0x02];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::ModbusExceptionCode(
                FunctionCode::WriteMultipleRegisters,
                Ok(ExceptionCode::IllegalDataAddress)
            ))
        );

        let buf: &[u8] = &[0x01, 0x02, 0xff, 0x7f];
        assert_eq!(
            Response::try_from(buf),
            Ok(Response::ReadCoils(DataCoils {
                data: &[0xff, 0x7f],
                quantity: 16
            }))
        );
        let buf: &[u8] = &[0x02, 0x02, 0xff, 0x7f];
        assert_eq!(
            Response::try_from(buf),
            Ok(Response::ReadDiscreteInput(DataCoils {
                data: &[0xff, 0x7f],
                quantity: 16
            }))
        );
        let buf: &[u8] = &[0x03, 0x04, 0x00, 0x11, 0x00, 0x04];
        assert_eq!(
            Response::try_from(buf),
            Ok(Response::ReadHoldingRegisters(DataWords {
                data: &[0x00, 0x11, 0x00, 0x04],
                quantity: 2
            }))
        );
        let buf: &[u8] = &[0x04, 0x04, 0x00, 0x11, 0x00, 0x04];
        assert_eq!(
            Response::try_from(buf),
            Ok(Response::ReadInputRegisters(DataWords {
                data: &[0x00, 0x11, 0x00, 0x04],
                quantity: 2
            }))
        );
    }

    #[test]
    fn response_from_incomplete_buffer() {
        let buf: &[u8] = &[0x01, 0x02, 0xff];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::IncompleteBuffer {
                current_size: 3,
                min_needed_size: 4,
            })
        );
        let buf: &[u8] = &[0x01];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::IncompleteBuffer {
                current_size: 1,
                min_needed_size: 2,
            })
        );
        let buf: &[u8] = &[0x02];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::IncompleteBuffer {
                current_size: 1,
                min_needed_size: 2,
            })
        );
        let buf: &[u8] = &[0x03];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::IncompleteBuffer {
                current_size: 1,
                min_needed_size: 2,
            })
        );
        let buf: &[u8] = &[0x04];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::IncompleteBuffer {
                current_size: 1,
                min_needed_size: 2,
            })
        );
        let buf: &[u8] = &[0x04, 0x04, 0x00, 0x11, 0x00];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::IncompleteBuffer {
                current_size: 5,
                min_needed_size: 6,
            })
        );
        let buf: &[u8] = &[0x04, 0x04, 0x00, 0x11];
        assert_eq!(
            Response::try_from(buf),
            Err(Error::IncompleteBuffer {
                current_size: 4,
                min_needed_size: 6,
            })
        );
    }

    #[test]
    fn buffer_from_response() {
        let res = Response::ReadCoils(DataCoils {
            data: &[0xff, 0x7f],
            quantity: 16,
        });
        let buf: &mut [u8] = &mut [0; 4];
        let pdu_len = res.encode(buf);
        assert_eq!(pdu_len, Ok(4));
        assert_eq!(buf, &[0x01, 0x02, 0xff, 0x7f]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn vec_buffer_from_response() {
        extern crate alloc;
        use alloc::vec;
        let res = Response::ReadCoils(DataCoils {
            data: &[0xff, 0x7f],
            quantity: 16,
        });
        let size = res.pdu_len();
        let mut buf = vec![0; size];
        let pdu_len = res.encode(&mut buf);
        assert_eq!(pdu_len, Ok(4));
        assert_eq!(buf, &[0x01, 0x02, 0xff, 0x7f]);
    }
}
