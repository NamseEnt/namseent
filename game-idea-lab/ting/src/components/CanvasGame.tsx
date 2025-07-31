import { useEffect, useRef } from 'react'
import type { GameState, GameResult } from '../types/game'

interface CanvasGameProps {
  gameState: GameState
  result: GameResult | null
}

// 파티클 클래스
class Particle {
  x: number
  y: number
  vx: number
  vy: number
  life: number
  maxLife: number
  size: number
  color: string

  constructor(x: number, y: number, color: string = 'rgba(255, 255, 255, 0.5)') {
    this.x = x
    this.y = y
    this.vx = (Math.random() - 0.5) * 2
    this.vy = (Math.random() - 0.5) * 2
    this.life = 0
    this.maxLife = 60 + Math.random() * 60
    this.size = Math.random() * 3 + 1
    this.color = color
  }

  update() {
    this.x += this.vx
    this.y += this.vy
    this.life++
    this.vx *= 0.99
    this.vy *= 0.99
  }

  draw(ctx: CanvasRenderingContext2D) {
    const alpha = 1 - (this.life / this.maxLife)
    ctx.save()
    ctx.globalAlpha = alpha * 0.5
    ctx.fillStyle = this.color
    ctx.beginPath()
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2)
    ctx.fill()
    ctx.restore()
  }

  get isDead() {
    return this.life >= this.maxLife
  }
}

export const CanvasGame = ({ gameState, result }: CanvasGameProps) => {
  const canvasRef = useRef<HTMLCanvasElement>(null)
  const animationRef = useRef<number>(0)
  const particlesRef = useRef<Particle[]>([])
  const timeRef = useRef(0)

  useEffect(() => {
    const canvas = canvasRef.current
    if (!canvas) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // Canvas 크기 설정
    const resize = () => {
      canvas.width = window.innerWidth
      canvas.height = window.innerHeight
    }
    resize()
    window.addEventListener('resize', resize)

    // 애니메이션 루프
    const animate = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height)
      timeRef.current += 1

      // 배경
      ctx.fillStyle = '#111827'
      ctx.fillRect(0, 0, canvas.width, canvas.height)

      // 상태별 렌더링
      switch (gameState) {
        case 'idle':
          renderIdle(ctx, canvas, timeRef.current)
          break
        case 'hint':
          renderHint(ctx, canvas, timeRef.current)
          break
        case 'cue':
          renderCue(ctx, canvas, timeRef.current)
          break
        case 'result':
          renderResult(ctx, canvas, result)
          break
      }

      // 파티클 업데이트
      particlesRef.current = particlesRef.current.filter(p => !p.isDead)
      particlesRef.current.forEach(p => {
        p.update()
        p.draw(ctx)
      })

      animationRef.current = requestAnimationFrame(animate)
    }

    animate()

    return () => {
      window.removeEventListener('resize', resize)
      cancelAnimationFrame(animationRef.current)
    }
  }, [gameState, result])

  return <canvas ref={canvasRef} className="fixed inset-0" />
}

// Idle 상태 렌더링
function renderIdle(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement, time: number) {
  // 부드러운 그라데이션 배경
  const gradient = ctx.createRadialGradient(
    canvas.width / 2, canvas.height / 2, 0,
    canvas.width / 2, canvas.height / 2, canvas.width / 2
  )
  gradient.addColorStop(0, 'rgba(255, 255, 255, 0.02)')
  gradient.addColorStop(1, 'rgba(255, 255, 255, 0)')
  ctx.fillStyle = gradient
  ctx.fillRect(0, 0, canvas.width, canvas.height)
  
  // 떠다니는 파티클들
  ctx.save()
  for (let i = 0; i < 5; i++) {
    const y = ((time * 0.3 + i * 200) % (canvas.height + 200)) - 100
    const x = canvas.width / 2 + Math.sin(time * 0.01 + i) * 200
    const size = 50 + Math.sin(time * 0.02 + i) * 20
    
    const particleGradient = ctx.createRadialGradient(x, y, 0, x, y, size)
    particleGradient.addColorStop(0, 'rgba(255, 255, 255, 0.1)')
    particleGradient.addColorStop(1, 'rgba(255, 255, 255, 0)')
    
    ctx.fillStyle = particleGradient
    ctx.beginPath()
    ctx.arc(x, y, size, 0, Math.PI * 2)
    ctx.fill()
  }
  ctx.restore()
}

// Hint 상태 렌더링
function renderHint(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement, time: number) {
  // 화면 전체 펄스 효과
  const pulse = Math.sin(time * 0.05) * 0.5 + 0.5
  ctx.save()
  ctx.fillStyle = `rgba(255, 255, 255, ${pulse * 0.03})`
  ctx.fillRect(0, 0, canvas.width, canvas.height)
  ctx.restore()
  
  // 중앙에서 퍼지는 에너지 파동
  ctx.save()
  const centerX = canvas.width / 2
  const centerY = canvas.height / 2
  
  // 여러 겹의 파동
  for (let i = 0; i < 3; i++) {
    const offset = i * 40
    const radius = ((time + offset) % 180) * 5
    const alpha = Math.max(0, 1 - radius / 900) * 0.3
    
    const gradient = ctx.createRadialGradient(
      centerX, centerY, radius * 0.8,
      centerX, centerY, radius
    )
    gradient.addColorStop(0, `rgba(255, 255, 255, 0)`)
    gradient.addColorStop(1, `rgba(255, 255, 255, ${alpha})`)
    
    ctx.fillStyle = gradient
    ctx.beginPath()
    ctx.arc(centerX, centerY, radius, 0, Math.PI * 2)
    ctx.fill()
  }
  
  // 가장자리 번개 효과
  ctx.strokeStyle = `rgba(255, 255, 255, ${pulse * 0.1})`
  ctx.lineWidth = 1
  ctx.setLineDash([10, 20])
  
  for (let i = 0; i < 8; i++) {
    const angle = (i / 8) * Math.PI * 2 + time * 0.001
    const x1 = centerX + Math.cos(angle) * 300
    const y1 = centerY + Math.sin(angle) * 300
    const x2 = centerX + Math.cos(angle) * (500 + pulse * 50)
    const y2 = centerY + Math.sin(angle) * (500 + pulse * 50)
    
    ctx.beginPath()
    ctx.moveTo(x1, y1)
    ctx.lineTo(x2, y2)
    ctx.stroke()
  }
  
  ctx.setLineDash([])
  ctx.restore()
}

// Cue 상태 렌더링
function renderCue(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement, time: number) {
  const progress = Math.min(time / 20, 1)
  const centerX = canvas.width / 2
  const centerY = canvas.height / 2
  
  // 플래시 오버레이
  ctx.save()
  const flashAlpha = progress < 0.5 ? progress : (1 - progress)
  ctx.fillStyle = `rgba(253, 224, 71, ${flashAlpha * 0.3})`
  ctx.fillRect(0, 0, canvas.width, canvas.height)
  ctx.restore()
  
  // 십자가 빛
  ctx.save()
  ctx.translate(centerX, centerY)
  ctx.scale(progress, progress)
  
  // 글로우 효과
  ctx.shadowBlur = 50
  ctx.shadowColor = '#fde047'
  
  // 수직 빛
  const gradient1 = ctx.createLinearGradient(0, -canvas.height/2, 0, canvas.height/2)
  gradient1.addColorStop(0, 'rgba(253, 224, 71, 0)')
  gradient1.addColorStop(0.4, 'rgba(251, 191, 36, 0.8)')
  gradient1.addColorStop(0.5, 'rgba(253, 224, 71, 1)')
  gradient1.addColorStop(0.6, 'rgba(251, 191, 36, 0.8)')
  gradient1.addColorStop(1, 'rgba(253, 224, 71, 0)')
  
  ctx.fillStyle = gradient1
  ctx.beginPath()
  ctx.moveTo(0, -canvas.height/2)
  ctx.lineTo(20, -50)
  ctx.lineTo(30, -20)
  ctx.lineTo(0, 0)
  ctx.lineTo(-30, -20)
  ctx.lineTo(-20, -50)
  ctx.closePath()
  ctx.fill()
  
  ctx.beginPath()
  ctx.moveTo(0, canvas.height/2)
  ctx.lineTo(-20, 50)
  ctx.lineTo(-30, 20)
  ctx.lineTo(0, 0)
  ctx.lineTo(30, 20)
  ctx.lineTo(20, 50)
  ctx.closePath()
  ctx.fill()
  
  // 수평 빛
  const gradient2 = ctx.createLinearGradient(-canvas.width/2, 0, canvas.width/2, 0)
  gradient2.addColorStop(0, 'rgba(253, 224, 71, 0)')
  gradient2.addColorStop(0.4, 'rgba(251, 191, 36, 0.8)')
  gradient2.addColorStop(0.5, 'rgba(253, 224, 71, 1)')
  gradient2.addColorStop(0.6, 'rgba(251, 191, 36, 0.8)')
  gradient2.addColorStop(1, 'rgba(253, 224, 71, 0)')
  
  ctx.fillStyle = gradient2
  ctx.beginPath()
  ctx.moveTo(-canvas.width/2, 0)
  ctx.lineTo(-50, -20)
  ctx.lineTo(-20, -30)
  ctx.lineTo(0, 0)
  ctx.lineTo(-20, 30)
  ctx.lineTo(-50, 20)
  ctx.closePath()
  ctx.fill()
  
  ctx.beginPath()
  ctx.moveTo(canvas.width/2, 0)
  ctx.lineTo(50, 20)
  ctx.lineTo(20, 30)
  ctx.lineTo(0, 0)
  ctx.lineTo(20, -30)
  ctx.lineTo(50, -20)
  ctx.closePath()
  ctx.fill()
  
  // 중앙 플레어
  ctx.fillStyle = '#fde047'
  ctx.beginPath()
  ctx.arc(0, 0, 10 * progress, 0, Math.PI * 2)
  ctx.fill()
  
  ctx.restore()
}

// Result 상태 렌더링
function renderResult(ctx: CanvasRenderingContext2D, canvas: HTMLCanvasElement, result: GameResult | null) {
  if (!result) return
  
  ctx.save()
  ctx.font = 'bold 48px system-ui'
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'
  
  if (result.success) {
    ctx.fillStyle = '#4ade80'
    ctx.fillText(`성공!`, canvas.width / 2, canvas.height / 2 - 30)
    ctx.font = '36px system-ui'
    ctx.fillText(`${result.reactionTime}ms`, canvas.width / 2, canvas.height / 2 + 30)
  } else {
    ctx.fillStyle = '#f87171'
    ctx.fillText('실패...', canvas.width / 2, canvas.height / 2)
  }
  
  ctx.restore()
}