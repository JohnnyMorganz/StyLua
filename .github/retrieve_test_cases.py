import os
import shutil

FULL_MOON_CLONE = "git clone https://github.com/Kampfkarren/full-moon.git"
FULL_MOON_LUA_TESTS_DIRS = "./full-moon/full-moon/tests/cases/pass"
FULL_MOON_LUAU_TESTS_DIR = "./full-moon/full-moon/tests/roblox_cases/pass"

FULL_MOON_LUA_OUTPUT = "./tests/inputs-full_moon"
FULL_MOON_LUAU_OUTPUT = "./tests/inputs-luau-full_moon"

# Clone the relevant repositories
os.system(FULL_MOON_CLONE)

# Create test outputs if not present
os.makedirs(FULL_MOON_LUA_OUTPUT, exist_ok=True)
os.makedirs(FULL_MOON_LUAU_OUTPUT, exist_ok=True)

# Clear old tests
def delete_children(directory):
    for root, dirs, files in os.walk(directory):
        for f in files:
            os.unlink(os.path.join(root, f))
        for d in dirs:
            shutil.rmtree(os.path.join(root, d))

delete_children(FULL_MOON_LUA_OUTPUT)
delete_children(FULL_MOON_LUAU_OUTPUT)

# Copy new tests
def copy_test_files(input, output):
    for test in os.listdir(input):
        source_file = os.path.join(input, test, "source.lua")
        shutil.copyfile(source_file, os.path.join(output, f"{test}.lua"))

copy_test_files(FULL_MOON_LUA_TESTS_DIRS, FULL_MOON_LUA_OUTPUT)
copy_test_files(FULL_MOON_LUAU_TESTS_DIR, FULL_MOON_LUAU_OUTPUT)

# Cleanup the cloned repositories
shutil.rmtree("./full-moon", ignore_errors=True)
