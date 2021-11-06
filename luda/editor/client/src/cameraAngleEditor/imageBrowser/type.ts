import { XywhRect } from "namui";
import { ImageFilenameObject } from "./ImageFilenameObject";
import { SyncBrowserItemsState } from "./SyncBrowserItems";

export type ImageBrowserState = {
  layout: XywhRect & {
    currentDirectoryLabel: XywhRect & {};
  };
  /**
   * key syntax
   *  alphanumeric
   *  key + "-" + alphanumeric
   */
  key: string;
  syncBrowserItems: SyncBrowserItemsState;
  imageFilenameObjects: ImageFilenameObject[];
};
