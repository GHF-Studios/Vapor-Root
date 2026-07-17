@echo off
setlocal EnableExtensions

set "TARGET=x86_64-pc-windows-gnullvm"
set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%\..\..\..") do set "APP_ROOT=%%~fI"
set "VAPOR=%APP_ROOT%\bin\%TARGET%\vapor.exe"

if not exist "%VAPOR%" (
    if exist "%CD%\bin\%TARGET%\vapor.exe" (
        set "APP_ROOT=%CD%"
        set "VAPOR=%CD%\bin\%TARGET%\vapor.exe"
    )
)

set "MODE=%~1"
if "%MODE%"=="" (
    set "MODE=shell"
) else (
    shift /1
)

set "FORWARD_ARGS="
:collect_args
if "%~1"=="" goto args_done
set "FORWARD_ARGS=%FORWARD_ARGS% "%~1""
shift /1
goto collect_args

:args_done
if not exist "%VAPOR%" goto missing_vapor

if /I "%MODE%"=="play" goto play
if /I "%MODE%"=="loo-cast" goto play
if /I "%MODE%"=="shell" goto shell
if /I "%MODE%"=="vapor-shell" goto shell
goto command

:play
call "%VAPOR%" --startup-script loo-cast %FORWARD_ARGS%
goto after_vapor

:shell
call "%VAPOR%" %FORWARD_ARGS%
goto after_vapor

:command
call "%VAPOR%" %MODE% %FORWARD_ARGS%
goto after_vapor

:missing_vapor
echo Vapor launch wrapper
echo.
echo error: expected Vapor executable is missing
echo   target:  %TARGET%
echo   checked: %VAPOR%
echo.
echo This Steam install is missing the platform binary for this launch option.
echo Update or reinstall the app on the selected Steam branch, then try again.
set "STATUS=9009"
goto keep_open

:after_vapor
set "STATUS=%ERRORLEVEL%"

:keep_open
echo.
echo Vapor exited with status %STATUS%.
echo Starting an interactive command prompt. Close this window when you are done.
cmd /k
exit /b %STATUS%
