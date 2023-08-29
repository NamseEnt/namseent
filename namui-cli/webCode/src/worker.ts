import { initCanvasKit } from "./canvasKit";

importScripts("./drawer/bundle.js");

declare var wasm_bindgen: any;
declare var CanvasKit: any;
const {
    init,
    draw,
    load_typeface,
    load_image,
    encode_loaded_image_to_png,
    refresh_surface,
} = wasm_bindgen;

function createWaiter(): { waiter: Promise<void>; resolve: () => void } {
    let resolve: any;
    const waiter = new Promise<void>((_resolve) => {
        resolve = _resolve;
    });
    return { waiter, resolve };
}

const { waiter: initWaiter, resolve: finishInit } = createWaiter();

(async () => {
    await initCanvasKit();
    await wasm_bindgen("./drawer/bundle_bg.wasm");

    (globalThis as any).CanvasKit = CanvasKit;
    (globalThis as any).getCanvasKit = () => CanvasKit;

    finishInit();
})();

let lastRequestedDrawInput: Uint8Array;
let offscreenCanvas: OffscreenCanvas;

self.onmessage = async (event) => {
    await initWaiter;

    switch (event.data.type) {
        case "init":
            {
                offscreenCanvas = event.data.offscreen;

                init(offscreenCanvas);
            }
            break;
        case "requestDraw":
            {
                const { buffer } = event.data as { buffer: ArrayBuffer };
                lastRequestedDrawInput = new Uint8Array(buffer);
            }
            break;
        case "loadTypeface":
            {
                const { typefaceName, buffer } = event.data as {
                    typefaceName: string;
                    buffer: ArrayBuffer;
                };
                load_typeface(typefaceName, new Uint8Array(buffer));
            }
            break;
        case "loadImage":
            {
                const { imageSource, imageBitmap } = event.data as {
                    imageSource: ArrayBuffer;
                    imageBitmap: ImageBitmap;
                };
                load_image(new Uint8Array(imageSource), imageBitmap);
            }
            break;
        case "encodeLoadedImageToPng":
            {
                const { id, image } = event.data as {
                    id: number;
                    image: Uint8Array;
                };
                const pngBytes = encode_loaded_image_to_png(image);
                self.postMessage(
                    {
                        type: "encodeLoadedImageToPng",
                        pngBytes,
                        id,
                    },
                    [pngBytes],
                );
            }
            break;
        case "resize":
            {
                const { width, height } = event.data as {
                    width: number;
                    height: number;
                };

                if (offscreenCanvas) {
                    offscreenCanvas.width = width;
                    offscreenCanvas.height = height;
                    refresh_surface(offscreenCanvas);
                }
            }
            break;
    }
};

(globalThis as any).loadImageBitmap = async (
    url: string,
): Promise<ImageBitmap> => {
    if (url.startsWith("bundle:")) {
        url = url.replace(
            "bundle:",
            self.location.origin +
                self.location.pathname.replace("worker.js", "bundle/"),
        );
    }
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error("failed to load image");
    }
    const blob = await response.blob();

    const bitmap = await createImageBitmap(blob, { premultiplyAlpha: "none" });
    return bitmap;
};
(globalThis as any).onLoadImage = () => {
    self.postMessage({
        type: "onLoadImage",
    });
};

let lastDrawnInput: Uint8Array;
let frameCount = 0;

requestAnimationFrame(function onAnimationFrame() {
    try {
        if (!lastRequestedDrawInput) {
            return;
        }

        frameCount++;

        if (lastRequestedDrawInput === lastDrawnInput) {
            return;
        }

        draw(lastRequestedDrawInput);
        lastDrawnInput = lastRequestedDrawInput;
    } finally {
        requestAnimationFrame(onAnimationFrame);
    }
});

setInterval(() => {
    console.log("fps", frameCount);
    frameCount = 0;
}, 1000);
