#!/bin/sh

BIN_DIR="$HOME/.cargo/bin"
if [ ! -d $BIN_DIR ]; then
    echo "Could not find dir \"$BIN_DIR\". Is there a cargo installed?"
    exit 1
fi

NAMUI_LINK_PATH="$BIN_DIR/namui"

if [ ! -e $NAMUI_LINK_PATH ]; then
    echo "Could not find link \"$NAMUI_LINK_PATH\". Seems already uninstalled."
    exit 0
fi

echo "Deleting link."
cd $BIN_DIR && rm -f namui
if [ $? -ne 0 ]; then
    echo "Delete failed."
    exit 2
fi

echo "Successfully uninstalled."
