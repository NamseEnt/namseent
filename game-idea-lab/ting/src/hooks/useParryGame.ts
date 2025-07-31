import { useState, useEffect, useRef } from 'react'
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
  
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null)

  const clearGameTimeout = () => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current)
      timeoutRef.current = null
    }
  }

  const scheduleNextRound = () => {
    const randomDelay = Math.random() * (GAME_CONFIG.MAX_IDLE_TIME - GAME_CONFIG.MIN_IDLE_TIME) + GAME_CONFIG.MIN_IDLE_TIME
    
    timeoutRef.current = setTimeout(() => {
      setGameState('hint')
      
      timeoutRef.current = setTimeout(() => {
        setGameState('cue')
        const timestamp = Date.now()
        setCueTimestamp(timestamp)
        
        getAudio().play().catch(() => {})
        
        timeoutRef.current = setTimeout(() => {
          setGameState('result')
          setResult({ success: false })
          
          timeoutRef.current = setTimeout(() => {
            setGameState('idle')
          }, GAME_CONFIG.RESULT_DISPLAY_TIME)
        }, GAME_CONFIG.REACTION_TIME_LIMIT)
      }, GAME_CONFIG.HINT_DURATION)
    }, randomDelay)
  }

  const handleSuccess = (reactionTime: number) => {
    clearGameTimeout()
    setGameState('result')
    setResult({ success: true, reactionTime })
    
    timeoutRef.current = setTimeout(() => {
      setGameState('idle')
    }, GAME_CONFIG.RESULT_DISPLAY_TIME)
  }

  const startGame = () => {
    setIsGameRunning(true)
    setResult(null)
    setGameState('idle')
  }

  const stopGame = () => {
    setIsGameRunning(false)
    clearGameTimeout()
    setGameState('idle')
    setResult(null)
  }

  useEffect(() => {
    if (isGameRunning && gameState === 'idle') {
      scheduleNextRound()
    }
  }, [isGameRunning, gameState])

  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.code === 'Space' && gameState === 'cue') {
        e.preventDefault()
        const reactionTime = Date.now() - cueTimestamp
        handleSuccess(reactionTime)
      }
    }

    window.addEventListener('keydown', handleKeyPress)
    return () => window.removeEventListener('keydown', handleKeyPress)
  }, [gameState, cueTimestamp])

  useEffect(() => {
    return () => clearGameTimeout()
  }, [])

  return {
    gameState,
    result,
    isGameRunning,
    startGame,
    stopGame
  }
}