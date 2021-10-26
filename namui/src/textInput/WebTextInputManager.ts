import { IManagerInternal } from "../device/IManager";
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
      // hide native blue text cursor on iOS
      this.inputElement.style.transform = "scale(0)";
    }
  }
  resetBeforeRender(): void {
    // nothing
  }
  destroy(): void {
    this.inputElement.remove();

    this.eventListenerTuples.forEach(([eventName, listener]) => {
      document.removeEventListener(eventName, listener);
    });
  }
  public updateSelection(selection: Selection | undefined): void {
    this.inputElement.focus();
    const direction = !selection
      ? "none"
      : selection.start <= selection.end
      ? "forward"
      : "backward";
    const min = !selection ? null : Math.min(selection.start, selection.end);
    const max = !selection ? null : Math.max(selection.start, selection.end);

    this.inputElement.setSelectionRange(min, max, direction);
  }
  public setFocus({
    text,
    onChange,
  }: {
    text: string;
    onChange: OnTextInputChange;
  }): void {
    this.inputElement.focus({ preventScroll: true });
    this.inputElement.value = text;
    this.isFocus = true;
    this.onChangeCallback = onChange;
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
  }
  public outFocus() {
    this.isFocus = false;
    this.inputElement.selectionStart = null;
    this.inputElement.selectionEnd = null;
    this.inputElement.blur();
    this.onChangeSomething();
    this.inputElement.value = "";
    this.onChangeCallback = undefined;
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
