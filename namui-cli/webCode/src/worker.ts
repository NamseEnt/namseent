import { runAsyncMessageLoop } from "./asyncMessage.js";
import { blockingRequest } from "./messageLoop.js";
import { cacheGet, cacheSet } from "./cache.js";

importScripts("./bundle.js");
importScripts("./canvaskit-wasm/canvaskit.js");

declare var wasm_bindgen: any;
const { start } = wasm_bindgen;
declare var CanvasKitInit: any;

console.log(performance.now());

runAsyncMessageLoop(self, async (message) => {
    console.log("message from main", message);
    switch (message.type) {
        case "init":
            {
                const {
                    workerToMainBufferSab,
                    mainToWorkerBufferSab,
                    offscreenCanvas,
                } = message;

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
                    return offscreenCanvas;
                };

                anyGlobalThis.cacheGet = cacheGet;
                anyGlobalThis.cacheSet = cacheSet;

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

    console.log("hi");

    (globalThis as any).CanvasKit = CanvasKit;
    (globalThis as any).getCanvasKit = () => CanvasKit;

    start();
}

async function initWasm() {
    await wasm_bindgen("./bundle_bg.wasm");
}
