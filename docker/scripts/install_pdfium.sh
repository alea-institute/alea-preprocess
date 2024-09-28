#!/usr/bin/env bash

# install pdfium from here:
# https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz
# https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-arm64.tgz
# based on architecture

# get architecture
ARCH=$(uname -m)

if [ "$ARCH" == "x86_64" ]; then
    PDFIUM_URL="https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-x64.tgz"
elif [ "$ARCH" == "aarch64" ]; then
    PDFIUM_URL="https://github.com/bblanchon/pdfium-binaries/releases/latest/download/pdfium-linux-arm64.tgz"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

# download and extract pdfium
PDFIUM_DIR="/usr"
PDFIUM_TMP_FILE="pdfium.tgz"

# push to /tmp, download, extract, and then copy to PDFIUM_DIR
pushd /tmp
mkdir pdfium
pushd pdfium

# download pdfium
curl -L $PDFIUM_URL -o $PDFIUM_TMP_FILE

# extract pdfium
tar -xvf $PDFIUM_TMP_FILE

# copy to PDFIUM_DIR
cp -R include/* "$PDFIUM_DIR/include/"
cp -R lib/* "$PDFIUM_DIR/lib/"

# cleanup
popd
rm -rf pdfium
popd
