import fileSystem from "../../fileSystem/fileSystem";
import { Track } from "../../timeline/type";
import { SequenceListViewState } from "../type";

export async function preloadSequence(
  state: SequenceListViewState,
  title: string,
) {
  const preloadedSequence = (state.preloadedSequence = {
    isLoading: true,
    title,
    isSequence: false,
    tracks: [],
    lengthMs: 0,
    seekerMs: 0,
    startedAt: Date.now(),
  } as typeof state.preloadedSequence)!;

  const fileReadResult = await fileSystem.read(`/sequence/${title}.json`);

  const targetSequenceChanged = preloadedSequence.title !== title;
  if (targetSequenceChanged) {
    return;
  }

  const loadingCanceled = !preloadedSequence.isLoading;
  if (loadingCanceled) {
    return;
  }

  if (!fileReadResult.isSuccessful) {
    preloadedSequence.isLoading = false;
    preloadedSequence.errorCode = fileReadResult.errorCode;
    return;
  }

  const dataBlob = new Blob([
    new Uint8Array(Object.values(fileReadResult.file)),
  ]);
  const dataString = await dataBlob.text();

  try {
    const tracks = JSON.parse(dataString) as Track[];
    preloadedSequence.isSequence = true;
    preloadedSequence.tracks = tracks;
    preloadedSequence.lengthMs = tracks.reduce(
      (trackLength, track) =>
        Math.max(
          trackLength,
          track.clips.reduce(
            (clipLength, clip) => Math.max(clipLength, clip.endMs),
            0,
          ),
        ),
      0,
    );
  } catch (error: any) {
    switch (error.name) {
      case "SyntaxError": {
        preloadedSequence.isSequence = false;
        preloadedSequence.errorCode = "SyntaxError";
        break;
      }

      default: {
        throw error;
      }
    }
  }

  preloadedSequence.isLoading = false;
}
