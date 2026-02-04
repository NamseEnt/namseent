import { useCallback, useMemo } from 'react'
import { Graphics, TextStyle } from 'pixi.js'
import type { ItemInstance } from '../types/items'
import { RARITY_COLORS } from '../types/items'
import { getItemDefinition } from '../data/itemDefinitions'

interface BagGridProps {
  x: number
  y: number
  items: (ItemInstance | null)[]
  bagSize: number
  columns?: number
  cellSize?: number
  onItemClick?: (index: number) => void
}

export function BagGrid({
  x,
  y,
  items,
  bagSize,
  columns = 5,
  cellSize = 60,
  onItemClick,
}: BagGridProps) {
  const rows = Math.ceil(bagSize / columns)
  const padding = 6

  const drawBackground = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: 0x0a0a1a, alpha: 0.8 })
      g.roundRect(-8, -8, columns * cellSize + 16, rows * cellSize + 16, 8)
      g.fill()
    },
    [columns, rows, cellSize]
  )

  const titleStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 18,
    fill: 0x888899,
  }), [])

  const cells = []
  for (let i = 0; i < bagSize; i++) {
    const row = Math.floor(i / columns)
    const col = i % columns
    const cellX = col * cellSize
    const cellY = row * cellSize
    const item = items[i]

    cells.push(
      <BagCell
        key={i}
        x={cellX}
        y={cellY}
        cellSize={cellSize}
        padding={padding}
        item={item}
        onClick={item && onItemClick ? () => onItemClick(i) : undefined}
      />
    )
  }

  return (
    <pixiContainer x={x} y={y}>
      <pixiGraphics draw={drawBackground} />
      <pixiText
        text={`Bag (${items.filter(Boolean).length}/${bagSize})`}
        x={0}
        y={-36}
        style={titleStyle}
      />
      {cells}
    </pixiContainer>
  )
}

interface BagCellProps {
  x: number
  y: number
  cellSize: number
  padding: number
  item: ItemInstance | null
  onClick?: () => void
}

function BagCell({ x, y, cellSize, padding, item, onClick }: BagCellProps) {
  const definition = item ? getItemDefinition(item.definitionId) : null
  const color = definition ? RARITY_COLORS[definition.rarity] : 0x333355

  const drawCell = useCallback(
    (g: Graphics) => {
      g.clear()
      g.setFillStyle({ color: item ? 0x2a2a4a : 0x1a1a3a })
      g.setStrokeStyle({ width: item ? 2 : 1, color })
      g.roundRect(0, 0, cellSize - padding, cellSize - padding, 4)
      g.fill()
      g.stroke()
    },
    [cellSize, padding, item, color]
  )

  const nameStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 11,
    fill: 0xffffff,
    align: 'center',
    wordWrap: true,
    wordWrapWidth: cellSize - padding - 4,
  }), [cellSize, padding])

  const quantityStyle = useMemo(() => new TextStyle({
    fontFamily: 'Arial',
    fontSize: 14,
    fill: 0xcccccc,
  }), [])

  return (
    <pixiContainer
      x={x}
      y={y}
      eventMode="static"
      cursor={item ? 'pointer' : 'default'}
      onPointerDown={onClick}
    >
      <pixiGraphics draw={drawCell} />
      {item && definition && (
        <>
          <pixiText
            text={definition.name}
            x={(cellSize - padding) / 2}
            y={(cellSize - padding) / 2 - 6}
            anchor={{ x: 0.5, y: 0.5 }}
            style={nameStyle}
          />
          {definition.stackSize > 1 && (
            <pixiText
              text={`x${item.quantity}`}
              x={cellSize - padding - 9}
              y={cellSize - padding - 12}
              anchor={{ x: 1, y: 1 }}
              style={quantityStyle}
            />
          )}
        </>
      )}
    </pixiContainer>
  )
}
