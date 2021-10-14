import { IManagerInternal } from "../IManager";
import { Code } from "./Code";
import { IKeyboardManager } from "./IKeyboardManager";
import { Key } from "./Key";

// [key in keyof WindowEventMap]?: (event: WindowEventMap[key]) => void;

export class WebKeyboardManager implements IKeyboardManager, IManagerInternal {
  private readonly pressingCodeSet: Set<Code> = new Set();
  private readonly pressingKeySet: Set<Key> = new Set();
  private readonly eventNameAndListenerTuples = [
    [
      "keydown",
      (event: KeyboardEvent) => {
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
    ],
    [
      "keyup",
      (event: KeyboardEvent) => {
        const { code, key } = event;
        if (code in Code) {
          this.pressingCodeSet.delete(code as Code);
        }
        if (key in Key) {
          this.pressingKeySet.delete(key as Key);
        }
      },
    ],
  ] as const;
  constructor() {
    this.eventNameAndListenerTuples.forEach(([eventName, listener]) => {
      document.addEventListener(eventName, listener);
    });
  }
  destroy(): void {
    this.eventNameAndListenerTuples.forEach(([eventName, listener]) => {
      document.removeEventListener(eventName, listener);
    });
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
