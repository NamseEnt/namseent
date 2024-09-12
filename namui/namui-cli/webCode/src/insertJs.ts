import { StrictArrayBuffer } from "./RingBufferWriter";
import {
    WorkerMessagePayload,
    sendMessageToMainThread,
} from "./interWorkerProtocol";
import { NewEventSystemHandleOnMainThread } from "./newEventSystem";

// # data callback protocol
// [data byte length: u16][message data: ...]

export function insertJsImports({ memory }: { memory: WebAssembly.Memory }) {
    return {
        _insert_js: (jsPtr: number, jsLen: number, jsId: number) => {
            if (!(memory.buffer instanceof SharedArrayBuffer)) {
                throw new Error("memory.buffer must be SharedArrayBuffer");
            }
            const jsBuffer = new Uint8Array(jsLen);
            jsBuffer.set(new Uint8Array(memory.buffer, jsPtr, jsLen));
            const js = new TextDecoder().decode(jsBuffer);

            sendMessageToMainThread({
                type: "insert-js",
                js,
                jsId,
            });
        },
        _insert_js_drop: (jsId: number) => {
            sendMessageToMainThread({
                type: "insert-js-drop",
                jsId,
            });
        },
        _insert_js_data_buffer: (
            jsId: number,
            requestId: number,
            bufferPtr: number,
        ) => {
            sendMessageToMainThread({
                type: "insert-js-data-buffer",
                jsId,
                requestId,
                bufferPtr,
            });
        },
    };
}

export function insertJsHandleOnMainThread(
    newEventSystemHandle: NewEventSystemHandleOnMainThread,
    memory: WebAssembly.Memory,
) {
    let nextRequestId = 0;
    const jsMap = new Map<
        number,
        {
            script: HTMLScriptElement;
            dataBufferRequestMap: Map<number, StrictArrayBuffer>;
        }
    >();

    (window as any).namui_onData = (jsId: number, data: StrictArrayBuffer) => {
        const js = jsMap.get(jsId);
        if (!js) {
            console.log(`No js for ${jsId} found but namui_onData is called.`);
            return;
        }

        const requestId = nextRequestId++;

        js.dataBufferRequestMap.set(requestId, data);

        newEventSystemHandle.sendEvent({
            type: "insert-js/request-data-buffer",
            jsId: ["u32", jsId],
            bufferLen: ["u32", data.byteLength],
            requestId: ["u32", requestId],
        });
    };

    return {
        onInsertJs({ js, jsId }: WorkerMessagePayload & { type: "insert-js" }) {
            const script = document.createElement("script");

            jsMap.set(jsId, {
                script,
                dataBufferRequestMap: new Map(),
            });

            script.textContent = `const namui_sendData = (data) => {
                window.namui_onData(${jsId}, data);
            }
            ${js}
            window.namui_onDrop_${jsId} = namui_onDrop;`;
            document.body.appendChild(script);
        },
        onInsertJsDrop({
            jsId,
        }: WorkerMessagePayload & { type: "insert-js-drop" }) {
            const js = jsMap.get(jsId);
            if (!js) {
                throw new Error(`No js for ${jsId} found`);
            }
            const onDropKey = `namui_onDrop_${jsId}`;
            (window as any)[onDropKey]?.();
            delete (window as any)[onDropKey];
            js.script.remove();
            jsMap.delete(jsId);
        },

        onInsertJsDataBuffer({
            bufferPtr,
            requestId,
            jsId,
        }: WorkerMessagePayload & {
            type: "insert-js-data-buffer";
        }) {
            const js = jsMap.get(jsId);
            if (!js) {
                throw new Error(`No js for ${jsId} found`);
            }
            const data = js.dataBufferRequestMap.get(requestId);
            if (!data) {
                throw new Error(`No data for ${requestId} found`);
            }

            new Uint8Array(memory.buffer, bufferPtr).set(new Uint8Array(data));

            js.dataBufferRequestMap.delete(requestId);

            newEventSystemHandle.sendEvent({
                type: "insert-js/data",
                jsId: ["u32", jsId],
                requestId: ["u32", requestId],
            });
        },
    };
}
