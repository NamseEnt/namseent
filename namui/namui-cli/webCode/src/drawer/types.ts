export type AssetInfo = {
    id: number;
    path: string;
};

export type DrawerExports = {
    memory: WebAssembly.Memory;
    malloc: (len: number) => number;
    free: (ptr: number) => void;
    _register_image: (imageId: number) => void;
    _image_count: () => number;
    _image_infos: (ptr: number) => void;
    _malloc_image_buffer: (imageId: number, len: number) => number;
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
};
