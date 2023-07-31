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

const indexNotificationFlagI32 = 0;
const indexMessageLengthI32 = 1;
const indexMessageBodyU8 = 8;

const notificationFlagIdle = 0;
const notificationFlagReqSent = 1;

export async function runMessageLoopForMain(
    sab: SharedArrayBuffer,
    handleMessage: (request: any) => Promise<any>,
) {
    const i32Buf = new Int32Array(sab);
    while (true) {
        const value = await Atomics.waitAsync(i32Buf, 0, notificationFlagIdle)
            .value;
        if (
            Atomics.load(i32Buf, indexNotificationFlagI32) !==
            notificationFlagReqSent
        ) {
            throw new Error(`Wrong Atomics.waitAsync, ${value}`);
        }

        const message = readMessage(sab);
        const response = await handleMessage(message);
        writeMessage(response, sab);

        i32Buf[indexNotificationFlagI32] = notificationFlagIdle;
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

    if (
        Atomics.load(i32Buf, indexNotificationFlagI32) !== notificationFlagIdle
    ) {
        throw new Error("wrong flag");
    }

    i32Buf[indexNotificationFlagI32] = notificationFlagReqSent;
    Atomics.notify(i32Buf, indexNotificationFlagI32);

    Atomics.wait(i32Buf, indexNotificationFlagI32, notificationFlagReqSent);
    if (
        Atomics.load(i32Buf, indexNotificationFlagI32) ===
        notificationFlagReqSent
    ) {
        throw new Error("wrong waiting code");
    }

    const response = readMessage(requestSab);

    return response;
}
