import { loadScript } from "./loadScript";

export async function loadRust(path: string) {
    await loadScript(path);
}
