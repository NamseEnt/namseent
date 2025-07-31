import { useParryGame } from './hooks/useParryGame'
import { GameArea } from './components/GameArea'
import './App.css'

function App() {
  const { gameState, result, startGame, hasGameStarted } = useParryGame()

  return (
    <div className="min-h-screen bg-gray-900 text-white relative overflow-hidden">
      {!hasGameStarted ? (
        <div className="h-screen flex items-center justify-center">
          <button
            onClick={startGame}
            className="px-8 py-4 bg-blue-600 hover:bg-blue-700 rounded-lg text-xl font-semibold transition-colors"
          >
            게임 시작
          </button>
        </div>
      ) : (
        <GameArea gameState={gameState} result={result} />
      )}
    </div>
  )
}

export default App