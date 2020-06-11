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

if is_mac;
then
    RELEASE="darwin"
elif is_linux;
then
    RELEASE="linux"
fi

NODE_VERSION="v12.16.3"
FFMPEG_VERSION="4.2.2"

NODE_URL="https://nodejs.org/dist/$NODE_VERSION/node-$NODE_VERSION-$RELEASE-x64.tar.gz"
FFMPEG_URL="https://github.com/eugeneware/ffmpeg-static/releases/download/b$FFMPEG_VERSION/$RELEASE-x64"
YOUTUBEDL_URL="https://yt-dl.org/downloads/latest/youtube-dl"

mkdir -p "$BUILD_DIR"/dist/{nodejs,ffmpeg,youtube-dl}
cp -a resources/workspace/* "$BUILD_DIR"

curl -s "$NODE_URL" | tar xz --strip-components 1 -C "$WORKSPACE_DIR"/build/dist/nodejs
curl -s -o "$BUILD_DIR"/dist/ffmpeg/ffmpeg -L "$FFMPEG_URL"
curl -k -s -o "$BUILD_DIR"/dist/youtube-dl/youtube-dl -L "$YOUTUBEDL_URL"

chmod +x "$BUILD_DIR"/dist/ffmpeg/ffmpeg
chmod +x "$BUILD_DIR"/dist/youtube-dl/youtube-dl

tar czf "$WORKSPACE_ARCHIVE" -C "$BUILD_DIR" .
