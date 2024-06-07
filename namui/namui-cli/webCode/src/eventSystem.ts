/*
eventBuffer (i32 array)
- 0: number of not consumed events
- 1~n: event packets

event packet
- 0: event type
- 1~n: event data. depends on event type

event type and body
- 0xFF: end of buffer. move buffer u8 index to 4. This also counts as an event.
- 0x00: on animation frame
- 0x01: on resize
    - u16: width
    - u16: height
- 0x02 ~ 0x03: on key down, up
    - u8: code byte length
    - bytes: code
- 0x04 ~ 0x06: on mouse down, move, up
    - u8: button
    - u8: buttons
    - u16: x
    - u16: y
- 0x07: on wheel
    - f32: delta x
    - f32: delta y
    - u16: mouse x
    - u16: mouse y
- 0x08: on blur
- 0x09: on visibility change
*/

const EVENT_TYPE = {
    END_OF_BUFFER: 0xff,
    ANIMATION_FRAME: 0x00,
    RESIZE: 0x01,
    KEY_DOWN: 0x02,
    KEY_UP: 0x03,
    MOUSE_DOWN: 0x04,
    MOUSE_MOVE: 0x05,
    MOUSE_UP: 0x06,
    WHEEL: 0x07,
    BLUR: 0x08,
    VISIBILITY_CHANGE: 0x09,
};

export class EventSystemOnWorker {
    private eventBufferIndex: number = 4;
    private lastEventTime: DOMHighResTimeStamp | undefined;
    constructor(
        private readonly eventBuffer: SharedArrayBuffer,
        private readonly memory: WebAssembly.Memory,
    ) {}

    /**
     * @return {number} the byte length of event.
     */
    public pollEvent(wasmBufferPtr: number): number {
        if (this.lastEventTime) {
            const now = performance.now();
            // console.log("last event handling time: ", now - this.lastEventTime);
            this.lastEventTime = now;
        } else {
            this.lastEventTime = performance.now();
        }

        const eventBufferView = new DataView(this.eventBuffer);
        const eventBufferI32Array = new Int32Array(this.eventBuffer);
        const wasmBuffer = new DataView(this.memory.buffer, wasmBufferPtr, 32);

        Atomics.wait(eventBufferI32Array, 0, 0);
        Atomics.sub(eventBufferI32Array, 0, 1);
        // TODO: Monitor how many event are in queue.

        const eventType = eventBufferView.getUint8(this.eventBufferIndex);
        let packetSize: number;

        switch (eventType) {
            case EVENT_TYPE.END_OF_BUFFER: {
                this.eventBufferIndex = 4;
                return this.pollEvent(wasmBufferPtr);
            }
            case EVENT_TYPE.ANIMATION_FRAME:
                packetSize = 1;
                break;
            case EVENT_TYPE.RESIZE:
                packetSize = 5;
                break;
            case EVENT_TYPE.KEY_DOWN:
            case EVENT_TYPE.KEY_UP: {
                const codeLength = eventBufferView.getUint8(
                    this.eventBufferIndex + 1,
                );
                packetSize = 2 + codeLength;
                break;
            }
            case EVENT_TYPE.MOUSE_DOWN:
            case EVENT_TYPE.MOUSE_MOVE:
            case EVENT_TYPE.MOUSE_UP: {
                packetSize = 7;
                break;
            }
            case EVENT_TYPE.WHEEL: {
                packetSize = 13;
                break;
            }
            case EVENT_TYPE.BLUR:
            case EVENT_TYPE.VISIBILITY_CHANGE: {
                packetSize = 1;
                break;
            }
            default: {
                throw new Error(`Unknown event type: ${eventType}`);
            }
        }

        wasmBuffer.setUint8(0, eventType);
        new Uint8Array(
            this.memory.buffer,
            wasmBufferPtr + 1,
            packetSize - 1,
        ).set(
            new Uint8Array(
                this.eventBuffer,
                this.eventBufferIndex + 1,
                packetSize - 1,
            ),
        );

        this.eventBufferIndex += packetSize;
        return packetSize;
    }
}

export function startEventSystemOnMainThread(eventBuffer: SharedArrayBuffer) {
    let eventBufferIndex = 4;

    const eventBufferView = new DataView(eventBuffer);
    const i32Array = new Int32Array(eventBuffer);

    function checkIndexOverflow() {
        const margin = 32;
        if (eventBufferIndex + margin >= eventBuffer.byteLength) {
            eventBufferView.setUint8(
                eventBufferIndex,
                EVENT_TYPE.END_OF_BUFFER,
            );
            eventBufferIndex = 4;

            Atomics.add(i32Array, 0, 1);
            Atomics.notify(i32Array, 0);
        }
    }

    function onAnimationFrame() {
        checkIndexOverflow();

        eventBufferView.setUint8(eventBufferIndex, EVENT_TYPE.ANIMATION_FRAME);
        eventBufferIndex += 1;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);

        requestAnimationFrame(onAnimationFrame);
    }
    requestAnimationFrame(onAnimationFrame);

    window.addEventListener("resize", () => {
        checkIndexOverflow();

        eventBufferView.setUint8(eventBufferIndex, EVENT_TYPE.RESIZE);
        eventBufferView.setUint16(eventBufferIndex + 1, window.innerWidth);
        eventBufferView.setUint16(eventBufferIndex + 3, window.innerHeight);
        eventBufferIndex += 5;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });

    function onKeyEvent(type: "down" | "up", code: string) {
        checkIndexOverflow();

        eventBufferView.setUint8(
            eventBufferIndex,
            type === "down" ? EVENT_TYPE.KEY_DOWN : EVENT_TYPE.KEY_UP,
        );
        const codeBuffer = new TextEncoder().encode(code);
        eventBufferView.setUint8(eventBufferIndex + 1, codeBuffer.length);
        new Uint8Array(eventBuffer, eventBufferIndex + 2).set(codeBuffer);
        eventBufferIndex += 2 + codeBuffer.length;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    }
    document.addEventListener("keydown", (e) => onKeyEvent("down", e.code));
    document.addEventListener("keyup", (e) => onKeyEvent("up", e.code));

    function onMouseEvent(type: "down" | "move" | "up", e: MouseEvent) {
        checkIndexOverflow();

        eventBufferView.setUint8(
            eventBufferIndex,
            type === "down"
                ? EVENT_TYPE.MOUSE_DOWN
                : type === "move"
                ? EVENT_TYPE.MOUSE_MOVE
                : EVENT_TYPE.MOUSE_UP,
        );
        eventBufferView.setUint8(eventBufferIndex + 1, e.button);
        eventBufferView.setUint8(eventBufferIndex + 2, e.buttons);
        eventBufferView.setUint16(eventBufferIndex + 3, e.clientX);
        eventBufferView.setUint16(eventBufferIndex + 5, e.clientY);
        eventBufferIndex += 7;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    }
    document.addEventListener("mousedown", (e) => onMouseEvent("down", e));
    document.addEventListener("mousemove", (e) => onMouseEvent("move", e));
    document.addEventListener("mouseup", (e) => onMouseEvent("up", e));

    document.addEventListener("wheel", (e) => {
        checkIndexOverflow();

        eventBufferView.setUint8(eventBufferIndex, EVENT_TYPE.WHEEL);
        eventBufferView.setFloat32(eventBufferIndex + 1, e.deltaX);
        eventBufferView.setFloat32(eventBufferIndex + 5, e.deltaY);
        eventBufferView.setUint16(eventBufferIndex + 10, e.clientX);
        eventBufferView.setUint16(eventBufferIndex + 12, e.clientY);
        eventBufferIndex += 14;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });

    window.addEventListener("blur", () => {
        checkIndexOverflow();

        eventBufferView.setUint8(eventBufferIndex, EVENT_TYPE.BLUR);
        eventBufferIndex += 1;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });

    document.addEventListener("visibilitychange", () => {
        checkIndexOverflow();

        eventBufferView.setUint8(
            eventBufferIndex,
            EVENT_TYPE.VISIBILITY_CHANGE,
        );
        eventBufferIndex += 1;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });
}
