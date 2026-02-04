import { Application, extend } from '@pixi/react'
import { Container, Graphics, Text } from 'pixi.js'
import { useState, useCallback, useEffect } from 'react'
import { Chest } from './components/Chest'
import { Inventory } from './components/Inventory'
import { DraggedItem } from './components/DraggedItem'
import { useInventory } from './hooks/useInventory'
import type { ItemInstance, EquipmentSlotType, WeaponSlotType, AttachmentType } from './types/items'

extend({ Container, Graphics, Text })

export interface DragState {
  item: ItemInstance
  source: {
    type: 'weapon_attachment'
    weaponSlot: WeaponSlotType
    attachmentType: AttachmentType
  }
}

function App() {
  const [chestItems, setChestItems] = useState<ItemInstance[]>([])
  const [fieldItems, setFieldItems] = useState<ItemInstance[]>([])
  const [chestState, setChestState] = useState<'closed' | 'opening' | 'open' | 'empty'>('closed')
  const [dragState, setDragState] = useState<DragState | null>(null)
  const [mousePos, setMousePos] = useState({ x: 0, y: 0 })
  const inventory = useInventory()

  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      setMousePos({ x: e.clientX, y: e.clientY })
    }

    const handleMouseUp = () => {
      if (dragState) {
        setFieldItems(prev => [...prev, dragState.item])
        setDragState(null)
      }
    }

    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)

    return () => {
      window.removeEventListener('mousemove', handleMouseMove)
      window.removeEventListener('mouseup', handleMouseUp)
    }
  }, [dragState])

  const handleItemPickup = useCallback((item: ItemInstance) => {
    inventory.addItem(item)
    setChestItems(prev => prev.filter(i => i.id !== item.id))
    setFieldItems(prev => prev.filter(i => i.id !== item.id))
  }, [inventory])

  const handleChestEmpty = useCallback(() => {
    if (chestItems.length === 0 && fieldItems.length === 0) {
      setChestState('empty')
    }
  }, [chestItems.length, fieldItems.length])

  const handleNewChest = useCallback(() => {
    setChestItems([])
    setFieldItems([])
    setChestState('closed')
  }, [])

  const handleItemDrop = useCallback((slotType: 'equipment' | 'weapon' | 'bag', slotIndex: number | EquipmentSlotType | WeaponSlotType) => {
    const droppedItem = inventory.dropItem(slotType, slotIndex)
    if (droppedItem) {
      setFieldItems(prev => [...prev, droppedItem])
    }
  }, [inventory])

  const handleAttachmentDragStart = useCallback((weaponSlot: WeaponSlotType, attachmentType: AttachmentType) => {
    const attachment = inventory.removeAttachment(weaponSlot, attachmentType)
    if (attachment) {
      setDragState({
        item: attachment,
        source: {
          type: 'weapon_attachment',
          weaponSlot,
          attachmentType,
        },
      })
    }
  }, [inventory])

  const handleBagDrop = useCallback(() => {
    if (!dragState) return
    const success = inventory.addToBag(dragState.item)
    if (success) {
      setDragState(null)
    }
  }, [dragState, inventory])

  const handleWeaponDrop = useCallback((weaponSlot: WeaponSlotType) => {
    if (!dragState) return
    const { success, replacedItem } = inventory.addAttachmentToWeapon(weaponSlot, dragState.item)
    if (success) {
      if (replacedItem) {
        setFieldItems(prev => [...prev, replacedItem])
      }
      setDragState(null)
    }
  }, [dragState, inventory])

  return (
    <div style={{ width: '100vw', height: '100vh', position: 'relative' }}>
      <Application background={0x1a1a2e} resizeTo={window}>
        <Chest
          x={640}
          y={150}
          state={chestState}
          items={[...chestItems, ...fieldItems]}
          onStateChange={setChestState}
          onItemsGenerated={setChestItems}
          onItemPickup={handleItemPickup}
          onEmpty={handleChestEmpty}
        />
        <Inventory
          x={140}
          y={320}
          state={inventory.state}
          onItemDrop={handleItemDrop}
          onAttachmentDragStart={handleAttachmentDragStart}
          onBagDrop={dragState ? handleBagDrop : undefined}
          onWeaponDrop={dragState ? handleWeaponDrop : undefined}
          isDragging={dragState !== null}
        />
        {dragState && (
          <DraggedItem
            item={dragState.item}
            x={mousePos.x}
            y={mousePos.y}
          />
        )}
      </Application>
      {chestState !== 'closed' && (
        <button
          onClick={handleNewChest}
          style={{
            position: 'absolute',
            top: 120,
            left: 820,
            padding: '12px 24px',
            fontSize: 16,
            backgroundColor: '#4a4a6a',
            color: 'white',
            border: '2px solid #6a6a8a',
            borderRadius: 8,
            cursor: 'pointer',
          }}
        >
          새 박스
        </button>
      )}
    </div>
  )
}

export default App
