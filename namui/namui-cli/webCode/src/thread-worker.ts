import { WASI } from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./imports/importObject";
import {
    WorkerMessagePayload,
    sendMessageToMainThread,
} from "./interWorkerProtocol";
import { Exports } from "./exports";
import { patchWasi } from "./patchWasi";
import { overrideWasiFs } from "./fileSystem";

self.onmessage = async (message) => {
    const payload: WorkerMessagePayload = message.data;

    if (payload.type !== "thread-spawn") {
        throw new Error(`Unexpected message type: ${payload.type}`);
    }
    const {
        tid,
        nextTid,
        wasmMemory,
        module,
        startArgPtr,
        eventBuffer,
        initialWindowWh,
    } = payload;

    const env = ["RUST_BACKTRACE=full"];
    const wasi = new WASI([], env, []);
    patchWasi(wasi);
    overrideWasiFs({ wasi, threadId: tid });

    let exports: Exports = "not initialized" as unknown as Exports;

    const importObject = createImportObject({
        memory: wasmMemory,
        module,
        nextTid,
        wasiImport: wasi.wasiImport,
        eventBuffer,
        initialWindowWh,
        exports: () => exports,
    });

    const instance = await WebAssembly.instantiate(module, importObject);
    exports = instance.exports as Exports;

    wasi.initialize(instance as any);
    console.debug("thread start", tid);
    (instance.exports.wasi_thread_start as any)(tid, startArgPtr);
    console.debug("thread end", tid);

    sendMessageToMainThread({
        type: "fs-thread-disconnect",
        threadId: tid,
    });
};
