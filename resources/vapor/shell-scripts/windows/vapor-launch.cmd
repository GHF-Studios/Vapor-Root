@echo off
setlocal EnableExtensions EnableDelayedExpansion

set "TARGET=x86_64-pc-windows-gnullvm"

if not "%VAPOR_APP_ROOT%"=="" (
    set "APP_ROOT=%VAPOR_APP_ROOT%"
) else (
    set "SCRIPT_DIR=%~dp0"
    for %%I in ("%SCRIPT_DIR%\..") do set "APP_ROOT=%%~fI"
)

set "VAPOR=%APP_ROOT%\bin\%TARGET%\vapor.exe"
set "INSTALLER=%APP_ROOT%\bin\%TARGET%\vapor-installer.exe"
set "LOG_DIR=%APP_ROOT%\.vapor\logs"
set "LAUNCH_LOG=%LOG_DIR%\launch-wrapper.log"

if not exist "%LOG_DIR%" mkdir "%LOG_DIR%" >nul 2>nul

set "MODE=%~1"
if "%MODE%"=="" (
    set "MODE=shell"
) else (
    shift /1
)

set "VAPOR_APP_ROOT=%APP_ROOT%"
set "VAPOR_STEAM_LAUNCH=1"
set "VAPOR_LAUNCH_MODE=%MODE%"
set "VAPOR_TERMINAL_RELAUNCHED=1"

set "FORWARD_ARGS="
:collect_args
if "%~1"=="" goto args_done
set "FORWARD_ARGS=%FORWARD_ARGS% ^"%~1^""
shift /1
goto collect_args

:args_done
call :log "mode=%MODE% app_root=%APP_ROOT% terminal=%VAPOR_LAUNCHER_TERMINAL% hold=%VAPOR_LAUNCHER_HOLD_ON_EXIT%"
pushd "%APP_ROOT%" >nul 2>nul
if errorlevel 1 call :log "could not change directory to app root"

echo Vapor launch wrapper
echo   mode: %MODE%
echo   app root: %APP_ROOT%
echo   log: %LAUNCH_LOG%
echo.

if /I "%MODE%"=="installer" goto installer
if /I "%MODE%"=="vapor-installer" goto installer
if not exist "%VAPOR%" goto missing_vapor

set "INSTALLER_LOG=%APP_ROOT%\.vapor\logs\installer.log"
if exist "%INSTALLER%" (
    call "%INSTALLER%" --quiet install --app-root "%APP_ROOT%"
    if errorlevel 1 (
        set "VAPOR_INSTALLER_INSTALL_FAILED=vapor-installer exited with status !ERRORLEVEL!"
        set "VAPOR_INSTALLER_LOG=%INSTALLER_LOG%"
    )
) else (
    set "VAPOR_INSTALLER_INSTALL_FAILED=vapor-installer is missing for %TARGET%"
    set "VAPOR_INSTALLER_LOG=%INSTALLER_LOG%"
)

if /I "%MODE%"=="play" goto play
if /I "%MODE%"=="loo-cast" goto play
if /I "%MODE%"=="shell" goto shell
if /I "%MODE%"=="vapor-shell" goto shell
goto command

:installer
if not exist "%INSTALLER%" goto missing_installer
call "%INSTALLER%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:play
call "%VAPOR%" --startup-script loo-cast %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:shell
call "%VAPOR%" %FORWARD_ARGS%
set "STATUS=%ERRORLEVEL%"
goto done

:command
call "%VAPOR%" %MODE% %FORWARD_ARGS%
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
call :log "vapor exited with status %STATUS%"
if defined VAPOR_LAUNCHER_HOLD_ON_EXIT (
    echo.
    echo Vapor exited with status %STATUS%.
    echo Log: %LAUNCH_LOG%
    echo This command prompt will stay open. Close it when you are done.
)
popd >nul 2>nul
exit /b %STATUS%

:log
if defined LAUNCH_LOG (
    >> "%LAUNCH_LOG%" echo [%DATE% %TIME%] vapor-launch: %~1
)
exit /b 0
