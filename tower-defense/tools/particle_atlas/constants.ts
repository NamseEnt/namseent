import path from "path";

export const ASSET_DIR = path.join(
    import.meta.dirname,
    "..",
    "..",
    "asset",
    "image",
);
export const OUTPUT_RS = path.join(
    import.meta.dirname,
    "..",
    "..",
    "src",
    "game_state",
    "field_particle",
    "atlas.rs",
);

export const CELL = 128;
export const LINE_H = 16;
export const ROW_W = 2048;
