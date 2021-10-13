#![allow(dead_code)]

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_SIZE: usize = PKG_VERSION.len();

const fn sdk_version() -> [u8; VERSION_SIZE] {
    let version_as_slice = PKG_VERSION.as_bytes();

    let mut version_as_array: [u8; VERSION_SIZE] = [0; VERSION_SIZE];
    let mut byte_id = 0;
    while byte_id < VERSION_SIZE {
        version_as_array[byte_id] = version_as_slice[byte_id];
        byte_id += 1;
    }

    version_as_array
}

#[cfg(target_arch = "wasm32")]
#[link_section = "__holium_sdk_version"]
#[doc(hidden)]
pub static __H_SDK_VERSION: [u8; VERSION_SIZE] = sdk_version();
