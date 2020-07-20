#!/usr/bin/env bash
set -e

msg_info() {
    printf "\r\033[2K\033[0;32m[ .. ] %s\033[0m\n" "$*"
}

msg_error() {
    printf "\r\033[2K\033[0;31m[ .. ] %s\033[0m\n" "$*"
}

usage() {
  msg_info "Usage: $0 <version>"
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

VERSION="$1"

if [ -z "$VERSION" ];
then
    except "Missing version, e.g. 0.1.0."
fi

uncallable "clog" && except "clog-cli not found. Run cargo install clog-cli."

make
make verify
make test

clog -C CHANGELOG.md --setversion "$VERSION" -F

while read -r p;
do
  perl -pi -e "s/^version = \".*?\"/version = \"$VERSION\"/" "src/rs/$p/Cargo.toml";
done < <(ls src/rs | sed -e 's/\(.*\)\//\1/g')

# This must be run on a mac
if is_mac; then
    /usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" resources/Info.plist
else
    except "Release on a mac."
fi

git add CHANGELOG.md
git add resources/Info.plist
while read -r p;
do
  git add "src/rs/$p/Cargo.toml"
done < <(ls src/rs | sed -e 's/\(.*\)\//\1/g')

msg_info "Open the editor to compose the commit message."

git commit -m "chore(release): release version $VERSION" -e
LAST_MSG=$(git log -1 --pretty=%B | sed 's/chore(release): //g')
git tag -a "$VERSION" -m "$LAST_MSG"
