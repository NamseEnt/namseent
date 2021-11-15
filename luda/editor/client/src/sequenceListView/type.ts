import { Selection, XywhRect } from "namui";

export type SequenceListViewState = {
  layout: {
    rect: XywhRect;
  };
  addingSequence: boolean;
  editingFileName?: string;
  newTitle: string;
  textInput: {
    focus: boolean;
    selection?: Selection;
  };
};
