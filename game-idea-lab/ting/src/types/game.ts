export type GameState = 'idle' | 'hint' | 'cue' | 'result'

export interface GameResult {
  success: boolean
  reactionTime?: number
}