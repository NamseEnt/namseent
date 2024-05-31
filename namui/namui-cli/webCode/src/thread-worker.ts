import { WASI, File, OpenFile, ConsoleStdout } from "@bjorn3/browser_wasi_shim";
import { createImportObject } from "./importObject";

console.debug("hello world from thread-worker");

self.onmessage = async (message) => {
  console.debug("event on thread-worker", message.data);

  const { tid, nextTid, importMemory, module, startArgPtr } = message.data as {
    tid: number;
    nextTid: SharedArrayBuffer;
    importMemory: WebAssembly.Memory;
    module: WebAssembly.Module;
    startArgPtr: number;
  };

  const args = ["bin", "arg1", "arg2"];
  const env = ["FOO=bar"];
  const fds = [
    new OpenFile(new File([])), // stdin
    ConsoleStdout.lineBuffered((msg) => console.log(`[WASI stdout] ${msg}`)),
    ConsoleStdout.lineBuffered((msg) => console.warn(`[WASI stderr] ${msg}`)),
  ];
  const wasi = new WASI(args, env, fds);

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
  });

  const instance = await WebAssembly.instantiate(module, importObject);
  exports = instance.exports as Record<string, Function>;

  wasi.initialize(instance as any);
  console.debug("before wasi_thread_start", tid, startArgPtr);
  (instance.exports.wasi_thread_start as any)(tid, startArgPtr);
  console.debug("after wasi_thread_start");
};
