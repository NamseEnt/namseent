import { initCanvasKit } from "./canvasKit";

importScripts("./drawer/bundle.js");

declare var wasm_bindgen: any;
declare var CanvasKit: any;
const { init, draw, load_typeface } = wasm_bindgen;

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
                const { buffer } = event.data as { buffer: ArrayBuffer };
                console.log("requestDraw", buffer.byteLength);
                draw(new Uint8Array(buffer));
            }
            break;
        case "loadTypeface":
            {
                const { typefaceName, buffer } = event.data as {
                    typefaceName: string;
                    buffer: ArrayBuffer;
                };
                console.log("loadTypeface", typefaceName);
                load_typeface(typefaceName, new Uint8Array(buffer));
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

(globalThis as any).loadImageBitmap2 = async (
    url: string,
): Promise<Uint8Array> => {
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
    const arrayBuffer = await response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
};

// runAsyncMessageLoop<AsyncMessageFromMain>(self, async (message) => {
//     switch (message.type) {
//         case "init":
//             {
//                 const {
//                     workerToMainBufferSab,
//                     mainToWorkerBufferSab,
//                     windowWidth,
//                     windowHeight,
//                 } = message;

//                 const cavnasElement = new OffscreenCanvas(
//                     windowWidth,
//                     windowHeight,
//                 );

//                 const anyGlobalThis = globalThis as any;

//                 anyGlobalThis.getBaseUrl = () => {
//                     const { baseUrl } = blockingRequest(
//                         {
//                             type: "getBaseUrl",
//                         },
//                         workerToMainBufferSab,
//                     );
//                     return baseUrl;
//                 };
//                 anyGlobalThis.canvasElement = () => {
//                     return cavnasElement;
//                 };

//                 // anyGlobalThis.cacheGet = async (key: string) => {
//                 //     const value = await cacheGet(key);
//                 //     return value;
//                 // };

//                 // anyGlobalThis.cacheSet = async (key: string, value: any) => {
//                 //     console.log("before cacheSet");
//                 //     await cacheSet(key, value);
//                 //     console.log("after cacheSet");
//                 // };

//                 anyGlobalThis.waitEvent = () => {
//                     const { webEvent }: { webEvent: WebEvent | undefined } =
//                         blockingRequest(
//                             {
//                                 type: "webEvent",
//                             },
//                             workerToMainBufferSab,
//                         );

//                     if (
//                         webEvent &&
//                         webEvent instanceof Object &&
//                         "AsyncFunction" in webEvent
//                     ) {
//                         storeAsyncFunctionResult(
//                             webEvent.AsyncFunction.id,
//                             webEvent.AsyncFunction.result,
//                         );
//                         delete webEvent.AsyncFunction.result;
//                     }

//                     return webEvent;
//                 };

//                 anyGlobalThis.flushCanvas = () => {
//                     const bitmap = cavnasElement.transferToImageBitmap();
//                     sendAsyncRequest(
//                         self,
//                         {
//                             type: "imageBitmap",
//                             imageBitmap: bitmap,
//                         },
//                         [bitmap],
//                     );
//                     return;
//                 };

//                 anyGlobalThis.executeFunctionSyncOnMain = (
//                     args_names: string[],
//                     code: string,
//                     args: any[],
//                 ) => {
//                     const response = blockingRequest(
//                         {
//                             type: "executeFunctionSyncOnMain",
//                             args_names,
//                             code,
//                             args,
//                         },
//                         workerToMainBufferSab,
//                     );
//                     return response;
//                 };

//                 let executeAsyncFunctionId = 0;
//                 anyGlobalThis.startExecuteAsyncFunction = (
//                     argsNames: string[],
//                     code: string,
//                     args: any[],
//                 ) => {
//                     const id = executeAsyncFunctionId++;

//                     sendAsyncRequest(self, {
//                         type: "executeAsyncFunction",
//                         argsNames,
//                         code,
//                         args,
//                         id,
//                     });
//                     return id;
//                 };
//                 anyGlobalThis.getAsyncFunctionResult = (id: number) => {
//                     const result = asyncFunctionResultMap.get(id);
//                     asyncFunctionResultMap.delete(id);
//                     return result;
//                 };

//                 anyGlobalThis.getInitialWindowSize = () => {
//                     return {
//                         width: windowWidth,
//                         height: windowHeight,
//                     };
//                 };

//                 await run();
//             }
//             break;
//     }
// });
