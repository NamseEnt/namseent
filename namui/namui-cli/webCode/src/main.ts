import { startEventSystemOnMainThread } from "./eventSystem";
import { insertJsHandleOnMainThread } from "./insertJs";
import { WorkerMessagePayload, sendToWorker } from "./interWorkerProtocol";
import MainWorker from "./main-worker?worker";
import { TextInput } from "./textInput";
import ThreadWorker from "./thread-worker?worker";
import { webSocketHandleOnMainThread } from "./webSocket";

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

const mainWorker = new MainWorker();

sendToWorker(mainWorker, {
    type: "start-main-thread",
    eventBuffer,
    initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
    wasmMemory,
});

let webSocketHandle: ReturnType<typeof webSocketHandleOnMainThread>;
const insertJsHandle = insertJsHandleOnMainThread();

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
        case "insert-js-drop": {
            insertJsHandle.onInsertJsDrop(payload);
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
