import * as vscode from "vscode";
import * as unzip from "unzipper";
import * as util from "./util";
import * as semver from "semver";
import fetch from "node-fetch";
import { createWriteStream } from "fs";
import { executeStylua } from "./stylua";
import { GitHub, GitHubRelease } from "./github";
import which = require("which");

export enum ResolveMode {
  configuration = "configuration",
  path = "PATH",
  bundled = "bundled",
}

export interface StyluaInfo {
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

export class StyluaDownloader implements vscode.Disposable {
  statusBarUpdateItem = vscode.window.createStatusBarItem(
    "stylua.installUpdate",
    vscode.StatusBarAlignment.Right
  );

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

      // Check bundled version matches requested version
      const desiredVersion = util.getDesiredVersion();
      if (stylua.version && desiredVersion !== "latest") {
        const desiredVersionSemver = semver.coerce(desiredVersion);
        const styluaVersionSemver = semver.parse(stylua.version);
        if (
          desiredVersionSemver &&
          styluaVersionSemver &&
          semver.neq(desiredVersionSemver, styluaVersionSemver)
        ) {
          this.openIncorrectVersionPrompt(stylua.version, desiredVersion);
        }
      }

      // Check for latest version
      if (
        !vscode.workspace.getConfiguration("stylua").get("disableVersionCheck")
      ) {
        try {
          const latestRelease = await this.github.getRelease("latest");
          if (
            stylua.version !==
            (latestRelease.tagName.startsWith("v")
              ? latestRelease.tagName.substr(1)
              : latestRelease.tagName)
          ) {
            this.showUpdateAvailable(latestRelease);
          } else {
            this.statusBarUpdateItem.hide();
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
      } else {
        this.statusBarUpdateItem.hide();
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

  private showUpdateAvailable(release: GitHubRelease) {
    this.statusBarUpdateItem.name = "StyLua Update";
    this.statusBarUpdateItem.text = `StyLua update available (${release.tagName}) $(cloud-download)`;
    this.statusBarUpdateItem.tooltip = "Click to update StyLua";
    this.statusBarUpdateItem.command = {
      title: "Update StyLua",
      command: "stylua.installUpdate",
      arguments: [release],
    };
    this.statusBarUpdateItem.backgroundColor = new vscode.ThemeColor(
      "statusBarItem.warningBackground"
    );
    this.statusBarUpdateItem.show();
  }

  private openIncorrectVersionPrompt(
    currentVersion: string,
    requestedVersion: string
  ) {
    vscode.window
      .showInformationMessage(
        `The currently installed version of StyLua (${currentVersion}) does not match the requested version (${requestedVersion})`,
        "Install"
      )
      .then((option) => {
        switch (option) {
          case "Install":
            vscode.commands.executeCommand("stylua.reinstall");
            break;
        }
      });
  }

  dispose() {
    this.statusBarUpdateItem.dispose();
  }
}
