import { useState, useEffect, useCallback, useRef } from 'react'
import './App.css'

type GameState = 'idle' | 'hint' | 'cue' | 'judgement' | 'result'

interface GameResult {
  success: boolean
  reactionTime?: number
}

function App() {
  const [gameState, setGameState] = useState<GameState>('idle')
  const [result, setResult] = useState<GameResult | null>(null)
  const [isGameRunning, setIsGameRunning] = useState(false)
  
  const cueTimestampRef = useRef<number>(0)
  const timeoutRef = useRef<NodeJS.Timeout | null>(null)
  const audioRef = useRef<HTMLAudioElement | null>(null)
  const isGameRunningRef = useRef(false)
  const functionsRef = useRef<{
    scheduleNextRound?: () => void
    startHintPhase?: () => void
    startCuePhase?: () => void
    handleFailure?: () => void
    handleSuccess?: (reactionTime: number) => void
  }>({})

  // isGameRunning 값을 ref에 동기화
  useEffect(() => {
    isGameRunningRef.current = isGameRunning
  }, [isGameRunning])

  // 함수들을 ref에 저장
  useEffect(() => {
    functionsRef.current.scheduleNextRound = () => {
      const randomDelay = Math.random() * 3000 + 2000 // 2-5초 사이
      timeoutRef.current = setTimeout(() => {
        if (!isGameRunningRef.current) return
        functionsRef.current.startHintPhase?.()
      }, randomDelay)
    }

    functionsRef.current.startHintPhase = () => {
      setGameState('hint')
      timeoutRef.current = setTimeout(() => {
        if (!isGameRunningRef.current) return
        functionsRef.current.startCuePhase?.()
      }, 1500) // 1.5초 동안 힌트 표시
    }

    functionsRef.current.startCuePhase = () => {
      setGameState('cue')
      cueTimestampRef.current = Date.now()
      
      // 효과음 재생
      if (audioRef.current) {
        audioRef.current.play().catch(() => {
          // 오디오 재생 실패 시 무시
        })
      }

      // 300ms 후 자동 실패 처리
      timeoutRef.current = setTimeout(() => {
        if (!isGameRunningRef.current) return
        functionsRef.current.handleFailure?.()
      }, 300)
    }

    functionsRef.current.handleFailure = () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current)
      }
      setGameState('result')
      setResult({ success: false })
      
      // 2초 후 다음 라운드
      timeoutRef.current = setTimeout(() => {
        if (!isGameRunningRef.current) return
        setGameState('idle')
        functionsRef.current.scheduleNextRound?.()
      }, 2000)
    }

    functionsRef.current.handleSuccess = (reactionTime: number) => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current)
      }
      setGameState('result')
      setResult({ success: true, reactionTime })
      
      // 2초 후 다음 라운드
      timeoutRef.current = setTimeout(() => {
        if (!isGameRunningRef.current) return
        setGameState('idle')
        functionsRef.current.scheduleNextRound?.()
      }, 2000)
    }
  }, [])

  // 게임 시작
  const startGame = useCallback(() => {
    setIsGameRunning(true)
    setResult(null)
    setGameState('idle')
    functionsRef.current.scheduleNextRound?.()
  }, [])

  // 게임 정지
  const stopGame = useCallback(() => {
    setIsGameRunning(false)
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current)
    }
    setGameState('idle')
    setResult(null)
  }, [])

  // 키보드 입력 처리
  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.code === 'Space' && gameState === 'cue') {
        e.preventDefault()
        const reactionTime = Date.now() - cueTimestampRef.current
        functionsRef.current.handleSuccess?.(reactionTime)
      }
    }

    window.addEventListener('keydown', handleKeyPress)
    return () => window.removeEventListener('keydown', handleKeyPress)
  }, [gameState])

  // 오디오 초기화
  useEffect(() => {
    audioRef.current = new Audio('/ting.mp3')
    audioRef.current.preload = 'auto'
    // 오디오 로드 실패 시 무시
    audioRef.current.addEventListener('error', () => {
      console.log('오디오 파일을 로드할 수 없습니다. 효과음 없이 진행합니다.')
    })
  }, [])

  // 언마운트 시 타이머 정리
  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current)
      }
    }
  }, [])

  // 상태별 UI 텍스트
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
    <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center">
      <div className="text-center space-y-8">
        <h1 className="text-4xl font-bold mb-8">띵! - 패링 게임</h1>
        
        {/* 게임 영역 */}
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
        
        {/* 컨트롤 버튼 */}
        <div className="space-x-4">
          {!isGameRunning ? (
            <button
              onClick={startGame}
              className="px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg font-semibold transition-colors"
            >
              게임 시작
            </button>
          ) : (
            <button
              onClick={stopGame}
              className="px-6 py-3 bg-red-600 hover:bg-red-700 rounded-lg font-semibold transition-colors"
            >
              게임 중지
            </button>
          )}
        </div>
        
        <p className="text-gray-400">
          스페이스바를 눌러 패링하세요!
        </p>
      </div>
    </div>
  )
}

export default App