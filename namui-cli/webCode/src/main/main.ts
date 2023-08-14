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
    const [{ start, on_load_image }, _] = await Promise.all([
        wasm_bindgen("./bundle_bg.wasm"),
        initCanvasKit(),
    ]);

    drawWorker.onmessage = (message) => {
        switch (message.data.type) {
            case "onLoadImage": {
                on_load_image();
            }
        }
    };

    await start();
})();
