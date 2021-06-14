#![allow(unreachable_code)]
use holium_rust_sdk::proc_macro::*;

#[derive(holium_rust_sdk::Deserialize, holium_rust_sdk::Serialize)]
pub struct MyStruct {
    number: u32
}

#[holium_bindgen]
pub fn good1() {}

#[holium_bindgen]
pub fn good2() -> u32 { 0 }

#[holium_bindgen]
pub fn good3(a: u32) -> u32 { a }

#[holium_bindgen]
pub fn good4(a: u32) -> MyStruct { MyStruct{ number: a} }

#[holium_bindgen]
pub fn good5(a: MyStruct) -> MyStruct { a }

#[holium_bindgen]
pub fn good6(a: &MyStruct) -> MyStruct { MyStruct { number: a.number } }

#[holium_bindgen]
pub fn good7(a: &mut MyStruct) -> MyStruct {
    a.number += 10;
    let new_struct = MyStruct {
        number: a.number
    };
    return new_struct;
}

#[holium_bindgen]
pub fn good8(a: &mut MyStruct) -> (MyStruct, u32) {
    let old_number = a.number;
    a.number += 10;
    let new_struct = MyStruct {
        number: a.number
    };
    return (new_struct, old_number);
}

#[holium_bindgen]
pub fn good9(a: &mut MyStruct, b: String) -> (MyStruct, String) {
    a.number += 10;
    let new_struct = MyStruct {
        number: a.number
    };
    return (new_struct, b);
}

#[holium_bindgen]
pub fn bad1(a: &str) -> &String {
    let string = String::from(a);
    &string
}

fn main() {}
