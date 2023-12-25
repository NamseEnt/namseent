const { default: test } = require("@playwright/test");
const assert = require("assert");
const { convertPathWin32ToPosix } = require("./convertPathWin32ToPosix");

test("convert absolute win32 path", async () => {
    [
        ["C:\\a\\b\\c", "/C:/a/b/c"],
        ["C:\\a\\.\\c", "/C:/a/c"],
        ["C:\\a\\..\\c", "/C:/c"],
        ["\\\\a\\b\\c", "/UNC/a/b/c"],
    ].forEach(([input, expected]) => {
        const actual = convertPathWin32ToPosix(input);
        assert.strictEqual(actual, expected);
    });
});

test("throw error for drive path that doesn't have drive label", async () => {
    ["\\a\\b\\c", "\\a\\.\\c", "\\a\\..\\c"].forEach((input) => {
        assert.throws(() => {
            convertPathWin32ToPosix(input);
        });
    });
});

test("convert relative win32 path", async () => {
    [
        ["C\\a\\b\\c", "C/a/b/c"],
        ["C\\a\\.\\c", "C/a/c"],
        ["C\\a\\..\\c", "C/c"],
    ].forEach(([input, expected]) => {
        const actual = convertPathWin32ToPosix(input);
        assert.strictEqual(actual, expected);
    });
});

test("current dir should be returned for win32 empty path", async () => {
    const input = "";
    const expected = ".";
    const actual = convertPathWin32ToPosix(input);
    assert.strictEqual(actual, expected);
});
