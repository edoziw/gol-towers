#!/bin/bash
set -o errexit && set -o pipefail && set -o nounset

web() {
  lint && bevy run web
}

win() {
  lint && bevy run
}

lint() {
  clippy && bevy_lint
}

clippy() {
    cargo clippy --locked --workspace --all-targets --profile ci --all-features
}

bevy_lint() {
  bevy lint
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
  web|win|clippy|bevy_lint|lint)
    "$mode" "$@"
    ;;
  *)
    echo "Usage: $0 [[web]|win]"
    return
    ;;
esac

}

main "$@"