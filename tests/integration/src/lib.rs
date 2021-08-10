use holium_rust_sdk::proc_macro::*;
use std::collections::HashMap;
use std::borrow::Borrow;


#[holium_bindgen]
fn main(a: u32, b: u32) -> (u32) {
    let result = a + b;

    (result)
}