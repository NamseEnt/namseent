import { createCanvas, loadImage } from 'canvas';
import fs from 'fs';
import path from 'path';

const ASSET_DIR = path.join(import.meta.dirname, '..', 'asset', 'image');
const OUTPUT_PNG = path.join(ASSET_DIR, 'particle_atlas.png');
const OUTPUT_RS = path.join(import.meta.dirname, '..', 'src', 'game_state', 'field_particle', 'atlas.rs');

const CELL = 128;
const LINE_H = 16;
const ATLAS_W = 2048;
const ATLAS_H = 2048;

const canvas = createCanvas(ATLAS_W, ATLAS_H);
const ctx = canvas.getContext('2d');

const sprites = {};
let cursorX = 0;
let cursorY = 0;
let rowMaxH = 0;

function alloc(name, w, h) {
  if (cursorX + w > ATLAS_W) {
    cursorX = 0;
    cursorY += rowMaxH;
    rowMaxH = 0;
  }
  const rect = { x: cursorX, y: cursorY, w, h };
  sprites[name] = rect;
  cursorX += w;
  rowMaxH = Math.max(rowMaxH, h);
  return rect;
}

function drawGlowCircle() {
  const r = alloc('GLOW_CIRCLE', CELL, CELL);
  const cx = r.x + CELL / 2, cy = r.y + CELL / 2;
  const grad = ctx.createRadialGradient(cx, cy, 0, cx, cy, CELL / 2);
  grad.addColorStop(0.0, 'rgba(255,255,255,1.0)');
  grad.addColorStop(0.3, 'rgba(255,255,255,0.8)');
  grad.addColorStop(0.6, 'rgba(255,255,255,0.4)');
  grad.addColorStop(1.0, 'rgba(255,255,255,0.0)');
  ctx.fillStyle = grad;
  ctx.beginPath();
  ctx.arc(cx, cy, CELL / 2, 0, Math.PI * 2);
  ctx.fill();
}

function drawStarBurst() {
  const r = alloc('STAR_BURST', CELL, CELL);
  const cx = r.x + CELL / 2, cy = r.y + CELL / 2;
  const spikes = 8;
  const outerR = CELL / 2 - 4;
  const innerR = CELL / 6;
  ctx.fillStyle = 'white';
  ctx.beginPath();
  for (let i = 0; i < spikes * 2; i++) {
    const angle = (i / (spikes * 2)) * Math.PI * 2 - Math.PI / 2;
    const radius = i % 2 === 0 ? outerR : innerR;
    const x = cx + radius * Math.cos(angle);
    const y = cy + radius * Math.sin(angle);
    if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
  }
  ctx.closePath();
  ctx.fill();
}

function drawCross() {
  const r = alloc('CROSS', CELL, CELL);
  const cx = r.x + CELL / 2, cy = r.y + CELL / 2;
  const arm = CELL / 2 - 8;
  const thickness = 12;
  ctx.fillStyle = 'white';
  ctx.fillRect(cx - arm, cy - thickness / 2, arm * 2, thickness);
  ctx.fillRect(cx - thickness / 2, cy - arm, thickness, arm * 2);
}

function drawRing() {
  const r = alloc('RING', CELL, CELL);
  const cx = r.x + CELL / 2, cy = r.y + CELL / 2;
  ctx.strokeStyle = 'white';
  ctx.lineWidth = 6;
  ctx.beginPath();
  ctx.arc(cx, cy, CELL / 2 - 8, 0, Math.PI * 2);
  ctx.stroke();
}

function drawCapsuleLine() {
  const w = 1024, h = LINE_H;
  const r = alloc('CAPSULE_LINE', w, h);
  const cy = r.y + h / 2;
  const radius = h / 2;
  ctx.fillStyle = 'white';
  ctx.beginPath();
  ctx.moveTo(r.x + radius, r.y);
  ctx.lineTo(r.x + w - radius, r.y);
  ctx.arcTo(r.x + w, r.y, r.x + w, cy, radius);
  ctx.arcTo(r.x + w, r.y + h, r.x + w - radius, r.y + h, radius);
  ctx.lineTo(r.x + radius, r.y + h);
  ctx.arcTo(r.x, r.y + h, r.x, cy, radius);
  ctx.arcTo(r.x, r.y, r.x + radius, r.y, radius);
  ctx.closePath();
  ctx.fill();
}

function drawZigzagLine() {
  const w = 512, h = 32;
  const r = alloc('ZIGZAG_LINE', w, h);
  const segments = 8;
  const segW = w / segments;
  ctx.strokeStyle = 'white';
  ctx.lineWidth = 4;
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';
  ctx.beginPath();
  ctx.moveTo(r.x, r.y + h / 2);
  for (let i = 1; i <= segments; i++) {
    const x = r.x + i * segW;
    const y = r.y + (i % 2 === 1 ? 4 : h - 4);
    ctx.lineTo(x, y);
  }
  ctx.stroke();
}

async function drawImage(name, filePath, size = CELL) {
  const r = alloc(name, size, size);
  try {
    const img = await loadImage(filePath);
    ctx.drawImage(img, r.x, r.y, size, size);
  } catch (e) {
    ctx.fillStyle = 'magenta';
    ctx.fillRect(r.x, r.y, size, size);
    console.warn(`Failed to load ${filePath}: ${e.message}`);
  }
}

async function main() {
  drawGlowCircle();
  drawStarBurst();
  drawCross();
  drawRing();
  drawCapsuleLine();
  drawZigzagLine();

  for (let i = 1; i <= 4; i++) {
    const num = String(i).padStart(2, '0');
    await drawImage(`TRASH_${num}`, path.join(ASSET_DIR, 'attack', 'projectile', `trash_${num}.png`));
  }

  for (let i = 1; i <= 15; i++) {
    const num = String(i).padStart(2, '0');
    await drawImage(`MOB${num}`, path.join(ASSET_DIR, 'monster', `mob${num}.png`));
  }

  for (let i = 1; i <= 11; i++) {
    const num = String(i).padStart(2, '0');
    await drawImage(`BOSS${num}`, path.join(ASSET_DIR, 'monster', `boss${num}.png`));
  }

  await drawImage('MONSTER_SOUL', path.join(ASSET_DIR, 'monster_soul.png'));

  const iconFiles = [
    'accept', 'add', 'attack_damage', 'attack_range', 'attack_speed',
    'card', 'config', 'down', 'enemy_boss', 'enemy_named', 'enemy_normal',
    'gold', 'health', 'invincible', 'item', 'level', 'lock', 'move_speed',
    'multiply', 'new', 'quest', 'rarity_common', 'rarity_epic',
    'rarity_legendary', 'rarity_rare', 'rating', 'refresh', 'reject',
    'shield', 'shop', 'speaker', 'suit_clubs', 'suit_diamonds',
    'suit_hearts', 'suit_spades', 'up',
  ];
  for (const name of iconFiles) {
    const constName = 'ICON_' + name.toUpperCase();
    await drawImage(constName, path.join(ASSET_DIR, 'icon', `${name}.png`));
  }

  const buf = canvas.toBuffer('image/png');
  fs.writeFileSync(OUTPUT_PNG, buf);
  console.log(`Atlas written to ${OUTPUT_PNG}`);

  let rs = `use namui::*;\n\n`;
  rs += `pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect<Px> {\n`;
  rs += `    Rect::Xywh { x: px(x), y: px(y), width: px(w), height: px(h) }\n`;
  rs += `}\n\n`;

  for (const [name, r] of Object.entries(sprites)) {
    rs += `pub fn ${name.toLowerCase()}() -> Rect<Px> { rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0) }\n`;
  }

  rs += `\npub fn projectile_rect(kind: crate::game_state::projectile::ProjectileKind) -> Rect<Px> {\n`;
  rs += `    use crate::game_state::projectile::ProjectileKind;\n`;
  rs += `    match kind {\n`;
  for (let i = 1; i <= 4; i++) {
    const num = String(i).padStart(2, '0');
    const r = sprites[`TRASH_${num}`];
    rs += `        ProjectileKind::Trash${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
  }
  rs += `    }\n}\n`;

  rs += `\npub fn monster_rect(kind: crate::game_state::MonsterKind) -> Rect<Px> {\n`;
  rs += `    use crate::game_state::MonsterKind;\n`;
  rs += `    match kind {\n`;
  for (let i = 1; i <= 15; i++) {
    const num = String(i).padStart(2, '0');
    const r = sprites[`MOB${num}`];
    rs += `        MonsterKind::Mob${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
  }
  const mob15 = sprites['MOB15'];
  for (let i = 16; i <= 50; i++) {
    const num = String(i).padStart(2, '0');
    rs += `        MonsterKind::Mob${num} => rect(${mob15.x}.0, ${mob15.y}.0, ${mob15.w}.0, ${mob15.h}.0),\n`;
  }
  for (let i = 1; i <= 11; i++) {
    const num = String(i).padStart(2, '0');
    const r = sprites[`BOSS${num}`];
    rs += `        MonsterKind::Boss${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
  }
  rs += `    }\n}\n`;

  const iconMapping = {
    'Accept': 'ICON_ACCEPT',
    'AttackDamage': 'ICON_ATTACK_DAMAGE',
    'AttackRange': 'ICON_ATTACK_RANGE',
    'AttackSpeed': 'ICON_ATTACK_SPEED',
    'Config': 'ICON_CONFIG',
    'EnemyBoss': 'ICON_ENEMY_BOSS',
    'EnemyNamed': 'ICON_ENEMY_NAMED',
    'EnemyNormal': 'ICON_ENEMY_NORMAL',
    'Gold': 'ICON_GOLD',
    'Health': 'ICON_HEALTH',
    'Invincible': 'ICON_INVINCIBLE',
    'Item': 'ICON_ITEM',
    'Level': 'ICON_LEVEL',
    'Lock': 'ICON_LOCK',
    'MoveSpeed': 'ICON_MOVE_SPEED',
    'Contract': 'ICON_QUEST',
    'Refresh': 'ICON_REFRESH',
    'Reject': 'ICON_REJECT',
    'Shield': 'ICON_SHIELD',
    'Shop': 'ICON_SHOP',
    'Speaker': 'ICON_SPEAKER',
    'Up': 'ICON_UP',
    'Down': 'ICON_DOWN',
    'Card': 'ICON_CARD',
    'New': 'ICON_NEW',
    'Add': 'ICON_ADD',
    'Multiply': 'ICON_MULTIPLY',
    'Rating': 'ICON_RATING',
  };
  const suitMapping = {
    'Spades': 'ICON_SUIT_SPADES',
    'Hearts': 'ICON_SUIT_HEARTS',
    'Diamonds': 'ICON_SUIT_DIAMONDS',
    'Clubs': 'ICON_SUIT_CLUBS',
  };
  const rarityMapping = {
    'Common': 'ICON_RARITY_COMMON',
    'Rare': 'ICON_RARITY_RARE',
    'Epic': 'ICON_RARITY_EPIC',
    'Legendary': 'ICON_RARITY_LEGENDARY',
  };

  rs += `\npub fn icon_rect(kind: &crate::icon::IconKind) -> Rect<Px> {\n`;
  rs += `    use crate::icon::IconKind;\n`;
  rs += `    match kind {\n`;
  for (const [variant, spriteName] of Object.entries(iconMapping)) {
    const r = sprites[spriteName];
    rs += `        IconKind::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
  }
  rs += `        IconKind::Suit { suit } => match suit {\n`;
  for (const [variant, spriteName] of Object.entries(suitMapping)) {
    const r = sprites[spriteName];
    rs += `            crate::Suit::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
  }
  rs += `        },\n`;
  rs += `        IconKind::Rarity { rarity } => match rarity {\n`;
  for (const [variant, spriteName] of Object.entries(rarityMapping)) {
    const r = sprites[spriteName];
    rs += `            crate::Rarity::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
  }
  rs += `        },\n`;
  rs += `    }\n}\n`;

  fs.writeFileSync(OUTPUT_RS, rs);
  console.log(`Rust constants written to ${OUTPUT_RS}`);
  console.log('Sprite map:', JSON.stringify(sprites, null, 2));
}

main().catch(console.error);
