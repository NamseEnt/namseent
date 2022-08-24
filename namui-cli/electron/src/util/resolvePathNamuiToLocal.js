const { convertPathNamuiToLocal } = require("./convertPathNamuiToLocal");
const path = require("path");
const { ipcRenderer } = require("electron");

let applicationRoot = "";

async function resolvePathNamuiToLocal(namuiPath) {
    if (!applicationRoot) {
        await initApplicationRoot();
    }
    const definiteLocalPath = convertPathNamuiToLocal(namuiPath);
    return path.resolve(applicationRoot, definiteLocalPath);
}

async function initApplicationRoot() {
    const config = await ipcRenderer.invoke("config");
    applicationRoot = config.applicationRoot;
}

exports.resolvePathNamuiToLocal = resolvePathNamuiToLocal;
