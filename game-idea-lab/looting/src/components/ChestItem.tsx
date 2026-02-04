import { useCallback, useEffect, useState, useMemo } from 'react'
import { Graphics, TextStyle } from 'pixi.js'
import { useTick } from '@pixi/react'
import type { ItemInstance } from '../types/items'
import { RARITY_COLORS } from '../types/items'
import { getItemDefinition } from '../data/itemDefinitions'

interface ChestItemProps {
  item: ItemInstance
  baseX: number
  baseY: number
  targetX: number
  targetY: number
  delay: number
  onClick: () => void
}

export function ChestItem({
  item,
  baseX,
  baseY,
  targetX,
  targetY,
  delay,
  onClick,
}: ChestItemProps) {
  const definition = getItemDefinition(item.definitionId)
  const [animProgress, setAnimProgress] = useState(0)
  const [isHovered, setIsHovered] = useState(false)
  const [startTime, setStartTime] = useState<number | null>(null)
  const [fixedTargetX] = useState(targetX)
  const [fixedTargetY] = useState(targetY)
  const [fixedDelay] = useState(delay)

  useEffect(() => {
    const timer = setTimeout(() => {
      setStartTime(performance.now())
    }, fixedDelay)
    return () => clearTimeout(timer)
  }, [])

  useTick(() => {
    if (startTime === null) return
    const elapsed = performance.now() - startTime
    const duration = 300
    const progress = Math.min(elapsed / duration, 1)
    const eased = 1 - Math.pow(1 - progress, 3)
    setAnimProgress(eased)
  })

  if (!definition) return null

  const color = RARITY_COLORS[definition.rarity]
  const width = 105
  const height = 90

  const currentX = baseX + (fixedTargetX - baseX) * animProgress
  const currentY = baseY + (fixedTargetY - baseY) * animProgress
  const scale = 0.5 + 0.5 * animProgress

  const drawBackground = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: isHovered ? 0x3a3a5a : 0x2a2a4a })
      g.setStrokeStyle({ width: isHovered ? 3 : 2, color })
      g.roundRect(-width / 2, -height / 2, width, height, 9)
      g.fill()
      g.stroke()
    },
    [width, height, color, isHovered]
  )

  const nameStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 15,
    fill: 0xffffff,
    align: 'center',
    wordWrap: true,
    wordWrapWidth: width - 12,
  }), [width])

  const quantityStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 14,
    fill: 0xcccccc,
  }), [])

  if (animProgress === 0 && startTime === null) return null

  return (
    <pixiContainer
      x={currentX}
      y={currentY}
      scale={scale}
      alpha={animProgress}
      eventMode="static"
      cursor="pointer"
      onPointerEnter={() => setIsHovered(true)}
      onPointerLeave={() => setIsHovered(false)}
      onPointerDown={onClick}
    >
      <pixiGraphics draw={drawBackground} />
      <pixiText
        text={definition.name}
        x={0}
        y={0}
        anchor={{ x: 0.5, y: 0.5 }}
        style={nameStyle}
      />
      {definition.stackSize > 1 && (
        <pixiText
          text={`x${item.quantity}`}
          x={0}
          y={30}
          anchor={{ x: 0.5, y: 0.5 }}
          style={quantityStyle}
        />
      )}
    </pixiContainer>
  )
}
