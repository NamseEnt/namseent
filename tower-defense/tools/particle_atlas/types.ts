import type { Canvas, CanvasRenderingContext2D } from "canvas";

export type SpriteRect = { x: number; y: number; w: number; h: number };
export type SpriteMap = Record<string, SpriteRect>;

export type Atlas = {
    name: string;
    canvas: Canvas;
    ctx: CanvasRenderingContext2D;
    width: number;
    height: number;
    sprites: SpriteMap;
    alloc: (spriteName: string, w: number, h: number) => SpriteRect;
};

export function spriteOrThrow(
    sprites: SpriteMap,
    spriteName: string,
): SpriteRect {
    const sprite = sprites[spriteName];
    if (!sprite) {
        throw new Error(`Missing sprite: ${spriteName}`);
    }
    return sprite;
}

export function errorMessage(error: unknown): string {
    if (error instanceof Error) {
        return error.message;
    }
    return String(error);
}
