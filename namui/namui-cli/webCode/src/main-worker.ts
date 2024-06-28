import { WASI } from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./imports/importObject";
import wasmUrl from "namui-runtime-wasm.wasm?url";
import {
    WorkerMessagePayload,
    sendMessageToMainThread,
} from "./interWorkerProtocol";
import { Exports } from "./exports";
import { patchWasi } from "./patchWasi";
import { overrideWasiFs } from "./fileSystem";

console.debug("crossOriginIsolated", crossOriginIsolated);

const env = ["RUST_BACKTRACE=full"];

const threadId = 0;

const nextTid = new SharedArrayBuffer(4);
new Uint32Array(nextTid)[0] = 1;

self.onmessage = async (message) => {
    const payload: WorkerMessagePayload = message.data;

    if (payload.type !== "start-main-thread") {
        throw new Error(`Unexpected message type: ${payload.type}`);
    }

    const wasi = new WASI([], env, []);
    patchWasi(wasi);
    overrideWasiFs({ wasi, threadId });

    const { eventBuffer, initialWindowWh, wasmMemory } = payload;

    const canvas = new OffscreenCanvas(
        (initialWindowWh >> 16) & 0xffff,
        initialWindowWh & 0xffff,
    );

    const module = await WebAssembly.compileStreaming(fetch(wasmUrl));

    let exports: Exports = "not initialized" as unknown as Exports;

    const importObject = createImportObject({
        memory: wasmMemory,
        module,
        nextTid,
        wasiImport: wasi.wasiImport,
        canvas,
        eventBuffer,
        initialWindowWh,
        exports: () => exports,
    });

    const instance = await WebAssembly.instantiate(module, importObject);
    exports = instance.exports as Exports;

    wasi.start(instance as any);

    sendMessageToMainThread({
        type: "fs-thread-disconnect",
        threadId,
    });
};
