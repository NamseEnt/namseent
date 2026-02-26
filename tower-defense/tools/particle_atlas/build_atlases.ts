import path from "path";

import { createAtlas } from "./atlas.ts";
import { ASSET_DIR, CELL, ROW_W } from "./constants.ts";
import { drawGlowCircle, drawImage, drawImageRect } from "./draw.ts";
import type { Atlas } from "./types.ts";

export async function createAttackAtlas(): Promise<Atlas> {
    const attack = createAtlas("attack", CELL * 11, CELL);
    await drawImage(
        attack,
        "BURNING_TAIL",
        path.join(ASSET_DIR, "attack", "particle", "burning_tail.png"),
    );
    await drawImage(
        attack,
        "EMBER_SPARK",
        path.join(ASSET_DIR, "attack", "particle", "ember_spark.png"),
    );
    await drawImage(
        attack,
        "BLUE_SPARK",
        path.join(ASSET_DIR, "attack", "particle", "blue_spark.png"),
    );
    await drawImageRect(
        attack,
        "LASER_LINE",
        path.join(ASSET_DIR, "attack", "particle", "laser_line.png"),
        256,
        64,
    );
    await drawImageRect(
        attack,
        "LIGHTNING_BOLT",
        path.join(ASSET_DIR, "attack", "particle", "lightning_bolt.png"),
        256,
        64,
    );
    await drawImage(
        attack,
        "SPARKLE",
        path.join(ASSET_DIR, "attack", "particle", "sparkle.png"),
    );
    await drawImageRect(
        attack,
        "WIND_CURVE_TRAIL",
        path.join(ASSET_DIR, "attack", "particle", "wind_curve_trail.png"),
        256,
        64,
    );
    await drawImage(
        attack,
        "BLACK_SMOKE",
        path.join(ASSET_DIR, "attack", "particle", "black_smoke.png"),
    );
    return attack;
}

export async function createProjectilesAtlas(): Promise<Atlas> {
    const projectiles = createAtlas("projectiles", ROW_W, CELL * 2);
    for (let i = 1; i <= 4; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            projectiles,
            `TRASH_${num}`,
            path.join(ASSET_DIR, "attack", "projectile", `trash_${num}.png`),
        );
    }
    for (let i = 0; i <= 4; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            projectiles,
            `GIRL_${num}`,
            path.join(ASSET_DIR, "attack", "projectile", `girl_${num}.png`),
        );
    }
    await drawImage(
        projectiles,
        "CARDS_00",
        path.join(ASSET_DIR, "attack", "projectile", "cards_00.png"),
    );
    await drawImage(
        projectiles,
        "HEART_PROJ_00",
        path.join(ASSET_DIR, "attack", "projectile", "heart_00.png"),
    );
    for (let i = 0; i <= 3; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            projectiles,
            `CARD_PARTICLE_${num}`,
            path.join(ASSET_DIR, "attack", "particle", `card_${num}.png`),
        );
    }
    for (let i = 0; i <= 2; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            projectiles,
            `HEART_PARTICLE_${num}`,
            path.join(ASSET_DIR, "attack", "particle", `heart_${num}.png`),
        );
    }
    await drawImage(
        projectiles,
        "PINK_SMOKE",
        path.join(ASSET_DIR, "attack", "particle", "pink_smoke.png"),
    );
    drawGlowCircle(projectiles);
    return projectiles;
}

export async function createMonstersAtlas(): Promise<Atlas> {
    const monsters = createAtlas("monsters", ROW_W, 320);
    for (let i = 1; i <= 15; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            monsters,
            `MOB${num}`,
            path.join(ASSET_DIR, "monster", `mob${num}.png`),
        );
    }
    for (let i = 1; i <= 11; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            monsters,
            `BOSS${num}`,
            path.join(ASSET_DIR, "monster", `boss${num}.png`),
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
        "attack_damage",
        "attack_range",
        "attack_speed",
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
        "level",
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
        "suit_clubs",
        "suit_diamonds",
        "suit_hearts",
        "suit_spades",
        "up",
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
