import { useState, useCallback } from 'react'
import type {
  InventoryState,
  ItemInstance,
  EquipmentSlotType,
  WeaponSlotType,
  AttachmentType,
} from '../types/items'
import { DEFAULT_BAG_SIZE } from '../types/items'
import { getItemDefinition } from '../data/itemDefinitions'
import {
  getEquipmentSlotForItem,
  shouldAutoEquip,
  findEmptyBagSlot,
  findStackableSlot,
  findWeaponSlotForAttachment,
  getBagSize,
} from '../utils/itemUtils'

const initialState: InventoryState = {
  equipment: {
    helmet: null,
    bodyShield: null,
    knockdownShield: null,
    backpack: null,
  },
  weapons: {
    weapon1: null,
    weapon2: null,
  },
  bag: Array(DEFAULT_BAG_SIZE).fill(null),
  bagSize: DEFAULT_BAG_SIZE,
}

export interface AddItemResult {
  success: boolean
  replacedItem?: ItemInstance
}

export function useInventory() {
  const [state, setState] = useState<InventoryState>(initialState)

  const addItem = useCallback((item: ItemInstance): AddItemResult => {
    const definition = getItemDefinition(item.definitionId)
    if (!definition) return { success: false }

    let result: AddItemResult = { success: false }

    setState(prev => {
      const newState = { ...prev }

      if (definition.category === 'equipment') {
        const slot = getEquipmentSlotForItem(item.definitionId)
        if (slot) {
          const currentItem = newState.equipment[slot]
          if (shouldAutoEquip(currentItem, item)) {
            newState.equipment = { ...newState.equipment, [slot]: item }
            result = { success: true, replacedItem: currentItem ?? undefined }

            if (slot === 'backpack') {
              const newBagSize = getBagSize(newState)
              if (newBagSize > newState.bag.length) {
                newState.bag = [...newState.bag, ...Array(newBagSize - newState.bag.length).fill(null)]
              }
              newState.bagSize = newBagSize
            }
            return newState
          }
        }
      }

      if (definition.category === 'weapon') {
        if (!newState.weapons.weapon1) {
          newState.weapons = { ...newState.weapons, weapon1: item }
          result = { success: true }
          return newState
        }
        if (!newState.weapons.weapon2) {
          newState.weapons = { ...newState.weapons, weapon2: item }
          result = { success: true }
          return newState
        }
      }

      if (definition.category === 'attachment') {
        const weaponSlot = findWeaponSlotForAttachment(newState, item)
        if (weaponSlot) {
          const weapon = newState.weapons[weaponSlot]
          if (weapon && weapon.attachments) {
            const attachmentType = definition.subType as AttachmentType
            const replacedAttachment = weapon.attachments[attachmentType]

            const newWeapon: ItemInstance = {
              ...weapon,
              attachments: {
                ...weapon.attachments,
                [attachmentType]: item,
              },
            }
            newState.weapons = { ...newState.weapons, [weaponSlot]: newWeapon }

            if (replacedAttachment) {
              const emptySlot = findEmptyBagSlot(newState)
              if (emptySlot !== -1) {
                newState.bag = [...newState.bag]
                newState.bag[emptySlot] = replacedAttachment
              }
            }

            result = { success: true }
            return newState
          }
        }
      }

      if (definition.stackSize > 1) {
        const stackSlot = findStackableSlot(newState, item)
        if (stackSlot !== -1) {
          const existingItem = newState.bag[stackSlot]!
          const totalQuantity = existingItem.quantity + item.quantity
          const maxStack = definition.stackSize

          newState.bag = [...newState.bag]

          if (totalQuantity <= maxStack) {
            newState.bag[stackSlot] = { ...existingItem, quantity: totalQuantity }
            result = { success: true }
            return newState
          } else {
            newState.bag[stackSlot] = { ...existingItem, quantity: maxStack }
            item = { ...item, quantity: totalQuantity - maxStack }
          }
        }
      }

      const emptySlot = findEmptyBagSlot(newState)
      if (emptySlot !== -1) {
        newState.bag = [...newState.bag]
        newState.bag[emptySlot] = item
        result = { success: true }
        return newState
      }

      return prev
    })

    return result
  }, [])

  const dropItem = useCallback((slotType: 'equipment' | 'weapon' | 'bag', slotIndex: number | EquipmentSlotType | WeaponSlotType) => {
    setState(prev => {
      const newState = { ...prev }

      if (slotType === 'equipment' && typeof slotIndex === 'string') {
        const slot = slotIndex as EquipmentSlotType
        newState.equipment = { ...newState.equipment, [slot]: null }

        if (slot === 'backpack') {
          newState.bagSize = DEFAULT_BAG_SIZE
          if (newState.bag.length > DEFAULT_BAG_SIZE) {
            newState.bag = newState.bag.slice(0, DEFAULT_BAG_SIZE)
          }
        }
      } else if (slotType === 'weapon' && typeof slotIndex === 'string') {
        const slot = slotIndex as WeaponSlotType
        newState.weapons = { ...newState.weapons, [slot]: null }
      } else if (slotType === 'bag' && typeof slotIndex === 'number') {
        newState.bag = [...newState.bag]
        newState.bag[slotIndex] = null
      }

      return newState
    })
  }, [])

  const swapSlots = useCallback((
    sourceType: 'equipment' | 'weapon' | 'bag',
    sourceIndex: number | EquipmentSlotType | WeaponSlotType,
    targetType: 'equipment' | 'weapon' | 'bag',
    targetIndex: number | EquipmentSlotType | WeaponSlotType
  ) => {
    setState(prev => {
      const newState = { ...prev }

      const getItem = (type: string, index: number | string): ItemInstance | null => {
        if (type === 'equipment') return newState.equipment[index as EquipmentSlotType]
        if (type === 'weapon') return newState.weapons[index as WeaponSlotType]
        if (type === 'bag') return newState.bag[index as number]
        return null
      }

      const setItem = (type: string, index: number | string, item: ItemInstance | null) => {
        if (type === 'equipment') {
          newState.equipment = { ...newState.equipment, [index as EquipmentSlotType]: item }
        } else if (type === 'weapon') {
          newState.weapons = { ...newState.weapons, [index as WeaponSlotType]: item }
        } else if (type === 'bag') {
          newState.bag = [...newState.bag]
          newState.bag[index as number] = item
        }
      }

      const sourceItem = getItem(sourceType, sourceIndex)
      const targetItem = getItem(targetType, targetIndex)

      setItem(sourceType, sourceIndex, targetItem)
      setItem(targetType, targetIndex, sourceItem)

      return newState
    })
  }, [])

  return {
    state,
    addItem,
    dropItem,
    swapSlots,
  }
}
