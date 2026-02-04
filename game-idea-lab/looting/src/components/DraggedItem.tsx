import { useCallback, useMemo } from 'react'
import { Graphics, TextStyle } from 'pixi.js'
import type { ItemInstance } from '../types/items'
import { RARITY_COLORS } from '../types/items'
import { getItemDefinition } from '../data/itemDefinitions'

interface DraggedItemProps {
  item: ItemInstance
  x: number
  y: number
}

export function DraggedItem({ item, x, y }: DraggedItemProps) {
  const definition = getItemDefinition(item.definitionId)
  const borderColor = definition ? RARITY_COLORS[definition.rarity] : 0x444466

  const drawBackground = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: 0x2a2a4a, alpha: 0.9 })
      g.setStrokeStyle({ width: 2, color: borderColor })
      g.roundRect(-40, -25, 80, 50, 6)
      g.fill()
      g.stroke()
    },
    [borderColor]
  )

  const nameStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 12,
    fill: 0xffffff,
    align: 'center',
  }), [])

  if (!definition) return null

  return (
    <pixiContainer x={x} y={y} alpha={0.85}>
      <pixiGraphics draw={drawBackground} />
      <pixiText
        text={definition.name}
        x={0}
        y={0}
        anchor={{ x: 0.5, y: 0.5 }}
        style={nameStyle}
      />
    </pixiContainer>
  )
}
