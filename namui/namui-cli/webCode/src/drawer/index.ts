import { startThread } from "../thread/startThread";
import drawerUrl from "virtual:namui-drawer.wasm?url";
import { assetList } from "virtual:asset-list";
import cursorMetadata from "../../../system_bundle/cursor/capitaine_24.txt?raw";
import { DrawerExports } from "@/exports";
import { loadFonts } from "@/font/loadFont";

export async function readyDrawer(): Promise<{
    exports: DrawerExports;
    canvas: HTMLCanvasElement;
}> {
    const canvas = document.createElement("canvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    canvas.style.cursor = "none";
    document.body.appendChild(canvas);

    const memory = new WebAssembly.Memory({
        initial: 128,
        maximum: 16384,
        shared: true,
    });

    const nextTid = new SharedArrayBuffer(4);
    new Uint32Array(nextTid)[0] = 1;

    const module = await WebAssembly.compileStreaming(fetch(drawerUrl));

    const instance = await startThread({
        type: "drawer",
        memory,
        module,
        nextTid,
        initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
        canvas,
        imageCount: assetList.length,
    });
    console.log("drawer instance initialized");
    const exports = instance.exports as DrawerExports;

    let now = performance.now();
    exports._init_skia(0, window.innerWidth, window.innerHeight);
    console.log(`_init_skia took: ${performance.now() - now}ms`);

    await Promise.all([
        (async () => {
            now = performance.now();
            await loadAssets({ memory, exports });
            console.log(`loadAssets took: ${performance.now() - now}ms`);

            now = performance.now();
            initCursorSpriteSet({ memory, exports });
            console.log(
                `initCursorSpriteSet took: ${performance.now() - now}ms`,
            );
        })(),
        (async () => {
            now = performance.now();
            await loadFonts({
                memory,
                module,
            });
            console.log(`loadFonts took: ${performance.now() - now}ms`);
        })(),
    ]);

    return {
        exports: exports,
        canvas,
    };
}

async function loadAssets({
    memory,
    exports,
}: {
    memory: WebAssembly.Memory;
    exports: DrawerExports;
}) {
    await Promise.all(
        assetList.map(async ({ id, path }) => {
            try {
                const response = await fetch(path);
                if (!response.ok) {
                    throw new Error(
                        `Failed to fetch image ${id} from ${path}: ${response.statusText}`,
                    );
                }

                const arrayBuffer = await response.arrayBuffer();
                const bytes = new Uint8Array(arrayBuffer);
                const len = bytes.length;

                const ptr = exports.malloc(len);
                const wasmMemory = new Uint8Array(memory.buffer);
                wasmMemory.set(bytes, ptr);

                exports._register_image(id, ptr, len);
            } catch (error) {
                console.error(`Error loading image ${id} from ${path}:`, error);
                throw error;
            }
        }),
    );
}

function initCursorSpriteSet({
    memory,
    exports,
}: {
    memory: WebAssembly.Memory;
    exports: DrawerExports;
}) {
    const metadataBytes = new TextEncoder().encode(cursorMetadata);
    const metadataLen = metadataBytes.length;
    const metadataPtr = exports.malloc(metadataLen);
    const wasmMemory = new Uint8Array(memory.buffer);
    wasmMemory.set(metadataBytes, metadataPtr);

    exports._init_standard_cursor_sprite_set(metadataPtr, metadataLen);

    exports.free(metadataPtr);
}
