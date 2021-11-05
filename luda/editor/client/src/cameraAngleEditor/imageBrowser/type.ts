import { XywhRect } from "namui";
import { ImageFileKeyObject } from "./ImageFileKeyObject";
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
  imageFileKeyObjects: ImageFileKeyObject[];
};
