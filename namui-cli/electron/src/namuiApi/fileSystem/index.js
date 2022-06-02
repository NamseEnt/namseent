const { read } = require("./read");
const { readDir } = require("./readDir");
const { write } = require("./write");

exports.fileSystem = {
    read,
    readDir,
    write,
};
