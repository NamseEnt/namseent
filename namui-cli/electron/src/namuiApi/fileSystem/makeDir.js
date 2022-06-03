const {
    resolvePathNamuiToLocal,
} = require("../../util/resolvePathNamuiToLocal");
const { mkdir } = require("fs/promises");

async function makeDir(path) {
    const resolvedLocalPath = await resolvePathNamuiToLocal(path);
    return mkdir(resolvedLocalPath, {
        recursive: true,
    });
}

exports.makeDir = makeDir;
