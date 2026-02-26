import type { Atlas } from "../types.ts";
import { spriteOrThrow } from "../types.ts";

export function appendProjectileSection(
    rs: string,
    projectiles: Atlas,
): string {
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
        const r = spriteOrThrow(ps, "PINK_SMOKE");
        rs += `\npub fn projectile_pink_smoke() -> Rect<Px> { rect(${r.x}.0, ${r.y}.0, ${r.w}.0, ${r.h}.0) }\n`;
    }

    return rs;
}
