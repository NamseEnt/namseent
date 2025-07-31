import { useEffect, useRef, useState, useCallback } from 'react'
import { Stage, Layer, Circle, Rect, Group, Text, Shape } from 'react-konva'
import type { GameState, GameResult } from '../types/game'

interface KonvaGameProps {
  gameState: GameState
  result: GameResult | null
}

export const KonvaGame = ({ gameState, result }: KonvaGameProps) => {
  const [dimensions, setDimensions] = useState({ width: window.innerWidth, height: window.innerHeight })
  const frameRef = useRef(0)
  const animationRef = useRef<number>(0)
  const prevGameStateRef = useRef<GameState>(gameState)

  // Window resize handler
  useEffect(() => {
    const handleResize = () => {
      setDimensions({ width: window.innerWidth, height: window.innerHeight })
    }
    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [])

  // Reset frame count when game state changes
  useEffect(() => {
    if (prevGameStateRef.current !== gameState) {
      frameRef.current = 0
      prevGameStateRef.current = gameState
    }
  }, [gameState])

  // Animation frame update
  const animate = useCallback(() => {
    frameRef.current += 1
    animationRef.current = requestAnimationFrame(animate)
  }, [])

  useEffect(() => {
    animationRef.current = requestAnimationFrame(animate)
    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current)
      }
    }
  }, [animate])

  const centerX = dimensions.width / 2
  const centerY = dimensions.height / 2

  return (
    <Stage width={dimensions.width} height={dimensions.height}>
      <Layer>
        {/* Background */}
        <Rect
          x={0}
          y={0}
          width={dimensions.width}
          height={dimensions.height}
          fill="#111827"
        />

        {/* Idle State */}
        {gameState === 'idle' && <IdleEffect width={dimensions.width} height={dimensions.height} />}

        {/* Hint State */}
        {gameState === 'hint' && <HintEffect width={dimensions.width} height={dimensions.height} />}

        {/* Cue State */}
        {gameState === 'cue' && <CueEffect width={dimensions.width} height={dimensions.height} />}

        {/* Result State */}
        {gameState === 'result' && result && (
          <Group>
            <Text
              x={centerX}
              y={centerY - 30}
              text={result.success ? '성공!' : '실패...'}
              fontSize={48}
              fontStyle="bold"
              fill={result.success ? '#4ade80' : '#f87171'}
              align="center"
              width={200}
              offsetX={100}
            />
            {result.success && (
              <Text
                x={centerX}
                y={centerY + 30}
                text={`${result.reactionTime}ms`}
                fontSize={36}
                fill="#4ade80"
                align="center"
                width={200}
                offsetX={100}
              />
            )}
          </Group>
        )}
      </Layer>
    </Stage>
  )
}

// Idle 효과 컴포넌트
const IdleEffect = ({ width, height }: { width: number; height: number }) => {
  const [time, setTime] = useState(0)
  
  useEffect(() => {
    const interval = setInterval(() => {
      setTime(t => t + 0.01)
    }, 16)
    return () => clearInterval(interval)
  }, [])

  return (
    <Group>
      {/* Floating orbs */}
      {[0, 1, 2, 3, 4].map(i => {
        const baseX = width * (0.2 + i * 0.15)
        const x = baseX + Math.sin(time + i) * 50
        const y = ((time * 30 + i * 100) % (height + 200)) - 100
        const scale = 0.8 + Math.sin(time * 2 + i) * 0.2
        
        return (
          <Circle
            key={i}
            x={x}
            y={y}
            radius={30 * scale}
            fillRadialGradientStartPoint={{ x: 0, y: 0 }}
            fillRadialGradientEndPoint={{ x: 0, y: 0 }}
            fillRadialGradientStartRadius={0}
            fillRadialGradientEndRadius={30 * scale}
            fillRadialGradientColorStops={[0, 'rgba(255,255,255,0.1)', 1, 'rgba(255,255,255,0)']}
          />
        )
      })}
    </Group>
  )
}

// Hint 효과 컴포넌트
const HintEffect = ({ width, height }: { width: number; height: number }) => {
  const [time, setTime] = useState(0)
  
  useEffect(() => {
    const interval = setInterval(() => {
      setTime(t => t + 0.02)
    }, 16)
    return () => clearInterval(interval)
  }, [])

  const centerX = width / 2
  const centerY = height / 2
  const pulse = Math.sin(time * 3) * 0.5 + 0.5

  return (
    <Group>
      {/* Screen pulse */}
      <Rect
        x={0}
        y={0}
        width={width}
        height={height}
        fill="white"
        opacity={pulse * 0.05}
      />
      
      {/* Energy waves */}
      {[0, 1, 2].map(i => {
        const progress = ((time * 60 + i * 20) % 100) / 100
        const radius = progress * Math.min(width, height) * 0.8
        const opacity = (1 - progress) * 0.3
        
        return (
          <Circle
            key={i}
            x={centerX}
            y={centerY}
            radius={radius}
            stroke="white"
            strokeWidth={2}
            opacity={opacity}
            fill="transparent"
          />
        )
      })}
      
      {/* Energy field */}
      <Shape
        sceneFunc={(context) => {
          const gradient = context.createRadialGradient(centerX, centerY, 0, centerX, centerY, 300)
          gradient.addColorStop(0, `rgba(255, 255, 255, ${pulse * 0.1})`)
          gradient.addColorStop(1, 'rgba(255, 255, 255, 0)')
          context.fillStyle = gradient
          context.fillRect(0, 0, width, height)
        }}
      />
    </Group>
  )
}

// Cue 효과 컴포넌트
const CueEffect = ({ width, height }: { width: number; height: number }) => {
  const [time, setTime] = useState(0)
  
  useEffect(() => {
    const interval = setInterval(() => {
      setTime(t => Math.min(t + 0.05, 1))
    }, 16)
    return () => clearInterval(interval)
  }, [])

  const centerX = width / 2
  const centerY = height / 2
  const scale = time < 0.3 ? time / 0.3 : 1
  const flashOpacity = time < 0.2 ? time / 0.2 * 0.5 : time < 0.5 ? 0.5 * (1 - (time - 0.2) / 0.3) : 0

  return (
    <Group>
      {/* Flash overlay */}
      <Rect
        x={0}
        y={0}
        width={width}
        height={height}
        fill="#fde047"
        opacity={flashOpacity}
      />
      
      {/* Cross beams */}
      <Group
        x={centerX}
        y={centerY}
        scaleX={scale}
        scaleY={scale}
      >
        {/* Vertical beam */}
        <Shape
          sceneFunc={(context) => {
            const gradient = context.createLinearGradient(0, -height/2, 0, height/2)
            gradient.addColorStop(0, 'rgba(253, 224, 71, 0)')
            gradient.addColorStop(0.3, 'rgba(251, 191, 36, 0.6)')
            gradient.addColorStop(0.5, 'rgba(253, 224, 71, 1)')
            gradient.addColorStop(0.7, 'rgba(251, 191, 36, 0.6)')
            gradient.addColorStop(1, 'rgba(253, 224, 71, 0)')
            
            context.fillStyle = gradient
            context.beginPath()
            context.moveTo(0, -height/2)
            context.lineTo(40, -60)
            context.lineTo(20, -20)
            context.lineTo(20, 20)
            context.lineTo(40, 60)
            context.lineTo(0, height/2)
            context.lineTo(-40, 60)
            context.lineTo(-20, 20)
            context.lineTo(-20, -20)
            context.lineTo(-40, -60)
            context.closePath()
            context.fill()
          }}
          shadowBlur={50}
          shadowColor="#fde047"
        />
        
        {/* Horizontal beam */}
        <Shape
          sceneFunc={(context) => {
            const gradient = context.createLinearGradient(-width/2, 0, width/2, 0)
            gradient.addColorStop(0, 'rgba(253, 224, 71, 0)')
            gradient.addColorStop(0.3, 'rgba(251, 191, 36, 0.6)')
            gradient.addColorStop(0.5, 'rgba(253, 224, 71, 1)')
            gradient.addColorStop(0.7, 'rgba(251, 191, 36, 0.6)')
            gradient.addColorStop(1, 'rgba(253, 224, 71, 0)')
            
            context.fillStyle = gradient
            context.beginPath()
            context.moveTo(-width/2, 0)
            context.lineTo(-60, -40)
            context.lineTo(-20, -20)
            context.lineTo(20, -20)
            context.lineTo(60, -40)
            context.lineTo(width/2, 0)
            context.lineTo(60, 40)
            context.lineTo(20, 20)
            context.lineTo(-20, 20)
            context.lineTo(-60, 40)
            context.closePath()
            context.fill()
          }}
          shadowBlur={50}
          shadowColor="#fde047"
        />
        
        {/* Center flare */}
        <Circle
          x={0}
          y={0}
          radius={30 * scale}
          fill="#fde047"
          shadowBlur={50}
          shadowColor="#fde047"
        />
      </Group>
    </Group>
  )
}