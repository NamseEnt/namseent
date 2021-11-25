import { Selection, XywhRect } from "namui";
import { Track } from "../timeline/type";
import { LoadSequence } from "./operations/loadSequence";

export type SequenceListViewState = {
  layout: {
    rect: XywhRect;
    listWidth: number;
  };
  actionState: SequenceListViewActionState;
  editingSequenceTitle?: string;
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
  preloadedSequence?: LoadingStateWithTimeout & {
    title: string;
    isSequence: boolean;
    tracks: Track[];
    lengthMs: number;
    seekerMs: number;
  };
};

export enum SequenceListViewActionState {
  none = "none",
  addSequence = "addSequence",
  renameSequence = "renameSequence",
}

export type LoadingStateWithTimeout = {
  isLoading: boolean;
  startedAt: number;
  errorCode?: string;
};
