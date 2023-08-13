import { cacheGet, cacheSet } from "../cache";
import { initHotReload } from "../hotReload";
import { initCanvasKit } from "../canvasKit";

declare global {
    const NAMUI_ENV: "production" | "development";
    const wasm_bindgen: (url: string) => Promise<any>;
}

if (NAMUI_ENV === "development") {
    initHotReload();
}

(window as any).cacheGet = cacheGet;
(window as any).cacheSet = cacheSet;

const canvas = document.createElement("canvas");
canvas.width = window.innerWidth;
canvas.height = window.innerHeight;
canvas.style.width = "100%";
canvas.style.height = "100%";
canvas.id = "canvas";
document.body.appendChild(canvas);
const offscreen = canvas.transferControlToOffscreen();

const drawWorker = new Worker("worker.js", {
    type: "classic",
});

drawWorker.postMessage(
    {
        type: "init",
        offscreen,
    },
    [offscreen],
);

// runAsyncMessageLoop<AsyncMessageFromWorker>(drawWorker, async (message) => {
//     switch (message.type) {
//         case "imageBitmap": {
//             const { imageBitmap } = message;

//             bitmapRendererCtx.transferFromImageBitmap(imageBitmap);

//             return {};
//         }
//     }
// });

// sendAsyncRequest(
//     drawWorker,
//     {
//         type: "init",
//         windowWidth: window.innerWidth,
//         windowHeight: window.innerHeight,
//     },
//     [],
// );

// drawWorker.onerror = (e) => {
//     console.error(e, "error on worker");
// };

// drawWorker.onmessageerror = (e) => {
//     console.error(e, "message error from worker");
// };

document.oncontextmenu = (event) => {
    event.preventDefault();
};

(globalThis as any).requestDraw = (buffer: ArrayBuffer) => {
    drawWorker.postMessage(
        {
            type: "requestDraw",
            buffer,
        },
        [buffer],
    );
};

(globalThis as any).loadTypeface = (
    typefaceName: string,
    buffer: ArrayBuffer,
) => {
    drawWorker.postMessage(
        {
            type: "loadTypeface",
            typefaceName,
            buffer,
        },
        [buffer],
    );
};

(async () => {
    const [{ start }] = await Promise.all([
        wasm_bindgen("./bundle_bg.wasm"),
        initCanvasKit(),
    ]);
    await start();
})();
