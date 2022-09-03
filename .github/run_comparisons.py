import os
import sys
import subprocess
from typing import List, Literal

REPOS = {
    "roact": {
        "url": "https://github.com/Roblox/roact.git",
        "command": "src"
    },
    "neovim": {
        "url": "https://github.com/neovim/neovim.git",
        "command": "."
    },
    "zombie-strike": {
        "url": "https://github.com/Kampfkarren/zombie-strike.git",
        "command": "src"
    },
    "nvim-lspconfig": {
        "url": "https://github.com/neovim/nvim-lspconfig.git",
        "command": "."
    },
    "nvim-treesitter": {
        "url": "https://github.com/nvim-treesitter/nvim-treesitter.git",
        "command": "."
    },
    "luvit": {
        "url": "https://github.com/luvit/luvit/",
        "command": "."
    },
    "lit": {
        "url": "https://github.com/luvit/lit/",
        "command": "libs commands deps"
    },
    # "BlizzardInterfaceCode": {
    #     "url": "https://github.com/tomrus88/BlizzardInterfaceCode",
    #     "command": "."
    # }
}

# Get formatting type
formattingType: Literal["diffAfterMainFormat", "diffMainVsChangeFormat"] = sys.argv[1] or "diffAfterMainFormat"  # type: ignore

print(f"RUNNING MODE: {formattingType}", file=sys.stderr)

os.chmod("./stylua-main", 0o700)
os.chmod("./stylua-latest", 0o700)
os.system('git config --global user.email "41898282+github-actions[bot]@users.noreply.github.com"')
os.system('git config --global user.name "github-actions[bot]"')

def executeTool(toolPath: str, command: str):
    # toolPath = os.path.join("../", tool)
    return subprocess.Popen([toolPath, *command.split()], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

diffs: List[str] = []

print("## Repo Comparison Test")
print()

# r = re.compile('`+')
def printCodeblock(content: str, lang: str = "diff"):
    # sequences: List[str] = r.findall(content)
    # maxLen = max(map(len, sequences))
    ticks = "```"
    print(ticks + lang)
    print(content)
    print(ticks)

# Run the comparison tool on different repositories
for repo, data in REPOS.items():
    print(f"Working on {repo}", file=sys.stderr)

    # Checkout the repository
    cloneProcess = subprocess.Popen(["git", "clone", data['url'], "--depth=1"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    cloneProcessStderr = cloneProcess.communicate()[1].decode()
    if cloneProcess.wait() != 0:
        print(f"**Error when cloning `{repo}`**:")
        printCodeblock(cloneProcessStderr or "<no output>", "")
        continue

    os.chdir(repo)

    print(f"Repo cloned and tool prepared", file=sys.stderr)

    # Run the base tool on the repository
    runMainProcess = executeTool("../stylua-main", data["command"])
    runMainStderr = runMainProcess.communicate()[1].decode()
    if runMainStderr and runMainStderr.strip() != "":
        print(f"**Error when running main on `{repo}`**:")
        printCodeblock(runMainStderr, "")

    print(f"Main tool executed", file=sys.stderr)

    # Commit the current changes
    commitProcess = subprocess.Popen(["git", "commit", "-a", "--allow-empty", "--no-verify", "-m", "base"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    commitProcessStderr = commitProcess.communicate()[1].decode()
    if commitProcess.wait() != 0:
        print(f"**Error when committing main changes on `{repo}`**:")
        printCodeblock(commitProcessStderr or "<no output>", "")
        continue

    print(f"Main changes committed", file=sys.stderr)

    # If we are diffing main vs change formatting, then reset to original code
    if formattingType == "diffMainVsChangeFormat":
        print(f"Restoring original code", file=sys.stderr)
        restoreProcess = subprocess.Popen(["git", "checkout", "HEAD~1", "."], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        restoreProcessStderr = restoreProcess.communicate()[1].decode()
        if restoreProcess.wait() != 0:
            print(f"**Error when restoring original changes on `{repo}`**:")
            printCodeblock(restoreProcessStderr or "<no output>", "")
            continue

    # Run the latest tool on the repository
    runLatestProcess = executeTool("../stylua-latest", data["command"])
    runLatestStderr = runLatestProcess.communicate()[1].decode()
    if runLatestStderr and runLatestStderr.strip() != "":
        print(f"**Error when running latest on `{repo}`**:")
        printCodeblock(runLatestStderr, "")

    print(f"Latest tool executed", file=sys.stderr)

    # If we are diffing main vs change formatting, we need to stage the changes
    if formattingType == "diffMainVsChangeFormat":
        print(f"Stage latest changes", file=sys.stderr)
        stageProcess = subprocess.Popen(["git", "add", "--all"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        stageProcessStderr = stageProcess.communicate()[1].decode()
        if stageProcess.wait() != 0:
            print(f"**Error when staging new changes on `{repo}`**:")
            printCodeblock(stageProcessStderr or "<no output>", "")
            continue

    # Compute the diff
    diffProcess = subprocess.Popen(['git', 'diff', f"--src-prefix=ORI/{repo}/", f"--dst-prefix=ALT/{repo}/"], stdout=subprocess.PIPE)
    diffStdout = diffProcess.communicate()[0].decode('utf-8')

    if diffStdout and diffStdout.strip() != "":
        diffs.append(diffStdout)

    print(f"Diff calculated, cleaning up", file=sys.stderr)

    # Cleanup: move out of the repository
    os.chdir("..")

# Report out the diffs
if len(diffs) == 0:
    print("**No diff produced**")
else:
    for diff in diffs:
        printCodeblock(diff)
        print()

