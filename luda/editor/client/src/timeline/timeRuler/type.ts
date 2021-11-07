export type TimeRulerState = {};

export type TimeRulerProps = {
  layout: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
  msPerPixel: number;
  startMs: number;
};
