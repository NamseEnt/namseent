import {
    ConsoleStdout,
    Fd,
    File,
    OpenFile,
    WASI,
} from "@bjorn3/browser_wasi_shim";
import "./style.css";
import wasmUrl from "/bundle.wasm?url";
import { patchWasi } from "./patchWasi";
import { threadMain } from "./thread/threadMain";
// import { run } from "./out/bundle";

// run();
// run();
// run();

const module = await WebAssembly.compileStreaming(fetch(wasmUrl));

const fd: Fd[] = [
    new OpenFile(new File([])), // stdin
    ConsoleStdout.lineBuffered((msg) => console.log(msg)),
    ConsoleStdout.lineBuffered((msg) => console.log(`[stderr] ${msg}`)),
];

const env = ["RUST_BACKTRACE=full"];

const wasi = new WASI([], env, fd);
patchWasi(wasi);

const memory = new WebAssembly.Memory({
    initial: 128,
    maximum: 16384,
    shared: true,
});

threadMain({
    memory,
    module,
    nextTid: new SharedArrayBuffer(4),
    type: "main",
});

// const app = document.getElementById("app")!;
// app.innerHTML = `
//     module: ${module}
//     instance: ${instance}
// `;

console.log("done");
