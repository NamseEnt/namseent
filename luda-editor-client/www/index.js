import * as wasm from "luda-editor-client";

(async () => {
  const CanvasKit = await CanvasKitInit({
    locateFile: (file) => "./canvaskit-wasm/" + file,
  });
  globalThis.CanvasKit = CanvasKit;
  globalThis.getCanvasKit = () => CanvasKit;
  wasm.start();
})();
