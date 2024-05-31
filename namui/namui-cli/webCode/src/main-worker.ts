import { WASI, File, OpenFile, ConsoleStdout } from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./importObject";
import wasmUrl from "../../namui-runtime-wasm.wasm?url";

console.debug("crossOriginIsolated", crossOriginIsolated);

const args = ["bin", "arg1", "arg2"];
const env = ["RUST_BACKTRACE=full"];
const fds = [
  new OpenFile(new File([])), // stdin
  ConsoleStdout.lineBuffered((msg) => console.log(`[WASI stdout] ${msg}`)),
  ConsoleStdout.lineBuffered((msg) => console.warn(`[WASI stderr] ${msg}`)),
];
const wasi = new WASI(args, env, fds);

const memory = new WebAssembly.Memory({
  initial: 320,
  maximum: 16384,
  shared: true,
});

const nextTid = new SharedArrayBuffer(4);

self.onmessage = async (message) => {
  const { canvas } = message.data as { canvas: OffscreenCanvas };
  const webgl = canvas.getContext("webgl2")!;

  const module = await WebAssembly.compileStreaming(fetch(wasmUrl));

  let exports: {
    _malloc: (size: number) => number;
    _free: (ptr: number) => void;
    memory: WebAssembly.Memory;
  } = {} as any;

  const importObject = createImportObject({
    memory,
    module,
    nextTid,
    wasiImport: wasi.wasiImport,
    malloc: (size: number) => {
      return exports._malloc(size);
    },
    free: (ptr: number) => {
      return exports._free(ptr);
    },
    webgl,
  });

  const instance = await WebAssembly.instantiate(module, importObject);
  exports = instance.exports as any;
  console.debug("instance.exports", instance.exports);

  wasi.start(instance as any);
};
