# Rust/WASM component benchmarks

Each package implements the same counter: a button labeled `Clicks: 0` that increments once for every click. Kaja registers a `<kaja-counter>` custom element; Yew, Leptos, and Dioxus render their component into the document body.

## Build and measure wasm size

Requirements: Rust with the `wasm32-unknown-unknown` target, `wasm-pack`, and access to crates.io for dependencies.

From this directory, build all four packages and print the size of each generated WebAssembly module:

```sh
sh build.sh
```

This reports uncompressed `*_bg.wasm` byte counts. It excludes JavaScript glue and HTML and does not apply gzip or Brotli compression.

## Run the speed benchmark

Build the packages first, then serve this directory over HTTP (the runner and adapters must share an origin):

```sh
python3 -m http.server 8000
```

Open <http://localhost:8000/> in a browser and select **Run benchmark**. The runner performs one warmup and five measured runs for each selected framework. Each measured run reloads the app, records time from navigation until the counter is mounted, dispatches the same number of synthetic button clicks, and waits until the DOM shows the final count. The table reports median startup time, median click-batch time, and median time per click.

The startup measurement includes page navigation and WebAssembly download/initialization, so later runs generally benefit from the browser cache. Click-batch time measures event handling through the DOM update; it does not measure paint time. Results depend on the browser, machine load, and browser cache state. Compare frameworks in the same browser session and treat small differences cautiously.

## Adding or changing an adapter

The shared runner in `runner.js` relies on a small DOM contract: each app renders a button with `id="counter"`, initially showing `Clicks: 0`, and each click updates it to `Clicks: N`. After its WASM module initializes, the page calls `signalBenchmarkReady()` from `adapter.js`; that helper waits for the button to render and notifies the runner. Keep the rendered markup and update behavior equivalent across adapters so the results stay comparable.

The individual `index.html` files can also be opened directly for manual testing after the packages have been built.
