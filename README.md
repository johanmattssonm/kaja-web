# Web Framework: kaja-web - for Web Applications written in Rust

The key concept and idea behind the design is that HTML content is managed as
strings and the code is compiled to WebAssembly. There is no Domain Specific
Language, just plain Rust with a little bit of glue code that calls the WebAssembly
functions in onclick handlers (and also other event handlers). The HTML code is
generated using the `kaja_html_macro` crate.

There are also a number of helper functions available to make it easier to interact
with the browser.

## Example:
```rust
use kaja_web::prelude::*;

#[callback(incrementClickCounter)]
fn increment_click_counter() {
    // update the click counter here
}

fn main() {
    // make sure functions annotated with `#[callback]` are available as global functions on the DOM
    init_callbacks(); 

    let content = html! {{
        <div class="click-count"></div>
        <button onclick="incrementClickCounter">Click me</button>
    }};

    inner_html(".main-content", content);
}

// This example requres an html file with this content:
// <div class="main-content"></div>
```

For more example see: the crates `kaja-html-macro` and `kaja-callback-macro` or
look in the examples directory.

## Functions Needed
Add these crates to your project's `Cargo.toml` file:

```toml
[dependencies]
gloo = "0.12.0"
gloo-console = "0.4.0"
kaja-html-macro = "1.0.3"
kaja-callback-macro = { version = "0.1.0" }
wasm-bindgen = "0.2.122"
web-sys = { version = "0.3.99", features = ["Document", "HtmlDocument", "Element", "HtmlElement", "HtmlInputElement", "NodeList"] }
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.5"
inventory = "0.3.24"
js-sys = "0.3"
wasm-bindgen-futures = "0.4.72"
```

## Author and Contact
- Written by Johan Mattsson
- johan.mattsson.m@gmail.com
- https://kajacode.com
