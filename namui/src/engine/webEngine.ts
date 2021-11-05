import { RenderingTree } from "..";
import { IManagerInternal } from "../managers/IManager";
import { WebKeyboardManager } from "../device/keyboard/WebKeyboardManager";
import { MouseButtonManager } from "../device/mouse/mouseButton/MouseButtonManager";
import { WebMouseEventManager } from "../device/mouse/mouseEvent/WebMouseEventManager";
import { WebMousePointerManager } from "../device/mouse/mousePointer/WebMousePointerManager";
import { WebMousePositionManager } from "../device/mouse/mousePosition/WebMousePositionManager";
import { WebScreenManager } from "../device/screen/WebScreenManager";
import { WebWheelManager } from "../device/wheel/WebWheelManager";
import { IImageLoader, ImageLoader } from "../image/ImageLoader";
import { WebTextInputManager } from "../textInput/WebTextInputManager";
import { EngineContext } from "../type";
import { RenderManager } from "../managers/render/RenderManager";

const managerMap = {
  mousePointer: new WebMousePointerManager(),
  mousePosition: new WebMousePositionManager(),
  mouseEvent: new WebMouseEventManager(),
  screen: new WebScreenManager(),
  wheel: new WebWheelManager(),
  keyboard: new WebKeyboardManager(),
  mouseButton: new MouseButtonManager(),
  textInput: new WebTextInputManager(),
  render: new RenderManager(),
} as const;

const managers = Object.values(managerMap) as IManagerInternal[];

export const webEngine = {
  resetBeforeRender() {
    managers.forEach((manager) => manager.resetBeforeRender?.());
  },
  destroy() {
    managers.forEach((manager) => manager.destroy?.());
  },
  afterRender(renderingTree: RenderingTree) {
    managers.forEach((manager) => manager.afterRender?.(renderingTree));
  },
  get imageLoader(): IImageLoader {
    if (!this._imageLoader) {
      throw new Error("engine is not initialized");
    }
    return this._imageLoader;
  },
  _imageLoader: undefined as IImageLoader | undefined,
  init(engineContext: EngineContext) {
    this._imageLoader = new ImageLoader(engineContext.canvasKit);
  },
  ...managerMap,
};
