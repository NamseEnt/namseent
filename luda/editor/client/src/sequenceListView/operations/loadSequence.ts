import fileSystem from "../../fileSystem/fileSystem";
import { sequenceJsonReviver } from "../../sequenceJson/sequenceJsonReviver";
import { TimelineState, Track } from "../../timeline/type";
import { SequenceListViewState } from "../type";

namespace LoadSequence {
  type LoadingState = {
    type: "loading";
  };

  type LoadedState = {
    type: "loaded";
    tracks: Track[];
  };

  type LoadFailedState = {
    type: "failed";
    errorCode: string;
  };

  type LoadSequenceState = LoadingState | LoadedState | LoadFailedState;
}

export enum LoadSequenceState {
  "loading" = "loading",
}

export async function loadSequence(
  state: {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  title: string,
) {
LoadSequence.  const { timeline, sequenceListView } = state;

  const loadingSequence = (sequenceListView.loadingSequence ??= {
    isLoading: false,
    title,
    startedAt: 0,
  });

  const isLoadingSameSequence =
    loadingSequence.isLoading && loadingSequence.title === title;
  if (isLoadingSameSequence) {
    return;
  }

  loadingSequence.isLoading = true;
  loadingSequence.title = title;
  loadingSequence.startedAt = Date.now();

  const fileReadResult = await fileSystem.read(`/sequence/${title}.json`);

  const targetSequenceChanged = loadingSequence.title !== title;
  if (targetSequenceChanged) {
    return;
  }

  const loadingCanceled = !loadingSequence.isLoading;
  if (loadingCanceled) {
    return;
  }

  if (!fileReadResult.isSuccessful) {
    loadingSequence.isLoading = false;
    loadingSequence.errorCode = fileReadResult.errorCode;
    return;
  }

  const dataBlob = new Blob([
    new Uint8Array(Object.values(fileReadResult.file)),
  ]);
  const dataString = await dataBlob.text();

  try {
    const tracks = JSON.parse(dataString, sequenceJsonReviver) as Track[];
    sequenceListView.editingSequenceTitle = title;
    timeline.tracks = tracks;
  } catch (error: any) {
    switch (error.name) {
      case "SyntaxError": {
        loadingSequence.errorCode = "SyntaxError";
        break;
      }

      default: {
        throw error;
      }
    }
  }

  loadingSequence.isLoading = false;
}
