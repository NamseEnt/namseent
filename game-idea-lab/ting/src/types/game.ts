export type GameState = 'idle' | 'hint' | 'cue' | 'judgement' | 'result'

export interface GameResult {
  success: boolean
  reactionTime?: number
}