#[derive(Debug, PartialEq, Eq)]
pub struct Words<'a> {
    pub data: &'a [u8],
    pub quantity: usize,
}
