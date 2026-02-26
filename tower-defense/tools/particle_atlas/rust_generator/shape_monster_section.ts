import type { Atlas } from "../types.ts";
import { spriteOrThrow } from "../types.ts";

export function appendShapeAndMonsterSection(
    rs: string,
    attack: Atlas,
    monsters: Atlas,
): string {
    const burningTail = spriteOrThrow(attack.sprites, "BURNING_TAIL");
    const emberSpark = spriteOrThrow(attack.sprites, "EMBER_SPARK");
    const blueSpark = spriteOrThrow(attack.sprites, "BLUE_SPARK");
    const lightningBolt = spriteOrThrow(attack.sprites, "LIGHTNING_BOLT");
    const sparkle = spriteOrThrow(attack.sprites, "SPARKLE");
    const windCurveTrail = spriteOrThrow(attack.sprites, "WIND_CURVE_TRAIL");
    const blackSmoke = spriteOrThrow(attack.sprites, "BLACK_SMOKE");
    rs += `pub fn burning_tail() -> Rect<Px> { rect(${burningTail.x}.0, ${burningTail.y}.0, ${burningTail.w}.0, ${burningTail.h}.0) }\n`;
    rs += `pub fn ember_spark() -> Rect<Px> { rect(${emberSpark.x}.0, ${emberSpark.y}.0, ${emberSpark.w}.0, ${emberSpark.h}.0) }\n`;
    rs += `pub fn blue_spark() -> Rect<Px> { rect(${blueSpark.x}.0, ${blueSpark.y}.0, ${blueSpark.w}.0, ${blueSpark.h}.0) }\n`;
    rs += `pub fn lightning_bolt_rect() -> Rect<Px> { rect(${lightningBolt.x}.0, ${lightningBolt.y}.0, ${lightningBolt.w}.0, ${lightningBolt.h}.0) }\n`;
    rs += `pub fn sparkle() -> Rect<Px> { rect(${sparkle.x}.0, ${sparkle.y}.0, ${sparkle.w}.0, ${sparkle.h}.0) }\n`;
    rs += `pub fn wind_curve_trail() -> Rect<Px> { rect(${windCurveTrail.x}.0, ${windCurveTrail.y}.0, ${windCurveTrail.w}.0, ${windCurveTrail.h}.0) }\n`;
    rs += `pub fn black_smoke() -> Rect<Px> { rect(${blackSmoke.x}.0, ${blackSmoke.y}.0, ${blackSmoke.w}.0, ${blackSmoke.h}.0) }\n`;

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
