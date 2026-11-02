import * as assert from "assert";
import { getAssetFilenamePatternForPlatform, getMinimalEdit } from "../../util";

suite("Utilities testing", () => {
  test("asset filename pattern matches legacy name with version", () => {
    const pattern = getAssetFilenamePatternForPlatform("win32", "x64");
    assert(pattern.test("stylua-0.12.2-win64.zip"));
  });

  test("asset filename pattern matches legacy name without version", () => {
    const pattern = getAssetFilenamePatternForPlatform("win32", "x64");
    assert(pattern.test("stylua-win64.zip"));
  });

  test("asset filename pattern matches name with version and machine", () => {
    const pattern = getAssetFilenamePatternForPlatform("win32", "x64");
    assert(pattern.test("stylua-windows-x86_64.zip"));
  });

  test("asset filename pattern does not match for wrong platform", () => {
    const pattern = getAssetFilenamePatternForPlatform("win32", "x64");
    assert.strictEqual(pattern.test("stylua-linux.zip"), false);
  });
});

suite("Minimal edit", () => {
  const applyEdit = (original: string, formatted: string) => {
    const { start, end, newText } = getMinimalEdit(original, formatted);
    return original.slice(0, start) + newText + original.slice(end);
  };

  test("only covers the changed lines", () => {
    const original = "local a = 1\nlocal  b=2\nlocal c = 3\n";
    const formatted = "local a = 1\nlocal b = 2\nlocal c = 3\n";
    assert.deepStrictEqual(getMinimalEdit(original, formatted), {
      start: 12,
      end: 23,
      newText: "local b = 2\n",
    });
  });

  test("edits for separately formatted ranges do not overlap", () => {
    const lines = [];
    for (let i = 0; i < 100; i++) {
      lines.push(`local x${i} = ${i}`);
    }
    lines[10] = "local  x10=10";
    lines[90] = "local  x90=90";
    const original = lines.join("\n");
    // Each range format only changes its own line
    const first = getMinimalEdit(
      original,
      original.replace("x10=10", "x10 = 10")
    );
    const second = getMinimalEdit(
      original,
      original.replace("x90=90", "x90 = 90")
    );
    assert(first.end <= second.start);
    assert.strictEqual(
      original.slice(first.start, first.end),
      "local  x10=10\n"
    );
    assert.strictEqual(
      original.slice(second.start, second.end),
      "local  x90=90\n"
    );
  });

  test("handles CRLF, multi-byte characters and edge changes", () => {
    const cases: [string, string][] = [
      ["a\r\nb  \r\nc\r\n", "a\r\nb\r\nc\r\n"],
      ["😀x\n😀y\n", "😀x\n😁y\n"],
      ["\nlocal a\n", "local a\n"],
      ["local a", "local a\n"],
      ["same\n", "same\n"],
      ["", "x"],
    ];
    for (const [original, formatted] of cases) {
      assert.strictEqual(applyEdit(original, formatted), formatted);
    }
  });
});
