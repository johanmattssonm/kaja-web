# Example

Run build.sh to build the example.

This example requires the nightly version of Rust. See rust-toolchain.toml

The more common way to do parallel work in the browser is to use async/await
style code with

```
use wasm_bindgen_futures::spawn_local;

spawn_local(async {
    // your async work here
});
```

See the async example.

The code is based on the raytrace example in the wasm-bindgen crate. License: Apache-2.0. Original author: Alex Crichton et. al.
