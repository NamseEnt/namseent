import fileSystem from "../../fileSystem/fileSystem";
import { SequenceListViewState } from "../type";

export async function renameSequence(
  state: SequenceListViewState,
  title: string,
) {
  await fileSystem.rename(
    `/sequence/${state.editingSequenceTitle}.json`,
    `/sequence/${title}.json`,
  );
  state.editingSequenceTitle = title;
  return;
}
