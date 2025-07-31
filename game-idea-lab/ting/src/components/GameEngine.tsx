import { useEffect, useRef, useState, useCallback } from 'react'
import { Stage, Layer, Circle, Rect, Group, Text, Shape, Star } from 'react-konva'
import type { GameState, GameResult } from '../types/game'

interface GameEngineProps {
  gameState: GameState
  result: GameResult | null
  onSpacePress?: () => void
}

// 파티클 시스템
class Particle {
  x: number
  y: number
  vx: number
  vy: number
  size: number
  life: number
  maxLife: number
  color: string
  
  constructor(x: number, y: number, options: Partial<Particle> = {}) {
    this.x = x
    this.y = y
    this.vx = options.vx ?? (Math.random() - 0.5) * 2
    this.vy = options.vy ?? (Math.random() - 0.5) * 2
    this.size = options.size ?? Math.random() * 3 + 1
    this.life = 0
    this.maxLife = options.maxLife ?? 60
    this.color = options.color ?? 'rgba(255, 255, 255, 0.5)'
  }
  
  update() {
    this.x += this.vx
    this.y += this.vy
    this.life++
    this.vx *= 0.99
    this.vy *= 0.99
  }
  
  get opacity() {
    return Math.max(0, 1 - this.life / this.maxLife)
  }
  
  get isDead() {
    return this.life >= this.maxLife
  }
}

export const GameEngine = ({ gameState, result, onSpacePress }: GameEngineProps) => {
  const [dimensions, setDimensions] = useState({ width: window.innerWidth, height: window.innerHeight })
  const frameRef = useRef(0)
  const gameStateTimeRef = useRef(0)
  const animationRef = useRef<number>(0)
  const particlesRef = useRef<Particle[]>([])
  const lastStateRef = useRef<GameState>(gameState)
  const spacePressed = useRef(false)
  

  // Window resize
  useEffect(() => {
    const handleResize = () => {
      setDimensions({ width: window.innerWidth, height: window.innerHeight })
    }
    window.addEventListener('resize', handleResize)
    return () => window.removeEventListener('resize', handleResize)
  }, [])

  // State change handler
  useEffect(() => {
    if (gameState !== lastStateRef.current) {
      gameStateTimeRef.current = 0
      lastStateRef.current = gameState
      spacePressed.current = false
      
      // State-specific initializations
      if (gameState === 'cue') {
        // Add explosion particles
        const centerX = dimensions.width / 2
        const centerY = dimensions.height / 2
        for (let i = 0; i < 30; i++) {
          const angle = (i / 30) * Math.PI * 2
          particlesRef.current.push(new Particle(centerX, centerY, {
            vx: Math.cos(angle) * 5,
            vy: Math.sin(angle) * 5,
            size: 5,
            maxLife: 30,
            color: '#fde047'
          }))
        }
      }
    }
  }, [gameState, dimensions])

  // Space key handler
  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      if (e.code === 'Space' && gameState === 'cue' && !spacePressed.current) {
        spacePressed.current = true
        // Add feedback particles
        const centerX = dimensions.width / 2
        const centerY = dimensions.height / 2
        for (let i = 0; i < 20; i++) {
          const angle = Math.random() * Math.PI * 2
          const speed = Math.random() * 10 + 5
          particlesRef.current.push(new Particle(centerX, centerY, {
            vx: Math.cos(angle) * speed,
            vy: Math.sin(angle) * speed,
            size: 8,
            maxLife: 40,
            color: '#4ade80'
          }))
        }
        onSpacePress?.()
      }
    }
    window.addEventListener('keydown', handleKeyPress)
    return () => window.removeEventListener('keydown', handleKeyPress)
  }, [gameState, onSpacePress, dimensions])

  // Main animation loop
  const animate = useCallback(() => {
    frameRef.current++
    gameStateTimeRef.current++
    
    // Update particles
    particlesRef.current = particlesRef.current.filter(p => {
      p.update()
      return !p.isDead
    })
    
    // Add ambient particles for idle state
    if (gameState === 'idle' && frameRef.current % 30 === 0) {
      particlesRef.current.push(new Particle(
        Math.random() * dimensions.width,
        dimensions.height + 50,
        {
          vx: (Math.random() - 0.5) * 0.5,
          vy: -Math.random() * 1 - 0.5,
          size: Math.random() * 2 + 1,
          maxLife: 300,
          color: 'rgba(255, 255, 255, 0.1)'
        }
      ))
    }
    
    animationRef.current = requestAnimationFrame(animate)
  }, [gameState, dimensions])

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
  const time = gameStateTimeRef.current

  return (
    <Stage width={dimensions.width} height={dimensions.height}>
      <Layer>
        {/* Background */}
        <Rect x={0} y={0} width={dimensions.width} height={dimensions.height} fill="#111827" />
        
        {/* Render particles */}
        {particlesRef.current.map((particle, i) => (
          <Circle
            key={i}
            x={particle.x}
            y={particle.y}
            radius={particle.size}
            fill={particle.color}
            opacity={particle.opacity}
          />
        ))}

        {/* Idle State */}
        {gameState === 'idle' && (
          <Group>
            {/* Ambient glow */}
            <Shape
              sceneFunc={(context) => {
                const gradient = context.createRadialGradient(
                  centerX, centerY, 0,
                  centerX, centerY, Math.max(dimensions.width, dimensions.height) * 0.5
                )
                const pulse = Math.sin(time * 0.01) * 0.5 + 0.5
                gradient.addColorStop(0, `rgba(255, 255, 255, ${0.02 + pulse * 0.01})`)
                gradient.addColorStop(1, 'rgba(255, 255, 255, 0)')
                context.fillStyle = gradient
                context.fillRect(0, 0, dimensions.width, dimensions.height)
              }}
            />
            
            {/* Floating light orbs */}
            {[0, 1, 2].map(i => {
              const angle = (time * 0.001 + i * 2.094) % (Math.PI * 2)
              const radius = 200 + Math.sin(time * 0.002 + i) * 50
              const x = centerX + Math.cos(angle) * radius
              const y = centerY + Math.sin(angle) * radius
              const size = 15 + Math.sin(time * 0.003 + i) * 5
              
              return (
                <Circle
                  key={i}
                  x={x}
                  y={y}
                  radius={size}
                  fillRadialGradientStartPoint={{ x: 0, y: 0 }}
                  fillRadialGradientEndPoint={{ x: 0, y: 0 }}
                  fillRadialGradientStartRadius={0}
                  fillRadialGradientEndRadius={size}
                  fillRadialGradientColorStops={[
                    0, 'rgba(255, 255, 255, 0.15)',
                    0.5, 'rgba(255, 255, 255, 0.05)',
                    1, 'rgba(255, 255, 255, 0)'
                  ]}
                />
              )
            })}
          </Group>
        )}

        {/* Hint State */}
        {gameState === 'hint' && (
          <Group>
            {/* Building tension effect */}
            <Shape
              sceneFunc={(context) => {
                // Screen-wide tension pulse
                const intensity = Math.min(time / 60, 1) // Build up over 1 second
                const pulse = Math.sin(time * 0.1) * 0.5 + 0.5
                context.fillStyle = `rgba(255, 255, 255, ${intensity * pulse * 0.05})`
                context.fillRect(0, 0, dimensions.width, dimensions.height)
                
                // Energy gathering at center
                const gradient = context.createRadialGradient(
                  centerX, centerY, 0,
                  centerX, centerY, 300 - intensity * 100
                )
                gradient.addColorStop(0, `rgba(255, 255, 255, ${intensity * 0.1})`)
                gradient.addColorStop(0.5, `rgba(255, 255, 255, ${intensity * 0.05})`)
                gradient.addColorStop(1, 'rgba(255, 255, 255, 0)')
                context.fillStyle = gradient
                context.fillRect(0, 0, dimensions.width, dimensions.height)
              }}
            />
            
            {/* Energy sparks */}
            {time % 10 === 0 && Array.from({ length: 3 }).map(() => {
              const angle = Math.random() * Math.PI * 2
              const distance = 100 + Math.random() * 100
              particlesRef.current.push(new Particle(
                centerX + Math.cos(angle) * distance,
                centerY + Math.sin(angle) * distance,
                {
                  vx: -Math.cos(angle) * 2,
                  vy: -Math.sin(angle) * 2,
                  size: 2,
                  maxLife: 30,
                  color: 'rgba(255, 255, 255, 0.8)'
                }
              ))
              return null
            })}
          </Group>
        )}

        {/* Cue State */}
        {gameState === 'cue' && !spacePressed.current && (
          <Group>
            {/* Flash effect */}
            <Shape
              sceneFunc={(context) => {
                const progress = Math.min(time / 10, 1) // 0 to 1 over ~166ms
                const flash = progress < 0.5 ? progress * 2 : 2 - progress * 2
                context.fillStyle = `rgba(253, 224, 71, ${flash * 0.3})`
                context.fillRect(0, 0, dimensions.width, dimensions.height)
              }}
            />
            
            {/* Dynamic cross */}
            <Group x={centerX} y={centerY}>
              <Shape
                sceneFunc={(context) => {
                  const scale = Math.min(time / 5, 1) // Quick scale up
                  const glow = 1 - Math.min(time / 20, 1) // Fade out glow
                  
                  context.save()
                  context.scale(scale, scale)
                  
                  // Glow effect
                  context.shadowBlur = 50 + glow * 50
                  context.shadowColor = '#fde047'
                  
                  // Vertical beam
                  const vGradient = context.createLinearGradient(0, -dimensions.height, 0, dimensions.height)
                  vGradient.addColorStop(0, 'rgba(253, 224, 71, 0)')
                  vGradient.addColorStop(0.4, 'rgba(253, 224, 71, 0.8)')
                  vGradient.addColorStop(0.5, 'rgba(253, 224, 71, 1)')
                  vGradient.addColorStop(0.6, 'rgba(253, 224, 71, 0.8)')
                  vGradient.addColorStop(1, 'rgba(253, 224, 71, 0)')
                  
                  context.fillStyle = vGradient
                  context.beginPath()
                  context.moveTo(0, -dimensions.height)
                  context.lineTo(30, -50)
                  context.lineTo(15, 0)
                  context.lineTo(30, 50)
                  context.lineTo(0, dimensions.height)
                  context.lineTo(-30, 50)
                  context.lineTo(-15, 0)
                  context.lineTo(-30, -50)
                  context.closePath()
                  context.fill()
                  
                  // Horizontal beam
                  const hGradient = context.createLinearGradient(-dimensions.width, 0, dimensions.width, 0)
                  hGradient.addColorStop(0, 'rgba(253, 224, 71, 0)')
                  hGradient.addColorStop(0.4, 'rgba(253, 224, 71, 0.8)')
                  hGradient.addColorStop(0.5, 'rgba(253, 224, 71, 1)')
                  hGradient.addColorStop(0.6, 'rgba(253, 224, 71, 0.8)')
                  hGradient.addColorStop(1, 'rgba(253, 224, 71, 0)')
                  
                  context.fillStyle = hGradient
                  context.beginPath()
                  context.moveTo(-dimensions.width, 0)
                  context.lineTo(-50, -30)
                  context.lineTo(0, -15)
                  context.lineTo(50, -30)
                  context.lineTo(dimensions.width, 0)
                  context.lineTo(50, 30)
                  context.lineTo(0, 15)
                  context.lineTo(-50, 30)
                  context.closePath()
                  context.fill()
                  
                  context.restore()
                }}
              />
              
              {/* Center burst */}
              <Star
                numPoints={8}
                innerRadius={10}
                outerRadius={30 + Math.sin(time * 0.3) * 10}
                fill="#fde047"
                opacity={0.8}
                rotation={time * 2}
              />
            </Group>
          </Group>
        )}

        {/* Result State */}
        {gameState === 'result' && result && (
          <Group>
            {/* Success/Failure background effect */}
            <Shape
              sceneFunc={(context) => {
                const gradient = context.createRadialGradient(
                  centerX, centerY, 0,
                  centerX, centerY, Math.max(dimensions.width, dimensions.height) * 0.7
                )
                const pulse = Math.sin(time * 0.05) * 0.5 + 0.5
                const color = result.success ? '74, 222, 128' : '248, 113, 113'
                gradient.addColorStop(0, `rgba(${color}, ${pulse * 0.1})`)
                gradient.addColorStop(1, `rgba(${color}, 0)`)
                context.fillStyle = gradient
                context.fillRect(0, 0, dimensions.width, dimensions.height)
              }}
            />
            
            <Text
              x={centerX}
              y={centerY - 40}
              text={result.success ? '성공!' : '실패...'}
              fontSize={56}
              fontStyle="bold"
              fill={result.success ? '#4ade80' : '#f87171'}
              align="center"
              width={300}
              offsetX={150}
              shadowColor={result.success ? '#4ade80' : '#f87171'}
              shadowBlur={20}
              shadowOpacity={0.5}
            />
            
            {result.success && (
              <Text
                x={centerX}
                y={centerY + 40}
                text={`${result.reactionTime}ms`}
                fontSize={36}
                fill="#4ade80"
                align="center"
                width={200}
                offsetX={100}
                opacity={Math.min(time / 30, 1)}
              />
            )}
          </Group>
        )}
      </Layer>
    </Stage>
  )
}