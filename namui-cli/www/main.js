import init, { start } from "./bundle.js";

async function run() {
    const [_, CanvasKit] = await Promise.all([
        init(),
        CanvasKitInit({
            locateFile: (file) => "./canvaskit-wasm/" + file,
        }),
    ]);

    globalThis.CanvasKit = CanvasKit;
    globalThis.getCanvasKit = () => CanvasKit;
    globalThis.getGpuDevice = getGpuDevice;

    start();
}

run();

async function getGpuDevice() {
    const adapter = await navigator.gpu.requestAdapter();
    if (!adapter) {
        throw new Error("No GPU adapter found");
    }
    const device = await adapter.requestDevice();
    return device;
}
