#!/usr/bin/env sh
set -eu

target="x86_64-unknown-linux-gnu"

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" 2>/dev/null && pwd || pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." 2>/dev/null && pwd || pwd)
vapor="$app_root/bin/$target/vapor"
installer="$app_root/bin/$target/vapor-installer"

if [ ! -x "$vapor" ]; then
    cwd=$(pwd)
    if [ -x "$cwd/bin/$target/vapor" ]; then
        app_root=$cwd
        vapor="$app_root/bin/$target/vapor"
        installer="$app_root/bin/$target/vapor-installer"
    fi
fi

mode="${1:-shell}"
if [ "$#" -gt 0 ]; then
    shift
fi
export VAPOR_STEAM_LAUNCH=1
export VAPOR_LAUNCH_MODE="$mode"

if [ "${VAPOR_LAUNCHER_TERMINAL:-0}" != "1" ] && { [ ! -t 0 ] || [ ! -t 1 ]; }; then
    konsole=
    terminal_title="Vapor Shell"
    if [ "$mode" = "installer" ] || [ "$mode" = "vapor-installer" ]; then
        terminal_title="Vapor Installer"
    fi
    if [ -x /usr/bin/konsole ]; then
        konsole=/usr/bin/konsole
    elif command -v konsole >/dev/null 2>&1; then
        konsole=$(command -v konsole)
    fi
    if [ -n "$konsole" ]; then
        exec env VAPOR_LAUNCHER_TERMINAL=1 \
            "$konsole" --nofork -p tabtitle="$terminal_title" --workdir "$app_root" \
            -e "$0" "$mode" "$@"
    fi
    echo "Vapor launch wrapper"
    echo
    echo "error: Steam started Vapor without a terminal, and Konsole was not found"
    echo "  checked: /usr/bin/konsole and PATH"
    echo
    echo "Install Konsole or run the Vapor launch option from a terminal."
    exit 127
fi

case "$mode" in
    installer|vapor-installer)
        if [ ! -x "$installer" ]; then
            echo "Vapor launch wrapper"
            echo
            echo "error: expected Vapor Installer executable is missing"
            echo "  target:  $target"
            echo "  checked: $installer"
            echo
            echo "This Steam install is missing the platform installer for this launch option."
            echo "Update or reinstall the app on the selected Steam branch, then try again."
            echo
            echo "Starting an interactive shell so the window stays open."
            exec "${SHELL:-/bin/sh}" -i
        fi
        exec "$installer" "$@"
        ;;
esac

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

installer_log="$app_root/.vapor/logs/installer.log"
if [ -x "$installer" ]; then
    if "$installer" --quiet install --app-root "$app_root"; then
        :
    else
        installer_status=$?
        export VAPOR_INSTALLER_INSTALL_FAILED="vapor-installer exited with status $installer_status"
        export VAPOR_INSTALLER_LOG="$installer_log"
    fi
else
    export VAPOR_INSTALLER_INSTALL_FAILED="vapor-installer is missing for $target"
    export VAPOR_INSTALLER_LOG="$installer_log"
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
