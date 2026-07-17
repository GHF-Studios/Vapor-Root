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

mode="${1:-shell}"
if [ "$#" -gt 0 ]; then
    shift
fi

if [ "${VAPOR_LAUNCHER_TERMINAL:-0}" != "1" ] && { [ ! -t 0 ] || [ ! -t 1 ]; }; then
    konsole=
    if [ -x /usr/bin/konsole ]; then
        konsole=/usr/bin/konsole
    elif command -v konsole >/dev/null 2>&1; then
        konsole=$(command -v konsole)
    fi
    if [ -n "$konsole" ]; then
        exec env VAPOR_LAUNCHER_TERMINAL=1 \
            "$konsole" --nofork -p tabtitle="Vapor Shell" --workdir "$app_root" \
            -e "$0" "$mode" "$@"
    fi
    echo "Vapor launch wrapper"
    echo
    echo "error: Steam started Vapor without a terminal, and Konsole was not found"
    echo "  checked: /usr/bin/konsole and PATH"
    echo
    echo "Install Konsole or run the Vapor Shell launch option from a terminal."
    exit 127
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
