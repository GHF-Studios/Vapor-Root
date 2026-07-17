@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
for %%I in ("%SCRIPT_DIR%\..\..\..") do set "APP_ROOT=%%~fI"
set "VAPOR=%APP_ROOT%\bin\x86_64-pc-windows-msvc\vapor.exe"

set "MODE=%~1"
if "%MODE%"=="" set "MODE=shell"
if not "%~1"=="" shift /1

if /I "%MODE%"=="play" goto play
if /I "%MODE%"=="loo-cast" goto play
if /I "%MODE%"=="shell" goto shell
if /I "%MODE%"=="vapor-shell" goto shell

start "Vapor" /wait cmd /k ""%VAPOR%" %MODE% %*"
exit /b %ERRORLEVEL%

:play
start "Play Loo-Cast" /wait cmd /k ""%VAPOR%" --startup-script loo-cast %*"
exit /b %ERRORLEVEL%

:shell
start "Vapor Shell" /wait cmd /k ""%VAPOR%" %*"
exit /b %ERRORLEVEL%
