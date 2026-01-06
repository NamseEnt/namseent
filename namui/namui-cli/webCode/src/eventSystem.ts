import { DrawerExports, Exports } from "./exports";
import { CODES } from "./imports/codes";

export function startEventSystem({
    exports,
    drawer,
}: {
    exports: Exports;
    drawer: {
        exports: DrawerExports;
        canvas: HTMLCanvasElement;
    };
}): {
    terminate: () => void;
} {
    let mouseX = 0;
    let mouseY = 0;
    let animationFrameId: number | null = null;

    const memory = exports.memory;

    function onEventHandlerReturn(renderingTreePtrLen: bigint) {
        if (renderingTreePtrLen === 0xffffffffffffffffn) {
            drawer.exports._redraw(mouseX, mouseY);
            return;
        }

        if (!renderingTreePtrLen) {
            return;
        }

        const renderingTreePtr = Number(renderingTreePtrLen >> 32n);
        const renderingTreeLen = Number(renderingTreePtrLen & 0xffffffffn);

        const renderingTreePtrOnDrawer =
            drawer.exports.malloc(renderingTreeLen);
        try {
            new Uint8Array(
                drawer.exports.memory.buffer,
                renderingTreePtrOnDrawer,
                renderingTreeLen,
            ).set(
                new Uint8Array(
                    memory.buffer,
                    renderingTreePtr,
                    renderingTreeLen,
                ),
            );

            drawer.exports._draw_rendering_tree(
                renderingTreePtrOnDrawer,
                renderingTreeLen,
                mouseX,
                mouseY,
            );
        } finally {
            drawer.exports.free(renderingTreePtrOnDrawer);
        }
    }

    function onAnimationFrame() {
        onEventHandlerReturn(exports._on_animation_frame());

        animationFrameId = requestAnimationFrame(onAnimationFrame);
    }
    animationFrameId = requestAnimationFrame(onAnimationFrame);

    function onResize() {
        const { innerHeight, innerWidth } = window;
        drawer.canvas.width = innerWidth;
        drawer.canvas.height = innerHeight;

        drawer.exports._on_window_resize(innerWidth, innerHeight);

        onEventHandlerReturn(
            exports._on_screen_resize(innerWidth, innerHeight),
        );
    }
    window.addEventListener("resize", onResize);

    function onKeyEvent(type: "down" | "up", event: KeyboardEvent) {
        const code = CODES[event.code as keyof typeof CODES];
        if (!code) {
            console.warn(`Unknown key code: ${event.code}`);
            return;
        }
        if (!isKeyPreventDefaultException(event)) {
            event.preventDefault();
        }

        const fn = type === "down" ? exports._on_key_down : exports._on_key_up;
        onEventHandlerReturn(fn(code));
    }
    function onKeyDown(event: KeyboardEvent) {
        onKeyEvent("down", event);
    }
    function onKeyUp(event: KeyboardEvent) {
        onKeyEvent("up", event);
    }
    document.addEventListener("keydown", onKeyDown);
    document.addEventListener("keyup", onKeyUp);

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
        );
    }
    function onMouseDown(event: MouseEvent) {
        onMouseEvent("down", event);
    }
    function onMouseMove(event: MouseEvent) {
        onMouseEvent("move", event);
    }
    function onMouseUp(event: MouseEvent) {
        onMouseEvent("up", event);
    }
    document.addEventListener("mousedown", onMouseDown);
    document.addEventListener("mousemove", onMouseMove);
    document.addEventListener("mouseup", onMouseUp);

    function onWheel(event: WheelEvent) {
        onEventHandlerReturn(
            exports._on_mouse_wheel(
                event.deltaX,
                event.deltaY,
                event.clientX,
                event.clientY,
            ),
        );
    }
    document.addEventListener("wheel", onWheel);

    function onBlur() {
        onEventHandlerReturn(exports._on_blur());
    }
    window.addEventListener("blur", onBlur);

    function onVisibilityChange() {
        onEventHandlerReturn(exports._on_visibility_change());
    }
    document.addEventListener("visibilitychange", onVisibilityChange);

    function onContextMenu(event: PointerEvent) {
        event.preventDefault();
    }
    document.addEventListener("contextmenu", onContextMenu);

    function terminate() {
        if (animationFrameId !== null) {
            cancelAnimationFrame(animationFrameId);
        }

        window.removeEventListener("resize", onResize);
        document.removeEventListener("keydown", onKeyDown);
        document.removeEventListener("keyup", onKeyUp);
        document.removeEventListener("mousedown", onMouseDown);
        document.removeEventListener("mousemove", onMouseMove);
        document.removeEventListener("mouseup", onMouseUp);
        document.removeEventListener("wheel", onWheel);
        window.removeEventListener("blur", onBlur);
        document.removeEventListener("visibilitychange", onVisibilityChange);
        document.removeEventListener("contextmenu", onContextMenu);
    }

    return { terminate };
}

function isKeyPreventDefaultException(event: KeyboardEvent): boolean {
    // TODO: Maybe we have to disable this on production.
    const isDevTools =
        event.code === "F12" ||
        (event.ctrlKey && event.shiftKey && event.code === "KeyI");
    const isReload =
        event.code === "F5" || (event.ctrlKey && event.code === "KeyR");

    return isDevTools || isReload;
}
