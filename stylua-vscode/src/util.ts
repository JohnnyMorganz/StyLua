// Based off https://github.com/Kampfkarren/selene/blob/master/selene-vscode/src/util.ts
// Licensed under https://github.com/Kampfkarren/selene/blob/master/LICENSE.md
import * as vscode from "vscode";
import * as os from "os";
import * as fs from "fs";
import * as path from "path";
import * as unzip from "unzipper";
import fetch from "node-fetch";
import { executeStylua } from "./stylua";

const RELEASES_URL = "https://damp-breeze-6671.johnnymorganz.workers.dev/";

type GithubRelease = {
  assets: {
    browser_download_url: string;
    name: string;
  }[];
  tag_name: string;
  html_url: string;
};

const getLatestRelease = async (): Promise<GithubRelease> => {
  return await fetch(RELEASES_URL)
    .then((r) => r.json())
    .catch((err) => {
      vscode.window.showErrorMessage(`Error fetching StyLua releases\n${err}`);
      throw new Error(err);
    });
};

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

export const fileExists = (path: string): Promise<boolean> => {
  return fs.promises
    .stat(path)
    .then(() => true)
    .catch((error) => error.code !== "ENOENT");
};

const downloadStylua = async (outputDirectory: string) => {
  const latestRelease = await getLatestRelease();
  const assetFilename = getAssetFilenamePattern();
  const outputFilename = getDownloadOutputFilename();

  for (const asset of latestRelease.assets) {
    if (assetFilename.test(asset.name)) {
      const file = fs.createWriteStream(
        path.join(outputDirectory, outputFilename),
        {
          mode: 0o755,
        }
      );

      return new Promise(async (resolve, reject) => {
        fetch(asset.browser_download_url, {
          headers: {
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
    () => downloadStylua(outputDirectory.fsPath)
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

  const downloadPath = path.join(
    storageDirectory.fsPath,
    getDownloadOutputFilename()
  );
  if (await fileExists(downloadPath)) {
    return downloadPath;
  }
};

export const ensureStyluaExists = async (
  storageDirectory: vscode.Uri
): Promise<string | undefined> => {
  const path = await getStyluaPath(storageDirectory);

  if (path === undefined) {
    await fs.promises.mkdir(storageDirectory.fsPath, { recursive: true });
    await downloadStyLuaVisual(storageDirectory);
    return await getStyluaPath(storageDirectory);
  } else {
    if (!(await fileExists(path))) {
      throw new Error("Path given for StyLua does not exist");
    }

    const version = (await executeStylua(path, ["--version"]))?.trim();
    const release = await getLatestRelease(); 
    if (version !== `stylua ${release.tag_name.startsWith('v') ? release.tag_name.substr(1) : release.tag_name}`) {
      openUpdatePrompt(storageDirectory, release);
    }

    return path;
  }
};

function openUpdatePrompt(directory: vscode.Uri, release: GithubRelease) {
  vscode.window
    .showInformationMessage(
      `There's an update available for StyLua: ${release.tag_name}`,
      "Install Update",
      "Later",
      "Release Notes"
    )
    .then((option) => {
      switch (option) {
        case "Install Update":
          downloadStyLuaVisual(directory);
          break;
        case "Release Notes":
          vscode.env.openExternal(vscode.Uri.parse(release.html_url));
          openUpdatePrompt(directory, release);
          break;
      }
    });
}
