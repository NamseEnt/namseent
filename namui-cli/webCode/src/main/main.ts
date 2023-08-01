import { runAsyncMessageLoop, sendAsyncRequest } from "../asyncMessage";
import { enqueueWebEvent, shiftWebEvent } from "./webEvent";
import { runMessageLoopForMain } from "../messageLoop";
import { AsyncMessageFromWorker } from "../type";
import { cacheGet, cacheSet } from "../cache";
import { initHotReload } from "../hotReload";

declare global {
    const NAMUI_ENV: "production" | "development";
}

if (NAMUI_ENV === "development") {
    initHotReload();
}

(window as any).cacheGet = cacheGet;
(window as any).cacheSet = cacheSet;

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
            const webEvent = shiftWebEvent();
            return {
                webEvent,
            };
        }
        case "executeFunctionSyncOnMain": {
            const { args_names, code, args } = message;
            return Function(...args_names, code)(...args);
        }
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
            return {};
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

runAsyncMessageLoop<AsyncMessageFromWorker>(myWorker, async (message) => {
    switch (message.type) {
        case "imageBitmap": {
            const { imageBitmap } = message;

            bitmapRendererCtx.transferFromImageBitmap(imageBitmap);

            return {};
        }
        case "executeAsyncFunction": {
            const { id, argsNames, code, args } = message;
            const result = await Function(...argsNames, code)(...args);

            enqueueWebEvent({
                AsyncFunction: {
                    id,
                    result: result,
                },
            });
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
    console.error(e, "message error from worker");
};

document.oncontextmenu = (event) => {
    event.preventDefault();
};
