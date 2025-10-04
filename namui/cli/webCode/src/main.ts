import wasmPath from "virtual:wasm";
import {
    WASI,
    ConsoleStdout,
    Fd,
    File,
    OpenFile,
} from "@bjorn3/browser_wasi_shim";
import { patchWasi } from "./patchWasi";
import { BincodeReader } from "./reader";
import { visitRenderingTree, Canvas, setCanvas } from "./draw";
import ThreadWorker from "./thread/ThreadWorker?worker";
import type { ThreadStartSupplies } from "./thread/threadMain";

console.log("Loading WASM from:", wasmPath);

// Memory를 전역으로 유지 (HMR 시에도 보존)
const memory = new WebAssembly.Memory({
    initial: 128,
    maximum: 16384,
    shared: true,
});

const tabId = crypto.randomUUID();
const nextTid = new SharedArrayBuffer(4);

let instance: WebAssembly.Instance;
let wasi: WASI;
let isFirstLoad = true;

// Canvas setup
const canvas = document.createElement("canvas");
canvas.width = 800;
canvas.height = 600;
document.body.appendChild(canvas);

const ctx = canvas.getContext("2d")!;
const canvasAdapter = new Canvas(ctx);
setCanvas(canvasAdapter);

async function loadWasm(path: string) {
    const module = await WebAssembly.compileStreaming(
        fetch(path + "?t=" + Date.now()),
    );

    const fd: Fd[] = [
        new OpenFile(new File([])), // stdin
        ConsoleStdout.lineBuffered((msg) => console.log(msg)),
        ConsoleStdout.lineBuffered((msg) => console.log(`[stderr] ${msg}`)),
    ];

    const env = ["RUST_BACKTRACE=full"];

    if (isFirstLoad) {
        // 첫 로드: WASI 생성
        wasi = new WASI([], env, fd);
        patchWasi(wasi);
    }

    const broadcastChannel = new BroadcastChannel(tabId);

    instance = await WebAssembly.instantiate(module, {
        env: {
            memory,
            _hardware_concurrency: () => navigator.hardwareConcurrency,
            _http_fetch_start: () => 0,
            _http_fetch_finish_request_body_stream: () => {},
            _http_fetch_error_on_rust_side: () => {},
        },
        wasi_snapshot_preview1: wasi.wasiImport,
        wasi: {
            "thread-spawn": (startArgPtr: number) => {
                const tid = Atomics.add(new Uint32Array(nextTid), 0, 1);
                const worker = new ThreadWorker();
                worker.postMessage({
                    memory,
                    module,
                    nextTid,
                    type: "thread",
                    startArgPtr,
                    tid,
                    tabId,
                } satisfies ThreadStartSupplies);
                return tid;
            },
        },
    });

    console.log("isFirstLoad", isFirstLoad);
    if (isFirstLoad) {
        // 첫 로드: wasi.start() 호출
        wasi.start(instance as any);
        isFirstLoad = false;
    } else {
        // HMR: wasi.start() 호출 안 함, instance만 교체
        wasi.inst = instance;
    }

    console.log("WASM loaded successfully");
    console.log("exports:", instance.exports);
}

// Rendering loop
function render() {
    if (!instance) {
        requestAnimationFrame(render);
        return;
    }

    const exports = instance.exports as any;

    // Call on_event to get rendering tree
    exports._on_event();
    const ptr = exports._get_last_rendering_tree_bytes_ptr();
    const len = exports._get_last_rendering_tree_bytes_len();

    // Parse rendering tree directly from WASM memory (no copy)
    const reader = new BincodeReader(memory.buffer, ptr, len);

    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Draw rendering tree
    try {
        visitRenderingTree(reader);
    } catch (e) {
        console.error("Error rendering:", e);
    }

    // Continue animation loop
    requestAnimationFrame(render);
}

// 초기 로드
await loadWasm(wasmPath);

// Start rendering
requestAnimationFrame(render);

// HMR 리스너
if (import.meta.hot) {
    import.meta.hot.on("wasm-update", async (data) => {
        console.log("🔄 WASM update detected, reloading module...");
        await loadWasm(data.path);
        console.log("✓ WASM module reloaded (memory preserved)");
    });
}
