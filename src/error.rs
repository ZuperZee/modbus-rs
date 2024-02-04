use crate::{exception_code::ExceptionCode, pdu::function_code::FunctionCode};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    EmptyBuffer,
    IncompleteBuffer {
        current_size: usize,
        min_needed_size: usize,
    },
    InvalidBufferSize,

    /// Returned when the function code is valid, but the response is an error
    ModbusExceptionError(FunctionCode, ExceptionError),
    /// Returned when the function code is an error itself
    ModbusExceptionCode(FunctionCode, Result<ExceptionCode, u8>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExceptionError {
    IllegalDataAddress(u16),
    IllegalDataValue,
}
