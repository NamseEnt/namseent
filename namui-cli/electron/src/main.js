const { app, BrowserWindow, ipcMain, shell } = require("electron");
const isDev = require("electron-is-dev");
const { existsSync, readFileSync } = require("fs");
const path = require("path");

const config = getConfig();
const gotTheLock = app.requestSingleInstanceLock();

if (!gotTheLock) {
    app.quit();
} else {
    setAppAsDefaultProtocolClient(app, config.deepLinkSchemes);

    app.whenReady().then(() => {
        ipcMain.handle("config", () => config);
        ipcMain.on("open-external", (event, url) => shell.openExternal(url));
        createWindow();
    });

    app.on("activate", function () {
        if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });

    app.on("window-all-closed", function () {
        if (process.platform !== "darwin") app.quit();
    });
}

function createWindow() {
    const mainWindow = new BrowserWindow({
        width: 1280,
        height: 720,
        webPreferences: {
            preload: path.join(__dirname, "preload.js"),
            webSecurity: false,
            allowRunningInsecureContent: false,
            sandbox: false,
        },
    });
    setOpenUrlEventHandler(app, mainWindow);

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
        mainWindow.loadFile("../../index.html");
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
        applicationRoot: path.join(__dirname, "../../.."),
        deepLinkSchemes: [],
    };
    const argv = [...process.argv];
    while (argv.length) {
        const arg = argv.pop();
        if (arg === "--test") {
            config.test = true;
        } else if (arg.startsWith("port=")) {
            const port = parseInt(arg.slice("port=".length));
            config.port = port;
        } else if (arg.startsWith("applicationRoot=")) {
            const applicationRoot = arg.slice("applicationRoot=".length);
            config.applicationRoot = applicationRoot;
        } else if (arg.startsWith("deepLink=")) {
            const deepLinkScheme = arg.slice("deepLink=".length);
            const schemeNotExist =
                config.deepLinkSchemes.findIndex(
                    (value) => value === deepLinkScheme,
                ) < 0;
            if (schemeNotExist) {
                config.deepLinkSchemes.push(deepLinkScheme);
            }
        }
    }
    config.deepLinkSchemes.push(...loadDeepLinkManifest(config.applicationRoot));
    return config;
}

function loadDeepLinkManifest(applicationRootPath) {
    const deepLinkManifestPath = path.join(applicationRootPath, ".namuideeplink");
    const deepLinkSchemes = [];
    if (existsSync(deepLinkManifestPath)) {
        const deepLinkManifestString = readFileSync(
            deepLinkManifestPath,
            "utf-8",
        );
        deepLinkManifestString.split("\n").forEach((line) => {
            const trimmedLine = line.trim();
            if (!trimmedLine) {
                return;
            }
            if (trimmedLine.startsWith("#")) {
                return;
            }
            deepLinkSchemes.push(trimmedLine);
        });
    }
    return deepLinkSchemes;
}

function setOpenUrlEventHandler(app, mainWindow) {
    let sendUrl = (url) => mainWindow.webContents.send("deep-link-opened", url);
    app.on("open-url", (event, url) => {
        sendUrl(url);
    });

    app.on("second-instance", (event, argv, workingDirectory) => {
        const url = argv[argv.length - 1];
        sendUrl(url);
        if (mainWindow) {
            if (mainWindow.isMinimized()) {
                mainWindow.restore();
            }
            mainWindow.focus();
        }
    });
}

function setAppAsDefaultProtocolClient(app, deepLinkSchemes) {
    let execPath = process.argv[0];
    let argv = [path.join(__dirname, ".."), ...process.argv.slice(2)];
    deepLinkSchemes.forEach((deepLinkScheme) => {
        app.setAsDefaultProtocolClient(deepLinkScheme, execPath, argv);
    });
}
