#!/bin/bash

function main() {
    install_docker_rootless_mode

    cli_root_path=$(cd $(dirname $0) && cd .. && pwd -P)
    cli_path="$cli_root_path/target/debug/namui-cli"
    cargo_bin_dir_path=$(dirname $(which cargo))
    electron_root_path="$cli_root_path/electron"

    check_cargo_installed
    check_wasm_pack_installed
    check_npm_installed
    check_cargo_bin_dir_exist $cargo_bin_dir_path

    build_cli $cli_root_path

    make_cli_symlink $cargo_bin_dir_path $cli_path

    install_npm_packages $cli_root_path

    install_electron $electron_root_path
    if [ $(is_os_wsl) -eq 1 ]; then
        export WSL_INTEROP=
        for socket in /run/WSL/*; do
            if ss -elx | grep -q "$socket"; then
                export WSL_INTEROP=$socket
            else
                rm $socket 
            fi
        done

        if [[ -z $WSL_INTEROP ]]; then
            echo -e "\033[31mNo working WSL_INTEROP socket found !\033[0m" 
        fi

        window_electron_root_path="$(wslpath $(wslvar APPDATA))/namui/electron"
        window_electron_exe_path="$window_electron_root_path/node_modules/electron/dist/electron.exe"

        install_electron_in_window $window_electron_root_path
        install_dot_env_file $electron_root_path $window_electron_exe_path
    fi

    echo "Successfully installed."
}

# Error Code
EXIT_CARGO_NOT_FOUND=1
EXIT_WASM_PACK_NOT_FOUND=2
EXIT_CARGO_BIN_DIR_NOT_FOUND=3
EXIT_CLI_BUILD_FAILED=4
EXIT_SYMLINK_MAKE_FAILED=5
EXIT_NPM_NOT_FOUND=6
EXIT_ELECTRON_INSTALL_FAILED=7
EXIT_ELECTRON_ON_WINDOWS_INSTALL_FAILED=8
ELECTRON_DOT_ENV_FILE_INSTALL_FAILED=9
EXIT_NPM_INSTALL_FAILED=7

function check_cargo_installed() {
    cargo --version
    if [ $? -ne 0 ]; then
        echo "Cargo command execution failed. Is there a cargo installed?"
        exit $EXIT_CARGO_NOT_FOUND
    fi
}

function check_wasm_pack_installed() {
    wasm-pack --version
    if [ $? -ne 0 ]; then
        echo "Wasm-pack command execution failed. Is there a wasm-pack installed?\nIf not, install it with \"cargo install wasm-pack\"."
        exit $EXIT_WASM_PACK_NOT_FOUND
    fi
}

function check_npm_installed() {
    npm --version
    if [ $? -ne 0 ]; then
        echo "npm command execution failed. Is there a npm installed?"
        exit $EXIT_NPM_NOT_FOUND
    fi
}

#######################################
# Arguments:
#   cargo_bin_dir_path: string
#######################################
function check_cargo_bin_dir_exist() {
    cargo_bin_dir_path=$1
    if [ ! -d $cargo_bin_dir_path ]; then
        echo "Could not find dir \"$cargo_bin_dir_path\". Is there a cargo installed?"
        exit $EXIT_CARGO_BIN_DIR_NOT_FOUND
    fi
}

#######################################
# Arguments:
#   cli_root_path: string
#######################################
function build_cli() {
    cli_root_path=$1
    echo $cli_root_path
    $(cd $cli_root_path && cargo build)
    if [ $? -ne 0 ]; then
        echo "Build failed."
        exit $EXIT_CLI_BUILD_FAILED
    fi
}

#######################################
# Arguments:
#   cargo_bin_dir_path: string
#   cli_path: string
#######################################
function make_cli_symlink() {
    cargo_bin_dir_path=$1
    cli_path=$2
    cd $cargo_bin_dir_path && ln -sf $cli_path namui
    if [ $? -ne 0 ]; then
        echo "Link failed."
        exit $EXIT_SYMLINK_MAKE_FAILED
    fi
}

#######################################
# Arguments:
#   electron_root_path: string
#######################################
function install_electron() {
    electron_root_path=$1
    cd $electron_root_path && npm i
    if [ $? -ne 0 ]; then
        echo "npm package install failed"
        exit $EXIT_ELECTRON_INSTALL_FAILED
    fi
}

#######################################
# Arguments:
#   cli_root_path: string
#######################################
function install_npm_packages() {
    cli_path=$1
    cd $cli_path && npm i
    if [ $? -ne 0 ]; then
        echo "npm package install failed"
        exit $EXIT_NPM_INSTALL_FAILED
    fi
}

function is_os_wsl() {
    # https://github.com/microsoft/WSL/issues/423
    if [ $(which explorer.exe) ]; then
        echo 1
    else
        echo 0
    fi
}

#######################################
# Arguments:
#   window_electron_root_path: string
#######################################
function install_electron_in_window() {
    window_electron_root_path=$1
    npm i electron --prefix $window_electron_root_path --platform=win32
    if [ $? -ne 0 ]; then
        echo "Electron on windows install failed"
        exit $EXIT_ELECTRON_ON_WINDOWS_INSTALL_FAILED
    fi
}

#######################################
# Arguments:
#   electron_root_path: string
#   window_electron_exe_path: string
#######################################
function install_dot_env_file() {
    electron_root_path=$1
    window_electron_exe_path=$2

    wsl_env_path="$electron_root_path/.wsl-env"
    echo "WINDOWS_ELECTRON_EXE_PATH=$window_electron_exe_path" >$wsl_env_path
    if [ $? -ne 0 ]; then
        echo "Electron dot env file install failed"
        exit $ELECTRON_DOT_ENV_FILE_INSTALL_FAILED
    fi
}

function install_docker_rootless_mode() {
    cargo --version
    if [ ! $(which newuidmap) ]; then
        echo "Installing newuidmap..."
        sudo apt install uidmap
    fi

    if [ ! $(which docker) ]; then
        echo "Please install docker."
        exit 1
    fi

    docker_version=$(docker version --format '{{.Server.Version}}')
    version_major=$(echo "$docker_version"| cut -d'.' -f 1)

    if [ $version_major -lt 20 ]; then
        echo "Please update docker to 20.xx or higher."
        exit 1
    fi

    subuid=$(grep ^$(whoami): /etc/subgid | cut -d':' -f 3)
    subgid=$(grep ^$(whoami): /etc/subuid | cut -d':' -f 3)
    if [ $subuid -lt 65536 ] || [ $subgid -lt 65536 ]; then
        echo "/etc/subuid and /etc/subgid should contain at least 65,536 subordinate UIDs/GIDs for the user."
        exit 1
    fi

    if ! dpkg -l | grep -q dbus-user-session; then
        echo "Installing dbus-user-session..."
        sudo apt-get install -y dbus-user-session
    fi

    echo "If you have a problem, please reinstall docker like this doc, https://docs.docker.com/engine/install/ubuntu/"
    echo "Setting docker as rootress mode..."

    if ! dpkg -l | grep -q docker-ce-rootless-extras; then
        echo "Installing docker-ce-rootless-extras..."
        sudo apt-get install -y docker-ce-rootless-extras
    fi

    if [! -d ~/.config/systemd/user/docker.service]; then
        dockerd-rootless-setuptool.sh install --force

        export PATH=/usr/bin:$PATH
        export DOCKER_HOST=unix:///run/user/1000/docker.sock
        grep -qxF 'PATH=/usr/bin:$PATH' ~/.bashrc || echo 'PATH=/usr/bin:$PATH' >> ~/.bashrc
        grep -qxF 'DOCKER_HOST=unix:///run/user/1000/docker.sock' ~/.bashrc || echo 'DOCKER_HOST=unix:///run/user/1000/docker.sock' >> ~/.bashrc
        # Should I set Docker Host like `export DOCKER_HOST=unix://$XDG_RUNTIME_DIR/docker.sock`?
    
        systemctl --user start docker
        systemctl --user enable docker
    fi

    if ! loginctl list-users | grep -q $(whoami); then
        sudo loginctl enable-linger $(whoami)
    fi
    
    if ! docker context use rootless; then
        echo "Fail to set docker context as rootless"
        exit 1
    fi
}

main
