import fileSystem from "../../fileSystem/fileSystem";
import { sequenceJsonReviver } from "../../sequenceJson/sequenceJsonReviver";
import { Track } from "../../timeline/type";

export namespace LoadSequence {
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

  export type LoadSequenceState = {
    startAtMs: number;
  } & (LoadingState | LoadedState | LoadFailedState);

  const loadingSequenceStates = new Map<string, LoadSequenceState>();

  export function getLoadingSequenceState(
    title: string,
  ): LoadSequenceState | undefined {
    return loadingSequenceStates.get(title);
  }

  export function trySaveLoadingSequenceState({
    title,
    state,
  }: {
    title: string;
    state: LoadSequenceState;
  }): boolean {
    const loadingSequenceState = loadingSequenceStates.get(title);
    if (
      loadingSequenceState &&
      loadingSequenceState.startAtMs > state.startAtMs
    ) {
      return false;
    }

    loadingSequenceStates.set(title, state);
    return true;
  }
}

export function loadSequence({
  loadStartAtMs,
  title,
}: {
  loadStartAtMs: number;
  title: string;
}): LoadSequence.LoadSequenceState {
  const loadingSequenceState = LoadSequence.getLoadingSequenceState(title);
  if (!loadingSequenceState || loadingSequenceState.startAtMs < loadStartAtMs) {
    startLoad(loadStartAtMs, title);
    return {
      type: "loading",
      startAtMs: loadStartAtMs,
    };
  }

  return loadingSequenceState;
}

async function startLoad(
  loadSequenceStartAtMs: number,
  title: string,
): Promise<void> {
  try {
    LoadSequence.trySaveLoadingSequenceState({
      title,
      state: {
        type: "loading",
        startAtMs: loadSequenceStartAtMs,
      },
    });

    const fileReadResult = await fileSystem.read(`/sequence/${title}.json`);

    if (!fileReadResult.isSuccessful) {
      LoadSequence.trySaveLoadingSequenceState({
        title,
        state: {
          type: "failed",
          startAtMs: loadSequenceStartAtMs,
          errorCode: fileReadResult.errorCode,
        },
      });
      return;
    }

    const textDecoder = new TextDecoder();

    const dataString = textDecoder.decode(
      new Uint8Array(Object.values(fileReadResult.file)),
    );
    const tracks = JSON.parse(dataString, sequenceJsonReviver) as Track[];
    LoadSequence.trySaveLoadingSequenceState({
      title,
      state: {
        type: "loaded",
        startAtMs: loadSequenceStartAtMs,
        tracks,
      },
    });
  } catch (error) {
    const errorCode =
      error instanceof SyntaxError ? "SyntaxError" : "UnknownError";

    LoadSequence.trySaveLoadingSequenceState({
      title,
      state: {
        type: "failed",
        startAtMs: loadSequenceStartAtMs,
        errorCode,
      },
    });
    console.error(error);
  }
}
