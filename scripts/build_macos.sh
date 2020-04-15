#!/usr/bin/env bash

set -e

RESOURCES=resources
MACOS_BIN_NAME=ncube-bin
MACOS_APP_NAME=Ncube
MACOS_APP_DIR=$MACOS_APP_NAME.app

echo "Creating app directory structure"
rm -rf "$MACOS_APP_DIR"
mkdir -p "$MACOS_APP_DIR/Contents/MacOS"

cargo build --bin ncube --release

echo "Copying binary"
MACOS_APP_BIN="$MACOS_APP_DIR/Contents/MacOS/$MACOS_BIN_NAME"
cp target/release/ncube "$MACOS_APP_BIN"

# echo "Linking binary with frameworks"
# for old in `otool -L $MACOS_APP_BIN | grep @rpath | cut -f2 | cut -d' ' -f1`; do
#     new=`echo $old | sed -e "s/@rpath/@executable_path\/..\/Frameworks/"`
#     echo "Replacing '$old' with '$new'"
#     install_name_tool -change $old $new $MACOS_APP_BIN
# done

echo "Copying resources directory"
mkdir "$MACOS_APP_DIR/Contents/Resources"
cp -r "$RESOURCES/dist" "$MACOS_APP_DIR/Contents/Resources"
cp -r "$RESOURCES/Info.plist" "$MACOS_APP_DIR/Contents"

echo "Copying launcher"
cp scripts/macos_launch.sh "$MACOS_APP_DIR/Contents/MacOS/$MACOS_APP_NAME"

OS=macos
MACHINE=x86_64

echo "Creating dmg"
mkdir "$MACOS_APP_NAME"
mv "$MACOS_APP_DIR" "$MACOS_APP_NAME"
ln -s /Applications "$MACOS_APP_NAME/Applications"
rm -rf "$MACOS_APP_NAME/.Trashes"

FULL_NAME="$MACOS_APP_NAME-$OS-$MACHINE"

test -f Ncube.dmg && rm Ncube.dmg
create-dmg --volname "Ncube Installer" \
    --background resources/dmg_background.png \
    --window-pos 200 120 \
    --window-size 800 600 \
    --icon-size 100 \
    --icon "Ncube.app" 200 190 \
    --hide-extension Ncube.app \
    --app-drop-link 600 185 \
    Ncube.dmg \
    Ncube/Ncube.app

# hdiutil create -fs HFS+ -volname "Ncube" "$FULL_NAME.dmg" -srcfolder "$MACOS_APP_NAME" -ov
rm -rf "$MACOS_APP_NAME"

