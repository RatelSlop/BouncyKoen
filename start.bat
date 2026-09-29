@echo off
title Bouncy Koen
cd /d "%~dp0"
powershell -ExecutionPolicy Bypass -File "%~dp0server.ps1"
if %ERRORLEVEL% NEQ 0 (
    echo Kon server niet starten, openen direct in browser...
    start "" "%~dp0index.html"
)
pause
