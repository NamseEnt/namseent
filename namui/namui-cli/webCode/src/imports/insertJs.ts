import { sendMessageToMainThread } from "../interWorkerProtocol";

export function insertJsImports({ memory }: { memory: WebAssembly.Memory }) {
    return {
        _insert_js: (jsPtr: number, jsLen: number): number => {
            const js = new TextDecoder().decode(
                new Uint8Array(memory.buffer, jsPtr, jsLen),
            );
            const idBuffer = new SharedArrayBuffer(4);
            sendMessageToMainThread(
                {
                    type: "insert-js",
                    js,
                    idBuffer,
                },
                [idBuffer],
            );
            Atomics.wait(new Int32Array(idBuffer), 0, 0);
            const id = new Uint32Array(idBuffer)[0];
            return id;
        },
        _drop_js: (jsId: number) => {
            sendMessageToMainThread({
                type: "drop-js",
                id: jsId,
            });
        },
    };
}
