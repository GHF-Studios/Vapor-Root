#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
app_root=$(CDPATH= cd -- "$script_dir/../../.." && pwd)
vapor="$app_root/bin/x86_64-unknown-linux-gnu/vapor"

mode="${1:-shell}"
if [ "$#" -gt 0 ]; then
    shift
fi

case "$mode" in
    play|loo-cast)
        exec "$vapor" --startup-script loo-cast "$@"
        ;;
    shell|vapor-shell)
        exec "$vapor" "$@"
        ;;
    *)
        exec "$vapor" "$mode" "$@"
        ;;
esac
