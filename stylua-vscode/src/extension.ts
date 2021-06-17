import * as vscode from "vscode";
import * as util from "./util";
import { formatCode, checkIgnored } from "./stylua";

/**
 * Convert a Position within a Document to a byte offset.
 * Required as `document.offsetAt(position)` returns a char offset, causing incosistencies when sending over to StyLua
 * @param document The document to retreive the byte offset in
 * @param position The possition to retreive the byte offset for
 */
const byteOffset = (
  document: vscode.TextDocument,
  position: vscode.Position
) => {
  // Retreive all the text from the start of the document to the position provided
  const textRange = new vscode.Range(document.positionAt(0), position);
  const text = document.getText(textRange);

  // Retreive the byte length of the text range in a buffer
  return Buffer.byteLength(text);
};

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

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration(async (change) => {
      if (change.affectsConfiguration("stylua")) {
        styluaBinaryPath = await util.ensureStyluaExists(
          context.globalStorageUri
        );
      }
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

        const currentWorkspace = vscode.workspace.getWorkspaceFolder(
          document.uri
        );
        const cwd = currentWorkspace?.uri?.fsPath;

        if (await checkIgnored(document.uri, currentWorkspace?.uri)) {
          return [];
        }

        const text = document.getText();

        try {
          const formattedText = await formatCode(
            styluaBinaryPath,
            text,
            cwd,
            byteOffset(document, range.start),
            byteOffset(document, range.end)
          );
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
