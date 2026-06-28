let wasmModule;

async function loadWasm() {
    try {
        const wasmModule = await import("./componentexample.js");
        await wasmModule.default("./componentexample_bg.wasm");

        return wasmModule;
    } catch (error) {
        console.error("Failed to load WASM module:", error);
    }

    return null;
}

window.addEventListener("DOMContentLoaded", async () => {
    wasmModule = await loadWasm();
});
