export function kvStoreImports({
    memory,
    kvStoreTarget,
}: {
    memory: WebAssembly.Memory;
    kvStoreTarget: Worker | MessagePort | null;
}) {
    function ensureTarget(): Worker | MessagePort {
        if (!kvStoreTarget) {
            throw new Error("kv_store is not available on this thread");
        }
        return kvStoreTarget;
    }

    function readKey(keyPtr: number, keyLen: number): string {
        return new TextDecoder().decode(
            new Uint8Array(memory.buffer, keyPtr, keyLen).slice(),
        );
    }

    return {
        _kv_store_get(requestId: number, keyPtr: number, keyLen: number) {
            const target = ensureTarget();
            const key = readKey(keyPtr, keyLen);
            target.postMessage({ requestId, op: "get", key });
        },
        _kv_store_put(
            requestId: number,
            keyPtr: number,
            keyLen: number,
            valuePtr: number,
            valueLen: number,
        ) {
            const target = ensureTarget();
            const key = readKey(keyPtr, keyLen);
            if (valuePtr === 0) {
                target.postMessage({ requestId, op: "delete", key });
            } else {
                const value = new Uint8Array(memory.buffer, valuePtr, valueLen).slice();
                target.postMessage({ requestId, op: "put", key, value });
            }
        },
    };
}
