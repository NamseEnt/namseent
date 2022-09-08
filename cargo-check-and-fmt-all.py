#!/usr/bin/env python3

import os
from platform import uname


def in_wsl() -> bool:
    return 'microsoft-standard' in uname().release


for menifast_path in os.popen("git ls-files | grep -e /Cargo.toml").read().splitlines():
    print("cargo check " + menifast_path)
    dir_path = os.path.dirname(menifast_path)
    exit = os.system(
        f"cd {dir_path} && cargo check")
    if exit != 0:
        exit(1)

    print("cargo fmt " + menifast_path)
    dir_path = os.path.dirname(menifast_path)
    exit = os.system(
        f"cd {dir_path} && cargo fmt")
    if exit != 0:
        exit(1)

if in_wsl():
    os.system("powershell.exe '[console]::beep(261.6,700)'")
else:
    # I don't know how to make beep sound in other OS
    pass
