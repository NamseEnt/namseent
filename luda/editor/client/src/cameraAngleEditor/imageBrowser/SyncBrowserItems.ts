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
    const imageFilenameObjects = files.map((dirent) => {
      const splitted = dirent.name.split("-");
      const [character, pose, emotionAndExtension] = splitted;
      if (!character || !pose || !emotionAndExtension) {
        throw new Error(`${dirent.name} is invalid`);
      }
      const extensionDotIndex = emotionAndExtension.lastIndexOf(".");
      if (extensionDotIndex === -1) {
        throw new Error(`${dirent.name} is invalid`);
      }

      const emotion = emotionAndExtension.substring(0, extensionDotIndex);
      const extension = emotionAndExtension.substring(extensionDotIndex + 1);
      return {
        character,
        pose,
        emotion,
        extension,
      };
    });

    state.imageBrowser.imageFilenameObjects = imageFilenameObjects;
  });
};
