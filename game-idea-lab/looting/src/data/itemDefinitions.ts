import type { ItemDefinition, ItemInstance } from '../types/items'

export const ITEM_DEFINITIONS: Record<string, ItemDefinition> = {
  helmet_1: {
    id: 'helmet_1',
    name: 'Helmet LV1',
    category: 'equipment',
    subType: 'helmet',
    rarity: 1,
    stackSize: 1,
  },
  helmet_2: {
    id: 'helmet_2',
    name: 'Helmet LV2',
    category: 'equipment',
    subType: 'helmet',
    rarity: 2,
    stackSize: 1,
  },
  helmet_3: {
    id: 'helmet_3',
    name: 'Helmet LV3',
    category: 'equipment',
    subType: 'helmet',
    rarity: 3,
    stackSize: 1,
  },
  helmet_4: {
    id: 'helmet_4',
    name: 'Helmet LV4',
    category: 'equipment',
    subType: 'helmet',
    rarity: 4,
    stackSize: 1,
  },

  bodyShield_1: {
    id: 'bodyShield_1',
    name: '아머 LV1',
    category: 'equipment',
    subType: 'bodyShield',
    rarity: 1,
    stackSize: 1,
  },
  bodyShield_2: {
    id: 'bodyShield_2',
    name: '아머 LV2',
    category: 'equipment',
    subType: 'bodyShield',
    rarity: 2,
    stackSize: 1,
  },
  bodyShield_3: {
    id: 'bodyShield_3',
    name: '아머 LV3',
    category: 'equipment',
    subType: 'bodyShield',
    rarity: 3,
    stackSize: 1,
  },
  bodyShield_4: {
    id: 'bodyShield_4',
    name: '아머 LV4',
    category: 'equipment',
    subType: 'bodyShield',
    rarity: 4,
    stackSize: 1,
  },
  bodyShield_5: {
    id: 'bodyShield_5',
    name: '아머 LV5',
    category: 'equipment',
    subType: 'bodyShield',
    rarity: 5,
    stackSize: 1,
  },

  knockdownShield_1: {
    id: 'knockdownShield_1',
    name: 'Knockdown Shield LV1',
    category: 'equipment',
    subType: 'knockdownShield',
    rarity: 1,
    stackSize: 1,
  },
  knockdownShield_2: {
    id: 'knockdownShield_2',
    name: 'Knockdown Shield LV2',
    category: 'equipment',
    subType: 'knockdownShield',
    rarity: 2,
    stackSize: 1,
  },
  knockdownShield_3: {
    id: 'knockdownShield_3',
    name: 'Knockdown Shield LV3',
    category: 'equipment',
    subType: 'knockdownShield',
    rarity: 3,
    stackSize: 1,
  },
  knockdownShield_4: {
    id: 'knockdownShield_4',
    name: 'Knockdown Shield LV4',
    category: 'equipment',
    subType: 'knockdownShield',
    rarity: 4,
    stackSize: 1,
  },

  backpack_1: {
    id: 'backpack_1',
    name: 'Backpack LV1',
    category: 'equipment',
    subType: 'backpack',
    rarity: 1,
    stackSize: 1,
  },
  backpack_2: {
    id: 'backpack_2',
    name: 'Backpack LV2',
    category: 'equipment',
    subType: 'backpack',
    rarity: 2,
    stackSize: 1,
  },
  backpack_3: {
    id: 'backpack_3',
    name: 'Backpack LV3',
    category: 'equipment',
    subType: 'backpack',
    rarity: 3,
    stackSize: 1,
  },
  backpack_4: {
    id: 'backpack_4',
    name: 'Backpack LV4',
    category: 'equipment',
    subType: 'backpack',
    rarity: 4,
    stackSize: 1,
  },

  weapon_ar: {
    id: 'weapon_ar',
    name: 'R-301',
    category: 'weapon',
    subType: 'ar',
    rarity: 1,
    stackSize: 1,
    ammoType: 'light',
    attachmentSlots: ['scope', 'extendedMag', 'barrel'],
  },
  weapon_smg: {
    id: 'weapon_smg',
    name: 'R-99',
    category: 'weapon',
    subType: 'smg',
    rarity: 1,
    stackSize: 1,
    ammoType: 'light',
    attachmentSlots: ['scope', 'extendedMag', 'barrel'],
  },
  weapon_shotgun: {
    id: 'weapon_shotgun',
    name: 'Peacekeeper',
    category: 'weapon',
    subType: 'shotgun',
    rarity: 1,
    stackSize: 1,
    ammoType: 'shotgun',
    attachmentSlots: ['scope'],
  },

  scope_1x: {
    id: 'scope_1x',
    name: '1x HCOG',
    category: 'attachment',
    subType: 'scope',
    rarity: 1,
    stackSize: 1,
  },
  scope_2x: {
    id: 'scope_2x',
    name: '2x HCOG',
    category: 'attachment',
    subType: 'scope',
    rarity: 2,
    stackSize: 1,
  },
  scope_3x: {
    id: 'scope_3x',
    name: '3x Ranger',
    category: 'attachment',
    subType: 'scope',
    rarity: 3,
    stackSize: 1,
  },

  extendedMag_1: {
    id: 'extendedMag_1',
    name: 'Mag LV1',
    category: 'attachment',
    subType: 'extendedMag',
    rarity: 1,
    stackSize: 1,
  },
  extendedMag_2: {
    id: 'extendedMag_2',
    name: 'Mag LV2',
    category: 'attachment',
    subType: 'extendedMag',
    rarity: 2,
    stackSize: 1,
  },
  extendedMag_3: {
    id: 'extendedMag_3',
    name: 'Mag LV3',
    category: 'attachment',
    subType: 'extendedMag',
    rarity: 3,
    stackSize: 1,
  },

  barrel_1: {
    id: 'barrel_1',
    name: 'Barrel LV1',
    category: 'attachment',
    subType: 'barrel',
    rarity: 1,
    stackSize: 1,
  },
  barrel_2: {
    id: 'barrel_2',
    name: 'Barrel LV2',
    category: 'attachment',
    subType: 'barrel',
    rarity: 2,
    stackSize: 1,
  },
  barrel_3: {
    id: 'barrel_3',
    name: 'Barrel LV3',
    category: 'attachment',
    subType: 'barrel',
    rarity: 3,
    stackSize: 1,
  },

  syringe: {
    id: 'syringe',
    name: 'Syringe',
    category: 'consumable',
    subType: 'syringe',
    rarity: 1,
    stackSize: 4,
  },
  medkit: {
    id: 'medkit',
    name: 'Medkit',
    category: 'consumable',
    subType: 'medkit',
    rarity: 2,
    stackSize: 2,
  },
  shieldCell: {
    id: 'shieldCell',
    name: 'Shield Cell',
    category: 'consumable',
    subType: 'shieldCell',
    rarity: 1,
    stackSize: 4,
  },
  shieldBattery: {
    id: 'shieldBattery',
    name: 'Shield Battery',
    category: 'consumable',
    subType: 'shieldBattery',
    rarity: 2,
    stackSize: 2,
  },

  ammo_light: {
    id: 'ammo_light',
    name: 'Light Ammo',
    category: 'ammo',
    subType: 'light',
    rarity: 1,
    stackSize: 60,
  },
  ammo_heavy: {
    id: 'ammo_heavy',
    name: 'Heavy Ammo',
    category: 'ammo',
    subType: 'heavy',
    rarity: 1,
    stackSize: 60,
  },
  ammo_energy: {
    id: 'ammo_energy',
    name: 'Energy Ammo',
    category: 'ammo',
    subType: 'energy',
    rarity: 1,
    stackSize: 60,
  },
  ammo_shotgun: {
    id: 'ammo_shotgun',
    name: 'Shotgun Ammo',
    category: 'ammo',
    subType: 'shotgun',
    rarity: 1,
    stackSize: 60,
  },

  grenade_frag: {
    id: 'grenade_frag',
    name: 'Frag Grenade',
    category: 'grenade',
    subType: 'frag',
    rarity: 1,
    stackSize: 1,
  },
  grenade_arcStar: {
    id: 'grenade_arcStar',
    name: 'Arc Star',
    category: 'grenade',
    subType: 'arcStar',
    rarity: 2,
    stackSize: 1,
  },
  grenade_thermite: {
    id: 'grenade_thermite',
    name: 'Thermite Grenade',
    category: 'grenade',
    subType: 'thermite',
    rarity: 1,
    stackSize: 1,
  },
}

export function getItemDefinition(id: string): ItemDefinition | undefined {
  return ITEM_DEFINITIONS[id]
}

export function getItemsByCategory(category: string): ItemDefinition[] {
  return Object.values(ITEM_DEFINITIONS).filter(item => item.category === category)
}

export function getEquipmentByType(type: string): ItemDefinition[] {
  return Object.values(ITEM_DEFINITIONS).filter(
    item => item.category === 'equipment' && item.subType === type
  )
}

export function getRandomItems(count: number): ItemDefinition[] {
  const allItems = Object.values(ITEM_DEFINITIONS)
  const result: ItemDefinition[] = []

  for (let i = 0; i < count; i++) {
    const randomIndex = Math.floor(Math.random() * allItems.length)
    result.push(allItems[randomIndex])
  }

  return result
}

export function createItemInstance(definitionId: string, quantity: number = 1): ItemInstance {
  const definition = getItemDefinition(definitionId)
  const instance: ItemInstance = {
    id: `${definitionId}_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
    definitionId,
    quantity: Math.min(quantity, definition?.stackSize ?? 1),
  }

  if (definition?.category === 'weapon' && definition.attachmentSlots) {
    instance.attachments = {
      scope: null,
      extendedMag: null,
      barrel: null,
    }
  }

  return instance
}
