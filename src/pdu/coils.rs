#[derive(Debug, PartialEq, Eq)]
pub struct Coils<'a> {
    pub data: &'a [u8],
    pub quantity: usize,
}
