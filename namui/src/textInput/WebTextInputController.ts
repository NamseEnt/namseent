import {
  ITextInputController,
  OnTextInputChange,
  Selection,
} from "./ITextInputController";

export class WebTextInputController implements ITextInputController {
  private readonly inputElement = document.createElement("input");
  private onChangeCallback: OnTextInputChange | undefined = undefined;
  private lastSelection:
    | {
        start: HTMLInputElement["selectionStart"];
        end: HTMLInputElement["selectionEnd"];
      }
    | undefined = undefined;
  constructor() {
    document.body.appendChild(this.inputElement);
    document.addEventListener("selectionchange", (event) => {
      const selection = this.getSelection();
      if (
        !this.lastSelection ||
        this.lastSelection.start !== selection?.start ||
        this.lastSelection.end !== selection.end
      ) {
        this.onChangeSomething();
        this.lastSelection = selection;
      }
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
  public setFocus(text: string, onChange: OnTextInputChange): void {
    this.inputElement.focus({ preventScroll: true });
    this.inputElement.value = text;
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
    this.inputElement.selectionStart = null;
    this.inputElement.selectionEnd = null;
    this.inputElement.blur();
    this.onChangeSomething();
    this.onChangeCallback = undefined;
    this.inputElement.value = "";
  }
  private getSelection(): Selection | undefined {
    if (
      this.inputElement.selectionStart === null ||
      this.inputElement.selectionEnd === null
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
