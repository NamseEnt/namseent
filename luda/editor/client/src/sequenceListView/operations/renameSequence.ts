import fileSystem from "../../fileSystem/fileSystem";
import { SequenceListViewState } from "../type";
import { loadSequenceTitles } from "./loadSequenceTitles";

export async function renameSequence(
  state: SequenceListViewState,
  title: string,
) {
  if (!state.preloadedSequence?.title) {
    return;
  }
  await fileSystem.rename(
    `/sequence/${state.preloadedSequence.title}.json`,
    `/sequence/${title}.json`,
  );
  await loadSequenceTitles(state);
  state.preloadedSequence.title = title;

  return;
}
