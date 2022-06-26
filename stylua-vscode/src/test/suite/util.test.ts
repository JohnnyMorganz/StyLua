import * as assert from "assert";
import { getAssetFilenamePatternForPlatform } from "../../util";

suite("Utilities testing", () => {
  test("asset filename pattern matches name with version", () => {
    const pattern = getAssetFilenamePatternForPlatform("win32");
    assert(pattern.test("stylua-0.12.2-win64.zip"));
  });

  test("asset filename pattern matches name without version", () => {
    const pattern = getAssetFilenamePatternForPlatform("win32");
    assert(pattern.test("stylua-win64.zip"));
  });

  test("asset filename pattern does not match for wrong platform", () => {
    const pattern = getAssetFilenamePatternForPlatform("win32");
    assert.strictEqual(pattern.test("stylua-linux.zip"), false);
  });
});
