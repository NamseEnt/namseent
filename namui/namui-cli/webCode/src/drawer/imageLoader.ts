import { AssetInfo, DrawerExports } from "./types";

export async function processImages(
    assetList: AssetInfo[],
    exports: DrawerExports,
    memory: WebAssembly.Memory,
) {
    await Promise.all(
        assetList.map((asset) =>
            loadImage(asset.id, asset.path, exports, memory),
        ),
    );
}

async function loadImage(
    imageId: number,
    imagePath: string,
    exports: DrawerExports,
    memory: WebAssembly.Memory,
) {
    try {
        const response = await fetch(imagePath);
        if (!response.ok) {
            throw new Error(
                `Failed to fetch image ${imageId} from ${imagePath}: ${response.statusText}`,
            );
        }

        const arrayBuffer = await response.arrayBuffer();
        const bytes = new Uint8Array(arrayBuffer);
        const len = bytes.length;

        const ptr = exports._malloc_image_buffer(imageId, len);
        const wasmMemory = new Uint8Array(memory.buffer);
        wasmMemory.set(bytes, ptr);

        exports._register_image(imageId);
    } catch (error) {
        console.error(
            `Error loading image ${imageId} from ${imagePath}:`,
            error,
        );
        throw error;
    }
}
