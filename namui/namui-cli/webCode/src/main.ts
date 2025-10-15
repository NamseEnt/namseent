import { startEventSystem } from "./eventSystem";
import { insertJsHandleOnMainThread } from "./insertJs";
import { WorkerMessagePayload, sendToWorker } from "./interWorkerProtocol";
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
import { pushLog } from "./logger";
import { startThread } from "./thread/startThread";
import wasmUrl from "/Users/namse/namseent2/tower-defense/target/namui/target/wasm32-wasip1-threads/debug/namui-runtime-wasm.wasm?url";
import "./drawer";
import { readyDrawer } from "./drawer";
import { Exports } from "./exports";
import { assetList } from "virtual:asset-list";
import { loadFonts } from "@/font/loadFont";

console.debug("crossOriginIsolated", crossOriginIsolated);

if (!crossOriginIsolated) {
    throw new Error("Not cross-origin isolated");
}

const memory = new WebAssembly.Memory({
    initial: 128,
    maximum: 16384,
    shared: true,
});

const nextTid = new SharedArrayBuffer(4);
new Uint32Array(nextTid)[0] = 1;

const [{ drawerExports, canvas }, module] = await Promise.all([
    readyDrawer(),
    WebAssembly.compileStreaming(fetch(wasmUrl)),
]);

const imageCount = assetList.length;
const imageInfoSize = 14;
const imageInfoBytes = new Uint8Array(imageCount * imageInfoSize);

const imageInfosPtr = drawerExports.malloc(imageInfoBytes.byteLength);
drawerExports._image_infos(imageInfosPtr);
imageInfoBytes.set(
    new Uint8Array(
        drawerExports.memory.buffer,
        imageInfosPtr,
        imageInfoBytes.byteLength,
    ),
);
console.log("imageInfoBytes", imageInfoBytes);
drawerExports.free(imageInfosPtr);

const instance = await startThread({
    type: "main",
    memory,
    module,
    nextTid,
    initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
    imageCount,
    imageInfoBytes,
});
const exports = instance.exports as Exports;

let now = performance.now();
await loadFonts({
    memory: exports.memory,
    module,
});
console.log(`main loadFonts took: ${performance.now() - now}ms`);

now = performance.now();
exports._init_system();
console.log(`main initSystem took: ${performance.now() - now}ms`);

const { onTextInputEvent } = startEventSystem({
    exports,
    drawerExports,
    canvas,
});
const textInput = new TextInput(onTextInputEvent);

// let webSocketHandle: ReturnType<typeof webSocketHandleOnMainThread>;
// let insertJsHandle: ReturnType<typeof insertJsHandleOnMainThread>;
// let newEventSystemHandle: NewEventSystemHandleOnMainThread;
// let bufferPoolHandle: BufferPoolHandleOnMainThread;
// const supportsRequestStreams = await isSupportsRequestStreams();
// let httpFetchHandle: ReturnType<typeof httpFetchHandleOnMainThread>;
// const audioHandle = audioHandleOnMainThread();

// function onMessage(this: Worker, message: MessageEvent) {
//     const payload: WorkerMessagePayload = message.data;

//     switch (payload.type) {
//         case "thread-spawn": {
//             const threadWorker = new ThreadWorker();
//             threadWorker.onmessage = onMessage;
//             threadWorker.postMessage(payload);
//             break;
//         }
//         case "bitmap": {
//             bitmapCtx.transferFromImageBitmap(payload.bitmap);
//             break;
//         }
//         case "update-canvas-wh": {
//             if (canvas.width != payload.width) {
//                 canvas.width = payload.width;
//             }
//             if (canvas.height != payload.height) {
//                 canvas.height = payload.height;
//             }
//             break;
//         }
//         case "text-input-set-selection-range":
//         case "text-input-focus":
//         case "text-input-blur": {
//             textInput.onMessage(payload);
//             break;
//         }
//         // WebSocket
//         case "init-web-socket-thread": {
//             webSocketHandle = webSocketHandleOnMainThread(payload);
//             break;
//         }
//         case "new-web-socket": {
//             if (!webSocketHandle) {
//                 throw new Error("WebSocket handle is not initialized");
//             }
//             webSocketHandle.onNewWebSocket(payload);
//             break;
//         }
//         case "web-socket-send": {
//             if (!webSocketHandle) {
//                 throw new Error("WebSocket handle is not initialized");
//             }
//             webSocketHandle.send(payload);
//             break;
//         }
//         // Js Insert
//         case "insert-js": {
//             insertJsHandle.onInsertJs(payload);
//             break;
//         }
//         case "insert-js-data-buffer": {
//             insertJsHandle.onInsertJsDataBuffer(payload);
//             break;
//         }
//         case "insert-js-drop": {
//             insertJsHandle.onInsertJsDrop(payload);
//             break;
//         }
//         case "insert-js-send-data-from-rust": {
//             insertJsHandle.onInsertJsSendDataFromRust(payload);
//             break;
//         }
//         // File System
//         case "storage-thread-connect": {
//             storageWorker.postMessage(payload);
//             break;
//         }
//         case "storage-thread-disconnect": {
//             storageWorker.postMessage(payload);
//             break;
//         }
//         // Http Fetch
//         case "new-http-fetch": {
//             httpFetchHandle.onNewHttpFetch(payload);
//             break;
//         }
//         case "http-fetch-set-header": {
//             httpFetchHandle.onHttpFetchSetHeader(payload);
//             break;
//         }
//         case "http-fetch-start": {
//             httpFetchHandle.onHttpFetchStart(payload);
//             break;
//         }
//         case "http-fetch-push-request-body-chunk": {
//             httpFetchHandle.onHttpFetchPushRequestBodyChunk(payload);
//             break;
//         }
//         case "http-fetch-finish-request-body-stream": {
//             httpFetchHandle.onHttpFetchFinishRequestBodyStream(payload);
//             break;
//         }
//         // Buffer Pool
//         case "buffer-pool-new-buffer": {
//             bufferPoolHandle.pushBuffer({
//                 ptr: payload.ptr,
//                 view: new Uint8Array(
//                     wasmMemory.buffer,
//                     payload.ptr,
//                     payload.len,
//                 ),
//             });
//             break;
//         }
//         case "http-fetch-error-on-rust-side": {
//             httpFetchHandle.onHttpFetchErrorOnRustSide(payload.fetchId);
//             break;
//         }
//         // New Event System
//         case "init-new-event-system-thread": {
//             newEventSystemHandle = new NewEventSystemHandleOnMainThread(
//                 payload,
//             );
//             bufferPoolHandle = new BufferPoolHandleOnMainThread(
//                 newEventSystemHandle,
//             );
//             httpFetchHandle = httpFetchHandleOnMainThread(
//                 supportsRequestStreams,
//                 newEventSystemHandle,
//                 bufferPoolHandle,
//             );
//             insertJsHandle = insertJsHandleOnMainThread(
//                 newEventSystemHandle,
//                 wasmMemory,
//             );
//             break;
//         }
//         // Audio
//         case "audio-init": {
//             audioHandle.audioInit(payload);
//             break;
//         }
//         case "audio-drop": {
//             audioHandle.audioDrop(payload);
//             break;
//         }
//         case "audio-play": {
//             audioHandle.audioPlay(payload);
//             break;
//         }
//         case "audio-play_and_forget": {
//             audioHandle.audioPlayAndForget(payload);
//             break;
//         }
//         case "audio-playback_drop": {
//             audioHandle.audioPlaybackDrop(payload);
//             break;
//         }
//         case "audio-context-volume-set": {
//             audioHandle.audioContextVolumeSet(payload);
//             break;
//         }
//         // Log
//         case "log": {
//             pushLog(payload.threadId, payload.msg);
//             break;
//         }
//         default:
//             throw new Error(`Unexpected message type: ${payload.type}`);
//     }
// }

// document.addEventListener("contextmenu", (e) => {
//     e.preventDefault();
// });
