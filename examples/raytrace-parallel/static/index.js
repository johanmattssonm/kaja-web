function loadWasm() {
  if (typeof SharedArrayBuffer !== 'function') {
    console.error('this browser does not have SharedArrayBuffer support enabled' + '\n\n' + msg);
    return
  }
  // Test for bulk memory operations with passive data segments
  //  (module (memory 1) (data passive ""))
  const buf = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    0x05, 0x03, 0x01, 0x00, 0x01, 0x0b, 0x03, 0x01, 0x01, 0x00]);
  if (!WebAssembly.validate(buf)) {
    console.error('This browser does not support passive Wasm memory, Background threads will not work' + '\n\n' + msg);
    return
  }

  wasm_bindgen()
    .catch(console.error);
}

loadWasm();
