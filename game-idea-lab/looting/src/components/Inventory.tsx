import { useCallback } from 'react'
import { Graphics } from 'pixi.js'
import type { InventoryState, EquipmentSlotType, WeaponSlotType, AttachmentType } from '../types/items'
import { EquipmentSlot } from './EquipmentSlot'
import { WeaponSlot } from './WeaponSlot'
import { BagGrid } from './BagGrid'

interface InventoryProps {
  x: number
  y: number
  state: InventoryState
  onItemDrop?: (slotType: 'equipment' | 'weapon' | 'bag', slotIndex: number | EquipmentSlotType | WeaponSlotType) => void
  onAttachmentDragStart?: (weaponSlot: WeaponSlotType, attachmentType: AttachmentType) => void
  onBagDrop?: () => void
  onWeaponDrop?: (weaponSlot: WeaponSlotType) => void
  isDragging?: boolean
}

export function Inventory({
  x,
  y,
  state,
  onItemDrop,
  onAttachmentDragStart,
  onBagDrop,
  onWeaponDrop,
  isDragging,
}: InventoryProps) {
  const columns = 6
  const cellSize = 84
  const rows = Math.ceil(state.bagSize / columns)
  const bagGridHeight = rows * cellSize + 16
  const totalHeight = 195 + bagGridHeight + 24

  const drawBackground = useCallback((g: Graphics) => {
    g.clear()
    g.setFillStyle({ color: 0x16162a, alpha: 0.95 })
    g.setStrokeStyle({ width: 2, color: 0x333355 })
    g.roundRect(0, 0, 780, totalHeight, 12)
    g.fill()
    g.stroke()
  }, [totalHeight])

  const equipmentSlots: EquipmentSlotType[] = ['helmet', 'bodyShield', 'knockdownShield', 'backpack']

  return (
    <pixiContainer x={x} y={y}>
      <pixiGraphics draw={drawBackground} />

      <pixiContainer x={24} y={24}>
        {equipmentSlots.map((slotType, i) => (
          <EquipmentSlot
            key={slotType}
            x={0}
            y={i * 72}
            width={135}
            height={66}
            slotType={slotType}
            item={state.equipment[slotType]}
          />
        ))}
      </pixiContainer>

      <pixiContainer x={180} y={24}>
        <WeaponSlot
          x={0}
          y={0}
          width={255}
          height={128}
          slotType="weapon1"
          item={state.weapons.weapon1}
          onDrop={onItemDrop ? () => onItemDrop('weapon', 'weapon1') : undefined}
          onAttachmentDragStart={onAttachmentDragStart ? (attachType) => onAttachmentDragStart('weapon1', attachType) : undefined}
          onWeaponDrop={onWeaponDrop ? () => onWeaponDrop('weapon1') : undefined}
          isDragging={isDragging}
        />
        <WeaponSlot
          x={270}
          y={0}
          width={255}
          height={128}
          slotType="weapon2"
          item={state.weapons.weapon2}
          onDrop={onItemDrop ? () => onItemDrop('weapon', 'weapon2') : undefined}
          onAttachmentDragStart={onAttachmentDragStart ? (attachType) => onAttachmentDragStart('weapon2', attachType) : undefined}
          onWeaponDrop={onWeaponDrop ? () => onWeaponDrop('weapon2') : undefined}
          isDragging={isDragging}
        />
      </pixiContainer>

      <BagGrid
        x={180}
        y={195}
        items={state.bag}
        bagSize={state.bagSize}
        columns={columns}
        cellSize={cellSize}
        onItemClick={onItemDrop ? (index) => onItemDrop('bag', index) : undefined}
        onDrop={onBagDrop}
        isDragging={isDragging}
      />
    </pixiContainer>
  )
}
