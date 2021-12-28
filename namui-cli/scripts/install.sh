#!/bin/sh

cargo --version
if [ $? -ne 0 ]; then
    echo "Cargo command execution failed. Is there a cargo installed?"
    exit 4
fi

wasm-pack --version
if [ $? -ne 0 ]; then
    echo "Wasm-pack command execution failed. Is there a wasm-pack installed?\nIf not, install it with \"cargo install wasm-pack\"."
    exit 5
fi

BIN_DIR="$HOME/.cargo/bin"
if [ ! -d $BIN_DIR ]; then
    echo "Could not find dir \"$BIN_DIR\". Is there a cargo installed?"
    exit 1
fi

ROOT_DIR=$(cd $(dirname $0) && cd .. && pwd -P)
NAMUI_CLI_PATH="$ROOT_DIR/target/debug/namui-cli"
if [ ! -e $NAMUI_CLI_PATH ]; then
    echo "Could not find \"$NAMUI_CLI_PATH\". Running \"cargo build\""
    $(cd $ROOT_DIR && cargo build)
    if [ $? -ne 0 ]; then
        echo "Build failed."
        exit 2
    fi
fi

echo "Making link."
cd $BIN_DIR && ln -sf $NAMUI_CLI_PATH namui
if [ $? -ne 0 ]; then
    echo "Link failed."
    exit 3
fi

echo "Successfully installed."
