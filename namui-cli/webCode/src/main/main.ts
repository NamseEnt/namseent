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

const gloalThisAny = globalThis as any;

gloalThisAny.requestDraw = (buffer: Uint8Array) => {
    drawWorker.postMessage(
        {
            type: "requestDraw",
            buffer,
        },
        [buffer],
    );
};

gloalThisAny.loadTypeface = (typefaceName: string, buffer: Uint8Array) => {
    drawWorker.postMessage(
        {
            type: "loadTypeface",
            typefaceName,
            buffer,
        },
        [buffer],
    );
};

gloalThisAny.loadImage = (
    imageSource: Uint8Array,
    imageBitmap: ImageBitmap,
) => {
    drawWorker.postMessage(
        {
            type: "loadImage",
            imageSource,
            imageBitmap,
        },
        [imageSource, imageBitmap],
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
