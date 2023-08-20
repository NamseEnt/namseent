import { cacheGet, cacheSet } from "../cache";
import { initHotReload } from "../hotReload";
import { initCanvasKit } from "../canvasKit";
import { getNextMessageId, onMessage, waitForMessage } from "./messageWaiting";

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

const globalThisAny = globalThis as any;

globalThisAny.requestDraw = (buffer: Uint8Array) => {
    drawWorker.postMessage(
        {
            type: "requestDraw",
            buffer,
        },
        [buffer],
    );
};

globalThisAny.loadTypeface = (typefaceName: string, buffer: Uint8Array) => {
    drawWorker.postMessage(
        {
            type: "loadTypeface",
            typefaceName,
            buffer,
        },
        [buffer],
    );
};

globalThisAny.loadImage = (
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

globalThisAny.encodeLoadedImageToPng = async (image: Uint8Array) => {
    const id = getNextMessageId();
    drawWorker.postMessage(
        {
            type: "encodeLoadedImageToPng",
            image,
            id,
        },
        [image],
    );
    const { pngBytes } = (await waitForMessage(id)) as {
        pngBytes: Uint8Array;
    };
    return pngBytes;
};

(async () => {
    const [{ start, on_load_image }, _] = await Promise.all([
        wasm_bindgen("./bundle_bg.wasm"),
        initCanvasKit(),
    ]);

    drawWorker.onmessage = (message) => {
        switch (message.data.type) {
            case "onLoadImage":
                {
                    on_load_image();
                }
                break;
            case "encodeLoadedImageToPng":
                {
                    onMessage(message.data);
                }
                break;
        }
    };

    await start();
})();
