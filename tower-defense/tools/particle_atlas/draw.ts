import { loadImage } from "canvas";

import { CELL, LINE_H } from "./constants.ts";
import type { Atlas } from "./types.ts";
import { errorMessage } from "./types.ts";

export function drawGlowCircle(atlas: Atlas): void {
    const r = atlas.alloc("GLOW_CIRCLE", CELL, CELL);
    const cx = r.x + CELL / 2;
    const cy = r.y + CELL / 2;
    const grad = atlas.ctx.createRadialGradient(cx, cy, 0, cx, cy, CELL / 2);
    grad.addColorStop(0.0, "rgba(255,255,255,1.0)");
    grad.addColorStop(0.3, "rgba(255,255,255,0.8)");
    grad.addColorStop(0.6, "rgba(255,255,255,0.4)");
    grad.addColorStop(1.0, "rgba(255,255,255,0.0)");
    atlas.ctx.fillStyle = grad;
    atlas.ctx.beginPath();
    atlas.ctx.arc(cx, cy, CELL / 2, 0, Math.PI * 2);
    atlas.ctx.fill();
}

export function drawCapsuleLine(atlas: Atlas): void {
    const w = 1024;
    const h = LINE_H;
    const r = atlas.alloc("CAPSULE_LINE", w, h);
    const cy = r.y + h / 2;
    const radius = h / 2;
    atlas.ctx.fillStyle = "white";
    atlas.ctx.beginPath();
    atlas.ctx.moveTo(r.x + radius, r.y);
    atlas.ctx.lineTo(r.x + w - radius, r.y);
    atlas.ctx.arcTo(r.x + w, r.y, r.x + w, cy, radius);
    atlas.ctx.arcTo(r.x + w, r.y + h, r.x + w - radius, r.y + h, radius);
    atlas.ctx.lineTo(r.x + radius, r.y + h);
    atlas.ctx.arcTo(r.x, r.y + h, r.x, cy, radius);
    atlas.ctx.arcTo(r.x, r.y, r.x + radius, r.y, radius);
    atlas.ctx.closePath();
    atlas.ctx.fill();
}

export async function drawImage(
    atlas: Atlas,
    name: string,
    filePath: string,
    size: number = CELL,
): Promise<void> {
    const r = atlas.alloc(name, size, size);
    try {
        const img = await loadImage(filePath);
        atlas.ctx.drawImage(img, r.x, r.y, size, size);
    } catch (error) {
        atlas.ctx.fillStyle = "magenta";
        atlas.ctx.fillRect(r.x, r.y, size, size);
        console.warn(`Failed to load ${filePath}: ${errorMessage(error)}`);
    }
}

export async function drawImageRect(
    atlas: Atlas,
    name: string,
    filePath: string,
    w: number,
    h: number,
): Promise<void> {
    const r = atlas.alloc(name, w, h);
    try {
        const img = await loadImage(filePath);
        atlas.ctx.drawImage(img, r.x, r.y, w, h);
    } catch (error) {
        atlas.ctx.fillStyle = "magenta";
        atlas.ctx.fillRect(r.x, r.y, w, h);
        console.warn(`Failed to load ${filePath}: ${errorMessage(error)}`);
    }
}
