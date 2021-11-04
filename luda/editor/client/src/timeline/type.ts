export type TimelineState = {
  layout: {
    x: number;
    y: number;
    width: number;
    height: number;
    headerWidth: number;
    startMs: number;
    msPerPixel: number;
  };
  tracks: Track[];
  actionState?: ResizeClip | DragClip;
  clipIdMouseIn?: string;
  contextMenu?: ContextMenu;
};

export type ContextMenu = {
  type: "trackBody";
  x: number;
  y: number;
  trackId: string;
  clickMs: number;
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
}

export type Track = {
  id: string;
  type: TrackType;
  clips: Clip[];
};

export type Clip = {
  id: string;
  startMs: number;
  endMs: number;
};
