const { makeDir } = require("./makeDir");
const { read } = require("./read");
const { readDir } = require("./readDir");
const { write } = require("./write");

exports.fileSystem = {
    makeDir,
    read,
    readDir,
    write,
};
