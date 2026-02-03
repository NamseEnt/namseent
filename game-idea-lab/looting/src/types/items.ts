export type ItemCategory =
  | 'equipment'
  | 'weapon'
  | 'attachment'
  | 'consumable'
  | 'ammo'
  | 'grenade'

export type EquipmentType = 'helmet' | 'bodyShield' | 'knockdownShield' | 'backpack'
export type WeaponType = 'ar' | 'smg' | 'shotgun'
export type AttachmentType = 'scope' | 'extendedMag' | 'barrel'
export type ConsumableType = 'syringe' | 'medkit' | 'shieldCell' | 'shieldBattery'
export type AmmoType = 'light' | 'heavy' | 'energy' | 'shotgun'
export type GrenadeType = 'frag' | 'arcStar' | 'thermite'

export type Rarity = 1 | 2 | 3 | 4 | 5

export const RARITY_COLORS: Record<Rarity, number> = {
  1: 0x808080,
  2: 0x4da6ff,
  3: 0x9933ff,
  4: 0xffcc00,
  5: 0xff3333,
}

export const RARITY_NAMES: Record<Rarity, string> = {
  1: 'Common',
  2: 'Rare',
  3: 'Epic',
  4: 'Legendary',
  5: 'Mythic',
}

export interface ItemDefinition {
  id: string
  name: string
  category: ItemCategory
  rarity: Rarity
  stackSize: number
  subType?: EquipmentType | WeaponType | AttachmentType | ConsumableType | AmmoType | GrenadeType
  ammoType?: AmmoType
  attachmentSlots?: AttachmentType[]
}

export interface ItemInstance {
  id: string
  definitionId: string
  quantity: number
  attachments?: Record<AttachmentType, ItemInstance | null>
}

export type EquipmentSlotType = 'helmet' | 'bodyShield' | 'knockdownShield' | 'backpack'
export type WeaponSlotType = 'weapon1' | 'weapon2'

export interface InventoryState {
  equipment: Record<EquipmentSlotType, ItemInstance | null>
  weapons: Record<WeaponSlotType, ItemInstance | null>
  bag: (ItemInstance | null)[]
  bagSize: number
}

export interface ChestState {
  status: 'closed' | 'opening' | 'open' | 'empty'
  items: ItemInstance[]
}

export const DEFAULT_BAG_SIZE = 10
export const BAG_SIZE_BY_BACKPACK: Record<Rarity, number> = {
  1: 12,
  2: 14,
  3: 16,
  4: 16,
  5: 16,
}

export const STACK_SIZES: Record<string, number> = {
  ammo: 60,
  syringe: 4,
  shieldCell: 4,
  medkit: 2,
  shieldBattery: 2,
  grenade: 1,
}
