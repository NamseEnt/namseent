import type { Atlas } from "../types.ts";
import { spriteOrThrow } from "../types.ts";

export function appendShapeAndMonsterSection(
    rs: string,
    monsters: Atlas,
): string {
    const ms = monsters.sprites;
    const monsterSoul = spriteOrThrow(ms, "MONSTER_SOUL");
    rs += `pub fn monster_soul() -> Rect<Px> { rect(${monsterSoul.x}.0, ${monsterSoul.y}.0, ${monsterSoul.w}.0, ${monsterSoul.h}.0) }\n`;

    rs += `\npub fn monster_rect(kind: crate::game_state::MonsterKind) -> Rect<Px> {\n`;
    rs += `    use crate::game_state::MonsterKind;\n`;
    rs += `    match kind {\n`;
    for (let i = 1; i <= 50; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ms, `MOB${num}`);
        rs += `        MonsterKind::Mob${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    for (let i = 1; i <= 14; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ms, `BOSS${num}`);
        rs += `        MonsterKind::Boss${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `    }\n}\n`;

    return rs;
}
