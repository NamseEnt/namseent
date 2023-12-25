const { readFile } = require("fs/promises");
const {
    resolvePathNamuiToLocal,
} = require("../../util/resolvePathNamuiToLocal");

async function read(path) {
    const resolvedLocalPath = await resolvePathNamuiToLocal(path);
    return readFile(resolvedLocalPath);
}

exports.read = read;
