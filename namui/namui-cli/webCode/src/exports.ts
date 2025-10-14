export type Exports = CommonExports & {
    _on_event: (
        ptr: number,
        len: number,
        outPtr: number,
        outLen: number,
    ) => void;
};

export type DrawerExports = CommonExports & {
    _register_image: (
        imageId: number,
        bufferPtr: number,
        bufferLen: number,
    ) => void;
    _image_count: () => number;
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
