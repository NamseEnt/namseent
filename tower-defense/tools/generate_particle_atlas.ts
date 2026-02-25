import { saveAtlas } from "./particle_atlas/atlas.ts";
import {
    createIconsAtlas,
    createLineAtlas,
    createMonstersAtlas,
    createProjectilesAtlas,
    createShapesAtlas,
} from "./particle_atlas/build_atlases.ts";
import { generateRust } from "./particle_atlas/rust_generator.ts";

async function main() {
    const shapes = createShapesAtlas();
    const line = createLineAtlas();
    const projectiles = await createProjectilesAtlas();
    const monsters = await createMonstersAtlas();
    const icons = await createIconsAtlas();

    saveAtlas(shapes, "particle_shapes.png");
    saveAtlas(line, "particle_line.png");
    saveAtlas(projectiles, "particle_projectiles.png");
    saveAtlas(monsters, "particle_monsters.png");
    saveAtlas(icons, "particle_icons.png");

    generateRust(shapes, line, projectiles, monsters, icons);
}

main().catch(console.error);
