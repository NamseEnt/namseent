export type AssetInfo = {
    id: number;
    path: string;
};

export type DrawerExports = {
    _register_image: (imageId: number) => void;
    _malloc_image_buffer: (imageId: number, len: number) => number;
};
