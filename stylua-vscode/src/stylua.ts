import { spawn, exec } from "child_process";
import ignore from "ignore";
import * as path from "path";
import * as fs from "fs";
import { fileExists } from "./util";

export async function checkIgnored(
  fileName?: string,
  cwd?: string
): Promise<boolean> {
  if (!fileName || !cwd) {
    return false;
  }

  const ignoreFilePath = path.join(cwd, ".styluaignore");
  if (await fileExists(ignoreFilePath)) {
    const ig = ignore().add(fs.readFileSync(ignoreFilePath).toString());
    return ig.ignores(path.relative(cwd, fileName));
  }

  return false;
}

export function formatCode(
  path: string,
  code: string,
  cwd?: string
): Promise<string> {
  return new Promise((resolve, reject) => {
    const child = spawn(`${path}`, ["-"], {
      cwd,
    });
    child.stdout.on("data", (data) => {
      child.kill(); // The process should close on its own, but we kill it here anyways
      resolve(data.toString());
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
      `${path} ${args?.join(" ") ?? ""}`,
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
