import { WebKeyboardManager } from "../device/keyboard/WebKeyboardManager";
import { MouseButtonManager } from "../device/mouse/mouseButton/MouseButtonManager";
import { WebMouseEventManager } from "../device/mouse/mouseEvent/WebMouseEventManager";
import { WebMousePointerManager } from "../device/mouse/mousePointer/WebMousePointerManager";
import { WebMousePositionManager } from "../device/mouse/mousePosition/WebMousePositionManager";
import { WebScreenManager } from "../device/screen/WebScreenManager";
import { WebWheelManager } from "../device/wheel/WebWheelManager";
import { IEngineInternal } from "./IEngine";

export const webEngine: IEngineInternal = {
  resetBeforeRender() {
    webEngine.mousePointer.resetBeforeRender();
    webEngine.mousePosition.resetBeforeRender();
    webEngine.mouseEvent.resetBeforeRender();
    webEngine.screen.resetBeforeRender();
    webEngine.wheel.resetBeforeRender();
    webEngine.keyboard.resetBeforeRender();
  },
  destroy() {
    webEngine.mousePointer.destroy();
    webEngine.mousePosition.destroy();
    webEngine.mouseEvent.destroy();
    webEngine.screen.destroy();
    webEngine.wheel.destroy();
    webEngine.keyboard.destroy();
    webEngine.mouseButton.destroy();
  },
  mousePointer: new WebMousePointerManager(),
  mousePosition: new WebMousePositionManager(),
  mouseEvent: new WebMouseEventManager(),
  screen: new WebScreenManager(),
  wheel: new WebWheelManager(),
  keyboard: new WebKeyboardManager(),
  mouseButton: new MouseButtonManager(),
};
