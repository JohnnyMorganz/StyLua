import * as vscode from "vscode";
import * as os from "os";
import * as which from "which";
import { formatCode, checkIgnored, executeStylua } from "./stylua";

/**
 * Convert a Position within a Document to a byte offset.
 * Required as `document.offsetAt(position)` returns a char offset, causing inconsistencies when sending over to StyLua
 * @param document The document to retrieve the byte offset in
 * @param position The position to retrieve the byte offset for
 */
const byteOffset = (
  document: vscode.TextDocument,
  position: vscode.Position
) => {
  // Retrieve all the text from the start of the document to the position provided
  const textRange = new vscode.Range(document.positionAt(0), position);
  const text = document.getText(textRange);

  // Retrieve the byte length of the text range in a buffer
  return Buffer.byteLength(text);
};

enum ResolveMode {
  configuration = "configuration",
  path = "PATH",
  bundled = "bundled",
}

type StyluaInstall = {
  path: string;
  resolveMode: ResolveMode;
};

const findStylua = (context: vscode.ExtensionContext): StyluaInstall => {
  // 1) If `stylua.styluaPath` has been specified, use that directly
  const settingPath = vscode.workspace
    .getConfiguration("stylua")
    .get<string | null>("styluaPath");
  if (settingPath) {
    return { path: settingPath, resolveMode: ResolveMode.configuration };
  }

  // 2) Find a `stylua` binary available on PATH
  if (
    vscode.workspace
      .getConfiguration("stylua")
      .get<boolean>("searchBinaryInPATH")
  ) {
    const resolvedPath = which.sync("stylua", { nothrow: true });
    if (resolvedPath) {
      return { path: resolvedPath, resolveMode: ResolveMode.path };
    }
  }

  // 3) Fallback to bundled stylua version
  const bundledPath = vscode.Uri.joinPath(
    context.extensionUri,
    "bin",
    os.platform() === "win32" ? "stylua.exe" : "stylua"
  );

  return { path: bundledPath.fsPath, resolveMode: ResolveMode.bundled };
};

const findStyluaWithLogs = (context: vscode.ExtensionContext) => {
  const styluaInstall = findStylua(context);
  console.log(`stylua: found binary at ${styluaInstall.path}`);
  console.log(`stylua: binary resolution mode: ${styluaInstall.resolveMode}`);

  const cwdForVersionDetection =
    vscode.workspace.workspaceFolders?.[0].uri.fsPath;

  executeStylua(styluaInstall.path, ["--version"], cwdForVersionDetection)
    .then((version) => {
      console.log(`stylua: version ${version}`);
    })
    .catch((err) => {
      console.log(`stylua: failed to find version: ${err}`);
    });

  return styluaInstall;
};

export async function activate(context: vscode.ExtensionContext) {
  console.log("stylua: activated");

  let styluaInstall = findStyluaWithLogs(context);

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration(async (change) => {
      if (change.affectsConfiguration("stylua")) {
        styluaInstall = findStyluaWithLogs(context);
      }
    })
  );

  let disposable = vscode.languages.registerDocumentRangeFormattingEditProvider(
    ["lua", "luau"],
    {
      async provideDocumentRangeFormattingEdits(
        document: vscode.TextDocument,
        range: vscode.Range,
        _options: vscode.FormattingOptions,
        _token: vscode.CancellationToken
      ) {
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
            styluaInstall.path,
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
