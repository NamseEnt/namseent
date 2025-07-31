import { useState, useEffect, useCallback, useRef } from 'react'
import type { GameState, GameResult } from '../types/game'
import { GAME_CONFIG } from '../constants/game'

export const useParryGame = () => {
  const [gameState, setGameState] = useState<GameState>('idle')
  const [result, setResult] = useState<GameResult | null>(null)
  const [isGameRunning, setIsGameRunning] = useState(false)
  
  const cueTimestampRef = useRef<number>(0)
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null)
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
      const randomDelay = Math.random() * (GAME_CONFIG.MAX_IDLE_TIME - GAME_CONFIG.MIN_IDLE_TIME) + GAME_CONFIG.MIN_IDLE_TIME
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
      }, GAME_CONFIG.HINT_DURATION)
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
      }, GAME_CONFIG.REACTION_TIME_LIMIT)
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
      }, GAME_CONFIG.RESULT_DISPLAY_TIME)
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
      }, GAME_CONFIG.RESULT_DISPLAY_TIME)
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
    audioRef.current = new Audio(GAME_CONFIG.AUDIO_FILE_PATH)
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

  return {
    gameState,
    result,
    isGameRunning,
    startGame,
    stopGame
  }
}