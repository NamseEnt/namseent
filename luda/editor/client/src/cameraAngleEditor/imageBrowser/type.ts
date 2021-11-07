import { XywhRect } from "namui";
import { ImageFilenameObject } from "./ImageFilenameObject";
import { ScrollState } from "./Scroll";
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
  directoryKey: string;
  selectedKey?: string;
  syncBrowserItems: SyncBrowserItemsState;
  imageFilenameObjects: ImageFilenameObject[];
  scrollState: ScrollState;
};
