const path = require("path");

function convertPathPosixToWin32(posixPath) {
    const normalizedPosix = path.posix.normalize(posixPath);
    if (path.posix.isAbsolute(normalizedPosix)) {
        const splitedPosix = normalizedPosix.slice(1).split(path.posix.sep);
        const driveLetter = splitedPosix[0];
        if (driveLetter) {
            splitedPosix[0] = driveLetter + ":";
            return splitedPosix.join(path.win32.sep);
        }
        return "";
    }
    return normalizedPosix.split(path.posix.sep).join(path.win32.sep);
}

exports.convertPathPosixToWin32 = convertPathPosixToWin32;
