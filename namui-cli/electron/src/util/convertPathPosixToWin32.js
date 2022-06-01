const path = require("path");

function convertPathPosixToWin32(posixPath) {
    const normalizedPosix = path.posix.normalize(posixPath);
    if (path.posix.isAbsolute(normalizedPosix)) {
        const splittedPosix = normalizedPosix.slice(1).split(path.posix.sep);
        const root = splittedPosix[0];
        if (!root) {
            return "";
        }
        if (root === "UNC") {
            return "\\\\" + splittedPosix.slice(1).join(path.win32.sep);
        }
        return splittedPosix.join(path.win32.sep);
    }
    return normalizedPosix.split(path.posix.sep).join(path.win32.sep);
}

exports.convertPathPosixToWin32 = convertPathPosixToWin32;
