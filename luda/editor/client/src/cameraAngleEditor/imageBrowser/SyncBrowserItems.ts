import {
  AfterDraw,
  ColorUtil,
  FontWeight,
  Language,
  PathDrawCommand,
  Rect,
  Render,
  Text,
  TextAlign,
  TextBaseline,
  Translate,
  WhSize,
  XywhRect,
} from "namui";
import fileSystem from "../../fileSystem/fileSystem";
import { ImageBrowserState } from "./type";

export type SyncBrowserItemsState = {
  isLoadingRequested: boolean;
};

export function getDefaultSyncBrowserItemsState(): SyncBrowserItemsState {
  return {
    isLoadingRequested: false,
  };
}

export const SyncBrowserItems: Render<
  {
    imageBrowser: ImageBrowserState;
    syncBrowserItems: SyncBrowserItemsState;
  },
  {}
> = (state, props) => {
  if (state.syncBrowserItems.isLoadingRequested) {
    return;
  }
  return AfterDraw(async () => {
    if (state.syncBrowserItems.isLoadingRequested) {
      return;
    }
    state.syncBrowserItems.isLoadingRequested = true;

    const directoryPath = "images";
    const path = directoryPath;
    const dirents = await fileSystem.list(path);
    const files = dirents.filter((dirent) => dirent.type === "file");
    const imageFileKeyObjects = files.map((dirent) => {
      const splitted = dirent.name.split("-");
      if (splitted.length !== 3) {
        throw new Error(`${dirent.name} is invalid`);
      }
      const [character, pose, emotion] = splitted;
      return {
        character: character!,
        pose: pose!,
        emotion: emotion!,
      };
    });

    state.imageBrowser.imageFileKeyObjects = imageFileKeyObjects;
  });
};
