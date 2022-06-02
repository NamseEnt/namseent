const { writeFile } = require("fs/promises");
const {
    resolvePathNamuiToLocal,
} = require("../../util/resolvePathNamuiToLocal");

async function write(path, content) {
    const resolvedLocalPath = await resolvePathNamuiToLocal(path);
    return writeFile(resolvedLocalPath, content);
}

exports.write = write;
