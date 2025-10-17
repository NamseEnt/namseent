import { startEventSystem } from "./eventSystem";
import { startThread } from "./thread/startThread";
import wasmUrl from "virtual:namui-runtime-wasm.wasm?url";
import "./drawer";
import { readyDrawer } from "./drawer";
import { DrawerExports, Exports } from "./exports";
import { assetList } from "virtual:asset-list";
import { loadFonts } from "@/font/loadFont";

console.debug("crossOriginIsolated", crossOriginIsolated);

if (!crossOriginIsolated) {
    throw new Error("Not cross-origin isolated");
}

const drawerPromise: Promise<{
    exports: DrawerExports;
    canvas: HTMLCanvasElement;
    imageCount: number;
    imageInfoBytes: Uint8Array;
}> = (async () => {
    const drawer = await readyDrawer();

    const imageCount = assetList.length;
    const imageInfoSize = 14;
    const imageInfoBytes = new Uint8Array(imageCount * imageInfoSize);

    const imageInfosPtr = drawer.exports.malloc(imageInfoBytes.byteLength);
    drawer.exports._image_infos(imageInfosPtr);
    imageInfoBytes.set(
        new Uint8Array(
            drawer.exports.memory.buffer,
            imageInfosPtr,
            imageInfoBytes.byteLength,
        ),
    );
    drawer.exports.free(imageInfosPtr);

    return {
        ...drawer,
        imageCount,
        imageInfoBytes,
    };
})();

if (import.meta.hot) {
    import.meta.hot.on("namui-wasm-updated", () => {
        startMainThread();
    });
}

let terminate = () => {};
let requestedDuringStart = false;
let starting = false;
let exports: Exports | undefined;
let frozenWorldBytes: Uint8Array | undefined;

async function startMainThread() {
    if (starting) {
        requestedDuringStart = true;
        return;
    }
    try {
        {
            starting = true;
            requestedDuringStart = false;
            terminate();

            if (exports) {
                const ptrAndLen = exports._freeze_world();
                const ptr = Number(ptrAndLen >> 32n);
                const len = Number(ptrAndLen & 0xffffffffn);
                frozenWorldBytes = new Uint8Array(
                    exports.memory.buffer,
                    ptr,
                    len,
                );
            }

            const memory = new WebAssembly.Memory({
                initial: 128,
                maximum: 16384,
                shared: true,
            });

            const nextTid = new SharedArrayBuffer(4);
            new Uint32Array(nextTid)[0] = 1;

            const [drawer, module] = await Promise.all([
                drawerPromise,
                WebAssembly.compileStreaming(fetch(wasmUrl)),
            ]);

            const instance = await startThread({
                type: "main",
                memory,
                module,
                nextTid,
                initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
                imageCount: drawer.imageCount,
                imageInfoBytes: drawer.imageInfoBytes,
            });
            exports = instance.exports as Exports;

            if (frozenWorldBytes) {
                const ptr = exports.malloc(frozenWorldBytes.byteLength);
                try {
                    new Uint8Array(
                        exports.memory.buffer,
                        ptr,
                        frozenWorldBytes.byteLength,
                    ).set(frozenWorldBytes);
                    exports._set_freeze_states(
                        ptr,
                        frozenWorldBytes.byteLength,
                    );
                } finally {
                    exports.free(ptr);
                    frozenWorldBytes = undefined;
                }
            }

            let now = performance.now();
            await loadFonts({
                memory: exports.memory,
                module,
            });
            console.log(`main loadFonts took: ${performance.now() - now}ms`);

            now = performance.now();
            exports._init_system();
            console.log(`main initSystem took: ${performance.now() - now}ms`);

            const eventSystem = startEventSystem({
                exports,
                drawer,
            });

            terminate = eventSystem.terminate;
        }
        while (requestedDuringStart);
    } finally {
        starting = false;
        requestedDuringStart = false;
    }
}

startMainThread();

document.addEventListener("keyup", (e) => {
    if (e.key === "F13") {
        startMainThread();
    }
});
