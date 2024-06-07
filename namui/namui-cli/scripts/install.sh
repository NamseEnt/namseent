#!/bin/bash

function main() {
    if ! which podman &>/dev/null; then
        sudo apt install -y podman
    fi

    cli_root_path=$(cd $(dirname $0) && cd .. && pwd -P)
    cli_path="$cli_root_path/target/debug/namui-cli"
    cli_completion_root_path="$cli_root_path/target/completions"
    cargo_bin_dir_path=$(dirname $(which cargo))

    check_cargo_installed
    rustup target add wasm32-wasip1-threads
    check_npm_installed
    check_cargo_bin_dir_exist $cargo_bin_dir_path
    check_wine_installed

    build_cli $cli_root_path

    install_completion_script $cli_completion_root_path

    make_cli_symlink $cargo_bin_dir_path $cli_path

    echo "Successfully installed."
}

# Error Code
EXIT_CARGO_NOT_FOUND=1
EXIT_CARGO_BIN_DIR_NOT_FOUND=3
EXIT_CLI_BUILD_FAILED=4
EXIT_SYMLINK_MAKE_FAILED=5
EXIT_NPM_NOT_FOUND=6
EXIT_NPM_INSTALL_FAILED=7
EXIT_REMOVE_OLD_COMPLETION_SCRIPT_FAILED=11

function check_cargo_installed() {
    cargo --version
    if [ $? -ne 0 ]; then
        echo "Cargo command execution failed. Is there a cargo installed?"
        exit $EXIT_CARGO_NOT_FOUND
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

function check_wine_installed() {
    wine --version

    if [ $? -ne 0 ]; then
        echo "Wine command execution failed. Is there a wine installed?"
        exit $EXIT_WINE_NOT_FOUND
    fi

    wine_version=$(wine --version | cut -d '-' -f 2)

    if [ $(echo $wine_version | cut -d '.' -f 1) -lt 8 ]; then
        echo "Wine version is lower than 8. Please install wine version 8 or higher."
        exit $EXIT_WINE_VERSION_TOO_LOW
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
#   cli_completion_root_path: string
#######################################
function install_completion_script() {
    cli_completion_root_path=$1
    completion_script_start_marker="# namui completion script start"
    completion_script_end_marker="# namui completion script end"

    if [ "$BASH" ]; then
        cli_completion_path="$cli_completion_root_path/namui.bash"

        # Remove old completion script
        sed -i "/^$completion_script_start_marker/,/^$completion_script_end_marker/d" ~/.bashrc
        if [ $? -ne 0 ]; then
            echo "Remove old completion script failed"
            exit $EXIT_REMOVE_OLD_COMPLETION_SCRIPT_FAILED
        fi

        # Ensure last line of .bashrc has newline
        sed -i '$a\' ~/.bashrc

        # Add completion script
        echo "$completion_script_start_marker" >>~/.bashrc
        cat $cli_completion_path >>~/.bashrc
        echo -e "\n$completion_script_end_marker" >>~/.bashrc

        echo "Completion script installed for bash. Please restart your shell"
    else
        echo "Not supported shell. Completion install failed"
    fi
}

main
