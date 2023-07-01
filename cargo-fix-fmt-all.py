#!/usr/bin/env python3

import os
from platform import uname


def in_wsl() -> bool:
    return 'microsoft-standard' in uname().release


def run():
    for manifest_path in os.popen("git ls-files | grep -e /Cargo.toml").read().splitlines():
        dir_path = os.path.dirname(manifest_path)
        print(f"cd {dir_path} && cargo fix --allow-dirty --allow-staged")
        exit = os.system(
            f"cd {dir_path} && cargo fix --allow-dirty --allow-staged")
        if exit != 0:
            print(f"\n\n-- fail cargo fix --allow-dirty --allow-staged on {dir_path}\n\n")
            return

        print(f"cd {dir_path} && cargo fmt")
        dir_path = os.path.dirname(manifest_path)
        exit = os.system(
            f"cd {dir_path} && cargo fmt")
        if exit != 0:
            print(f"\n\n-- fail cargo fmt on {dir_path}\n\n")
            return


run()
if in_wsl():
    os.system("powershell.exe '[console]::beep(261.6,700)'")
else:
    # I don't know how to make beep sound in other OS
    pass
