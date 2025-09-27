#!/usr/bin/env python3
# Run in root of repository
# scripts/release.py <version number>

import sys
import json
import subprocess
from datetime import datetime

README_FILE = "README.md"
CHANGELOG_FILE = "CHANGELOG.md"
CARGO_FILE = "Cargo.toml"
CARGO_LOCK_FILE = "Cargo.lock"
PACKAGE_JSON_FILE = "stylua-npm-bin/package.json"
PACKAGE_LOCK_JSON_FILE = "stylua-npm-bin/package-lock.json"
WASM_PACKAGE_JSON_FILE = "wasm/package.json"

assert len(sys.argv) == 2, "Usage: .github/release.py <version number>"
VERSION = sys.argv[1]

assert not VERSION.startswith("v"), "Do not prefix the version with 'v'"

CHANGELOG_DATE = datetime.now().strftime("%Y-%m-%d")

# Update version in Cargo.toml
old_version = None
new_cargo_toml_lines: list[str] = []
with open(CARGO_FILE, "r") as file:
    for line in file:
        if line.startswith("version = "):
            old_version = line.removeprefix('version = "').strip().removesuffix('"')
            new_line = f'version = "{VERSION}"\n'
            new_cargo_toml_lines.append(new_line)
        else:
            new_cargo_toml_lines.append(line)

assert old_version != None, "Old version not found"

with open(CARGO_FILE, "w") as file:
    file.writelines(new_cargo_toml_lines)

# Update version in CHANGELOG.md
new_changelog_lines: list[str] = []
with open(CHANGELOG_FILE, "r") as file:
    lines = file.readlines()

    for line in lines:
        if line.startswith("## [Unreleased]"):
            new_changelog_lines.append(line)
            new_changelog_lines.append("\n")
            new_changelog_lines.append(f"## [{VERSION}] - {CHANGELOG_DATE}")
            new_changelog_lines.append("\n")
        elif line.startswith("[unreleased]: "):
            new_changelog_lines.append(
                f"[unreleased]: https://github.com/JohnnyMorganz/StyLua/compare/v{VERSION}...HEAD\n"
            )
            new_changelog_lines.append(
                f"[{VERSION}]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v{VERSION}\n"
            )
        else:
            new_changelog_lines.append(line)

with open(CHANGELOG_FILE, "w") as file:
    file.writelines(new_changelog_lines)

# Update version in README.md
README_REPLACEMENTS = (
    ("rev: v{old}", "rev: v{new}"),
    ("StyLua:{old}", "StyLua:{new}"),
    ("stylua@{old}", "stylua@{new}"),
)

new_readme_text = None
with open(README_FILE, "r") as file:
    new_readme_text = file.read()
    for pattern, replacement in README_REPLACEMENTS:
        needle = pattern.format(old=old_version)
        repl = replacement.format(old=old_version, new=VERSION)
        if needle in new_readme_text:
            new_readme_text = new_readme_text.replace(needle, repl)

assert new_readme_text != None
with open(README_FILE, "w") as file:
    file.write(new_readme_text)

# Update version in package.json
package_json_data = None
with open(PACKAGE_JSON_FILE, "r") as t:
    package_json_data = json.load(t)
    package_json_data["version"] = VERSION

with open(PACKAGE_JSON_FILE, "w") as t:
    json.dump(package_json_data, t)

# Update version in wasm package.json
package_json_data = None
with open(WASM_PACKAGE_JSON_FILE, "r") as t:
    package_json_data = json.load(t)
    package_json_data["version"] = VERSION

with open(WASM_PACKAGE_JSON_FILE, "w") as t:
    json.dump(package_json_data, t)

# Update lockfiles
subprocess.run(["cargo", "check"], check=True)
# we expect this command to fail:
subprocess.run(["npm", "install"], cwd="stylua-npm-bin", check=False)

# Run prettier
subprocess.run(
    [
        "npx",
        "prettier",
        "--write",
        README_FILE,
        CHANGELOG_FILE,
        PACKAGE_JSON_FILE,
        WASM_PACKAGE_JSON_FILE,
    ],
    check=True,
)

# Commit
subprocess.run(
    [
        "git",
        "add",
        CHANGELOG_FILE,
        README_FILE,
        PACKAGE_JSON_FILE,
        PACKAGE_LOCK_JSON_FILE,
        WASM_PACKAGE_JSON_FILE,
        CARGO_FILE,
        CARGO_LOCK_FILE,
    ],
    check=True,
)
subprocess.run(["git", "commit", "-m", f"v{VERSION}"], check=True)

# Tag
subprocess.run(["git", "tag", "-a", f"v{VERSION}", "-m", f"v{VERSION}"], check=True)

print("Created commit and tag. Validate with 'git show'")
print("Run `git push` and `git push --tags` to complete release")
