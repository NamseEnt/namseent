import { RingBufferWriter, StrictArrayBuffer } from "./RingBufferWriter";
import {
    WorkerMessagePayload,
    sendMessageToMainThread,
} from "./interWorkerProtocol";

// # data callback protocol
// [data byte length: u16][message data: ...]

export function insertJsImports({ memory }: { memory: WebAssembly.Memory }) {
    const writtenBuffer = new SharedArrayBuffer(4);

    function insertJs(
        jsPtr: number,
        jsLen: number,
        ringBuffer?: {
            ringBufferPtr: number;
            ringBufferLen: number;
        },
    ): number {
        if (!(memory.buffer instanceof SharedArrayBuffer)) {
            throw new Error("memory.buffer must be SharedArrayBuffer");
        }
        const jsBuffer = new Uint8Array(jsLen);
        jsBuffer.set(new Uint8Array(memory.buffer, jsPtr, jsLen));
        const js = new TextDecoder().decode(jsBuffer);
        const idBuffer = new SharedArrayBuffer(4);

        sendMessageToMainThread({
            type: "insert-js",
            js,
            idBuffer,
            ringBuffer: ringBuffer
                ? {
                      wasmMemory: memory.buffer,
                      ptr: ringBuffer.ringBufferPtr,
                      len: ringBuffer.ringBufferLen,
                      writtenBuffer,
                  }
                : undefined,
        });

        Atomics.wait(new Int32Array(idBuffer), 0, 0);
        const id = new Uint32Array(idBuffer)[0];
        return id;
    }

    return {
        _insert_js: (jsPtr: number, jsLen: number): number => {
            return insertJs(jsPtr, jsLen);
        },
        _insert_js_with_data_callback: (
            jsPtr: number,
            jsLen: number,
            ringBufferPtr: number,
            ringBufferLen: number,
        ): number => {
            return insertJs(jsPtr, jsLen, {
                ringBufferPtr,
                ringBufferLen,
            });
        },
        _insert_js_drop: (jsId: number) => {
            sendMessageToMainThread({
                type: "insert-js-drop",
                id: jsId,
            });
        },
        _insert_js_data_poll: (timeoutMs: number): number => {
            Atomics.wait(new Int32Array(writtenBuffer), 0, 0, timeoutMs);
            return Atomics.load(new Uint32Array(writtenBuffer), 0);
        },
        _insert_js_data_commit: (byteLength: number) => {
            Atomics.sub(new Uint32Array(writtenBuffer), 0, byteLength);
        },
    };
}

export function insertJsHandleOnMainThread(): {
    onInsertJs: (payload: WorkerMessagePayload & { type: "insert-js" }) => void;
    onInsertJsDrop: (
        payload: WorkerMessagePayload & { type: "insert-js-drop" },
    ) => void;
} {
    let nextId = 1;
    const jsMap = new Map<
        number,
        {
            script: HTMLScriptElement;
            ringBuffer?: RingBufferWriter;
        }
    >();

    (window as any).namui_onData = (id: number, data: StrictArrayBuffer) => {
        const js = jsMap.get(id);
        if (!js) {
            console.log(`No js for ${id} found but namui_onData is called.`);
            return;
        }
        if (!js.ringBuffer) {
            console.warn(
                "No ring buffer found. Make sure that you are using `Some(OnData)`.",
            );
            return;
        }

        if (data.byteLength > 0xffff) {
            throw new Error(
                `Data byte length ${data.byteLength} is larger than 0xffff`,
            );
        }
        js.ringBuffer.write(["u16", data.byteLength], ["bytes", data]);
    };

    return {
        onInsertJs({ idBuffer, js, ringBuffer }) {
            const id = nextId++;

            const script = document.createElement("script");

            jsMap.set(id, {
                script,
                ringBuffer: ringBuffer
                    ? new RingBufferWriter(
                          ringBuffer.wasmMemory,
                          ringBuffer.ptr,
                          ringBuffer.len,
                          ringBuffer.writtenBuffer,
                      )
                    : undefined,
            });

            script.textContent = `const namui_sendData = (data) => {
                window.namui_onData(${id}, data);
            }
            ${js}
            window.namui_onDrop_${id} = namui_onDrop;`;
            document.body.appendChild(script);

            Atomics.store(new Int32Array(idBuffer), 0, id);
            Atomics.notify(new Int32Array(idBuffer), 0);
        },
        onInsertJsDrop({ id }) {
            const js = jsMap.get(id);
            if (!js) {
                return;
            }
            const onDropKey = `namui_onDrop_${id}`;
            (window as any)[onDropKey]?.();
            delete (window as any)[onDropKey];
            js.script.remove();
            jsMap.delete(id);
        },
    };
}
