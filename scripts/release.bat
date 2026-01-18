@echo off
REM Release Build Script for Hacker News Reader
REM Usage: release.bat [version]

setlocal enabledelayedexpansion

REM Get version from argument or Cargo.toml
if "%~1"=="" (
    for /f "tokens=2 delims==" %%a in ('findstr "^version" Cargo.toml') do set VERSION=%%a
) else (
    set VERSION=%~1
)

set APP_NAME=my_egui_pro
set APP_DISPLAY_NAME=Hacker News Reader

echo ========================================
echo Building %APP_DISPLAY_NAME% v%VERSION%
echo ========================================

REM Determine platform
set PLATFORM=windows-amd64
set BINARY_NAME=%APP_NAME%.exe
set EXTENSION=.exe

REM Clean previous build
echo Cleaning previous build...
cargo clean

REM Build release
echo Building release binary...
cargo build --release

REM Create release directory
set RELEASE_DIR=release\%APP_NAME%-%VERSION%-%PLATFORM%
if exist "%RELEASE_DIR%" rmdir /s /q "%RELEASE_DIR%"
mkdir "%RELEASE_DIR%"

REM Copy binary
echo Copying binary to release directory...
copy target\release\%BINARY_NAME% "%RELEASE_DIR%\%APP_NAME%%EXTENSION%"

REM Create archive (requires 7-Zip or similar)
echo Creating release archive...
set ARCHIVE_NAME=%APP_NAME%-%VERSION%-%PLATFORM%.zip
powershell -Command "Compress-Archive -Path '%RELEASE_DIR%' -DestinationPath 'release\%ARCHIVE_NAME%' -Force"

REM Generate checksum
echo Generating checksums...
powershell -Command "Get-FileHash 'release\%ARCHIVE_NAME%' -Algorithm SHA256 | Select-Object -ExpandProperty Hash | Out-File 'release\%APP_NAME%-%VERSION%-%PLATFORM%.sha256'"

REM Create release notes
echo Creating release notes...
(
echo # %APP_DISPLAY_NAME% v%VERSION% - %PLATFORM%
echo.
echo ## Installation
echo.
echo ### Extract
echo ```powershell
echo Expand-Archive -Path %APP_NAME%-%VERSION%-%PLATFORM%.zip
echo cd %APP_NAME%-%VERSION%-%PLATFORM%
echo ```
echo.
echo ### Run
echo ```powershell
echo .\%APP_NAME%%EXTENSION%
echo ```
echo.
echo ## Features
echo.
echo - Hacker News reader with 6 categories ^(Top, New, Best, Ask, Show, Jobs^)
echo - Clickable story titles to open articles
echo - Save favorite stories ^SQLite storage^)
echo - Gruvbox theme ^(Dark/Light^) with toggle
echo.
echo ## System Requirements
echo.
echo - Windows system
echo - No additional dependencies required
echo.
echo ## Data Location
echo.
echo - Config: %%LOCALAPPDATA%%\my_egui_pro\config.json
echo - Database: %%LOCALAPPDATA%%\my_egui_pro\favorites.db
echo.
echo ## Changelog
echo.
echo - Initial release
) > "%RELEASE_DIR%\RELEASE_NOTES.md"

REM Summary
echo.
echo ========================================
echo Build Complete
echo ========================================
echo Platform: %PLATFORM%
echo Version: %VERSION%
echo Binary: %RELEASE_DIR%\%APP_NAME%%EXTENSION%
echo Archive: release\%ARCHIVE_NAME%
echo.
echo To install:
echo   Expand-Archive -Path release\%ARCHIVE_NAME%
echo   cd %APP_NAME%-%VERSION%-%PLATFORM%
echo   .\%APP_NAME%%EXTENSION%
echo.

endlocal
