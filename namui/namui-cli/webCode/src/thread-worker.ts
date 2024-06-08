import { WASI } from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./imports/importObject";
import { getFds } from "./fds";
import { WorkerMessagePayload } from "./interWorkerProtocol";

self.onmessage = async (message) => {
    const payload: WorkerMessagePayload = message.data;

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

    let exports: Record<string, Function> = {};

    const importObject = createImportObject({
        memory: importMemory,
        module,
        nextTid,
        wasiImport: wasi.wasiImport,
        malloc: (size: number) => {
            return exports._malloc(size);
        },
        free: (ptr: number) => {
            return exports._free(ptr);
        },
        bundleSharedTree,
        eventBuffer,
        initialWindowWh,
    });

    const instance = await WebAssembly.instantiate(module, importObject);
    exports = instance.exports as Record<string, Function>;

    wasi.initialize(instance as any);
    console.debug("thread start", tid);
    (instance.exports.wasi_thread_start as any)(tid, startArgPtr);
    console.debug("thread end", tid);
};
