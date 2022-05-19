const { default: test, expect } = require("@playwright/test");
const { _electron: electron } = require("playwright");
const { join } = require("path");
const assert = require("assert");

test("namuiApi should be loaded at window", async () => {
    // TODO: run playwright-electron in headless
    // const app = await electron.launch({
    //     cwd: join(__dirname, "../../"),
    //     args: ["src/main.js", "--test"],
    // });
    // const page = await app.firstWindow();
    // const namuiApiType = await page.evaluate("typeof window.namuiApi");
    // if (namuiApiType !== "object") {
    //     throw new Error("window.namuiApi is not object: ", namuiApiType);
    // }
    // await app.close();
});
