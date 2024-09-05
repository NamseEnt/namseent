import {
    sendMessageToMainThread,
    WorkerMessagePayload,
} from "./interWorkerProtocol";
import { RingBufferInputs, RingBufferWriter } from "./RingBufferWriter";

// TODO: Move other event related code to this system.

/*
# eventBuffer protocol
[message type: u8][message data: ...]
*/

type U8T<T extends number> = ["u8", T];
// @ts-ignore unused
type U8 = U8T<number>;
type U16 = ["u16", number];
type U32 = ["u32", number];
type Bytes = ["bytes", Uint8Array];

export type NewSystemEvent =
    | {
          type: "http-fetch/on-response";
          fetchId: U32;
          status: U16;
          headerCount: U16;
          headers: {
              keyByteLength: U16;
              key: Bytes;
              valueByteLength: U16;
              value: Bytes;
          }[];
      }
    | {
          type: "http-fetch/on-response-body-chunk";
          fetchId: U32;
          pooledBufferPtr: U32;
          written: U32;
      }
    | {
          type: "http-fetch/on-response-body-done";
          fetchId: U32;
      }
    | {
          type: "http-fetch/on-error";
          fetchId: U32;
          messageByteLength: U32;
          message: Bytes;
      }
    | {
          type: "buffer-pool/request-buffer";
      };

export function newEventSystemImports({
    memory,
}: {
    memory: WebAssembly.Memory;
}) {
    const writtenBuffer = new SharedArrayBuffer(4);

    return {
        _new_event_system_init_thread: (
            eventBufferPtr: number,
            eventBufferLen: number,
        ) => {
            if (!(memory.buffer instanceof SharedArrayBuffer)) {
                throw new Error("memory.buffer must be SharedArrayBuffer");
            }

            sendMessageToMainThread({
                type: "init-new-event-system-thread",
                wasmMemory: memory,
                writtenBuffer,
                eventBufferPtr,
                eventBufferLen,
            });
        },
        _new_event_system_event_poll: (): number => {
            Atomics.wait(new Int32Array(writtenBuffer), 0, 0);
            return Atomics.load(new Int32Array(writtenBuffer), 0);
        },
        _new_event_system_event_commit: (len: number) => {
            Atomics.sub(new Int32Array(writtenBuffer), 0, len);
        },
    };
}

export class NewEventSystemHandleOnMainThread {
    private readonly ringBuffer: RingBufferWriter;

    constructor({
        wasmMemory,
        eventBufferPtr,
        eventBufferLen,
        writtenBuffer,
    }: WorkerMessagePayload & {
        type: "init-new-event-system-thread";
    }) {
        this.ringBuffer = new RingBufferWriter(
            wasmMemory.buffer,
            eventBufferPtr,
            eventBufferLen,
            writtenBuffer,
        );
    }

    public sendEvent(event: NewSystemEvent) {
        const input: RingBufferInputs = [];
        switch (event.type) {
            case "http-fetch/on-response": {
                input.push(["u8", 1]);
                input.push(event.fetchId);
                input.push(event.status);
                input.push(event.headerCount);
                for (const header of event.headers) {
                    input.push(header.keyByteLength);
                    input.push(header.key);
                    input.push(header.valueByteLength);
                    input.push(header.value);
                }
                break;
            }
            case "http-fetch/on-response-body-chunk": {
                input.push(["u8", 2]);
                input.push(event.fetchId);
                input.push(event.pooledBufferPtr);
                input.push(event.written);
                break;
            }
            case "http-fetch/on-response-body-done": {
                input.push(["u8", 3]);
                input.push(event.fetchId);
                break;
            }
            case "http-fetch/on-error": {
                input.push(["u8", 4]);
                input.push(event.fetchId);
                input.push(event.messageByteLength);
                input.push(event.message);
                break;
            }
            case "buffer-pool/request-buffer": {
                input.push(["u8", 5]);
                break;
            }
            default: {
                throw new Error(`Unknown event! ${JSON.stringify(event)}`);
            }
        }
        this.ringBuffer.write(...input);
    }
}
