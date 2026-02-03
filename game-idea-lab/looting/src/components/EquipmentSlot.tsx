import { useCallback, useMemo } from 'react'
import { Graphics, TextStyle } from 'pixi.js'
import type { ItemInstance, EquipmentSlotType } from '../types/items'
import { RARITY_COLORS } from '../types/items'
import { getItemDefinition } from '../data/itemDefinitions'

interface EquipmentSlotProps {
  x: number
  y: number
  width: number
  height: number
  slotType: EquipmentSlotType
  item: ItemInstance | null
  onDrop?: () => void
}

const SLOT_LABELS: Record<EquipmentSlotType, string> = {
  helmet: 'Helmet',
  bodyShield: 'Shield',
  knockdownShield: 'Knockdown',
  backpack: 'Backpack',
}

export function EquipmentSlot({
  x,
  y,
  width,
  height,
  slotType,
  item,
  onDrop,
}: EquipmentSlotProps) {
  const definition = item ? getItemDefinition(item.definitionId) : null
  const borderColor = definition ? RARITY_COLORS[definition.rarity] : 0x444466

  const drawSlot = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: item ? 0x2a2a4a : 0x1a1a3a })
      g.setStrokeStyle({ width: 2, color: borderColor })
      g.roundRect(0, 0, width, height, 6)
      g.fill()
      g.stroke()
    },
    [width, height, item, borderColor]
  )

  const labelStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 10,
    fill: 0x666688,
    align: 'center',
  }), [])

  const nameStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 10,
    fill: 0xffffff,
    align: 'center',
  }), [])

  const levelStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 9,
    fill: borderColor,
  }), [borderColor])

  return (
    <pixiContainer x={x} y={y} eventMode="static" cursor={item ? 'pointer' : 'default'} onPointerDown={item && onDrop ? onDrop : undefined}>
      <pixiGraphics draw={drawSlot} />
      {!item && (
        <pixiText
          text={SLOT_LABELS[slotType]}
          x={width / 2}
          y={height / 2}
          anchor={{ x: 0.5, y: 0.5 }}
          style={labelStyle}
        />
      )}
      {item && definition && (
        <>
          <pixiText
            text={definition.name}
            x={width / 2}
            y={height / 2 - 6}
            anchor={{ x: 0.5, y: 0.5 }}
            style={nameStyle}
          />
          <pixiText
            text={`LV${definition.rarity}`}
            x={width / 2}
            y={height / 2 + 8}
            anchor={{ x: 0.5, y: 0.5 }}
            style={levelStyle}
          />
        </>
      )}
    </pixiContainer>
  )
}
