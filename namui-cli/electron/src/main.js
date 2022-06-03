const { app, BrowserWindow, ipcMain } = require("electron");
const isDev = require("electron-is-dev");
const path = require("path");

const config = getConfig();

function createWindow() {
    const mainWindow = new BrowserWindow({
        width: 1280,
        height: 720,
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    if (config.test) {
        mainWindow.loadFile("../test.html");
    } else if (isDev) {
        const port = config.port;
        if (typeof port !== "number" || isNaN(port)) {
            console.error("Port not served");
            process.exit(1);
        }

        mainWindow.webContents.openDevTools();
        mainWindow.loadURL(`http://localhost:${port}`);
    } else {
        mainWindow.loadFile("../index.html");
    }
}

app.whenReady().then(() => {
    ipcMain.handle("config", () => config);
    createWindow();
    app.on("activate", function () {
        if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });
});

app.on("window-all-closed", function () {
    if (process.platform !== "darwin") app.quit();
});

function getConfig() {
    const config = {
        test: false,
        port: 8000,
        resourceRoot: path.join(__dirname, "../.."),
    };
    const argv = [...process.argv];
    while (argv.length) {
        const arg = argv.pop();
        if (arg === "--test") {
            config.test = true;
        } else if (arg.startsWith("port=")) {
            const port = parseInt(arg.slice("port=".length));
            config.port = port;
        } else if (arg.startsWith("resourceRoot=")) {
            const resourceRoot = arg.slice("resourceRoot=".length);
            config.resourceRoot = resourceRoot;
        }
    }
    return config;
}
