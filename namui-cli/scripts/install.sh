#!/bin/bash

function main() {
    cli_root_path=$(cd $(dirname $0) && cd .. && pwd -P)
    cli_path="$cli_root_path/target/debug/namui-cli"
    cargo_bin_dir_path="$HOME/.cargo/bin"

    check_cargo_installed
    check_wasm_pack_installed
    check_cargo_bin_dir_exist $cargo_bin_dir_path

    build_cli $cli_root_path

    make_cli_symlink $cargo_bin_dir_path $cli_path

    echo "Successfully installed."
}

# Error Code
EXIT_CARGO_NOT_FOUND=1
EXIT_WASM_PACK_NOT_FOUND=2
EXIT_CARGO_BIN_DIR_NOT_FOUND=3
EXIT_CLI_BUILD_FAILED=4
EXIT_SYMLINK_MAKE_FAILED=5

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

main
