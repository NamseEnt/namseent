import { Plugin } from "vite";
import * as fs from "fs";
import * as path from "path";

export function assetCollectorPlugin(assetDir: string): Plugin {
    const imageFiles = collectImageFiles(assetDir);
    imageFiles.sort();

    const imageInfos: ImageInfo[] = imageFiles.map((file, id) => ({
        path: file,
        relativePath: path.relative(assetDir, file),
        id,
    }));

    const systemBundleDir = path.join(__dirname, "../system_bundle");
    const cursorSpritePath = path.join(
        systemBundleDir,
        "cursor/capitaine_24.png",
    );
    imageInfos.push({
        path: cursorSpritePath,
        relativePath: "cursor/capitaine_24.png",
        id: 100000,
    });

    console.log(`Collected ${imageInfos.length} image files from ${assetDir}`);

    // Collect font assets
    const fontInfos: FontInfo[] = collectFontFiles(systemBundleDir);
    console.log(`Collected ${fontInfos.length} font files from system_bundle`);

    const virtualModuleId = "virtual:asset-list";
    const resolvedVirtualModuleId = "\0" + virtualModuleId;
    const virtualFontModuleId = "virtual:font-asset";
    const resolvedVirtualFontModuleId = "\0" + virtualFontModuleId;

    return {
        name: "asset-collector-plugin",
        resolveId(id) {
            if (id === virtualModuleId) {
                return resolvedVirtualModuleId;
            }
            if (id === virtualFontModuleId) {
                return resolvedVirtualFontModuleId;
            }
        },
        load(id) {
            if (id === resolvedVirtualModuleId) {
                const assetList = imageInfos.map((info) => ({
                    id: info.id,
                    path: `/@fs${info.path}`,
                }));
                return `export const assetList = ${JSON.stringify(
                    assetList,
                    null,
                    2,
                )};`;
            }
            if (id === resolvedVirtualFontModuleId) {
                const fontAsset = fontInfos.map((info) => ({
                    name: info.name,
                    path: `/@fs${info.path}`,
                }));
                return `export const fontAsset = ${JSON.stringify(
                    fontAsset,
                    null,
                    2,
                )};`;
            }
        },
    };
}

interface ImageInfo {
    path: string;
    relativePath: string;
    id: number;
}

interface FontInfo {
    name: string;
    path: string;
}

function collectFontFiles(systemBundleDir: string): FontInfo[] {
    const fontPaths = [
        "font/Ko/NotoSansKR-Thin.woff2",
        "font/Ko/NotoSansKR-Light.woff2",
        "font/Ko/NotoSansKR-Regular.woff2",
        "font/Ko/NotoSansKR-Medium.woff2",
        "font/Ko/NotoSansKR-Bold.woff2",
        "font/Ko/NotoSansKR-Black.woff2",
    ];

    const fontInfos: FontInfo[] = [];

    for (const fontPath of fontPaths) {
        const absolutePath = path.join(systemBundleDir, fontPath);

        fontInfos.push({
            name: path.parse(fontPath).name,
            path: absolutePath,
        });
    }

    return fontInfos;
}

function collectImageFiles(assetDir: string): string[] {
    const files: string[] = [];

    if (!fs.existsSync(assetDir)) {
        return files;
    }

    function visitDirs(dir: string) {
        const entries = fs.readdirSync(dir, { withFileTypes: true });
        for (const entry of entries) {
            const fullPath = path.join(dir, entry.name);
            if (entry.isDirectory()) {
                visitDirs(fullPath);
            } else if (entry.isFile()) {
                const ext = path.extname(entry.name).toLowerCase();
                if (ext === ".jpg" || ext === ".jpeg" || ext === ".png") {
                    files.push(fullPath);
                }
            }
        }
    }

    visitDirs(assetDir);
    return files;
}
