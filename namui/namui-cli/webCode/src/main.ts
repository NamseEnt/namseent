import { startEventSystemOnMainThread } from "./eventSystem";
import MainWorker from "./main-worker?worker";
import ThreadWorker from "./thread-worker?worker";

const canvas = document.createElement("canvas");
document.body.appendChild(canvas);
const offscreen = canvas.transferControlToOffscreen();

const eventBuffer = new SharedArrayBuffer(512 * 1024);

const mainWorker = new MainWorker();

mainWorker.postMessage(
    {
        canvas: offscreen,
        eventBuffer,
    },
    [offscreen],
);

function onMessage(message: MessageEvent) {
    const threadWorker = new ThreadWorker();
    threadWorker.onmessage = onMessage;
    threadWorker.postMessage(message.data);
}

mainWorker.onmessage = onMessage;

startEventSystemOnMainThread(eventBuffer);
