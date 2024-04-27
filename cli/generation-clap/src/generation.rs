use rand::distributions::{uniform::SampleUniform, Alphanumeric, DistString};
use rand::Rng;

pub const LENGTH: usize = 32;

pub const CHARACTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~";

pub fn float() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn signed() -> i32 {
    rand::thread_rng().gen::<i32>()
}

pub fn unsigned() -> u32 {
    rand::thread_rng().gen::<u32>()
}

pub fn string(length: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), length)
}

pub fn range<T: SampleUniform + PartialOrd>(start: T, end: T) -> T {
    rand::thread_rng().gen_range(start..end)
}

pub fn custom(length: usize, characters: &[u8]) -> String {
    (0..length)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..characters.len());
            characters[idx] as char
        })
        .collect()
}
