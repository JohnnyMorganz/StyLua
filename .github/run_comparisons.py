import os
import sys
import subprocess
import re
from typing import List

REPOS = {
    "roact": {
        "url": "https://github.com/Roblox/roact.git",
        "command": "src"
    }
}

# Get paths to downloaded executables
master_tool = sys.argv[1]
latest_tool = sys.argv[2]

def executeTool(tool: str, command: str):
    toolPath = os.path.join("../", tool)
    os.system(f"{toolPath} {command}")

diffs: List[str] = []

# Run the comparison tool on different repositories
for repo, data in REPOS.items():
    # Checkout the repository
    os.system(f"git clone {data['url']} --depth=1")

    # Move into the repository
    os.system("cd {repo}")

    # Run the base tool on the repository
    executeTool(master_tool, data["command"])

    # Commit the current changes
    os.system(f"git commit --allow-empty --no-verify -m Base StyLua")

    # Run the latest tool on the repository
    executeTool(latest_tool, data["command"])

    # Compute the diff
    diffProcess = subprocess.Popen(['git', 'diff', f"--src-prefix=ORI/{repo}/", f"--dst-prefix=ALT/{repo}/"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    diffStdout = diffProcess.communicate()[0].decode('utf-8')

    if diffStdout and diffStdout.strip() != "":
        diffs.append(diffStdout)

    # Cleanup: move out of the repository
    os.system("cd ..")

print("## Repo Comparison Test")
print()

r = re.compile('`+')

def printDiff(content: str):
    sequences: List[str] = r.findall(content)
    maxLen = max(map(len, sequences))
    ticks = "`" * max(3, maxLen + 1)
    print(ticks + "diff")
    print(content)
    print(ticks)

# Report out the diffs
if len(diffs) == 0:
    print("**No diff produced**")
else:
    for diff in diffs:
        printDiff(diff)
        print()

