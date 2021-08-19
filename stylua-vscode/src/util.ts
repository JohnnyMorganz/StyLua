// Based off https://github.com/Kampfkarren/selene/blob/master/selene-vscode/src/util.ts
// Licensed under https://github.com/Kampfkarren/selene/blob/master/LICENSE.md
import * as vscode from "vscode";
import * as os from "os";

export const getDownloadOutputFilename = () => {
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

export const getAssetFilenamePattern = () => {
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

export const getDesiredVersion = (): string => {
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
