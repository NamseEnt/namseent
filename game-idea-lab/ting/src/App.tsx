import { useParryGame } from './hooks/useParryGame'
import { GameArea } from './components/GameArea'
import { GameControls } from './components/GameControls'
import './App.css'

function App() {
  const { gameState, result, isGameRunning, startGame, stopGame } = useParryGame()

  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center">
      <div className="text-center space-y-8">
        <h1 className="text-4xl font-bold mb-8">띵! - 패링 게임</h1>
        
        <GameArea gameState={gameState} result={result} />
        
        <GameControls 
          isGameRunning={isGameRunning}
          onStart={startGame}
          onStop={stopGame}
        />
        
        <p className="text-gray-400">
          스페이스바를 눌러 패링하세요!
        </p>
      </div>
    </div>
  )
}

export default App