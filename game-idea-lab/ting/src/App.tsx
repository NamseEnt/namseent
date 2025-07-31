import { useParryGame } from './hooks/useParryGame'
import { GameArea } from './components/GameArea'
import './App.css'

function App() {
  const { gameState, result, isGameRunning, startGame, hasGameStarted } = useParryGame()

  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center">
      <div className="text-center space-y-8">
        {!hasGameStarted ? (
          <>
            <button
              onClick={startGame}
              className="px-8 py-4 bg-blue-600 hover:bg-blue-700 rounded-lg text-xl font-semibold transition-colors"
            >
              게임 시작
            </button>
            <p className="text-gray-400">
              스페이스바를 눌러 패링하세요!
            </p>
          </>
        ) : (
          <>
            <GameArea gameState={gameState} result={result} />
            {isGameRunning && (
              <p className="text-gray-400">
                스페이스바를 눌러 패링하세요!
              </p>
            )}
          </>
        )}
      </div>
    </div>
  )
}

export default App