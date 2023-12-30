import * as vscode from "vscode";
import * as semver from "semver";
import { formatCode, checkIgnored } from "./stylua";
import { GitHub } from "./github";
import { StyluaDownloader } from "./download";
import { getDesiredVersion } from "./util";

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

export async function activate(context: vscode.ExtensionContext) {
  console.log("stylua activated");

  const outputChannel = vscode.window.createOutputChannel("StyLua", {
    log: true,
  });
  outputChannel.info("StyLua activated");

  const github = new GitHub();
  context.subscriptions.push(github);

  const downloader = new StyluaDownloader(context.globalStorageUri, github);

  let cwdForVersionDetection =
    vscode.workspace.workspaceFolders?.[0].uri.fsPath;

  let styluaBinaryPath: string | undefined =
    await downloader.ensureStyluaExists(cwdForVersionDetection);

  context.subscriptions.push(
    vscode.commands.registerCommand("stylua.reinstall", async () => {
      await downloader.downloadStyLuaVisual(getDesiredVersion());
      styluaBinaryPath = await downloader.getStyluaPath();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("stylua.authenticate", async () => {
      await github.authenticate();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("stylua.showOutputChannel", async () => {
      outputChannel.show();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("stylua.selectVersion", async () => {
      const versions = (await github.getAllReleases()).sort((a, b) =>
        semver.rcompare(a.tagName, b.tagName)
      );
      if (versions.length === 0) {
        return;
      }
      const latestVersion = versions[0];

      const selectedVersion = await vscode.window.showQuickPick(
        versions
          .sort((a, b) => semver.rcompare(a.tagName, b.tagName))
          .map((release) => {
            if (release.tagName === latestVersion.tagName) {
              return { label: `${release.tagName} (latest)` };
            } else {
              return { label: release.tagName };
            }
          }),
        {
          placeHolder: "Select the version of StyLua to install",
        }
      );

      if (selectedVersion) {
        const updateConfigValue = selectedVersion.label.includes("latest")
          ? "latest"
          : selectedVersion.label;
        await downloader.downloadStyLuaVisual(selectedVersion.label);
        vscode.workspace
          .getConfiguration("stylua")
          .update(
            "targetReleaseVersion",
            updateConfigValue,
            vscode.ConfigurationTarget.Workspace
          );
      }
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration(async (change) => {
      if (change.affectsConfiguration("stylua")) {
        styluaBinaryPath = await downloader.ensureStyluaExists(
          cwdForVersionDetection
        );
      }
    })
  );

  const documentSelector = ["lua", "luau"];

  const languageStatusItem = vscode.languages.createLanguageStatusItem(
    "stylua",
    documentSelector
  );
  languageStatusItem.name = "StyLua";
  languageStatusItem.text = "$(check) StyLua";
  languageStatusItem.detail = "Ready";
  languageStatusItem.command = {
    title: "Show Output",
    command: "stylua.showOutputChannel",
  };

  let disposable = vscode.languages.registerDocumentRangeFormattingEditProvider(
    documentSelector,
    {
      async provideDocumentRangeFormattingEdits(
        document: vscode.TextDocument,
        range: vscode.Range,
        _options: vscode.FormattingOptions,
        _token: vscode.CancellationToken
      ) {
        if (!styluaBinaryPath) {
          vscode.window
            .showErrorMessage(
              "StyLua not found. Could not format file",
              "Install"
            )
            .then((option) => {
              if (option === "Install") {
                vscode.commands.executeCommand("stylua.reinstall");
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
          languageStatusItem.text = "$(check) StyLua";
          languageStatusItem.detail = "File formatted successfully";
          languageStatusItem.severity =
            vscode.LanguageStatusSeverity.Information;
          return [format];
        } catch (err) {
          languageStatusItem.text = "StyLua";
          languageStatusItem.detail = "Failed to format file";
          languageStatusItem.severity = vscode.LanguageStatusSeverity.Error;
          outputChannel.error(err as string);
          return [];
        }
      },
    }
  );

  context.subscriptions.push(disposable);
  context.subscriptions.push(languageStatusItem);
  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      languageStatusItem.text = "$(check) StyLua";
      languageStatusItem.detail = "Ready";
      languageStatusItem.severity = vscode.LanguageStatusSeverity.Information;
    })
  );
}

// this method is called when your extension is deactivated
export function deactivate() {}
