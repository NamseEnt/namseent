export type Turn = "hayeon" | "trainer" | "gameover";

export type EvaluationRenderingContext = {
  context: CanvasRenderingContext2D;
  unitSize: number;
  canvasSize: {
    width: number;
    height: number;
  };
  turn: Turn;
  turnChangedAt: number;
  currentTime: number;
};

export const keys = ["w", "a", "s", "d"] as const;

export type SimonKey = typeof keys[number];
