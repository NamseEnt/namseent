import { saveAtlas } from "./particle_atlas/atlas.ts";
import {
    createAttackAtlas,
    createIconsAtlas,
    createMonstersAtlas,
    createProjectilesAtlas,
} from "./particle_atlas/build_atlases.ts";
import { generateRust } from "./particle_atlas/rust_generator.ts";

async function main() {
    const attack = await createAttackAtlas();
    const projectiles = await createProjectilesAtlas();
    const monsters = await createMonstersAtlas();
    const icons = await createIconsAtlas();

    saveAtlas(attack, "particle_attack.png");
    saveAtlas(projectiles, "particle_projectiles.png");
    saveAtlas(monsters, "particle_monsters.png");
    saveAtlas(icons, "particle_icons.png");

    generateRust(attack, projectiles, monsters, icons);
}

main().catch(console.error);
