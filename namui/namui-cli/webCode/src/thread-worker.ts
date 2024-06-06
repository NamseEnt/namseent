import { WASI } from "./wasi_shim";
import { createImportObject } from "./importObject";
import { BundleSharedTree, getFds } from "./fds";

self.onmessage = async (message) => {
    const {
        tid,
        nextTid,
        importMemory,
        module,
        startArgPtr,
        bundleSharedTree,
        eventBuffer,
    } = message.data as {
        tid: number;
        nextTid: SharedArrayBuffer;
        importMemory: WebAssembly.Memory;
        module: WebAssembly.Module;
        startArgPtr: number;
        bundleSharedTree: BundleSharedTree;
        eventBuffer: SharedArrayBuffer;
    };

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
    });

    const instance = await WebAssembly.instantiate(module, importObject);
    exports = instance.exports as Record<string, Function>;

    wasi.initialize(instance as any);
    console.debug("thread start", tid);
    (instance.exports.wasi_thread_start as any)(tid, startArgPtr);
    console.debug("thread end", tid);
};
