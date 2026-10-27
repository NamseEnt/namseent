import { saveAtlas } from "./particle_atlas/atlas.ts";
import {
    createIconsAtlas,
    createMapDecorationsAtlas,
    createMonstersAtlas,
} from "./particle_atlas/build_atlases.ts";
import { generateRust } from "./particle_atlas/rust_generator.ts";

async function main() {
    const monsters = await createMonstersAtlas();
    const icons = await createIconsAtlas();
    const decorations = await createMapDecorationsAtlas();

    saveAtlas(monsters, "particle_monsters.png");
    saveAtlas(icons, "particle_icons.png");
    saveAtlas(decorations, "map_decorations_atlas.png");

    generateRust(monsters, icons, decorations);
}

main().catch(console.error);
