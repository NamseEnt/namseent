import { useEffect, useRef, useState } from 'react'
import { Stage, Layer, Circle, Line, Rect, Group, Text } from 'react-konva'
import type { GameState, GameResult } from '../types/game'

interface KonvaGameProps {
  gameState: GameState
  result: GameResult | null
}

interface Particle {
  x: number
  y: number
  radius: number
  opacity: number
  velocity: { x: number; y: number }
}

export const KonvaGame = ({ gameState, result }: KonvaGameProps) => {
  const [dimensions, setDimensions] = useState({ width: window.innerWidth, height: window.innerHeight })
  const [particles, setParticles] = useState<Particle[]>([])
  const [time, setTime] = useState(0)
  const animationRef = useRef<number>(0)

  // Window resize handler
  useEffect(() => {
    const handleResize = () => {
      setDimensions({ width: window.innerWidth, height: window.innerHeight })
    }
    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [])

  // Animation loop
  useEffect(() => {
    const animate = () => {
      setTime(t => t + 1)
      animationRef.current = requestAnimationFrame(animate)
    }
    animationRef.current = requestAnimationFrame(animate)
    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current)
      }
    }
  }, [])

  // Generate particles for idle state
  useEffect(() => {
    if (gameState === 'idle') {
      const newParticles: Particle[] = []
      for (let i = 0; i < 5; i++) {
        newParticles.push({
          x: Math.random() * dimensions.width,
          y: Math.random() * dimensions.height,
          radius: Math.random() * 50 + 20,
          opacity: Math.random() * 0.1 + 0.05,
          velocity: {
            x: (Math.random() - 0.5) * 0.5,
            y: (Math.random() - 0.5) * 0.5
          }
        })
      }
      setParticles(newParticles)
    }
  }, [gameState, dimensions])

  // Update particles
  useEffect(() => {
    if (gameState === 'idle' && particles.length > 0) {
      const interval = setInterval(() => {
        setParticles(prev => prev.map(p => ({
          ...p,
          x: (p.x + p.velocity.x + dimensions.width) % dimensions.width,
          y: (p.y + p.velocity.y + dimensions.height) % dimensions.height,
          opacity: p.opacity + Math.sin(time * 0.01) * 0.01
        })))
      }, 16)
      return () => clearInterval(interval)
    }
  }, [gameState, particles.length, time, dimensions])

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
        {gameState === 'idle' && (
          <Group>
            {/* Floating particles */}
            {particles.map((particle, i) => (
              <Circle
                key={i}
                x={particle.x}
                y={particle.y}
                radius={particle.radius}
                fill="white"
                opacity={particle.opacity}
              />
            ))}
            
            {/* Subtle gradient overlay */}
            <Circle
              x={centerX}
              y={centerY}
              radius={Math.max(dimensions.width, dimensions.height)}
              fillRadialGradientStartPoint={{ x: 0, y: 0 }}
              fillRadialGradientEndPoint={{ x: 0, y: 0 }}
              fillRadialGradientStartRadius={0}
              fillRadialGradientEndRadius={Math.max(dimensions.width, dimensions.height) / 2}
              fillRadialGradientColorStops={[0, 'rgba(255,255,255,0.02)', 1, 'rgba(255,255,255,0)']}
            />
          </Group>
        )}

        {/* Hint State */}
        {gameState === 'hint' && (
          <Group>
            {/* Pulsing overlay */}
            <Rect
              x={0}
              y={0}
              width={dimensions.width}
              height={dimensions.height}
              fill="white"
              opacity={Math.sin(time * 0.05) * 0.02 + 0.02}
            />
            
            {/* Energy waves */}
            {[0, 1, 2].map(i => {
              const radius = ((time + i * 40) % 180) * 5
              const opacity = Math.max(0, 1 - radius / 900) * 0.3
              return (
                <Circle
                  key={i}
                  x={centerX}
                  y={centerY}
                  radius={radius}
                  stroke="white"
                  strokeWidth={2}
                  opacity={opacity}
                />
              )
            })}
            
            {/* Radial lines */}
            {Array.from({ length: 8 }).map((_, i) => {
              const angle = (i / 8) * Math.PI * 2 + time * 0.001
              const pulse = Math.sin(time * 0.05) * 0.5 + 0.5
              const innerRadius = 300
              const outerRadius = 500 + pulse * 50
              
              return (
                <Line
                  key={i}
                  points={[
                    centerX + Math.cos(angle) * innerRadius,
                    centerY + Math.sin(angle) * innerRadius,
                    centerX + Math.cos(angle) * outerRadius,
                    centerY + Math.sin(angle) * outerRadius
                  ]}
                  stroke="white"
                  strokeWidth={1}
                  opacity={pulse * 0.1}
                  dash={[10, 20]}
                />
              )
            })}
          </Group>
        )}

        {/* Cue State */}
        {gameState === 'cue' && (
          <Group>
            {/* Flash overlay */}
            <Rect
              x={0}
              y={0}
              width={dimensions.width}
              height={dimensions.height}
              fill="#fde047"
              opacity={time < 10 ? time / 30 : Math.max(0, 1 - (time - 10) / 20)}
            />
            
            {/* Cross light beams */}
            <Group>
              {/* Vertical beam */}
              <Line
                points={[centerX, 0, centerX - 30, centerY - 50, centerX, centerY, centerX + 30, centerY - 50, centerX, 0]}
                closed
                fillLinearGradientStartPoint={{ x: centerX, y: 0 }}
                fillLinearGradientEndPoint={{ x: centerX, y: dimensions.height }}
                fillLinearGradientColorStops={[
                  0, 'rgba(253,224,71,0)',
                  0.4, 'rgba(251,191,36,0.8)',
                  0.5, 'rgba(253,224,71,1)',
                  0.6, 'rgba(251,191,36,0.8)',
                  1, 'rgba(253,224,71,0)'
                ]}
                shadowBlur={50}
                shadowColor="#fde047"
                scaleY={time < 10 ? time / 10 : 1}
              />
              
              <Line
                points={[centerX, dimensions.height, centerX + 30, centerY + 50, centerX, centerY, centerX - 30, centerY + 50, centerX, dimensions.height]}
                closed
                fillLinearGradientStartPoint={{ x: centerX, y: 0 }}
                fillLinearGradientEndPoint={{ x: centerX, y: dimensions.height }}
                fillLinearGradientColorStops={[
                  0, 'rgba(253,224,71,0)',
                  0.4, 'rgba(251,191,36,0.8)',
                  0.5, 'rgba(253,224,71,1)',
                  0.6, 'rgba(251,191,36,0.8)',
                  1, 'rgba(253,224,71,0)'
                ]}
                shadowBlur={50}
                shadowColor="#fde047"
                scaleY={time < 10 ? time / 10 : 1}
              />
              
              {/* Horizontal beam */}
              <Line
                points={[0, centerY, centerX - 50, centerY - 30, centerX, centerY, centerX - 50, centerY + 30, 0, centerY]}
                closed
                fillLinearGradientStartPoint={{ x: 0, y: centerY }}
                fillLinearGradientEndPoint={{ x: dimensions.width, y: centerY }}
                fillLinearGradientColorStops={[
                  0, 'rgba(253,224,71,0)',
                  0.4, 'rgba(251,191,36,0.8)',
                  0.5, 'rgba(253,224,71,1)',
                  0.6, 'rgba(251,191,36,0.8)',
                  1, 'rgba(253,224,71,0)'
                ]}
                shadowBlur={50}
                shadowColor="#fde047"
                scaleX={time < 10 ? time / 10 : 1}
              />
              
              <Line
                points={[dimensions.width, centerY, centerX + 50, centerY + 30, centerX, centerY, centerX + 50, centerY - 30, dimensions.width, centerY]}
                closed
                fillLinearGradientStartPoint={{ x: 0, y: centerY }}
                fillLinearGradientEndPoint={{ x: dimensions.width, y: centerY }}
                fillLinearGradientColorStops={[
                  0, 'rgba(253,224,71,0)',
                  0.4, 'rgba(251,191,36,0.8)',
                  0.5, 'rgba(253,224,71,1)',
                  0.6, 'rgba(251,191,36,0.8)',
                  1, 'rgba(253,224,71,0)'
                ]}
                shadowBlur={50}
                shadowColor="#fde047"
                scaleX={time < 10 ? time / 10 : 1}
              />
            </Group>
            
            {/* Center flare */}
            <Circle
              x={centerX}
              y={centerY}
              radius={time < 10 ? time * 2 : 20}
              fill="#fde047"
              shadowBlur={30}
              shadowColor="#fde047"
            />
          </Group>
        )}

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
              offsetX={result.success ? 40 : 60}
            />
            {result.success && (
              <Text
                x={centerX}
                y={centerY + 30}
                text={`${result.reactionTime}ms`}
                fontSize={36}
                fill="#4ade80"
                align="center"
                offsetX={50}
              />
            )}
          </Group>
        )}
      </Layer>
    </Stage>
  )
}