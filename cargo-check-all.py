import os

for menifast_path in os.popen("git ls-files | grep -e /Cargo.toml").read().splitlines():
    print("cargo check " + menifast_path)
    dir_path = os.path.dirname(menifast_path)
    exit = os.system(
        f"cd {dir_path} && cargo check")
    if exit != 0:
        exit(1)
