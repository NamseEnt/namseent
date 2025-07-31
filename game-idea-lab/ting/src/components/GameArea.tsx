import type { GameState, GameResult } from '../types/game'
import { CueFlash } from './CueFlash'

interface GameAreaProps {
  gameState: GameState
  result: GameResult | null
}

export const GameArea = ({ gameState, result }: GameAreaProps) => {
  const statusColor = result?.success ? 'text-green-400' : 'text-red-400'

  return (
    <div className="fixed inset-0 bg-gray-900">
      {gameState === 'idle' && (
        <div className="idle-effect"></div>
      )}
      
      {gameState === 'hint' && (
        <div className="hint-effect"></div>
      )}
      
      {gameState === 'cue' && <CueFlash />}
      
      {gameState === 'result' && (
        <div className="absolute inset-0 flex items-center justify-center">
          <div className={`text-4xl font-semibold ${statusColor}`}>
            {result?.success ? `성공! ${result.reactionTime}ms` : '실패...'}
          </div>
        </div>
      )}
    </div>
  )
}