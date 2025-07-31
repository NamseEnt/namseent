import type { GameState, GameResult } from '../types/game'

interface GameAreaProps {
  gameState: GameState
  result: GameResult | null
}

export const GameArea = ({ gameState, result }: GameAreaProps) => {
  const getStatusText = () => {
    switch (gameState) {
      case 'idle':
        return '준비...'
      case 'hint':
        return '주의!'
      case 'cue':
        return '지금!'
      case 'result':
        if (result?.success) {
          return `성공! ${result.reactionTime}ms`
        }
        return '실패...'
      default:
        return ''
    }
  }

  return (
    <div className="relative w-96 h-96 bg-gray-800 rounded-lg flex items-center justify-center">
      {/* 힌트 애니메이션 */}
      {gameState === 'hint' && (
        <div className="absolute inset-0 flex items-center justify-center">
          <div className="dust-effect"></div>
        </div>
      )}
      
      {/* 공격 신호 */}
      {gameState === 'cue' && (
        <div className="text-yellow-400 text-8xl font-bold animate-pulse">
          +
        </div>
      )}
      
      {/* 상태 텍스트 */}
      {(gameState === 'idle' || gameState === 'result') && (
        <div className={`text-2xl font-semibold ${
          gameState === 'result' && result?.success ? 'text-green-400' :
          gameState === 'result' && !result?.success ? 'text-red-400' :
          'text-gray-400'
        }`}>
          {getStatusText()}
        </div>
      )}
    </div>
  )
}