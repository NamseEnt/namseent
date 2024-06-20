import { sendMessageToMainThread } from "./interWorkerProtocol";

/*
# eventBuffer protocol

[ws id: u32][message type: u8][message data: ...]

- 0x01: on open
- 0x02: on close
- 0x03: on small message (<= 64KB)
    - u16: byte length
    - u8[data length]: data
- 0x04: on big message start (< 4GB)
    - u32: total byte length
    - u16: chunk count
- 0x05: on big message chunk
    - u16: chunk byte length
    - u8[data length]: data
*/

export function webSocketImports({ memory }: { memory: WebAssembly.Memory }) {
    // ASSUME: this imports only run on specific one thread, except _new_web_socket and _web_socket_send.

    const writtenBuffer = new SharedArrayBuffer(4);

    return {
        _init_web_socket_thread: (
            eventBufferPtr: number,
            eventBufferLen: number,
        ) => {
            const eventBuffer = memory.buffer.slice(
                eventBufferPtr,
                eventBufferPtr + eventBufferLen,
            );
            if (!(eventBuffer instanceof SharedArrayBuffer)) {
                throw new Error("eventBuffer must be SharedArrayBuffer");
            }

            sendMessageToMainThread({
                type: "init-web-socket-thread",
                eventBuffer,
                writtenBuffer,
            });
        },
        _web_socket_event_poll: (): number => {
            Atomics.wait(new Int32Array(writtenBuffer), 0, 0);
            return Atomics.load(new Int32Array(writtenBuffer), 0);
        },
        _web_socket_event_commit: (len: number) => {
            Atomics.sub(new Int32Array(writtenBuffer), 0, len);
        },
        _new_web_socket: (urlPtr: number, urlLen: number): number => {
            const urlBuffer = new Uint8Array(urlLen);
            urlBuffer.set(new Uint8Array(memory.buffer, urlPtr, urlLen));
            const url = new TextDecoder().decode(urlBuffer);

            const idBuffer = new SharedArrayBuffer(4);
            sendMessageToMainThread({
                type: "new-web-socket",
                url,
                idBuffer,
            });

            Atomics.wait(new Int32Array(idBuffer), 0, 0);
            const id = new Uint32Array(idBuffer)[0];

            return id;
        },
        _web_socket_send: (id: number, data_ptr: number, data_len: number) => {
            const data = new Uint8Array(data_len);
            data.set(new Uint8Array(memory.buffer, data_ptr, data_len));
            sendMessageToMainThread({
                type: "web-socket-send",
                id,
                data,
            });
        },
    };
}

export function webSocketHandleOnMainThread({
    eventBuffer,
    writtenBuffer,
}: {
    eventBuffer: SharedArrayBuffer;
    writtenBuffer: SharedArrayBuffer;
}) {
    let nextId = 1;
    const webSockets = new Map<number, WebSocket>();
    const messages: {
        id: number;
        data: ArrayBuffer;
    }[] = [];
    let writerIndex = 0;

    function onNewWebSocket({
        url,
        idBuffer,
    }: {
        url: string;
        idBuffer: SharedArrayBuffer;
    }) {
        const webSocket = new WebSocket(url);
        const id = nextId++;
        webSockets.set(id, webSocket);

        webSocket.onopen = async () => {
            await write(["u32", id], ["u8", 0x01]);
        };
        webSocket.onclose = async () => {
            await write(["u32", id], ["u8", 0x02]);
        };
        webSocket.onmessage = (event: MessageEvent<ArrayBuffer>) => {
            messages.push({
                id,
                data: event.data,
            });
            if (messages.length === 1) {
                loopSendingMessage();
            }
        };

        new Uint32Array(idBuffer)[0] = id;
        Atomics.notify(new Int32Array(idBuffer), 0);
    }

    function send({ id, data }: { id: number; data: ArrayBuffer }) {
        webSockets.get(id)?.send(data);
    }

    return {
        onNewWebSocket,
        send,
    };

    // below implementations
    /**
     * Only one execution at a time.
     */
    async function loopSendingMessage() {
        const message = messages[0];
        if (!message) {
            return;
        }

        const _64KB = 64 * 1024;
        const { data, id } = message;

        const isSmallMessage = data.byteLength <= _64KB;
        if (isSmallMessage) {
            await write(
                ["u32", id],
                ["u8", 0x03],
                ["u16", data.byteLength],
                ["bytes", data],
            );
        } else {
            const chunkCount = Math.ceil(data.byteLength / _64KB);

            await write(
                ["u32", id],
                ["u8", 0x04],
                ["u32", data.byteLength],
                ["u16", chunkCount],
            );

            for (
                let sentBytes = 0;
                sentBytes < data.byteLength;
                sentBytes += _64KB
            ) {
                const chunkSize = Math.min(_64KB, data.byteLength - sentBytes);

                write(
                    ["u32", id],
                    ["u8", 0x05],
                    ["u16", chunkSize],
                    ["bytes", data.slice(sentBytes, sentBytes + chunkSize)],
                );
            }
        }

        messages.shift();
        loopSendingMessage();
    }

    async function write(
        ...tuples: (
            | ["u8", number]
            | ["u16", number]
            | ["u32", number]
            | ["bytes", ArrayBuffer]
        )[]
    ) {
        const totalByteLength = tuples.reduce((a, b) => {
            switch (b[0]) {
                case "u8":
                    return a + 1;
                case "u16":
                    return a + 2;
                case "u32":
                    return a + 4;
                case "bytes":
                    return a + b[1].byteLength;
            }
        }, 0);

        await waitForBufferAvailable(totalByteLength);

        for (const tuple of tuples) {
            write(tuple);
        }

        Atomics.add(new Int32Array(writtenBuffer), 0, totalByteLength);
        Atomics.notify(new Int32Array(writtenBuffer), 1);

        return; // below implementations

        function write(
            tuple:
                | ["u8", number]
                | ["u16", number]
                | ["u32", number]
                | ["bytes", ArrayBuffer],
        ) {
            if (eventBuffer.byteLength <= writerIndex) {
                writerIndex = 0;
            }

            const type = tuple[0];
            let value: ArrayBuffer;
            switch (type) {
                case "u8": {
                    value = new Uint8Array([tuple[1]]);
                    break;
                }
                case "u16": {
                    value = new Uint16Array([tuple[1]]);
                    break;
                }
                case "u32": {
                    value = new Uint32Array([tuple[1]]);
                    break;
                }
                case "bytes": {
                    value = tuple[1];
                    break;
                }
                default: {
                    throw new Error(`Unsupported type: ${type}`);
                }
            }

            const bufferRight = eventBuffer.byteLength - writerIndex;
            new Uint8Array(eventBuffer).set(
                new Uint8Array(value, 0, bufferRight),
                writerIndex,
            );

            if (value.byteLength <= bufferRight) {
                writerIndex += value.byteLength;
                return value.byteLength;
            }

            const left = value.byteLength - bufferRight;
            new Uint8Array(eventBuffer).set(
                new Uint8Array(value, bufferRight),
                0,
            );
            writerIndex = left;
        }
        async function waitForBufferAvailable(byteLength: number) {
            while (true) {
                const written = Atomics.load(new Int32Array(writtenBuffer), 0);
                const bufferAvailable = eventBuffer.byteLength - written;
                if (byteLength <= bufferAvailable) {
                    return;
                }
                const { async, value } = Atomics.waitAsync(
                    new Int32Array(writtenBuffer),
                    0,
                    written,
                );
                if (async) {
                    await value;
                }
            }
        }
    }
}
