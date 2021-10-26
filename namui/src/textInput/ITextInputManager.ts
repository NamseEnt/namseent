export interface ITextInputManager {
  setFocus(param: {
    text: string;
    selection: Selection | undefined;
    onChange: OnTextInputChange;
  }): void;
}

// NOTE : Not same with Web's Selection. start would be greater than end.
export type Selection = {
  start: number;
  end: number;
};

export type OnTextInputChange = (event: {
  text: string;
  selection?: Selection;
}) => void;
