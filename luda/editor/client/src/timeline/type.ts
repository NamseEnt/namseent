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
};

export type ResizeClip = {
  type: "resizeClip";
  clipId: string;
  side: "left" | "right";
};
export type DragClip = {
  type: "dragClip";
  clipId: string;
  mouseAnchorMs: number;
};

export type Track = {
  clips: Clip[];
};

export type Clip = {
  id: string;
  startMs: number;
  endMs: number;
  mouseIn?: "left" | "right";
};
