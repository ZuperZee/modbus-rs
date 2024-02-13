use crate::error::{DecodeError, EncodeError};

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub transaction_id: u16,
    pub protocol_id: u16,
    pub length: u16,
    pub unit_id: u8,
}

impl Header {
    pub fn new(transaction_id: u16, length: u16, unit_id: u8) -> Self {
        Self {
            transaction_id,
            protocol_id: 0,
            length,
            unit_id,
        }
    }

    pub const fn size() -> usize {
        7
    }

    pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        if Self::size() > buf.len() {
            return Err(EncodeError::InvalidBufferSize);
        }

        buf[0..2].copy_from_slice(&self.transaction_id.to_be_bytes());
        buf[2..4].copy_from_slice(&self.protocol_id.to_be_bytes());
        buf[4..6].copy_from_slice(&self.length.to_be_bytes());
        buf[6] = self.unit_id;

        Ok(Self::size())
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        if buf.len() < Self::size() {
            return Err(DecodeError::IncompleteBuffer {
                current_size: buf.len(),
                min_needed_size: Self::size(),
            });
        };

        let transaction_id = u16::from_be_bytes([buf[0], buf[1]]);
        let protocol_id = u16::from_be_bytes([buf[2], buf[3]]);
        let length = u16::from_be_bytes([buf[4], buf[5]]);
        let unit_id = buf[6];

        Ok(Self {
            transaction_id,
            protocol_id,
            length,
            unit_id,
        })
    }
}

impl<'a> TryFrom<&'a [u8]> for Header {
    type Error = DecodeError;

    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        Self::decode(buf)
    }
}
