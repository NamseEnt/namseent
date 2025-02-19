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
    - u8: code
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
- 0x0A ~ 0x0B: on text input, selection change
    - u16: text byte length
    - bytes: text
    - u8: selection direction. 0: none, 1: forward, 2: backward
    - u16: selection start
    - u16: selection end
- 0x0C: on text input key down
    - u16: text byte length
    - bytes: text
    - u8: selection direction. 0: none, 1: forward, 2: backward
    - u16: selection start
    - u16: selection end
    - u8: code
*/

import { CODES } from "./imports/codes";

export const EVENT_TYPE = {
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
    TEXT_INPUT: 0x0a,
    SELECTION_CHANGE: 0x0b,
    TEXT_INPUT_KEY_DOWN: 0x0c,
};

export class EventSystemOnWorker {
    private eventBufferIndex: number = 4;
    constructor(
        private readonly eventBuffer: SharedArrayBuffer,
        private readonly memory: WebAssembly.Memory,
    ) {}

    /**
     * @return {number} the byte length of event. 0 if no event.
     */
    public pollEvent(wasmBufferPtr: number, waitTimeoutMs: number): number {
        const eventBufferView = new DataView(this.eventBuffer);
        const eventBufferI32Array = new Int32Array(this.eventBuffer);
        const wasmBuffer = new DataView(this.memory.buffer, wasmBufferPtr, 32);

        if (waitTimeoutMs) {
            Atomics.wait(eventBufferI32Array, 0, 0, waitTimeoutMs);
        } else {
            if (Atomics.load(eventBufferI32Array, 0) === 0) {
                return 0;
            }
        }

        Atomics.sub(eventBufferI32Array, 0, 1);
        // TODO: Monitor how many event are in queue.

        const eventType = eventBufferView.getUint8(this.eventBufferIndex);
        let packetSize: number;

        switch (eventType) {
            case EVENT_TYPE.END_OF_BUFFER: {
                this.eventBufferIndex = 4;
                return this.pollEvent(wasmBufferPtr, waitTimeoutMs);
            }
            case EVENT_TYPE.ANIMATION_FRAME:
                packetSize = 1;
                break;
            case EVENT_TYPE.RESIZE:
                packetSize = 5;
                break;
            case EVENT_TYPE.KEY_DOWN:
            case EVENT_TYPE.KEY_UP: {
                packetSize = 2;
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
            case EVENT_TYPE.TEXT_INPUT:
            case EVENT_TYPE.SELECTION_CHANGE: {
                packetSize =
                    8 + eventBufferView.getUint16(this.eventBufferIndex + 1);
                break;
            }
            case EVENT_TYPE.TEXT_INPUT_KEY_DOWN: {
                packetSize =
                    8 +
                    eventBufferView.getUint16(this.eventBufferIndex + 1) +
                    1;
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

export type OnTextInputEvent = (
    textarea: HTMLTextAreaElement,
    eventType:
        | typeof EVENT_TYPE.TEXT_INPUT
        | typeof EVENT_TYPE.TEXT_INPUT_KEY_DOWN
        | typeof EVENT_TYPE.SELECTION_CHANGE,
    code?: number,
) => void;

export function startEventSystemOnMainThread(eventBuffer: SharedArrayBuffer): {
    onTextInputEvent: OnTextInputEvent;
} {
    let eventBufferIndex = 4;

    const eventBufferView = new DataView(eventBuffer);
    const i32Array = new Int32Array(eventBuffer);

    function checkIndexOverflow(packetSize: number) {
        const margin = packetSize + 8;
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
        const packetSize = 1;
        checkIndexOverflow(packetSize);

        eventBufferView.setUint8(eventBufferIndex, EVENT_TYPE.ANIMATION_FRAME);
        eventBufferIndex += packetSize;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);

        requestAnimationFrame(onAnimationFrame);
    }
    requestAnimationFrame(onAnimationFrame);

    window.addEventListener("resize", () => {
        const packetSize = 5;
        checkIndexOverflow(packetSize);

        eventBufferView.setUint8(eventBufferIndex, EVENT_TYPE.RESIZE);
        eventBufferView.setUint16(eventBufferIndex + 1, window.innerWidth);
        eventBufferView.setUint16(eventBufferIndex + 3, window.innerHeight);
        eventBufferIndex += 5;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });

    function onKeyEvent(type: "down" | "up", event: KeyboardEvent) {
        const packetSize = 2;

        const code = CODES[event.code as keyof typeof CODES];
        if (!code) {
            console.warn(`Unknown key code: ${event.code}`);
            return;
        }
        if (!isKeyPreventDefaultException(event)) {
            event.preventDefault();
        }

        checkIndexOverflow(packetSize);

        eventBufferView.setUint8(
            eventBufferIndex,
            type === "down" ? EVENT_TYPE.KEY_DOWN : EVENT_TYPE.KEY_UP,
        );
        eventBufferView.setUint8(eventBufferIndex + 1, code);
        eventBufferIndex += 2;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    }
    document.addEventListener("keydown", (e) => {
        onKeyEvent("down", e);
    });
    document.addEventListener("keyup", (e) => {
        onKeyEvent("up", e);
    });

    function onMouseEvent(type: "down" | "move" | "up", event: MouseEvent) {
        event.preventDefault();
        const packetSize = 7;
        checkIndexOverflow(packetSize);

        eventBufferView.setUint8(
            eventBufferIndex,
            type === "down"
                ? EVENT_TYPE.MOUSE_DOWN
                : type === "move"
                ? EVENT_TYPE.MOUSE_MOVE
                : EVENT_TYPE.MOUSE_UP,
        );
        eventBufferView.setUint8(eventBufferIndex + 1, event.button);
        eventBufferView.setUint8(eventBufferIndex + 2, event.buttons);
        eventBufferView.setUint16(eventBufferIndex + 3, event.clientX);
        eventBufferView.setUint16(eventBufferIndex + 5, event.clientY);
        eventBufferIndex += 7;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    }
    document.addEventListener("mousedown", (e) => {
        onMouseEvent("down", e);
    });
    document.addEventListener("mousemove", (e) => {
        onMouseEvent("move", e);
    });
    document.addEventListener("mouseup", (e) => {
        onMouseEvent("up", e);
    });

    document.addEventListener("wheel", (event) => {
        const packetSize = 13;
        checkIndexOverflow(packetSize);

        eventBufferView.setUint8(eventBufferIndex, EVENT_TYPE.WHEEL);
        eventBufferView.setFloat32(eventBufferIndex + 1, event.deltaX);
        eventBufferView.setFloat32(eventBufferIndex + 5, event.deltaY);
        eventBufferView.setUint16(eventBufferIndex + 9, event.clientX);
        eventBufferView.setUint16(eventBufferIndex + 11, event.clientY);
        eventBufferIndex += 13;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });

    window.addEventListener("blur", () => {
        const packetSize = 1;
        checkIndexOverflow(packetSize);

        eventBufferView.setUint8(eventBufferIndex, EVENT_TYPE.BLUR);
        eventBufferIndex += 1;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });

    document.addEventListener("visibilitychange", () => {
        const packetSize = 1;
        checkIndexOverflow(packetSize);

        eventBufferView.setUint8(
            eventBufferIndex,
            EVENT_TYPE.VISIBILITY_CHANGE,
        );
        eventBufferIndex += 1;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    });

    const onTextInputEvent: OnTextInputEvent = (textarea, eventType, code) => {
        const textBuffer = new TextEncoder().encode(textarea.value);

        const packetSize = 8 + textBuffer.byteLength + (code ? 1 : 0);

        checkIndexOverflow(packetSize);

        eventBufferView.setUint8(eventBufferIndex, eventType);
        eventBufferView.setUint16(eventBufferIndex + 1, textBuffer.byteLength);
        new Uint8Array(eventBuffer, eventBufferIndex + 3).set(textBuffer);
        eventBufferView.setUint8(
            eventBufferIndex + 3 + textBuffer.byteLength,
            textarea.selectionDirection === "forward"
                ? 1
                : textarea.selectionDirection === "backward"
                ? 2
                : 0,
        );
        eventBufferView.setUint16(
            eventBufferIndex + 4 + textBuffer.byteLength,
            textarea.selectionStart || 0,
        );
        eventBufferView.setUint16(
            eventBufferIndex + 6 + textBuffer.byteLength,
            textarea.selectionEnd || 0,
        );
        if (code) {
            eventBufferView.setUint8(
                eventBufferIndex + 8 + textBuffer.byteLength,
                code,
            );
        }
        eventBufferIndex += packetSize;

        Atomics.add(i32Array, 0, 1);
        Atomics.notify(i32Array, 0);
    };

    return { onTextInputEvent };
}

export function isKeyPreventDefaultException(event: KeyboardEvent): boolean {
    // TODO: Maybe we have to disable this on production.
    const isDevTools =
        event.code === "F12" ||
        (event.ctrlKey && event.shiftKey && event.code === "KeyI");
    const isReload =
        event.code === "F5" || (event.ctrlKey && event.code === "KeyR");

    return isDevTools || isReload;
}
