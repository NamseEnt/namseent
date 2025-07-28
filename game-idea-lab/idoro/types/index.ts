// Type definitions for the app

export interface GameState {
  playerName: string;
  cheerPower: number;
  currentSession: SessionType;
}

export type SessionType = 'idle' | 'focusing' | 'resting';

export type IdolState = 'idle' | 'focusing' | 'resting';

export interface SessionParams {
  success: string;
  earnedPower?: string;
  totalPower?: string;
}

export interface TimerState {
  timeLeft: number;
  startTime: number;
  isRunning: boolean;
}