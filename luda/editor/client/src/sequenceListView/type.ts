import { Selection, XywhRect } from "namui";
import { LoadSequence } from "./operations/loadSequence";

export type SequenceListViewState = {
  layout: {
    rect: XywhRect;
    listWidth: number;
  };
  actionState: SequenceListViewActionState;
  newTitle: string;
  textInput: {
    focus: boolean;
    selection?: Selection;
  };
  sequenceTitles?: string[];
  loadingSequenceTitles?: {
    isLoading: boolean;
    shouldReload: boolean;
  };
  loadingSequence?: {
    loadStartAtMs: number;
    title: string;
    state?: LoadSequence.LoadSequenceState;
  };
  sequenceListScrollY: number;
  preloadedSequence?: {
    loadStartAtMs: number;
    title: string;
    state?: LoadSequence.LoadSequenceState;
    lengthMs?: number;
    seekerMs: number;
  };
};

export enum SequenceListViewActionState {
  none = "none",
  addSequence = "addSequence",
  renameSequence = "renameSequence",
}
