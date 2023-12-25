const { ipcRenderer } = require("electron");

function openExternal(url) {
    ipcRenderer.send("open-external", url);
}

exports.openExternal = openExternal;
