export function localStorageImports({
    memory,
}: {
    memory: WebAssembly.Memory;
}) {
    let temp: Uint8Array;
    return {
        _local_storage_get_start: (keyPtr: number, keyLen: number): number => {
            const key = new TextDecoder().decode(
                new Uint8Array(memory.buffer, keyPtr, keyLen).slice(),
            );
            const valueBase64 = localStorage.getItem(key);
            if (valueBase64 === null) {
                return -1;
            }
            const value = Uint8Array.from(atob(valueBase64), (c) =>
                c.charCodeAt(0),
            );
            temp = value;
            return value.byteLength;
        },
        _local_storage_get_end: (valuePtr: number): void => {
            if (!temp) {
                throw new Error(
                    `No value to read, call _local_storage_get_start first`,
                );
            }
            new Uint8Array(memory.buffer, valuePtr).set(temp);
        },
        _local_storage_set: (
            keyPtr: number,
            keyLen: number,
            valuePtr: number,
            valueLen: number,
        ): void => {
            const key = new TextDecoder().decode(
                new Uint8Array(memory.buffer, keyPtr, keyLen).slice(),
            );
            if (valuePtr === 0) {
                localStorage.removeItem(key);
                return;
            }
            const value = new Uint8Array(
                memory.buffer,
                valuePtr,
                valueLen,
            ).slice();
            localStorage.setItem(key, btoa(String.fromCharCode(...value)));
        },
    };
}
