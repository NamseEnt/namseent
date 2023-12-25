const { convertPathWin32ToPosix } = require("./convertPathWin32ToPosix");

function convertPathLocalToNamui(localPath) {
    if (process.platform === "win32") {
        return convertPathWin32ToPosix(localPath);
    }
    return localPath;
}

exports.convertPathLocalToNamui = convertPathLocalToNamui;
