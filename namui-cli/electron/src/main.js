const { app, BrowserWindow } = require("electron");

const port = parseInt(process.argv[2]);
if (typeof port !== "number" || isNaN(port)) {
    console.error("Port not served");
    process.exit(1);
}

function createWindow() {
    const mainWindow = new BrowserWindow({
        width: 1280,
        height: 720,
    });
    mainWindow.webContents.openDevTools();

    app.getGPUInfo("complete").then((data) => console.log(data));
    mainWindow.loadURL(`http://localhost:${port}`);
}

app.whenReady().then(() => {
    createWindow();

    app.on("activate", function () {
        if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });
});

app.on("window-all-closed", function () {
    if (process.platform !== "darwin") app.quit();
});
