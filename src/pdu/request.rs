use crate::{error::Error, exception_code::ExceptionCode};

use super::{
    coil_to_u16_coil, DataCoils, function_code::FunctionCode, u16_coil_to_coil, DataWords,
    Address, Quantity,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Request<'a> {
    ReadCoils(Address, Quantity),
    ReadDiscreteInput(Address, Quantity),
    ReadHoldingRegisters(Address, Quantity),
    ReadInputRegisters(Address, Quantity),
    WriteSingleCoil(Address, bool),
    WriteSingleRegister(Address, u16),
    WriteMultipleCoils(Address, DataCoils<'a>),
    WriteMultipleRegisters(Address, DataWords<'a>),
    MaskWriteRegister(Address, u16, u16),
    ReadWriteMultipleRegisters(Address, Quantity, Address, DataWords<'a>),
    Custom(FunctionCode, &'a [u8]),
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = Error;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        if buf.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let fn_code: FunctionCode = buf[0].try_into()?;

        let request = match fn_code {
            FunctionCode::ReadCoils
            | FunctionCode::ReadDiscreteInput
            | FunctionCode::ReadHoldingRegisters
            | FunctionCode::ReadInputRegisters => {
                if 5 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let address = u16::from_be_bytes(buf[1..3].try_into().unwrap());
                let quantity = u16::from_be_bytes(buf[3..5].try_into().unwrap());

                match fn_code {
                    FunctionCode::ReadCoils => {
                        if quantity == 0 || quantity > 0x07d0 {
                            return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                        }
                        Request::ReadCoils(address, quantity)
                    }
                    FunctionCode::ReadDiscreteInput => {
                        if quantity == 0 || quantity > 0x07d0 {
                            return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                        }
                        Request::ReadDiscreteInput(address, quantity)
                    }
                    FunctionCode::ReadHoldingRegisters => {
                        if quantity == 0 || quantity > 0x7d {
                            return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                        }
                        Request::ReadHoldingRegisters(address, quantity)
                    }
                    FunctionCode::ReadInputRegisters => {
                        if quantity == 0 || quantity > 0x7d {
                            return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                        }
                        Request::ReadInputRegisters(address, quantity)
                    }
                    _ => unreachable!(),
                }
            }
            FunctionCode::WriteSingleCoil | FunctionCode::WriteSingleRegister => {
                if 5 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let address = u16::from_be_bytes(buf[1..3].try_into().unwrap());
                let value = u16::from_be_bytes(buf[3..5].try_into().unwrap());

                match fn_code {
                    FunctionCode::WriteSingleCoil => {
                        let Some(coil_bool) = u16_coil_to_coil(value) else {
                            return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                        };
                        Request::WriteSingleCoil(address, coil_bool)
                    }
                    FunctionCode::WriteSingleRegister => {
                        Request::WriteSingleRegister(address, value)
                    }
                    _ => unreachable!(),
                }
            }
            FunctionCode::WriteMultipleCoils => {
                if 6 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let address = u16::from_be_bytes(buf[1..3].try_into().unwrap());
                let quantity = u16::from_be_bytes(buf[3..5].try_into().unwrap());
                if quantity == 0 || quantity > 0x07b0 {
                    return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                }
                let byte_count = buf[5] as usize;
                if byte_count + 6 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let data = &buf[6..byte_count + 6];
                Request::WriteMultipleCoils(
                    address,
                    DataCoils {
                        data,
                        quantity: quantity as usize,
                    },
                )
            }
            FunctionCode::WriteMultipleRegisters => {
                if 6 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let address = u16::from_be_bytes(buf[1..3].try_into().unwrap());
                let quantity = u16::from_be_bytes(buf[3..5].try_into().unwrap());
                if quantity == 0 || quantity > 0x7b {
                    return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                }
                let byte_count = buf[5] as usize;
                if byte_count + 6 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let data = &buf[6..byte_count + 6];
                Request::WriteMultipleRegisters(
                    address,
                    DataWords {
                        data,
                        quantity: quantity as usize,
                    },
                )
            }
            FunctionCode::MaskWriteRegister => {
                if 7 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let reference_address = u16::from_be_bytes(buf[1..3].try_into().unwrap());
                let and_mask = u16::from_be_bytes(buf[3..5].try_into().unwrap());
                let or_mask = u16::from_be_bytes(buf[5..7].try_into().unwrap());
                Request::MaskWriteRegister(reference_address, and_mask, or_mask)
            }
            FunctionCode::ReadWriteMultipleRegisters => {
                if 10 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let read_address = u16::from_be_bytes(buf[1..3].try_into().unwrap());
                let read_quantity = u16::from_be_bytes(buf[3..5].try_into().unwrap());
                if read_quantity == 0 || read_quantity > 0x7d {
                    return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                }
                let write_address = u16::from_be_bytes(buf[5..7].try_into().unwrap());
                let write_quantity = u16::from_be_bytes(buf[7..9].try_into().unwrap());
                if write_quantity == 0 || write_quantity > 0x7d {
                    return Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue));
                }
                let write_byte_count = buf[9] as usize;
                if write_byte_count + 10 > buf.len() {
                    return Err(Error::IncompleteBuffer);
                }
                let data = &buf[10..write_byte_count + 10];
                Request::ReadWriteMultipleRegisters(
                    read_address,
                    read_quantity,
                    write_address,
                    DataWords {
                        data,
                        quantity: write_quantity as usize,
                    },
                )
            }
            FunctionCode::Custom(_) => Request::Custom(fn_code, &buf[1..]),
        };

        Ok(request)
    }
}

impl<'a> Request<'a> {
    pub fn pdu_len(&self) -> usize {
        match &self {
            Request::ReadCoils(_, _)
            | Request::ReadDiscreteInput(_, _)
            | Request::ReadHoldingRegisters(_, _)
            | Request::ReadInputRegisters(_, _)
            | Request::WriteSingleCoil(_, _)
            | Request::WriteSingleRegister(_, _) => 5,
            Request::WriteMultipleCoils(_, coils) => 6 + coils.data.len(),
            Request::WriteMultipleRegisters(_, words) => 6 + words.data.len(),
            Request::MaskWriteRegister(_, _, _) => 7,
            Request::ReadWriteMultipleRegisters(_, _, _, words) => 10 + words.data.len(),
            Request::Custom(_, d) => 1 + d.len(),
        }
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.pdu_len() > buf.len() {
            return Err(Error::InvalidBufferSize);
        }

        buf[0] = FunctionCode::from(self).into();

        match &self {
            Request::ReadCoils(address, quantity)
            | Request::ReadDiscreteInput(address, quantity)
            | Request::ReadHoldingRegisters(address, quantity)
            | Request::ReadInputRegisters(address, quantity) => {
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                buf[3..5].copy_from_slice(&quantity.to_be_bytes());
            }
            Request::WriteSingleCoil(address, coil) => {
                let data = coil_to_u16_coil(*coil);
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                buf[3..5].copy_from_slice(&data.to_be_bytes());
            }
            Request::WriteSingleRegister(address, word) => {
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                buf[3..5].copy_from_slice(&word.to_be_bytes());
            }
            Request::WriteMultipleCoils(address, coils) => {
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                buf[3..5].copy_from_slice(&coils.quantity.to_be_bytes());
                buf[5] = coils.data.len() as u8;
                buf[6..coils.data.len() + 6].copy_from_slice(coils.data);
            }
            Request::WriteMultipleRegisters(address, words) => {
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                buf[3..5].copy_from_slice(&words.quantity.to_be_bytes());
                buf[5] = words.data.len() as u8;
                buf[6..words.data.len() + 6].copy_from_slice(words.data);
            }
            Request::MaskWriteRegister(address, and_mask, or_mask) => {
                buf[1..3].copy_from_slice(&address.to_be_bytes());
                buf[3..5].copy_from_slice(&and_mask.to_be_bytes());
                buf[5..7].copy_from_slice(&or_mask.to_be_bytes());
            }
            Request::ReadWriteMultipleRegisters(
                read_address,
                read_quantity,
                write_address,
                write_words,
            ) => {
                buf[1..3].copy_from_slice(&read_address.to_be_bytes());
                buf[3..5].copy_from_slice(&read_quantity.to_be_bytes());
                buf[5..7].copy_from_slice(&write_address.to_be_bytes());
                buf[7..9].copy_from_slice(&write_words.quantity.to_be_bytes());
                buf[9..write_words.data.len() + 9].copy_from_slice(write_words.data);
            }
            Request::Custom(_, data) => {
                buf[1..1 + data.len()].copy_from_slice(data);
            }
        }

        Ok(self.pdu_len())
    }
}

#[cfg(test)]
mod test {
    use crate::{exception_code::ExceptionCode, pdu::DataCoils};

    use super::{Error, Request};

    #[test]
    fn request_from_buffer() {
        let buf: &[u8] = &[];
        assert_eq!(Request::try_from(buf), Err(Error::EmptyBuffer));

        let buf: &[u8] = &[0x80];
        assert_eq!(
            Request::try_from(buf),
            Err(Error::ExceptionFunctionCode(0x80))
        );
        let buf: &[u8] = &[0x90];
        assert_eq!(
            Request::try_from(buf),
            Err(Error::ExceptionFunctionCode(0x90))
        );

        let buf: &[u8] = &[0x01, 0x00, 0x06, 0x03, 0xe8];
        assert_eq!(Request::try_from(buf), Ok(Request::ReadCoils(6, 1000)));
        let buf: &[u8] = &[0x01, 0x00, 0x06, 0x80, 0x00];
        assert_eq!(
            Request::try_from(buf),
            Err(Error::ExceptionCode(ExceptionCode::IllegalDataValue))
        );

        let buf: &[u8] = &[0x0f, 0x00, 0x13, 0x00, 0x0a, 0x02, 0xcd, 0x01];
        assert_eq!(
            Request::try_from(buf),
            Ok(Request::WriteMultipleCoils(
                0x13,
                DataCoils {
                    data: &[0xcd, 0x01],
                    quantity: 0x0a
                }
            ))
        );
    }

    #[test]
    fn buffer_from_response() {
        let res = Request::ReadCoils(0x03e8, 0x0123);
        let buf: &mut [u8] = &mut [0; 5];
        let pdu_len = res.encode(buf);
        assert_eq!(pdu_len, Ok(5));
        assert_eq!(buf, &[0x01, 0x03, 0xe8, 0x01, 0x23]);
    }
}
