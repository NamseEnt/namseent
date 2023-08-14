import { initCanvasKit } from "./canvasKit";

importScripts("./drawer/bundle.js");

declare var wasm_bindgen: any;
declare var CanvasKit: any;
const { init, draw, load_typeface, load_image } = wasm_bindgen;

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

self.onmessage = async (event) => {
    await initWaiter;

    switch (event.data.type) {
        case "init":
            {
                const { offscreen } = event.data;

                init(offscreen);
            }
            break;
        case "requestDraw":
            {
                const { buffer } = event.data as { buffer: Uint8Array };
                lastRequestedDrawInput = buffer;
            }
            break;
        case "loadTypeface":
            {
                const { typefaceName, buffer } = event.data as {
                    typefaceName: string;
                    buffer: Uint8Array;
                };
                console.log("loadTypeface", typefaceName);
                load_typeface(typefaceName, new Uint8Array(buffer));
            }
            break;
        case "loadImage":
            {
                const { imageSource, imageBitmap } = event.data as {
                    imageSource: Uint8Array;
                    imageBitmap: ImageBitmap;
                };
                load_image(imageSource, imageBitmap);
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

    const bitmap = await createImageBitmap(blob);
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
