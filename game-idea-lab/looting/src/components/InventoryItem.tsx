import { useCallback, useMemo } from 'react'
import { Graphics, TextStyle } from 'pixi.js'
import type { ItemInstance } from '../types/items'
import { RARITY_COLORS } from '../types/items'
import { getItemDefinition } from '../data/itemDefinitions'

interface InventoryItemProps {
  item: ItemInstance
  x: number
  y: number
  width: number
  height: number
  onClick?: () => void
}

export function InventoryItem({
  item,
  x,
  y,
  width,
  height,
  onClick,
}: InventoryItemProps) {
  const definition = getItemDefinition(item.definitionId)
  if (!definition) return null

  const color = RARITY_COLORS[definition.rarity]

  const drawBackground = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: 0x2a2a4a })
      g.setStrokeStyle({ width: 2, color })
      g.roundRect(0, 0, width - 4, height - 4, 4)
      g.fill()
      g.stroke()
    },
    [width, height, color]
  )

  const nameStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 15,
    fill: 0xffffff,
    align: 'center',
  }), [])

  const quantityStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 15,
    fill: 0xcccccc,
  }), [])

  const shortName = definition.name.length > 10
    ? definition.name.substring(0, 8) + '..'
    : definition.name

  return (
    <pixiContainer x={x} y={y} eventMode="static" cursor="pointer" onPointerDown={onClick}>
      <pixiGraphics draw={drawBackground} />
      <pixiText
        text={shortName}
        x={width / 2 - 2}
        y={height / 2 - 15}
        anchor={{ x: 0.5, y: 0.5 }}
        style={nameStyle}
      />
      {definition.stackSize > 1 && (
        <pixiText
          text={`x${item.quantity}`}
          x={width - 15}
          y={height - 21}
          anchor={{ x: 1, y: 1 }}
          style={quantityStyle}
        />
      )}
    </pixiContainer>
  )
}
