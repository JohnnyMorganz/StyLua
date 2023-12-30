import * as vscode from "vscode";
import * as semver from "semver";
import { formatCode, checkIgnored } from "./stylua";
import { GitHub } from "./github";
import { StyluaDownloader } from "./download";
import { getDesiredVersion } from "./util";

const documentSelector = ["lua", "luau"];

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

class StatusInfo implements vscode.Disposable {
  statusItem: vscode.LanguageStatusItem;
  version: string | undefined;

  constructor() {
    this.statusItem = vscode.languages.createLanguageStatusItem(
      "stylua",
      documentSelector
    );
    this.statusItem.name = "StyLua";
    this.statusItem.command = {
      title: "Show Output",
      command: "stylua.showOutputChannel",
    };
    this.updateReady();
  }

  setVersion(version: string | undefined) {
    this.version = version;
    this.updateReady();
  }

  getStyluaText() {
    return this.version ? `StyLua (${this.version})` : "StyLua";
  }

  updateReady() {
    this.statusItem.text = `$(check) ${this.getStyluaText()}`;
    this.statusItem.detail = "Ready";
    this.statusItem.severity = vscode.LanguageStatusSeverity.Information;
  }

  updateFormatSuccess() {
    this.statusItem.text = `$(check) ${this.getStyluaText()}`;
    this.statusItem.detail = "File formatted successfully";
    this.statusItem.severity = vscode.LanguageStatusSeverity.Information;
  }

  updateFormatFailure() {
    this.statusItem.text = `${this.getStyluaText()}`;
    this.statusItem.detail = "Failed to format file";
    this.statusItem.severity = vscode.LanguageStatusSeverity.Error;
  }

  dispose() {
    this.statusItem.dispose();
  }
}

export async function activate(context: vscode.ExtensionContext) {
  console.log("stylua activated");

  const outputChannel = vscode.window.createOutputChannel("StyLua", {
    log: true,
  });
  outputChannel.info("StyLua activated");

  const statusItem = new StatusInfo();
  const github = new GitHub();
  context.subscriptions.push(github);

  const downloader = new StyluaDownloader(context.globalStorageUri, github);

  let cwdForVersionDetection =
    vscode.workspace.workspaceFolders?.[0].uri.fsPath;

  let styluaBinaryPath = await downloader.ensureStyluaExists(
    cwdForVersionDetection
  );
  statusItem.setVersion(styluaBinaryPath?.version);

  context.subscriptions.push(
    vscode.commands.registerCommand("stylua.reinstall", async () => {
      await downloader.downloadStyLuaVisual(getDesiredVersion());
      styluaBinaryPath = await downloader.ensureStyluaExists(
        cwdForVersionDetection
      );
      statusItem.setVersion(styluaBinaryPath?.version);
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
        await downloader.downloadStyLuaVisual(updateConfigValue);
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
        statusItem.setVersion(styluaBinaryPath?.version);
      }
    })
  );

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
            styluaBinaryPath.path,
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
          statusItem.updateFormatSuccess();
          return [format];
        } catch (err) {
          statusItem.updateFormatFailure();
          outputChannel.error(err as string);
          return [];
        }
      },
    }
  );

  context.subscriptions.push(disposable);
  context.subscriptions.push(statusItem);
  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      statusItem.updateReady();
    })
  );
}

// this method is called when your extension is deactivated
export function deactivate() {}
