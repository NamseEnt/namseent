import fileSystem from "../../fileSystem/fileSystem";
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
  if (loadingSequence.isLoading && loadingSequence.title === title) {
    loadingSequence.shouldReload = true;
    return;
  }

  loadingSequence.isLoading = true;
  loadingSequence.title = title;
  loadingSequence.shouldReload = false;

  const dataBuffer = await fileSystem.read(`/sequence/${title}.json`);
  const dataBlob = new Blob([new Uint8Array(Object.values(dataBuffer))]);
  const dataString = await dataBlob.text();

  if (loadingSequence.title !== title) {
    return;
  }

  try {
    const tracks = JSON.parse(dataString) as Track[];
    sequenceListView.editingFileName = title;
    timeline.tracks = tracks;
  } catch {}

  loadingSequence.isLoading = false;
}
