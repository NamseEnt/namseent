import fs from "fs";

import { OUTPUT_RS } from "./constants.ts";
import type { Atlas } from "./types.ts";
import { spriteOrThrow } from "./types.ts";
import { appendBaseSection } from "./rust_generator/base_section.ts";
import { appendIconAndDigitSection } from "./rust_generator/icon_digit_section.ts";
import { appendProjectileSection } from "./rust_generator/projectile_section.ts";
import { appendShapeAndMonsterSection } from "./rust_generator/shape_monster_section.ts";

export function generateRust(
    attack: Atlas,
    projectiles: Atlas,
    monsters: Atlas,
    icons: Atlas,
): void {
    let rs = "";
    rs = appendBaseSection(rs);
    {
        const laserLine = spriteOrThrow(attack.sprites, "LASER_LINE");
        rs += `pub fn laser_line_rect() -> Rect<Px> { rect(${laserLine.x}.0, ${laserLine.y}.0, ${laserLine.w}.0, ${laserLine.h}.0) }\n`;
    }
    rs = appendShapeAndMonsterSection(rs, attack, monsters);
    rs = appendProjectileSection(rs, projectiles);
    rs = appendIconAndDigitSection(rs, icons);

    fs.writeFileSync(OUTPUT_RS, rs);
    console.log(`Rust constants written to ${OUTPUT_RS}`);

    console.log("Attack sprites:", JSON.stringify(attack.sprites, null, 2));
    console.log(
        "Projectile sprites:",
        JSON.stringify(projectiles.sprites, null, 2),
    );
    console.log("Monster sprites:", JSON.stringify(monsters.sprites, null, 2));
    console.log("Icon sprites:", JSON.stringify(icons.sprites, null, 2));
}
