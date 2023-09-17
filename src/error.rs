use super::exception_code::ExceptionCode;

/// modbus error
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    ExceptionFunctionCode(u8),
    ExceptionCode(ExceptionCode),
    EmptyBuffer,
    IncompleteBuffer,
    InvalidBufferSize,
}
