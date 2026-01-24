/**
 * Smoke tests for StyLua WASM module
 * These tests verify that the WASM bindings work correctly after build
 */

import assert from "node:assert";
import { describe, it } from "node:test";

// Import the Node.js ESM version
import * as stylua from "../stylua_lib_node.mjs";

describe("StyLua WASM Smoke Tests (ESM)", () => {
  describe("Module exports", () => {
    it("should export formatCode function", () => {
      assert.strictEqual(typeof stylua.formatCode, "function");
    });

    it("should export Config class", () => {
      assert.strictEqual(typeof stylua.Config, "function");
    });

    it("should export Range class", () => {
      assert.strictEqual(typeof stylua.Range, "function");
    });

    it("should export SortRequiresConfig class", () => {
      assert.strictEqual(typeof stylua.SortRequiresConfig, "function");
    });

    it("should export OutputVerification enum", () => {
      assert.ok(stylua.OutputVerification !== undefined);
      assert.strictEqual(typeof stylua.OutputVerification.Full, "number");
      assert.strictEqual(typeof stylua.OutputVerification.None, "number");
    });

    it("should export LuaVersion enum", () => {
      assert.ok(stylua.LuaVersion !== undefined);
      assert.strictEqual(typeof stylua.LuaVersion.All, "number");
      assert.strictEqual(typeof stylua.LuaVersion.Lua51, "number");
    });

    it("should export IndentType enum", () => {
      assert.ok(stylua.IndentType !== undefined);
      assert.strictEqual(typeof stylua.IndentType.Tabs, "number");
      assert.strictEqual(typeof stylua.IndentType.Spaces, "number");
    });

    it("should export LineEndings enum", () => {
      assert.ok(stylua.LineEndings !== undefined);
      assert.strictEqual(typeof stylua.LineEndings.Unix, "number");
      assert.strictEqual(typeof stylua.LineEndings.Windows, "number");
    });

    it("should export QuoteStyle enum", () => {
      assert.ok(stylua.QuoteStyle !== undefined);
      assert.strictEqual(typeof stylua.QuoteStyle.AutoPreferDouble, "number");
      assert.strictEqual(typeof stylua.QuoteStyle.AutoPreferSingle, "number");
      assert.strictEqual(typeof stylua.QuoteStyle.ForceDouble, "number");
      assert.strictEqual(typeof stylua.QuoteStyle.ForceSingle, "number");
    });

    it("should export CallParenType enum", () => {
      assert.ok(stylua.CallParenType !== undefined);
      assert.strictEqual(typeof stylua.CallParenType.Always, "number");
      assert.strictEqual(typeof stylua.CallParenType.NoSingleString, "number");
      assert.strictEqual(typeof stylua.CallParenType.NoSingleTable, "number");
      assert.strictEqual(typeof stylua.CallParenType.None, "number");
      assert.strictEqual(typeof stylua.CallParenType.Input, "number");
    });

    it("should export CollapseSimpleStatement enum", () => {
      assert.ok(stylua.CollapseSimpleStatement !== undefined);
      assert.strictEqual(typeof stylua.CollapseSimpleStatement.Never, "number");
      assert.strictEqual(
        typeof stylua.CollapseSimpleStatement.FunctionOnly,
        "number"
      );
      assert.strictEqual(
        typeof stylua.CollapseSimpleStatement.ConditionalOnly,
        "number"
      );
      assert.strictEqual(
        typeof stylua.CollapseSimpleStatement.Always,
        "number"
      );
    });

    it("should export SpaceAfterFunctionNames enum", () => {
      assert.ok(stylua.SpaceAfterFunctionNames !== undefined);
      assert.strictEqual(
        typeof stylua.SpaceAfterFunctionNames.Never,
        "number"
      );
      assert.strictEqual(
        typeof stylua.SpaceAfterFunctionNames.Definitions,
        "number"
      );
      assert.strictEqual(typeof stylua.SpaceAfterFunctionNames.Calls, "number");
      assert.strictEqual(
        typeof stylua.SpaceAfterFunctionNames.Always,
        "number"
      );
    });
  });

  describe("Config", () => {
    it("should create a default config", () => {
      const config = stylua.Config.new();
      assert.ok(config !== undefined);
    });

    it("should have default column_width of 120", () => {
      const config = stylua.Config.new();
      assert.strictEqual(config.column_width, 120);
    });

    it("should have default indent_width of 4", () => {
      const config = stylua.Config.new();
      assert.strictEqual(config.indent_width, 4);
    });

    it("should allow modifying config values", () => {
      const config = stylua.Config.new();
      config.column_width = 80;
      config.indent_width = 2;
      config.indent_type = stylua.IndentType.Spaces;
      assert.strictEqual(config.column_width, 80);
      assert.strictEqual(config.indent_width, 2);
      assert.strictEqual(config.indent_type, stylua.IndentType.Spaces);
    });
  });

  describe("Range", () => {
    it("should create a range from values", () => {
      const range = stylua.Range.from_values(0, 100);
      assert.ok(range !== undefined);
      assert.strictEqual(range.start, 0);
      assert.strictEqual(range.end, 100);
    });

    it("should support undefined start/end", () => {
      const range = stylua.Range.from_values(undefined, 50);
      assert.ok(range !== undefined);
      assert.strictEqual(range.start, undefined);
      assert.strictEqual(range.end, 50);
    });
  });

  describe("SortRequiresConfig", () => {
    it("should create a default sort requires config", () => {
      const config = stylua.SortRequiresConfig.new();
      assert.ok(config !== undefined);
      assert.strictEqual(config.enabled, false);
    });

    it("should allow setting enabled", () => {
      const config = stylua.SortRequiresConfig.new();
      const newConfig = config.set_enabled(true);
      assert.strictEqual(newConfig.enabled, true);
    });
  });

  describe("formatCode", () => {
    it("should format simple Lua code", () => {
      const config = stylua.Config.new();
      const result = stylua.formatCode(
        "local   x   =    1",
        config,
        undefined,
        stylua.OutputVerification.None
      );
      assert.strictEqual(result, "local x = 1\n");
    });

    it("should format code with custom column width", () => {
      const config = stylua.Config.new();
      config.column_width = 40;
      const code =
        'local very_long_variable_name = "some very long string value here"';
      const result = stylua.formatCode(
        code,
        config,
        undefined,
        stylua.OutputVerification.None
      );
      assert.ok(result.includes("\n")); // Should wrap due to narrow column width
    });

    it("should format code with spaces instead of tabs", () => {
      const config = stylua.Config.new();
      config.indent_type = stylua.IndentType.Spaces;
      config.indent_width = 2;
      const code = "if true then\nreturn 1\nend";
      const result = stylua.formatCode(
        code,
        config,
        undefined,
        stylua.OutputVerification.None
      );
      assert.ok(result.includes("  return")); // 2 spaces for indent
    });

    it("should throw on invalid Lua code", () => {
      const config = stylua.Config.new();
      assert.throws(() => {
        stylua.formatCode(
          "local x = ",
          config,
          undefined,
          stylua.OutputVerification.None
        );
      });
    });

    it("should format with OutputVerification.Full", () => {
      const config = stylua.Config.new();
      const result = stylua.formatCode(
        "local x = 1",
        config,
        undefined,
        stylua.OutputVerification.Full
      );
      assert.strictEqual(result, "local x = 1\n");
    });

    it("should format a range of code", () => {
      const config = stylua.Config.new();
      const range = stylua.Range.from_values(0, 10);
      const result = stylua.formatCode(
        "local   x   =    1\nlocal y = 2",
        config,
        range,
        stylua.OutputVerification.None
      );
      assert.ok(result !== undefined);
    });

    it("should format Lua tables", () => {
      const config = stylua.Config.new();
      const code = "local t = {a=1,b=2,c=3}";
      const result = stylua.formatCode(
        code,
        config,
        undefined,
        stylua.OutputVerification.None
      );
      assert.ok(result.includes("a = 1"));
      assert.ok(result.includes("b = 2"));
      assert.ok(result.includes("c = 3"));
    });

    it("should format function definitions", () => {
      const config = stylua.Config.new();
      const code = "function foo(a,b,c) return a+b+c end";
      const result = stylua.formatCode(
        code,
        config,
        undefined,
        stylua.OutputVerification.None
      );
      assert.ok(result.includes("function foo(a, b, c)"));
    });

    it("should handle different quote styles", () => {
      const config = stylua.Config.new();
      config.quote_style = stylua.QuoteStyle.ForceSingle;
      const code = 'local x = "hello"';
      const result = stylua.formatCode(
        code,
        config,
        undefined,
        stylua.OutputVerification.None
      );
      assert.ok(result.includes("'hello'"));
    });
  });

  describe("Lua syntax versions", () => {
    it("should format with Lua51 syntax", () => {
      const config = stylua.Config.new();
      config.syntax = stylua.LuaVersion.Lua51;
      const result = stylua.formatCode(
        "local x = 1",
        config,
        undefined,
        stylua.OutputVerification.None
      );
      assert.strictEqual(result, "local x = 1\n");
    });

    it("should format with Luau syntax", () => {
      const config = stylua.Config.new();
      config.syntax = stylua.LuaVersion.Luau;
      // Luau-specific type annotation
      const code = "local x: number = 1";
      const result = stylua.formatCode(
        code,
        config,
        undefined,
        stylua.OutputVerification.None
      );
      assert.ok(result.includes("local x: number = 1"));
    });
  });
});
