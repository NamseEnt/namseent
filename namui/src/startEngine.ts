import { CanvasKit as CanvasKitType } from "canvaskit-wasm";
import { BuildServerConnection } from "./build/BuildServerConnection";
import { draw } from "./draw/draw";
import { FontStorage } from "./font/FontStorage";
import { loadSansTypefaceOfAllLanguages } from "./font/loadSansTypefaceOfAllLanguages";
import { TypefaceStorage } from "./font/TypefaceStorage";
import { getGcCanvasKitPackage } from "./getGcCanvasKitPackage";
import { getSavedState } from "./build/hotReload/getSavedState";
import { setHotReload } from "./build/hotReload/hotReload";
import { isHotReloaded } from "./build/hotReload/isHotReloaded";
import { ImageLoader } from "./image/ImageLoader";
import { handleMouseClick } from "./device/mouse/handleMouseClick";
import { EngineContext, Render } from "./type";
import { BuildErrorNotifier } from "./build/BuildErrorNotifier";
import { WebTextInputController } from "./textInput/WebTextInputController";
import { handleMouseInOut } from "./device/mouse/handleMouseInOut";
import { webEngine } from "./engine/webEngine";
import { MouseButtonManager } from "./device/mouse/mouseButton/MouseButtonManager";
import { handleMouseDown } from "./device/mouse/handleMouseDown";
import { toNamuiMouseEvent } from "./device/mouse/webMouse";
import { engineInternal } from "./engine/engine";
import { handleMouseEvent } from "./device/mouse/handleMouseEvent";

// TODO : Put everything in globalThis.namui
globalThis.typefaceStorage ??= new TypefaceStorage();
globalThis.fontStorage ??= new FontStorage(globalThis.typefaceStorage);
globalThis.buildServerConnection ??= new BuildServerConnection();
globalThis.buildErrorNotifier ??= new BuildErrorNotifier(
  globalThis.buildServerConnection,
);
globalThis.textInputController ??= new WebTextInputController();

export async function startEngine<TState>(
  state: TState,
  render: Render<TState>,
  options?: {
    hotReload?: {
      buildServerUrl: string;
    };
  },
): Promise<void> {
  const currentScript = document.currentScript as HTMLScriptElement;
  async function start(canvasKit: CanvasKitType) {
    if (options?.hotReload) {
      if (isHotReloaded()) {
        state = getSavedState();
      }
      setHotReload({
        buildServerConnection: globalThis.buildServerConnection,
        getState: () => engineContext.state,
        stopEngine: () => {
          engineContext.isStopped = true;
          engineContext.surface.delete();
          engineContext.deleteGarbages();
          canvasElement.remove();
          globalThis.textInputController.outFocus();
          engineInternal.destroy();
        },
        currentScript,
      });
    }

    const canvasElement = document.createElement("canvas");
    document.body.appendChild(canvasElement);
    canvasElement.width = 1920;
    canvasElement.height = 1080;

    const surface = canvasKit.MakeCanvasSurface(canvasElement);
    if (!surface) {
      throw new Error("Could not create surface");
    }

    await loadSansTypefaceOfAllLanguages(typefaceStorage);

    const { gcCanvasKit, deleteGarbages } = getGcCanvasKitPackage(canvasKit);
    globalThis.CanvasKit = gcCanvasKit;
    const engineContext: EngineContext<TState> = {
      render,
      canvasKit,
      deleteGarbages,
      state,
      surface,
      canvas: surface.getCanvas(),
      fpsInfo: {
        fps: 0,
        last60FrameTimeMs: Date.now(),
        frameCount: 0,
      },
      isStopped: false,
      imageLoader: new ImageLoader(canvasKit),
      fontStorage,
    };

    // TODO : Move this into webEngine
    canvasElement.onclick = (event) => {
      handleMouseClick(engineContext, toNamuiMouseEvent(event));
    };
    canvasElement.onmousemove = (event) => {
      handleMouseInOut(engineContext, toNamuiMouseEvent(event));
    };
    canvasElement.onmousedown = (event) => {
      handleMouseDown(engineContext, toNamuiMouseEvent(event));
    };
    canvasElement.onmouseup = (event) => {
      handleMouseEvent(engineContext, toNamuiMouseEvent(event), "onMouseUp");
    };
    canvasElement.onwheel = (event) => {};
    canvasElement.oncontextmenu = (event) => {
      event.preventDefault();
    };

    requestAnimationFrame(() => {
      onAnimationFrame(engineContext);
    });
  }
  (window as any).start = start;
  if (globalThis.CanvasKit) {
    start(globalThis.CanvasKit);
  }
}

/* Markdown
# Phase
1. State To RenderingTree (We call this as 'Render')
2. Draw RenderingTree
3. Event Handling
  - Mouse
  - Keyboard
  - Network
  - File
*/

function onAnimationFrame<TState>(engineContext: EngineContext<TState>): void {
  let shouldSlowDown = false; // TODO : Support toggling this using hot key

  if (engineContext.isStopped) {
    return;
  }
  try {
    // Phase 0. Update fps info
    updateFpsInfo(engineContext);

    // Phase 0. Gc
    engineContext.deleteGarbages();

    // Phase 0. Reset Engine
    webEngine.resetBeforeRender();

    // Phase 1. Render
    const renderingTree = engineContext.render(engineContext.state);

    // Phase 2. Draw
    draw(engineContext, renderingTree);
    engineContext.surface.flush();

    // Phase 3. Event Handling
    engineContext.lastRenderedTree = renderingTree;
  } catch (error) {
    shouldSlowDown = true;
    throw error;
  } finally {
    if (shouldSlowDown) {
      shouldSlowDown = false;
      setTimeout(() => {
        onAnimationFrame(engineContext);
      }, 1000);
    } else {
      requestAnimationFrame(() => {
        onAnimationFrame(engineContext);
      });
    }
  }
}

function updateFpsInfo(engineContext: EngineContext) {
  const { fpsInfo } = engineContext;
  const now = Date.now();
  if (now - fpsInfo.last60FrameTimeMs > 1000) {
    fpsInfo.fps = fpsInfo.frameCount;
    fpsInfo.frameCount = 0;
    fpsInfo.last60FrameTimeMs = now;

    console.log(`FPS: ${fpsInfo.fps}`);
  }
  fpsInfo.frameCount++;
}
