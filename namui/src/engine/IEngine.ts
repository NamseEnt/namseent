import { IManagerInternal } from "../device/IManager";
import { IKeyboardManager } from "../device/keyboard/IKeyboardManager";
import { IMouseButtonManager } from "../device/mouse/mouseButton/IMouseButtonManager";
import { IMouseEventManager } from "../device/mouse/mouseEvent/IMouseEventManager";
import { IMousePointerManager } from "../device/mouse/mousePointer/IMousePointerManager";
import { IMousePositionManager } from "../device/mouse/mousePosition/IMousePositionManager";
import { IScreenManager } from "../device/screen/IScreenManager";
import { IWheelManager } from "../device/wheel/IWheelManager";
import { IImageLoader } from "../image/ImageLoader";
import { ITextInputManager } from "../textInput/ITextInputManager";
import { EngineContext } from "../type";

export interface IEngine {
  mousePointer: IMousePointerManager;
  mousePosition: IMousePositionManager;
  mouseEvent: IMouseEventManager;
  screen: IScreenManager;
  wheel: IWheelManager;
  keyboard: IKeyboardManager;
  textInput: ITextInputManager;
  imageLoader: IImageLoader;
}

export interface IEngineInternal extends IManagerInternal {
  init(engineContext: EngineContext): void;
  mousePointer: IMousePointerManager & IManagerInternal;
  mousePosition: IMousePositionManager & IManagerInternal;
  mouseEvent: IMouseEventManager & IManagerInternal;
  mouseButton: IMouseButtonManager & IManagerInternal;
  screen: IScreenManager & IManagerInternal;
  wheel: IWheelManager & IManagerInternal;
  keyboard: IKeyboardManager & IManagerInternal;
  textInput: ITextInputManager & IManagerInternal;
}
