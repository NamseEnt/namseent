import { TypedArray } from "./type";
import { encode, decode } from "@msgpack/msgpack";

/*
    [ ] Notification flag
    [ ] ..
    [ ] ..
    [ ] ..
    
    [ ] message type
    [ ] Nothing
    [ ] Nothing
    [ ] Nothing

    [ ] message length
    [ ] ..
    [ ] ..
    [ ] ..

    [ ] message body
    ..
*/

const indexNotificationFlagI32 = 0; // 0: request, 1: response
const indexMessageLengthI32 = 1;
const indexMessageBodyU8 = 8;

export async function runMessageLoopForMain(
    sab: SharedArrayBuffer,
    handleMessage: (request: any) => Promise<any>,
) {
    const i32Buf = new Int32Array(sab);
    while (true) {
        // TODO: Check if the message already sent by the worker. maybe timeout and retry?
        const wait = Atomics.waitAsync(i32Buf, 0, 0);
        if (wait.async) {
            await wait.value;
        }

        const message = readMessage(sab);
        const response = await handleMessage(message);
        writeMessage(response, sab);

        Atomics.notify(i32Buf, indexNotificationFlagI32);
    }
}

function readMessage(sab: SharedArrayBuffer): any {
    const messageLength = new Int32Array(sab)[indexMessageLengthI32];
    const messageBuf = new Uint8Array(sab, indexMessageBodyU8, messageLength);

    // NOTE: this is for error "The provided ArrayBufferView value must not be shared."
    const cloned = new ArrayBuffer(messageBuf.byteLength);
    new Uint8Array(cloned).set(new Uint8Array(messageBuf));

    return decode(cloned);
}

export function writeMessage(message: any, sab: SharedArrayBuffer) {
    const encoded = encode(message);

    const i32Buf = new Int32Array(sab);

    const messageLength = encoded.length;

    i32Buf[indexMessageLengthI32] = messageLength;
    const buffer = new Uint8Array(
        i32Buf.buffer,
        indexMessageBodyU8,
        messageLength,
    );
    buffer.set(encoded);
}

export function blockingRequest(
    request: any,
    requestSab: SharedArrayBuffer,
): any {
    writeMessage(request, requestSab);

    const i32Buf = new Int32Array(requestSab);

    i32Buf[indexNotificationFlagI32] = 0;
    Atomics.notify(i32Buf, indexNotificationFlagI32);
    Atomics.wait(i32Buf, indexNotificationFlagI32, 0);

    const response = readMessage(requestSab);

    return response;
}
