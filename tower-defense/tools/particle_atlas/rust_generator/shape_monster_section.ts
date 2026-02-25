import type { Atlas } from "../types.ts";
import { spriteOrThrow } from "../types.ts";

export function appendShapeAndMonsterSection(
    rs: string,
    shapes: Atlas,
    attack: Atlas,
    monsters: Atlas,
): string {
    const sh = shapes.sprites;
    const glowCircle = spriteOrThrow(sh, "GLOW_CIRCLE");
    const starBurst = spriteOrThrow(sh, "STAR_BURST");
    const cross = spriteOrThrow(sh, "CROSS");
    const ring = spriteOrThrow(sh, "RING");
    const blueSpark = spriteOrThrow(attack.sprites, "BLUE_SPARK");
    rs += `pub fn glow_circle() -> Rect<Px> { rect(${glowCircle.x}.0, ${glowCircle.y}.0, ${glowCircle.w}.0, ${glowCircle.h}.0) }\n`;
    rs += `pub fn star_burst() -> Rect<Px> { rect(${starBurst.x}.0, ${starBurst.y}.0, ${starBurst.w}.0, ${starBurst.h}.0) }\n`;
    rs += `pub fn cross() -> Rect<Px> { rect(${cross.x}.0, ${cross.y}.0, ${cross.w}.0, ${cross.h}.0) }\n`;
    rs += `pub fn ring() -> Rect<Px> { rect(${ring.x}.0, ${ring.y}.0, ${ring.w}.0, ${ring.h}.0) }\n`;
    rs += `pub fn blue_spark() -> Rect<Px> { rect(${blueSpark.x}.0, ${blueSpark.y}.0, ${blueSpark.w}.0, ${blueSpark.h}.0) }\n`;

    const ms = monsters.sprites;
    const monsterSoul = spriteOrThrow(ms, "MONSTER_SOUL");
    rs += `pub fn monster_soul() -> Rect<Px> { rect(${monsterSoul.x}.0, ${monsterSoul.y}.0, ${monsterSoul.w}.0, ${monsterSoul.h}.0) }\n`;

    rs += `\npub fn monster_rect(kind: crate::game_state::MonsterKind) -> Rect<Px> {\n`;
    rs += `    use crate::game_state::MonsterKind;\n`;
    rs += `    match kind {\n`;
    for (let i = 1; i <= 15; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ms, `MOB${num}`);
        rs += `        MonsterKind::Mob${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    const mob15 = spriteOrThrow(ms, "MOB15");
    for (let i = 16; i <= 50; i++) {
        const num = String(i).padStart(2, "0");
        rs += `        MonsterKind::Mob${num} => rect(${mob15.x}.0, ${mob15.y}.0, ${mob15.w}.0, ${mob15.h}.0),\n`;
    }
    for (let i = 1; i <= 11; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ms, `BOSS${num}`);
        rs += `        MonsterKind::Boss${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `    }\n}\n`;

    return rs;
}
