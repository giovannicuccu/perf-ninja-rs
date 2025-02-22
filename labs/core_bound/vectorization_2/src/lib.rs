use std::num::Wrapping;

mod tests;

// Assume this constant never changes
pub const N: usize = 64 * 1024;

pub type Blob = [u16; N];

pub fn checksum_opt(blob: &Blob) -> u16 {
    let mut acc = 0_u32;
    for value in blob {
        acc += *value as u32;
    }
    (acc%(u16::MAX as u32)) as u16
}
pub fn checksum(blob: &Blob) -> u16 {
    let mut acc = Wrapping(0);
    for value in blob {
        acc += value;
        acc += (acc.0 < *value) as u16; // add carry
    }
    acc.0
}

pub fn init(blob: &mut Blob) {
    use rand::prelude::*;

    let mut generator = thread_rng();
    for v in blob {
        *v = generator.gen();
    }
}
