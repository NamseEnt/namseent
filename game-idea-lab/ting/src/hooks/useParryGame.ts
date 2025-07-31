import { useState, useEffect } from 'react'
import type { GameState, GameResult } from '../types/game'
import { GAME_CONFIG } from '../constants/game'

// 오디오 객체를 전역으로 관리 (한 번만 생성)
let audio: HTMLAudioElement | null = null

const getAudio = () => {
  if (!audio) {
    audio = new Audio(GAME_CONFIG.AUDIO_FILE_PATH)
    audio.preload = 'auto'
  }
  return audio
}

export const useParryGame = () => {
  const [gameState, setGameState] = useState<GameState>('idle')
  const [result, setResult] = useState<GameResult | null>(null)
  const [isGameRunning, setIsGameRunning] = useState(false)
  const [cueTimestamp, setCueTimestamp] = useState<number>(0)

  const startGame = () => {
    setIsGameRunning(true)
    setResult(null)
    setGameState('idle')
  }

  const stopGame = () => {
    setIsGameRunning(false)
    setGameState('idle')
    setResult(null)
  }

  // 게임 로직 effect
  useEffect(() => {
    if (!isGameRunning || gameState !== 'idle') return

    const randomDelay = Math.random() * (GAME_CONFIG.MAX_IDLE_TIME - GAME_CONFIG.MIN_IDLE_TIME) + GAME_CONFIG.MIN_IDLE_TIME
    
    const idleTimeout = setTimeout(() => {
      if (!isGameRunning) return
      setGameState('hint')
    }, randomDelay)

    return () => clearTimeout(idleTimeout)
  }, [isGameRunning, gameState])

  // hint -> cue 전환
  useEffect(() => {
    if (gameState !== 'hint') return

    const hintTimeout = setTimeout(() => {
      setGameState('cue')
      setCueTimestamp(Date.now())
      getAudio().play().catch(() => {})
    }, GAME_CONFIG.HINT_DURATION)

    return () => clearTimeout(hintTimeout)
  }, [gameState])

  // cue -> 자동 실패
  useEffect(() => {
    if (gameState !== 'cue') return

    const failTimeout = setTimeout(() => {
      setGameState('result')
      setResult({ success: false })
    }, GAME_CONFIG.REACTION_TIME_LIMIT)

    return () => clearTimeout(failTimeout)
  }, [gameState])

  // result -> idle 전환
  useEffect(() => {
    if (gameState !== 'result') return

    const resultTimeout = setTimeout(() => {
      setGameState('idle')
    }, GAME_CONFIG.RESULT_DISPLAY_TIME)

    return () => clearTimeout(resultTimeout)
  }, [gameState])

  // 키보드 입력 처리
  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.code === 'Space' && gameState === 'cue') {
        e.preventDefault()
        const reactionTime = Date.now() - cueTimestamp
        setGameState('result')
        setResult({ success: true, reactionTime })
      }
    }

    window.addEventListener('keydown', handleKeyPress)
    return () => window.removeEventListener('keydown', handleKeyPress)
  }, [gameState, cueTimestamp])

  return {
    gameState,
    result,
    isGameRunning,
    startGame,
    stopGame
  }
}