pub mod proc_macro {
    pub use holium_macro::holium_bindgen;
}

pub mod data;
pub mod interface;
pub use serde::{Deserialize, Serialize};