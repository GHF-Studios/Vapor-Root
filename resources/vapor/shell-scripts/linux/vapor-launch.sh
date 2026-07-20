#!/bin/sh
set -eu

target="x86_64-unknown-linux-gnu"

if [ -n "${VAPOR_APP_ROOT:-}" ]; then
    app_root=$VAPOR_APP_ROOT
else
    script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" 2>/dev/null && pwd || pwd)
    app_root=$(CDPATH= cd -- "$script_dir/.." 2>/dev/null && pwd || pwd)
fi

vapor="$app_root/bin/$target/vapor"
installer="$app_root/bin/$target/vapor-installer"
log_dir="$app_root/.vapor/logs"
launch_log="$log_dir/launch-wrapper.log"

mkdir -p "$log_dir" 2>/dev/null || true

log() {
    timestamp=$(date '+%Y-%m-%dT%H:%M:%S%z' 2>/dev/null || date 2>/dev/null || echo "unknown-time")
    printf '%s vapor-launch[%s]: %s\n' "$timestamp" "$$" "$*" >>"$launch_log" 2>/dev/null || true
}

run_command() {
    set +e
    "$@"
    status=$?
    set -e
    log "command exited with status $status: $*"
    if [ "${VAPOR_LAUNCHER_HOLD_ON_EXIT:-0}" = "1" ]; then
        echo
        echo "Vapor exited with status $status."
        echo "Log: $launch_log"
        echo "Starting an interactive shell. Close this window when you are done."
        log "keeping terminal open after command status $status"
        exec "${SHELL:-/bin/sh}" -i
    fi
    exit "$status"
}

mode="${1:-shell}"
if [ "$#" -gt 0 ]; then
    shift
fi

export VAPOR_APP_ROOT="$app_root"
export VAPOR_STEAM_LAUNCH=1
export VAPOR_LAUNCH_MODE="$mode"
export VAPOR_TERMINAL_RELAUNCHED=1

cd "$app_root" 2>/dev/null || true
log "mode=$mode app_root=$app_root terminal=${VAPOR_LAUNCHER_TERMINAL:-0} hold=${VAPOR_LAUNCHER_HOLD_ON_EXIT:-0}"

echo "Vapor launch wrapper"
echo "  mode: $mode"
echo "  app root: $app_root"
echo "  log: $launch_log"
echo

case "$mode" in
    installer|vapor-installer)
        if [ ! -x "$installer" ]; then
            echo "error: expected Vapor Installer executable is missing"
            echo "  target:  $target"
            echo "  checked: $installer"
            echo
            echo "This Steam install is missing the platform installer for this launch option."
            echo "Update or reinstall the app on the selected Steam branch, then try again."
            exit 127
        fi
        run_command "$installer" "$@"
        ;;
esac

if [ ! -x "$vapor" ]; then
    echo "error: expected Vapor executable is missing"
    echo "  target:  $target"
    echo "  checked: $vapor"
    echo
    echo "This Steam install is missing the platform binary for this launch option."
    echo "Update or reinstall the app on the selected Steam branch, then try again."
    exit 127
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
