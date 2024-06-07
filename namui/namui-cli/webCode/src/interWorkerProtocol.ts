import { BundleSharedTree } from "./fds";

export type WorkerMessagePayload =
    | {
          type: "thread-spawn";
          tid: number;
          nextTid: SharedArrayBuffer;
          importMemory: WebAssembly.Memory;
          module: WebAssembly.Module;
          startArgPtr: number;
          bundleSharedTree: BundleSharedTree;
          eventBuffer: SharedArrayBuffer;
          initialWindowWh: number;
      }
    | {
          type: "start-main-thread";
          eventBuffer: SharedArrayBuffer;
          initialWindowWh: number;
      }
    | {
          type: "bitmap";
          bitmap: ImageBitmap;
      }
    | {
          type: "update-canvas-wh";
          width: number;
          height: number;
      };

export function sendMessageToMainThread(
    payload: WorkerMessagePayload,
    transfer?: Transferable[],
) {
    self.postMessage(payload, transfer ?? []);
}

export function sendToWorker(
    worker: Worker,
    payload: WorkerMessagePayload,
    transfer?: Transferable[],
) {
    worker.postMessage(payload, transfer ?? []);
}
