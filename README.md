# Web Framework: kaja-web - for Web Applications written in Rust

The key concept and idea behind the design is that HTML content is managed as
strings and the code is compiled to WebAssembly. There is no Domain Specific
Language, just plain Rust with a little bit of glue code that calls the WebAssembly
functions in onclick handlers (and also other event handlers).

There are also a number of helper functions available to make it easier to interact
with the browser.

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
    update_counter_display(&app_data);
}

fn update_counter_display(app_data: &AppData) {
    let content = html! {{
        <div class="click-count">$(app_data.current_count)</div>
        <button onclick="incrementClickCounter()">Click me</button>
    }};

    inner_html(".main-content", content);
}

fn main() {
    // Init callbacks to make sure functions annotated with `#[callback]`
    // are available as global functions on the DOM
    init_callbacks(); 

    let mut app_data = APP_DATA.write().unwrap();
    update_counter_display(&app_data);
}
```

This example requres an html file with this content:
```html
<div class="main-content"></div>
```

See examples/clickcounter for full example.

## The Macros
The HTML code is generated using the `kaja-html-macro` crate. The callback
functions are definedusing the `kaja-callback-macro` crate. Helper functions
are added in this crate and reexportedfrom the `kaja_web::prelude` module.

## Author and Contact
- Written by Johan Mattsson
- johan.mattsson.m@gmail.com
- https://kajacode.com
