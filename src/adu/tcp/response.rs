use crate::{
    error::{DecodeError, EncodeError},
    pdu::{exception_response::ExceptionResponse, response::Response as PduResponse},
};

use super::header::Header;

#[derive(Debug, PartialEq, Eq)]
pub struct Response<'a> {
    header: Header,
    pdu: Result<PduResponse<'a>, ExceptionResponse>,
}

impl<'a> Response<'a> {
    pub fn new(
        transaction_id: u16,
        unit_id: u8,
        pdu_res: Result<PduResponse<'a>, ExceptionResponse>,
    ) -> Self {
        let pdu_len = match &pdu_res {
            Ok(pdu) => pdu.pdu_len(),
            Err(pdu) => pdu.pdu_len(),
        };
        Self {
            // length is + 1 because of the unit_id
            header: Header::new(transaction_id, (pdu_len + 1) as u16, unit_id),
            pdu: pdu_res,
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn pdu(&self) -> &Result<PduResponse<'a>, ExceptionResponse> {
        &self.pdu
    }

    pub fn pdu_len(&self) -> usize {
        match &self.pdu {
            Ok(pdu) => pdu.pdu_len(),
            Err(pdu) => pdu.pdu_len(),
        }
    }

    pub fn adu_len(&self) -> usize {
        self.pdu_len() + Header::size()
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        if self.adu_len() > buf.len() {
            return Err(EncodeError::InvalidBufferSize);
        }

        let (header_buf, pdu_buf) = buf.split_at_mut(Header::size());

        let header_size = self.header.encode(header_buf)?;
        let pdu_size = match &self.pdu {
            Ok(pdu) => pdu.encode(pdu_buf)?,
            Err(pdu) => pdu.encode(pdu_buf)?,
        };

        Ok(header_size + pdu_size)
    }

    pub fn decode(buf: &'a [u8]) -> Result<Self, DecodeError> {
        if buf.len() < Header::size() {
            return Err(DecodeError::IncompleteBuffer {
                current_size: buf.len(),
                min_needed_size: Header::size(),
            });
        };

        let (header_buf, pdu_buf) = buf.split_at(Header::size());

        let header = Header::try_from(header_buf)?;
        if *header.length() as usize > pdu_buf.len() + 1 {
            return Err(DecodeError::IncompleteBuffer {
                current_size: buf.len(),
                // unit_id is included in the header.length and header.size
                // so we need to subtract 1
                min_needed_size: *header.length() as usize + Header::size() - 1,
            });
        };

        let pdu = PduResponse::try_from(pdu_buf).map_err(|err| match err {
            DecodeError::IncompleteBuffer {
                current_size,
                min_needed_size,
            } => DecodeError::IncompleteBuffer {
                current_size: current_size + Header::size(),
                min_needed_size: min_needed_size + Header::size(),
            },
            err => err,
        })?;

        Ok(Self {
            header,
            pdu: Ok(pdu),
        })
    }
}

impl<'a> TryFrom<&'a [u8]> for Response<'a> {
    type Error = DecodeError;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        Self::decode(buf)
    }
}

impl<'a> From<(Header, Result<PduResponse<'a>, ExceptionResponse>)> for Response<'a> {
    fn from(value: (Header, Result<PduResponse<'a>, ExceptionResponse>)) -> Self {
        Self {
            header: value.0,
            pdu: value.1,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pdu::DataWords;

    use super::{Header, PduResponse, Response};

    #[test]
    fn response_from_buffer() {
        let buf: &[u8] = &[0, 1, 0, 0, 0, 13, 1, 4, 10, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5];
        assert_eq!(
            Response::try_from(buf),
            Ok(Response {
                header: Header::new(1, 13, 1),
                pdu: Ok(PduResponse::ReadInputRegisters(DataWords::new(
                    &[0, 1, 0, 2, 0, 3, 0, 4, 0, 5],
                    5
                )))
            })
        );
    }

    #[test]
    fn buffer_from_response() {
        let res = Response {
            header: Header::new(1, 13, 1),
            pdu: Ok(PduResponse::ReadInputRegisters(DataWords::new(
                &[0, 1, 0, 2, 0, 3, 0, 4, 0, 5],
                5,
            ))),
        };
        let buf = &mut [0_u8; 19];
        let adu_len = res.encode(buf);
        assert_eq!(adu_len, Ok(19));
        assert_eq!(
            buf,
            &[0, 1, 0, 0, 0, 13, 1, 4, 10, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5]
        );
    }
}
