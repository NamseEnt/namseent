import {
    ConsoleStdout,
    File,
    OpenFile,
    PreopenDirectory,
    WASI,
} from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./imports/importObject";
import {
    WorkerMessagePayload,
    sendMessageToMainThread,
} from "./interWorkerProtocol";
import { Exports } from "./exports";
import { patchWasi } from "./patchWasi";
import { stdout } from "./stdio";

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
        bundleSqlite,
    } = payload;

    const env = ["RUST_BACKTRACE=full"];

    const wasi = new WASI([], env, [
        new OpenFile(new File([])), // stdin
        stdout(tid),
        ConsoleStdout.lineBuffered((msg) =>
            console.warn(`[${tid}] ${msg}`),
        ),
        new PreopenDirectory(
            ".",
            new Map([
                [
                    "bundle.sqlite",
                    new File(new Uint8Array(bundleSqlite), { readonly: true }),
                ],
            ]),
        ),
    ]);
    patchWasi(wasi);

    let exports: Exports = "not initialized" as unknown as Exports;

    const storageProtocolBuffer = new SharedArrayBuffer(32);
    sendMessageToMainThread({
        type: "storage-thread-connect",
        threadId: tid,
        protocolBuffer: storageProtocolBuffer,
    });

    const importObject = createImportObject({
        memory: wasmMemory,
        module,
        nextTid,
        wasiImport: wasi.wasiImport,
        eventBuffer,
        initialWindowWh,
        exports: () => exports,
        bundleSqlite: () => bundleSqlite,
        storageProtocolBuffer,
    });

    const instance = await WebAssembly.instantiate(module, importObject);
    exports = instance.exports as Exports;

    wasi.initialize(instance as any);
    console.debug("thread start", tid);
    (instance.exports.wasi_thread_start as any)(tid, startArgPtr);
    console.debug("thread end", tid);

    sendMessageToMainThread({
        type: "storage-thread-disconnect",
        threadId: tid,
    });
};
