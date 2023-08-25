#!/usr/bin/env python3

import os
from platform import uname

ignored_projects = list(
    filter(
        lambda line: not line.startswith("#"),
        open(".ignored_projects", "r").readlines(),
    )
)


def in_wsl() -> bool:
    return "microsoft-standard" in uname().release


def run():
    for manifest_path in (
        os.popen("git ls-files | grep -e /Cargo.toml").read().splitlines()
    ):
        dir_path = os.path.dirname(manifest_path)
        if dir_path in ignored_projects:
            continue

        print(f"cd {dir_path} && cargo clean")
        exit = os.system(f"cd {dir_path} && cargo clean")
        if exit != 0:
            print(f"\n\n-- fail cargo clean on {dir_path}\n\n")
            return

        print(f"cd {dir_path} && cargo check")
        exit = os.system(f"cd {dir_path} && cargo check")
        if exit != 0:
            print(f"\n\n-- fail cargo check on {dir_path}\n\n")
            return

        print(f"cd {dir_path} && cargo fmt")
        dir_path = os.path.dirname(manifest_path)
        exit = os.system(f"cd {dir_path} && cargo fmt")
        if exit != 0:
            print(f"\n\n-- fail cargo fmt on {dir_path}\n\n")
            return


run()
if in_wsl():
    os.system("powershell.exe '[console]::beep(261.6,700)'")
else:
    pass
