#![allow(unreachable_code)]
use holium_rs_sdk::*;
use serde::{Deserialize, Serialize};

#[holium_bindgen]
pub struct GoodStruct {
    number: u32,
}

#[holium_bindgen]
pub fn good1() {}

#[holium_bindgen]
pub fn good2() -> u32 {
    0
}

#[holium_bindgen]
pub fn good3(a: u32) -> u32 {
    a
}

#[holium_bindgen]
pub fn good4(a: u32) -> GoodStruct {
    GoodStruct { number: a }
}

#[holium_bindgen]
pub fn good5(a: GoodStruct) -> GoodStruct {
    a
}

#[holium_bindgen]
pub fn good6(a: &GoodStruct) -> GoodStruct {
    GoodStruct { number: a.number }
}

#[holium_bindgen]
pub fn good7(a: &mut GoodStruct) -> GoodStruct {
    a.number += 10;
    let new_struct = GoodStruct { number: a.number };
    return new_struct;
}

#[holium_bindgen]
pub fn good8(a: &mut GoodStruct) -> (GoodStruct, u32) {
    let old_number = a.number;
    a.number += 10;
    let new_struct = GoodStruct { number: a.number };
    return (new_struct, old_number);
}

#[holium_bindgen]
pub fn good9(a: &mut GoodStruct, b: String) -> (GoodStruct, String) {
    a.number += 10;
    let new_struct = GoodStruct { number: a.number };
    return (new_struct, b);
}

#[holium_bindgen]
pub fn good10(a: Option<u32>) -> Option<u32> {
    a
}

#[holium_bindgen]
pub fn good11(a: Vec<u32>) -> Vec<u32> {
    a
}

struct BadStructNoMacro {
    number: u32,
}

#[holium_bindgen]
pub fn bad1(a: BadStruct) -> BadStruct {
    a
}

#[derive(Serialize, Deserialize)]
struct BadStructOnlySerde {
    number: u32,
}

#[holium_bindgen]
pub fn bad2(a: BadStructOnlySerde) -> BadStructOnlySerde {
    a
}

#[holium_bindgen]
pub fn bad3<'a>(x: &'a GoodStruct, y: &'a GoodStruct) -> &'a GoodStruct {
    GoodStruct {
        number: x.number + y.number,
    }
}

#[holium_bindgen]
pub fn bad4<T>(x: T) -> T {
    x
}

fn main() {}
