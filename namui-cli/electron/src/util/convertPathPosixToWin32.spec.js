const { default: test } = require("@playwright/test");
const assert = require("assert");
const { convertPathPosixToWin32 } = require("./convertPathPosixToWin32");

test("convert absolute posix path", async () => {
    [
        ["/C:/a/b/c", "C:\\a\\b\\c"],
        ["/C:/a/./c", "C:\\a\\c"],
        ["/C:/a/../c", "C:\\c"],
        ["/UNC/a/b/c", "\\\\a\\b\\c"],
        ["/UNC/a/./c", "\\\\a\\c"],
        ["/UNC/a/../c", "\\\\c"],
    ].forEach(([input, expected]) => {
        const actual = convertPathPosixToWin32(input);
        assert.strictEqual(actual, expected);
    });
});

test("convert relative posix path", async () => {
    [
        ["C/a/b/c", "C\\a\\b\\c"],
        ["C/a/./c", "C\\a\\c"],
        ["C/a/../c", "C\\c"],
    ].forEach(([input, expected]) => {
        const actual = convertPathPosixToWin32(input);
        assert.strictEqual(actual, expected);
    });
});

test("empty string should be returned for posix root path", async () => {
    const input = "/";
    const expected = "";
    const actual = convertPathPosixToWin32(input);
    assert.strictEqual(actual, expected);
});

test("current dir should be returned for posix empty path", async () => {
    const input = "";
    const expected = ".";
    const actual = convertPathPosixToWin32(input);
    assert.strictEqual(actual, expected);
});
