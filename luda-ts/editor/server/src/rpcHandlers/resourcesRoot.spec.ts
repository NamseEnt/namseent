import fs from "fs";
import { resourcesRoot } from "./resourcesRoot";

test("resourcesRoot should be right place", async () => {
  expect(fs.existsSync(resourcesRoot)).toBe(true);
});
