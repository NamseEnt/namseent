export function textInputImports({
    memory: _,
}: {
    memory: WebAssembly.Memory;
}): {
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
            _start: number,
            _end: number,
            _direction: number,
        ) => {
            throw new Error("Not implemented");
        },
        text_input_focus: (
            _width: number,
            _text_ptr: number,
            _text_len: number,
            _selection_start: number,
            _selection_end: number,
            _direction: number,
            _prevent_default_codes_ptr: number,
            _prevent_default_codes_len: number,
        ) => {
            throw new Error("Not implemented");
        },
        text_input_blur: () => {
            throw new Error("Not implemented");
        },
    };
}
