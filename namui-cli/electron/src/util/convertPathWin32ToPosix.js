const path = require("path");

function convertPathWin32ToPosix(win32Path) {
    const normalizedWin32 = path.win32.normalize(win32Path);
    if (path.win32.isAbsolute(normalizedWin32)) {
        if (normalizedWin32.startsWith("\\\\")) {
            const posix = normalizedWin32
                .slice(2)
                .split(path.win32.sep)
                .join(path.posix.sep);
            return `/UNC/${posix}`;
        }
        if (normalizedWin32.startsWith("\\")) {
            throw new Error(`UNEXPECTED_WIN32_ROOT: ${normalizedWin32}`);
        }
        const posix = normalizedWin32
            .split(path.win32.sep)
            .join(path.posix.sep);
        return `/${posix}`;
    }
    return normalizedWin32.split(path.win32.sep).join(path.posix.sep);
}

exports.convertPathWin32ToPosix = convertPathWin32ToPosix;
