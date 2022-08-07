import init, { main } from './bundle.js';

async function run() {
    await init();
    const CanvasKit = await CanvasKitInit({
        locateFile: (file) => "./canvaskit-wasm/" + file,
    });
    globalThis.CanvasKit = CanvasKit;
    globalThis.getCanvasKit = () => CanvasKit;
    main();
}

run();
