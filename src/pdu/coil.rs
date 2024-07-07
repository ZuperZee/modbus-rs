#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DataCoils<'a> {
    data: &'a [u8],
    quantity: usize,
}

impl<'a> DataCoils<'a> {
    pub fn new(data: &'a [u8], quantity: usize) -> Self {
        Self { data, quantity }
    }

    pub fn from_coils(coils: &[bool], buf: &'a mut [u8]) -> Self {
        for (i, bits) in coils.chunks(8).enumerate() {
            let mut byte: u8 = 0;
            for (j, &bit) in bits.iter().enumerate() {
                byte |= (bit as u8) << j;
            }
            buf[i] = byte;
        }

        let data_len = (coils.len() + 7) / 8;

        Self {
            data: &buf[..data_len],
            quantity: coils.len(),
        }
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }
    pub fn quantity(&self) -> usize {
        self.quantity
    }

    pub fn coils_len(&self) -> usize {
        self.quantity
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}

impl<'a, 'b> DataCoils<'a> {
    pub fn copy_coils_to(&self, coils: &'b mut [bool]) -> &'b [bool] {
        for (i, coil) in coils.iter_mut().enumerate().take(self.quantity) {
            let byte = self.data[i / 8];
            let bit_pos = i % 8;
            *coil = (byte & (1 << (bit_pos))) != 0;
        }

        &coils[..self.quantity]
    }
}

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
impl From<DataCoils<'_>> for Vec<bool> {
    fn from(data_coils: DataCoils<'_>) -> Self {
        Vec::from_iter((0..data_coils.quantity).map(|i| {
            let byte = data_coils.data[i / 8];
            let bit_pos = i % 8;
            (byte & (1 << (bit_pos))) != 0
        }))
    }
}

#[cfg(test)]
pub mod test {
    use super::DataCoils;

    #[test]
    fn data_coils_from_coils() {
        let mut buf = [0_u8; 0x13];
        let coils = &[
            true, false, true, true, false, false, true, true, true, true, false, true, false,
            true, true, false, true, false, true,
        ];
        let data_coils = DataCoils::from_coils(coils, &mut buf);

        assert_eq!(
            data_coils,
            DataCoils {
                data: &[0xCD, 0x6B, 0x05],
                quantity: 0x13
            }
        );

        let mut buf = [0_u8; 20];
        let coils = &[true, false];
        let data_coils = DataCoils::from_coils(coils, &mut buf);

        assert_eq!(
            data_coils,
            DataCoils {
                data: &[1],
                quantity: 2
            }
        );
    }

    #[test]
    fn coils_from_data_coils() {
        let mut coils_buf = [false; 0x13];
        let data_coils = DataCoils {
            data: &[0xCD, 0x6B, 0x05],
            quantity: 0x13,
        };

        let coils = data_coils.copy_coils_to(&mut coils_buf);

        assert_eq!(
            coils,
            &[
                true, false, true, true, false, false, true, true, true, true, false, true, false,
                true, true, false, true, false, true
            ]
        );
        assert_eq!(
            coils_buf,
            [
                true, false, true, true, false, false, true, true, true, true, false, true, false,
                true, true, false, true, false, true
            ]
        );

        let mut coils_buf = [false; 0x8];
        let data_coils = DataCoils {
            data: &[0xfe],
            quantity: 0x8,
        };

        let coils = data_coils.copy_coils_to(&mut coils_buf);

        assert_eq!(coils, &[false, true, true, true, true, true, true, true]);
        assert_eq!(coils_buf, [false, true, true, true, true, true, true, true]);
    }

    #[cfg(feature = "alloc")]
    extern crate alloc;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;
    #[cfg(feature = "alloc")]
    #[test]
    fn vec_coils_from_data_coils() {
        let data_coils = DataCoils {
            data: &[255, 9, 2],
            quantity: 18,
        };

        let coils = Vec::from(data_coils);

        assert_eq!(
            coils,
            &[
                true, true, true, true, true, true, true, true, true, false, false, true, false,
                false, false, false, false, true,
            ]
        );
    }
}
