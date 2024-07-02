export type WorkerMessagePayload =
    | {
          type: "thread-spawn";
          tid: number;
          nextTid: SharedArrayBuffer;
          wasmMemory: WebAssembly.Memory;
          module: WebAssembly.Module;
          startArgPtr: number;
          eventBuffer: SharedArrayBuffer;
          initialWindowWh: number;
          bundleSqlite: ArrayBuffer;
      }
    | {
          type: "start-main-thread";
          eventBuffer: SharedArrayBuffer;
          initialWindowWh: number;
          wasmMemory: WebAssembly.Memory;
      }
    | {
          type: "bitmap";
          bitmap: ImageBitmap;
      }
    | {
          type: "update-canvas-wh";
          width: number;
          height: number;
      }
    | {
          type: "text-input-set-selection-range";
          start: number;
          end: number;
          direction: "forward" | "backward" | "none";
      }
    | {
          type: "text-input-focus";
          width: number;
          text: string;
          selection_start: number;
          selection_end: number;
          direction: "forward" | "backward" | "none";
          prevent_default_codes: number[];
      }
    | {
          type: "text-input-blur";
      }
    // WebSocket
    | {
          type: "init-web-socket-thread";
          wasmMemory: WebAssembly.Memory;
          writtenBuffer: SharedArrayBuffer;
          eventBufferPtr: number;
          eventBufferLen: number;
      }
    | {
          type: "new-web-socket";
          url: string;
          /** Uint32, non-zero id. */
          idBuffer: SharedArrayBuffer;
      }
    | {
          type: "web-socket-send";
          id: number;
          data: ArrayBuffer;
      }
    // Js Insert
    | {
          type: "insert-js";
          js: string;
          /** Uint32, non-zero id. */
          idBuffer: SharedArrayBuffer;
          ringBuffer?: {
              wasmMemory: SharedArrayBuffer;
              ptr: number;
              len: number;
              writtenBuffer: SharedArrayBuffer;
          };
      }
    | {
          type: "insert-js-drop";
          id: number;
      }
    // Storage System
    | {
          type: "storage-init";
          wasmMemory: WebAssembly.Memory;
      }
    | {
          type: "storage-thread-connect";
          threadId: number;
          protocolBuffer: SharedArrayBuffer;
      }
    | {
          type: "storage-thread-disconnect";
          threadId: number;
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
