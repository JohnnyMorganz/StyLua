import * as vscode from "vscode";
import * as unzip from "unzipper";
import * as util from "./util";
import fetch from "node-fetch";
import { createWriteStream } from "fs";
import { executeStylua } from "./stylua";
import { GitHub, GitHubRelease } from "./github";
import which = require("which");

enum ResolveMode {
  configuration = "configuration",
  path = "PATH",
  bundled = "bundled",
}

interface StyluaInfo {
  path: string;
  resolveMode: ResolveMode;
  version?: string | undefined;
}

const getStyluaVersion = async (path: string, cwd?: string) => {
  try {
    const currentVersion = (
      await executeStylua(path, ["--version"], cwd)
    )?.trim();
    return currentVersion.substring("stylua ".length);
  } catch (err) {
    return undefined;
  }
};

export class StyluaDownloader {
  constructor(
    private readonly storageDirectory: vscode.Uri,
    private readonly github: GitHub
  ) {}

  public async findStylua(): Promise<StyluaInfo> {
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
      const resolvedPath = await which("stylua", { nothrow: true });
      if (resolvedPath) {
        // TODO: foreman/aftman handling
        return { path: resolvedPath, resolveMode: ResolveMode.path };
      }
    }

    // 3) Fallback to bundled stylua version
    const downloadPath = vscode.Uri.joinPath(
      this.storageDirectory,
      util.getDownloadOutputFilename()
    );
    return { path: downloadPath.fsPath, resolveMode: ResolveMode.bundled };
  }

  public async ensureStyluaExists(
    cwd?: string
  ): Promise<StyluaInfo | undefined> {
    const stylua = await this.findStylua();

    if (stylua.resolveMode === ResolveMode.bundled) {
      if (!(await util.fileExists(stylua.path))) {
        await vscode.workspace.fs.createDirectory(this.storageDirectory);
        await this.downloadStyLuaVisual(util.getDesiredVersion());
      }
      stylua.version = await getStyluaVersion(stylua.path, cwd);

      // TODO: Check bundled version matches requested version
      if (
        !vscode.workspace.getConfiguration("stylua").get("disableVersionCheck")
      ) {
        try {
          const desiredVersion = util.getDesiredVersion();
          const release = await this.github.getRelease(desiredVersion);
          if (
            stylua.version !==
            (release.tagName.startsWith("v")
              ? release.tagName.substr(1)
              : release.tagName)
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
                  return this.ensureStyluaExists(cwd);
                }
            }
          }
        }
      }
    } else if (stylua.resolveMode === ResolveMode.configuration) {
      if (!(await util.fileExists(stylua.path))) {
        vscode.window.showErrorMessage(
          `The path given for StyLua (${stylua.path}) does not exist`
        );
        return;
      }
      stylua.version = await getStyluaVersion(stylua.path, cwd);
    } else if (stylua.resolveMode === ResolveMode.path) {
      stylua.version = await getStyluaVersion(stylua.path, cwd);
    }

    return stylua;
  }

  public downloadStyLuaVisual(version: string): Thenable<void> {
    return vscode.window.withProgress(
      {
        cancellable: false,
        location: vscode.ProgressLocation.Notification,
        title: `Downloading StyLua (${version})`,
      },
      () => this.downloadStylua(version)
    );
  }

  private async downloadStylua(version: string): Promise<void> {
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
            this.downloadStyLuaVisual(release.tagName);
            break;
          case "Release Notes":
            vscode.env.openExternal(vscode.Uri.parse(release.htmlUrl));
            this.openUpdatePrompt(release);
            break;
        }
      });
  }
}
