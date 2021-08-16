// Based off https://github.com/Kampfkarren/selene/blob/master/selene-vscode/src/util.ts
// Licensed under https://github.com/Kampfkarren/selene/blob/master/LICENSE.md
import * as vscode from "vscode";
import * as os from "os";
import * as fs from "fs";
import * as unzip from "unzipper";
import fetch from "node-fetch";
import { executeStylua } from "./stylua";
import { GitHubRelease } from "./github";

const getDownloadOutputFilename = () => {
  switch (os.platform()) {
    case "win32":
      return "stylua.exe";
    case "linux":
    case "darwin":
      return "stylua";
    default:
      throw new Error("platform not supported");
  }
};

const getAssetFilenamePattern = () => {
  switch (os.platform()) {
    case "win32":
      return /stylua-[\d\w\-\.]+-win64.zip/;
    case "linux":
      return /stylua-[\d\w\-\.]+-linux.zip/;
    case "darwin":
      return /stylua-[\d\w\-\.]+-macos.zip/;
    default:
      throw new Error("Platform not supported");
  }
};

const getDesiredVersion = (): string => {
  const config = vscode.workspace.getConfiguration("stylua");
  const targetVersion = config.get<string>("targetReleaseVersion", "").trim();
  if (targetVersion.length === 0) {
    return config.get<string>("releaseVersion", "latest");
  }
  return targetVersion;
};

export const fileExists = (path: vscode.Uri | string): Thenable<boolean> => {
  const uri = path instanceof vscode.Uri ? path : vscode.Uri.file(path);
  return vscode.workspace.fs.stat(uri).then(
    () => true,
    () => false
  );
};

const downloadStylua = async (outputDirectory: vscode.Uri) => {
  const version = getDesiredVersion();
  const release = await getRelease(version);
  const assetFilename = getAssetFilenamePattern();
  const outputFilename = getDownloadOutputFilename();

  for (const asset of release.assets) {
    if (assetFilename.test(asset.name)) {
      const file = fs.createWriteStream(
        vscode.Uri.joinPath(outputDirectory, outputFilename).fsPath,
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
};

export const downloadStyLuaVisual = (outputDirectory: vscode.Uri) => {
  return vscode.window.withProgress(
    {
      cancellable: false,
      location: vscode.ProgressLocation.Notification,
      title: "Downloading StyLua",
    },
    () => downloadStylua(outputDirectory)
  );
};

export const getStyluaPath = async (
  storageDirectory: vscode.Uri
): Promise<string | undefined> => {
  const settingPath = vscode.workspace
    .getConfiguration("stylua")
    .get<string | null>("styluaPath");
  if (settingPath) {
    return settingPath;
  }

  const downloadPath = vscode.Uri.joinPath(
    storageDirectory,
    getDownloadOutputFilename()
  );
  if (await fileExists(downloadPath)) {
    return downloadPath.fsPath;
  }
};

export const ensureStyluaExists = async (
  storageDirectory: vscode.Uri
): Promise<string | undefined> => {
  const path = await getStyluaPath(storageDirectory);

  if (path === undefined) {
    await vscode.workspace.fs.createDirectory(storageDirectory);
    await downloadStyLuaVisual(storageDirectory);
    return await getStyluaPath(storageDirectory);
  } else {
    if (!(await fileExists(path))) {
      vscode.window.showErrorMessage(
        `The path given for StyLua (${path}) does not exist`
      );
      return;
    }

    try {
      const currentVersion = (await executeStylua(path, ["--version"]))?.trim();
      const desiredVersion = getDesiredVersion();
      const release = await getRelease(desiredVersion);
      if (
        currentVersion !==
        `stylua ${
          release.tagName.startsWith("v")
            ? release.tagName.substr(1)
            : release.tagName
        }`
      ) {
        openUpdatePrompt(storageDirectory, release);
      }
    } catch (err) {
      vscode.window.showWarningMessage(
        `Error checking the selected StyLua version, falling back to the currently installed version:\n${err}`
      );
    }

    return path;
  }
};

function openUpdatePrompt(directory: vscode.Uri, release: GitHubRelease) {
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
          downloadStyLuaVisual(directory);
          break;
        case "Release Notes":
          vscode.env.openExternal(vscode.Uri.parse(release.htmlUrl));
          openUpdatePrompt(directory, release);
          break;
      }
    });
}
