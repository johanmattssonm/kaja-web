const READY_TIMEOUT_MS = 30_000;

export async function signalBenchmarkReady() {
    const startedAt = performance.now();

    while (!document.getElementById("counter")) {
        if (performance.now() - startedAt > READY_TIMEOUT_MS) {
            throw new Error("Counter benchmark did not render within 30 seconds");
        }

        await new Promise((resolve) => requestAnimationFrame(resolve));
    }

    await new Promise((resolve) => requestAnimationFrame(resolve));
    window.parent.postMessage({ type: "benchmark:ready" }, window.location.origin);
}
