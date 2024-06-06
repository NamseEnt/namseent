import { WASI } from "./wasi_shim";
import { createImportObject } from "./importObject";
import wasmUrl from "namui-runtime-wasm.wasm?url";
import { init } from "./__generated__/bundle";
import { getFds } from "./fds";

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
    const bundleSharedTree = await init();

    const fds = getFds(bundleSharedTree);
    const wasi = new WASI([], env, fds);

    const { canvas, eventBuffer } = message.data as {
        canvas: OffscreenCanvas;
        eventBuffer: SharedArrayBuffer;
    };
    const webgl = canvas.getContext("webgl2")!;

    const module = await WebAssembly.compileStreaming(fetch(wasmUrl));

    let exports: {
        malloc: (size: number) => number;
        free: (ptr: number) => void;
        memory: WebAssembly.Memory;
    } = {} as any;

    const importObject = createImportObject({
        memory,
        module,
        nextTid,
        wasiImport: wasi.wasiImport,
        malloc: (size: number) => {
            return exports.malloc(size);
        },
        free: (ptr: number) => {
            return exports.free(ptr);
        },
        webgl,
        bundleSharedTree,
        eventBuffer,
    });

    const instance = await WebAssembly.instantiate(module, importObject);
    exports = instance.exports as any;

    wasi.start(instance as any);
};
