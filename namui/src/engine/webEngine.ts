import { RenderingTree } from "..";
import { IManagerInternal } from "../managers/IManager";
import { WebKeyboardManager } from "../device/keyboard/WebKeyboardManager";
import { MouseButtonManager } from "../device/mouse/mouseButton/MouseButtonManager";
import { WebMouseEventManager } from "../device/mouse/mouseEvent/WebMouseEventManager";
import { WebMousePointerManager } from "../device/mouse/mousePointer/WebMousePointerManager";
import { WebMousePositionManager } from "../device/mouse/mousePosition/WebMousePositionManager";
import { WebScreenManager } from "../device/screen/WebScreenManager";
import { WebWheelManager } from "../device/wheel/WebWheelManager";
import { ImageLoadManager } from "../image/ImageLoadManager";
import { WebTextInputManager } from "../textInput/WebTextInputManager";
import { EngineContext } from "../type";
import { RenderManager } from "../managers/render/RenderManager";
import { IEngineInternal } from "./IEngine";

export const webEngine = {
  resetBeforeRender() {
    this.managers.forEach((manager: IManagerInternal) =>
      manager.resetBeforeRender?.(),
    );
  },
  destroy() {
    this.managers.forEach((manager: IManagerInternal) => manager.destroy?.());
  },
  afterRender(renderingTree: RenderingTree) {
    this.managers.forEach((manager: IManagerInternal) =>
      manager.afterRender?.(renderingTree),
    );
  },
  init(engineContext: EngineContext) {
    const mousePointerManager = new WebMousePointerManager();
    const mousePositionManager = new WebMousePositionManager();
    const mouseButtonManager = new MouseButtonManager();
    const mouseEventManager = new WebMouseEventManager(mouseButtonManager);
    const screenManager = new WebScreenManager();
    const wheelManager = new WebWheelManager();
    const keyboardManager = new WebKeyboardManager();
    const textInputManager = new WebTextInputManager();
    const renderManager = new RenderManager(engineContext, mouseEventManager);
    const imageLoadManager = new ImageLoadManager(engineContext.canvasKit);

    const managerMap = {
      mousePointer: mousePointerManager,
      mousePosition: mousePositionManager,
      mouseEvent: mouseEventManager,
      screen: screenManager,
      wheel: wheelManager,
      keyboard: keyboardManager,
      mouseButton: mouseButtonManager,
      textInput: textInputManager,
      render: renderManager,
      imageLoadManager: imageLoadManager,
    } as const;

    this.managers = Object.values(managerMap);
    Object.entries(managerMap).forEach(([key, manager]) => {
      this[key] = manager;
    });
  },
} as any as IEngineInternal;
