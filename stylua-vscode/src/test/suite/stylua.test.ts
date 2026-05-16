import * as assert from "assert";
import * as vscode from "vscode";
import { formatCode } from "../../stylua";

suite("formatCode", () => {
  let outputChannel: vscode.LogOutputChannel;

  setup(() => {
    outputChannel = vscode.window.createOutputChannel("StyLua Test", {
      log: true,
    });
  });

  teardown(() => {
    outputChannel.dispose();
  });

  // Regression test for https://github.com/JohnnyMorganz/StyLua/issues/1001
  test("rejects when the binary path does not exist", async () => {
    await assert.rejects(
      formatCode(outputChannel, "/nonexistent/path/to/stylua", "local x = 1\n")
    );
  });
});
