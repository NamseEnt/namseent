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

import { Exports } from "./exports";
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

export type OnTextInputEvent = (
    textarea: HTMLTextAreaElement,
    eventType:
        | typeof EVENT_TYPE.TEXT_INPUT
        | typeof EVENT_TYPE.TEXT_INPUT_KEY_DOWN
        | typeof EVENT_TYPE.SELECTION_CHANGE,
    code?: number,
) => void;

export function startEventSystem(instance: WebAssembly.Instance): {
    onTextInputEvent: OnTextInputEvent;
} {
    const exports = instance.exports as Exports;
    const memory = exports.memory;
    function sendEvent(
        packetSize: number,
        on: (buffer: {
            u8: (value: number) => void;
            u16: (value: number) => void;
            u32: (value: number) => void;
            f32: (value: number) => void;
        }) => void,
    ) {
        const ptr = exports.malloc(packetSize);
        const view = new DataView(memory.buffer, ptr, packetSize);
        let index = 0;
        on({
            u8: (value: number) => {
                view.setUint8(index, value);
                index += 1;
            },
            u16: (value: number) => {
                view.setUint16(index, value);
                index += 2;
            },
            u32: (value: number) => {
                view.setUint32(index, value);
                index += 4;
            },
            f32: (value: number) => {
                view.setFloat32(index, value);
                index += 4;
            },
        });
        if (index !== packetSize) {
            throw new Error(
                `Event packet size mismatch: expected ${packetSize}, got ${index}`,
            );
        }
        exports._on_event(ptr, packetSize);
        exports.free(ptr);
    }

    function onAnimationFrame() {
        sendEvent(1, (buffer) => {
            buffer.u8(EVENT_TYPE.ANIMATION_FRAME);
        });

        requestAnimationFrame(onAnimationFrame);
    }
    requestAnimationFrame(onAnimationFrame);

    window.addEventListener("resize", () => {
        sendEvent(5, (buffer) => {
            buffer.u8(EVENT_TYPE.RESIZE);
            buffer.u16(window.innerWidth);
            buffer.u16(window.innerHeight);
        });
    });

    function onKeyEvent(type: "down" | "up", event: KeyboardEvent) {
        const code = CODES[event.code as keyof typeof CODES];
        if (!code) {
            console.warn(`Unknown key code: ${event.code}`);
            return;
        }
        if (!isKeyPreventDefaultException(event)) {
            event.preventDefault();
        }

        sendEvent(2, (buffer) => {
            buffer.u8(
                type === "down" ? EVENT_TYPE.KEY_DOWN : EVENT_TYPE.KEY_UP,
            );
            buffer.u8(code);
        });
    }
    document.addEventListener("keydown", (e) => {
        onKeyEvent("down", e);
    });
    document.addEventListener("keyup", (e) => {
        onKeyEvent("up", e);
    });

    function onMouseEvent(type: "down" | "move" | "up", event: MouseEvent) {
        event.preventDefault();

        sendEvent(7, (buffer) => {
            buffer.u8(
                type === "down"
                    ? EVENT_TYPE.MOUSE_DOWN
                    : type === "move"
                    ? EVENT_TYPE.MOUSE_MOVE
                    : EVENT_TYPE.MOUSE_UP,
            );
            buffer.u8(event.button);
            buffer.u8(event.buttons);
            buffer.u16(event.clientX);
            buffer.u16(event.clientY);
        });
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
        sendEvent(13, (buffer) => {
            buffer.u8(EVENT_TYPE.WHEEL);
            buffer.f32(event.deltaX);
            buffer.f32(event.deltaY);
            buffer.u16(event.clientX);
            buffer.u16(event.clientY);
        });
    });

    window.addEventListener("blur", () => {
        sendEvent(1, (buffer) => {
            buffer.u8(EVENT_TYPE.BLUR);
        });
    });

    document.addEventListener("visibilitychange", () => {
        sendEvent(1, (buffer) => {
            buffer.u8(EVENT_TYPE.VISIBILITY_CHANGE);
        });
    });

    const onTextInputEvent: OnTextInputEvent = (textarea, eventType, code) => {
        const textBuffer = new TextEncoder().encode(textarea.value);

        sendEvent(8 + textBuffer.byteLength + (code ? 1 : 0), (buffer) => {
            buffer.u8(eventType);
            buffer.u16(textBuffer.byteLength);
            buffer.u8(
                textarea.selectionDirection === "forward"
                    ? 1
                    : textarea.selectionDirection === "backward"
                    ? 2
                    : 0,
            );
            buffer.u16(textarea.selectionStart || 0);
            buffer.u16(textarea.selectionEnd || 0);
            if (code) {
                buffer.u8(code);
            }
        });
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
