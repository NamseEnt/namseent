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
  sequenceTitles: string[];
  loadingSequenceTitles?: LoadState;
  sequenceListScrollY: number;
};

type LoadState = {
  isLoading: boolean;
  shouldReload: boolean;
};
