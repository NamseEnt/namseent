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
  } as typeof state.preloadedSequence)!;

  const dataBuffer = await fileSystem.read(`/sequence/${title}.json`);
  const dataBlob = new Blob([new Uint8Array(Object.values(dataBuffer))]);
  const dataString = await dataBlob.text();

  const targetSequenceChanged = preloadedSequence.title !== title;
  if (targetSequenceChanged) {
    return;
  }

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
  } catch {
    preloadedSequence.isSequence = false;
  }

  preloadedSequence.isLoading = false;
}
