import { createCanvas, loadImage } from "canvas";
import fs from "node:fs";
import path from "node:path";

type ComponentSource = {
    name: string;
    files: { filePath: string; frameIndex?: number }[];
    animated: boolean;
};

type AtlasRect = { x: number; y: number; width: number; height: number };

type LoadedSprite = {
    filePath: string;
    image: Awaited<ReturnType<typeof loadImage>>;
    frameIndex?: number;
};

type PackedSprite = LoadedSprite & {
    x: number;
    y: number;
};

type PackedLayout = {
    sprites: PackedSprite[];
    width: number;
    height: number;
    score: number;
};

const appDir = path.join(import.meta.dir, "..", "app");
const sourceDir = ["atlas_source"]
    .map((name) => path.join(appDir, name))
    .find((candidate) => fs.existsSync(candidate));
const imageOutputDir = path.join(appDir, "asset", "image");
const rustOutputPath = path.join(appDir, "src", "atlas", "generated.rs");
const padding = 1;

function constName(parts: string[]): string {
    const name = parts
        .map((part) =>
            part
                .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
                .replace(/[^a-zA-Z0-9]+/g, "_")
                .replace(/^_+|_+$/g, "")
                .toUpperCase(),
        )
        .filter(Boolean)
        .join("_");
    if (!name) {
        throw new Error(
            `Cannot create a Rust constant name from: ${parts.join("/")}`,
        );
    }
    return /^\d/.test(name) ? `_${name}` : name;
}

function listComponents(
    atlasName: string,
    atlasDir: string,
): ComponentSource[] {
    const components: ComponentSource[] = [];

    function visit(dirPath: string, parts: string[]): void {
        const entries = fs
            .readdirSync(dirPath, { withFileTypes: true })
            .sort((left, right) => left.name.localeCompare(right.name));
        const files = entries.filter(
            (entry) =>
                entry.isFile() &&
                path.extname(entry.name).toLowerCase() === ".png",
        );
        const directories = entries.filter((entry) => entry.isDirectory());
        const numberedFiles = files.filter((entry) =>
            /^\d+\.png$/i.test(entry.name),
        );

        if (
            files.length > 0 &&
            files.length === entries.length &&
            numberedFiles.length === files.length
        ) {
            const sortedFrames = numberedFiles
                .map((entry) => ({
                    entry,
                    frameIndex: Number.parseInt(
                        path.parse(entry.name).name,
                        10,
                    ),
                }))
                .sort((left, right) => left.frameIndex - right.frameIndex);
            sortedFrames.forEach((frame, index) => {
                if (frame.frameIndex !== index) {
                    throw new Error(
                        `Animation frames must start at 0000 and be contiguous: ${dirPath}`,
                    );
                }
            });
            components.push({
                name: constName([atlasName, ...parts]),
                files: sortedFrames.map(({ entry, frameIndex }) => ({
                    filePath: path.join(dirPath, entry.name),
                    frameIndex,
                })),
                animated: true,
            });
            return;
        }

        for (const entry of files) {
            components.push({
                name: constName([
                    atlasName,
                    ...parts,
                    path.parse(entry.name).name,
                ]),
                files: [{ filePath: path.join(dirPath, entry.name) }],
                animated: false,
            });
        }

        for (const entry of directories) {
            visit(path.join(dirPath, entry.name), [...parts, entry.name]);
        }
    }

    visit(atlasDir, []);

    const names = new Set<string>();
    for (const component of components) {
        if (names.has(component.name)) {
            throw new Error(
                `Duplicate sprite component name: ${component.name}`,
            );
        }
        names.add(component.name);
    }
    return components;
}

function packSprites(sprites: LoadedSprite[]): PackedLayout {
    const ordered = [...sprites].sort(
        (left, right) =>
            right.image.height - left.image.height ||
            right.image.width - left.image.width ||
            left.filePath.localeCompare(right.filePath),
    );
    const imageArea = ordered.reduce(
        (sum, sprite) => sum + sprite.image.width * sprite.image.height,
        0,
    );
    const minimumWidth =
        Math.max(...ordered.map((sprite) => sprite.image.width)) + padding * 2;
    const candidateWidths = new Set<number>([
        minimumWidth,
        Math.max(minimumWidth, Math.ceil(Math.sqrt(imageArea))),
    ]);
    const prefixWidths: number[] = [];
    let prefixWidth = padding;
    for (const sprite of ordered) {
        prefixWidth += sprite.image.width + padding;
        prefixWidths.push(prefixWidth);
    }

    const sampleCount = Math.min(64, ordered.length);
    const prefixIndices = new Set<number>();
    for (let index = 0; index < Math.min(16, sampleCount); index += 1) {
        prefixIndices.add(index);
    }
    for (let sample = 1; sample <= sampleCount - 16; sample += 1) {
        const index =
            15 +
            Math.ceil((sample * (ordered.length - 16)) / (sampleCount - 16));
        prefixIndices.add(index);
    }
    for (const index of prefixIndices) {
        const width = prefixWidths[index];
        if (width !== undefined) {
            candidateWidths.add(Math.max(minimumWidth, width));
        }
    }

    function layoutAtWidth(targetWidth: number): PackedLayout {
        let cursorX = padding;
        let cursorY = padding;
        let rowHeight = 0;
        let usedWidth = 0;
        const packedSprites: PackedSprite[] = [];

        for (const sprite of ordered) {
            if (cursorX + sprite.image.width + padding > targetWidth) {
                cursorX = padding;
                cursorY += rowHeight + padding;
                rowHeight = 0;
            }
            packedSprites.push({ ...sprite, x: cursorX, y: cursorY });
            cursorX += sprite.image.width + padding;
            usedWidth = Math.max(usedWidth, cursorX);
            rowHeight = Math.max(rowHeight, sprite.image.height);
        }

        const width = usedWidth;
        const height = cursorY + rowHeight + padding;
        const aspectPenalty = Math.abs(Math.log(width / height));
        return {
            sprites: packedSprites,
            width,
            height,
            score: width * height * (1 + aspectPenalty * 0.1),
        };
    }

    let bestLayout: PackedLayout | undefined;
    for (const candidateWidth of candidateWidths) {
        const layout = layoutAtWidth(candidateWidth);
        if (!bestLayout || layout.score < bestLayout.score) {
            bestLayout = layout;
        }
    }
    if (!bestLayout) {
        throw new Error("Could not pack an empty atlas");
    }
    return bestLayout;
}

async function buildAtlas(atlasName: string, atlasDir: string) {
    const components = listComponents(atlasName, atlasDir);
    const loaded: LoadedSprite[] = await Promise.all(
        components.flatMap((component) =>
            component.files.map(async (file) => ({
                ...file,
                image: await loadImage(file.filePath),
            })),
        ),
    );
    if (loaded.length === 0) {
        throw new Error(`No PNG images found in ${atlasDir}`);
    }

    const layout = packSprites(loaded);
    const canvas = createCanvas(layout.width, layout.height);
    const context = canvas.getContext("2d");
    const spritesByPath = new Map<string, PackedSprite>();

    for (const sprite of layout.sprites) {
        context.drawImage(sprite.image, sprite.x, sprite.y);
        spritesByPath.set(sprite.filePath, sprite);
    }

    const componentRects = components.map((component) => ({
        name: component.name,
        animated: component.animated,
        rects: component.files.map((file) => {
            const sprite = spritesByPath.get(file.filePath);
            if (!sprite) {
                throw new Error(`Atlas placement missing for ${file.filePath}`);
            }
            return {
                x: sprite.x,
                y: sprite.y,
                width: sprite.image.width,
                height: sprite.image.height,
            } satisfies AtlasRect;
        }),
    }));

    const outputPath = path.join(imageOutputDir, `${atlasName}_atlas.png`);
    await Bun.write(outputPath, canvas.toBuffer("image/png"));
    console.log(
        `Atlas written to ${outputPath} (${layout.width}x${layout.height})`,
    );

    return componentRects;
}

async function generateRust(
    atlases: { name: string; animated: boolean; rects: AtlasRect[] }[][],
): Promise<void> {
    const lines = ["use namui::*;", ""];

    for (const component of atlases.flat()) {
        if (component.animated) {
            lines.push(
                `pub const ${component.name}: [Rect<Px>; ${component.rects.length}] = [`,
            );
            for (const rect of component.rects) {
                lines.push(
                    "    Rect::Xywh {",
                    `        x: px(${rect.x}.0),`,
                    `        y: px(${rect.y}.0),`,
                    `        width: px(${rect.width}.0),`,
                    `        height: px(${rect.height}.0),`,
                    "    },",
                );
            }
            lines.push("];", "");
        } else {
            const rect = component.rects[0];
            if (!rect) {
                throw new Error(
                    `Sprite component has no image: ${component.name}`,
                );
            }
            lines.push(
                `pub const ${component.name}: Rect<Px> = Rect::Xywh {`,
                `    x: px(${rect.x}.0),`,
                `    y: px(${rect.y}.0),`,
                `    width: px(${rect.width}.0),`,
                `    height: px(${rect.height}.0),`,
                "};",
                "",
            );
        }
    }

    fs.mkdirSync(path.dirname(rustOutputPath), { recursive: true });
    await Bun.write(rustOutputPath, `${lines.join("\n").trimEnd()}\n`);
    console.log(`Rust atlas metadata written to ${rustOutputPath}`);
}

async function main(): Promise<void> {
    if (!sourceDir) {
        throw new Error(`Atlas source directory not found under ${appDir}`);
    }

    fs.mkdirSync(imageOutputDir, { recursive: true });
    const atlasDirs = fs
        .readdirSync(sourceDir, { withFileTypes: true })
        .filter((entry) => entry.isDirectory())
        .sort((left, right) => left.name.localeCompare(right.name));
    if (atlasDirs.length === 0) {
        throw new Error(`No atlas directories found in ${sourceDir}`);
    }

    const atlases = [];
    for (const entry of atlasDirs) {
        atlases.push(
            await buildAtlas(entry.name, path.join(sourceDir, entry.name)),
        );
    }
    await generateRust(atlases);
}

main().catch((error: unknown) => {
    console.error(error);
    throw error;
});
