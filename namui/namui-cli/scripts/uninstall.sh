#!/bin/bash

function main() {
    cargo_bin_dir_path=$(dirname $(which cargo))
    symlink_path="$cargo_bin_dir_path/namui"

    remove_symlink $symlink_path

    remove_completion_script

    if [ $(is_os_wsl) -eq 1 ]; then
        remove_electron_on_windows
    fi

    echo "Successfully uninstalled."
}

# Error Code
EXIT_SYMLINK_REMOVE_FAIL=1
ELECTRON_ON_WINDOW_REMOVE_FAILED=2
EXIT_REMOVE_COMPLETION_SCRIPT_FAILED=3

#######################################
# Arguments:
#   symlink_path: string
#######################################
function remove_symlink() {
    symlink_path=$1
    if [ -e $symlink_path ]; then
        rm -f $symlink_path
        if [ $? -ne 0 ]; then
            echo "Delete failed."
            exit $EXIT_SYMLINK_REMOVE_FAIL
        fi
    else
        echo "Could not find link \"$symlink_path\". Seems already uninstalled."
    fi
}

function is_os_wsl() {
    # https://github.com/microsoft/WSL/issues/423
    if [ $(uname -r | sed -n 's/.*\( *Microsoft *\).*/\1/ip') ]; then
        echo 1
    else
        echo 0
    fi
}

function remove_electron_on_windows() {
    window_electron_root_path="$(wslpath $(wslvar APPDATA))/namui/electron"

    rm -rf $window_electron_root_path
    if [ $? -ne 0 ]; then
        echo "Electron on window remove failed."
        exit $ELECTRON_ON_WINDOW_REMOVE_FAILED
    fi
}

function remove_completion_script() {
    cli_completion_root_path=$1
    completion_script_start_marker="# namui completion script start"
    completion_script_end_marker="# namui completion script end"
    
    if [ "$BASH" ]; then
        cli_completion_path="$cli_completion_root_path/namui.bash"

        # Remove old completion script
        sed -i "/^$completion_script_start_marker/,/^$completion_script_end_marker/d" ~/.bashrc
        if [ $? -ne 0 ]; then
            echo "Remove completion script failed"
            exit $EXIT_REMOVE_COMPLETION_SCRIPT_FAILED
        fi
    fi
}

main
