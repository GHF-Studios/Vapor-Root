#!/usr/bin/env sh
set -eu

target="x86_64-unknown-linux-gnu"

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" 2>/dev/null && pwd || pwd)
app_root=$(CDPATH= cd -- "$script_dir/.." 2>/dev/null && pwd || pwd)
vapor="$app_root/bin/$target/vapor"
installer="$app_root/bin/$target/vapor-installer"
log_dir="$app_root/.vapor/logs"
launch_log="$log_dir/launch-wrapper.log"

if [ ! -x "$vapor" ]; then
    cwd=$(pwd)
    if [ -x "$cwd/bin/$target/vapor" ]; then
        app_root=$cwd
        vapor="$app_root/bin/$target/vapor"
        installer="$app_root/bin/$target/vapor-installer"
        log_dir="$app_root/.vapor/logs"
        launch_log="$log_dir/launch-wrapper.log"
    fi
fi

mkdir -p "$log_dir" 2>/dev/null || true

log() {
    timestamp=$(date '+%Y-%m-%dT%H:%M:%S%z' 2>/dev/null || date 2>/dev/null || echo "unknown-time")
    printf '%s vapor-launch[%s]: %s\n' "$timestamp" "$$" "$*" >>"$launch_log" 2>/dev/null || true
}

launch_terminal() {
    label=$1
    shift
    log "trying terminal '$label': $*"
    if "$@" >>"$launch_log" 2>&1; then
        log "terminal '$label' exited successfully"
        exit 0
    fi
    status=$?
    log "terminal '$label' exited with status $status"
    return 0
}

run_command() {
    set +e
    "$@"
    status=$?
    set -e
    log "command exited with status $status: $*"
    if [ "$status" -ne 0 ] && [ "${VAPOR_LAUNCHER_HOLD_ON_EXIT:-0}" = "1" ]; then
        echo
        echo "Vapor exited with status $status."
        echo "Log: $launch_log"
        echo "Starting an interactive shell. Close this window when you are done."
        exec "${SHELL:-/bin/sh}" -i
    fi
    exit "$status"
}

show_error_dialog() {
    title=$1
    message=$2
    if [ -n "${STEAM_ZENITY:-}" ] && [ -x "$STEAM_ZENITY" ]; then
        "$STEAM_ZENITY" --error --title="$title" --text="$message" >/dev/null 2>&1 || true
        return
    fi
    if command -v zenity >/dev/null 2>&1; then
        zenity --error --title="$title" --text="$message" >/dev/null 2>&1 || true
        return
    fi
    if command -v kdialog >/dev/null 2>&1; then
        kdialog --title "$title" --error "$message" >/dev/null 2>&1 || true
    fi
}

mode="${1:-shell}"
if [ "$#" -gt 0 ]; then
    shift
fi
export VAPOR_STEAM_LAUNCH=1
export VAPOR_LAUNCH_MODE="$mode"

if [ "${VAPOR_LAUNCHER_TERMINAL:-0}" != "1" ] && { [ ! -t 0 ] || [ ! -t 1 ]; }; then
    terminal_title="Vapor Shell"
    if [ "$mode" = "installer" ] || [ "$mode" = "vapor-installer" ]; then
        terminal_title="Vapor Installer"
    fi
    log "no controlling terminal; mode=$mode app_root=$app_root container=${container:-unset} pressure_vessel_runtime=${PRESSURE_VESSEL_RUNTIME:-unset}"
    if [ "${container:-}" = "pressure-vessel" ] || [ -n "${PRESSURE_VESSEL_RUNTIME:-}" ]; then
        if command -v xterm >/dev/null 2>&1; then
            xterm=$(command -v xterm)
            launch_terminal "steam-runtime-xterm" env VAPOR_LAUNCHER_TERMINAL=1 VAPOR_LAUNCHER_HOLD_ON_EXIT=1 \
                "$xterm" -T "$terminal_title" -e "$0" "$mode" "$@"
        fi
    fi
    konsole=
    if [ -x /usr/bin/konsole ]; then
        konsole=/usr/bin/konsole
    elif [ -x /run/host/usr/bin/konsole ]; then
        konsole=/run/host/usr/bin/konsole
    elif command -v konsole >/dev/null 2>&1; then
        konsole=$(command -v konsole)
    fi
    if [ -n "$konsole" ]; then
        launch_terminal "konsole" env VAPOR_LAUNCHER_TERMINAL=1 VAPOR_LAUNCHER_HOLD_ON_EXIT=1 \
            "$konsole" --nofork -p tabtitle="$terminal_title" --workdir "$app_root" \
            -e "$0" "$mode" "$@"
    fi
    if command -v x-terminal-emulator >/dev/null 2>&1; then
        x_terminal_emulator=$(command -v x-terminal-emulator)
        launch_terminal "x-terminal-emulator" env VAPOR_LAUNCHER_TERMINAL=1 VAPOR_LAUNCHER_HOLD_ON_EXIT=1 \
            "$x_terminal_emulator" -e "$0" "$mode" "$@"
    fi
    if command -v xterm >/dev/null 2>&1; then
        xterm=$(command -v xterm)
        launch_terminal "xterm" env VAPOR_LAUNCHER_TERMINAL=1 VAPOR_LAUNCHER_HOLD_ON_EXIT=1 \
            "$xterm" -T "$terminal_title" -e "$0" "$mode" "$@"
    fi
    message="Steam started Vapor without a terminal, and no usable desktop terminal could be launched.

Log: $launch_log

Install a terminal emulator, update/reinstall Vapor, or run:
$0 shell"
    show_error_dialog "Vapor launch wrapper" "$message"
    echo "Vapor launch wrapper"
    echo
    echo "error: Steam started Vapor without a terminal, and no usable desktop terminal could be launched"
    echo "  log: $launch_log"
    echo
    echo "Install a terminal emulator, update/reinstall Vapor, or run:"
    echo "  $0 shell"
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
        run_command "$installer" "$@"
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
        run_command "$vapor" --startup-script loo-cast "$@"
        ;;
    shell|vapor-shell)
        run_command "$vapor" "$@"
        ;;
    *)
        run_command "$vapor" "$mode" "$@"
        ;;
esac
