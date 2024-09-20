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
        _insert_js_send_data_from_rust: (
            jsId: number,
            sendDataId: number,
            bufferPtr: number,
            bufferLen: number,
        ) => {
            const data = new Uint8Array(bufferLen);
            data.set(new Uint8Array(memory.buffer, bufferPtr, bufferLen));

            sendMessageToMainThread(
                {
                    type: "insert-js-send-data-from-rust",
                    sendDataId,
                    jsId,
                    data,
                },
                [data.buffer],
            );
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
            nextSendDataId: number;
            sendDataQueueMap: Map<number, StrictArrayBuffer>;
        }
    >();

    (window as any).namui_onDataFromJs = (
        jsId: number,
        data: StrictArrayBuffer,
    ) => {
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
                nextSendDataId: 0,
                sendDataQueueMap: new Map(),
            });

            script.textContent = `{
                const namui_sendData = (data) => {
                    window.namui_onDataFromJs(${jsId}, data);
                }
                ${js}
                window.namui_onDrop_${jsId} = namui_onDrop;
                window.namui_onDataFromRust_${jsId} = namui_onData;
            }`;
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
        onInsertJsSendDataFromRust({
            jsId,
            sendDataId,
            data,
        }: WorkerMessagePayload & {
            type: "insert-js-send-data-from-rust";
        }) {
            const js = jsMap.get(jsId);
            if (!js) {
                throw new Error(`No js for ${jsId} found`);
            }

            js.sendDataQueueMap.set(sendDataId, data);

            while (js.sendDataQueueMap.size) {
                const data = js.sendDataQueueMap.get(js.nextSendDataId);
                if (!data) {
                    break;
                }

                js.sendDataQueueMap.delete(js.nextSendDataId);
                js.nextSendDataId++;

                const onDataKey = `namui_onDataFromRust_${jsId}`;
                (window as any)[onDataKey]?.(data);
            }
        },
    };
}
