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
  // When the binary is missing (ENOENT), the promise must reject rather than
  // resolve with an empty string, which would otherwise cause the document to
  // be replaced with empty content.
  test("rejects when the binary path does not exist", async () => {
    let didResolve = false;

    try {
      await formatCode(
        outputChannel,
        "/nonexistent/path/to/stylua",
        "local x = 1\n"
      );
      didResolve = true;
    } catch (_err) {
      // Expected: promise should reject, not resolve
    }

    assert.strictEqual(
      didResolve,
      false,
      "formatCode resolved with empty string instead of rejecting on ENOENT"
    );
  });
});
