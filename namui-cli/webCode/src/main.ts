import { runAsyncMessageLoop, sendAsyncRequest } from "./asyncMessage.js";
import { cacheGet, cacheSet } from "./cache.js";
import { runMessageLoop } from "./messageLoop.js";

const workerToMainBufferSab = new SharedArrayBuffer(16 * 1024 * 1024);
const mainToWorkerBufferSab = new SharedArrayBuffer(16 * 1024 * 1024);

runMessageLoop(workerToMainBufferSab, (message) => {
    switch (message.type) {
        case "getBaseUrl": {
            return {
                baseUrl: window.document.URL,
            };
        }
    }
});

const canvas = document.createElement("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
canvas.style.width = "100%";
canvas.style.height = "100%";
canvas.id = "canvas";
document.body.appendChild(canvas);

window.addEventListener("resize", () => {
    myWorker.postMessage({
        type: "windowResize",
        width: window.innerWidth,
        height: window.innerHeight,
    });
});

const offscreenCanvas = canvas.transferControlToOffscreen();

const myWorker = new Worker("worker.js", {
    type: "classic",
});

runAsyncMessageLoop(myWorker, async (message) => {
    switch (message.type) {
        case "cacheGet": {
            const { key } = message;
            const value = await cacheGet(key);
            return {
                value,
            };
        }
        case "cacheSet": {
            const { key, value } = message;
            await cacheSet(key, value);
            return;
        }
    }
});

sendAsyncRequest(
    myWorker,
    {
        type: "init",
        workerToMainBufferSab,
        mainToWorkerBufferSab,
        offscreenCanvas,
    },
    [offscreenCanvas],
);
myWorker.postMessage;

myWorker.onerror = (e) => {
    console.error(e, "error on worker");
};

myWorker.onmessageerror = (e) => {
    console.log("message error from worker", e);
};

document.oncontextmenu = (event) => {
    event.preventDefault();
};
