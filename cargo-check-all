import os

ignores = [
    ".git",
    "target",
    "node_modules",
    "dist",
    "build",
    "vendor",
    "bin",
    "lib",
    "obj",
    "out",
    "release",
    "src",
    "include",
    "doc",
]


def search_cargo_project_and_run_check(path: str):
    list = os.listdir(path)
    for dir in list:
        if dir in ignores:
            continue
        if os.path.isdir(os.path.join(path, dir)):
            if os.path.isfile(os.path.join(path, dir, "Cargo.toml")):
                print("cargo check " + os.path.join(path, dir))
                exit = os.system(
                    f"cd {os.path.join(path, dir)} && cargo check")
                if exit != 0:
                    exit(1)
            search_cargo_project_and_run_check(os.path.join(path, dir))


search_cargo_project_and_run_check(".")
