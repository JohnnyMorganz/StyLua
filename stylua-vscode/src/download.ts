import * as vscode from "vscode";
import * as unzip from "unzipper";
import * as util from "./util";
import fetch from "node-fetch";
import { createWriteStream } from "fs";
import { executeStylua } from "./stylua";
import { GitHub, GitHubRelease } from "./github";

export class StyluaDownloader {
  constructor(
    private readonly storageDirectory: vscode.Uri,
    private readonly github: GitHub
  ) {}

  public async ensureStyluaExists(): Promise<string | undefined> {
    const path = await this.getStyluaPath();

    if (path === undefined) {
      await vscode.workspace.fs.createDirectory(this.storageDirectory);
      await this.downloadStyLuaVisual();
      return await this.getStyluaPath();
    } else {
      if (!(await util.fileExists(path))) {
        vscode.window.showErrorMessage(
          `The path given for StyLua (${path}) does not exist`
        );
        return;
      }

      if (
        !vscode.workspace.getConfiguration("stylua").get("disableVersionCheck")
      ) {
        try {
          const currentVersion = (
            await executeStylua(path, ["--version"])
          )?.trim();
          const desiredVersion = util.getDesiredVersion();
          const release = await this.github.getRelease(desiredVersion);
          if (
            currentVersion !==
            `stylua ${
              release.tagName.startsWith("v")
                ? release.tagName.substr(1)
                : release.tagName
            }`
          ) {
            this.openUpdatePrompt(release);
          }
        } catch (err) {
          vscode.window.showWarningMessage(
            `Error checking the selected StyLua version, falling back to the currently installed version:\n${err}`
          );
          if (!this.github.authenticated) {
            const option = await vscode.window.showInformationMessage(
              "Authenticating with GitHub can fix rate limits.",
              "Authenticate with GitHub"
            );
            switch (option) {
              case "Authenticate with GitHub":
                if (await this.github.authenticate()) {
                  return this.ensureStyluaExists();
                }
            }
          }
        }
      }

      return path;
    }
  }

  public downloadStyLuaVisual(): Thenable<void> {
    return vscode.window.withProgress(
      {
        cancellable: false,
        location: vscode.ProgressLocation.Notification,
        title: "Downloading StyLua",
      },
      () => this.downloadStylua()
    );
  }

  private async downloadStylua(): Promise<void> {
    const version = util.getDesiredVersion();
    const release = await this.github.getRelease(version);
    const assetFilename = util.getAssetFilenamePattern();
    const outputFilename = util.getDownloadOutputFilename();

    for (const asset of release.assets) {
      if (assetFilename.test(asset.name)) {
        const file = createWriteStream(
          vscode.Uri.joinPath(this.storageDirectory, outputFilename).fsPath,
          {
            mode: 0o755,
          }
        );

        return new Promise(async (resolve, reject) => {
          fetch(asset.downloadUrl, {
            headers: {
              // eslint-disable-next-line @typescript-eslint/naming-convention
              "User-Agent": "stylua-vscode",
            },
          })
            .then((res) => res.body.pipe(unzip.Parse()))
            .then((stream) => {
              stream.on("entry", (entry: unzip.Entry) => {
                if (entry.path !== outputFilename) {
                  entry.autodrain();
                  return;
                }

                entry.pipe(file).on("finish", resolve).on("error", reject);
              });
            });
        });
      }
    }
  }

  private openUpdatePrompt(release: GitHubRelease) {
    vscode.window
      .showInformationMessage(
        `StyLua ${release.tagName} is available to install.`,
        "Install",
        "Later",
        "Release Notes"
      )
      .then((option) => {
        switch (option) {
          case "Install":
            this.downloadStyLuaVisual();
            break;
          case "Release Notes":
            vscode.env.openExternal(vscode.Uri.parse(release.htmlUrl));
            this.openUpdatePrompt(release);
            break;
        }
      });
  }

  public async getStyluaPath(): Promise<string | undefined> {
    const settingPath = vscode.workspace
      .getConfiguration("stylua")
      .get<string | null>("styluaPath");
    if (settingPath) {
      return settingPath;
    }

    const downloadPath = vscode.Uri.joinPath(
      this.storageDirectory,
      util.getDownloadOutputFilename()
    );
    if (await util.fileExists(downloadPath)) {
      return downloadPath.fsPath;
    }
  }
}
