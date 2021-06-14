use holium_rust_sdk::proc_macro::*;

#[derive(holium_rust_sdk::Deserialize, holium_rust_sdk::Serialize)]
pub struct MyStruct {
    string: String
}

#[holium_bindgen]
pub fn main(mut a: u32, mut b: String, c: &mut MyStruct) -> (u32, String, &MyStruct) {
    a += 10;
    b += c.string.as_str();
    c.string += b.as_str();
    (a, b, c)
}
