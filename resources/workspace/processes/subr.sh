make_id() {
  date +%s | sha256sum | cut -c 1-8
}

pipeline_name() {
    echo "$1" |
        grep name |
        sed -e 's/"name": "\(.*\)".*/\1/' |
        sed -e 's/^[[:space:]]*//' |
        sed -e 's/[[:space:]]*$//'
}

snake_case() {
  echo "$1" |
    tr '[:upper:]' '[:lower:]' |
    sed -e 's/[[:space:]]/_/g' |
    sed -e 's/-/_/g'
}
