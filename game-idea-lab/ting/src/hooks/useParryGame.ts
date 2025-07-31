import { useState, useEffect, useRef } from 'react'
import type { GameState, GameResult } from '../types/game'
import { GAME_CONFIG } from '../constants/game'

export const useParryGame = () => {
  const [gameState, setGameState] = useState<GameState>('idle')
  const [result, setResult] = useState<GameResult | null>(null)
  const [isGameRunning, setIsGameRunning] = useState(false)
  
  const cueTimestampRef = useRef<number>(0)
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const audioRef = useRef<HTMLAudioElement | null>(null)

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
        cueTimestampRef.current = Date.now()
        
        if (audioRef.current) {
          audioRef.current.play().catch(() => {})
        }
        
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
        const reactionTime = Date.now() - cueTimestampRef.current
        handleSuccess(reactionTime)
      }
    }

    window.addEventListener('keydown', handleKeyPress)
    return () => window.removeEventListener('keydown', handleKeyPress)
  }, [gameState])

  useEffect(() => {
    audioRef.current = new Audio(GAME_CONFIG.AUDIO_FILE_PATH)
    audioRef.current.preload = 'auto'
    audioRef.current.addEventListener('error', () => {})
  }, [])

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