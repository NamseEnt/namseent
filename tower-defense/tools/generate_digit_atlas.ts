import { createCanvas, registerFont } from "canvas";
import fs from "fs";
import path from "path";

const ASSET_DIR = path.join(import.meta.dirname, "..", "asset", "image");
const FONT_PATH = path.join(
    import.meta.dirname,
    "..",
    "..",
    "namui",
    "namui-cli",
    "system_bundle",
    "font",
    "Ko",
    "NotoSansKR-Black.ttf",
);

registerFont(FONT_PATH, { family: "NotoSansKR", weight: "black" });

const CELL = 64;
const GLYPHS = [
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    ".",
    "k",
    "m",
    "b",
];
const WIDTH = CELL * GLYPHS.length;
const HEIGHT = CELL;

const canvas = createCanvas(WIDTH, HEIGHT);
const ctx = canvas.getContext("2d");

ctx.textAlign = "center";
ctx.textBaseline = "middle";
ctx.font = "bold 48px NotoSansKR";

for (const [i, glyph] of GLYPHS.entries()) {
    const cx = i * CELL + CELL / 2;
    const cy = CELL / 2;

    ctx.lineWidth = 6;
    ctx.strokeStyle = "black";
    ctx.strokeText(glyph, cx, cy);

    ctx.fillStyle = "white";
    ctx.fillText(glyph, cx, cy);
}

const buf = canvas.toBuffer("image/png");
const outputPath = path.join(ASSET_DIR, "particle_digits.png");
fs.writeFileSync(outputPath, buf);
console.log(`Digit atlas written to ${outputPath}`);
