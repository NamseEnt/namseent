const { convertPathNamuiToLocal } = require("./convertPathNamuiToLocal");
const path = require("path");

const resourceRoot = path.join(__dirname, "../../..");

function resolvePathNamuiToLocal(namuiPath) {
    const definiteLocalPath = convertPathNamuiToLocal(namuiPath);
    return path.resolve(resourceRoot, definiteLocalPath);
}

exports.resolvePathNamuiToLocal = resolvePathNamuiToLocal;
