@echo off
setlocal EnableExtensions

set "TARGET=x86_64-pc-windows-gnullvm"

if defined VAPOR_APP_ROOT goto app_root_from_env
set "SCRIPT_DIR=%~dp0"
pushd "%SCRIPT_DIR%\.." >nul 2>nul
if errorlevel 1 goto app_root_from_script_fallback
set "APP_ROOT=%CD%"
popd >nul 2>nul
goto app_root_ready

:app_root_from_script_fallback
set "APP_ROOT=%SCRIPT_DIR%\.."
goto app_root_ready

:app_root_from_env
set "APP_ROOT=%VAPOR_APP_ROOT%"
goto app_root_ready

:app_root_ready

set "VAPOR=%APP_ROOT%\bin\%TARGET%\vapor.exe"
set "INSTALLER=%APP_ROOT%\bin\%TARGET%\vapor-installer.exe"
set "LOG_DIR=%APP_ROOT%\.vapor\logs"
set "LAUNCH_LOG=%LOG_DIR%\launch-wrapper.log"

if not exist "%LOG_DIR%" mkdir "%LOG_DIR%" >nul 2>nul

set "MODE=%~1"
if "%MODE%"=="" goto default_mode
shift /1
goto mode_ready

:default_mode
set "MODE=shell"

:mode_ready

set "VAPOR_APP_ROOT=%APP_ROOT%"
set "VAPOR_HOME=%APP_ROOT%"
set "CARGO_HOME=%APP_ROOT%\cargo-home"
set "RUSTUP_HOME=%APP_ROOT%\rustup-home"
set "VAPOR_STEAM_LAUNCH=1"
set "VAPOR_LAUNCH_MODE=%MODE%"
set "VAPOR_TERMINAL_RELAUNCHED=1"
set "PATH=%APP_ROOT%\bin\%TARGET%;%APP_ROOT%\cargo-home\bin;%APP_ROOT%\tools\steamcmd;%APP_ROOT%\tools\zig;%APP_ROOT%\tools\cross\bin;%APP_ROOT%\tools\llvm-mingw\bin;%PATH%"
for /d %%T in ("%APP_ROOT%\rustup-home\toolchains\*") do if exist "%%~fT\bin" set "PATH=%%~fT\bin;%PATH%"

set "FORWARD_ARGS="
:collect_args
if "%~1"=="" goto args_done
set "FORWARD_ARGS=%FORWARD_ARGS% ^"%~1^""
shift /1
goto collect_args

:args_done
if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: mode=%MODE% app_root="%APP_ROOT%" terminal=%VAPOR_LAUNCHER_TERMINAL% hold=%VAPOR_LAUNCHER_HOLD_ON_EXIT%
pushd "%APP_ROOT%" >nul 2>nul
if errorlevel 1 if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: could not change directory to app root

echo Vapor launch wrapper
echo   mode: %MODE%
echo   app root: "%APP_ROOT%"
echo   log: "%LAUNCH_LOG%"
echo.

if /I "%MODE%"=="installer" goto installer
if /I "%MODE%"=="vapor-installer" goto installer
if not exist "%VAPOR%" goto missing_vapor

set "INSTALLER_LOG=%APP_ROOT%\.vapor\logs\installer.log"
if exist "%INSTALLER%" goto run_installer_bootstrap
set "VAPOR_INSTALLER_INSTALL_FAILED=vapor-installer is missing for %TARGET%"
set "VAPOR_INSTALLER_LOG=%INSTALLER_LOG%"
goto installer_bootstrap_done

:run_installer_bootstrap
"%INSTALLER%" --quiet install --app-root "%APP_ROOT%"
if errorlevel 1 goto installer_bootstrap_failed
goto installer_bootstrap_done

:installer_bootstrap_failed
set "VAPOR_INSTALLER_INSTALL_FAILED=vapor-installer exited with status %ERRORLEVEL%"
set "VAPOR_INSTALLER_LOG=%INSTALLER_LOG%"
goto installer_bootstrap_done

:installer_bootstrap_done

if /I "%MODE%"=="play" goto play
if /I "%MODE%"=="loo-cast" goto play
if /I "%MODE%"=="shell" goto shell
if /I "%MODE%"=="vapor-shell" goto shell
goto command

:installer
if not exist "%INSTALLER%" goto missing_installer
"%INSTALLER%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:play
"%VAPOR%" --startup-script loo-cast %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:shell
"%VAPOR%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:command
"%VAPOR%" "%MODE%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:missing_vapor
echo error: expected Vapor executable is missing
echo   target:  %TARGET%
echo   checked: %VAPOR%
echo.
echo This Steam install is missing the platform binary for this launch option.
echo Update or reinstall the app on the selected Steam branch, then try again.
set "STATUS=9009"
goto done

:missing_installer
echo error: expected Vapor Installer executable is missing
echo   target:  %TARGET%
echo   checked: %INSTALLER%
echo.
echo This Steam install is missing the platform installer for this launch option.
echo Update or reinstall the app on the selected Steam branch, then try again.
set "STATUS=9009"
goto done

:done
if defined LAUNCH_LOG >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: vapor exited with status %STATUS%
if not defined VAPOR_LAUNCHER_HOLD_ON_EXIT goto done_no_hold_message
echo.
echo Vapor exited with status %STATUS%.
echo Log: "%LAUNCH_LOG%"
echo Starting an interactive command prompt. Type exit when you are done.
if defined ComSpec (
    "%ComSpec%" /D /K
) else (
    cmd.exe /D /K
)

:done_no_hold_message
popd >nul 2>nul
exit /b %STATUS%
