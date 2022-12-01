#!/usr/bin/env python3

import os
from platform import uname


def in_wsl() -> bool:
    return 'microsoft-standard' in uname().release


def run():
    for menifast_path in os.popen("git ls-files | grep -e /Cargo.toml").read().splitlines():
        dir_path = os.path.dirname(menifast_path)
        print(f"cd {dir_path} && cargo update")
        exit = os.system(
            f"cd {dir_path} && cargo update")
        if exit != 0:
            print(f"\n\n-- fail cargo update on {dir_path}\n\n")
            return


run()
if in_wsl():
    os.system("powershell.exe '[console]::beep(261.6,700)'")
else:
    # I don't know how to make beep sound in other OS
    pass
