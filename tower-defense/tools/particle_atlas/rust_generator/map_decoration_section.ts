import type { Atlas } from "../types.ts";
import { spriteOrThrow } from "../types.ts";

export function appendMapDecorationSection(
    rs: string,
    decorations: Atlas,
): string {
    const ds = decorations.sprites;
    const bush = spriteOrThrow(ds, "BUSH");
    const club = spriteOrThrow(ds, "CLUB");
    const dia = spriteOrThrow(ds, "DIA");
    const flower = spriteOrThrow(ds, "FLOWER");
    const heart = spriteOrThrow(ds, "HEART");
    const mushroom = spriteOrThrow(ds, "MUSHROOM");
    const rock = spriteOrThrow(ds, "ROCK");
    const spade = spriteOrThrow(ds, "SPADE");

    rs += `\npub fn decoration_rect(kind: crate::game_state::background::DecorationKind) -> Rect<Px> {\n`;
    rs += `    use crate::game_state::background::DecorationKind;\n`;
    rs += `    match kind {\n`;
    rs += `        DecorationKind::Bush => rect(${bush.x}.0, ${bush.y}.0, ${bush.w}.0, ${bush.h}.0),\n`;
    rs += `        DecorationKind::Club => rect(${club.x}.0, ${club.y}.0, ${club.w}.0, ${club.h}.0),\n`;
    rs += `        DecorationKind::Dia => rect(${dia.x}.0, ${dia.y}.0, ${dia.w}.0, ${dia.h}.0),\n`;
    rs += `        DecorationKind::Flower => rect(${flower.x}.0, ${flower.y}.0, ${flower.w}.0, ${flower.h}.0),\n`;
    rs += `        DecorationKind::Heart => rect(${heart.x}.0, ${heart.y}.0, ${heart.w}.0, ${heart.h}.0),\n`;
    rs += `        DecorationKind::Mushroom => rect(${mushroom.x}.0, ${mushroom.y}.0, ${mushroom.w}.0, ${mushroom.h}.0),\n`;
    rs += `        DecorationKind::Rock => rect(${rock.x}.0, ${rock.y}.0, ${rock.w}.0, ${rock.h}.0),\n`;
    rs += `        DecorationKind::Spade => rect(${spade.x}.0, ${spade.y}.0, ${spade.w}.0, ${spade.h}.0),\n`;
    rs += `    }\n`;
    rs += `}\n`;

    return rs;
}
