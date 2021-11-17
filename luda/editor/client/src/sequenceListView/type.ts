import { Selection, XywhRect } from "namui";
import { Track } from "../timeline/type";

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
  preloadedSequence?: {
    title: string;
    isLoading: boolean;
    isSequence: boolean;
    tracks: Track[];
    lengthMs: number;
    seekerMs: number;
  };
};

type LoadState = {
  isLoading: boolean;
  shouldReload: boolean;
};
