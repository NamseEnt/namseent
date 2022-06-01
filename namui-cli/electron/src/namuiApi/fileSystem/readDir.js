const { readdir } = require("fs/promises");
const {
    convertPathLocalToNamui,
} = require("../../util/convertPathLocalToNamui");
const {
    resolvePathNamuiToLocal,
} = require("../../util/resolvePathNamuiToLocal");
const { join } = require("path");

async function readDir(path) {
    const resolvedLocalPath = resolvePathNamuiToLocal(path);
    const direntList = await readdir(resolvedLocalPath, {
        withFileTypes: true,
    });
    const namuiReadyDirentList = direntList.map((dirent) => {
        return {
            path: convertPathLocalToNamui(join(resolvedLocalPath, dirent.name)),
            isDir: dirent.isDirectory(),
        };
    });
    return JSON.stringify(namuiReadyDirentList);
}

exports.readDir = readDir;
