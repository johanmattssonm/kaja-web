const FRAMEWORKS = [
    { id: "kaja", name: "Kaja" },
    { id: "yew", name: "Yew" },
    { id: "leptos", name: "Leptos" },
    { id: "dioxus", name: "Dioxus" },
];
const WARMUP_RUNS = 1;
const MEASURED_RUNS = 5;
const READY_TIMEOUT_MS = 30_000;

const form = document.getElementById("benchmark-form");
const frameworkSelect = document.getElementById("framework");
const clickCountInput = document.getElementById("click-count");
const runButton = document.getElementById("run-benchmark");
const status = document.getElementById("status");
const results = document.getElementById("results");
const frame = document.getElementById("benchmark-frame");
let navigationId = 0;

function nextFrame() {
    return new Promise((resolve) => requestAnimationFrame(resolve));
}

function median(values) {
    const sorted = [...values].sort((a, b) => a - b);
    return sorted[Math.floor(sorted.length / 2)];
}

function formatMs(value) {
    return value.toFixed(2);
}

function appendResult(name, startupSamples, interactionSamples, clickCount) {
    const row = document.createElement("tr");
    const startupMs = median(startupSamples);
    const interactionMs = median(interactionSamples);
    const perClickMicroseconds = (interactionMs * 1000) / clickCount;

    for (const value of [
        name,
        formatMs(startupMs),
        formatMs(interactionMs),
        perClickMicroseconds.toFixed(2),
    ]) {
        const cell = document.createElement("td");
        cell.textContent = value;
        row.append(cell);
    }

    results.append(row);
}

function loadFramework(framework) {
    return new Promise((resolve, reject) => {
        const startedAt = performance.now();
        const timeout = window.setTimeout(() => {
            window.removeEventListener("message", onMessage);
            reject(new Error(`${framework.name} did not signal that its counter rendered`));
        }, READY_TIMEOUT_MS);

        function onMessage(event) {
            if (
                event.source !== frame.contentWindow ||
                event.origin !== window.location.origin ||
                event.data?.type !== "benchmark:ready"
            ) {
                return;
            }

            window.clearTimeout(timeout);
            window.removeEventListener("message", onMessage);

            const button = frame.contentDocument?.getElementById("counter");
            if (!button || button.textContent.trim() !== "Clicks: 0") {
                reject(new Error(`${framework.name} does not match the counter adapter contract`));
                return;
            }

            resolve({ button, startupMs: performance.now() - startedAt });
        }

        window.addEventListener("message", onMessage);
        const url = new URL(`./${framework.id}/index.html`, window.location.href);
        url.searchParams.set("benchmark_run", String(++navigationId));
        frame.src = url.href;
    });
}

async function measureClickBatch(button, clickCount) {
    const startedAt = performance.now();

    for (let click = 0; click < clickCount; click += 1) {
        button.click();
    }

    const expectedText = `Clicks: ${clickCount}`;
    const deadline = performance.now() + READY_TIMEOUT_MS;
    while (button.textContent.trim() !== expectedText) {
        if (performance.now() > deadline) {
            throw new Error(`Counter did not reach ${expectedText}`);
        }

        await nextFrame();
    }

    return performance.now() - startedAt;
}

async function benchmarkFramework(framework, clickCount) {
    for (let run = 0; run < WARMUP_RUNS; run += 1) {
        status.textContent = `Warming up ${framework.name}…`;
        const warmup = await loadFramework(framework);
        await measureClickBatch(warmup.button, clickCount);
    }

    const startupSamples = [];
    const interactionSamples = [];

    for (let run = 0; run < MEASURED_RUNS; run += 1) {
        status.textContent = `Measuring ${framework.name}: run ${run + 1} of ${MEASURED_RUNS}…`;
        const { button, startupMs } = await loadFramework(framework);
        const interactionMs = await measureClickBatch(button, clickCount);
        startupSamples.push(startupMs);
        interactionSamples.push(interactionMs);
    }

    appendResult(framework.name, startupSamples, interactionSamples, clickCount);
}

form.addEventListener("submit", async (event) => {
    event.preventDefault();
    runButton.disabled = true;
    results.replaceChildren();

    const clickCount = Number.parseInt(clickCountInput.value, 10);
    const selected = frameworkSelect.value;
    const frameworks = selected === "all"
        ? FRAMEWORKS
        : FRAMEWORKS.filter((framework) => framework.id === selected);

    try {
        for (const framework of frameworks) {
            await benchmarkFramework(framework, clickCount);
        }
        status.textContent = `Done. Each framework used ${WARMUP_RUNS} warmup run and ${MEASURED_RUNS} measured runs.`;
    } catch (error) {
        status.textContent = `Benchmark failed: ${error.message}`;
    } finally {
        runButton.disabled = false;
    }
});
