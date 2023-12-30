import * as vscode from "vscode";
import { spawn, exec } from "child_process";
import ignore from "ignore";
import { fileExists } from "./util";

export async function checkIgnored(
  filePath?: vscode.Uri,
  currentWorkspace?: vscode.Uri
): Promise<boolean> {
  if (!filePath || !currentWorkspace) {
    return false;
  }

  const ignoreFilePath = vscode.Uri.joinPath(currentWorkspace, ".styluaignore");
  if (await fileExists(ignoreFilePath)) {
    try {
      const contents = await vscode.workspace.fs.readFile(ignoreFilePath);
      const ig = ignore().add(contents.toString());
      return ig.ignores(vscode.workspace.asRelativePath(filePath));
    } catch (err) {
      vscode.window.showErrorMessage(
        `Could not read StyLua ignore file at ${ignoreFilePath}:\n${err}`
      );
      return false;
    }
  }

  return false;
}

export function formatCode(
  path: string,
  code: string,
  cwd?: string,
  startPos?: number,
  endPos?: number
): Promise<string> {
  return new Promise((resolve, reject) => {
    const args = [];
    if (startPos) {
      args.push("--range-start");
      args.push(startPos.toString());
    }
    if (endPos) {
      args.push("--range-end");
      args.push(endPos.toString());
    }

    const configPath = vscode.workspace
      .getConfiguration("stylua")
      .get<string>("configPath");
    if (configPath && configPath.trim() !== "") {
      args.push("--config-path");
      args.push(configPath);
    }

    if (
      vscode.workspace.getConfiguration("stylua").get("searchParentDirectories")
    ) {
      args.push("--search-parent-directories");
    }
    if (vscode.workspace.getConfiguration("stylua").get("verify")) {
      args.push("--verify");
    }

    args.push("-");

    const child = spawn(`${path}`, args, {
      cwd,
    });
    let output = "";
    child.stdout.on("data", (data) => {
      output += data.toString();
    });
    child.stdout.on("close", () => {
      resolve(output);
    });
    child.stderr.on("data", (data) => reject(data.toString()));
    child.on("err", (err) => reject("Failed to start StyLua"));

    // Write our code to stdin
    child.stdin.write(code);
    child.stdin.end();
  });
}

export function executeStylua(
  path: string,
  args?: string[],
  cwd?: string
): Promise<string> {
  return new Promise((resolve, reject) => {
    const child = exec(
      `"${path}" ${args?.join(" ") ?? ""}`,
      {
        cwd,
      },
      (err, stdout) => {
        if (err) {
          reject(err);
        }
        resolve(stdout);
      }
    );
  });
}
