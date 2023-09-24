#[derive(Debug, PartialEq, Eq)]
pub struct DataWords<'a> {
    pub data: &'a [u8],
    pub quantity: usize,
}
