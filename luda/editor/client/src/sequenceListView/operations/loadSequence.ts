import fileSystem from "../../fileSystem/fileSystem";
import { sequenceJsonReviver } from "../../sequenceJson/sequenceJsonReviver";
import { TimelineState, Track } from "../../timeline/type";
import { SequenceListViewState } from "../type";

export async function loadSequence(
  state: {
    timeline: TimelineState;
    sequenceListView: SequenceListViewState;
  },
  title: string,
) {
  const { timeline, sequenceListView } = state;

  const loadingSequence = (sequenceListView.loadingSequence ??= {
    isLoading: false,
    shouldReload: false,
    title,
  });

  const isLoadingSameSequence =
    loadingSequence.isLoading && loadingSequence.title === title;
  if (isLoadingSameSequence) {
    loadingSequence.shouldReload = true;
    return;
  }

  loadingSequence.isLoading = true;
  loadingSequence.title = title;
  loadingSequence.shouldReload = false;

  const dataBuffer = await fileSystem.read(`/sequence/${title}.json`);
  const dataBlob = new Blob([new Uint8Array(Object.values(dataBuffer))]);
  const dataString = await dataBlob.text();

  const targetSequenceChanged = loadingSequence.title !== title;
  if (targetSequenceChanged) {
    return;
  }

  try {
    const tracks = JSON.parse(dataString, sequenceJsonReviver) as Track[];
    sequenceListView.editingSequenceTitle = title;
    timeline.tracks = tracks;
  } catch {}

  loadingSequence.isLoading = false;

  if (loadingSequence.shouldReload) {
    loadSequence(state, title);
  }
}
