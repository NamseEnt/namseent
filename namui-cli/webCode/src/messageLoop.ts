// sab i32 [0] = notification flag. 0: request, 1: response
// sab i32 [1] = message length
// sab u8 [8~] = message body

export async function runMessageLoop(
    sab: SharedArrayBuffer,
    handleMessage: (request: any) => any,
) {
    const i32Buf = new Int32Array(sab);
    while (true) {
        // TODO: Check if the message already sent by the worker. maybe timeout and retry?
        const wait = Atomics.waitAsync(i32Buf, 0, 0);
        if (wait.async) {
            await wait.value;
        }

        const message = readMessage(sab);
        const response = handleMessage(message);
        writeMessage(response, i32Buf);
        Atomics.notify(i32Buf, 0);
    }
}

function readMessage(sab: SharedArrayBuffer): any {
    const messageLength = new Int32Array(sab)[1];
    const messageBuf = new Uint8Array(sab, 8, messageLength);

    // NOTE: this is for error "The provided ArrayBufferView value must not be shared."
    const cloned = new ArrayBuffer(messageBuf.byteLength);
    new Uint8Array(cloned).set(new Uint8Array(messageBuf));

    const textDecoder = new TextDecoder();
    const message = textDecoder.decode(cloned);
    return JSON.parse(message);
}

export function writeMessage(message: any, i32Buf: Int32Array) {
    const textEncoder = new TextEncoder();
    const messageBuf = textEncoder.encode(JSON.stringify(message));
    const messageLength = messageBuf.length;
    console.log("messageLength", messageLength);

    i32Buf[1] = messageLength;
    const buffer = new Uint8Array(i32Buf.buffer, 8, messageLength);
    buffer.set(messageBuf);
}

export function blockingRequest(
    request: any,
    requestSab: SharedArrayBuffer,
): any {
    const i32Buf = new Int32Array(requestSab);

    console.log("request", request);
    writeMessage(request, i32Buf);

    i32Buf[0] = 0;
    Atomics.notify(i32Buf, 0);
    Atomics.wait(i32Buf, 0, 0);

    const response = readMessage(requestSab);
    console.log("response", response);

    return response;
}
