import { Plugin } from "vite";
import * as fs from "fs";
import * as path from "path";

interface ImageInfo {
    path: string;
    relativePath: string;
    id: number;
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

export function assetCollectorPlugin(assetDir: string): Plugin {
    // Collect and sort image files (same as asset-macro)
    const imageFiles = collectImageFiles(assetDir);
    imageFiles.sort(); // Sort alphabetically like asset-macro does

    const imageInfos: ImageInfo[] = imageFiles.map((file, id) => ({
        path: file,
        relativePath: path.relative(assetDir, file),
        id,
    }));

    console.log(`Collected ${imageInfos.length} image files from ${assetDir}`);

    const virtualModuleId = "virtual:asset-list";
    const resolvedVirtualModuleId = "\0" + virtualModuleId;

    return {
        name: "asset-collector-plugin",
        config() {
            return {
                define: {
                    __IMAGE_COUNT__: imageInfos.length.toString(),
                },
            };
        },
        resolveId(id) {
            if (id === virtualModuleId) {
                return resolvedVirtualModuleId;
            }
        },
        load(id) {
            if (id === resolvedVirtualModuleId) {
                const assetList = imageInfos.map((info) => ({
                    id: info.id,
                    path: "/@fs" + info.path,
                }));
                return `export const assetList = ${JSON.stringify(assetList, null, 2)};`;
            }
        },
    };
}
