import {
    ConsoleStdout,
    Fd,
    File,
    OpenFile,
    WASI,
} from "@bjorn3/browser_wasi_shim";
import "./style.css";
import wasmUrl from "/bundle.wasm?url";
import { patchWasi } from "./patchWasi";
import { threadMain } from "./thread/threadMain";
import { BincodeReader } from "./reader";
import { visitRenderingTree, Canvas, setCanvas } from "./draw";

const module = await WebAssembly.compileStreaming(fetch(wasmUrl));

const fd: Fd[] = [
    new OpenFile(new File([])), // stdin
    ConsoleStdout.lineBuffered((msg) => console.log(msg)),
    ConsoleStdout.lineBuffered((msg) => console.log(`[stderr] ${msg}`)),
];

const env = ["RUST_BACKTRACE=full"];

const wasi = new WASI([], env, fd);
patchWasi(wasi);

const memory = new WebAssembly.Memory({
    initial: 128,
    maximum: 16384,
    shared: true,
});

const tabId = crypto.randomUUID();

const instance = await threadMain({
    memory,
    module,
    nextTid: new SharedArrayBuffer(4),
    type: "main",
    tabId,
});

console.log("exports", instance.exports);

// Canvas setup
const canvas = document.createElement("canvas");
canvas.width = 800;
canvas.height = 600;
document.body.appendChild(canvas);

const ctx = canvas.getContext("2d")!;

// Set global canvas for draw.ts
const canvasAdapter = new Canvas(ctx);
setCanvas(canvasAdapter);

// const app = document.getElementById("app")!;
// app.innerHTML = `
//     module: ${module}
//     instance: ${instance}
// `;

const broadcastChannel = new BroadcastChannel(tabId);

broadcastChannel.onmessage = async (event) => {
    if (event.data.tabId !== tabId) {
        return;
    }
    const exports = instance.exports as any;

    const data = event.data as BroadcastChannelMessageEvent;
    switch (data.type) {
        case "fetch":
            const response = await fetch(data.uri, {
                method: data.method,
                headers: data.headers,
            });
            // TODO: Directly write to memory
            const body = await response.arrayBuffer();
            const bodyPtr = exports.malloc(body.byteLength);
            new Uint8Array(memory.buffer).set(new Uint8Array(body), bodyPtr);

            // headers를 직렬화: key-byte-len, key bytes, value-byte-len, value bytes 형식
            const textEncoder = new TextEncoder();
            const headerEntries = Array.from(response.headers.entries());

            // 필요한 총 메모리 크기 계산
            let headersByteLength = 0;
            const encodedHeaders: Array<{
                key: Uint8Array;
                value: Uint8Array;
            }> = [];
            for (const [key, value] of headerEntries) {
                const encodedKey = textEncoder.encode(key);
                const encodedValue = textEncoder.encode(value);
                encodedHeaders.push({ key: encodedKey, value: encodedValue });
                headersByteLength +=
                    4 + encodedKey.byteLength + 4 + encodedValue.byteLength;
            }

            // 메모리 할당 및 데이터 쓰기
            const headersPtr = exports.malloc(headersByteLength);
            const memoryView = new Uint8Array(memory.buffer);
            const dataView = new DataView(memory.buffer);
            let offset = headersPtr;

            for (const { key, value } of encodedHeaders) {
                // key length (4 bytes, little-endian)
                dataView.setUint32(offset, key.byteLength, true);
                offset += 4;
                // key bytes
                memoryView.set(key, offset);
                offset += key.byteLength;
                // value length (4 bytes, little-endian)
                dataView.setUint32(offset, value.byteLength, true);
                offset += 4;
                // value bytes
                memoryView.set(value, offset);
                offset += value.byteLength;
            }

            exports._http_fetch_response(
                data.fetchId,
                response.status,
                headersPtr,
                headersByteLength,
                bodyPtr,
                body.byteLength,
            );
            break;
    }
};

type BroadcastChannelMessageEvent = {
    tabId: string;
} & {
    type: "fetch";
    fetchId: number;
    uri: string;
    method: string;
    headers: Record<string, string>;
};

// Animation loop with on_event
function render() {
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

    // ctx.fillText("hello", 10, 10);
}

// Start animation loop
requestAnimationFrame(render);
