#!/bin/bash

function main() {
    cargo_bin_dir_path="$HOME/.cargo/bin"
    symlink_path="$cargo_bin_dir_path/namui"

    remove_symlink $symlink_path

    echo "Successfully uninstalled."
}

# Error Code
EXIT_SYMLINK_REMOVE_FAIL=1

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

main
