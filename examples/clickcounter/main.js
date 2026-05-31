let wasmModule;

async function loadWasm() {
  try {
    const wasmModule = await import("./clickcounter.js");
    await wasmModule.default("./clickcounter_bg.wasm");
    await wasmModule.initCallbacks();
    return wasmModule;
  } catch (error) {
    console.error("Failed to load WASM module:", error);
    return null;
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  wasmModule = await loadWasm();

  window.realjs = function () {
    console.log("RealJS");

    wasmModule.test();
    //wasmModule.incrementCounter();
  };

  window.runTest = wasmModule.test;
});
