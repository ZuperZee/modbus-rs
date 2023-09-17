use crate::{error::Error, pdu::request::Request as PduRequest};

use super::header::Header;

#[derive(Debug, PartialEq, Eq)]
pub struct Request<'a> {
    pub header: Header,
    pub pdu: PduRequest<'a>,
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
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

        let pdu = PduRequest::try_from(pdu_buf)?;

        Ok(Self { header, pdu })
    }
}

impl<'a> Request<'a> {
    pub fn new(transaction_id: u16, unit_id: u8, pdu_req: PduRequest<'a>) -> Self {
        Self {
            header: Header::new(transaction_id, (pdu_req.pdu_len() + 1) as u16, unit_id),
            pdu: pdu_req,
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
    use super::{Header, PduRequest, Request};

    #[test]
    fn response_from_buffer() {
        let buf: &[u8] = &[0, 1, 0, 0, 0, 6, 1, 4, 0, 10, 0, 15];
        assert_eq!(
            Request::try_from(buf),
            Ok(Request {
                header: Header {
                    transaction_id: 1,
                    protocol_id: 0,
                    length: 6,
                    unit_id: 1,
                },
                pdu: PduRequest::ReadInputRegisters(10, 15)
            })
        );
    }
}
