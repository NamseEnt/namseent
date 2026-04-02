export type Exports = CommonExports & {
    _init_system: () => void;
    _shutdown: () => void;
    _on_mouse_down: (
        x: number,
        y: number,
        mouseEventButton: number,
        mouseEventButtons: number,
    ) => number;
    _on_mouse_up: (
        x: number,
        y: number,
        mouseEventButton: number,
        mouseEventButtons: number,
    ) => number;
    _on_mouse_move: (
        x: number,
        y: number,
        mouseEventButton: number,
        mouseEventButtons: number,
    ) => number;
    _on_mouse_wheel: (
        delta_x: number,
        delta_y: number,
        x: number,
        y: number,
    ) => number;
    _on_key_down: (code: number) => number;
    _on_key_up: (code: number) => number;
    _on_blur: () => number;
    _on_visibility_change: () => number;
    _on_screen_resize: (width: number, height: number) => number;
    _on_animation_frame: () => number;
    _on_text_input: (
        text_ptr: number,
        text_len: number,
        selection_direction: number,
        selection_start: number,
        selection_end: number,
    ) => number;
    _on_text_input_key_down: (
        text_ptr: number,
        text_len: number,
        selection_direction: number,
        selection_start: number,
        selection_end: number,
        code: number,
    ) => number;
    _on_text_input_selection_change: (
        text_ptr: number,
        text_len: number,
        selection_direction: number,
        selection_start: number,
        selection_end: number,
    ) => number;
    _set_image_infos: (ptr: number, count: number) => void;
    _freeze_world: () => number;
    _set_freeze_states(ptr: number, len: number): void;
    _on_kv_store_get_response(
        requestId: number,
        hasData: number,
        ptr: number,
        len: number,
    ): void;
    _on_kv_store_put_response(requestId: number): void;
};

export type DrawerExports = CommonExports & {
    _register_image: (
        imageId: number,
        bufferPtr: number,
        bufferLen: number,
    ) => void;
    _image_infos: (ptr: number, maxCount: number) => number;
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
