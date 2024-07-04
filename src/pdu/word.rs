#[derive(Debug, PartialEq, Eq)]
pub struct DataWords<'a> {
    data: &'a [u8],
    quantity: usize,
}

impl<'a> DataWords<'a> {
    pub fn new(data: &'a [u8], quantity: usize) -> Self {
        Self { data, quantity }
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }
    pub fn quantity(&self) -> usize {
        self.quantity
    }

    pub fn from_words(words: &[u16], buf: &'a mut [u8]) -> Self {
        let data_len = words.len() * 2;

        for (i, word) in words.iter().enumerate() {
            let [high, low] = word.to_be_bytes();
            buf[i * 2] = high;
            buf[i * 2 + 1] = low;
        }

        Self {
            data: &buf[..data_len],
            quantity: words.len(),
        }
    }
}

impl<'a, 'b> DataWords<'a> {
    pub fn copy_words_to(&self, words: &'b mut [u16]) -> &'b [u16] {
        for (i, word) in words.iter_mut().enumerate().take(self.quantity) {
            let high = self.data[i * 2];
            let low = self.data[i * 2 + 1];
            *word = u16::from_be_bytes([high, low]);
        }

        &words[..self.quantity]
    }
}

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
impl From<DataWords<'_>> for Vec<u16> {
    fn from(data_words: DataWords) -> Self {
        Vec::from_iter((0..data_words.quantity).map(|i| {
            let high = data_words.data[i * 2];
            let low = data_words.data[i * 2 + 1];
            u16::from_be_bytes([high, low])
        }))
    }
}

#[cfg(test)]
mod test {
    use super::DataWords;

    #[test]
    fn data_words_from_words() {
        let mut buf = [0_u8; 20];
        let words: &[u16] = &[0xffff, 0x0900];
        let data_words = DataWords::from_words(words, &mut buf);

        assert_eq!(
            data_words,
            DataWords {
                data: &[255, 255, 9, 0],
                quantity: 2
            }
        )
    }

    #[test]
    fn words_from_data_words() {
        let mut words_buf = [0_u16; 4];
        let data_words = DataWords {
            data: &[0xff, 0xff, 0x09, 0],
            quantity: 2,
        };

        let words = data_words.copy_words_to(&mut words_buf);

        assert_eq!(words, &[0xffff, 0x0900]);
        assert_eq!(words_buf, [0xffff, 0x0900, 0, 0]);
    }

    #[cfg(feature = "alloc")]
    extern crate alloc;
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;
    #[cfg(feature = "alloc")]
    #[test]
    fn vec_words_from_data_words() {
        let data_words = DataWords {
            data: &[0xff, 0xff, 0x09, 0],
            quantity: 2,
        };

        let words = Vec::from(data_words);

        assert_eq!(words, &[0xffff, 0x0900]);
    }
}
