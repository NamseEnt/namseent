import { startEventSystemOnMainThread } from "./eventSystem";
import { insertJsHandleOnMainThread } from "./insertJs";
import { WorkerMessagePayload, sendToWorker } from "./interWorkerProtocol";
import MainWorker from "./main-worker?worker";
import { TextInput } from "./textInput";
import ThreadWorker from "./thread-worker?worker";
import { webSocketHandleOnMainThread } from "./webSocket";
import StorageWorker from "./storage/worker?worker";
import {
    httpFetchHandleOnMainThread,
    isSupportsRequestStreams,
} from "./httpFetch/httpFetch";
import { NewEventSystemHandleOnMainThread } from "./newEventSystem";
import { BufferPoolHandleOnMainThread } from "./bufferPool";
import { audioHandleOnMainThread } from "./audio";

(async function main() {
    const canvas = document.createElement("canvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    document.body.appendChild(canvas);

    const bitmapCtx = canvas.getContext("bitmaprenderer")!;
    if (!bitmapCtx) {
        throw new Error("Failed to get bitmap context");
    }

    const eventBuffer = new SharedArrayBuffer(512 * 1024);

    const { onTextInputEvent } = startEventSystemOnMainThread(eventBuffer);
    const textInput = new TextInput(onTextInputEvent);

    const wasmMemory = new WebAssembly.Memory({
        initial: 128,
        maximum: 16384,
        shared: true,
    });

    const storageWorker = new StorageWorker();
    sendToWorker(storageWorker, {
        type: "storage-init",
        wasmMemory,
    });

    const mainWorker = new MainWorker();
    sendToWorker(mainWorker, {
        type: "start-main-thread",
        eventBuffer,
        initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
        wasmMemory,
    });

    let webSocketHandle: ReturnType<typeof webSocketHandleOnMainThread>;
    let insertJsHandle: ReturnType<typeof insertJsHandleOnMainThread>;
    let newEventSystemHandle: NewEventSystemHandleOnMainThread;
    let bufferPoolHandle: BufferPoolHandleOnMainThread;
    const supportsRequestStreams = await isSupportsRequestStreams();
    let httpFetchHandle: ReturnType<typeof httpFetchHandleOnMainThread>;
    const audioHandle = audioHandleOnMainThread();

    function onMessage(this: Worker, message: MessageEvent) {
        const payload: WorkerMessagePayload = message.data;

        switch (payload.type) {
            case "thread-spawn": {
                const threadWorker = new ThreadWorker();
                threadWorker.onmessage = onMessage;
                threadWorker.postMessage(payload);
                break;
            }
            case "bitmap": {
                bitmapCtx.transferFromImageBitmap(payload.bitmap);
                break;
            }
            case "update-canvas-wh": {
                if (canvas.width != payload.width) {
                    canvas.width = payload.width;
                }
                if (canvas.height != payload.height) {
                    canvas.height = payload.height;
                }
                break;
            }
            case "text-input-set-selection-range":
            case "text-input-focus":
            case "text-input-blur": {
                textInput.onMessage(payload);
                break;
            }
            // WebSocket
            case "init-web-socket-thread": {
                webSocketHandle = webSocketHandleOnMainThread(payload);
                break;
            }
            case "new-web-socket": {
                if (!webSocketHandle) {
                    throw new Error("WebSocket handle is not initialized");
                }
                webSocketHandle.onNewWebSocket(payload);
                break;
            }
            case "web-socket-send": {
                if (!webSocketHandle) {
                    throw new Error("WebSocket handle is not initialized");
                }
                webSocketHandle.send(payload);
                break;
            }
            // Js Insert
            case "insert-js": {
                insertJsHandle.onInsertJs(payload);
                break;
            }
            case "insert-js-data-buffer": {
                insertJsHandle.onInsertJsDataBuffer(payload);
                break;
            }
            case "insert-js-drop": {
                insertJsHandle.onInsertJsDrop(payload);
                break;
            }
            case "insert-js-send-data-from-rust": {
                insertJsHandle.onInsertJsSendDataFromRust(payload);
                break;
            }
            // File System
            case "storage-thread-connect": {
                storageWorker.postMessage(payload);
                break;
            }
            case "storage-thread-disconnect": {
                storageWorker.postMessage(payload);
                break;
            }
            // Http Fetch
            case "new-http-fetch": {
                httpFetchHandle.onNewHttpFetch(payload);
                break;
            }
            case "http-fetch-set-header": {
                httpFetchHandle.onHttpFetchSetHeader(payload);
                break;
            }
            case "http-fetch-start": {
                httpFetchHandle.onHttpFetchStart(payload);
                break;
            }
            case "http-fetch-push-request-body-chunk": {
                httpFetchHandle.onHttpFetchPushRequestBodyChunk(payload);
                break;
            }
            case "http-fetch-finish-request-body-stream": {
                httpFetchHandle.onHttpFetchFinishRequestBodyStream(payload);
                break;
            }
            // Buffer Pool
            case "buffer-pool-new-buffer": {
                bufferPoolHandle.pushBuffer({
                    ptr: payload.ptr,
                    view: new Uint8Array(
                        wasmMemory.buffer,
                        payload.ptr,
                        payload.len,
                    ),
                });
                break;
            }
            case "http-fetch-error-on-rust-side": {
                httpFetchHandle.onHttpFetchErrorOnRustSide(payload.fetchId);
                break;
            }
            // New Event System
            case "init-new-event-system-thread": {
                newEventSystemHandle = new NewEventSystemHandleOnMainThread(
                    payload,
                );
                bufferPoolHandle = new BufferPoolHandleOnMainThread(
                    newEventSystemHandle,
                );
                httpFetchHandle = httpFetchHandleOnMainThread(
                    supportsRequestStreams,
                    newEventSystemHandle,
                    bufferPoolHandle,
                );
                insertJsHandle = insertJsHandleOnMainThread(
                    newEventSystemHandle,
                    wasmMemory,
                );
                break;
            }
            // Audio
            case "audio-init": {
                audioHandle.audioInit(payload);
                break;
            }
            case "audio-drop": {
                audioHandle.audioDrop(payload);
                break;
            }
            case "audio-play": {
                audioHandle.audioPlay(payload);
                break;
            }
            case "audio-play_and_forget": {
                audioHandle.audioPlayAndForget(payload);
                break;
            }
            case "audio-playback_drop": {
                audioHandle.audioPlaybackDrop(payload);
                break;
            }
            case "audio-context-volume-set": {
                audioHandle.audioContextVolumeSet(payload);
                break;
            }
            default:
                throw new Error(`Unexpected message type: ${payload.type}`);
        }
    }

    mainWorker.onmessage = onMessage;

    document.addEventListener("contextmenu", (e) => {
        e.preventDefault();
    });
})();
