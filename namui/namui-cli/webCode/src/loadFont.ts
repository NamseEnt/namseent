import { fontAsset } from "virtual:font-asset";
import { CommonExports } from "./exports";

export async function loadFonts(exports: CommonExports): Promise<void> {
    await Promise.all(
        fontAsset.map(async ({ name, path }) => {
            try {
                const response = await fetch(path);
                if (!response.ok) {
                    throw new Error(
                        `Failed to fetch font ${name} from ${path}: ${response.statusText}`,
                    );
                }

                const arrayBuffer = await response.arrayBuffer();
                const bytes = new Uint8Array(arrayBuffer);
                const len = bytes.length;
                const fontPtr = exports.malloc(len);
                new Uint8Array(exports.memory.buffer, fontPtr, len).set(bytes);

                const nameBytes = new TextEncoder().encode(name);
                const nameLen = nameBytes.length;
                const namePtr = exports.malloc(nameLen);
                new Uint8Array(exports.memory.buffer, namePtr, nameLen).set(
                    nameBytes,
                );

                exports._register_font(namePtr, nameLen, fontPtr, len);

                exports.free(fontPtr);
                exports.free(namePtr);

                console.log(`Font ${name} loaded successfully`);
            } catch (error) {
                console.error(
                    "_on_window_resize" in exports ? "drawer" : "main",
                    `Error loading font ${name} from ${path}:`,
                    error,
                );
                throw error;
            }
        }),
    );
}
