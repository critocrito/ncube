#!/usr/bin/env bash
set -euo pipefail

msg_info() {
    printf "\r\033[2K\033[0;32m[ .. ] %s\033[0m\n" "$*"
}

msg_error() {
    printf "\r\033[2K\033[0;31m[ .. ] %s\033[0m\n" "$*"
}

usage() {
  msg_info "Usage: $0 <workspace_dir> <workspace_zip>"
  exit 2
}

except() {
  # $1 -> reason
  msg_error "Error: $1"
  usage
}

uncallable() {
  ! command -v "$1" > /dev/null
}

_is_os() {
  [ "$(uname)" = "$1" ]
}

is_mac() {
  _is_os "Darwin"
}

is_linux() {
    _is_os "Linux"
}

WORKSPACE_DIR="$1"
WORKSPACE_ARCHIVE="$2"

BUILD_DIR="$WORKSPACE_DIR"/build

NODE_VERSION="v12.16.3"

if is_mac;
then
    NODE_RELEASE="darwin"
elif is_linux;
then
    NODE_RELEASE="linux"
fi
NODE_URL="https://nodejs.org/dist/$NODE_VERSION/node-$NODE_VERSION-$NODE_RELEASE-x64.tar.gz"

mkdir -p "$BUILD_DIR"/dist/nodejs
cp -a resources/workspace/* "$BUILD_DIR"
curl -s "$NODE_URL" | tar xz --strip-components 1 -C "$WORKSPACE_DIR"/build/dist/nodejs
tar czf "$WORKSPACE_ARCHIVE" -C "$BUILD_DIR" .
