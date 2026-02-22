import { createCanvas } from 'canvas';
import fs from 'fs';
import path from 'path';

const ASSET_DIR = path.join(import.meta.dirname, '..', 'asset', 'image');
const SIZE = 128;
const canvas = createCanvas(SIZE, SIZE);
const ctx = canvas.getContext('2d');

const cx = SIZE / 2;
const cy = SIZE / 2;
const radius = SIZE / 2;

const imageData = ctx.createImageData(SIZE, SIZE);
const data = imageData.data;

for (let y = 0; y < SIZE; y++) {
  for (let x = 0; x < SIZE; x++) {
    const dx = x - cx;
    const dy = y - cy;
    const dist = Math.sqrt(dx * dx + dy * dy);

    const idx = (y * SIZE + x) * 4;
    if (dist < radius) {
      data[idx + 0] = 255;
      data[idx + 1] = 255;
      data[idx + 2] = 255;
      data[idx + 3] = 255;
    } else {
      data[idx + 0] = 0;
      data[idx + 1] = 0;
      data[idx + 2] = 0;
      data[idx + 3] = 0;
    }
  }
}

ctx.putImageData(imageData, 0, 0);

const buf = canvas.toBuffer('image/png');
const outputPath = path.join(ASSET_DIR, 'particle_burning_trail.png');
fs.writeFileSync(outputPath, buf);
console.log(`Written to ${outputPath}`);
