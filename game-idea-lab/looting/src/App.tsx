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

  return (
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
  )
}

export default App
