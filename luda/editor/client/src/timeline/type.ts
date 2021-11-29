import { History, Immutable } from "history";
import { Clip } from "../type";

export type Sequence = {
  tracks: Track[];
};
export type TimelineHistoryState = Immutable<Sequence>;

export type TimelineHistory = History<TimelineHistoryState>;

export type TimelineState = {
  layout: {
    x: number;
    y: number;
    width: number;
    height: number;
    headerWidth: number;
    startMs: number;
    msPerPixel: number;
    timeRulerHeight: number;
  };
  actionState?: ResizeClip | DragClip;
  clipIdMouseIn?: string;
  contextMenu?: ContextMenuState;
  selectedClip?: Clip;
  readonly timelineBorderId: string;
  history: TimelineHistory;
};

export type ContextMenuState = TrackBodyContextMenuState | ClipContextMenuState;

export type TrackBodyContextMenuState = {
  type: "trackBody";
  x: number;
  y: number;
  trackId: string;
  clickMs: number;
  mouseInItemId?: string;
};

export type ClipContextMenuState = {
  type: "clip";
  x: number;
  y: number;
  trackId: string;
  clipId: string;
  mouseInItemId?: string;
};

export type ResizeClip = {
  type: "resizeClip";
  clipId: string;
  side: "left" | "right";
  /**
   * The delta ms from side to mouse anchor.
   * For left side, value starts from left to right.
   * For right side, value starts from right to left.
   */
  sashMouseAnchorMs: number;
};
export type DragClip = {
  type: "dragClip";
  clipId: string;
  mouseAnchorMs: number;
};

export enum TrackType {
  camera = "camera",
  subtitle = "subtitle",
}

export type Track = {
  id: string;
  type: TrackType;
  clips: Clip[];
};
