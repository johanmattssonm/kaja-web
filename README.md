# kaja-web – A Framework for Web Applications Written in Rust

The key concept and idea behind the design is that HTML content is managed as
strings and the code is compiled to WebAssembly. There is no Domain Specific
Language, just plain Rust with a little bit of glue code that calls the
WebAssembly functions in onclick handlers (and also other event handlers).

There are also a number of helper functions available to make it easier to
interact with the browser.

No state changes triggers re-render automatically, run your update function 
manually when you want to render a component and use 
`inner_html("#the-id", content)` to update the  DOM.

## Example:
```rust
use kaja_web::prelude::*;
use std::sync::RwLock;

struct AppData {
    current_count: i32,
}

pub static APP_DATA: RwLock<AppData> = RwLock::new(AppData { current_count: 0 });

#[callback(incrementClickCounter)]
fn increment_click_counter() {
    let mut app_data = APP_DATA.write().unwrap();
    app_data.current_count += 1;
    update_count(&app_data);
}

fn update_counter_display(app_data: &AppData) {
    let content = html! {{
        <div class="click-count">$(app_data.current_count)</div>
        <button onclick="incrementClickCounter()">Click me</button>
    }};

    try_inner_html(".main-content", content);
}

fn update_count(app_data: &AppData) {
    let count = html! {{
        $(app_data.current_count)
    }};

    try_inner_html(".click-count", count);
}

fn main() {
    init_callbacks(); 

    let mut app_data = APP_DATA.read().unwrap();
    update_counter_display(&app_data);
}
```

This example requres an html file with this content:
```html
<div class="main-content"></div>
```

See examples/clickcounter for a full example.

# Passing Objects from JS Callbacks to Rust WASM

This requires `serde::Deserialize;`.

```rust
use kaja_web::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TestData {
    test_parameter: String,
}

fn update_component() {
    let content = html! {{
        <button onclick="foo({ test_parameter: 'test' })">Click me</button>
    }};

    inner_html(".main-content", content);
}

#[callback(foo)]
fn foo(data: TestData) {
    log!("test_parameter: {}", data.test_parameter);
}
```

## The Macros
The HTML code is generated using the `kaja-html-macro` crate. The callback
functions are definedusing the `kaja-callback-macro` crate. Helper functions
are added in this crate and reexportedfrom the `kaja_web::prelude` module.

## Building WASM Packages

```bash
cargo build

wasm-pack build \
    --release \
    --target web \
    --out-dir pkg \
    --out-name main \
    --no-typescript

cp index.html pkg/
cp loader.js pkg/
cp styles.css pkg/
```

## Loading the WASM Package with JavaScript

See examples for complete examples with WebAssembly, html and JavaScript.

```javascript
let wasmModule;

async function loadWasm() {
    try {
        const wasmModule = await import("./main.js");
        await wasmModule.default("./main_bg.wasm");

        return wasmModule;
    } catch (error) {
        console.error("Failed to load WASM module:", error);
    }

    return null;
}

window.addEventListener("DOMContentLoaded", async () => {
    wasmModule = await loadWasm();
});
```

## Home Page
https://kajacode.com/kajaweb.html

## Author and Contact
- Written by Johan Mattsson
- johan.mattsson.m@gmail.com
- https://kajacode.com
