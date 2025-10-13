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
      }
    | {
          type: "sub";
          startArgPtr: number;
          tid: number;
      }
    | {
          type: "drawer";
      }
    | {
          type: "drawer-sub";
          startArgPtr: number;
          tid: number;
      }
);

export async function startThread(supplies: ThreadStartSupplies) {
    const { module } = supplies;

    const env = ["RUST_BACKTRACE=full"];

    const tid = supplies.type === "sub" ? supplies.tid : 0;

    const nextTid = new SharedArrayBuffer(4);
    new Uint32Array(nextTid)[0] = 1;

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
            break;
        case "sub":
            exports.wasi_thread_start(supplies.tid, supplies.startArgPtr);
            break;
        case "drawer":
            break;
    }

    return instance;
}
