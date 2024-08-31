import { RingBufferWriter } from "./RingBufferWriter";
import {
    WorkerMessagePayload,
    sendMessageToMainThread,
} from "./interWorkerProtocol";

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
            if (!(memory.buffer instanceof SharedArrayBuffer)) {
                throw new Error("memory.buffer must be SharedArrayBuffer");
            }

            sendMessageToMainThread({
                type: "init-web-socket-thread",
                wasmMemory: memory,
                writtenBuffer,
                eventBufferPtr,
                eventBufferLen,
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
            sendMessageToMainThread(
                {
                    type: "web-socket-send",
                    id,
                    data: data.buffer,
                },
                [data.buffer],
            );
        },
    };
}

export function webSocketHandleOnMainThread({
    wasmMemory,
    writtenBuffer,
    eventBufferPtr,
    eventBufferLen,
}: WorkerMessagePayload & { type: "init-web-socket-thread" }) {
    let nextId = 1;
    const webSockets = new Map<number, WebSocket>();
    const events: (
        | {
              type: "message";
              id: number;
              data: ArrayBuffer;
          }
        | {
              type: "onopen" | "onclose";
              id: number;
          }
    )[] = [];
    function writeEvent(event: (typeof events)[number]) {
        const _64KB = 64 * 1024;
        switch (event.type) {
            case "onopen": {
                ringBuffer.write(["u32", event.id], ["u8", 0x01]);
                break;
            }
            case "onclose": {
                ringBuffer.write(["u32", event.id], ["u8", 0x02]);
                break;
            }
            case "message": {
                const { data, id } = event;

                const isSmallMessage = data.byteLength <= _64KB;
                if (isSmallMessage) {
                    ringBuffer.write(
                        ["u32", id],
                        ["u8", 0x03],
                        ["u16", data.byteLength],
                        ["bytes", data],
                    );
                    break;
                }

                const chunkCount = Math.ceil(data.byteLength / _64KB);

                ringBuffer.write(
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
                    const chunkSize = Math.min(
                        _64KB,
                        data.byteLength - sentBytes,
                    );

                    ringBuffer.write(
                        ["u32", id],
                        ["u8", 0x05],
                        ["u16", chunkSize],
                        ["bytes", data.slice(sentBytes, sentBytes + chunkSize)],
                    );
                }
            }
        }
    }
    const ringBuffer = new RingBufferWriter(
        wasmMemory.buffer,
        eventBufferPtr,
        eventBufferLen,
        writtenBuffer,
    );

    function onNewWebSocket({
        url,
        idBuffer,
    }: {
        url: string;
        idBuffer: SharedArrayBuffer;
    }) {
        const webSocket = new WebSocket(url);
        webSocket.binaryType = "arraybuffer";
        const id = nextId++;
        webSockets.set(id, webSocket);

        webSocket.onopen = async () => {
            writeEvent({ type: "onopen", id });
        };
        webSocket.onclose = async () => {
            writeEvent({ type: "onclose", id });
        };
        webSocket.onmessage = (event: MessageEvent) => {
            writeEvent({
                type: "message",
                id,
                data:
                    typeof event.data === "string"
                        ? new TextEncoder().encode(event.data)
                        : event.data,
            });
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
}
