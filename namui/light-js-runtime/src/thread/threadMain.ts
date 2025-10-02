import {
    ConsoleStdout,
    File,
    OpenFile,
    WASI,
    type Fd,
} from "@bjorn3/browser_wasi_shim";
import { patchWasi } from "../patchWasi";
import ThreadWorker from "./ThreadWorker?worker";

export type ThreadStartSupplies = {
    nextTid: SharedArrayBuffer; // 4 bytes
    memory: WebAssembly.Memory;
    module: WebAssembly.Module;
} & (
    | {
          type: "main";
      }
    | {
          type: "spawn";
          startArgPtr: number;
          tid: number;
      }
);

export async function threadMain(supplies: ThreadStartSupplies) {
    const { memory, module } = supplies;
    const fd: Fd[] = [
        new OpenFile(new File([])), // stdin
        ConsoleStdout.lineBuffered((msg) => console.log(msg)),
        ConsoleStdout.lineBuffered((msg) => console.log(`[stderr] ${msg}`)),
    ];

    const env = ["RUST_BACKTRACE=full"];

    const wasi = new WASI([], env, fd);
    patchWasi(wasi);

    const instance = await WebAssembly.instantiate(module, {
        env: {
            memory,
            _hardware_concurrency: () => navigator.hardwareConcurrency,
        },
        wasi_snapshot_preview1: wasi.wasiImport,
        wasi: {
            "thread-spawn": (startArgPtr: number) => {
                const tid = Atomics.add(
                    new Uint32Array(supplies.nextTid),
                    0,
                    1,
                );
                const worker = new ThreadWorker();
                worker.postMessage({
                    ...supplies,
                    type: "spawn",
                    startArgPtr,
                    tid,
                });

                return tid;
            },
        },
    });

    const exports = instance.exports as any;

    if (supplies.type === "main") {
        console.log("main thread start");
        wasi.start(instance as any);
        console.log("main thread end");
    } else {
        wasi.initialize(instance as any);
        exports.wasi_thread_start(supplies.tid, supplies.startArgPtr);
    }
}
