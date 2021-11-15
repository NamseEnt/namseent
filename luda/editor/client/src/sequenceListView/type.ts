import { Selection, XywhRect } from "namui";

export type SequenceListViewState = {
  layout: {
    rect: XywhRect;
  };
  editingFileName?: string;
};
