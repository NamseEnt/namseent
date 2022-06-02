const { convertPathNamuiToLocal } = require("./convertPathNamuiToLocal");
const path = require("path");
const { ipcRenderer } = require("electron");

let resourceRoot = "";

async function resolvePathNamuiToLocal(namuiPath) {
    if (!resourceRoot) {
        await initResourceRoot();
    }
    const definiteLocalPath = convertPathNamuiToLocal(namuiPath);
    return path.resolve(resourceRoot, definiteLocalPath);
}

async function initResourceRoot() {
    const config = await ipcRenderer.invoke("config");
    resourceRoot = config.resourceRoot;
}

exports.resolvePathNamuiToLocal = resolvePathNamuiToLocal;
