const { ipcRenderer } = require("electron/renderer");

let recentlyOpenedUrl = undefined;
let deepLinkOpenedEventListenerSet = new Set();

function getRecentlyOpenedUrl() {
    return recentlyOpenedUrl;
}

function addDeepLinkOpenedEventListener(listener) {
    deepLinkOpenedEventListenerSet.add(listener);
}

ipcRenderer.addListener("deep-link-opened", (event, url) => {
    recentlyOpenedUrl = url;
    deepLinkOpenedEventListenerSet.forEach((listener) => {
        listener(url);
    });
});

exports.deepLink = {
    getRecentlyOpenedUrl,
    addDeepLinkOpenedEventListener,
};
