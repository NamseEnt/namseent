import { sendMessageToMainThread } from "../interWorkerProtocol";

export function textInputImports({ memory }: { memory: WebAssembly.Memory }): {
    text_input_set_selection_range: (
        start: number,
        end: number,
        direction: number,
    ) => void;
    text_input_focus: (
        width: number,
        text_ptr: number,
        text_len: number,
        selection_start: number,
        selection_end: number,
        direction: number,
        prevent_default_codes_ptr: number,
        prevent_default_codes_len: number,
    ) => void;
    text_input_blur: () => void;
} {
    return {
        text_input_set_selection_range: (
            start: number,
            end: number,
            direction: number,
        ) => {
            sendMessageToMainThread({
                type: "text-input-set-selection-range",
                start,
                end,
                direction:
                    direction === 0
                        ? "none"
                        : direction === 1
                        ? "forward"
                        : "backward",
            });
        },
        text_input_focus: (
            width: number,
            text_ptr: number,
            text_len: number,
            selection_start: number,
            selection_end: number,
            direction: number,
            prevent_default_codes_ptr: number,
            prevent_default_codes_len: number,
        ) => {
            const buffer = new Uint8Array(text_len);
            buffer.set(new Uint8Array(memory.buffer, text_ptr, text_len));
            const text = new TextDecoder().decode(buffer);
            const preventDefaultCodes = new Uint8Array(
                memory.buffer,
                prevent_default_codes_ptr,
                prevent_default_codes_len,
            );
            sendMessageToMainThread({
                type: "text-input-focus",
                width,
                text,
                selection_start,
                selection_end,
                direction:
                    direction === 0
                        ? "none"
                        : direction === 1
                        ? "forward"
                        : "backward",
                prevent_default_codes: Array.from(preventDefaultCodes),
            });
        },
        text_input_blur: () => {
            sendMessageToMainThread({
                type: "text-input-blur",
            });
        },
    };
}
