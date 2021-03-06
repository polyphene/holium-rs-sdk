# Welcome to Holium Rust SDK 👋

[![crates.io](https://img.shields.io/crates/v/holium_rs_sdk.svg)](https://crates.io/crates/holium_rs_sdk)
[![dependency status](https://deps.rs/repo/github/polyphene/holium-rs-sdk/status.svg)](https://deps.rs/repo/github/polyphene/holium-rs-sdk)

[![GitHub latest commit](https://badgen.net/github/last-commit/polyphene/holium-rs-sdk/main)](https://github.com/polyphene/holium-rs-sdk/commit/)
[![GitHub issues](https://img.shields.io/github/issues/polyphene/holium-rs-sdk.svg)](https://github.com/polyphene/holium-rs-sdk/issues/)
[![GitHub pull-requests](https://img.shields.io/github/issues-pr/polyphene/holium-rs-sdk.svg)](https://github.com/polyphene/holium-rs-sdk/pull/)

[![maintainer](https://img.shields.io/badge/maintainer-Polyphene-blue)](https://twitter.com/polyphenehq/)
[![Discord](https://img.shields.io/discord/882061839347908678.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/A3t5ZFSbCG)
[![license](https://badgen.net/github/license/polyphene/holium-rs-sdk)](https://raw.githubusercontent.com/polyphene/holium-rs-sdk/main/LICENSE)

> Holium Rust SDK is a tool used to compile Rust code to proper Holium transformations. It leverages procedural macro to do so.


### 🏠 [Homepage](https://holium.org)

### 🗂 Content

The project is divided in 4 main parts.

#### Holium Rust SDK

Located in `./crates/sdk`, the Holium Rust SDK is the crate that exposes the procedural macro to the rust code. It is also in
charge of exposing internal dependencies to ensure that the generated code works.

#### Macro

Located in `./crates/macro`, the macro crate is a `proc-macro` crate that implements the procedural macro
used for compilation of transformations in the Holium Framework.

It is also in this crate that our tests on the procedural macro are conducted. Know more about our testing method 
[here](./crates/macro/tests/proc-macro-tests/README.md).

#### Macro Support

Located in `./crates/macro-support`, the macro support crate is in charge of parsing `Item` objects that
are fetched by the procedural macro. This allows to convert the different elements to structures that 
can be manipulated in the backend.

#### Backend

Located in `./crates/backend`, the backend crate contains all the logical sequence that generates necessary code for a 
transformation to run inside a Holium runtime.

### 📝 [Usage](https://docs.holium.org)

Be sure to check the [official documentation](https://docs.holium.org) to know better how to use the Holium Rust SDK, with practical examples.

The Holium Rust SDK has to be used as a procedural macro.
Here is an example of how it is used in source code.

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

### 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check the dedicated section in the documentation.

### 🙋 Show your support

Give a ⭐️ if this project helped you and use the official badge to link to the project!

[![Made with Holium](https://img.shields.io/badge/Made%20with-HOLIUM-AD6CD6?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABoAAAAaCAYAAACpSkzOAAAErklEQVR42q2VA7DsWhaGuzC2Z0rjmdKwMLbLGj+rXM+2fW0dPF7b1rFtq3WSDjt7Z+/vdZKqa9f9q/928i1lJXU1Kitr/uzxtrbPRe9Xn8h+KnU9tXB79vedw9ajYzP24aFJixOdDhWHPVl2yIveT+Vm7Rcty/riNQNe2Wj8dled2ZPO2fi+jxCiZEnnuGR7o2RfmyRtCIIgwPeKw9aof9f09PQ3rgry7hHj1vYhW7uuR7EoKMyECF+hlCKQipytKLiKMIwcIoMQu0uQb3fI5XKvXhFkcDz7v5mcUwIUkVJiTCjyIxqlNFprhKdJd2umOjXubPKdUopQKLxxiZW3ycxk37skxDCMb6RHPGu0UWDno0gVmV6NOakhemjI9kPje3B8MTS+TxyEVqeAcXAFw2O8Kr/qoqDcjDV8bImgeo2iMJ1EPNkGs2MJJAxg6AQcmQe7ny75GTgyH6Y6QJ0B6xoL2VgWsH+O8+fzIIVC4emu/aUfX1V07UlKZE7AZCslKLECB/oOwqE3YtApH3odptqJMwtDTW2/Ys7ykPKnxNHzSmYbvteyMaRjh2Z2VKMV2BliWCgBkoxG66BqRZTN2bBjC8EcBxlqqvo0S7aErJ0nGF5p//4UyLbtRY4hmO5UODmNCjUAKiQCniU7DT374ei8s2F7noXWTWDnNF0Tmi31ioZqSe6kN+8UyPf9diEkKlRxnS8pBYUp6D0AJ5fBvpcSyL4Xoa4cJlsgZ2p6JhWuL5kZtjtPgzw/nhYVKGQ+LtulpcEzkv517YbmtZGTTNM9IAKNDJMJzEwUKLun+bMJyPXT0Zehq/C6NWIa0FxWWoFvgzVd8hR4sxAKTk2fEAKzYLNx7sjXUpmOzCdt0yEGFRVmi8ZuA5lNTnT1OgWK15NZUirS8PD0NyzTiul2VjF+VJM9CU4dFPtIgMWrg0SWUsbbJZ/Pl8Wg1ytbP5HN5Y2Inh1UNFZqRnZAdheY26GwB5waEBNXVM6zNoTneZy1aNPp9NPRhrayYWm1aJpXw/hmSK+AzFLIlYG1H2Tu6kClcx5OnSvXdRuj8nXvVRydq+lfD2Ml0NRrMDMHZtdBsR/Ql4YImUxcdK5SRofOA5nj1stRVm5BUlupOVYJXVtgYBmkl8DsBvC7LwzSGiwvunY01X2KzrEQwxaYlvP6eaCRddYvCuMuca9GFAc2ahqPaQaOQnYrFPaB3wcoTkkpmDKgql+zrkazZJ9i3i7F0v2SjdUB1d3+hW+EmQbz0SgrKSRDA4raek3/iMYaALcFioOgBbE0MJLV7GzWLNqreW6j4qn1Ja8LeXKt4Jn1/jOpS8lxnB+WxjKdtyWNg4qucYVb1MiMJhjTKC8Z31Bp+qc12xsVc3YmgAQieeL9YE3qSuRm3Z+aprCbBiW1fSEZU8V7MPQUKkimKnIgFBP5kB2Nkpc2S558P+DxtcX5qavRRK/zo6rO4vDmWsGRTkHOkkh5ynEvS5kTlXp42mNTlTv54mb3H6lr1bpj1i0r9hrDa49bVHdZtA5aNPXbset7S+/7jM7WIfP2fvhw6nrokdWj33po9eT3Hy+b+fkD5Zl/Pbg6fdODZZnvpq5QHwDCaw/yZpQynwAAAABJRU5ErkJggg==)](https://github.com/polyphene/holium-rs)

### 📝 License

This project is [MIT](https://raw.githubusercontent.com/polyphene/holium-rs-sdk/main/LICENSE) licensed.


***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_