pub mod coils;
pub mod function_code;
pub mod request;
pub mod response;
pub mod words;

pub type Address = u16;
pub type Coil = bool;
pub type Word = u16;
pub type Quantity = u16;

// pub type Coils<'a> = &'a [Coil];
// pub type Words<'a> = &'a [Word];

pub fn u16_coil_to_coil(u16_coil: u16) -> Option<Coil> {
    match u16_coil {
        0x0000 => Some(false),
        0xff00 => Some(true),
        _ => None,
    }
}

pub fn coil_to_u16_coil(coil: Coil) -> u16 {
    match coil {
        false => 0x0000,
        true => 0xff00,
    }
}
