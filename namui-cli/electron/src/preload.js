const { contextBridge } = require("electron");
const { namuiApi } = require("./namuiApi");

contextBridge.exposeInMainWorld("namuiApi", {
    ...namuiApi,
});
