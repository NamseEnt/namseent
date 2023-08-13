export async function loadScript(path: string): Promise<void> {
    if ("importScripts" in globalThis) {
        return importScripts(path);
    }
    await import(path);
}
