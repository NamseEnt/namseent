import type {
  ItemInstance,
  InventoryState,
  EquipmentSlotType,
  WeaponSlotType,
  AttachmentType,
  Rarity,
} from '../types/items'
import {
  RARITY_COLORS,
  DEFAULT_BAG_SIZE,
  BAG_SIZE_BY_BACKPACK,
} from '../types/items'
import { getItemDefinition } from '../data/itemDefinitions'

export function getRarityColor(rarity: Rarity): number {
  return RARITY_COLORS[rarity]
}

export function canStackItems(item1: ItemInstance, item2: ItemInstance): boolean {
  if (item1.definitionId !== item2.definitionId) return false
  const definition = getItemDefinition(item1.definitionId)
  if (!definition) return false
  return definition.stackSize > 1
}

export function getEquipmentSlotForItem(definitionId: string): EquipmentSlotType | null {
  const definition = getItemDefinition(definitionId)
  if (!definition || definition.category !== 'equipment') return null

  switch (definition.subType) {
    case 'helmet':
      return 'helmet'
    case 'bodyShield':
      return 'bodyShield'
    case 'knockdownShield':
      return 'knockdownShield'
    case 'backpack':
      return 'backpack'
    default:
      return null
  }
}

export function shouldAutoEquip(
  currentItem: ItemInstance | null,
  newItem: ItemInstance
): boolean {
  if (!currentItem) return true

  const currentDef = getItemDefinition(currentItem.definitionId)
  const newDef = getItemDefinition(newItem.definitionId)

  if (!currentDef || !newDef) return false

  return newDef.rarity > currentDef.rarity
}

export function canAttachToWeapon(
  weapon: ItemInstance,
  attachment: ItemInstance
): boolean {
  const weaponDef = getItemDefinition(weapon.definitionId)
  const attachmentDef = getItemDefinition(attachment.definitionId)

  if (!weaponDef || !attachmentDef) return false
  if (weaponDef.category !== 'weapon') return false
  if (attachmentDef.category !== 'attachment') return false
  if (!weaponDef.attachmentSlots) return false

  return weaponDef.attachmentSlots.includes(attachmentDef.subType as AttachmentType)
}

export function getBagSize(state: InventoryState): number {
  const backpack = state.equipment.backpack
  if (!backpack) return DEFAULT_BAG_SIZE

  const backpackDef = getItemDefinition(backpack.definitionId)
  if (!backpackDef) return DEFAULT_BAG_SIZE

  return BAG_SIZE_BY_BACKPACK[backpackDef.rarity]
}

export function findEmptyBagSlot(state: InventoryState): number {
  const bagSize = getBagSize(state)
  for (let i = 0; i < bagSize; i++) {
    if (!state.bag[i]) return i
  }
  return -1
}

export function findStackableSlot(
  state: InventoryState,
  item: ItemInstance
): number {
  const definition = getItemDefinition(item.definitionId)
  if (!definition || definition.stackSize <= 1) return -1

  for (let i = 0; i < state.bag.length; i++) {
    const bagItem = state.bag[i]
    if (bagItem && bagItem.definitionId === item.definitionId) {
      if (bagItem.quantity < definition.stackSize) {
        return i
      }
    }
  }
  return -1
}

export function findWeaponSlotForAttachment(
  state: InventoryState,
  attachment: ItemInstance
): WeaponSlotType | null {
  const attachmentDef = getItemDefinition(attachment.definitionId)
  if (!attachmentDef || attachmentDef.category !== 'attachment') return null

  const attachmentType = attachmentDef.subType as AttachmentType

  for (const slotKey of ['weapon1', 'weapon2'] as const) {
    const weapon = state.weapons[slotKey]
    if (weapon && canAttachToWeapon(weapon, attachment)) {
      if (!weapon.attachments?.[attachmentType]) {
        return slotKey
      }
    }
  }

  for (const slotKey of ['weapon1', 'weapon2'] as const) {
    const weapon = state.weapons[slotKey]
    if (weapon && canAttachToWeapon(weapon, attachment)) {
      const currentAttachment = weapon.attachments?.[attachmentType]
      if (currentAttachment) {
        const currentDef = getItemDefinition(currentAttachment.definitionId)
        if (currentDef && currentDef.rarity < attachmentDef.rarity) {
          return slotKey
        }
      }
    }
  }

  return null
}
