const { readFile } = require("fs/promises");
const { join } = require("path");

const resourceRoot = join(__dirname, "../../../..");

async function read(path) {
    const normalizedPath = join(resourceRoot, path).normalize();
    return readFile(normalizedPath);
}

exports.read = read;
