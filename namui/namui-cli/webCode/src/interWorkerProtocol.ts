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
          jsId: number;
      }
    | {
          type: "insert-js-drop";
          jsId: number;
      }
    | {
          type: "insert-js-data-buffer";
          jsId: number;
          requestId: number;
          bufferPtr: number;
      }
    | {
          type: "insert-js-send-data-from-rust";
          jsId: number;
          sendDataId: number;
          data: ArrayBuffer;
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
      }
    // Http Fetch
    | {
          type: "new-http-fetch";
          url: string;
          method: string;
          idBuffer: SharedArrayBuffer;
      }
    | {
          type: "http-fetch-set-header";
          fetchId: number;
          key: string;
          value: string;
      }
    | {
          type: "http-fetch-start";
          fetchId: number;
      }
    | {
          type: "http-fetch-push-request-body-chunk";
          fetchId: number;
          data: ArrayBuffer;
      }
    | {
          type: "http-fetch-finish-request-body-stream";
          fetchId: number;
      }
    | {
          type: "http-fetch-error-on-rust-side";
          fetchId: number;
      }
    // New Event System
    | {
          type: "init-new-event-system-thread";
          wasmMemory: WebAssembly.Memory;
          writtenBuffer: SharedArrayBuffer;
          eventBufferPtr: number;
          eventBufferLen: number;
      }
    | {
          type: "buffer-pool-new-buffer";
          ptr: number;
          len: number;
      }
    // Audio
    | {
          type: "audio-init";
          audioId: number;
          buffer: ArrayBuffer;
      }
    | {
          type: "audio-drop";
          audioId: number;
      }
    | {
          type: "audio-play";
          audioId: number;
          playbackId: number;
          repeat: boolean;
      }
    | {
          type: "audio-play_and_forget";
          audioId: number;
      }
    | {
          type: "audio-playback_drop";
          playbackId: number;
      }
    | {
          type: "audio-context-volume-set";
          volume: number;
          requestSequenceNumber: number;
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
