use crate::{error::Error, pdu::request::Request as PduRequest};

use super::header::Header;

#[derive(Debug, PartialEq, Eq)]
pub struct Request<'a> {
    pub header: Header,
    pub pdu: PduRequest<'a>,
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
        self.pdu_len() + Header::size()
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, Error> {
        if self.adu_len() > buf.len() {
            return Err(Error::InvalidBufferSize);
        }

        let (header_buf, pdu_buf) = buf.split_at_mut(Header::size());

        let header_size = self.header.encode(header_buf)?;
        let pdu_size = self.pdu.encode(pdu_buf)?;

        Ok(header_size + pdu_size)
    }
}
