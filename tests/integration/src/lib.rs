use holium_rust_sdk::*;

#[holium_bindgen]
pub struct Test {
    pub key: u8,
}

#[holium_bindgen]
pub struct Structure {
    pub key: Vec<u8>,
}

#[holium_bindgen]
pub fn main(a: u32, b: u32) -> u32 {
    a + b
}
