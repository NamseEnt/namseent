import { IManagerInternal } from "../device/IManager";
import { WebKeyboardManager } from "../device/keyboard/WebKeyboardManager";
import { MouseButtonManager } from "../device/mouse/mouseButton/MouseButtonManager";
import { WebMouseEventManager } from "../device/mouse/mouseEvent/WebMouseEventManager";
import { WebMousePointerManager } from "../device/mouse/mousePointer/WebMousePointerManager";
import { WebMousePositionManager } from "../device/mouse/mousePosition/WebMousePositionManager";
import { WebScreenManager } from "../device/screen/WebScreenManager";
import { WebWheelManager } from "../device/wheel/WebWheelManager";
import { WebTextInputManager } from "../textInput/WebTextInputManager";
import { IEngineInternal } from "./IEngine";

const managerMap = {
  mousePointer: new WebMousePointerManager(),
  mousePosition: new WebMousePositionManager(),
  mouseEvent: new WebMouseEventManager(),
  screen: new WebScreenManager(),
  wheel: new WebWheelManager(),
  keyboard: new WebKeyboardManager(),
  mouseButton: new MouseButtonManager(),
  textInput: new WebTextInputManager(),
} as const;

const managers = Object.values(managerMap) as IManagerInternal[];

export const webEngine: IEngineInternal = {
  resetBeforeRender() {
    managers.forEach((manager) => manager.resetBeforeRender?.());
  },
  destroy() {
    managers.forEach((manager) => manager.destroy?.());
  },
  afterRender(renderingTree) {
    managers.forEach((manager) => manager.afterRender?.(renderingTree));
  },
  ...managerMap,
};
