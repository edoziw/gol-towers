#!/bin/bash
set -o errexit && set -o pipefail && set -o nounset

web() {
  bevy run web
}

win() {
  bevy run
}

clippy() {
    cargo clippy --locked --workspace --all-targets --profile ci --all-features
}

main() {
if [[ "--debug" == "${1:-""}" ]]; then
  shift
  set -o xtrace
  export RUST_LOG=bevy_enhanced_input
fi

local -r mode="${1:-web}"

# clear the screen
clear

case "${mode}" in
  web|win|clippy)
    "$mode" "$@"
    ;;
  *)
    echo "Usage: $0 [[web]|win]"
    return
    ;;
esac

}

main "$@"