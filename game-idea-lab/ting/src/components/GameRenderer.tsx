import { useEffect, useState, memo, useMemo } from 'react'
import { Stage, Layer, Circle, Rect, Group, Text, Shape } from 'react-konva'
import { useGame } from '../contexts/GameContext'
import { GAME_CONFIG } from '../constants/game'

// 메모이제이션된 배경 컴포넌트
const Background = memo(() => (
  <Rect x={0} y={0} width={window.innerWidth} height={window.innerHeight} fill="#111827" />
))
Background.displayName = 'Background'

// Idle 상태 효과
const IdleEffect = memo(({ elapsedTime }: { elapsedTime: number }) => {
  const centerX = window.innerWidth / 2
  const centerY = window.innerHeight / 2
  const time = elapsedTime * 0.001 // Convert to seconds
  
  return (
    <Group>
      {/* Central ambient glow */}
      <Circle
        x={centerX}
        y={centerY}
        radius={200 + Math.sin(time) * 20}
        fillRadialGradientStartPoint={{ x: 0, y: 0 }}
        fillRadialGradientEndPoint={{ x: 0, y: 0 }}
        fillRadialGradientStartRadius={0}
        fillRadialGradientEndRadius={200}
        fillRadialGradientColorStops={[
          0, 'rgba(255, 255, 255, 0.05)',
          1, 'rgba(255, 255, 255, 0)'
        ]}
      />
      
      {/* Floating orbs */}
      {[0, 120, 240].map(offset => {
        const angle = time + (offset * Math.PI / 180)
        const x = centerX + Math.cos(angle) * 150
        const y = centerY + Math.sin(angle) * 150
        
        return (
          <Circle
            key={offset}
            x={x}
            y={y}
            radius={10}
            fill="rgba(255, 255, 255, 0.1)"
          />
        )
      })}
    </Group>
  )
})
IdleEffect.displayName = 'IdleEffect'

// Hint 상태 효과
const HintEffect = memo(({ elapsedTime }: { elapsedTime: number }) => {
  const progress = Math.min(elapsedTime / GAME_CONFIG.HINT_DURATION, 1)
  const centerX = window.innerWidth / 2
  const centerY = window.innerHeight / 2
  
  return (
    <Group>
      {/* Building tension */}
      <Shape
        sceneFunc={(context) => {
          const gradient = context.createRadialGradient(
            centerX, centerY, 300 * (1 - progress),
            centerX, centerY, 400
          )
          gradient.addColorStop(0, `rgba(255, 255, 255, ${progress * 0.1})`)
          gradient.addColorStop(1, 'rgba(255, 255, 255, 0)')
          
          context.fillStyle = gradient
          context.fillRect(0, 0, window.innerWidth, window.innerHeight)
        }}
      />
      
      {/* Energy rings */}
      <Circle
        x={centerX}
        y={centerY}
        radius={100 * (1 - progress * 0.5)}
        stroke="white"
        strokeWidth={2}
        opacity={progress * 0.3}
        fill="transparent"
      />
    </Group>
  )
})
HintEffect.displayName = 'HintEffect'

// Cue 상태 효과
const CueEffect = memo(({ elapsedTime, spacePressed }: { elapsedTime: number; spacePressed: boolean }) => {
  const progress = Math.min(elapsedTime / 100, 1) // Quick animation
  const centerX = window.innerWidth / 2
  const centerY = window.innerHeight / 2
  
  if (spacePressed) return null // Hide when space is pressed
  
  return (
    <Group>
      {/* Flash overlay */}
      <Rect
        x={0}
        y={0}
        width={window.innerWidth}
        height={window.innerHeight}
        fill="#fde047"
        opacity={progress < 0.3 ? progress : 0.3 * (1 - (progress - 0.3) / 0.7)}
      />
      
      {/* Cross */}
      <Group x={centerX} y={centerY} scaleX={progress} scaleY={progress}>
        {/* Vertical beam */}
        <Rect
          x={-30}
          y={-window.innerHeight / 2}
          width={60}
          height={window.innerHeight}
          fillLinearGradientStartPoint={{ x: 0, y: 0 }}
          fillLinearGradientEndPoint={{ x: 0, y: window.innerHeight }}
          fillLinearGradientColorStops={[
            0, 'rgba(253, 224, 71, 0)',
            0.4, 'rgba(253, 224, 71, 0.8)',
            0.5, 'rgba(253, 224, 71, 1)',
            0.6, 'rgba(253, 224, 71, 0.8)',
            1, 'rgba(253, 224, 71, 0)'
          ]}
        />
        
        {/* Horizontal beam */}
        <Rect
          x={-window.innerWidth / 2}
          y={-30}
          width={window.innerWidth}
          height={60}
          fillLinearGradientStartPoint={{ x: 0, y: 0 }}
          fillLinearGradientEndPoint={{ x: window.innerWidth, y: 0 }}
          fillLinearGradientColorStops={[
            0, 'rgba(253, 224, 71, 0)',
            0.4, 'rgba(253, 224, 71, 0.8)',
            0.5, 'rgba(253, 224, 71, 1)',
            0.6, 'rgba(253, 224, 71, 0.8)',
            1, 'rgba(253, 224, 71, 0)'
          ]}
        />
        
        {/* Center flare */}
        <Circle
          x={0}
          y={0}
          radius={20}
          fill="#fde047"
          shadowBlur={30}
          shadowColor="#fde047"
        />
      </Group>
    </Group>
  )
})
CueEffect.displayName = 'CueEffect'

// Result 상태 효과
const ResultEffect = memo(({ elapsedTime, success, reactionTime }: { 
  elapsedTime: number
  success: boolean
  reactionTime?: number 
}) => {
  const centerX = window.innerWidth / 2
  const centerY = window.innerHeight / 2
  const progress = Math.min(elapsedTime / 500, 1)
  
  return (
    <Group>
      {success ? (
        <>
          {/* Success - Expanding rings */}
          {[0, 100, 200].map((delay, i) => {
            const ringProgress = Math.max(0, (elapsedTime - delay) / 1000)
            if (ringProgress === 0) return null
            
            return (
              <Circle
                key={i}
                x={centerX}
                y={centerY}
                radius={ringProgress * 400}
                stroke="#4ade80"
                strokeWidth={3}
                opacity={Math.max(0, 1 - ringProgress)}
              />
            )
          })}
          
          {/* Success particles */}
          {Array.from({ length: 12 }).map((_, i) => {
            const angle = (i / 12) * Math.PI * 2
            const distance = progress * 200
            const x = centerX + Math.cos(angle) * distance
            const y = centerY + Math.sin(angle) * distance
            
            return (
              <Circle
                key={i}
                x={x}
                y={y}
                radius={8}
                fill="#4ade80"
                opacity={1 - progress}
              />
            )
          })}
          
          {/* Reaction time display */}
          {reactionTime && (
            <Text
              x={centerX}
              y={centerY}
              text={`${reactionTime}ms`}
              fontSize={48}
              fontStyle="bold"
              fill="#4ade80"
              align="center"
              width={200}
              offsetX={100}
              opacity={progress}
              shadowColor="#4ade80"
              shadowBlur={20}
            />
          )}
        </>
      ) : (
        <>
          {/* Failure - Screen flash */}
          <Rect
            x={0}
            y={0}
            width={window.innerWidth}
            height={window.innerHeight}
            fill="#dc2626"
            opacity={progress < 0.2 ? progress * 2 : 0.4 * (1 - (progress - 0.2) / 0.8)}
          />
          
          {/* Failure - Collapsing effect */}
          <Circle
            x={centerX}
            y={centerY}
            radius={200 * (1 - progress * 0.8)}
            stroke="#f87171"
            strokeWidth={5}
            opacity={progress}
            fill="transparent"
          />
          
          {/* Dark vignette */}
          <Shape
            sceneFunc={(context) => {
              const gradient = context.createRadialGradient(
                centerX, centerY, 0,
                centerX, centerY, Math.max(window.innerWidth, window.innerHeight)
              )
              gradient.addColorStop(0, 'rgba(0, 0, 0, 0)')
              gradient.addColorStop(0.7, `rgba(0, 0, 0, ${progress * 0.3})`)
              gradient.addColorStop(1, `rgba(0, 0, 0, ${progress * 0.5})`)
              
              context.fillStyle = gradient
              context.fillRect(0, 0, window.innerWidth, window.innerHeight)
            }}
          />
        </>
      )}
    </Group>
  )
})
ResultEffect.displayName = 'ResultEffect'

export const GameRenderer = () => {
  const { gameState, result, stateElapsedTime, handleSpacePress } = useGame()
  const [dimensions, setDimensions] = useState({ 
    width: window.innerWidth, 
    height: window.innerHeight 
  })
  const [spacePressed, setSpacePressed] = useState(false)

  // Window resize handler
  useEffect(() => {
    const handleResize = () => {
      setDimensions({ width: window.innerWidth, height: window.innerHeight })
    }
    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [])

  // Reset space pressed on state change
  useEffect(() => {
    setSpacePressed(false)
  }, [gameState])

  // Keyboard handler
  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.code === 'Space' && gameState === 'cue' && !spacePressed) {
        e.preventDefault()
        setSpacePressed(true)
        handleSpacePress()
      }
    }
    
    window.addEventListener('keydown', handleKeyPress)
    return () => window.removeEventListener('keydown', handleKeyPress)
  }, [gameState, spacePressed, handleSpacePress])

  // Memoized stage content
  const stageContent = useMemo(() => (
    <Layer>
      <Background />
      
      {gameState === 'idle' && <IdleEffect elapsedTime={stateElapsedTime} />}
      {gameState === 'hint' && <HintEffect elapsedTime={stateElapsedTime} />}
      {gameState === 'cue' && <CueEffect elapsedTime={stateElapsedTime} spacePressed={spacePressed} />}
      {gameState === 'result' && result && (
        <ResultEffect 
          elapsedTime={stateElapsedTime} 
          success={result.success} 
          reactionTime={result.reactionTime}
        />
      )}
    </Layer>
  ), [gameState, stateElapsedTime, spacePressed, result])

  return (
    <Stage width={dimensions.width} height={dimensions.height}>
      {stageContent}
    </Stage>
  )
}