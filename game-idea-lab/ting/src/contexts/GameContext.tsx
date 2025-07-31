import { createContext, useContext, useState, useRef, useCallback, useEffect } from 'react'
import type { GameState, GameResult } from '../types/game'
import { GAME_CONFIG } from '../constants/game'

interface GameContextType {
  gameState: GameState
  result: GameResult | null
  hasGameStarted: boolean
  stateStartTime: number
  stateElapsedTime: number
  startGame: () => void
  handleSpacePress: () => void
}

const GameContext = createContext<GameContextType | null>(null)

let audio: HTMLAudioElement | null = null
const getAudio = () => {
  if (!audio) {
    audio = new Audio(GAME_CONFIG.AUDIO_FILE_PATH)
    audio.preload = 'auto'
  }
  return audio
}

export const GameProvider = ({ children }: { children: React.ReactNode }) => {
  const [gameState, setGameState] = useState<GameState>('idle')
  const [result, setResult] = useState<GameResult | null>(null)
  const [hasGameStarted, setHasGameStarted] = useState(false)
  const [stateStartTime, setStateStartTime] = useState(0)
  const [stateElapsedTime, setStateElapsedTime] = useState(0)
  
  const animationFrameRef = useRef<number>(0)
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null)
  const isRunningRef = useRef(false)
  const cueStartTimeRef = useRef(0)

  // Update elapsed time
  useEffect(() => {
    const updateTime = () => {
      if (isRunningRef.current && stateStartTime > 0) {
        setStateElapsedTime(Date.now() - stateStartTime)
      }
      animationFrameRef.current = requestAnimationFrame(updateTime)
    }
    
    animationFrameRef.current = requestAnimationFrame(updateTime)
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current)
      }
    }
  }, [stateStartTime])

  // State transition handler
  const transitionToState = useCallback((newState: GameState, newResult?: GameResult | null) => {
    setGameState(newState)
    setStateStartTime(Date.now())
    setStateElapsedTime(0)
    
    if (newResult !== undefined) {
      setResult(newResult)
    }
    
    // Clear any existing timeout
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current)
    }
  }, [])

  // Start game
  const startGame = useCallback(() => {
    setHasGameStarted(true)
    isRunningRef.current = true
    transitionToState('idle')
    
    // Schedule first round
    const scheduleRound = () => {
      const delay = Math.random() * (GAME_CONFIG.MAX_IDLE_TIME - GAME_CONFIG.MIN_IDLE_TIME) + GAME_CONFIG.MIN_IDLE_TIME
      timeoutRef.current = setTimeout(() => {
        if (!isRunningRef.current) return
        transitionToState('hint')
        
        // Hint -> Cue
        timeoutRef.current = setTimeout(() => {
          if (!isRunningRef.current) return
          cueStartTimeRef.current = Date.now()
          transitionToState('cue')
          getAudio().play().catch(() => {})
          
          // Auto fail after time limit
          timeoutRef.current = setTimeout(() => {
            if (!isRunningRef.current) return
            transitionToState('result', { success: false })
            
            // Back to idle
            timeoutRef.current = setTimeout(() => {
              if (!isRunningRef.current) return
              transitionToState('idle')
              scheduleRound()
            }, GAME_CONFIG.RESULT_DISPLAY_TIME)
          }, GAME_CONFIG.REACTION_TIME_LIMIT)
        }, GAME_CONFIG.HINT_DURATION)
      }, delay)
    }
    
    scheduleRound()
  }, [transitionToState])

  // Handle space press
  const handleSpacePress = useCallback(() => {
    if (gameState === 'cue' && isRunningRef.current) {
      const reactionTime = Date.now() - cueStartTimeRef.current
      if (timeoutRef.current) clearTimeout(timeoutRef.current)
      
      transitionToState('result', { success: true, reactionTime })
      
      // Back to idle and continue
      timeoutRef.current = setTimeout(() => {
        if (!isRunningRef.current) return
        transitionToState('idle')
        
        // Schedule next round
        const delay = Math.random() * (GAME_CONFIG.MAX_IDLE_TIME - GAME_CONFIG.MIN_IDLE_TIME) + GAME_CONFIG.MIN_IDLE_TIME
        timeoutRef.current = setTimeout(() => {
          if (!isRunningRef.current) return
          transitionToState('hint')
          
          timeoutRef.current = setTimeout(() => {
            if (!isRunningRef.current) return
            cueStartTimeRef.current = Date.now()
            transitionToState('cue')
            getAudio().play().catch(() => {})
            
            timeoutRef.current = setTimeout(() => {
              if (!isRunningRef.current) return
              transitionToState('result', { success: false })
              
              timeoutRef.current = setTimeout(() => {
                if (!isRunningRef.current) return
                transitionToState('idle')
                startGame() // Restart the cycle
              }, GAME_CONFIG.RESULT_DISPLAY_TIME)
            }, GAME_CONFIG.REACTION_TIME_LIMIT)
          }, GAME_CONFIG.HINT_DURATION)
        }, delay)
      }, GAME_CONFIG.RESULT_DISPLAY_TIME)
    }
  }, [gameState, transitionToState, startGame])

  // Cleanup
  useEffect(() => {
    return () => {
      isRunningRef.current = false
      if (timeoutRef.current) clearTimeout(timeoutRef.current)
      if (animationFrameRef.current) cancelAnimationFrame(animationFrameRef.current)
    }
  }, [])

  return (
    <GameContext.Provider value={{
      gameState,
      result,
      hasGameStarted,
      stateStartTime,
      stateElapsedTime,
      startGame,
      handleSpacePress
    }}>
      {children}
    </GameContext.Provider>
  )
}

export const useGame = () => {
  const context = useContext(GameContext)
  if (!context) throw new Error('useGame must be used within GameProvider')
  return context
}