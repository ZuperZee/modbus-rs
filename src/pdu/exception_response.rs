use crate::{error::EncodeError, exception_code::ExceptionCode};

use super::function_code::FunctionCode;

#[derive(Debug, PartialEq, Eq)]
pub struct ExceptionResponse {
    function_code: FunctionCode,
    exception_code: ExceptionCode,
}

impl ExceptionResponse {
    pub fn new(function_code: FunctionCode, exception_code: ExceptionCode) -> Self {
        Self {
            function_code,
            exception_code,
        }
    }

    pub fn pdu_len(&self) -> usize {
        2
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        if self.pdu_len() > buf.len() {
            return Err(EncodeError::InvalidBufferSize);
        }
        buf[0] = u8::from(self.function_code) | 0x80;
        buf[1] = self.exception_code as u8;

        Ok(self.pdu_len())
    }
}
