@echo off

if not exist .git (
    echo ERROR: .git folder not found. Please run this script in the root directory of your UpOrDawn git repository.
    exit /b 1
)

set "REPO=https://github.com/Snoupix/pre-commit_upordawn/releases/download/i686-pc-windows-gnu/pre-commit.exe"

where curl >nul 2>nul
IF %ERRORLEVEL% NEQ 0 (
    powershell -command "(New-Object System.Net.WebClient).DownloadFile('%REPO%', 'pre-commit.exe')"
) else (
    curl -o pre-commit.exe %REPO%
)
IF %ERRORLEVEL% NEQ 0 (
    echo Failed to download pre-commit script.
    exit /b 1
)

move pre-commit.exe .git\hooks\pre-commit.exe
IF %ERRORLEVEL% NEQ 0 (
    echo Failed to move pre-commit to .git\hooks directory.
    exit /b 1
)

echo Installed pre-commit hook successfully.
exit /b 0
