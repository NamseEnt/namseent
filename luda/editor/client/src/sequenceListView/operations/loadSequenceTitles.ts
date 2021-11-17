import fileSystem from "../../fileSystem/fileSystem";
import { SequenceListViewState } from "../type";

const extname = ".json";

export async function loadSequenceTitles(state: SequenceListViewState) {
  const loadingState = (state.loadingSequenceTitles ??= {
    isLoading: false,
    shouldReload: false,
  });

  if (loadingState.isLoading) {
    loadingState.shouldReload = true;
    return;
  } else {
    loadingState.isLoading = true;
    loadingState.shouldReload = false;
  }

  const dirents = await fileSystem.list("/sequence");
  const titles = dirents
    .filter((dirent) => dirent.type === "file" && dirent.name.endsWith(extname))
    .map((dirent) => dirent.name.slice(0, -extname.length));
  console.log(titles);
  state.sequenceTitles = titles;

  loadingState.isLoading = false;

  if (loadingState.shouldReload) {
    loadSequenceTitles(state);
  }
}
