import { cacheGet, cacheSet } from "../cache";
import { initHotReload } from "../hotReload";
import { initCanvasKit } from "../canvasKit";
import { getNextMessageId, onMessage, waitForMessage } from "./messageWaiting";
import {
    InspectTree,
    isInspectOn,
    setInspectTree,
    toggleInspectOn,
} from "../inspect/inspect";

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

document.oncontextmenu = (event) => {
    event.preventDefault();
};

const globalThisAny = globalThis as any;

globalThisAny.requestDraw = (array: Uint8Array) => {
    const buffer = array.buffer;
    drawWorker.postMessage(
        {
            type: "requestDraw",
            buffer,
        },
        [buffer],
    );
};

globalThisAny.loadTypeface = async (
    typefaceName: string,
    array: Uint8Array,
) => {
    const buffer = array.buffer;
    const id = getNextMessageId();
    drawWorker.postMessage(
        {
            type: "loadTypeface",
            id,
            typefaceName,
            buffer: buffer,
        },
        [buffer],
    );
    const { error } = (await waitForMessage(id)) as {
        error: Error | undefined;
    };

    if (error) {
        throw error;
    }
};

globalThisAny.loadImage = (
    imageSource: Uint8Array,
    imageBitmap: ImageBitmap,
) => {
    const imageSourceBuffer = imageSource.buffer;
    drawWorker.postMessage(
        {
            type: "loadImage",
            imageSource: imageSourceBuffer,
            imageBitmap,
        },
        [imageSourceBuffer, imageBitmap],
    );
};

globalThisAny.encodeLoadedImageToPng = async (image: Uint8Array) => {
    const id = getNextMessageId();
    const imageBuffer = image.buffer;
    drawWorker.postMessage(
        {
            type: "encodeLoadedImageToPng",
            imageBuffer,
            id,
        },
        [imageBuffer],
    );
    const { pngBytes } = (await waitForMessage(id)) as {
        pngBytes: ArrayBuffer;
    };
    return new Uint8Array(pngBytes);
};

globalThisAny.onInspect = async (inspectTree: InspectTree) => {
    setInspectTree(inspectTree);
};

(async () => {
    drawWorker.postMessage(
        {
            type: "init",
            offscreen,
        },
        [offscreen],
    );

    const error = await new Promise((resolve) => {
        drawWorker.onmessage = (message) => {
            if (message.data.type === "init") {
                const { error } = message.data as { error: string | undefined };
                resolve(error);
            }
        };
    });
    if (error) {
        throw error;
    }

    const [{ start, on_load_image, set_inspect_toggle_on, panicked }, _] =
        await Promise.all([wasm_bindgen("./bundle_bg.wasm"), initCanvasKit()]);

    // TODO
    // globalThisAny.inspect = () => {
    //     toggleInspectOn();
    //     set_inspect_toggle_on(isInspectOn());
    // };
    // set_inspect_toggle_on(isInspectOn());

    globalThisAny.panic = async (msg: string) => {
        console.error(msg);
        panicked();
        drawWorker.postMessage({
            type: "panic",
        });
    };

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
            case "panic":
                {
                    panicked();
                }
                break;
            case "loadTypeface":
                {
                    onMessage(message.data);
                }
                break;
        }
    };

    await start();
})();

window.addEventListener("resize", () => {
    drawWorker.postMessage({
        type: "resize",
        width: window.innerWidth,
        height: window.innerHeight,
    });
});
