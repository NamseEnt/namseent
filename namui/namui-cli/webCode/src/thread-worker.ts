import { WASI } from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./imports/importObject";
import { getFds } from "./fds";
import { WorkerMessagePayload } from "./interWorkerProtocol";
import { Exports } from "./exports";
import { patchWasi } from "./patchWasi";

self.onmessage = (message) => {
    const payload: WorkerMessagePayload = message.data;
    startThreadWorker(payload);
};

async function startThreadWorker(payload: WorkerMessagePayload) {
    try {
        if (payload.type !== "thread-spawn") {
            throw new Error(`Unexpected message type: ${payload.type}`);
        }
        const {
            tid,
            nextTid,
            importMemory,
            module,
            startArgPtr,
            bundleSharedTree,
            eventBuffer,
            initialWindowWh,
        } = payload;

        const env = ["RUST_BACKTRACE=full"];
        const fds = getFds(bundleSharedTree);
        const wasi = new WASI([], env, fds);
        patchWasi(wasi);

        let exports: Exports = "not initialized" as unknown as Exports;

        const importObject = createImportObject({
            memory: importMemory,
            module,
            nextTid,
            wasiImport: wasi.wasiImport,
            bundleSharedTree,
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
    } catch (err) {
        console.error("thread error", err);
        debugger;
    }
}

self.onrejectionhandled = (event) => {
    console.error("unhandled rejection", event);
    debugger;
};
