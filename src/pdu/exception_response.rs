use crate::{error::Error, exception_code::ExceptionCode};

use super::function_code::FunctionCode;

#[derive(Debug, PartialEq, Eq)]
pub struct ExceptionResponse(FunctionCode, ExceptionCode);

impl ExceptionResponse {
    pub fn pdu_len(&self) -> usize {
        2
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.pdu_len() > buf.len() {
            return Err(Error::InvalidBufferSize);
        }
        buf[0] = self.0.into();
        buf[1] = self.1 as u8;

        Ok(self.pdu_len())
    }
}
