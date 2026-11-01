# Rust/WASM component size benchmark

Each package implements the same click counter: a button labeled `Clicks: 0` that increments when clicked. Kaja registers a `<kaja-counter>` custom element; Yew, Leptos, and Dioxus render their framework component into the document body.

## Build and measure

Requirements: Rust with the `wasm32-unknown-unknown` target, `wasm-pack`, and access to crates.io for dependencies.

```sh
sh wasm-benchmarks/build.sh
```

The script builds each package in release mode and prints the byte length of the wasm-bindgen-generated `*_bg.wasm` file. The workspace uses the same size-focused release profile for all packages. The reported byte count is the uncompressed file size; it does not include JavaScript glue or HTML and does not apply gzip/Brotli compression.

Each package has an `index.html` for manual browser testing after building. Serve a package directory over HTTP (for example, `python3 -m http.server 8000` from that directory) and open it in a browser.
