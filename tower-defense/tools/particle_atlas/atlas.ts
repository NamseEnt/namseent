import { createCanvas } from "canvas";
import fs from "fs";
import path from "path";

import { ASSET_DIR } from "./constants.ts";
import type { Atlas, SpriteMap, SpriteRect } from "./types.ts";

export function createAtlas(
    name: string,
    width: number,
    height: number,
): Atlas {
    const canvas = createCanvas(width, height);
    const ctx = canvas.getContext("2d");
    let cursorX = 0;
    let cursorY = 0;
    let rowMaxH = 0;
    const sprites: SpriteMap = {};

    function alloc(spriteName: string, w: number, h: number): SpriteRect {
        if (cursorX + w > width) {
            cursorX = 0;
            cursorY += rowMaxH;
            rowMaxH = 0;
        }
        const rect = { x: cursorX, y: cursorY, w, h };
        sprites[spriteName] = rect;
        cursorX += w;
        rowMaxH = Math.max(rowMaxH, h);
        return rect;
    }

    return { name, canvas, ctx, width, height, sprites, alloc };
}

export function saveAtlas(atlas: Atlas, filename: string): void {
    const buf = atlas.canvas.toBuffer("image/png");
    const outputPath = path.join(ASSET_DIR, filename);
    fs.writeFileSync(outputPath, buf);
    console.log(`Atlas written to ${outputPath}`);
}
