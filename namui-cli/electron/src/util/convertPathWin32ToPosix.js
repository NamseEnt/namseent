const path = require("path");

function convertPathWin32ToPosix(win32Path) {
    const normalizedWin32 = path.win32.normalize(win32Path);
    if (path.win32.isAbsolute(normalizedWin32)) {
        const splitedWin32 = normalizedWin32.split(path.win32.sep);
        const driveLetter = splitedWin32[0];
        if (driveLetter) {
            splitedWin32[0] = driveLetter.replace(":", "");
            return `/${splitedWin32.join(path.posix.sep)}`;
        }
        return "";
    }
    return normalizedWin32.split(path.win32.sep).join(path.posix.sep);
}

exports.convertPathWin32ToPosix = convertPathWin32ToPosix;
