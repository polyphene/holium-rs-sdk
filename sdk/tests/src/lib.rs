use holium_rs_sdk::*;

#[holium_bindgen]
pub struct Value {
    pub a: u32,
}

#[holium_bindgen]
pub fn main(a: u32, b: u32) -> u32 {
    a + b
}
