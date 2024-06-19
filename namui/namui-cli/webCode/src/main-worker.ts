import { WASI } from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./imports/importObject";
import wasmUrl from "namui-runtime-wasm.wasm?url";
import { init } from "./__generated__/bundle";
import { getFds } from "./fds";
import { WorkerMessagePayload } from "./interWorkerProtocol";
import { Exports } from "./exports";

console.debug("crossOriginIsolated", crossOriginIsolated);

const env = ["RUST_BACKTRACE=full"];

const memory = new WebAssembly.Memory({
    initial: 128,
    maximum: 16384,
    shared: true,
});

const nextTid = new SharedArrayBuffer(4);
new Uint32Array(nextTid)[0] = 1;

self.onmessage = async (message) => {
    const payload: WorkerMessagePayload = message.data;

    if (payload.type !== "start-main-thread") {
        throw new Error(`Unexpected message type: ${payload.type}`);
    }

    const bundleSharedTree = await init();

    const fds = getFds(bundleSharedTree);
    const wasi = new WASI([], env, fds);

    const { eventBuffer, initialWindowWh } = payload;

    const canvas = new OffscreenCanvas(
        (initialWindowWh >> 16) & 0xffff,
        initialWindowWh & 0xffff,
    );

    const module = await WebAssembly.compileStreaming(fetch(wasmUrl));

    let exports: Exports = "not initialized" as unknown as Exports;

    const importObject = createImportObject({
        memory,
        module,
        nextTid,
        wasiImport: wasi.wasiImport,
        canvas,
        bundleSharedTree,
        eventBuffer,
        initialWindowWh,
        exports: () => exports,
    });

    const instance = await WebAssembly.instantiate(module, importObject);
    exports = instance.exports as Exports;

    wasi.start(instance as any);
};
