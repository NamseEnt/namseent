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

import { DrawerExports, Exports } from "./exports";
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

export function startEventSystem({
    exports,
    drawerExports,
    canvas,
}: {
    exports: Exports;
    drawerExports: DrawerExports;
    canvas: HTMLCanvasElement;
}): {
    onTextInputEvent: OnTextInputEvent;
} {
    let mouseX = 0;
    let mouseY = 0;

    const memory = exports.memory;

    function onEventHandlerReturn(out: bigint, shouldRedraw: boolean = false) {
        const outPtr = Number(out >> 32n);
        const outLen = Number(out & 0xffffffffn);

        if (!outLen) {
            if (shouldRedraw) {
                drawerExports._redraw(mouseX, mouseY);
            }

            return;
        }

        const renderingTreePtrOnDrawer = drawerExports.malloc(outLen);
        try {
            const renderingTreeView = new Uint8Array(
                memory.buffer,
                outPtr,
                outLen,
            );
            const renderingTreeViewOnDrawer = new Uint8Array(
                drawerExports.memory.buffer,
                renderingTreePtrOnDrawer,
                outLen,
            );
            renderingTreeViewOnDrawer.set(renderingTreeView);

            drawerExports._draw_rendering_tree(
                renderingTreePtrOnDrawer,
                outLen,
                mouseX,
                mouseY,
            );
        } finally {
            drawerExports.free(renderingTreePtrOnDrawer);
        }
    }

    function onAnimationFrame() {
        onEventHandlerReturn(exports._on_animation_frame());

        requestAnimationFrame(onAnimationFrame);
    }
    requestAnimationFrame(onAnimationFrame);

    window.addEventListener("resize", () => {
        const { innerHeight, innerWidth } = window;
        canvas.width = innerWidth;
        canvas.height = innerHeight;

        drawerExports._on_window_resize(innerWidth, innerHeight);

        onEventHandlerReturn(
            exports._on_screen_resize(innerWidth, innerHeight),
            true,
        );
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

        const fn =
            type === "down" ? exports._on_key_down : exports._on_key_up;
        onEventHandlerReturn(fn(code));
    }
    document.addEventListener("keydown", (e) => {
        onKeyEvent("down", e);
    });
    document.addEventListener("keyup", (e) => {
        onKeyEvent("up", e);
    });

    function onMouseEvent(type: "down" | "move" | "up", event: MouseEvent) {
        event.preventDefault();

        mouseX = event.clientX;
        mouseY = event.clientY;

        const fn =
            type === "down"
                ? exports._on_mouse_down
                : type === "move"
                ? exports._on_mouse_move
                : exports._on_mouse_up;
        onEventHandlerReturn(
            fn(event.clientX, event.clientY, event.button, event.buttons),
            true,
        );
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
        onEventHandlerReturn(
            exports._on_mouse_wheel(
                event.deltaX,
                event.deltaY,
                event.clientX,
                event.clientY,
            ),
        );
    });

    window.addEventListener("blur", () => {
        onEventHandlerReturn(exports._on_blur());
    });

    document.addEventListener("visibilitychange", () => {
        onEventHandlerReturn(exports._on_visibility_change());
    });

    const onTextInputEvent: OnTextInputEvent = (textarea, eventType, code) => {
        const textBuffer = new TextEncoder().encode(textarea.value);
        const textPtr = exports.malloc(textBuffer.byteLength);

        try {
            const textView = new Uint8Array(
                memory.buffer,
                textPtr,
                textBuffer.byteLength,
            );
            textView.set(textBuffer);

            const selectionDirection =
                textarea.selectionDirection === "forward"
                    ? 1
                    : textarea.selectionDirection === "backward"
                    ? 2
                    : 0;
            const selectionStart = textarea.selectionStart || 0;
            const selectionEnd = textarea.selectionEnd || 0;

            let result: bigint;
            if (eventType === EVENT_TYPE.TEXT_INPUT) {
                result = exports._on_text_input(
                    textPtr,
                    textBuffer.byteLength,
                    selectionDirection,
                    selectionStart,
                    selectionEnd,
                );
            } else if (eventType === EVENT_TYPE.TEXT_INPUT_KEY_DOWN) {
                result = exports._on_text_input_key_down(
                    textPtr,
                    textBuffer.byteLength,
                    selectionDirection,
                    selectionStart,
                    selectionEnd,
                    code!,
                );
            } else {
                // EVENT_TYPE.SELECTION_CHANGE
                result = exports._on_text_input_selection_change(
                    textPtr,
                    textBuffer.byteLength,
                    selectionDirection,
                    selectionStart,
                    selectionEnd,
                );
            }

            onEventHandlerReturn(result);
        } finally {
            exports.free(textPtr);
        }
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
