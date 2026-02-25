import { saveAtlas } from "./particle_atlas/atlas.ts";
import {
    createAttackAtlas,
    createIconsAtlas,
    createLineAtlas,
    createMonstersAtlas,
    createProjectilesAtlas,
    createShapesAtlas,
} from "./particle_atlas/build_atlases.ts";
import { generateRust } from "./particle_atlas/rust_generator.ts";

async function main() {
    const shapes = await createShapesAtlas();
    const attack = await createAttackAtlas();
    const line = createLineAtlas();
    const projectiles = await createProjectilesAtlas();
    const monsters = await createMonstersAtlas();
    const icons = await createIconsAtlas();

    saveAtlas(shapes, "particle_shapes.png");
    saveAtlas(attack, "particle_attack.png");
    saveAtlas(line, "particle_line.png");
    saveAtlas(projectiles, "particle_projectiles.png");
    saveAtlas(monsters, "particle_monsters.png");
    saveAtlas(icons, "particle_icons.png");

    generateRust(shapes, attack, line, projectiles, monsters, icons);
}

main().catch(console.error);
