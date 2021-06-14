<div align="center">

  <h1><code>holium-rust-sdk</code></h1>

<strong>Repository containing the Holium SDK to build data transformation in Rust<strong>
</div>

## About

This repository contains the source code of the software development kit meant to develop data transformation for the
holium protcol.

Its main component is a procedural macro that is in charge of generating bindings for the transformation to work in a 
Holium runtime. The procedural macro will also generate a transformation file containing transformation metadata.&

## 🚴 Usage

### Utilities

The SDK expose a procedural macro, `holium_bindgen`, that can be used to expose functions to the Holium protocol form
a wasm runtime. The procedural macro is in charge of generating a wrapper around all exposed functions so that functions
argument and return values are retrieved and written on the host running the transformation.
<!-- TODO: complete with example of proc macro application, generated code & file  -->
