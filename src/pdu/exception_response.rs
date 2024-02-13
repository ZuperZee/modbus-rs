use crate::{error::EncodeError, exception_code::ExceptionCode};

use super::function_code::FunctionCode;

#[derive(Debug, PartialEq, Eq)]
pub struct ExceptionResponse {
    pub function_code: FunctionCode,
    pub exception_code: ExceptionCode,
}

impl ExceptionResponse {
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
