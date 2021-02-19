import * as vscode from "vscode";
import * as util from "./util";
import { formatCode, checkIgnored } from "./stylua";

export async function activate(context: vscode.ExtensionContext) {
  console.log("stylua activated");

  let styluaBinaryPath: string | undefined = await util.ensureStyluaExists(
    context.globalStorageUri
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("stylua.reinstall", async () => {
      await util.downloadStyLuaVisual(context.globalStorageUri);
      styluaBinaryPath = await util.getStyluaPath(context.globalStorageUri);
    })
  );

  let disposable = vscode.languages.registerDocumentRangeFormattingEditProvider(
    "lua",
    {
      async provideDocumentRangeFormattingEdits(
        document: vscode.TextDocument,
        range: vscode.Range,
        options: vscode.FormattingOptions,
        token: vscode.CancellationToken
      ) {
        if (!styluaBinaryPath) {
          vscode.window
            .showErrorMessage(
              "StyLua not found. Could not format file",
              "Install"
            )
            .then((option) => {
              if (option === "Install") {
                util.downloadStyLuaVisual(context.globalStorageUri);
              }
            });
          return [];
        }

        const fileName = document.fileName;
        const cwd = vscode.workspace.getWorkspaceFolder(
          vscode.Uri.file(document.uri.fsPath)
        )?.uri?.fsPath;
        if (await checkIgnored(fileName, cwd)) {
          return [];
        }

        const text = document.getText();

        try {
          const formattedText = await formatCode(styluaBinaryPath, text, cwd, document.offsetAt(range.start), document.offsetAt(range.end));
          // Replace the whole document with our new formatted version
          const lastLineNumber = document.lineCount - 1;
          const fullDocumentRange = new vscode.Range(
            0,
            0,
            lastLineNumber,
            document.lineAt(lastLineNumber).text.length
          );
          const format = vscode.TextEdit.replace(
            fullDocumentRange,
            formattedText
          );
          return [format];
        } catch (err) {
          vscode.window.showErrorMessage(`Could not format file: ${err}`);
          return [];
        }
      },
    }
  );

  context.subscriptions.push(disposable);
}

// this method is called when your extension is deactivated
export function deactivate() {}
