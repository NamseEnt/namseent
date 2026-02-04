import { Application, extend } from '@pixi/react'
import { Container, Graphics, Text } from 'pixi.js'
import { useState, useCallback } from 'react'
import { Chest } from './components/Chest'
import { Inventory } from './components/Inventory'
import { useInventory } from './hooks/useInventory'
import type { ItemInstance } from './types/items'

extend({ Container, Graphics, Text })

function App() {
  const [chestItems, setChestItems] = useState<ItemInstance[]>([])
  const [chestState, setChestState] = useState<'closed' | 'opening' | 'open' | 'empty'>('closed')
  const inventory = useInventory()

  const handleItemPickup = useCallback((item: ItemInstance) => {
    inventory.addItem(item)
    setChestItems(prev => prev.filter(i => i.id !== item.id))
  }, [inventory])

  const handleChestEmpty = useCallback(() => {
    if (chestItems.length === 0) {
      setChestState('empty')
    }
  }, [chestItems.length])

  const handleNewChest = useCallback(() => {
    setChestItems([])
    setChestState('closed')
  }, [])

  return (
    <div style={{ width: '100vw', height: '100vh', position: 'relative' }}>
      <Application background={0x1a1a2e} resizeTo={window}>
        <Chest
          x={640}
          y={150}
          state={chestState}
          items={chestItems}
          onStateChange={setChestState}
          onItemsGenerated={setChestItems}
          onItemPickup={handleItemPickup}
          onEmpty={handleChestEmpty}
        />
        <Inventory
          x={140}
          y={320}
          state={inventory.state}
          onItemDrop={inventory.dropItem}
        />
      </Application>
      <button
        onClick={handleNewChest}
        style={{
          position: 'absolute',
          top: 40,
          left: 640,
          transform: 'translateX(-50%)',
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
    </div>
  )
}

export default App
