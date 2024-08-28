import {
    WorkerMessagePayload,
    sendMessageToMainThread,
} from "../interWorkerProtocol";

export function hardwareConcurrencyImports({
    memory,
}: {
    memory: WebAssembly.Memory;
}) {
    function hardwareConcurrency(): number {
        if (!(memory.buffer instanceof SharedArrayBuffer)) {
            throw new Error("memory.buffer must be SharedArrayBuffer");
        }
        const concurrencyBuffer = new SharedArrayBuffer(4);

        sendMessageToMainThread({
            type: "hardware-concurrency",
            concurrencyBuffer,
        });

        Atomics.wait(new Int32Array(concurrencyBuffer), 0, 0);
        const concurrency = new Uint32Array(concurrencyBuffer)[0];
        return concurrency;
    }

    return {
        _hardware_concurrency: (): number => {
            return hardwareConcurrency();
        },
    };
}

export function hardwareConcurrencyHandleOnMainThread(): {
    onHardwareConcurrency: (
        payload: WorkerMessagePayload & { type: "hardware-concurrency" },
    ) => void;
} {
    return {
        onHardwareConcurrency({ concurrencyBuffer }) {
            const concurrency = navigator.hardwareConcurrency;
            Atomics.store(new Int32Array(concurrencyBuffer), 0, concurrency);
            Atomics.notify(new Int32Array(concurrencyBuffer), 0);
        },
    };
}
