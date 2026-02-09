import { useCallback, useEffect, useState, useMemo } from 'react'
import { Graphics, TextStyle } from 'pixi.js'
import { useTick } from '@pixi/react'
import type { ItemInstance } from '../types/items'
import { createItemInstance, getRandomItems } from '../data/itemDefinitions'
import { ChestItem } from './ChestItem'

interface ChestProps {
  x: number
  y: number
  state: 'closed' | 'opening' | 'open' | 'empty'
  items: ItemInstance[]
  onStateChange: (state: 'closed' | 'opening' | 'open' | 'empty') => void
  onItemsGenerated: (items: ItemInstance[]) => void
  onItemPickup: (item: ItemInstance) => void
  onEmpty: () => void
}

export function Chest({
  x,
  y,
  state,
  items,
  onStateChange,
  onItemsGenerated,
  onItemPickup,
  onEmpty,
}: ChestProps) {
  const [isHovered, setIsHovered] = useState(false)
  const [openProgress, setOpenProgress] = useState(0)
  const [openStartTime, setOpenStartTime] = useState<number | null>(null)

  const chestWidth = 120
  const chestHeight = 90

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'e' || e.key === 'E') {
        if (state === 'closed' && isHovered) {
          openChest()
        }
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [state, isHovered])

  useEffect(() => {
    if (items.length === 0 && state === 'open') {
      onEmpty()
    }
  }, [items.length, state, onEmpty])

  const openChest = useCallback(() => {
    if (state !== 'closed') return

    onStateChange('opening')
    setOpenStartTime(performance.now())

    const randomDefs = getRandomItems(2 + Math.floor(Math.random() * 3))
    const newItems = randomDefs.map(def => {
      const quantity = def.stackSize > 1
        ? Math.floor(Math.random() * def.stackSize) + 1
        : 1
      return createItemInstance(def.id, quantity)
    })
    onItemsGenerated(newItems)
  }, [state, onStateChange, onItemsGenerated])

  useTick(() => {
    if (state !== 'opening' || openStartTime === null) return

    const elapsed = performance.now() - openStartTime
    const duration = 400
    const progress = Math.min(elapsed / duration, 1)
    setOpenProgress(progress)

    if (progress >= 1) {
      onStateChange('open')
    }
  })

  const drawChestBody = useCallback(
    (g: Graphics) => {
      g.clear()

      const highlightColor = isHovered && state === 'closed' ? 0xffcc00 : 0x8b4513
      const bodyColor = state === 'empty' ? 0x3a2a1a : 0x654321

      g.setFillStyle({ color: bodyColor })
      g.setStrokeStyle({ width: isHovered && state === 'closed' ? 4 : 3, color: highlightColor })
      g.roundRect(-chestWidth / 2, -chestHeight / 2, chestWidth, chestHeight, 12)
      g.fill()
      g.stroke()

      if (state !== 'empty') {
        g.setFillStyle({ color: 0x8b6914 })
        g.roundRect(-chestWidth / 2 + 15, -chestHeight / 2 + 15, chestWidth - 30, 12, 3)
        g.fill()
      }

      const lockY = state === 'closed' ? 8 : 8 - openProgress * 30
      g.setFillStyle({ color: state === 'empty' ? 0x333333 : 0xffd700 })
      g.circle(0, lockY, 12)
      g.fill()
    },
    [chestWidth, chestHeight, isHovered, state, openProgress]
  )

  const drawLid = useCallback(
    (g: Graphics) => {
      if (state === 'closed') return

      g.clear()
      const lidHeight = 22

      g.setFillStyle({ color: state === 'empty' ? 0x3a2a1a : 0x654321 })
      g.setStrokeStyle({ width: 3, color: 0x8b4513 })
      g.roundRect(-chestWidth / 2, -chestHeight / 2 - lidHeight, chestWidth, lidHeight, 6)
      g.fill()
      g.stroke()
    },
    [chestWidth, chestHeight, state]
  )

  const handleClick = useCallback(() => {
    if (state === 'closed') {
      openChest()
    }
  }, [state, openChest])

  const handleItemPickup = useCallback(
    (item: ItemInstance) => {
      onItemPickup(item)
    },
    [onItemPickup]
  )

  const hintStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 18,
    fill: 0xffcc00,
    align: 'center',
  }), [])

  const emptyStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 18,
    fill: 0x666666,
    align: 'center',
  }), [])

  const itemSpacing = 120
  const itemsStartX = x - ((items.length - 1) * itemSpacing) / 2

  return (
    <pixiContainer>
      <pixiContainer
        x={x}
        y={y}
        eventMode="static"
        cursor={state === 'closed' ? 'pointer' : 'default'}
        onPointerEnter={() => setIsHovered(true)}
        onPointerLeave={() => setIsHovered(false)}
        onPointerDown={handleClick}
      >
        <pixiGraphics draw={drawChestBody} />
        {state !== 'closed' && <pixiGraphics draw={drawLid} y={-openProgress * 22} />}

        {state === 'closed' && isHovered && (
          <pixiText
            text="Press E or Click"
            x={0}
            y={-75}
            anchor={{ x: 0.5, y: 0.5 }}
            style={hintStyle}
          />
        )}

        {state === 'empty' && (
          <pixiText
            text="Empty"
            x={0}
            y={0}
            anchor={{ x: 0.5, y: 0.5 }}
            style={emptyStyle}
          />
        )}
      </pixiContainer>

      {(state === 'opening' || state === 'open') &&
        items.map((item, i) => (
          <ChestItem
            key={item.id}
            item={item}
            baseX={x}
            baseY={y - 45}
            targetX={itemsStartX + i * itemSpacing}
            targetY={y + 105}
            delay={i * 100}
            onClick={() => handleItemPickup(item)}
          />
        ))}
    </pixiContainer>
  )
}
