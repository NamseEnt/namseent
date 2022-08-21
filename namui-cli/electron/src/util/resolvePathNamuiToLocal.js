const { convertPathNamuiToLocal } = require("./convertPathNamuiToLocal");
const path = require("path");
const { ipcRenderer } = require("electron");

let applicationRoot = "";

async function resolvePathNamuiToLocal(namuiPath) {
    if (!applicationRoot) {
        await initapplicationRoot();
    }
    const definiteLocalPath = convertPathNamuiToLocal(namuiPath);
    return path.resolve(applicationRoot, definiteLocalPath);
}

async function initapplicationRoot() {
    const config = await ipcRenderer.invoke("config");
    applicationRoot = config.applicationRoot;
}

exports.resolvePathNamuiToLocal = resolvePathNamuiToLocal;
