import { useCallback } from 'react'
import { Graphics } from 'pixi.js'
import type { InventoryState, EquipmentSlotType, WeaponSlotType } from '../types/items'
import { EquipmentSlot } from './EquipmentSlot'
import { WeaponSlot } from './WeaponSlot'
import { BagGrid } from './BagGrid'

interface InventoryProps {
  x: number
  y: number
  state: InventoryState
  onItemDrop?: (slotType: 'equipment' | 'weapon' | 'bag', slotIndex: number | EquipmentSlotType | WeaponSlotType) => void
}

export function Inventory({
  x,
  y,
  state,
  onItemDrop,
}: InventoryProps) {
  const drawBackground = useCallback((g: Graphics) => {
    g.clear()
    g.setFillStyle({ color: 0x16162a, alpha: 0.95 })
    g.setStrokeStyle({ width: 2, color: 0x333355 })
    g.roundRect(0, 0, 520, 380, 12)
    g.fill()
    g.stroke()
  }, [])

  const equipmentSlots: EquipmentSlotType[] = ['helmet', 'bodyShield', 'knockdownShield', 'backpack']

  return (
    <pixiContainer x={x} y={y}>
      <pixiGraphics draw={drawBackground} />

      <pixiContainer x={16} y={16}>
        {equipmentSlots.map((slotType, i) => (
          <EquipmentSlot
            key={slotType}
            x={0}
            y={i * 48}
            width={90}
            height={44}
            slotType={slotType}
            item={state.equipment[slotType]}
            onDrop={onItemDrop ? () => onItemDrop('equipment', slotType) : undefined}
          />
        ))}
      </pixiContainer>

      <pixiContainer x={120} y={16}>
        <WeaponSlot
          x={0}
          y={0}
          width={170}
          height={85}
          slotType="weapon1"
          item={state.weapons.weapon1}
          onDrop={onItemDrop ? () => onItemDrop('weapon', 'weapon1') : undefined}
        />
        <WeaponSlot
          x={180}
          y={0}
          width={170}
          height={85}
          slotType="weapon2"
          item={state.weapons.weapon2}
          onDrop={onItemDrop ? () => onItemDrop('weapon', 'weapon2') : undefined}
        />
      </pixiContainer>

      <BagGrid
        x={120}
        y={130}
        items={state.bag}
        bagSize={state.bagSize}
        columns={6}
        cellSize={56}
        onItemClick={onItemDrop ? (index) => onItemDrop('bag', index) : undefined}
      />
    </pixiContainer>
  )
}
