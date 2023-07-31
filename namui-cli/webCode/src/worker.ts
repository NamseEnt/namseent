import { runAsyncMessageLoop, sendAsyncRequest } from "./asyncMessage.js";
import { blockingRequest } from "./messageLoop.js";
import { cacheGet, cacheSet } from "./cache.js";

importScripts("./bundle.js");
importScripts("./canvaskit-wasm/canvaskit.js");

declare var wasm_bindgen: any;
const { start } = wasm_bindgen;
declare var CanvasKitInit: any;

runAsyncMessageLoop(self, async (message) => {
    switch (message.type) {
        case "init":
            {
                const {
                    workerToMainBufferSab,
                    mainToWorkerBufferSab,
                    windowWidth,
                    windowHeight,
                }: {
                    workerToMainBufferSab: SharedArrayBuffer;
                    mainToWorkerBufferSab: SharedArrayBuffer;
                    windowWidth: number;
                    windowHeight: number;
                } = message;

                const cavnasElement = new OffscreenCanvas(
                    windowWidth,
                    windowHeight,
                );

                const anyGlobalThis = globalThis as any;

                anyGlobalThis.getBaseUrl = () => {
                    const { baseUrl } = blockingRequest(
                        {
                            type: "getBaseUrl",
                        },
                        workerToMainBufferSab,
                    );
                    return baseUrl;
                };
                anyGlobalThis.canvasElement = () => {
                    return cavnasElement;
                };

                anyGlobalThis.cacheGet = cacheGet;
                anyGlobalThis.cacheSet = cacheSet;

                anyGlobalThis.waitEvent = () => {
                    const { webEvent } = blockingRequest(
                        {
                            type: "webEvent",
                        },
                        workerToMainBufferSab,
                    );
                    return webEvent;
                };

                anyGlobalThis.flushCanvas = () => {
                    const bitmap = cavnasElement.transferToImageBitmap();
                    sendAsyncRequest(
                        self,
                        {
                            type: "imageBitmap",
                            imageBitmap: bitmap,
                        },
                        [bitmap],
                    );
                    return;
                };

                await run();
            }
            break;
    }
});

async function run() {
    const [_, CanvasKit] = await Promise.all([
        initWasm(),
        CanvasKitInit({
            locateFile: (file: string) => "./canvaskit-wasm/" + file,
        }),
    ]);

    (globalThis as any).CanvasKit = CanvasKit;
    (globalThis as any).getCanvasKit = () => CanvasKit;

    start();
}

async function initWasm() {
    await wasm_bindgen("./bundle_bg.wasm");
}
