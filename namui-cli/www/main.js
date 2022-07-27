import init, { start } from './bundle.js';

async function run() {
    await init();
    const CanvasKit = await CanvasKitInit({
        locateFile: (file) => "./canvaskit-wasm/" + file,
    });
    globalThis.CanvasKit = CanvasKit;
    globalThis.getCanvasKit = () => CanvasKit;
    start();
}

run();
