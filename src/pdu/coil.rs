#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DataCoils<'a> {
    pub data: &'a [u8],
    pub quantity: usize,
}

impl<'a, 'b> DataCoils<'a> {
    pub fn new(data: &'a [u8], quantity: usize) -> Self {
        Self { data, quantity }
    }

    pub fn from_coils(coils: &'b [bool], buf: &'a mut [u8]) -> Self {
        for (i, bits) in coils.chunks(8).enumerate() {
            let mut byte: u8 = 0;
            for (j, &bit) in bits.iter().enumerate() {
                byte |= (bit as u8) << (7 - (j % 8));
            }
            buf[i] = byte;
        }

        let data_len = (coils.len() + 7) / 8;

        Self {
            data: &buf[..data_len],
            quantity: coils.len(),
        }
    }

    pub fn coils_len(&self) -> usize {
        self.quantity
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn copy_coils_to(&self, coils: &'b mut [bool]) -> &'b [bool] {
        for (i, coil) in coils.iter_mut().enumerate().take(self.quantity) {
            let byte = self.data[i / 8];
            let bit_pos = 7 - (i % 8);
            *coil = (byte & (1 << (bit_pos))) != 0;
        }

        &coils[..self.quantity]
    }
}

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
extern crate std;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
impl From<DataCoils<'_>> for Vec<bool> {
    fn from(data_coils: DataCoils) -> Self {
        Vec::from_iter((0..data_coils.quantity).map(|i| {
            let byte = data_coils.data[i / 8];
            let bit_pos = 7 - (i % 8);
            (byte & (1 << (bit_pos))) != 0
        }))
    }
}

#[cfg(test)]
pub mod test {
    use super::DataCoils;

    #[test]
    fn data_coils_from_coils() {
        let mut buf = [0_u8; 20];
        let coils = &[
            true, true, true, true, true, true, true, true, false, false, false, false, true,
            false, false, true, false, true,
        ];
        let data_coils = DataCoils::from_coils(coils, &mut buf);

        assert_eq!(
            data_coils,
            DataCoils {
                data: &[255, 9, 64],
                quantity: 18
            }
        )
    }

    #[test]
    fn coils_from_data_coils() {
        let mut coils_buf = [false; 20];
        let data_coils = DataCoils {
            data: &[255, 9, 64],
            quantity: 18,
        };

        let coils = data_coils.copy_coils_to(&mut coils_buf);

        assert_eq!(
            coils,
            &[
                true, true, true, true, true, true, true, true, false, false, false, false, true,
                false, false, true, false, true,
            ]
        );
        assert_eq!(
            coils_buf,
            [
                true, true, true, true, true, true, true, true, false, false, false, false, true,
                false, false, true, false, true, false, false,
            ]
        );
    }

    #[cfg(feature = "alloc")]
    extern crate alloc;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;
    #[cfg(feature = "alloc")]
    #[test]
    fn vec_coils_from_data_coils() {
        let data_coils = DataCoils {
            data: &[255, 9, 64],
            quantity: 18,
        };

        let coils = Vec::from(data_coils);

        assert_eq!(
            coils,
            &[
                true, true, true, true, true, true, true, true, false, false, false, false, true,
                false, false, true, false, true,
            ]
        );
    }
}
