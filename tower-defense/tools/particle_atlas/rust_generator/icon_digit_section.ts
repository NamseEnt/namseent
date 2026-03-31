import type { Atlas } from "../types.ts";
import { spriteOrThrow } from "../types.ts";

export function appendIconAndDigitSection(rs: string, icons: Atlas): string {
    const iconMapping = {
        Accept: "ICON_ACCEPT",
        AttackDamage: "ICON_ATTACK_DAMAGE",
        Config: "ICON_CONFIG",
        EnemyBoss: "ICON_ENEMY_BOSS",
        EnemyNamed: "ICON_ENEMY_NAMED",
        EnemyNormal: "ICON_ENEMY_NORMAL",
        Gold: "ICON_GOLD",
        Health: "ICON_HEALTH",
        Invincible: "ICON_INVINCIBLE",
        Item: "ICON_ITEM",
        Level: "ICON_LEVEL",
        Lock: "ICON_LOCK",
        MoveSpeed: "ICON_MOVE_SPEED",
        Contract: "ICON_QUEST",
        Refresh: "ICON_REFRESH",
        Reject: "ICON_REJECT",
        Shield: "ICON_SHIELD",
        Shop: "ICON_SHOP",
        Speaker: "ICON_SPEAKER",
        Treasure: "ICON_TREASURE",
        Up: "ICON_UP",
        Down: "ICON_DOWN",
        Card: "ICON_CARD",
        New: "ICON_NEW",
        Add: "ICON_ADD",
        Multiply: "ICON_MULTIPLY",
        Rating: "ICON_RATING",
    };
    const suitMapping = {
        Spades: "ICON_SUIT_SPADES",
        Hearts: "ICON_SUIT_HEARTS",
        Diamonds: "ICON_SUIT_DIAMONDS",
        Clubs: "ICON_SUIT_CLUBS",
    };
    const rarityMapping = {
        Common: "ICON_RARITY_COMMON",
        Rare: "ICON_RARITY_RARE",
        Epic: "ICON_RARITY_EPIC",
        Legendary: "ICON_RARITY_LEGENDARY",
    };

    const ic = icons.sprites;
    rs += `\npub fn icon_rect(kind: &crate::icon::IconKind) -> Rect<Px> {\n`;
    rs += `    use crate::icon::IconKind;\n`;
    rs += `    match kind {\n`;
    for (const [variant, spriteName] of Object.entries(iconMapping)) {
        const r = spriteOrThrow(ic, spriteName);
        rs += `        IconKind::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `        IconKind::Suit { suit } => match suit {\n`;
    for (const [variant, spriteName] of Object.entries(suitMapping)) {
        const r = spriteOrThrow(ic, spriteName);
        rs += `            crate::Suit::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `        },\n`;
    rs += `        IconKind::Rarity { rarity } => match rarity {\n`;
    for (const [variant, spriteName] of Object.entries(rarityMapping)) {
        const r = spriteOrThrow(ic, spriteName);
        rs += `            crate::Rarity::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `        },\n`;
    rs += `    }\n}\n`;

    rs += `\npub fn digit_rect(ch: u8) -> Rect<Px> {\n`;
    rs += `    let idx = match ch {\n`;
    rs += `        b'0'..=b'9' => (ch - b'0') as f32,\n`;
    rs += `        b'.' => 10.0,\n`;
    rs += `        b'k' => 11.0,\n`;
    rs += `        b'm' => 12.0,\n`;
    rs += `        b'b' => 13.0,\n`;
    rs += `        _ => 0.0,\n`;
    rs += `    };\n`;
    rs += `    rect(idx * 64.0, 0.0, 64.0, 64.0)\n`;
    rs += `}\n`;

    return rs;
}
