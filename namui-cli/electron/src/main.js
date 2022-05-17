const { app, BrowserWindow } = require("electron");
const isDev = require("electron-is-dev");
const { writeFile, write, writeFileSync } = require("fs");
const path = require("path");
const { exit } = require("process");

function createWindow() {
    const mainWindow = new BrowserWindow({
        width: 1280,
        height: 720,
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
        },
    });

    if (isTest()) {
        mainWindow.loadFile("../test.html");
    } else if (isDev) {
        const port = parseInt(process.argv[2]);
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
    createWindow();

    app.on("activate", function () {
        if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });
});

app.on("window-all-closed", function () {
    if (process.platform !== "darwin") app.quit();
});

function isTest() {
    return process.argv.some((arg) => arg === "--test");
}
