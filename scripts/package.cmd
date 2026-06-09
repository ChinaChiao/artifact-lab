@echo off
setlocal

title Artifact Lab Packager
cd /d "%~dp0.."

echo.
echo Artifact Lab packaging started.
echo Project: %CD%
echo.

powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0package.ps1" %*
set "EXIT_CODE=%ERRORLEVEL%"

echo.
if "%EXIT_CODE%"=="0" (
  echo Packaging finished successfully.
) else (
  echo Packaging failed with exit code %EXIT_CODE%.
)
echo.
pause
exit /b %EXIT_CODE%
