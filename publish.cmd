@echo off
setlocal enableDelayedExpansion

echo current path: !cd!
cd /d !cd!

if "%~1"=="" (
        echo "Usage: publish.cmd [SUBMODULE]"
        exit /b 1
    ) else (
        call cargo publish --manifest-path crates/%1/Cargo.toml
    )

endlocal
