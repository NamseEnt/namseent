import {
    EVENT_TYPE,
    OnTextInputEvent,
    isKeyPreventDefaultException,
} from "./eventSystem";
import { CODES } from "./imports/codes";

export class TextInput {
    textarea: HTMLTextAreaElement;
    textInputPreventDefaultCodes: string[] = [];

    constructor(onTextInputEvent: OnTextInputEvent) {
        const textarea = (this.textarea = document.createElement("textarea"));

        // NOTE: Below codes from https://github.com/goldfire/CanvasInput/blob/5adbaf00bd42665f3c691796881c7a7a9cf7036c/CanvasInput.js#L126
        textarea.style.position = "absolute";
        textarea.style.opacity = "0";
        textarea.style.pointerEvents = "none";
        textarea.style.zIndex = "0";
        textarea.style.top = "0px";
        // hide native blue text cursor on iOS
        textarea.style.transform = "scale(0)";

        document.body.appendChild(textarea);

        textarea.addEventListener("input", () => {
            onTextInputEvent(textarea, EVENT_TYPE.TEXT_INPUT);
        });

        textarea.addEventListener("keydown", (event) => {
            event.stopImmediatePropagation();
            if (
                [
                    "ArrowUp",
                    "ArrowDown",
                    "Home",
                    "End",
                    "PageUp",
                    "PageDown",
                    ...this.textInputPreventDefaultCodes,
                ].includes(event.code) &&
                !isKeyPreventDefaultException(event)
            ) {
                event.preventDefault();
            }
            const codeU8 = CODES[event.code as keyof typeof CODES];
            if (!codeU8) {
                console.warn(`Unknown key code: ${event.code}`);
                return;
            }
            onTextInputEvent(textarea, EVENT_TYPE.TEXT_INPUT_KEY_DOWN, codeU8);
        });

        document.addEventListener("selectionchange", () => {
            onTextInputEvent(textarea, EVENT_TYPE.SELECTION_CHANGE);
        });
    }

    public onMessage(
        message:
            | {
                  type: "text-input-set-selection-range";
                  start: number;
                  end: number;
                  direction: "forward" | "backward" | "none";
              }
            | {
                  type: "text-input-focus";
                  width: number;
                  text: string;
                  selection_start: number;
                  selection_end: number;
                  direction: "forward" | "backward" | "none";
                  prevent_default_codes: number[];
              }
            | { type: "text-input-blur" },
    ) {
        switch (message.type) {
            case "text-input-set-selection-range": {
                this.textarea.setSelectionRange(
                    message.start,
                    message.end,
                    message.direction,
                );
                break;
            }
            case "text-input-focus": {
                console.warn("text-input-focus");
                this.textInputPreventDefaultCodes =
                    message.prevent_default_codes.map(
                        (code) =>
                            Object.keys(CODES).find(
                                (key) =>
                                    CODES[key as keyof typeof CODES] === code,
                            )!,
                    );
                this.textarea.style.width = `${message.width}px`;
                this.textarea.value = message.text;
                this.textarea.setSelectionRange(
                    message.selection_start,
                    message.selection_end,
                    message.direction,
                );
                this.textarea.focus();
                break;
            }
            case "text-input-blur": {
                this.textarea.blur();
                break;
            }
        }
    }
}
