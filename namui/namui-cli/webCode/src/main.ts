import { startEventSystem } from "./eventSystem";
import { startThread, ThreadStartSupplies } from "./thread/startThread";
import wasmUrl from "virtual:namui-runtime-wasm.wasm?url";
import "./drawer";
import { readyDrawer } from "./drawer";
import { DrawerExports, Exports } from "./exports";
import { assetList } from "virtual:asset-list";
import { audioAssetList } from "virtual:audio-asset-list";
import { loadFonts } from "@/font/loadFont";
import { loadAudioAssets } from "@/audio";

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
    import.meta.hot.on("namui-app-wasm-updated", () => {
        startMainThread();
    });

    import.meta.hot.on("namui-drawer-wasm-updated", () => {
        window.location.reload();
    });
}

type BaseSupplies = Omit<
    ThreadStartSupplies & { type: "main" },
    "type" | "storageWorker"
>;

function listenSpawnPort(port: MessagePort, baseSupplies: BaseSupplies) {
    port.onmessage = (e: MessageEvent<{ startArgPtr: number; tid: number }>) => {
        const { startArgPtr, tid } = e.data;
        spawnWorker(startArgPtr, tid, baseSupplies);
    };
}

function spawnWorker(
    startArgPtr: number,
    tid: number,
    baseSupplies: BaseSupplies,
) {
    const worker = new Worker(
        new URL("./thread/SubThreadWorker.ts?worker_file&type=module", import.meta.url),
        { type: "module" },
    );
    worker.onerror = (e) => {
        console.error("[spawnWorker] worker error tid:", tid, e);
    };

    const spawnChannel = new MessageChannel();
    listenSpawnPort(spawnChannel.port1, baseSupplies);

    const kvChannel = new MessageChannel();
    kvChannel.port1.onmessage = (e) => {
        storageWorker!.postMessage(e.data);
    };

    const supplies: ThreadStartSupplies = {
        ...baseSupplies,
        type: "sub",
        startArgPtr,
        tid,
        spawnPort: spawnChannel.port2,
        kvStorePort: kvChannel.port2,
    };
    worker.postMessage(supplies, [spawnChannel.port2, kvChannel.port2]);
}

let terminate = () => {};
let requestedDuringStart = false;
let starting = false;
let exports: Exports | undefined;
let frozenWorldBytes: Uint8Array | undefined;
let storageWorker: Worker | undefined;

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

            if (storageWorker) {
                storageWorker.terminate();
            }
            storageWorker = new Worker(
                new URL(
                    "./storage/StorageWorker.ts?worker_file&type=module",
                    import.meta.url,
                ),
                { type: "module" },
            );
            storageWorker.onerror = (e) => {
                console.error("[StorageWorker] failed to load:", e);
            };

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

            const spawnChannel = new MessageChannel();
            const baseSupplies: BaseSupplies = {
                memory,
                module,
                nextTid,
                initialWindowWh: (window.innerWidth << 16) | window.innerHeight,
                imageCount: drawer.imageCount,
                imageInfoBytes: drawer.imageInfoBytes,
                spawnPort: spawnChannel.port2,
            };
            listenSpawnPort(spawnChannel.port1, baseSupplies);

            const instance = await startThread({
                ...baseSupplies,
                type: "main",
                storageWorker,
            });
            exports = instance.exports as Exports;

            const currentExports = exports;
            const currentMemory = memory;
            storageWorker.onmessage = (e: MessageEvent) => {
                const { requestId, op, hasData, data } = e.data;
                if (op === "get") {
                    if (hasData && data) {
                        const bytes =
                            data instanceof Uint8Array
                                ? data
                                : new Uint8Array(data);
                        const ptr = currentExports.malloc(bytes.length);
                        new Uint8Array(
                            currentMemory.buffer,
                            ptr,
                            bytes.length,
                        ).set(bytes);
                        currentExports._on_kv_store_get_response(
                            requestId,
                            1,
                            ptr,
                            bytes.length,
                        );
                        currentExports.free(ptr);
                    } else {
                        currentExports._on_kv_store_get_response(requestId, 0, 0, 0);
                    }
                } else {
                    currentExports._on_kv_store_put_response(requestId);
                }
            };

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
            await Promise.all([
                (async () => {
                    const fontStart = performance.now();
                    await loadFonts({
                        memory: exports.memory,
                        module,
                    });
                    console.log(`main loadFonts took: ${performance.now() - fontStart}ms`);
                })(),
                (async () => {
                    const audioStart = performance.now();
                    await loadAudioAssets(audioAssetList);
                    console.log(`main loadAudioAssets took: ${performance.now() - audioStart}ms`);
                })(),
            ]);

            now = performance.now();
            exports._init_system();
            console.log(`main initSystem took: ${performance.now() - now}ms`);

            const eventSystem = startEventSystem({
                exports,
                drawer,
            });

            terminate = () => {
                eventSystem.terminate();
                exports!._shutdown();
            };
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
