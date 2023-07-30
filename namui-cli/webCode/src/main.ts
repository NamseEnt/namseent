import { runAsyncMessageLoop, sendAsyncRequest } from "./asyncMessage.js";
import { waitWebEvent } from "./main/webEvent.js";
import { runMessageLoopForMain } from "./messageLoop.js";

const workerToMainBufferSab = new SharedArrayBuffer(16 * 1024 * 1024);
const mainToWorkerBufferSab = new SharedArrayBuffer(16 * 1024 * 1024);

runMessageLoopForMain(workerToMainBufferSab, async (message) => {
    switch (message.type) {
        case "getBaseUrl": {
            return {
                baseUrl: window.document.URL,
            };
        }
        case "webEvent": {
            const webEvent = await waitWebEvent();
            return {
                webEvent,
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
const bitmapRendererCtx = canvas.getContext("bitmaprenderer");
if (!bitmapRendererCtx) {
    throw new Error("no bitmapRendererCtx");
}

window.addEventListener("resize", () => {
    myWorker.postMessage({
        type: "windowResize",
        width: window.innerWidth,
        height: window.innerHeight,
    });
});

const myWorker = new Worker("worker.js", {
    type: "classic",
});

runAsyncMessageLoop(myWorker, async (message) => {
    switch (message.type) {
        case "imageBitmap": {
            const {
                imageBitmap,
            }: {
                imageBitmap: ImageBitmap;
            } = message;

            bitmapRendererCtx.transferFromImageBitmap(imageBitmap);

            return {};
        }
    }
});

sendAsyncRequest(
    myWorker,
    {
        type: "init",
        workerToMainBufferSab,
        mainToWorkerBufferSab,
        windowWidth: window.innerWidth,
        windowHeight: window.innerHeight,
    },
    [],
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
