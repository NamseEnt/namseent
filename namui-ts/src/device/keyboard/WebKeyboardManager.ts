import { IManagerInternal } from "../../managers/IManager";
import { Code } from "./Code";
import { IKeyboardManager } from "./IKeyboardManager";
import { Key } from "./Key";

export class WebKeyboardManager implements IKeyboardManager, IManagerInternal {
  private readonly pressingCodeSet: Set<Code> = new Set();
  private readonly pressingKeySet: Set<Key> = new Set();
  private readonly eventNameAndListenerTuples: {
    [key in keyof DocumentEventMap]?: (event: DocumentEventMap[key]) => void;
  } = {
    keydown: (event: KeyboardEvent) => {
      const { code, key } = event;
      if (code in Code) {
        this.pressingCodeSet.add(code as Code);
      }
      if (key in Key) {
        this.pressingKeySet.add(key as Key);
      }

      if (key === Key.Alt) {
        event.preventDefault();
      }
    },
    keyup: (event: KeyboardEvent) => {
      const { code, key } = event;
      if (code in Code) {
        this.pressingCodeSet.delete(code as Code);
      }
      if (key in Key) {
        this.pressingKeySet.delete(key as Key);
      }
    },
    blur: (event: FocusEvent) => {
      this.pressingCodeSet.clear();
      this.pressingKeySet.clear();
    },
    visibilitychange: () => {
      if (document.hidden) {
        this.pressingCodeSet.clear();
        this.pressingKeySet.clear();
      }
    },
  };
  constructor() {
    Object.entries(this.eventNameAndListenerTuples).forEach(
      ([eventName, listener]) => {
        window.addEventListener(eventName, listener as any);
      },
    );
  }
  destroy(): void {
    Object.entries(this.eventNameAndListenerTuples).forEach(
      ([eventName, listener]) => {
        window.removeEventListener(eventName, listener as any);
      },
    );
  }
  isKeyPress(key: Key): boolean {
    return this.pressingKeySet.has(key);
  }
  resetBeforeRender(): void {
    // do nothing
  }
  anyCodePress(codes: Code[]): boolean {
    return codes.some((code) => this.isCodePress(code));
  }
  isCodePress(code: Code): boolean {
    return this.pressingCodeSet.has(code);
  }
}
