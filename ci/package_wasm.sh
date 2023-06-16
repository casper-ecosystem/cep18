#!/usr/bin/env bash

set -e

BUILD_ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." >/dev/null 2>&1 && pwd)"
IGNORE='cep18_test_contract'
WASM_FILE_PATH_ARRAY=($(cat "$BUILD_ROOT_DIR/Makefile" | grep 'wasm-strip' | awk -F' ' '{print $2}'))
TAG=${GITHUB_REF_NAME:-local}
TEMP_DIR="/tmp/ci_package_wasm_$TAG"
TARBALL="cep-18-wasm.tar.gz"

# Hygiene for local debugging. Won't apply to CI. 
if [ -d "$TEMP_DIR" ]; then
    rm -rf "$TEMP_DIR"
fi

# Create temporary directory for staging tarball
mkdir -p "$TEMP_DIR"

if [ -d "$TEMP_DIR" ]; then
    # Loop over the contracts
    for wasm_path in "${WASM_FILE_PATH_ARRAY[@]}"; do
        # Ignore minting_contract, used only in testing
        if [[ "$wasm_path" != *"$IGNORE"* ]]; then
            # Copy the other wasm files if they exist
            if [ -f "$wasm_path" ]; then
                echo "copying $wasm_path to $TEMP_DIR"
                cp "$wasm_path" "$TEMP_DIR/"
            fi
        fi
    done

    # Move to the staging directory
    pushd "$TEMP_DIR" > /dev/null
    echo ""
    echo "creating $TEMP_DIR/$TARBALL"
    echo ""
    # create the tarball
    tar -czf "$TARBALL" *.wasm --remove-files
    # Move back
    popd > /dev/null
fi

echo "success!"