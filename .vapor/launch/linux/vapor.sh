#!/usr/bin/env sh
set -eu

target="x86_64-unknown-linux-gnu"

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" 2>/dev/null && pwd || pwd)
app_root=$(CDPATH= cd -- "$script_dir/../../.." 2>/dev/null && pwd || pwd)
vapor="$app_root/bin/$target/vapor"

if [ ! -x "$vapor" ]; then
    cwd=$(pwd)
    if [ -x "$cwd/bin/$target/vapor" ]; then
        app_root=$cwd
        vapor="$app_root/bin/$target/vapor"
    fi
fi

if [ ! -x "$vapor" ]; then
    echo "Vapor launch wrapper"
    echo
    echo "error: expected Vapor executable is missing"
    echo "  target:  $target"
    echo "  checked: $vapor"
    echo
    echo "This Steam install is missing the platform binary for this launch option."
    echo "Update or reinstall the app on the selected Steam branch, then try again."
    echo
    echo "Starting an interactive shell so the window stays open."
    exec "${SHELL:-/bin/sh}" -i
fi

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
