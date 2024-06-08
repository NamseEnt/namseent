import { startEventSystemOnMainThread } from "./eventSystem";
import { WorkerMessagePayload, sendToWorker } from "./interWorkerProtocol";
import MainWorker from "./main-worker?worker";
import { TextInput } from "./textInput";
import ThreadWorker from "./thread-worker?worker";

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

const mainWorker = new MainWorker();

sendToWorker(mainWorker, {
    type: "start-main-thread",
    eventBuffer,
    initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
});

function onMessage(message: MessageEvent) {
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
        default:
            throw new Error(`Unexpected message type: ${payload.type}`);
    }
}

mainWorker.onmessage = onMessage;

document.addEventListener("contextmenu", (e) => {
    e.preventDefault();
});
