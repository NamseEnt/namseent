import type { GameState, GameResult } from '../types/game'

interface GameAreaProps {
  gameState: GameState
  result: GameResult | null
}

export const GameArea = ({ gameState, result }: GameAreaProps) => {
  const statusText = {
    idle: '준비...',
    hint: '주의!',
    cue: '지금!',
    result: result?.success ? `성공! ${result.reactionTime}ms` : '실패...'
  }

  const statusColor = gameState === 'result' 
    ? (result?.success ? 'text-green-400' : 'text-red-400')
    : 'text-gray-400'

  return (
    <div className="relative w-96 h-96 bg-gray-800 rounded-lg flex items-center justify-center">
      {gameState === 'hint' && (
        <div className="absolute inset-0 flex items-center justify-center">
          <div className="dust-effect"></div>
        </div>
      )}
      
      {gameState === 'cue' && (
        <div className="text-yellow-400 text-8xl font-bold animate-pulse">+</div>
      )}
      
      {(gameState === 'idle' || gameState === 'result') && (
        <div className={`text-2xl font-semibold ${statusColor}`}>
          {statusText[gameState]}
        </div>
      )}
    </div>
  )
}