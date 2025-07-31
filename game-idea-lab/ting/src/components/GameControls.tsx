interface GameControlsProps {
  isGameRunning: boolean
  onStart: () => void
  onStop: () => void
}

export const GameControls = ({ isGameRunning, onStart, onStop }: GameControlsProps) => {
  return (
    <div className="space-x-4">
      {!isGameRunning ? (
        <button
          onClick={onStart}
          className="px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors"
        >
          게임 시작
        </button>
      ) : (
        <button
          onClick={onStop}
          className="px-6 py-3 bg-red-600 hover:bg-red-700 rounded-lg font-semibold transition-colors"
        >
          게임 중지
        </button>
      )}
    </div>
  )
}