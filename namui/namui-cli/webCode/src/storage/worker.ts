import { WorkerMessagePayload } from "../interWorkerProtocol";

const waitDisconnectMap = new Map<number, () => void>();
let wasmMemoryResolve: (value: WebAssembly.Memory) => void;
const wasmMemory: Promise<WebAssembly.Memory> = new Promise((resolve) => {
    wasmMemoryResolve = resolve;
});
const storageRootDirPromise = (async () => {
    const root = await navigator.storage.getDirectory();
    return root.getDirectoryHandle("storage", { create: true });
})();

self.onmessage = async (message) => {
    const payload = message.data as WorkerMessagePayload;

    switch (payload.type) {
        case "storage-init":
            {
                wasmMemoryResolve(payload.wasmMemory);
            }
            break;
        case "storage-thread-disconnect":
            {
                const { threadId } = payload;
                waitDisconnectMap.get(threadId)?.();
                waitDisconnectMap.delete(threadId);
            }
            break;
        case "storage-thread-connect":
            {
                const { protocolBuffer, threadId } = payload;
                const protocolInt32 = new Int32Array(protocolBuffer);

                const waitDisconnect = new Promise<typeof DISCONNECTED>(
                    (resolve) => {
                        waitDisconnectMap.set(threadId, () =>
                            resolve(DISCONNECTED),
                        );
                    },
                );

                const implement = implementation({
                    wasmMemory: await wasmMemory,
                    storageRootDir: await storageRootDirPromise,
                    protocolBuffer,
                });

                while (true) {
                    const result = await Promise.any([
                        waitForRequest(protocolInt32),
                        waitDisconnect,
                    ]);
                    if (result == DISCONNECTED) {
                        break;
                    }
                    await implement.onRequest();
                }
            }
            break;
    }
};

const REQUEST_ARRIVED = "REQUEST_ARRIVED";
const DISCONNECTED = "DISCONNECTED";

async function waitForRequest(
    int32: Int32Array,
): Promise<typeof REQUEST_ARRIVED> {
    await Atomics.waitAsync(int32, 0, 0).value;
    return REQUEST_ARRIVED;
}

function implementation({
    wasmMemory,
    storageRootDir,
    protocolBuffer,
}: {
    wasmMemory: WebAssembly.Memory;
    storageRootDir: FileSystemDirectoryHandle;
    protocolBuffer: SharedArrayBuffer;
}) {
    const protInt32 = new Int32Array(protocolBuffer);
    const protUint32 = new Uint32Array(protocolBuffer);
    const responseUint32 = new Uint32Array(protocolBuffer, 4);

    const getKey = (keyPtr: number, keyLen: number) =>
        new TextDecoder().decode(
            new Uint8Array(wasmMemory.buffer, keyPtr, keyLen).slice(),
        );

    const { locks } = navigator;

    let nextFd = 1;
    /*
        rpcBuffer
        - read
            - request
                - buffer ptr
                    - -1 for close
                - buffer len
            - response
                - 0 if not done yet, 1 if EOF
                - byte length copied to the buffer.
        - write
            - request
                - buffer ptr
                    - -1 for close
                    - -2 for flush
                - buffer len
            - response
                - 0 if success, 1 if quota exceeded
    */

    const fdRpcBufferMap = new Map<number, Int32Array>();

    return {
        onRequest: async () => {
            const [requestType, ...args] = protUint32;
            switch (requestType) {
                case REQ.openRead: {
                    const [keyPtr, keyLen] = args;
                    const key = getKey(keyPtr, keyLen);

                    const fdWaitBuffer = new Int32Array(
                        new SharedArrayBuffer(4),
                    );
                    locks.request(
                        `storage-${key}`,
                        {
                            mode: "shared",
                        },
                        async () => {
                            const fileHandle = await storageRootDir
                                .getFileHandle(key, {
                                    create: false,
                                })
                                .catch((err) => {
                                    if (
                                        err instanceof DOMException &&
                                        err.name === "NotFoundError"
                                    ) {
                                        return undefined;
                                    }
                                    throw err;
                                });

                            if (!fileHandle) {
                                fdWaitBuffer[0] = 0;
                                Atomics.notify(fdWaitBuffer, 0);
                                return;
                            }

                            const fd = nextFd++;
                            const rpcBuffer = new Int32Array(
                                new SharedArrayBuffer(8),
                            );
                            fdRpcBufferMap.set(fd, rpcBuffer);

                            fdWaitBuffer[0] = fd;
                            Atomics.notify(fdWaitBuffer, 0);

                            const fileStream = await fileHandle
                                .getFile()
                                .then((x) => x.stream());
                            const reader = fileStream.getReader({
                                mode: "byob",
                            });

                            let firstValue = 0;

                            while (true) {
                                await Atomics.waitAsync(
                                    rpcBuffer,
                                    0,
                                    firstValue,
                                ).value;

                                const [ptr, len] = rpcBuffer;
                                if (ptr === -1) {
                                    rpcBuffer[0] = 0;
                                    Atomics.notify(rpcBuffer, 0);
                                    break;
                                }

                                const tempBuffer = new Uint8Array(len);
                                const { done, value } = await reader.read(
                                    tempBuffer,
                                );

                                if (value) {
                                    new Uint8Array(
                                        wasmMemory.buffer,
                                        ptr,
                                        len,
                                    ).set(value);
                                }

                                rpcBuffer[1] = value?.byteLength ?? 0;
                                rpcBuffer[0] = done ? 1 : 0;
                                firstValue = rpcBuffer[0];
                                Atomics.notify(rpcBuffer, 0);
                            }
                            fdRpcBufferMap.delete(fd);
                        },
                    );

                    await Atomics.waitAsync(fdWaitBuffer, 0, 0).value;
                    const fd = fdWaitBuffer[0];

                    responseUint32[0] = fd;

                    break;
                }
                case REQ.read: {
                    const [fd, bufferPtr, bufferLen] = args;

                    const readRpc = fdRpcBufferMap.get(fd)!;
                    readRpc[0] = bufferPtr;
                    readRpc[1] = bufferLen;

                    Atomics.notify(readRpc, 0);
                    await Atomics.waitAsync(readRpc, 0, bufferPtr).value;

                    responseUint32[0] = readRpc[1];
                    responseUint32[1] = readRpc[0];

                    break;
                }
                case REQ.openWrite: {
                    const [keyPtr, keyLen] = args;
                    const key = getKey(keyPtr, keyLen);

                    const fdWaitBuffer = new Int32Array(
                        new SharedArrayBuffer(4),
                    );
                    locks.request(
                        `storage-${key}`,
                        {
                            mode: "exclusive",
                        },
                        async () => {
                            const fileHandle =
                                await storageRootDir.getFileHandle(key, {
                                    create: true,
                                });

                            if (!fileHandle) {
                                fdWaitBuffer[0] = 0;
                                Atomics.notify(fdWaitBuffer, 0);
                                return;
                            }

                            const fd = nextFd++;
                            const rpcBuffer = new Int32Array(
                                new SharedArrayBuffer(8),
                            );
                            fdRpcBufferMap.set(fd, rpcBuffer);

                            fdWaitBuffer[0] = fd;
                            Atomics.notify(fdWaitBuffer, 0);

                            const syncHandle =
                                await fileHandle.createSyncAccessHandle();

                            syncHandle.truncate(0);

                            while (true) {
                                await Atomics.waitAsync(rpcBuffer, 0, 0).value;

                                const [ptr, len] = rpcBuffer;
                                if (ptr === WRITE_OPS.close) {
                                    break;
                                }

                                if (ptr === WRITE_OPS.flush) {
                                    syncHandle.flush();
                                    rpcBuffer[0] = 0;
                                    Atomics.notify(rpcBuffer, 0);
                                    continue;
                                }

                                try {
                                    const written = syncHandle.write(
                                        new Uint8Array(
                                            wasmMemory.buffer,
                                            ptr,
                                            len,
                                        ),
                                    );
                                    if (written !== len) {
                                        throw new Error(
                                            `Write failed, written: ${written}, expected: ${len}`,
                                        );
                                    }
                                } catch (err) {
                                    if (
                                        err instanceof DOMException &&
                                        err.name === "QuotaExceededError"
                                    ) {
                                        rpcBuffer[0] = 1;
                                        Atomics.notify(rpcBuffer, 0);
                                        break;
                                    }
                                }

                                rpcBuffer[0] = 0;
                                Atomics.notify(rpcBuffer, 0);
                            }
                            syncHandle.flush();
                            syncHandle.close();

                            rpcBuffer[0] = 0;
                            Atomics.notify(rpcBuffer, 0);

                            fdRpcBufferMap.delete(fd);
                        },
                    );

                    await Atomics.waitAsync(fdWaitBuffer, 0, 0).value;
                    const fd = fdWaitBuffer[0];

                    responseUint32[0] = fd;

                    break;
                }
                case REQ.write: {
                    const [fd, bufferPtr, bufferLen] = args;

                    const writeRpc = fdRpcBufferMap.get(fd)!;
                    writeRpc[0] = bufferPtr;
                    writeRpc[1] = bufferLen;

                    Atomics.notify(writeRpc, 0);
                    await Atomics.waitAsync(writeRpc, 0, bufferPtr).value;

                    responseUint32[0] = writeRpc[0];

                    break;
                }
                case REQ.flush: {
                    const [fd] = args;

                    const writeRpc = fdRpcBufferMap.get(fd)!;
                    writeRpc[0] = WRITE_OPS.flush;

                    Atomics.notify(writeRpc, 0);
                    await Atomics.waitAsync(writeRpc, 0, WRITE_OPS.flush).value;

                    break;
                }
                case REQ.close: {
                    const [fd] = args;

                    const rpcBuffer = fdRpcBufferMap.get(fd);
                    if (!rpcBuffer) {
                        break;
                    }

                    rpcBuffer[0] = WRITE_OPS.close;
                    Atomics.notify(rpcBuffer, 0);
                    await Atomics.waitAsync(rpcBuffer, 0, WRITE_OPS.close)
                        .value;

                    break;
                }
                case REQ.delete: {
                    const [keyPtr, keyLen] = args;
                    const key = getKey(keyPtr, keyLen);

                    await locks.request(
                        `storage-${key}`,
                        {
                            mode: "exclusive",
                        },
                        async () => {
                            await storageRootDir
                                .removeEntry(key)
                                .catch((err) => {
                                    if (
                                        err instanceof DOMException &&
                                        err.name === "NotFoundError"
                                    ) {
                                        return undefined;
                                    }
                                    throw err;
                                });
                        },
                    );

                    break;
                }
            }

            protInt32[0] = 0;
            Atomics.notify(protInt32, 0);
        },
    };
}

export const REQ = {
    openRead: 0x01,
    read: 0x02,
    openWrite: 0x03,
    write: 0x04,
    flush: 0x05,
    close: 0x06,
    delete: 0x07,
};

const WRITE_OPS = {
    close: -1,
    flush: -2,
};
