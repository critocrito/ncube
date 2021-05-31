#!/usr/bin/env bash
set -Eeuo pipefail
trap cleanup SIGINT SIGTERM ERR EXIT

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

usage() {
  cat <<EOF
Usage: $(basename "${BASH_SOURCE[0]}") [-h] [-f] PKG_DIR OUT_DIR

Build a DMG installer image.

Available options:

-h, --help      Print this help and exit
-f, --force     Overwrite any existing DMG image.
EOF
  exit
}

cleanup() {
  trap - SIGINT SIGTERM ERR EXIT
  # script cleanup here
}

setup_colors() {
  if [[ -t 2 ]] && [[ -z "${NO_COLOR-}" ]] && [[ "${TERM-}" != "dumb" ]]; then
    NOFORMAT='\033[0m' RED='\033[0;31m' GREEN='\033[0;32m' ORANGE='\033[0;33m' BLUE='\033[0;34m' PURPLE='\033[0;35m' CYAN='\033[0;36m' YELLOW='\033[1;33m'
  else
    NOFORMAT='' RED='' GREEN='' ORANGE='' BLUE='' PURPLE='' CYAN='' YELLOW=''
  fi
}

msg() {
  echo >&2 -e "${1-}"
}

msg_error() {
    msg "${RED}error${NOFORMAT} - ${1}"
}

msg_info() {
    msg "${CYAN}info${NOFORMAT}  - ${1}"
}

die() {
  local msg=$1
  local code=${2-1} # default exit status 1
  msg_error "$msg"
  exit "$code"
}

parse_params() {
  force=false

  while :; do
    case "${1-}" in
    -h | --help) usage ;;
    --no-color) NO_COLOR=1 ;;
    -f | --force)
      force=true
      ;;
    -?*) die "Unknown option: $1" ;;
    *) break ;;
    esac
    shift
  done

  args=("$@")

  # check required params and arguments
  [[ ${#args[@]} -eq 0 ]] && die "Missing script arguments"

  return 0
}

clean_image() {
  local image=$1
  test -f "$image" && rm "$image"
  return 0
}

create_image() {
  local source=$1
  local target=$2

  create-dmg \
    --volname Ncube \
    --volicon resources/icon.icns \
    --background resources/dmg-background@2x.png \
    --window-pos 200 120 \
    --window-size 660 400 \
    --icon-size 100 \
    --icon "Ncube.app" 160 190 \
    --hide-extension "Ncube.app" \
    --app-drop-link 500 185 \
    "$target" \
    "$source"
}

parse_params "$@"
setup_colors

pkg_dir="${args[0]}"
dmg_path="${args[1]}/Ncube.dmg"

if [[ -f "$dmg_path" && "$force" = false ]]; then
  die "DMG image already exists at $dmg_path. Use -f to overwrite."
fi

clean_image "$dmg_path"
create_image "$pkg_dir" "$dmg_path"
