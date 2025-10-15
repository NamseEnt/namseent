export type Exports = CommonExports & {
    _init_system: () => void;
    _on_event: (
        ptr: number,
        len: number,
        outPtrPtr: number,
        outLenPtr: number,
    ) => number;
    _on_mouse_down: (
        x: number,
        y: number,
        mouseEventButton: number,
        mouseEventButtons: number,
    ) => bigint;
    _on_mouse_up: (
        x: number,
        y: number,
        mouseEventButton: number,
        mouseEventButtons: number,
    ) => bigint;
    _on_mouse_move: (
        x: number,
        y: number,
        mouseEventButton: number,
        mouseEventButtons: number,
    ) => bigint;
    _on_mouse_wheel: (
        delta_x: number,
        delta_y: number,
        x: number,
        y: number,
    ) => bigint;
};

export type DrawerExports = CommonExports & {
    _register_image: (
        imageId: number,
        bufferPtr: number,
        bufferLen: number,
    ) => void;
    _image_infos: (ptr: number) => void;
    _register_font: (
        namePtr: number,
        nameLen: number,
        bufferPtr: number,
        bufferLen: number,
    ) => void;
    _init_skia(
        screenId: number,
        windowWidth: number,
        windowHeight: number,
    ): void;
    _init_standard_cursor_sprite_set: (
        metadataBytesPtr: number,
        metadataBytesLen: number,
    ) => void;
    _draw_rendering_tree: (
        renderingTreeBytesPtr: number,
        renderingTreeBytesLen: number,
        mouseX: number,
        mouseY: number,
    ) => void;
    _redraw(mouseX: number, mouseY: number): void;
    _on_window_resize: (windowWidth: number, windowHeight: number) => void;
};

export type CommonExports = {
    memory: WebAssembly.Memory;
    wasi_thread_start: (tid: number, startArgPtr: number) => void;
    malloc: (size: number) => number;
    free: (ptr: number) => void;
    _register_font: (
        namePtr: number,
        nameLen: number,
        bufferPtr: number,
        bufferLen: number,
    ) => void;
};
