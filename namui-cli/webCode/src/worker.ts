import { runAsyncMessageLoop, sendAsyncRequest } from "./asyncMessage";
import { cacheGet, cacheSet } from "./cache";
import { WebEvent } from "./main/webEvent";
import { blockingRequest } from "./messageLoop";
import { AsyncMessageFromMain } from "./type";

importScripts("./bundle.js");
importScripts("./canvaskit-wasm/canvaskit.js");

declare var wasm_bindgen: any;
const { start } = wasm_bindgen;
declare var CanvasKitInit: any;

runAsyncMessageLoop<AsyncMessageFromMain>(self, async (message) => {
    switch (message.type) {
        case "init":
            {
                const {
                    workerToMainBufferSab,
                    mainToWorkerBufferSab,
                    windowWidth,
                    windowHeight,
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

                // anyGlobalThis.cacheGet = async (key: string) => {
                //     const value = await cacheGet(key);
                //     return value;
                // };

                // anyGlobalThis.cacheSet = async (key: string, value: any) => {
                //     console.log("before cacheSet");
                //     await cacheSet(key, value);
                //     console.log("after cacheSet");
                // };

                anyGlobalThis.waitEvent = () => {
                    const { webEvent }: { webEvent: WebEvent | undefined } =
                        blockingRequest(
                            {
                                type: "webEvent",
                            },
                            workerToMainBufferSab,
                        );

                    if (
                        webEvent &&
                        webEvent instanceof Object &&
                        "AsyncFunction" in webEvent
                    ) {
                        storeAsyncFunctionResult(
                            webEvent.AsyncFunction.id,
                            webEvent.AsyncFunction.result,
                        );
                        delete webEvent.AsyncFunction.result;
                    }

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

                anyGlobalThis.executeFunctionSyncOnMain = (
                    args_names: string[],
                    code: string,
                    args: any[],
                ) => {
                    const response = blockingRequest(
                        {
                            type: "executeFunctionSyncOnMain",
                            args_names,
                            code,
                            args,
                        },
                        workerToMainBufferSab,
                    );
                    return response;
                };

                let executeAsyncFunctionId = 0;
                anyGlobalThis.startExecuteAsyncFunction = (
                    argsNames: string[],
                    code: string,
                    args: any[],
                ) => {
                    const id = executeAsyncFunctionId++;
                    sendAsyncRequest(self, {
                        type: "executeAsyncFunction",
                        argsNames,
                        code,
                        args,
                        id,
                    });
                    return id;
                };
                anyGlobalThis.getAsyncFunctionResult = (id: number) => {
                    const result = asyncFunctionResultMap.get(id);
                    asyncFunctionResultMap.delete(id);
                    return result;
                };

                anyGlobalThis.getInitialWindowSize = () => {
                    return {
                        width: windowWidth,
                        height: windowHeight,
                    };
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

const asyncFunctionResultMap = new Map<number, any>();
function storeAsyncFunctionResult(id: number, result: any) {
    asyncFunctionResultMap.set(id, result);
}
