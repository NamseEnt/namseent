import { IManagerInternal } from "../managers/IManager";
import {
  ITextInputManager,
  OnTextInputChange,
  Selection,
} from "./ITextInputManager";

export class WebTextInputManager
  implements ITextInputManager, IManagerInternal
{
  private readonly inputElement = document.createElement("input");
  private onChangeCallback: OnTextInputChange | undefined = undefined;
  private lastSelection:
    | {
        start: HTMLInputElement["selectionStart"];
        end: HTMLInputElement["selectionEnd"];
      }
    | undefined = undefined;
  private isFocus: boolean = false;
  private setFocusParam?: {
    text: string;
    selection: Selection | undefined;
    onChange: OnTextInputChange;
  };
  private lastSetFocusParam?: {
    text: string;
    selection: Selection | undefined;
  };
  private readonly eventListenerTuples = [
    [
      "mousedown",
      (event: Event) => {
        event.preventDefault();
      },
    ],
    [
      "selectionchange",
      (event: Event) => {
        const selection = this.getSelection();
        if (
          !this.lastSelection ||
          this.lastSelection.start !== selection?.start ||
          this.lastSelection.end !== selection.end
        ) {
          this.onChangeSomething();
          this.lastSelection = selection;
        }
      },
    ],
  ] as const;
  constructor() {
    document.body.appendChild(this.inputElement);
    this.eventListenerTuples.forEach(([eventName, listener]) => {
      document.addEventListener(eventName, listener);
    });
    this.inputElement.addEventListener("input", () => {
      this.onChangeSomething();
    });

    // NOTE: Below codes from https://github.com/goldfire/CanvasInput/blob/5adbaf00bd42665f3c691796881c7a7a9cf7036c/CanvasInput.js#L126
    {
      this.inputElement.type = "text";
      this.inputElement.style.position = "absolute";
      this.inputElement.style.opacity = "0";
      this.inputElement.style.pointerEvents = "none";
      this.inputElement.style.zIndex = "0";
      this.inputElement.style.top = "0px";
      // hide native blue text cursor on iOS
      this.inputElement.style.transform = "scale(0)";
    }
  }
  resetBeforeRender(): void {
    this.setFocusParam = undefined;
  }
  afterRender() {
    this.updateFocus();
  }
  private updateFocus() {
    if (!this.setFocusParam) {
      this.outFocus();
      return;
    }

    const isSameSetFocusParam = this.checkIsSameSetFocusParam();
    if (isSameSetFocusParam) {
      return;
    }

    const { onChange, selection, text } = this.setFocusParam;
    this.inputElement.focus({ preventScroll: true });
    this.inputElement.value = text;
    this.isFocus = true;
    this.onChangeCallback = onChange;
    const direction = !selection
      ? "none"
      : selection.start <= selection.end
      ? "forward"
      : "backward";
    const min = !selection ? null : Math.min(selection.start, selection.end);
    const max = !selection ? null : Math.max(selection.start, selection.end);
    this.inputElement.setSelectionRange(min, max, direction);
    this.lastSetFocusParam = {
      text: this.setFocusParam.text,
      selection: this.setFocusParam.selection
        ? {
            ...this.setFocusParam.selection,
          }
        : undefined,
    };
  }
  private checkIsSameSetFocusParam(): boolean {
    if (!this.setFocusParam) {
      throw new Error("setFocusParam is undefined");
    }

    if (!this.lastSetFocusParam) {
      return false;
    }

    return (
      this.lastSetFocusParam.text === this.setFocusParam.text &&
      this.lastSetFocusParam.selection?.start ===
        this.setFocusParam.selection?.start &&
      this.lastSetFocusParam.selection?.end ===
        this.setFocusParam.selection?.end
    );
  }
  destroy(): void {
    this.inputElement.remove();

    this.eventListenerTuples.forEach(([eventName, listener]) => {
      document.removeEventListener(eventName, listener);
    });
  }
  public setFocus(param: {
    text: string;
    selection: Selection | undefined;
    onChange: OnTextInputChange;
  }): void {
    this.setFocusParam = param;
  }
  private onChangeSomething() {
    if (!this.onChangeCallback) {
      return;
    }
    const selection = this.getSelection();
    this.onChangeCallback({
      text: this.inputElement.value,
      selection,
    });
    this.lastSetFocusParam = {
      text: this.inputElement.value,
      selection: selection
        ? {
            ...selection,
          }
        : undefined,
    };
  }
  public outFocus() {
    this.isFocus = false;
    this.inputElement.selectionStart = null;
    this.inputElement.selectionEnd = null;
    this.inputElement.blur();
    this.onChangeSomething();
    this.inputElement.value = "";
    this.onChangeCallback = undefined;
    this.lastSetFocusParam = undefined;
  }
  private getSelection(): Selection | undefined {
    if (
      this.inputElement.selectionStart === null ||
      this.inputElement.selectionEnd === null ||
      !this.isFocus
    ) {
      return undefined;
    }
    const isBackward = this.inputElement.selectionDirection === "backward";
    return {
      start: isBackward
        ? this.inputElement.selectionEnd
        : this.inputElement.selectionStart,
      end: isBackward
        ? this.inputElement.selectionStart
        : this.inputElement.selectionEnd,
    };
  }
}
