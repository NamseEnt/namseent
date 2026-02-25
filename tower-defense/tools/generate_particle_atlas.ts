import { createCanvas, loadImage } from "canvas";
import type { Canvas, CanvasRenderingContext2D } from "canvas";
import fs from "fs";
import path from "path";

const ASSET_DIR = path.join(import.meta.dirname, "..", "asset", "image");
const OUTPUT_RS = path.join(
    import.meta.dirname,
    "..",
    "src",
    "game_state",
    "field_particle",
    "atlas.rs",
);

const CELL = 128;
const LINE_H = 16;
const ROW_W = 2048;

type SpriteRect = { x: number; y: number; w: number; h: number };
type SpriteMap = Record<string, SpriteRect>;

type Atlas = {
    name: string;
    canvas: Canvas;
    ctx: CanvasRenderingContext2D;
    width: number;
    height: number;
    sprites: SpriteMap;
    alloc: (spriteName: string, w: number, h: number) => SpriteRect;
};

function spriteOrThrow(sprites: SpriteMap, spriteName: string): SpriteRect {
    const sprite = sprites[spriteName];
    if (!sprite) {
        throw new Error(`Missing sprite: ${spriteName}`);
    }
    return sprite;
}

function errorMessage(error: unknown): string {
    if (error instanceof Error) {
        return error.message;
    }
    return String(error);
}

function createAtlas(name: string, width: number, height: number): Atlas {
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

function drawGlowCircle(atlas: Atlas): void {
    const r = atlas.alloc("GLOW_CIRCLE", CELL, CELL);
    const cx = r.x + CELL / 2,
        cy = r.y + CELL / 2;
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

function drawStarBurst(atlas: Atlas): void {
    const r = atlas.alloc("STAR_BURST", CELL, CELL);
    const cx = r.x + CELL / 2,
        cy = r.y + CELL / 2;
    const spikes = 8;
    const outerR = CELL / 2 - 4;
    const innerR = CELL / 6;
    atlas.ctx.fillStyle = "white";
    atlas.ctx.beginPath();
    for (let i = 0; i < spikes * 2; i++) {
        const angle = (i / (spikes * 2)) * Math.PI * 2 - Math.PI / 2;
        const radius = i % 2 === 0 ? outerR : innerR;
        const x = cx + radius * Math.cos(angle);
        const y = cy + radius * Math.sin(angle);
        if (i === 0) atlas.ctx.moveTo(x, y);
        else atlas.ctx.lineTo(x, y);
    }
    atlas.ctx.closePath();
    atlas.ctx.fill();
}

function drawCross(atlas: Atlas): void {
    const r = atlas.alloc("CROSS", CELL, CELL);
    const cx = r.x + CELL / 2,
        cy = r.y + CELL / 2;
    const arm = CELL / 2 - 8;
    const thickness = 12;
    atlas.ctx.fillStyle = "white";
    atlas.ctx.fillRect(cx - arm, cy - thickness / 2, arm * 2, thickness);
    atlas.ctx.fillRect(cx - thickness / 2, cy - arm, thickness, arm * 2);
}

function drawRing(atlas: Atlas): void {
    const r = atlas.alloc("RING", CELL, CELL);
    const cx = r.x + CELL / 2,
        cy = r.y + CELL / 2;
    atlas.ctx.strokeStyle = "white";
    atlas.ctx.lineWidth = 6;
    atlas.ctx.beginPath();
    atlas.ctx.arc(cx, cy, CELL / 2 - 8, 0, Math.PI * 2);
    atlas.ctx.stroke();
}

function drawCapsuleLine(atlas: Atlas): void {
    const w = 1024,
        h = LINE_H;
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

async function drawImage(
    atlas: Atlas,
    name: string,
    filePath: string,
    size: number = CELL,
): Promise<void> {
    const r = atlas.alloc(name, size, size);
    try {
        const img = await loadImage(filePath);
        atlas.ctx.drawImage(img, r.x, r.y, size, size);
    } catch (e) {
        atlas.ctx.fillStyle = "magenta";
        atlas.ctx.fillRect(r.x, r.y, size, size);
        console.warn(`Failed to load ${filePath}: ${errorMessage(e)}`);
    }
}

async function drawImageRect(
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
    } catch (e) {
        atlas.ctx.fillStyle = "magenta";
        atlas.ctx.fillRect(r.x, r.y, w, h);
        console.warn(`Failed to load ${filePath}: ${errorMessage(e)}`);
    }
}

function saveAtlas(atlas: Atlas, filename: string): void {
    const buf = atlas.canvas.toBuffer("image/png");
    const outputPath = path.join(ASSET_DIR, filename);
    fs.writeFileSync(outputPath, buf);
    console.log(`Atlas written to ${outputPath}`);
}

async function main() {
    const shapes = createAtlas("shapes", 512, CELL);
    drawGlowCircle(shapes);
    drawStarBurst(shapes);
    drawCross(shapes);
    drawRing(shapes);

    const line = createAtlas("line", 1024, LINE_H);
    drawCapsuleLine(line);

    const projectiles = createAtlas("projectiles", ROW_W, CELL * 2);
    for (let i = 1; i <= 4; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            projectiles,
            `TRASH_${num}`,
            path.join(ASSET_DIR, "attack", "projectile", `trash_${num}.png`),
        );
    }
    for (let i = 0; i <= 4; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            projectiles,
            `GIRL_${num}`,
            path.join(ASSET_DIR, "attack", "projectile", `girl_${num}.png`),
        );
    }
    await drawImage(
        projectiles,
        "CARDS_00",
        path.join(ASSET_DIR, "attack", "projectile", "cards_00.png"),
    );
    await drawImage(
        projectiles,
        "HEART_PROJ_00",
        path.join(ASSET_DIR, "attack", "projectile", "heart_00.png"),
    );
    for (let i = 0; i <= 3; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            projectiles,
            `CARD_PARTICLE_${num}`,
            path.join(ASSET_DIR, "attack", "particle", `card_${num}.png`),
        );
    }
    for (let i = 0; i <= 2; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            projectiles,
            `HEART_PARTICLE_${num}`,
            path.join(ASSET_DIR, "attack", "particle", `heart_${num}.png`),
        );
    }
    drawGlowCircle(projectiles);

    const monsters = createAtlas("monsters", ROW_W, 320);
    for (let i = 1; i <= 15; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            monsters,
            `MOB${num}`,
            path.join(ASSET_DIR, "monster", `mob${num}.png`),
        );
    }
    for (let i = 1; i <= 11; i++) {
        const num = String(i).padStart(2, "0");
        await drawImage(
            monsters,
            `BOSS${num}`,
            path.join(ASSET_DIR, "monster", `boss${num}.png`),
        );
    }
    await drawImageRect(
        monsters,
        "MONSTER_SOUL",
        path.join(ASSET_DIR, "monster_soul.png"),
        128,
        192,
    );

    const iconFiles = [
        "accept",
        "add",
        "attack_damage",
        "attack_range",
        "attack_speed",
        "card",
        "config",
        "down",
        "enemy_boss",
        "enemy_named",
        "enemy_normal",
        "gold",
        "health",
        "invincible",
        "item",
        "level",
        "lock",
        "move_speed",
        "multiply",
        "new",
        "quest",
        "rarity_common",
        "rarity_epic",
        "rarity_legendary",
        "rarity_rare",
        "rating",
        "refresh",
        "reject",
        "shield",
        "shop",
        "speaker",
        "suit_clubs",
        "suit_diamonds",
        "suit_hearts",
        "suit_spades",
        "up",
    ];
    const icons = createAtlas("icons", ROW_W, CELL * 3);
    for (const name of iconFiles) {
        const constName = "ICON_" + name.toUpperCase();
        await drawImage(
            icons,
            constName,
            path.join(ASSET_DIR, "icon", `${name}.png`),
        );
    }

    saveAtlas(shapes, "particle_shapes.png");
    saveAtlas(line, "particle_line.png");
    saveAtlas(projectiles, "particle_projectiles.png");
    saveAtlas(monsters, "particle_monsters.png");
    saveAtlas(icons, "particle_icons.png");

    generateRust(shapes, line, projectiles, monsters, icons);
}

function generateRust(
    shapes: Atlas,
    line: Atlas,
    projectiles: Atlas,
    monsters: Atlas,
    icons: Atlas,
): void {
    let rs = `use namui::*;\n\n`;
    rs += `fn rect(x: f32, y: f32, w: f32, h: f32) -> Rect<Px> {\n`;
    rs += `    Rect::Xywh { x: px(x), y: px(y), width: px(w), height: px(h) }\n`;
    rs += `}\n\n`;

    rs += `const LINE_SPRITE_W: f32 = 1024.0;\n`;
    rs += `const LINE_SPRITE_H: f32 = 16.0;\n\n`;

    rs += `pub fn centered_sprite(\n`;
    rs += `    src_rect: Rect<Px>,\n`;
    rs += `    cx: Px,\n`;
    rs += `    cy: Px,\n`;
    rs += `    scale: f32,\n`;
    rs += `    color: Option<Color>,\n`;
    rs += `) -> ImageSprite {\n`;
    rs += `    let sw = src_rect.width().as_f32();\n`;
    rs += `    let sh = src_rect.height().as_f32();\n`;
    rs += `    ImageSprite {\n`;
    rs += `        src_rect,\n`;
    rs += `        xform: RSXform {\n`;
    rs += `            scos: scale,\n`;
    rs += `            ssin: 0.0,\n`;
    rs += `            tx: cx - px(scale * sw / 2.0),\n`;
    rs += `            ty: cy - px(scale * sh / 2.0),\n`;
    rs += `        },\n`;
    rs += `        color,\n`;
    rs += `    }\n`;
    rs += `}\n\n`;

    rs += `pub fn centered_rotated_sprite(\n`;
    rs += `    src_rect: Rect<Px>,\n`;
    rs += `    cx: Px,\n`;
    rs += `    cy: Px,\n`;
    rs += `    scale: f32,\n`;
    rs += `    angle_rad: f32,\n`;
    rs += `    color: Option<Color>,\n`;
    rs += `) -> ImageSprite {\n`;
    rs += `    let sw = src_rect.width().as_f32();\n`;
    rs += `    let sh = src_rect.height().as_f32();\n`;
    rs += `    let cos_a = angle_rad.cos();\n`;
    rs += `    let sin_a = angle_rad.sin();\n`;
    rs += `    let scos = scale * cos_a;\n`;
    rs += `    let ssin = scale * sin_a;\n`;
    rs += `    ImageSprite {\n`;
    rs += `        src_rect,\n`;
    rs += `        xform: RSXform {\n`;
    rs += `            scos,\n`;
    rs += `            ssin,\n`;
    rs += `            tx: cx - px(scos * sw / 2.0 - ssin * sh / 2.0),\n`;
    rs += `            ty: cy - px(ssin * sw / 2.0 + scos * sh / 2.0),\n`;
    rs += `        },\n`;
    rs += `        color,\n`;
    rs += `    }\n`;
    rs += `}\n\n`;

    rs += `pub fn line_sprite(\n`;
    rs += `    start_x: Px,\n`;
    rs += `    start_y: Px,\n`;
    rs += `    end_x: Px,\n`;
    rs += `    end_y: Px,\n`;
    rs += `    thickness: f32,\n`;
    rs += `    color: Option<Color>,\n`;
    rs += `) -> Option<ImageSprite> {\n`;
    rs += `    let dx = (end_x - start_x).as_f32();\n`;
    rs += `    let dy = (end_y - start_y).as_f32();\n`;
    rs += `    let length = (dx * dx + dy * dy).sqrt();\n`;
    rs += `    if length < 0.001 || thickness < 0.001 {\n`;
    rs += `        return None;\n`;
    rs += `    }\n`;
    rs += `    let angle = dy.atan2(dx);\n`;
    rs += `    let scale = thickness / LINE_SPRITE_H;\n`;
    rs += `    let src_w = (length / scale).min(LINE_SPRITE_W);\n`;
    rs += `    let cos_a = angle.cos();\n`;
    rs += `    let sin_a = angle.sin();\n`;
    rs += `    let scos = scale * cos_a;\n`;
    rs += `    let ssin = scale * sin_a;\n`;
    rs += `    let half_h = LINE_SPRITE_H / 2.0;\n`;
    rs += `    Some(ImageSprite {\n`;
    rs += `        src_rect: Rect::Xywh {\n`;
    rs += `            x: px(0.0),\n`;
    rs += `            y: px(0.0),\n`;
    rs += `            width: px(src_w),\n`;
    rs += `            height: px(LINE_SPRITE_H),\n`;
    rs += `        },\n`;
    rs += `        xform: RSXform {\n`;
    rs += `            scos,\n`;
    rs += `            ssin,\n`;
    rs += `            tx: start_x + px(ssin * half_h),\n`;
    rs += `            ty: start_y - px(scos * half_h),\n`;
    rs += `        },\n`;
    rs += `        color,\n`;
    rs += `    })\n`;
    rs += `}\n\n`;

    const sh = shapes.sprites;
    const glowCircle = spriteOrThrow(sh, "GLOW_CIRCLE");
    const starBurst = spriteOrThrow(sh, "STAR_BURST");
    const cross = spriteOrThrow(sh, "CROSS");
    const ring = spriteOrThrow(sh, "RING");
    rs += `pub fn glow_circle() -> Rect<Px> { rect(${glowCircle.x}.0, ${glowCircle.y}.0, ${glowCircle.w}.0, ${glowCircle.h}.0) }\n`;
    rs += `pub fn star_burst() -> Rect<Px> { rect(${starBurst.x}.0, ${starBurst.y}.0, ${starBurst.w}.0, ${starBurst.h}.0) }\n`;
    rs += `pub fn cross() -> Rect<Px> { rect(${cross.x}.0, ${cross.y}.0, ${cross.w}.0, ${cross.h}.0) }\n`;
    rs += `pub fn ring() -> Rect<Px> { rect(${ring.x}.0, ${ring.y}.0, ${ring.w}.0, ${ring.h}.0) }\n`;

    const ms = monsters.sprites;
    const monsterSoul = spriteOrThrow(ms, "MONSTER_SOUL");
    rs += `pub fn monster_soul() -> Rect<Px> { rect(${monsterSoul.x}.0, ${monsterSoul.y}.0, ${monsterSoul.w}.0, ${monsterSoul.h}.0) }\n`;

    const ps = projectiles.sprites;
    rs += `\npub fn projectile_rect(kind: crate::game_state::projectile::ProjectileKind) -> Rect<Px> {\n`;
    rs += `    use crate::game_state::projectile::ProjectileKind;\n`;
    rs += `    match kind {\n`;
    for (let i = 1; i <= 4; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ps, `TRASH_${num}`);
        rs += `        ProjectileKind::Trash${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    for (let i = 0; i <= 4; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ps, `GIRL_${num}`);
        rs += `        ProjectileKind::Girl${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    {
        const r = spriteOrThrow(ps, "CARDS_00");
        rs += `        ProjectileKind::Cards00 => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    {
        const r = spriteOrThrow(ps, "HEART_PROJ_00");
        rs += `        ProjectileKind::Heart00 => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `    }\n}\n`;

    rs += `\npub fn card_particle_rect(kind: crate::game_state::field_particle::particle::CardKind) -> Rect<Px> {\n`;
    rs += `    use crate::game_state::field_particle::particle::CardKind;\n`;
    rs += `    match kind {\n`;
    for (let i = 0; i <= 3; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ps, `CARD_PARTICLE_${num}`);
        rs += `        CardKind::Card${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `    }\n}\n`;

    rs += `\npub fn heart_particle_rect(kind: crate::game_state::field_particle::particle::HeartParticleKind) -> Rect<Px> {\n`;
    rs += `    use crate::game_state::field_particle::particle::HeartParticleKind;\n`;
    rs += `    match kind {\n`;
    for (let i = 0; i <= 2; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ps, `HEART_PARTICLE_${num}`);
        rs += `        HeartParticleKind::Heart${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    {
        const r = spriteOrThrow(ps, "HEART_PROJ_00");
        rs += `        HeartParticleKind::RisingHeart { .. } => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    {
        const glow = spriteOrThrow(ps, "GLOW_CIRCLE");
        rs += `        _ => rect(${glow.x}.0, ${glow.y}.0, ${glow.w}.0, ${glow.h}.0),\n`;
    }
    rs += `    }\n}\n`;

    {
        const r = spriteOrThrow(ps, "GLOW_CIRCLE");
        rs += `\npub fn projectile_glow_circle() -> Rect<Px> { rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0) }\n`;
    }

    rs += `\npub fn monster_rect(kind: crate::game_state::MonsterKind) -> Rect<Px> {\n`;
    rs += `    use crate::game_state::MonsterKind;\n`;
    rs += `    match kind {\n`;
    for (let i = 1; i <= 15; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ms, `MOB${num}`);
        rs += `        MonsterKind::Mob${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    const mob15 = spriteOrThrow(ms, "MOB15");
    for (let i = 16; i <= 50; i++) {
        const num = String(i).padStart(2, "0");
        rs += `        MonsterKind::Mob${num} => rect(${mob15.x}.0, ${mob15.y}.0, ${mob15.w}.0, ${mob15.h}.0),\n`;
    }
    for (let i = 1; i <= 11; i++) {
        const num = String(i).padStart(2, "0");
        const r = spriteOrThrow(ms, `BOSS${num}`);
        rs += `        MonsterKind::Boss${num} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `    }\n}\n`;

    const iconMapping = {
        Accept: "ICON_ACCEPT",
        AttackDamage: "ICON_ATTACK_DAMAGE",
        AttackRange: "ICON_ATTACK_RANGE",
        AttackSpeed: "ICON_ATTACK_SPEED",
        Config: "ICON_CONFIG",
        EnemyBoss: "ICON_ENEMY_BOSS",
        EnemyNamed: "ICON_ENEMY_NAMED",
        EnemyNormal: "ICON_ENEMY_NORMAL",
        Gold: "ICON_GOLD",
        Health: "ICON_HEALTH",
        Invincible: "ICON_INVINCIBLE",
        Item: "ICON_ITEM",
        Level: "ICON_LEVEL",
        Lock: "ICON_LOCK",
        MoveSpeed: "ICON_MOVE_SPEED",
        Contract: "ICON_QUEST",
        Refresh: "ICON_REFRESH",
        Reject: "ICON_REJECT",
        Shield: "ICON_SHIELD",
        Shop: "ICON_SHOP",
        Speaker: "ICON_SPEAKER",
        Up: "ICON_UP",
        Down: "ICON_DOWN",
        Card: "ICON_CARD",
        New: "ICON_NEW",
        Add: "ICON_ADD",
        Multiply: "ICON_MULTIPLY",
        Rating: "ICON_RATING",
    };
    const suitMapping = {
        Spades: "ICON_SUIT_SPADES",
        Hearts: "ICON_SUIT_HEARTS",
        Diamonds: "ICON_SUIT_DIAMONDS",
        Clubs: "ICON_SUIT_CLUBS",
    };
    const rarityMapping = {
        Common: "ICON_RARITY_COMMON",
        Rare: "ICON_RARITY_RARE",
        Epic: "ICON_RARITY_EPIC",
        Legendary: "ICON_RARITY_LEGENDARY",
    };

    const ic = icons.sprites;
    rs += `\npub fn icon_rect(kind: &crate::icon::IconKind) -> Rect<Px> {\n`;
    rs += `    use crate::icon::IconKind;\n`;
    rs += `    match kind {\n`;
    for (const [variant, spriteName] of Object.entries(iconMapping)) {
        const r = spriteOrThrow(ic, spriteName);
        rs += `        IconKind::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `        IconKind::Suit { suit } => match suit {\n`;
    for (const [variant, spriteName] of Object.entries(suitMapping)) {
        const r = spriteOrThrow(ic, spriteName);
        rs += `            crate::Suit::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `        },\n`;
    rs += `        IconKind::Rarity { rarity } => match rarity {\n`;
    for (const [variant, spriteName] of Object.entries(rarityMapping)) {
        const r = spriteOrThrow(ic, spriteName);
        rs += `            crate::Rarity::${variant} => rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0),\n`;
    }
    rs += `        },\n`;
    rs += `    }\n}\n`;

    rs += `\npub fn digit_rect(ch: u8) -> Rect<Px> {\n`;
    rs += `    let idx = match ch {\n`;
    rs += `        b'0'..=b'9' => (ch - b'0') as f32,\n`;
    rs += `        b'.' => 10.0,\n`;
    rs += `        b'k' => 11.0,\n`;
    rs += `        b'm' => 12.0,\n`;
    rs += `        b'b' => 13.0,\n`;
    rs += `        _ => 0.0,\n`;
    rs += `    };\n`;
    rs += `    rect(idx * 64.0, 0.0, 64.0, 64.0)\n`;
    rs += `}\n`;

    fs.writeFileSync(OUTPUT_RS, rs);
    console.log(`Rust constants written to ${OUTPUT_RS}`);

    console.log("Shapes sprites:", JSON.stringify(shapes.sprites, null, 2));
    console.log("Line sprites:", JSON.stringify(line.sprites, null, 2));
    console.log(
        "Projectile sprites:",
        JSON.stringify(projectiles.sprites, null, 2),
    );
    console.log("Monster sprites:", JSON.stringify(monsters.sprites, null, 2));
    console.log("Icon sprites:", JSON.stringify(icons.sprites, null, 2));
}

main().catch(console.error);
