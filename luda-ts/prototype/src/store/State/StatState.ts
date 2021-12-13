export type StatState = {
  vocal: number;
  dance: number;
  visual: number;
  sense: number;
  mentality: number;
  stress: number;
  health: number;
  tiredness: number;
  will: number;
};

export const initialStatState: StatState = {
  vocal: 0,
  dance: 0,
  visual: 0,
  sense: 0,
  mentality: 20,
  stress: 0,
  health: 20,
  tiredness: 0,
  will: 20,
};
