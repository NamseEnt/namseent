import {
    ConsoleStdout,
    Fd,
    File,
    OpenFile,
    PreopenDirectory,
    WASI,
} from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./imports/importObject";
import wasmUrl from "namui-runtime-wasm.wasm?url";
import { sendMessageToMainThread, WorkerMessagePayload } from "./interWorkerProtocol";
import { Exports } from "./exports";
import { patchWasi } from "./patchWasi";
import bundleSqliteUrl from "bundle.sqlite?url";
import { stdout } from "./stdio";

console.debug("crossOriginIsolated", crossOriginIsolated);

if (!crossOriginIsolated) {
    throw new Error("Not cross-origin isolated");
}

const env = ["RUST_BACKTRACE=full"];

const threadId = 0;

const nextTid = new SharedArrayBuffer(4);
new Uint32Array(nextTid)[0] = 1;

self.onmessage = async (message) => {
    const payload: WorkerMessagePayload = message.data;

    if (payload.type !== "start-main-thread") {
        throw new Error(`Unexpected message type: ${payload.type}`);
    }

    const fd: Fd[] = [
        new OpenFile(new File([])), // stdin
        stdout(threadId),
        ConsoleStdout.lineBuffered((msg) =>
            console.warn(`[${threadId}] ${msg}`),
        ),
    ];
    const wasi = new WASI([], env, fd);
    patchWasi(wasi);

    const [instance, bundleSqlite] = await Promise.all([
        (async () => {
            const { eventBuffer, initialWindowWh, wasmMemory } = payload;

            const canvas = new OffscreenCanvas(
                (initialWindowWh >> 16) & 0xffff,
                initialWindowWh & 0xffff,
            );

            const module = await WebAssembly.compileStreaming(fetch(wasmUrl));

            let exports: Exports = "not initialized" as unknown as Exports;

            const storageProtocolBuffer = new SharedArrayBuffer(32);
            sendMessageToMainThread({
                type: "storage-thread-connect",
                threadId,
                protocolBuffer: storageProtocolBuffer,
            });

            const importObject = createImportObject({
                memory: wasmMemory,
                module,
                nextTid,
                wasiImport: wasi.wasiImport,
                canvas,
                eventBuffer,
                initialWindowWh,
                exports: () => exports,
                bundleSqlite: () => bundleSqlite,
                storageProtocolBuffer,
            });

            const instance = await WebAssembly.instantiate(
                module,
                importObject,
            );
            exports = instance.exports as Exports;
            return instance;
        })(),
        fetch(bundleSqliteUrl).then((res) => res.arrayBuffer()),
    ]);

    fd.push(
        new PreopenDirectory(
            ".",
            new Map([
                [
                    "bundle.sqlite",
                    new File(new Uint8Array(bundleSqlite), { readonly: true }),
                ],
            ]),
        ),
    );

    wasi.start(instance as any);

    sendMessageToMainThread({
        type: "storage-thread-disconnect",
        threadId,
    });
};
