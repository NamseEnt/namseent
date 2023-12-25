const { convertPathPosixToWin32 } = require("./convertPathPosixToWin32");

function convertPathNamuiToLocal(namuiPath) {
    if (process.platform === "win32") {
        return convertPathPosixToWin32(namuiPath);
    }
    return namuiPath;
}

exports.convertPathNamuiToLocal = convertPathNamuiToLocal;
