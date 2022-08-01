const { deepLink } = require("./deepLink");
const { fileSystem } = require("./fileSystem");
const { openExternal } = require("./openExternal");

exports.namuiApi = {
    fileSystem,
    deepLink,
    openExternal,
};
