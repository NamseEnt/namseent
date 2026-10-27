import path from "path";

import { createAtlas } from "./atlas.ts";
import { ASSET_DIR, CELL, ROW_W } from "./constants.ts";
import { drawImage, drawImageRect } from "./draw.ts";
import type { Atlas } from "./types.ts";

export async function createMonstersAtlas(): Promise<Atlas> {
    const mobCount = 50;
    const bossCount = 14;
    const monsterSpriteCount = mobCount + bossCount;
    const monstersPerRow = ROW_W / CELL;
    const monsterRows = Math.ceil(monsterSpriteCount / monstersPerRow);
    const monsterSoulHeight = 192;
    const monsters = createAtlas(
        "monsters",
        ROW_W,
        monsterRows * CELL + monsterSoulHeight,
    );
    for (let i = 1; i <= mobCount; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            monsters,
            `MOB${num}`,
            path.join(ASSET_DIR, "monster", `mob_${num}.png`),
        );
    }
    for (let i = 1; i <= bossCount; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            monsters,
            `BOSS${num}`,
            path.join(ASSET_DIR, "monster", `boss_${num}.png`),
        );
    }
    await drawImageRect(
        monsters,
        "MONSTER_SOUL",
        path.join(ASSET_DIR, "monster_soul.png"),
        128,
        192,
    );
    return monsters;
}

export async function createIconsAtlas(): Promise<Atlas> {
    const iconFiles = [
        "accept",
        "add",
        "damage",
        "card",
        "config",
        "down",
        "enemy_boss",
        "enemy_named",
        "enemy_normal",
        "gold",
        "health",
        "invincible",
        "item",
        "lock",
        "move_speed",
        "multiply",
        "new",
        "quest",
        "rarity_common",
        "rarity_epic",
        "rarity_legendary",
        "rarity_rare",
        "rating",
        "refresh",
        "reject",
        "shield",
        "shop",
        "speaker",
        "treasure",
        "suit_clubs",
        "suit_diamonds",
        "suit_hearts",
        "suit_spades",
        "up",
        "play",
        "warning",
        "deck",
    ];
    const icons = createAtlas("icons", ROW_W, CELL * 3);
    for (const name of iconFiles) {
        const constName = "ICON_" + name.toUpperCase();
        await drawImage(
            icons,
            constName,
            path.join(ASSET_DIR, "icon", `${name}.png`),
        );
    }
    return icons;
}

export async function createMapDecorationsAtlas(): Promise<Atlas> {
    const decorations = createAtlas("map_decorations", CELL * 8, 192);
    const decorationFiles = [
        "bush.PNG",
        "club.PNG",
        "dia.PNG",
        "flower.PNG",
        "heart.PNG",
        "mushroom.PNG",
        "rock.PNG",
        "spade.PNG",
    ];

    for (const fileName of decorationFiles) {
        const constName = fileName.replace(/\.[^.]+$/, "").toUpperCase();
        await drawImageRect(
            decorations,
            constName,
            path.join(ASSET_DIR, "map_decorations", fileName),
            CELL,
            192,
        );
    }

    return decorations;
}
