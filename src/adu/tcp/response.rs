use crate::{error::Error, pdu::response::Response as PduResponse};

use super::header::Header;

#[derive(Debug, PartialEq, Eq)]
pub struct Response<'a> {
    pub header: Header,
    pub pdu: PduResponse<'a>,
}

impl<'a> TryFrom<&'a [u8]> for Response<'a> {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() < 7 {
            return Err(Error::IncompleteBuffer);
        };

        let (header_buf, pdu_buf) = value.split_at(7);

        let header = Header::try_from(header_buf)?;
        if header.length as usize > pdu_buf.len() + 1 {
            return Err(Error::IncompleteBuffer);
        };

        let pdu = PduResponse::try_from(pdu_buf)?;

        Ok(Self { header, pdu })
    }
}

impl<'a> Response<'a> {
    pub fn new(transaction_id: u16, unit_id: u8, pdu_res: PduResponse<'a>) -> Self {
        Self {
            header: Header::new(transaction_id, (pdu_res.pdu_len() + 1) as u16, unit_id),
            pdu: pdu_res,
        }
    }
    pub fn pdu_len(&self) -> usize {
        self.pdu.pdu_len()
    }

    pub fn adu_len(&self) -> usize {
        self.pdu_len() + self.header.size()
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.adu_len() > buf.len() {
            return Err(Error::InvalidBufferSize);
        }

        let (header_buf, pdu_buf) = buf.split_at_mut(self.header.size());

        let header_size = self.header.encode(header_buf)?;
        let pdu_size = self.pdu.encode(pdu_buf)?;

        Ok(header_size + pdu_size)
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
                header: Header {
                    transaction_id: 1,
                    protocol_id: 0,
                    length: 13,
                    unit_id: 1,
                },
                pdu: PduResponse::ReadInputRegisters(DataWords {
                    data: &[0, 1, 0, 2, 0, 3, 0, 4, 0, 5],
                    quantity: 5
                })
            })
        );
    }

    #[test]
    fn buffer_from_response() {
        let res = Response {
            header: Header {
                transaction_id: 1,
                protocol_id: 0,
                length: 13,
                unit_id: 1,
            },
            pdu: PduResponse::ReadInputRegisters(DataWords {
                data: &[0, 1, 0, 2, 0, 3, 0, 4, 0, 5],
                quantity: 5,
            }),
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
