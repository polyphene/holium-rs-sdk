<h1 align="center">Welcome to holium rust SDK 👋</h1>

> Holium Rust SDK is a tool used to compile rust code to proper Holium transformation. It leverages procedural macro to do so.

### 🏠 [Homepage](https://holium.org/)

## Content

The project is divided in 4 main parts.

### Holium Rust SDK

Located in `./src`, the Holium Rust SDK is the crate that exposes the procdural macro to the rust code. It is also in
charge of exposing internal dependencies to ensure that the generated code works.

### Macro

Located in `./crates/macro`, the macro crate is a `proc-macro` crate that implements the procedural macro
used for compilation of transformations in the Holium Framework.

### Macro Support

Located in `./crates/macro-support`, the macro support crate is in charge of parsing `Item` objects that
are fetched by the procedural macro. This allows to convert the different elements to structures that 
can be manipulated in the backend.

### Backend

Located in `./crates/backend`, the backend crate contains all the logical sequence that generates necessary code for a 
transformation to run inside a Holium runtime.

## Usage

The Holium Rust SDK has to be used as a procedural macro. Here is an example of how it is used in source code.

```rust
use holium_rs_sdk::holium_bindgen;

#[holium_bindgen]
pub struct Values {
    pub a: u32,
    pub b: u32
}

#[holium_bindgen]
pub fn main(values: Values) -> u32 {
    values.a + values.b
}

```

## 🤝 Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/polyphene/holium-rust-sdk/issues).


***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_