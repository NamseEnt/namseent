import fs from "fs";

import { OUTPUT_MAP_DECORATION_RS, OUTPUT_RS } from "./constants.ts";
import type { Atlas } from "./types.ts";
import { spriteOrThrow } from "./types.ts";
import { appendBaseSection } from "./rust_generator/base_section.ts";
import { appendIconAndDigitSection } from "./rust_generator/icon_digit_section.ts";
import { appendMapDecorationSection } from "./rust_generator/map_decoration_section.ts";
import { appendShapeAndMonsterSection } from "./rust_generator/shape_monster_section.ts";

export function generateRust(
    monsters: Atlas,
    icons: Atlas,
    decorations: Atlas,
): void {
    let rs = "";
    rs = appendBaseSection(rs);
    rs = appendShapeAndMonsterSection(rs, monsters);
    rs = appendIconAndDigitSection(rs, icons);

    fs.writeFileSync(OUTPUT_RS, rs);
    console.log(`Rust constants written to ${OUTPUT_RS}`);

    const mapRs = appendMapDecorationSection(
        appendBaseSection(""),
        decorations,
    );
    fs.writeFileSync(OUTPUT_MAP_DECORATION_RS, mapRs);
    console.log(`Rust constants written to ${OUTPUT_MAP_DECORATION_RS}`);

    console.log("Monster sprites:", JSON.stringify(monsters.sprites, null, 2));
    console.log("Icon sprites:", JSON.stringify(icons.sprites, null, 2));
}
