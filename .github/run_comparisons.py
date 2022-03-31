import os
import subprocess
from typing import List

REPOS = {
    "roact": {
        "url": "https://github.com/Roblox/roact.git",
        "command": "src"
    },
    "neovim": {
        "url": "https://github.com/neovim/neovim.git",
        "command": "."
    }
}

# Get paths to downloaded executables
# master_tool = sys.argv[1]
# latest_tool = sys.argv[2]

os.chmod("./stylua-master", 0o700)
os.chmod("./stylua-latest", 0o700)
os.system('git config --global user.email "41898282+github-actions[bot]@users.noreply.github.com"')
os.system('git config --global user.name "github-actions[bot]"')

def executeTool(toolPath: str, command: str):
    # toolPath = os.path.join("../", tool)
    return subprocess.Popen([toolPath, command], stdout=subprocess.PIPE, stderr=subprocess.PIPE)

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
    # Checkout the repository
    cloneProcess = subprocess.Popen(["git", "clone", data['url'], "--depth=1"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    cloneProcessStderr = cloneProcess.communicate()[1].decode()
    if cloneProcess.wait() != 0:
        print(f"**Error when cloning `{repo}`**:")
        printCodeblock(cloneProcessStderr or "<no output>", "")
        continue

    os.chdir(repo)

    # Run the base tool on the repository
    runMasterProcess = executeTool("../stylua-master", data["command"])
    runMasterStderr = runMasterProcess.communicate()[1].decode()
    if runMasterStderr and runMasterStderr.strip() != "":
        print(f"**Error when running master on `{repo}`**:")
        printCodeblock(runMasterStderr, "")

    # Commit the current changes
    commitProcess = subprocess.Popen(["git", "commit", "--allow-empty", "--no-verify", "-m", "base"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    commitProcessStderr = commitProcess.communicate()[1].decode()
    if commitProcess.wait() != 0:
        print(f"**Error when commiting master changes on `{repo}`**:")
        printCodeblock(commitProcessStderr or "<no output>", "")
        continue

    # Run the latest tool on the repository
    runLatestProcess = executeTool("../stylua-latest", data["command"])
    runLatestStderr = runLatestProcess.communicate()[1].decode()
    if runLatestStderr and runLatestStderr.strip() != "":
        print(f"**Error when running latest on `{repo}`**:")
        printCodeblock(runLatestStderr, "")

    # Compute the diff
    diffProcess = subprocess.Popen(['git', 'diff', f"--src-prefix=ORI/{repo}/", f"--dst-prefix=ALT/{repo}/"], stdout=subprocess.PIPE)
    diffStdout = diffProcess.communicate()[0].decode('utf-8')

    if diffStdout and diffStdout.strip() != "":
        diffs.append(diffStdout)

    # Cleanup: move out of the repository
    os.chdir("..")

# Report out the diffs
if len(diffs) == 0:
    print("**No diff produced**")
else:
    for diff in diffs:
        printCodeblock(diff)
        print()

