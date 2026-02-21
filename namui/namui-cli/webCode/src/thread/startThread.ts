import {
    ConsoleStdout,
    Fd,
    File,
    OpenFile,
    WASI,
} from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "@/imports/importObject";
import { Exports } from "@/exports";
import { patchWasi } from "@/patchWasi";
import { stdout } from "@/stdio";

export type ThreadStartSupplies = {
    nextTid: SharedArrayBuffer; // 4 bytes
    memory: WebAssembly.Memory;
    module: WebAssembly.Module;
    initialWindowWh: number;
} & (
    | {
          type: "main";
          imageInfoBytes: Uint8Array;
          imageCount: number;
          spawnPort: MessagePort;
      }
    | {
          type: "sub";
          startArgPtr: number;
          tid: number;
          imageInfoBytes: Uint8Array;
          imageCount: number;
          spawnPort: MessagePort;
      }
    | {
          type: "drawer";
          canvas: HTMLCanvasElement;
          imageCount: number;
      }
    | {
          type: "font-load";
      }
);

export async function startThread(supplies: ThreadStartSupplies) {
    const { module } = supplies;

    const env = [
        "RUST_BACKTRACE=full",
        `RAYON_NUM_THREADS=${navigator.hardwareConcurrency}`,
    ];

    const tid = supplies.type === "sub" ? supplies.tid : 0;

    const fd: Fd[] = [
        new OpenFile(new File([])), // stdin
        stdout(tid),
        ConsoleStdout.lineBuffered((msg) => console.warn(`[${tid}] ${msg}`)),
    ];
    const wasi = new WASI([], env, fd);
    patchWasi(wasi);

    const storageProtocolBuffer = new SharedArrayBuffer(32);

    const importObject = createImportObject({
        supplies,
        wasiImport: wasi.wasiImport,
        exports: () => exports,
        storageProtocolBuffer,
    });

    const instance = await WebAssembly.instantiate(module, importObject);
    const exports = instance.exports as Exports;

    wasi.initialize(instance as any);

    switch (supplies.type) {
        case "main":
            wasi.start(instance as any);
            break;
        case "sub":
            exports.wasi_thread_start(supplies.tid, supplies.startArgPtr);
            break;
        case "drawer":
            wasi.start(instance as any);
            break;
        case "font-load":
            break;
    }

    return instance;
}
