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

    start();
}

run();
