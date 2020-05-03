#!/usr/bin/env bash
set -e

usage() {
  echo "Usage: $0 <version>"
  exit 2
}

except() {
  # $1 -> reason
  echo "Error: $1"
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
    except "Missing version, e.g. 0.1."
fi

uncallable "clog" && except "clog-cli not found. Run cargo install clog-cli."

make
make test

clog -C CHANGELOG.md --setversion "$VERSION" -F

perl -pi -e "s/^version = \".*?\"/version = \"$VERSION\"/" src/rs/ncube/Cargo.toml
perl -pi -e "s/^version = \".*?\"/version = \"$VERSION\"/" src/rs/ncubed/Cargo.toml
perl -pi -e "s/^version = \".*?\"/version = \"$VERSION\"/" src/rs/ncube-data/Cargo.toml
perl -pi -e "s/^version = \".*?\"/version = \"$VERSION\"/" src/rs/ncubectl/Cargo.toml

# This must be run on a mac
if is_mac; then
    /usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" resources/Info.plist
else
    except "Release on a mac."
fi

git add CHANGELOG.md
git add resources/Info.plist
git add src/rs/ncube-data/Cargo.toml
git add src/rs/ncube/Cargo.toml
git add src/rs/ncubectl/Cargo.toml
git add src/rs/ncubed/Cargo.toml

git commit -m "chore(release): released version $VERSION"
git tag -a "$VERSION" -m "released version $VERSION"
